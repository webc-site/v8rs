//! v8/src/wasm/wasm-features.{h,cc} 的 1:1 转写
//! 特性表源: wasm-features.h FOREACH_WASM_NON_FLAG_FEATURE +
//! src/flags/feature-flags.h 四组 WASM_FEATURE (experimental/pre-staged/staged/shipped,
//! 即 FOREACH_WASM_FEATURE_FLAG 展开序)。
//! 登记差异:
//! - FromFlags()/FromIsolate()/FromContext() 依赖 v8_flags/Isolate/NativeContext
//!   → 推迟至 flags/engine 梯队 (见 crate README)。
//! - base::EnumSet → v8_base::enum_set::EnumSet (位集语义 1:1)。
//! - ostream << → strum Display (name() 同语义)。
//! - CompileTimeImports 移动语义 → Clone; DEBUG 自毁行为省略。
// 由 tool/src/wasm/wasm-features.js 从 v8/src/wasm/wasm-features.h + src/flags/feature-flags.h (WASM_FEATURE 展开序) 生成; 修订就地改, 复跑覆盖
// 对数见脚本运行输出
use std::{
  fmt,
  ops::{Deref, DerefMut},
};

use v8_base::enum_set::{EnumOrdinal, EnumSet};

// ---------------------------------------------------------------------------
// WasmEnabledFeature: 有 flag 的特性 (FOREACH_WASM_FEATURE_FLAG)
// ---------------------------------------------------------------------------

/// 启用特性枚举 (序号 = FOREACH_WASM_FEATURE_FLAG 展开序)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum WasmEnabledFeature {
  /// instruction tracing section (experimental)
  #[strum(serialize = "instruction_tracing")]
  InstructionTracing,
  /// shared-everything threads (experimental)
  #[strum(serialize = "shared")]
  Shared,
  /// fp16 (experimental)
  #[strum(serialize = "fp16")]
  Fp16,
  /// memory control (experimental)
  #[strum(serialize = "memory_control")]
  MemoryControl,
  /// core stack switching (experimental)
  #[strum(serialize = "wasmfx")]
  Wasmfx,
  /// compilation hints (experimental)
  #[strum(serialize = "compilation_hints")]
  CompilationHints,
  /// growable stacks for jspi (experimental)
  #[strum(serialize = "growable_stacks")]
  GrowableStacks,
  /// reference-typed strings (pre_staged)
  #[strum(serialize = "stringref")]
  Stringref,
  /// imported strings (utf8 features) (pre_staged)
  #[strum(serialize = "imported_strings_utf8")]
  ImportedStringsUtf8,
  /// acquire_release memory ordering (pre_staged)
  #[strum(serialize = "acquire_release")]
  AcquireRelease,
  /// compact import section (pre_staged)
  #[strum(serialize = "compact_imports")]
  CompactImports,
  /// custom descriptors (staged)
  #[strum(serialize = "custom_descriptors")]
  CustomDescriptors,
  /// wide arithmetic (staged)
  #[strum(serialize = "wide_arithmetic")]
  WideArithmetic,
  /// legacy exception handling opcodes (shipped)
  #[strum(serialize = "legacy_eh")]
  LegacyEh,
}

/// name(WasmEnabledFeature) — 与 strum Display 同源 (零分配 match)
pub fn enabled_feature_name(f: WasmEnabledFeature) -> &'static str {
  match f {
    WasmEnabledFeature::InstructionTracing => "instruction_tracing",
    WasmEnabledFeature::Shared => "shared",
    WasmEnabledFeature::Fp16 => "fp16",
    WasmEnabledFeature::MemoryControl => "memory_control",
    WasmEnabledFeature::Wasmfx => "wasmfx",
    WasmEnabledFeature::CompilationHints => "compilation_hints",
    WasmEnabledFeature::GrowableStacks => "growable_stacks",
    WasmEnabledFeature::Stringref => "stringref",
    WasmEnabledFeature::ImportedStringsUtf8 => "imported_strings_utf8",
    WasmEnabledFeature::AcquireRelease => "acquire_release",
    WasmEnabledFeature::CompactImports => "compact_imports",
    WasmEnabledFeature::CustomDescriptors => "custom_descriptors",
    WasmEnabledFeature::WideArithmetic => "wide_arithmetic",
    WasmEnabledFeature::LegacyEh => "legacy_eh",
  }
}

