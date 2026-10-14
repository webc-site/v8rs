#!/usr/bin/env bun
// tool/src/interpreter/block-coverage-builder.h.js
// 机械提取 v8/src/interpreter/block-coverage-builder.h 的覆盖槽常量与成员签名
// → v8-interpreter/src/block_coverage_builder.rs
import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/interpreter/block-coverage-builder.h");

// 结构锚
for (const [needle, name] of [
  ["static constexpr int kNoCoverageArraySlot = -1;", "kNoCoverageArraySlot"],
  ["int AllocateBlockCoverageSlot(ZoneObject* node, SourceRangeKind kind)", "AllocateBlockCoverageSlot 签名"],
  ["void IncrementBlockCounter(int coverage_array_slot)", "IncrementBlockCounter 签名"],
  ["const ZoneVector<SourceRange>& slots() const", "slots 存取器"],
]) {
  if (!hSrc.includes(needle)) throw new Error(`结构锚缺失: ${name}`);
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

const blocks = classBlocks(stripComments(hSrc));
const cls = blocks.find((b) => b.name === "BlockCoverageBuilder");
if (!cls) throw new Error("类缺失: BlockCoverageBuilder");
const decls = members(cls.body);
const methods = decls.filter((d) => d.kind === "method");
if (methods.length !== 7) throw new Error(`方法对数不符: 期望 7 实得 ${methods.length}`);

let rust = banner("src/interpreter/block-coverage-builder.h.js", "src/interpreter/block-coverage-builder.h", "块覆盖率槽位分配器骨架");

rust += `
/// 无覆盖槽哨兵 (原版 kNoCoverageArraySlot)
pub const NO_COVERAGE_ARRAY_SLOT: i32 = -1;

/// 原版 class BlockCoverageBuilder 成员声明 (机械 1:1 提取, ${decls.length} 条)
pub const BLOCK_COVERAGE_BUILDER_DECLARATIONS: &[&str] = &[
`;
for (const d of decls) rust += `  "[${d.vis}/${d.kind}] ${d.sig}",\n`;
rust += `];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BLOCK_COVERAGE_BUILDER_DECLARATIONS.len() == ${decls.length});
  assert!(NO_COVERAGE_ARRAY_SLOT == -1);
};
`;

const p = writeSrc("v8-interpreter", "block_coverage_builder.rs", rust);
console.log(`written ${p} (${decls.length} declarations)`);
