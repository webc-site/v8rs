#!/usr/bin/env bun
// v8/src/compiler/operator.h + operator.cc
// → v8-compiler-graph/src/operator.rs 基线
//
// Operator 基类: opcode/属性位标志/输入输出计数/mnemonic;
// Operator1<T>: 单静态参数模板 (Equals/HashCode 含参数);
// ClassTag (DEBUG 构建) 与 OpParameter<T> 下取不迁移 (Rust 无 unchecked downcast,
// 参数提取改强类型 &Operator1<T> 直传, 见 README 映射)。
// 对数: Property 位值与组合常量 (kFoldable/kEliminatable/kKontrol/kPure) 与原版逐位一致。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/operator.h");
// 族内 .cc 实读对数
const operatorCC = readV8("src/compiler/operator.cc");
if (!operatorCC.includes("Operator::PrintToImpl(")) throw new Error("operator.cc 锚点缺失");

// 抽取 Property 枚举位值: 逐行解析 RHS (数字 / 1<<n / k 名并集)
const propBits = {};
for (const line of h.split("\n")) {
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
// 组合常量逐位核对 (operator.h:50-53)
if (propBits.Foldable !== (propBits.NoRead | propBits.NoWrite)) throw new Error("kFoldable 位不一致");
if (propBits.Eliminatable !== (propBits.NoDeopt | propBits.NoWrite | propBits.NoThrow)) throw new Error("kEliminatable 位不一致");
if (propBits.Kontrol !== (propBits.NoDeopt | propBits.Foldable | propBits.NoThrow)) throw new Error("kKontrol 位不一致");
if (propBits.Pure !== (propBits.Kontrol | propBits.Idempotent)) throw new Error("kPure 位不一致");
// OPERATOR_PROPERTY_LIST (可视izer 打印序, operator.h:57-59)
const plist = h.match(/OPERATOR_PROPERTY_LIST\(V\)\s*\\([\s\S]*?)\n\n/);
if (!plist) throw new Error("OPERATOR_PROPERTY_LIST 未找到");
const printOrder = [...plist[1].matchAll(/V\((\w+)\)/g)].map((m) => m[1]);
if (printOrder.join(",") !== need.join(",")) throw new Error(`打印序 ${printOrder}`);

const pb = (n) => `0x${propBits[n].toString(16)}`;
const constName = (n) => n.replace(/^No/, "NO_").toUpperCase();
const combos = [
  ["FOLDABLE", "Foldable", "kFoldable = kNoRead|kNoWrite"],
  ["ELIMINATABLE", "Eliminatable", "kEliminatable = kNoDeopt|kNoWrite|kNoThrow"],
  ["KONTROL", "Kontrol", "kKontrol = kNoDeopt|kFoldable|kNoThrow (原版拼写如此)"],
  ["PURE", "Pure", "kPure = kKontrol|kIdempotent"],
];

const body = `${banner("src/compiler/operator.js", "src/compiler/operator.h + operator.cc")}
//! Operator — IR 节点"计算"描述 (opcode + 属性位 + 输入输出计数 + 助记符)
//!
//! 不可变、可共享 (静态单例或 zone 分配)。value/effect/control 输入输出计数
//! 是布局数据; 静态参数走 [\`Operator1\`]<T>。
//! Operator1 的 OpParameter<T>/ClassTag 下取不迁移: Rust 无 unchecked downcast,
//! 参数经强类型 \`&Operator1<T>\` 直取 (README 映射登记)。

use v8_base::hashing::hash_combine;

use crate::opcodes::IrOpcode;

/// Operator::Opcode — 全局 opcode 裸值 (IrOpcode 判别值)
pub type Opcode = u16;

/// Operator::Property 位标志 (operator.h:40-54; u8 域, 原版 base::Flags<Property,uint8_t>)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Properties(u8);

/// 单属性位 (与原版 k##name 同值)
#[allow(non_upper_case_globals)]
impl Properties {
${need.map((n) => `  /// k${n}\n  pub const ${constName(n)}: Self = Self(${pb(n)});`).join("\n")}
${combos.map(([rn, cn, d]) => `  /// ${d}\n  pub const ${rn}: Self = Self(${pb(cn)});`).join("\n")}
  /// kNoProperties
  pub const NONE: Self = Self(0);

  /// 位并 (DEFINE_OPERATORS_FOR_FLAGS 的 &)
  pub const fn bits(self) -> u8 {
    self.0
  }
  /// 由裸位构造 (base::Flags 构造)
  pub const fn from_bits(bits: u8) -> Self {
    Self(bits)
  }
  /// 含全部给定位 (HasProperty: (props & p) == p)
  pub const fn contains(self, p: Self) -> bool {
    self.0 & p.0 == p.0
  }
  /// 并集
  pub const fn union(self, p: Self) -> Self {
    Self(self.0 | p.0)
  }
  /// 交集
  pub const fn intersect(self, p: Self) -> Self {
    Self(self.0 & p.0)
  }
  /// Operator::ZeroIfEliminatable — 全 kEliminatable 位则 0 否则 1 (effect 输入数)
  pub const fn zero_if_eliminatable(self) -> u32 {
    if self.contains(Self::ELIMINATABLE) { 0 } else { 1 }
  }
  /// Operator::ZeroIfNoThrow — kNoThrow 则 0 否则 2 (exception 值输出数)
  pub const fn zero_if_no_throw(self) -> u32 {
    if self.contains(Self::NO_THROW) { 0 } else { 2 }
  }
  /// Operator::ZeroIfPure — kPure 则 0 否则 1
  pub const fn zero_if_pure(self) -> u32 {
    if self.contains(Self::PURE) { 0 } else { 1 }
  }
}

