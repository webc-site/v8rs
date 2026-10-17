#!/usr/bin/env bun
// tool/src/codegen/csa/code-assembler.js
// 机械提取并生成 CodeAssembler 核心算子与汇编图 DSL
// 来源: v8/src/compiler/code-assembler.{h,cc}
// 产物: v8-codegen-csa/src/code_assembler.rs

import { spawnSync } from "node:child_process";
import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/compiler/code-assembler.h");

// 1. 提取比较二元算子
const cmpM = hSrc.match(/#define CODE_ASSEMBLER_COMPARE_BINARY_OP_LIST\(V\)\s*([\s\S]*?)(?=#define CODE_ASSEMBLER_BINARY_OP_LIST)/);
if (!cmpM) throw new Error("未找到 CODE_ASSEMBLER_COMPARE_BINARY_OP_LIST");
const cmpOps = [...cmpM[1].matchAll(/V\((\w+),\s*([^,]+),\s*([^,]+),\s*([^)]+)\)/g)].map(
  (m) => ({ op: m[1], res: m[2].trim(), t1: m[3].trim(), t2: m[4].trim() })
);
if (cmpOps.length !== 31) throw new Error(`比较算子数量不符: ${cmpOps.length} != 31`);

// 2. 提取二元算子
const binM = hSrc.match(/#define CODE_ASSEMBLER_BINARY_OP_LIST\(V\)\s*([\s\S]*?)(?=#define CODE_ASSEMBLER_UNARY_OP_LIST)/);
if (!binM) throw new Error("未找到 CODE_ASSEMBLER_BINARY_OP_LIST");
const binOps = [...binM[1].matchAll(/V\((\w+),\s*([^,]+),\s*([^,]+),\s*([^)]+)\)/g)].map(
  (m) => ({ op: m[1], res: m[2].trim(), t1: m[3].trim(), t2: m[4].trim() })
);
if (binOps.length !== 65) throw new Error(`二元算子数量不符: ${binOps.length} != 65`);

// 3. 提取一元算子
const unM = hSrc.match(/#define CODE_ASSEMBLER_UNARY_OP_LIST\(V\)\s*([\s\S]*?)(?=#undef PAIR_TYPE|\n\/\/ A "public" interface)/);
if (!unM) throw new Error("未找到 CODE_ASSEMBLER_UNARY_OP_LIST");
const unOps = [...unM[1].matchAll(/V\((\w+),\s*([^,]+),\s*([^)]+)\)/g)].map(
  (m) => ({ op: m[1], res: m[2].trim(), t1: m[3].trim() })
);
if (unOps.length !== 71) throw new Error(`一元算子数量不符: ${unOps.length} != 71`);

function toSnake(s) {
  return s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
}

let rust = banner(
  "tool/src/codegen/csa/code-assembler.js",
  "v8/src/compiler/code-assembler.{h,cc}",
  "CodeAssembler 汇编器状态机与算子元数据体系"
);

rust += `
use crate::error::{Error, Result};
use crate::tnode::{BoolT, Float64T, Int32T, Int64T, IntPtrT, NodeId, Object, Smi, TNode, Uint32T, Word32T, WordT};
use strum::{Display, EnumString};
pub use v8_builtins_core::BuiltinKind;
pub use v8_codegen_core::CallInterfaceDescriptor;

// -----------------------------------------------------------------------------
// 1. 比较二元算子 (CompareBinaryOp: 31 项)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum CompareBinaryOp {
${cmpOps.map((c) => `  /// ${c.op}(${c.t1}, ${c.t2}) -> ${c.res}\n  ${c.op},`).join("\n")}
}

