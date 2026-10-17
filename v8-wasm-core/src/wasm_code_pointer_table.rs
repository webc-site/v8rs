// 由 tool/src/wasm/wasm-code-pointer-table.h.js 从 v8/src/wasm/wasm-code-pointer-table.{h,cc} 生成; 修订就地改, 复跑覆盖
// WasmCodePointerTable 布局与哨兵常量

/// 表项是否写保护 (原版 WasmCodePointerTableEntry::IsWriteProtected)
pub const IS_WRITE_PROTECTED: bool = true;

/// 无效句柄 (原版 kInvalidHandle = -1, 即 u32 最大值)
pub const INVALID_HANDLE: u32 = u32::MAX;

/// freelist 重试标记 length (原版 kRetryMarker length)
pub const RETRY_MARKER_LENGTH: u32 = 0xFFFF_FFFF;

/// freelist 重试标记 next (原版 kRetryMarker next)
pub const RETRY_MARKER_NEXT: u32 = 0;

/// 表项布局参数 (sandbox 关: 仅 entrypoint_; sandbox 开: + signature_hash_)
pub mod entry_layout {
  /// entrypoint_ 字段宽 (std::atomic<Address>)
  pub const ENTRYPOINT_SIZE: usize = 8;

  /// sandbox 开启时表项总宽 (Address + u64)
  pub const ENTRY_SIZE_SANDBOX: usize = 16;

  /// sandbox 关闭时表项总宽
  pub const ENTRY_SIZE_NO_SANDBOX: usize = 8;

  /// sandbox 开启时 signature_hash_ 偏移 (原版 kOffsetOfSignatureHash = offsetof)
  pub const OFFSET_OF_SIGNATURE_HASH: usize = 8;
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(INVALID_HANDLE == 0xFFFF_FFFF);
  assert!(entry_layout::OFFSET_OF_SIGNATURE_HASH == entry_layout::ENTRYPOINT_SIZE);
  assert!(entry_layout::ENTRY_SIZE_SANDBOX == 16);
};
