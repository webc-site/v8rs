// 由 tool/src/codegen/arm64/decoder-arm64.h.js 从 v8/src/codegen/arm64/decoder-arm64.{h,cc} 生成; 修订就地改, 复跑覆盖
// arm64 解码器: VISITOR_LIST 展开 + 解码树骨架

use std::ptr::eq;

use crate::instructions::Instruction;

/// 原版 VISITOR_LIST 展开的访问者类别 (共 74 项)。
/// DecoderVisitor 以 trait 表达: 每个类别一个 visit_* 方法。
#[rustfmt::skip]
pub const VISITOR_LIST: &[&str] = &[
  "PCRelAddressing",
  "AddSubImmediate",
  "LogicalImmediate",
  "MinMaxImmediate",
  "MoveWideImmediate",
  "Bitfield",
  "Extract",
  "UnconditionalBranch",
  "UnconditionalBranchToRegister",
  "CompareBranch",
  "TestBranch",
  "ConditionalBranch",
  "System",
  "Exception",
  "LoadStorePairPostIndex",
  "LoadStorePairOffset",
  "LoadStorePairPreIndex",
  "LoadLiteral",
  "LoadStoreUnscaledOffset",
  "LoadStorePostIndex",
  "LoadStorePreIndex",
  "LoadStoreRegisterOffset",
  "LoadStoreUnsignedOffset",
  "LoadStoreAcquireRelease",
  "AtomicMemory",
  "LogicalShifted",
  "AddSubShifted",
  "AddSubExtended",
  "AddSubWithCarry",
  "ConditionalCompareRegister",
  "ConditionalCompareImmediate",
  "ConditionalSelect",
  "DataProcessing1Source",
  "DataProcessing2Source",
  "DataProcessing3Source",
  "FPCompare",
  "FPConditionalCompare",
  "FPConditionalSelect",
  "FPImmediate",
  "FPDataProcessing1Source",
  "FPDataProcessing2Source",
  "FPDataProcessing3Source",
  "FPIntegerConvert",
  "FPFixedPointConvert",
  "NEON2RegMisc",
  "NEON3Different",
  "NEON3Extension",
  "NEON3Same",
  "NEON3SameHP",
  "NEONAcrossLanes",
  "NEONByIndexedElement",
  "NEONCopy",
  "NEONExtract",
  "NEONLoadStoreMultiStruct",
  "NEONLoadStoreMultiStructPostIndex",
  "NEONLoadStoreSingleStruct",
  "NEONLoadStoreSingleStructPostIndex",
  "NEONModifiedImmediate",
  "NEONScalar2RegMisc",
  "NEONScalar3Diff",
  "NEONScalar3Same",
  "NEONScalarByIndexedElement",
  "NEONScalarCopy",
  "NEONScalarPairwise",
  "NEONScalarShiftImmediate",
  "NEONShiftImmediate",
  "NEONTable",
  "NEONPerm",
  "NEONSHA3",
  "Cpy",
  "Set",
  "SVEBitPerm",
  "Unallocated",
  "Unimplemented",
];

