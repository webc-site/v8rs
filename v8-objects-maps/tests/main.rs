use aok::{OK, Void};
use log::info;
use v8_objects_core::name::Name;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test() -> Void {
  info!("> test {}", 123456);
  OK
}

#[test]
fn test_name_tagged_ext_matches_v8_string() -> Void {
  use v8_objects_core::{map::Map, tagged::Tagged};
  use v8_objects_maps::NameTaggedExt;
  use v8_objects_strings::string::String as V8String;

  let null_name = Tagged::<Name>::new(0);
  let v8_str = V8String::new(Tagged::<Map>::new(0), 0);
  assert!(!null_name.matches_v8_string(&v8_str));
  OK
}

#[test]
fn test_property_descriptor_conversion() -> Void {
  use v8_objects_maps::property_descriptor::PropertyDescriptor;

  let mut desc = PropertyDescriptor::empty();
  desc.enumerable = true;
  desc.has_enumerable = true;
  desc.writable = true;
  desc.has_writable = true;
  desc.value = Some(42);

  let obj = desc.to_object("isolate-1");
  assert_ne!(obj, 0);

  let desc_obj = desc.to_property_descriptor_object("isolate-1");
  assert_ne!(desc_obj, 0);

  let mut incomplete = PropertyDescriptor::empty();
  PropertyDescriptor::complete_property_descriptor(&mut incomplete, "isolate-1");
  assert!(incomplete.has_enumerable);
  assert!(incomplete.has_configurable);
  assert!(incomplete.has_writable);

  OK
}
