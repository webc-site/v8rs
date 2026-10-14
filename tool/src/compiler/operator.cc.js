#!/usr/bin/env bun
// tool/src/compiler/operator.cc.js
// 机械提取 v8/src/compiler/operator.cc (+ operator.h):
// Operator 字段布局 / CheckRange 收窄规则 / 属性打印序 (OPERATOR_PROPERTY_LIST)
// 产物为 v8-compiler-passes 侧基线; Properties 位面正典在
// v8-compiler-graph/src/operator.rs (operator.js), 挂接时由 Developer 收敛

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/compiler/operator.h");
const ccSrc = readV8("src/compiler/operator.cc");

// —— op 域类型锚 (operator.h:41) ——
if (!hSrc.includes("using Opcode = uint16_t;")) throw new Error("Opcode = uint16_t 锚缺失");

// —— 字段布局 (operator.h:150-160 私有段) ——
const priv = hSrc.match(/ private:\s*(const char\* mnemonic_;[\s\S]*?)\n\};/);
if (!priv) throw new Error("Operator 私有字段段缺失");
const fieldLines = priv[1].split("\n").map((l) => l.trim()).filter(Boolean);
const fields = fieldLines.map((l) => {
  const m = l.match(/^([\w:<>\s*]+?)\s+(\w+);/);
  if (!m) throw new Error(`字段行不可解析: ${l}`);
  return { type: m[1].replace(/\s+/g, " ").trim(), name: m[2] };
});
const FIELD_EXPECT = [
  "const char*:mnemonic_",
  "Opcode:opcode_",
  "Properties:properties_",
  "uint8_t:effect_out_",
  "uint32_t:value_in_",
  "uint32_t:effect_in_",
  "uint32_t:control_in_",
  "uint32_t:value_out_",
  "uint32_t:control_out_",
];
if (fields.map((f) => `${f.type}:${f.name}`).join("|") !== FIELD_EXPECT.join("|")) {
  throw new Error(`字段布局漂移: ${fields.map((f) => `${f.type} ${f.name}`).join("; ")}`);
}
// effect_out_ 位打包注释锚 (h:155)
if (!/uint8_t effect_out_;.*Stored out of order for better field packing/.test(hSrc.replace(/\n\s*/g, " ").replace(/\/\/\s*/g, "// ").replace(/ {2,}/g, " ")) && !hSrc.includes("Stored out of order for better field packing")) {
  throw new Error("effect_out_ 打包注释锚缺失");
}