/// 原版 DecoderVisitor 接口 (C++ 虚基类 → Rust trait):
/// 实现方为 74 类别提供 Visit 方法 (反汇编器/模拟器)。默认空实现。
pub trait DecoderVisitor {
  /// 原版 VisitPCRelAddressing
  fn visit_pcrel_addressing(&mut self, _instr: &Instruction) {}
  /// 原版 VisitAddSubImmediate
  fn visit_add_sub_immediate(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLogicalImmediate
  fn visit_logical_immediate(&mut self, _instr: &Instruction) {}
  /// 原版 VisitMinMaxImmediate
  fn visit_min_max_immediate(&mut self, _instr: &Instruction) {}
  /// 原版 VisitMoveWideImmediate
  fn visit_move_wide_immediate(&mut self, _instr: &Instruction) {}
  /// 原版 VisitBitfield
  fn visit_bitfield(&mut self, _instr: &Instruction) {}
  /// 原版 VisitExtract
  fn visit_extract(&mut self, _instr: &Instruction) {}
  /// 原版 VisitUnconditionalBranch
  fn visit_unconditional_branch(&mut self, _instr: &Instruction) {}
  /// 原版 VisitUnconditionalBranchToRegister
  fn visit_unconditional_branch_to_register(&mut self, _instr: &Instruction) {}
  /// 原版 VisitCompareBranch
  fn visit_compare_branch(&mut self, _instr: &Instruction) {}
  /// 原版 VisitTestBranch
  fn visit_test_branch(&mut self, _instr: &Instruction) {}
  /// 原版 VisitConditionalBranch
  fn visit_conditional_branch(&mut self, _instr: &Instruction) {}
  /// 原版 VisitSystem
  fn visit_system(&mut self, _instr: &Instruction) {}
  /// 原版 VisitException
  fn visit_exception(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLoadStorePairPostIndex
  fn visit_load_store_pair_post_index(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLoadStorePairOffset
  fn visit_load_store_pair_offset(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLoadStorePairPreIndex
  fn visit_load_store_pair_pre_index(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLoadLiteral
  fn visit_load_literal(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLoadStoreUnscaledOffset
  fn visit_load_store_unscaled_offset(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLoadStorePostIndex
  fn visit_load_store_post_index(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLoadStorePreIndex
  fn visit_load_store_pre_index(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLoadStoreRegisterOffset
  fn visit_load_store_register_offset(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLoadStoreUnsignedOffset
  fn visit_load_store_unsigned_offset(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLoadStoreAcquireRelease
  fn visit_load_store_acquire_release(&mut self, _instr: &Instruction) {}
  /// 原版 VisitAtomicMemory
  fn visit_atomic_memory(&mut self, _instr: &Instruction) {}
  /// 原版 VisitLogicalShifted
  fn visit_logical_shifted(&mut self, _instr: &Instruction) {}
  /// 原版 VisitAddSubShifted
  fn visit_add_sub_shifted(&mut self, _instr: &Instruction) {}
  /// 原版 VisitAddSubExtended
  fn visit_add_sub_extended(&mut self, _instr: &Instruction) {}
  /// 原版 VisitAddSubWithCarry
  fn visit_add_sub_with_carry(&mut self, _instr: &Instruction) {}
  /// 原版 VisitConditionalCompareRegister
  fn visit_conditional_compare_register(&mut self, _instr: &Instruction) {}
  /// 原版 VisitConditionalCompareImmediate
  fn visit_conditional_compare_immediate(&mut self, _instr: &Instruction) {}
  /// 原版 VisitConditionalSelect
  fn visit_conditional_select(&mut self, _instr: &Instruction) {}
  /// 原版 VisitDataProcessing1Source
  fn visit_data_processing1_source(&mut self, _instr: &Instruction) {}
  /// 原版 VisitDataProcessing2Source
  fn visit_data_processing2_source(&mut self, _instr: &Instruction) {}
  /// 原版 VisitDataProcessing3Source
  fn visit_data_processing3_source(&mut self, _instr: &Instruction) {}
  /// 原版 VisitFPCompare
  fn visit_fpcompare(&mut self, _instr: &Instruction) {}
  /// 原版 VisitFPConditionalCompare
  fn visit_fpconditional_compare(&mut self, _instr: &Instruction) {}
  /// 原版 VisitFPConditionalSelect
  fn visit_fpconditional_select(&mut self, _instr: &Instruction) {}
  /// 原版 VisitFPImmediate
  fn visit_fpimmediate(&mut self, _instr: &Instruction) {}
  /// 原版 VisitFPDataProcessing1Source
  fn visit_fpdata_processing1_source(&mut self, _instr: &Instruction) {}
  /// 原版 VisitFPDataProcessing2Source
  fn visit_fpdata_processing2_source(&mut self, _instr: &Instruction) {}
  /// 原版 VisitFPDataProcessing3Source
  fn visit_fpdata_processing3_source(&mut self, _instr: &Instruction) {}
  /// 原版 VisitFPIntegerConvert
  fn visit_fpinteger_convert(&mut self, _instr: &Instruction) {}
  /// 原版 VisitFPFixedPointConvert
  fn visit_fpfixed_point_convert(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEON2RegMisc
  fn visit_neon2_reg_misc(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEON3Different
  fn visit_neon3_different(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEON3Extension
  fn visit_neon3_extension(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEON3Same
  fn visit_neon3_same(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEON3SameHP
  fn visit_neon3_same_hp(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONAcrossLanes
  fn visit_neonacross_lanes(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONByIndexedElement
  fn visit_neonby_indexed_element(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONCopy
  fn visit_neoncopy(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONExtract
  fn visit_neonextract(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONLoadStoreMultiStruct
  fn visit_neonload_store_multi_struct(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONLoadStoreMultiStructPostIndex
  fn visit_neonload_store_multi_struct_post_index(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONLoadStoreSingleStruct
  fn visit_neonload_store_single_struct(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONLoadStoreSingleStructPostIndex
  fn visit_neonload_store_single_struct_post_index(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONModifiedImmediate
  fn visit_neonmodified_immediate(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONScalar2RegMisc
  fn visit_neonscalar2_reg_misc(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONScalar3Diff
  fn visit_neonscalar3_diff(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONScalar3Same
  fn visit_neonscalar3_same(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONScalarByIndexedElement
  fn visit_neonscalar_by_indexed_element(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONScalarCopy
  fn visit_neonscalar_copy(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONScalarPairwise
  fn visit_neonscalar_pairwise(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONScalarShiftImmediate
  fn visit_neonscalar_shift_immediate(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONShiftImmediate
  fn visit_neonshift_immediate(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONTable
  fn visit_neontable(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONPerm
  fn visit_neonperm(&mut self, _instr: &Instruction) {}
  /// 原版 VisitNEONSHA3
  fn visit_neonsha3(&mut self, _instr: &Instruction) {}
  /// 原版 VisitCpy
  fn visit_cpy(&mut self, _instr: &Instruction) {}
  /// 原版 VisitSet
  fn visit_set(&mut self, _instr: &Instruction) {}
  /// 原版 VisitSVEBitPerm
  fn visit_svebit_perm(&mut self, _instr: &Instruction) {}
  /// 原版 VisitUnallocated
  fn visit_unallocated(&mut self, _instr: &Instruction) {}
  /// 原版 VisitUnimplemented
  fn visit_unimplemented(&mut self, _instr: &Instruction) {}
}

fn ptr_eq(a: *mut dyn DecoderVisitor, b: *mut dyn DecoderVisitor) -> bool {
  eq(a as *mut (), b as *mut ())
}

/// 原版 DispatchingDecoderVisitor: 访问者链派发
#[derive(Default)]
pub struct DispatchingDecoderVisitor {
  /// 原版 visitors_ (注册顺序即调用顺序)
  visitors_: Vec<*mut dyn DecoderVisitor>,
}

impl DispatchingDecoderVisitor {
  #[must_use]
  pub const fn new() -> Self {
    Self {
      visitors_: Vec::new(),
    }
  }

  /// 原版 AppendVisitor: 尾插 (重复注册 = 移位到尾部)
  pub fn append_visitor(&mut self, visitor: *mut dyn DecoderVisitor) {
    self.remove_visitor(visitor);
    self.visitors_.push(visitor);
  }

  /// 原版 PrependVisitor: 头插
  pub fn prepend_visitor(&mut self, visitor: *mut dyn DecoderVisitor) {
    self.remove_visitor(visitor);
    self.visitors_.insert(0, visitor);
  }

  /// 原版 InsertVisitorBefore
  pub fn insert_visitor_before(
    &mut self,
    new_visitor: *mut dyn DecoderVisitor,
    registered: *mut dyn DecoderVisitor,
  ) {
    self.remove_visitor(new_visitor);
    if let Some(pos) = self.visitors_.iter().position(|&v| ptr_eq(v, registered)) {
      self.visitors_.insert(pos, new_visitor);
    } else {
      self.visitors_.push(new_visitor);
    }
  }

  /// 原版 InsertVisitorAfter
  pub fn insert_visitor_after(
    &mut self,
    new_visitor: *mut dyn DecoderVisitor,
    registered: *mut dyn DecoderVisitor,
  ) {
    self.remove_visitor(new_visitor);
    if let Some(pos) = self.visitors_.iter().position(|&v| ptr_eq(v, registered)) {
      self.visitors_.insert(pos + 1, new_visitor);
    } else {
      self.visitors_.push(new_visitor);
    }
  }

  /// 原版 RemoveVisitor
  pub fn remove_visitor(&mut self, visitor: *mut dyn DecoderVisitor) {
    self.visitors_.retain(|&v| !ptr_eq(v, visitor));
  }

  #[inline]
  #[must_use]
  pub fn visitors_count(&self) -> usize {
    self.visitors_.len()
  }

  pub fn decode(&mut self, instr: &Instruction) {
    Decoder::new(self).decode(instr);
  }
}

impl DecoderVisitor for DispatchingDecoderVisitor {
  fn visit_pcrel_addressing(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_pcrel_addressing(instr) };
    }
  }
  fn visit_add_sub_immediate(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_add_sub_immediate(instr) };
    }
  }
  fn visit_logical_immediate(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_logical_immediate(instr) };
    }
  }
  fn visit_min_max_immediate(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_min_max_immediate(instr) };
    }
  }
  fn visit_move_wide_immediate(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_move_wide_immediate(instr) };
    }
  }
  fn visit_bitfield(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_bitfield(instr) };
    }
  }
  fn visit_extract(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_extract(instr) };
    }
  }
  fn visit_unconditional_branch(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_unconditional_branch(instr) };
    }
  }
  fn visit_unconditional_branch_to_register(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_unconditional_branch_to_register(instr) };
    }
  }
  fn visit_compare_branch(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_compare_branch(instr) };
    }
  }
  fn visit_test_branch(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_test_branch(instr) };
    }
  }
  fn visit_conditional_branch(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_conditional_branch(instr) };
    }
  }
  fn visit_system(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_system(instr) };
    }
  }
  fn visit_exception(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_exception(instr) };
    }
  }
  fn visit_load_store_pair_post_index(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_load_store_pair_post_index(instr) };
    }
  }
  fn visit_load_store_pair_offset(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_load_store_pair_offset(instr) };
    }
  }
  fn visit_load_store_pair_pre_index(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_load_store_pair_pre_index(instr) };
    }
  }
  fn visit_load_literal(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_load_literal(instr) };
    }
  }
  fn visit_load_store_unscaled_offset(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_load_store_unscaled_offset(instr) };
    }
  }
  fn visit_load_store_post_index(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_load_store_post_index(instr) };
    }
  }
  fn visit_load_store_pre_index(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_load_store_pre_index(instr) };
    }
  }
  fn visit_load_store_register_offset(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_load_store_register_offset(instr) };
    }
  }
  fn visit_load_store_unsigned_offset(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_load_store_unsigned_offset(instr) };
    }
  }
  fn visit_load_store_acquire_release(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_load_store_acquire_release(instr) };
    }
  }
  fn visit_atomic_memory(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_atomic_memory(instr) };
    }
  }
  fn visit_logical_shifted(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_logical_shifted(instr) };
    }
  }
  fn visit_add_sub_shifted(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_add_sub_shifted(instr) };
    }
  }
  fn visit_add_sub_extended(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_add_sub_extended(instr) };
    }
  }
  fn visit_add_sub_with_carry(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_add_sub_with_carry(instr) };
    }
  }
  fn visit_conditional_compare_register(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_conditional_compare_register(instr) };
    }
  }
  fn visit_conditional_compare_immediate(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_conditional_compare_immediate(instr) };
    }
  }
  fn visit_conditional_select(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_conditional_select(instr) };
    }
  }
  fn visit_data_processing1_source(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_data_processing1_source(instr) };
    }
  }
  fn visit_data_processing2_source(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_data_processing2_source(instr) };
    }
  }
  fn visit_data_processing3_source(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_data_processing3_source(instr) };
    }
  }
  fn visit_fpcompare(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_fpcompare(instr) };
    }
  }
  fn visit_fpconditional_compare(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_fpconditional_compare(instr) };
    }
  }
  fn visit_fpconditional_select(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_fpconditional_select(instr) };
    }
  }
  fn visit_fpimmediate(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_fpimmediate(instr) };
    }
  }
  fn visit_fpdata_processing1_source(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_fpdata_processing1_source(instr) };
    }
  }
  fn visit_fpdata_processing2_source(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_fpdata_processing2_source(instr) };
    }
  }
  fn visit_fpdata_processing3_source(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_fpdata_processing3_source(instr) };
    }
  }
  fn visit_fpinteger_convert(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_fpinteger_convert(instr) };
    }
  }
  fn visit_fpfixed_point_convert(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_fpfixed_point_convert(instr) };
    }
  }
  fn visit_neon2_reg_misc(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neon2_reg_misc(instr) };
    }
  }
  fn visit_neon3_different(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neon3_different(instr) };
    }
  }
  fn visit_neon3_extension(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neon3_extension(instr) };
    }
  }
  fn visit_neon3_same(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neon3_same(instr) };
    }
  }
  fn visit_neon3_same_hp(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neon3_same_hp(instr) };
    }
  }
  fn visit_neonacross_lanes(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonacross_lanes(instr) };
    }
  }
  fn visit_neonby_indexed_element(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonby_indexed_element(instr) };
    }
  }
  fn visit_neoncopy(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neoncopy(instr) };
    }
  }
  fn visit_neonextract(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonextract(instr) };
    }
  }
  fn visit_neonload_store_multi_struct(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonload_store_multi_struct(instr) };
    }
  }
  fn visit_neonload_store_multi_struct_post_index(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonload_store_multi_struct_post_index(instr) };
    }
  }
  fn visit_neonload_store_single_struct(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonload_store_single_struct(instr) };
    }
  }
  fn visit_neonload_store_single_struct_post_index(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonload_store_single_struct_post_index(instr) };
    }
  }
  fn visit_neonmodified_immediate(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonmodified_immediate(instr) };
    }
  }
  fn visit_neonscalar2_reg_misc(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonscalar2_reg_misc(instr) };
    }
  }
  fn visit_neonscalar3_diff(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonscalar3_diff(instr) };
    }
  }
  fn visit_neonscalar3_same(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonscalar3_same(instr) };
    }
  }
  fn visit_neonscalar_by_indexed_element(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonscalar_by_indexed_element(instr) };
    }
  }
  fn visit_neonscalar_copy(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonscalar_copy(instr) };
    }
  }
  fn visit_neonscalar_pairwise(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonscalar_pairwise(instr) };
    }
  }
  fn visit_neonscalar_shift_immediate(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonscalar_shift_immediate(instr) };
    }
  }
  fn visit_neonshift_immediate(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonshift_immediate(instr) };
    }
  }
  fn visit_neontable(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neontable(instr) };
    }
  }
  fn visit_neonperm(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonperm(instr) };
    }
  }
  fn visit_neonsha3(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_neonsha3(instr) };
    }
  }
  fn visit_cpy(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_cpy(instr) };
    }
  }
  fn visit_set(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_set(instr) };
    }
  }
  fn visit_svebit_perm(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_svebit_perm(instr) };
    }
  }
  fn visit_unallocated(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_unallocated(instr) };
    }
  }
  fn visit_unimplemented(&mut self, instr: &Instruction) {
    for &visitor in &self.visitors_ {
      unsafe { (*visitor).visit_unimplemented(instr) };
    }
  }
}

