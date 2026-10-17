#!/usr/bin/env bun
// tool/src/interpreter/bytecode-jump-table.h.js
// 机械提取 v8/src/interpreter/bytecode-jump-table.h 的跳转表骨架
// → v8-interpreter/src/bytecode_jump_table.rs
import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/interpreter/bytecode-jump-table.h");

for (const [needle, name] of [
  ["static const size_t kInvalidIndex = static_cast<size_t>(-1);", "kInvalidIndex"],
  ["static const size_t kInvalidOffset = static_cast<size_t>(-1);", "kInvalidOffset"],
  ["size_t ConstantPoolEntryFor(int case_value)", "ConstantPoolEntryFor"],
  ["OperandScale switch_bytecode_operand_scale_ = OperandScale::kSingle;", "scale 字段默认值"],
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

const cls = classBlocks(stripComments(hSrc)).find((b) => b.name === "BytecodeJumpTable");
if (!cls) throw new Error("类缺失: BytecodeJumpTable");
const decls = members(cls.body);
const fields = decls.filter((d) => d.kind === "field");
if (fields.length !== 6) throw new Error(`字段对数不符: 期望 6 实得 ${fields.length}`);

let rust = banner("src/interpreter/bytecode-jump-table.h.js", "src/interpreter/bytecode-jump-table.h", "字节码 switch 跳转表骨架");

rust += `
/// 无效索引哨兵 (原版 kInvalidIndex)
pub const INVALID_INDEX: usize = usize::MAX;
/// 无效偏移哨兵 (原版 kInvalidOffset)
pub const INVALID_OFFSET: usize = usize::MAX;

/// 原版 class BytecodeJumpTable 成员声明 (switch 目标表; 表项绑定即字节码位置, 全不匹配则落穿)
pub const BYTECODE_JUMP_TABLE_DECLARATIONS: &[&str] = &[
`;
for (const d of decls) rust += `  "[${d.vis}/${d.kind}] ${d.sig}",\n`;
rust += `];

/// switch 表项常量池索引 = 基址 + (case 值 - case 基值) (原版 ConstantPoolEntryFor 公式)
#[inline]
#[must_use]
pub const fn constant_pool_entry_for(constant_pool_index: usize, case_value_base: i32, case_value: i32) -> usize {
  constant_pool_index + (case_value - case_value_base) as usize
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_JUMP_TABLE_DECLARATIONS.len() == ${decls.length});
  assert!(INVALID_INDEX == usize::MAX);
  assert!(constant_pool_entry_for(100, 3, 5) == 102);
};
`;

const p = writeSrc("v8-interpreter", "bytecode_jump_table.rs", rust);
console.log(`written ${p} (${decls.length} declarations)`);
