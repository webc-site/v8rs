#!/usr/bin/env bun
// v8/src/wasm/wasm-constants.h → v8-wasm-core/src/wasm_constants.rs
// 解析全部常量与枚举: 模块头魔数/值类型码/类型定义码/导入导出种类/limits 标志/
// 段码 (含自增段与 helper 别名)/自定义段名字符串/name-section 种类/catch/switch/
// 内存序/页大小/位置与索引哨兵/包装预算等。
// 对数: 运行时打印各枚举变体数与关键值, 供人工与原版逐项核对。
import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/wasm-constants.h");

// 去注释、压空白 (保留字符串字面量内的内容: 先抽出字符串再还原)
const strings = [];
const flat = src
  .split("\n")
  .map((l) => l.replace(/\/\/.*$/, ""))
  .join(" ")
  .replace(/"(?:[^"\\]|\\.)*"/g, (s) => {
    strings.push(s.slice(1, -1));
    return `"@S${strings.length - 1}@"`;
  })
  .replace(/\s+/g, " ");

// ---- 解析枚举 -------------------------------------------------------------
// 返回 [{name, valueExpr}] (valueExpr 为 null 表示自增)
function parseEnum(enumName) {
  const m = flat.match(new RegExp(`enum ${enumName}\\s*(?::\\s*\\w+)?\\s*\\{([^}]*)\\}`));
  if (!m) throw new Error(`enum ${enumName} 未找到`);
  const items = [];
  for (const part of m[1].split(",")) {
    const t = part.trim();
    if (!t || t.startsWith("//")) continue;
    const im = t.match(/^(\w+)\s*(?:=\s*([^=]+?))?$/);
    if (!im) continue;
    items.push({ name: im[1], expr: im[2] ? im[2].trim() : null });
  }
  return items;
}

// 枚举值求值 (支持数字/十六进制/引用前序变体/<<|or/字符串占位)
function evalExpr(expr, env) {
  if (expr === null) return null;
  let e = expr.replace(/0[xX][0-9a-fA-F]+/g, (h) => String(parseInt(h, 16)));
  for (let pass = 0; pass < 3; pass++) {
    for (const [k, v] of env) if (typeof v === "number") e = e.replace(new RegExp(`\\b${k}\\b`, "g"), `(${v})`);
  }
  if (/^[\d()+\-*/|&<> x]+$/.test(e)) {
    return Function(`"use strict"; return (${e});`)();
  }
  throw new Error(`枚举表达式无法求值: ${expr} → ${e}`);
}

// 自增展开: null 值 = 前值 + 1
function expand(items, env) {
  let prev = -1;
  const out = [];
  for (const it of items) {
    const v = it.expr === null ? prev + 1 : evalExpr(it.expr, env);
    env.set(it.name, v);
    out.push({ name: it.name, v });
    prev = v;
  }
  return out;
}

// ---- Rust 命名转换 --------------------------------------------------------
// 枚举变体: 去 k 前缀与 Code/SectionCode 后缀, 保留 PascalCase (kI32Code → I32,
// kUnknownSectionCode → UnknownSection, kStringRefSectionCode → StringRefSection)
const variant = (n) =>
  n.replace(/^k(?=[A-Z0-9])/, "").replace(/Code$/, "").replace(/^(\w+)Section$/, "$1Section");
// 常量: kSourceMappingURLString → K_SOURCE_MAPPING_URL_STRING (连续大写 acronym 分隔)
const CONST = (n) =>
  "K_" + n.replace(/^k(?=[A-Z0-9])/, "")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/__/g, "_")
    .toUpperCase();

// ---- 各枚举 ---------------------------------------------------------------
const env = new Map([["kExternalExactBit", 1 << 5]]);

