// 由 tool/src/maglev/x64/maglev-assembler-x64-inl.h.js 从 v8/src/maglev/x64/maglev-assembler-x64-inl.h + src/codegen/x64/assembler-x64.h (Condition/ScaleFactor 数值) 生成; 修订就地改, 复跑覆盖
// x64 MaglevAssembler 内联层: 条件映射/缩放表/签名骨架 (机械基线)

//! 产物为规则代码基线: 表格与签名清单 1:1 对应原版;
//! 方法体语义 (movq/Ucomisd 等) 不在此展开, 待 Developer 就地实现。

// —— 条件编译块展开登记 (x64 默认配置) ————
// COMPRESS_POINTERS=true, V8_ENABLE_SANDBOX=true (v8-common/src/globals/config.rs);
// V8_STATIC_ROOTS 未定义 → V8_STATIC_ROOTS_BOOL=false (include/v8config.h);
// V8_ENABLE_UNDEFINED_DOUBLE 未启用 (config.rs V8_UNDEFINED_DOUBLE=false);
// V8_DUMPLING 本仓非构建开关, 未启用。
#[rustfmt::skip]
pub static CONDITIONAL_BLOCKS: &[(&str, usize, bool)] = &[
  ("V8_ENABLE_SANDBOX", 3, true),
  ("V8_COMPRESS_POINTERS", 1, true),
  ("V8_STATIC_ROOTS_BOOL", 1, false),
  ("V8_ENABLE_UNDEFINED_DOUBLE", 1, false),
  ("V8_DUMPLING", 1, false),
];

/// ConditionForFloat64: Operation → x64 Condition (名 + 数值, 数值取自 assembler-x64.h)
#[rustfmt::skip]
pub static CONDITION_FOR_FLOAT64: &[(&str, &str, i8)] = &[
  ("kEqual", "equal", 4),
  ("kStrictEqual", "equal", 4),
  ("kLessThan", "below", 2),
  ("kLessThanOrEqual", "below_equal", 6),
  ("kGreaterThan", "above", 7),
  ("kGreaterThanOrEqual", "above_equal", 3),
];

/// ConditionForNaN: x64 浮点无序比较走奇偶标志位 (原版 constexpr ConditionForNaN)
pub const CONDITION_FOR_NAN: (&str, i8) = ("parity_even", 10);

/// ScaleFactor 枚举 (assembler-x64.h): 缩放因子 = log2(步长)
#[rustfmt::skip]
pub static SCALE_FACTOR: &[(&str, i32)] = &[
  ("times_1", 0),
  ("times_2", 1),
  ("times_4", 2),
  ("times_8", 3),
  ("times_int_size", 2),
  ("times_half_system_pointer_size", 2),
  ("times_system_pointer_size", 3),
];

/// 原版 ScaleFactorFromInt(n): {1,2,4,8} → 缩放因子
#[inline]
#[must_use]
pub const fn scale_factor_from_int(n: i32) -> Option<u32> {
  match n {
    1 => Some(0),
    2 => Some(1),
    4 => Some(2),
    8 => Some(3),
    _ => None,
  }
}

/// 方法签名骨架: name / 参数个数 / 外层条件编译守卫 / C++ 原文签名 (折叠空白)
#[derive(Debug, Clone, Copy)]
pub struct MethodSig {
  pub name: &'static str,
  pub params: u8,
  pub guard: Option<&'static str>,
  pub sig: &'static str,
}

