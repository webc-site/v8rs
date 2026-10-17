#!/usr/bin/env bun
// tool/src/compiler/turboshaft/index.h.js
// 机械提取 v8/src/compiler/turboshaft/index.h
// → v8-turboshaft/src/index.rs
//
// 提取面: OperationStorageSlot/kSlotsPerId 常量; OpIndex/BlockIndex 索引类型
// (Invalid/valid/id/hash/offset/外部 id 编码/全序比较); OptionalOpIndex;
// 抽象类型标签族 (Any/None/Word32/.../EagerFrameState) 与 v_traits rep 映射表;
// 世代掩码常量 (DEBUG 世代位, 仅注释登记 — release 布局不含)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/compiler/turboshaft/index.h");

// ---------------------------------------------------------------------------
// 1. 常量机械提取
// ---------------------------------------------------------------------------
function constexprNum(re, label, expect) {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  return { label, value: m[1], expect };
}

const constBinds = [
  constexprNum(/constexpr size_t kSlotsPerId = (\d+);/, "kSlotsPerId", "2"),
  constexprNum(/static constexpr uint32_t kGenerationMaskShift = (\d+);/, "kGenerationMaskShift", "1"),
  constexprNum(/static constexpr uint32_t kExternalIdFlag = (\d+);/, "kExternalIdFlag", "1"),
];
for (const b of constBinds) {
  if (b.expect && b.value !== b.expect) throw new Error(`${b.label}=${b.value} 预期 ${b.expect}`);
}
const slotsPerId = Number(constBinds[0].value);

// ---------------------------------------------------------------------------
// 2. 抽象类型标签 + v_traits rep 映射表
// ---------------------------------------------------------------------------
// struct X : public Any {} / using X = WordWithBits<N> / struct X : public InternalTag {}
const tagDecls = [];
const aliasRe = /using (\w+) = WordWithBits<(\d+)>;|using (\w+) = FloatWithBits<(\d+)>;|using WordPtr = Word64;/g;
let am;
while ((am = aliasRe.exec(src)) !== null) {
  if (am[0].startsWith("using WordPtr")) {
    tagDecls.push({ name: "WordPtr", kind: "word64alias" });
  } else if (am[1]) {
    tagDecls.push({ name: am[1], kind: `word${am[2]}` });
  } else {
    tagDecls.push({ name: am[3], kind: `float${am[4]}` });
  }
}
const structRe = /struct (\w+) : public (Any|InternalTag)\s*\{\}/g;
let sm;
while ((sm = structRe.exec(src)) !== null) {
  tagDecls.push({ name: sm[1], kind: sm[2] === "Any" ? "any" : "internal" });
}
// 对数: 原文标签面 (Word32/Word64/WordPtr/Float32/Float64/Simd128/Simd256 + Compressed/
// InternalTag/EagerFrameState/LazyFrameState + Any/None)
const tagNames = tagDecls.map((t) => t.name);
for (const expect of ["Word32", "Word64", "WordPtr", "Float32", "Float64", "Simd128", "Simd256", "Compressed", "InternalTag", "EagerFrameState", "LazyFrameState"]) {
  if (!tagNames.includes(expect)) throw new Error(`缺标签 ${expect}`);
}