macro_rules! impl_ordinal {
  ($ty:ty) => {
    impl EnumOrdinal for $ty {
      #[inline]
      fn ordinal(self) -> usize {
        self as usize
      }
      fn from_ordinal(ordinal: usize) -> Option<Self> {
        match ordinal {
          0 => Some(<$ty>::InstructionTracing),
          1 => Some(<$ty>::Shared),
          2 => Some(<$ty>::Fp16),
          3 => Some(<$ty>::MemoryControl),
          4 => Some(<$ty>::Wasmfx),
          5 => Some(<$ty>::CompilationHints),
          6 => Some(<$ty>::GrowableStacks),
          7 => Some(<$ty>::Stringref),
          8 => Some(<$ty>::ImportedStringsUtf8),
          9 => Some(<$ty>::AcquireRelease),
          10 => Some(<$ty>::CompactImports),
          11 => Some(<$ty>::CustomDescriptors),
          12 => Some(<$ty>::WideArithmetic),
          13 => Some(<$ty>::LegacyEh),
          _ => None,
        }
      }
    }
  };
}

impl_ordinal!(WasmEnabledFeature);

/// 启用特性集 (仅含带 flag 的特性); EnumSet 的 newtype 包装 (原版继承 EnumSet)
/// (EnumSet 无 Debug → 手写转发 Display)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct WasmEnabledFeatures(pub EnumSet<WasmEnabledFeature>);

impl fmt::Debug for WasmEnabledFeatures {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&self.0, f)
  }
}