/// MaglevAssembler 内联方法定义 (183 处, 按源文件顺序; 含 143 个唯一名)
#[rustfmt::skip]
pub static ASM_METHODS: &[MethodSig] = &[
  MethodSig { name: "Push", params: 1, guard: None, sig: "template <typename... T> void MaglevAssembler::Push(T... vals)" },
  MethodSig { name: "PushReverse", params: 1, guard: None, sig: "template <typename... T> void MaglevAssembler::PushReverse(T... vals)" },
  MethodSig { name: "BindJumpTarget", params: 1, guard: None, sig: "inline void MaglevAssembler::BindJumpTarget(Label* label)" },
  MethodSig { name: "BindBlock", params: 1, guard: None, sig: "inline void MaglevAssembler::BindBlock(BasicBlock* block)" },
  MethodSig { name: "TrySmiTagInt32", params: 1, guard: None, sig: "inline Condition MaglevAssembler::TrySmiTagInt32(Register dst, Register src)" },
  MethodSig { name: "CheckInt32IsSmi", params: 1, guard: None, sig: "inline void MaglevAssembler::CheckInt32IsSmi(Register obj, Label* fail, Register scratch)" },
  MethodSig { name: "SmiAddConstant", params: 1, guard: None, sig: "inline void MaglevAssembler::SmiAddConstant(Register dst, Register src, int value, Label* fail, Label::Distance distance)" },
  MethodSig { name: "SmiSubConstant", params: 1, guard: None, sig: "inline void MaglevAssembler::SmiSubConstant(Register dst, Register src, int value, Label* fail, Label::Distance distance)" },
  MethodSig { name: "MoveHeapNumber", params: 1, guard: None, sig: "inline void MaglevAssembler::MoveHeapNumber(Register dst, double value)" },
  MethodSig { name: "IsRootConstant", params: 1, guard: None, sig: "inline Condition MaglevAssembler::IsRootConstant(Input input, RootIndex root_index)" },
  MethodSig { name: "GetFramePointer", params: 1, guard: None, sig: "inline Register MaglevAssembler::GetFramePointer()" },
  MethodSig { name: "GetStackSlot", params: 1, guard: None, sig: "inline MemOperand MaglevAssembler::GetStackSlot(const compiler::AllocatedOperand& operand)" },
  MethodSig { name: "ToMemOperand", params: 1, guard: None, sig: "inline MemOperand MaglevAssembler::ToMemOperand(const compiler::InstructionOperand& operand)" },
  MethodSig { name: "BuildTypedArrayDataPointer", params: 1, guard: None, sig: "inline void MaglevAssembler::BuildTypedArrayDataPointer(Register data_pointer, Register object)" },
  MethodSig { name: "TypedArrayElementOperand", params: 1, guard: None, sig: "inline MemOperand MaglevAssembler::TypedArrayElementOperand(Register data_pointer, Register index, int element_size)" },
  MethodSig { name: "StoreDataViewElement", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreDataViewElement(Register value, Register data_pointer, Register index, int element_size)" },
  MethodSig { name: "LoadDataViewElement", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadDataViewElement(Register result, Register data_pointer, Register index, int element_size)" },
  MethodSig { name: "LoadUnsignedDataViewElement", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadUnsignedDataViewElement(Register result, Register data_pointer, Register index, int element_size)" },
  MethodSig { name: "LoadTaggedFieldByIndex", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadTaggedFieldByIndex(Register result, Register object, Register index, int scale, int offset)" },
  MethodSig { name: "LoadBoundedSizeFromObject", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadBoundedSizeFromObject(Register result, Register object, int offset)" },
  MethodSig { name: "LoadExternalPointerField", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadExternalPointerField(Register result, Operand operand)" },
  MethodSig { name: "LoadFixedArrayElement", params: 1, guard: None, sig: "void MaglevAssembler::LoadFixedArrayElement(Register result, Register array, Register index)" },
  MethodSig { name: "LoadTaggedFieldWithoutDecompressing", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadTaggedFieldWithoutDecompressing(Register result, Register object, int offset)" },
  MethodSig { name: "LoadFixedArrayElementWithoutDecompressing", params: 1, guard: None, sig: "void MaglevAssembler::LoadFixedArrayElementWithoutDecompressing(Register result, Register array, Register index)" },
  MethodSig { name: "LoadFixedDoubleArrayElement", params: 1, guard: None, sig: "void MaglevAssembler::LoadFixedDoubleArrayElement(DoubleRegister result, Register array, Register index)" },
  MethodSig { name: "StoreFixedDoubleArrayElement", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreFixedDoubleArrayElement(Register array, Register index, DoubleRegister value)" },
  MethodSig { name: "LoadSignedField", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadSignedField(Register result, Operand operand, int size)" },
  MethodSig { name: "LoadUnsignedField", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadUnsignedField(Register result, Operand operand, int size)" },
  MethodSig { name: "SetSlotAddressForTaggedField", params: 1, guard: None, sig: "inline void MaglevAssembler::SetSlotAddressForTaggedField(Register slot_reg, Register object, int offset)" },
  MethodSig { name: "SetSlotAddressForFixedArrayElement", params: 1, guard: None, sig: "inline void MaglevAssembler::SetSlotAddressForFixedArrayElement(Register slot_reg, Register object, Register index)" },
  MethodSig { name: "StoreTaggedFieldNoWriteBarrier", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreTaggedFieldNoWriteBarrier(Register object, int offset, Register value)" },
  MethodSig { name: "StoreTaggedFieldNoWriteBarrier", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreTaggedFieldNoWriteBarrier(Register object, int offset, ValueNode* constant)" },
  MethodSig { name: "StoreTaggedFieldNoWriteBarrier", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreTaggedFieldNoWriteBarrier(Register object, int offset, Handle<HeapObject> constant)" },
  MethodSig { name: "StoreFixedArrayElementNoWriteBarrier", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreFixedArrayElementNoWriteBarrier(Register array, Register index, Register value)" },
  MethodSig { name: "StoreTaggedSignedField", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreTaggedSignedField(Register object, int offset, Register value)" },
  MethodSig { name: "StoreTaggedSignedField", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreTaggedSignedField(Register object, int offset, Tagged<Smi> value)" },
  MethodSig { name: "StoreInt32Field", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreInt32Field(Register object, int offset, int32_t value)" },
  MethodSig { name: "StoreField", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreField(Operand operand, Register value, int size)" },
  MethodSig { name: "StoreTrustedPointerFieldNoWriteBarrier", params: 1, guard: Some("V8_ENABLE_SANDBOX"), sig: "inline void MaglevAssembler::StoreTrustedPointerFieldNoWriteBarrier(Register object, int offset, Register value)" },
  MethodSig { name: "ReverseByteOrder", params: 1, guard: None, sig: "inline void MaglevAssembler::ReverseByteOrder(Register value, int size)" },
  MethodSig { name: "ReverseByteOrderUnsigned", params: 1, guard: None, sig: "inline void MaglevAssembler::ReverseByteOrderUnsigned(Register value, int size)" },
  MethodSig { name: "StackSlotOperand", params: 1, guard: None, sig: "inline MemOperand MaglevAssembler::StackSlotOperand(StackSlot stack_slot)" },
  MethodSig { name: "IncrementInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::IncrementInt32(Register reg)" },
  MethodSig { name: "DecrementInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::DecrementInt32(Register reg)" },
  MethodSig { name: "AddInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::AddInt32(Register reg, int amount)" },
  MethodSig { name: "AddInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::AddInt32(Register reg, Register other)" },
  MethodSig { name: "AddInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::AddInt32(Register dst, Register src, int amount)" },
  MethodSig { name: "AndInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::AndInt32(Register reg, int mask)" },
  MethodSig { name: "AndInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::AndInt32(Register dst, Register src, int mask)" },
  MethodSig { name: "OrInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::OrInt32(Register reg, int mask)" },
  MethodSig { name: "AndInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::AndInt32(Register reg, Register other)" },
  MethodSig { name: "OrInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::OrInt32(Register reg, Register other)" },
  MethodSig { name: "ShiftLeft", params: 1, guard: None, sig: "inline void MaglevAssembler::ShiftLeft(Register reg, int amount)" },
  MethodSig { name: "IncrementAddress", params: 1, guard: None, sig: "inline void MaglevAssembler::IncrementAddress(Register reg, int32_t delta)" },
  MethodSig { name: "LoadAddress", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadAddress(Register dst, MemOperand location)" },
  MethodSig { name: "MakeWeak", params: 1, guard: None, sig: "inline void MaglevAssembler::MakeWeak(Register dst, Register src)" },
  MethodSig { name: "EmitEnterExitFrame", params: 1, guard: None, sig: "inline void MaglevAssembler::EmitEnterExitFrame(int extra_slots, StackFrame::Type frame_type, Register scratch)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(StackSlot dst, Register src)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(StackSlot dst, DoubleRegister src)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, StackSlot src)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(DoubleRegister dst, StackSlot src)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(MemOperand dst, Register src)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, Tagged<TaggedIndex> i)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(DoubleRegister dst, DoubleRegister src)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, Tagged<Smi> src)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, ExternalReference src)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, MemOperand src)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, Register src)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, int32_t i)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, uint32_t i)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, IndirectPointerTag i)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(DoubleRegister dst, double n)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(DoubleRegister dst, Float64 n)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, Handle<HeapObject> obj)" },
  MethodSig { name: "MoveTagged", params: 1, guard: None, sig: "void MaglevAssembler::MoveTagged(Register dst, Handle<HeapObject> obj)" },
  MethodSig { name: "Move", params: 1, guard: None, sig: "inline void MaglevAssembler::Move(Register dst, intptr_t p)" },
  MethodSig { name: "LoadInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadInt32(Register dst, MemOperand src)" },
  MethodSig { name: "StoreInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreInt32(MemOperand dst, Register src)" },
  MethodSig { name: "LoadFloat32", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadFloat32(DoubleRegister dst, MemOperand src)" },
  MethodSig { name: "StoreFloat32", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreFloat32(MemOperand dst, DoubleRegister src)" },
  MethodSig { name: "LoadFloat64", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadFloat64(DoubleRegister dst, MemOperand src)" },
  MethodSig { name: "StoreFloat64", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreFloat64(MemOperand dst, DoubleRegister src)" },
  MethodSig { name: "LoadUnalignedFloat32", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadUnalignedFloat32(DoubleRegister dst, Register base, Register index)" },
  MethodSig { name: "LoadUnalignedFloat32AndReverseByteOrder", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadUnalignedFloat32AndReverseByteOrder(DoubleRegister dst, Register base, Register index)" },
  MethodSig { name: "StoreUnalignedFloat32", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreUnalignedFloat32(Register base, Register index, DoubleRegister src)" },
  MethodSig { name: "ReverseByteOrderAndStoreUnalignedFloat32", params: 1, guard: None, sig: "inline void MaglevAssembler::ReverseByteOrderAndStoreUnalignedFloat32(Register base, Register index, DoubleRegister src)" },
  MethodSig { name: "LoadUnalignedFloat64", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadUnalignedFloat64(DoubleRegister dst, Register base, Register index)" },
  MethodSig { name: "LoadUnalignedFloat64AndReverseByteOrder", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadUnalignedFloat64AndReverseByteOrder(DoubleRegister dst, Register base, Register index)" },
  MethodSig { name: "StoreUnalignedFloat64", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreUnalignedFloat64(Register base, Register index, DoubleRegister src)" },
  MethodSig { name: "ReverseByteOrderAndStoreUnalignedFloat64", params: 1, guard: None, sig: "inline void MaglevAssembler::ReverseByteOrderAndStoreUnalignedFloat64(Register base, Register index, DoubleRegister src)" },
  MethodSig { name: "SignExtend32To64Bits", params: 1, guard: None, sig: "inline void MaglevAssembler::SignExtend32To64Bits(Register dst, Register src)" },
  MethodSig { name: "NegateInt32", params: 1, guard: None, sig: "inline void MaglevAssembler::NegateInt32(Register val)" },
  MethodSig { name: "ToUint8Clamped", params: 1, guard: None, sig: "inline void MaglevAssembler::ToUint8Clamped(Register result, DoubleRegister value, Label* min, Label* max, Label* done)" },
  MethodSig { name: "DeoptIfBufferNotValid", params: 1, guard: None, sig: "template <typename NodeT> inline void MaglevAssembler::DeoptIfBufferNotValid(Register array, Register scratch, TypedArrayAccessMode mode, NodeT* node)" },
  MethodSig { name: "LoadByte", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadByte(Register dst, MemOperand src)" },
  MethodSig { name: "IsCallableAndNotUndetectable", params: 1, guard: None, sig: "inline Condition MaglevAssembler::IsCallableAndNotUndetectable(Register map, Register scratch)" },
  MethodSig { name: "IsNotCallableNorUndetactable", params: 1, guard: None, sig: "inline Condition MaglevAssembler::IsNotCallableNorUndetactable(Register map, Register scratch)" },
  MethodSig { name: "LoadInstanceType", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadInstanceType(Register instance_type, Register heap_object)" },
  MethodSig { name: "JumpIfObjectType", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpIfObjectType(Register heap_object, InstanceType type, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfNotObjectType", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpIfNotObjectType(Register heap_object, InstanceType type, Label* target, Label::Distance distance)" },
  MethodSig { name: "AssertObjectType", params: 1, guard: None, sig: "inline void MaglevAssembler::AssertObjectType(Register heap_object, InstanceType type, AbortReason reason)" },
  MethodSig { name: "BranchOnObjectType", params: 1, guard: None, sig: "inline void MaglevAssembler::BranchOnObjectType(Register heap_object, InstanceType type, Label* if_true, Label::Distance true_distance, bool fallthrough_when_true, Label* if_false, Label::Distance false_distance, bool fallthrough_when_false)" },
  MethodSig { name: "JumpIfObjectTypeInRange", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpIfObjectTypeInRange(Register heap_object, InstanceType lower_limit, InstanceType higher_limit, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfObjectTypeNotInRange", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpIfObjectTypeNotInRange(Register heap_object, InstanceType lower_limit, InstanceType higher_limit, Label* target, Label::Distance distance)" },
  MethodSig { name: "AssertObjectTypeInRange", params: 1, guard: None, sig: "inline void MaglevAssembler::AssertObjectTypeInRange(Register heap_object, InstanceType lower_limit, InstanceType higher_limit, AbortReason reason)" },
  MethodSig { name: "BranchOnObjectTypeInRange", params: 1, guard: None, sig: "inline void MaglevAssembler::BranchOnObjectTypeInRange(Register heap_object, InstanceType lower_limit, InstanceType higher_limit, Label* if_true, Label::Distance true_distance, bool fallthrough_when_true, Label* if_false, Label::Distance false_distance, bool fallthrough_when_false)" },
  MethodSig { name: "JumpIfJSAnyIsNotPrimitive", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpIfJSAnyIsNotPrimitive(Register heap_object, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfObjectInRange", params: 1, guard: Some("V8_STATIC_ROOTS_BOOL"), sig: "inline void MaglevAssembler::JumpIfObjectInRange(Register heap_object, Tagged_t lower_limit, Tagged_t higher_limit, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfObjectNotInRange", params: 1, guard: Some("V8_STATIC_ROOTS_BOOL"), sig: "inline void MaglevAssembler::JumpIfObjectNotInRange(Register heap_object, Tagged_t lower_limit, Tagged_t higher_limit, Label* target, Label::Distance distance)" },
  MethodSig { name: "AssertObjectInRange", params: 1, guard: Some("V8_STATIC_ROOTS_BOOL"), sig: "inline void MaglevAssembler::AssertObjectInRange(Register heap_object, Tagged_t lower_limit, Tagged_t higher_limit, AbortReason reason)" },
  MethodSig { name: "CompareMapWithRoot", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareMapWithRoot(Register object, RootIndex index, Register scratch)" },
  MethodSig { name: "CompareInstanceType", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareInstanceType(Register map, InstanceType instance_type)" },
  MethodSig { name: "CompareInstanceTypeRange", params: 1, guard: None, sig: "inline Condition MaglevAssembler::CompareInstanceTypeRange(Register map, Register instance_type_out, InstanceType lower_limit, InstanceType higher_limit)" },
  MethodSig { name: "CompareFloat64AndJumpIf", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareFloat64AndJumpIf(DoubleRegister src1, DoubleRegister src2, Condition cond, Label* target, Label* nan_failed, Label::Distance distance)" },
  MethodSig { name: "CompareFloat64AndBranch", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareFloat64AndBranch(DoubleRegister src1, DoubleRegister src2, Condition cond, BasicBlock* if_true, BasicBlock* if_false, BasicBlock* next_block, BasicBlock* nan_failed)" },
  MethodSig { name: "PrepareCallCFunction", params: 1, guard: None, sig: "inline void MaglevAssembler::PrepareCallCFunction(int num_reg_arguments, int num_double_registers)" },
  MethodSig { name: "CallSelf", params: 1, guard: None, sig: "inline void MaglevAssembler::CallSelf()" },
  MethodSig { name: "Jump", params: 1, guard: None, sig: "inline void MaglevAssembler::Jump(Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpToDeopt", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpToDeopt(Label* target)" },
  MethodSig { name: "EmitEagerDeoptStress", params: 1, guard: None, sig: "inline void MaglevAssembler::EmitEagerDeoptStress(Label* target)" },
  MethodSig { name: "JumpIf", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpIf(Condition cond, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfRoot", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpIfRoot(Register with, RootIndex index, Label* if_equal, Label::Distance distance)" },
  MethodSig { name: "JumpIfNotRoot", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpIfNotRoot(Register with, RootIndex index, Label* if_not_equal, Label::Distance distance)" },
  MethodSig { name: "JumpIfSmi", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpIfSmi(Register src, Label* on_smi, Label::Distance distance)" },
  MethodSig { name: "JumpIfNotSmi", params: 1, guard: None, sig: "inline void MaglevAssembler::JumpIfNotSmi(Register src, Label* on_not_smi, Label::Distance distance)" },
  MethodSig { name: "JumpIfByte", params: 1, guard: None, sig: "void MaglevAssembler::JumpIfByte(Condition cc, Register value, int32_t byte, Label* target, Label::Distance distance)" },
  MethodSig { name: "Float64SilenceNan", params: 1, guard: None, sig: "void MaglevAssembler::Float64SilenceNan(DoubleRegister value)" },
  MethodSig { name: "Float64ExtractHighWord32", params: 1, guard: None, sig: "void MaglevAssembler::Float64ExtractHighWord32(Register dst, DoubleRegister src)" },
  MethodSig { name: "JumpIfUndefinedNan", params: 1, guard: Some("V8_ENABLE_UNDEFINED_DOUBLE"), sig: "void MaglevAssembler::JumpIfUndefinedNan(DoubleRegister value, Register scratch, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfUndefinedNan", params: 1, guard: Some("V8_ENABLE_UNDEFINED_DOUBLE"), sig: "void MaglevAssembler::JumpIfUndefinedNan(MemOperand operand, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfNotUndefinedNan", params: 1, guard: Some("V8_ENABLE_UNDEFINED_DOUBLE"), sig: "void MaglevAssembler::JumpIfNotUndefinedNan(DoubleRegister value, Register scratch, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfHoleNan", params: 1, guard: None, sig: "void MaglevAssembler::JumpIfHoleNan(DoubleRegister value, Register scratch, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfHoleNan", params: 1, guard: None, sig: "void MaglevAssembler::JumpIfHoleNan(MemOperand operand, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfNotHoleNan", params: 1, guard: None, sig: "void MaglevAssembler::JumpIfNotHoleNan(DoubleRegister value, Register scratch, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfNotHoleNan", params: 1, guard: None, sig: "void MaglevAssembler::JumpIfNotHoleNan(MemOperand operand, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfNan", params: 1, guard: None, sig: "void MaglevAssembler::JumpIfNan(DoubleRegister value, Label* target, Label::Distance distance)" },
  MethodSig { name: "JumpIfNotNan", params: 1, guard: None, sig: "void MaglevAssembler::JumpIfNotNan(DoubleRegister value, Label* target, Label::Distance distance)" },
  MethodSig { name: "SubInt32", params: 1, guard: None, sig: "void MaglevAssembler::SubInt32(Register dst, Register src)" },
  MethodSig { name: "SubInt32", params: 1, guard: None, sig: "void MaglevAssembler::SubInt32(Register dst, Register src1, Register src2)" },
  MethodSig { name: "ShiftRightLogical32", params: 1, guard: None, sig: "void MaglevAssembler::ShiftRightLogical32(Register dst, int32_t value)" },
  MethodSig { name: "ShiftRightLogical32", params: 1, guard: None, sig: "void MaglevAssembler::ShiftRightLogical32(Register dst, Register src, int32_t value)" },
  MethodSig { name: "LoadBitsFromWord32", params: 1, guard: None, sig: "void MaglevAssembler::LoadBitsFromWord32(Register dst, Register src, int width, int shift)" },
  MethodSig { name: "CompareInt32AndJumpIf", params: 1, guard: None, sig: "void MaglevAssembler::CompareInt32AndJumpIf(Register r1, Register r2, Condition cond, Label* target, Label::Distance distance)" },
  MethodSig { name: "CompareIntPtrAndJumpIf", params: 1, guard: None, sig: "void MaglevAssembler::CompareIntPtrAndJumpIf(Register r1, Register r2, Condition cond, Label* target, Label::Distance distance)" },
  MethodSig { name: "CompareInt32AndJumpIf", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareInt32AndJumpIf(Register r1, int32_t value, Condition cond, Label* target, Label::Distance distance)" },
  MethodSig { name: "CompareIntPtrAndJumpIf", params: 1, guard: None, sig: "void MaglevAssembler::CompareIntPtrAndJumpIf(Register r1, int32_t value, Condition cond, Label* target, Label::Distance distance)" },
  MethodSig { name: "CompareInt32AndBranch", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareInt32AndBranch(Register r1, int32_t value, Condition cond, Label* if_true, Label::Distance true_distance, bool fallthrough_when_true, Label* if_false, Label::Distance false_distance, bool fallthrough_when_false)" },
  MethodSig { name: "CompareInt32AndBranch", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareInt32AndBranch(Register r1, Register r2, Condition cond, Label* if_true, Label::Distance true_distance, bool fallthrough_when_true, Label* if_false, Label::Distance false_distance, bool fallthrough_when_false)" },
  MethodSig { name: "CompareIntPtrAndBranch", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareIntPtrAndBranch(Register r1, int32_t value, Condition cond, Label* if_true, Label::Distance true_distance, bool fallthrough_when_true, Label* if_false, Label::Distance false_distance, bool fallthrough_when_false)" },
  MethodSig { name: "CompareInt32AndAssert", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareInt32AndAssert(Register r1, Register r2, Condition cond, AbortReason reason)" },
  MethodSig { name: "CompareInt32AndAssert", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareInt32AndAssert(Register r1, int32_t value, Condition cond, AbortReason reason)" },
  MethodSig { name: "CompareSmiAndJumpIf", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareSmiAndJumpIf(Register r1, Tagged<Smi> value, Condition cond, Label* target, Label::Distance distance)" },
  MethodSig { name: "CompareSmiAndAssert", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareSmiAndAssert(Register r1, Tagged<Smi> value, Condition cond, AbortReason reason)" },
  MethodSig { name: "CompareByteAndJumpIf", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareByteAndJumpIf(MemOperand left, int8_t right, Condition cond, Register scratch, Label* target, Label::Distance distance)" },
  MethodSig { name: "CompareTaggedAndJumpIf", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareTaggedAndJumpIf(Register r1, Tagged<Smi> value, Condition cond, Label* target, Label::Distance distance)" },
  MethodSig { name: "CompareTaggedAndJumpIf", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareTaggedAndJumpIf(Register r1, Handle<HeapObject> obj, Condition cond, Label* target, Label::Distance distance)" },
  MethodSig { name: "CompareTaggedAndJumpIf", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareTaggedAndJumpIf(Register src1, Register src2, Condition cond, Label* target, Label::Distance distance)" },
  MethodSig { name: "CompareDoubleAndJumpIfZeroOrNaN", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareDoubleAndJumpIfZeroOrNaN(DoubleRegister reg, Label* target, Label::Distance distance)" },
  MethodSig { name: "CompareDoubleAndJumpIfZeroOrNaN", params: 1, guard: None, sig: "inline void MaglevAssembler::CompareDoubleAndJumpIfZeroOrNaN(MemOperand operand, Label* target, Label::Distance distance)" },
  MethodSig { name: "TestInt32AndJumpIfAnySet", params: 1, guard: None, sig: "inline void MaglevAssembler::TestInt32AndJumpIfAnySet(Register r1, int32_t mask, Label* target, Label::Distance distance)" },
  MethodSig { name: "TestInt32AndJumpIfAnySet", params: 1, guard: None, sig: "inline void MaglevAssembler::TestInt32AndJumpIfAnySet(MemOperand operand, int32_t mask, Label* target, Label::Distance distance)" },
  MethodSig { name: "TestUint8AndJumpIfAnySet", params: 1, guard: None, sig: "inline void MaglevAssembler::TestUint8AndJumpIfAnySet(MemOperand operand, uint8_t mask, Label* target, Label::Distance distance)" },
  MethodSig { name: "TestInt32AndJumpIfAllClear", params: 1, guard: None, sig: "inline void MaglevAssembler::TestInt32AndJumpIfAllClear(Register r1, int32_t mask, Label* target, Label::Distance distance)" },
  MethodSig { name: "TestInt32AndJumpIfAllClear", params: 1, guard: None, sig: "inline void MaglevAssembler::TestInt32AndJumpIfAllClear(MemOperand operand, int32_t mask, Label* target, Label::Distance distance)" },
  MethodSig { name: "TestUint8AndJumpIfAllClear", params: 1, guard: None, sig: "inline void MaglevAssembler::TestUint8AndJumpIfAllClear(MemOperand operand, uint8_t mask, Label* target, Label::Distance distance)" },
  MethodSig { name: "LoadContextCellState", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadContextCellState(Register state, Register cell)" },
  MethodSig { name: "LoadContextCellInt32Value", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadContextCellInt32Value(Register value, Register cell)" },
  MethodSig { name: "LoadContextCellFloat64Value", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadContextCellFloat64Value(DoubleRegister value, Register cell)" },
  MethodSig { name: "StoreContextCellInt32Value", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreContextCellInt32Value(Register cell, Register value)" },
  MethodSig { name: "StoreContextCellFloat64Value", params: 1, guard: None, sig: "inline void MaglevAssembler::StoreContextCellFloat64Value(Register cell, DoubleRegister value)" },
  MethodSig { name: "LoadHeapNumberValue", params: 1, guard: None, sig: "inline void MaglevAssembler::LoadHeapNumberValue(DoubleRegister result, Register heap_number)" },
  MethodSig { name: "Int32ToDouble", params: 1, guard: None, sig: "inline void MaglevAssembler::Int32ToDouble(DoubleRegister result, Register src)" },
  MethodSig { name: "Uint32ToDouble", params: 1, guard: None, sig: "inline void MaglevAssembler::Uint32ToDouble(DoubleRegister result, Register src)" },
  MethodSig { name: "IntPtrToDouble", params: 1, guard: None, sig: "inline void MaglevAssembler::IntPtrToDouble(DoubleRegister result, Register src)" },
  MethodSig { name: "Pop", params: 1, guard: None, sig: "inline void MaglevAssembler::Pop(Register dst)" },
  MethodSig { name: "EmitEagerDeoptIfNotEqual", params: 1, guard: None, sig: "template <typename NodeT> inline void MaglevAssembler::EmitEagerDeoptIfNotEqual(DeoptimizeReason reason, NodeT* node)" },
  MethodSig { name: "AssertStackSizeCorrect", params: 1, guard: None, sig: "inline void MaglevAssembler::AssertStackSizeCorrect()" },
  MethodSig { name: "FunctionEntryStackCheck", params: 1, guard: None, sig: "inline Condition MaglevAssembler::FunctionEntryStackCheck(int stack_check_offset)" },
  MethodSig { name: "FinishCode", params: 1, guard: None, sig: "inline void MaglevAssembler::FinishCode()" },
  MethodSig { name: "MoveRepr", params: 1, guard: None, sig: "template <typename Dest, typename Source> inline void MaglevAssembler::MoveRepr(MachineRepresentation repr, Dest dst, Source src)" },
  MethodSig { name: "MoveRepr", params: 1, guard: None, sig: "template <> inline void MaglevAssembler::MoveRepr(MachineRepresentation repr, MemOperand dst, MemOperand src)" },
  MethodSig { name: "MaybeEmitPlaceHolderForDeopt", params: 1, guard: None, sig: "inline void MaglevAssembler::MaybeEmitPlaceHolderForDeopt()" },
  MethodSig { name: "MemoryBarrier", params: 1, guard: None, sig: "inline void MaglevAssembler::MemoryBarrier(AtomicMemoryOrder order)" },
];

/// MapCompare 架构内联实现 (4: 构造/Generate/GetMap/TemporaryCount;
/// TemporaryCount 规则: map_count==1 → 0, 否则 1)
#[rustfmt::skip]
pub static MAP_COMPARE_METHODS: &[MethodSig] = &[
  MethodSig { name: "MapCompare", params: 1, guard: None, sig: "inline MapCompare::MapCompare(MaglevAssembler* masm, Register object, size_t map_count) : masm_(masm), object_(object), map_count_(map_count)" },
  MethodSig { name: "Generate", params: 1, guard: None, sig: "void MapCompare::Generate(Handle<Map> map, Condition cond, Label* if_true, Label::Distance distance)" },
  MethodSig { name: "GetMap", params: 1, guard: None, sig: "Register MapCompare::GetMap()" },
  MethodSig { name: "TemporaryCount", params: 1, guard: None, sig: "int MapCompare::TemporaryCount(size_t map_count)" },
];

/// 文件级自由函数签名 (规则函数, 映射表见上)
#[rustfmt::skip]
pub static FREE_FUNCTIONS: &[&str] = &[
  "constexpr Condition ConditionForFloat64(Operation operation)",
  "constexpr Condition ConditionForNaN()",
  "inline ScaleFactor ScaleFactorFromInt(int n)",
];

// —— 类结构登记 ————
// MaglevAssembler::TemporaryRegisterScope (Base=TemporaryRegisterScopeBase):
//   SavedData{has_scratch_register_, has_double_scratch_register_};
//   AcquireScratch→kScratchRegister, AcquireScratchDouble→kScratchDoubleReg;
//   ResetToDefaultImpl 双布尔复位。7 个结构锚点全过。

// —— 提取不变式断言 ——
const _: () = {
  assert!(CONDITION_FOR_FLOAT64.len() == 6);
  assert!(CONDITION_FOR_FLOAT64[0].2 == 4);
  assert!(CONDITION_FOR_FLOAT64[1].2 == 4);
  assert!(CONDITION_FOR_NAN.1 == 10);
  assert!(SCALE_FACTOR.len() == 7);
  assert!(ASM_METHODS.len() == 183);
  assert!(MAP_COMPARE_METHODS.len() == 4);
  assert!(FREE_FUNCTIONS.len() == 3);
  assert!(CONDITIONAL_BLOCKS.len() == 5);
};
