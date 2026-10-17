#!/usr/bin/env bun
// tool/src/maglev/maglev-ir-opcodes.js — Maglev Opcode 清单 + 分组区间 + opcode 集合谓词
// 覆盖: v8/src/maglev/maglev-ir.h (NODE_BASE_LIST 宏族 → Opcode 枚举;
//       std::min/max 分组区间常量; switch/== 链式 opcode 集合谓词)
// 产出: v8-maglev-ir/src/opcodes.rs
// 配置: 目标快照 V8_INTL_SUPPORT=off / V8_ENABLE_UNDEFINED_DOUBLE=off (V8 默认),
//       其余三种组合的计数在断言中登记, 禁静默吞账。
// 复跑幂等: 纯文本宏展开 + 原版宏顺序 + 确定性输出, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/maglev-ir-opcodes.js";
const SRC = "src/maglev/maglev-ir.h";
const txt = readV8(SRC);
const die = (m) => {
  throw new Error(`[maglev-ir-opcodes.js] ${m}`);
};

// —— 白名单: 参与展开的 *_NODE_LIST 宏 (逐一对应原版 #define) ———
const LIST_MACROS = [
  "GENERIC_OPERATIONS_NODE_LIST",
  "BIGINT_OPERATIONS_NODE_LIST",
  "INT32_BITWISE_BINARY_OPERATIONS_NODE_LIST",
  "INT32_OPERATIONS_NODE_LIST",
  "FLOAT64_OPERATIONS_NODE_LIST",
  "SMI_OPERATIONS_NODE_LIST",
  "CONSTANT_VALUE_NODE_LIST",
  "INLINE_BUILTIN_NODE_LIST",
  "TURBOLEV_VALUE_NODE_LIST",
  "TURBOLEV_NON_VALUE_NODE_LIST",
  "CONVERSION_NODE_LIST",
  "VALUE_NODE_LIST",
  "GAP_MOVE_NODE_LIST",
  "NON_VALUE_NODE_LIST",
  "NODE_LIST",
  "BRANCH_CONTROL_NODE_LIST",
  "CONDITIONAL_CONTROL_NODE_LIST",
  "UNCONDITIONAL_CONTROL_NODE_LIST",
  "TERMINAL_CONTROL_NODE_LIST",
  "CONTROL_NODE_LIST",
  "NODE_BASE_LIST",
];
// 对数: 21 个子列表宏 (期望值独立抄自 maglev-ir.h:105-553)
if (LIST_MACROS.length !== 21) die(`白名单宏数=${LIST_MACROS.length}, 期望 21`);

// —— 解析 #define NAME(V) ... 续行体 ———
const lines = txt.split("\n");
const defs = new Map();
for (let i = 0; i < lines.length; i++) {
  const m = /^#define\s+(\w+)\(V\)\s*(.*)$/.exec(lines[i]);
  if (!m || !LIST_MACROS.includes(m[1])) continue;
  const body = [m[2]];
  let j = i;
  while (lines[j].replace(/\s+$/, "").endsWith("\\")) {
    j++;
    body.push(lines[j]);
  }
  if (defs.has(m[1])) die(`宏重复定义 ${m[1]}`);
  defs.set(m[1], body);
  i = j;
}
for (const l of LIST_MACROS) if (!defs.has(l)) die(`缺宏 #define ${l}(V)`);