impl<T: DecoderVisitor + ?Sized> DecoderVisitor for &mut T {
  fn visit_pcrel_addressing(&mut self, instr: &Instruction) {
    (**self).visit_pcrel_addressing(instr);
  }
  fn visit_add_sub_immediate(&mut self, instr: &Instruction) {
    (**self).visit_add_sub_immediate(instr);
  }
  fn visit_logical_immediate(&mut self, instr: &Instruction) {
    (**self).visit_logical_immediate(instr);
  }
  fn visit_min_max_immediate(&mut self, instr: &Instruction) {
    (**self).visit_min_max_immediate(instr);
  }
  fn visit_move_wide_immediate(&mut self, instr: &Instruction) {
    (**self).visit_move_wide_immediate(instr);
  }
  fn visit_bitfield(&mut self, instr: &Instruction) {
    (**self).visit_bitfield(instr);
  }
  fn visit_extract(&mut self, instr: &Instruction) {
    (**self).visit_extract(instr);
  }
  fn visit_unconditional_branch(&mut self, instr: &Instruction) {
    (**self).visit_unconditional_branch(instr);
  }
  fn visit_unconditional_branch_to_register(&mut self, instr: &Instruction) {
    (**self).visit_unconditional_branch_to_register(instr);
  }
  fn visit_compare_branch(&mut self, instr: &Instruction) {
    (**self).visit_compare_branch(instr);
  }
  fn visit_test_branch(&mut self, instr: &Instruction) {
    (**self).visit_test_branch(instr);
  }
  fn visit_conditional_branch(&mut self, instr: &Instruction) {
    (**self).visit_conditional_branch(instr);
  }
  fn visit_system(&mut self, instr: &Instruction) {
    (**self).visit_system(instr);
  }
  fn visit_exception(&mut self, instr: &Instruction) {
    (**self).visit_exception(instr);
  }
  fn visit_load_store_pair_post_index(&mut self, instr: &Instruction) {
    (**self).visit_load_store_pair_post_index(instr);
  }
  fn visit_load_store_pair_offset(&mut self, instr: &Instruction) {
    (**self).visit_load_store_pair_offset(instr);
  }
  fn visit_load_store_pair_pre_index(&mut self, instr: &Instruction) {
    (**self).visit_load_store_pair_pre_index(instr);
  }
  fn visit_load_literal(&mut self, instr: &Instruction) {
    (**self).visit_load_literal(instr);
  }
  fn visit_load_store_unscaled_offset(&mut self, instr: &Instruction) {
    (**self).visit_load_store_unscaled_offset(instr);
  }
  fn visit_load_store_post_index(&mut self, instr: &Instruction) {
    (**self).visit_load_store_post_index(instr);
  }
  fn visit_load_store_pre_index(&mut self, instr: &Instruction) {
    (**self).visit_load_store_pre_index(instr);
  }
  fn visit_load_store_register_offset(&mut self, instr: &Instruction) {
    (**self).visit_load_store_register_offset(instr);
  }
  fn visit_load_store_unsigned_offset(&mut self, instr: &Instruction) {
    (**self).visit_load_store_unsigned_offset(instr);
  }
  fn visit_load_store_acquire_release(&mut self, instr: &Instruction) {
    (**self).visit_load_store_acquire_release(instr);
  }
  fn visit_atomic_memory(&mut self, instr: &Instruction) {
    (**self).visit_atomic_memory(instr);
  }
  fn visit_logical_shifted(&mut self, instr: &Instruction) {
    (**self).visit_logical_shifted(instr);
  }
  fn visit_add_sub_shifted(&mut self, instr: &Instruction) {
    (**self).visit_add_sub_shifted(instr);
  }
  fn visit_add_sub_extended(&mut self, instr: &Instruction) {
    (**self).visit_add_sub_extended(instr);
  }
  fn visit_add_sub_with_carry(&mut self, instr: &Instruction) {
    (**self).visit_add_sub_with_carry(instr);
  }
  fn visit_conditional_compare_register(&mut self, instr: &Instruction) {
    (**self).visit_conditional_compare_register(instr);
  }
  fn visit_conditional_compare_immediate(&mut self, instr: &Instruction) {
    (**self).visit_conditional_compare_immediate(instr);
  }
  fn visit_conditional_select(&mut self, instr: &Instruction) {
    (**self).visit_conditional_select(instr);
  }
  fn visit_data_processing1_source(&mut self, instr: &Instruction) {
    (**self).visit_data_processing1_source(instr);
  }
  fn visit_data_processing2_source(&mut self, instr: &Instruction) {
    (**self).visit_data_processing2_source(instr);
  }
  fn visit_data_processing3_source(&mut self, instr: &Instruction) {
    (**self).visit_data_processing3_source(instr);
  }
  fn visit_fpcompare(&mut self, instr: &Instruction) {
    (**self).visit_fpcompare(instr);
  }
  fn visit_fpconditional_compare(&mut self, instr: &Instruction) {
    (**self).visit_fpconditional_compare(instr);
  }
  fn visit_fpconditional_select(&mut self, instr: &Instruction) {
    (**self).visit_fpconditional_select(instr);
  }
  fn visit_fpimmediate(&mut self, instr: &Instruction) {
    (**self).visit_fpimmediate(instr);
  }
  fn visit_fpdata_processing1_source(&mut self, instr: &Instruction) {
    (**self).visit_fpdata_processing1_source(instr);
  }
  fn visit_fpdata_processing2_source(&mut self, instr: &Instruction) {
    (**self).visit_fpdata_processing2_source(instr);
  }
  fn visit_fpdata_processing3_source(&mut self, instr: &Instruction) {
    (**self).visit_fpdata_processing3_source(instr);
  }
  fn visit_fpinteger_convert(&mut self, instr: &Instruction) {
    (**self).visit_fpinteger_convert(instr);
  }
  fn visit_fpfixed_point_convert(&mut self, instr: &Instruction) {
    (**self).visit_fpfixed_point_convert(instr);
  }
  fn visit_neon2_reg_misc(&mut self, instr: &Instruction) {
    (**self).visit_neon2_reg_misc(instr);
  }
  fn visit_neon3_different(&mut self, instr: &Instruction) {
    (**self).visit_neon3_different(instr);
  }
  fn visit_neon3_extension(&mut self, instr: &Instruction) {
    (**self).visit_neon3_extension(instr);
  }
  fn visit_neon3_same(&mut self, instr: &Instruction) {
    (**self).visit_neon3_same(instr);
  }
  fn visit_neon3_same_hp(&mut self, instr: &Instruction) {
    (**self).visit_neon3_same_hp(instr);
  }
  fn visit_neonacross_lanes(&mut self, instr: &Instruction) {
    (**self).visit_neonacross_lanes(instr);
  }
  fn visit_neonby_indexed_element(&mut self, instr: &Instruction) {
    (**self).visit_neonby_indexed_element(instr);
  }
  fn visit_neoncopy(&mut self, instr: &Instruction) {
    (**self).visit_neoncopy(instr);
  }
  fn visit_neonextract(&mut self, instr: &Instruction) {
    (**self).visit_neonextract(instr);
  }
  fn visit_neonload_store_multi_struct(&mut self, instr: &Instruction) {
    (**self).visit_neonload_store_multi_struct(instr);
  }
  fn visit_neonload_store_multi_struct_post_index(&mut self, instr: &Instruction) {
    (**self).visit_neonload_store_multi_struct_post_index(instr);
  }
  fn visit_neonload_store_single_struct(&mut self, instr: &Instruction) {
    (**self).visit_neonload_store_single_struct(instr);
  }
  fn visit_neonload_store_single_struct_post_index(&mut self, instr: &Instruction) {
    (**self).visit_neonload_store_single_struct_post_index(instr);
  }
  fn visit_neonmodified_immediate(&mut self, instr: &Instruction) {
    (**self).visit_neonmodified_immediate(instr);
  }
  fn visit_neonscalar2_reg_misc(&mut self, instr: &Instruction) {
    (**self).visit_neonscalar2_reg_misc(instr);
  }
  fn visit_neonscalar3_diff(&mut self, instr: &Instruction) {
    (**self).visit_neonscalar3_diff(instr);
  }
  fn visit_neonscalar3_same(&mut self, instr: &Instruction) {
    (**self).visit_neonscalar3_same(instr);
  }
  fn visit_neonscalar_by_indexed_element(&mut self, instr: &Instruction) {
    (**self).visit_neonscalar_by_indexed_element(instr);
  }
  fn visit_neonscalar_copy(&mut self, instr: &Instruction) {
    (**self).visit_neonscalar_copy(instr);
  }
  fn visit_neonscalar_pairwise(&mut self, instr: &Instruction) {
    (**self).visit_neonscalar_pairwise(instr);
  }
  fn visit_neonscalar_shift_immediate(&mut self, instr: &Instruction) {
    (**self).visit_neonscalar_shift_immediate(instr);
  }
  fn visit_neonshift_immediate(&mut self, instr: &Instruction) {
    (**self).visit_neonshift_immediate(instr);
  }
  fn visit_neontable(&mut self, instr: &Instruction) {
    (**self).visit_neontable(instr);
  }
  fn visit_neonperm(&mut self, instr: &Instruction) {
    (**self).visit_neonperm(instr);
  }
  fn visit_neonsha3(&mut self, instr: &Instruction) {
    (**self).visit_neonsha3(instr);
  }
  fn visit_cpy(&mut self, instr: &Instruction) {
    (**self).visit_cpy(instr);
  }
  fn visit_set(&mut self, instr: &Instruction) {
    (**self).visit_set(instr);
  }
  fn visit_svebit_perm(&mut self, instr: &Instruction) {
    (**self).visit_svebit_perm(instr);
  }
  fn visit_unallocated(&mut self, instr: &Instruction) {
    (**self).visit_unallocated(instr);
  }
  fn visit_unimplemented(&mut self, instr: &Instruction) {
    (**self).visit_unimplemented(instr);
  }
}

