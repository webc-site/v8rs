#!/usr/bin/env bun
// tool/src/interpreter/interpreter.h.js
// 机械提取 src/interpreter/interpreter.{h,cc} 的解释器本位: 分发表常数/索引公式与骨架 (含 InterpreterCompilationJob 登记)
// → v8-interpreter/src/interpreter_core.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/interpreter.h");
const ccSrc = readV8("src/interpreter/interpreter.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "static const int kNumberOfWideVariants = BytecodeOperands::kOperandScaleCount;", "kNumberOfWideVariants"],
  [hSrc, "static const int kDispatchTableSize = kNumberOfWideVariants * (kMaxUInt8 + 1);", "kDispatchTableSize \u516c\u5f0f"],
  [hSrc, "static const int kNumberOfBytecodes = static_cast<int>(Bytecode::kLast) + 1;", "kNumberOfBytecodes"],
  [hSrc, "struct BytecodeHandlerData {", "BytecodeHandlerData"],
  [ccSrc, "static const size_t kEntriesPerOperandScale = 1u << kBitsPerByte;", "kEntriesPerOperandScale"],
  [ccSrc, "size_t Interpreter::GetDispatchTableIndex(Bytecode bytecode,", "GetDispatchTableIndex \u5b9a\u4e49"],
  [ccSrc, "class InterpreterCompilationJob final : public UnoptimizedCompilationJob {", "InterpreterCompilationJob"],
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
// 分发表常数机械求值: kBitsPerByte=8, kMaxUInt8=255, kOperandScaleCount=3, kBytecodeCount=217
const kNumberOfWideVariants = 3;
const kEntriesPerOperandScale = 256;
const kDispatchTableSize = kNumberOfWideVariants * 256;
const kNumberOfBytecodes = 217;
const stripped = stripComments(hSrc);
const blocks = classBlocks(stripped);
const CLASS_TABLES = [["BytecodeHandlerData", "BYTECODE_HANDLER_DATA_DECLARATIONS", 7, 0], ["Interpreter", "INTERPRETER_DECLARATIONS", 20, 0]];
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
use crate::bytecode_operands::OperandScale;
use crate::bytecodes::{Bytecode, BYTECODE_COUNT};

/// 宽变体数 (原版 kNumberOfWideVariants = kOperandScaleCount)
pub const NUMBER_OF_WIDE_VARIANTS: usize = 3;
/// 每刻度分发表项数 (原版 kEntriesPerOperandScale = 1 << 8)
pub const ENTRIES_PER_OPERAND_SCALE: usize = 256;
/// 分发表总容量 (原版 kDispatchTableSize = 3 * 256)
pub const DISPATCH_TABLE_SIZE: usize = 768;
/// 字节码总数 (原版 kNumberOfBytecodes = kLast + 1)
pub const NUMBER_OF_BYTECODES: usize = BYTECODE_COUNT;

/// 分发表索引 = 字节码值 + 刻度索引 * 256 (原版 GetDispatchTableIndex)
#[inline]
#[must_use]
pub const fn dispatch_table_index(bytecode: Bytecode, operand_scale: OperandScale) -> usize {
  bytecode as usize + operand_scale.as_index() * ENTRIES_PER_OPERAND_SCALE
}

/// handler 元数据 (原版 struct BytecodeHandlerData 字段面)
#[derive(Debug, Clone, Copy)]
pub struct BytecodeHandlerDataSkeleton {
  pub bytecode: Bytecode,
  pub operand_scale: OperandScale,
  /// 默认 kNone (原版 implicit_register_use = ImplicitRegisterUse::kNone)
  pub implicit_register_use: crate::bytecode_operands::ImplicitRegisterUse,
  pub made_call: bool,
  pub reloaded_frame_ptr: bool,
  /// 默认 true
  pub bytecode_array_valid: bool,
}
`;

let rust = banner("src/interpreter/interpreter.h.js", "src/interpreter/interpreter.{h,cc}", "");

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
  assert!(INTERPRETER_DECLARATIONS.len() == 20);
  assert!(BYTECODE_HANDLER_DATA_DECLARATIONS.len() == 7);
  assert!(DISPATCH_TABLE_SIZE == 768);
  assert!(NUMBER_OF_BYTECODES == 217);
  assert!(dispatch_table_index(crate::bytecodes::Bytecode::Wide, OperandScale::Single) == 0);
  assert!(dispatch_table_index(crate::bytecodes::Bytecode::Wide, OperandScale::Double) == 256);
};
`;

const p = writeSrc("v8-interpreter", "interpreter_core.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
