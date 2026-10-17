// 由 tool/src/diagnostics/arm64/unwinder-arm64.cc.js 从 v8/src/diagnostics/arm64/unwinder-arm64.cc 生成; 修订就地改, 复跑覆盖
// arm64 unwinder 桩 (EntryFrame 被调方保存寄存器)

/// 原版 GetCalleeSavedRegistersFromEntryFrame(fp, register_state):
/// arm64 平台为空实现桩。
/// RegisterState 定义见 v8-profiler 侧 (前置声明)。
pub fn get_callee_saved_registers_from_entry_frame(
  _fp: *mut (),
  _register_state: *mut (),
) {
  // arm64: 空实现 (原版语义)
}