impl Deref for WasmEnabledFeatures {
  type Target = EnumSet<WasmEnabledFeature>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for WasmEnabledFeatures {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl WasmEnabledFeatures {
  /// All(): 全部 flag 特性
  pub fn all() -> Self {
    Self(EnumSet::from_elements(&[
      WasmEnabledFeature::InstructionTracing,
      WasmEnabledFeature::Shared,
      WasmEnabledFeature::Fp16,
      WasmEnabledFeature::MemoryControl,
      WasmEnabledFeature::Wasmfx,
      WasmEnabledFeature::CompilationHints,
      WasmEnabledFeature::GrowableStacks,
      WasmEnabledFeature::Stringref,
      WasmEnabledFeature::ImportedStringsUtf8,
      WasmEnabledFeature::AcquireRelease,
      WasmEnabledFeature::CompactImports,
      WasmEnabledFeature::CustomDescriptors,
      WasmEnabledFeature::WideArithmetic,
      WasmEnabledFeature::LegacyEh,
    ]))
  }

  /// None()
  pub fn none() -> Self {
    Self(EnumSet::default())
  }

  /// ForAsmjs()
  pub fn for_asmjs() -> Self {
    Self::none()
  }
}

/// has_##feat getter (原版 X-macro 展开)
impl WasmEnabledFeatures {
  /// has_instruction_tracing()
  #[inline]
  pub fn has_instruction_tracing(&self) -> bool {
    self.contains(WasmEnabledFeature::InstructionTracing)
  }
  /// has_shared()
  #[inline]
  pub fn has_shared(&self) -> bool {
    self.contains(WasmEnabledFeature::Shared)
  }
  /// has_fp16()
  #[inline]
  pub fn has_fp16(&self) -> bool {
    self.contains(WasmEnabledFeature::Fp16)
  }
  /// has_memory_control()
  #[inline]
  pub fn has_memory_control(&self) -> bool {
    self.contains(WasmEnabledFeature::MemoryControl)
  }
  /// has_wasmfx()
  #[inline]
  pub fn has_wasmfx(&self) -> bool {
    self.contains(WasmEnabledFeature::Wasmfx)
  }
  /// has_compilation_hints()
  #[inline]
  pub fn has_compilation_hints(&self) -> bool {
    self.contains(WasmEnabledFeature::CompilationHints)
  }
  /// has_growable_stacks()
  #[inline]
  pub fn has_growable_stacks(&self) -> bool {
    self.contains(WasmEnabledFeature::GrowableStacks)
  }
  /// has_stringref()
  #[inline]
  pub fn has_stringref(&self) -> bool {
    self.contains(WasmEnabledFeature::Stringref)
  }
  /// has_imported_strings_utf8()
  #[inline]
  pub fn has_imported_strings_utf8(&self) -> bool {
    self.contains(WasmEnabledFeature::ImportedStringsUtf8)
  }
  /// has_acquire_release()
  #[inline]
  pub fn has_acquire_release(&self) -> bool {
    self.contains(WasmEnabledFeature::AcquireRelease)
  }
  /// has_compact_imports()
  #[inline]
  pub fn has_compact_imports(&self) -> bool {
    self.contains(WasmEnabledFeature::CompactImports)
  }
  /// has_custom_descriptors()
  #[inline]
  pub fn has_custom_descriptors(&self) -> bool {
    self.contains(WasmEnabledFeature::CustomDescriptors)
  }
  /// has_wide_arithmetic()
  #[inline]
  pub fn has_wide_arithmetic(&self) -> bool {
    self.contains(WasmEnabledFeature::WideArithmetic)
  }
  /// has_legacy_eh()
  #[inline]
  pub fn has_legacy_eh(&self) -> bool {
    self.contains(WasmEnabledFeature::LegacyEh)
  }
}

// ---------------------------------------------------------------------------
// WasmDetectedFeature: flag 特性 + 恒开特性 (FOREACH_WASM_FEATURE)
// ---------------------------------------------------------------------------

/// 检出特性枚举 (序号 = FOREACH_WASM_FEATURE_FLAG 后接 FOREACH_WASM_NON_FLAG_FEATURE)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum WasmDetectedFeature {
  #[strum(serialize = "instruction_tracing")]
  InstructionTracing,
  #[strum(serialize = "shared")]
  Shared,
  #[strum(serialize = "fp16")]
  Fp16,
  #[strum(serialize = "memory_control")]
  MemoryControl,
  #[strum(serialize = "wasmfx")]
  Wasmfx,
  #[strum(serialize = "compilation_hints")]
  CompilationHints,
  #[strum(serialize = "growable_stacks")]
  GrowableStacks,
  #[strum(serialize = "stringref")]
  Stringref,
  #[strum(serialize = "imported_strings_utf8")]
  ImportedStringsUtf8,
  #[strum(serialize = "acquire_release")]
  AcquireRelease,
  #[strum(serialize = "compact_imports")]
  CompactImports,
  #[strum(serialize = "custom_descriptors")]
  CustomDescriptors,
  #[strum(serialize = "wide_arithmetic")]
  WideArithmetic,
  #[strum(serialize = "legacy_eh")]
  LegacyEh,
  #[strum(serialize = "shared_memory")]
  SharedMemory,
  #[strum(serialize = "reftypes")]
  Reftypes,
  #[strum(serialize = "simd")]
  Simd,
  #[strum(serialize = "threads")]
  Threads,
  #[strum(serialize = "return_call")]
  ReturnCall,
  #[strum(serialize = "extended_const")]
  ExtendedConst,
  #[strum(serialize = "relaxed_simd")]
  RelaxedSimd,
  #[strum(serialize = "gc")]
  Gc,
  #[strum(serialize = "imported_strings")]
  ImportedStrings,
  #[strum(serialize = "typed_funcref")]
  TypedFuncref,
  #[strum(serialize = "multi_memory")]
  MultiMemory,
  #[strum(serialize = "multi_value")]
  MultiValue,
  #[strum(serialize = "memory64")]
  Memory64,
  #[strum(serialize = "bulk_memory")]
  BulkMemory,
  #[strum(serialize = "mutable_globals")]
  MutableGlobals,
  #[strum(serialize = "non_trapping_float_to_int")]
  NonTrappingFloatToInt,
  #[strum(serialize = "sign_extension_ops")]
  SignExtensionOps,
  #[strum(serialize = "jspi")]
  Jspi,
  #[strum(serialize = "exnref")]
  Exnref,
  #[strum(serialize = "branch_hinting")]
  BranchHinting,
  #[strum(serialize = "gc_allocation")]
  GcAllocation,
}

/// name(WasmDetectedFeature) — 与 strum Display 同源 (零分配 match)
pub fn detected_feature_name(f: WasmDetectedFeature) -> &'static str {
  match f {
    WasmDetectedFeature::InstructionTracing => "instruction_tracing",
    WasmDetectedFeature::Shared => "shared",
    WasmDetectedFeature::Fp16 => "fp16",
    WasmDetectedFeature::MemoryControl => "memory_control",
    WasmDetectedFeature::Wasmfx => "wasmfx",
    WasmDetectedFeature::CompilationHints => "compilation_hints",
    WasmDetectedFeature::GrowableStacks => "growable_stacks",
    WasmDetectedFeature::Stringref => "stringref",
    WasmDetectedFeature::ImportedStringsUtf8 => "imported_strings_utf8",
    WasmDetectedFeature::AcquireRelease => "acquire_release",
    WasmDetectedFeature::CompactImports => "compact_imports",
    WasmDetectedFeature::CustomDescriptors => "custom_descriptors",
    WasmDetectedFeature::WideArithmetic => "wide_arithmetic",
    WasmDetectedFeature::LegacyEh => "legacy_eh",
    WasmDetectedFeature::SharedMemory => "shared_memory",
    WasmDetectedFeature::Reftypes => "reftypes",
    WasmDetectedFeature::Simd => "simd",
    WasmDetectedFeature::Threads => "threads",
    WasmDetectedFeature::ReturnCall => "return_call",
    WasmDetectedFeature::ExtendedConst => "extended_const",
    WasmDetectedFeature::RelaxedSimd => "relaxed_simd",
    WasmDetectedFeature::Gc => "gc",
    WasmDetectedFeature::ImportedStrings => "imported_strings",
    WasmDetectedFeature::TypedFuncref => "typed_funcref",
    WasmDetectedFeature::MultiMemory => "multi_memory",
    WasmDetectedFeature::MultiValue => "multi_value",
    WasmDetectedFeature::Memory64 => "memory64",
    WasmDetectedFeature::BulkMemory => "bulk_memory",
    WasmDetectedFeature::MutableGlobals => "mutable_globals",
    WasmDetectedFeature::NonTrappingFloatToInt => "non_trapping_float_to_int",
    WasmDetectedFeature::SignExtensionOps => "sign_extension_ops",
    WasmDetectedFeature::Jspi => "jspi",
    WasmDetectedFeature::Exnref => "exnref",
    WasmDetectedFeature::BranchHinting => "branch_hinting",
    WasmDetectedFeature::GcAllocation => "gc_allocation",
  }
}

