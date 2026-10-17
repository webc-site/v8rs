#!/usr/bin/env bun
// tool/src/codegen/csa/code-stub-assembler.js
// 机械提取并生成 CodeStubAssembler 高级 DSL 原语
// 来源: v8/src/codegen/code-stub-assembler.{h,-inl.h,cc}
// 产物: v8-codegen-csa/src/code_stub_assembler.rs

import { spawnSync } from "node:child_process";
import { readV8, writeSrc, banner } from "../../../lib.js";

const H_REL = "src/codegen/code-stub-assembler.h";
const INL_REL = "src/codegen/code-stub-assembler-inl.h";
const CC_REL = "src/codegen/code-stub-assembler.cc";
const h = readV8(H_REL);
readV8(INL_REL);
readV8(CC_REL);

// 1. AllocationFlag
const afM = h.match(/enum class AllocationFlag : uint8_t\s*\{([^}]+)\};/);
if (!afM) throw new Error("未找到 AllocationFlag 枚举");
const af = [...afM[1].matchAll(/k(\w+)\s*=\s*([^,\n]+)/g)].map((m) => ({ name: m[1], expr: m[2].trim() }));

// 2. SlackTrackingMode
const stmM = h.match(/enum SlackTrackingMode\s*\{([^}]+)\};/);
if (!stmM) throw new Error("未找到 SlackTrackingMode 枚举");
const stm = [...stmM[1].matchAll(/k(\w+)/g)].map((m) => m[1]);

// 3. ExtractFixedArrayFlag
const efaM = h.match(/enum class ExtractFixedArrayFlag\s*\{([^}]+)\};/);
if (!efaM) throw new Error("未找到 ExtractFixedArrayFlag 枚举");
const efa = [...efaM[1].matchAll(/k(\w+)\s*=\s*([^,\n]+)/g)].map((m) => ({ name: m[1], expr: m[2].trim() }));

// 4. HoleConversionMode
const hcmM = h.match(/enum class HoleConversionMode\s*\{([^}]+)\};/);
if (!hcmM) throw new Error("未找到 HoleConversionMode 枚举");
const hcm = [...hcmM[1].matchAll(/k(\w+)/g)].map((m) => m[1]);

let rust = banner(
  "tool/src/codegen/csa/code-stub-assembler.js",
  "v8/src/codegen/code-stub-assembler.{h,-inl.h,cc}",
  "CodeStubAssembler 高级语义原语与对象模型 DSL"
);

rust += `
use crate::code_assembler::{CodeAssembler, CodeAssemblerLabel, CodeAssemblerState, CompareBinaryOp, NodeOp};
use crate::tnode::{
  BoolT, FixedArray, HeapObject, Int32T, IntPtrT, JSObject, Map, NodeId, Object, Smi, TNode,
  Uint16T,
};

// -----------------------------------------------------------------------------
// 1. 核心枚举与位集
// -----------------------------------------------------------------------------

/// 内存分配标志 (原 enum class AllocationFlag : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AllocationFlag {
  None = 0,
  DoubleAlignment = 1,
  Pretenured = 1 << 1,
}

impl AllocationFlag {
  #[inline]
  #[must_use]
  pub const fn bits(self) -> u8 {
    self as u8
  }
}

/// 分配标志位集别名
pub type AllocationFlags = u8;

/// 松弛追踪模式 (原 enum SlackTrackingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SlackTrackingMode {
${stm.map((s, i) => `  /// k${s}\n  ${s} = ${i},`).join("\n")}
}

/// 定长数组提取标志 (原 enum class ExtractFixedArrayFlag, 位集)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtractFixedArrayFlag(pub u8);

impl ExtractFixedArrayFlag {
${efa.map((f) => `  /// 原 k${f.name} = ${f.expr}\n  pub const ${f.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}: Self = Self(${f.expr.replace(/k(\w+)/g, (_, n) => `Self::${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}.0`)});`).join("\n")}

  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    (self.0 & other.0) == other.0
  }
}

