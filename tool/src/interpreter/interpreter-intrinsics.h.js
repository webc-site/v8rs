#!/usr/bin/env bun
// tool/src/interpreter/interpreter-intrinsics.h.js
// 机械提取 src/interpreter/interpreter-intrinsics.{h,cc} 的内在函数全表 (INTRINSICS_LIST 展开) 与辅助器骨架
// → v8-interpreter/src/interpreter_intrinsics.rs
import { readV8, writeSrc, banner } from "../../lib.js";
const hSrc = readV8("src/interpreter/interpreter-intrinsics.h");
const ccSrc = readV8("src/interpreter/interpreter-intrinsics.cc");
// 结构锚
for (const [src, needle, name] of [
  [hSrc, "#define INTRINSICS_LIST(V)", "INTRINSICS \u5b8f"],
  [hSrc, "enum class IntrinsicId {", "IntrinsicId \u679a\u4e3e"],
  [hSrc, "static_assert(static_cast<uint32_t>(IntrinsicId::kIdCount) <= kMaxUInt8);", "kIdCount \u4e0a\u9650\u65ad\u8a00"],
  [ccSrc, "bool IntrinsicsHelper::IsSupported(Runtime::FunctionId function_id) {", "IsSupported \u5b9a\u4e49"],
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
// —— INTRINSICS_LIST 机械展开 (17 项: 大写名/小写运行时名/参数个数) ——
const listStart = hSrc.indexOf("#define INTRINSICS_LIST(V)");
const listSeg = hSrc.slice(listStart).split("\n");
const bodyLines = [];
for (let i = 1; i < listSeg.length; i++) {
  const line = listSeg[i];
  const cont = line.endsWith("\\");
  bodyLines.push(cont ? line.slice(0, -1) : line);
  if (!cont) break;
}
const intrBody = bodyLines.join(" ").replace(/\/\/.*$/g, "");
const intrinsics = [...intrBody.matchAll(/V\((\w+),\s*(\w+),\s*(-?\d+)\)/g)].map((m) => [m[1], m[2], Number(m[3])]);
if (intrinsics.length !== 17) throw new Error(`INTRINSICS_LIST 对数不符: 期望 17 实得 ${intrinsics.length}`);

const RUST_HEAD = `
/// 内在函数表 (原版 INTRINSICS_LIST 展开, 17 项; 列 = 大写名 / 运行时函数名 / 参数个数, -1 为可变)
pub static INTRINSICS: [(&str, &str, i32); 17] = [
${intrinsics.map(([n, r, c]) => "  (\"" + n + "\", \"" + r + "\", " + c + "),").join("\n")}
];

/// 内在函数 id (原版 IntrinsicsHelper::IntrinsicId; kIdCount = 17 <= kMaxUInt8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum IntrinsicId {
${intrinsics.map(([n], i) => "  /// 原版 k" + n + " (" + intrinsics[i][2] + " 参数)\n  " + n + " = " + i + ",").join("\n")}
}

impl IntrinsicId {
  /// 总数 (原版 kIdCount)
  pub const ID_COUNT: usize = 17;

  /// 对应运行时函数名
  #[must_use]
  pub const fn runtime_function_name(self) -> &'static str {
    INTRINSICS[self as usize].1
  }

  /// 期望参数个数 (-1 表示可变)
  #[must_use]
  pub const fn argument_count(self) -> i32 {
    INTRINSICS[self as usize].2
  }
}

/// 原版 class IntrinsicsHelper 成员声明之外运行时 id 映射为纯查表:
/// FromRuntimeId/ToRuntimeId 由 INTRINSICS 第二列 1:1 反查, 修订阶段接入 runtime crate。`;
const stripped = stripComments(hSrc);
const blocks = classBlocks(stripped);
const CLASS_TABLES = [["IntrinsicsHelper", "INTRINSICS_HELPER_DECLARATIONS", 3, 0]];
const emitTables = [];
for (const [className, constName, expected, occurrence] of CLASS_TABLES) {
  const hits = blocks.filter((b) => b.name === className);
  const idx = occurrence ?? 0;
  if (hits.length <= idx) throw new Error(`类缺失: ${className} (#${idx})`);
  const decls = members(hits[idx].body);
  if (decls.length !== expected) throw new Error(`${className} 对数不符: 期望 ${expected} 实得 ${decls.length}`);
  emitTables.push({ constName, decls, className });
}



let rust = banner("src/interpreter/interpreter-intrinsics.h.js", "src/interpreter/interpreter-intrinsics.{h,cc}", "");

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
  assert!(INTRINSICS_HELPER_DECLARATIONS.len() == 3);
  assert!(IntrinsicId::ID_COUNT == 17);
  assert!(matches!(IntrinsicId::CopyDataProperties.runtime_function_name().as_bytes(), b"copy_data_properties"));
  assert!(IntrinsicId::CreateAsyncFromSyncIterator.argument_count() == 1);
};
`;

const p = writeSrc("v8-interpreter", "interpreter_intrinsics.rs", rust);
console.log(`written ${p} (${emitTables.map((t) => t.constName + "=" + t.decls.length).join(", ")})`);
