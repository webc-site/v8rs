// 由 tool/src/codegen/label.h.js 从 v8/src/codegen/label.h 生成; 修订就地改, 复跑覆盖
// Label 距离枚举

/// 跳转距离 (原 enum Distance): near = 8 位有符号位移, far = 32 位
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Distance {
  /// kNear: 8 位有符号位移
  Near = 0,
  /// kFar: 32 位有符号位移
  Far = 1,
}

/// 原 class Label — 一次性绑定标号 (bind 仅一次)
/// 平台分支登记:
/// - arm64: Assembler 持 Label 指针回填远目标, 禁移动/拷贝
/// - 其他目标 (含 x64): DEBUG 态提供移动构造 (旧 Label 清零防 DCHECK)
pub struct Label;

// —— 提取不变式断言 ——
const _: () = {
  assert!((Distance::Near as u8) < (Distance::Far as u8));
};
