#!/usr/bin/env bun
// tool/src/maglev/maglev-ir-props.js — OpProperties 位域 + 工厂 + 查询 + NodeBase 位段编码
// 覆盖: v8/src/maglev/maglev-ir.h (`class OpProperties` 全量 + `class NodeBase` 位域规格
//       "[opcode:16][input_count:16][properties:16][extras:16]")
// 产出: v8-maglev-ir/src/op_properties.rs
// 对数: 位域 10 + 工厂 27(26 公开 + 1 私有登记) + 无参查询 20 + 变换 3 + NodeBase 位段 6
//       + AttachedDeoptInfo 5 + ValueRepresentation 8; 每项带 C++ 原文锚与数值互校,
//       锚文本漂移/缺项即 throw, 禁静默吞账。
// 复跑幂等: 纯文本扫描 + 原版声明序 + 数值由位域链推导, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/maglev/maglev-ir-props.js";
const SRC = "src/maglev/maglev-ir.h";
const txt = readV8(SRC);
const die = (m) => {
  throw new Error(`[maglev-ir-props.js] ${m}`);
};

// —— 类体圈定 ——
const clsAt = txt.indexOf("class OpProperties {");
if (clsAt < 0) die("缺 class OpProperties {");
const cls = txt.slice(clsAt, txt.indexOf("\n};", clsAt));
if (cls.length < 100) die("class OpProperties 体未闭合");
const norm = (s) => s.replace(/\s+/g, " ").trim();
const clsN = norm(cls);

// —— 位域链: `using kX = base::BitField<T, shift, width>` / `using kX = Prev::Next<T, width>`
const bitRows = [...cls.matchAll(/using\s+(k\w+)\s*=\s*(base::BitField<[^;]+>|(\w+)::Next<[^;]+>);/g)]
  .map((m) => ({ alias: m[1], decl: norm(m[2]), prev: m[3] }));
if (bitRows.length !== 10) die(`位域别名数=${bitRows.length}, 期望 10`);
const bits = [];
for (const row of bitRows) {
  const wm = /,\s*(\d+)\s*>$/.exec(row.decl);
  if (!wm) die(`${row.alias} 宽度不可解析: ${row.decl}`);
  const width = Number(wm[1]);
  let shift;
  if (row.prev) {
    const p = bits.find((b) => b.alias === row.prev);
    if (!p) die(`${row.alias} 前驱 ${row.prev} 未在前文`);
    shift = p.shift + p.width;
  } else {
    const sm = /,\s*(\d+)\s*,\s*\d+\s*>$/.exec(row.decl);
    if (!sm) die(`${row.alias} 起始位不可解析`);
    shift = Number(sm[1]);
  }
  bits.push({ alias: row.alias, shift, width, ty: row.decl.split("<")[1].split(",")[0].trim() });
}
// 期望链 (独立抄自 maglev-ir.h 的 OpProperties 私有段)
const EXPECT_BITS = [
  ["kIsCallBit", 0, 1, "bool"],
  ["kAttachedDeoptInfoBits", 1, 3, "AttachedDeoptInfo"],
  ["kCanThrowBit", 4, 1, "bool"],
  ["kCanReadBit", 5, 1, "bool"],
  ["kCanWriteBit", 6, 1, "bool"],
  ["kCanAllocateBit", 7, 1, "bool"],
  ["kNotIdempotentBit", 8, 1, "bool"],
  ["kValueRepresentationBits", 9, 4, "ValueRepresentation"],
  ["kIsConversionBit", 13, 1, "bool"],
  ["kNeedsRegisterSnapshotBit", 14, 1, "bool"],
];
bits.forEach((b, i) => {
  const [alias, shift, width, ty] = EXPECT_BITS[i];
  if (b.alias !== alias || b.shift !== shift || b.width !== width || b.ty !== ty) {
    die(`位域链[${i}] = ${JSON.stringify(b)}, 期望 ${JSON.stringify({ alias, shift, width, ty })}`);
  }
});
const bit = (alias) => {
  const b = bits.find((x) => x.alias === alias);
  if (!b) die(`缺位域 ${alias}`);
  return b;
};
const maskOf = (alias) => {
  const b = bit(alias);
  return ((1 << b.width) - 1) << b.shift;
};
const kSizeM = /static const size_t kSize = (\d+);/.exec(clsN);
const kUsedM = /static const size_t kUsedSize = kNeedsRegisterSnapshotBit::kLastUsedBit \+ (\d+);/.exec(clsN);
if (!kSizeM || kSizeM[1] !== "16") die(`kSize 锚漂移: ${kSizeM?.[1]}`);
if (!kUsedM || kUsedM[1] !== "1") die(`kUsedSize 锚漂移: ${kUsedM?.[1]}`);
const kSize = Number(kSizeM[1]);
const kUsedSize = bit("kNeedsRegisterSnapshotBit").shift + Number(kUsedM[1]);
if (kUsedSize !== 15) die(`kUsedSize=${kUsedSize}, 期望 15`);
if (!clsN.includes("static_assert(kUsedSize <= kSize);")) die("缺 kUsedSize<=kSize 锚");
const pureMask = maskOf("kCanReadBit") | maskOf("kCanWriteBit") | maskOf("kCanAllocateBit");
for (const a of [
  "static const uint32_t kPureMask =\n      kCanReadBit::kMask | kCanWriteBit::kMask | kCanAllocateBit::kMask;",
  "kPureValue = kCanReadBit::encode(false)",
]) {
  if (!clsN.includes(norm(a))) die(`缺 kPure 锚 ${a.slice(0, 30)}`);
}
const PURE_VALUE = 0;

