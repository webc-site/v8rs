// 由 tool/src/objects/code-kind.h.js 从 v8/src/objects/code-kind.{h,cc} 生成; 修订就地改, 复跑覆盖
// CodeKind 分层代码类型: X-macro 全量展开

use core::ops::{BitAnd, BitOr, BitOrAssign};

/// 代码种类 (原版 enum class CodeKind : uint8_t, CODE_KIND_LIST 展开)
/// INTERPRETED_FUNCTION..TURBOFAN_JS 顺序即分层升级次序, 不可重排
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CodeKind {
  /// BYTECODE_HANDLER
  BytecodeHandler = 0,
  /// FOR_TESTING
  ForTesting = 1,
  /// FOR_TESTING_JS
  ForTestingJs = 2,
  /// BUILTIN
  Builtin = 3,
  /// REGEXP
  Regexp = 4,
  /// WASM_FUNCTION
  WasmFunction = 5,
  /// WASM_TO_CAPI_FUNCTION
  WasmToCapiFunction = 6,
  /// WASM_TO_JS_FUNCTION
  WasmToJsFunction = 7,
  /// JS_TO_WASM_FUNCTION
  JsToWasmFunction = 8,
  /// C_WASM_ENTRY
  CWasmEntry = 9,
  /// INTERPRETED_FUNCTION
  InterpretedFunction = 10,
  /// BASELINE
  Baseline = 11,
  /// MAGLEV
  Maglev = 12,
  /// TURBOFAN_JS
  TurbofanJs = 13,
  /// WASM_STACK_ENTRY
  WasmStackEntry = 14,
}

impl CodeKind {
  pub const BYTECODE_HANDLER: Self = Self::BytecodeHandler;
  pub const FOR_TESTING: Self = Self::ForTesting;
  pub const FOR_TESTING_JS: Self = Self::ForTestingJs;
  pub const BUILTIN: Self = Self::Builtin;
  pub const REGEXP: Self = Self::Regexp;
  pub const WASM_FUNCTION: Self = Self::WasmFunction;
  pub const WASM_TO_CAPI_FUNCTION: Self = Self::WasmToCapiFunction;
  pub const WASM_TO_JS_FUNCTION: Self = Self::WasmToJsFunction;
  pub const JS_TO_WASM_FUNCTION: Self = Self::JsToWasmFunction;
  pub const C_WASM_ENTRY: Self = Self::CWasmEntry;
  pub const INTERPRETED_FUNCTION: Self = Self::InterpretedFunction;
  pub const BASELINE: Self = Self::Baseline;
  pub const MAGLEV: Self = Self::Maglev;
  pub const TURBOFAN_JS: Self = Self::TurbofanJs;
  pub const WASM_STACK_ENTRY: Self = Self::WasmStackEntry;

  /// 种类总数 (原版 kCodeKindCount)
  pub const COUNT: usize = 15;

