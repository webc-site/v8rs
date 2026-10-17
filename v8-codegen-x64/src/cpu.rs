// 由 tool/src/codegen/x64/cpu-x64.cc.js 从 v8/src/codegen/x64/cpu-x64.cc 生成; 修订就地改, 复跑覆盖
// x64 平台 CpuFeatures 实现 (FlushICache 空操作)

/// x64 平台 FlushICache (原版 CpuFeatures::FlushICache):
/// Intel 无需显式刷新 — 单线程代码补丁场景下核心指令缓存自动一致;
/// 若确需刷新由 Windows FlushInstructionCache API 承担。
/// VALGRIND_DISCARD_TRANSLATIONS 分支: 非 valgrind 构建, 登记跳过。
#[inline]
pub fn flush_i_cache(_start: *mut u8, _size: usize) {
  // x64: 空操作 (原版语义)
}
