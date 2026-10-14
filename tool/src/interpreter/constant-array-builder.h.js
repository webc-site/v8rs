#!/usr/bin/env bun
// tool/src/interpreter/constant-array-builder.h.js
// 机械提取 src/interpreter/constant-array-builder.{h,cc} 的常量池数组构建器: 分片容量/单例条目/Tag 枚举与骨架
// → v8-interpreter/src/constant_array_builder.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/constant-array-builder.h");
const ccSrc = readV8("src/interpreter/constant-array-builder.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "#define SINGLETON_CONSTANT_ENTRY_TYPES(V)", "SINGLETON \u5b8f"],
  [hSrc, "static const size_t k8BitCapacity = 1u << kBitsPerByte;", "k8BitCapacity"],
  [hSrc, "static const size_t k16BitCapacity = (1u << 2 * kBitsPerByte) - k8BitCapacity;", "k16BitCapacity"],
  [hSrc, "kDeferred,", "Tag \u679a\u4e3e\u5e8f\u9996"],
  [ccSrc, "#include \"src/interpreter/constant-array-builder.h\"", "cc \u5f15\u5165\u951a"],
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
// —— SINGLETON_CONSTANT_ENTRY_TYPES 机械展开 (8 项) ——
function rawMacroBody(src, name) {
  const re = new RegExp(`#define\\s+${name}\\(`);
  const m = re.exec(src);
  if (!m) throw new Error(`宏缺失: ${name}`);
  const allLines = src.slice(m.index).split("\n");
  const head = allLines[0].slice(allLines[0].indexOf(")") + 1);
  const body = [];
  let start = 1;
  if (head.trim()) {
    body.push(head.trimEnd().endsWith("\\") ? head.trimEnd().slice(0, -1) : head);
    if (!head.trimEnd().endsWith("\\")) start = allLines.length;
  }
  for (let i = start; i < allLines.length; i++) {
    const line = allLines[i];
    const cont = line.endsWith("\\");
    body.push(cont ? line.slice(0, -1) : line);
    if (!cont) break;
  }
  return body.join("\n").replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");
}
const singletonBody = rawMacroBody(hSrc, "SINGLETON_CONSTANT_ENTRY_TYPES");
const singletons = [...singletonBody.matchAll(/V\((\w+),\s*(\w+)\)/g)].map((m) => [m[1], m[2]]);
if (singletons.length !== 8) throw new Error(`SINGLETON 对数不符: 期望 8 实得 ${singletons.length}`);
// cc 侧 Tag 消费锚: SINGLETON 项经 X-macro 进入 Tag 枚举, cc 仅显式消费基础 8 项
for (const t of ["kSmi", "kHeapNumber", "kRawString", "kConsString", "kBigInt", "kScope", "kHandle", "kDeferred"]) {
  if (!ccSrc.includes(`Entry::Tag::${t}`)) throw new Error(`cc 缺 Tag 消费锚: ${t}`);
}
// Entry::Tag 全枚举: 显式 10 项 + SINGLETON 展开 8 项 (头文件顺序)
const tagBase = ["Deferred", "Handle", "Smi", "RawString", "ConsString", "HeapNumber", "BigInt", "Scope", "UninitializedJumpTableSmi", "JumpTableSmi"];
const tags = [...tagBase, ...singletons.map(([n]) => n)];
if (tags.length !== 18) throw new Error(`Tag 对数不符: 期望 18 实得 ${tags.length}`);

const pascal = (x) => x.split("_").map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join("");

const RUST_HEAD = `
/// 8 位操作数分片容量 (原版 k8BitCapacity = 1 << 8)
pub const K8BIT_CAPACITY: usize = 256;
/// 16 位操作数分片容量 (原版 k16BitCapacity = (1 << 16) - 256)
pub const K16BIT_CAPACITY: usize = 65280;
/// 32 位操作数分片容量 (原版 k32BitCapacity = kMaxUInt32 - k16 - k8 + 1)
pub const K32BIT_CAPACITY: usize = 4294901760;

/// 单例常量条目类型 (原版 SINGLETON_CONSTANT_ENTRY_TYPES 展开, 8 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SingletonConstantEntryType {
${singletons.map(([n, r]) => "  /// 运行时根: " + r + "\n  " + pascal(n) + ",").join("\n")}
}

/// 条目类别 (原版 Entry::Tag, 10 显式 + 8 单例, 顺序敏感)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EntryTag {
${tags.map((t, i) => "  /// 原版 k" + t + "\n  " + pascal(t) + " = " + i + ",").join("\n")}
}
`; // RUST_HEAD
const stripped = stripComments(hSrc);
const blocks = classBlocks(stripped);
const CLASS_TABLES = [["ConstantArrayBuilder", "CONSTANT_ARRAY_BUILDER_DECLARATIONS", 32, 0], ["Entry", "CONSTANT_ARRAY_ENTRY_DECLARATIONS", 16, 0]];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}



let rust = banner("src/interpreter/constant-array-builder.h.js", "src/interpreter/constant-array-builder.{h,cc}", "");

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
  assert!(CONSTANT_ARRAY_BUILDER_DECLARATIONS.len() == 32);
  assert!(CONSTANT_ARRAY_ENTRY_DECLARATIONS.len() == 16);
  assert!(K8BIT_CAPACITY == 256);
  assert!(K16BIT_CAPACITY == 65280);
  assert!(K32BIT_CAPACITY == 4294901760);
};
`;

const p = writeSrc("v8-interpreter", "constant_array_builder.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