/// 解码树一级分支 (原版 Decoder<V> 私有 Decode* 方法, 按 bits 27:24 分派)
#[rustfmt::skip]
pub const DECODE_TREE_NODES: &[(&str, &str)] = &[
  ("DecodePCRelAddressing", "PCRelAddressing"),
  ("DecodeDataProcessingImmediate", "DataProcessingImmediate"),
  ("DecodeBranchSystemException", "BranchSystemException"),
  ("DecodeLoadStore", "LoadStore"),
  ("DecodeLogical", "Logical"),
  ("DecodeBitfieldExtract", "BitfieldExtract"),
  ("DecodeDataProcessing", "DataProcessing"),
  ("DecodeFP", "FP"),
  ("DecodeNEONLoadStore", "NEONLoadStore"),
  ("DecodeNEONVectorDataProcessing", "NEONVectorDataProcessing"),
  ("DecodeNEONScalarDataProcessing", "NEONScalarDataProcessing"),
];

/// 解码器核心 (参数化 V 访问者)
pub struct Decoder<V: DecoderVisitor> {
  pub visitor: V,
}

impl<V: DecoderVisitor> Decoder<V> {
  pub const fn new(visitor: V) -> Self {
    Self { visitor }
  }

  /// 顶层指令解码分派 (对齐 decoder-arm64-inl.h: Decode)
  pub fn decode(&mut self, instr: &Instruction) {
    if instr.bit_range(28, 27) == 0 {
      if instr.bit_range(31, 24) == 0x45
        && instr.bit(21) == 0
        && instr.bit_range(15, 12) == 0xB
        && instr.bit_range(11, 10) != 3
      {
        self.visitor.visit_svebit_perm(instr);
      } else {
        self.visitor.visit_unallocated(instr);
      }
    } else {
      match instr.bit_range(27, 24) {
        0x0 => self.decode_pcrel_addressing(instr),
        0x1 => self.decode_data_processing_immediate(instr),
        0xA | 0xB => self.decode_data_processing(instr),
        0x2 => self.decode_logical(instr),
        0x3 => self.decode_bitfield_extract(instr),
        0x4..=0x7 => self.decode_branch_system_exception(instr),
        0x8 | 0x9 | 0xC | 0xD => self.decode_load_store(instr),
        0xE | 0xF => self.decode_fp(instr),
        _ => self.visitor.visit_unallocated(instr),
      }
    }
  }

