#!/usr/bin/env bun
// tool/src/interpreter/bytecode-register-optimizer.h.js
// 机械提取 src/interpreter/bytecode-register-optimizer.{h,cc} 的寄存器传输优化器骨架与等价类常数
// → v8-interpreter/src/bytecode_register_optimizer.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/bytecode-register-optimizer.h");
const ccSrc = readV8("src/interpreter/bytecode-register-optimizer.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "static const uint32_t kInvalidEquivalenceId;", "kInvalidEquivalenceId \u58f0\u660e"],
  [ccSrc, "enum class VariableHintMode { kDefinitelyHasVariable, kMightHaveVariable };", "VariableHintMode \u679a\u4e3e"],
  [hSrc, "virtual void EmitLdar(Register input) = 0;", "BytecodeWriter \u7eaf\u865a"],
  [ccSrc, "const uint32_t BytecodeRegisterOptimizer::kInvalidEquivalenceId = kMaxUInt32;", "kInvalidEquivalenceId \u5b9a\u4e49"],
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
const CLASS_TABLES = [["BytecodeRegisterOptimizer", "BYTECODE_REGISTER_OPTIMIZER_DECLARATIONS", 59, 0], ["BytecodeWriter", "BYTECODE_WRITER_DECLARATIONS", 7, 0]];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}

let rust = banner("src/interpreter/bytecode-register-optimizer.h.js", "src/interpreter/bytecode-register-optimizer.{h,cc}", "");

rust += `
/// 无效等价类 id (原版 kInvalidEquivalenceId = kMaxUInt32)
pub const INVALID_EQUIVALENCE_ID: u32 = u32::MAX;

/// 寄存器变量持有确性 (原版 enum class VariableHintMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VariableHintMode {
  /// 确定持有变量
  DefinitelyHasVariable = 0,
  /// 可能持有变量
  MightHaveVariable = 1,
}

/// 字节码写出回调 trait 骨架 (原版 class BytecodeWriter 纯虚三发射)
pub trait BytecodeWriter {
  fn emit_ldar(&mut self, input: i32);
  fn emit_star(&mut self, output: i32);
  fn emit_mov(&mut self, input: i32, output: i32);
}
`;

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
  assert!(BYTECODE_REGISTER_OPTIMIZER_DECLARATIONS.len() == 59);
  assert!(BYTECODE_WRITER_DECLARATIONS.len() == 7);
  assert!(INVALID_EQUIVALENCE_ID == u32::MAX);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_register_optimizer.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
