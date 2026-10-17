#!/usr/bin/env bun
// tool/src/interpreter/bytecode-array-builder.h.js
// 机械提取 src/interpreter/bytecode-array-builder.{h,cc} 的字节码数组构建器: ToBooleanMode/NilValue 枚举与 221 条成员签名骨架 (cc 1886 行仅登记)
// → v8-interpreter/src/bytecode_array_builder.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/bytecode-array-builder.h");
const ccSrc = readV8("src/interpreter/bytecode-array-builder.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "enum class ToBooleanMode {", "ToBooleanMode \u679a\u4e3e"],
  [hSrc, "enum NilValue { kNullValue, kUndefinedValue };", "NilValue \u679a\u4e3e"],
  [hSrc, "class V8_EXPORT_PRIVATE BytecodeArrayBuilder final {", "BytecodeArrayBuilder"],
  [ccSrc, "#include \"src/interpreter/bytecode-array-builder.h\"", "cc \u5f15\u5165\u951a (1886 \u884c, \u8bed\u4e49\u4fee\u8ba2\u9636\u6bb5\u5904\u7406)"],
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
const CLASS_TABLES = [["BytecodeArrayBuilder", "BYTECODE_ARRAY_BUILDER_DECLARATIONS", 221, 0], ["ThrowTrackingScope", "THROW_TRACKING_SCOPE_DECLARATIONS", 4, 0]];
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
/// ToBoolean 转换模式 (原版 enum class ToBooleanMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ToBooleanMode {
  /// 对累加器执行 ToBoolean 转换
  ConvertToBoolean = 0,
  /// 累加器已是布尔值
  AlreadyBoolean = 1,
}

/// JS 两种 nil (原版 enum NilValue)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NilValue {
  /// null
  NullValue = 0,
  /// undefined
  UndefinedValue = 1,
}
`;

let rust = banner("src/interpreter/bytecode-array-builder.h.js", "src/interpreter/bytecode-array-builder.{h,cc}", "");

rust += RUST_HEAD;

for (const { constName, decls, className } of emitTables) {
  rust += `
/// 原版 class/struct ${className} 成员声明 (机械 1:1 提取, ${decls.length} 条)
pub const ${constName}: &[&str] = &[
`;
  for (const d of decls) {
    if (d.sig.includes("\\")) {
      rust += `  r"[${d.vis}/${d.kind}] ${d.sig}",\n`;
    } else {
      rust += `  "[${d.vis}/${d.kind}] ${d.sig}",\n`;
    }
  }
  rust += `];
`;
}

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_ARRAY_BUILDER_DECLARATIONS.len() == 221);
  assert!(THROW_TRACKING_SCOPE_DECLARATIONS.len() == 4);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_array_builder.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
