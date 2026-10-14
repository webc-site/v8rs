use std::{mem, ptr};

use v8_objects_core::{instance_type::InstanceType, map::Map, tagged::Tagged};

/// 8 字节对齐 bump 竞技场 (对象永不回收, 测试期存活)。
pub struct Arena {
  buf: Vec<u64>,
}

const ARENA_CAPACITY_WORDS: usize = 1 << 17;

impl Default for Arena {
  fn default() -> Self {
    Self::new()
  }
}

impl Arena {
  pub fn new() -> Self {
    Self {
      buf: Vec::with_capacity(ARENA_CAPACITY_WORDS),
    }
  }

  /// 分配 size 字节 (8 对齐, 零填充), 返回绝对基址。
  fn alloc(&mut self, size: usize) -> usize {
    let words = size.div_ceil(8);
    assert!(
      self.buf.len() + words <= ARENA_CAPACITY_WORDS,
      "竞技场容量不足"
    );
    let base = self.buf.as_ptr() as usize + self.buf.len() * 8;
    self.buf.resize(self.buf.len() + words, 0);
    base
  }

  /// 分配一个指定实例类型的 Map。
  pub fn map(&mut self, ty: InstanceType) -> Tagged<Map> {
    let base = self.alloc(mem::size_of::<Map>());
    let map = Map::new(Tagged::NULL, ty, 0);
    // SAFETY: base 8 对齐且已保留 Map 大小; move 写入
    unsafe { ptr::write(base as *mut Map, map) };
    // SAFETY: base 对齐 (最低 2 位为 0)
    unsafe { Tagged::from_raw_address(base) }
  }
}