  /// 数值反查 (Code::kind 位域解码用; 越界值 None)
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::BytecodeHandler),
      1 => Some(Self::ForTesting),
      2 => Some(Self::ForTestingJs),
      3 => Some(Self::Builtin),
      4 => Some(Self::Regexp),
      5 => Some(Self::WasmFunction),
      6 => Some(Self::WasmToCapiFunction),
      7 => Some(Self::WasmToJsFunction),
      8 => Some(Self::JsToWasmFunction),
      9 => Some(Self::CWasmEntry),
      10 => Some(Self::InterpretedFunction),
      11 => Some(Self::Baseline),
      12 => Some(Self::Maglev),
      13 => Some(Self::TurbofanJs),
      14 => Some(Self::WasmStackEntry),
      _ => None,
    }
  }

  /// 原版 CodeKindToString (cc 侧 CASE 宏展开)
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::BytecodeHandler => "BYTECODE_HANDLER",
      Self::ForTesting => "FOR_TESTING",
      Self::ForTestingJs => "FOR_TESTING_JS",
      Self::Builtin => "BUILTIN",
      Self::Regexp => "REGEXP",
      Self::WasmFunction => "WASM_FUNCTION",
      Self::WasmToCapiFunction => "WASM_TO_CAPI_FUNCTION",
      Self::WasmToJsFunction => "WASM_TO_JS_FUNCTION",
      Self::JsToWasmFunction => "JS_TO_WASM_FUNCTION",
      Self::CWasmEntry => "C_WASM_ENTRY",
      Self::InterpretedFunction => "INTERPRETED_FUNCTION",
      Self::Baseline => "BASELINE",
      Self::Maglev => "MAGLEV",
      Self::TurbofanJs => "TURBOFAN_JS",
      Self::WasmStackEntry => "WASM_STACK_ENTRY",
    }
  }

  /// 原版 CodeKindToMarker (剖析用变体标记)
  #[must_use]
  pub fn to_marker(self, context_specialized: bool, osr: bool) -> &'static str {
    match self {
      Self::InterpretedFunction => "~",
      Self::Baseline => "^",
      Self::Maglev => match (osr, context_specialized) {
        (true, _) => "o+",
        (false, true) => "+'",
        (false, false) => "+",
      },
      Self::TurbofanJs => match (osr, context_specialized) {
        (true, _) => "o*",
        (false, true) => "*'",
        (false, false) => "*",
      },
      _ => "",
    }
  }

  /// 原版 CodeKindIsInterpretedJSFunction
  #[inline]
  #[must_use]
  pub const fn is_interpreted_js_function(self) -> bool {
    matches!(self, Self::InterpretedFunction)
  }

  /// 原版 CodeKindIsBaselinedJSFunction
  #[inline]
  #[must_use]
  pub const fn is_baselined_js_function(self) -> bool {
    matches!(self, Self::Baseline)
  }

  /// 原版 CodeKindIsUnoptimizedJSFunction (INTERPRETED_FUNCTION..=BASELINE)
  #[inline]
  #[must_use]
  pub const fn is_unoptimized_js_function(self) -> bool {
    matches!(self, Self::InterpretedFunction | Self::Baseline)
  }

  /// 原版 CodeKindIsOptimizedJSFunction (MAGLEV..=TURBOFAN_JS)
  #[inline]
  #[must_use]
  pub const fn is_optimized_js_function(self) -> bool {
    matches!(self, Self::Maglev | Self::TurbofanJs)
  }

  /// 原版 CodeKindIsJSFunction (INTERPRETED_FUNCTION..=TURBOFAN_JS)
  #[inline]
  #[must_use]
  pub const fn is_js_function(self) -> bool {
    self as u8 <= Self::TurbofanJs as u8 && self as u8 >= Self::InterpretedFunction as u8
  }

  /// 原版 CodeKindIsBuiltinOrJSFunction
  #[inline]
  #[must_use]
  pub const fn is_builtin_or_js_function(self) -> bool {
    matches!(self, Self::Builtin) || self.is_js_function()
  }

  /// 原版 CodeKindCanOSR (wasm_deopt 语义分支见原版 CodeKindCanDeoptimize)
  #[inline]
  #[must_use]
  pub const fn can_osr(self) -> bool {
    matches!(self, Self::TurbofanJs | Self::Maglev)
  }

  /// 原版 CodeKindCanTierUp
  #[inline]
  #[must_use]
  pub const fn can_tier_up(self) -> bool {
    self.is_unoptimized_js_function() || matches!(self, Self::Maglev)
  }

  /// 原版 CodeKindIsStoredInOptimizedCodeCache
  #[inline]
  #[must_use]
  pub const fn is_stored_in_optimized_code_cache(self) -> bool {
    matches!(self, Self::Maglev | Self::TurbofanJs)
  }

  /// 原版 CodeKindUsesBytecodeOrInterpreterData
  #[inline]
  #[must_use]
  pub const fn uses_bytecode_or_interpreter_data(self) -> bool {
    self.is_baselined_js_function()
  }

  /// 原版 CodeKindUsesDeoptimizationData (排除 wasm_deopt 运行时分支)
  #[inline]
  #[must_use]
  pub const fn uses_deoptimization_data(self) -> bool {
    self.is_optimized_js_function()
  }

  /// 原版 CodeKindUsesBytecodeOffsetTable
  #[inline]
  #[must_use]
  pub const fn uses_bytecode_offset_table(self) -> bool {
    matches!(self, Self::Baseline)
  }

  /// 原版 CodeKindMayLackSourcePositionTable
  #[inline]
  #[must_use]
  pub const fn may_lack_source_position_table(self) -> bool {
    self.uses_bytecode_offset_table()
      || matches!(
        self,
        Self::Builtin | Self::BytecodeHandler | Self::ForTesting
      )
  }

  /// 原版 CodeKindForTopTier
  #[inline]
  #[must_use]
  pub const fn for_top_tier() -> Self {
    Self::TurbofanJs
  }

  /// 转换为 CodeKindFlag 位集 (原版 CodeKindToCodeKindFlag)
  #[inline]
  #[must_use]
  pub const fn to_code_kind_flag(self) -> CodeKindFlag {
    CodeKindFlag(1 << (self as u8))
  }
}