const valueTypeCode = expand(parseEnum("ValueTypeCode"), env);
// 对数: kFirstHeapTypeCode/kLastHeapTypeCode 圈定堆类型码区间 [0x5b..0x75];
// 区间外的码均为值类型码 (kVoidCode=0x40 与 packed 0x76-0x7f)
const firstHeap = valueTypeCode.find((x) => x.name === "kFirstHeapTypeCode");
const lastHeap = valueTypeCode.find((x) => x.name === "kLastHeapTypeCode");
const explicitVT = valueTypeCode.filter((x) => x.name !== "kFirstHeapTypeCode" && x.name !== "kLastHeapTypeCode");
if (firstHeap.v !== 0x5b || lastHeap.v !== 0x75) {
  throw new Error(`heap type 码界异常: first=0x${firstHeap.v.toString(16)} last=0x${lastHeap.v.toString(16)}`);
}
for (const x of explicitVT) {
  const inHeapRange = x.v >= firstHeap.v && x.v <= lastHeap.v;
  const isValueType = x.name === "kVoidCode" || x.v >= 0x76;
  if (inHeapRange === isValueType && x.name !== "kRefCode" && x.name !== "kRefNullCode" && x.name !== "kExactCode") {
    throw new Error(`码区间归属异常: ${x.name}=0x${x.v.toString(16)}`);
  }
}

const importExport = expand(parseEnum("ImportExportKindCode"), env);
const compactImport = expand(parseEnum("CompactImportCode"), env);
const limitsFlags = expand(parseEnum("LimitsFlags"), env);
const segmentFlags = expand(parseEnum("SegmentFlags"), env);
const sectionCode = expand(parseEnum("SectionCode"), env);
const nameSectionKind = expand(parseEnum("NameSectionKindCode"), env);
const catchKind = expand(parseEnum("CatchKind"), env);
const switchKind = expand(parseEnum("SwitchKind"), env);
const memoryOrdering = expand(parseEnum("MemoryOrdering"), env);
const cWasmEntry = expand(parseEnum("CWasmEntryParameters"), env);

// ---- 散常量 ---------------------------------------------------------------
function scalarConst(type, name) {
  const m = flat.match(new RegExp(`constexpr ${type} ${name} = ([^;]+);`));
  if (!m) throw new Error(`常量 ${name} 未找到`);
  return m[1].trim();
}
const magic = scalarConst("uint32_t", "kWasmMagic");
const version = scalarConst("uint32_t", "kWasmVersion");
const scalars = [
  ["u8", "kSharedFlagCode"], ["u8", "kWasmFunctionTypeCode"], ["u8", "kWasmStructTypeCode"],
  ["u8", "kWasmArrayTypeCode"], ["u8", "kWasmContTypeCode"], ["u8", "kWasmSubtypeCode"],
  ["u8", "kWasmSubtypeFinalCode"], ["u8", "kWasmRecursiveTypeGroupCode"], ["u8", "kWasmDescriptorCode"],
  ["u8", "kWasmDescribesCode"],
].map(([t, n]) => ({ type: t, name: n, v: parseInt(scalarConst("uint8_t", n)) }));

