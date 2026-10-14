#!/usr/bin/env bun
// v8/src/wasm/wasm-features.{h,cc} + v8/src/flags/feature-flags.h 的 WASM_FEATURE 项
//   → v8-wasm-core/src/wasm_features.rs
// 解析: FOREACH_WASM_NON_FLAG_FEATURE (恒开特性) + feature-flags.h 四组
// WASM_FEATURE (experimental/pre-staged/staged/shipped, 即 FOREACH_WASM_FEATURE_FLAG)。
// 生成: WasmEnabledFeature (15 flag 特性) / WasmDetectedFeature (flag + 恒开) /
// EnumSet 包装 + has_*/add_* getter / CompileTimeImport 族。
// 对数: 运行时打印两组特性名, 供人工与原版逐项核对。
import { readV8, writeSrc, banner } from "../../lib.js";

const fh = readV8("src/wasm/wasm-features.h");
const flagsH = readV8("src/flags/feature-flags.h");

// ---- FOREACH_WASM_NON_FLAG_FEATURE (恒开特性, 无 flag) ----
function macroBody(src, name) {
  const re = new RegExp(`#define ${name}\\([^)]*\\)[^\\n]*\\n((?:[^\\n]*\\\\\\n)*[^\\n]*)`);
  const m = re.exec(src);
  if (!m || !m[1].trim()) throw new Error(`宏 ${name} 未找到`);
  return m[1];
}
const nonFlag = [...macroBody(fh, "FOREACH_WASM_NON_FLAG_FEATURE").matchAll(/V\((\w+)\)/g)].map((m) => m[1]);

// ---- feature-flags.h 的 WASM_FEATURE 项 (按组序: experimental → pre_staged → staged → shipped) ----
const groups = ["FOREACH_EXPERIMENTAL_FEATURE_FLAG", "FOREACH_PRE_STAGED_FEATURE_FLAG",
  "FOREACH_STAGED_FEATURE_FLAG", "FOREACH_SHIPPED_FEATURE_FLAG"];
const flagFeatures = []; // {name, desc, group}
for (const g of groups) {
  const body = macroBody(flagsH, g);
  // 条件宏 (IF_INTL_ENABLED(JS_FEATURE, ...)) 剥壳后仅取 WASM_FEATURE(name, "desc")
  const cleaned = body.replace(/IF_(?:INTL|REVEC|SPARKPLUG_PLUS)_ENABLED\(([^()]*)\)/g, "");
  for (const m of cleaned.matchAll(/WASM_FEATURE\((\w+),\s*"((?:[^"\\]|\\.)*)"\)/g)) {
    flagFeatures.push({ name: m[1], desc: m[2], group: g.replace("FOREACH_", "").replace("_FEATURE_FLAG", "") });
  }
}

// ---- 对数 ----
console.log("wasm-features 对数:");
console.log(`  FOREACH_WASM_FEATURE_FLAG (flag 特性) ${flagFeatures.length} 项:`);
for (const f of flagFeatures) console.log(`    ${f.group}: ${f.name} — ${f.desc}`);
console.log(`  FOREACH_WASM_NON_FLAG_FEATURE (恒开) ${nonFlag.length} 项:`);
console.log(`    ${nonFlag.join(", ")}`);
console.log(`  WasmDetectedFeature = ${flagFeatures.length} + ${nonFlag.length} = ${flagFeatures.length + nonFlag.length}`);

// ---- 生成 ----
const variant = (n) => n.split("_").map((p) => p[0].toUpperCase() + p.slice(1)).join("");