impl core::ops::BitOr for Properties {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    self.union(rhs)
  }
}
impl core::ops::BitAnd for Properties {
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self {
    self.intersect(rhs)
  }
}
impl core::ops::BitOrAssign for Properties {
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 |= rhs.0;
  }
}
impl core::ops::Not for Properties {
  type Output = Self;
  fn not(self) -> Self {
    Self(!self.0)
  }
}

/// Operator — 不可变计算描述 (原版 ZoneObject 子类; Rust 侧普通值, 静态或 zone 均可存放)
#[derive(Debug, Clone, Copy)]
pub struct Operator {
  /// mnemonic_ — 助记符 (不含静态参数)
  pub(crate) mnemonic: &'static str,
  /// opcode_ — IrOpcode 判别值
  pub(crate) opcode: Opcode,
  /// properties_ — 属性位
  pub(crate) properties: Properties,
  /// effect_out_ (原版为打包而乱序存储, Rust 不必)
  pub(crate) effect_out: u8,
  /// value_in_
  pub(crate) value_in: u32,
  /// effect_in_
  pub(crate) effect_in: u32,
  /// control_in_
  pub(crate) control_in: u32,
  /// value_out_
  pub(crate) value_out: u32,
  /// control_out_
  pub(crate) control_out: u32,
}

impl Operator {
  /// 原版 Operator(...) 构造 (operator.cc:27; CheckRange 语义 → Err)
  ///
  /// opcode 域校验 [0, IrOpcode::LAST_U16] 为 Rust 侧加强 (原版仅 DCHECK)。
  pub fn new(
    opcode: Opcode,
    properties: Properties,
    mnemonic: &'static str,
    counts: OperatorCounts,
  ) -> Result<Self, crate::Error> {
    if opcode > IrOpcode::LAST_U16 {
      return Err(crate::Error::OpcodeOutOfRange { opcode, last: IrOpcode::LAST_U16 });
    }
    if counts.effect_out > u8::MAX as u32 {
      return Err(crate::Error::CountOutOfRange { what: "effect_out", val: counts.effect_out as u64 });
    }
    Ok(Self {
      mnemonic,
      opcode,
      properties,
      effect_out: counts.effect_out as u8,
      value_in: counts.value_in,
      effect_in: counts.effect_in,
      control_in: counts.control_in,
      value_out: counts.value_out,
      control_out: counts.control_out,
    })
  }

  /// 原版 opcode()
  #[inline]
  pub const fn opcode(&self) -> Opcode {
    self.opcode
  }
  /// 原版 mnemonic()
  #[inline]
  pub const fn mnemonic(&self) -> &'static str {
    self.mnemonic
  }
  /// 原版 properties()
  #[inline]
  pub const fn properties(&self) -> Properties {
    self.properties
  }
  /// 原版 HasProperty
  #[inline]
  pub const fn has_property(&self, p: Properties) -> bool {
    self.properties.contains(p)
  }

  /// 原版 ValueInputCount
  #[inline]
  pub const fn value_input_count(&self) -> u32 {
    self.value_in
  }
  /// 原版 EffectInputCount
  #[inline]
  pub const fn effect_input_count(&self) -> u32 {
    self.effect_in
  }
  /// 原版 ControlInputCount
  #[inline]
  pub const fn control_input_count(&self) -> u32 {
    self.control_in
  }
  /// 原版 ValueOutputCount
  #[inline]
  pub const fn value_output_count(&self) -> u32 {
    self.value_out
  }
  /// 原版 EffectOutputCount
  #[inline]
  pub const fn effect_output_count(&self) -> u32 {
    self.effect_out as u32
  }
  /// 原版 ControlOutputCount
  #[inline]
  pub const fn control_output_count(&self) -> u32 {
    self.control_out
  }

  /// 原版 Equals 虚函数基类实现 — 仅比 opcode; 带参数语义见 Operator1::equals
  pub fn equals(&self, that: &Operator) -> bool {
    self.opcode == that.opcode
  }

  /// 原版 HashCode 虚函数基类实现 — hash(opcode)
  pub fn hash_code(&self) -> usize {
    hash_combine(self.opcode as usize, self.opcode as usize)
  }

  /// 原版 PrintPropsTo — 属性名逗号串 (OPERATOR_PROPERTY_LIST 序)
  pub fn props_str(&self) -> String {
    const NAMES: [&str; ${need.length}] = [${need.map((n) => `"${n}"`).join(", ")}];
    let bits = [${need.map((n) => "Properties::" + constName(n)).join(", ")}];
    let mut out = String::new();
    for (name, b) in NAMES.iter().zip(bits) {
      if self.has_property(b) {
        if !out.is_empty() {
          out.push_str(", ");
        }
        out.push_str(name);
      }
    }
    out
  }
}

