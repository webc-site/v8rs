//! MemoryMappedFile：对应 V8 `OS::MemoryMappedFile`。
//!
//! 使用 `std::fs::File` + `memmap2::{Mmap, MmapMut}`；语义与 `platform-posix.cc`
//! 的 `PosixMemoryMappedFile` 对齐：
//! - `open(name, mode)`：`ReadOnly` 使用 `MAP_PRIVATE PROT_READ`；`ReadWrite` 使用
//!   `MAP_SHARED PROT_READ|PROT_WRITE`。
//! - `create(name, size, initial)`：若文件已存在则截断；整块写入 `initial`
//!   （长度不足 `size` 即失败，对齐原版短写 nullptr）；以 shared+rw 映射。

use std::{
  fmt::{self, Debug, Formatter},
  fs::{File, OpenOptions},
  io::{self, Seek, SeekFrom, Write},
  path::Path,
  ptr::null,
};

use memmap2::{Mmap, MmapMut};

/// 与 V8 `OS::MemoryMappedFile::FileMode` 对应。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
  ReadOnly,
  ReadWrite,
}

/// 具体映射；`open` / `create` 的返回值。
pub struct MemoryMappedFileHandle {
  inner: MemoryMappedFileImpl,
}

enum MemoryMappedFileImpl {
  /// 原版 size==0 时返回 (file, nullptr, 0) 的有效对象（platform-posix.cc:869/894）。
  Empty,
  ReadOnly(Mmap),
  ReadWriteShared(MmapMut),
}

impl Debug for MemoryMappedFileHandle {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let (kind, len) = match &self.inner {
      MemoryMappedFileImpl::Empty => ("empty", 0),
      MemoryMappedFileImpl::ReadOnly(m) => ("ro", m.len()),
      MemoryMappedFileImpl::ReadWriteShared(m) => ("rw", m.len()),
    };
    f.debug_struct("MemoryMappedFileHandle")
      .field("kind", &kind)
      .field("size", &len)
      .finish()
  }
}

impl MemoryMappedFileHandle {
  /// 对应 V8 `memory()`。返回指向映射基址的不可变指针；写访问由 `as_mut` 提供。
  /// 空文件（size==0）返回 null（原版同语义）。
  pub fn memory(&self) -> *const u8 {
    match &self.inner {
      MemoryMappedFileImpl::Empty => null(),
      MemoryMappedFileImpl::ReadOnly(m) => m.as_ptr(),
      MemoryMappedFileImpl::ReadWriteShared(m) => m.as_ptr(),
    }
  }

  /// 对应 V8 `size()`。
  pub fn size(&self) -> usize {
    match &self.inner {
      MemoryMappedFileImpl::Empty => 0,
      MemoryMappedFileImpl::ReadOnly(m) => m.len(),
      MemoryMappedFileImpl::ReadWriteShared(m) => m.len(),
    }
  }

  /// 只读字节切片。
  pub fn as_slice(&self) -> &[u8] {
    match &self.inner {
      MemoryMappedFileImpl::Empty => &[],
      MemoryMappedFileImpl::ReadOnly(m) => m.as_ref(),
      MemoryMappedFileImpl::ReadWriteShared(m) => m.as_ref(),
    }
  }

  /// 可写字节切片（仅 `ReadWrite` 映射可用）。
  pub fn as_mut_slice(&mut self) -> Option<&mut [u8]> {
    match &mut self.inner {
      MemoryMappedFileImpl::ReadWriteShared(m) => Some(m.as_mut()),
      _ => None,
    }
  }

  /// 显式将共享映射刷回磁盘。
  pub fn flush(&mut self) -> io::Result<()> {
    match &mut self.inner {
      MemoryMappedFileImpl::ReadWriteShared(m) => m.flush(),
      _ => Ok(()),
    }
  }
}

/// 与 V8 `OS::MemoryMappedFile` 静态方法对齐；Rust 侧仅暴露工厂函数。
#[derive(Debug, Clone, Copy)]
pub struct MemoryMappedFile;

impl MemoryMappedFile {
  /// 对应 V8 `MemoryMappedFile::open(name, mode)`；空文件返回有效 handle
  /// （size=0、memory=null，原版 platform-posix.cc:869 语义）。
  pub fn open(path: impl AsRef<Path>, mode: FileMode) -> Option<MemoryMappedFileHandle> {
    match mode {
      FileMode::ReadOnly => {
        let file = File::open(&path).ok()?;
        // SAFETY: 文件在映射期间存活（Mmap 拥有 File）。
        match unsafe { Mmap::map(&file) } {
          Ok(mmap) if mmap.is_empty() => Some(MemoryMappedFileHandle {
            inner: MemoryMappedFileImpl::Empty,
          }),
          Ok(mmap) => Some(MemoryMappedFileHandle {
            inner: MemoryMappedFileImpl::ReadOnly(mmap),
          }),
          // 零长文件在部分平台 mmap 报错：同样按原版返回有效空对象。
          Err(_) if file.metadata().is_ok_and(|m| m.len() == 0) => Some(MemoryMappedFileHandle {
            inner: MemoryMappedFileImpl::Empty,
          }),
          Err(_) => None,
        }
      }
      FileMode::ReadWrite => {
        let file = OpenOptions::new().read(true).write(true).open(&path).ok()?;
        // SAFETY: 见上。
        match unsafe { MmapMut::map_mut(&file) } {
          Ok(mmap) if mmap.is_empty() => Some(MemoryMappedFileHandle {
            inner: MemoryMappedFileImpl::Empty,
          }),
          Ok(mmap) => Some(MemoryMappedFileHandle {
            inner: MemoryMappedFileImpl::ReadWriteShared(mmap),
          }),
          Err(_) if file.metadata().is_ok_and(|m| m.len() == 0) => Some(MemoryMappedFileHandle {
            inner: MemoryMappedFileImpl::Empty,
          }),
          Err(_) => None,
        }
      }
    }
  }

  /// 对应 V8 `MemoryMappedFile::create(name, size, initial)`。
  /// 若文件已存在会被截断；size==0 返回有效空 handle（原版 cc:894 语义）。
  /// `initial` 长度不足 `size` 视为失败返回 `None`——对齐原版
  /// `fwrite(initial, 1, size)` 短写/错误即 nullptr（platform-posix.cc:888-892）。
  pub fn create(
    path: impl AsRef<Path>,
    size: usize,
    initial: &[u8],
  ) -> Option<MemoryMappedFileHandle> {
    if initial.len() < size {
      return None;
    }
    let mut file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .truncate(true)
      .open(path)
      .ok()?;
    if size == 0 {
      return Some(MemoryMappedFileHandle {
        inner: MemoryMappedFileImpl::Empty,
      });
    }
    // 原版整块写 size 字节；此处 head == &initial[..size]（长度已在上方保证）。
    let head = &initial[..size];
    file.write_all(head).ok()?;
    file.seek(SeekFrom::Start(0)).ok()?;
    // SAFETY: 文件在映射期间存活（MmapMut 拥有 File）。
    let mmap = unsafe { MmapMut::map_mut(&file) }.ok()?;
    if mmap.is_empty() {
      return None;
    }
    Some(MemoryMappedFileHandle {
      inner: MemoryMappedFileImpl::ReadWriteShared(mmap),
    })
  }
}
