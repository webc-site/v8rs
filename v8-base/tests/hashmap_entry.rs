//! v8-base/src/hashmap_entry.rs 集成测试 — 对应 v8/src/base/hashmap-entry.h
//!
//! 上游 v8/test/unittests 无 hashmap-entry 专测 (条目语义在 hashmap-unittest.cc 中
//! 间接覆盖), 本文件按头文件的 BitField 编码逐条断言:
//! - HashField<uint32_t,0,31> ⇒ kHashValueMask = 2^31-1, encode 截断高位
//! - ExistsField = HashField::Next<bool,1> ⇒ bit31, clear 仅清 bit31 (hash 保留)
//! - NoHashMapValue 特化: value 与 key 共 union ⇒ 条目不含独立 value 字段

use std::{
  ffi::c_void,
  mem::{align_of, size_of},
  ptr::{NonNull, null_mut},
};

use aok::{OK, Void};
use v8_base::hashmap_entry::{
  HashSetEntry, K_HASH_VALUE_MASK, NoHashMapValue, TemplateHashMapEntry,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

type PtrEntry = TemplateHashMapEntry<*mut c_void, *mut c_void>;

/// kHashValueMask = HashField::kMax = 2^31 - 1
#[test]
fn hashmap_entry_hash_value_mask() -> Void {
  assert_eq!(0x7FFF_FFFF_u32, K_HASH_VALUE_MASK);
  assert_eq!((1u32 << 31) - 1, K_HASH_VALUE_MASK);
  OK
}

/// ctor(Key, Value, hash) → HashField::encode(hash) | ExistsField::encode(true)
#[test]
fn hashmap_entry_new_sets_hash_and_exists() -> Void {
  let key: *mut c_void = NonNull::dangling().as_ptr();
  let value: *mut c_void = unsafe { key.add(1) };
  let e = PtrEntry::new(key, value, 12345);
  assert_eq!(12345, e.hash());
  assert!(e.exists());
  assert_eq!(key, e.key);
  assert_eq!(value, e.value);

  // HashField::encode 截断 bit31 以上 (0xFFFFFFFF → 0x7FFFFFFF)
  let e = PtrEntry::new(key, value, u32::MAX);
  assert_eq!(K_HASH_VALUE_MASK, e.hash());
  assert!(e.exists());

  // hash 含 bit31 时仍只保留 31 位
  let e = PtrEntry::new(key, value, 1 << 31);
  assert_eq!(0, e.hash());
  assert!(e.exists());
  OK
}

/// clear() = ExistsField::update(.., false): 仅清存在位, hash 字段不动
#[test]
fn hashmap_entry_clear_keeps_hash() -> Void {
  let mut e = PtrEntry::new(null_mut(), null_mut(), 777);
  assert!(e.exists());
  e.clear();
  assert!(!e.exists());
  assert_eq!(777, e.hash());
  // 重复 clear 幂等
  e.clear();
  assert!(!e.exists());
  assert_eq!(777, e.hash());

  // clear 后重建 (原版表复用槽位路径)
  let e = PtrEntry::new(0x9usize as *mut c_void, 0x8usize as *mut c_void, 1);
  assert!(e.exists());
  assert_eq!(1, e.hash());
  OK
}

/// Default = 原版 AllocateArray 的 memset 0 槽位
#[test]
fn hashmap_entry_default_is_empty_slot() -> Void {
  let e: PtrEntry = Default::default();
  assert!(!e.exists());
  assert_eq!(0, e.hash());
  assert!(e.key.is_null());
  assert!(e.value.is_null());

  let h: HashSetEntry<*mut c_void> = Default::default();
  assert!(!h.exists());
  assert_eq!(0, h.hash());
  assert!(h.key.is_null());
  OK
}

/// NoHashMapValue 特化: 不为 value 分配空间 (union { Key key; NoHashMapValue })
#[test]
fn hashmap_entry_hash_set_entry_no_value_space() -> Void {
  let ptr_w = size_of::<usize>();

  // 原版 union: key 与 NoHashMapValue 共址 → HashSetEntry 仅 key + hash_and_exists
  let hs = HashSetEntry::<*mut c_void>::new(0x5usize as *mut c_void, 42);
  assert_eq!(42, hs.hash());
  assert!(hs.exists());
  assert_eq!(0x5usize as *mut c_void, hs.key);
  assert_eq!(
    2 * ptr_w,
    size_of::<HashSetEntry<*mut c_void>>(),
    "key(8) + hash_and_exists(4) 按对齐补齐"
  );

  // 通用条目: key + value + hash_and_exists, 比 hashset 条目多一个 value 宽度
  let kv = TemplateHashMapEntry::<*mut c_void, *mut c_void>::new(
    0x5usize as *mut c_void,
    0x6usize as *mut c_void,
    42,
  );
  assert_eq!(42, kv.hash());
  assert!(kv.exists());
  assert_eq!(3 * ptr_w, size_of::<PtrEntry>());
  assert_eq!(
    ptr_w,
    size_of::<PtrEntry>() - size_of::<HashSetEntry<*mut c_void>>()
  );

  // repr(C): 字段顺序 key,value,hash_and_exists, 对齐与 C 布局一致
  assert_eq!(align_of::<usize>(), align_of::<PtrEntry>());
  assert_eq!(align_of::<usize>(), align_of::<HashSetEntry<*mut c_void>>());

  // clear/重建对 hashset 条目同样成立
  let mut hs = hs;
  hs.clear();
  assert!(!hs.exists());
  assert_eq!(42, hs.hash());
  OK
}

/// NoHashMapValue 标记类型 (原版 struct NoHashMapValue {})
#[test]
fn hashmap_entry_no_hash_map_value_marker() -> Void {
  let v = NoHashMapValue {};
  assert_eq!(NoHashMapValue {}, v);
  assert_eq!(NoHashMapValue {}, Default::default());
  assert_eq!(0, size_of::<NoHashMapValue>());
  OK
}
