// 由 tool/src/wasm/wasm-debug.h.js 从 v8/src/wasm/wasm-debug.h 生成; 修订就地改, 复跑覆盖
// 调试侧表条目值的存储类别

/// DebugSideTable 条目值的存放类别 (对应原版 DebugSideTable::Entry::Storage : int8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum DebugSideTableStorage {
  /// kConstant — 常量值
  Constant = 0,
  /// kRegister — 寄存器 (reg_code)
  Register = 1,
  /// kStack — 栈槽 (stack_offset)
  Stack = 2,
}

impl DebugSideTableStorage {
  /// 总数
  pub const COUNT: usize = 3;
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(DebugSideTableStorage::Stack as i8 == 2);
};
