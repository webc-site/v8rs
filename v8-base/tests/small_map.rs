//! v8-base/src/small_map.rs 集成测试 — 对应 smallmap-unittest.cc
//! 用例名对齐原版 TEST(SmallMap, X): General / PostFixIteratorIncrement /
//! CopyConstructor / AssignmentOperator / Insert / InsertRange / Erase /
//! EraseReturnsIteratorFollowingRemovedElement / NonHashMap /
//! DefaultEqualKeyWorks / MoveOnlyValueType / Emplace。
//! 登记跳过: SubclassInit×2 (MapInit 构造钩子无 Rust 对应物, 全量段固定
//! BTreeMap, 见模块头差异登记)。
//! 原版 pair<iterator,bool> → Result<bool,(K,V)>; iterator 比较 → Option 引用。

use std::mem::take;

use aok::{OK, Void};
use v8_base::small_map::SmallMap;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

type Map = SmallMap<i32, &'static str, 4>;

#[test]
fn general() -> Void {
  // 原版 General: find 未命中返回 end; 迭代顺序读取
  let mut map = Map::new();
  assert!(map.is_empty());
  assert!(map.find(&1).is_none());
  assert_eq!(map.size(), 0);
  assert!(map.insert(1, "one").is_ok());
  assert_eq!(map.find(&1).map(|(k, _)| *k), Some(1));
  assert_eq!(map.insert(1, "one again"), Err((1, "one again")));
  assert_eq!(map.find(&1).map(|(_, v)| *v), Some("one"));
  assert!(map.insert(2, "two").is_ok());
  assert_eq!(
    map.iter().collect::<Vec<_>>(),
    vec![(&1, &"one"), (&2, &"two")]
  );
  assert_eq!(map.count(&2), 1);
  assert_eq!(map.count(&7), 0);
  // 原版 operator[]: 存在返回引用 / 缺失插默认值
  assert_eq!(*map.at_mut(&2), "two");
  assert_eq!(*map.at_mut(&3), "");
  assert_eq!(map.size(), 3);
  assert!(!map.using_full_map());
  OK
}

#[test]
fn postfix_iterator_increment() -> Void {
  // 原版 PostFixIteratorIncrement: it++ 回传旧位快照后再前进
  let mut map = Map::new();
  map.insert(1, "a").ok();
  map.insert(2, "b").ok();
  let mut it = map.iter();
  // it++ ≡ { result = it; ++it; result } — Clone 快照承接旧位
  let result = it.clone();
  let first_new = it.next();
  assert_eq!(first_new.map(|(k, _)| *k), Some(1));
  // 旧快照独立: 从头读取, 不受 it 自增影响 (原版后置 ++ 回传旧位)
  let mut result = result;
  assert_eq!(result.next().map(|(k, _)| *k), Some(1));
  assert_eq!(result.next().map(|(k, _)| *k), Some(2));
  assert_eq!(result.next(), None);
  OK
}

#[test]
fn copy_constructor() -> Void {
  let mut map = Map::new();
  map.insert(1, "a").ok();
  map.insert(2, "b").ok();
  let copy = map.clone();
  assert_eq!(copy.size(), 2);
  assert_eq!(copy.find(&1).map(|(_, v)| *v), Some("a"));
  // 深拷贝独立演化 (原版 InitFrom)
  let mut mut_copy = copy.clone();
  mut_copy.insert(3, "c").ok();
  assert_eq!(map.size(), 2);
  assert_eq!(mut_copy.size(), 3);
  OK
}

#[test]
fn assignment_operator() -> Void {
  let mut src = Map::new();
  src.insert(1, "a").ok();
  let mut dst = Map::new();
  dst.insert(2, "b").ok();
  dst = src.clone(); // 原版 operator= (Destroy + InitFrom)
  assert_eq!(dst.find(&1).map(|(_, v)| *v), Some("a"));
  assert!(dst.find(&2).is_none());
  // 自赋值保护 (原版 if (&src == this) return; Rust 赋值天然安全)
  let same = dst.clone();
  dst = same;
  assert_eq!(dst.size(), 1);
  // 全量段拷贝 (map_ = src.map_ 形态)
  let mut big = Map::new();
  for k in 0..6 {
    big.insert(k, "v").ok();
  }
  assert!(big.using_full_map());
  let big_copy = big.clone();
  assert!(big_copy.using_full_map());
  assert_eq!(big_copy.size(), 6);
  OK
}

#[test]
fn insert() -> Void {
  let mut map = Map::new();
  assert!(map.insert(1, "one").is_ok());
  assert!(map.insert(1, "uno").is_err());
  assert_eq!(map.find(&1).map(|(_, v)| *v), Some("one"));
  // 填满内联段 (kArraySize=4) 后仍为线性
  for (k, v) in [(2, "two"), (3, "three"), (4, "four")] {
    assert!(map.insert(k, v).is_ok());
  }
  assert!(!map.using_full_map());
  // 第 5 项触发 ConvertToRealMap
  assert!(map.insert(5, "five").is_ok());
  assert!(map.using_full_map());
  assert_eq!(map.size(), 5);
  for k in 1..=5 {
    assert_eq!(map.count(&k), 1);
  }
  assert_eq!(
    map.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
    vec![1, 2, 3, 4, 5]
  );
  OK
}

