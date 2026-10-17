//! 测试 casting_inl 与 api_callbacks_inl 的类型转型与回调语义

use aok::{OK, Void};
use v8_objects_core::{
  Error,
  api_callbacks_inl::{
    SideEffectType, accessor_info_has_getter, accessor_info_has_setter,
    accessor_info_setter_side_effect_type, clear_padding, get_holder_for_api,
  },
  casting_inl::{
    GcCheckContext, GcState, cast, gc_aware_object_type_check, is, try_cast, unchecked_cast,
  },
  heap_number::HeapNumber,
  heap_object::HeapObject,
  map::Map,
  objects_body_descriptors::body_descriptor_of,
  oddball::Oddball,
  smi::Smi,
  symbol::Symbol,
  tagged::{Object, Tagged},
  union::{is_union, union_flatten},
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_casting_primitives() -> Void {
  let smi = Tagged::<Object>::new(42 << 1); // SMI (最低位为 0)
  assert!(is::<Smi>(smi));
  assert!(!is::<HeapObject>(smi));
  assert!(is::<Object>(smi));

  let tried_smi: Option<Tagged<Smi>> = try_cast::<Smi>(smi);
  assert!(tried_smi.is_some());
  assert_eq!(tried_smi.unwrap().ptr(), smi.ptr());

  let tried_ho: Option<Tagged<HeapObject>> = try_cast::<HeapObject>(smi);
  assert!(tried_ho.is_none());

  // Smi 非堆对象，无法转型为 Map / HeapNumber / Symbol / Oddball
  assert!(!is::<Map>(smi));
  assert!(!is::<HeapNumber>(smi));
  assert!(!is::<Symbol>(smi));
  assert!(!is::<Oddball>(smi));
  assert!(try_cast::<Map>(smi).is_none());

  let cast_smi = cast::<Smi>(smi);
  assert_eq!(cast_smi.ptr(), smi.ptr());

  let unchecked: Tagged<HeapObject> = unchecked_cast(smi);
  assert_eq!(unchecked.ptr(), smi.ptr());

  OK
}

#[test]
fn test_gc_aware_object_type_check() -> Void {
  let smi = Tagged::<Object>::new(10 << 1);
  assert!(gc_aware_object_type_check::<Smi>(smi, None, None));

  // 模拟堆对象指针
  let ho_ptr = Tagged::<Object>::new(0x2001); // 最低位 1 为堆对象标记

  // 1. Scavenge 下的大对象页
  let scavenge_large_ctx = GcCheckContext {
    gc_state: GcState::Scavenge,
    is_large_page: true,
    in_young_generation: false,
    is_conservative_pinning: false,
  };
  assert!(gc_aware_object_type_check::<HeapObject>(
    ho_ptr,
    Some(&scavenge_large_ctx),
    None
  ));

  // 2. 疏散且转发地址指向合法类型
  let gc_ctx = GcCheckContext {
    gc_state: GcState::MarkCompact,
    is_large_page: false,
    in_young_generation: false,
    is_conservative_pinning: false,
  };
  let forwarded_target = Tagged::<Object>::new(0x3001);
  assert!(gc_aware_object_type_check::<HeapObject>(
    ho_ptr,
    Some(&gc_ctx),
    Some(forwarded_target)
  ));

  OK
}

#[test]
fn test_api_callbacks() -> Void {
  // 1. getter / setter 存在性
  assert!(accessor_info_has_getter(0x4000));
  assert!(!accessor_info_has_getter(0));
  assert!(accessor_info_has_setter(0x5000));
  assert!(!accessor_info_has_setter(0));

  // 2. side effect type 设置与禁止 HasNoSideEffect
  let mut flags: u32 = 0;
  assert!(accessor_info_setter_side_effect_type(&mut flags, SideEffectType::HasSideEffect).is_ok());
  assert_eq!((flags & 0x30) >> 4, SideEffectType::HasSideEffect as u32);

  assert!(
    accessor_info_setter_side_effect_type(&mut flags, SideEffectType::HasSideEffectToReceiver)
      .is_ok()
  );
  assert_eq!(
    (flags & 0x30) >> 4,
    SideEffectType::HasSideEffectToReceiver as u32
  );

  // 禁止 HasNoSideEffect
  let err = accessor_info_setter_side_effect_type(&mut flags, SideEffectType::HasNoSideEffect);
  assert!(matches!(err, Err(Error::InvalidSetterSideEffectType)));

  // 3. clear_padding
  let mut pad_buf = [0xffu8; 16];
  unsafe {
    clear_padding(pad_buf.as_mut_ptr() as usize, 4, 8);
  }
  assert_eq!(&pad_buf[4..12], &[0u8; 8]);
  assert_eq!(pad_buf[0], 0xff);

  // 4. get_holder_for_api
  let raw_holder = 0x8888;
  assert_eq!(get_holder_for_api(raw_holder, false, None), raw_holder);
  let proxy_fn: fn(usize) -> usize = |h| h + 1;
  assert_eq!(
    get_holder_for_api(raw_holder, true, Some(proxy_fn)),
    raw_holder + 1
  );

  OK
}

#[test]
fn test_body_descriptor_and_union() -> Void {
  // 1. body_descriptor_of
  assert_eq!(
    body_descriptor_of("HeapNumber"),
    Some("public DataOnlyBodyDescriptor")
  );
  assert!(body_descriptor_of("UnknownType").is_none());

  // 2. union
  assert!(!is_union::<Map>());
  assert_eq!(union_flatten(123), 123);

  OK
}