// v_traits<X> 特化 → rep 映射
const traitsRe = /struct v_traits<(\w+)(?:,\s*std::enable_if_t<[^>]*>)?>\s*\{/g;
const repMap = [];
let tm;
const seen = new Set();
while ((tm = traitsRe.exec(src)) !== null) {
  const name = tm[1];
  if (seen.has(name)) continue; // 前向/特化链只取首现
  seen.add(name);
  const tail = src.slice(tm.index, tm.index + 1200);
  const rm = tail.match(/static constexpr auto rep = ([^;]+);/);
  if (!rm) continue;
  repMap.push({ name, rep: rm[1].trim() });
}
const repEntries = repMap.length;
if (repEntries < 8) throw new Error(`v_traits rep 映射仅 ${repEntries} 条, 预期 >=8`);

function rustRep(repText) {
  if (repText === "nullrep") return "None";
  let m = repText.match(/RegisterRepresentation::(\w+)\(\)/);
  if (m) return `Some(RegisterRepresentation::${m[1]})`;
  m = repText.match(/WordRepresentation::(\w+)\(\)/);
  if (m) return `Some(RegisterRepresentation::${m[1]})`;
  m = repText.match(/FloatRepresentation::(\w+)\(\)/);
  if (m) return `Some(RegisterRepresentation::${m[1]})`;
  if (repText.startsWith("v_traits<")) return "None // 透传特化链";
  throw new Error(`未登记 rep 映射: ${repText}`);
}

// ---------------------------------------------------------------------------
// 3. 生成 Rust
// ---------------------------------------------------------------------------
let rust = banner(
  "src/compiler/turboshaft/index.h.js",
  "src/compiler/turboshaft/index.h",
  "OpIndex/BlockIndex 索引与抽象类型标签",
);

rust += `use crate::representations::RegisterRepresentation;\n\n`;

rust += `
/// 原版 OperationStorageSlot — 操作缓冲存储槽 (64 位)
pub type OperationStorageSlot = u64;

/// 原版 kSlotsPerId — 每个操作至少占 2 槽, 每 2 槽分配一个 id
pub const SLOTS_PER_ID: usize = ${slotsPerId};

/// 原版 kGenerationMaskShift — DEBUG 世代位偏移 (release 布局不含世代位)
pub const GENERATION_MASK_SHIFT: u32 = ${constBinds[1].value};

/// 原版 kGenerationMask
pub const GENERATION_MASK: u32 = 1 << GENERATION_MASK_SHIFT;

/// 原版 kUnmaskGenerationMask
pub const UNMASK_GENERATION_MASK: u32 = !GENERATION_MASK;

/// 原版 kExternalIdFlag — 外部节点 id 编码标志 (offset 低 0 位残余)
pub const EXTERNAL_ID_FLAG: u32 = ${constBinds[2].value};

// DEBUG 世代位编码 (FromOffset 里 offset |= generation_bit << 1) 仅 DEBUG 布局;
// release 布局 offset 即纯偏移, 此处按 release 语义生成。

/// 原版 OpIndex — 操作缓冲偏移索引 (32 位, 缓冲重分配下稳定)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct OpIndex(u32);

impl OpIndex {
  /// 原版 Invalid() — 全 1 哨兵
  pub const INVALID: OpIndex = OpIndex(u32::MAX);

  /// 原版 FromOffset (offset 须为 OperationStorageSlot 大小的倍数)
  #[inline]
  #[must_use]
  pub const fn from_offset(offset: u32) -> Self {
    Self(offset)
  }

  /// 原版 id() — offset / sizeof(slot) / kSlotsPerId (紧凑 id, 供 side-table)
  #[inline]
  #[must_use]
  pub const fn id(self) -> u32 {
    self.0 / 8 / SLOTS_PER_ID as u32
  }

  /// 原版 hash() — Invalid 也返回 id
  #[inline]
  #[must_use]
  pub const fn hash(self) -> u32 {
    self.0 / 8 / SLOTS_PER_ID as u32
  }

  /// 原版 offset()
  #[inline]
  #[must_use]
  pub const fn offset(self) -> u32 {
    self.0 & UNMASK_GENERATION_MASK
  }

  /// 原版 valid()
  #[inline]
  #[must_use]
  pub const fn valid(self) -> bool {
    self.0 != Self::INVALID.0
  }

  /// 原版 IsExternalId
  #[inline]
  #[must_use]
  pub const fn is_external_id(self) -> bool {
    self.0 % 8 == EXTERNAL_ID_FLAG
  }

  /// 原版 EncodeExternalId — 跨图节点来源 (Turbofan/Maglev) 编码
  #[must_use]
  pub const fn encode_external_id(id: u32) -> Self {
    Self(id * 8 + EXTERNAL_ID_FLAG)
  }

  /// 原版 DecodeExternalId
  #[inline]
  #[must_use]
  pub const fn decode_external_id(self) -> u32 {
    self.0 / 8
  }
}

impl Default for OpIndex {
  fn default() -> Self {
    Self::INVALID
  }
}

/// 原版 BlockIndex — 已绑定块索引 (支配块序号更小)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BlockIndex(u32);

impl BlockIndex {
  /// 原版 Invalid()
  pub const INVALID: BlockIndex = BlockIndex(u32::MAX);

  #[inline]
  #[must_use]
  pub const fn new(id: u32) -> Self {
    Self(id)
  }

  #[inline]
  #[must_use]
  pub const fn id(self) -> u32 {
    self.0
  }

  /// 原版 valid()
  #[inline]
  #[must_use]
  pub const fn valid(self) -> bool {
    self.0 != Self::INVALID.0
  }
}

impl Default for BlockIndex {
  fn default() -> Self {
    Self::INVALID
  }
}

/// 原版 OptionalOpIndex — Rust 侧直接用 Option<OpIndex>;
/// 保留别名以登记原版形态。
pub type OptionalOpIndex = Option<OpIndex>;

// ---------------------------------------------------------------------------
// 抽象类型标签族 (V<T> 的类型层标记; Rust 侧以单元结构体登记)
// ---------------------------------------------------------------------------
`;

for (const t of tagDecls) {
  const doc =
    t.kind === "any" ? "原版抽象标签 (Any)" :
    t.kind === "internal" ? "原版内部标签 (InternalTag 子类)" :
    t.kind === "word64alias" ? "原版 WordPtr = Word64 (64 位目标)" :
    t.kind.startsWith("word") ? `原版 WordWithBits<${t.kind.slice(4)}>` :
    t.kind.startsWith("float") ? `原版 FloatWithBits<${t.kind.slice(5)}>` :
    null;
  if (!doc) throw new Error(`未登记标签种类: ${t.kind}`);
  rust += `/// ${doc}\npub struct ${t.name};\n\n`;
}

rust += `/// 原版 AnyFrameState = UntaggedUnion<EagerFrameState, LazyFrameState>
pub struct AnyFrameState;

/// 原版 v_traits<T>::rep 映射表 — 抽象类型 → 寄存器表示 (None = nullrep)
pub fn abstract_rep(name: &str) -> Option<crate::representations::RegisterRepresentation> {
  match name {
`;
for (const e of repMap) {
  rust += `    "${e.name}" => ${rustRep(e.rep)},\n`;
}
rust += `    _ => None,
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SLOTS_PER_ID == 2);
  assert!(OpIndex::INVALID.0 == u32::MAX);
  assert!(OpIndex::from_offset(16).id() == 1);
  assert!(!OpIndex::INVALID.valid());
  assert!(OpIndex::encode_external_id(3).is_external_id());
  assert!(OpIndex::encode_external_id(3).decode_external_id() == 3);
  assert!(BlockIndex::INVALID.id() == u32::MAX);
  assert!(GENERATION_MASK == 2);
};
`;

const p = writeSrc("v8-turboshaft", "index.rs", rust);
console.log(`written ${p}`);
console.log(`常量=${constBinds.length} 标签=${tagDecls.length} v_traits rep 映射=${repEntries}`);
