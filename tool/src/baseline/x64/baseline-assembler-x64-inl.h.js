#!/usr/bin/env bun
// tool/src/baseline-assembler-x64-inl.h.js
// 机械提取 src/baseline/x64/baseline-assembler-x64-inl.h 的x64 scratch 寄存器组与推送助手骨架
// → v8-interpreter/src/baseline_x64_assembler.rs
import { readV8, writeSrc, banner } from "../../../lib.js";
const hSrc = readV8("src/baseline/x64/baseline-assembler-x64-inl.h");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "static constexpr Register kScratchRegisters[] = {r8, r9, r11, r12, r15};", "kScratchRegisters"],
  [hSrc, "static constexpr int kNumScratchRegisters = arraysize(kScratchRegisters);", "kNumScratchRegisters"],
  [hSrc, "class BaselineAssembler::ScratchRegisterScope {", "ScratchRegisterScope \u7279\u5316"],
  [hSrc, "Avoid using kScratchRegister(==r10)", "r10 \u51b2\u7a81\u6ce8\u91ca\u951a"],
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
// 类块名修正: class BaselineAssembler::ScratchRegisterScope 的 \w+ 捕获为 BaselineAssembler
const inlBlocks = classBlocks(stripComments(hSrc));
const scopeBlock = inlBlocks.find((b) => b.name === "BaselineAssembler" && b.body.includes("AcquireScratch"));
if (!scopeBlock) throw new Error("类缺失: ScratchRegisterScope (x64 特化)");
const scopeDecls = members(scopeBlock.body);
if (scopeDecls.length !== 6) throw new Error(`ScratchRegisterScope 对数不符: 期望 6 实得 ${scopeDecls.length}`);

const RUST_HEAD = `
/// x64 scratch 寄存器组 (原版 detail::kScratchRegisters; 避开 r10 — 宏汇编器保留)
/// 名单按 v8-codegen-x64 寄存器枚举对齐, 修订阶段接入 Register 类型
pub const SCRATCH_REGISTERS: [&str; 5] = ["r8", "r9", "r11", "r12", "r15"];

/// scratch 寄存器数 (原版 kNumScratchRegisters)
pub const NUM_SCRATCH_REGISTERS: usize = 5;

/// x64 ScratchRegisterScope 特化成员声明 (机械 1:1)
pub const X64_SCRATCH_REGISTER_SCOPE_DECLARATIONS: &[&str] = &[
${scopeDecls.map((d) => "  \"[" + d.vis + "/" + d.kind + "] " + d.sig + "\",").join("\n")}
];`;
const stripped = stripComments(hSrc);
const blocks = classBlocks(stripped);
const CLASS_TABLES = [];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}



let rust = banner("src/baseline/x64/baseline-assembler-x64-inl.h.js", "src/baseline/x64/baseline-assembler-x64-inl.h", "");

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
  assert!(X64_SCRATCH_REGISTER_SCOPE_DECLARATIONS.len() == 6);
  assert!(SCRATCH_REGISTERS.len() == 5);
  assert!(NUM_SCRATCH_REGISTERS == 5);
};
`;

const p = writeSrc("v8-baseline", "baseline_x64_assembler.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