/// 操作符输入输出端口计数
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OperatorCounts {
  /// value_in_
  pub value_in: u32,
  /// effect_in_
  pub effect_in: u32,
  /// control_in_
  pub control_in: u32,
  /// value_out_
  pub value_out: u32,
  /// effect_out_
  pub effect_out: u32,
  /// control_out_
  pub control_out: u32,
}

impl OperatorCounts {
  /// 位置参数构造 (与原版构造参数序一致)
  pub const fn new(
    value_in: u32,
    effect_in: u32,
    control_in: u32,
    value_out: u32,
    effect_out: u32,
    control_out: u32,
  ) -> Self {
    Self {
      value_in,
      effect_in,
      control_in,
      value_out,
      effect_out,
      control_out,
    }
  }
}

/// Operator1<T> — 单静态参数模板 (operator.h:179-235)
///
/// 原版经虚表 Equals/HashCode 覆写 + ClassTag 下取; Rust 以组合代替继承,
/// Node 持 \`&op1.base\` 即可, 参数侧走强类型引用。
#[derive(Debug, Clone, Copy)]
pub struct Operator1<T> {
  /// 基类字段
  pub base: Operator,
  /// parameter_ — 静态参数 (不可变)
  pub parameter: T,
}

impl<T: PartialEq> Operator1<T> {
  /// 原版 Operator1 构造 (基类构造 + 参数)
  pub fn new(
    opcode: Opcode,
    properties: Properties,
    mnemonic: &'static str,
    counts: OperatorCounts,
    parameter: T,
  ) -> Result<Self, crate::Error> {
    Ok(Self { base: Operator::new(opcode, properties, mnemonic, counts)?, parameter })
  }

  /// 原版 parameter()
  #[inline]
  pub const fn parameter(&self) -> &T {
    &self.parameter
  }

  /// 原版 Equals 覆写 — opcode 相等且参数相等
  /// (float/double 的位级比较由参数类型 PartialEq 自带语义承担, 原版特化不单列)
  pub fn equals(&self, other: &Operator1<T>) -> bool {
    self.base.opcode == other.base.opcode && self.parameter == other.parameter
  }
}

impl<T: PartialEq + core::hash::Hash> Operator1<T> {
  /// 原版 HashCode 覆写 — hash_combine(opcode, hash(parameter))
  pub fn hash_code(&self) -> usize {
    let mut h = std::hash::DefaultHasher::new();
    self.parameter.hash(&mut h);
    hash_combine(self.base.opcode as usize, core::hash::Hasher::finish(&h) as usize)
  }
}

// —— 生成不变式 (属性位与原版逐位一致) ——

const _: () = {
  assert!(Properties::COMMUTATIVE.bits() == ${pb("Commutative")});
  assert!(Properties::ASSOCIATIVE.bits() == ${pb("Associative")});
  assert!(Properties::IDEMPOTENT.bits() == ${pb("Idempotent")});
  assert!(Properties::NO_READ.bits() == ${pb("NoRead")});
  assert!(Properties::NO_WRITE.bits() == ${pb("NoWrite")});
  assert!(Properties::NO_THROW.bits() == ${pb("NoThrow")});
  assert!(Properties::NO_DEOPT.bits() == ${pb("NoDeopt")});
  assert!(Properties::FOLDABLE.bits() == Properties::NO_READ.union(Properties::NO_WRITE).bits());
  assert!(Properties::ELIMINATABLE.bits() == Properties::NO_DEOPT.union(Properties::NO_WRITE).union(Properties::NO_THROW).bits());
  assert!(Properties::KONTROL.bits() == Properties::NO_DEOPT.union(Properties::FOLDABLE).union(Properties::NO_THROW).bits());
  assert!(Properties::PURE.bits() == Properties::KONTROL.union(Properties::IDEMPOTENT).bits());
  assert!(Properties::PURE.contains(Properties::FOLDABLE));
  assert!(!Properties::FOLDABLE.contains(Properties::COMMUTATIVE));
};
`;

const p = writeSrc("v8-compiler-graph", "operator.rs", body);
console.log(`written ${p} (props ${need.length} + combos ${combos.length}, print-order ok)`);
