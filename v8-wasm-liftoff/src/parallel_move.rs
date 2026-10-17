// 由 tool/src/wasm/baseline/parallel-move.h.js 从 v8/src/wasm/baseline/parallel-move.h 生成; 修订就地改, 复跑覆盖
// 并行寄存器搬移的装载类别

/// 寄存器装载类别 (对应原版 RegisterLoad::LoadKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LoadKind {
  /// kNop — no-op, used for high fp of a fp pair.
  Nop = 0,
  /// kConstant — load a constant value into a register.
  Constant = 1,
  /// kStack — fill a register from a stack slot.
  Stack = 2,
  /// kLowHalfStack — fill a register from the low half of a stack slot.
  LowHalfStack = 3,
  /// kHighHalfStack — fill a register from the high half of a stack slot.
  HighHalfStack = 4,
}

/// 寄存器搬移请求 (对应原版 RegisterMove; 语义面待 Developer 修订)
pub struct RegisterMove;

// —— 提取不变式断言 ——
const _: () = {
  assert!(LoadKind::HighHalfStack as u8 == 4);
};
