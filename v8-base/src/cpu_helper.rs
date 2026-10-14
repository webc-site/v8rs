// 由 tool/src/base/cpu/cpu-helper.h.js 从 v8/src/base/cpu/cpu-helper.h 生成; 修订就地改, 复跑覆盖
// /proc/cpuinfo 采集器骨架 (ARM64+Linux 分支; darwin 不可达)

/// /proc/cpuinfo 采集器 (原版 base::CPUInfo, ARM64+Linux 构建分支)
/// 其他架构分支 (MIPS64/RISCV64/LOONG64) 与非 Linux 系统登记跳过
pub struct CpuInfo;

impl CpuInfo {
  /// 提取 cpuinfo 中首个指定字段, 堆串返回; 未找到返回 None
  /// 原版返回 malloc 串 (delete[] 释放), Rust 侧由 Developer 定所有权
  pub fn extract_field(&self, field: &str) -> Option<String>;
}

/// 空格分隔列表包含判定 (原版 HasListItem)
pub fn has_list_item(list: &str, item: &str) -> bool {
  list.split(' ').any(|x| x == item)
}

// —— 提取不变式断言 ——
const _: () = {
  // HasListItem 语义锚点
  assert!(has_list_item("a b c", "b"));
  assert!(!has_list_item("a b c", "d"));
};
