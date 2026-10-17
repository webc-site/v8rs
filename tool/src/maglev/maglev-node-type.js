#!/usr/bin/env bun
// tool/src/maglev/maglev-node-type.js — NodeType 位集枚举 + 集合运算
// 覆盖: v8/src/maglev/maglev-node-type.h (LEAF/COMBINED_NODE_TYPE_LIST → enum NodeType,
//       kNumberOfLeafNodeTypes, Intersect/Union/Remove/Is/CanBe/IsEmpty,
//       IsNeverStandalone, NodeTypeIsVariant, DEFINE_NODE_TYPE_CHECK 生成 38 个检查函数)
// 产出: v8-maglev-ir/src/node_type.rs
// 对数: 叶子 18 + 组合 20 + 合计 38; 逐条原文表达式求值 + 7 项位集互校 + 原文锚 16;
//       漂移或缺项即 throw, 禁静默吞账。
// 复跑幂等: 纯文本扫描 + 原版表序 + 确定性输出。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/maglev-node-type.js";
const SRC = "src/maglev/maglev-node-type.h";
const txt = readV8(SRC);
const die = (m) => {
  throw new Error(`[maglev-node-type.js] ${m}`);
};

// —— 圈定列表宏体 (去续行) ——
function listBody(mac) {
  const at = txt.indexOf(`#define ${mac}(V)`);
  if (at < 0) die(`缺宏 #define ${mac}(V)`);
  const lines = txt.split("\n");
  const startLine = txt.slice(0, at).split("\n").length - 1;
  const body = [];
  for (let i = startLine + 1; i < lines.length; i++) {
    body.push(lines[i]);
    if (!lines[i].trimEnd().endsWith("\\")) return body.join("\n");
    if (i - startLine > 80) die(`${mac} 宏体超 80 行`);
  }
  die(`${mac} 宏体未结束`);
}
const leafSrc = listBody("LEAF_NODE_TYPE_LIST");
const combSrc = listBody("COMBINED_NODE_TYPE_LIST");

// 结构锚
for (const a of [
  "enum class NodeType : uint32_t {",
  "#define DEFINE_NODE_TYPE(Name, Value) k##Name = Value,",
  "NODE_TYPE_LIST(DEFINE_NODE_TYPE)",
  "using NodeTypeInt = std::underlying_type_t<NodeType>;",
  "static constexpr int kNumberOfLeafNodeTypes = 0 LEAF_NODE_TYPE_LIST(COUNT);",
  "#define NODE_TYPE_LIST(V) \\\n  LEAF_NODE_TYPE_LIST(V)  \\\n  COMBINED_NODE_TYPE_LIST(V)",
]) {
  if (!txt.includes(a)) die(`缺锚 ${JSON.stringify(a)}`);
}

