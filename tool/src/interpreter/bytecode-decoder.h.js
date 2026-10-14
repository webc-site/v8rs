#!/usr/bin/env bun
// tool/src/interpreter/bytecode-decoder.h.js
// 机械提取 v8/src/interpreter/bytecode-decoder.{h,cc} 的解码器静态接口骨架
// → v8-interpreter/src/bytecode_decoder.rs
import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/interpreter/bytecode-decoder.h");
const ccSrc = readV8("src/interpreter/bytecode-decoder.cc");

// 结构锚
for (const [src, needle, name] of [
  [hSrc, "static Register DecodeRegisterOperand(Address operand_start,", "DecodeRegisterOperand 声明"],
  [hSrc, "static uint8_t RacyDecodeEmbeddedFeedback(Address operand_start);", "RacyDecodeEmbeddedFeedback 声明"],
  [ccSrc, "Register BytecodeDecoder::DecodeRegisterOperand(", "DecodeRegisterOperand 定义"],
  [ccSrc, "uint32_t BytecodeDecoder::DecodeUnsignedOperand(Address operand_start,", "DecodeUnsignedOperand 定义"],
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

const cls = classBlocks(stripComments(hSrc)).find((b) => b.name === "BytecodeDecoder");
if (!cls) throw new Error("类缺失: BytecodeDecoder");
const decls = members(cls.body);
if (decls.length !== 6) throw new Error(`方法对数不符: 期望 6 实得 ${decls.length}`);

let rust = banner("src/interpreter/bytecode-decoder.h.js", "src/interpreter/bytecode-decoder.{h,cc}", "字节码解码器静态接口骨架");

rust += `
/// 原版 class BytecodeDecoder 成员声明 (机械 1:1 提取, ${decls.length} 条, 全静态)
pub const BYTECODE_DECODER_DECLARATIONS: &[&str] = &[
`;
for (const d of decls) rust += `  "[${d.vis}] ${d.sig}",\n`;
rust += `];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_DECODER_DECLARATIONS.len() == 6);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_decoder.rs", rust);
console.log(`written ${p} (${decls.length} declarations)`);
