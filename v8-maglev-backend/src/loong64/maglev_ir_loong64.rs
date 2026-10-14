// 由 tool/src/maglev/loong64/maglev-ir-loong64.cc.js 从 v8/src/maglev/loong64/maglev-ir-loong64.cc 生成; 修订就地改, 复跑覆盖
// LoongArch64 maglev-ir 跳过登记 + 预算枚举 (非目标架构)

//! 规则九: 平台分支按 x64+arm64 展开, LoongArch64 跳过语义转换。
//! 本文件语义为各 IR 节点 GenerateCode/SetValueLocationConstraints (101 锚),
//! 不展开; 仅登记文件面与预算类型枚举。

/// 原版 enum class ReduceInterruptBudgetType (中断预算计费点)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ReduceInterruptBudgetType {
  /// kLoop
  Loop = 0,
  /// kReturn
  Return = 1,
}

pub static REGISTERED_FILES: &[(&str, usize, usize)] =
  &[("src/maglev/loong64/maglev-ir-loong64.cc", 1320, 101)];

const _: () = {
  assert!(REGISTERED_FILES.len() == 1);
  assert!(REGISTERED_FILES[0].1 == 1320);
  assert!(REGISTERED_FILES[0].2 == 101);
};