impl EnumOrdinal for WasmDetectedFeature {
  #[inline]
  fn ordinal(self) -> usize {
    self as usize
  }
  fn from_ordinal(ordinal: usize) -> Option<Self> {
    match ordinal {
      0 => Some(Self::InstructionTracing),
      1 => Some(Self::Shared),
      2 => Some(Self::Fp16),
      3 => Some(Self::MemoryControl),
      4 => Some(Self::Wasmfx),
      5 => Some(Self::CompilationHints),
      6 => Some(Self::GrowableStacks),
      7 => Some(Self::Stringref),
      8 => Some(Self::ImportedStringsUtf8),
      9 => Some(Self::AcquireRelease),
      10 => Some(Self::CompactImports),
      11 => Some(Self::CustomDescriptors),
      12 => Some(Self::WideArithmetic),
      13 => Some(Self::LegacyEh),
      14 => Some(Self::SharedMemory),
      15 => Some(Self::Reftypes),
      16 => Some(Self::Simd),
      17 => Some(Self::Threads),
      18 => Some(Self::ReturnCall),
      19 => Some(Self::ExtendedConst),
      20 => Some(Self::RelaxedSimd),
      21 => Some(Self::Gc),
      22 => Some(Self::ImportedStrings),
      23 => Some(Self::TypedFuncref),
      24 => Some(Self::MultiMemory),
      25 => Some(Self::MultiValue),
      26 => Some(Self::Memory64),
      27 => Some(Self::BulkMemory),
      28 => Some(Self::MutableGlobals),
      29 => Some(Self::NonTrappingFloatToInt),
      30 => Some(Self::SignExtensionOps),
      31 => Some(Self::Jspi),
      32 => Some(Self::Exnref),
      33 => Some(Self::BranchHinting),
      34 => Some(Self::GcAllocation),
      _ => None,
    }
  }
}

