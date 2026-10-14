// 由 tool/src/codegen/arm64/decoder-arm64-inl.h.js 从 v8/src/codegen/arm64/decoder-arm64-inl.h 生成; 修订就地改, 复跑覆盖
// arm64 解码器分发骨架

// 指令位型树为模式匹配语义面, 登记跳过。
// 顶层分发 case (Decode<27,24>): 16 处 — 0x0, 0x1, 0xA, 0xB, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xC, 0xD, 0xE, 0xF
// 子解码器注册表 (11 条):
// - DecodePCRelAddressing
// - DecodeDataProcessingImmediate
// - DecodeDataProcessing
// - DecodeLogical
// - DecodeBitfieldExtract
// - DecodeBranchSystemException
// - DecodeLoadStore
// - DecodeFP
// - DecodeNEONLoadStore
// - DecodeNEONVectorDataProcessing
// - DecodeNEONScalarDataProcessing

pub struct DecoderInl;

// TODO(dev): 依手写 decoder.rs 访客框架补位型树。

// —— 提取不变式断言 ——
const _: () = {};
