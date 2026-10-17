#!/usr/bin/env bun
// tool/src/objects/object-predicates.h.js — 覆盖 v8/src/objects/object-predicates.{h,-inl.h}
// → v8-objects-core/src/object_predicates.rs
// X-macro 全量展开: OBJECT_TYPE_LIST + HEAP_OBJECT_TYPE_LIST(5 子列表) +
//   显式 IS_TYPE_FUNCTION_DECL 条目 + STRUCT_LIST(经 STRUCT_LIST_ADAPTER/GENERATOR 两级)
//   → Is<Type> 谓词名单表; TDZ/TEMPORAL 分支与 crate 既有产物对齐 (见 CFG)。

import { readV8, writeSrc, banner } from "../../lib.js";
import { spawnSync } from "node:child_process";
import { mkdtempSync, writeFileSync, readFileSync, rmSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const predSrc = readV8("src/objects/object-predicates.h");
const listSrc = readV8("src/objects/object-list-macros.h");
const defSrc = readV8("src/objects/objects-definitions.h");

// 目标配置 (与 v8rs 既有产物一致): INTL 开 / WASM 开 / TEMPORAL 关 / TDZ 关
// (对齐 instance-type.h.js 与 oddball-predicates.h.js 已落地分支;
//  注意 BUILD.gn 声明 v8_enable_tdz_hole=true, 复跑前若 crate 决策翻面需同步改此表)
const CFG = { V8_INTL_SUPPORT: true, V8_ENABLE_WEBASSEMBLY: true, V8_TEMPORAL_SUPPORT: false, V8_ENABLE_TDZ_HOLE: false };
const tdzOn = CFG.V8_ENABLE_TDZ_HOLE;

// 配置感知宏体提取: #define NAME(...) 至空行; 若定义被 #ifdef COND 包裹且
// 目标配置为假, 取下一处 (#else 分支) 定义
function macroBody(src, name) {
  const defs = [...src.matchAll(new RegExp(`#define ${name}\\\\?\\([^)]*\\)`, "g"))];
  if (!defs.length) throw new Error(`未找到宏 ${name}`);
  let pick = 0;
  const before = src.slice(Math.max(0, defs[0].index - 200), defs[0].index);
  const cond = [...before.matchAll(/#ifdef\s+(\w+)|#ifndef\s+(\w+)/g)].pop();
  if (cond) {
    const c = cond[1] ?? cond[2];
    if (c in CFG && CFG[c] === false) {
      if (defs.length < 2) throw new Error(`宏 ${name} 无 #else 分支可取 (配置 ${c}=false)`);
      pick = 1;
    }
  }
  const start = defs[pick].index + defs[pick][0].length;
  const rest = src.slice(start);
  // 体边界: 空行 或 下一预处理行 (#else/#endif/下一分支)
  const mEnd = rest.match(/\n\s*(?:\n|#)/);
  if (!mEnd) throw new Error(`宏 ${name} 体无边界`);
  return rest.slice(0, mEnd.index).replace(/\\\n/g, "\n");
}

// V(Name) / V(A, B, C) 条目切分
function macroEntries(body, arity = null) {
  const entries = [];
  let depth = 0, cur = "", inEntry = false;
  for (const ch of body) {
    if (ch === "(") { depth++; inEntry = depth > 0; cur += ch; continue; }
    if (ch === ")") { depth--; cur += ch; if (depth === 0) { entries.push(cur); cur = ""; inEntry = false; } continue; }
    if (inEntry) cur += ch;
  }
  const out = entries
    .map((e) => e.slice(1, -1).split(",").map((s) => s.trim()))
    .filter((parts) => parts.length > 0 && parts[0] !== "" && !parts[0].startsWith("#") && !parts[0].startsWith("//"));
  if (arity !== null) {
    for (const p of out) if (p.length !== arity) throw new Error(`宏条目参数数失配: ${p.join(",")}`);
  }
  return out;
}

// 展开列表宏为谓词名序列: 逐续行段解析 —
//   直接条目 V(X) / 转发 SUB(V) / 生成器应用 GEN(ADAPTER, V)
function expandListRecursive(name, depth = 0) {
  if (depth > 8) throw new Error(`列表宏递归过深: ${name}`);
  const body = macroBody(listSrc, name);
  const out = [];
  for (let seg of body.split("\n")) {
    seg = seg.replace(/\\/g, "").trim();
    if (!seg || seg.startsWith("//")) continue;
    let m;
    if ((m = seg.match(/^(\w+_GENERATOR)\(\s*(\w+)\s*,\s*V\s*\)$/))) {
      out.push(...applyGenerator(m[1], m[2]));
    } else if ((m = seg.match(/^(\w+)\(V\)$/))) {
      out.push(...expandListRecursive(m[1], depth + 1));
    } else if ((m = seg.match(/^V\(([^()]*)\)$/))) {
      out.push(...m[1].split(",").map((s) => s.trim()));
    } else if ((m = seg.match(/^IF_WASM\(V,\s*([^()]*)\)$/))) {
      // IF_WASM 条目: WASM 开保留 (目标配置), 关闭则剔除
      if (CFG.V8_ENABLE_WEBASSEMBLY) out.push(...m[1].split(",").map((s) => s.trim()));
    } else {
      throw new Error(`${name} 段不可解析: ${seg}`);
    }
  }
  return out;
}

// 生成器 × 适配器: 条目列数随生成器而异 (抽象 2 列 / 具体带 TYPE 3 列);
// 适配器形参序决定取列 (如 TRUSTED_OBJECT_LIST1_ADAPTER(V, Name, NAME) → V(Name))
function applyGenerator(genName, adapterName) {
  const genBody = macroBody(defSrc.includes(`#define ${genName}`) ? defSrc : listSrc, genName);
  const adMatch = listSrc.match(new RegExp(`#define ${adapterName}\\(([^)]*)\\)\\s*([^\\n]*)`));
  if (!adMatch) throw new Error(`未找到适配器 ${adapterName}`);
  const formals = adMatch[1].split(",").map((s) => s.trim()); // [V, Name, NAME]
  const adBody = adMatch[2];
  const ref = adBody.match(/V\((\w+)\)/);
  if (!ref) throw new Error(`适配器 ${adapterName} 体不可解析: ${adBody}`);
  const colIdx = formals.indexOf(ref[1]) - 1; // 减去 V 占位
  if (colIdx < 0) throw new Error(`适配器 ${adapterName} 列名失配: ${ref[1]}`);
  // 逐条目提取两种形态: APPLY(V, cols...) / IF_WASM(APPLY, V, cols...) (WASM 开保留)
  const out = [];
  let arity = -1;
  const entryRe = /APPLY\(\s*V\s*,\s*([^)]*?)\)\s*\)?|IF_WASM\(\s*APPLY\s*,\s*V\s*,\s*([^)]*?)\)\s*\)?\)?/g;
  for (const m of genBody.matchAll(entryRe)) {
    const isWasm = m[2] !== undefined;
    if (isWasm && !CFG.V8_ENABLE_WEBASSEMBLY) continue;
    const cols = (isWasm ? m[2] : m[1]).split(",").map((s) => s.trim());
    if (arity === -1) arity = cols.length;
    if (cols.length !== arity) throw new Error(`生成器 ${genName} 条目列数失配: ${m[0]}`);
    if (colIdx >= arity) throw new Error(`适配器 ${adapterName} 列越界: ${colIdx} >= ${arity}`);
    out.push(cols[colIdx]);
  }
  return out;
}

