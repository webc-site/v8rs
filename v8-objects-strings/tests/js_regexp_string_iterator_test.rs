// 迁移规约:
//   js-regexp-string-iterator.h:43-45 BitField 链 (Done(0,1)/Global(1,1)/
//     Unicode(2,1))
//   js-regexp-string-iterator-inl.h:21-47 字段访问器 + BOOL_ACCESSORS
//     (done/global/unicode 位读写) + flags Smi 解包/打包
//   @@iterator 运行时行为 (js-regexp-string-iterator.tq): 推迟 L4
use aok::{OK, Void};
use v8_objects_core::{
  smi::Smi,
  tagged::{Object, Tagged},
};
use v8_objects_strings::js_regexp_string_iterator::{
  DONE_SHIFT, GLOBAL_SHIFT, JSRegExpStringIterator, UNICODE_SHIFT,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

/// 规约 (h:43-45): 位段布局 0/1/2 顺序固定。
#[test]
fn bit_field_layout() -> Void {
  assert_eq!(DONE_SHIFT, 0);
  assert_eq!(GLOBAL_SHIFT, 1);
  assert_eq!(UNICODE_SHIFT, 2);
  OK
}

/// 规约 (inl.h:39-47): flags 整型往返 + 三个布尔位独立读写。
#[test]
fn flags_roundtrip_and_boolean_accessors() -> Void {
  let re = Tagged::<Object>::new(0x1_0001);
  let s = Tagged::<Object>::new(0x1_0011);
  let mut it = JSRegExpStringIterator::new(re, s, Smi::from_int(0));

  // 初值: 三位皆 false
  assert!(!it.done());
  assert!(!it.global());
  assert!(!it.unicode());
  assert_eq!(it.flags(), 0);

  // 独立置位
  it.set_done(true);
  assert!(it.done());
  assert!(!it.global());
  it.set_global(true);
  assert!(it.global());
  it.set_unicode(true);
  assert!(it.unicode());
  assert_eq!(it.flags(), 0b111);

  // 独立复位
  it.set_global(false);
  assert!(it.done());
  assert!(!it.global());
  assert!(it.unicode());
  assert_eq!(it.flags(), 0b101);

  // 整型写入覆盖全部位
  it.set_flags(0);
  assert!(!it.done() && !it.global() && !it.unicode());
  OK
}

/// 规约 (inl.h:21-37): 对象字段读写往返。
#[test]
fn field_accessors_roundtrip() -> Void {
  let re1 = Tagged::<Object>::new(0x1_0001);
  let s1 = Tagged::<Object>::new(0x1_0011);
  let mut it = JSRegExpStringIterator::new(re1, s1, Smi::from_int(0));

  assert_eq!(it.iterating_reg_exp().ptr(), re1.ptr());
  assert_eq!(it.iterated_string().ptr(), s1.ptr());

  let re2 = Tagged::<Object>::new(0x1_0021);
  let s2 = Tagged::<Object>::new(0x1_0031);
  it.set_iterating_reg_exp(re2);
  it.set_iterated_string(s2);
  assert_eq!(it.iterating_reg_exp().ptr(), re2.ptr());
  assert_eq!(it.iterated_string().ptr(), s2.ptr());
  OK
}