/// 检出特性集 (u64 位图; 35 项); EnumSet newtype 包装
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct WasmDetectedFeatures(pub EnumSet<WasmDetectedFeature, u64>);

impl fmt::Debug for WasmDetectedFeatures {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&self.0, f)
  }
}

impl Deref for WasmDetectedFeatures {
  type Target = EnumSet<WasmDetectedFeature, u64>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for WasmDetectedFeatures {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// add_##feat / has_##feat getter (原版 X-macro 展开)
impl WasmDetectedFeatures {
  /// 从 EnumSet 构造 (原版隐式转换)
  #[inline]
  pub fn from_set(set: EnumSet<WasmDetectedFeature, u64>) -> Self {
    Self(set)
  }
  /// add_instruction_tracing()
  #[inline]
  pub fn add_instruction_tracing(&mut self) {
    self.add(WasmDetectedFeature::InstructionTracing);
  }
  /// has_instruction_tracing()
  #[inline]
  pub fn has_instruction_tracing(&self) -> bool {
    self.contains(WasmDetectedFeature::InstructionTracing)
  }
  /// add_shared()
  #[inline]
  pub fn add_shared(&mut self) {
    self.add(WasmDetectedFeature::Shared);
  }
  /// has_shared()
  #[inline]
  pub fn has_shared(&self) -> bool {
    self.contains(WasmDetectedFeature::Shared)
  }
  /// add_fp16()
  #[inline]
  pub fn add_fp16(&mut self) {
    self.add(WasmDetectedFeature::Fp16);
  }
  /// has_fp16()
  #[inline]
  pub fn has_fp16(&self) -> bool {
    self.contains(WasmDetectedFeature::Fp16)
  }
  /// add_memory_control()
  #[inline]
  pub fn add_memory_control(&mut self) {
    self.add(WasmDetectedFeature::MemoryControl);
  }
  /// has_memory_control()
  #[inline]
  pub fn has_memory_control(&self) -> bool {
    self.contains(WasmDetectedFeature::MemoryControl)
  }
  /// add_wasmfx()
  #[inline]
  pub fn add_wasmfx(&mut self) {
    self.add(WasmDetectedFeature::Wasmfx);
  }
  /// has_wasmfx()
  #[inline]
  pub fn has_wasmfx(&self) -> bool {
    self.contains(WasmDetectedFeature::Wasmfx)
  }
  /// add_compilation_hints()
  #[inline]
  pub fn add_compilation_hints(&mut self) {
    self.add(WasmDetectedFeature::CompilationHints);
  }
  /// has_compilation_hints()
  #[inline]
  pub fn has_compilation_hints(&self) -> bool {
    self.contains(WasmDetectedFeature::CompilationHints)
  }
  /// add_growable_stacks()
  #[inline]
  pub fn add_growable_stacks(&mut self) {
    self.add(WasmDetectedFeature::GrowableStacks);
  }
  /// has_growable_stacks()
  #[inline]
  pub fn has_growable_stacks(&self) -> bool {
    self.contains(WasmDetectedFeature::GrowableStacks)
  }
  /// add_stringref()
  #[inline]
  pub fn add_stringref(&mut self) {
    self.add(WasmDetectedFeature::Stringref);
  }
  /// has_stringref()
  #[inline]
  pub fn has_stringref(&self) -> bool {
    self.contains(WasmDetectedFeature::Stringref)
  }
  /// add_imported_strings_utf8()
  #[inline]
  pub fn add_imported_strings_utf8(&mut self) {
    self.add(WasmDetectedFeature::ImportedStringsUtf8);
  }
  /// has_imported_strings_utf8()
  #[inline]
  pub fn has_imported_strings_utf8(&self) -> bool {
    self.contains(WasmDetectedFeature::ImportedStringsUtf8)
  }
  /// add_acquire_release()
  #[inline]
  pub fn add_acquire_release(&mut self) {
    self.add(WasmDetectedFeature::AcquireRelease);
  }
  /// has_acquire_release()
  #[inline]
  pub fn has_acquire_release(&self) -> bool {
    self.contains(WasmDetectedFeature::AcquireRelease)
  }
  /// add_compact_imports()
  #[inline]
  pub fn add_compact_imports(&mut self) {
    self.add(WasmDetectedFeature::CompactImports);
  }
  /// has_compact_imports()
  #[inline]
  pub fn has_compact_imports(&self) -> bool {
    self.contains(WasmDetectedFeature::CompactImports)
  }
  /// add_custom_descriptors()
  #[inline]
  pub fn add_custom_descriptors(&mut self) {
    self.add(WasmDetectedFeature::CustomDescriptors);
  }
  /// has_custom_descriptors()
  #[inline]
  pub fn has_custom_descriptors(&self) -> bool {
    self.contains(WasmDetectedFeature::CustomDescriptors)
  }
  /// add_wide_arithmetic()
  #[inline]
  pub fn add_wide_arithmetic(&mut self) {
    self.add(WasmDetectedFeature::WideArithmetic);
  }
  /// has_wide_arithmetic()
  #[inline]
  pub fn has_wide_arithmetic(&self) -> bool {
    self.contains(WasmDetectedFeature::WideArithmetic)
  }
  /// add_legacy_eh()
  #[inline]
  pub fn add_legacy_eh(&mut self) {
    self.add(WasmDetectedFeature::LegacyEh);
  }
  /// has_legacy_eh()
  #[inline]
  pub fn has_legacy_eh(&self) -> bool {
    self.contains(WasmDetectedFeature::LegacyEh)
  }
  /// add_shared_memory()
  #[inline]
  pub fn add_shared_memory(&mut self) {
    self.add(WasmDetectedFeature::SharedMemory);
  }
  /// has_shared_memory()
  #[inline]
  pub fn has_shared_memory(&self) -> bool {
    self.contains(WasmDetectedFeature::SharedMemory)
  }
  /// add_reftypes()
  #[inline]
  pub fn add_reftypes(&mut self) {
    self.add(WasmDetectedFeature::Reftypes);
  }
  /// has_reftypes()
  #[inline]
  pub fn has_reftypes(&self) -> bool {
    self.contains(WasmDetectedFeature::Reftypes)
  }
  /// add_simd()
  #[inline]
  pub fn add_simd(&mut self) {
    self.add(WasmDetectedFeature::Simd);
  }
  /// has_simd()
  #[inline]
  pub fn has_simd(&self) -> bool {
    self.contains(WasmDetectedFeature::Simd)
  }
  /// add_threads()
  #[inline]
  pub fn add_threads(&mut self) {
    self.add(WasmDetectedFeature::Threads);
  }
  /// has_threads()
  #[inline]
  pub fn has_threads(&self) -> bool {
    self.contains(WasmDetectedFeature::Threads)
  }
  /// add_return_call()
  #[inline]
  pub fn add_return_call(&mut self) {
    self.add(WasmDetectedFeature::ReturnCall);
  }
  /// has_return_call()
  #[inline]
  pub fn has_return_call(&self) -> bool {
    self.contains(WasmDetectedFeature::ReturnCall)
  }
  /// add_extended_const()
  #[inline]
  pub fn add_extended_const(&mut self) {
    self.add(WasmDetectedFeature::ExtendedConst);
  }
  /// has_extended_const()
  #[inline]
  pub fn has_extended_const(&self) -> bool {
    self.contains(WasmDetectedFeature::ExtendedConst)
  }
  /// add_relaxed_simd()
  #[inline]
  pub fn add_relaxed_simd(&mut self) {
    self.add(WasmDetectedFeature::RelaxedSimd);
  }
  /// has_relaxed_simd()
  #[inline]
  pub fn has_relaxed_simd(&self) -> bool {
    self.contains(WasmDetectedFeature::RelaxedSimd)
  }
  /// add_gc()
  #[inline]
  pub fn add_gc(&mut self) {
    self.add(WasmDetectedFeature::Gc);
  }
  /// has_gc()
  #[inline]
  pub fn has_gc(&self) -> bool {
    self.contains(WasmDetectedFeature::Gc)
  }
  /// add_imported_strings()
  #[inline]
  pub fn add_imported_strings(&mut self) {
    self.add(WasmDetectedFeature::ImportedStrings);
  }
  /// has_imported_strings()
  #[inline]
  pub fn has_imported_strings(&self) -> bool {
    self.contains(WasmDetectedFeature::ImportedStrings)
  }
  /// add_typed_funcref()
  #[inline]
  pub fn add_typed_funcref(&mut self) {
    self.add(WasmDetectedFeature::TypedFuncref);
  }
  /// has_typed_funcref()
  #[inline]
  pub fn has_typed_funcref(&self) -> bool {
    self.contains(WasmDetectedFeature::TypedFuncref)
  }
  /// add_multi_memory()
  #[inline]
  pub fn add_multi_memory(&mut self) {
    self.add(WasmDetectedFeature::MultiMemory);
  }
  /// has_multi_memory()
  #[inline]
  pub fn has_multi_memory(&self) -> bool {
    self.contains(WasmDetectedFeature::MultiMemory)
  }
  /// add_multi_value()
  #[inline]
  pub fn add_multi_value(&mut self) {
    self.add(WasmDetectedFeature::MultiValue);
  }
  /// has_multi_value()
  #[inline]
  pub fn has_multi_value(&self) -> bool {
    self.contains(WasmDetectedFeature::MultiValue)
  }
  /// add_memory64()
  #[inline]
  pub fn add_memory64(&mut self) {
    self.add(WasmDetectedFeature::Memory64);
  }
  /// has_memory64()
  #[inline]
  pub fn has_memory64(&self) -> bool {
    self.contains(WasmDetectedFeature::Memory64)
  }
  /// add_bulk_memory()
  #[inline]
  pub fn add_bulk_memory(&mut self) {
    self.add(WasmDetectedFeature::BulkMemory);
  }
  /// has_bulk_memory()
  #[inline]
  pub fn has_bulk_memory(&self) -> bool {
    self.contains(WasmDetectedFeature::BulkMemory)
  }
  /// add_mutable_globals()
  #[inline]
  pub fn add_mutable_globals(&mut self) {
    self.add(WasmDetectedFeature::MutableGlobals);
  }
  /// has_mutable_globals()
  #[inline]
  pub fn has_mutable_globals(&self) -> bool {
    self.contains(WasmDetectedFeature::MutableGlobals)
  }
  /// add_non_trapping_float_to_int()
  #[inline]
  pub fn add_non_trapping_float_to_int(&mut self) {
    self.add(WasmDetectedFeature::NonTrappingFloatToInt);
  }
  /// has_non_trapping_float_to_int()
  #[inline]
  pub fn has_non_trapping_float_to_int(&self) -> bool {
    self.contains(WasmDetectedFeature::NonTrappingFloatToInt)
  }
  /// add_sign_extension_ops()
  #[inline]
  pub fn add_sign_extension_ops(&mut self) {
    self.add(WasmDetectedFeature::SignExtensionOps);
  }
  /// has_sign_extension_ops()
  #[inline]
  pub fn has_sign_extension_ops(&self) -> bool {
    self.contains(WasmDetectedFeature::SignExtensionOps)
  }
  /// add_jspi()
  #[inline]
  pub fn add_jspi(&mut self) {
    self.add(WasmDetectedFeature::Jspi);
  }
  /// has_jspi()
  #[inline]
  pub fn has_jspi(&self) -> bool {
    self.contains(WasmDetectedFeature::Jspi)
  }
  /// add_exnref()
  #[inline]
  pub fn add_exnref(&mut self) {
    self.add(WasmDetectedFeature::Exnref);
  }
  /// has_exnref()
  #[inline]
  pub fn has_exnref(&self) -> bool {
    self.contains(WasmDetectedFeature::Exnref)
  }
  /// add_branch_hinting()
  #[inline]
  pub fn add_branch_hinting(&mut self) {
    self.add(WasmDetectedFeature::BranchHinting);
  }
  /// has_branch_hinting()
  #[inline]
  pub fn has_branch_hinting(&self) -> bool {
    self.contains(WasmDetectedFeature::BranchHinting)
  }
  /// add_gc_allocation()
  #[inline]
  pub fn add_gc_allocation(&mut self) {
    self.add(WasmDetectedFeature::GcAllocation);
  }
  /// has_gc_allocation()
  #[inline]
  pub fn has_gc_allocation(&self) -> bool {
    self.contains(WasmDetectedFeature::GcAllocation)
  }
}

// ---------------------------------------------------------------------------
// CompileTimeImport
// ---------------------------------------------------------------------------

/// 编译期导入种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum CompileTimeImport {
  /// "wasm:js-string"
  #[strum(serialize = "js-string")]
  JsString,
  #[strum(serialize = "string-constants")]
  StringConstants,
  #[strum(serialize = "text-encoder")]
  TextEncoder,
  #[strum(serialize = "text-decoder")]
  TextDecoder,
  #[strum(serialize = "js-prototypes")]
  JsPrototypes,
  /// 非真导入, 但需与编译期导入同处理
  #[strum(serialize = "disable-denormal-floats")]
  DisableDenormalFloats,
}

impl EnumOrdinal for CompileTimeImport {
  #[inline]
  fn ordinal(self) -> usize {
    self as usize
  }
  fn from_ordinal(ordinal: usize) -> Option<Self> {
    match ordinal {
      0 => Some(Self::JsString),
      1 => Some(Self::StringConstants),
      2 => Some(Self::TextEncoder),
      3 => Some(Self::TextDecoder),
      4 => Some(Self::JsPrototypes),
      5 => Some(Self::DisableDenormalFloats),
      _ => None,
    }
  }
}

/// CompileTimeImportFlags
pub type CompileTimeImportFlags = EnumSet<CompileTimeImport>;

/// 编译期导入集 (位标志 + string-constants 模块名)
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct CompileTimeImports {
  bits: CompileTimeImportFlags,
  constants_module: &'static str,
}

impl fmt::Debug for CompileTimeImports {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} ['{}']", self.flags(), self.constants_module)
  }
}

impl CompileTimeImports {
  /// empty()
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.bits.empty()
  }

  /// has_string_constants(name): 模块名匹配
  #[inline]
  pub fn has_string_constants(&self, name: &[u8]) -> bool {
    self.bits.contains(CompileTimeImport::StringConstants)
      && self.constants_module.as_bytes() == name
  }

  /// contains(imp)
  #[inline]
  pub fn contains(&self, imp: CompileTimeImport) -> bool {
    self.bits.contains(imp)
  }

  /// Add(imp)
  #[inline]
  pub fn add(&mut self, imp: CompileTimeImport) {
    self.bits.add(imp);
  }

  /// Remove(imp)
  #[inline]
  pub fn remove(&mut self, imp: CompileTimeImport) {
    self.bits.remove(imp);
  }

  /// constants_module()
  #[inline]
  pub fn constants_module(&self) -> &str {
    self.constants_module
  }

  /// constants_module() 可变 (模块名登记)
  #[inline]
  pub fn set_constants_module(&mut self, module: &'static str) {
    self.constants_module = module;
  }

  /// flags()
  #[inline]
  pub fn flags(&self) -> CompileTimeImportFlags {
    self.bits
  }
}