let out = `//! v8/src/wasm/wasm-features.{h,cc} 的 1:1 转写
//! 特性表源: wasm-features.h FOREACH_WASM_NON_FLAG_FEATURE +
//! src/flags/feature-flags.h 四组 WASM_FEATURE (experimental/pre-staged/staged/shipped,
//! 即 FOREACH_WASM_FEATURE_FLAG 展开序)。
//! 登记差异:
//! - FromFlags()/FromIsolate()/FromContext() 依赖 v8_flags/Isolate/NativeContext
//!   → 推迟至 flags/engine 梯队 (见 crate README)。
//! - base::EnumSet → v8_base::enum_set::EnumSet (位集语义 1:1)。
//! - ostream << → strum Display (name() 同语义)。
//! - CompileTimeImports 移动语义 → Clone; DEBUG 自毁行为省略。
`;
out += banner("src/wasm/wasm-features.js", "src/wasm/wasm-features.h + src/flags/feature-flags.h (WASM_FEATURE 展开序)", "对数见脚本运行输出");
out += `use v8_base::enum_set::{EnumOrdinal, EnumSet};

// ---------------------------------------------------------------------------
// WasmEnabledFeature: 有 flag 的特性 (FOREACH_WASM_FEATURE_FLAG)
// ---------------------------------------------------------------------------

/// 启用特性枚举 (序号 = FOREACH_WASM_FEATURE_FLAG 展开序)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum WasmEnabledFeature {
`;
for (const f of flagFeatures) out += `  /// ${f.desc} (${f.group.toLowerCase()})\n  #[strum(serialize = "${f.name}")]\n  ${variant(f.name)},\n`;
out += `}

/// name(WasmEnabledFeature) — 与 strum Display 同源 (零分配 match)
pub fn enabled_feature_name(f: WasmEnabledFeature) -> &'static str {
  match f {
`;
for (const f of flagFeatures) out += `    WasmEnabledFeature::${variant(f.name)} => "${f.name}",\n`;
out += `  }
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
`;
for (const [i, f] of flagFeatures.entries()) out += `          ${i} => Some(<$ty>::${variant(f.name)}),\n`;
out += `          _ => None,
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

impl std::fmt::Debug for WasmEnabledFeatures {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.0, f)
  }
}

impl std::ops::Deref for WasmEnabledFeatures {
  type Target = EnumSet<WasmEnabledFeature>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for WasmEnabledFeatures {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl WasmEnabledFeatures {
  /// All(): 全部 flag 特性
  pub fn all() -> Self {
    Self(EnumSet::from_elements(&[${flagFeatures.map((f) => `WasmEnabledFeature::${variant(f.name)}`).join(", ")}]))
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
`;
for (const f of flagFeatures) out += `  /// has_${f.name}()
  #[inline]
  pub fn has_${f.name}(&self) -> bool {
    self.contains(WasmEnabledFeature::${variant(f.name)})
  }
`;
out += `}

// ---------------------------------------------------------------------------
// WasmDetectedFeature: flag 特性 + 恒开特性 (FOREACH_WASM_FEATURE)
// ---------------------------------------------------------------------------

/// 检出特性枚举 (序号 = FOREACH_WASM_FEATURE_FLAG 后接 FOREACH_WASM_NON_FLAG_FEATURE)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum WasmDetectedFeature {
`;
for (const f of flagFeatures) out += `  #[strum(serialize = "${f.name}")]\n  ${variant(f.name)},\n`;
for (const n of nonFlag) out += `  #[strum(serialize = "${n}")]\n  ${variant(n)},\n`;
out += `}

/// name(WasmDetectedFeature) — 与 strum Display 同源 (零分配 match)
pub fn detected_feature_name(f: WasmDetectedFeature) -> &'static str {
  match f {
`;
for (const f of flagFeatures) out += `    WasmDetectedFeature::${variant(f.name)} => "${f.name}",\n`;
for (const n of nonFlag) out += `    WasmDetectedFeature::${variant(n)} => "${n}",\n`;
out += `  }
}

impl EnumOrdinal for WasmDetectedFeature {
  #[inline]
  fn ordinal(self) -> usize {
    self as usize
  }
  fn from_ordinal(ordinal: usize) -> Option<Self> {
    match ordinal {
`;
for (const [i, f] of flagFeatures.entries()) out += `      ${i} => Some(Self::${variant(f.name)}),\n`;
for (const [i, n] of nonFlag.entries()) out += `      ${flagFeatures.length + i} => Some(Self::${variant(n)}),\n`;
out += `      _ => None,
    }
  }
}

/// 检出特性集 (u64 位图; 35 项); EnumSet newtype 包装
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct WasmDetectedFeatures(pub EnumSet<WasmDetectedFeature, u64>);

impl std::fmt::Debug for WasmDetectedFeatures {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.0, f)
  }
}

impl std::ops::Deref for WasmDetectedFeatures {
  type Target = EnumSet<WasmDetectedFeature, u64>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for WasmDetectedFeatures {
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
`;
for (const f of [...flagFeatures.map((f) => f.name), ...nonFlag]) {
  out += `  /// add_${f}()
  #[inline]
  pub fn add_${f}(&mut self) {
    self.add(WasmDetectedFeature::${variant(f)});
  }
  /// has_${f}()
  #[inline]
  pub fn has_${f}(&self) -> bool {
    self.contains(WasmDetectedFeature::${variant(f)})
  }
`;
}
out += `}

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

impl std::fmt::Debug for CompileTimeImports {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
`;

const p = writeSrc("v8-wasm-core", "wasm_features.rs", out);
console.log(`→ ${p}`);
