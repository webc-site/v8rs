// 由 tool/src/codegen/linkage-location.h.js 从 v8/src/codegen/linkage-location.h 生成; 修订就地改, 复跑覆盖
// LinkageLocation 位域编码 + 常量 + 签名骨架

use crate::{
  machine_type::{MachineRepresentation, element_size_in_pointers},
  signature::Signature,
};

/// 位置类别 (原版 enum LocationType): 寄存器 / 栈槽
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LocationType {
  REGISTER = 0,
  STACK_SLOT = 1,
}

/// TypeField: bit 0 (1 位)
pub const TYPE_FIELD_SHIFT: u32 = 0;
pub const TYPE_FIELD_BITS: u32 = 1;
/// LocationField: bit 1..32 (31 位, 容负值按位重解释)
pub const LOCATION_FIELD_SHIFT: u32 = 1;
pub const LOCATION_FIELD_BITS: u32 = 31;

/// ANY_REGISTER 哨兵 (原版 ANY_REGISTER = -1)
pub const ANY_REGISTER: i32 = -1;
/// 栈槽上界 (原版 MAX_STACK_SLOT)
pub const MAX_STACK_SLOT: i32 = 32767;

/// 位置描述: 调用参数/返回值落在寄存器或栈槽。
/// bit_field 编码: [0] 类型位 | [1..32] 位置 (int32 按位存储, 允许 -1)。
/// MachineType 本体见 machine-type 覆盖面; 此处以其 repr(u8) 编码占位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkageLocation {
  bit_field_: u32,
  /// 原版 machine_type_ (MachineType; 以 MachineRepresentation 序号 u8 编码占位)
  machine_type_: u8,
}

impl LinkageLocation {
  /// 原版私有构造: (type, location, machine_type) 打包位域
  #[inline]
  #[must_use]
  const fn pack(ty: LocationType, location: i32, machine_type: u8) -> Self {
    let type_bits = (ty as u32) << TYPE_FIELD_SHIFT;
    // location 可为 -1 (ANY_REGISTER), 按位截断存入
    let loc_bits = ((location as u32) << LOCATION_FIELD_SHIFT) & LOC_MASK;
    Self {
      bit_field_: type_bits | loc_bits,
      machine_type_: machine_type,
    }
  }

  /// 原版 ForNullRegister
  #[inline]
  #[must_use]
  pub const fn for_null_register(reg: i32, machine_type: u8) -> Self {
    Self::pack(LocationType::REGISTER, reg, machine_type)
  }

  /// 原版 ForAnyRegister
  #[inline]
  #[must_use]
  pub const fn for_any_register(machine_type: u8) -> Self {
    Self::pack(LocationType::REGISTER, ANY_REGISTER, machine_type)
  }

  /// 原版 ForRegister (reg >= 0)
  #[inline]
  #[must_use]
  pub const fn for_register(reg: i32, machine_type: u8) -> Self {
    Self::pack(LocationType::REGISTER, reg, machine_type)
  }

  /// 原版 ForCallerFrameSlot (slot < 0)
  #[inline]
  #[must_use]
  pub const fn for_caller_frame_slot(slot: i32, machine_type: u8) -> Self {
    Self::pack(LocationType::STACK_SLOT, slot, machine_type)
  }

  /// 原版 ForCalleeFrameSlot (0 <= slot < MAX_STACK_SLOT)
  #[inline]
  #[must_use]
  pub const fn for_callee_frame_slot(slot: i32, machine_type: u8) -> Self {
    Self::pack(LocationType::STACK_SLOT, slot, machine_type)
  }

  /// 原版 GetType: machine_type 编码 (占位)
  #[inline]
  #[must_use]
  pub const fn get_type(&self) -> u8 {
    self.machine_type_
  }

  /// 原版 GetLocation: 解码位置 (负值安全: 先按位取再符号化)
  #[inline]
  #[must_use]
  pub const fn get_location(&self) -> i32 {
    ((self.bit_field_ & LOC_MASK) >> LOCATION_FIELD_SHIFT) as i32
  }

  /// 原版 IsNullRegister: 寄存器位且 < ANY_REGISTER
  #[inline]
  #[must_use]
  pub const fn is_null_register(&self) -> bool {
    self.is_register() && self.get_location() < ANY_REGISTER
  }

