//! Handle 语义测试: 间接访问, 解引用, PatchValue, equals vs is_identical_to, 换型
use std::ptr::null_mut;

use aok::{OK, Void};
use log::info;
use v8_handles::{
  DirectHandle, DirectHandleSmallVector, DirectHandleVector, Handle, HandleScope, HeapObject,
  Object, Smi, Tagged, is_smi,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 规约: Handle 是 tagged 指针的间接层 —— 解引用取槽内值
#[test]
fn indirect_dereference() -> Void {
  let scope = HandleScope::new();
  let tagged = Tagged::<HeapObject>::from_ptr(0x1234);
  let h = Handle::new(tagged, &scope);
  assert_eq!(h.value().ptr(), 0x1234);
  // location 即槽地址, address() 与之相等
  assert_eq!(h.address(), h.location() as usize);
  OK
}

/// 规约: PatchValue 原地改槽值, 同 location 的所有副本可见
#[test]
fn patch_value_shared_location() -> Void {
  let scope = HandleScope::new();
  let h = Handle::new(Tagged::<Object>::from_ptr(0x1111), &scope);
  let alias = Handle::from_location(h.location());
  assert!(h.equals(&alias));
  h.patch_value(Tagged::<Object>::from_ptr(0x2222));
  assert_eq!(alias.value().ptr(), 0x2222, "同 location 副本应见新值");
  assert_eq!(h.value().ptr(), 0x2222);
  OK
}

/// 规约: equals 比 location, is_identical_to 比对象身份 —— 不同槽同值时语义分叉
#[test]
fn equals_vs_is_identical_to() -> Void {
  let scope = HandleScope::new();
  let a = Handle::new(Tagged::<Object>::from_ptr(0x9001), &scope);
  let b = Handle::new(Tagged::<Object>::from_ptr(0x9001), &scope);
  let same_slot = Handle::from_location(a.location());

  assert!(!a.equals(&b), "不同槽 location 不等");
  assert!(a.is_identical_to(&b), "槽值相等即同一对象");
  assert!(a.equals(&same_slot));
  assert!(a.is_identical_to(&same_slot));
  OK
}

/// 规约: 空句柄 —— is_null, 与任何句柄不 identical
#[test]
fn null_handle_semantics() -> Void {
  let scope = HandleScope::new();
  let h = Handle::<Object>::null();
  assert!(h.is_null());
  assert_eq!(h.location(), null_mut());
  assert_eq!(h.address(), 0);
  let real = Handle::new(Tagged::<Object>::from_ptr(1), &scope);
  assert!(!h.is_identical_to(&real));
  assert!(!real.is_identical_to(&h));
  assert!(h.is_identical_to(&Handle::<Object>::null()));
  OK
}

/// 规约: UncheckedCast 换型复用 location; as_object 上转顶层
#[test]
fn unchecked_cast_reuses_location() -> Void {
  let scope = HandleScope::new();
  let h = Handle::new(Tagged::<HeapObject>::from_ptr(0x501), &scope);
  let obj = h.unchecked_cast::<Object>();
  assert_eq!(obj.location(), h.location());
  assert_eq!(obj.value().ptr(), 0x501);
  let top = h.as_object();
  assert_eq!(top.location(), h.location());
  OK
}

/// 规约: Smi 编解码 31 位 (ptr = v << 1), is_smi 判定
#[test]
fn tagged_smi_encoding() -> Void {
  let s = Tagged::<Smi>::smi(42);
  assert_eq!(s.ptr(), 84);
  assert_eq!(s.value(), 42);
  assert!(is_smi(s.ptr()));
  assert!(!is_smi(0x1001), "堆对象 tag=1 非 Smi");
  let neg = Tagged::<Smi>::smi(-7);
  assert_eq!(neg.value(), -7);
  OK
}

/// 规约: DirectHandle 包装 Handle —— 值/相等/改值均转发
#[test]
fn direct_handle_wraps_handle() -> Void {
  let scope = HandleScope::new();
  let d = DirectHandle::new(Tagged::<Object>::from_ptr(0x321), &scope);
  assert!(!d.is_null());
  assert_eq!(d.value().ptr(), 0x321);
  assert_eq!(d.address(), d.indirect().address());

  let d2 = DirectHandle::new(Tagged::<Object>::from_ptr(0x321), &scope);
  assert!(!d.equals(&d2));
  assert!(d.is_identical_to(&d2));

  let mut d3 = DirectHandle::new(Tagged::<Object>::from_ptr(0x1), &scope);
  d3.set_value(Tagged::<Object>::from_ptr(0x999));
  assert_eq!(d3.value().ptr(), 0x999);

  let null = DirectHandle::<Object>::null();
  assert!(null.is_null());
  assert!(!d.is_null());
  OK
}

/// 规约: DirectHandle::from_slot 空槽 → null; from_address 分配槽
#[test]
fn direct_handle_from_slot_and_address() -> Void {
  let scope = HandleScope::new();
  let empty = DirectHandle::<Object>::from_slot(null_mut());
  assert!(empty.is_null());
  let d = DirectHandle::<Object>::from_address(0x777, &scope);
  assert_eq!(d.value().ptr(), 0x777);
  // from_slot 非空槽: 与原句柄同 location
  let h = Handle::new(Tagged::<Object>::from_ptr(0x778), &scope);
  let from_slot = DirectHandle::<Object>::from_slot(h.location());
  assert_eq!(from_slot.address(), h.address());
  OK
}

/// 规约: DirectHandleVector 容器面 —— push/len/get/swap
#[test]
fn direct_handle_vector_container() -> Void {
  let scope = HandleScope::new();
  let mut v = DirectHandleVector::new();
  assert!(v.is_empty());
  for i in 0..5 {
    v.push(DirectHandle::new(
      Tagged::<Object>::from_ptr(0x10 + i),
      &scope,
    ));
  }
  assert_eq!(v.len(), 5);
  assert_eq!(v.get(2).map(|d| d.value().ptr()), Some(0x12));
  assert_eq!(v.back().map(|d| d.value().ptr()), Some(0x14));
  assert_eq!(v.front().map(|d| d.value().ptr()), Some(0x10));
  let mut w = DirectHandleVector::with_len(1);
  v.swap(&mut w);
  assert_eq!(w.len(), 5);
  assert_eq!(v.len(), 1);
  OK
}

/// 规约: DirectHandleSmallVector 容器面 (SmallVector 后备)
#[test]
fn direct_handle_small_vector_container() -> Void {
  let scope = HandleScope::new();
  let mut sv = DirectHandleSmallVector::<Object, 4>::new();
  assert_eq!(DirectHandleSmallVector::<Object, 4>::K_INLINE_SIZE, 4);
  for i in 0..6 {
    sv.push(DirectHandle::new(
      Tagged::<Object>::from_ptr(0x40 + i),
      &scope,
    ));
  }
  assert_eq!(sv.len(), 6);
  assert_eq!(
    sv.get(5).unwrap_or(&DirectHandle::null()).value().ptr(),
    0x45
  );
  sv.pop(2);
  assert_eq!(sv.len(), 4);
  info!("small_vector len {}", sv.len());
  OK
}