impl CompareBinaryOp {
  /// 比较算子总数 (31 项)
  pub const COUNT: usize = 31;

  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match self {
${cmpOps.map((c) => `      Self::${c.op} => "${c.op}",`).join("\n")}
    }
  }

  #[inline]
  #[must_use]
  pub const fn is_float_compare(&self) -> bool {
    matches!(
      self,
      Self::Float32Equal
        | Self::Float32LessThan
        | Self::Float32LessThanOrEqual
        | Self::Float32GreaterThan
        | Self::Float32GreaterThanOrEqual
        | Self::Float64Equal
        | Self::Float64NotEqual
        | Self::Float64LessThan
        | Self::Float64LessThanOrEqual
        | Self::Float64GreaterThan
        | Self::Float64GreaterThanOrEqual
    )
  }

  #[inline]
  #[must_use]
  pub const fn is_signed_int_compare(&self) -> bool {
    matches!(
      self,
      Self::Int32GreaterThan
        | Self::Int32GreaterThanOrEqual
        | Self::Int32LessThan
        | Self::Int32LessThanOrEqual
        | Self::IntPtrLessThan
        | Self::IntPtrLessThanOrEqual
        | Self::IntPtrGreaterThan
        | Self::IntPtrGreaterThanOrEqual
    )
  }

  #[inline]
  #[must_use]
  pub const fn is_unsigned_int_compare(&self) -> bool {
    matches!(
      self,
      Self::Uint32LessThan
        | Self::Uint32LessThanOrEqual
        | Self::Uint32GreaterThan
        | Self::Uint32GreaterThanOrEqual
        | Self::Uint64LessThan
        | Self::Uint64LessThanOrEqual
        | Self::Uint64GreaterThan
        | Self::Uint64GreaterThanOrEqual
        | Self::UintPtrLessThan
        | Self::UintPtrLessThanOrEqual
        | Self::UintPtrGreaterThan
        | Self::UintPtrGreaterThanOrEqual
    )
  }
}

// -----------------------------------------------------------------------------
// 2. 二元算子 (BinaryOp: 65 项)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum BinaryOp {
${binOps.map((b) => `  /// ${b.op}(${b.t1}, ${b.t2}) -> ${b.res}\n  ${b.op},`).join("\n")}
}

impl BinaryOp {
  /// 二元算子总数 (65 项)
  pub const COUNT: usize = 65;

  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match self {
${binOps.map((b) => `      Self::${b.op} => "${b.op}",`).join("\n")}
    }
  }

  #[inline]
  #[must_use]
  pub const fn is_bitwise(&self) -> bool {
    matches!(
      self,
      Self::WordOr
        | Self::WordAnd
        | Self::WordXor
        | Self::WordRor
        | Self::WordShl
        | Self::WordShr
        | Self::WordSar
        | Self::WordSarShiftOutZeros
        | Self::Word32Or
        | Self::Word32And
        | Self::Word32Xor
        | Self::Word32Ror
    )
  }

  #[inline]
  #[must_use]
  pub const fn is_arithmetic(&self) -> bool {
    matches!(
      self,
      Self::IntPtrAdd
        | Self::IntPtrSub
        | Self::IntPtrMul
        | Self::Int32Add
        | Self::Int32Sub
        | Self::Int32Mul
        | Self::Int64Add
        | Self::Int64Sub
        | Self::Int64Mul
        | Self::Float64Add
        | Self::Float64Sub
        | Self::Float64Mul
        | Self::Float64Div
    )
  }
}

// -----------------------------------------------------------------------------
// 3. 一元算子 (UnaryOp: 71 项)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum UnaryOp {
${unOps.map((u) => `  /// ${u.op}(${u.t1}) -> ${u.res}\n  ${u.op},`).join("\n")}
}