// —— 解析 `V(Name, expr)`: 括号配对扫描, 不信正则的非贪婪 ——
function entries(src, where) {
  const out = [];
  for (let i = 0; i < src.length; i++) {
    if (src.slice(i, i + 2) !== "V(") continue;
    let d = 1;
    let j = i + 2;
    while (d > 0) {
      if (j >= src.length) die(`${where} V( 未闭合`);
      if (src[j] === "(") d++;
      else if (src[j] === ")") d--;
      j++;
    }
    const inner = src
      .slice(i + 2, j - 1)
      .replace(/\/\*[\s\S]*?\*\//g, "")
      .replace(/\/\/[^\n]*/g, "")
      .replace(/\\/g, ""); // 去续行反斜杠
    const comma = (() => {
      let depth = 0;
      for (let k = 0; k < inner.length; k++) {
        const c = inner[k];
        if (c === "(") depth++;
        else if (c === ")") depth--;
        else if (c === "," && depth === 0) return k;
      }
      return -1;
    })();
    if (comma < 0) die(`${where} 条目缺逗号: ${JSON.stringify(inner)}`);
    const name = inner.slice(0, comma).trim();
    const expr = inner.slice(comma + 1).replace(/\s+/g, " ").trim();
    if (!/^\w+$/.test(name)) die(`${where} 条目名非法: ${JSON.stringify(name)}`);
    out.push({ name, expr });
    i = j - 1;
  }
  if (!out.length) die(`${where} 未解析出条目`);
  return out;
}
const leafRows = entries(leafSrc, "LEAF_NODE_TYPE_LIST");
const combRows = entries(combSrc, "COMBINED_NODE_TYPE_LIST");
if (leafRows.length !== 18) die(`叶子类型=${leafRows.length}, 期望 18`);
if (combRows.length !== 20) die(`组合类型=${combRows.length}, 期望 20`);
const N_LEAF = leafRows.length;

// 叶子必为逐位 `(1 << i)`
const stripPar = (e) => (e.startsWith("(") && e.endsWith(")") ? stripPar(e.slice(1, -1)) : e);
leafRows.forEach((r, i) => {
  if (stripPar(r.expr) !== `1 << ${i}`) die(`叶子[${i}] ${r.name} 表达式=${r.expr}, 期望 (1 << ${i})`);
});

// —— 常量表达式求值器: 仅 ( ) << | - + 与 kName/十进制 ——
function evalBits(expr, map) {
  const s = expr;
  let i = 0;
  const skip = () => {
    while (s[i] === " ") i++;
  };
  const primary = () => {
    skip();
    if (s[i] === "(") {
      i++;
      const v = orExpr();
      skip();
      if (s[i] !== ")") die(`括号不匹配: ${expr}`);
      i++;
      return v;
    }
    let m = /^k\w+/.exec(s.slice(i));
    if (m) {
      if (!map.has(m[0])) die(`求值缺名 ${m[0]}`);
      i += m[0].length;
      return map.get(m[0]);
    }
    m = /^\d+/.exec(s.slice(i));
    if (!m) die(`无法解析 ${JSON.stringify(expr)} 于 ${i}`);
    i += m[0].length;
    return Number(m[0]);
  };
  const shiftExpr = () => {
    let v = primary();
    skip();
    if (s.slice(i, i + 2) === "<<") {
      i += 2;
      v = Number(BigInt(v) << BigInt(primary()));
    }
    return v;
  };
  const addExpr = () => {
    let v = shiftExpr();
    skip();
    while (s[i] === "-" || s[i] === "+") {
      const op = s[i];
      i++;
      const r = shiftExpr();
      v = op === "-" ? v - r : v + r;
      skip();
    }
    return v;
  };
  const orExpr = () => {
    let v = addExpr();
    skip();
    while (s[i] === "|") {
      i++;
      v |= addExpr();
      skip();
    }
    return v;
  };
  const v = orExpr();
  skip();
  if (i !== s.length) die(`表达式未完全消费: ${JSON.stringify(expr)}`);
  return v;
}

// 名字表: k<Name> → 位集值, 按原版声明顺序 (组合可引用前文组合)
const env = new Map();
leafRows.forEach((r, i) => env.set(`k${r.name}`, 1 << i));
for (const r of combRows) {
  const e = r.expr.replace(/kNumberOfLeafNodeTypes/g, String(N_LEAF));
  if (!/^[0-9kA-Za-z_ |()<>\s+-]+$/.test(e)) die(`组合 ${r.name} 含不支持算符: ${e}`);
  env.set(`k${r.name}`, evalBits(e, env));
}
// 位集互校 (期望独立抄自原版注释语义)
const chk = (name, want) => {
  const got = env.get(`k${name}`);
  if (got !== want) die(`k${name}=${got}, 期望 ${want}`);
};
chk("Unknown", (1 << N_LEAF) - 1);
chk("None", 0);
chk("AnyHeapObject", env.get("kUnknown") - env.get("kSmi"));
chk("Callable", env.get("kJSFunction") | env.get("kOtherCallable"));
chk("Number", env.get("kSmi") | env.get("kHeapNumber"));
chk("JSPrimitive",
  env.get("kNumber") | env.get("kString") | env.get("kBoolean") |
  env.get("kNullOrUndefined") | env.get("kSymbol") | env.get("kBigInt"));
chk("Oddball", env.get("kNullOrUndefined") | env.get("kBoolean"));
// 全部位值必落在 32 位枚举域且互异
const vals = [...env.values()];
if (vals.some((v) => v < 0 || v > 0xffffffff)) die("位值超 uint32 域");
if (new Set(vals).size !== vals.length) die("位值重复 (原版枚举允许重复判别值, 此处漂移需登记)");

// —— IsNeverStandalone 集合 ——
const nsAt = txt.indexOf("inline constexpr bool NodeTypeIsNeverStandalone(NodeType type) {");
if (nsAt < 0) die("缺 NodeTypeIsNeverStandalone");
const nsBody = txt.slice(nsAt, txt.indexOf("}", txt.indexOf("default:", nsAt)));
const nsNames = [...nsBody.matchAll(/case NodeType::k(\w+):/g)].map((m) => m[1]);
if (nsNames.join(",") !== "OtherCallable,OtherString") die(`IsNeverStandalone 漂移: ${nsNames.join(",")}`);

// —— NodeTypeIsVariant ——
const varAt = txt.indexOf("enum class NodeTypeIsVariant {");
if (varAt < 0) die("缺 enum class NodeTypeIsVariant");
// 先去整行注释, 再按逗号分裂 (注释里含逗号)
const varNames = txt
  .slice(varAt + "enum class NodeTypeIsVariant {".length, txt.indexOf("};", varAt))
  .split("\n")
  .filter((l) => !l.trim().startsWith("//"))
  .join(" ")
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (varNames.join(",") !== "kDefault,kAllowNone") die(`NodeTypeIsVariant 漂移: ${varNames.join(",")}`);

// —— 其余 constexpr 原文锚 ———
for (const a of [
  "#define DEFINE_NODE_TYPE_CHECK(Type, _)",
  "NODE_TYPE_LIST(DEFINE_NODE_TYPE_CHECK)",
  "inline constexpr NodeType EmptyNodeType() { return static_cast<NodeType>(0); }",
  "inline constexpr NodeType IntersectType(NodeType left, NodeType right) {",
  "inline constexpr NodeType UnionType(NodeType left, NodeType right) {",
  "inline constexpr NodeType RemoveType(NodeType left, NodeType right) {",
  "static_cast<NodeTypeInt>(left) &\n                               static_cast<NodeTypeInt>(right)",
  "static_cast<NodeTypeInt>(left) &\n                               ~static_cast<NodeTypeInt>(right)",
  "NodeTypeInt right = static_cast<NodeTypeInt>(to_check);\n  return (static_cast<NodeTypeInt>(type) & (~right)) == 0;",
  "inline constexpr bool NodeTypeIsForPrinting(NodeType type, NodeType to_check) {",
  "return (static_cast<NodeTypeInt>(type) & (right)) != 0;",
  "inline constexpr bool IsEmptyNodeType(NodeType type) {",
  "return static_cast<int>(type) == 0;",
  "static_assert(!NodeTypeCanBe(NodeType::kJSPrimitive, NodeType::kJSReceiver));",
  "inline bool NodeTypeMayBeNullOrUndefined(NodeType type) {",
  "if (variant != NodeTypeIsVariant::kAllowNone) {\n    DCHECK_NE(type, NodeType::kNone);\n  }",
]) {
  if (!txt.includes(a)) die(`缺锚 ${JSON.stringify(a.slice(0, 60))}`);
}

// —— 产出 ———
const all = [...leafRows, ...combRows];
if (all.length !== 38) die(`合计类型数=${all.length}, 期望 38`);
const L = [];
const push = (s = "") => L.push(s);
push(banner(GEN, SRC, `NodeType 位集: 叶子 ${leafRows.length} + 组合 ${combRows.length} = ${all.length}`));
push("//! Maglev IR 节点类型位集 (原版 `enum class NodeType : uint32_t`)。");
push("//! 位值逐条抄自 LEAF/COMBINED_NODE_TYPE_LIST 表达式并由脚本求值互校。");
push("//! 模板/引用面 (`Is<T>`/`As<T>`/`StaticTypeForMap`/`StaticTypeForConstant` 等) 非本波范围,");
push("//! 见 readme/zh.md skip 登记。");
push();
push("/// 原版 `kNumberOfLeafNodeTypes`。");
push(`pub const NUMBER_OF_LEAF_NODE_TYPES: u32 = ${N_LEAF};`);
push("/// 原版 `using NodeTypeInt = std::underlying_type_t<NodeType>`。");
push("pub type NodeTypeInt = u32;");
push();
push("/// 叶子类型名表 (原版 LEAF_NODE_TYPE_LIST 顺序)。");
push(`pub const LEAF_NODE_TYPE_NAMES: [&str; ${leafRows.length}] = [`);
for (const r of leafRows) push(`  ${JSON.stringify(r.name)},`);
push("];");
push();
push("/// 组合类型名 + 原文位表达式 (原版 COMBINED_NODE_TYPE_LIST 顺序)。");
push(`pub const COMBINED_NODE_TYPE_NAMES: [&str; ${combRows.length}] = [`);
for (const r of combRows) push(`  ${JSON.stringify(r.name)},`);
push("];");
push();
push("/// 节点类型位集: 原版是 `enum class NodeType : uint32_t` 当位集用,");
push("/// rust 侧建模为透明位集结构体, 判别值 1:1 复刻原版 `V(Name, value)`。");
push("#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]");
push("pub struct NodeType {");
push("  bits: NodeTypeInt,");
push("}");
push();
push("impl NodeType {");
push("  /// 原版裸位值入口 (`static_cast<NodeType>(int)`)。");
push("  pub const fn from_bits(bits: NodeTypeInt) -> Self {");
push("    Self { bits }");
push("  }");
push();
push("  /// 原版 `static_cast<NodeTypeInt>(type)`。");
push("  pub const fn to_int(self) -> NodeTypeInt {");
push("    self.bits");
push("  }");
push();
for (const [i, r] of all.entries()) {
  const note = i < leafRows.length ? `叶子位 (1 << ${i})` : `组合位 ${r.expr}`;
  push(`  /// ${note}`);
  push(`  pub const ${shout(r.name)}: Self = Self::from_bits(${env.get(`k${r.name}`)});`);
}
push("}");
push();
push("/// 类型名表 (原版 NODE_TYPE_LIST 顺序: 18 叶子后接 20 组合)。");
push(`pub const NODE_TYPE_NAMES: [(&str, NodeType); ${all.length}] = [`);
for (const r of all) push(`  (${JSON.stringify(r.name)}, NodeType::${shout(r.name)}),`);
push("];");
push();
push("/// 原版 `enum class NodeTypeIsVariant`。");
push("#[derive(Clone, Copy, Debug, PartialEq, Eq)]");
push("#[repr(u32)]");
push("pub enum NodeTypeIsVariant {");
push("  /// 不允许 lhs 为 None (原版 DCHECK_NE(type, kNone))");
push(`  ${varNames[0].slice(1)} = 0,`);
push("  /// 允许 lhs 为 None (原版注释: 通常意味着死代码)");
push(`  ${varNames[1].slice(1)} = 1,`);
push("}");
push();
push("/// 原版 `EmptyNodeType()`: 全位清零。");
push("pub const fn empty_node_type() -> NodeType {");
push("  NodeType::NONE");
push("}");
push();
push("/// 原版 `NodeTypeIsNeverStandalone` 登记的类型 (永不作为单独叶子类型出现)。");
push(`pub const NEVER_STANDALONE: [NodeType; ${nsNames.length}] = [`);
for (const n of nsNames) push(`  NodeType::${shout(n)},`);
push("];");
push();
push("/// 原版 `NodeTypeIsNeverStandalone(type)`。");
push("pub const fn node_type_is_never_standalone(node_type: NodeType) -> bool {");
push("  " + nsNames.map((n) => `node_type.to_int() == NodeType::${shout(n)}.to_int()`).join("\n    || "));
push("}");
push();
push("/// 原版 `IntersectType`: left & right。");
push("pub const fn intersect_type(left: NodeType, right: NodeType) -> NodeType {");
push("  NodeType::from_bits(left.to_int() & right.to_int())");
push("}");
push();
push("/// 原版 `UnionType`: left | right。");
push("pub const fn union_type(left: NodeType, right: NodeType) -> NodeType {");
push("  NodeType::from_bits(left.to_int() | right.to_int())");
push("}");
push();
push("/// 原版 `RemoveType`: left & !right。");
push("pub const fn remove_type(left: NodeType, right: NodeType) -> NodeType {");
push("  NodeType::from_bits(left.to_int() & !right.to_int())");
push("}");
push();
push("/// 原版 `IsEmptyNodeType`: 无位被置。");
push("pub const fn is_empty_node_type(node_type: NodeType) -> bool {");
push("  node_type.to_int() == 0");
push("}");
push();
push("/// 原版 `NodeTypeIs(type, to_check, variant)`: type 位集 ⊆ to_check。");
push("pub const fn node_type_is(");
push("  node_type: NodeType,");
push("  to_check: NodeType,");
push("  variant: NodeTypeIsVariant,");
push(") -> bool {");
push("  // 原版非 AllowNone 分支只有 DCHECK_NE(type, kNone); rust 侧无 DCHECK, 以 false 承接该非法输入");
push("  if matches!(variant, NodeTypeIsVariant::Default) && is_empty_node_type(node_type) {");
push("    return false;");
push("  }");
push("  (node_type.to_int() & !to_check.to_int()) == 0");
push("}");
push();
push("/// 原版 `NodeTypeIsForPrinting`: 同 NodeTypeIs 但去 DCHECK。");
push("pub const fn node_type_is_for_printing(node_type: NodeType, to_check: NodeType) -> bool {");
push("  (node_type.to_int() & !to_check.to_int()) == 0");
push("}");
push();
push("/// 原版 `NodeTypeCanBe(type, to_check)`: 位集有交集 (默认 allow_standalone=false)。");
push("pub const fn node_type_can_be(node_type: NodeType, to_check: NodeType) -> bool {");
push("  (node_type.to_int() & to_check.to_int()) != 0");
push("}");
push();
push("/// 原版 `NodeTypeMayBeNullOrUndefined`。");
push("pub const fn node_type_may_be_null_or_undefined(node_type: NodeType) -> bool {");
push("  (node_type.to_int() & NodeType::NULL_OR_UNDEFINED.to_int()) != 0");
push("}");
push();
push("// —— 原版 DEFINE_NODE_TYPE_CHECK 生成的逐类型检查 (默认 variant=kAllowNone) ——");
for (const r of all) {
  push(`/// 原版 \`NodeTypeIs${r.name}(type, variant = kAllowNone)\`。`);
  push(`pub const fn node_type_is_${snake(r.name)}(node_type: NodeType) -> bool {`);
  push(`  node_type_is(node_type, NodeType::${shout(r.name)}, NodeTypeIsVariant::AllowNone)`);
  push("}");
  push();
}
push("// 对数: 叶子 18 (逐条 (1<<i)) + 组合 20 (表达式求值 + 7 项互校) + 变体 2 +");
push("//   NodeTypeIsNeverStandalone 2 + 原文锚 16 + 生成检查 38 = 全过。");

writeSrc("v8-maglev-ir", "node_type.rs", L.join("\n") + "\n");
console.log(
  `OK maglev-node-type.js: ${all.length} NodeType (leaf ${leafRows.length}/comb ${combRows.length}) + ${all.length} 检查函数 → v8-maglev-ir/src/node_type.rs`,
);

// UpperCamel(含连续大写) → SCREAMING_SNAKE (rust 常量名)
function shout(n) {
  return snake(n).toUpperCase();
}
// UpperCamel(含连续大写) → snake_case
function snake(n) {
  return n
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .toLowerCase();
}