#[test]
fn insert_range() -> Void {
  // 原版 insert(first, last) 区间形态
  let mut map = Map::new();
  map.insert_range([(1, "one"), (2, "two"), (3, "three")]);
  assert_eq!(map.size(), 3);
  // 重复键跳过 (原版循环逐条 insert 语义)
  map.insert_range([(2, "dos"), (4, "four")]);
  assert_eq!(map.size(), 4);
  assert_eq!(map.find(&2).map(|(_, v)| *v), Some("two"));
  OK
}

#[test]
fn erase() -> Void {
  let mut map = Map::new();
  map.insert_range([(1, "a"), (2, "b"), (3, "c")]);
  assert_eq!(map.erase(&9), 0);
  assert_eq!(map.erase(&2), 1);
  assert!(map.find(&2).is_none());
  assert_eq!(map.size(), 2);
  // 全量段 erase
  map.insert_range([(4, "d"), (5, "e"), (6, "f"), (7, "g")]);
  assert!(map.using_full_map());
  assert_eq!(map.erase(&5), 1);
  assert_eq!(map.erase(&5), 0);
  assert_eq!(map.size(), 5);
  // clear 回退线性段 (原版 map_.~NormalMap(); size_ = 0)
  map.clear();
  assert!(map.is_empty());
  assert!(!map.using_full_map());
  OK
}

#[test]
fn erase_returns_iterator_following_removed_element() -> Void {
  // 原版: erase(position) swap-move 末元素入空位, 返回同位新元素迭代器
  let mut map = Map::new();
  map.insert_range([(1, "a"), (2, "b"), (3, "c")]);
  // 删下标 0 (键 1) → 末元素 (3,c) 移到下标 0, 后继 Some(0)
  assert_eq!(map.erase_at(0), Some(0));
  assert!(map.find(&3).is_some());
  assert_eq!(map.iter().map(|(k, _)| *k).collect::<Vec<_>>(), vec![3, 2]);
  // 删末位 → 返回 end (None)
  assert_eq!(map.erase_at(1), None);
  assert_eq!(map.iter().map(|(k, _)| *k).collect::<Vec<_>>(), vec![3]);
  OK
}

#[test]
fn non_hash_map() -> Void {
  // 原版 NonHashMap: 退化全量映射后的查找/删除/迭代序 (BTreeMap 键序)
  let mut map = Map::new();
  for k in (0..10).rev() {
    map.insert(k, "v").ok();
  }
  assert!(map.using_full_map());
  assert_eq!(
    map.iter().map(|(k, _)| *k).collect::<Vec<_>>(),
    (0..10).collect::<Vec<_>>()
  );
  map.erase(&4);
  assert_eq!(map.size(), 9);
  assert!(map.find(&4).is_none());
  assert_eq!(map.get_mut(&9).copied(), Some("v"));
  OK
}

#[test]
fn default_equal_key_works() -> Void {
  // 原版 DefaultEqualKeyWorks: 默认 key_equal 即判等 (Rust Ord/PartialEq 一致)
  let mut map = SmallMap::<String, i32, 2>::new();
  assert!(map.insert("aa".to_string(), 1).is_ok());
  assert!(map.insert("aa".to_string(), 2).is_err());
  *map.at_mut(&"aa".to_string()) += 10;
  assert_eq!(map.find(&"aa".to_string()).map(|(_, v)| *v), Some(11));
  // 缺失键 operator[] 得默认值 0 (原版 data_type() 值初始化)
  assert_eq!(*map.at_mut(&"bb".to_string()), 0);
  assert_eq!(map.size(), 2);
  // 满后退化仍可判等取回
  assert!(map.insert("cc".to_string(), 3).is_ok());
  assert!(map.using_full_map());
  assert_eq!(map.find(&"cc".to_string()).map(|(_, v)| *v), Some(3));
  OK
}

#[test]
fn move_only_value_type() -> Void {
  // 原版 MoveOnlyValueType: 仅移动类型入表; Rust 侧泛型要求 V: Clone
  // (深拷贝构造对应物) → 以 take 移交整表 + 值移动断言承接
  let mut map = SmallMap::<i32, u32, 2>::new();
  map.insert(1, 100).ok();
  let mut slot = take(&mut map); // 原版 move 构造: 源表复位
  assert!(map.is_empty());
  slot.insert(2, 200).ok();
  assert_eq!(slot.get_mut(&1).copied(), Some(100));
  assert_eq!(slot.erase(&1), 1);
  assert_eq!(slot.size(), 1);
  OK
}

#[test]
fn emplace() -> Void {
  // 原版 Emplace: 就地构造; Rust 参数即值 → emplace == insert 形态
  let mut map = Map::new();
  assert!(map.emplace(1, "one").is_ok());
  assert!(map.emplace(1, "ein").is_err());
  // try_emplace: 既有项原值保留
  assert_eq!(*map.try_emplace(1, "uno"), "one");
  assert_eq!(*map.try_emplace(2, "two"), "two");
  assert_eq!(map.size(), 2);
  // try_emplace 触发退化后仍正确回传引用
  for k in 3..7 {
    let r = map.try_emplace(k, "x");
    *r = "y";
  }
  assert!(map.using_full_map());
  assert_eq!(map.find(&6).map(|(_, v)| *v), Some("y"));
  OK
}
