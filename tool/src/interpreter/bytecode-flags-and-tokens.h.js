#!/usr/bin/env bun
// tool/src/interpreter/bytecode-flags-and-tokens.h.js
// 机械提取 src/interpreter/bytecode-flags-and-tokens.{h,cc} 的字面量旗标位域与 typeof 字面量枚举
// → v8-interpreter/src/bytecode_flags_and_tokens.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/bytecode-flags-and-tokens.h");
const ccSrc = readV8("src/interpreter/bytecode-flags-and-tokens.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "using FlagsBits = base::BitField8<int, 0, 5>;", "FlagsBits \u4f4d\u57df"],
  [hSrc, "using FastCloneSupportedBit = FlagsBits::Next<bool, 1>;", "FastCloneSupportedBit"],
  [hSrc, "#define TYPEOF_LITERAL_LIST(V)", "TYPEOF \u5b8f"],
  [hSrc, "using PretenuredBit = base::BitField8<bool, 0, 1>;", "PretenuredBit"],
  [hSrc, "using LanguageModeBit = base::BitField8<LanguageMode, 0, 1>;", "LanguageModeBit"],
  [hSrc, "using LookupHoistingModeBit = LanguageModeBit::Next<bool, 1>;", "LookupHoistingModeBit"],
  [hSrc, "kFallthroughToken = -1,", "TryFinally Token"],
  [ccSrc, "uint8_t CreateArrayLiteralFlags::Encode(bool use_fast_shallow_clone,", "CreateArrayLiteralFlags::Encode \u5b9a\u4e49"],
  [ccSrc, "uint8_t CreateClosureFlags::Encode(bool pretenure, bool is_function_scope) {", "CreateClosureFlags::Encode \u5b9a\u4e49"],
]) {
  if (!src.includes(needle)) throw new Error(`结构锚缺失: ${name}`);
}

