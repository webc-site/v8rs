// 由 tool/src/objects/js-regexp-string-iterator.h.js 从 v8/src/objects/js-regexp-string-iterator.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// 对象字段三元组 + 标志位语义 1:1; JSObject 头布局与 @@iterator 运行时推迟 (登记见脚本头)

//! JSRegExpStringIterator: `String.prototype.matchAll` 返回的迭代器对象。
//!
//! 字段布局占位 (JSObject 头部与堆偏移归 v8-objects-js-core, 登记见
//! 脚本头); 本模块落地字段三元组与 flags 位段语义, 运行时行为
//! (@@iterator 协议 next) 推迟 L4。

use v8_objects_core::{
  smi::Smi,
  tagged::{Object, Tagged},
};

// BitField 链布局自检: 三位互不重叠
const _: () = assert!(DONE_MASK & GLOBAL_MASK == 0);
const _: () = assert!(DONE_MASK & UNICODE_MASK == 0);
const _: () = assert!(GLOBAL_MASK & UNICODE_MASK == 0);

// ── flags 位段 (BitField 链机械展开: Done(0,1)→Global(1,1)→Unicode(2,1)) ──
/// DONE 位偏移 (DONEBit::kShift)。
pub const DONE_SHIFT: u32 = 0;
/// DONE 位宽。
pub const DONE_SIZE: u32 = 1;
/// DONE 掩码。
pub const DONE_MASK: u32 = ((1u32 << 1) - 1) << 0;

/// GLOBAL 位偏移 (GLOBALBit::kShift)。
pub const GLOBAL_SHIFT: u32 = 1;
/// GLOBAL 位宽。
pub const GLOBAL_SIZE: u32 = 1;
/// GLOBAL 掩码。
pub const GLOBAL_MASK: u32 = ((1u32 << 1) - 1) << 1;

/// UNICODE 位偏移 (UNICODEBit::kShift)。
pub const UNICODE_SHIFT: u32 = 2;
/// UNICODE 位宽。
pub const UNICODE_SIZE: u32 = 1;
/// UNICODE 掩码。
pub const UNICODE_MASK: u32 = ((1u32 << 1) - 1) << 2;

/// JSRegExpStringIterator 对象 (字段占位形态)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JSRegExpStringIterator {
  /// 被迭代的 RegExp 对象 (TaggedMember<JSReceiver>; JSReceiver 占位 Object)。
  pub iterating_reg_exp: Tagged<Object>,
  /// 被迭代的字符串 (TaggedMember<String>; String 占位 Object)。
  pub iterated_string: Tagged<Object>,
  /// 标志位 Smi (SmiTagged<JSRegExpStringIteratorFlags>)。
  pub flags: Tagged<Smi>,
}

impl JSRegExpStringIterator {
  /// 全零构造 (工厂分配语义归 L4; 测试/占位用)。
  #[inline]
  #[must_use]
  pub const fn new(
    iterating_reg_exp: Tagged<Object>,
    iterated_string: Tagged<Object>,
    flags: Tagged<Smi>,
  ) -> Self {
    Self {
      iterating_reg_exp,
      iterated_string,
      flags,
    }
  }

  /// flags 的整型位型 (inl.h:39 flags(): Smi 解包)。
  #[inline]
  const fn flags_bits(flags: Tagged<Smi>) -> u32 {
    Smi::value(flags) as u32
  }

  /// 读 flags 整型值 (原版 flags())。
  #[inline]
  #[must_use]
  pub const fn flags(&self) -> i32 {
    Smi::value(self.flags)
  }

  /// 写 flags 整型值 (inl.h:41 set_flags: From31BitPattern → from_int)。
  #[inline]
  pub fn set_flags(&mut self, value: i32) {
    self.flags = Smi::from_int(value);
  }

  /// 读被迭代的 RegExp 对象 (iterating_reg_exp)。
  #[inline]
  #[must_use]
  pub const fn iterating_reg_exp(&self) -> Tagged<Object> {
    self.iterating_reg_exp
  }

  /// 写被迭代的 RegExp 对象 (set_iterating_reg_exp; 写屏障归 heap 层)。
  #[inline]
  pub fn set_iterating_reg_exp(&mut self, value: Tagged<Object>) {
    self.iterating_reg_exp = value;
  }

  /// 读被迭代字符串 (iterated_string)。
  #[inline]
  #[must_use]
  pub const fn iterated_string(&self) -> Tagged<Object> {
    self.iterated_string
  }

  /// 写被迭代字符串 (set_iterated_string)。
  #[inline]
  pub fn set_iterated_string(&mut self, value: Tagged<Object>) {
    self.iterated_string = value;
  }

  /// [[Done]] 内部属性 (BOOL_ACCESSORS done)。
  #[inline]
  #[must_use]
  pub const fn done(&self) -> bool {
    (Self::flags_bits(self.flags) & DONE_MASK) != 0
  }

  /// 置 [[Done]] 属性。
  #[inline]
  pub fn set_done(&mut self, value: bool) {
    let bits = Self::flags_bits(self.flags);
    let updated = if value {
      bits | DONE_MASK
    } else {
      bits & !DONE_MASK
    };
    self.flags = Smi::from_int(updated as i32);
  }

  /// [[Global]] 内部属性 (BOOL_ACCESSORS global)。
  #[inline]
  #[must_use]
  pub const fn global(&self) -> bool {
    (Self::flags_bits(self.flags) & GLOBAL_MASK) != 0
  }

  /// 置 [[Global]] 属性。
  #[inline]
  pub fn set_global(&mut self, value: bool) {
    let bits = Self::flags_bits(self.flags);
    let updated = if value {
      bits | GLOBAL_MASK
    } else {
      bits & !GLOBAL_MASK
    };
    self.flags = Smi::from_int(updated as i32);
  }

  /// [[Unicode]] 内部属性 (BOOL_ACCESSORS unicode)。
  #[inline]
  #[must_use]
  pub const fn unicode(&self) -> bool {
    (Self::flags_bits(self.flags) & UNICODE_MASK) != 0
  }

  /// 置 [[Unicode]] 属性。
  #[inline]
  pub fn set_unicode(&mut self, value: bool) {
    let bits = Self::flags_bits(self.flags);
    let updated = if value {
      bits | UNICODE_MASK
    } else {
      bits & !UNICODE_MASK
    };
    self.flags = Smi::from_int(updated as i32);
  }
}