// —— 条目语法: V(Name) / IF_xxx(V, Name) / Sub(V) ———
const CONDS = {
  IF_UD: (cfg) => cfg.UD,
  IF_NOT_UD: (cfg) => !cfg.UD,
  IF_INTL: (cfg) => cfg.INTL,
  IF_NOT_INTL: (cfg) => !cfg.INTL,
};
function items(body) {
  const out = [];
  for (const raw of body) {
    const s = raw.trim().replace(/\\$/, "").trim();
    if (!s || s.startsWith("//") || s.startsWith("/*") || s.startsWith("*")) continue;
    let m = /^V\((\w+)\)$/.exec(s);
    if (m) {
      out.push({ kind: "name", name: m[1] });
      continue;
    }
    m = /^(IF_UD|IF_NOT_UD|IF_INTL|IF_NOT_INTL)\(V,\s*(\w+)\)$/.exec(s);
    if (m) {
      out.push({ kind: m[1], name: m[2] });
      continue;
    }
    m = /^(\w+)\(V\)$/.exec(s);
    if (m) {
      out.push({ kind: "inc", name: m[1] });
      continue;
    }
    die(`宏体无法解析: ${JSON.stringify(s)}`);
  }
  return out;
}
function expand(mac, cfg, depth = 0) {
  if (depth > 8) die(`宏递归过深 ${mac}`);
  const sub = defs.get(mac);
  if (!sub) die(`未登记宏 ${mac}`);
  const out = [];
  for (const it of items(sub)) {
    if (it.kind === "name") out.push(it.name);
    else if (it.kind === "inc") out.push(...expand(it.name, cfg, depth + 1));
    else if (CONDS[it.kind](cfg)) out.push(it.name);
  }
  return out;
}

// —— 目标配置展开 + 对数断言 (期望值独立抄自原版宏表 python/clang 实测) ———
const CFG = { UD: false, INTL: false };
const EXPECT = {
  GENERIC_OPERATIONS_NODE_LIST: 22,
  BIGINT_OPERATIONS_NODE_LIST: 3,
  INT32_BITWISE_BINARY_OPERATIONS_NODE_LIST: 6,
  INT32_OPERATIONS_NODE_LIST: 25,
  FLOAT64_OPERATIONS_NODE_LIST: 19,
  SMI_OPERATIONS_NODE_LIST: 2,
  CONSTANT_VALUE_NODE_LIST: 10,
  INLINE_BUILTIN_NODE_LIST: 2,
  TURBOLEV_VALUE_NODE_LIST: 8,
  TURBOLEV_NON_VALUE_NODE_LIST: 6,
  CONVERSION_NODE_LIST: 42,
  VALUE_NODE_LIST: 271,
  GAP_MOVE_NODE_LIST: 2,
  NON_VALUE_NODE_LIST: 83,
  NODE_LIST: 354,
  BRANCH_CONTROL_NODE_LIST: 16,
  CONDITIONAL_CONTROL_NODE_LIST: 17,
  UNCONDITIONAL_CONTROL_NODE_LIST: 3,
  TERMINAL_CONTROL_NODE_LIST: 4,
  CONTROL_NODE_LIST: 24,
  NODE_BASE_LIST: 378,
};
const groups = {};
for (const [mac, want] of Object.entries(EXPECT)) {
  const g = expand(mac, CFG);
  if (g.length !== want) die(`${mac} 展开=${g.length}, 期望 ${want}`);
  if (new Set(g).size !== g.length) die(`${mac} 展开含重复项`);
  groups[mac] = g;
}
// 条件分支对数 (禁静默吞账): UD-on 379 / INTL-on 379 / both 380
const ALT = [
  [{ UD: true, INTL: false }, 379],
  [{ UD: false, INTL: true }, 379],
  [{ UD: true, INTL: true }, 380],
];
const altCounts = ALT.map(([cfg, want]) => {
  const n = expand("NODE_BASE_LIST", cfg).length;
  if (n !== want) die(`非目标配置 ${JSON.stringify(cfg)} 展开=${n}, 期望 ${want}`);
  return n;
});
const opcodes = groups.NODE_BASE_LIST; // 顺序即 C++ 枚举序 (DEF_OPCODES 连续分配)
const value = new Map(opcodes.map((n, i) => [n, i]));
// 原版 switch/等式里的拼写是 `Opcode::k##Name`, 反解回类名 (k 前缀由宏拼接, 非猜测)
const cxxToName = new Map(opcodes.map((n) => [`k${n}`, n]));
function resolveK(rawList, where) {
  return rawList.map((raw) => {
    const n = cxxToName.get(raw);
    if (!n) die(`${where}: ${raw} 非 NODE_BASE_LIST 成员`);
    return n;
  });
}
const count = opcodes.length;
if (count > 0xffff) die(`opcode 数 ${count} 超 OpcodeField 16 位 (static_assert is_valid)`);
// 原版枚举声明锚
for (const needle of [
  "enum class Opcode : uint16_t { NODE_BASE_LIST(DEF_OPCODES) };",
  "#define DEF_OPCODES(type) k##type,",
  "static constexpr int kOpcodeCount = NODE_BASE_LIST(PLUS_ONE);",
  "static constexpr Opcode kFirstOpcode = static_cast<Opcode>(0);",
]) {
  if (!txt.includes(needle)) die(`缺针 ${needle}`);
}