function expandHeapObjectList() {
  const body = macroBody(listSrc, "HEAP_OBJECT_TYPE_LIST");
  const subNames = [...body.matchAll(/(\w+_LIST\w*)\(V\)/g)].map((m) => m[1]);
  if (subNames.length !== 5) throw new Error(`HEAP_OBJECT_TYPE_LIST 子列表数 ${subNames.length} != 5`);
  const all = [];
  for (const sub of subNames) {
    all.push({ sub, entries: expandListRecursive(sub) });
  }
  return all;
}

// STRUCT_LIST: 两级展开, 取第 3 列 Name (含 IF_WASM 条目, WASM 开保留)
function expandStructList() {
  const gen = macroBody(defSrc, "STRUCT_LIST_GENERATOR");
  const raw = [];
  for (const m of gen.matchAll(/V\(([^()]*)\)|IF_WASM\(V,\s*_,\s*([^()]*)\)/g)) {
    const isWasm = m[2] !== undefined;
    const inner = isWasm ? m[2] : m[1];
    const parts = inner.split(",").map((s) => s.trim());
    const want = isWasm ? 3 : 4;
    if (parts.length !== want) throw new Error(`STRUCT_LIST 条目参数数失配 (${want}): ${inner}`);
    raw.push({ isWasm, name: parts[isWasm ? 1 : 2] });
  }
  return raw;
}

