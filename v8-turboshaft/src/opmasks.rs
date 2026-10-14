// 由 tool/src/compiler/turboshaft/opmasks.h.js 从 v8/src/compiler/turboshaft/opmasks.h 生成; 修订就地改, 复跑覆盖
// 操作位掩码机制 (小端) — 具名表已收编至 opmasks_named.rs (挂载面)

/// 原版机制: 操作前 8 字节 (小端: 低 64 位) 按 mask 提取后与编码值全等比较。
/// - BuildBaseMask: opcode 占第 0 字节 (OFFSET_OF(Operation, opcode)==0,
///   sizeof(Operation::opcode)==1, sizeof(Operation)==4), 小端 base_mask = 0xFF
///   (大端移至最高字节 — 本 crate 目标 x64+arm64 均小端)
/// - 每 FIELD(Op, f) 贡献 [f.offset, f.offset+f.size) 字节的 ones 掩码
/// - EncodeValue = base(opcode) | field 值按各自 offset 左移
///
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
    let ones = u64::MAX >> ((8 - f.size) * 8u32);
    ones << (f.offset * 8u32)
  }

  /// v8: opmasks.h MaskBuilder::EncodeFieldValue
  /// encode_for_mask(value) = static_cast<uint64_t>(value), 左移 offset*8
  #[must_use]
  pub const fn encode_field_value(f: &OpMaskField, value: u64) -> u64 {
    assert!(f.offset + f.size <= 8, "offset + size <= 8");
    value << (f.offset * 8u32)
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
