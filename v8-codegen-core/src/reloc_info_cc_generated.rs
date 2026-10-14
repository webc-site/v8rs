// 由 tool/src/codegen/reloc-info.cc.js 从 v8/src/codegen/reloc-info.{cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// 重定位信息 inl 常量与骨架

// reloc-info.h 枚举/表由 reloc_info.rs (手写) 覆盖; 此处登记 inl 分支常量。

/// 目标地址寻址模式掩码 — HasTargetAddressAddress 内双分支 (x64/ia32 vs 其他):
/// - 分支 0 (压缩指针/默认布局): ModeMask(CODE_TARGET) | ModeMask(FULL_EMBEDDED_OBJECT) | ModeMask(COMPRESSED_EMBEDDED_OBJECT) | ModeMask(EXTERNAL_REFERENCE) | ModeMask(OFF_HEAP_TARGET) | ModeMask(WASM_CALL) | ModeMask(WASM_STUB_CALL)
/// - 分支 1: ModeMask(CODE_TARGET) | ModeMask(RELATIVE_CODE_TARGET) | ModeMask(FULL_EMBEDDED_OBJECT) | ModeMask(EXTERNAL_REFERENCE) | ModeMask(OFF_HEAP_TARGET) | ModeMask(WASM_CALL)
pub mod target_address_address_mode {
  /// 分支 0 原文: ModeMask(CODE_TARGET) | ModeMask(FULL_EMBEDDED_OBJECT) | ModeMask(COMPRESSED_EMBEDDED_OBJECT) | ModeMask(EXTERNAL_REFERENCE) | ModeMask(OFF_HEAP_TARGET) | ModeMask(WASM_CALL) | ModeMask(WASM_STUB_CALL)
  pub const BRANCH_0_EXPR: &str = "ModeMask(CODE_TARGET) | ModeMask(FULL_EMBEDDED_OBJECT) | ModeMask(COMPRESSED_EMBEDDED_OBJECT) | ModeMask(EXTERNAL_REFERENCE) | ModeMask(OFF_HEAP_TARGET) | ModeMask(WASM_CALL) | ModeMask(WASM_STUB_CALL)";
  /// 分支 1 原文: ModeMask(CODE_TARGET) | ModeMask(RELATIVE_CODE_TARGET) | ModeMask(FULL_EMBEDDED_OBJECT) | ModeMask(EXTERNAL_REFERENCE) | ModeMask(OFF_HEAP_TARGET) | ModeMask(WASM_CALL)
  pub const BRANCH_1_EXPR: &str = "ModeMask(CODE_TARGET) | ModeMask(RELATIVE_CODE_TARGET) | ModeMask(FULL_EMBEDDED_OBJECT) | ModeMask(EXTERNAL_REFERENCE) | ModeMask(OFF_HEAP_TARGET) | ModeMask(WASM_CALL)";
}

/// 原 RelocInfo::HasTargetAddressAddress 判定使用的模式掩码 (枚举名引用,
/// 数值由 reloc_info.rs 手写产物中的 ModeMask 对应枚举决定)。
/// 语义面: RelocInfoWriter/RelocIterator 编解码实现登记跳过。
pub struct RelocInfoWriter;

// —— 提取不变式断言 ——
const _: () = {};