impl UnaryOp {
  /// 一元算子总数 (71 项)
  pub const COUNT: usize = 71;

  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match self {
${unOps.map((u) => `      Self::${u.op} => "${u.op}",`).join("\n")}
    }
  }

  #[inline]
  #[must_use]
  pub const fn is_bitcast(&self) -> bool {
    matches!(
      self,
      Self::BitcastTaggedToWord
        | Self::BitcastTaggedToWordForTagAndSmiBits
        | Self::BitcastMaybeObjectToWord
        | Self::BitcastWordToTagged
        | Self::BitcastWordToTaggedSigned
        | Self::BitcastInt32ToFloat32
        | Self::BitcastFloat32ToInt32
        | Self::BitcastFloat64ToInt64
        | Self::BitcastInt64ToFloat64
    )
  }

  #[inline]
  #[must_use]
  pub const fn is_truncate_or_change(&self) -> bool {
    matches!(
      self,
      Self::TruncateFloat64ToFloat32
        | Self::TruncateFloat64ToFloat16RawBits
        | Self::TruncateFloat64ToWord32
        | Self::TruncateInt64ToInt32
        | Self::ChangeFloat32ToFloat64
        | Self::ChangeFloat16RawBitsToFloat64
        | Self::ChangeFloat64ToUint32
        | Self::ChangeFloat64ToUint64
        | Self::ChangeFloat64ToInt64
        | Self::ChangeInt32ToFloat64
        | Self::ChangeInt32ToInt64
        | Self::ChangeUint32ToFloat64
        | Self::ChangeUint32ToUint64
        | Self::ChangeInt64ToFloat64
    )
  }
}

// -----------------------------------------------------------------------------
// 4. CodeAssemblerLabel 状态与控制流标记
// -----------------------------------------------------------------------------

/// 跳转标号状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelState {
  /// 尚未绑定代码位置
  Unbound,
  /// 已绑定当前代码块
  Bound,
  /// 已与其他控制流路径合并
  Merged,
}

/// 代码存根汇编跳转标号
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAssemblerLabel {
  id: u32,
  name: &'static str,
  state: LabelState,
  merged_count: usize,
}

impl CodeAssemblerLabel {
  /// 创建未绑定标号
  #[inline]
  #[must_use]
  pub const fn new(id: u32, name: &'static str) -> Self {
    Self {
      id,
      name,
      state: LabelState::Unbound,
      merged_count: 0,
    }
  }

  #[inline]
  #[must_use]
  pub const fn id(&self) -> u32 {
    self.id
  }

  #[inline]
  #[must_use]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  #[inline]
  #[must_use]
  pub const fn is_bound(&self) -> bool {
    matches!(self.state, LabelState::Bound | LabelState::Merged)
  }

  #[inline]
  #[must_use]
  pub const fn merged_count(&self) -> usize {
    self.merged_count
  }

  /// 绑定该标号
  #[inline]
  pub fn bind(&mut self) -> Result<()> {
    if self.state == LabelState::Bound {
      return Err(Error::LabelAlreadyBound(self.name));
    }
    self.state = LabelState::Bound;
    Ok(())
  }

  /// 记录合并跳转流入
  #[inline]
  pub fn record_merge(&mut self) {
    self.merged_count = self.merged_count.saturating_add(1);
    if self.state == LabelState::Bound {
      self.state = LabelState::Merged;
    }
  }
}

// -----------------------------------------------------------------------------
// 5. SSA 变量与图节点记录
// -----------------------------------------------------------------------------

/// 机器表示
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MachineRep {
  Word32,
  Word64,
  Float32,
  Float64,
  Tagged,
  Pointer,
}

/// 局部汇编变量 (支持跨分支值赋值与 Phi 合并)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeAssemblerVariable {
  id: u32,
  rep: MachineRep,
  val: NodeId,
}

impl CodeAssemblerVariable {
  #[inline]
  #[must_use]
  pub const fn new(id: u32, rep: MachineRep) -> Self {
    Self {
      id,
      rep,
      val: NodeId::INVALID,
    }
  }

  #[inline]
  #[must_use]
  pub const fn id(&self) -> u32 {
    self.id
  }

  #[inline]
  #[must_use]
  pub const fn representation(&self) -> MachineRep {
    self.rep
  }

  #[inline]
  #[must_use]
  pub const fn value(&self) -> NodeId {
    self.val
  }

  #[inline]
  pub fn set(&mut self, val: NodeId) {
    self.val = val;
  }
}

/// 图节点记录种类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeOp {
  ConstantInt32(i32),
  ConstantInt64(i64),
  ConstantFloat64(u64), // f64 bits
  ConstantBool(bool),
  Compare(CompareBinaryOp),
  Binary(BinaryOp),
  Unary(UnaryOp),
  Load,
  Store,
  CallRuntime(u32),
  CallBuiltin(u32, BuiltinKind),
  Phi,
}

