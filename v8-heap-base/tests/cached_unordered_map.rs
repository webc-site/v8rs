//! cached-unordered-map API 语义测试 (原版无 gtest 用例, 按 LRU 语义补)。
use std::thread::spawn;

use aok::{OK, Void};
use v8_heap_base::CachedUnorderedMap;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// operator[]: 缺省插入 + 同键 LRU 快路径返回同一槽
#[test]
fn default_insert_and_cache_hit() -> Void {
  let mut m: CachedUnorderedMap<String, usize> = CachedUnorderedMap::new();
  assert!(m.is_empty());
  // 缺省值 0 (原 emplace(key, Mapped()))
  assert_eq!(*m.get_or_insert(&"a".into()), 0);
  *m.get_or_insert(&"a".into()) = 5;
  // LRU 命中路径仍指向同一槽
  assert_eq!(*m.get_or_insert(&"a".into()), 5);
  *m.get_or_insert(&"a".into()) += 1;
  assert_eq!(*m.get_or_insert(&"a".into()), 6);
  assert_eq!(m.find(&"a".into()), Some(&6));
  assert!(m.contains(&"a".into()));
  OK
}

// 大量插入触发扩容重排后缓存仍健全 (语义收严点: 插入即老化)
#[test]
fn rehash_keeps_cache_sound() -> Void {
  let mut m: CachedUnorderedMap<u32, Vec<u32>> = CachedUnorderedMap::new();
  for i in 0..200u32 {
    m.get_or_insert(&i).push(i);
  }
  for i in 0..200u32 {
    assert_eq!(m.get_or_insert(&i), &vec![i]);
  }
  assert_eq!(m.iter().count(), 200);
  OK
}

// erase: 命中缓存键时失效; 再取为缺省值
#[test]
fn erase_invalidates() -> Void {
  let mut m: CachedUnorderedMap<u32, u32> = CachedUnorderedMap::new();
  *m.get_or_insert(&1) = 10;
  assert!(m.erase(&1));
  assert!(!m.erase(&1));
  assert_eq!(m.find(&1), None);
  assert_eq!(*m.get_or_insert(&1), 0);
  OK
}

// clear / take: 缓存与表一并清空/转移
#[test]
fn clear_and_take() -> Void {
  let mut m: CachedUnorderedMap<u32, u32> = CachedUnorderedMap::new();
  *m.get_or_insert(&1) = 10;
  *m.get_or_insert(&2) = 20;
  m.clear();
  assert!(m.is_empty());
  assert_eq!(*m.get_or_insert(&1), 0); // 缓存已失效, 不复活旧值

  *m.get_or_insert(&3) = 30;
  let taken = m.take();
  assert_eq!(taken.get(&3), Some(&30));
  assert!(m.is_empty());
  assert_eq!(*m.get_or_insert(&3), 0);
  OK
}

// erase 非缓存键也必须整体失效缓存 (README 登记语义):
// 原版仅擦缓存键时失效, 依赖 absl "erase 不迁移其余条目" 的文档保证;
// std HashMap 无此保证 (hashbrown ≤0.15 的 remove 即 swap_remove 会搬移),
// 故任何 erase 后缓存一律作废。本测试在删除搅动下逐键交叉验证
// 缓存命中路径与 find 落在同一槽 — 若未来表实现删除时迁移条目,
// 该不变量破坏即在此暴露 (依赖漂移 tripwire)。
#[test]
fn erase_any_key_invalidates_cache() -> Void {
  const N: u32 = 128;
  for anchor in 0..N {
    let mut m: CachedUnorderedMap<u32, u32> = CachedUnorderedMap::new();
    for k in 0..N {
      *m.get_or_insert(&k) = k;
    }
    // 缓存锚定 anchor 后删除其余键 (含触发表内删除搅动)
    assert_eq!(*m.get_or_insert(&anchor), anchor);
    for k in 0..N {
      if k != anchor {
        assert!(m.erase(&k));
      }
    }
    // 缓存命中路径写必须与 find 一致 (写错槽即不一致)
    *m.get_or_insert(&anchor) = 0xDEAD;
    assert_eq!(m.find(&anchor), Some(&0xDEAD));
    assert_eq!(m.iter().count(), 1);
  }
  OK
}

// 跨线程 Send 验证: 移动至其他线程继续读写
#[test]
fn send_across_threads() -> Void {
  let mut m: CachedUnorderedMap<u32, u32> = CachedUnorderedMap::new();
  *m.get_or_insert(&42) = 100;
  let handle = spawn(move || {
    assert_eq!(*m.get_or_insert(&42), 100);
    *m.get_or_insert(&42) = 200;
    m
  });
  let mut m = handle.join().unwrap();
  assert_eq!(*m.get_or_insert(&42), 200);
  OK
}
