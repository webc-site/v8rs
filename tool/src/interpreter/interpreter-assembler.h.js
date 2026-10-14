#!/usr/bin/env bun
// tool/src/interpreter/interpreter-assembler.h.js
// 机械提取 src/interpreter/interpreter-assembler.{h,cc} 的解释器汇编器: 栈检查/OSR 参数枚举、首字节码偏移公式与 143 条签名骨架 (cc 1853 行仅登记)
// → v8-interpreter/src/interpreter_assembler.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/interpreter-assembler.h");
const ccSrc = readV8("src/interpreter/interpreter-assembler.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "enum StackCheckBehavior {", "StackCheckBehavior \u679a\u4e3e"],
  [hSrc, "enum OnStackReplacementParams {", "OnStackReplacementParams \u679a\u4e3e"],
  [hSrc, "static constexpr int kFirstBytecodeOffset =", "kFirstBytecodeOffset \u516c\u5f0f"],
  [hSrc, "class V8_EXPORT_PRIVATE InterpreterAssembler : public CodeStubAssembler {", "InterpreterAssembler"],
  [ccSrc, "#include \"src/interpreter/interpreter-assembler.h\"", "cc \u5f15\u5165\u951a (1853 \u884c, \u8bed\u4e49\u4fee\u8ba2\u9636\u6bb5\u5904\u7406)"],
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
const CLASS_TABLES = [["InterpreterAssembler", "INTERPRETER_ASSEMBLER_DECLARATIONS", 143, 0], ["RegListNodePair", "REG_LIST_NODE_PAIR_DECLARATIONS", 5, 0]];
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
/// 栈检查开关 (原版 enum StackCheckBehavior)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StackCheckBehavior {
  /// 启用栈检查
  EnableStackCheck = 0,
  /// 禁用栈检查
  DisableStackCheck = 1,
}

/// OSR 参数 (原版 enum OnStackReplacementParams)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OnStackReplacementParams {
  /// baseline code 已缓存
  BaselineCodeIsCached = 0,
  /// 默认
  Default = 1,
}

/// 首字节码偏移 = BytecodeArray::kHeaderSize - kHeapObjectTag (原版 kFirstBytecodeOffset)
/// 字节码数组头大小与堆对象 tag 属 objects crate, 公式原样保留, 数值由参数注入
#[inline]
#[must_use]
pub const fn first_bytecode_offset(bytecode_array_header_size: i32, heap_object_tag: i32) -> i32 {
  bytecode_array_header_size - heap_object_tag
}
`;

let rust = banner("src/interpreter/interpreter-assembler.h.js", "src/interpreter/interpreter-assembler.{h,cc}", "");

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
  assert!(INTERPRETER_ASSEMBLER_DECLARATIONS.len() == 143);
  assert!(REG_LIST_NODE_PAIR_DECLARATIONS.len() == 5);
  assert!(first_bytecode_offset(40, 1) == 39);
};
`;

const p = writeSrc("v8-interpreter", "interpreter_assembler.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