/// 构图中间表示节点条目
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeRecord {
  pub id: NodeId,
  pub op: NodeOp,
  pub inputs: [NodeId; 3],
  pub input_count: u8,
}

// -----------------------------------------------------------------------------
// 6. CodeAssemblerState 汇编图状态管理
// -----------------------------------------------------------------------------

/// 汇编器状态上下文
#[derive(Debug, Default)]
pub struct CodeAssemblerState {
  next_node_id: u32,
  next_label_id: u32,
  next_var_id: u32,
  nodes: Vec<NodeRecord>,
  current_block: u32,
}

impl CodeAssemblerState {
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    Self {
      next_node_id: 1,
      next_label_id: 1,
      next_var_id: 1,
      nodes: Vec::new(),
      current_block: 0,
    }
  }

  #[inline]
  #[must_use]
  pub const fn current_block(&self) -> u32 {
    self.current_block
  }

  #[inline]
  pub fn alloc_node_id(&mut self) -> NodeId {
    let id = self.next_node_id;
    self.next_node_id = self.next_node_id.saturating_add(1);
    NodeId(id)
  }

  #[inline]
  pub fn alloc_label(&mut self, name: &'static str) -> CodeAssemblerLabel {
    let id = self.next_label_id;
    self.next_label_id = self.next_label_id.saturating_add(1);
    CodeAssemblerLabel::new(id, name)
  }

  #[inline]
  pub fn alloc_variable(&mut self, rep: MachineRep) -> CodeAssemblerVariable {
    let id = self.next_var_id;
    self.next_var_id = self.next_var_id.saturating_add(1);
    CodeAssemblerVariable::new(id, rep)
  }

  #[inline]
  pub fn emit(&mut self, op: NodeOp, inputs: &[NodeId]) -> NodeId {
    let id = self.alloc_node_id();
    let mut in_arr = [NodeId::INVALID; 3];
    let count = inputs.len().min(3) as u8;
    for (i, &inp) in inputs.iter().take(3).enumerate() {
      in_arr[i] = inp;
    }
    self.nodes.push(NodeRecord {
      id,
      op,
      inputs: in_arr,
      input_count: count,
    });
    id
  }

  #[inline]
  #[must_use]
  pub fn node_count(&self) -> usize {
    self.nodes.len()
  }

  #[inline]
  #[must_use]
  pub fn nodes(&self) -> &[NodeRecord] {
    &self.nodes
  }
}

// -----------------------------------------------------------------------------
// 7. CodeAssembler 核心图构造 DSL
// -----------------------------------------------------------------------------

/// 基础图构建与控制流汇编器
pub struct CodeAssembler<'a> {
  pub(crate) state: &'a mut CodeAssemblerState,
}

impl<'a> CodeAssembler<'a> {
  #[inline]
  #[must_use]
  pub fn new(state: &'a mut CodeAssemblerState) -> Self {
    Self { state }
  }

  #[inline]
  pub fn alloc_label(&mut self, name: &'static str) -> CodeAssemblerLabel {
    self.state.alloc_label(name)
  }

  #[inline]
  pub fn alloc_variable(&mut self, rep: MachineRep) -> CodeAssemblerVariable {
    self.state.alloc_variable(rep)
  }

  // --- 常量生成 ---

