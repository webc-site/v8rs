#!/usr/bin/env bun
// tool/src/interpreter/bytecode-generator.h.js
// 机械提取 src/interpreter/bytecode-generator.{h,cc} 的字节码生成器: TypeHint 位族/落穿/累加器模式枚举与 254 条成员签名骨架 (cc 9237 行仅登记)
// → v8-interpreter/src/bytecode_generator.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/bytecode-generator.h");
const ccSrc = readV8("src/interpreter/bytecode-generator.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "enum TypeHint : uint8_t {", "TypeHint \u4f4d\u65cf"],
  [hSrc, "kString = kInternalizedString | (1 << 2),", "kString \u7ec4\u5408\u516c\u5f0f"],
  [hSrc, "enum class TestFallthrough { kThen, kElse, kNone };", "TestFallthrough \u679a\u4e3e"],
  [hSrc, "enum class AccumulatorPreservingMode { kNone, kPreserve };", "AccumulatorPreservingMode \u679a\u4e3e"],
  [hSrc, "static constexpr ToBooleanMode ToBooleanModeFromTypeHint(TypeHint type_hint) {", "ToBooleanModeFromTypeHint \u516c\u5f0f"],
  [ccSrc, "#include \"src/interpreter/bytecode-generator.h\"", "cc \u5f15\u5165\u951a (9237 \u884c, \u8bed\u4e49\u4fee\u8ba2\u9636\u6bb5\u5904\u7406)"],
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

const stripped = stripComments(hSrc);
const blocks = classBlocks(stripped);
const CLASS_TABLES = [["BytecodeGenerator", "BYTECODE_GENERATOR_DECLARATIONS", 254, 0]];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}

const RUST_HEAD = `
use crate::bytecode_array_builder::ToBooleanMode;

/// 表达式类型提示位族 (原版 enum TypeHint : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TypeHint {
  /// 布尔 (1 << 0)
  Boolean = 1 << 0,
  /// 内部化字符串 (1 << 1)
  InternalizedString = 1 << 1,
  /// 字符串 = 内部化 | (1 << 2)
  String = (1 << 1) | (1 << 2),
  /// 布尔 | 字符串
  Any = (1 << 0) | (1 << 1) | (1 << 2),
  /// 未知 (全位)
  Unknown = 0xFF,
}

impl TypeHint {
  /// hint2 是否与 hint1 相同或为其子型 (原版 IsSameOrSubTypeHint: hint1 == hint1 | hint2)
  #[inline]
  #[must_use]
  pub const fn is_same_or_sub_type(hint1: Self, hint2: Self) -> bool {
    hint1 as u8 == (hint1 as u8 | hint2 as u8)
  }

  /// 是否字符串型 (原版 IsStringTypeHint)
  #[inline]
  #[must_use]
  pub const fn is_string_type(hint: Self) -> bool {
    Self::is_same_or_sub_type(Self::String, hint)
  }

  /// 类型提示 -> ToBoolean 模式 (原版 ToBooleanModeFromTypeHint)
  #[inline]
  #[must_use]
  pub const fn to_boolean_mode(self) -> ToBooleanMode {
    match self {
      Self::Boolean => ToBooleanMode::AlreadyBoolean,
      _ => ToBooleanMode::ConvertToBoolean,
    }
  }
}

/// 比较落穿分支 (原版 enum class TestFallthrough)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TestFallthrough {
  Then = 0,
  Else = 1,
  None = 2,
}

/// 累加器保存模式 (原版 enum class AccumulatorPreservingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AccumulatorPreservingMode {
  None = 0,
  Preserve = 1,
}
`;

let rust = banner("src/interpreter/bytecode-generator.h.js", "src/interpreter/bytecode-generator.{h,cc}", "");

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
  assert!(BYTECODE_GENERATOR_DECLARATIONS.len() == 254);
  assert!(TypeHint::String as u8 == 6);
  assert!(TypeHint::Any as u8 == 7);
  assert!(TypeHint::is_same_or_sub_type(TypeHint::Any, TypeHint::Boolean));
  assert!(!TypeHint::is_same_or_sub_type(TypeHint::Boolean, TypeHint::Any));
  assert!(TypeHint::is_string_type(TypeHint::InternalizedString));
  assert!(matches!(TypeHint::Boolean.to_boolean_mode(), ToBooleanMode::AlreadyBoolean));
};
`;

const p = writeSrc("v8-interpreter", "bytecode_generator.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
