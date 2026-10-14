// 由 tool/src/wasm/wasm-arguments.h.js 从 v8/src/wasm/wasm-arguments.h 生成; 修订就地改, 复跑覆盖
// CWasmEntry 栈参数打包器常量

/// 指针宽 (x64/arm64 目标, 原版 kSystemPointerSize)
pub const SYSTEM_POINTER_SIZE: usize = 8;

/// 栈上缓冲最大字节数 (原版 CWasmArgumentsPacker::kMaxOnStackBuffer =
/// 10 * kSystemPointerSize); 超过则转堆缓冲
pub const MAX_ON_STACK_BUFFER: usize = 80;

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_ON_STACK_BUFFER == 80);
  assert!(SYSTEM_POINTER_SIZE == 8);
};
