#!/usr/bin/env bun
// tool/src/objects/code-kind.h.js — 覆盖 v8/src/objects/code-kind.h + code-kind.cc
// CODE_KIND_LIST X-macro 全量展开 → CodeKind 枚举 / CodeKindFlag 位集 / 掩码 / 谓词
// → v8-objects-rest/src/code_kind.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/code-kind.h");
const ccSrc = readV8("src/objects/code-kind.cc");

// 1. 提取 CODE_KIND_LIST 宏体: 逐行 V(NAME)
const listMatch = hSrc.match(/#define CODE_KIND_LIST\(V\)([\s\S]*?)\n\n/);
if (!listMatch) throw new Error("未找到 CODE_KIND_LIST 宏定义");

const kinds = [];
for (const m of listMatch[1].matchAll(/V\((\w+)\)/g)) kinds.push(m[1]);
if (kinds.length !== 15) throw new Error(`CODE_KIND_LIST 条数不符: 期望 15, 得 ${kinds.length}`);
if (kinds[0] !== "BYTECODE_HANDLER" || kinds[14] !== "WASM_STACK_ENTRY")
  throw new Error("CODE_KIND_LIST 首尾锚点不符, 原版可能已变更");

// 2. CodeKindToMarker 分支提取 (cc 侧字符串表)
const markerBody = ccSrc.match(
  /const char\* CodeKindToMarker\([\s\S]*?switch \(kind\) \{([\s\S]*?)\n  \}\n\}/,
);
if (!markerBody) throw new Error("未找到 CodeKindToMarker switch 体");
const markerMap = new Map();
// 抓 case X: return "str" (含 if 分支内的首个 return)
for (const cm of markerBody[1].matchAll(/case CodeKind::(\w+):([\s\S]*?)(?=case CodeKind::|default:)/g)) {
  const rm = cm[2].match(/return "((?:[^"\\]|\\.)*)"/);
  if (rm) markerMap.set(cm[1], rm[1]);
}
if (markerMap.size !== 4) throw new Error(`CodeKindToMarker 分支数不符: 期望 4 (INTERPRETED/BASELINE/MAGLEV/TURBOFAN), 得 ${markerMap.size}`);

// 3. 位集掩码成员 (h 侧 kJSFunctionCodeKindsMask 等)
const maskMatch = hSrc.match(/static constexpr CodeKinds kJSFunctionCodeKindsMask\{([\s\S]*?)\};/);
if (!maskMatch) throw new Error("未找到 kJSFunctionCodeKindsMask");
const jsMaskKinds = [...maskMatch[1].matchAll(/CodeKindFlag::(\w+)/g)].map((m) => m[1]);
const optMaskMatch = hSrc.match(/static constexpr CodeKinds kOptimizedJSFunctionCodeKindsMask\{([\s\S]*?)\};/);
if (!optMaskMatch) throw new Error("未找到 kOptimizedJSFunctionCodeKindsMask");
const optMaskKinds = [...optMaskMatch[1].matchAll(/CodeKindFlag::(\w+)/g)].map((m) => m[1]);

const flagVal = (name) => 1 << kinds.indexOf(name);
const maskVal = (names) => names.reduce((a, n) => a | flagVal(n), 0);

console.log(`CodeKind ${kinds.length} 项, marker 分支 ${markerMap.size}, 掩码 ${jsMaskKinds.length}+${optMaskKinds.length} 项`);

// 4. 生成 Rust
let rust = banner("src/objects/code-kind.h.js", "src/objects/code-kind.{h,cc}", "CodeKind 分层代码类型: X-macro 全量展开");

rust += `
/// 代码种类 (原版 enum class CodeKind : uint8_t, CODE_KIND_LIST 展开)
/// INTERPRETED_FUNCTION..TURBOFAN_JS 顺序即分层升级次序, 不可重排
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CodeKind {
`;

kinds.forEach((k, i) => {
  rust += `  /// ${k}\n  ${k} = ${i},\n`;
});

rust += `}

impl CodeKind {
  /// 种类总数 (原版 kCodeKindCount)
  pub const COUNT: usize = ${kinds.length};

  /// 原版 CodeKindToString (cc 侧 CASE 宏展开)
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
`;

for (const k of kinds) rust += `      Self::${k} => "${k}",\n`;

rust += `    }
  }

  /// 原版 CodeKindToMarker (剖析用变体标记)
  #[must_use]
  pub fn to_marker(self, context_specialized: bool, osr: bool) -> &'static str {
    match self {
`;

for (const [k, s] of markerMap) {
  // MAGLEV/TURBOFAN_JS 带 osr/context 分支; 原样展开四种组合
  if (k === "MAGLEV" || k === "TURBOFAN_JS") {
    const sym = k === "MAGLEV" ? "+" : "*";
    rust += `      Self::${k} => match (osr, context_specialized) {
        (true, _) => "o${sym}",
        (false, true) => "${sym}'",
        (false, false) => "${sym}",
      },\n`;
  } else {
    rust += `      Self::${k} => "${s}",\n`;
  }
}

