// 由 tool/src/wasm/baseline/liftoff-varstate.h.js 从 v8/src/wasm/baseline/liftoff-varstate.h 生成; 修订就地改, 复跑覆盖
// Liftoff 值栈槽位状态的位置类别

/// 值的位置 (对应原版 LiftoffVarState::Location : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VarStateLocation {
  /// kStack — 已溢出到栈槽
  Stack = 0,
  /// kRegister — 驻留寄存器
  Register = 1,
  /// kIntConst — 常量折叠
  IntConst = 2,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(VarStateLocation::IntConst as u8 == 2);
};
