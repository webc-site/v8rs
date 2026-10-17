//! opmasks_named 收编核验 — 具名掩码 (mask,value) 对与原版首 8 字节小端布局逐项对账
//! v8: src/compiler/turboshaft/opmasks.h 具名表 (断言右值按原版字节布局独立推得)

use aok::{OK, Void};
use log::info;
use v8_turboshaft::{
  CHANGE_OP, Opcode, WORD_BINOP, WORD_BINOP_KIND, k_bitcast_word_ptr_to_smi, k_bitwise_and,
  k_change_int32_to_int64, k_comparison_equal, k_projection0, k_projection1, k_simd128_binop,
  k_tagged_phi, k_word32_add, k_word32_shift_left, k_word64_bitwise_and, k_word64_equal,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版首 8 字节小端拼接: byte0=opcode, 载荷字段自 byte4 按声明序 (对齐 1 类型)
const fn head(opcode: u8, byte4: u64, byte5: u64, byte6: u64, byte7: u64) -> u64 {
  (opcode as u64) | (byte4 << 32) | (byte5 << 40) | (byte6 << 48) | (byte7 << 56)
}

/// WordBinop 族: kWord32Add 全掩码 + kBitwiseAnd kind 部分掩码
#[test]
fn named_mask_word_binop() -> Void {
  info!("> kWord32Add / kBitwiseAnd / kWord64BitwiseAnd");
  let add = k_word32_add();
  // 原版掩码: opcode byte0 + kind byte4 + rep byte5
  assert_eq!(add.mask, 0x0000_FFFF_0000_00FF);
  // Word32=0/Word64=1 (RegisterRepresentation 编码), kAdd=0
  assert_eq!(add.value, head(Opcode::WordBinop.index(), 0, 0, 0, 0));
  assert!(add.matches(head(Opcode::WordBinop.index(), 0, 0, 0, 0)));
  // kSub=7 不得命中
  assert!(!add.matches(head(Opcode::WordBinop.index(), 7, 0, 0, 0)));

  let bitand64 = k_word64_bitwise_and();
  // kBitwiseAnd=4, rep Word64=1
  assert!(bitand64.matches(head(Opcode::WordBinop.index(), 4, 1, 0, 0)));
  // rep Word32=0 不得命中
  assert!(!bitand64.matches(head(Opcode::WordBinop.index(), 4, 0, 0, 0)));

  let bitand = k_bitwise_and();
  assert_eq!(bitand.mask, 0x0000_00FF_0000_00FF);
  // kind-only: 任意 rep 均命中
  assert!(bitand.matches(head(Opcode::WordBinop.index(), 4, 0, 0, 0)));
  assert!(bitand.matches(head(Opcode::WordBinop.index(), 4, 1, 0, 0)));
  // kBitwiseXor=6 不命中
  assert!(!bitand.matches(head(Opcode::WordBinop.index(), 6, 1, 0, 0)));
  OK
}

/// ChangeOp 族: 4 字段 (kind/assumption/from/to) 全 8 字节掩码
#[test]
fn named_mask_change_op() -> Void {
  info!("> kChangeInt32ToInt64");
  let m = k_change_int32_to_int64();
  // kind@4 assumption@5 from@6 to@7 → byte0 + byte4..7
  assert_eq!(m.mask, 0xFFFF_FFFF_0000_00FF);
  // kSignExtend=11, kNoAssumption=0, from Word32=0, to Word64=1
  assert_eq!(m.value, head(Opcode::Change.index(), 11, 0, 0, 1));
  assert!(m.matches(head(Opcode::Change.index(), 11, 0, 0, 1)));
  // assumption=kNoOverflow(1) 不得命中
  assert!(!m.matches(head(Opcode::Change.index(), 11, 1, 0, 1)));
  // to=Word32(0) (即 uint32→uint32 不存在) 不得命中
  assert!(!m.matches(head(Opcode::Change.index(), 11, 0, 0, 0)));
  OK
}

/// ProjectionOp 族: index u16@4 (size 2)
#[test]
fn named_mask_projection() -> Void {
  info!("> kProjection0 / kProjection1");
  let p0 = k_projection0();
  let p1 = k_projection1();
  assert_eq!(p0.mask, 0x0000_FFFF_0000_00FF);
  let opcode = Opcode::Projection.index();
  assert!(p0.matches(head(opcode, 0, 0, 0, 0)));
  assert!(p1.matches(head(opcode, 1, 0, 0, 0)));
  assert!(!p0.matches(head(opcode, 1, 0, 0, 0)));
  assert!(!p1.matches(head(opcode, 2, 0, 0, 0)));
  OK
}

/// Phi/Comparison/Shift kind-only 部分掩码跨其余字段成立
#[test]
fn named_mask_partial_families() -> Void {
  info!("> kTaggedPhi / kComparisonEqual / kWord32ShiftLeft");
  let phi = k_tagged_phi();
  // PhiOp 首字段 rep@4, Tagged=4
  assert!(phi.matches(head(Opcode::Phi.index(), 4, 0, 0, 0)));
  assert!(!phi.matches(head(Opcode::Phi.index(), 0, 0, 0, 0)));

  let eq = k_comparison_equal();
  // ComparisonOp kind@4 (kEqual=0), rep byte5 不参与
  assert!(eq.matches(head(Opcode::Comparison.index(), 0, 0, 0, 0)));
  assert!(eq.matches(head(Opcode::Comparison.index(), 0, 1, 0, 0)));
  assert!(!eq.matches(head(Opcode::Comparison.index(), 1, 0, 0, 0)));

  let w64eq = k_word64_equal();
  // kEqual=0 + rep Word64=1@5
  assert!(w64eq.matches(head(Opcode::Comparison.index(), 0, 1, 0, 0)));
  assert!(!w64eq.matches(head(Opcode::Comparison.index(), 0, 0, 0, 0)));

  let shl = k_word32_shift_left();
  // ShiftOp: kind@4 (kShiftLeft=3), rep@5 (Word32=0)
  assert!(shl.matches(head(Opcode::Shift.index(), 3, 0, 0, 0)));
  assert!(!shl.matches(head(Opcode::Shift.index(), 3, 1, 0, 0)));
  OK
}

/// TaggedBitcastMask 字段序 (from@5, to@6, kind@4) 对账
#[test]
fn named_mask_tagged_bitcast() -> Void {
  info!("> kBitcastWordPtrToSmi");
  let m = k_bitcast_word_ptr_to_smi();
  assert_eq!(m.mask, 0x00FF_FFFF_0000_00FF);
  // from WordPtr=kWord64=1 @byte5, to Tagged=4 @byte6, kind kSmi=0 @byte4
  assert_eq!(m.value, head(Opcode::TaggedBitcast.index(), 0, 1, 4, 0));
  assert!(m.matches(head(Opcode::TaggedBitcast.index(), 0, 1, 4, 0)));
  assert!(!m.matches(head(Opcode::TaggedBitcast.index(), 1, 1, 4, 0)));
  OK
}

/// SIMD X-macro 机械面: 仅 opcode+kind byte4 参与
#[test]
fn named_mask_simd_generic_family() -> Void {
  info!("> k_simd128_binop 泛型面");
  let m = k_simd128_binop(0x2A);
  assert_eq!(m.mask, 0x0000_00FF_0000_00FF);
  assert_eq!(m.value, head(Opcode::Simd128Binop.index(), 0x2A, 0, 0, 0));
  assert!(m.matches(head(Opcode::Simd128Binop.index(), 0x2A, 9, 9, 9)));
  assert!(!m.matches(head(Opcode::Simd128Binop.index(), 0x2B, 0, 0, 0)));
  OK
}

/// 族间隔离: 全掩码严格于部分掩码, 不误配
#[test]
fn named_mask_family_isolation() -> Void {
  info!("> WORD_BINOP vs WORD_BINOP_KIND 掩码宽度");
  let full = WORD_BINOP.r#for(Opcode::WordBinop.index(), &[4, 1]);
  let kind_only = WORD_BINOP_KIND.r#for(Opcode::WordBinop.index(), &[4]);
  assert!(full.mask & kind_only.mask == kind_only.mask);
  // 部分掩码命中全掩码值; 全掩码不命中缺 rep 字节的值
  assert!(kind_only.matches(full.value));
  assert!(!full.matches(kind_only.value));
  OK
}

/// 合并语义 (逐位对): mask 为 base 与逐字段掩码的无交并, value 为逐字段值的按位或
/// v8: opmasks.h MaskBuilder::BuildMask = base_mask | ... | BuildFieldMask
///     MaskBuilder::EncodeValue = base_mask | ... | EncodeFieldValue
#[test]
fn named_mask_field_union_bitwise() -> Void {
  info!("> mask/value 逐字段位并");
  const REP5: u64 = 0xFFu64 << 40; // FIELD rep@offset5 size1
  const KIND4: u64 = 0xFFu64 << 32; // FIELD kind@offset4 size1
  let op = Opcode::WordBinop.index();

  // mask 无交并: base(0xFF) | kind4 | rep5 == WORD_BINOP mask
  assert_eq!(v8_turboshaft::OP_MASK_OPCODE_BASE, 0xFF);
  let wop_mask = WORD_BINOP.build_mask();
  let wkind_mask = WORD_BINOP_KIND.build_mask();
  assert_eq!(wop_mask, v8_turboshaft::OP_MASK_OPCODE_BASE | KIND4 | REP5);
  // WORD_BINOP_KIND = base | kind4 (不含 rep5)
  assert_eq!(wkind_mask, v8_turboshaft::OP_MASK_OPCODE_BASE | KIND4);
  // 全掩码 = 部分掩码 | rep 字段掩码 (字段无重叠, 并集精确)
  assert_eq!(wop_mask, wkind_mask | REP5);
  assert_eq!(wkind_mask & REP5, 0, "kind 与 rep 掩码位不相交");

  // value 按位或可加: 同 opcode 下 [k,r] = kind 部分值 | (r 左移到 rep 位)
  for (k, r) in [(4u64, 0u64), (4, 1), (6, 1), (7, 0)] {
    let full = WORD_BINOP.r#for(op, &[k, r]);
    let kind_only = WORD_BINOP_KIND.r#for(op, &[k]);
    assert_eq!(full.value, kind_only.value | (r << 40));
    // 两掩码间差异仅落在被掩掉的字节: 异或必为掩码子集
    assert_eq!((full.value ^ kind_only.value) & !full.mask, 0);
  }

  // 多字段: CHANGE_OP = base | kind4 | byte5 | byte6 | byte7, 逐位分解
  let change = CHANGE_OP.r#for(
    Opcode::Change.index(),
    &[11, 0, 0, 1], // SignExtend, NoAssumption, Word32, Word64
  );
  assert_eq!(
    change.mask,
    v8_turboshaft::OP_MASK_OPCODE_BASE | KIND4 | (0xFF << 40) | (0xFF << 48) | (0xFF << 56)
  );
  // 期望值: opcode@0 | kind=11<<32 | assump=0 | from=0 | to=1<<56
  assert_eq!(
    change.value,
    (Opcode::Change.index() as u64) | (11 << 32) | (1 << 56)
  );
  // 合并两独立字段子掩码值仍等于全字段值 (base + kind + to, 省略 assump/from 为 0)
  let merged = (Opcode::Change.index() as u64) | (11 << 32) | (1u64 << 56);
  assert_eq!(change.value, merged, "字段值合并逐位对齐");
  OK
}