rust += `      _ => "",
    }
  }

  /// 原版 CodeKindIsInterpretedJSFunction
  #[inline]
  #[must_use]
  pub const fn is_interpreted_js_function(self) -> bool {
    matches!(self, Self::INTERPRETED_FUNCTION)
  }

  /// 原版 CodeKindIsBaselinedJSFunction
  #[inline]
  #[must_use]
  pub const fn is_baselined_js_function(self) -> bool {
    matches!(self, Self::BASELINE)
  }

  /// 原版 CodeKindIsUnoptimizedJSFunction (INTERPRETED_FUNCTION..=BASELINE)
  #[inline]
  #[must_use]
  pub const fn is_unoptimized_js_function(self) -> bool {
    matches!(self, Self::INTERPRETED_FUNCTION | Self::BASELINE)
  }

  /// 原版 CodeKindIsOptimizedJSFunction (MAGLEV..=TURBOFAN_JS)
  #[inline]
  #[must_use]
  pub const fn is_optimized_js_function(self) -> bool {
    matches!(self, Self::MAGLEV | Self::TURBOFAN_JS)
  }

  /// 原版 CodeKindIsJSFunction (INTERPRETED_FUNCTION..=TURBOFAN_JS)
  #[inline]
  #[must_use]
  pub const fn is_js_function(self) -> bool {
    self as u8 <= Self::TURBOFAN_JS as u8
      && self as u8 >= Self::INTERPRETED_FUNCTION as u8
  }

  /// 原版 CodeKindIsBuiltinOrJSFunction
  #[inline]
  #[must_use]
  pub const fn is_builtin_or_js_function(self) -> bool {
    matches!(self, Self::BUILTIN) || self.is_js_function()
  }

  /// 原版 CodeKindCanOSR (wasm_deopt 语义分支见原版 CodeKindCanDeoptimize)
  #[inline]
  #[must_use]
  pub const fn can_osr(self) -> bool {
    matches!(self, Self::TURBOFAN_JS | Self::MAGLEV)
  }

  /// 原版 CodeKindCanTierUp
  #[inline]
  #[must_use]
  pub const fn can_tier_up(self) -> bool {
    self.is_unoptimized_js_function() || matches!(self, Self::MAGLEV)
  }

  /// 原版 CodeKindIsStoredInOptimizedCodeCache
  #[inline]
  #[must_use]
  pub const fn is_stored_in_optimized_code_cache(self) -> bool {
    matches!(self, Self::MAGLEV | Self::TURBOFAN_JS)
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
    matches!(self, Self::BASELINE)
  }

  /// 原版 CodeKindMayLackSourcePositionTable
  #[inline]
  #[must_use]
  pub const fn may_lack_source_position_table(self) -> bool {
    self.uses_bytecode_offset_table()
      || matches!(self, Self::BUILTIN | Self::BYTECODE_HANDLER | Self::FOR_TESTING)
  }

  /// 原版 CodeKindForTopTier
  #[inline]
  #[must_use]
  pub const fn for_top_tier() -> Self {
    Self::TURBOFAN_JS
  }
}

/// 位集形式的 CodeKind (原版 enum class CodeKindFlag, 1 << CodeKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CodeKindFlag(pub u32);

impl CodeKindFlag {
`;

for (const k of kinds) {
  rust += `  /// CodeKind::${k} 对应位\n  pub const ${k}: Self = Self(${flagVal(k)});\n`;
}

rust += `}

/// CodeKind 集合 (原版 base::Flags<CodeKindFlag> 别名 CodeKinds)
pub type CodeKinds = CodeKindFlag;

impl core::ops::BitOr for CodeKindFlag {
  type Output = Self;
  #[inline]
  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}

impl core::ops::BitOrAssign for CodeKindFlag {
  #[inline]
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 |= rhs.0;
  }
}

impl core::ops::BitAnd for CodeKindFlag {
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
}

/// JS 函数类 CodeKind 掩码 (原版 kJSFunctionCodeKindsMask)
pub const JS_FUNCTION_CODE_KINDS_MASK: CodeKindFlag =
  CodeKindFlag(${maskVal(jsMaskKinds)});

/// 已优化 JS 函数掩码 (原版 kOptimizedJSFunctionCodeKindsMask)
pub const OPTIMIZED_JS_FUNCTION_CODE_KINDS_MASK: CodeKindFlag =
  CodeKindFlag(${maskVal(optMaskKinds)});

// —— 提取不变式断言 ——
const _: () = {
  assert!(CodeKind::COUNT == 15);
  assert!(CodeKind::INTERPRETED_FUNCTION as u8 < CodeKind::BASELINE as u8);
  assert!(CodeKind::BASELINE as u8 < CodeKind::TURBOFAN_JS as u8);
  assert!(JS_FUNCTION_CODE_KINDS_MASK.contains(CodeKindFlag::INTERPRETED_FUNCTION));
  assert!(JS_FUNCTION_CODE_KINDS_MASK.contains(CodeKindFlag::TURBOFAN_JS));
  assert!(!JS_FUNCTION_CODE_KINDS_MASK.contains(CodeKindFlag::BUILTIN));
  assert!(OPTIMIZED_JS_FUNCTION_CODE_KINDS_MASK.contains(CodeKindFlag::MAGLEV));
};
`;

const p = writeSrc("v8-objects-rest", "code_kind.rs", rust);
console.log(`written ${p}`);
