//! v8-base/src/hashmap.rs 集成测试 — 1:1 对应 hashmap-unittest.cc
//!
//! 原版用例: `HashmapTest.HashSet` (TestSet(Hash,100) + TestSet(CollisionHash,50))。
//! 原版测试文件仅此 1 例, 其余用例为本轮针对 hashmap.rs 公开 API 的补充断言
//! (增长阈值 / probe 空槽与 hash 掩码 / invalidate+initialize / clone / insert_new /
//! 自定义 matcher / 异质 LookupOrInsert / TemplateHashMap 视图 / Start-Next 遍历),
//! 均在函数注释中标注。

use std::{ffi::c_void, ptr};

use aok::{OK, Void};
use v8_base::{
  hashmap::{
    AllocationPolicy, CustomMatcherHashMap, DefaultAllocationPolicy, HashEqualityThenKeyMatcher,
    HashMap, KeyEqualityMatcher, MatchFn, TemplateHashMap, TemplateHashMapImpl,
  },
  hashmap_entry::{HashSetEntry, K_HASH_VALUE_MASK, TemplateHashMapEntry},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 `using IntKeyHash = uint32_t (*)(uint32_t key)`
type IntKeyHash = fn(i32) -> u32;

/// 原版 `reinterpret_cast<void*>(x)`
fn as_key(x: i32) -> *mut c_void {
  (x as isize) as usize as *mut c_void
}

/// 原版 class IntSet (CHECK 系列 → assert!)
struct IntSet {
  hash: IntKeyHash,
  map: HashMap,
}

impl IntSet {
  fn new(hash: IntKeyHash) -> Self {
    Self {
      hash,
      map: HashMap::new_default(),
    }
  }

  fn insert(&mut self, x: i32) {
    assert_ne!(0, x, "0 corresponds to (void*)nullptr - illegal key value");
    let key = as_key(x);
    let i = self
      .map
      .lookup_or_insert(key, (self.hash)(x))
      .expect("insert is set!");
    assert_eq!(key, self.map.entry(i).unwrap().key);
    // we don't care about p->value
  }

  fn remove(&mut self, x: i32) {
    assert_ne!(0, x, "0 corresponds to (void*)nullptr - illegal key value");
    let key = as_key(x);
    self.map.remove(&key, (self.hash)(x));
  }

  fn present(&self, x: i32) -> bool {
    let key = as_key(x);
    let p = self.map.lookup(&key, (self.hash)(x));
    if let Some(i) = p {
      assert_eq!(key, self.map.entry(i).unwrap().key);
    }
    p.is_some()
  }

  fn clear(&mut self) {
    self.map.clear();
  }

  /// 原版 occupancy(): 迭代计数须与表内计数一致
  fn occupancy(&self) -> u32 {
    let mut count: u32 = 0;
    let mut p = self.map.start();
    while let Some(i) = p {
      count += 1;
      p = self.map.next(i);
    }
    assert_eq!(self.map.occupancy(), count);
    count
  }
}

/// static uint32_t Hash(uint32_t key) { return 23; }
fn hash(_key: i32) -> u32 {
  23
}

/// static uint32_t CollisionHash(uint32_t key) { return key & 0x3; }
fn collision_hash(key: i32) -> u32 {
  (key as u32) & 0x3
}

/// 原版 AddWithWraparound(MulWithWraparound(x, factor), offset), int 回绕
fn next_x(x: i32) -> i32 {
  x.wrapping_mul(13).wrapping_add(7)
}

/// void TestSet(IntKeyHash hash, int size)
fn test_set(hash: IntKeyHash, size: u32) {
  let mut set = IntSet::new(hash);
  assert_eq!(0, set.occupancy());

  set.insert(1);
  set.insert(2);
  set.insert(3);
  assert_eq!(3, set.occupancy());

  set.insert(2);
  set.insert(3);
  assert_eq!(3, set.occupancy());

  assert!(set.present(1));
  assert!(set.present(2));
  assert!(set.present(3));
  assert!(!set.present(4));
  assert_eq!(3, set.occupancy());

  set.remove(1);
  assert!(!set.present(1));
  assert!(set.present(2));
  assert!(set.present(3));
  assert_eq!(2, set.occupancy());

  set.remove(3);
  assert!(!set.present(1));
  assert!(set.present(2));
  assert!(!set.present(3));
  assert_eq!(1, set.occupancy());

  set.clear();
  assert_eq!(0, set.occupancy());

  // Insert a long series of values.
  const START: i32 = 453;
  let n = size;

  let mut x = START;
  for i in 0..n {
    assert_eq!(i, set.occupancy());
    set.insert(x);
    x = next_x(x);
  }
  assert_eq!(n, set.occupancy());

  // Verify the same sequence of values.
  let mut x = START;
  for _ in 0..n {
    assert!(set.present(x));
    x = next_x(x);
  }
  assert_eq!(n, set.occupancy());

  // Remove all these values.
  let mut x = START;
  for i in 0..n {
    assert_eq!(n - i, set.occupancy());
    assert!(set.present(x));
    set.remove(x);
    assert!(!set.present(x));
    x = next_x(x);

    // Verify the expected values are still there.
    let mut y = START;
    for j in 0..n {
      if j <= i {
        assert!(!set.present(y));
      } else {
        assert!(set.present(y));
      }
      y = next_x(y);
    }
  }
  assert_eq!(0, set.occupancy());
}

/// 原版 TEST_F(HashmapTest, HashSet)
#[test]
fn hashmap_test_hash_set() -> Void {
  test_set(hash, 100);
  test_set(collision_hash, 50);
  OK
}

// ---------------------------------------------------------------------------
// 补充用例 (原版 hashmap-unittest.cc 无对应 TEST_F)
// ---------------------------------------------------------------------------

/// 80% 增长阈值: occupancy + occupancy/4 >= capacity ⇒ 扩容翻倍且条目全存活
#[test]
fn hashmap_test_growth_threshold() -> Void {
  let mut map = HashMap::new_default();
  assert_eq!(8, map.capacity());
  assert_eq!(0, map.occupancy());

  // capacity 8: 第 7 个条目使 7 + 7/4 = 8 >= 8 → resize 到 16
  for i in 1..=7i32 {
    let key = as_key(i);
    let slot = map.lookup_or_insert(key, hash(i)).expect("槽位可用");
    assert_eq!(key, map.entry(slot).unwrap().key);
  }
  assert_eq!(7, map.occupancy());
  assert_eq!(16, map.capacity());

  // 扩容后原有键仍可查
  for i in 1..=7i32 {
    assert!(map.lookup(&as_key(i), hash(i)).is_some());
  }

  // capacity 16: 需 occupancy >= 13 才再次扩容 (13 + 3 = 16)
  for i in 8..=12i32 {
    map.lookup_or_insert(as_key(i), hash(i));
  }
  assert_eq!(12, map.occupancy());
  assert_eq!(16, map.capacity());
  for i in 13..=20i32 {
    map.lookup_or_insert(as_key(i), hash(i));
  }
  assert_eq!(20, map.occupancy());
  assert_eq!(32, map.capacity());
  for i in 1..=20i32 {
    assert!(map.lookup(&as_key(i), hash(i)).is_some());
  }
  OK
}

/// probe 未命中返回空槽 (原版返回可写入的 Entry*), lookup 未命中返回 None
#[test]
fn hashmap_test_probe_vs_lookup() -> Void {
  let mut map = HashMap::new_default();
  map.lookup_or_insert(as_key(11), 3);

  let hit = map.probe(&as_key(11), 3).expect("表可用");
  assert!(map.entry(hit).is_some());
  assert_eq!(as_key(11), map.entry(hit).unwrap().key);

  let miss = map.probe(&as_key(22), 3).expect("表可用");
  // entry() 以 exists 过滤 → 空槽为 None
  assert!(map.entry(miss).is_none());
  assert_eq!(None, map.lookup(&as_key(22), 3));

  // 31 位 hash 截断: bit31 (exists 位) 被 Probe 掩码丢弃 → 同槽
  let high = map.probe(&as_key(22), 3 | (1 << 31)).expect("表可用");
  assert_eq!(miss, high);
  // 条目侧同样只存 31 位 hash
  let e = TemplateHashMapEntry::new(0u32, 0u32, 3 | (1 << 31) | (K_HASH_VALUE_MASK << 1));
  assert_eq!(3, e.hash() & 3);
  assert_eq!(
    (3 | (1 << 31) | (K_HASH_VALUE_MASK << 1)) & K_HASH_VALUE_MASK,
    e.hash()
  );
  assert!(e.exists());
  OK
}

/// DefaultAllocationPolicy 分配面 (原版 AllocateArray + memset 0 / DeleteArray)
fn default_policy_alloc_roundtrip() {
  let policy = DefaultAllocationPolicy;
  let buf: Vec<HashSetEntry<i32>> = policy.allocate_array(4);
  assert_eq!(4, buf.len());
  assert!(buf.iter().all(|e| !e.exists()));
  policy.delete_array(buf);
}

/// invalidate 后表不可用; initialize 重新可用 (原版 DCHECK 面以 None 承接)
#[test]
fn hashmap_test_invalidate_initialize() -> Void {
  default_policy_alloc_roundtrip();

  let mut map = HashMap::new_default();
  map.lookup_or_insert(as_key(1), 1);
  assert_eq!(1, map.occupancy());

  map.invalidate();
  assert_eq!(0, map.capacity());
  assert_eq!(0, map.occupancy());
  assert_eq!(None, map.probe(&as_key(1), 1));
  assert_eq!(None, map.lookup(&as_key(1), 1));
  assert_eq!(None, map.lookup_or_insert(as_key(1), 1));
  assert_eq!(None, map.start());

  map.initialize(4);
  assert_eq!(4, map.capacity());
  assert!(map.lookup_or_insert(as_key(1), 1).is_some());

  // Clear: 表保留容量, 条目清空
  map.clear();
  assert_eq!(0, map.occupancy());
  assert_eq!(4, map.capacity());
  assert_eq!(None, map.lookup(&as_key(1), 1));
  OK
}

/// clone_from_map: 逐槽位复制 (原版 memcpy 语义), 与原表互相独立
#[test]
fn hashmap_test_clone_from_map() -> Void {
  let mut map = HashMap::new_default();
  for i in 1..=5i32 {
    map.lookup_or_insert(as_key(i), collision_hash(i));
  }
  let cloned = map.clone_from_map(KeyEqualityMatcher::default(), DefaultAllocationPolicy);
  assert_eq!(map.capacity(), cloned.capacity());
  assert_eq!(map.occupancy(), cloned.occupancy());
  for i in 1..=5i32 {
    let a = map.lookup(&as_key(i), collision_hash(i));
    let b = cloned.lookup(&as_key(i), collision_hash(i));
    assert!(a.is_some() && b.is_some());
    assert_eq!(
      map.entry(a.unwrap()).unwrap().key,
      cloned.entry(b.unwrap()).unwrap().key
    );
  }
  // 副本只读快照: 原表删除不影响副本
  map.remove(&as_key(3), collision_hash(3));
  assert_eq!(4, map.occupancy());
  assert_eq!(5, cloned.occupancy());
  OK
}

/// insert_new 直接占空槽 (原版不查重); entry_mut 可改 value;
/// lookup_or_insert_with 提供 value 且命中既有键不覆盖
#[test]
fn hashmap_test_insert_new_and_entry_mut() -> Void {
  let mut map: TemplateHashMapImpl<
    *mut c_void,
    u32,
    KeyEqualityMatcher<*mut c_void>,
    DefaultAllocationPolicy,
  > = TemplateHashMapImpl::new(8, KeyEqualityMatcher::default(), DefaultAllocationPolicy);

  let s0 = map.insert_new(as_key(7), 5).expect("空槽");
  assert_eq!(0, map.entry(s0).unwrap().value);
  map.entry_mut(s0).unwrap().value = 99;
  assert_eq!(99, map.entry(s0).unwrap().value);

  // 不同 key 同 hash → 线性探测到下一个槽
  let s1 = map.insert_new(as_key(8), 5).expect("空槽");
  assert_ne!(s0, s1);
  assert_eq!(2, map.occupancy());

  // lookup_or_insert_with: 新键取 value_func, 既有键不覆盖
  let s2 = map
    .lookup_or_insert_with(as_key(9), 5, || 42)
    .expect("槽位");
  assert_eq!(42, map.entry(s2).unwrap().value);
  let s3 = map.lookup_or_insert_with(as_key(7), 5, || 1).expect("槽位");
  assert_eq!(s0, s3);
  assert_eq!(99, map.entry(s3).unwrap().value);
  assert_eq!(3, map.occupancy());

  // remove_at 直接按槽位删除并交回 value
  let v = map.remove_at(s3).expect("存在条目");
  assert_eq!(99, v);
  assert_eq!(2, map.occupancy());
  // key7 已删; 同 hash 集群前移回填, s3 槽位被 key8/key9 依次占据 (开放寻址删除语义)
  assert_eq!(None, map.lookup(&as_key(7), 5));
  assert!(map.lookup(&as_key(8), 5).is_some());
  assert!(map.lookup(&as_key(9), 5).is_some());
  OK
}

/// 自定义 matcher (原版 CustomMatcherHashMap: hash 相等后再走用户比较函数)
#[test]
fn hashmap_test_custom_matcher_hash_map() -> Void {
  // 以低 16 位作为"相等"判据, 模拟昂贵的自定义 key 比较
  fn custom_match(a: *mut c_void, b: *mut c_void) -> bool {
    (a as usize & 0xFFFF) == (b as usize & 0xFFFF)
  }
  let mut map = CustomMatcherHashMap::with_matcher(custom_match);
  let k1: *mut c_void = 0x1_0000usize as *mut c_void; // 低 16 位为 0
  let k2: *mut c_void = 0x2_0000usize as *mut c_void; // 低 16 位为 0 → 与 k1 "相等"
  let k3: *mut c_void = 0x3_0001usize as *mut c_void;
  let h = |k: *mut c_void| (k as usize >> 16) as u32;

  let i1 = map.lookup_or_insert(k1, h(k1)).expect("槽位");
  assert_eq!(k1, map.entry(i1).unwrap().key);
  // 同 hash 且自定义比较相等 → 命中, 不新增
  let i2 = map.lookup_or_insert(k2, h(k1)).expect("槽位");
  assert_eq!(i1, i2);
  assert_eq!(1, map.occupancy());
  // 不同 hash → 新条目
  let i3 = map.lookup_or_insert(k3, h(k3)).expect("槽位");
  assert_eq!(2, map.occupancy());
  assert!(map.lookup(&k3, h(k3)).is_some());
  assert_ne!(i1, i3);

  // HashEqualityThenKeyMatcher: hash 不等即短路, 不调用 key 比较
  let m: HashEqualityThenKeyMatcher<*mut c_void, fn(*mut c_void, *mut c_void) -> bool> =
    HashEqualityThenKeyMatcher::new(|_a, _b| true);
  assert!(!MatchFn::match_fn(&m, 1, 2, &k1, &k1));
  assert!(MatchFn::match_fn(&m, 1, 1, &k1, &k1));
  OK
}

/// 异质 LookupOrInsert (原版 LookupOrInsert(lookup_key, hash, key_func, value_func))
#[derive(Debug, PartialEq)]
struct LookupByKey(i32);

/// 同时支持 (void*, void*) 与 (LookupByKey, void*) 两种比较
#[derive(Clone, Copy)]
struct MixedMatcher;

impl MatchFn<*mut c_void, *mut c_void> for MixedMatcher {
  fn match_fn(&self, _h1: u32, _h2: u32, lookup: &*mut c_void, entry: &*mut c_void) -> bool {
    lookup == entry
  }
}

impl MatchFn<LookupByKey, *mut c_void> for MixedMatcher {
  fn match_fn(&self, _h1: u32, _h2: u32, lookup: &LookupByKey, entry: &*mut c_void) -> bool {
    // key 编码与 IntSet 的 reinterpret_cast 约定一致
    *entry as isize as i32 == lookup.0
  }
}

#[test]
fn hashmap_test_heterogeneous_lookup_or_insert() -> Void {
  let mut map: TemplateHashMapImpl<*mut c_void, u32, MixedMatcher, DefaultAllocationPolicy> =
    TemplateHashMapImpl::new(8, MixedMatcher, DefaultAllocationPolicy);

  let i = map
    .lookup_or_insert_heterogeneous(&LookupByKey(9), 4, || as_key(9), || 77)
    .expect("槽位");
  assert_eq!(as_key(9), map.entry(i).unwrap().key);
  assert_eq!(77, map.entry(i).unwrap().value);

  // 同 key 异质命中, 不重复插入
  let j = map
    .lookup_or_insert_heterogeneous(&LookupByKey(9), 4, || as_key(9), || 1)
    .expect("槽位");
  assert_eq!(i, j);
  assert_eq!(1, map.occupancy());

  // 同质路径亦命中同一槽
  assert_eq!(Some(i), map.lookup(&as_key(9), 4));
  assert_eq!(Some(i), map.probe(&LookupByKey(9), 4));
  OK
}

/// TemplateHashMap 类型化视图 (begin/next/find/entry_pair)
#[test]
fn hashmap_test_template_hash_map_view() -> Void {
  let mut m: TemplateHashMap<i32, i32, KeyEqualityMatcher<*mut c_void>, DefaultAllocationPolicy> =
    TemplateHashMap::new(HashMap::new_default());

  let mut key1 = 11i32;
  let mut key2 = 22i32;
  let p1 = ptr::from_mut(&mut key1);
  let p2 = ptr::from_mut(&mut key2);

  assert_eq!(None, m.begin());
  let i = m.find(p1, 5, true).expect("insert 得槽位");
  // find(insert=false) 命中同一槽
  assert_eq!(Some(i), m.find(p1, 5, false));
  // 未插入的 key 查询为 None (原版 end())
  assert_eq!(None, m.find(p2, 5, false));

  let (fk, fv) = m.entry_pair(i).expect("存在条目");
  assert_eq!(p1, fk);
  // 原版 value_type 视图: 空 value 即 nullptr, 不可解引用
  assert!(fv.is_null());

  assert_eq!(m.begin(), Some(i));
  assert_eq!(None, m.next(i));
  assert_eq!(1, m.inner().occupancy());
  assert_eq!(8, m.inner().capacity());
  let _ = m.inner_mut();
  OK
}

/// Start/Next 遍历顺序 = 槽位递增 (原版 map_ + 1 起扫)
#[test]
fn hashmap_test_iteration_slots() -> Void {
  let mut map = HashMap::new_default();
  assert_eq!(None, map.start());

  let mut slots = Vec::new();
  for i in 1..=6i32 {
    slots.push(
      map
        .lookup_or_insert(as_key(i), collision_hash(i))
        .expect("槽位"),
    );
  }
  slots.sort_unstable();
  let mut iterated = Vec::new();
  let mut p = map.start();
  while let Some(i) = p {
    iterated.push(i);
    p = map.next(i);
  }
  assert_eq!(slots, iterated);
  assert_eq!(map.occupancy() as usize, iterated.len());

  // 全部删除后回到空遍历
  for i in 1..=6i32 {
    map.remove(&as_key(i), collision_hash(i));
  }
  assert_eq!(None, map.start());
  assert_eq!(0, map.occupancy());
  OK
}