  pub fn decode_pcrel_addressing(&mut self, instr: &Instruction) {
    debug_assert_eq!(0x0, instr.bit_range(27, 24));
    self.visitor.visit_pcrel_addressing(instr);
  }

  pub fn decode_data_processing_immediate(&mut self, instr: &Instruction) {
    debug_assert_eq!(instr.bit(28), 1);
    debug_assert_eq!(0x1, instr.bit_range(27, 24));
    if instr.bit_range(23, 22) == 2 {
      self.visitor.visit_unallocated(instr);
    } else if instr.bit(23) == 1 {
      if instr.bit_range(30, 29) != 0 || instr.bit_range(21, 20) != 0 {
        self.visitor.visit_unallocated(instr);
      } else {
        self.visitor.visit_min_max_immediate(instr);
      }
    } else {
      self.visitor.visit_add_sub_immediate(instr);
    }
  }

  pub fn decode_logical(&mut self, instr: &Instruction) {
    if instr.bit(23) == 1 {
      self.visitor.visit_unallocated(instr);
    } else {
      self.visitor.visit_logical_immediate(instr);
    }
  }

  pub fn decode_bitfield_extract(&mut self, instr: &Instruction) {
    if instr.bit(23) == 0 {
      self.visitor.visit_bitfield(instr);
    } else {
      self.visitor.visit_extract(instr);
    }
  }

