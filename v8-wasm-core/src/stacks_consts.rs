// 由 tool/src/wasm/stacks.h.js 从 v8/src/wasm/stacks.h 生成; 修订就地改, 复跑覆盖
// Wasm 栈内存状态机与栈基安全偏移

/// 栈生命周期状态 (对应原版 StackState : int32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum StackState {
  /// Active — The (unique) active stack. The jump buffer is invalid in that
  Active = 0,
  /// Suspended — state. A stack suspended by WasmSuspend.
  Suspended = 1,
  /// Inactive — A parent/ancestor of the active stack. In other words, a
  Inactive = 2,
  /// Retired — stack that either called or resumed a suspendable stack. A finished stack. The jump buffer is invalid in that state.
  Retired = 3,
}

/// 栈基安全偏移 (USE_SIMULATOR 分支: 20 * kSystemPointerSize;
/// 模拟器为运行时调用预留映射区, 防栈起点附近未映射崩溃)
pub const STACK_BASE_SAFETY_OFFSET_SIMULATOR: i32 = 160;

/// 栈基安全偏移 (非模拟器: 0)
pub const STACK_BASE_SAFETY_OFFSET: i32 = 0;

/// 指针宽 (x64/arm64, 原版 kSystemPointerSize)
pub const SYSTEM_POINTER_SIZE: i32 = 8;

// —— 提取不变式断言 ——
const _: () = {
  assert!(StackState::Retired as i32 == 3);
  assert!(STACK_BASE_SAFETY_OFFSET_SIMULATOR == 160);
  assert!(STACK_BASE_SAFETY_OFFSET == 0);
};
