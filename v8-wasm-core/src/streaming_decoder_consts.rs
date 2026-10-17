// 由 tool/src/wasm/streaming-decoder.h.js 从 v8/src/wasm/streaming-decoder.{h,cc} 生成; 修订就地改, 复跑覆盖
// 流式解码状态机与模块头布局

/// 流式解码器状态 (对应原版 AsyncStreamingDecoder::StreamState)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StreamState {
  /// kReceivingBytes — 正在接收字节
  ReceivingBytes = 0,
  /// kAborted — 已中止
  Aborted = 1,
  /// kFinished — 已完成
  Finished = 2,
  /// kDiscarded — 已丢弃
  Discarded = 3,
}

/// 模块头大小 (原版 streaming-decoder.cc kModuleHeaderSize; magic + version)
pub const MODULE_HEADER_SIZE: usize = 8;

// —— 提取不变式断言 ——
const _: () = {
  assert!(MODULE_HEADER_SIZE == 8);
  assert!(StreamState::Discarded as u8 == 3);
};
