#!/usr/bin/env bun
// tool/src/compiler/turboshaft/opmasks.h.js
// 机械提取 v8/src/compiler/turboshaft/opmasks.h
// → v8-turboshaft/src/opmasks.rs.skel
//
// 提取面: MaskBuilder/FIELD 位掩码机制; 全部具名 opmask (using kXxx =
// XxxMask::For<...>) 逐条登记; SIMD X-macro 展开锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/opmasks.h");

// 1. 机制哨兵
for (const [re, name] of [
  [/struct OpMaskField \{/, "OpMaskField"],
  [/struct MaskBuilder \{/, "MaskBuilder"],
  [/static constexpr uint64_t BuildBaseMask\(\)/, "BuildBaseMask"],
  [/OFFSET_OF\(Operation, opcode\) == 0/, "opcode 偏移 0"],
  [/sizeof\(Operation\) == 4/, "Operation 4 字节"],
  [/struct UnwrapRepresentation/, "UnwrapRepresentation"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. MaskBuilder 族: using XxxMask = MaskBuilder<Op, FIELD(...)...>
const builders = [...h.matchAll(/using (\w+Mask) =\s*\n?\s*MaskBuilder<([\w:]+)((?:,\s*FIELD\([\w:]+, (\w+)\))+)>;/g)].map((m) => ({
  name: m[1],
  op: m[2],
  fields: [...m[3].matchAll(/FIELD\([\w:]+, (\w+)\)/g)].map((f) => f[1]),
}));
if (builders.length < 20) throw new Error(`MaskBuilder 族 ${builders.length} < 20`);

// 3. 具名 opmask: using kXxx = YyyMask::For<args>
const masks = [...h.matchAll(/using (k\w+) =\s*\n?\s*(\w+Mask)::For<([\s\S]*?)>;/g)].map((m) => ({
  name: m[1],
  mask: m[2],
  args: m[3].replace(/\s+/g, " ").trim(),
}));
if (masks.length < 55) throw new Error(`具名 opmask ${masks.length} < 55`);
for (const must of ["kWord32Add", "kTaggedPhi", "kProjection0", "kWord64Equal", "kBitcastWordPtrToSmi"]) {
  if (!masks.some((x) => x.name === must)) throw new Error(`opmask 缺 ${must}`);
}

// 4. SIMD X-macro 锚 (语义面, 不展开)
const xmacros = [...h.matchAll(/FOREACH_SIMD_\w+\((\w+)\)/g)].map((m) => m[1]);
if (xmacros.length !== 5) throw new Error(`SIMD X-macro ${xmacros.length} != 5`);

// 机制区 (可机械 1:1 实现): ones 掩码与位移公式从 opmasks.h 提取
const bitsPerByte = 8;

let rust = banner(
  "src/compiler/turboshaft/opmasks.h.js",
  "src/compiler/turboshaft/opmasks.h",
  "操作位掩码机制 (小端) — 具名表在 opmasks_named.rs.skel 推迟",
);

rust += `
/// 原版机制: 操作前 8 字节 (小端: 低 64 位) 按 mask 提取后与编码值全等比较。
/// - BuildBaseMask: opcode 占第 0 字节 (OFFSET_OF(Operation, opcode)==0,
///   sizeof(Operation::opcode)==1, sizeof(Operation)==4), 小端 base_mask = 0xFF
///   (大端移至最高字节 — 本 crate 目标 x64+arm64 均小端)
/// - 每 FIELD(Op, f) 贡献 [f.offset, f.offset+f.size) 字节的 ones 掩码
/// - EncodeValue = base(opcode) | field 值按各自 offset 左移
/// v8: src/compiler/turboshaft/opmasks.h MaskBuilder::BuildBaseMask
pub const OP_MASK_OPCODE_BASE: u64 = 0xFF;

/// 原版 OpMaskField<T, Offset> — 字段形状 (名, 字节偏移, 字节宽度)
/// v8: src/compiler/turboshaft/opmasks.h OpMaskField
#[derive(Debug, Clone, Copy)]
pub struct OpMaskField {
  /// FIELD 字段名 (仅诊断用)
  pub name: &'static str,
  /// 相对操作首字节偏移
  pub offset: u32,
  /// sizeof(字段类型)
  pub size: u32,
}

/// 原版 OpMaskT<Op, Mask, Value> — (mask, value) 对
/// v8: src/compiler/turboshaft/opmasks.h OpMaskT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpMask {
  pub mask: u64,
  pub value: u64,
}

impl OpMask {
  /// 原版 Operation::Is<Mask>() / TryCast: (首 8 字节 & mask) == value
  /// v8: src/compiler/turboshaft/opmasks.h Operation 匹配语义
  #[must_use]
  pub const fn matches(&self, head: u64) -> bool {
    (head & self.mask) == self.value
  }
}

/// 原版 MaskBuilder 机制面 — Fields 用运行时切片代替模板参数包
/// v8: src/compiler/turboshaft/opmasks.h MaskBuilder<Op, Fields...>
#[derive(Debug, Clone, Copy)]
pub struct MaskBuilder {
  /// FIELD(Op, f) 集 (opcode 基础掩码隐含, 不在此列)
  pub fields: &'static [OpMaskField],
}

impl MaskBuilder {
  /// v8: opmasks.h MaskBuilder::BuildFieldMask
  /// ones = (u64::MAX >> ((8 - size) * 8)) << (offset * 8)
  #[must_use]
  pub const fn build_field_mask(f: &OpMaskField) -> u64 {
    assert!(f.size < 8, "FIELD::size < sizeof(uint64_t)");
    assert!(f.offset + f.size <= 8, "offset + size <= 8");
    let ones = u64::MAX >> ((${bitsPerByte} - f.size) * ${bitsPerByte}u32);
    ones << (f.offset * ${bitsPerByte}u32)
  }

  /// v8: opmasks.h MaskBuilder::EncodeFieldValue
  /// encode_for_mask(value) = static_cast<uint64_t>(value), 左移 offset*8
  #[must_use]
  pub const fn encode_field_value(f: &OpMaskField, value: u64) -> u64 {
    assert!(f.offset + f.size <= 8, "offset + size <= 8");
    value << (f.offset * ${bitsPerByte}u32)
  }

  /// v8: opmasks.h MaskBuilder::BuildMask — base | ... | field 掩码
  #[must_use]
  pub const fn build_mask(&self) -> u64 {
    let mut mask = OP_MASK_OPCODE_BASE;
    let mut i = 0;
    while i < self.fields.len() {
      mask |= Self::build_field_mask(&self.fields[i]);
      i += 1;
    }
    mask
  }

  /// v8: opmasks.h MaskBuilder::EncodeValue — args 与 fields 一一对应
  #[must_use]
  pub const fn encode_value(&self, opcode: u8, args: &[u64]) -> u64 {
    assert!(args.len() == self.fields.len(), "args 与 fields 等长");
    let mut value = opcode as u64;
    let mut i = 0;
    while i < args.len() {
      value |= Self::encode_field_value(&self.fields[i], args[i]);
      i += 1;
    }
    value
  }

  /// v8: opmasks.h MaskBuilder::For — BuildMask() + EncodeValue(args) 合成
  #[must_use]
  pub const fn r#for(&self, opcode: u8, args: &[u64]) -> OpMask {
    OpMask {
      mask: self.build_mask(),
      value: self.encode_value(opcode, args),
    }
  }
}
`;

const p = writeSrc("v8-turboshaft", "opmasks.rs", rust);
console.log(`written ${p}`);
console.log(`机制=MaskBuilder 全五式 + OpMask::matches`);

// 具名表推迟件: 24 族与 ${masks.length} 条常量需要 operations.h 结构布局
// (sizeof/offsetof 编译期值) — 无法机械展开, 保留登记面 skel。
let named = banner(
  "src/compiler/turboshaft/opmasks.h.js",
  "src/compiler/turboshaft/opmasks.h",
  "具名 opmask 表 — 推迟: 依赖 operations.h 各 Op 字段 offset 布局",
);
named += `
/// 原版 MaskBuilder 族登记 (${builders.length} 族):
${builders.map((b) => `/// - ${b.name} = MaskBuilder<${b.op}, ${b.fields.map((f) => "FIELD(" + f + ")").join(", ")}>`).join("\n")}

/// 原版具名 opmask (${masks.length} 条) — 小端编码下的 (mask, value) 对
/// 待前置: 各 Op 的 opcode 值 (operations.rs Opcode 已有) + 字段 offset/size 表
/// v8: src/compiler/turboshaft/opmasks.h 各 using kXxx
pub mod opmasks_named {
${masks.map((m) => `  /// ${m.name} = ${m.mask}::For<${m.args}>`).join("\n")}
}

/// SIMD X-macro 展开面 (仅 simd128 编译期可见; simd256 另有 1 族):
${xmacros.map((x) => `/// - FOREACH_SIMD_128_*(${x}): kSimd128${x[0].toUpperCase()}${x.slice(1)} 系列`).join("\n")}
/// v8: src/compiler/turboshaft/opmasks.h SIMD_*_MASK 宏块
`;
const p2 = writeSrc("v8-turboshaft", "opmasks_named.rs.skel", named);
console.log(`written ${p2}`);
console.log(`MaskBuilder族=${builders.length} 具名opmask=${masks.length} SIMD-X-macro=${xmacros.length}`);
