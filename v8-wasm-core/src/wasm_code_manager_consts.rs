// 由 tool/src/wasm/wasm-code-manager.h.js 从 v8/src/wasm/wasm-code-manager.h 生成; 修订就地改, 复跑覆盖
// NativeModule / WasmCodeManager 生命周期常量

/// dying 标记位掩码 (原版 WasmEngine::NativeModuleInfo? — kIsDyingMask)
pub const IS_DYING_MASK: u32 = 0x80000000;

/// 无假设 journal 哨兵 (原版 NativeModule::kNoAssumptions)
pub const NO_ASSUMPTIONS: Option<()> = None;

/// NativeModule 的托管类型 id (原版 NativeModule::kTypeID = ManagedTypeId::kWasmNativeModule)
pub const NATIVE_MODULE_TYPE_ID: &str = "kWasmNativeModule";

/// 代码空间之间是否需要远跳转 (原版 kNeedsFarJumpsBetweenCodeSpaces)
/// 两个编译配置分支均已展开:
pub const NEEDS_FAR_JUMPS_BETWEEN_CODE_SPACES_NEAR: bool = false;
pub const NEEDS_FAR_JUMPS_BETWEEN_CODE_SPACES_FAR: bool = true;

/// 无效 wasm 代码指针 (原版 kInvalidWasmCodePointer)
pub const INVALID_WASM_CODE_POINTER: u32 = u32::MAX;

// —— 提取不变式断言 ——
const _: () = {
  assert!(IS_DYING_MASK == 0x8000_0000);
  assert!(!NEEDS_FAR_JUMPS_BETWEEN_CODE_SPACES_NEAR);
  assert!(NEEDS_FAR_JUMPS_BETWEEN_CODE_SPACES_FAR);
};
