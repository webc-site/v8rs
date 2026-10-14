//! 内存分配抽象特征与基础实现

use crate::error::Result;

/// 内存分配抽象特征，支持 Vec<u64> 或外部内存池
pub trait MemoryAllocator {
  fn allocate(&mut self, size: usize) -> Result<usize>;
}

/// 基于独立堆块的稳定地址内存分配器，保证已分配对象的绝对指针永不失效
#[derive(Default, Debug)]
pub struct HeapStorage {
  blocks: Vec<Box<[u64]>>,
}

impl HeapStorage {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }
}

impl MemoryAllocator for HeapStorage {
  #[inline]
  fn allocate(&mut self, size: usize) -> Result<usize> {
    let words = size.div_ceil(8);
    let mut block = vec![0u64; words].into_boxed_slice();
    let ptr = block.as_mut_ptr() as usize;
    self.blocks.push(block);
    Ok(ptr)
  }
}

impl MemoryAllocator for Vec<u64> {
  #[inline]
  fn allocate(&mut self, size: usize) -> Result<usize> {
    let words = size.div_ceil(8);
    let old_len = self.len();
    if self.capacity() < old_len + words {
      self.reserve((old_len + words).max(65536));
    }
    self.resize(old_len + words, 0);
    let base = self.as_ptr() as usize + old_len * 8;
    Ok(base)
  }
}

impl<F: FnMut(usize) -> Result<usize>> MemoryAllocator for F {
  #[inline]
  fn allocate(&mut self, size: usize) -> Result<usize> {
    self(size)
  }
}