// —— AttachedDeoptInfo / ValueRepresentation 名字表 ——
const deoptAt = clsN.indexOf("enum class AttachedDeoptInfo {");
if (deoptAt < 0) die("缺 enum class AttachedDeoptInfo");
const deoptNames = clsN
  .slice(deoptAt + "enum class AttachedDeoptInfo {".length, clsN.indexOf("}", deoptAt))
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (deoptNames.join(",") !== "kNone,kEager,kLazy,kEagerAndLazy,kCheckpoint") {
  die(`AttachedDeoptInfo 漂移: ${deoptNames.join(",")}`);
}
const vrAt = txt.indexOf("enum class ValueRepresentation : uint8_t {");
if (vrAt < 0) die("缺 enum class ValueRepresentation");
const vrNames = [...txt.slice(vrAt, txt.indexOf("};", vrAt)).matchAll(/^\s{2}k(\w+),$/gm)].map((m) => m[1]);
if (vrNames.join(",") !== "Tagged,Int32,Uint32,Float64,HoleyFloat64,IntPtr,RawPtr,None") {
  die(`ValueRepresentation 漂移: ${vrNames.join(",")}`);
}
// 复刻 Bits::encode(arg): 参数 → 位域数值
function encodeVal(alias, arg) {
  const b = bit(alias);
  let v;
  if (arg === "true") v = 1;
  else if (arg === "false") v = 0;
  else {
    let m = /^AttachedDeoptInfo::(k\w+)$/.exec(arg);
    if (m) {
      const i = deoptNames.indexOf(m[1]);
      if (i < 0) die(`未知 AttachedDeoptInfo ${arg}`);
      v = i;
    } else {
      m = /^ValueRepresentation::k(\w+)$/.exec(arg);
      if (!m) die(`encode 参数不可解析: ${arg}`);
      const i = vrNames.indexOf(m[1]);
      if (i < 0) die(`未知 ValueRepresentation ${arg}`);
      v = i;
    }
  }
  return v << b.shift;
}

