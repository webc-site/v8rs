//! 压缩值包装往返 (tagged-value-inl.h: StrongTaggedValue / TaggedValue)
//! 钉死测试态: 主 cage base = 0, 压缩截断低 32 位、解压零扩展。

use aok::{OK, Void};
use v8_objects_core::tagged_value_inl::{StrongTaggedValue, TaggedValue};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_strong_tagged_value_roundtrip() -> Void {
  // from_object (CompressObject) → to_object (DecompressTagged) 全等
  let obj = 0x1001usize;
  let wrapped = StrongTaggedValue::from_object(obj);
  assert_eq!(wrapped.raw, obj as u32);
  assert_eq!(wrapped.to_object(0), obj);

  OK
}

#[test]
fn test_tagged_value_roundtrip() -> Void {
  // from_maybe_object (CompressAny) → to_maybe_object / trusted 解压全等 (cage base=0)
  let value = 0x2001usize;
  let wrapped = TaggedValue::from_maybe_object(value);
  assert_eq!(wrapped.raw, value as u32);
  assert_eq!(wrapped.to_maybe_object(0), value);
  assert_eq!(wrapped.to_maybe_object_trusted(0), value);

  OK
}
