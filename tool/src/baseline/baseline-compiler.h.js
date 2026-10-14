#!/usr/bin/env bun
// tool/src/baseline/baseline-compiler.h.js
// 机械提取 src/baseline/baseline-compiler.{h,cc} 的Sparkplug 编译器: 栈检查枚举/Visit* X-macro 全展开 (217+17) 与骨架 (cc 3067 行仅登记)
// → v8-interpreter/src/baseline_compiler.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/baseline/baseline-compiler.h");
const ccSrc = readV8("src/baseline/baseline-compiler.cc");
const bcH = readV8("src/interpreter/bytecodes.h");
const intrH = readV8("src/interpreter/interpreter-intrinsics.h");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "class BytecodeOffsetTableBuilder {", "BytecodeOffsetTableBuilder"],
  [hSrc, "class BaselineCompiler {", "BaselineCompiler"],
  [hSrc, "enum StackCheckBehavior {", "StackCheckBehavior \u679a\u4e3e"],
  [hSrc, "#define DECLARE_VISITOR(name, ...) void Visit##name();", "DECLARE_VISITOR X-macro"],
  [hSrc, "BYTECODE_LIST(DECLARE_VISITOR, DECLARE_VISITOR)", "Visit* \u5168\u8868\u5c55\u5f00\u70b9"],
  [hSrc, "INTRINSICS_LIST(DECLARE_VISITOR)", "VisitIntrinsic* \u5c55\u5f00\u70b9"],
  [ccSrc, "void BaselineCompiler::GenerateCode() {", "GenerateCode \u5b9a\u4e49"],
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
// —— Visit* X-macro 展开: BYTECODE_LIST 217 条 + INTRINSICS_LIST 17 条 ——
function rawMacroBody(src, name) {
  const re = new RegExp(`#define\\s+${name}\\(`);
  const m = re.exec(src);
  if (!m) throw new Error(`宏缺失: ${name}`);
  const allLines = src.slice(m.index).split("\n");
  const head = allLines[0].slice(allLines[0].indexOf(")") + 1);
  const body = [];
  let start = 1;
  if (head.trim()) {
    body.push(head.trimEnd().endsWith("\\") ? head.trimEnd().slice(0, -1) : head);
    if (!head.trimEnd().endsWith("\\")) start = allLines.length;
  }
  for (let i = start; i < allLines.length; i++) {
    const line = allLines[i];
    const cont = line.endsWith("\\");
    body.push(cont ? line.slice(0, -1) : line);
    if (!cont) break;
  }
  return body.join("\n").replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");
}
const macroCache = new Map();
function expandList(src, name) {
  if (macroCache.has(name)) return macroCache.get(name);
  const body = rawMacroBody(src, name).replace(
    /\b([A-Z][A-Z_0-9]+)\((V(?:\s*,\s*V(?:_TSA)?)?)\)/g,
    (full, inner) => {
      if (inner === "V" || inner === "V_TSA") return full;
      return expandList(src, inner).map((e) => `V(${e.join(", ")})`).join("\n");
    },
  );
  const out = [];
  const re = /\bV(?:_TSA)?\s*\(([^()]*)\)/g;
  let m;
  while ((m = re.exec(body)) !== null) out.push(m[1].split(",").map((x) => x.trim()).filter(Boolean));
  macroCache.set(name, out);
  return out;
}
const bytecodeList = expandList(bcH, "BYTECODE_LIST");
if (bytecodeList.length !== 217) throw new Error(`BYTECODE_LIST 对数不符: 期望 217 实得 ${bytecodeList.length}`);
const visitBytecodes = bytecodeList.map((e) => e[0]);
const intrListSrc = intrH.slice(intrH.indexOf("#define INTRINSICS_LIST(V)"));
const intrSeg = intrListSrc.slice(intrListSrc.indexOf("\n") + 1).split("\n");
const intrLines = [];
for (const line of intrSeg) {
  const cont = line.endsWith("\\");
  intrLines.push(cont ? line.slice(0, -1) : line);
  if (!cont) break;
}
const intrinsics = [...intrLines.join(" ").matchAll(/V\((\w+),/g)].map((m) => m[1]);
if (intrinsics.length !== 17) throw new Error(`INTRINSICS_LIST 对数不符: 期望 17 实得 ${intrinsics.length}`);

const RUST_HEAD = `
/// 栈检查开关 (原版 enum StackCheckBehavior)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum StackCheckBehavior {
  EnableStackCheck = 0,
  DisableStackCheck = 1,
}

impl StackCheckBehavior {
  #[inline]
  #[must_use]
  pub const fn is_enabled(self) -> bool {
    matches!(self, Self::EnableStackCheck)
  }

  #[inline]
  #[must_use]
  pub const fn is_disabled(self) -> bool {
    matches!(self, Self::DisableStackCheck)
  }
}

/// builtin 调用/跳转模式 (原版 src/codegen/assembler.h enum class BuiltinCallJumpMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum BuiltinCallJumpMode {
  /// 绝对地址内嵌
  Absolute = 0,
  /// PC 相对
  PcRelative = 1,
  /// 间接 (经 builtins 入口表)
  Indirect = 2,
  /// 仅 mksnapshot
  ForMksnapshot = 3,
}

impl BuiltinCallJumpMode {
  #[inline]
  #[must_use]
  pub const fn is_indirect(self) -> bool {
    matches!(self, Self::Indirect)
  }
}

/// 单字节码访问器全表 (原版 BYTECODE_LIST(DECLARE_VISITOR) 展开, 217 个 Visit<Xxx>)
pub const VISIT_BYTECODE_METHODS: [&str; 217] = [
${visitBytecodes.map((n) => "  \"Visit" + n + "\",").join("\n")}
];

/// 内在函数访问器全表 (原版 INTRINSICS_LIST(DECLARE_VISITOR) 展开, 17 个 VisitIntrinsic<Xxx>)
pub const VISIT_INTRINSIC_METHODS: [&str; 17] = [
${intrinsics.map((n) => "  \"VisitIntrinsic" + n + "\",").join("\n")}
];`;
const stripped = stripComments(hSrc);
const blocks = classBlocks(stripped);
const CLASS_TABLES = [["BytecodeOffsetTableBuilder", "BYTECODE_OFFSET_TABLE_BUILDER_DECLARATIONS", 4, 0], ["BaselineCompiler", "BASELINE_COMPILER_DECLARATIONS", 76, 0], ["CompareBranchCandidate", "COMPARE_BRANCH_CANDIDATE_DECLARATIONS", 2, 0], ["EffectState", "EFFECT_STATE_DECLARATIONS", 6, 0], ["SaveAccumulatorScope", "SAVE_ACCUMULATOR_SCOPE_DECLARATIONS", 4, 0]];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}



let rust = banner("src/baseline/baseline-compiler.h.js", "src/baseline/baseline-compiler.{h,cc}", "");

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
  assert!(BASELINE_COMPILER_DECLARATIONS.len() == 76);
  assert!(BYTECODE_OFFSET_TABLE_BUILDER_DECLARATIONS.len() == 4);
  assert!(VISIT_BYTECODE_METHODS.len() == 217);
  assert!(VISIT_INTRINSIC_METHODS.len() == 17);
  assert!(BuiltinCallJumpMode::Indirect as u8 == 2);
  assert!(StackCheckBehavior::EnableStackCheck as u8 == 0);
  assert!(StackCheckBehavior::DisableStackCheck as u8 == 1);
};
`;

const p = writeSrc("v8-baseline", "baseline_compiler.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
