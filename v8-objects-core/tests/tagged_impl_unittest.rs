use std::str::from_utf8;

use aok::{OK, Void};
use v8_objects_core::{
  heap_object::HeapObject,
  smi::Smi,
  tagged::{
    CLEARED_WEAK_VALUE, MaybeObject, Object, TAGGED_INDEX_MAX, TAGGED_INDEX_MIN, Tagged,
    TaggedIndex, make_strong, make_weak, make_weak_or_smi,
  },
  tagged_constants::WEAK_HEAP_OBJECT_TAG,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_make_weak_and_strong_roundtrip() -> Void {
  // 原版 tagged.h:912-932 MakeWeak/MakeStrong
  let strong = unsafe { Tagged::<HeapObject>::from_raw_address(0x1000) };

  // 弱化: 置 bit1 (0b01 → 0b11)
  let weak = make_weak(strong);
  assert_eq!(weak.ptr(), strong.ptr() | WEAK_HEAP_OBJECT_TAG);
  assert!(weak.is_weak());

  // 还原强引用: 清 bit1 保 bit0
  let back = make_strong(weak);
  assert_eq!(back, strong);
  assert!(back.is_heap_object());

  OK
}

#[test]
fn test_make_weak_or_smi() -> Void {
  // 原版 tagged.h: MakeWeakOrSmi — Smi 原样透传, 堆对象置弱位
  let smi = Smi::from_int(42);
  let weak_smi = make_weak_or_smi(smi.cast());
  assert!(weak_smi.is_smi());
  assert_eq!(weak_smi.ptr(), smi.ptr());

  let obj = unsafe { Tagged::<HeapObject>::from_raw_address(0x2000) }.cast::<Object>();
  let weak_obj = make_weak_or_smi(obj);
  assert!(weak_obj.is_weak_or_cleared());
  assert!(weak_obj.is_weak_live());

  OK
}

#[test]
fn test_maybe_object_predicates() -> Void {
  // 原版 tagged-impl.h 谓词族 (WeakTaggedBase, kCanBeWeak=true)
  let smi: Tagged<MaybeObject> = Smi::from_int(7).cast();
  let strong: Tagged<MaybeObject> =
    unsafe { Tagged::<HeapObject>::from_raw_address(0x1000) }.cast();
  let weak: Tagged<MaybeObject> =
    make_weak(unsafe { Tagged::<HeapObject>::from_raw_address(0x1000) }).cast();
  let cleared = Tagged::<MaybeObject>::cleared();

  // Smi 态
  assert!(smi.is_smi());
  assert!(smi.is_strong_or_smi());
  assert!(!smi.is_strong());
  assert!(!smi.is_weak_or_cleared());
  assert!(!smi.is_cleared());

  // 强引用态
  assert!(strong.is_strong());
  assert!(strong.is_strong_or_weak());
  assert!(strong.is_object());
  assert!(!strong.is_cleared());
  assert!(!strong.is_weak_live());

  // 活弱引用态
  assert!(weak.is_weak_live());
  assert!(weak.is_weak_or_cleared());
  assert!(weak.is_strong_or_weak());
  assert!(!weak.is_strong());
  assert!(!weak.is_cleared());
  assert!(!weak.is_strong_or_smi());

  // 已清除弱引用态: 低 32 位 == 0b11
  assert_eq!(cleared.ptr(), CLEARED_WEAK_VALUE);
  assert!(cleared.is_cleared());
  assert!(cleared.is_weak_or_cleared());
  assert!(!cleared.is_weak_live()); // IsWeak 排除 cleared
  assert!(!cleared.is_strong_or_weak());
  assert!(!cleared.is_strong_or_smi());

  OK
}

#[test]
fn test_maybe_object_getters() -> Void {
  // 原版 tagged-impl-inl.h GetHeapObject 系列
  let strong_raw = 0x1000;
  let strong: Tagged<MaybeObject> =
    unsafe { Tagged::<HeapObject>::from_raw_address(strong_raw) }.cast();
  let weak: Tagged<MaybeObject> =
    make_weak(unsafe { Tagged::<HeapObject>::from_raw_address(strong_raw) }).cast();
  let smi: Tagged<MaybeObject> = Smi::from_int(-3).cast();
  let cleared = Tagged::<MaybeObject>::cleared();

  // 强引用取出
  assert_eq!(
    strong.get_heap_object_if_strong().map(|t| t.ptr()),
    Some(strong_raw | 1)
  );
  assert_eq!(strong.get_heap_object_if_weak(), None);
  assert_eq!(
    strong.get_heap_object().map(|t| t.ptr()),
    Some(strong_raw | 1)
  );

  // 弱引用取出 (清 bit1 还原强指针)
  assert_eq!(
    weak.get_heap_object_if_weak().map(|t| t.ptr()),
    Some(strong_raw | 1)
  );
  assert_eq!(weak.get_heap_object_if_strong(), None);
  assert_eq!(
    weak.get_heap_object().map(|t| t.ptr()),
    Some(strong_raw | 1)
  );

  // Smi 取出
  assert_eq!(smi.get_heap_object(), None);
  assert_eq!(smi.as_smi().map(|t| t.value()), Some(-3));
  assert!(smi.get_heap_object_or_smi().is_smi());

  // 堆对象走 get_heap_object_or_smi 还原
  assert_eq!(weak.get_heap_object_or_smi().ptr(), strong_raw | 1);

  // 已清除态: 无堆对象可取
  assert_eq!(cleared.get_heap_object(), None);
  assert_eq!(cleared.get_heap_object_if_weak(), None);
  assert_eq!(cleared.get_heap_object_if_strong(), None);

  OK
}

#[test]
fn test_tagged_index() -> Void {
  // 原版 tagged-index.h: 31 位恒符号扩展编码
  assert_eq!(TAGGED_INDEX_MIN, -(2i64.pow(30)));
  assert_eq!(TAGGED_INDEX_MAX, 2i64.pow(30) - 1);

  for &v in &[0isize, 1, -1, 42, -42, 1 << 29, -(1 << 29)] {
    assert!(TaggedIndex::is_valid(v));
    let tagged = TaggedIndex::from_intptr(v);
    assert_eq!(TaggedIndex::value(tagged), v);
    // 编码 = (value << 1) | 0, 恒符号扩展
    assert_eq!(tagged.ptr(), ((v as usize) << 1));
  }

  // 值域边界与越界
  assert!(TaggedIndex::is_valid(TAGGED_INDEX_MIN as isize));
  assert!(TaggedIndex::is_valid(TAGGED_INDEX_MAX as isize));
  assert!(!TaggedIndex::is_valid(TAGGED_INDEX_MIN as isize - 1));
  assert!(!TaggedIndex::is_valid(TAGGED_INDEX_MAX as isize + 1));

  OK
}

#[test]
fn test_tagged_impl_functions() -> Void {
  use v8_objects_core::tagged_impl::{
    HeapObjectReferenceType, cage_base_checks, check_object_comparison_allowed, get_heap_object,
    get_heap_object_assume_strong, get_heap_object_assume_weak, get_heap_object_if_strong,
    get_heap_object_if_weak, get_heap_object_or_smi, get_heap_object_out,
    get_heap_object_with_ref_type, to_smi, to_smi_out,
  };

  // Smi 测试
  let smi_val = Smi::from_int(123);
  let mut smi_out = Tagged::<Smi>::new(0);
  assert!(to_smi_out(smi_val.ptr(), &mut smi_out));
  assert_eq!(smi_out.value(), 123);
  assert_eq!(to_smi(smi_val.ptr()).value(), 123);

  // 堆对象强引用测试
  let strong_ptr = 0x2001;
  assert!(!to_smi_out(strong_ptr, &mut smi_out));

  let mut heap_out = Tagged::<HeapObject>::NULL;
  assert!(get_heap_object_out(strong_ptr, &mut heap_out));
  assert_eq!(heap_out.ptr(), 0x2001);

  let mut ref_type = HeapObjectReferenceType::WEAK;
  assert!(get_heap_object_with_ref_type(
    strong_ptr,
    &mut heap_out,
    &mut ref_type
  ));
  assert_eq!(ref_type, HeapObjectReferenceType::STRONG);
  assert_eq!(heap_out.ptr(), 0x2001);

  assert!(get_heap_object_if_strong(strong_ptr, &mut heap_out));
  assert_eq!(heap_out.ptr(), 0x2001);
  assert_eq!(get_heap_object_assume_strong(strong_ptr).ptr(), 0x2001);

  assert!(!get_heap_object_if_weak(strong_ptr, &mut heap_out));

  // 堆对象弱引用测试
  let weak_ptr = 0x2003; // bit1 为 1
  assert!(get_heap_object_if_weak(weak_ptr, &mut heap_out));
  assert_eq!(heap_out.ptr(), 0x2001); // 剥掉弱引用 bit1
  assert_eq!(get_heap_object_assume_weak(weak_ptr).ptr(), 0x2001);
  assert!(!get_heap_object_if_strong(weak_ptr, &mut heap_out));

  assert!(get_heap_object_with_ref_type(
    weak_ptr,
    &mut heap_out,
    &mut ref_type
  ));
  assert_eq!(ref_type, HeapObjectReferenceType::WEAK);
  assert_eq!(heap_out.ptr(), 0x2001);

  // get_heap_object & get_heap_object_or_smi
  assert_eq!(get_heap_object(weak_ptr).ptr(), 0x2001);
  assert_eq!(get_heap_object_or_smi(smi_val.ptr()).ptr(), smi_val.ptr());
  assert_eq!(get_heap_object_or_smi(weak_ptr).ptr(), 0x2001);

  // cage_base_checks
  let (in_main, in_trusted) = cage_base_checks(0x2001);
  assert!(in_main);
  assert!(in_trusted);

  // check_object_comparison_allowed
  assert!(check_object_comparison_allowed(smi_val.ptr(), strong_ptr));
  assert!(check_object_comparison_allowed(strong_ptr, strong_ptr));

  // safe_equals
  use v8_objects_core::tagged_impl::safe_equals;
  assert!(safe_equals(strong_ptr, strong_ptr));
  assert!(!safe_equals(strong_ptr, weak_ptr));
  let tagged_a = Tagged::<Object>::new(0x2001);
  let tagged_b = Tagged::<HeapObject>::new(0x2001);
  let tagged_c = Tagged::<Object>::new(0x3001);
  assert!(tagged_a.safe_equals(tagged_b));
  assert!(!tagged_a.safe_equals(tagged_c));

  OK
}

#[test]
fn test_tagged_impl_print() -> Void {
  use v8_objects_core::tagged_impl::{print, short_print_ostream, short_print_string_stream};

  // 1. Smi 打印
  let mut buf = Vec::new();
  let smi = Smi::from_int(42);
  print(smi.ptr(), &mut buf)?;
  let smi_str = String::from_utf8(buf)?;
  assert!(smi_str.starts_with("Smi: "));
  assert!(smi_str.contains("(42)"));

  // 2. [cleared] 弱引用已清除态
  let mut buf = Vec::new();
  print(CLEARED_WEAK_VALUE, &mut buf)?;
  assert_eq!(from_utf8(&buf)?, "[cleared]");

  // 3. [weak] 弱引用
  let mut buf = Vec::new();
  print(0x2003, &mut buf)?; // 弱引用 bit 1
  assert_eq!(from_utf8(&buf)?, "[weak] HeapObject: 0x2000");

  // 4. 强引用
  let mut buf = Vec::new();
  print(0x2001, &mut buf)?;
  assert_eq!(from_utf8(&buf)?, "HeapObject: 0x2000");

  // 5. short_print_string_stream & short_print_ostream
  let mut s = String::new();
  short_print_string_stream(0x1234, &mut s);
  assert_eq!(s, "TaggedImpl(0x1234)");

  let mut stream_buf = Vec::new();
  short_print_ostream(0x5678, &mut stream_buf)?;
  assert_eq!(from_utf8(&stream_buf)?, "TaggedImpl(0x5678)");

  OK
}
