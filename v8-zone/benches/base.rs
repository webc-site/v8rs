//! v8-zone 核心评测基座: 定义本 crate 核心操作 trait, v8rs 实现与各对比库实现分别 impl
//!
//! 评测 id 基于 `BenchOps::NAME` 生成, 驱动入口见 benches/main.rs。
//! 核心操作对应原版 Zone 的主要用法: bump 批量分配 (小对象高频/大块)、
//! 对齐分配、ZoneScope 作用域嵌套、DeleteAll/Reset 多段复位回收。

use std::hint::black_box;

#[cfg(feature = "bench-bumpalo")]
use bumpalo::{Bump, collections::Vec as BumpVec};
use v8_zone::{AccountingAllocator, Zone, ZoneScope};

/// 批量分配的尺寸扫描 (字节): 小热点对象到大块 (16KB 逼近段上限, 触发多段扩容)
pub const SIZES: [usize; 5] = [16, 64, 256, 1024, 16 * 1024];
/// 每轮批量分配的对象个数 (alloc 操作)
pub const ALLOC_N: usize = 200;
/// 对齐分配的对象基准大小 (字节)
pub const ALIGNED_SIZE: usize = 32;
/// 对齐扫描 (字节)
pub const ALIGNS: [usize; 2] = [8, 32];
/// 每轮对齐分配的对象个数
pub const ALIGNED_N: usize = 100;
/// 作用域嵌套深度扫描
pub const DEPTHS: [usize; 3] = [1, 4, 16];
/// 每个作用域内分配的对象个数
pub const SCOPE_N: usize = 32;
/// 作用域内对象大小 (字节)
pub const SCOPE_SIZE: usize = 32;
/// 复位回收的对象个数 (1600x64B ≈ 100KB, 必然跨越多个段)
pub const RESET_N: usize = 1600;
/// 复位回收的对象大小 (字节)
pub const RESET_SIZE: usize = 64;

/// 核心评测操作: 每个实现以自身原生方式完成同样的语义
pub trait BenchOps {
  /// 评测 id 基准名
  const NAME: &'static str;

  fn new() -> Self;

  /// 批量分配 count 个 size 字节对象后整体回收 (稳定态吞吐; 覆盖小对象高频与大块)
  fn alloc(&mut self, count: usize, size: usize) -> usize;

  /// 对齐分配: 取 size+align 字节块, 把起始地址对齐到 align 字节, 共 count 个后整体回收
  fn aligned(&mut self, count: usize, size: usize, align: usize) -> usize;

  /// 作用域嵌套: depth 层, 每层分配 count 个 size 字节对象, 离开该层即回收该层内存
  fn scoped(&mut self, depth: usize, count: usize, size: usize) -> usize;

  /// 多段复位回收: 分配 count 个大跨段对象后一次性 Reset/DeleteAll
  fn reset(&mut self, count: usize, size: usize) -> usize;
}

/// v8rs 实现: Zone bump 分配 + Reset 复位 + ZoneScope 快照回滚
pub struct ZoneBench {
  zone: Zone<'static>,
}

impl BenchOps for ZoneBench {
  const NAME: &'static str = "v8rs-zone";

  fn new() -> Self {
    // 分配器生命周期提升为 'static, 供 Zone 长期借用 (评测专用)
    let allocator: &'static AccountingAllocator = Box::leak(Box::new(AccountingAllocator::new()));
    ZoneBench {
      zone: Zone::new(allocator, Self::NAME),
    }
  }

  fn alloc(&mut self, count: usize, size: usize) -> usize {
    let mut sum = 0;
    for _ in 0..count {
      if let Ok(buf) = self.zone.allocate_bytes_ref(size) {
        sum += buf.len();
        black_box(buf);
      }
    }
    self.zone.reset();
    sum
  }

  fn aligned(&mut self, count: usize, size: usize, align: usize) -> usize {
    let mut sum = 0;
    for _ in 0..count {
      // Zone 原生仅保证 8 字节对齐, 更大对齐按通用做法过量分配后就地取整
      if let Ok(buf) = self.zone.allocate_bytes_ref(size + align) {
        let base = buf.as_mut_ptr() as usize;
        let ptr = (base + align - 1) & !(align - 1);
        black_box(ptr as *mut u8);
        sum += ptr - base + size;
      }
    }
    self.zone.reset();
    sum
  }

  fn scoped(&mut self, depth: usize, count: usize, size: usize) -> usize {
    let mut sum = 0;
    enter(&mut self.zone, depth, count, size, &mut sum);
    sum
  }

