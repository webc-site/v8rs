#!/usr/bin/env bun
// tool/src/compiler/turboshaft/types.h.js
// 机械提取 v8/src/compiler/turboshaft/types.{h,cc}
// → v8-turboshaft/src/types.rs
//
// 提取面: Type::Kind / Type::ResolutionMode / WordType::SubKind /
// FloatType::SubKind / FloatType::Special 位标志 + 常量表
// (kMaxInlineSetSize/kMaxSetSize) + Payload 结构 + 集合分派表 (types.cc
// LeastUpperBound/AllocateOnHeap 的 Kind→子类型 case 表, 登记为骨架)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/types.h");
const cc = readV8("src/compiler/turboshaft/types.cc");

function strip(s) {
  return s.replace(/\/\/[^\n]*/g, "");
}

// ---------------------------------------------------------------------------
// 1. 枚举提取
// ---------------------------------------------------------------------------
function enumOf(re, label, expect) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  const names = [...strip(m[1]).matchAll(/\bk(\w+)\b/g)].map((x) => x[1]);
  if (expect && names.length !== expect) throw new Error(`${label} ${names.length} != ${expect}`);
  return names;
}

const typeKinds = enumOf(/enum class Kind : uint8_t\s*\{([^}]+)\};/, "Type::Kind", 8);
const resolutionModes = enumOf(/enum class ResolutionMode\s*\{([\s\S]*?)\};/, "Type::ResolutionMode", 3);
const wordSubKinds = enumOf(/class WordType : public Type\s*\{[\s\S]{0,400}?enum class SubKind : uint8_t\s*\{([^}]+)\};/, "WordType::SubKind", 2);
const floatSubKinds = enumOf(/class FloatType : public Type\s*\{[\s\S]{0,400}?enum class SubKind : uint8_t\s*\{([^}]+)\};/, "FloatType::SubKind", 3);

// FloatType::Special 位标志 (enum Special : uint32_t, 显式值)
const specialM = h.match(/enum Special : uint32_t\s*\{([^}]+)\}/);
if (!specialM) throw new Error("未找到 FloatType::Special");
const specialFlags = [...strip(specialM[1]).matchAll(/k(\w+) = (0x[0-9a-fA-F]+)/g)].map((m) => ({
  name: m[1],
  value: m[2],
}));
if (specialFlags.length !== 3) throw new Error(`Special 位标志 ${specialFlags.length} != 3`);
for (const [i, expect] of ["0x0", "0x1", "0x2"].entries()) {
  if (specialFlags[i].value.toLowerCase() !== expect) {
    throw new Error(`Special[${i}]=${specialFlags[i].value} 预期 ${expect}`);
  }
}