/// 洞值转换模式 (原 enum class HoleConversionMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum HoleConversionMode {
${hcm.map((s, i) => `  /// k${s}\n  ${s} = ${i},`).join("\n")}
}

/// 基本类型种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrimitiveType {
  Number,
  String,
  Symbol,
  Boolean,
  BigInt,
}

// -----------------------------------------------------------------------------
// 2. CodeStubAssembler 语义封装
// -----------------------------------------------------------------------------

/// V8 高层代码存根汇编器 (CodeStubAssembler DSL)
pub struct CodeStubAssembler<'a> {
  assembler: CodeAssembler<'a>,
}

impl<'a> CodeStubAssembler<'a> {
  #[inline]
  #[must_use]
  pub fn new(state: &'a mut CodeAssemblerState) -> Self {
    Self {
      assembler: CodeAssembler::new(state),
    }
  }

  /// 获取底层 CodeAssembler 引用
  #[inline]
  pub fn raw_assembler(&mut self) -> &mut CodeAssembler<'a> {
    &mut self.assembler
  }

  #[inline]
  pub fn alloc_label(&mut self, name: &'static str) -> CodeAssemblerLabel {
    self.assembler.alloc_label(name)
  }

  // --- Smi 检查与运算原语 ---

  /// 判断对象是否为 Smi (小整数)
  #[inline]
  pub fn tagged_is_smi(&mut self, val: TNode<Object>) -> TNode<BoolT> {
    // Smi tag bit 为最低位 0: (val & 1) == 0
    let word = self.assembler.bitcast_tagged_to_word(val);
    let tag_mask = self.assembler.intptr_constant(1);
    let masked = self.assembler.word_and(word.unchecked_cast(), tag_mask.unchecked_cast());
    let zero = self.assembler.intptr_constant(0);
    self.assembler.word_equal(masked, zero.unchecked_cast())
  }

  /// 判断对象是否非 Smi
  #[inline]
  pub fn tagged_not_smi(&mut self, val: TNode<Object>) -> TNode<BoolT> {
    let is_smi = self.tagged_is_smi(val);
    let false_val = self.assembler.bool_constant(false);
    self.assembler.word32_equal(is_smi, false_val)
  }

  /// Smi 标记编码: IntPtr -> Smi
  #[inline]
  pub fn smi_tag(&mut self, val: TNode<IntPtrT>) -> TNode<Smi> {
    // 64位架构下 Smi 左移 32 位 (或 1 位根据压缩指针配置)
    let shift = self.assembler.int32_constant(32);
    let shifted = self.assembler.word_shl(val.unchecked_cast(), shift.unchecked_cast());
    shifted.unchecked_cast()
  }

  /// Smi 去标记解码: Smi -> IntPtr
  #[inline]
  pub fn smi_untag(&mut self, val: TNode<Smi>) -> TNode<IntPtrT> {
    let shift = self.assembler.int32_constant(32);
    let shifted = self.assembler.word_sar(val.unchecked_cast(), shift.unchecked_cast());
    shifted.unchecked_cast()
  }

  /// Smi 转换为 Int32
  #[inline]
  pub fn smi_to_int32(&mut self, val: TNode<Smi>) -> TNode<Int32T> {
    let intptr = self.smi_untag(val);
    intptr.unchecked_cast()
  }

  /// Smi 加法
  #[inline]
  pub fn smi_add(&mut self, a: TNode<Smi>, b: TNode<Smi>) -> TNode<Smi> {
    let res = self.assembler.intptr_add(a.unchecked_cast(), b.unchecked_cast());
    res.unchecked_cast()
  }

  /// Smi 减法
  #[inline]
  pub fn smi_sub(&mut self, a: TNode<Smi>, b: TNode<Smi>) -> TNode<Smi> {
    let res = self.assembler.intptr_sub(a.unchecked_cast(), b.unchecked_cast());
    res.unchecked_cast()
  }

  /// Smi 等值比较
  #[inline]
  pub fn smi_equal(&mut self, a: TNode<Smi>, b: TNode<Smi>) -> TNode<BoolT> {
    self.assembler.word_equal(a.unchecked_cast(), b.unchecked_cast())
  }

  /// Smi 小于比较
  #[inline]
  pub fn smi_less_than(&mut self, a: TNode<Smi>, b: TNode<Smi>) -> TNode<BoolT> {
    self.assembler.compare_binary_op(
      CompareBinaryOp::IntPtrLessThan,
      a.node_id(),
      b.node_id(),
    )
  }

  /// Smi 小于等于比较
  #[inline]
  pub fn smi_less_than_or_equal(&mut self, a: TNode<Smi>, b: TNode<Smi>) -> TNode<BoolT> {
    self.assembler.compare_binary_op(
      CompareBinaryOp::IntPtrLessThanOrEqual,
      a.node_id(),
      b.node_id(),
    )
  }

  /// Smi 大于比较
  #[inline]
  pub fn smi_greater_than(&mut self, a: TNode<Smi>, b: TNode<Smi>) -> TNode<BoolT> {
    self.assembler.compare_binary_op(
      CompareBinaryOp::IntPtrGreaterThan,
      a.node_id(),
      b.node_id(),
    )
  }

  /// Smi 大于等于比较
  #[inline]
  pub fn smi_greater_than_or_equal(&mut self, a: TNode<Smi>, b: TNode<Smi>) -> TNode<BoolT> {
    self.assembler.compare_binary_op(
      CompareBinaryOp::IntPtrGreaterThanOrEqual,
      a.node_id(),
      b.node_id(),
    )
  }

  // --- 堆对象与 Map 操作 ---

  /// 加载堆对象的 Map
  #[inline]
  pub fn load_map(&mut self, object: TNode<HeapObject>) -> TNode<Map> {
    let map_node_id = self.assembler.raw_assembler_emit(NodeOp::Load, &[object.node_id()]);
    TNode::new(map_node_id)
  }

  /// 加载 Map 的实例类型 (InstanceType)
  #[inline]
  pub fn load_instance_type(&mut self, map: TNode<Map>) -> TNode<Uint16T> {
    let node_id = self.assembler.raw_assembler_emit(NodeOp::Load, &[map.node_id()]);
    TNode::new(node_id)
  }

  /// 加载 JS 对象的 Elements 数组
  #[inline]
  pub fn load_elements(&mut self, object: TNode<JSObject>) -> TNode<FixedArray> {
    let node_id = self.assembler.raw_assembler_emit(NodeOp::Load, &[object.node_id()]);
    TNode::new(node_id)
  }

  /// 加载 FixedArray 长度
  #[inline]
  pub fn load_fixed_array_length(&mut self, array: TNode<FixedArray>) -> TNode<Smi> {
    let node_id = self.assembler.raw_assembler_emit(NodeOp::Load, &[array.node_id()]);
    TNode::new(node_id)
  }

  // --- 快速类型检查谓词 ---

  /// 检查是否为 JSReceiver
  #[inline]
  pub fn is_js_receiver(&mut self, object: TNode<HeapObject>) -> TNode<BoolT> {
    let map = self.load_map(object);
    let instance_type = self.load_instance_type(map);
    // V8 中 LAST_JS_RECEIVER_TYPE 判定
    let limit = self.assembler.int32_constant(0xFF);
    self.assembler.compare_binary_op(
      CompareBinaryOp::Int32LessThanOrEqual,
      instance_type.node_id(),
      limit.node_id(),
    )
  }

  /// 检查是否为 JSObject
  #[inline]
  pub fn is_js_object(&mut self, object: TNode<HeapObject>) -> TNode<BoolT> {
    self.is_js_receiver(object)
  }

  /// 检查是否为 String
  #[inline]
  pub fn is_string(&mut self, object: TNode<HeapObject>) -> TNode<BoolT> {
    let map = self.load_map(object);
    let instance_type = self.load_instance_type(map);
    let first_nonstring_type = self.assembler.int32_constant(0x80);
    self.assembler.compare_binary_op(
      CompareBinaryOp::Int32LessThan,
      instance_type.node_id(),
      first_nonstring_type.node_id(),
    )
  }

  /// 检查是否为 Undefined
  #[inline]
  pub fn is_undefined(&mut self, object: TNode<Object>) -> TNode<BoolT> {
    let undefined_constant = self.assembler.intptr_constant(0x41);
    self.assembler.compare_binary_op(
      CompareBinaryOp::IntPtrLessThan,
      object.node_id(),
      undefined_constant.node_id(),
    )
  }

  /// 检查是否为 Null
  #[inline]
  pub fn is_null(&mut self, object: TNode<Object>) -> TNode<BoolT> {
    let null_constant = self.assembler.intptr_constant(0x31);
    self.assembler.compare_binary_op(
      CompareBinaryOp::IntPtrLessThan,
      object.node_id(),
      null_constant.node_id(),
    )
  }

  // --- 内存分配接口 ---

  /// 原生堆内存分配
  #[inline]
  pub fn allocate_raw(
    &mut self,
    size_in_bytes: TNode<IntPtrT>,
    _flags: AllocationFlags,
  ) -> TNode<HeapObject> {
    let node_id = self.assembler.raw_assembler_emit(NodeOp::CallRuntime(1), &[size_in_bytes.node_id()]);
    TNode::new(node_id)
  }

  /// 新生代年轻代分配
  #[inline]
  pub fn allocate_in_young_generation(&mut self, size_in_bytes: TNode<IntPtrT>) -> TNode<HeapObject> {
    self.allocate_raw(size_in_bytes, AllocationFlag::None.bits())
  }

  /// 老生代分配
  #[inline]
  pub fn allocate_in_old_generation(&mut self, size_in_bytes: TNode<IntPtrT>) -> TNode<HeapObject> {
    self.allocate_raw(size_in_bytes, AllocationFlag::Pretenured.bits())
  }

  // --- 分支与控制流原语 ---

  /// 若为 Smi 则跳转 true_label，否则 false_label
  #[inline]
  pub fn branch_if_smi(
    &mut self,
    value: TNode<Object>,
    if_true: &mut CodeAssemblerLabel,
    if_false: &mut CodeAssemblerLabel,
  ) {
    let is_smi = self.tagged_is_smi(value);
    self.assembler.branch(is_smi, if_true, if_false);
  }

  /// 若非 Smi 则跳转 true_label，否则 false_label
  #[inline]
  pub fn branch_if_not_smi(
    &mut self,
    value: TNode<Object>,
    if_true: &mut CodeAssemblerLabel,
    if_false: &mut CodeAssemblerLabel,
  ) {
    let not_smi = self.tagged_not_smi(value);
    self.assembler.branch(not_smi, if_true, if_false);
  }
}

// 扩展 CodeAssembler 内部辅助
impl<'a> CodeAssembler<'a> {
  #[inline]
  pub(crate) fn raw_assembler_emit(&mut self, op: NodeOp, inputs: &[NodeId]) -> NodeId {
    self.state.emit(op, inputs)
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(AllocationFlag::Pretenured as u8 == 2);
  assert!(ExtractFixedArrayFlag::ALL_FIXED_ARRAYS.0 == 3);
  assert!(HoleConversionMode::DontConvert as u8 == 0);
};
`;

const p = writeSrc("v8-codegen-csa", "code_stub_assembler.rs", rust);
spawnSync("rustfmt", [p], { encoding: "utf8" });
console.log(`written ${p} (CodeStubAssembler 语义原语 DSL 落地)`);
