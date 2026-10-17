// 上游 address-map 无独立单测 (随 snapshot/serializer 覆盖); 此处按
// PointerToIndexHashMap 语义补测: Set/Get 覆盖、低 32 位哈希截断、
// 80% 负载扩容、探测回绕。
use aok::{OK, Void};
use v8_utils::address_map::AddressToIndexHashMap;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// 基本 Set/Get 与覆盖写
#[test]
fn set_get_overwrite() -> Void {
  let mut m = AddressToIndexHashMap::new();
  assert!(m.is_empty());
  assert_eq!(m.get(0x1000_0001), None);
  m.set(0x1000_0001, 7);
  m.set(0x2000_0001, 9);
  assert_eq!(m.get(0x1000_0001), Some(7));
  assert_eq!(m.get(0x2000_0001), Some(9));
  assert_eq!(m.get(0x3000_0001), None);
  // 同键覆盖
  m.set(0x1000_0001, 42);
  assert_eq!(m.get(0x1000_0001), Some(42));
  assert_eq!(m.len(), 2);
  OK
}

// 低 32 位截断哈希: 仅高 32 位不同的键必碰撞 (同槽线性探测仍可区分)
#[test]
fn low32_hash_collision() -> Void {
  let mut m = AddressToIndexHashMap::new();
  let lo = 0x0000_1234u64;
  m.set(lo, 1); // hash = 0x1234
  m.set(lo | (0x1 << 32), 2); // hash 同为 0x1234 → 线性探测邻槽
  assert_eq!(m.get(lo), Some(1));
  assert_eq!(m.get(lo | (0x1 << 32)), Some(2));
  assert_eq!(m.len(), 2);
  OK
}

// 越过 80% 负载触发倍容, 全部键仍可查 (rehash 重插)
#[test]
fn grow_beyond_threshold() -> Void {
  let mut m = AddressToIndexHashMap::new();
  // 初始容量 8, occupancy + occupancy/4 >= capacity (即第 7 个插入) 触发倍容
  for i in 0..100u64 {
    m.set(i << 8 | 1, i as u32);
  }
  assert_eq!(m.len(), 100);
  for i in 0..100u64 {
    assert_eq!(m.get(i << 8 | 1), Some(i as u32), "key {i}");
  }
  OK
}

// 扩容后重置覆盖: 旧值不残留
#[test]
fn overwrite_after_grow() -> Void {
  let mut m = AddressToIndexHashMap::new();
  for i in 0..50u64 {
    m.set(i, i as u32);
  }
  for i in 0..50u64 {
    m.set(i, (i * 2) as u32);
  }
  assert_eq!(m.len(), 50);
  for i in 0..50u64 {
    assert_eq!(m.get(i), Some((i * 2) as u32));
  }
  OK
}