  pub fn decode_branch_system_exception(&mut self, instr: &Instruction) {
    match instr.bit_range(31, 29) {
      0 | 4 => self.visitor.visit_unconditional_branch(instr),
      1 | 5 => {
        if instr.bit(25) == 0 {
          self.visitor.visit_compare_branch(instr);
        } else {
          self.visitor.visit_test_branch(instr);
        }
      }
      2 => {
        if instr.bit(25) == 0 && instr.bit(24) == 0 {
          self.visitor.visit_conditional_branch(instr);
        } else {
          self.visitor.visit_unallocated(instr);
        }
      }
      6 => {
        if instr.bit(25) == 0 {
          if instr.bit(24) == 0 {
            if instr.bit_range(4, 2) != 0
              || instr.mask(0x00E0001D) == 0x00200001
              || instr.mask(0x00E0001D) == 0x00400001
              || instr.mask(0x00E0001E) == 0x00200002
              || instr.mask(0x00E0001E) == 0x00400002
              || instr.mask(0x00E0001C) == 0x00600000
              || instr.mask(0x00E0001C) == 0x00800000
              || instr.mask(0x00E0001F) == 0x00A00000
              || instr.mask(0x00C0001C) == 0x00C00000
            {
              self.visitor.visit_unallocated(instr);
            } else {
              self.visitor.visit_exception(instr);
            }
          } else if instr.bit_range(23, 22) == 0 {
            let masked = instr.mask(0x003FF0E0);
            if instr.bit_range(21, 19) == 0x4
              || masked == 0x00033000
              || masked == 0x003FF020
              || masked == 0x003FF060
              || masked == 0x003FF0E0
              || instr.mask(0x00388000) == 0x00008000
              || instr.mask(0x0038E000) == 0x00000000
              || instr.mask(0x0039E000) == 0x00002000
              || instr.mask(0x003AE000) == 0x00002000
              || instr.mask(0x003CE000) == 0x00042000
              || instr.mask(0x0038F000) == 0x00005000
              || instr.mask(0x0038E000) == 0x00006000
            {
              self.visitor.visit_unallocated(instr);
            } else {
              self.visitor.visit_system(instr);
            }
          } else {
            self.visitor.visit_unallocated(instr);
          }
        } else if instr.bit(24) == 1
          || instr.bit_range(20, 16) != 0x1F
          || instr.bit_range(15, 10) != 0
          || instr.bit_range(4, 0) != 0
          || instr.bit_range(24, 21) == 0x3
          || instr.bit_range(24, 22) == 0x3
        {
          self.visitor.visit_unallocated(instr);
        } else {
          self.visitor.visit_unconditional_branch_to_register(instr);
        }
      }
      _ => self.visitor.visit_unallocated(instr),
    }
  }

