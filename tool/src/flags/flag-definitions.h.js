#!/usr/bin/env bun
// v8/src/flags/{flag-definitions.h, feature-flags.h} + flags.cc 矛盾表 → v8-flags/src 直出
// 转换规则 (宏先展开, 见 .agents/skills/transpile 7.1 / todo.md 十一章):
//   - 用真实 C 预处理器 (clang -E) 按规范配置 (macOS arm64 release, wasm/intl/maglev/turbofan 开,
//     DEBUG/模拟器关) 机械展开 X-macro 体系, 杜绝人工漏项
//   - 4 趟展开:
//       APPLY(pass1): FLAG_MODE_APPLY+INCLUDE_READONLY+INCLUDE_ALIASES → 名称/主名/注释 (含别名, 定义序)
//       DEFAULTS(pass2): FLAG_MODE_DEFINE_DEFAULTS → 每 flag 默认值表达式文本
//       META(pass3): FLAG_MODE_META → FlagType / 可写性 (valptr==nullptr → readonly), 顺序=原版 flags[] 序
//       IMPL(pass4): FLAG_MODE_DEFINE_IMPLICATIONS → 全部 implication/generic/requirement/warning
//   - feature-flags.h 单独一趟 (4 个 FOREACH 宏 × js/wasm/internal 槽) → 特性表
//   - flags.cc 的 ResolveContradictionsWhenFuzzing 区域正则提取 CONTRADICTION/RESET 清单
//   - 表达式翻译: C++ 布尔/算术表达式 → Rust 闭包体; 未知标识符一律硬报错 (防漂移)
// 产物 (非成品, 语义仍需人工核对): defs.rs / implications_gen.rs / features_gen.rs /
//   contradictions_gen.rs / counts.rs + tool/src/flags/stats.json (对数核对基线)
import { spawnSync } from "node:child_process";
import { mkdtempSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-flags";
const GEN = "src/flags/flag-definitions.h.js"; // banner() 用: 相对 tool/ 的脚本路径

// ---------- 规范构建配置 (单一事实源: 本表; 登记于 readme/zh.md) ----------
// 值为 true 的在 driver 里 #define X 1; false 的保持未定义 (既满足 #ifdef 也满足 #if X→0)
const CONFIG = {
  V8_TARGET_ARCH_ARM64: true,
  V8_HOST_ARCH_ARM64: true,
  V8_TARGET_ARCH_64_BIT: true,
  V8_OS_MACOS: true,
  V8_OS_DARWIN: true,
  V8_ENABLE_WEBASSEMBLY: true,
  V8_INTL_SUPPORT: true,
  V8_ENABLE_SPARKPLUG: true,
  V8_ENABLE_MAGLEV: true,
  V8_ENABLE_TURBOFAN: true,
  V8_COMPRESS_POINTERS: true,
  V8_ENABLE_SANDBOX: true,
  // 关: DEBUG / USE_SIMULATOR / V8_EXTERNAL_CODE_SPACE / V8_SHORT_BUILTIN_CALLS /
  //     V8_ENABLE_SPARKPLUG_PLUS / V8_ENABLE_WASM_SIMD256_REVEC / V8_ENABLE_FUTURE /
  //     V8_TRACE_FEEDBACK_UPDATES / V8_OS_WIN / V8_ENABLE_DOUBLE_CONST_STORE_CHECK /
  //     V8_ENABLE_LAZY_SOURCE_POSITIONS / V8_STATIC_ROOTS / V8_CODE_COMMENTS / V8_ENABLE_ETW...
};
// 文件本身要求 DEBUG_BOOL 必须已定义 (globals.h 的 constexpr bool 宏)
const CONFIG_EXTRA = [
  "DEBUG_BOOL false",
  "COMPRESS_POINTERS_BOOL true",
  "V8_EXTERNAL_CODE_SPACE_BOOL false",
  "COMPRESS_POINTERS_IN_CAGE_IMPL true",
  "COMPRESS_POINTERS_IN_SHARED_CAGE_BOOL false",
  "V8_DICT_PROPERTY_CONST_TRACKING_BOOL false",
  "V8_CAN_CREATE_SHARED_HEAP_BOOL false",
  "V8_STATIC_ROOTS_GENERATION_BOOL false",
  "V8_DEFAULT_STACK_SIZE_KB 984", // src/common/globals.h (非 ARM/IA32/ASan 分支)
  "V8_STACK_LIMIT_MARGIN_KB 40",
];

// ---------- 外部 constexpr 常量表 (从原版头文件人工核对抄录) ----------
// 名称 → { rust: Rust 字面量表达式, num: 数值 (C++ 常量折叠用), src: 出处 }
const CONST_MAP = {
  kDefaultMaxWasmCodeSpaceSizeMb: { rust: "128", num: 128, src: "src/common/globals.h (ARM64 分支 kDefaultMaxWasmCodeSpaceSizeMb)" },
  kMaxCommittedWasmCodeMB: { rust: "4095", num: 4095, src: "src/common/globals.h kMaxCommittedWasmCodeMB" },
  kMaxUInt32: { rust: "u32::MAX", num: 4294967295, src: "std::numeric_limits<uint32_t>::max()" },
  "wasm::kV8MaxWasmDefinedFunctions": { rust: "1_000_000", num: 1_000_000, src: "src/wasm/wasm-limits.h:31" },
  "wasm::kV8MaxWasmModuleSize": { rust: "1024 * 1024 * 1024", num: 1073741824, src: "src/wasm/wasm-limits.h:48" },
  "wasm::kV8MaxWasmTableSize": { rust: "10_000_000", num: 10_000_000, src: "src/wasm/wasm-limits.h:57" },
  "wasm::kGenericWrapperBudget": { rust: "1000", num: 1000, src: "src/wasm/wasm-constants.h:240" },
  KB: { rust: "1024", num: 1024, src: "src/base/bit-field.h KB" },
  MB: { rust: "1024 * 1024", num: 1048576, src: "src/base/bit-field.h MB" },
  GB: { rust: "1024 * 1024 * 1024", num: 1073741824, src: "src/base/bit-field.h GB" },
};

// ctype → [FlagType 变体名, Rust 类型]
const TYPE_OF = {
  bool: ["Bool", "bool"],
  "std::optional<bool>": ["MaybeBool", "Option<bool>"],
  int: ["Int", "i32"],
  "unsigned int": ["Uint", "u32"],
  uint64_t: ["Uint64", "u64"],
  double: ["Float", "f64"],
  size_t: ["SizeT", "usize"],
  "const char*": ["Str", "Option<&'static str>"],
};
const CTYPE2TAG = {
  bool: "Bool", "std::optional<bool>": "MaybeBool", int: "Int", "unsigned int": "Uint",
  uint64_t: "Uint64", double: "Float", size_t: "SizeT", "const char*": "Str",
};
// ctype → 原版 Flag::FlagType 的 TYPE_XXX 后缀 (FLAG_MODE_META 表核对用)
const CTYPE2FTYPE = {
  bool: "BOOL", "std::optional<bool>": "MAYBE_BOOL", int: "INT", "unsigned int": "UINT",
  uint64_t: "UINT64", double: "FLOAT", size_t: "SIZE_T", "const char*": "STRING",
};
// FlagType 变体名 → SCREAMING_SNAKE 计数后缀 (counts.rs / NUM_* 命名)
const TAG2SNAKE = {
  Bool: "BOOL", MaybeBool: "MAYBE_BOOL", Int: "INT", Uint: "UINT",
  Uint64: "UINT64", Float: "FLOAT", SizeT: "SIZE_T", Str: "STRING",
};
// 生成物里使用的类型化默认值访问器名后缀 (Flags::default_xxx)
const DEF_ACC = {
  Bool: "bool", MaybeBool: "mb", Int: "i32", Uint: "u32", Uint64: "u64",
  Float: "f64", SizeT: "usize", Str: "str",
};

const errors = [];

// ---------- clang -E 驱动 ----------
const tmp = mkdtempSync("/tmp/v8rs-flags-");
function preprocess(name, extra) {
  const drv = [
    "// v8rs driver (生成临时文件, 勿入库)",
    ...Object.entries(CONFIG).filter(([, v]) => v).map(([k]) => `#define ${k} 1`),
    ...CONFIG_EXTRA.map((d) => `#define ${d}`),
    extra,
    `#include "src/flags/flag-definitions.h"`,
  ].join("\n") + "\n";
  const file = join(tmp, `${name}.cpp`);
  writeFileSync(file, drv);
  const r = spawnSync("clang", ["-E", "-P", "-x", "c++", "-std=c++17", file, `-I`, process.cwd() + "/v8"], {
    encoding: "utf8", maxBuffer: 64 * 1024 * 1024,
  });
  if (r.status !== 0) throw new Error(`clang -E ${name} 失败:\n${r.stderr}`);
  const realErr = r.stderr.split("\n").filter((l) => l.includes("error"));
  if (realErr.length) throw new Error(`clang -E ${name} 有错误:\n${realErr.join("\n")}`);
  if (process.env.V8RS_DUMP) writeFileSync(join(process.env.V8RS_DUMP, `pass-${name}.txt`), r.stdout);
  return r.stdout;
}

// ---------- 词法小工具 ----------
// 跳过 C 字符串/字符字面量的扫描指针; 返回字面量结束后的位置
function scanStringLit(s, i) {
  // s[i] === '"'
  i++;
  while (i < s.length) {
    if (s[i] === "\\") { i += 2; continue; }
    if (s[i] === '"') return i + 1;
    i++;
  }
  throw new Error("未闭合字符串: " + s.slice(Math.max(0, i - 40), i + 10));
}
// 提取相邻字符串字面量并拼接 (C 翻译阶段 6 语义); 返回 { str, end }
function parseStringSeq(s, i) {
  let out = "";
  while (i < s.length) {
    while (i < s.length && /\s/.test(s[i])) i++;
    if (s[i] !== '"') break;
    let j = i + 1, buf = "";
    while (j < s.length) {
      if (s[j] === "\\") { buf += s[j] + s[j + 1]; j += 2; continue; }
      if (s[j] === '"') break;
      buf += s[j]; j++;
    }
    out += buf;
    i = j + 1;
  }
  return { str: out, end: i };
}
// 括号平衡切分顶层实参
function splitArgs(s, i) {
  // s[i] === '('
  const args = [];
  let depth = 0, cur = "";
  for (; i < s.length; i++) {
    const c = s[i];
    if (c === '"') { const e = scanStringLit(s, i); cur += s.slice(i, e); i = e - 1; continue; }
    if (c === "(" && depth === 0) { depth = 1; continue; } // 起始左括号不计入首个实参
    if (c === "(" || c === "[" || c === "{") depth++;
    else if (c === ")" || c === "]" || c === "}") { depth--; if (depth === 0) { if (cur.trim()) args.push(cur.trim()); return { args, end: i + 1 }; } }
    else if (c === "," && depth === 1) { args.push(cur.trim()); cur = ""; continue; }
    cur += c;
  }
  throw new Error("括号不平衡: " + s.slice(0, 60));
}
const cUnescape = (s) =>
  s.replace(/\\(.)/g, (_, c) => ({ n: "\n", t: "\t", r: "\r", "\\": "\\", '"': '"', "'": "'", "0": "\0" })[c] ?? c);
const rustLit = (s) => JSON.stringify(s);

// ---------- pass1 APPLY: 名称/主名/注释, 定义序 (含别名/readonly) ----------
const applySrc = preprocess("apply", `
#define FLAG_MODE_APPLY(ctype, nam, primary, cmt) V8RS_APPLY(#ctype, #nam, #primary, cmt);
#define FLAG_MODE_INCLUDE_READONLY
#define FLAG_MODE_INCLUDE_ALIASES
`);
const applyEntries = []; // { ctype, name, primary, comment|undefined }
{
  const re = /V8RS_APPLY\(/g; let m;
  while ((m = re.exec(applySrc))) {
    const { args, end } = splitArgs(applySrc, m.index + 10);
    re.lastIndex = end;
    const [ctypeQ, nameQ, primQ, cmt] = args;
    const ctype = ctypeQ.slice(1, -1), name = nameQ.slice(1, -1), primary = primQ.slice(1, -1);
    let comment;
    if (cmt.trim() === "nullptr" || cmt.trim() === "") comment = undefined;
    else comment = parseStringSeq(cmt, 0).str;
    if (!TYPE_OF[ctype]) errors.push(`未知 ctype: ${ctype} (${name})`);
    applyEntries.push({ ctype, name, primary, comment });
  }
}

// ---------- pass2 DEFAULTS: 名称 → 默认值文本 ----------
const defSrc = preprocess("defaults", `#define FLAG_MODE_DEFINE_DEFAULTS`);
const defaults = new Map();
{
  const re = /static constexpr ([^;]+?) FLAGDEFAULT_([a-zA-Z0-9_]+)\{/g; let m;
  while ((m = re.exec(defSrc))) {
    const ctype = m[1].trim(), name = m[2];
    const open = re.lastIndex - 1;
    let depth = 0, i = open, buf = "";
    for (; i < defSrc.length; i++) {
      const c = defSrc[i];
      if (c === '"') { const e = scanStringLit(defSrc, i); buf += defSrc.slice(i, e); i = e - 1; continue; }
      if (c === "{") { depth++; if (depth === 1) continue; }
      if (c === "}") { depth--; if (depth === 0) { i++; break; } }
      if (depth > 0) buf += c;
    }
    if (defaults.has(name)) errors.push(`重复默认值: ${name}`);
    defaults.set(name, { ctype, value: buf.trim() });
  }
}

// ---------- pass3 META: flags[] 顺序 + FlagType + readonly ----------
const metaSrc = preprocess("meta", `
#define Flag V8RSFLAGT_
#define v8_flags V8RSHOST_
#define FLAG_MODE_META
`);
const metaEntries = []; // { type, name, readonly }
{
  const re = /\{V8RSFLAGT_::TYPE_([A-Z0-9_]+), (?:&V8RSHOST_\s*\.\s*([a-zA-Z0-9_]+)|nullptr),\s*&FLAGDEFAULT_([a-zA-Z0-9_]+),/g; let m;
  while ((m = re.exec(metaSrc))) {
    metaEntries.push({ ftype: m[1], name: m[2] ?? m[3], readonly: !m[2] });
  }
}

// ---------- pass4 IMPLICATIONS ----------
const implSrc = preprocess("impl", `
#define V8_UNLIKELY(x) V8RS_UNLIKELY(x)
#define FlagError V8RS_FLAGERROR
#define UNREACHABLE() V8RS_UNREACHABLE_
#define PrintF V8RS_PRINTF
#define FLAG_MODE_DEFINE_IMPLICATIONS
`);

// ---------- 主表拼装 ----------
// 原版: flags[] = META 序 (primary, 含 readonly); kFlagsMetadata = APPLY 序 (primary+alias+readonly)
// FLAGS 必须逐条等于 META 序: Flag::name()/comment() 用 `this - flags` 当下标, 错位即全表错
const metaByName = new Map();
for (const e of metaEntries) {
  if (metaByName.has(e.name)) errors.push(`META 重名: ${e.name}`);
  metaByName.set(e.name, e);
}
const primaries = []; // { id, name, ctype, ftype, readonly, comment, defaultText }
{
  for (const a of applyEntries) {
    if (a.primary !== "") continue; // 别名后处理
    const meta = metaByName.get(a.name);
    if (!meta) { errors.push(`APPLY 主 flag 无 META 记录: ${a.name}`); continue; }
    const def = defaults.get(a.name);
    if (!def) { errors.push(`缺默认值: ${a.name}`); continue; }
    if (def.ctype !== a.ctype) errors.push(`DEFAULTS/APPLY 类型不符: ${a.name} ${def.ctype} vs ${a.ctype}`);
    if (CTYPE2FTYPE[a.ctype] !== meta.ftype) errors.push(`META FlagType 不符: ${a.name} TYPE_${meta.ftype} vs ${CTYPE2FTYPE[a.ctype]}`);
    primaries.push({
      id: primaries.length,
      name: a.name,
      ctype: a.ctype,
      ftype: meta.ftype,
      readonly: meta.readonly,
      comment: a.comment ?? "",
      defaultText: def.value,
    });
  }
}
// 顺序核对: APPLY 主 flag 序必须与 META flags[] 序逐位一致 (原版两处 #include 同一份声明序)
{
  if (metaEntries.length !== primaries.length) errors.push(`META 条目数 ${metaEntries.length} != APPLY 主 flag 数 ${primaries.length}`);
  const n = Math.min(metaEntries.length, primaries.length);
  let firstDiff = -1;
  for (let i = 0; i < n; i++) if (metaEntries[i].name !== primaries[i].name) { firstDiff = i; break; }
  if (firstDiff >= 0) {
    errors.push(`FLAGS 顺序与原版 flags[] 不符 @${firstDiff}: ${primaries[firstDiff].name} vs ${metaEntries[firstDiff].name}`);
  }
}
// 重名 + 原版 static_assert (#nam[0] != 'n' || #nam[1] != 'o') 的等价核对
{
  const seen = new Set();
  for (const p of primaries) {
    if (seen.has(p.name)) errors.push(`主 flag 重名: ${p.name}`);
    seen.add(p.name);
  }
  for (const a of applyEntries) {
    if (a.name.startsWith("no")) errors.push(`flag 名不得以 'no' 开头 (原版 kFlagsMetadata static_assert): ${a.name}`);
  }
}
const byName = new Map(primaries.map((p) => [p.name, p]));
const norm = (s) => s.replaceAll("_", "-");
// 归一化重名检测 (原版二进制搜索语义会歧义)
{
  const seen = new Map();
  for (const a of applyEntries) {
    const k = norm(a.name);
    if (seen.has(k)) errors.push(`归一化后重名: ${seen.get(k)} vs ${a.name}`);
    else seen.set(k, a.name);
  }
}

const aliases = applyEntries.filter((a) => a.primary !== "");
for (const a of aliases) {
  const p = byName.get(a.primary);
  if (!p) { // 主 flag 可能被配置裁剪 (如 alias 指向 #if 关闭的 flag) — 报错待人工裁
    errors.push(`别名 ${a.name} 的主 flag ${a.primary} 不在规范配置内`);
  }
}

// ---------- 默认值 → Rust 表达式 ----------
// 剥离显式类型转换: static_cast<T>(e) / (T)e / T{e} —— 语义即"同一数值换类型"
function stripCasts(t) {
  let s = String(t).trim();
  for (;;) {
    let m = /^static_cast\s*<[^<>]*>\s*\(([\s\S]*)\)$/.exec(s);
    if (m) { s = m[1].trim(); continue; }
    m = /^\(\s*(?:size_t|unsigned\s+int|signed\s+int|int|uint64_t|double|float|bool|char\s*\*|const\s+char\s*\*|u?int(?:8|16|32|64)_t)\s*\)\s*([\s\S]+)$/.exec(s);
    if (m) { s = m[1].trim(); continue; }
    m = /^(?:size_t|int|unsigned\s+int|uint64_t|double|bool|std::optional<\s*bool\s*>)\s*\{([\s\S]*)\}$/.exec(s);
    if (m) { s = m[1].trim(); continue; }
    break;
  }
  return s;
}
function defaultExpr(p) {
  const tag = CTYPE2TAG[p.ctype];
  const text = stripCasts(p.defaultText);
  switch (tag) {
    case "Bool": {
      const c = cEvalConst(text, `bool ${p.name}`);
      if (c === null) return null;
      return `FlagValue::Bool(${c.v !== 0})`;
    }
    case "MaybeBool":
      if (text === "std::nullopt" || text === "") return "FlagValue::MaybeBool(None)";
      errors.push(`默认值无法解析 maybe_bool ${p.name}: ${text}`);
      return null;
    case "Int": {
      const c = cEvalConst(text, `int ${p.name}`);
      if (c === null) return null;
      if (c.v < -2147483648 || c.v > 2147483647) errors.push(`int 越界 ${p.name}: ${text}`);
      return `FlagValue::Int(${fmtInt(c.v)}i32)`;
    }
    case "Uint": {
      const c = cEvalConst(text, `unsigned int ${p.name}`);
      if (c === null) return null;
      if (c.v < 0 || c.v > 0xffffffff) errors.push(`uint 越界 ${p.name}: ${text}`);
      return `FlagValue::Uint(${fmtInt(c.v)}u32)`;
    }
    case "Uint64": {
      const c = cEvalConst(text, `uint64_t ${p.name}`);
      if (c === null) return null;
      if (c.v < 0) errors.push(`uint64 负值 ${p.name}: ${text}`);
      return `FlagValue::Uint64(${fmtInt(c.v)}u64)`;
    }
    case "SizeT": {
      const c = cEvalConst(text, `size_t ${p.name}`);
      if (c === null) return null;
      if (c.v < 0) errors.push(`size_t 负值 ${p.name}: ${text}`);
      return `FlagValue::SizeT(${fmtInt(c.v)}usize)`;
    }
    case "Float": {
      const c = cEvalConst(text, `double ${p.name}`);
      if (c === null) return null;
      return `FlagValue::Float(${rustFloat(c.v)})`;
    }
    case "Str": {
      if (text === "nullptr") return "FlagValue::Str(None)";
      const { str, end } = parseStringSeq(text, 0);
      if (text.slice(end).trim() !== "") errors.push(`str 默认值尾部残留 ${p.name}: ${text}`);
      return `FlagValue::Str(Some(Cow::Borrowed(${rustLit(cUnescape(str))})))`;
    }
  }
  errors.push(`未知类型默认值 ${p.name}: ${p.ctype}`);
  return null;
}
const fmtInt = (v) => (Number.isInteger(v) ? String(v) : (errors.push(`非整数常量 ${v}`), String(Math.trunc(v))));
const rustFloat = (v) => {
  let s = String(v);
  if (!s.includes(".") && !s.includes("e") && !s.includes("inf") && !s.includes("NaN")) s += ".0";
  return s;
};
// C++ 常量表达式求值 (与运行时同一套 tokenizer/parser, 保证语义一致)
// 返回 { v: number, kind: "int"|"float"|"bool" }, 失败返回 null 并记录硬错误
function cEvalConst(text, ctxName) {
  let node;
  try {
    node = parseC(tokenizeC(text));
  } catch (e) {
    errors.push(`常量表达式无法解析 [${ctxName}]: ${text} :: ${e.message}`);
    return null;
  }
  const intOp = (op, a, b) => {
    const A = BigInt(Math.trunc(a)), B = BigInt(Math.trunc(b));
    switch (op) {
      case "&": return Number(A & B);
      case "|": return Number(A | B);
      case "^": return Number(A ^ B);
      case "<<": return Number(A << B);
      case ">>": return Number(A >> B);
    }
    return null;
  };
  const ev = (n) => {
    switch (n.op) {
      case "num": {
        const s = String(n.v).replace(/'/g, "");
        if (/^0[xX]/.test(s)) return { v: parseInt(s, 16), kind: "int" };
        if (/[.]/.test(s) || /e[0-9+-]/i.test(s)) return { v: parseFloat(s), kind: "float" };
        return { v: parseInt(s.replace(/[uUlL]+$/, ""), 10), kind: "int" };
      }
      case "id": {
        if (n.v === "true") return { v: 1, kind: "bool" };
        if (n.v === "false") return { v: 0, kind: "bool" };
        const k = CONST_MAP[n.v];
        if (k) return { v: k.num, kind: Number.isInteger(k.num) ? "int" : "float" };
        errors.push(`常量表达式含未知标识符 [${ctxName}]: ${n.v} :: ${text}`);
        return null;
      }
      case "u!": { const a = ev(n.e); return a ? { v: a.v === 0 ? 1 : 0, kind: "bool" } : null; }
      case "u-": { const a = ev(n.e); return a ? { v: -a.v, kind: a.kind } : null; }
      case "u+": return ev(n.e);
      case "u~": { const a = ev(n.e); return a ? { v: intOp("^", a.v, -1), kind: "int" } : null; }
      case "&&": { const a = ev(n.lhs), b = ev(n.rhs); return a && b ? { v: a.v !== 0 && b.v !== 0 ? 1 : 0, kind: "bool" } : null; }
      case "||": { const a = ev(n.lhs), b = ev(n.rhs); return a && b ? { v: a.v !== 0 || b.v !== 0 ? 1 : 0, kind: "bool" } : null; }
      case "&": case "|": case "^": case "<<": case ">>": {
        const a = ev(n.lhs), b = ev(n.rhs);
        if (!a || !b) return null;
        const v = intOp(n.op, a.v, b.v);
        if (v === null) { errors.push(`常量表达式不支持位运算 [${ctxName}]: ${n.op}`); return null; }
        return { v, kind: "int" };
      }
      case "==": case "!=": case "<": case "<=": case ">": case ">=": {
        const a = ev(n.lhs), b = ev(n.rhs);
        if (!a || !b) return null;
        const r = { "==": a.v === b.v, "!=": a.v !== b.v, "<": a.v < b.v, "<=": a.v <= b.v, ">": a.v > b.v, ">=": a.v >= b.v }[n.op];
        return { v: r ? 1 : 0, kind: "bool" };
      }
      case "+": case "-": case "*": case "/": case "%": {
        const a = ev(n.lhs), b = ev(n.rhs);
        if (!a || !b) return null;
        const fl = a.kind === "float" || b.kind === "float";
        let v;
        switch (n.op) {
          case "+": v = a.v + b.v; break;
          case "-": v = a.v - b.v; break;
          case "*": v = a.v * b.v; break;
          case "/": v = fl ? a.v / b.v : Math.trunc(a.v / b.v); break;
          case "%": v = fl ? a.v % b.v : Math.trunc(a.v) % Math.trunc(b.v); break;
        }
        return { v, kind: fl ? "float" : "int" };
      }
      default:
        errors.push(`常量表达式不支持的节点 [${ctxName}]: ${n.op} :: ${text}`);
        return null;
    }
  };
  const r = ev(node);
  if (!r) return null;
  if (!Number.isFinite(r.v)) { errors.push(`常量表达式结果非有限数 [${ctxName}]: ${text}`); return null; }
  return r;
}

// ---------- pass4 解析: 语句树 ----------

// 顶层语句切分: ';' 结束表达式语句, 顶层 '}' 结束块语句 (并吞掉 do-while 尾与 else 分支)
function matchBalanced(src, i) {
  // src[i] === '{' → 返回匹配的 '}' 之后位置
  let d = 0;
  for (; i < src.length; i++) {
    const c = src[i];
    if (c === '"') { i = scanStringLit(src, i) - 1; continue; }
    if (c === "{") d++;
    else if (c === "}") { d--; if (d === 0) return i + 1; }
  }
  throw new Error("大括号不平衡");
}
function matchCall(src, i) {
  // src[i] === '(' → 返回匹配的 ')' 之后位置
  let d = 0;
  for (; i < src.length; i++) {
    const c = src[i];
    if (c === '"') { i = scanStringLit(src, i) - 1; continue; }
    if (c === "(") d++;
    else if (c === ")") { d--; if (d === 0) return i + 1; }
  }
  throw new Error("圆括号不平衡");
}
function topLevelStatements(src) {
  const stmts = [];
  let buf = "", depthP = 0, depthB = 0;
  const braceInit = []; // 每层花括号是否为"初值/空括号" (如 FlagError{}), 而非语句块
  for (let i = 0; i < src.length; i++) {
    const c = src[i];
    if (c === '"') { const e = scanStringLit(src, i); buf += src.slice(i, e); i = e - 1; continue; }
    if (c === "'" && /^[0-9a-zA-Z\\']/.test(src[i + 1] || "")) {
      let j = i + 1;
      while (j < src.length && src[j] !== "'") { if (src[j] === "\\") j++; j++; }
      buf += src.slice(i, j + 1); i = j; continue;
    }
    if (c === "(") depthP++;
    if (c === ")") depthP--;
    if (c === "{") {
      // 区分块起始与花括号初值 (如 `FlagError{}`): 紧贴标识符且无空格 → 初值, 不构成语句块
      const prev = i > 0 ? src[i - 1] : " ";
      braceInit.push(!/\s/.test(prev) && /[A-Za-z0-9_>]/.test(prev));
      if (!braceInit[braceInit.length - 1]) depthB++;
      buf += c;
      continue;
    }
    if (c === "}") {
      const wasInit = braceInit.pop();
      if (wasInit) { buf += c; continue; }
      depthB--;
      buf += c;
      if (depthB === 0 && depthP === 0) {
        // 块语句结束; 尾部可能接 while (...) ; (do-while) 或 else 分支
        for (;;) {
          const rest = src.slice(i + 1);
          const wm = /^\s*while\s*\(/.exec(rest);
          if (wm) {
            let j = i + 1 + wm[0].length - 1; // 指向 '('
            const end = matchCall(src, j);
            let k = end;
            while (k < src.length && (/\s/.test(src[k]) || src[k] === ";")) k++;
            buf += src.slice(i + 1, k);
            i = k - 1;
            continue;
          }
          const em = /^\s*else\s*\{/.exec(rest);
          if (em) {
            const j = i + 1 + em[0].length - 1; // 指向 '{'
            const end = matchBalanced(src, j);
            buf += src.slice(i + 1, end);
            i = end - 1;
            continue;
          }
          const es = /^\s*else\s+/.exec(rest);
          if (es) {
            let j = i + 1 + es[0].length;
            while (j < src.length && src[j] === "{") j = matchBalanced(src, j);
            while (j < src.length && src[j] !== ";") j++;
            buf += src.slice(i + 1, Math.min(j + 1, src.length));
            i = j;
            continue;
          }
          break;
        }
        stmts.push(buf);
        buf = "";
      }
      continue;
    }
    buf += c;
    if (c === ";" && depthP === 0 && depthB === 0) { stmts.push(buf); buf = ""; }
  }
  if (buf.trim()) stmts.push(buf);
  return stmts;
}

const implications = []; // { guard: [cond...], entry }
{
  const stmts = topLevelStatements(implSrc);
  if (process.env.V8RS_INSPECT) writeFileSync(process.env.V8RS_INSPECT, stmts.map((s) => s.trim()).join("\n/*==*/\n"));
  const walk = (text, guards) => {
    const t = text.trim();
    if (!t) return;
    if (/^if\s*\(/.test(t)) {
      // if (COND) BODY; 或 if (COND) { ... }
      const p = t.indexOf("(");
      let depth = 0, condEnd = -1;
      for (let i = p; i < t.length; i++) {
        const c = t[i];
        if (c === '"') { i = scanStringLit(t, i) - 1; continue; }
        if (c === "(") depth++;
        if (c === ")") { depth--; if (depth === 0) { condEnd = i; break; } }
      }
      const cond = t.slice(p + 1, condEnd).trim();
      let rest = t.slice(condEnd + 1).trim();
      if (rest.startsWith("{")) {
        // 用平衡匹配取块体 (lastIndexOf 会把后续语句吞进来)
        let d = 0, bodyEnd = -1;
        for (let i = 0; i < rest.length; i++) {
          const c = rest[i];
          if (c === '"') { i = scanStringLit(rest, i) - 1; continue; }
          if (c === "{") d++;
          else if (c === "}") { d--; if (d === 0) { bodyEnd = i; break; } }
        }
        if (bodyEnd < 0) { errors.push(`if 块不平衡: ${t.slice(0, 80)}`); return; }
        const body = rest.slice(1, bodyEnd);
        const tail = rest.slice(bodyEnd + 1).replace(/^;\s*/, "").trim();
        for (const sub of topLevelStatements(body)) walk(sub, [...guards, cond]);
        if (tail) walk(tail, guards); // 例如 else/后续顶层语句 (未识别则硬报错)
      } else {
        walk(rest, [...guards, cond]);
      }
      return;
    }
    if (/^do\s*\{/.test(t)) {
      let d = 0, bodyEnd = -1;
      for (let i = 2; i < t.length; i++) {
        const c = t[i];
        if (c === '"') { i = scanStringLit(t, i) - 1; continue; }
        if (c === "{") d++;
        else if (c === "}") { d--; if (d === 0) { bodyEnd = i; break; } }
      }
      if (bodyEnd < 0) { errors.push(`do 块不平衡: ${t.slice(0, 80)}`); return; }
      const body = t.slice(t.indexOf("{") + 1, bodyEnd); // 跳过大括号本身
      for (const sub of topLevelStatements(body)) walk(sub, guards);
      const tail = t.slice(bodyEnd + 1).replace(/^\s*while\s*\([^)]*\)\s*;?\s*/, "").trim();
      if (tail) walk(tail, guards);
      return;
    }
    if (/^changed\s*\|=/.test(t)) {
      const inner = t.replace(/^changed\s*\|=/, "").trim();
      parseTrigger(inner, guards);
      return;
    }
    if (t.startsWith("V8RS_PRINTF")) {
      parseWarn(t, guards);
      return;
    }
    if (t.startsWith("TracingFlags::")) {
      parseGeneric(t, guards);
      return;
    }
    if (t.startsWith("V8RS_FLAGERROR")) {
      parseRequire(t, guards);
      return;
    }
    if (t === ";" ) return;
    // UNREACHABLE() 宏的展开 (V8RS_UNREACHABLE_;) 无语义, 忽略
    if (/^V8RS_UNREACHABLE_;?$/.test(t)) return;
    errors.push(`implication 未识别语句: ${t.slice(0, 120)}`);
  };
  const findCall = (s, name) => {
    const idx = s.indexOf(name + "(");
    return idx;
  };
  const parseTrigger = (inner, guards) => {
    const ci = findCall(inner, "TriggerImplication");
    if (ci < 0) { errors.push(`changed|= 中无 TriggerImplication: ${inner.slice(0, 100)}`); return; }
    const { args } = splitArgs(inner, ci + "TriggerImplication".length);
    if (args.length !== 6) { errors.push(`TriggerImplication 参数数异常: ${args.length}: ${inner.slice(0, 100)}`); return; }
    const [cond, premName, conclPtr, conclName, value, strength] = args;
    // 拼 premise 名 (相邻字面量)
    const { str: premStr, end } = parseStringSeq(premName, 0);
    const tail = premName.slice(end).trim();
    let premiseName;
    if (tail.startsWith("NegValuePremiseName")) {
      const { args: na } = splitArgs(tail, tail.indexOf("("));
      const whenval = evalWhen(na[0]);
      const nm = na[1].slice(1, -1);
      const neg = na[2] ? parseStringSeq(na[2], 0).str : "!" + nm;
      premiseName = whenval ? neg : nm;
    } else if (tail !== "") {
      errors.push(`premise 名尾部异常: ${tail.slice(0, 60)}`);
      premiseName = premStr;
    } else premiseName = premStr;
    const tm = /v8_flags\s*\.\s*([a-zA-Z0-9_]+)/.exec(conclPtr);
    const cnm = conclName.slice(1, -1);
    if (!tm || tm[1] !== cnm) errors.push(`conclusion 指针/名不符: ${conclPtr} ${conclName}`);
    const target = byName.get(cnm);
    if (!target) { errors.push(`conclusion flag 不存在: ${cnm}`); return; }
    implications.push({ kind: "value", guards, cond, premiseName, target, value, strength: strength.includes("kWeak") ? "Weak" : "Strong" });
  };
  const evalWhen = (tok) => {
    const m = /^FLAGDEFAULT_([a-zA-Z0-9_]+)$/.exec(tok.trim());
    if (m) { const p = byName.get(m[1]); const d = p && defaults.get(p.name)?.value; return d === "true"; }
    return tok.trim() === "true";
  };
  const parseWarn = (t, guards) => {
    // 展开形如: if (A && B) { V8RS_PRINTF(stderr, "Warning: ..." "..."); }
    // 条件在 walk 的 guards 里, 本函数只取消息文本 (args[0]=stderr, 其余为相邻字面量)
    const { args } = splitArgs(t, t.indexOf("("));
    const msg = args.slice(1).map((a) => parseStringSeq(a.trim(), 0).str).join("");
    implications.push({ kind: "warn", guards, cond: null, msgText: msg });
  };
  const parseGeneric = (t, guards) => {
    const m = /^TracingFlags::([a-z_]+)\.store\(/.exec(t.trim());
    if (!m) { errors.push(`generic 形式异常: ${t.slice(0, 80)}`); return; }
    implications.push({ kind: "generic", guards, cond: null, tracing: m[1] });
  };
  const parseRequire = (t, guards) => {
    const m = /V8RS_FLAGERROR\s*\{\}\s*<<\s*(.*)$/.exec(t.trim());
    if (!m) { errors.push(`require 形式异常: ${t.slice(0, 80)}`); return; }
    const { str } = parseStringSeq(m[1].replace(/;\s*$/, ""), 0);
    implications.push({ kind: "require", guards, stmt: str });
  };
  // if (cond) X; 的 X 已被 walk 递归; 但 V8RS_PRINTF 出现在 { } 内 → guards=[cond]
  // 而 generic/require 的 cond 在 guards 里; 若 guards 为空: 单条语句 (不应发生)
  for (const s of stmts) walk(s, []);
}
// 归并 guard: 顶层 if 链的条件全部并入前提 (字面量 `true` 的 guard 无信息, 丢弃)
for (const it of implications) {
  const gs = it.guards.filter((g) => g.trim() !== "true");
  delete it.guards;
  if (it.kind === "require") {
    // DEFINE_REQUIREMENT 自身的 `if (V8_UNLIKELY(!(statement)))` 包装不是外部前提,
    // 违规条件由 statement 取反得到 (driver 把 V8_UNLIKELY 改名为 V8RS_UNLIKELY)
    it.conds = gs.filter((g) => !/^V8(RS)?_UNLIKELY\b/.test(g.trim()));
  } else if (it.kind === "value") {
    it.conds = [it.cond, ...gs].filter((c) => c && c.trim() !== "true");
  } else {
    it.conds = gs;
  }
}

// ---------- 表达式 C++ → Rust ----------
const flagTypeOf = new Map(primaries.map((p) => [p.name, p]));
const KEYWORDS = new Set(["as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "box", "do", "final", "macro", "override", "priv", "try", "yield", "abstract", "become", "virtual", "proc"]);
const mname = (s) => { const low = s.toLowerCase(); return KEYWORDS.has(low) ? `r#${low}` : low; };

function tokenizeC(input) {
  // 折叠 `::` 两侧空白, 保证 "std :: nullopt" 与 "std::nullopt" 同形
  const code = input.replace(/\s*::\s*/g, "::");
  const toks = [];
  let i = 0;
  while (i < code.length) {
    const c = code[i];
    if (/\s/.test(c)) { i++; continue; }
    if (c === '"') {
      const { str, end } = parseStringSeq(code, i);
      toks.push({ k: "str", v: str });
      i = end;
      continue;
    }
    if (/[A-Za-z_]/.test(c)) {
      let j = i;
      while (j < code.length && /[A-Za-z0-9_:]/.test(code[j])) {
        // :: 连写
        if (code[j] === ":") { if (code[j + 1] === ":") { j += 2; continue; } break; }
        j++;
      }
      toks.push({ k: "id", v: code.slice(i, j) });
      i = j;
      continue;
    }
    if (/[0-9]/.test(c) || (c === "." && /[0-9]/.test(code[i + 1]))) {
      let j = i;
      while (j < code.length && /[0-9a-fA-F'.]/.test(code[j]) && !(code[j] === "'" && code[j + 1] === "'")) j++;
      // 处理 1'000 / 8'000u / 0x1p4 等: 简单起见读整段含后缀
      let s = code.slice(i, j);
      // 若前面以 0x 开头, 允许继续读字母数字
      if (/^0[xX]/.test(s)) { while (j < code.length && /[0-9a-fApP.]/i.test(code[j])) j++; s = code.slice(i, j); }
      else if (/[eE]$/.test(s) && /[0-9]/.test(s.replace(/[^0-9]/g, ""))) {
        // 十进制科学计数法: 上述字符类 (a-f 含 e) 会把 3e-10 的 'e' 吞进尾而停在 '-'/'+',
        // 若其后是指数 (可选符号+数字) 则续读为完整数字, 否则硬报错 (裸 'e' 结尾必为错切)
        let k = j;
        if (k < code.length && (code[k] === "+" || code[k] === "-")) k++;
        if (k < code.length && /[0-9]/.test(code[k])) {
          while (k < code.length && /[0-9]/.test(code[k])) k++;
          s = code.slice(i, k);
          j = k;
          // 科学计数法不含 0x 前缀, u/l 后缀仍可跟 (3e-10f)
          while (j < code.length && /[uUlLfF]/.test(code[j])) { j++; s += code[j - 1]; }
        } else {
          errors.push(`数字以 'e' 结尾且非科学计数法: ${s} @ ${code.slice(Math.max(0, i - 20), i + 20)}`);
        }
      }
      // 若下一字符是字母后缀 (u/UL/f) 吃掉
      while (j < code.length && /[uUlLfF]/.test(code[j])) { j++; s += code[j - 1]; }
      toks.push({ k: "num", v: s });
      i = j;
      continue;
    }
    const three = code.slice(i, i + 3);
    if (three === "<<=" || three === ">>=") { toks.push({ k: "op", v: three }); i += 3; continue; }
    const two = code.slice(i, i + 2);
    if (["&&", "||", "==", "!=", "<=", ">=", "++", "--", "::"].includes(two)) { toks.push({ k: "op", v: two }); i += 2; continue; }
    if ("+-*/%<>!=()&|^~?:.,".includes(c)) { toks.push({ k: "op", v: c }); i++; continue; }
    errors.push(`tokenize 未知字符 ${c} @ ${code.slice(Math.max(0, i - 30), i + 30)}`);
    i++;
  }
  return toks;
}
// 递归下降 → AST {op, ...} ; 类型推断: bool / num(i) / f / str / flag(name)
const PREC = { "||": 1, "&&": 2, "|": 3, "^": 4, "&": 5, "==": 6, "!=": 6, "<": 7, ">": 7, "<=": 7, ">=": 7, "<<": 8, ">>": 8, "+": 9, "-": 9, "*": 10, "/": 10, "%": 10 };
function parseC(toks) {
  let pos = 0;
  const peek = () => toks[pos];
  const eat = (v) => { const t = toks[pos]; if (!t || (v && !(t.k === "op" && t.v === v))) return null; pos++; return t; };
  const expect = (v) => { const t = eat(v); if (!t) throw new Error(`期望 ${v} @ ${JSON.stringify(toks.slice(pos - 2, pos + 2))}`); return t; };
  const parseExpr = () => parseTernary();
  function parseTernary() {
    const c = parseBinary(1);
    if (peek()?.k === "op" && peek().v === "?") {
      pos++;
      const a = parseTernary();
      expect(":");
      const b = parseTernary();
      return { op: "?", c, a, b };
    }
    return c;
  }
  function parseBinary(minPrec) {
    let lhs = parseUnary();
    for (;;) {
      const t = peek();
      if (!t || t.k !== "op" || !(t.v in PREC) || PREC[t.v] < minPrec) break;
      pos++;
      const rhs = parseBinary(PREC[t.v] + 1);
      lhs = { op: t.v, lhs, rhs };
    }
    return lhs;
  }
  function parseUnary() {
    const t = peek();
    if (t && t.k === "op" && ["!", "-", "+", "~", "++", "--"].includes(t.v)) {
      pos++;
      if (t.v === "++" || t.v === "--") throw new Error("不支持 ++/--");
      return { op: "u" + t.v, e: parseUnary() };
    }
    return parsePostfix();
  }
  function parsePostfix() {
    let e = parsePrimary();
    for (;;) {
      const t = peek();
      if (t?.k === "op" && t.v === "." ) { pos++; const n = toks[pos++]; if (!n || n.k !== "id") throw new Error("点号后非标识符"); e = { op: ".", e, name: n.v }; continue; }
      if (t?.k === "op" && t.v === "(") { pos++; const args = []; if (!(peek()?.k === "op" && peek().v === ")")) { for (;;) { args.push(parseExpr()); if (peek()?.k === "op" && peek().v === ",") { pos++; continue; } break; } } expect(")"); e = { op: "call", e, args }; continue; }
      break;
    }
    return e;
  }
  function parsePrimary() {
    const t = toks[pos];
    if (!t) throw new Error("表达式提前结束");
    if (t.k === "op" && t.v === "(") { pos++; const e = parseExpr(); expect(")"); return e; }
    if (t.k === "num") { pos++; return { op: "num", v: t.v }; }
    if (t.k === "str") { pos++; return { op: "str", v: t.v }; }
    if (t.k === "id") { pos++; return { op: "id", v: t.v }; }
    throw new Error(`表达式起始异常: ${JSON.stringify(t)}`);
  }
  const e = parseExpr();
  if (pos !== toks.length) throw new Error(`表达式未耗尽: ${JSON.stringify(toks.slice(pos, pos + 4))}`);
  return e;
}

// 表达式 → Rust; ctx.kind: "bool"/"num"/"float"/"str" 期望
function toRust(node) {
  // 返回 { src, kind } kind ∈ bool|num|float|str
  if (node.op === "num") {
    const s = node.v.replace(/'/g, "");
    if (/^0[xX]/.test(s)) return { src: `${parseInt(s, 16)}`, kind: "num" };
    if (/[.eE]/.test(s) && !/[uUlL]/.test(s)) return { src: rustFloat(parseFloat(s)), kind: "float" };
    return { src: s.replace(/[uUlL]+$/, ""), kind: "num" };
  }
  if (node.op === "str") return { src: rustLit(node.v), kind: "str" };
  if (node.op === "id") {
    const v = node.v;
    if (v === "true" || v === "false") return { src: v, kind: "bool" };
    if (flagTypeOf.has(v)) { const p = flagTypeOf.get(v); return { src: `f.${mname(v)}()`, kind: kindOfTag(CTYPE2TAG[p.ctype]), flag: v }; }
    const fdm = /^FLAGDEFAULT_([a-zA-Z0-9_]+)$/.exec(v);
    if (fdm) {
      const p = flagTypeOf.get(fdm[1]);
      if (!p) { errors.push(`未知 FLAGDEFAULT: ${v}`); return { src: "false", kind: "bool", failed: true }; }
      const tag = CTYPE2TAG[p.ctype];
      return { src: `f.default_${DEF_ACC[tag]}(${p.id})`, kind: kindOfTag(tag) };
    }
    const k = CONST_MAP[v] ?? CONST_MAP[v.replace(/\s+/g, "")];
    if (k) return { src: `(${k.rust})`, kind: /1024 \* 1024 \* 1024/.test(k.rust) ? "num" : "num" };
    if (v === "nullptr") return { src: "None", kind: "str" };
    if (v === "std::nullopt") return { src: "None", kind: "mb" };
    errors.push(`未知标识符: ${v}`);
    return { src: v, kind: "num" };
  }
  if (node.op === "u!") { const e = toRust(node.e); if (e.kind === "bool") return { src: `!${paren(e.src)}`, kind: "bool" }; if (e.kind === "mb") return { src: `!${paren(e.src)}`, kind: "bool" }; if (e.kind === "str") return { src: `${paren(e.src)}.is_none()`, kind: "bool" }; return { src: `${paren(e.src)} == 0`, kind: "bool" }; }
  if (node.op === "u-") { const e = toRust(node.e); return { src: `-${paren(e.src)}`, kind: e.kind === "float" ? "float" : "num" }; }
  if (node.op === "u+") return toRust(node.e);
  if (node.op === ".") {
    // v8_flags.x → 类型化访问器; v8_flags.x.value (无调用) → 同值
    const base = calleeName(node.e);
    if (base === "v8_flags") {
      const p = flagTypeOf.get(node.name);
      if (!p) { errors.push(`未知 v8_flags 访问: v8_flags.${node.name}`); return { src: "false", kind: "bool", failed: true }; }
      return { src: `f.${mname(node.name)}()`, kind: kindOfTag(CTYPE2TAG[p.ctype]) };
    }
    if (base && base.startsWith("v8_flags.") && node.name === "value") return toRust(node.e);
    if (base && base.startsWith("v8_flags.")) return toRust(node.e);
    return toRust(node.e);
  }
  if (node.op === "call") {
    const callee = node.e;
    // 归一 callee 名: id 或 id.id...
    const name = calleeName(callee);
    // flag 访问 (可能带 .value 后缀, 也可能裸名 —— flag-definitions.h 内两者混用)
    {
      let s = name ?? "";
      if (s.endsWith(".value")) s = s.slice(0, -".value".length);
      if (s.startsWith("v8_flags.")) s = s.slice("v8_flags.".length);
      const p = flagTypeOf.get(s);
      if (p && node.args.length === 0) return { src: `f.${mname(s)}()`, kind: kindOfTag(CTYPE2TAG[p.ctype]) };
    }
    if (name === "V8RS_UNLIKELY" || name === "V8RS_LIKELY") return toRust(node.args[0]);
    if (name === "FlagValueEquals") {
      const a = toRust(node.args[0]), b = toRust(node.args[1]);
      return { src: `${cmpEq(a, b)}`, kind: "bool" };
    }
    if (name === "base::bits::IsPowerOfTwo") {
      const a = toRust(node.args[0]);
      return { src: `is_pow2(${asU64(a)})`, kind: "bool" };
    }
    if (name === "Internals::IsValidSmi") {
      const a = toRust(node.args[0]);
      return { src: `is_valid_smi(${asI64(a)})`, kind: "bool" };
    }
    if (/^TracingFlags::[a-z_]+\.store$/.test(name ?? "")) {
      return { src: `TRACE_EFFECT("${name.split("::")[1]}")`, kind: "effect" };
    }
    if (name === "NegValuePremiseName") { return { src: "()", kind: "bool" }; }
    errors.push(`未知调用: ${name} (${node.args.length} args)`);
    return { src: "?", kind: "bool" };
  }
  if (node.op === "?") {
    const c = coerce(toRust(node.c), "bool");
    const a = toRust(node.a), b = toRust(node.b);
    if (a.kind === "str" || b.kind === "str") return { src: `(if ${c.src} { ${a.src} } else { ${b.src} })`, kind: "str" };
    const num = unify(a, b);
    return { src: `(if ${c.src} { ${num.a} } else { ${num.b} })`, kind: num.kind };
  }
  if (["&&", "||"].includes(node.op)) {
    const a = coerce(toRust(node.lhs), "bool"), b = coerce(toRust(node.rhs), "bool");
    return { src: `${paren(a.src)} ${node.op} ${paren(b.src)}`, kind: "bool" };
  }
  if (["==", "!=", "<", "<=", ">", ">="].includes(node.op)) {
    const L = toRust(node.lhs), R = toRust(node.rhs);
    // 字符串比较 (Option<&str> vs str)
    if (L.kind === "str" || R.kind === "str") {
      const l = asOptStr(L), r = asOptStr(R);
      const op = node.op === "==" ? "==" : node.op === "!=" ? "!=" : null;
      if (op) return { src: `${paren(l)} ${op} ${paren(r)}`, kind: "bool" };
      errors.push(`字符串不支持比较 ${node.op}`);
      return { src: "?", kind: "bool" };
    }
    const u = unify(L, R);
    const op = node.op;
    return { src: `${paren(u.a)} ${op} ${paren(u.b)}`, kind: "bool" };
  }
  if (["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>"].includes(node.op)) {
    const L = toRust(node.lhs), R = toRust(node.rhs);
    if (node.op === "&" || node.op === "|" || node.op === "^" || node.op === "<<" || node.op === ">>") {
      if (L.kind === "bool" && R.kind === "bool" && node.op !== ">>" && node.op !== "<<") {
        errors.push(`bool 位运算 (C++ 与 || 混用?), 人工核对: ${node.op}`);
      }
      return { src: `${asI64(L)} ${node.op} ${bitShift(R, node.op)}`, kind: "num" };
    }
    const u = unify(L, R);
    return { src: `${paren(u.a)} ${node.op} ${paren(u.b)}`, kind: u.kind };
  }
  errors.push(`未知表达式节点: ${node.op}`);
  return { src: "?", kind: "bool" };
}
function calleeName(n) {
  if (n.op === "id") return n.v;
  if (n.op === ".") { const base = calleeName(n.e); return base ? `${base}.${n.name}` : null; }
  return null;
}
// 只在「原子」或「整体已被一对括号完整包住」时省略括号。
// 旧实现只看首字符是 '(' , 会把 `(a) == (b)` 误判为自带括号, 导致 `!` 只否定左半 (语义错)。
function isSelfWrapped(s) {
  if (!s.startsWith("(")) return false;
  let depth = 0;
  for (let i = 0; i < s.length; i++) {
    const c = s[i];
    if (c === "(") depth++;
    else if (c === ")") { depth--; if (depth === 0) return i === s.length - 1; }
  }
  return false;
}
function paren(s) {
  if (/^-?[\d.]+[a-zA-Z_0-9']*u?[lLfF]*$/.test(s)) return s;                 // 字面量
  if (/^"[^"]*"$/.test(s)) return s;                                         // 字符串字面量
  if (/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(s)) return s;                          // 标识符
  if (/^[a-zA-Z_][a-zA-Z0-9_.]*(::[a-zA-Z_][a-zA-Z0-9_]*)*\(\)$/.test(s)) return s; // 无参调用
  if (isSelfWrapped(s)) return s;
  if (isSingleCall(s)) return s;                                             // 整串就是一次带参调用
  return `(${s})`;
}
// `foo.bar(a)` / `crate::x(a)` 这类整体是单个调用的串是原子的, 不必再加括号
function isSingleCall(s) {
  if (!/^[a-zA-Z_][a-zA-Z0-9_.]*(::[a-zA-Z_][a-zA-Z0-9_]*)*\(/.test(s) || !s.endsWith(")")) return false;
  let depth = 0;
  for (let i = 0; i < s.length; i++) {
    const c = s[i];
    if (c === "(") depth++;
    else if (c === ")") { depth--; if (depth === 0) return i === s.length - 1; }
  }
  return false;
}
function kindOfTag(t) { return { Bool: "bool", MaybeBool: "mb", Int: "num", Uint: "num", Uint64: "num", SizeT: "num", Float: "float", Str: "str" }[t]; }
function unify(L, R) {
  if (L.kind === "float" || R.kind === "float") {
    if (L.kind === "bool") errors.push("bool→float 未处理");
    return { a: asF64(L), b: asF64(R), kind: "float" };
  }
  if (L.kind === "num" && R.kind === "num") return { a: L.src, b: R.src, kind: "num" };
  if (L.kind === "bool" || R.kind === "bool") {
    // bool 参与数值比较: 极少; 转 u8
    return { a: asI64(L), b: asI64(R), kind: "num" };
  }
  if (L.kind === "mb" || R.kind === "mb") return { a: L.src, b: R.src, kind: "mb" };
  if (L.kind === "str" && R.kind === "str") return { a: L.src, b: R.src, kind: "str" };
  // num vs mb (maybe_bool == bool 情况?) 或未知 → i64
  return { a: asI64(L), b: asI64(R), kind: "num" };
}
function asI64(e) {
  if (e.kind === "bool") return `${paren(e.src)} as u8 as i64`;
  if (e.kind === "float") return `${paren(e.src)} as i64`;
  if (e.kind === "mb") return `${paren(e.src)}.map_or(-1, |v| v as i64)`;
  if (/^[-\d]+$/.test(e.src)) return `${e.src}i64`;
  return `${paren(e.src)} as i64`;
}
function asU64(e) {
  if (e.kind === "num") return /^[-\d]+$/.test(e.src) ? `${e.src}u64` : `${paren(e.src)} as u64`;
  if (e.kind === "bool") return `${paren(e.src)} as u64`;
  return `${paren(e.src)} as u64`;
}
function asF64(e) {
  if (e.kind === "float") return e.src;
  if (/^[-\d]+$/.test(e.src)) return `${e.src}f64`;
  return `${paren(e.src)} as f64`;
}
function asOptStr(e) {
  if (e.kind === "str") {
    if (e.src.startsWith("\"")) return `Some(${e.src})`;
    return e.src;
  }
  if (e.kind === "bool") errors.push("bool→str?");
  return e.src;
}
function cmpEq(a, b) {
  if (a.kind === "str" || b.kind === "str") return `${asOptStr(a)} == ${asOptStr(b)}`;
  if (a.kind === "float" || b.kind === "float") return `${asF64(a)} == ${asF64(b)}`;
  if (a.kind === "bool" && b.kind === "bool") return `${a.src} == ${b.src}`;
  return `${asI64(a)} == ${asI64(b)}`;
}
function bitShift(r, op) {
  if (op === "<<" || op === ">>") {
    if (/^[-\d]+$/.test(r.src)) return r.src;
    return `${paren(r.src)} as u32`;
  }
  return asI64(r);
}
function coerce(e, want) {
  if (e.kind === want) return e;
  if (want === "bool") {
    if (e.kind === "mb") return { src: `${paren(e.src)}.unwrap_or(false)`, kind: "bool" };
    if (e.kind === "num") return { src: `${paren(e.src)} != 0`, kind: "bool" };
    if (e.kind === "float") return { src: `${paren(e.src)} != 0.0`, kind: "bool" };
    if (e.kind === "str") return { src: `${paren(e.src)}.is_some()`, kind: "bool" };
  }
  return e;
}

// ---------- 翻译 implication 表 ----------
// 表达式翻译失败不中断: 记录硬错误 (整体仍会非零退出), 便于一次看全所有待核对项
function translateExpr(cppText, want = "bool") {
  try {
    return coerce(toRust(parseC(tokenizeC(cppText))), want);
  } catch (e) {
    errors.push(`表达式翻译失败: ${e.message} :: ${String(cppText).slice(0, 160)}`);
    return { src: "/*译错*/ false", kind: "bool", failed: true };
  }
}
function idOf(n) {
  const p = byName.get(n);
  if (!p) { errors.push(`引用了不在规范配置内的 flag: ${n}`); return 0; }
  return p.id;
}
// 条件链 → Rust bool 闭包体 (多段用 && 连接, 单段原样)
function rustCond(parts) {
  if (!parts.length) return { src: "true", kind: "bool" };
  const expr = parts.length === 1 ? parts[0] : parts.map((p) => `(${p})`).join(" && ");
  return translateExpr(expr);
}
const genImp = [];
for (const it of implications) {
  if (it.kind === "value") {
    const pe = rustCond(it.conds);
    if (pe.failed) continue;
    // 目标 value
    const target = it.target;
    const tag = CTYPE2TAG[target.ctype];
    let vsrc;
    const vtok = it.value.trim();
    const dm = /^FLAGDEFAULT_([a-zA-Z0-9_]+)$/.exec(vtok);
    if (dm) vsrc = `ImpValue::DefaultOf(${idOf(dm[1])})`;
    else if (vtok === "true") vsrc = "ImpValue::B(true)";
    else if (vtok === "false") vsrc = "ImpValue::B(false)";
    else if (tag === "MaybeBool" && (vtok === "std::nullopt" || vtok === "std :: nullopt")) vsrc = "ImpValue::MB(None)";
    else {
      // 数值/字符串: 走默认值同款常量折叠
      const fake = { name: `imp:${target.name}`, ctype: target.ctype, defaultText: vtok };
      const r = defaultExpr(fake);
      vsrc = r ? `ImpValue::V(${r})` : null;
      if (!vsrc) continue;
    }
    if (pe.kind !== "bool") { errors.push(`implication 前提非 bool: ${it.conds.join(" && ")}`); continue; }
    genImp.push(`Implication::value(${rustLit(it.premiseName)}, |f: &Flags| ${pe.src}, ${target.id}, ${vsrc}, ImpStrength::${it.strength})`);
  } else if (it.kind === "warn") {
    // 原版: if (A && B) PrintF(stderr, "Warning: disabling flag --B due to conflicting flags\n")
    const pe = rustCond(it.conds);
    if (pe.failed) continue;
    genImp.push(`Implication::warn(${rustLit(cUnescape(it.msgText))}, |f: &Flags| ${pe.src})`);
  } else if (it.kind === "generic") {
    // 原版: if (cond) TracingFlags::xxx.store(...);
    const pe = rustCond(it.conds);
    if (pe.failed) continue;
    genImp.push(`Implication::generic("${it.tracing}", |f: &Flags| ${pe.src})`);
  } else if (it.kind === "require") {
    // 原版: if (guards) { if (!(statement)) FlagError{} << #statement; }
    // Rust 闭包返回 "是否违规" (guard 成立且 statement 不成立)
    const stmt = translateExpr(it.stmt);
    if (stmt.kind !== "bool") { errors.push(`requirement 非 bool: ${it.stmt}`); continue; }
    const neg = `!${paren(stmt.src)}`;
    const body = it.conds.length ? rustCond(it.conds).src + " && " + neg : neg;
    genImp.push(`Implication::require(${rustLit(it.stmt)}, |f: &Flags| ${body})`);
  }
}
// warn 的 msg 文本: "Warning: disabling flag --X due to conflicting flags\n" — 与原版一致, 需提取 flag 名并做名字归一; 已保留原文

// ---------- feature-flags.h ----------
const stages = ["EXPERIMENTAL", "PRE_STAGED", "STAGED", "SHIPPED"];
// feature-flags.h 的条件特性宏 (IF_*_ENABLED) 由下面的 IF 表按规范配置展开 (与 CONFIG 一致性有断言)
const featuresFixed = [];
{
  const text = readV8("src/flags/feature-flags.h");
  // 条件特性宏的开关必须与规范配置一致 (防配置漂移)
  const IF = {
    IF_INTL_ENABLED: !!CONFIG.V8_INTL_SUPPORT,
    IF_REVEC_ENABLED: !!CONFIG.V8_ENABLE_WASM_SIMD256_REVEC,
    IF_SPARKPLUG_PLUS_ENABLED: !!CONFIG.V8_ENABLE_SPARKPLUG_PLUS,
  };
  for (const stage of stages) {
    const start = text.indexOf(`#define FOREACH_${stage}_FEATURE_FLAG`);
    let seg = "";
    const linesArr = text.slice(start).split("\n");
    for (const ln of linesArr) {
      seg += ln.replace(/\\\s*$/, "") + " ";
      if (!ln.endsWith("\\")) break;
    }
    const head = seg.slice(0, seg.indexOf(") ") + 2);
    const body = seg.slice(head.length);
    const re = /(JS_FEATURE|WASM_FEATURE|INTERNAL_FEATURE|IF_INTL_ENABLED|IF_REVEC_ENABLED|IF_SPARKPLUG_PLUS_ENABLED)\s*\(/g;
    let m;
    while ((m = re.exec(body))) {
      const fname = m[1];
      const { args, end } = splitArgs(body, m.index + fname.length);
      re.lastIndex = m.index + fname.length + (end - (m.index + fname.length));
      let kind, thenArgs = args;
      if (fname.startsWith("IF_")) {
        if (!IF[fname]) continue;
        thenArgs = args.slice(1);
        const t = args[0].trim();
        kind = t === "JS_FEATURE" ? "js" : t === "WASM_FEATURE" ? "wasm" : "internal";
      } else {
        kind = fname === "JS_FEATURE" ? "js" : fname === "WASM_FEATURE" ? "wasm" : "internal";
      }
      const name = thenArgs[0].trim();
      const desc = parseStringSeq(thenArgs[1], 0).str;
      const flagName = (kind === "wasm" ? "wasm_" : "") + name;
      if (!byName.has(flagName)) errors.push(`feature flag 不在规范配置表内: ${flagName} (${stage})`);
      featuresFixed.push({ stage, kind, name: flagName, desc });
    }
  }
}

// ---------- flags.cc 矛盾区 ----------
const ccText = readV8("src/flags/flags.cc");
const regionStart = ccText.indexOf("void FlagList::ResolveContradictionsWhenFuzzing()");
const regionEnd = ccText.indexOf("#undef CONTRADICTION");
if (regionStart < 0 || regionEnd < 0) {
  console.error("找不到 flags.cc 矛盾区边界 (ResolveContradictionsWhenFuzzing / #undef CONTRADICTION)");
  process.exit(1);
}
const region = ccText.slice(regionStart, regionEnd).replace(/\\\s*\n\s*/g, " ");
const contradictions = [];
const resetFuzzing = [];
const resetCorrectness = [];
let contradictionsWasmScoped = 0;
{
  // 单趟顺序扫描: 预处理指令只跟踪 #if V8_ENABLE_WEBASSEMBLY / #endif (其余一律硬报错),
  // 调用实参用括号平衡消费 —— 原版 CONTRADICTION 允许跨行, 顺序即优先级, 不得重排
  const scan = /(^[\t ]*#[\t ]*(?:if|ifdef|ifndef|else|elif|endif)[^\n]*)|(CONTRADICTION|RESET_WHEN_CORRECTNESS_FUZZING|RESET_WHEN_FUZZING)[\t ]*\(/gm;
  let inWasm = false, m;
  // 原版 RESET_WHEN_FUZZING(flag) = CONTRADICTION(flag, fuzzing), RESET_WHEN_CORRECTNESS_FUZZING(flag)
  // = CONTRADICTION(flag, correctness_fuzzer_suppressions): 三者进同一张有序表
  const push = (kind, args) => {
    const names = kind === "CONTRADICTION"
      ? args
      : [args[0], kind === "RESET_WHEN_FUZZING" ? "fuzzing" : "correctness_fuzzer_suppressions"];
    if (kind === "RESET_WHEN_FUZZING") resetFuzzing.push(args[0]);
    else if (kind === "RESET_WHEN_CORRECTNESS_FUZZING") resetCorrectness.push(args[0]);
    const ia = byName.get(names[0]), ib = byName.get(names[1]);
    if (!ia || !ib) { errors.push(`contradiction flag 缺失: ${names[0]} / ${names[1]}`); return; }
    contradictions.push({ a: ia.id, b: ib.id, wasm: inWasm });
  };
  while ((m = scan.exec(region))) {
    if (m[1] !== undefined) {
      // 原版 #endif 带行尾注释 (如 `#endif  // V8_ENABLE_WEBASSEMBLY`), 先剥掉再判
      const d = m[1].trim().replace(/\/\/.*$/, "").trim();
      if (/^#[\t ]*(if|ifdef)[\t ]+V8_ENABLE_WEBASSEMBLY$/.test(d)) inWasm = true;
      else if (/^#[\t ]*endif$/.test(d)) {
        if (!inWasm) errors.push(`矛盾区 #endif 无配对 #if: ${d}`);
        inWasm = false;
      } else errors.push(`矛盾区出现未跟踪的预处理指令: ${d}`);
      continue;
    }
    const parenAt = m.index + m[0].length - 1;
    const { args, end } = splitArgs(region, parenAt);
    if (args.length !== (m[2] === "CONTRADICTION" ? 2 : 1)) {
      errors.push(`${m[2]} 实参数异常: ${args.length}`);
      continue;
    }
    push(m[2], args.map((a) => a.trim()));
    scan.lastIndex = end;
  }
  if (inWasm) errors.push("矛盾区 #if/#endif 不配对");
  contradictionsWasmScoped = contradictions.filter((c) => c.wasm).length;
}

// ---------- 输出 defs.rs ----------
const outDefs = [
  banner(GEN, "src/flags/flag-definitions.h", "规范配置: macOS arm64 release (见脚本 CONFIG); 生成物, 修订就地改"),
  `use crate::value::{Cow, FlagType, FlagValue, Flags};`,
  ``,
  `/// 每 flag 的静态描述 (对应原版 Flag + kFlagsMetadata)`,
  `pub struct FlagDef {`,
  `  pub name: &'static str,`,
  `  pub ty: FlagType,`,
  `  pub readonly: bool,`,
  `  pub comment: &'static str,`,
  `  pub default: fn() -> FlagValue,`,
  `}`,
  `// 别名不单列表: ALL_ENTRIES 的 flag_index/canonical_index 已完整表达别名→主 flag 映射`,
  `#[derive(Copy, Clone)]`,
  `pub struct EntryMeta {`,
  `  pub name: &'static str,`,
  `  pub comment: Option<&'static str>,`,
  `  pub flag_index: u32,`,
  `  pub canonical_index: u32,`,
  `}`,
  ``,
  `pub static FLAGS: &[FlagDef] = &[`,
  ...primaries.map((p) => {
    const de = defaultExpr(p);
    return `  FlagDef { name: ${rustLit(p.name)}, ty: FlagType::${CTYPE2TAG[p.ctype]}, readonly: ${p.readonly}, comment: ${rustLit(cUnescape(p.comment))}, default: || ${de ?? "FlagValue::Bool(false)"} },`;
  }),
  `];`,
  ``,
  // ALL_ENTRIES: APPLY 序 (含别名/readonly), 与原版 kFlagsMetadata 一致
  `pub static ALL_ENTRIES: &[EntryMeta] = &[`,
];
{
  const canonByPrimary = new Map();
  let fi = 0;
  // 第一遍: primary (与原版相同顺序 = APPLY 遍历序, flag_index 按 META/FLAGS 序)
  const flagIdx = new Map(primaries.map((p, i) => [p.name, i]));
  let ci = 0;
  applyEntries.forEach((a, i) => {
    if (a.primary === "") canonByPrimary.set(a.name, ci);
    ci++;
  });
  ci = 0;
  const rows = applyEntries.map((a) => {
    const canonical = a.primary === "" ? ci : canonByPrimary.get(a.primary);
    const idx = a.primary === "" ? flagIdx.get(a.name) : flagIdx.get(a.primary);
    ci++;
    if (idx === undefined) { errors.push(`ALL_ENTRIES flag_index 缺失: ${a.name}`); return null; }
    const cmt = a.comment === undefined ? "None" : a.comment === "" ? "None" : `Some(${rustLit(cUnescape(a.comment))})`;
    return `  EntryMeta { name: ${rustLit(a.name)}, comment: ${cmt}, flag_index: ${idx}, canonical_index: ${canonical ?? 0} },`;
  }).filter(Boolean);
  outDefs.push(...rows);
}
outDefs.push(
  `];`,
  ``,
  // 排序表 (归一化名升序) — 复刻原版 kSortedFlagIndices (std::sort 语义: 名称唯一, 稳定即可)
  `/// 按归一化名升序的 ALL_ENTRIES 下标 (原版 kSortedFlagIndices; 名称唯一, 稳定排序等价)`,
  `pub(crate) static SORTED_ALL: &[u32] = &[`,
);
{
  const names = applyEntries.map((a) => a.name);
  const idxs = names.map((_, i) => i);
  idxs.sort((a, b) => cmpNames(names[a], names[b]));
  outDefs.push(`  ${idxs.map((v) => v + "u32").join(", ")},`);
}
outDefs.push(
  `];`,
  ``,
  `/// 主 flag 下标 → ALL_ENTRIES 下标 (原版 kPrimaryToAllIndices, 即该主名的首个条目)`,
  `pub(crate) static PRIMARY_TO_ALL: &[u32] = &[`,
);
{
  // 原版: 遍历 metadata, i == canonical_index 时 primary_indices[flag_index] = i
  const flagIdx = new Map(primaries.map((p, i) => [p.name, i]));
  const p2a = new Array(primaries.length).fill(-1);
  applyEntries.forEach((a, i) => {
    if (a.primary === "") p2a[flagIdx.get(a.name)] = i;
  });
  if (p2a.some((v) => v < 0)) errors.push("PRIMARY_TO_ALL 存在未映射主 flag");
  outDefs.push(`  ${p2a.map((v) => v + "u32").join(", ")},`);
}
outDefs.push(
  `];`,
  ``,
  `/// 归一化名 (含别名) → 主 flag 下标 (原版 FindFlagByName 精确查表; phf 编译期完美散列)`,
  `pub(crate) static NAME_MAP: phf::Map<&'static str, u32> = phf::phf_map! {`,
);
{
  const flagIdx = new Map(primaries.map((p, i) => [p.name, i]));
  for (const a of applyEntries) {
    const id = flagIdx.get(a.primary === "" ? a.name : a.primary);
    outDefs.push(`  ${rustLit(norm(a.name))} => ${id}u32,`);
  }
}
outDefs.push(
  `};`,
  ``,
  `// 逐 flag 类型化存取器 (对应原版 v8_flags.xxx 字段读写)`,
  `impl Flags {`,
);
for (const p of primaries) {
  const tag = CTYPE2TAG[p.ctype];
  const m = mname(p.name);
  const doc = p.comment ? `  /// ${p.comment.replaceAll("\n", " ")}\n` : "";
  const rt = TYPE_OF[p.ctype][1];
  if (tag === "Str") {
    outDefs.push(
      `${doc}  #[inline]\n  pub fn ${m}(&self) -> Option<&str> { self.get_str(${p.id}) }`,
      `  #[inline]\n  pub fn set_${m}(&mut self, v: &str) -> crate::Result<()> { self.assign_str(${p.id}, Some(Cow::Owned(v.to_string()))) }\n`,
    );
  } else {
    const acc = DEF_ACC[tag]; // 与 Flags::get_xxx / assign_xxx 命名一一对应
    outDefs.push(
      `${doc}  #[inline]\n  pub fn ${m}(&self) -> ${rt} { self.get_${acc}(${p.id}) }`,
      `  #[inline]\n  pub fn set_${m}(&mut self, v: ${rt}) -> crate::Result<()> { self.assign_${acc}(${p.id}, v) }\n`,
    );
  }
}
outDefs.push(`}`, ``, `// ── 常量 (原版 flags.cc kNumFlags 等; 分类计数见 counts.rs) ──`);
outDefs.push(`pub const NUM_FLAGS: usize = ${primaries.length}; // 主 flag 数 (原版 kNumFlags)`);
outDefs.push(`pub const NUM_ALL_FLAGS: usize = ${applyEntries.length}; // 主 flag + 别名 (原版 kNumAllFlags)`);
const typeCount = {};
for (const p of primaries) {
  const k = TAG2SNAKE[CTYPE2TAG[p.ctype]];
  typeCount[k] = (typeCount[k] || 0) + 1;
}

// ---- 名字比较 (复刻原版 FlagNamesCmp: '_' 归一为 '-' 后逐字节, 终止符最小) ----
function normByte(ch) {
  if (ch === undefined || ch === "") return -1; // C 字符串 '\0' 小于任何字符
  const c = ch.charCodeAt(0);
  return c === 95 /* '_' */ ? 45 /* '-' */ : c;
}
function cmpNames(a, b) {
  const n = Math.max(a.length, b.length);
  for (let i = 0; i < n; i++) {
    const ac = normByte(a[i]);
    const bc = normByte(b[i]);
    if (ac < bc) return -1;
    if (ac > bc) return 1;
  }
  return 0;
}

// ---------- implications_gen.rs ----------
const outImp = [
  banner(GEN, "src/flags/flag-definitions.h (FLAG_MODE_DEFINE_IMPLICATIONS)", "机械展开; 定义序即原版求值序"),
  `use crate::base::{is_pow2, is_valid_smi};`,
  `use crate::flags::{Flags, ImpStrength, ImpValue, Implication};`,
  `use crate::value::{Cow, FlagValue};`,
  ``,
  `/// 顺序敏感: 与原版 flag-definitions.h 内 DEFINE_*_IMPLICATION 出现顺序一致`,
  `pub(crate) static IMPLICATIONS: &[Implication] = &[`,
  ...genImp.map((s) => `  ${s},`),
  `];`,
  ``,
  `pub const NUM_IMPLICATIONS: usize = ${genImp.filter((s) => s.startsWith("Implication::value")).length};`,
  `pub const NUM_WARNINGS: usize = ${genImp.filter((s) => s.startsWith("Implication::warn")).length};`,
  `pub const NUM_GENERICS: usize = ${genImp.filter((s) => s.startsWith("Implication::generic")).length};`,
  `pub const NUM_REQUIRES: usize = ${genImp.filter((s) => s.startsWith("Implication::require")).length};`,
];

// ---------- features_gen.rs ----------
const outFeat = [
  banner(GEN, "src/flags/feature-flags.h", "规范配置: INTL 开 / REVEC 关 / SPARKPLUG_PLUS 关"),
  `/// 特性 flag 表 (阶段, 种类, flag 名, 描述)`,
  `pub struct Feature { pub stage: &'static str, pub kind: &'static str, pub name: &'static str, pub desc: &'static str }`,
  `pub(crate) static FEATURES: &[Feature] = &[`,
  ...featuresFixed.map((f) => `  Feature { stage: ${rustLit(f.stage)}, kind: ${rustLit(f.kind)}, name: ${rustLit(f.name)}, desc: ${rustLit(f.desc)} },`),
  `];`,
  `pub const NUM_FEATURES: usize = ${featuresFixed.length};`,
];

// ---------- contradictions_gen.rs ----------
// 原版把 RESET_WHEN_FUZZING(flag) 就地展开成 CONTRADICTION(flag, fuzzing),
// 顺序即优先级, 因此只生成一张有序矛盾表; RESET_* 计数仅进 stats.json 对数。
const outCont = [
  banner(GEN, "src/flags/flags.cc ResolveContradictionsWhenFuzzing", "fuzzing 矛盾表 (顺序即优先级)"),
  `use crate::lookup::FlagId;`,
  ``,
  `/// 顺序敏感: 与原版 std::vector<std::pair<const char*, const char*>> 顺序一致`,
  `pub(crate) static CONTRADICTIONS: &[(FlagId, FlagId)] = &[`,
  ...contradictions.map((c) => `  (${c.a}u32, ${c.b}u32),`),
  `];`,
];

// ---------- counts.rs (对数核对基线) ----------
const rawCounts = (() => {
  const text = readV8("src/flags/flag-definitions.h");
  const count = (re) => (text.match(re) || []).length;
  return {
    define_bool: count(/\bDEFINE_BOOL\(/g),
    define_int: count(/\bDEFINE_INT\(/g),
    define_uint: count(/\bDEFINE_UINT\(/g),
    define_uint64: count(/\bDEFINE_UINT64\(/g),
    define_float: count(/\bDEFINE_FLOAT\(/g),
    define_size_t: count(/\bDEFINE_SIZE_T\(/g),
    define_string: count(/\bDEFINE_STRING\(/g),
    define_maybe_bool: count(/\bDEFINE_MAYBE_BOOL\(/g),
    define_debug_bool: count(/\bDEFINE_DEBUG_BOOL\(/g),
    define_experimental_feature: count(/\bDEFINE_EXPERIMENTAL_FEATURE\(/g),
    define_unsafe_flag: count(/\bDEFINE_UNSAFE_FLAG\(/g),
    define_developer_flag: count(/\bDEFINE_DEVELOPER_FLAG\(/g),
    define_debug_developer_flag: count(/\bDEFINE_DEBUG_DEVELOPER_FLAG\(/g),
    define_smi: count(/\bDEFINE_SMI\(/g),
    define_positive_smi: count(/\bDEFINE_POSITIVE_SMI\(/g),
    define_alias: count(/\bDEFINE_ALIAS(_[A-Z_]+)*\(/g),
    implication_total: count(/\bDEFINE_(NEG_|WEAK_|DISABLE_FLAG_|MIN_VALUE_|MAX_VALUE_|NEG_VALUE_VALUE_)?[A-Z_]*IMPLICATION\(/g),
    requirement: count(/\bDEFINE_REQUIREMENT\(/g),
    generic: count(/\bDEFINE_GENERIC_IMPLICATION\(/g),
  };
})();
const stats = {
  config: { os: "darwin/macOS", arch: "arm64", build: "release", on: Object.entries(CONFIG).filter(([, v]) => v).map(([k]) => k), extra: CONFIG_EXTRA },
  canonical: {
    primary: primaries.length,
    aliases: aliases.length,
    all_entries: applyEntries.length,
    readonly: primaries.filter((p) => p.readonly).length,
    by_type: typeCount,
    implications: genImp.length,
    by_kind: {
      value: genImp.filter((s) => s.startsWith("Implication::value")).length,
      warn: genImp.filter((s) => s.startsWith("Implication::warn")).length,
      generic: genImp.filter((s) => s.startsWith("Implication::generic")).length,
      require: genImp.filter((s) => s.startsWith("Implication::require")).length,
    },
    features: featuresFixed.length,
    contradictions: contradictions.length,
    contradictions_wasm_scoped: contradictionsWasmScoped,
    reset_fuzzing: resetFuzzing.length,
    reset_correctness: resetCorrectness.length,
  },
  raw_file: rawCounts,
};

// ---------- 硬失败门槛 ----------
if (errors.length) {
  console.error("生成失败, 共 " + errors.length + " 个硬错误:");
  for (const e of new Set(errors)) console.error("  - " + e.slice(0, 200));
  process.exit(1);
}

// ---------- 落盘 ----------
writeSrc(CRATE, "defs.rs", outDefs.join("\n") + "\n");
writeSrc(CRATE, "implications_gen.rs", outImp.join("\n") + "\n");
writeSrc(CRATE, "features_gen.rs", outFeat.join("\n") + "\n");
writeSrc(CRATE, "contradictions_gen.rs", outCont.join("\n") + "\n");
writeFileSync(join(import.meta.dir, "stats.json"), JSON.stringify(stats, null, 2) + "\n");
writeSrc(CRATE, "counts.rs", banner(GEN, "src/flags/flag-definitions.h", "对数核对基线 (生成)") +
  `/// 规范配置展开计数 + 原版文件原始宏计数, 与 tool/src/flags/stats.json 逐项同步。\n` +
  `/// tests/main.rs 的 counts 用例会同时核对: 生成表实数 == 本文件常量 == stats.json。\n` +
  Object.entries({
    ...Object.fromEntries(Object.entries(stats.canonical).filter(([, v]) => typeof v === "number").map(([k, v]) => ["CANON_" + k.toUpperCase(), v])),
    ...Object.fromEntries(Object.entries(stats.canonical.by_type).map(([k, v]) => ["CANON_TYPE_" + k, v])),
    ...Object.fromEntries(Object.entries(stats.canonical.by_kind).map(([k, v]) => ["CANON_KIND_" + k.toUpperCase(), v])),
    ...Object.fromEntries(Object.entries(stats.raw_file).map(([k, v]) => ["RAW_" + k.toUpperCase(), v])),
  }).map(([k, v]) => `pub const ${k}: usize = ${v};`).join("\n") + "\n");

console.log(JSON.stringify(stats.canonical));
