#!/usr/bin/env bun
// tool/src/interpreter/interpreter-generator.h.js
// 机械提取 src/interpreter/interpreter-generator.{h,cc} 的handler 生成入口注册 (cc 3702 行仅登记, 语义属 CodeStubAssembler/TSA 侧)
// → v8-interpreter/src/interpreter_generator.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/interpreter-generator.h");
const ccSrc = readV8("src/interpreter/interpreter-generator.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "extern void GenerateBytecodeHandler(compiler::CodeAssemblerState* state,", "GenerateBytecodeHandler \u58f0\u660e"],
  [hSrc, "extern void GenerateBytecodeHandlerTSA(compiler::turboshaft::PipelineData* data,", "GenerateBytecodeHandlerTSA \u58f0\u660e"],
  [ccSrc, "void GenerateBytecodeHandler(compiler::CodeAssemblerState* state,", "GenerateBytecodeHandler \u5b9a\u4e49"],
  [ccSrc, "void GenerateBytecodeHandlerTSA(compiler::turboshaft::PipelineData* data,", "GenerateBytecodeHandlerTSA \u5b9a\u4e49"],
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

const RUST_HEAD = `
/// 两个生成入口 (原版 extern 函数, 机械登记; 语义属 CodeStubAssembler/Turboshaft 生成器族)
pub const ENTRY_SIGNATURES: [&str; 2] = [
  "extern void GenerateBytecodeHandler(compiler::CodeAssemblerState* state, Bytecode bytecode, OperandScale operand_scale);",
  "extern void GenerateBytecodeHandlerTSA(compiler::turboshaft::PipelineData* data, Isolate* isolate, compiler::turboshaft::Graph& graph, Zone* zone, Bytecode bytecode, OperandScale operand_scale);",
];

/// cc 侧生成函数定义锚 (3702 行, 每字节码 handler 的 DoXxxGenerator 语义修订阶段处理)
pub const CC_DEFINITIONS: [&str; 2] = [
  "void GenerateBytecodeHandler(compiler::CodeAssemblerState* state,",
  "void GenerateBytecodeHandlerTSA(compiler::turboshaft::PipelineData* data,",
];
`;

let rust = banner("src/interpreter/interpreter-generator.h.js", "src/interpreter/interpreter-generator.{h,cc}", "");

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
  assert!(ENTRY_SIGNATURES.len() == 2);
  assert!(CC_DEFINITIONS.len() == 2);
};
`;

const p = writeSrc("v8-interpreter", "interpreter_generator.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