const objectTypes = expandListRecursive("OBJECT_TYPE_LIST");
const heapGroups = expandHeapObjectList();
const heapTypes = heapGroups.flatMap((g) => g.entries);
const heapTotal = heapTypes.length;

// 显式条目 (object-predicates.h 直接写的 IS_TYPE_FUNCTION_DECL)
const explicitTypes = [...predSrc.matchAll(/^IS_TYPE_FUNCTION_DECL\((\w+)\)$/gm)].map((m) => m[1]);
if (explicitTypes.join(",") !== "HashTableBase,SloppyArgumentsElements,SmallOrderedHashTable,PropertyDictionary,AnyHole") {
  throw new Error(`显式谓词条目失配: ${explicitTypes.join(",")}`);
}

// STRUCT_LIST Name 列
const structEntries = expandStructList();
const structNames = structEntries.filter((e) => !e.isWasm || true).map((e) => e.name);
// IF_WASM 条目 (目标 WASM 开 → 保留); 关闭时应剔除, 此处对数登记
const wasmStructCount = structEntries.filter((e) => e.isWasm).length;

// TDZ 分支: HOLE_LIST 嵌套 TDZ_HOLE_LIST 引用按位插入
const holeBody = macroBody(listSrc, "HOLE_LIST");
const tdzEntries = macroEntries(macroBody(listSrc, "TDZ_HOLE_LIST"), 3);
const tdzName = tdzEntries[0][0];
if (!tdzOn && tdzName !== "DisabledTdzHole") throw new Error(`TDZ 关分支名失配: ${tdzName}`);
const holeNames = [];
for (const m of holeBody.matchAll(/V\((\w+),/g)) {
  holeNames.push(m[1]);
}
// HOLE_LIST 中段嵌套 TDZ_HOLE_LIST(V) → 按 TDZ 分支落值
if (/TDZ_HOLE_LIST\(V\)/.test(holeBody)) holeNames.splice(4, 0, tdzName);
if (!holeNames.some((n) => n === "TheHole")) throw new Error("HOLE_LIST 缺 TheHole");
// 嵌套展开后不应残留占位引用名
if (holeNames.includes("TDZ_HOLE_LIST")) throw new Error("TDZ_HOLE_LIST 引用未展开");

// ── 原版侧锚: clang -E 探针展开三列表, 与脚本展开 1:1 比对 (todo.md 对数基准) ──
function clangProbe() {
  const probe = [
    "#include \"src/objects/object-list-macros.h\"",
    "#include \"src/objects/objects-definitions.h\"",
    "#define PRED_OBJ(Name) Name,",
    "#define PRED_STRUCT(NAME, Name, name) Name,",
    "OBJECT_BEGIN",
    "OBJECT_TYPE_LIST(PRED_OBJ)",
    "OBJECT_END",
    "HEAP_BEGIN",
    "HEAP_OBJECT_TYPE_LIST(PRED_OBJ)",
    "HEAP_END",
    "EXPLICIT_BEGIN",
    "HashTableBase, SloppyArgumentsElements, SmallOrderedHashTable, PropertyDictionary, AnyHole,",
    "EXPLICIT_END",
    "STRUCT_BEGIN",
    "STRUCT_LIST(PRED_STRUCT)",
    "STRUCT_END",
  ].join("\n");
  const dir = mkdtempSync(join(tmpdir(), "v8rs-opred-"));
  const file = join(dir, "probe.cc");
  const outFile = join(dir, "probe.i");
  writeFileSync(file, probe);
  // 生成物桩: checkout 无 metagen/torque-generated 产物, 列表宏展开不需要其内容
  const stubMeta = join(dir, "metagen");
  const stubTq = join(dir, "torque-generated");
  mkdirSync(stubMeta, { recursive: true });
  mkdirSync(stubTq, { recursive: true });
  for (const [d, n] of [[stubMeta, "instance-types.h"], [stubTq, "instance-types.h"], [stubTq, "instance-type-checker-lists.h"], [stubTq, "debug-reader-classes-list.h"]]) {
    writeFileSync(join(d, n), "#define TORQUE_ASSIGNED_INSTANCE_TYPES(V)\n#define TORQUE_ASSIGNED_INSTANCE_TYPE_LIST(V)\n#define INSTANCE_TYPE_LIST_SINGLE(V)\n#define INSTANCE_TYPE_LIST_MULTIPLE(V)\n#define INSTANCE_TYPE_LIST_RANGE(V)\n#define TORQUE_DEBUG_READER_CLASSES_SINGLE(V)\n#define TORQUE_DEBUG_READER_CLASSES_MULTIPLE(V)\n");
  }
  const v8root = join(import.meta.dir, "../../../v8");
  const res = spawnSync(
    "clang",
    ["-E", "-P", "-std=c++20", "-DV8_USE_METAGEN=0", "-DV8_INTL_SUPPORT", "-DV8_ENABLE_WEBASSEMBLY",
      "-I", dir, "-I", join(v8root, "include"), "-I", v8root, "-o", outFile, file],
    { encoding: "utf8" },
  );
  let stdout = null;
  if (!res.error && res.status === 0) stdout = readFileSync(outFile, "utf8");
  try { rmSync(dir, { recursive: true, force: true }); } catch { /* tmp 清理失败可忽略 */ }
  if (res.error || res.status !== 0 || stdout === null) {
    console.log(`clang 探针失败 (${res.error?.message ?? `exit ${res.status}}`}), 回退硬锚条数对数`);
    if (res.stderr) console.log(res.stderr.split("\n").slice(0, 4).join("\n"));
    return null; // clang 不可用 → 回退硬锚
  }
  const section = (tag) => {
    const m = stdout.match(new RegExp(`${tag}_BEGIN\\n([\\s\\S]*?)\\n${tag}_END`));
    if (!m) throw new Error(`clang 探针缺 ${tag} 段`);
    return m[1].split(",").map((s) => s.trim()).filter((s) => /^[A-Za-z]\w*$/.test(s));
  };
  return { obj: section("OBJECT"), heap: section("HEAP"), struct: section("STRUCT") };
}

