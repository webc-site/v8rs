#!/usr/bin/env bun
// tool/src/baseline-assembler-arm64-inl.h.js
// 机械提取 src/baseline/arm64/baseline-assembler-arm64-inl.h 的arm64 scratch 栈与推送/弹出助手骨架
// → v8-interpreter/src/baseline_arm64_assembler.rs
import { readV8, writeSrc, banner } from "../../../lib.js";
const hSrc = readV8("src/baseline/arm64/baseline-assembler-arm64-inl.h");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "class BaselineAssembler::ScratchRegisterScope {", "ScratchRegisterScope \u7279\u5316"],
  [hSrc, "constexpr int instructions_per_jump_target = 1;", "\u8df3\u8f6c\u76ee\u6807\u6307\u4ee4\u6570 (\u975e\u538b\u7f29\u6307\u9488)"],
  [hSrc, "constexpr int instructions_per_label = 1 + instructions_per_jump_target;", "label \u6307\u4ee4\u6570\u516c\u5f0f"],
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
const inlBlocks = classBlocks(stripComments(hSrc));
const scopeBlock = inlBlocks.find((b) => b.name === "BaselineAssembler" && b.body.includes("AcquireScratch"));
if (!scopeBlock) throw new Error("类缺失: ScratchRegisterScope (arm64 特化)");
const scopeDecls = members(scopeBlock.body);
if (scopeDecls.length !== 6) throw new Error(`ScratchRegisterScope 对数不符: 期望 6 实得 ${scopeDecls.length}`);
// 推送/弹出助手模板计数 (CountPushHelper 3 + PushAllHelper 5 + PopAllHelper 3)
const helperCount = inlBlocks.filter((b) => /Helper$/.test(b.name)).length;
if (helperCount !== 11) throw new Error(`助手模板对数不符: 期望 11 实得 ${helperCount}`);

const RUST_HEAD = `
/// arm64 ScratchRegisterScope 特化成员声明 (机械 1:1)
pub const ARM64_SCRATCH_REGISTER_SCOPE_DECLARATIONS: &[&str] = &[
${scopeDecls.map((d) => "  \"[" + d.vis + "/" + d.kind + "] " + d.sig + "\",").join("\n")}
];

/// 跳转目标预留指令数 (V8_COMPRESS_POINTERS 开启时为 0, 本目标按非压缩取 1;
/// 原版 #if 分支, 修订阶段按构建特性二选一)
pub const INSTRUCTIONS_PER_JUMP_TARGET: i32 = 1;
/// 每 label 指令数 (原版 instructions_per_label = 1 + instructions_per_jump_target)
pub const INSTRUCTIONS_PER_LABEL: i32 = 1 + INSTRUCTIONS_PER_JUMP_TARGET;

/// 推送/弹出助手模板登记 (CountPushHelper×3 + PushAllHelper×5 + PopAllHelper×3)
pub const HELPER_TEMPLATE_COUNT: usize = 11;`;
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



let rust = banner("src/baseline/arm64/baseline-assembler-arm64-inl.h.js", "src/baseline/arm64/baseline-assembler-arm64-inl.h", "");

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
  assert!(ARM64_SCRATCH_REGISTER_SCOPE_DECLARATIONS.len() == 6);
  assert!(HELPER_TEMPLATE_COUNT == 11);
  assert!(INSTRUCTIONS_PER_LABEL == 2);
};
`;

const p = writeSrc("v8-baseline", "baseline_arm64_assembler.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
