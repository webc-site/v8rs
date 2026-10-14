// 由 tool/src/wasm/string-builder.h.js 从 v8/src/wasm/string-builder.h + string-builder-multiline.h 生成; 修订就地改, 复跑覆盖
// Wasm StringBuilder 容量常量

/// 缓冲增长策略 (对应原版 StringBuilder::OnGrowth, enum : bool)
pub type OnGrowth = bool;

/// kKeepOldChunks
pub const KEEP_OLD_CHUNKS: OnGrowth = false;
/// kReplacePreviousChunk
pub const REPLACE_PREVIOUS_CHUNK: OnGrowth = true;

/// 栈上内联缓冲字节数 (原版 StringBuilder::kStackSize)
pub const STACK_SIZE: usize = 256;

/// 堆块字节数 (原版 StringBuilder::kChunkSize)
pub const CHUNK_SIZE: usize = 1048576;

/// uint32 十进制缓冲宽 (原版 kBufferSize; 恰容一个 uint32)
pub const BUFFER_SIZE: usize = 10;

/// 多行分隔写入的 uint32 缓冲宽 (原版 string-builder-multiline.h kBufSize; 容 uint32 + '|')
pub const MULTILINE_BUF_SIZE: usize = 12;

// —— 提取不变式断言 ——
const _: () = {
  assert!(STACK_SIZE == 256);
  assert!(CHUNK_SIZE == 1024 * 1024);
  assert!(BUFFER_SIZE == 10);
  assert!(MULTILINE_BUF_SIZE == 12);
};