// —— 机械提取: 去注释 + 类块 + 成员声明 (对数守卫内嵌) ——
function stripComments(src) {
  return src.replace(/\/\*[\s\S]*?\*\//g, "").replace(/^\s*\/\/.*$/gm, "").replace(/^\s*#.*$/gm, "");
}
function classBlocks(src) {
  const out = [];
  const re = /\b(class|struct)\s+(?:[A-Z][A-Z_0-9]*\s+)?(\w+)[^{;]*\{/g;
  let m;
  while ((m = re.exec(src)) !== null) {
    let depth = 1, i = re.lastIndex;
    while (depth > 0 && i < src.length) {
      if (src[i] === "{") depth++;
      else if (src[i] === "}") depth--;
      i++;
    }
    out.push({ name: m[2], kind: m[1], body: src.slice(re.lastIndex, i - 1) });
  }
  return out;
}
function members(body) {
  const marked = body
    .replace(/\bpublic\s*:/g, "\x01public:")
    .replace(/\bprivate\s*:/g, "\x01private:")
    .replace(/\bprotected\s*:/g, "\x01protected:");
  const out = [];
  let vis = "private";
  let paren = 0, start = 0, i = 0;
  const emit = (seg) => {
    const visM = [...seg.matchAll(/\x01(public|private|protected):/g)];
    if (visM.length) vis = visM[visM.length - 1][1];
    seg = seg.replace(/\x01(?:public|private|protected):/g, " ").trim().replace(/\s+/g, " ");
    if (!seg) return;
    if (/^(enum|template|typedef|using|static_assert|friend)\b/.test(seg)) return;
    if (/DISALLOW|NON_EXPORTED|^#/.test(seg)) return;
    out.push({ sig: seg, vis, kind: /constexpr|static const/.test(seg) ? "const" : (/\(/.test(seg) ? "method" : "field") });
  };
  while (i < marked.length) {
    const c = marked[i];
    if (c === "(") paren++;
    else if (c === ")") paren = Math.max(0, paren - 1);
    else if (paren === 0 && c === ";") {
      emit(marked.slice(start, i));
      start = i + 1;
    } else if (paren === 0 && c === "{") {
      const sigEnd = i;
      let d = 1;
      i++;
      while (d > 0 && i < marked.length) {
        if (marked[i] === "{") d++;
        else if (marked[i] === "}") d--;
        i++;
      }
      emit(marked.slice(start, sigEnd));
      start = i;
      continue;
    }
    i++;
  }
  emit(marked.slice(start));
  return out;
}
// —— TYPEOF_LITERAL_LIST 机械展开 (9 项) ——
const typeofBody = hSrc.slice(hSrc.indexOf("#define TYPEOF_LITERAL_LIST(V)"));
const end = typeofBody.indexOf("\n\n");
const seg = typeofBody.slice(0, end).replace(/\\\n/g, " ").replace(/\/\/.*$/gm, "");
const literals = [...seg.matchAll(/V\((\w+),\s*(\w+)\)/g)].map((m) => [m[1], m[2]]);
if (literals.length !== 9) throw new Error(`TYPEOF_LITERAL_LIST 对数不符: 期望 9 实得 ${literals.length}`);
const pascal = (x) => x.split("_").map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join("");

const RUST_HEAD = `
use v8_common::{LanguageMode, LookupHoistingMode};

/// typeof 字面量旗标 (原版 TestTypeOfFlags::LiteralFlag; TYPEOF_LITERAL_LIST 展开, 9 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LiteralFlag {
${literals.map(([n, lower], i) => "  /// 原版 k" + n + " (\"" + lower + "\")\n  " + pascal(n) + " = " + i + ",").join("\n")}
}

pub type TestTypeOfLiteralFlag = LiteralFlag;

impl LiteralFlag {
  /// 原版名 (TestTypeOfFlags::ToString)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
${literals.map(([n, lower]) => "      Self::" + pascal(n) + " => \"" + lower + "\",").join("\n")}
    }
  }

  #[inline]
  #[must_use]
  pub const fn encode(self) -> u8 {
    self as u8
  }

  #[inline]
  pub fn decode(raw: u8) -> crate::Result<Self> {
    TestTypeOfFlags::decode(raw)
  }
}

/// CreateArrayLiteralFlags (原版 class CreateArrayLiteralFlags)
pub struct CreateArrayLiteralFlags;

impl CreateArrayLiteralFlags {
  pub const FLAGS_MASK: u8 = 0b0001_1111;
  pub const FLAGS_SHIFT: u32 = 0;
  pub const FAST_CLONE_SUPPORTED_SHIFT: u32 = 5;
  pub const FAST_CLONE_SUPPORTED_BIT: u8 = 1 << 5;

  #[inline]
  #[must_use]
  pub const fn encode(use_fast_shallow_clone: bool, runtime_flags: u8) -> u8 {
    (runtime_flags & Self::FLAGS_MASK) | ((use_fast_shallow_clone as u8) << Self::FAST_CLONE_SUPPORTED_SHIFT)
  }
}

/// CreateObjectLiteralFlags (原版 class CreateObjectLiteralFlags)
pub struct CreateObjectLiteralFlags;

impl CreateObjectLiteralFlags {
  pub const FLAGS_MASK: u8 = 0b0001_1111;
  pub const FLAGS_SHIFT: u32 = 0;
  pub const FAST_CLONE_SUPPORTED_SHIFT: u32 = 5;
  pub const FAST_CLONE_SUPPORTED_BIT: u8 = 1 << 5;

  #[inline]
  #[must_use]
  pub const fn encode(runtime_flags: u8, fast_clone_supported: bool) -> u8 {
    (runtime_flags & Self::FLAGS_MASK) | ((fast_clone_supported as u8) << Self::FAST_CLONE_SUPPORTED_SHIFT)
  }
}

/// CreateClosureFlags (原版 class CreateClosureFlags)
pub struct CreateClosureFlags;

impl CreateClosureFlags {
  pub const PRETENURED_SHIFT: u32 = 0;
  pub const FAST_NEW_CLOSURE_SHIFT: u32 = 1;
  pub const PRETENURED_BIT: u8 = 1 << 0;
  pub const FAST_NEW_CLOSURE_BIT: u8 = 1 << 1;

  #[inline]
  #[must_use]
  pub const fn encode(pretenure: bool, is_function_scope: bool) -> u8 {
    (pretenure as u8) | (((!pretenure && is_function_scope) as u8) << 1)
  }
}

/// TestTypeOfFlags (原版 class TestTypeOfFlags)
pub struct TestTypeOfFlags;

impl TestTypeOfFlags {
  #[inline]
  #[must_use]
  pub const fn encode(literal_flag: LiteralFlag) -> u8 {
    literal_flag as u8
  }

  #[inline]
  pub fn decode(raw_flag: u8) -> crate::Result<LiteralFlag> {
    match raw_flag {
      0 => Ok(LiteralFlag::Number),
      1 => Ok(LiteralFlag::String),
      2 => Ok(LiteralFlag::Symbol),
      3 => Ok(LiteralFlag::Boolean),
      4 => Ok(LiteralFlag::BigInt),
      5 => Ok(LiteralFlag::Undefined),
      6 => Ok(LiteralFlag::Function),
      7 => Ok(LiteralFlag::Object),
      8 => Ok(LiteralFlag::Other),
      _ => Err(crate::Error::InvalidFlag(raw_flag)),
    }
  }

  #[inline]
  #[must_use]
  pub const fn to_string(literal_flag: LiteralFlag) -> &'static str {
    literal_flag.as_str()
  }
}

/// StoreLookupSlotFlags (原版 class StoreLookupSlotFlags)
pub struct StoreLookupSlotFlags;

impl StoreLookupSlotFlags {
  pub const LANGUAGE_MODE_SHIFT: u32 = 0;
  pub const LOOKUP_HOISTING_SHIFT: u32 = 1;

  #[inline]
  #[must_use]
  pub fn encode(language_mode: LanguageMode, lookup_hoisting_mode: LookupHoistingMode) -> u8 {
    let lang_val = (language_mode == LanguageMode::Strict) as u8;
    let hoist_val = (lookup_hoisting_mode == LookupHoistingMode::LegacySloppy) as u8;
    (lang_val & 1) | (hoist_val << 1)
  }

  #[inline]
  #[must_use]
  pub fn get_language_mode(flags: u8) -> LanguageMode {
    if (flags & 1) != 0 {
      LanguageMode::Strict
    } else {
      LanguageMode::Sloppy
    }
  }

  #[inline]
  #[must_use]
  pub fn is_lookup_hoisting_mode(flags: u8) -> bool {
    (flags & 2) != 0
  }
}

/// CreateArrayLiteral / CreateObjectLiteral 共用位域模块 (兼容模块别名)
pub mod literal_flags {
  pub const FLAGS_MASK: u8 = super::CreateArrayLiteralFlags::FLAGS_MASK;
  pub const FLAGS_SHIFT: u32 = super::CreateArrayLiteralFlags::FLAGS_SHIFT;
  pub const FAST_CLONE_SUPPORTED_SHIFT: u32 = super::CreateArrayLiteralFlags::FAST_CLONE_SUPPORTED_SHIFT;
  pub const FAST_CLONE_SUPPORTED_MASK: u8 = super::CreateArrayLiteralFlags::FAST_CLONE_SUPPORTED_BIT;

  #[inline]
  #[must_use]
  pub const fn encode(runtime_flags: u8, fast_clone_supported: bool) -> u8 {
    super::CreateArrayLiteralFlags::encode(fast_clone_supported, runtime_flags)
  }
}

/// CreateClosure 位域模块 (兼容模块别名)
pub mod closure_flags {
  pub const PRETENURED_SHIFT: u32 = super::CreateClosureFlags::PRETENURED_SHIFT;
  pub const FAST_NEW_CLOSURE_SHIFT: u32 = super::CreateClosureFlags::FAST_NEW_CLOSURE_SHIFT;
  pub const PRETENURED_BIT: u8 = super::CreateClosureFlags::PRETENURED_BIT;
  pub const FAST_NEW_CLOSURE_BIT: u8 = super::CreateClosureFlags::FAST_NEW_CLOSURE_BIT;

  #[inline]
  #[must_use]
  pub const fn encode(pretenure: bool, is_function_scope: bool) -> u8 {
    super::CreateClosureFlags::encode(pretenure, is_function_scope)
  }
}

/// StoreLookupSlot 位域模块 (兼容模块别名)
pub mod store_lookup_slot_flags {
  pub const LANGUAGE_MODE_SHIFT: u32 = super::StoreLookupSlotFlags::LANGUAGE_MODE_SHIFT;
  pub const LOOKUP_HOISTING_SHIFT: u32 = super::StoreLookupSlotFlags::LOOKUP_HOISTING_SHIFT;

  #[inline]
  #[must_use]
  pub fn encode(language_mode: super::LanguageMode, lookup_hoisting_mode: super::LookupHoistingMode) -> u8 {
    super::StoreLookupSlotFlags::encode(language_mode, lookup_hoisting_mode)
  }

  #[inline]
  #[must_use]
  pub fn get_language_mode(flags: u8) -> super::LanguageMode {
    super::StoreLookupSlotFlags::get_language_mode(flags)
  }

  #[inline]
  #[must_use]
  pub fn is_lookup_hoisting_mode(flags: u8) -> bool {
    super::StoreLookupSlotFlags::is_lookup_hoisting_mode(flags)
  }
}

/// try-finally 续延 token (原版 enum class TryFinallyContinuationToken : int)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TryFinallyContinuationToken {
  /// 落穿 (跳转表 fallthrough 槽位)
  FallthroughToken = -1,
  /// 重抛
  RethrowToken = 0,
}`;
const stripped = stripComments(hSrc);
const blocks = classBlocks(stripped);
const CLASS_TABLES = [["CreateArrayLiteralFlags", "CREATE_ARRAY_LITERAL_FLAGS_DECLARATIONS", 1, 0], ["CreateObjectLiteralFlags", "CREATE_OBJECT_LITERAL_FLAGS_DECLARATIONS", 1, 0], ["CreateClosureFlags", "CREATE_CLOSURE_FLAGS_DECLARATIONS", 1, 0], ["TestTypeOfFlags", "TEST_TYPEOF_FLAGS_DECLARATIONS", 4, 0], ["StoreLookupSlotFlags", "STORE_LOOKUP_SLOT_FLAGS_DECLARATIONS", 3, 0]];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}



let rust = banner("src/interpreter/bytecode-flags-and-tokens.h.js", "src/interpreter/bytecode-flags-and-tokens.{h,cc}", "");

rust += RUST_HEAD;

for (const { constName, decls, className } of emitTables) {
  rust += `
/// 原版 class/struct ${className} 成员声明 (机械 1:1 提取, ${decls.length} 条)
pub const ${constName}: &[&str] = &[
`;
  for (const d of decls) rust += `  "[${d.vis}/${d.kind}] ${d.sig}",
`;
  rust += `];
`;
}

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(CREATE_ARRAY_LITERAL_FLAGS_DECLARATIONS.len() == 1);
  assert!(TEST_TYPEOF_FLAGS_DECLARATIONS.len() == 4);
  assert!(STORE_LOOKUP_SLOT_FLAGS_DECLARATIONS.len() == 3);
  assert!(literal_flags::FAST_CLONE_SUPPORTED_SHIFT == 5);
  assert!(LiteralFlag::BigInt as u8 == 4);
  assert!(LiteralFlag::Other.as_str().len() == 5);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_flags_and_tokens.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