  pub fn decode_load_store(&mut self, instr: &Instruction) {
    if instr.bit(28) == 0 && instr.bit(29) == 0 && instr.bit(26) == 1 {
      self.decode_neon_load_store(instr);
      return;
    }

    if instr.bit(24) == 0 {
      if instr.bit(28) == 0 {
        if instr.bit(29) == 0 {
          if instr.bit(26) == 0 {
            if instr.mask(0xA08000) == 0x800000 {
              self.visitor.visit_unallocated(instr);
            } else if instr.mask(0xA08000) == 0 {
              self.visitor.visit_unimplemented(instr);
            } else {
              self.visitor.visit_load_store_acquire_release(instr);
            }
          } else {
            self.decode_neon_load_store(instr);
          }
        } else if instr.bit(23) == 0 {
          self.visitor.visit_load_store_pair_post_index(instr);
        } else {
          self.visitor.visit_load_store_pair_offset(instr);
        }
      } else if instr.bit(29) == 0 {
        self.visitor.visit_load_literal(instr);
      } else if instr.bit(21) == 0 {
        match instr.bit_range(11, 10) {
          0 => self.visitor.visit_load_store_unscaled_offset(instr),
          1 => self.visitor.visit_load_store_post_index(instr),
          3 => self.visitor.visit_load_store_pre_index(instr),
          _ => self.visitor.visit_unimplemented(instr),
        }
      } else if instr.bit_range(11, 10) == 2 {
        self.visitor.visit_load_store_register_offset(instr);
      } else {
        self.visitor.visit_unallocated(instr);
      }
    } else if instr.bit(28) == 0 {
      if instr.bit(29) == 0 {
        self.visitor.visit_unallocated(instr);
      } else if instr.bit(23) == 0 {
        self.visitor.visit_load_store_pair_pre_index(instr);
      } else {
        self.visitor.visit_load_store_pair_offset(instr);
      }
    } else if instr.bit(29) == 0 {
      self.visitor.visit_unallocated(instr);
    } else {
      self.visitor.visit_load_store_unsigned_offset(instr);
    }
  }

