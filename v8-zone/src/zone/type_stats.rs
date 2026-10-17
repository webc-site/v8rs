//! 原版 src/zone/type-stats.h/.cc 转写 (V8_ENABLE_PRECISE_ZONE_STATS)
//!
//! 逐 TypeTag 的分配/归还分类账。原版以 std::type_index 为键,
//! Rust 侧以 std::any::type_name 静态串替身 (零分配, 唯一性由
//! 调用点的具体类型决定); 原版 void/T[] 不完整类型记 sizeof(char)=1
//! 的口径由调用方以 instance_size 传入。

use std::{
  any::type_name,
  collections::BTreeMap,
  io::{self, Write},
};

/// 原版 TypeStats::StatsEntry
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StatsEntry {
  pub allocation_count: usize,
  pub allocated_bytes: usize,
  pub deallocated_bytes: usize,
  pub instance_size: usize,
}

/// 原版 TypeStats
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TypeStats {
  /// BTreeMap: dump 输出按类型名确定序 (原版 unordered_map 迭代序不定;
  /// 确定序为本侧偏差, 便于测试断言与差分)
  map: BTreeMap<&'static str, StatsEntry>,
}

impl TypeStats {
  /// 原版 AddAllocated<TypeTag>(bytes);
  /// instance_size 对应 sizeof(TypeTagForSizeof) (不完整类型取 1)
  pub(crate) fn add_allocated<TypeTag: ?Sized>(&mut self, bytes: usize, instance_size: usize) {
    let entry = self.map.entry(type_name::<TypeTag>()).or_default();
    entry.allocation_count += 1;
    entry.allocated_bytes += bytes;
    entry.instance_size = instance_size;
  }

  /// 原版 AddDeallocated<TypeTag>(bytes)
  pub(crate) fn add_deallocated<TypeTag: ?Sized>(&mut self, bytes: usize) {
    let entry = self.map.entry(type_name::<TypeTag>()).or_default();
    entry.deallocated_bytes += bytes;
  }

  /// 原版 MergeWith(const TypeStats&): 逐条目累并
  pub fn merge_with(&mut self, other: &TypeStats) {
    for (&name, other_entry) in &other.map {
      let entry = self.map.entry(name).or_default();
      entry.allocation_count += other_entry.allocation_count;
      entry.allocated_bytes += other_entry.allocated_bytes;
      entry.deallocated_bytes += other_entry.deallocated_bytes;
      entry.instance_size = other_entry.instance_size;
    }
  }

  /// 按类型名取条目拷贝 (原版无此接口; 测试/诊断辅助)
  pub fn entry(&self, name: &str) -> Option<StatsEntry> {
    self.map.get(name).copied()
  }

  /// 原版 Dump(): 表格直写 stdout (PrintF 口径), 写失败静默
  pub fn dump(&self) {
    let mut out = io::stdout();
    let _ = writeln!(out, "===== TypeStats =====");
    let _ = writeln!(
      out,
      "-------------+--------------+------------+--------+--------------"
    );
    let _ = writeln!(
      out,
      "       alloc |      dealloc |      count | sizeof | name"
    );
    let _ = writeln!(
      out,
      "-------------+--------------+------------+--------+--------------"
    );
    let (mut total_count, mut total_alloc, mut total_dealloc) = (0u64, 0u64, 0u64);
    for (name, entry) in &self.map {
      total_count += entry.allocation_count as u64;
      total_alloc += entry.allocated_bytes as u64;
      total_dealloc += entry.deallocated_bytes as u64;
      let _ = writeln!(
        out,
        "{:>12} | {:>12} | {:>10} | {:>6} | {}",
        entry.allocated_bytes,
        entry.deallocated_bytes,
        entry.allocation_count,
        entry.instance_size,
        name
      );
    }
    let _ = writeln!(
      out,
      "{total_alloc:>12} | {total_dealloc:>12} | {total_count:>10} | ===== TOTAL STATS ====="
    );
  }
}