impl From<CodeKind> for CodeKindFlag {
  #[inline]
  fn from(kind: CodeKind) -> Self {
    kind.to_code_kind_flag()
  }
}

/// 原版 CodeKindToCodeKindFlag
#[inline]
#[must_use]
pub const fn code_kind_to_code_kind_flag(kind: CodeKind) -> CodeKindFlag {
  kind.to_code_kind_flag()
}

/// 位集形式的 CodeKind (原版 enum class CodeKindFlag, 1 << CodeKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CodeKindFlag(pub u32);

impl CodeKindFlag {
  /// CodeKind::BYTECODE_HANDLER 对应位
  pub const BYTECODE_HANDLER: Self = Self(1);
  /// CodeKind::FOR_TESTING 对应位
  pub const FOR_TESTING: Self = Self(2);
  /// CodeKind::FOR_TESTING_JS 对应位
  pub const FOR_TESTING_JS: Self = Self(4);
  /// CodeKind::BUILTIN 对应位
  pub const BUILTIN: Self = Self(8);
  /// CodeKind::REGEXP 对应位
  pub const REGEXP: Self = Self(16);
  /// CodeKind::WASM_FUNCTION 对应位
  pub const WASM_FUNCTION: Self = Self(32);
  /// CodeKind::WASM_TO_CAPI_FUNCTION 对应位
  pub const WASM_TO_CAPI_FUNCTION: Self = Self(64);
  /// CodeKind::WASM_TO_JS_FUNCTION 对应位
  pub const WASM_TO_JS_FUNCTION: Self = Self(128);
  /// CodeKind::JS_TO_WASM_FUNCTION 对应位
  pub const JS_TO_WASM_FUNCTION: Self = Self(256);
  /// CodeKind::C_WASM_ENTRY 对应位
  pub const C_WASM_ENTRY: Self = Self(512);
  /// CodeKind::INTERPRETED_FUNCTION 对应位
  pub const INTERPRETED_FUNCTION: Self = Self(1024);
  /// CodeKind::BASELINE 对应位
  pub const BASELINE: Self = Self(2048);
  /// CodeKind::MAGLEV 对应位
  pub const MAGLEV: Self = Self(4096);
  /// CodeKind::TURBOFAN_JS 对应位
  pub const TURBOFAN_JS: Self = Self(8192);
  /// CodeKind::WASM_STACK_ENTRY 对应位
  pub const WASM_STACK_ENTRY: Self = Self(16384);
}

/// CodeKind 集合 (原版 base::Flags<CodeKindFlag> 别名 CodeKinds)
pub type CodeKinds = CodeKindFlag;

impl BitOr for CodeKindFlag {
  type Output = Self;
  #[inline]
  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}

impl BitOrAssign for CodeKindFlag {
  #[inline]
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 |= rhs.0;
  }
}

impl BitAnd for CodeKindFlag {
  type Output = Self;
  #[inline]
  fn bitand(self, rhs: Self) -> Self {
    Self(self.0 & rhs.0)
  }
}

impl CodeKindFlag {
  /// 是否含该位
  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }

  /// 是否包含指定 CodeKind
  #[inline]
  #[must_use]
  pub const fn contains_kind(self, kind: CodeKind) -> bool {
    self.contains(kind.to_code_kind_flag())
  }
}

/// JS 函数类 CodeKind 掩码 (原版 kJSFunctionCodeKindsMask)
pub const JS_FUNCTION_CODE_KINDS_MASK: CodeKindFlag = CodeKindFlag(15360);

/// 已优化 JS 函数掩码 (原版 kOptimizedJSFunctionCodeKindsMask)
pub const OPTIMIZED_JS_FUNCTION_CODE_KINDS_MASK: CodeKindFlag = CodeKindFlag(12288);

// —— 提取不变式断言 ——
const _: () = {
  assert!(CodeKind::COUNT == 15);
  assert!((CodeKind::InterpretedFunction as u8) < (CodeKind::Baseline as u8));
  assert!((CodeKind::Baseline as u8) < (CodeKind::TurbofanJs as u8));
  assert!(JS_FUNCTION_CODE_KINDS_MASK.contains(CodeKindFlag::INTERPRETED_FUNCTION));
  assert!(JS_FUNCTION_CODE_KINDS_MASK.contains(CodeKindFlag::TURBOFAN_JS));
  assert!(!JS_FUNCTION_CODE_KINDS_MASK.contains(CodeKindFlag::BUILTIN));
  assert!(OPTIMIZED_JS_FUNCTION_CODE_KINDS_MASK.contains(CodeKindFlag::MAGLEV));
};