// —— 分组区间常量: 复刻 std::min({LIST(V) kLastOpcode}) / std::max({LIST(V) kFirstOpcode}) ———
const RANGE_GROUPS = [
  ["ValueNode", "VALUE_NODE_LIST"],
  ["ConstantNode", "CONSTANT_VALUE_NODE_LIST"],
  ["GapMoveNode", "GAP_MOVE_NODE_LIST"],
  ["ConversionNode", "CONVERSION_NODE_LIST"],
  ["Node", "NODE_LIST"],
  ["BranchControlNode", "BRANCH_CONTROL_NODE_LIST"],
  ["ConditionalControlNode", "CONDITIONAL_CONTROL_NODE_LIST"],
  ["UnconditionalControlNode", "UNCONDITIONAL_CONTROL_NODE_LIST"],
  ["TerminalControlNode", "TERMINAL_CONTROL_NODE_LIST"],
  ["ControlNode", "CONTROL_NODE_LIST"],
];
const ranges = RANGE_GROUPS.map(([label, mac]) => {
  const g = groups[mac];
  const vs = g.map((n) => value.get(n));
  // 原版 min/max 语义: 成员必属全表
  if (vs.some((v) => v === undefined)) die(`${mac} 成员不在 NODE_BASE_LIST`);
  return { label, mac, first: Math.min(...vs), last: Math.max(...vs), size: g.length };
});
// 锚: NODE_LIST 与 CONTROL_NODE_LIST 必严格分块 (kFirstNodeOpcode < kFirstControlNodeOpcode)
const rNode = ranges.find((r) => r.label === "Node");
const rCtrl = ranges.find((r) => r.label === "ControlNode");
if (!(rNode.last < rCtrl.first)) die("NODE_LIST 与 CONTROL_NODE_LIST 交错, 违反原版区间假设");

// —— opcode 集合谓词抽取 (switch(case)/等式链 两种原版写法) ———
function fnBody(name) {
  const at = txt.indexOf(`constexpr bool ${name}(`);
  if (at < 0) die(`缺函数 ${name}`);  const open = txt.indexOf("{", at);
  let d = 0;
  for (let k = open; k < txt.length; k++) {
    if (txt[k] === "{") d++;
    else if (txt[k] === "}") {
      d--;
      if (d === 0) return txt.slice(open + 1, k);
    }
  }
  die(`函数 ${name} 体未闭合`);
}
// switch 形: 累积 `case Opcode::X:` 直到 return 表达式
function switchSets(name) {
  const body = fnBody(name).replace(/\s+/g, " ");
  const buckets = [];
  const re = /(?:case Opcode::(\w+):\s*)+return ([^;]+);/g;
  let m;
  let consumed = 0;
  while ((m = re.exec(body))) {
    const seg = m[0];
    const cases = [...seg.matchAll(/case Opcode::(\w+):/g)].map((x) => x[1]);
    const ret = m[2].trim();
    buckets.push({ ret, cases });
    consumed += cases.length;
  }
  const total = (body.match(/case Opcode::\w+:/g) ?? []).length;
  if (total !== consumed) die(`${name} case 计数 ${consumed} != ${total}`);
  return buckets;
}
// 等式链形: `opcode == Opcode::X || ...`
function eqSet(name) {
  const body = fnBody(name).replace(/\s+/g, " ");
  const names = [...body.matchAll(/opcode == Opcode::(\w+)/g)].map((m) => m[1]);
  if (!names.length) die(`${name} 等式链为空`);
  if (body.includes("case Opcode::")) die(`${name} 混用 switch 与等式链`);
  return names;
}
function toOpcodes(list, where) {
  return list.map((n) => {
    const v = value.get(n);
    if (v === undefined) die(`${where}: ${n} 非 opcode`);
    return v;
  });
}

