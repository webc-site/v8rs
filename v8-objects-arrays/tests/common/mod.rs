//! 测试公共件: 数组对象竞技场 (同 v8-objects-strings tests/common 先例)。
//!
//! 布局约定 (与 src/fixed_array*.rs 一致): 对象内存 = Rust 结构体逻辑布局
//! (Tagged 槽 usize 宽), 元素尾在 `基址 + OBJECTS_OFFSET`; 竞技场按 8 字节
//! 对齐分配。
//!

use std::{mem, ptr};

use v8_objects_core::{instance_type::InstanceType, map::Map, tagged::Tagged};

/// 8 字节对齐 bump 竞技场 (对象永不回收, 测试期存活)。
///
/// 预留大容量保证 Vec 不重分配 — Tagged 指针指向 buf 内部, 搬迁即悬垂。
pub struct Arena {
  buf: Vec<u64>,
}

/// 预留容量 (字): 128K 字 = 1MB, 超出即断 (测试对象远小于此)。
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

  /// 分配 [头结构体][tail_bytes 尾区] 的对象, 返回 tagged 指针。
  pub fn alloc_with<H>(&mut self, header: H, tail_bytes: usize) -> Tagged<H> {
    let base = self.alloc(mem::size_of::<H>() + tail_bytes);
    // SAFETY: base 8 对齐且已保留头大小; move 写入
    unsafe { ptr::write(base as *mut H, header) };
    // SAFETY: base 对齐
    unsafe { Tagged::from_raw_address(base) }
  }
}
