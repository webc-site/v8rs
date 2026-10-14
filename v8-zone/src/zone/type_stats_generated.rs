// 由 tool/src/zone/type-stats.h.js 从 v8/src/zone/type-stats.{h,cc} 生成; 修订就地改, 复跑覆盖
// Zone 类型统计骨架

/// 精确 zone 统计分支 (原版 #ifdef V8_ENABLE_PRECISE_ZONE_STATS; 默认关闭 → 空实现)
pub mod type_stats_branch {
  /// 当前目标: x64/arm64 常规构建关闭精确统计
  pub const PRECISE_ZONE_STATS_ENABLED: bool = false;
  /// 接口面 (对数)
  pub const ADD_METHODS: [&str; 2] = ["AddAllocated", "AddDeallocated"];
}

/// 统计条目 (原版 StatsEntry)
#[derive(Debug, Clone, Copy, Default)]
pub struct StatsEntry {
  /// 分配次数 (原版 allocation_count)
  pub allocation_count: u64,
  /// 累计分配字节 (原版 allocated_bytes)
  pub allocated_bytes: u64,
  /// 累计释放字节 (原版 deallocated_bytes)
  pub deallocated_bytes: u64,
  /// 实例宽 (不完整类型/void 记 sizeof(char)=1, 原版 kIsIncomplete 分支)
  pub instance_size: usize,
}

/// TypeStats 签名骨架 (语义见手写 zone/type_stats.rs)
pub mod type_stats_skel {
  use std::any::TypeId;

  use super::StatsEntry;

  /// 按类型标签累加分配 (原版 AddAllocated<TypeTag>)
  pub fn add_allocated(_bytes: usize) {
    unimplemented!("骨架")
  }
  /// 按类型标签累加释放 (原版 AddDeallocated<TypeTag>)
  pub fn add_deallocated(_bytes: usize) {
    unimplemented!("骨架")
  }
  /// 汇总打印 (原版 Print)
  pub fn print() -> String {
    unimplemented!("骨架")
  }
  /// 存储协议: type_index → StatsEntry 无序表 (原版 map_)
  pub fn entry_table() -> Vec<(TypeId, StatsEntry)> {
    unimplemented!("骨架")
  }
}