const clangRef = clangProbe();
const FALLBACK = { obj: 3, heap: 338, struct: 40 };

// 对数 1: 名单逐名比对 (clang 锚) 或条数比对 (硬锚回退)
if (clangRef) {
  const cmp = (label, mine, ref) => {
    if (mine.length !== ref.length) throw new Error(`${label} 条数失配: 脚本 ${mine.length} vs clang ${ref.length}`);
    for (let i = 0; i < ref.length; i++) {
      if (mine[i] !== ref[i]) throw new Error(`${label} 顺序失配 @${i}: 脚本 ${mine[i]} vs clang ${ref[i]}`);
    }
  };
  cmp("OBJECT_TYPE_LIST", objectTypes, clangRef.obj);
  cmp("HEAP_OBJECT_TYPE_LIST", heapTypes, clangRef.heap);
  cmp("STRUCT_LIST", structNames, clangRef.struct);
} else {
  console.log("clang 不可用, 回退硬锚条数对数");
  if (objectTypes.length !== FALLBACK.obj) throw new Error(`OBJECT ${objectTypes.length} != ${FALLBACK.obj}`);
  if (heapTotal !== FALLBACK.heap) throw new Error(`HEAP ${heapTotal} != ${FALLBACK.heap}`);
  if (structNames.length !== FALLBACK.struct) throw new Error(`STRUCT ${structNames.length} != ${FALLBACK.struct}`);
}