// —— ctor 初始化序 (cc:27-38): 声明序执行, effect_out_{...} 第 4 项 ——
const ctor = ccSrc.slice(ccSrc.indexOf("Operator::Operator(Opcode opcode"), ccSrc.indexOf("std::ostream& operator<<", ccSrc.indexOf("Operator::Operator(")));
const initNames = [...ctor.matchAll(/^\s*:?\s*(\w+)[({]/gm)].map((m) => m[1]).filter((n) => fields.some((f) => f.name === n));
if (initNames.join("|") !== "mnemonic_|opcode_|properties_|effect_out_|value_in_|effect_in_|control_in_|value_out_|control_out_") {
  throw new Error(`ctor 初始化序漂移: ${initNames.join("|")}`);
}
for (const [field, ty] of [["effect_out", "uint8_t"], ["value_in", "uint32_t"], ["effect_in", "uint32_t"], ["control_in", "uint32_t"], ["value_out", "uint32_t"], ["control_out", "uint32_t"]]) {
  if (!ctor.includes(`${field}_{CheckRange<${ty}>(`)) throw new Error(`CheckRange<${ty}> 锚缺失 (${field})`);
}

// —— CheckRange 规则锚 (cc:15-23) ——
const cr = ccSrc.slice(ccSrc.indexOf("V8_INLINE N CheckRange(size_t val)"), ccSrc.indexOf("}  // namespace", ccSrc.indexOf("V8_INLINE N CheckRange")));
if (!/CHECK_LE\(val, std::min\(static_cast<size_t>\(std::numeric_limits<N>::max\(\)\),\s*static_cast<size_t>\(kMaxInt\)\)\)/.test(cr)) {
  throw new Error("CheckRange 上界式缺失");
}
if (!/return static_cast<N>\(val\);/.test(cr)) throw new Error("CheckRange 收窄式缺失");
if (!hSrc.includes("enum class PrintVerbosity { kVerbose, kSilent };")) throw new Error("PrintVerbosity 缺失");

// —— Property 位值解析 (operator.h:44-54) ——
const propBits = {};
for (const line of hSrc.split("\n")) {
  const m = line.match(/^\s*k(\w+)\s*=\s*([^,]+)/);
  if (!m) continue;
  const name = m[1];
  if (name === "NoProperties") {
    propBits[name] = 0;
    continue;
  }
  const rhs = m[2].trim();
  const val = rhs.split("|").reduce((acc, term) => {
    const t = term.trim();
    const sh = t.match(/^1 << (\d+)$/);
    if (sh) return acc | (1 << Number(sh[1]));
    const kn = t.match(/^k(\w+)$/);
    if (kn) {
      if (propBits[kn[1]] === undefined) throw new Error(`前向引用 k${kn[1]}`);
      return acc | propBits[kn[1]];
    }
    const num = t.match(/^\d+$/);
    if (num) return acc | Number(t);
    throw new Error(`未识别 RHS '${t}' (k${name})`);
  }, 0);
  propBits[name] = val;
}
const need = ["Commutative", "Associative", "Idempotent", "NoRead", "NoWrite", "NoThrow", "NoDeopt"];
for (const n of need) if (propBits[n] === undefined) throw new Error(`Property ${n} 缺失`);
if (propBits.Foldable !== (propBits.NoRead | propBits.NoWrite)) throw new Error("kFoldable 位不一致");
if (propBits.Eliminatable !== (propBits.NoDeopt | propBits.NoWrite | propBits.NoThrow)) throw new Error("kEliminatable 位不一致");
if (propBits.Kontrol !== (propBits.NoDeopt | propBits.Foldable | propBits.NoThrow)) throw new Error("kKontrol 位不一致");
if (propBits.Pure !== (propBits.Kontrol | propBits.Idempotent)) throw new Error("kPure 位不一致");

// —— OPERATOR_PROPERTY_LIST 打印序 (operator.h:57-59) ——
const plist = hSrc.match(/#define OPERATOR_PROPERTY_LIST\(V\) \\([\s\S]*?)\n\n/);
if (!plist) throw new Error("OPERATOR_PROPERTY_LIST 未找到");
const printOrder = [...plist[1].matchAll(/V\((\w+)\)/g)].map((m) => m[1]);
if (printOrder.join(",") !== need.join(",")) throw new Error(`打印序 ${printOrder}`);

// —— PrintPropsTo 分隔规则锚 (cc:49-60) ——
const ppt = ccSrc.slice(ccSrc.indexOf("void Operator::PrintPropsTo"), ccSrc.indexOf("}  // namespace", ccSrc.indexOf("void Operator::PrintPropsTo")));
if (!ppt.includes('separator = ", ";')) throw new Error("PrintPropsTo 分隔符锚缺失");
if (!ppt.includes("OPERATOR_PROPERTY_LIST(PRINT_PROP_IF_SET)")) throw new Error("PrintPropsTo 宏展开锚缺失");
if (!ppt.includes("HasProperty(Operator::k##name)")) throw new Error("PRINT_PROP_IF_SET 谓词锚缺失");

// —— operator.cc 定义清单 ——
const defs = [
  ["Operator::Operator(Opcode opcode, Properties properties, const char* mnemonic, size_t value_in, size_t effect_in, size_t control_in, size_t value_out, size_t effect_out, size_t control_out)", 27],
  ["std::ostream& operator<<(std::ostream& os, const Operator& op)", 40],
  ["void Operator::PrintToImpl(std::ostream& os, PrintVerbosity verbose) const", 45],
  ["void Operator::PrintPropsTo(std::ostream& os) const", 49],
];
const ccFlat = ccSrc.replace(/\s+/g, " ").replace(/\(\s+/g, "(").replace(/\s+\)/g, ")");
for (const [sig] of defs) {
  if (!ccFlat.includes(sig)) throw new Error(`定义缺失: ${sig}`);
}
if (!ccSrc.includes("os << mnemonic();")) throw new Error("PrintToImpl 助记符直写锚缺失");

const pb = (n) => `0x${propBits[n].toString(16)}`;

console.log(`operator.cc: 字段 ${fields.length}, 属性 7 位 + 组合 4, 打印序 ${printOrder.length}, 定义 ${defs.length}`);

let rust = banner(
  "src/compiler/operator.cc.js",
  "src/compiler/operator.cc + operator.h",
  "Operator 布局/CheckRange/属性打印序; 位面正典在 v8-compiler-graph/src/operator.rs",
);

rust += `
//! Operator 实现面基线 — operator.cc 的机械规则。
//!
//! - 字段序 (声明序 = ctor 初始化序): mnemonic_/opcode_/properties_ 先行,
//!   effect_out_ (u8) 提前到 u32 组之前做位打包 (h:155 注释), 其余 5 计数 u32 收尾
//! - CheckRange<N>: CHECK 上界 = min(N::MAX, kMaxInt) 后收窄 (getter 仍返 int 的过渡约束)
//! - PrintPropsTo: 按 OPERATOR_PROPERTY_LIST 固定序打印已置位名, ", " 分隔

/// 操作码裸值域 (Operator::Opcode = uint16_t)
pub type Opcode = u16;

/// 打印冗长度 (operator.h:61 PrintVerbosity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrintVerbosity {
  /// kVerbose
  VERBOSE = 0,
  /// kSilent
  SILENT = 1,
}

/// kMaxInt — std::numeric_limits<int>::max() (CheckRange 的公共上界分量)
pub const K_MAX_INT: u64 = 2_147_483_647;

/// CheckRange<uint8_t> 的 1:1 转写 — 上界 = min(255, kMaxInt) = 255
/// v8: src/compiler/operator.cc:16 CheckRange (effect_out_)
#[must_use]
pub const fn check_range_u8(val: u64) -> u8 {
  assert!(val <= u8::MAX as u64);
  val as u8
}

/// CheckRange<uint32_t> 的 1:1 转写 — 上界 = min(u32::MAX, kMaxInt) = kMaxInt
/// v8: src/compiler/operator.cc:16 CheckRange (value/effect/control 计数)
#[must_use]
pub const fn check_range_u32(val: u64) -> u32 {
  assert!(val <= K_MAX_INT);
  val as u32
}

// —— 字段布局台账 (类型面由 Developer 落型; 序与打包意图照原版) ——
// h:152  const char* mnemonic_;
// h:153  Opcode opcode_;                       // uint16_t
// h:154  Properties properties_;               // base::Flags<Property, uint8_t>
// h:155  uint8_t  effect_out_;                 // 声明序提前, 字段打包 (原注释)
// h:156  uint32_t value_in_;
// h:157  uint32_t effect_in_;
// h:158  uint32_t control_in_;
// h:159  uint32_t value_out_;
// h:160  uint32_t control_out_;
// ctor (cc:27-38) 初始化序与声明序一致; 全部计数经 CheckRange 收窄

/// 属性位打印序 — OPERATOR_PROPERTY_LIST (可视化器固定序, operator.h:57-59)
pub const PROPERTY_PRINT_ORDER: [&str; ${printOrder.length}] = [${printOrder.map((n) => `"${n}"`).join(", ")}];

/// 打印序对应的属性位 (k 名 → 位值; 与 operator.h:44-50 逐位一致)
pub const PROPERTY_PRINT_BITS: [(&str, u8); ${printOrder.length}] = [
${printOrder.map((n) => `  ("${n}", ${pb(n)}),`).join("\n")}
];

/// PrintPropsTo 的 1:1 转写 — 已置位属性名按固定序拼接, ", " 分隔
/// (原版 PRINT_PROP_IF_SET 宏展开: HasProperty(k##name) 真则打印 #name)
/// v8: src/compiler/operator.cc:49 Operator::PrintPropsTo
#[must_use]
pub fn print_props_to(properties: u8) -> String {
  let mut out = String::new();
  let mut separator = "";
  for (name, bit) in PROPERTY_PRINT_BITS {
    if properties & bit == bit {
      out.push_str(separator);
      out.push_str(name);
      separator = ", ";
    }
  }
  out
}

/// kFoldable = kNoRead|kNoWrite (组合位, 对数锚)
pub const K_FOLDABLE: u8 = ${pb("Foldable")};
/// kEliminatable = kNoDeopt|kNoWrite|kNoThrow
pub const K_ELIMINATABLE: u8 = ${pb("Eliminatable")};
/// kKontrol = kNoDeopt|kFoldable|kNoThrow (原版拼写如此)
pub const K_KONTROL: u8 = ${pb("Kontrol")};
/// kPure = kKontrol|kIdempotent
pub const K_PURE: u8 = ${pb("Pure")};

// —— 方法签名骨架 (语义实现由 Developer 就地修订) ——
/// 原版签名: ${defs[0][0]}
/// v8: src/compiler/operator.cc:${defs[0][1]}
pub fn operator_new() {
  todo!("就地修订: 九参构造, 计数经 CheckRange 收窄")
}

/// 原版签名: ${defs[1][0]}
/// v8: src/compiler/operator.cc:${defs[1][1]}
pub fn operator_stream_out() {
  todo!("就地修订: 转发 PrintTo")
}

/// 原版签名: ${defs[2][0]}
/// v8: src/compiler/operator.cc:${defs[2][1]}
pub fn print_to_impl() {
  todo!("就地修订: 基类直写 mnemonic, 子类覆写带参打印")
}

/// 原版签名: ${defs[3][0]}
/// v8: src/compiler/operator.cc:${defs[3][1]}
pub fn print_props_to_skel() {
  todo!("就地修订: 挂接 print_props_to 纯函数")
}

// —— 提取不变式断言 (编译期可求值面) ——
const _: () = {
  assert!(K_MAX_INT == 2_147_483_647);
  assert!(check_range_u8(0) == 0);
  assert!(check_range_u8(255) == 255);
  assert!(check_range_u32(2_147_483_647) == 2_147_483_647);
  assert!(PROPERTY_PRINT_ORDER.len() == 7);
  assert!(K_FOLDABLE == 0x18);
  assert!(K_ELIMINATABLE == 0x70);
  assert!(K_KONTROL == 0x78);
  assert!(K_PURE == 0x7c);
};

// —— 行为面对数 (String/str 非 const, 挂接后随 crate 测试跑; 暂以标准 #[test] 占位,
//    挂接时由 Developer 改写为仓库规范测试样式) ——
#[cfg(test)]
mod generated_checks {
  use super::{print_props_to, PROPERTY_PRINT_ORDER};

  #[test]
  fn print_props_separator_rule() {
    assert_eq!(PROPERTY_PRINT_ORDER[0], "Commutative");
    assert_eq!(PROPERTY_PRINT_ORDER[6], "NoDeopt");
    assert_eq!(print_props_to(0), "");
    assert_eq!(print_props_to(0x01), "Commutative");
    assert_eq!(print_props_to(0x18), "NoRead, NoWrite");
    assert_eq!(
      print_props_to(0x7f),
      "Commutative, Associative, Idempotent, NoRead, NoWrite, NoThrow, NoDeopt"
    );
  }
}
`;

const p = writeSrc("v8-compiler-passes", "operator_cc.rs.skel", rust);
console.log(`written ${p}`);
