#!/usr/bin/env bun
// tool/src/interpreter/bytecode-label.h.js
// 机械提取 v8/src/interpreter/bytecode-label.{h,cc} 的标签体系骨架
// → v8-interpreter/src/bytecode_label.rs
import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/interpreter/bytecode-label.h");
const ccSrc = readV8("src/interpreter/bytecode-label.cc");

for (const [src, needle, name] of [
  [hSrc, "static const size_t kInvalidOffset = static_cast<size_t>(-1);", "kInvalidOffset"],
  [hSrc, "class V8_EXPORT_PRIVATE BytecodeLoopHeader final {", "BytecodeLoopHeader"],
  [hSrc, "class V8_EXPORT_PRIVATE BytecodeLabel final {", "BytecodeLabel"],
  [hSrc, "class V8_EXPORT_PRIVATE BytecodeLabels {", "BytecodeLabels"],
  [ccSrc, "BytecodeLabel* BytecodeLabels::New()", "BytecodeLabels::New 定义"],
]) {
  if (!src.includes(needle)) throw new Error(`结构锚缺失: ${name}`);
}

// —— 机械提取: 去注释 + 类块 + 成员声明 ——
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
const loopHeader = members(classBlocks(stripped).find((b) => b.name === "BytecodeLoopHeader").body);
const label = members(classBlocks(stripped).find((b) => b.name === "BytecodeLabel").body);
const labels = members(classBlocks(stripped).find((b) => b.name === "BytecodeLabels").body);
if (loopHeader.length !== 5) throw new Error(`BytecodeLoopHeader 对数不符: 期望 5 实得 ${loopHeader.length}`);
if (label.length !== 9) throw new Error(`BytecodeLabel 对数不符: 期望 9 实得 ${label.length}`);
if (labels.length !== 9) throw new Error(`BytecodeLabels 对数不符: 期望 9 实得 ${labels.length}`);

let rust = banner("src/interpreter/bytecode-label.h.js", "src/interpreter/bytecode-label.{h,cc}", "字节码标签/循环头/标签组骨架");

rust += `
/// 无效偏移哨兵 (原版 kInvalidOffset = static_cast<size_t>(-1))
pub const INVALID_OFFSET: usize = usize::MAX;

/// 原版 class BytecodeLoopHeader 成员声明 (循环头, 先绑后跳, 位置恒已知)
pub const BYTECODE_LOOP_HEADER_DECLARATIONS: &[&str] = &[
`;
for (const d of loopHeader) rust += `  "[${d.vis}/${d.kind}] ${d.sig}",\n`;
rust += `];

/// 原版 class BytecodeLabel 成员声明 (前向跳转目标, 至多一个引用跳转)
pub const BYTECODE_LABEL_DECLARATIONS: &[&str] = &[
`;
for (const d of label) rust += `  "[${d.vis}/${d.kind}] ${d.sig}",\n`;
rust += `];

/// 原版 class BytecodeLabels 成员声明 (多重跳转分支目标组)
pub const BYTECODE_LABELS_DECLARATIONS: &[&str] = &[
`;
for (const d of labels) rust += `  "[${d.vis}/${d.kind}] ${d.sig}",\n`;
rust += `];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_LOOP_HEADER_DECLARATIONS.len() == 5);
  assert!(BYTECODE_LABEL_DECLARATIONS.len() == 9);
  assert!(BYTECODE_LABELS_DECLARATIONS.len() == 9);
  assert!(INVALID_OFFSET == usize::MAX);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_label.rs", rust);
console.log(`written ${p} (${loopHeader.length + label.length + labels.length} declarations)`);