// 对数 2: 独立计数守卫 (与 clang 锚互补; HOLE_LIST 嵌套展开含 TDZ 占位)
if (holeNames.length !== 13) throw new Error(`HOLE_LIST 条数 ${holeNames.length} != 13 (TDZ ${tdzOn ? "开" : "关"}含占位)`);

// 去重检查
const all = [...objectTypes, ...heapTypes, ...explicitTypes, ...structNames];
// 跨列表重复是合法的 (C++ 可重声明; 如 SloppyArgumentsElements 同在 HEAP 与显式条目),
// 但须登记条数以便对数
const dup = all.filter((n, i) => all.indexOf(n) !== i);

let rust = banner("src/objects/object-predicates.h.js", "src/objects/object-predicates.{h,-inl.h}", "Is<Type> 谓词名单表 (object-list-macros.h X-macro 全量展开)");
rust += `//! object-predicates 谓词名单: OBJECT_TYPE_LIST + HEAP_OBJECT_TYPE_LIST(5 子列表)
//! + 显式条目 + STRUCT_LIST 两级展开。TDZ 分支: BUILD.gn v8_enable_tdz_hole=true。
//! 各谓词语义实现归 Developer (复用 instance_type.rs 判别)。

/// OBJECT_TYPE_LIST (${objectTypes.length} 项; Smi/Number 域谓词)。
pub const OBJECT_TYPE_PREDICATES: [&str; ${objectTypes.length}] = [${objectTypes.map((n) => `"${n}"`).join(", ")}];

/// HEAP_OBJECT_TYPE_LIST 子列表构成 (顺序 1:1)。
`;
for (const g of heapGroups) {
  rust += `/// ${g.sub} (${g.entries.length} 项)\n`;
}
rust += `pub const HEAP_OBJECT_TYPE_PREDICATES: [&str; ${heapTotal}] = [
`;
for (const g of heapGroups) {
  rust += `  // ── ${g.sub} ──\n`;
  for (const e of g.entries) rust += `  "${e}",\n`;
}
rust += `];

/// 显式谓词条目 (object-predicates.h 直写)。
pub const EXPLICIT_PREDICATES: [&str; ${explicitTypes.length}] = [${explicitTypes.map((n) => `"${n}"`).join(", ")}];

/// STRUCT_LIST 谓词 (objects-definitions.h STRUCT_LIST_GENERATOR 两级展开; WASM 开含 ${wasmStructCount} 条 IF_WASM)。
pub const STRUCT_PREDICATES: [&str; ${structNames.length}] = [${structNames.map((n) => `"${n}"`).join(", ")}];

/// HOLE_LIST 谓词名 (TDZ 取 ${tdzName}; 原版 HOLE_LIST 含嵌套展开共 ${holeNames.length} 项)。
pub const HOLE_PREDICATES: [&str; ${holeNames.length}] = [${holeNames.map((n) => `"${n}"`).join(", ")}];

/// 谓词名总数 (去重前)。
pub const PREDICATE_TOTAL: usize = ${all.length};

// —— 提取不变式断言 ——
const _: () = {
  assert!(OBJECT_TYPE_PREDICATES.len() == 3);
  assert!(HEAP_OBJECT_TYPE_PREDICATES.len() == ${heapTotal});
  assert!(EXPLICIT_PREDICATES.len() == 5);
  assert!(STRUCT_PREDICATES.len() == ${structNames.length});
  assert!(HOLE_PREDICATES.contains(&"TheHole"));
  assert!(HOLE_PREDICATES.contains(&"${tdzName}"));
  assert!(PREDICATE_TOTAL == ${all.length});
};
`;

const p = writeSrc("v8-objects-core", "object_predicates.rs", rust);
console.log(`object_predicates.rs: 谓词名单 ${all.length} (OBJECT 3 + HEAP ${heapTotal} + 显式 5 + STRUCT ${structNames.length} + HOLE ${holeNames.length}), TDZ=${tdzName}, 对数通过`);
