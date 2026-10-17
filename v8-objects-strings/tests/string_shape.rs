//! StringShape 派发与 snapshot 布局常量规约 (string.h + string-inl.h:160-501)。

use aok::{OK, Void};
use v8_objects_core::instance_type::InstanceType as It;
use v8_objects_strings::string::{
  StringKind, StringShape, is_in_place_internalizable_excluding_external,
  is_in_place_internalizable_type, prefix_for_debug_print, suffix_for_debug_print,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// snapshot 布局锚点 (压缩默认 kTaggedSize=4; string.h + v8-internal.h:1014)。
#[test]
fn layout_snapshot_anchors() -> Void {
  use v8_objects_strings::string::*;
  assert_eq!(K_HEADER_SIZE, 12);
  assert_eq!(K_HASH_FIELD_OFFSET, 4);
  assert_eq!(K_LENGTH_OFFSET, 8);
  assert_eq!(K_SEQ_CHARS_OFFSET, 12);
  assert_eq!(K_CONS_FIRST_OFFSET, 12);
  assert_eq!(K_CONS_SECOND_OFFSET, 16);
  assert_eq!(K_CONS_SIZE, 20);
  assert_eq!(K_THIN_ACTUAL_OFFSET, 12);
  assert_eq!(K_THIN_SIZE, 16);
  assert_eq!(K_SLICED_PARENT_OFFSET, 12);
  assert_eq!(K_SLICED_OFFSET_OFFSET, 16);
  assert_eq!(K_SLICED_SIZE, 20);
  assert_eq!(K_EXTERNAL_RESOURCE_OFFSET, 12);
  assert_eq!(K_EXTERNAL_RESOURCE_DATA_OFFSET, 16);
  assert_eq!(K_EXTERNAL_SIZE, 20);
  assert_eq!(K_UNCACHED_EXTERNAL_SIZE, 16);
  // 64 位 kMaxLength (v8-primitive.h:129) 与 Smi/int 容量约束 (string.h:823-837)
  assert_eq!(K_MAX_LENGTH, (1 << 29) - 24);
  assert_eq!(CONS_STRING_MIN_LENGTH, 13);
  assert_eq!(SLICED_STRING_MIN_LENGTH, 13);
  assert_eq!(K_MAX_HASH_CALC_LENGTH, 16383);
  OK
}

/// StringKind::of 全 24 字符串实例类型派发表 (string-inl.h:407 switch)。
#[test]
fn string_kind_dispatch_table() -> Void {
  let table = [
    (It::InternalizedTwoByteString, StringKind::SeqTwoByte),
    (It::InternalizedOneByteString, StringKind::SeqOneByte),
    (
      It::ExternalInternalizedTwoByteString,
      StringKind::ExternalTwoByte,
    ),
    (
      It::ExternalInternalizedOneByteString,
      StringKind::ExternalOneByte,
    ),
    (
      It::UncachedExternalInternalizedTwoByteString,
      StringKind::ExternalTwoByte,
    ),
    (
      It::UncachedExternalInternalizedOneByteString,
      StringKind::ExternalOneByte,
    ),
    (It::SeqTwoByteString, StringKind::SeqTwoByte),
    (It::SeqOneByteString, StringKind::SeqOneByte),
    (It::ConsTwoByteString, StringKind::Cons),
    (It::ConsOneByteString, StringKind::Cons),
    (It::SlicedTwoByteString, StringKind::Sliced),
    (It::SlicedOneByteString, StringKind::Sliced),
    (It::ExternalTwoByteString, StringKind::ExternalTwoByte),
    (It::ExternalOneByteString, StringKind::ExternalOneByte),
    (
      It::UncachedExternalTwoByteString,
      StringKind::ExternalTwoByte,
    ),
    (
      It::UncachedExternalOneByteString,
      StringKind::ExternalOneByte,
    ),
    (It::ThinTwoByteString, StringKind::Thin),
    (It::ThinOneByteString, StringKind::Thin),
    (It::SharedSeqTwoByteString, StringKind::SeqTwoByte),
    (It::SharedSeqOneByteString, StringKind::SeqOneByte),
    (It::SharedExternalTwoByteString, StringKind::ExternalTwoByte),
    (It::SharedExternalOneByteString, StringKind::ExternalOneByte),
    (
      It::SharedUncachedExternalTwoByteString,
      StringKind::ExternalTwoByte,
    ),
    (
      It::SharedUncachedExternalOneByteString,
      StringKind::ExternalOneByte,
    ),
  ];
  for &(ty, want) in &table {
    assert_eq!(StringKind::of(ty), want, "ty={:?}", ty);
  }
  OK
}

/// StringShape 谓词矩阵 (string-inl.h:180-457)。
#[test]
fn string_shape_predicates() -> Void {
  let seq1 = StringShape::from_instance_type(It::SeqOneByteString);
  let seq2 = StringShape::from_instance_type(It::SeqTwoByteString);
  let cons = StringShape::from_instance_type(It::ConsOneByteString);
  let sliced = StringShape::from_instance_type(It::SlicedTwoByteString);
  let thin = StringShape::from_instance_type(It::ThinOneByteString);
  let ext1 = StringShape::from_instance_type(It::ExternalOneByteString);
  let ext2 = StringShape::from_instance_type(It::ExternalTwoByteString);
  let uncached = StringShape::from_instance_type(It::UncachedExternalOneByteString);
  let shared = StringShape::from_instance_type(It::SharedSeqOneByteString);
  let internalized = StringShape::from_instance_type(It::InternalizedOneByteString);

  assert!(seq1.is_one_byte() && seq1.is_sequential() && seq1.is_direct());
  assert!(!seq1.is_two_byte() && !seq1.is_indirect() && !seq1.is_external());
  assert!(seq1.is_sequential_one_byte() && !seq1.is_sequential_two_byte());

  assert!(seq2.is_two_byte() && seq2.is_sequential_two_byte());
  assert!(cons.is_cons() && cons.is_indirect() && !cons.is_direct() && cons.is_one_byte());
  assert!(sliced.is_sliced() && sliced.is_indirect() && sliced.is_two_byte());
  assert!(thin.is_thin() && thin.is_indirect());
  assert!(ext1.is_external() && ext1.is_direct() && ext1.is_external_one_byte());
  assert!(ext2.is_external_two_byte());
  assert!(uncached.is_uncached_external());
  assert!(shared.is_shared());
  assert!(internalized.is_internalized());
  assert!(!seq1.is_shared() && !seq1.is_internalized() && !seq1.is_uncached_external());
  OK
}

/// IsInPlaceInternalizable 表 (string-inl.h:1929-1951)。
#[test]
fn in_place_internalizable() -> Void {
  assert!(is_in_place_internalizable_type(It::SeqOneByteString));
  assert!(is_in_place_internalizable_type(It::SeqTwoByteString));
  assert!(is_in_place_internalizable_type(It::SharedSeqOneByteString));
  assert!(is_in_place_internalizable_type(It::SharedSeqTwoByteString));
  assert!(is_in_place_internalizable_type(It::ExternalOneByteString));
  assert!(is_in_place_internalizable_type(It::ExternalTwoByteString));
  assert!(is_in_place_internalizable_type(
    It::SharedExternalOneByteString
  ));
  assert!(is_in_place_internalizable_type(
    It::SharedExternalTwoByteString
  ));
  // Cons/Sliced/Thin/内部化型不可原地内部化
  for ty in [
    It::ConsOneByteString,
    It::SlicedTwoByteString,
    It::ThinOneByteString,
    It::InternalizedOneByteString,
    It::UncachedExternalOneByteString,
  ] {
    assert!(!is_in_place_internalizable_type(ty), "{:?}", ty);
  }
  // 排除 External: Seq 系保留, External 系剔除
  assert!(is_in_place_internalizable_excluding_external(
    It::SeqOneByteString
  ));
  assert!(!is_in_place_internalizable_excluding_external(
    It::ExternalOneByteString
  ));
  OK
}

/// Prefix/SuffixForDebugPrint 表 (string.cc:589-623)。
#[test]
fn debug_print_affixes() -> Void {
  use v8_objects_core::instance_type::InstanceType as It;
  let cases = [
    (It::SeqOneByteString, "\"", "\""),
    (It::SeqTwoByteString, "u\"", "\""),
    (It::InternalizedOneByteString, "#", ""),
    (It::InternalizedTwoByteString, "u#", ""),
    (It::ConsOneByteString, "c\"", "\""),
    (It::ThinOneByteString, ">\"", "\""),
    (It::ExternalOneByteString, "e\"", "\""),
    (It::UncachedExternalTwoByteString, "ue\"", "\""),
  ];
  for (ty, prefix, suffix) in cases {
    let shape = StringShape::from_instance_type(ty);
    assert_eq!(prefix_for_debug_print(shape), prefix, "{:?}", ty);
    assert_eq!(suffix_for_debug_print(shape), suffix, "{:?}", ty);
  }
  OK
}
