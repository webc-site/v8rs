#!/usr/bin/env bun
// tool/src/compiler/turboshaft/operations.h.js
// 机械提取 v8/src/compiler/turboshaft/operations.{h,cc} (超 2k 行, 只提码表)
// → v8-turboshaft/src/operations.rs
//
// 提取面: TURBOSHAFT_*_OPERATION_LIST 宏全展开 → Opcode 枚举; block-terminator
// 前缀数 kNumberOfBlockTerminatorOpcodes; kNumberOfOpcodes; OpcodeName 名表
// (operations.cc); MayThrow 操作集; HashingStrategy; 各 *Op::Kind 子枚举 +
// operations.cc 对应 operator<< 名表; StorageSlotCount 公式。
//
// 目标配置 (与 opcodes.h.js 对齐):
//   V8_ENABLE_WEBASSEMBLY=1, V8_ENABLE_SIMD128=1, V8_ENABLE_SIMD256=0,
//   V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA=1
//   kOperationSizeTable 依赖 C++ sizeof(Op 结构布局), 不可机械展开 — skip 登记
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/operations.h");
const cc = readV8("src/compiler/turboshaft/operations.cc");

const CONFIG = {
  V8_ENABLE_WEBASSEMBLY: true,
  V8_ENABLE_SIMD128: true,
  V8_ENABLE_SIMD256: false,
  V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA: true,
  V8_INTL_SUPPORT: true,
  V8_ENABLE_SANDBOX: true,
  V8_ENABLE_SANDBOX_HARDWARE_SUPPORT: false,
};
const IF_FLAGS = {
  IF_WASM: CONFIG.V8_ENABLE_WEBASSEMBLY,
  IF_INTL: CONFIG.V8_INTL_SUPPORT,
  IF_SANDBOX: CONFIG.V8_ENABLE_SANDBOX,
  IF_HARDWARE_SANDBOX: CONFIG.V8_ENABLE_SANDBOX_HARDWARE_SUPPORT,
};