  /// 原版 IsRegister
  #[inline]
  #[must_use]
  pub const fn is_register(&self) -> bool {
    (self.bit_field_ & TYPE_MASK) == (LocationType::REGISTER as u32)
  }

  /// 原版 IsAnyRegister
  #[inline]
  #[must_use]
  pub const fn is_any_register(&self) -> bool {
    self.is_register() && self.get_location() == ANY_REGISTER
  }

  /// 原版 IsCallerFrameSlot: 非寄存器且位置 < 0
  #[inline]
  #[must_use]
  pub const fn is_caller_frame_slot(&self) -> bool {
    !self.is_register() && self.get_location() < 0
  }

  /// 原版 IsCalleeFrameSlot: 非寄存器且位置 >= 0
  #[inline]
  #[must_use]
  pub const fn is_callee_frame_slot(&self) -> bool {
    !self.is_register() && self.get_location() >= 0
  }

  /// 原版 AsRegister
  #[inline]
  #[must_use]
  pub const fn as_register(&self) -> i32 {
    self.get_location()
  }

  /// 原版 AsCallerFrameSlot
  #[inline]
  #[must_use]
  pub const fn as_caller_frame_slot(&self) -> i32 {
    self.get_location()
  }

  /// 原版 AsCalleeFrameSlot
  #[inline]
  #[must_use]
  pub const fn as_callee_frame_slot(&self) -> i32 {
    self.get_location()
  }

  /// 原版 GetSizeInPointers (linkage-location.h:120):
  /// `ElementSizeInPointers(GetType().representation())`; 此处 machine_type_ 编码
  /// 即 MachineRepresentation 精确序号 (见 machine_type.rs from_code)。
  #[must_use]
  pub const fn get_size_in_pointers(&self) -> i32 {
    match MachineRepresentation::from_code(self.machine_type_) {
      Some(rep) => element_size_in_pointers(rep),
      // v8: 原版类型系统保证合法 MachineType; 非法编码即非法构造
      None => panic!("GetSizeInPointers: 非法 machine_type 编码"),
    }
  }

  /// 原版 ConvertToTailCallerLocation (尾调用参数槽平移)
  #[must_use]
  pub const fn convert_to_tail_caller_location(
    caller_location: Self,
    stack_param_delta: i32,
  ) -> Self {
    if !caller_location.is_register() {
      return Self::pack(
        LocationType::STACK_SLOT,
        caller_location.get_location() + stack_param_delta,
        caller_location.get_type(),
      );
    }
    caller_location
  }

  /// 原版 IsSameLocation: 位域相等且 repr 互为子类型 (签名骨架, repr 判等占位)
  #[inline]
  #[must_use]
  pub const fn is_same_location(a: &Self, b: &Self) -> bool {
    a.bit_field_ == b.bit_field_ && a.machine_type_ == b.machine_type_
  }
}

/// 原版 using LocationSignature = Signature<LinkageLocation>
pub type LocationSignature = Signature<LinkageLocation>;

// —— 提取不变式断言 ——
const _: () = {
  const LOC_MASK: u32 = ((1u32 << LOCATION_FIELD_BITS) - 1) << LOCATION_FIELD_SHIFT;
  const TYPE_MASK: u32 = 1;
  assert!(LOCATION_FIELD_SHIFT == 1 && LOCATION_FIELD_BITS == 31);
  assert!(ANY_REGISTER == -1 && MAX_STACK_SLOT == 32767);
  let null_reg = LinkageLocation::for_null_register(-1, 0);
  assert!(null_reg.is_null_register());
  let r = LinkageLocation::for_register(3, 0);
  assert!(r.is_register() && r.as_register() == 3 && !r.is_any_register());
  let any = LinkageLocation::for_any_register(0);
  assert!(any.is_any_register());
  let caller = LinkageLocation::for_caller_frame_slot(-8, 0);
  assert!(caller.is_caller_frame_slot() && caller.as_caller_frame_slot() == -8);
  let callee = LinkageLocation::for_callee_frame_slot(4, 0);
  assert!(callee.is_callee_frame_slot() && callee.as_callee_frame_slot() == 4);
  let tail = LinkageLocation::convert_to_tail_caller_location(caller, 2);
  assert!(tail.as_caller_frame_slot() == -6);
  let _ = (LOC_MASK, TYPE_MASK);
};
