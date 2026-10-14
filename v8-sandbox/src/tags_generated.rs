// 由 tool/src/sandbox/code-entrypoint-tag.h.js 从 v8/src/sandbox/code-entrypoint-tag.h + indirect-pointer-tag.h + cppheap-pointer-tag.h + check.h 生成; 修订就地改, 复跑覆盖
// 沙箱标签族: 入口点 CFI 标签/间接指针标签/沙箱检查

/// 入口点标签移位 (原版 kCodeEntrypointTagShift; x64 LA57=57, 其余=48; 当前目标取 48)
pub const CODE_ENTRYPOINT_TAG_SHIFT: u32 = 48;

/// 入口点 CFI 标签 (原版 enum CodeEntrypointTag : uint64_t, 1:1 展开;
/// 实现: 标签 XOR 进 CPT 入口指针高位, 调用前 XOR 还原; 不匹配则安全崩溃)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum CodeEntrypointTag {
  /// `kJSEntrypointTag`
  JS = (0)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
  /// `kWasmEntrypointTag`
  WASM = (1)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
  /// `kBytecodeHandlerEntrypointTag`
  BYTECODE_HANDLER = (2)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
  /// `kLoadWithVectorICHandlerEntrypointTag`
  LOAD_WITH_VECTOR_ICHANDLER = (3)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
  /// `kStoreWithVectorICHandlerEntrypointTag`
  STORE_WITH_VECTOR_ICHANDLER = (4)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
  /// `kStoreTransitionICHandlerEntrypointTag`
  STORE_TRANSITION_ICHANDLER = (5)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
  /// `kRegExpEntrypointTag`
  REG_EXP = (6)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
  /// `kCEntryEntrypointTag`
  CENTRY = (7)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
  /// `kCodeEntrypointTagForTesting`
  CODE_ENTRYPOINT_TAG_FOR_TESTING = (0x7c)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
  /// `kDisabledBuiltinEntrypointTag`
  DISABLED_BUILTIN = (0x7d)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
  /// `kInvalidEntrypointTag`
  INVALID = (0x7f)u64 << CODE_ENTRYPOINT_TAG_SHIFT,
}

/// 间接指针标签 (原版 enum IndirectPointerTag : uint16_t; u16, 末值 0x7f);
/// 分段: 共享可信指针 / 每 isolate 可信指针 / SFI 可信数据 / 特殊哨兵
/// (Unpublished=0x7c, Zapped=0x7d, Evacuation=0x7e, Free/Last=0x7f)
pub mod indirect_pointer_tags {
  /// 标签名序 (区间哨兵 kFirst*/kLast* 在列尾, 数值由 Developer 修订定序)
  pub const NAMES: [&str; 27] = ["kIndirectPointerNullTag", "kFirstSharedTrustedPointerTag", "kSharedWasmTrustedInstanceDataIndirectPointerTag", "kSharedWasmDispatchTableIndirectPointerTag", "kLastSharedTrustedPointerTag", "kFirstPerIsolateTrustedPointerTag", "kWasmInternalFunctionIndirectPointerTag", "kWasmTrustedInstanceDataIndirectPointerTag", "kWasmDispatchTableIndirectPointerTag", "kWasmSuspenderIndirectPointerTag", "kRegExpDataIndirectPointerTag", "kFirstSFITrustedDataTag", "kWasmExportedFunctionDataIndirectPointerTag", "kWasmCapiFunctionDataIndirectPointerTag", "kInterpreterDataIndirectPointerTag", "kUncompiledDataIndirectPointerTag", "kBytecodeArrayIndirectPointerTag", "kCodeIndirectPointerTag", "kLastSFITrustedDataTag", "kDebugInfoIndirectPointerTag", "kLastPerIsolateTrustedPointerTag", "kLastIndirectPointerFastTag", "kUnpublishedIndirectPointerTag", "kIndirectPointerZappedEntryTag", "kIndirectPointerEvacuationEntryTag", "kIndirectPointerFreeEntryTag", "kLastIndirectPointerTag"];
  /// 特殊哨兵值 (原版显式数值)
  pub const UNPUBLISHED: u16 = 0x7c;
  pub const ZAPPED_ENTRY: u16 = 0x7d;
  pub const EVACUATION_ENTRY: u16 = 0x7e;
  pub const FREE_ENTRY: u16 = 0x7f;
  pub const LAST: u16 = 0x7f;
  /// 静态断言锚点 (原版 kLastIndirectPointerTag <= 0x7f)
  pub const FITS_7_BITS: bool = true;
}

/// 沙箱检查面 (原版 check.h SBXCHECK 家族: 沙箱内不变量破坏即终止进程)
pub const SBXCHECK_FNS: [&str; 1] = ["SBXCHECK"];