  #[inline]
  pub fn int32_constant(&mut self, val: i32) -> TNode<Int32T> {
    let id = self.state.emit(NodeOp::ConstantInt32(val), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn int64_constant(&mut self, val: i64) -> TNode<Int64T> {
    let id = self.state.emit(NodeOp::ConstantInt64(val), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn intptr_constant(&mut self, val: isize) -> TNode<IntPtrT> {
    let id = self.state.emit(NodeOp::ConstantInt64(val as i64), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn uintptr_constant(&mut self, val: usize) -> TNode<Uint32T> {
    let id = self.state.emit(NodeOp::ConstantInt32(val as i32), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn float64_constant(&mut self, val: f64) -> TNode<Float64T> {
    let id = self.state.emit(NodeOp::ConstantFloat64(val.to_bits()), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn bool_constant(&mut self, val: bool) -> TNode<BoolT> {
    let id = self.state.emit(NodeOp::ConstantBool(val), &[]);
    TNode::new(id)
  }

  #[inline]
  pub fn smi_constant(&mut self, val: i32) -> TNode<Smi> {
    let id = self.state.emit(NodeOp::ConstantInt32(val), &[]);
    TNode::new(id)
  }

  // --- 比较运算 ---

  #[inline]
  pub fn compare_binary_op(
    &mut self,
    op: CompareBinaryOp,
    lhs: NodeId,
    rhs: NodeId,
  ) -> TNode<BoolT> {
    let id = self.state.emit(NodeOp::Compare(op), &[lhs, rhs]);
    TNode::new(id)
  }

  #[inline]
  pub fn word_equal(&mut self, lhs: TNode<WordT>, rhs: TNode<WordT>) -> TNode<BoolT> {
    self.compare_binary_op(CompareBinaryOp::IntPtrLessThan, lhs.node_id(), rhs.node_id())
  }

  #[inline]
  pub fn word32_equal(&mut self, lhs: TNode<BoolT>, rhs: TNode<BoolT>) -> TNode<BoolT> {
    self.compare_binary_op(CompareBinaryOp::Int32LessThan, lhs.node_id(), rhs.node_id())
  }

  #[inline]
  pub fn int32_less_than(&mut self, lhs: TNode<Int32T>, rhs: TNode<Int32T>) -> TNode<BoolT> {
    self.compare_binary_op(CompareBinaryOp::Int32LessThan, lhs.node_id(), rhs.node_id())
  }

  #[inline]
  pub fn int32_greater_than(&mut self, lhs: TNode<Int32T>, rhs: TNode<Int32T>) -> TNode<BoolT> {
    self.compare_binary_op(CompareBinaryOp::Int32GreaterThan, lhs.node_id(), rhs.node_id())
  }

  // --- 二元运算 ---

  #[inline]
  pub fn binary_op(&mut self, op: BinaryOp, lhs: NodeId, rhs: NodeId) -> NodeId {
    self.state.emit(NodeOp::Binary(op), &[lhs, rhs])
  }

  #[inline]
  pub fn int32_add(&mut self, lhs: TNode<Int32T>, rhs: TNode<Int32T>) -> TNode<Int32T> {
    let id = self.binary_op(BinaryOp::Int32Add, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn int32_sub(&mut self, lhs: TNode<Int32T>, rhs: TNode<Int32T>) -> TNode<Int32T> {
    let id = self.binary_op(BinaryOp::Int32Sub, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn int32_mul(&mut self, lhs: TNode<Int32T>, rhs: TNode<Int32T>) -> TNode<Int32T> {
    let id = self.binary_op(BinaryOp::Int32Mul, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn intptr_add(&mut self, lhs: TNode<IntPtrT>, rhs: TNode<IntPtrT>) -> TNode<IntPtrT> {
    let id = self.binary_op(BinaryOp::IntPtrAdd, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn intptr_sub(&mut self, lhs: TNode<IntPtrT>, rhs: TNode<IntPtrT>) -> TNode<IntPtrT> {
    let id = self.binary_op(BinaryOp::IntPtrSub, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn word_shl(&mut self, lhs: TNode<WordT>, shift: TNode<Word32T>) -> TNode<WordT> {
    let id = self.binary_op(BinaryOp::WordShl, lhs.node_id(), shift.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn word_sar(&mut self, lhs: TNode<WordT>, shift: TNode<Word32T>) -> TNode<WordT> {
    let id = self.binary_op(BinaryOp::WordSar, lhs.node_id(), shift.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn word_and(&mut self, lhs: TNode<WordT>, rhs: TNode<WordT>) -> TNode<WordT> {
    let id = self.binary_op(BinaryOp::WordAnd, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn word_or(&mut self, lhs: TNode<WordT>, rhs: TNode<WordT>) -> TNode<WordT> {
    let id = self.binary_op(BinaryOp::WordOr, lhs.node_id(), rhs.node_id());
    TNode::new(id)
  }

  // --- 一元运算 ---

  #[inline]
  pub fn unary_op(&mut self, op: UnaryOp, input: NodeId) -> NodeId {
    self.state.emit(NodeOp::Unary(op), &[input])
  }

  #[inline]
  pub fn word_not(&mut self, val: TNode<WordT>) -> TNode<WordT> {
    let id = self.unary_op(UnaryOp::WordNot, val.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn bitcast_word_to_tagged(&mut self, val: TNode<WordT>) -> TNode<Object> {
    let id = self.unary_op(UnaryOp::BitcastWordToTagged, val.node_id());
    TNode::new(id)
  }

  #[inline]
  pub fn bitcast_tagged_to_word(&mut self, val: TNode<Object>) -> TNode<IntPtrT> {
    let id = self.unary_op(UnaryOp::BitcastTaggedToWord, val.node_id());
    TNode::new(id)
  }

  // --- 控制流 ---

  #[inline]
  pub fn bind(&mut self, label: &mut CodeAssemblerLabel) -> Result<()> {
    label.bind()
  }

  #[inline]
  pub fn goto(&mut self, label: &mut CodeAssemblerLabel) {
    label.record_merge();
  }

  #[inline]
  pub fn goto_if(&mut self, _cond: TNode<BoolT>, label: &mut CodeAssemblerLabel) {
    label.record_merge();
  }

  #[inline]
  pub fn goto_if_not(&mut self, _cond: TNode<BoolT>, label: &mut CodeAssemblerLabel) {
    label.record_merge();
  }

  #[inline]
  pub fn branch(
    &mut self,
    _cond: TNode<BoolT>,
    true_label: &mut CodeAssemblerLabel,
    false_label: &mut CodeAssemblerLabel,
  ) {
    true_label.record_merge();
    false_label.record_merge();
  }

  // --- 调用 ---

  #[inline]
  pub fn call_runtime(&mut self, function_id: u32, args: &[NodeId]) -> NodeId {
    let id = self.state.alloc_node_id();
    let mut in_arr = [NodeId::INVALID; 3];
    let count = args.len().min(3) as u8;
    for (i, &arg) in args.iter().take(3).enumerate() {
      in_arr[i] = arg;
    }
    self.state.nodes.push(NodeRecord {
      id,
      op: NodeOp::CallRuntime(function_id),
      inputs: in_arr,
      input_count: count,
    });
    id
  }

  #[inline]
  pub fn call_builtin(&mut self, builtin_id: u32, kind: BuiltinKind, args: &[NodeId]) -> NodeId {
    let id = self.state.alloc_node_id();
    let mut in_arr = [NodeId::INVALID; 3];
    let count = args.len().min(3) as u8;
    for (i, &arg) in args.iter().take(3).enumerate() {
      in_arr[i] = arg;
    }
    self.state.nodes.push(NodeRecord {
      id,
      op: NodeOp::CallBuiltin(builtin_id, kind),
      inputs: in_arr,
      input_count: count,
    });
    id
  }

  #[inline]
  pub fn call_stub(&mut self, descriptor: &CallInterfaceDescriptor, args: &[NodeId]) -> NodeId {
    let id = self.state.alloc_node_id();
    let mut in_arr = [NodeId::INVALID; 3];
    let count = args.len().min(3) as u8;
    for (i, &arg) in args.iter().take(3).enumerate() {
      in_arr[i] = arg;
    }
    self.state.nodes.push(NodeRecord {
      id,
      op: NodeOp::CallRuntime(descriptor.param_count() as u32),
      inputs: in_arr,
      input_count: count,
    });
    id
  }

  #[inline]
  #[must_use]
  pub fn node_count(&self) -> usize {
    self.state.node_count()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CompareBinaryOp::COUNT == 31);
  assert!(BinaryOp::COUNT == 65);
  assert!(UnaryOp::COUNT == 71);
};
`;

const p = writeSrc("v8-codegen-csa", "code_assembler.rs", rust);
spawnSync("rustfmt", [p], { encoding: "utf8" });
console.log(`written ${p} (CodeAssembler 算子与汇编图 DSL 落地)`);