// ---------------------------------------------------------------------------
// 1. 宏展开: TURBOSHAFT_*_OPERATION_LIST
// ---------------------------------------------------------------------------
// 抓 #define NAME(V) 反斜杠续行体, V(X) 列表
// 注: 续行组写法 (?:[^\n]*\\\n)*[^\n]* — JSC 对 (?:\\\n[^\n]*)* 回溯异常
const CONT_LINE = "(?:[^\\n]*\\\\\\n)*[^\\n]*";
function macroList(name) {
  const re = new RegExp(`#define ${name}\\(V\\)${CONT_LINE}`, "g");
  const defs = [...h.matchAll(re)];
  if (defs.length === 0) throw new Error(`未找到宏 ${name}`);
  const out = new Map(); // name → true (保持展开序, 去重)
  for (const d of defs) {
    for (const m of d[0].matchAll(/V\((\w+)\)/g)) out.set(m[1], true);
    for (const m of d[0].matchAll(/IF_(\w+)\(V,\s*(\w+)\)/g)) {
      const flag = IF_FLAGS[`IF_${m[1]}`];
      if (flag === undefined) throw new Error(`未登记 IF 宏: IF_${m[1]}`);
      if (flag) out.set(m[2], true);
    }
    // 嵌套宏引用
    for (const m of d[0].matchAll(/TURBOSHAFT_(\w+)_OPERATION_LIST\(V\)/g)) {
      const sub = macroRef(m[1]);
      for (const n of sub) out.set(n, true);
    }
  }
  return [...out.keys()];
}
const macroCache = new Map();
function macroRef(bareName) {
  if (macroCache.has(bareName)) return macroCache.get(bareName);
  macroCache.set(bareName, []); // 先占位防环
  const full = `TURBOSHAFT_${bareName}_OPERATION_LIST`;
  // 该宏的定义文本 (可能多处 #ifdef 分支, 取目标配置下生效者)
  const re = new RegExp(`#define ${full}\\(V\\)${CONT_LINE}`, "g");
  const defs = [...h.matchAll(re)].map((d) => d[0]);
  if (defs.length === 0) throw new Error(`未找到宏 ${full}`);
  // 选最后一个定义 (源序: 空定义#else 在后时取生效分支前的一个; 这里按
  // 配置判定: 若存在 "空" 定义与非空定义, 以配置开关决定)
  const nonEmpty = defs.filter((d) => /V\(\w+\)/.test(d));
  let chosen;
  const simd256 = full.includes("SIMD256");
  const wasm = full.includes("WASM");
  const cped = full.includes("CPED");
  if (simd256 && !CONFIG.V8_ENABLE_SIMD256) chosen = defs[defs.length - 1];
  else if (wasm && !CONFIG.V8_ENABLE_WEBASSEMBLY) chosen = defs[defs.length - 1];
  else if (cped && !CONFIG.V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA) chosen = defs[defs.length - 1];
  else chosen = nonEmpty[nonEmpty.length - 1] ?? defs[defs.length - 1];
  const names = [];
  const bodyOnly = chosen.replace(/^#define [^\n]*/, ""); // 剥宏头, 防自引用
  for (const m of bodyOnly.matchAll(/V\((\w+)\)/g)) names.push(m[1]);
  // IF_X(V, Item) 条件项: 开则展开 Item
  for (const m of bodyOnly.matchAll(/IF_(\w+)\(V,\s*(\w+)\)/g)) {
    const flag = IF_FLAGS[`IF_${m[1]}`];
    if (flag === undefined) throw new Error(`未登记 IF 宏: IF_${m[1]}`);
    if (flag) names.push(m[2]);
  }
  for (const m of bodyOnly.matchAll(/TURBOSHAFT_(\w+)_OPERATION_LIST\(V\)/g)) {
    if (m[1] === bareName) continue;
    names.push(...macroRef(m[1]));
  }
  macroCache.set(bareName, names); // 先占位防环
  return names;
}

// 顶层组合 (opcodes.h 展开序): BLOCK_TERMINATOR 先, 其后按
// WASM → SIMD → MACHINE → SIMPLIFIED → JS → OTHER
const BLOCK_TERMINATOR = macroList("TURBOSHAFT_OPERATION_LIST_BLOCK_TERMINATOR");
const wasmOps = CONFIG.V8_ENABLE_WEBASSEMBLY ? macroRef("WASM") : [];
const simdOps = CONFIG.V8_ENABLE_SIMD128 ? macroRef("SIMD") : [];
const machineOps = macroRef("MACHINE");
const simplifiedOps = macroRef("SIMPLIFIED");
const jsThrowingOps = macroRef("JS_THROWING");
const jsOps = macroRef("JS");
const otherOps = macroRef("OTHER");
const cpedOps = CONFIG.V8_ENABLE_CONTINUATION_PRESERVED_EMBEDDER_DATA ? macroRef("CPED") : [];

// SIMD256 开启时插在 SIMD 尾部 (此处关, 仅登记)
const ALL = [
  ...BLOCK_TERMINATOR,
  ...wasmOps,
  ...simdOps,
  ...machineOps,
  ...simplifiedOps,
  ...jsOps,
  ...otherOps,
];
// 对数: CPED 不在 OPERATION_LIST 组合内 (独立列表), 登记不计入 Opcode
console.log(`CPED 独立列表 ${cpedOps.length} 项 (不入 Opcode); JS_THROWING ${jsThrowingOps.length} 项`);

// JS 列表含 THROWING 组合展开 (TURBOSHAFT_JS_OPERATION_LIST 引用 JS_THROWING)
// 已由 macroRef 递归处理。

const dupCheck = new Set();
for (const n of ALL) {
  if (dupCheck.has(n)) throw new Error(`Opcode 重复: ${n}`);
  dupCheck.add(n);
}
const kNumberOfOpcodes = ALL.length;
const kNumberOfBlockTerminatorOpcodes = BLOCK_TERMINATOR.length;
if (kNumberOfBlockTerminatorOpcodes !== (CONFIG.V8_ENABLE_WEBASSEMBLY ? 9 : 8)) {
  // 8 直项 (CheckException/Goto/TailCall/Unreachable/Return/Branch/Switch/Deoptimize)
  // + IF_WASM(V, WasmTrap) (WASM 开时 9)
  throw new Error(`块终结符 ${kNumberOfBlockTerminatorOpcodes} 异常`);
}
if (kNumberOfOpcodes < 150) throw new Error(`Opcode 总数 ${kNumberOfOpcodes} 异常偏少`);
// 组合自洽: 各子表之和 == 总表 (含 SIMD 嵌套展开)
const sumParts = BLOCK_TERMINATOR.length + wasmOps.length + simdOps.length + machineOps.length + simplifiedOps.length + jsOps.length + otherOps.length;
if (sumParts !== kNumberOfOpcodes) throw new Error(`子表和 ${sumParts} != 总表 ${kNumberOfOpcodes}`);

// ---------------------------------------------------------------------------
// 2. MayThrow 集合
// ---------------------------------------------------------------------------
const throwingOps = [...jsThrowingOps, "Call", "FastApiCall"];
for (const n of throwingOps) {
  if (!dupCheck.has(n) && n !== "FastApiCall") throw new Error(`MayThrow 操作不在 Opcode 表: ${n}`);
}

// ---------------------------------------------------------------------------
// 3. OpcodeName 名表 (operations.cc)
// ---------------------------------------------------------------------------
const nameFn = cc.match(/const char\* OpcodeName\(Opcode opcode\)\s*\{[\s\S]*?\n\}/);
if (!nameFn) throw new Error("未找到 OpcodeName");
if (!nameFn[0].includes("TURBOSHAFT_OPERATION_LIST(OPCODE_NAME)")) throw new Error("OpcodeName 表结构异常");
// 名表 = 操作名本身 (由宏生成), 条数 == kNumberOfOpcodes

// ---------------------------------------------------------------------------
// 4. HashingStrategy
// ---------------------------------------------------------------------------
const hashingM = h.match(/enum class HashingStrategy\s*\{([\s\S]*?)\};/);
if (!hashingM) throw new Error("未找到 HashingStrategy");
const hashingVariants = [...hashingM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
if (hashingVariants.length !== 2) throw new Error("HashingStrategy != 2 变体");

// ---------------------------------------------------------------------------
// 5. 各 *Op::Kind 子枚举
// ---------------------------------------------------------------------------
const kindEnums = [];
const enumRe = /enum class Kind : uint8_t\s*\{([^}]*)\}/g;
let em;
while ((em = enumRe.exec(h)) !== null) {
  // 回溯最近 struct 名
  const before = h.slice(Math.max(0, em.index - 1200), em.index);
  const sm = [...before.matchAll(/struct (\w+Op)\b/g)].pop();
  if (!sm) continue;
  const structName = sm[1];
  const body = em[1].replace(/\/\/[^\n]*/g, "");
  const variants = [...body.matchAll(/\bk([A-Z]\w*)\b/g)].map((m) => m[1]);
  if (variants.length === 0) continue;
  // 去重 (同 struct 多 Kind 定义不常见; 显式赋值枚举可能重复扫描)
  const seen = new Set();
  const uniq = variants.filter((v) => (seen.has(v) ? false : (seen.add(v), true)));
  kindEnums.push({ structName, variants: uniq });
}
const dupKind = new Set();
for (const k of kindEnums) {
  if (dupKind.has(k.structName)) throw new Error(`重复 Kind 枚举: ${k.structName}`);
  dupKind.add(k.structName);
}
if (kindEnums.length < 30) throw new Error(`Kind 枚举仅 ${kindEnums.length}, 预期 >=30`);

// ---------------------------------------------------------------------------
// 6. operations.cc 各 Kind 名表 (operator<< case 串)
// ---------------------------------------------------------------------------
function ccNameTables() {
  const out = new Map(); // "Struct::Kind" → [[variant, str]]
  const re = /std::ostream& operator<<\(std::ostream& os, ([\w:]+) (?:kind|assumption|implementation)\)\s*\{[\s\S]*?\n\}/g;
  let m;
  while ((m = re.exec(cc)) !== null) {
    const typeName = m[1]; // 形如 GenericBinopOp::Kind / ChangeOp::Assumption
    const rows = [];
    for (const c of m[0].matchAll(/case ([\w:]+)::k(\w+):\s*(?:\{)?\s*return os << "([^"]+)"/g)) {
      rows.push([c[2], c[3]]);
    }
    if (rows.length === 0) continue;
    out.set(typeName, rows);
  }
  return out;
}
const ccTables = ccNameTables();

// ---------------------------------------------------------------------------
// 7. 生成 Rust
// ---------------------------------------------------------------------------
let rust = banner(
  "src/compiler/turboshaft/operations.h.js",
  "src/compiler/turboshaft/operations.{h,cc}",
  "Turboshaft 操作码表: Opcode 枚举/块终结符前缀/MayThrow 集/各 Op::Kind 子枚举",
);

rust += `
// 目标配置: WASM=1, SIMD128=1, SIMD256=0 (arm64 无 SIMD256), CPED=1
// kOperationSizeTable / kOperationEffectsTable 依赖 C++ sizeof 与 OpEffects
// 构造, 不可机械展开 — skip 登记, Developer 按 struct 布局补。

/// 原版 enum class Opcode : uint8_t — 操作码 (块终结符占前缀)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Opcode {
`;
ALL.forEach((n, i) => {
  rust += `  /// k${n}\n  ${n} = ${i},\n`;
});
rust += `}

/// 原版 kNumberOfBlockTerminatorOpcodes
pub const NUMBER_OF_BLOCK_TERMINATOR_OPCODES: u16 = ${kNumberOfBlockTerminatorOpcodes};

/// 原版 kNumberOfOpcodes
pub const NUMBER_OF_OPCODES: u16 = ${kNumberOfOpcodes};

impl Opcode {
  /// 原版 OpcodeIndex
  #[inline]
  #[must_use]
  pub const fn index(self) -> u8 {
    self as u8
  }

  /// 原版 IsBlockTerminator — 块终结符占码表前缀
  #[inline]
  #[must_use]
  pub const fn is_block_terminator(self) -> bool {
    (self as u16) < NUMBER_OF_BLOCK_TERMINATOR_OPCODES
  }

  /// 原版 OpcodeName (operations.cc 表)
  #[must_use]
  pub const fn name(self) -> &'static str {
    OPCODE_NAMES[self as usize]
  }

  /// 原版 MayThrow — 后随 DidntThrow 的操作
  #[inline]
  #[must_use]
  pub const fn may_throw(self) -> bool {
    matches!(
      self,
      ${throwingOps.map((n) => `Opcode::${n}`).join(" | ")}
    )
  }
}

/// 原版 OpcodeName 表 (operations.cc, 由宏展开生成, 顺序同 Opcode)
#[rustfmt::skip]
pub static OPCODE_NAMES: [&str; NUMBER_OF_OPCODES as usize] = [
`;
for (let i = 0; i < ALL.length; i += 8) {
  rust += `  ${ALL.slice(i, i + 8).map((n) => `"${n}"`).join(", ")},\n`;
}
rust += `];

/// 原版 enum class HashingStrategy — 图哈希策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HashingStrategy {
  /// kDefault
  Default = 0,
  /// kMakeSnapshotStable — mksnapshot 复跑同哈希 (禁指针/外部常量入哈希)
  MakeSnapshotStable = 1,
}
`;

// Kind 子枚举
rust += "\n// ---------------------------------------------------------------------------\n// 各操作 Kind 子枚举 (struct 内 enum class Kind; 显式赋值已展开)\n// ---------------------------------------------------------------------------\n";
for (const k of kindEnums) {
  const rustEnumName = k.structName.replace(/Op$/, "Kind");
  rust += `
/// 原版 ${k.structName}::Kind (${k.variants.length} 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ${rustEnumName} {
`;
  // 数字开头变体 (k8x32 等) 加 B 前缀保持合法标识符
  const rv = (v) => (/^\d/.test(v) ? `B${v}` : v);
  k.variants.forEach((v) => {
    rust += `  /// k${v}\n  ${rv(v)} = ${k.variants.indexOf(v)},\n`;
  });
  rust += `}

impl ${rustEnumName} {
  /// 从数值反查 (越界回退首变体)
  #[must_use]
  pub const fn from_repr(val: u8) -> Self {
`;
  k.variants.forEach((v, i) => {
    rust += `    if val == ${i} { return Self::${rv(v)}; }\n`;
  });
  rust += `    Self::${rv(k.variants[0])}
  }
`;
  // as_str 名表 (cc 有对应 operator<< 时)
  const typeName = `${k.structName}::Kind`;
  const tbl = ccTables.get(typeName);
  if (tbl) {
    rust += `
  /// 原版 operator<< 名 (operations.cc, ${tbl.length} 项)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
`;
    for (const [variant, str] of tbl) {
      if (!k.variants.includes(variant)) throw new Error(`${typeName} 名表变体 ${variant} 不在枚举`);
      rust += `      Self::${variant} => "${str}",\n`;
    }
    rust += `    }
  }
`;
  }
  rust += `}
`;
}

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(NUMBER_OF_OPCODES == ${kNumberOfOpcodes});
  assert!(NUMBER_OF_BLOCK_TERMINATOR_OPCODES == ${kNumberOfBlockTerminatorOpcodes});
  assert!(Opcode::CheckException.is_block_terminator());
  assert!(Opcode::WasmTrap.is_block_terminator());
  assert!(!Opcode::Call.is_block_terminator());
  assert!(Opcode::Call.may_throw());
  assert!(Opcode::Goto.is_block_terminator());
  assert!(Opcode::index(Opcode::Call) >= 1);
  assert!(OPCODE_NAMES.len() == NUMBER_OF_OPCODES as usize);
};
// OPCODE_NAMES[0] == "${ALL[0]}" — 字符串相等入集成测试 (const 上下文不可比)
`;

const p = writeSrc("v8-turboshaft", "operations.rs", rust);
console.log(`written ${p}`);
console.log(`Opcode=${kNumberOfOpcodes} (terminator=${kNumberOfBlockTerminatorOpcodes}, wasm=${wasmOps.length}, simd=${simdOps.length}, machine=${machineOps.length}, simplified=${simplifiedOps.length}, js=${jsOps.length}, other=${otherOps.length})`);
console.log(`MayThrow=${throwingOps.length} HashingStrategy=2 Kind子枚举=${kindEnums.length} cc名表=${ccTables.size}`);