const SET_FNS = {
  IsCommutativeNode: { expect: 17, args: "Opcode opcode" },
  IsZeroCostNode: { expect: 4, args: "Opcode opcode" },
  IsTruncatingToInt32: { expect: 9, args: "Opcode opcode" },
  CanBeStoreToNonEscapedObject: { expect: 7, args: "Opcode opcode" },
  HasRangeField: { expect: 5, args: "Opcode opcode" },
  CanTriggerTruncationPass: { expect: 6, args: "Opcode opcode", prefix: "IsInt32BitwiseBinaryOperationNode" },
};
const sets = {};
for (const [fn, meta] of Object.entries(SET_FNS)) {
  const buckets = switchSets(fn);
  const t = buckets.filter((b) => b.ret === "true");
  if (t.length !== 1) die(`${fn} return-true 分支数=${t.length}, 期望 1`);
  const others = buckets.filter((b) => b.ret !== "true");
  if (others.length) die(`${fn} 存在非 true 分支, 需登记: ${others.map((b) => b.ret)}`);
  if (t[0].cases.length !== meta.expect) die(`${fn} case=${t[0].cases.length}, 期望 ${meta.expect}`);
  const names = resolveK(t[0].cases, fn);
  sets[fn] = { cases: names, values: toOpcodes(names, fn) };
}
const EQ_FNS = {
  IsSimpleFieldStore: 15,
  IsElementsArrayWrite: 2,
  PreservesTaggedKeyedProperties: 5,
  IsTypedArrayStore: 6,
};
for (const [fn, want] of Object.entries(EQ_FNS)) {
  const names = resolveK(eqSet(fn), fn);
  if (names.length !== want) die(`${fn} 等式=${names.length}, 期望 ${want}`);
  sets[fn] = { cases: names, values: toOpcodes(names, fn) };
}
// 子列表宏形 (IsInt32BitwiseBinaryOperationNode)
sets.IsInt32BitwiseBinaryOperationNode = {
  cases: groups.INT32_BITWISE_BINARY_OPERATIONS_NODE_LIST,
  values: toOpcodes(groups.INT32_BITWISE_BINARY_OPERATIONS_NODE_LIST, "IsInt32BitwiseBinaryOperationNode"),
};
// 三分支形: CanBeHoleValue(hole_index, opcode)
const hole = switchSets("CanBeHoleValue").flatMap((b) => b.cases.map((c) => [c, b.ret]));
if (hole.length !== 11) die(`CanBeHoleValue case=${hole.length}, 期望 11`);
const HOLE_TRUE = "true";
const HOLE_THE = "hole_index == RootIndex::kTheHoleValue";
const HOLE_TDZ = "hole_index == RootIndex::kTdzHoleValue";
const pick = (kind) => resolveK(hole.filter(([, r]) => r === kind).map(([n]) => n), "CanBeHoleValue");
const holeBuckets = [
  { rust: "always", cxx: HOLE_TRUE, names: pick(HOLE_TRUE), want: 5 },
  { rust: "the_hole_only", cxx: HOLE_THE, names: pick(HOLE_THE), want: 2 },
  { rust: "tdz_hole_only", cxx: HOLE_TDZ, names: pick(HOLE_TDZ), want: 4 },
];
for (const b of holeBuckets) {
  if (b.names.length !== b.want) die(`CanBeHoleValue[${b.rust}]=${b.names.length}, 期望 ${b.want}`);
}
if (holeBuckets.reduce((a, b) => a + b.names.length, 0) !== 11) die("CanBeHoleValue 三桶总数 != 11");
if (new Set(hole.map(([n]) => n)).size !== 11) die("CanBeHoleValue case 重复");