// ---------------------------------------------------------------------------
// 2. 常量
// ---------------------------------------------------------------------------
function numConst(re, label, expect) {
  const m = h.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  if (expect !== undefined && Number(m[1]) !== expect) throw new Error(`${label}=${m[1]} 预期 ${expect}`);
  return Number(m[1]);
}
const wordMaxInlineSetSize = numConst(/class WordType : public Type\s*\{[\s\S]{0,200}?static constexpr int kMaxInlineSetSize = (\d+);/, "WordType::kMaxInlineSetSize", 2);
const wordMaxSetSize = numConst(/class WordType : public Type[\s\S]{0,800}?static constexpr int kMaxSetSize = (\d+);/, "WordType::kMaxSetSize", 8);
const floatMaxInlineSetSize = numConst(/class FloatType : public Type\s*\{[\s\S]{0,200}?static constexpr int kMaxInlineSetSize = (\d+);/, "FloatType::kMaxInlineSetSize", 2);
const floatMaxSetSize = numConst(/class FloatType : public Type[\s\S]{0,800}?static constexpr int kMaxSetSize = (\d+);/, "FloatType::kMaxSetSize", 8);

// Word/Float → Kind 映射 (KIND = Bits == 32 ? kWord32 : kWord64)
const kindMapM = h.match(/static constexpr Kind KIND = Bits == 32 \? Kind::k(\w+) : Kind::k(\w+);/);
if (!kindMapM) throw new Error("未找到 WordType::KIND 映射");
const floatKindM = [...h.matchAll(/static constexpr Kind KIND = Bits == (\d+) \? Kind::k(\w+) : Kind::k(\w+);/g)];
if (floatKindM.length < 2) throw new Error(`KIND 映射 ${floatKindM.length} < 2`);

// ---------------------------------------------------------------------------
// 3. types.cc 分派骨架 (Kind → 子类型方法 case 表, 登记面)
// ---------------------------------------------------------------------------
function dispatchTable(ccText, anchor, label) {
  const seg = ccText.slice(ccText.indexOf(anchor));
  const fnM = seg.match(/\{([\s\S]*?)\n\}/);
  if (!fnM) throw new Error(`未找到 ${label} 函数体`);
  const rows = [];
  for (const m of fnM[1].matchAll(/case (?:Type::)?Kind::k(\w+):/g)) {
    if (!rows.includes(m[1])) rows.push(m[1]);
  }
  return rows;
}
const lubDispatch = dispatchTable(cc, "Type Type::LeastUpperBound", "LeastUpperBound");
const heapDispatch = dispatchTable(cc, "Handle<TurboshaftType> Type::AllocateOnHeap", "AllocateOnHeap");
if (lubDispatch.length !== 8) throw new Error(`LUB 分派 ${lubDispatch.length} != 8 (全 Kind 覆盖)`);
if (heapDispatch.length < 5) throw new Error(`堆分配分派 ${heapDispatch.length} < 5`);

// ---------------------------------------------------------------------------
// 4. 生成 Rust
// ---------------------------------------------------------------------------
let rust = banner(
  "src/compiler/turboshaft/types.h.js",
  "src/compiler/turboshaft/types.{h,cc}",
  "Turboshaft 类型体系: Type::Kind/SubKind/Special 位标志与常量",
);

rust += `
/// 原版 Type::Kind — 类型大类
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TypeKind {
`;
typeKinds.forEach((n, i) => {
  rust += `  /// k${n}\n  ${n} = ${i},\n`;
});
rust += `}

/// 原版 Type::ResolutionMode — 类型运算不可精表达时的策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ResolutionMode {
`;
resolutionModes.forEach((n, i) => {
  rust += `  /// k${n}\n  ${n} = ${i},\n`;
});
rust += `}

/// 原版 WordType::SubKind — 字类型形态 (区间/集合)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum WordSubKind {
`;
wordSubKinds.forEach((n, i) => {
  rust += `  /// k${n}\n  ${n} = ${i},\n`;
});
rust += `}

/// 原版 FloatType::SubKind — 浮点类型形态
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FloatSubKind {
`;
floatSubKinds.forEach((n, i) => {
  rust += `  /// k${n}\n  ${n} = ${i},\n`;
});
rust += `}

/// 原版 FloatType::Special — 特殊浮点值位标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Special {
`;
specialFlags.forEach((f) => {
  rust += `  /// k${f.name} = ${f.value}\n  ${f.name} = ${f.value},\n`;
});
rust += `}

/// 原版 kMaxInlineSetSize (Word/Float 同值, 集合内联容量)
pub const MAX_INLINE_SET_SIZE: usize = ${wordMaxInlineSetSize};

/// 原版 kMaxSetSize (集合最大元素数; 超出走 Payload_OutlineSet 外联)
pub const MAX_SET_SIZE: usize = ${wordMaxSetSize};

/// 原版 Payload 结构面 (detail 命名空间):
/// - Payload_Empty { u8 } — 无负载 (Invalid/None/Any/Tuple)
/// - Payload_Range<T> { min, max }
/// - Payload_InlineSet<T> { T elements[2] }
/// - Payload_OutlineSet<T> { T* array }
/// Rust 侧以枚举 Payload 表达:
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Payload<T: Copy> {
  Empty,
  /// 原版 Payload_Range
  Range { min: T, max: T },
  /// 原版 Payload_InlineSet (≤${wordMaxInlineSetSize} 元素内联)
  InlineSet([T; ${wordMaxInlineSetSize}]),
  /// 原版 Payload_OutlineSet (外联数组; Rust 侧以引用承载, Developer 修订)
  OutlineSet,
}

/// 原版 WordType<Bits>::KIND 映射 (32 → Word32, 64 → Word64)
#[must_use]
pub const fn word_kind(bits: u8) -> TypeKind {
  if bits == ${floatKindM[0][1]} { TypeKind::${floatKindM[0][2]} } else { TypeKind::${floatKindM[0][3]} }
}

/// 原版 FloatType<Bits>::KIND 映射 (32 → Float32, 64 → Float64)
#[must_use]
pub const fn float_kind(bits: u8) -> TypeKind {
  if bits == ${floatKindM[1][1]} { TypeKind::${floatKindM[1][2]} } else { TypeKind::${floatKindM[1][3]} }
}

/// 原版 next_smaller<T> (浮点: nextafter 向 -inf; 整型: v-1)
pub trait NextValue: Copy {
  /// 原版 next_smaller
  fn next_smaller(self) -> Self;
  /// 原版 next_larger
  fn next_larger(self) -> Self;
}

/// v8: src/compiler/turboshaft/types.cc Type::LeastUpperBound
///
/// 原版 Type::LeastUpperBound 分派面 (types.cc):
/// - kInvalid/kNone/kAny → UNREACHABLE
${lubDispatch.map((k) => `/// - k${k} → ${k}Type::LeastUpperBound`).join("\n")}
///
/// 原版 Type::AllocateOnHeap 分派面 (types.cc):
${heapDispatch.map((k) => `/// - k${k}`).join("\n")}
pub fn least_upper_bound() {
  todo!()
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(TypeKind::Invalid as u8 == 0);
  assert!(TypeKind::Any as u8 == 7);
  assert!(WordSubKind::Range as u8 == 0);
  assert!(WordSubKind::Set as u8 == 1);
  assert!(FloatSubKind::OnlySpecialValues as u8 == 2);
  assert!(Special::NoSpecialValues as u32 == 0x0);
  assert!(Special::NaN as u32 == 0x1);
  assert!(Special::MinusZero as u32 == 0x2);
  assert!(MAX_INLINE_SET_SIZE == 2);
  assert!(MAX_SET_SIZE == 8);
  assert!(matches!(word_kind(32), TypeKind::Word32));
  assert!(matches!(word_kind(64), TypeKind::Word64));
  assert!(matches!(float_kind(32), TypeKind::Float32));
  assert!(matches!(float_kind(64), TypeKind::Float64));
};
`;

const p = writeSrc("v8-turboshaft", "types.rs", rust);
console.log(`written ${p}`);
console.log(`TypeKind=${typeKinds.length} ResMode=${resolutionModes.length} WordSub=${wordSubKinds.length} FloatSub=${floatSubKinds.length} Special=${specialFlags.length} 常量=4 分派=${lubDispatch.length}/${heapDispatch.length}`);