// 字符串常量 (名字段)
const strConsts = [...flat.matchAll(/constexpr char (\w+)\[\] = "(@S\d+@)"/g)].map((m) => ({
  name: m[1], v: strings[Number(m[2].slice(2, -1))].replace(/\\"/g, '"'),
}));

// pageSize / log2
const pageSize = parseInt(scalarConst("size_t", "kWasmPageSize"));
const pageSizeLog2 = parseInt(scalarConst("uint32_t", "kWasmPageSizeLog2"));

// int 常量
const intConsts = [
  ["kExceptionAttribute", "u32"], ["kAnonymousFuncIndex", "i32"], ["kInvalidCanonicalIndex", "u32"],
  ["kGenericWrapperBudget", "u32"], ["kMinimumSupertypeArraySize", "u32"], ["kMaxPolymorphism", "i32"],
  ["kMaxStructFieldIndexForImplicitNullCheck", "i32"], ["kOptimizationPriorityExecutedOnceSentinel", "i32"],
  ["kOptimizationPriorityNotSpecifiedSentinel", "i32"], ["kInt31MaxValue", "i32"],
].map(([n, t]) => {
  const raw = flat.match(new RegExp(`constexpr (?:uint32_t|int32_t|int) ${n} = ([^;]+);`))[1]
    .replace(/static_cast<uint32_t>\(-1\)/, "4294967295").replace(/-kInt31MaxValue - 1/, "-1073741824");
  return { name: n, type: t, v: Function(`return (${raw});`)() };
});
// kInt31MinValue = -kInt31MaxValue - 1
intConsts.push({ name: "kInt31MinValue", type: "i32", v: -intConsts.find((x) => x.name === "kInt31MaxValue").v - 1 });

// ---- 对数输出 -------------------------------------------------------------
const counts = { valueTypeCode: explicitVT.length, importExport: importExport.length, compactImport: compactImport.length,
  limitsFlags: limitsFlags.length, segmentFlags: segmentFlags.length, sectionCode: sectionCode.length,
  nameSectionKind: nameSectionKind.length, catchKind: catchKind.length, switchKind: switchKind.length,
  memoryOrdering: memoryOrdering.length, cWasmEntry: cWasmEntry.length };
console.log("wasm-constants.h 对数:");
console.log(`  kWasmMagic=${magic} kWasmVersion=${version} kWasmPageSize=${pageSize}(0x${pageSize.toString(16)}) log2=${pageSizeLog2}`);
console.log(`  ValueTypeCode ${explicitVT.length} 显式变体, heap 码界 [${firstHeap.v}..${lastHeap.v}] = [0x${firstHeap.v.toString(16)}..0x${lastHeap.v.toString(16)}]`);
for (const [k, n] of Object.entries(counts)) console.log(`  ${k}: ${n} 变体`);
console.log(`  字符串常量 ${strConsts.length} 项: ${strConsts.map((s) => s.v).join(", ")}`);
console.log(`  SectionCode: 末位 ${sectionCode.at(-1).name}=${sectionCode.at(-1).v}; helper ` +
  sectionCode.filter((x) => ["kFirstSectionInModule", "kLastKnownModuleSection", "kFirstUnorderedSection"].includes(x.name)).map((x) => `${x.name}=${x.v}`).join(" "));
console.log(`  int 常量: ${intConsts.map((x) => `${x.name}=${x.v}`).join(", ")}`);

// ---- 生成 -----------------------------------------------------------------
const hx = (v) => `0x${v.toString(16)}`;
const num = (v) => (Number.isInteger(v) && v <= 0xff ? `${v} /* ${hx(v)} */` : `${v}`);

function enumBlock(rustName, repr, items, { skip = [], doc = "", fromInt = null, hex = false } = {}) {
  let s = `${doc}#[repr(${repr})]\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum ${rustName} {\n`;
  for (const it of items) {
    if (skip.includes(it.name)) continue;
    s += `  ${variant(it.name)} = ${hex ? hx(it.v) : it.v},\n`;
  }
  s += "}\n";
  if (fromInt !== null) {
    s += `\nimpl ${rustName} {\n  /// wire 字节 → 枚举 (未分配码 → None)\n  pub const fn from_${fromInt}(v: ${fromInt}) -> Option<Self> {\n    match v {\n`;
    for (const it of items) {
      if (skip.includes(it.name)) continue;
      s += `      ${hex ? hx(it.v) : it.v} => Some(Self::${variant(it.name)}),\n`;
    }
    s += `      _ => None,\n    }\n  }\n}\n`;
  }
  s += "\n";
  return s;
}

let out = `//! v8/src/wasm/wasm-constants.h 的 1:1 转写
//! 登记差异:
//! - kOSRTargetOffset 仅 V8_TARGET_ARCH_X64 → 推迟 (arch 梯队)。
//! - kInvalidCanonicalIndex 的 Smi 往返 static_assert 依赖 Internals → 跳过
//!   (Rust 侧无 Smi 表示), 值本身 1:1。
//! - WasmCodePosition = i32 类型别名; enum 保持原判别值。
`;
out += banner("src/wasm/wasm-constants.h.js", "src/wasm/wasm-constants.h", "常量与枚举表; 对数见脚本运行输出");
out += `// --- 模块头二进制编码 ---
pub const K_WASM_MAGIC: u32 = ${magic}; // "\\0asm"
pub const K_WASM_VERSION: u32 = ${version};

`;
out += enumBlock("ValueTypeCode", "u8", explicitVT, { doc: "// --- 值/堆类型二进制编码 (wire 字节) ---\n", fromInt: "u8", hex: true });
out += `/// 解码用堆类型码界 (wasm-constants.h:66-67): [kFirstHeapTypeCode, kLastHeapTypeCode]
pub const FIRST_HEAP_TYPE_CODE: u8 = ${hx(firstHeap.v)}; // kNoWaitqueueCode, 最低分配码
pub const LAST_HEAP_TYPE_CODE: u8 = ${hx(lastHeap.v)}; // kNoContCode, 最高分配码

// --- 类型定义二进制编码 ---
`;
for (const s of scalars) out += `pub const ${CONST(s.name)}: u8 = ${hx(s.v)};\n`;
out += `
// --- 导入/导出种类二进制编码 ---
pub const K_EXTERNAL_EXACT_BIT: u8 = ${hx(1 << 5)};
`;
out += enumBlock("ImportExportKindCode", "u8", importExport);
out += enumBlock("CompactImportCode", "u8", compactImport);
out += enumBlock("LimitsFlags", "u8", limitsFlags, { doc: "// --- limits 结构 (内存与表 limits 通用) ---\n" });
out += enumBlock("SegmentFlags", "u8", segmentFlags, { doc: "// --- 数据/元素段标志 ---\n" });
out += `// --- 段标识符编码 ---
// 0-14 为已知段; 15+ 为自定义段 (以字符串标识, 枚举值不保证稳定)。
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionCode {
`;
const sectionHelpers = ["kFirstSectionInModule", "kLastKnownModuleSection", "kFirstUnorderedSection"];
for (const it of sectionCode) {
  if (sectionHelpers.includes(it.name)) continue;
  out += `  ${variant(it.name)} = ${it.v},\n`;
}
const secVar = (n) => variant(sectionCode.find((x) => x.name === n).name);
out += `}

impl SectionCode {
  /// wire 字节 → 枚举 (未定义段码 → None)
  pub const fn from_i8(v: i8) -> Option<Self> {
    match v {
`;
for (const it of sectionCode) {
  if (sectionHelpers.includes(it.name)) continue;
  out += `      ${it.v} => Some(Self::${variant(it.name)}),\n`;
}
out += `      _ => None,
    }
  }
  /// kFirstSectionInModule = kTypeSectionCode
  pub const FIRST_SECTION_IN_MODULE: Self = Self::${secVar("kTypeSectionCode")};
  /// kLastKnownModuleSection = kStringRefSectionCode
  pub const LAST_KNOWN_MODULE_SECTION: Self = Self::${secVar("kStringRefSectionCode")};
  /// kFirstUnorderedSection = kDataCountSectionCode
  pub const FIRST_UNORDERED_SECTION: Self = Self::${secVar("kDataCountSectionCode")};
}

`;
out += "// --- 自定义段名 ---\n";
for (const s of strConsts) out += `pub const ${CONST(s.name)}: &str = ${JSON.stringify(s.v)};\n`;
out += "\n";
out += enumBlock("NameSectionKindCode", "u8", nameSectionKind, { doc: "// --- name section 子段种类编码 ---\n" });
out += enumBlock("CatchKind", "u8", catchKind.filter((x) => x.name !== "kLastCatchKind"), { doc: "// --- catch 子句种类 ---\n" });
out += `impl CatchKind {
  /// kLastCatchKind = kCatchAllRef
  pub const LAST: Self = Self::CatchAllRef;
}

`;
out += enumBlock("SwitchKind", "u8", switchKind.filter((x) => x.name !== "kLastSwitchKind"), { doc: "// --- switch 子句种类 ---\n" });
out += `impl SwitchKind {
  /// kLastSwitchKind = kSwitch
  pub const LAST: Self = Self::Switch;
}

`;
out += enumBlock("MemoryOrdering", "u8", memoryOrdering, { doc: "// --- 原子内存序 ---\n" });
out += `// --- 页大小 ---
pub const WASM_PAGE_SIZE: usize = ${pageSize}; // 0x10000
pub const WASM_PAGE_SIZE_LOG2: u32 = ${pageSizeLog2};
const _: () = assert!(WASM_PAGE_SIZE == 1usize << WASM_PAGE_SIZE_LOG2);

/// 代码位置 (wasm 二进制内字节偏移); TODO(wasm): 原版注释保留 — 待包装为结构
pub type WasmCodePosition = i32;
pub const K_NO_CODE_POSITION: WasmCodePosition = -1;

`;
for (const c of intConsts) out += `pub const ${CONST(c.name)}: ${c.type} = ${c.v};\n`;
out += `
// --- CWasmEntry 参数槽位 (kNumParameters 为计数标记) ---
`;
out += enumBlock("CWasmEntryParameters", "u8", cWasmEntry);

const p = writeSrc("v8-wasm-core", "wasm_constants.rs", out);
console.log(`→ ${p}`);