  pub fn decode_data_processing(&mut self, instr: &Instruction) {
    if instr.bit(24) == 0 {
      if instr.bit(28) == 0 {
        if instr.mask(0x80008000) == 0x00008000 {
          self.visitor.visit_unallocated(instr);
        } else {
          self.visitor.visit_logical_shifted(instr);
        }
      } else {
        match instr.bit_range(23, 21) {
          0 => self.visitor.visit_add_sub_with_carry(instr),
          2 => {
            if instr.bit(11) == 0 {
              self.visitor.visit_conditional_compare_register(instr);
            } else {
              self.visitor.visit_conditional_compare_immediate(instr);
            }
          }
          4 => {
            if instr.mask(0x20000000) != 0
              || instr.mask(0x00000800) != 0
              || instr.mask(0x00000010) != 0
            {
              self.visitor.visit_unallocated(instr);
            } else {
              self.visitor.visit_conditional_select(instr);
            }
          }
          6 => {
            if instr.bit(30) == 1 {
              self.visitor.visit_unallocated(instr);
            } else if instr.bit(11) == 0 {
              self.visitor.visit_data_processing2_source(instr);
            } else {
              self.visitor.visit_data_processing1_source(instr);
            }
          }
          _ => self.visitor.visit_unallocated(instr),
        }
      }
    } else if instr.bit(28) == 0 {
      if instr.bit(21) == 0 {
        self.visitor.visit_add_sub_shifted(instr);
      } else if instr.bit(23) == 0 {
        self.visitor.visit_add_sub_extended(instr);
      } else {
        self.visitor.visit_unallocated(instr);
      }
    } else {
      self.visitor.visit_data_processing3_source(instr);
    }
  }

  pub fn decode_fp(&mut self, instr: &Instruction) {
    if instr.bit(28) == 0 {
      self.decode_neon_vector_data_processing(instr);
    } else if instr.bit_range(31, 30) == 0x3 {
      self.visitor.visit_unallocated(instr);
    } else if instr.bit_range(31, 30) == 0x1 {
      self.decode_neon_scalar_data_processing(instr);
    } else if instr.bit(29) == 0 {
      if instr.bit(24) == 0 {
        if instr.bit(21) == 0 {
          self.visitor.visit_fpfixed_point_convert(instr);
        } else if instr.bit_range(15, 10) == 0 {
          self.visitor.visit_fpinteger_convert(instr);
        } else {
          self.visitor.visit_fpdata_processing1_source(instr);
        }
      } else if instr.bit(21) == 0 {
        self.visitor.visit_fpcompare(instr);
      } else {
        self.visitor.visit_fpimmediate(instr);
      }
    } else if instr.bit(24) == 0 {
      self.visitor.visit_fpdata_processing2_source(instr);
    } else if instr.bit(21) == 0 {
      if instr.bit(11) == 0 {
        self.visitor.visit_fpconditional_compare(instr);
      } else {
        self.visitor.visit_fpconditional_select(instr);
      }
    } else {
      self.visitor.visit_fpdata_processing3_source(instr);
    }
  }

  pub fn decode_neon_load_store(&mut self, instr: &Instruction) {
    if instr.bit(24) == 0 {
      if instr.bit(21) == 0 {
        self.visitor.visit_neonload_store_multi_struct(instr);
      } else {
        self.visitor.visit_neonload_store_single_struct(instr);
      }
    } else if instr.bit(23) == 0 {
      if instr.bit(21) == 0 {
        self
          .visitor
          .visit_neonload_store_multi_struct_post_index(instr);
      } else {
        self
          .visitor
          .visit_neonload_store_single_struct_post_index(instr);
      }
    } else {
      self.visitor.visit_unallocated(instr);
    }
  }

  pub fn decode_neon_vector_data_processing(&mut self, instr: &Instruction) {
    if instr.bit(23) == 1 {
      self.visitor.visit_neonextract(instr);
    } else if instr.bit(24) == 0 {
      if instr.bit(21) == 0 {
        self.visitor.visit_neon3_same(instr);
      } else if instr.bit(19) == 0 {
        self.visitor.visit_neon2_reg_misc(instr);
      } else {
        self.visitor.visit_neoncopy(instr);
      }
    } else if instr.bit(21) == 0 {
      self.visitor.visit_neon3_different(instr);
    } else if instr.bit(15) == 0 {
      self.visitor.visit_neonshift_immediate(instr);
    } else {
      self.visitor.visit_neonby_indexed_element(instr);
    }
  }

  pub fn decode_neon_scalar_data_processing(&mut self, instr: &Instruction) {
    if instr.bit(23) == 1 {
      self.visitor.visit_unallocated(instr);
    } else if instr.bit(24) == 0 {
      if instr.bit(21) == 0 {
        self.visitor.visit_neonscalar3_same(instr);
      } else if instr.bit(19) == 0 {
        self.visitor.visit_neonscalar2_reg_misc(instr);
      } else {
        self.visitor.visit_neonscalar_copy(instr);
      }
    } else if instr.bit(21) == 0 {
      self.visitor.visit_neonscalar3_diff(instr);
    } else if instr.bit(15) == 0 {
      self.visitor.visit_neonscalar_shift_immediate(instr);
    } else {
      self.visitor.visit_neonscalar_by_indexed_element(instr);
    }
  }
}

/// 原版 Decoder<V>::Decode — 顶层分派函数
pub fn decode(instr: &Instruction) {
  struct NullVisitor;
  impl DecoderVisitor for NullVisitor {}
  let mut v = NullVisitor;
  Decoder::new(&mut v).decode(instr);
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(VISITOR_LIST.len() == 74);
  assert!(DECODE_TREE_NODES.len() == 11);
};
