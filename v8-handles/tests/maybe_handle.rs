//! MaybeHandle 族语义测试: 空/转换, is_identical_to, MaybeObjectHandle 弱/强引用
use aok::{OK, Void};
use log::info;
use v8_handles::{
  DirectHandle, Error, Handle, HandleScope, HeapObjectReferenceType, MaybeDirectHandle,
  MaybeHandle, MaybeObjectDirectHandle, MaybeObjectHandle, Object, Tagged, is_weak, make_weak,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 规约: 空可空句柄 —— is_null/is_empty/Default, 转换走 None/Err
#[test]
fn maybe_handle_null_and_conversion() -> Void {
  let scope = HandleScope::new();
  let m = MaybeHandle::<Object>::null();
  assert!(m.is_null());
  assert!(m.is_empty());
  assert!(MaybeHandle::<Object>::default().is_null());

  // 空 → None / Err(NullHandle)
  assert!(m.to_handle().is_none());
  assert!(matches!(m.to_handle_checked(), Err(Error::NullHandle)));

  // Handle → MaybeHandle 单向, 反向经检查
  let h = Handle::new(Tagged::<Object>::from_ptr(0x601), &scope);
  let m2 = MaybeHandle::from_handle(h);
  assert!(!m2.is_null());
  assert_eq!(m2.to_handle().map(|b| b.value().ptr()), Some(0x601));
  assert_eq!(m2.to_handle().map(|b| b.location()), Some(h.location()));
  assert_eq!(
    m2.to_handle_checked().ok().map(|b| b.value().ptr()),
    Some(0x601)
  );
  OK
}

/// 规约: MaybeHandle 语义检查面 Assert/Check (debug 断言)
#[test]
fn maybe_handle_assert_check() -> Void {
  let scope = HandleScope::new();
  let m = MaybeHandle::from_handle(Handle::new(Tagged::<Object>::from_ptr(1), &scope));
  m.assert();
  m.check();
  OK
}

/// 规约: equals 比 location; is_identical_to 语义不提供 (原版刻意省略)
#[test]
fn maybe_handle_equals_location() -> Void {
  let scope = HandleScope::new();
  let h1 = Handle::new(Tagged::<Object>::from_ptr(0x71), &scope);
  let h2 = Handle::new(Tagged::<Object>::from_ptr(0x71), &scope);
  let m1 = MaybeHandle::from_handle(h1);
  let m2 = MaybeHandle::from_handle(h2);
  let alias = MaybeHandle::from_handle(Handle::from_location(h1.location()));
  assert!(!m1.equals(&m2), "不同槽 location 不等");
  assert!(m1.equals(&alias));
  OK
}

/// 规约: MaybeDirectHandle 委托语义 —— 空转换/非空取值
#[test]
fn maybe_direct_handle_delegation() -> Void {
  let scope = HandleScope::new();
  let md = MaybeDirectHandle::<Object>::null();
  assert!(md.is_null() && md.is_empty());
  assert!(md.to_handle().is_none());

  let d = DirectHandle::new(Tagged::<Object>::from_ptr(0x81), &scope);
  let md2 = MaybeDirectHandle::from_direct(d);
  assert_eq!(md2.to_handle().map(|d| d.value().ptr()), Some(0x81));
  // MaybeHandle → MaybeDirectHandle 上转
  let h = Handle::new(Tagged::<Object>::from_ptr(0x82), &scope);
  let md3 = MaybeDirectHandle::from_maybe(MaybeHandle::from_handle(h));
  assert_eq!(md3.indirect().location(), h.location());
  OK
}

/// 规约: MaybeObjectHandle 弱引用 —— 读出复原弱 tag, 强引用原值
#[test]
fn maybe_object_handle_weak_strong() -> Void {
  let scope = HandleScope::new();
  let strong_ptr = 0x2001; // 堆对象 tag=1
  let mo = MaybeObjectHandle::strong(Tagged::<Object>::from_ptr(strong_ptr), &scope);
  assert_eq!(mo.reference_type(), HeapObjectReferenceType::Strong);
  assert_eq!(mo.value().ptr(), strong_ptr);
  assert_eq!(mo.deref().ptr(), strong_ptr);

  let mw = MaybeObjectHandle::weak(Tagged::<Object>::from_ptr(strong_ptr), &scope);
  assert_eq!(mw.reference_type(), HeapObjectReferenceType::Weak);
  let weak_ptr = mw.value().ptr();
  assert_eq!(weak_ptr, make_weak(strong_ptr));
  assert!(is_weak(weak_ptr));
  assert!(!is_weak(strong_ptr));

  // 句柄版构造
  let h = Handle::new(Tagged::<Object>::from_ptr(strong_ptr), &scope);
  assert_eq!(
    MaybeObjectHandle::from_handle(h).reference_type(),
    HeapObjectReferenceType::Strong
  );
  assert_eq!(
    MaybeObjectHandle::weak_from_handle(h).value().ptr(),
    make_weak(strong_ptr)
  );
  // 保活句柄与空判定
  assert!(!mw.is_null());
  assert_eq!(mw.object().ok().map(|h| h.value().ptr()), Some(strong_ptr));
  assert!(MaybeObjectHandle::new().is_null());
  OK
}

/// 规约: MaybeObjectHandle::is_identical_to —— 强度一致 + 对象一致
#[test]
fn maybe_object_handle_identical() -> Void {
  let scope = HandleScope::new();
  let ptr = 0x2009;
  let s1 = MaybeObjectHandle::strong(Tagged::<Object>::from_ptr(ptr), &scope);
  let s2 = MaybeObjectHandle::strong(Tagged::<Object>::from_ptr(ptr), &scope);
  let w1 = MaybeObjectHandle::weak(Tagged::<Object>::from_ptr(ptr), &scope);
  assert!(s1.is_identical_to(&s2));
  assert!(!s1.is_identical_to(&w1), "强度不同不 identical");
  assert!(!w1.is_identical_to(&s1));
  // 空对空
  assert!(MaybeObjectHandle::new().is_identical_to(&MaybeObjectHandle::new()));
  OK
}

/// 规约: MaybeObjectDirectHandle —— 直连版弱/强与转换
#[test]
fn maybe_object_direct_handle() -> Void {
  let scope = HandleScope::new();
  let ptr = 0x2011;
  let md_strong = MaybeObjectDirectHandle::strong(Tagged::<Object>::from_ptr(ptr), &scope);
  let md_weak = MaybeObjectDirectHandle::weak(Tagged::<Object>::from_ptr(ptr), &scope);
  assert_eq!(md_strong.value().ptr(), ptr);
  assert_eq!(md_weak.value().ptr(), make_weak(ptr));
  assert!(!md_strong.is_identical_to(&md_weak));
  assert!(MaybeObjectDirectHandle::new().is_null());

  // 直连句柄构造与 MaybeObjectHandle 转换
  let d = DirectHandle::new(Tagged::<Object>::from_ptr(ptr), &scope);
  let from_d = MaybeObjectDirectHandle::from_direct(d);
  assert_eq!(from_d.reference_type(), HeapObjectReferenceType::Strong);
  let weak_d = MaybeObjectDirectHandle::weak_from_direct(d);
  assert_eq!(weak_d.value().ptr(), make_weak(ptr));
  let mo = MaybeObjectHandle::weak(Tagged::<Object>::from_ptr(ptr), &scope);
  let converted = MaybeObjectDirectHandle::from_maybe_object(mo);
  assert_eq!(converted.reference_type(), HeapObjectReferenceType::Weak);
  assert_eq!(converted.value().ptr(), make_weak(ptr));
  info!("maybe_object_direct ok");
  OK
}

/// 规约: MaybeObjectHandle / MaybeObjectDirectHandle 空句柄解引用防护与 panic 免疫
#[test]
fn maybe_object_null_protection() -> Void {
  let mo_null = MaybeObjectHandle::new();
  assert!(mo_null.is_null());
  assert!(matches!(mo_null.object(), Err(Error::NullHandle)));

  let md_null = MaybeObjectDirectHandle::new();
  assert!(md_null.is_null());
  assert!(matches!(md_null.object(), Err(Error::NullHandle)));
  OK
}