// —— 产出 ———
const L = [];
const push = (s = "") => L.push(s);
push(banner(GEN, SRC, `Opcode 全表 ${count}; 非目标配置计数 ${altCounts.join("/")} (UD-only / INTL-only / UD+INTL)`));
push("//! Maglev IR opcode 纯数据面: 枚举、分组区间常量、opcode 集合谓词。");
push("//! 原版锚: maglev-ir.h `enum class Opcode : uint16_t { NODE_BASE_LIST(DEF_OPCODES) }`。");
push("//! 目标配置 V8_INTL_SUPPORT=off / V8_ENABLE_UNDEFINED_DOUBLE=off; 其余组合见台账 readme/zh.md。");
push();
push("/// 原版 `static constexpr int kOpcodeCount`。");
push(`pub const OPCODE_COUNT: usize = ${count};`);
push("/// 原版 `kLastOpcode`: 枚举最大判别值。");
push(`pub const LAST_OPCODE: u16 = ${count - 1};`);
push("/// 原版 `kFirstOpcode`: 枚举从 0 起连续分配。");
push("pub const FIRST_OPCODE: u16 = 0;");
push();
push("/// Opcode 全表, 顺序与判别值 1:1 复刻 `NODE_BASE_LIST(DEF_OPCODES)` 连续分配序。");
push("#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]");
push("#[repr(u16)]");
push("pub enum Opcode {");
for (const [i, n] of opcodes.entries()) push(`  ${n} = ${i},`);
push("}");
push();
push("/// C++ 类名表, 下标即枚举值 (原版 `Opcode::k##Name` 去掉 k 前缀)。");
push(`pub const OPCODE_NAMES: [&str; OPCODE_COUNT] = [`);
for (const n of opcodes) push(`  ${JSON.stringify(n)},`);
push("];");
push();
push("/// 判别值 → opcode 反查表, 下标即原版枚举序。");
push(`pub const OPCODES: [Opcode; OPCODE_COUNT] = [`);
for (const n of opcodes) push(`  Opcode::${n},`);
push("];");
push();
push("impl Opcode {");
push("  /// 判别值 (= 原版 `static_cast<uint16_t>(opcode)`)。");
push("  pub const fn to_u16(self) -> u16 {");
push("    self as u16");
push("  }");
push();
push("  /// 由判别值还原 opcode (越界返回 None, 对应原版 `OpcodeField::is_valid` 域)。");
push("  pub const fn from_u16(raw: u16) -> Option<Self> {");
push("    let idx = raw as usize;");
push("    if idx < OPCODE_COUNT {");
push("      Some(OPCODES[idx])");
push("    } else {");
push("      None");
push("    }");
push("  }");
push();
push("  /// 原版宏拼写 `k##Name` 去 k 前缀后的 C++ 类名。");
push("  pub const fn cxx_name(self) -> &'static str {");
push("    let idx = self as usize;");
push("    if idx < OPCODE_COUNT { OPCODE_NAMES[idx] } else { \"?\" }");
push("  }");
push("}");
push();
push("// —— 分组区间常量: 复刻 `std::min({LIST(V) kLastOpcode})` / `std::max({LIST(V) kFirstOpcode})` ——");
for (const r of ranges) {
  push();
  push(`/// ${r.mac} (${r.size} 项)`);
  push(`pub const FIRST_${SH(r.label)}_OPCODE: Opcode = Opcode::${opcodes[r.first]};`);
  push(`pub const LAST_${SH(r.label)}_OPCODE: Opcode = Opcode::${opcodes[r.last]};`);
  push(`const ${SH(r.label)}_FIRST_RAW: u16 = ${r.first};`);
  push(`const ${SH(r.label)}_LAST_RAW: u16 = ${r.last};`);
}
push();
push("// —— 区间谓词 (原版 `constexpr bool IsXxxNode(Opcode)`) ——");
for (const r of ranges) {
  const fn = `is_${snake(r.label)}`;
  push(`/// 原版 \`constexpr bool Is${r.label}(Opcode)\`.`);
  push(`pub const fn ${fn}(opcode: Opcode) -> bool {`);
  push(`  in_range(opcode, ${SH(r.label)}_FIRST_RAW, ${SH(r.label)}_LAST_RAW)`);
  push("}");
  push();
}
push("/// 闭区间判定, 等价原版 `FIRST <= opcode && opcode <= LAST`。");
push("const fn in_range(opcode: Opcode, first: u16, last: u16) -> bool {");
push("  let v = opcode.to_u16();");
push("  v >= first && v <= last");
push("}");
push();
push("// —— opcode 集合谓词 (原版 switch(case)/等式链 白名单枚举) ——");
const order = [
  "IsInt32BitwiseBinaryOperationNode",
  "IsCommutativeNode",
  "IsZeroCostNode",
  "IsTruncatingToInt32",
  "CanBeStoreToNonEscapedObject",
  "HasRangeField",
  "IsSimpleFieldStore",
  "IsElementsArrayWrite",
  "PreservesTaggedKeyedProperties",
  "IsTypedArrayStore",
];
for (const fn of order) {
  const s = sets[fn];
  push(`/// 原版 \`constexpr bool ${fn}(Opcode)\` — ${s.cases.length} 项。`);
  push(`pub const fn ${snake(fn)}(opcode: Opcode) -> bool {`);
  push("  matches!(");
  push("    opcode,");
  push("    " + s.cases.map((c) => `Opcode::${c}`).join("\n    | "));
  push("  )");
  push("}");
  push();
}
// CanTriggerTruncationPass: 前置区间谓词 + switch
push("/// 原版 `constexpr bool CanTriggerTruncationPass(Opcode)`: 先查 int32 位运算集合, 再查表。");
push("pub const fn can_trigger_truncation_pass(opcode: Opcode) -> bool {");
push("  is_int32_bitwise_binary_operation_node(opcode)");
push("    || matches!(");
push("      opcode,");
push("      " + sets.CanTriggerTruncationPass.cases.map((c) => `Opcode::${c}`).join("\n      | "));
push("    )");
push("}");
push();
push("/// 原版 `RootIndex` 的 hole 二态 (零耦合本地建模, 对应 kTheHoleValue / kTdzHoleValue)。");
push("#[derive(Clone, Copy, Debug, PartialEq, Eq)]");
push("pub enum HoleIndex {");
push("  /// RootIndex::kTheHoleValue");
push("  TheHole,");
push("  /// RootIndex::kTdzHoleValue");
push("  TdzHole,");
push("}");
push();
push("/// 原版 `constexpr bool CanBeHoleValue(RootIndex, Opcode)` — 11 项分三桶。");
push("pub const fn can_be_hole_value(hole_index: HoleIndex, opcode: Opcode) -> bool {");
push("  match opcode {");
push("    " + holeBuckets[0].names.map((c) => `Opcode::${c}`).join("\n    | ") + " => true,");
push("    " + holeBuckets[1].names.map((c) => `Opcode::${c}`).join("\n    | ") +
      " => matches!(hole_index, HoleIndex::TheHole),");
push("    " + holeBuckets[2].names.map((c) => `Opcode::${c}`).join("\n    | ") +
      " => matches!(hole_index, HoleIndex::TdzHole),");
push("    _ => false,");
push("  }");
push("}");
push();
push("// 对数: 宏 21 + 各宏展开计数全锚 + 非目标配置 3 锚 + 区间组 10 +");
push("//   switch 集合 6 + 等式链集合 4 + hole 三桶(5/2/4) = 全过; 缺项即 throw。");

writeSrc("v8-maglev-ir", "opcodes.rs", L.join("\n") + "\n");
console.log(
  `OK maglev-ir-opcodes.js: ${count} opcode / ${ranges.length} 区间组 / ${order.length + 2} 集合谓词 → v8-maglev-ir/src/opcodes.rs`,
);

// 原版 UpperCamel 标签 → SCREAMING_SNAKE (rust 常量名)
function SH(s) {
  return snake(s).toUpperCase();
}
// Name → snake_case: 原版 UpperCamel 函数名/标签转 rust 命名
function snake(s) {
  return s.split(/(?=[A-Z])/).join("_").toLowerCase();
}
