// 由 tool/src/wasm/fuzzing/random-module-generation.h.js 从 v8/src/wasm/fuzzing/random-module-generation.h 生成; 修订就地改, 复跑覆盖
// 随机模块生成选项 (fuzzing 面)

/// 随机模块生成选项位 (对应原版 enum WasmModuleGenerationOption)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WasmModuleGenerationOption {
  /// kGenerateSIMD
  GenerateSimd = 0,
  /// kGenerateWasmGC
  GenerateWasmGc = 1,
}

/// 选项集合 (原版 WasmModuleGenerationOptions, base::EnumSet)
pub type WasmModuleGenerationOptions = u8;

/// 原版 MVP() — 空集合
pub const GEN_MVP: WasmModuleGenerationOptions = 0;
/// 原版 Simd() — 仅 kGenerateSIMD
pub const GEN_SIMD: WasmModuleGenerationOptions =
  1 << WasmModuleGenerationOption::GenerateSimd as u8;
/// 原版 WasmGC() — 仅 kGenerateWasmGC
pub const GEN_WASM_GC: WasmModuleGenerationOptions =
  1 << WasmModuleGenerationOption::GenerateWasmGc as u8;
/// 原版 All() — 全部
pub const GEN_ALL: WasmModuleGenerationOptions = GEN_SIMD | GEN_WASM_GC;

// —— 提取不变式断言 ——
const _: () = {
  assert!(GEN_MVP == 0);
  assert!(GEN_ALL == 0b11);
};
