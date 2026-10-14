// 由 tool/src/diagnostics/x64/unwinder-x64.cc.js 从 v8/src/diagnostics/x64/unwinder-x64.cc 生成; 修订就地改, 复跑覆盖
// x64 unwinder 桩 (EntryFrame 被调方保存寄存器)

/// 原版 GetCalleeSavedRegistersFromEntryFrame(fp, register_state):
/// x64 平台为空实现桩 (寄存器状态由采样器側其余路径填充)。
/// RegisterState 定义见 v8-profiler 侧 (前置声明)。
pub fn get_callee_saved_registers_from_entry_frame(
  _fp: *mut (),
  _register_state: *mut (),
) {
  // x64: 空实现 (原版语义)
}
