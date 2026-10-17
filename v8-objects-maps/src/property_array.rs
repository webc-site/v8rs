// v8-objects-maps/src/property_array.rs
// 对应原版 v8/src/objects/property-array.h + property-array-inl.h

//! PropertyArray: 存储超出对象内字段（out-of-object）的命名属性堆数组。
//!
//! 包含一个双义编码的 `length_and_hash` 字段（以 Smi 存储）：
//! - 低 10 位（0..9）编码数组长度（最大 1023）
//! - 高 20 位（10..29）编码哈希码（0 表示无哈希）

use std::{
  mem,
  ops::Deref,
  ptr,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_core::{
  heap_object::HeapObject,
  map::Map,
  smi::Smi,
  tagged::{Object, Tagged},
  tagged_field::TaggedSlot,
};

use crate::error::{Error, Result};

pub const K_LENGTH_FIELD_SHIFT: u32 = 0;
pub const K_LENGTH_FIELD_SIZE: u32 = 10;
pub const K_LENGTH_FIELD_MASK: u32 = (1 << K_LENGTH_FIELD_SIZE) - 1; // 0x3ff = 1023
pub const K_MAX_LENGTH: usize = K_LENGTH_FIELD_MASK as usize;

pub const K_HASH_FIELD_SHIFT: u32 = K_LENGTH_FIELD_SIZE;
pub const K_HASH_FIELD_SIZE: u32 = 20;
pub const K_HASH_FIELD_MASK: u32 = ((1 << K_HASH_FIELD_SIZE) - 1) << K_HASH_FIELD_SHIFT;
pub const K_MAX_HASH: u32 = (1 << K_HASH_FIELD_SIZE) - 1;

pub const K_NO_HASH_SENTINEL: i32 = 0;

/// PropertyArray 头结构体
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PropertyArray {
  pub header: HeapObject,
  pub length_and_hash: Tagged<Smi>,
}

impl PropertyArray {
  pub const K_HEADER_SIZE: usize = mem::size_of::<Self>();
  pub const K_SLOT_SIZE: usize = mem::size_of::<TaggedSlot>();

  #[inline]
  pub const fn new(map: Tagged<Map>, length: u32) -> Self {
    let initial_smi = Smi::from_int((length & K_LENGTH_FIELD_MASK) as i32);
    Self {
      header: HeapObject::new(map),
      length_and_hash: initial_smi,
    }
  }

  /// 计算指定长度所需的内存字节大小
  #[inline]
  #[must_use]
  pub const fn size_for(length: usize) -> usize {
    Self::K_HEADER_SIZE + length * Self::K_SLOT_SIZE
  }

  /// 计算指定下标元素的内存偏移量
  #[inline]
  #[must_use]
  pub const fn offset_of_element_at(index: usize) -> usize {
    Self::size_for(index)
  }

  /// 从 word 偏移转换为数组元素下标
  #[inline]
  #[must_use]
  pub const fn offset_in_words_to_index(offset_in_words: usize) -> usize {
    offset_in_words.saturating_sub(Self::K_HEADER_SIZE / Self::K_SLOT_SIZE)
  }

  /// 在 Zone 中分配 PropertyArray
  pub fn allocate_in_zone(
    zone: &v8_zone::Zone,
    map: Tagged<Map>,
    length: u32,
  ) -> Result<Tagged<Self>> {
    if length as usize > K_MAX_LENGTH {
      return Err(Error::InvalidCapacity(length as usize));
    }
    let size = Self::size_for(length as usize);
    let slice = zone.allocate_bytes_ref(size)?;
    slice.fill(mem::MaybeUninit::new(0));
    let ptr = slice.as_mut_ptr();
    let header = Self::new(map, length);
    // SAFETY: zone 分配的内存有效且未对齐问题被 zone 保证
    unsafe {
      ptr::write(ptr as *mut Self, header);
      Ok(Tagged::from_raw_address(ptr as usize))
    }
  }
}

/// 对 `Tagged<PropertyArray>` 的操作扩展
pub trait PropertyArrayExt {
  fn length(&self) -> usize;
  fn initialize_length(&self, length: usize) -> Result<()>;
  fn hash(&self) -> i32;
  fn set_hash(&self, hash: i32) -> Result<()>;

  fn get(&self, index: usize) -> Result<Tagged<Object>>;
  fn set(&self, index: usize, value: Tagged<Object>) -> Result<()>;
  fn swap(&self, index: usize, value: Tagged<Object>) -> Result<Tagged<Object>>;
  fn swap_elements(&self, index1: usize, index2: usize) -> Result<()>;
  fn compare_and_swap(
    &self,
    index: usize,
    expected: Tagged<Object>,
    value: Tagged<Object>,
  ) -> Result<Tagged<Object>>;

  fn copy_elements(
    &self,
    dst_index: usize,
    src: Tagged<PropertyArray>,
    src_index: usize,
    len: usize,
  ) -> Result<()>;
}

impl PropertyArrayExt for Tagged<PropertyArray> {
  #[inline]
  fn length(&self) -> usize {
    let slot_addr =
      (self.address() + mem::offset_of!(PropertyArray, length_and_hash)) as *mut usize;
    // SAFETY: slot_addr 是有效 PropertyArray 对象的 length_and_hash 槽位
    let raw = unsafe { AtomicUsize::from_ptr(slot_addr).load(Ordering::Acquire) };
    let smi = Tagged::<Smi>::new(raw);
    (smi.value() as u32 & K_LENGTH_FIELD_MASK) as usize
  }

  #[inline]
  fn initialize_length(&self, length: usize) -> Result<()> {
    if length > K_MAX_LENGTH {
      return Err(Error::InvalidCapacity(length));
    }
    let slot_addr =
      (self.address() + mem::offset_of!(PropertyArray, length_and_hash)) as *mut usize;
    // SAFETY: slot_addr 有效且独占初始化
    let cur_raw = unsafe { AtomicUsize::from_ptr(slot_addr).load(Ordering::Relaxed) };
    let cur_smi = Tagged::<Smi>::new(cur_raw);
    let cur = cur_smi.value() as u32;
    let new_val = (cur & !K_LENGTH_FIELD_MASK) | (length as u32 & K_LENGTH_FIELD_MASK);
    let new_smi = Smi::from_int(new_val as i32);
    // SAFETY: 发布新长度，对齐 ReleaseStore 语义
    unsafe {
      AtomicUsize::from_ptr(slot_addr).store(new_smi.ptr(), Ordering::Release);
    }
    Ok(())
  }

  #[inline]
  fn hash(&self) -> i32 {
    let slot_addr =
      (self.address() + mem::offset_of!(PropertyArray, length_and_hash)) as *mut usize;
    // SAFETY: slot_addr 是有效 PropertyArray 对象的 length_and_hash 槽位
    let raw = unsafe { AtomicUsize::from_ptr(slot_addr).load(Ordering::Acquire) };
    let smi = Tagged::<Smi>::new(raw);
    ((smi.value() as u32 & K_HASH_FIELD_MASK) >> K_HASH_FIELD_SHIFT) as i32
  }

  #[inline]
  fn set_hash(&self, hash: i32) -> Result<()> {
    if hash < 0 || (hash as u32) > K_MAX_HASH {
      return Err(Error::HashOverflow(hash as u32));
    }
    let slot_addr =
      (self.address() + mem::offset_of!(PropertyArray, length_and_hash)) as *mut usize;
    // SAFETY: 原子槽位更新，施加 Release/Acquire 语义避免并发竞争冲突
    let atomic = unsafe { AtomicUsize::from_ptr(slot_addr) };
    let _ = atomic.try_update(Ordering::Release, Ordering::Acquire, |raw| {
      let cur_smi = Tagged::<Smi>::new(raw);
      let cur = cur_smi.value() as u32;
      let new_val =
        (cur & !K_HASH_FIELD_MASK) | (((hash as u32) << K_HASH_FIELD_SHIFT) & K_HASH_FIELD_MASK);
      Some(Smi::from_int(new_val as i32).ptr())
    });
    Ok(())
  }

  #[inline]
  fn get(&self, index: usize) -> Result<Tagged<Object>> {
    let len = self.length();
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    let slot_addr =
      self.address() + PropertyArray::K_HEADER_SIZE + index * PropertyArray::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于已边界校验的元素槽内
    let val = unsafe { AtomicUsize::from_ptr(slot_addr as *mut usize).load(Ordering::Relaxed) };
    Ok(Tagged::new(val))
  }

  #[inline]
  fn set(&self, index: usize, value: Tagged<Object>) -> Result<()> {
    let len = self.length();
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    let slot_addr =
      self.address() + PropertyArray::K_HEADER_SIZE + index * PropertyArray::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于已边界校验的元素槽内
    unsafe {
      AtomicUsize::from_ptr(slot_addr as *mut usize).store(value.ptr(), Ordering::Relaxed);
    }
    Ok(())
  }

  #[inline]
  fn swap(&self, index: usize, value: Tagged<Object>) -> Result<Tagged<Object>> {
    let len = self.length();
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    let slot_addr =
      self.address() + PropertyArray::K_HEADER_SIZE + index * PropertyArray::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于已边界校验的元素槽内
    let old =
      unsafe { AtomicUsize::from_ptr(slot_addr as *mut usize).swap(value.ptr(), Ordering::SeqCst) };
    Ok(Tagged::new(old))
  }

  #[inline]
  fn swap_elements(&self, index1: usize, index2: usize) -> Result<()> {
    if index1 == index2 {
      return Ok(());
    }
    let val1 = self.get(index1)?;
    let val2 = self.swap(index2, val1)?;
    self.set(index1, val2)?;
    Ok(())
  }

  #[inline]
  fn compare_and_swap(
    &self,
    index: usize,
    expected: Tagged<Object>,
    value: Tagged<Object>,
  ) -> Result<Tagged<Object>> {
    let len = self.length();
    if index >= len {
      return Err(Error::OutOfBounds { index, length: len });
    }
    let slot_addr =
      self.address() + PropertyArray::K_HEADER_SIZE + index * PropertyArray::K_SLOT_SIZE;
    // SAFETY: slot_addr 位于已边界校验的元素槽内
    let old = unsafe {
      AtomicUsize::from_ptr(slot_addr as *mut usize)
        .compare_exchange(
          expected.ptr(),
          value.ptr(),
          Ordering::SeqCst,
          Ordering::SeqCst,
        )
        .unwrap_or_else(|x| x)
    };
    Ok(Tagged::new(old))
  }

  fn copy_elements(
    &self,
    dst_index: usize,
    src: Tagged<PropertyArray>,
    src_index: usize,
    len: usize,
  ) -> Result<()> {
    if len == 0 {
      return Ok(());
    }
    if dst_index + len > self.length() {
      return Err(Error::OutOfBounds {
        index: dst_index + len,
        length: self.length(),
      });
    }
    if src_index + len > src.length() {
      return Err(Error::OutOfBounds {
        index: src_index + len,
        length: src.length(),
      });
    }
    let dst_ptr = (self.address()
      + PropertyArray::K_HEADER_SIZE
      + dst_index * PropertyArray::K_SLOT_SIZE) as *mut usize;
    let src_ptr = (src.address()
      + PropertyArray::K_HEADER_SIZE
      + src_index * PropertyArray::K_SLOT_SIZE) as *const usize;
    // SAFETY: dst 与 src 的内存范围均已校验无越界，且槽位按 usize 对齐
    unsafe {
      ptr::copy(src_ptr, dst_ptr, len);
    }
    Ok(())
  }
}

/// 拥有独立内存块的 PropertyArray 容器
pub struct PropertyArrayBuf {
  _storage: Vec<u64>,
  tagged: Tagged<PropertyArray>,
}

impl PropertyArrayBuf {
  pub fn new(map: Tagged<Map>, length: usize) -> Result<Self> {
    if length > K_MAX_LENGTH {
      return Err(Error::InvalidCapacity(length));
    }
    let bytes = PropertyArray::size_for(length);
    let words = bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;
    let header = PropertyArray::new(map, length as u32);
    // SAFETY: base 指向已分配的 storage 向量首地址，满足 u64 对齐与尺寸需求
    unsafe {
      ptr::write(base as *mut PropertyArray, header);
    }
    Ok(Self {
      _storage: storage,
      // SAFETY: base 指向有效的 PropertyArray 内存
      tagged: unsafe { Tagged::from_raw_address(base) },
    })
  }

  #[inline]
  pub fn as_tagged(&self) -> Tagged<PropertyArray> {
    self.tagged
  }
}

impl Deref for PropertyArrayBuf {
  type Target = Tagged<PropertyArray>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}
