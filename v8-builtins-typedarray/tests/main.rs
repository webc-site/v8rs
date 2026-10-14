use std::str::FromStr;

use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_builtins_typedarray::{
  BUFFER_TQ_DECLS, BufferBuiltin, DataViewOp, Error, TYPED_ARRAY_TQ_DECLS, TypedArrayBuiltin,
  TypedArrayElementsKind, array_buffer_constructor, array_buffer_constructor_do_not_initialize,
  array_buffer_prototype_resize, array_buffer_prototype_slice,
  array_buffer_prototype_slice_to_immutable, array_buffer_prototype_transfer,
  array_buffer_prototype_transfer_to_fixed_length, array_buffer_prototype_transfer_to_immutable,
  atomics_compare_exchange, atomics_exchange, atomics_is_lock_free, atomics_load, atomics_notify,
  atomics_pause, atomics_store, atomics_wait, atomics_wait_async, data_view_constructor,
  shared_array_buffer_prototype_get_byte_length, shared_array_buffer_prototype_grow,
  shared_array_buffer_prototype_slice, shared_array_constructor, shared_array_is_shared_array,
  typed_array_base_constructor, typed_array_constructor, typed_array_prototype_buffer,
  typed_array_prototype_byte_length, typed_array_prototype_byte_offset,
  typed_array_prototype_copy_within, typed_array_prototype_fill, typed_array_prototype_includes,
  typed_array_prototype_index_of, typed_array_prototype_last_index_of,
  typed_array_prototype_length, typed_array_prototype_reverse, typed_array_prototype_to_string_tag,
  uint8_array_from_base64, uint8_array_from_hex, uint8_array_prototype_set_from_base64,
  uint8_array_prototype_set_from_hex, uint8_array_prototype_to_base64,
  uint8_array_prototype_to_hex,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_typed_array_builtins_enum_and_table() -> Void {
  let all_builtins: Vec<_> = TypedArrayBuiltin::iter().collect();
  assert_eq!(all_builtins.len(), 19);

  let mut constructors = 0;
  let mut prototype_methods = 0;
  let mut search_methods = 0;
  let mut base64_hex_methods = 0;

  for b in &all_builtins {
    let name = b.name();
    let display_str = b.to_string();
    assert_eq!(name, display_str.as_str());

    let parsed = TypedArrayBuiltin::from_str(name).unwrap();
    assert_eq!(*b, parsed);

    let gid = b.global_id();
    assert!(
      gid.is_some(),
      "Builtin {} must exist in BUILTIN_TABLE",
      name
    );

    let desc = b.descriptor_name();
    assert!(desc.starts_with("Builtin_"));
    assert!(desc.ends_with("_InterfaceDescriptor"));

    if b.is_constructor() {
      constructors += 1;
    }
    if b.is_prototype_method() {
      prototype_methods += 1;
    }
    if b.is_search_method() {
      search_methods += 1;
    }
    if b.is_base64_or_hex() {
      base64_hex_methods += 1;
    }
  }

  assert_eq!(constructors, 2);
  assert_eq!(prototype_methods, 15);
  assert_eq!(search_methods, 3);
  assert_eq!(base64_hex_methods, 6);

  info!("test_typed_array_builtins_enum_and_table passed");
  OK
}

#[test]
fn test_typed_array_factory_functions() -> Void {
  assert_eq!(
    typed_array_base_constructor(),
    TypedArrayBuiltin::TypedArrayBaseConstructor
  );
  assert_eq!(
    typed_array_constructor(),
    TypedArrayBuiltin::TypedArrayConstructor
  );
  assert_eq!(
    typed_array_prototype_buffer(),
    TypedArrayBuiltin::TypedArrayPrototypeBuffer
  );
  assert_eq!(
    typed_array_prototype_byte_length(),
    TypedArrayBuiltin::TypedArrayPrototypeByteLength
  );
  assert_eq!(
    typed_array_prototype_byte_offset(),
    TypedArrayBuiltin::TypedArrayPrototypeByteOffset
  );
  assert_eq!(
    typed_array_prototype_copy_within(),
    TypedArrayBuiltin::TypedArrayPrototypeCopyWithin
  );
  assert_eq!(
    typed_array_prototype_fill(),
    TypedArrayBuiltin::TypedArrayPrototypeFill
  );
  assert_eq!(
    typed_array_prototype_includes(),
    TypedArrayBuiltin::TypedArrayPrototypeIncludes
  );
  assert_eq!(
    typed_array_prototype_index_of(),
    TypedArrayBuiltin::TypedArrayPrototypeIndexOf
  );
  assert_eq!(
    typed_array_prototype_last_index_of(),
    TypedArrayBuiltin::TypedArrayPrototypeLastIndexOf
  );
  assert_eq!(
    typed_array_prototype_length(),
    TypedArrayBuiltin::TypedArrayPrototypeLength
  );
  assert_eq!(
    typed_array_prototype_reverse(),
    TypedArrayBuiltin::TypedArrayPrototypeReverse
  );
  assert_eq!(
    typed_array_prototype_to_string_tag(),
    TypedArrayBuiltin::TypedArrayPrototypeToStringTag
  );
  assert_eq!(
    uint8_array_from_base64(),
    TypedArrayBuiltin::Uint8ArrayFromBase64
  );
  assert_eq!(uint8_array_from_hex(), TypedArrayBuiltin::Uint8ArrayFromHex);
  assert_eq!(
    uint8_array_prototype_set_from_base64(),
    TypedArrayBuiltin::Uint8ArrayPrototypeSetFromBase64
  );
  assert_eq!(
    uint8_array_prototype_set_from_hex(),
    TypedArrayBuiltin::Uint8ArrayPrototypeSetFromHex
  );
  assert_eq!(
    uint8_array_prototype_to_base64(),
    TypedArrayBuiltin::Uint8ArrayPrototypeToBase64
  );
  assert_eq!(
    uint8_array_prototype_to_hex(),
    TypedArrayBuiltin::Uint8ArrayPrototypeToHex
  );

  assert_eq!(TYPED_ARRAY_TQ_DECLS.len(), 61);

  info!("test_typed_array_factory_functions passed");
  OK
}

#[test]
fn test_buffer_builtins_enum_and_table() -> Void {
  let all_builtins: Vec<_> = BufferBuiltin::iter().collect();
  assert_eq!(all_builtins.len(), 23);

  let mut array_buffer_count = 0;
  let mut atomics_count = 0;
  let mut shared_array_buffer_count = 0;
  let mut dataview_count = 0;

  for b in &all_builtins {
    let name = b.name();
    let display_str = b.to_string();
    assert_eq!(name, display_str.as_str());

    let parsed = BufferBuiltin::from_str(name).unwrap();
    assert_eq!(*b, parsed);

    let gid = b.global_id();
    assert!(
      gid.is_some(),
      "Builtin {} must exist in BUILTIN_TABLE",
      name
    );

    let desc = b.descriptor_name();
    assert!(desc.starts_with("Builtin_"));
    assert!(desc.ends_with("_InterfaceDescriptor"));

    if b.is_array_buffer() {
      array_buffer_count += 1;
    }
    if b.is_atomics() {
      atomics_count += 1;
    }
    if b.is_shared_array_buffer() {
      shared_array_buffer_count += 1;
    }
    if b.is_dataview() {
      dataview_count += 1;
    }
  }

  assert_eq!(array_buffer_count, 8);
  assert_eq!(atomics_count, 9);
  assert_eq!(shared_array_buffer_count, 5);
  assert_eq!(dataview_count, 1);

  // 特殊序列化验证
  assert_eq!(
    BufferBuiltin::ArrayBufferConstructorDoNotInitialize.to_string(),
    "ArrayBufferConstructor_DoNotInitialize"
  );
  assert_eq!(
    BufferBuiltin::from_str("ArrayBufferConstructor_DoNotInitialize").unwrap(),
    BufferBuiltin::ArrayBufferConstructorDoNotInitialize
  );

  info!("test_buffer_builtins_enum_and_table passed");
  OK
}

#[test]
fn test_buffer_factory_functions() -> Void {
  assert_eq!(
    array_buffer_constructor(),
    BufferBuiltin::ArrayBufferConstructor
  );
  assert_eq!(
    array_buffer_constructor_do_not_initialize(),
    BufferBuiltin::ArrayBufferConstructorDoNotInitialize
  );
  assert_eq!(
    array_buffer_prototype_resize(),
    BufferBuiltin::ArrayBufferPrototypeResize
  );
  assert_eq!(
    array_buffer_prototype_slice(),
    BufferBuiltin::ArrayBufferPrototypeSlice
  );
  assert_eq!(
    array_buffer_prototype_slice_to_immutable(),
    BufferBuiltin::ArrayBufferPrototypeSliceToImmutable
  );
  assert_eq!(
    array_buffer_prototype_transfer(),
    BufferBuiltin::ArrayBufferPrototypeTransfer
  );
  assert_eq!(
    array_buffer_prototype_transfer_to_fixed_length(),
    BufferBuiltin::ArrayBufferPrototypeTransferToFixedLength
  );
  assert_eq!(
    array_buffer_prototype_transfer_to_immutable(),
    BufferBuiltin::ArrayBufferPrototypeTransferToImmutable
  );
  assert_eq!(
    atomics_compare_exchange(),
    BufferBuiltin::AtomicsCompareExchange
  );
  assert_eq!(atomics_exchange(), BufferBuiltin::AtomicsExchange);
  assert_eq!(atomics_is_lock_free(), BufferBuiltin::AtomicsIsLockFree);
  assert_eq!(atomics_load(), BufferBuiltin::AtomicsLoad);
  assert_eq!(atomics_notify(), BufferBuiltin::AtomicsNotify);
  assert_eq!(atomics_pause(), BufferBuiltin::AtomicsPause);
  assert_eq!(atomics_store(), BufferBuiltin::AtomicsStore);
  assert_eq!(atomics_wait(), BufferBuiltin::AtomicsWait);
  assert_eq!(atomics_wait_async(), BufferBuiltin::AtomicsWaitAsync);
  assert_eq!(data_view_constructor(), BufferBuiltin::DataViewConstructor);
  assert_eq!(
    shared_array_buffer_prototype_get_byte_length(),
    BufferBuiltin::SharedArrayBufferPrototypeGetByteLength
  );
  assert_eq!(
    shared_array_buffer_prototype_grow(),
    BufferBuiltin::SharedArrayBufferPrototypeGrow
  );
  assert_eq!(
    shared_array_buffer_prototype_slice(),
    BufferBuiltin::SharedArrayBufferPrototypeSlice
  );
  assert_eq!(
    shared_array_constructor(),
    BufferBuiltin::SharedArrayConstructor
  );
  assert_eq!(
    shared_array_is_shared_array(),
    BufferBuiltin::SharedArrayIsSharedArray
  );

  assert_eq!(BUFFER_TQ_DECLS.len(), 36);

  info!("test_buffer_factory_functions passed");
  OK
}

#[test]
fn test_dataview_ops_and_elements_kind() -> Void {
  let ops: Vec<_> = DataViewOp::iter().collect();
  assert_eq!(ops.len(), 21);

  for op in &ops {
    let s = op.as_str();
    assert!(!s.is_empty());
    if *op == DataViewOp::ByteLength {
      assert_eq!(s, "get DataView.prototype.byteLength");
      assert!(op.is_getter());
      assert!(!op.is_setter());
      assert_eq!(op.element_size(), 0);
    } else if op.is_getter() {
      assert!(s.starts_with("DataView.prototype.get"));
      assert!(!op.is_setter());
      assert!(op.element_size() > 0);
    } else if op.is_setter() {
      assert!(s.starts_with("DataView.prototype.set"));
      assert!(!op.is_getter());
      assert!(op.element_size() > 0);
    }
  }

  // 测试 TypedArrayElementsKind
  assert_eq!(TypedArrayElementsKind::Uint8.element_size(), 1);
  assert_eq!(TypedArrayElementsKind::Int8.element_size(), 1);
  assert_eq!(TypedArrayElementsKind::Uint16.element_size(), 2);
  assert_eq!(TypedArrayElementsKind::Int16.element_size(), 2);
  assert_eq!(TypedArrayElementsKind::Float16.element_size(), 2);
  assert_eq!(TypedArrayElementsKind::Uint32.element_size(), 4);
  assert_eq!(TypedArrayElementsKind::Int32.element_size(), 4);
  assert_eq!(TypedArrayElementsKind::Float32.element_size(), 4);
  assert_eq!(TypedArrayElementsKind::Float64.element_size(), 8);
  assert_eq!(TypedArrayElementsKind::BigInt64.element_size(), 8);
  assert_eq!(TypedArrayElementsKind::BigUint64.element_size(), 8);
  assert_eq!(TypedArrayElementsKind::Uint8Clamped.element_size(), 1);
  assert_eq!(TypedArrayElementsKind::RabGsabUint8.element_size(), 1);

  assert!(TypedArrayElementsKind::Uint8.is_uint8());
  assert!(TypedArrayElementsKind::Uint8Clamped.is_uint8());
  assert!(TypedArrayElementsKind::RabGsabUint8.is_uint8());
  assert!(!TypedArrayElementsKind::Int8.is_uint8());

  assert!(TypedArrayElementsKind::BigInt64.is_bigint());
  assert!(TypedArrayElementsKind::BigUint64.is_bigint());
  assert!(!TypedArrayElementsKind::Int32.is_bigint());

  assert!(TypedArrayElementsKind::Float16.is_float());
  assert!(TypedArrayElementsKind::Float32.is_float());
  assert!(TypedArrayElementsKind::Float64.is_float());
  assert!(!TypedArrayElementsKind::Uint32.is_float());

  info!("test_dataview_ops_and_elements_kind passed");
  OK
}

#[test]
fn test_error_formatting() -> Void {
  let err1 = Error::InvalidLength(42);
  assert_eq!(format!("{err1}"), "Invalid typed array length: 42");

  let err2 = Error::DetachedOrOutOfBounds;
  assert_eq!(format!("{err2}"), "Typed array detached or out of bounds");

  let err3 = Error::InvalidByteOffset {
    offset: 3,
    element_size: 4,
  };
  assert_eq!(
    format!("{err3}"),
    "Invalid byte offset 3 for element size 4"
  );

  let err4 = Error::InvalidDataViewOp("unknown".into());
  assert_eq!(format!("{err4}"), "Invalid data view operation: unknown");

  info!("test_error_formatting passed");
  OK
}