// —— 工厂: 抽原文 + 数值求值 (固定点解相互引用) ——
const factorySrc = [...cls.matchAll(/static constexpr OpProperties (\w+)\(([^)]*)\)\s*\{/g)].map((m) => {
  const at = Number(m.index);
  const open = cls.indexOf("{", at);
  let d = 0;
  let k = open;
  for (; k < cls.length; k++) {
    if (cls[k] === "{") d++;
    else if (cls[k] === "}") {
      d--;
      if (d === 0) break;
    }
  }
  const body = norm(cls.slice(open + 1, k).replace(/\/\/[^\n]*/g, ""));
  const rm = /^return (.*);$/s.exec(body);
  if (!rm) die(`工厂体非 return 形: ${body.slice(0, 60)}`);
  let e = rm[1];
  // 仅当整体被 OpProperties(...) 构造包裹时才剥壳
  if (e.startsWith("OpProperties(")) {
    let depth = 0;
    for (let i = 0; i < e.length; i++) {
      if (e[i] === "(") depth++;
      else if (e[i] === ")") {
        depth--;
        if (depth === 0) {
          if (i === e.length - 1) e = e.slice("OpProperties(".length, i);
          break;
        }
      }
    }
  }
  return { name: m[1], params: norm(m[2]), expr: norm(e), src: norm(cls.slice(at, k + 2)) };
});
if (factorySrc.length !== 27) die(`工厂数=${factorySrc.length}, 期望 27 (26 公开 + 1 私有)`);
const PRIVATE_FACTORIES = ["NeedsRegisterSnapshot"]; // 原版 private: 只内联其数值
const val = new Map();
function resolve(f, round = 0) {
  if (round > 8) die(`工厂互引用不可解: ${f.name}`);
  if (f.params) return undefined; // 带参工厂不做常数值求值
  let e = f.expr;
  e = e.replace(/(\w+)::encode\(([^)]*)\)/g, (_, alias, arg) => String(encodeVal(alias, arg.trim())));
  e = e.replace(/\bkPureValue\b/g, String(PURE_VALUE));
  e = e.replace(/\b([A-Z]\w*)\(\)/g, (_, n) => {
    const g = factorySrc.find((x) => x.name === n);
    if (!g) die(`${f.name} 引用了未知工厂 ${n}`);
    if (!val.has(n)) resolve(g, round + 1);
    return String(val.get(n));
  });
  for (let guard = 0; guard < 8; guard++) {
    const m = /OpProperties\(([^()]*)\)/.exec(e);
    if (!m) break;
    e = e.replace(m[0], m[1]);
  }
  if (/[A-Za-z_(]/.test(e)) die(`${f.name} 表达式残留不可解析符号: ${f.expr} → ${e}`);
  const v = e.split("|").reduce((acc, p) => acc | Number(p.trim()), 0);
  val.set(f.name, v);
  return v;
}
for (const f of factorySrc) resolve(f);
const pubCxx = (clsN.match(/static constexpr OpProperties \w+\(/g) ?? []).length;
if (pubCxx !== factorySrc.length) die(`原文工厂计数=${pubCxx}, 表=${factorySrc.length}`);
if (factorySrc.filter((f) => !f.params).length !== 26) die("无参工厂数漂移, 期望 26");
// 数值互校 (与求值路径独立手抄)
const expect = {
  Pure: 0,
  Call: 1 << bit("kIsCallBit").shift,
  EagerDeopt: 1 << bit("kAttachedDeoptInfoBits").shift,
  LazyDeopt: 2 << bit("kAttachedDeoptInfoBits").shift,
  DeoptCheckpoint: 4 << bit("kAttachedDeoptInfoBits").shift,
  CanThrow:
    (1 << bit("kCanThrowBit").shift) | (2 << bit("kAttachedDeoptInfoBits").shift) | (1 << bit("kCanAllocateBit").shift),
  TaggedValue: 0,
  IntPtr: 5 << bit("kValueRepresentationBits").shift,
  RawPtr: 6 << bit("kValueRepresentationBits").shift,
  Float64: 3 << bit("kValueRepresentationBits").shift,
  AnySideEffects: (1 << bit("kCanReadBit").shift) | (1 << bit("kCanWriteBit").shift) | (1 << bit("kCanAllocateBit").shift),
};
for (const [n, want] of Object.entries(expect)) {
  if (val.get(n) !== want) die(`工厂 ${n} = 0x${(val.get(n) ?? -1).toString(16)}, 期望 0x${want.toString(16)}`);
}
if (val.get("JSCall") !== (val.get("Call") | val.get("CanCallUserCode"))) die("JSCall 互校失败");
if (val.get("CanCallUserCode") !== (val.get("AnySideEffects") | val.get("LazyDeopt") | val.get("CanThrow"))) {
  die("CanCallUserCode 互校失败");
}
if (val.get("DeferredCall") !== val.get("NeedsRegisterSnapshot")) die("DeferredCall != NeedsRegisterSnapshot (原版 static_assert)");
if (val.get("GenericRuntimeOrBuiltinCall") !== (val.get("Call") | val.get("CanCallUserCode") | val.get("NotIdempotent"))) {
  die("GenericRuntimeOrBuiltinCall 互校失败");
}
if (val.get("TrustedPointer") !== val.get("TaggedValue")) die("TrustedPointer 应与 TaggedValue 同为 kTagged");
if (val.get("ExternalReference") !== val.get("IntPtr")) die("ExternalReference 应与 IntPtr 同为 kIntPtr");
const vrp = factorySrc.find((f) => f.params);
if (!vrp || vrp.name !== "ForValueRepresentation") die(`带参工厂漂移: ${vrp?.name}`);
for (const f of factorySrc) if (!clsN.includes(f.src)) die(`工厂锚自校验失败 ${f.name}`);

// —— 无参查询 ——
const QUERIES = [
  ["is_call", "kIsCallBit", "bool", "bool", "constexpr bool is_call() const"],
  ["is_any_call", "", "bool", "expr", "constexpr bool is_any_call() const { return is_call() || is_deferred_call(); }"],
  ["can_eager_deopt", "kAttachedDeoptInfoBits", "bool", "eager", "constexpr bool can_eager_deopt() const"],
  ["can_lazy_deopt", "kAttachedDeoptInfoBits", "bool", "lazy", "constexpr bool can_lazy_deopt() const"],
  ["is_deopt_checkpoint", "kAttachedDeoptInfoBits", "bool", "checkpoint", "constexpr bool is_deopt_checkpoint() const"],
  ["has_eager_deopt_info", "", "bool", "expr", "constexpr bool has_eager_deopt_info() const {"],
  ["can_deopt", "", "bool", "expr", "constexpr bool can_deopt() const {"],
  ["can_throw", "kCanThrowBit", "bool", "and", "constexpr bool can_throw() const {"],
  ["can_read", "kCanReadBit", "bool", "bool", "constexpr bool can_read() const { return kCanReadBit::decode(bitfield_); }"],
  ["can_write", "kCanWriteBit", "bool", "bool", "constexpr bool can_write() const { return kCanWriteBit::decode(bitfield_); }"],
  ["can_allocate", "kCanAllocateBit", "bool", "bool", "constexpr bool can_allocate() const {"],
  ["not_idempotent", "kNotIdempotentBit", "bool", "bool", "constexpr bool not_idempotent() const {"],
  ["value_representation", "kValueRepresentationBits", "u8", "u8", "constexpr ValueRepresentation value_representation() const {"],
  ["is_tagged", "", "bool", "expr", "constexpr bool is_tagged() const {"],
  ["is_conversion", "kIsConversionBit", "bool", "bool", "constexpr bool is_conversion() const {"],
  ["needs_register_snapshot", "kNeedsRegisterSnapshotBit", "bool", "bool", "constexpr bool needs_register_snapshot() const {"],
  ["is_deferred_call", "", "bool", "expr", "constexpr bool is_deferred_call() const {"],
  ["is_pure", "", "bool", "pure", "constexpr bool is_pure() const {\n    return (bitfield_ & kPureMask) == kPureValue;\n  }"],
  ["is_required_when_unused", "", "bool", "req", "constexpr bool is_required_when_unused() const {"],
  ["can_participate_in_cse", "", "bool", "expr", "constexpr bool can_participate_in_cse() const {\n    return !can_write() && !not_idempotent();\n  }"],
];
if (QUERIES.length !== 20) die("查询表须 20 项");
const cxxQueryCount = (clsN.match(/constexpr (?:bool|ValueRepresentation) \w+\(\) const/g) ?? []).length;
if (cxxQueryCount !== QUERIES.length) die(`原文无参查询数=${cxxQueryCount}, 表=${QUERIES.length}`);
for (const [rust, , , , anchor] of QUERIES) {
  if (!clsN.includes(norm(anchor))) die(`查询锚缺失: ${rust}`);
}
for (const a of [
  "OpProperties operator|(const OpProperties& that) {\n    return OpProperties(bitfield_ | that.bitfield_);\n  }",
  "OpProperties WithNewValueRepresentation(ValueRepresentation new_repr) const {\n    return OpProperties(kValueRepresentationBits::update(bitfield_, new_repr));\n  }",
  "OpProperties WithoutDeopt() const {\n    return OpProperties(\n        kAttachedDeoptInfoBits::update(bitfield_, AttachedDeoptInfo::kNone));\n  }",
]) {
  if (!clsN.includes(norm(a))) die(`变换锚缺失 ${a.slice(0, 40)}`);
}

// —— NodeBase 位段 ——
if (!txt.includes("  // Bitfield specification.\n  // [opcode:16][input_count:16][properties:16][extras:16]")) {
  die("缺 NodeBase 位域规格注释");
}
const nbAt = txt.indexOf("class NodeBase : public ZoneObject {");
if (nbAt < 0) die("缺 class NodeBase");
const nbBody = txt.slice(nbAt, txt.indexOf("public:", nbAt));
const nbRows = [...nbBody.matchAll(/using\s+(\w+)\s*=\s*(base::BitField64<[^;]+>|(\w+)::Next<[^;]+>);/g)]
  .map((m) => ({ alias: m[1], decl: norm(m[2]), prev: m[3] }));
if (nbRows.length !== 7) die(`NodeBase using 行数=${nbRows.length}, 期望 7 (6 实段 + NextBitField)`);
if (nbRows[6].alias !== "NextBitField") die(`第 7 行应为 NextBitField, 实为 ${nbRows[6].alias}`);
if (!txt.includes("using NextBitField = ReservedFields::Next<T, size>;")) die("缺 NextBitField 锚");
nbRows.length = 6;
if (!txt.includes("using LastNodeBaseField = NumDoubleTemporariesNeededField;")) die("缺 LastNodeBaseField 别名锚");
const nbAlias = new Map([["LastNodeBaseField", "NumDoubleTemporariesNeededField"]]);
const nbBits = [];
for (const row of nbRows) {
  const wm = /,\s*(\d+)\s*>$/.exec(row.decl);
  let width;
  if (wm) width = Number(wm[1]);
  else if (/, OpProperties::kSize>$/.test(row.decl)) width = kSize;
  else die(`${row.alias} 宽度不可解析: ${row.decl}`);
  let shift;
  if (row.prev) {
    const p = nbBits.find((b) => b.alias === (nbAlias.get(row.prev) ?? row.prev));
    if (!p) die(`${row.alias} 前驱 ${row.prev} 未在前文`);
    shift = p.shift + p.width;
  } else {
    shift = Number(/,\s*(\d+)\s*,\s*\d+>$/.exec(row.decl)[1]);
  }
  nbBits.push({ alias: row.alias, shift, width });
}
const NB_EXPECT = [
  ["OpcodeField", 0, 16],
  ["InputCountField", 16, 16],
  ["OpPropertiesField", 32, 16],
  ["NumTemporariesNeededField", 48, 2],
  ["NumDoubleTemporariesNeededField", 50, 1],
  ["ReservedFields", 51, 2],
];
nbBits.forEach((b, i) => {
  const [alias, shift, width] = NB_EXPECT[i];
  if (b.alias !== alias || b.shift !== shift || b.width !== width) {
    die(`NodeBase 位段[${i}] = ${JSON.stringify(b)}, 期望 ${JSON.stringify({ alias, shift, width })}`);
  }
});
for (const a of [
  "static_assert(OpcodeField::is_valid(kLastOpcode));",
  "constexpr Opcode opcode() const { return OpcodeField::decode(bitfield_); }",
  "void set_properties(OpProperties properties) {",
  "static constexpr int kMaxInputs = InputCountField::kMax;",
  "constexpr Opcode opcode() const { return OpcodeField::decode(bitfield_); }",
  "  constexpr OpProperties properties() const {\n    return OpPropertiesField::decode(bitfield_);\n  }",
  "bitfield_ = OpPropertiesField::update(bitfield_, properties);",
  "bitfield_ = OpcodeField::update(bitfield_, new_opcode);",
  "bitfield_ = InputCountField::update(bitfield_, input_count() - num);",
  "bitfield_ = NumTemporariesNeededField::update(bitfield_, value);",
  "bitfield_ = NumDoubleTemporariesNeededField::update(bitfield_, value);",
]) {
  if (!txt.includes(a)) die(`缺 NodeBase 锚 ${a.slice(0, 40)}`);
}

// —— 产出 ——
const L = [];
const push = (s = "") => L.push(s);
const shout = (n) => n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
const lower = (n) => n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
push(banner(GEN, SRC,
  `OpProperties: 位域 ${bits.length} + 工厂 ${factorySrc.length}(私有 ${PRIVATE_FACTORIES.length}) + 查询 ${QUERIES.length} + 变换 3; NodeBase 位段 ${nbBits.length}`));
push("//! Maglev IR 节点性质位域纯数据面 (原版 `class OpProperties` + `NodeBase` 位域规格)。");
push("//! 位域: [is_call:1][attached_deopt_info:3][can_throw:1][can_read:1][can_write:1]");
push("//!       [can_allocate:1][not_idempotent:1][value_representation:4][is_conversion:1]");
push(`//!       [needs_register_snapshot:1] → kUsedSize ${kUsedSize}, kSize ${kSize}。`);
push("//! 原版 constexpr 静态工厂 (`OpProperties::CanRead()` 之类) 以关联常量承载, 数值由位域链求出并互校;");
push("//! 查询保留 self 方法形且同名小写, 故与大写工厂常量不冲突。");
push("//! `value_representation` 的语义枚举属 repr 模块, 此处只给原版位宽下的 u8 编码。");
push();
push("/// 原版 `OpProperties::kSize`: 位域总宽 (即 NodeBase properties 段宽度)。");
push(`pub const PROPERTIES_SIZE: u32 = ${kSize};`);
push("/// 原版 `OpProperties::kUsedSize`: 实际占用位数。");
push(`pub const PROPERTIES_USED_SIZE: u32 = ${kUsedSize};`);
push("/// 原版 `kPureMask` = can_read|can_write|can_allocate 三段掩码。");
push(`pub const K_PURE_MASK: u32 = 0x${pureMask.toString(16)};`);
push("/// 原版 `kPureValue` = 三段 encode(false) 之或, 恒 0。");
push(`pub const K_PURE_VALUE: u32 = ${PURE_VALUE};`);
push();
for (const b of bits) {
  const stem = shout(b.alias.replace(/^k/, "").replace(/Bits?$/, ""));
  push(`/// 原版 \`${b.alias}\`: 起始位 ${b.shift}, 宽度 ${b.width}, 类型 ${b.ty}。`);
  push(`pub const ${stem}_SHIFT: u32 = ${b.shift};`);
  push(`pub const ${stem}_WIDTH: u32 = ${b.width};`);
  push(`pub const ${stem}_MASK: u32 = 0x${maskOf(b.alias).toString(16)};`);
}
push();
push("/// 原版私有 `enum class AttachedDeoptInfo` (kNone 起连续分配)。");
push("#[derive(Clone, Copy, Debug, PartialEq, Eq)]");
push("#[repr(u32)]");
push("pub enum AttachedDeoptInfo {");
deoptNames.forEach((n, i) => push(`  ${n.slice(1)} = ${i},`));
push("}");
push();
push("/// 原版 `ValueRepresentation` 在本位域中的编码值 (声明序抄自 maglev-ir.h)。");
vrNames.forEach((n, i) => push(`pub const REPR_${shout(n)}: u8 = ${i};`));
push();
push("/// 节点性质位集, 1:1 对应原版 `class OpProperties` (原版内部 `uint16_t bitfield_`)。");
push("#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]");
push("pub struct OpProperties {");
push("  /// 原版 `bitfield_`; `operator uint32_t()` 即其读取。");
push("  bits: u32,");
push("}");
push();
push("impl OpProperties {");
push("  /// 原版 `explicit OpProperties(uint32_t bitfield)`：成员 `const uint16_t bitfield_`");
push("  /// (maglev-ir.h:1383) 构造即截 16 位 (:1337)。注意 `BitField::encode` 枚举支仅");
push("  /// DCHECK 不掩宽 (base/bit-field.h:84-102)，故 with_new_value_representation 等");
push("  /// update 路径保持不截断，与 C++ release 语义一致；截断只发生在构造入口。");
push("  pub const fn from_bits(bits: u32) -> Self {");
push("    Self { bits: bits & 0xffff }");
push("  }");
push();
push("  /// 原版 `operator uint32_t()`。");
push("  pub const fn to_u32(self) -> u32 {");
push("    self.bits");
push("  }");
push();
push("  /// 原版 `operator|`。");
push("  pub const fn or(self, that: Self) -> Self {");
push("    Self::from_bits(self.bits | that.bits)");
push("  }");
push();
push("  /// 原版 `WithNewValueRepresentation(new_repr)`: `kValueRepresentationBits::update`。");
push("  pub const fn with_new_value_representation(self, new_repr: u8) -> Self {");
push(`    let mask = 0x${maskOf("kValueRepresentationBits").toString(16)};`);
push(`    Self::from_bits((self.bits & !mask) | (new_repr as u32 * ${1 << bit("kValueRepresentationBits").shift}))`);
push("  }");
push();
push("  /// 原版 `WithoutDeopt()`: attached_deopt_info 段清为 kNone。");
push("  pub const fn without_deopt(self) -> Self {");
push(`    Self::from_bits(self.bits & !0x${maskOf("kAttachedDeoptInfoBits").toString(16)})`);
push("  }");
push();
push("  // —— 原版静态工厂 (constexpr 值) ——");
for (const f of factorySrc) {
  if (PRIVATE_FACTORIES.includes(f.name)) {
    push(`  // 原版 private \`OpProperties::${f.name}()\` = ${f.expr};`);
    push("  // 原版仅 DeferredCall() 使用, 故不外露常量, 其值 0x4000 内联进 DEFERRED_CALL。");
    continue;
  }
  if (f.params) {
    push(`  /// 原版 \`OpProperties::${f.name}(${f.params})\`: 表位起 ${bit("kValueRepresentationBits").shift}, 宽 ${bit("kValueRepresentationBits").width}。`);
    push(`  pub const fn ${lower(f.name)}(repr: u8) -> Self {`);
    push(`    Self::from_bits((repr as u32) << ${bit("kValueRepresentationBits").shift})`);
    push("  }");
    continue;
  }
  push(`  /// 原版 \`OpProperties::${f.name}()\` = ${f.expr}。`);
  push(`  pub const ${shout(f.name)}: Self = Self::from_bits(0x${val.get(f.name).toString(16)});`);
}
push();
push("  // —— 原版查询方法 ——");
for (const [rust, alias, ret, kind, anchor] of QUERIES) {
  const b = alias ? bit(alias) : null;
  const raw = b === null ? "" : b.shift === 0 ? "self.bits" : `(self.bits >> ${b.shift})`;
  const field = b === null ? "" : `${raw} & 0x${((1 << b.width) - 1).toString(16)}`;
  const table = {
    bool: `(${field}) != 0`,
    u8: `(${field}) as u8`,
    eager: `matches!(${field}, 1 | 3)`,
    lazy: `matches!(${field}, 2 | 3)`,
    checkpoint: `${field} == 4`,
    and: `(${field}) != 0 && self.can_lazy_deopt()`,
    pure: "(self.bits & K_PURE_MASK) == K_PURE_VALUE",
    req:
      "if self.is_conversion() {\n      self.can_write() || self.can_throw() || self.can_deopt()\n    } else {\n      self.can_write() || self.can_throw() || self.can_deopt() || self.is_any_call()\n    }",
    expr: {
      is_any_call: "self.is_call() || self.is_deferred_call()",
      has_eager_deopt_info: "self.can_eager_deopt() || self.is_deopt_checkpoint()",
      can_deopt: "self.can_eager_deopt() || self.can_lazy_deopt()",
      is_tagged: "self.value_representation() == REPR_TAGGED",
      is_deferred_call: "self.needs_register_snapshot()",
      can_participate_in_cse: "!self.can_write() && !self.not_idempotent()",
    }[rust],
  };
  const emit = table[kind];
  if (!emit) die(`查询 ${rust} 缺发射规则`);
  push(`  /// 原版 \`${norm(anchor).split("{")[0].trim()}\`。`);
  push(`  pub const fn ${rust}(self) -> ${ret} {`);
  push(`    ${emit}`);
  push("  }");
  push();
}
push("}");
push();
push("// —— NodeBase 位段: 原版注释 `[opcode:16][input_count:16][properties:16][extras:16]` ——");
for (const b of nbBits) {
  const stem = shout(b.alias.replace(/Field$/, ""));
  push(`/// 原版 \`${b.alias}\`: 起始位 ${b.shift}, 宽度 ${b.width} (BitField64)。`);
  push(`pub const NODE_${stem}_SHIFT: u32 = ${b.shift};`);
  push(`pub const NODE_${stem}_WIDTH: u32 = ${b.width};`);
  push(`pub const NODE_${stem}_MASK: u64 = 0x${(((1n << BigInt(b.width)) - 1n) << BigInt(b.shift)).toString(16)};`);
}
push();
push("/// 原版节点位域组装: `[opcode:16][input_count:16][properties:16][num_temporaries:2][num_double_temporaries:1]`");
push("/// 各段 `Field::encode(...)` 之或 (原版由构造器/`update` 逐段写入, 此处一次成型)。");
push("pub const fn encode_node_bitfield(");
push("  opcode: u16,");
push("  input_count: u64,");
push("  properties: OpProperties,");
push("  num_temporaries: u64,");
push("  num_double_temporaries: u64,");
push(") -> u64 {");
push("  ((opcode as u64) << NODE_OPCODE_SHIFT)");
push("    | (input_count << NODE_INPUT_COUNT_SHIFT)");
push("    | ((properties.to_u32() as u64) << NODE_OP_PROPERTIES_SHIFT)");
push("    | (num_temporaries << NODE_NUM_TEMPORARIES_NEEDED_SHIFT)");
push("    | (num_double_temporaries << NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_SHIFT)");
push("}");
push();
push("/// 原版 `NodeBase::opcode()`: `OpcodeField::decode(bitfield_)`。");
push("pub const fn decode_opcode(bitfield: u64) -> u16 {");
push("  ((bitfield & NODE_OPCODE_MASK) >> NODE_OPCODE_SHIFT) as u16");
push("}");
push();
push("/// 原版 `NodeBase::input_count()`: `InputCountField::decode(bitfield_)`。");
push("pub const fn decode_input_count(bitfield: u64) -> u64 {");
push("  (bitfield & NODE_INPUT_COUNT_MASK) >> NODE_INPUT_COUNT_SHIFT");
push("}");
push();
push("/// 原版 `NodeBase::properties()`: `OpPropertiesField::decode(bitfield_)`。");
push("pub const fn decode_properties(bitfield: u64) -> OpProperties {");
push("  OpProperties::from_bits(((bitfield & NODE_OP_PROPERTIES_MASK) >> NODE_OP_PROPERTIES_SHIFT) as u32)");
push("}");
push();
push("/// 原版 `NodeBase::NumTemporariesNeededField::decode(bitfield_)`。");
push("pub const fn decode_num_temporaries(bitfield: u64) -> u64 {");
push("  (bitfield & NODE_NUM_TEMPORARIES_NEEDED_MASK) >> NODE_NUM_TEMPORARIES_NEEDED_SHIFT");
push("}");
push();
push("/// 原版 `NodeBase::NumDoubleTemporariesNeededField::decode(bitfield_)`。");
push("pub const fn decode_num_double_temporaries(bitfield: u64) -> u64 {");
push("  (bitfield & NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_MASK) >> NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_SHIFT");
push("}");
push();
push("/// 原版 `NodeBase::set_properties()`: `OpPropertiesField::update(bitfield_, properties)`。");
push("pub const fn update_properties(bitfield: u64, properties: OpProperties) -> u64 {");
push("  (bitfield & !NODE_OP_PROPERTIES_MASK)");
push("    | ((properties.to_u32() as u64) << NODE_OP_PROPERTIES_SHIFT)");
push("}");
push();
push("/// 原版 `NodeBase::kMaxInputs` = `InputCountField::kMax` (16 位全置)。");
push("pub const MAX_INPUTS: u64 = (1 << NODE_INPUT_COUNT_WIDTH) - 1;");
push();
push(`// 对数: 位域 ${bits.length} (位/宽/类型逐项锚) + 工厂 ${factorySrc.length} (原文锚 + 固定点数值求值 + 16 项互校)`);
push(`//   + 无参查询 ${QUERIES.length} (原文反向计数核) + 变换 3 + NodeBase 位段 ${nbBits.length} (前驱链核)`);
push("//   + kPure/NextBitField/LastNodeBaseField/kMaxInputs/encode 组装等原文锚 = 全过。");

writeSrc("v8-maglev-ir", "op_properties.rs", L.join("\n") + "\n");
console.log(
  `OK maglev-ir-props.js: bits=${bits.length} factories=${factorySrc.length} queries=${QUERIES.length} nodebase=${nbBits.length} → v8-maglev-ir/src/op_properties.rs`,
);