  fn reset(&mut self, count: usize, size: usize) -> usize {
    self.alloc(count, size)
  }
}

/// 逐层进入 ZoneScope: 离开时快照回滚, 归还本层新增段并复位居中
fn enter<'a>(zone: &mut Zone<'a>, depth: usize, count: usize, size: usize, sum: &mut usize) {
  if depth == 0 {
    return;
  }
  let mut scope = ZoneScope::new(zone);
  for _ in 0..count {
    if let Ok(buf) = scope.zone().allocate_bytes_ref(size) {
      *sum += buf.len();
      black_box(buf);
    }
  }
  enter(scope.zone(), depth - 1, count, size, sum);
}

/// bumpalo 对照: Zone 语义 API 保持自研, 此处仅作 bump 内核性能对照
#[cfg(feature = "bench-bumpalo")]
pub struct BumpBench {
  bump: Bump,
}

#[cfg(feature = "bench-bumpalo")]
impl BenchOps for BumpBench {
  const NAME: &'static str = "bumpalo";

  fn new() -> Self {
    BumpBench { bump: Bump::new() }
  }

  fn alloc(&mut self, count: usize, size: usize) -> usize {
    let mut sum = 0;
    for _ in 0..count {
      // with_capacity_in 分配未初始化字节, 与 Zone 口径一致
      let buf = BumpVec::<u8>::with_capacity_in(size, &self.bump);
      sum += black_box(buf.capacity());
    }
    self.bump.reset();
    sum
  }

  fn aligned(&mut self, count: usize, size: usize, align: usize) -> usize {
    let mut sum = 0;
    for _ in 0..count {
      let buf = BumpVec::<u8>::with_capacity_in(size + align, &self.bump);
      let base = buf.as_ptr() as usize;
      let ptr = (base + align - 1) & !(align - 1);
      black_box(ptr as *mut u8);
      sum += ptr - base + size;
    }
    self.bump.reset();
    sum
  }

  /// bumpalo 无逐层回滚原语: 各层对象照常分配, 整体结束后一次性 reset
  fn scoped(&mut self, depth: usize, count: usize, size: usize) -> usize {
    let mut sum = 0;
    for _ in 0..depth {
      for _ in 0..count {
        let buf = BumpVec::<u8>::with_capacity_in(size, &self.bump);
        sum += black_box(buf.capacity());
      }
    }
    self.bump.reset();
    sum
  }

  fn reset(&mut self, count: usize, size: usize) -> usize {
    self.alloc(count, size)
  }
}

/// std 对照: Vec::with_capacity 批量分配 (经全局 jemalloc), 通用分配器基线
#[cfg(feature = "bench-std")]
pub struct StdBench {
  keep: Vec<Vec<u8>>,
}

#[cfg(feature = "bench-std")]
impl BenchOps for StdBench {
  const NAME: &'static str = "std-vec";

  fn new() -> Self {
    StdBench { keep: Vec::new() }
  }

  fn alloc(&mut self, count: usize, size: usize) -> usize {
    let mut sum = 0;
    for _ in 0..count {
      let buf = Vec::<u8>::with_capacity(size);
      sum += black_box(buf.capacity());
      self.keep.push(buf);
    }
    // 批量归还不逐个 free, 与 arena 整体回收口径对齐
    self.keep.clear();
    sum
  }

  fn aligned(&mut self, count: usize, size: usize, align: usize) -> usize {
    let mut sum = 0;
    for _ in 0..count {
      let buf = Vec::<u8>::with_capacity(size + align);
      let base = buf.as_ptr() as usize;
      let ptr = (base + align - 1) & !(align - 1);
      black_box(ptr as *mut u8);
      sum += ptr - base + size;
      self.keep.push(buf);
    }
    self.keep.clear();
    sum
  }

  /// 逐层退出时释放本层对象, 还原嵌套作用域的 LIFO 释放序列
  fn scoped(&mut self, depth: usize, count: usize, size: usize) -> usize {
    let mut sum = 0;
    let mark = self.keep.len();
    if depth == 0 {
      return sum;
    }
    for _ in 0..count {
      let buf = Vec::<u8>::with_capacity(size);
      sum += black_box(buf.capacity());
      self.keep.push(buf);
    }
    let inner = self.scoped(depth - 1, count, size);
    self.keep.truncate(mark);
    sum + inner
  }

  fn reset(&mut self, count: usize, size: usize) -> usize {
    self.alloc(count, size)
  }
}
