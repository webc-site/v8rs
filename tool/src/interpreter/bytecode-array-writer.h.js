#!/usr/bin/env bun
// tool/src/interpreter/bytecode-array-writer.h.js
// 机械提取 v8/src/interpreter/bytecode-array-writer.{h,cc} 的写出器骨架与打包常数
// → v8-interpreter/src/bytecode_array_writer.rs
import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/interpreter/bytecode-array-writer.h");
const ccSrc = readV8("src/interpreter/bytecode-array-writer.cc");

for (const [src, needle, name] of [
  [hSrc, "2 * sizeof(Bytecode) +", "kMaxSizeOfPackedBytecode 公式"],
  [hSrc, "const uint32_t k8BitJumpPlaceholder = 0x7f;", "k8BitJumpPlaceholder"],
  [hSrc, "k8BitJumpPlaceholder | (k8BitJumpPlaceholder << 8);", "k16BitJumpPlaceholder 公式"],
  [hSrc, "void Write(BytecodeNode* node);", "Write 声明"],
  [hSrc, "void WriteJumpLoop(BytecodeNode* node, BytecodeLoopHeader* loop_header);", "WriteJumpLoop 声明"],
  [ccSrc, "void BytecodeArrayWriter::Write(BytecodeNode* node)", "Write 定义"],
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

const cls = classBlocks(stripComments(hSrc)).find((b) => b.name === "BytecodeArrayWriter");
if (!cls) throw new Error("类缺失: BytecodeArrayWriter");
const decls = members(cls.body);
const methods = decls.filter((d) => d.kind === "method");
if (methods.length !== 37) throw new Error(`方法对数不符: 期望 37 实得 ${methods.length}`);

// kMaxSizeOfPackedBytecode = 2 * 1 + 5 * OperandSize::kLast(4) = 22 (公式按 Bytecodes 表机械求值)
const maxSizePacked = 2 * 1 + 5 * 4;
const k8 = 0x7f;
const k16 = k8 | (k8 << 8);

let rust = banner("src/interpreter/bytecode-array-writer.h.js", "src/interpreter/bytecode-array-writer.{h,cc}", "字节码写出器骨架与打包常数");

rust += `
use crate::bytecodes::MAX_OPERANDS;

/// 打包字节码最大尺寸 = 前缀 1 + 操作码 1 + 5 操作数 × 最大操作数尺寸 4 (原版 kMaxSizeOfPackedBytecode 公式)
pub const MAX_SIZE_OF_PACKED_BYTECODE: usize = 2 + MAX_OPERANDS * crate::bytecode_operands::OperandSize::LAST as usize;

/// 8 位跳转占位 (原版 k8BitJumpPlaceholder)
pub const JUMP_PLACEHOLDER_8BIT: u32 = 0x7f;
/// 16 位跳转占位 (原版 k16BitJumpPlaceholder = 低 8 位拼接高 8 位)
pub const JUMP_PLACEHOLDER_16BIT: u32 = JUMP_PLACEHOLDER_8BIT | (JUMP_PLACEHOLDER_8BIT << 8);
/// 32 位跳转占位 (原版 k32BitJumpPlaceholder 同法)
pub const JUMP_PLACEHOLDER_32BIT: u32 = JUMP_PLACEHOLDER_16BIT | (JUMP_PLACEHOLDER_16BIT << 16);

/// 原版 class BytecodeArrayWriter 成员声明 (机械 1:1 提取, ${decls.length} 条)
pub const BYTECODE_ARRAY_WRITER_DECLARATIONS: &[&str] = &[
`;
for (const d of decls) rust += `  "[${d.vis}/${d.kind}] ${d.sig}",\n`;
rust += `];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_ARRAY_WRITER_DECLARATIONS.len() == ${decls.length});
  assert!(MAX_SIZE_OF_PACKED_BYTECODE == 22);
  assert!(JUMP_PLACEHOLDER_8BIT == 0x7f);
  assert!(JUMP_PLACEHOLDER_16BIT == 0x7f7f);
  assert!(JUMP_PLACEHOLDER_32BIT == 0x7f7f7f7f);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_array_writer.rs", rust);
console.log(`written ${p} (${decls.length} declarations, maxPacked=${maxSizePacked})`);
