//! 原版 v8/test/unittests/zone/zone-vector-unittest.cc 转写
//!
//! 类型映射 (readme/zh.md): C++ 按 trivially-copyable / copy-assignable /
//! move-assignable / not-assignable 分支; Rust 无赋值重载与平凡性概念,
//! 三类可追踪类型统一为 Clone+Drop + 序列号 LiveSet (每实例构造/克隆
//! 登记唯一 serial, Drop 注销), 覆盖同等不变量: 任何时刻存活对象集合
//! == 向量元素集合 (无重复、无泄漏、无误析构)。
//! MoveAssignable 与 NotAssignable 在 Rust 下与 CopyAssignable 同形
//! (移动由借用检查器保证, 赋值即 drop+clone), 保留三个原名验证分支。
//! 注: 传给容器的临时量 (C++ T(1) 等) 一律以内层作用域构造并先行析构,
//! 与原版临时量语义对齐, 保证 CheckContainsAll 的 size 断言成立。

use std::{
  cell::RefCell,
  collections::BTreeSet,
  mem,
  sync::atomic::{AtomicU64, Ordering},
};

use aok::{OK, Void};
use v8_zone::{AccountingAllocator, Zone, ZoneVector};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

static NEXT_SERIAL: AtomicU64 = AtomicU64::new(1);

trait Tracked: Clone {
  /// 原版 T(int id) 构造
  fn new(id: i32) -> Self;
  fn serial(&self) -> u64;
  fn id(&self) -> i32;
  fn lives() -> BTreeSet<u64>;
  /// 原版 LiveSet::CheckEmpty
  fn check_empty();
}

/// 原版 class Trivial (trivially-copyable, LiveSet 特化为空操作)
#[derive(Copy, Clone)]
struct Trivial {
  ident: i32,
}

impl Trivial {
  fn new(id: i32) -> Trivial {
    Trivial { ident: id }
  }
}

impl Tracked for Trivial {
  fn new(id: i32) -> Trivial {
    Trivial::new(id)
  }
  fn serial(&self) -> u64 {
    0
  }
  fn id(&self) -> i32 {
    self.ident
  }
  fn lives() -> BTreeSet<u64> {
    BTreeSet::new()
  }
  fn check_empty() {}
}

/// 原版 LiveSet<Trivial> 特化: 不参与登记
const TRACKED_TRIVIAL: bool = false;

/// 生成一个 Clone+Drop 登记类型 (对应 CopyAssignable 等三类)
macro_rules! tracked_type {
  ($name:ident, $lives:ident) => {
    thread_local! {
      static $lives: RefCell<BTreeSet<u64>> = const { RefCell::new(BTreeSet::new()) };
    }

    struct $name {
      serial: u64,
      ident: i32,
    }

    impl $name {
      fn new(id: i32) -> $name {
        let serial = NEXT_SERIAL.fetch_add(1, Ordering::Relaxed);
        $lives.with(|l| assert!(l.borrow_mut().insert(serial), "duplicate live entry"));
        $name { serial, ident: id }
      }
    }

    impl Clone for $name {
      /// 原版拷贝构造: 新对象登记 (Add)
      fn clone(&self) -> $name {
        let serial = NEXT_SERIAL.fetch_add(1, Ordering::Relaxed);
        $lives.with(|l| assert!(l.borrow_mut().insert(serial), "duplicate live entry"));
        $name {
          serial,
          ident: self.ident,
        }
      }
    }

    impl Drop for $name {
      /// 原版析构: 注销 (Remove)
      fn drop(&mut self) {
        $lives.with(|l| assert!(l.borrow_mut().remove(&self.serial), "remove non-live entry"));
      }
    }

    impl Tracked for $name {
      fn new(id: i32) -> $name {
        $name::new(id)
      }
      fn serial(&self) -> u64 {
        self.serial
      }
      fn id(&self) -> i32 {
        self.ident
      }
      fn lives() -> BTreeSet<u64> {
        $lives.with(|l| l.borrow().clone())
      }
      fn check_empty() {
        $lives.with(|l| assert!(l.borrow().is_empty(), "live set not empty"));
      }
    }
  };
}

tracked_type!(CopyAssignable, CA_LIVES);
tracked_type!(MoveAssignable, MA_LIVES);
tracked_type!(NotAssignable, NA_LIVES);

/// 原版 CheckConsistency: id 序列一致 + LiveSet 与向量元素一一对应
fn check_consistency<T: Tracked>(v: &ZoneVector<'_, '_, T>, ids: &[i32], tracked: bool) {
  assert_eq!(v.size(), ids.len(), "vector size");
  for (i, expected) in ids.iter().enumerate() {
    assert_eq!(&v.at(i).id(), expected, "element {i} id");
  }
  if tracked {
    // 原版 CheckContainsAll: 全局存活数 == 向量 size 且逐元素在册
    let lives = T::lives();
    assert_eq!(lives.len(), v.size(), "live set size");
    for i in 0..v.size() {
      assert!(lives.contains(&v.at(i).serial()), "element not in live set");
    }
  }
}

/// 原版 TEST_F(ZoneVectorTest, Basic) 泛型体
fn basic<T: Tracked>(zone: &Zone<'_>, tracked: bool) -> Void {
  {
    // 原版 ZoneVector<T> v(1, T(1), zone): T(1) 为临时量
    let def = T::new(1);
    let v: ZoneVector<'_, '_, T> = {
      let result = ZoneVector::with_value(zone, 1, &def)?;
      drop(def);
      result
    };
    check_consistency(&v, &[1], tracked);
  }
  T::check_empty();

  {
    let v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    check_consistency(&v, &[1, 2, 3], tracked);
  }
  T::check_empty();

  {
    // 原版 std::optional<ZoneVector<T>> v1 + emplace/reset 形态:
    // 以先后两段绑定模拟, 语义一致 (reset == drop)
    let v1a: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3)];
      v1a = ZoneVector::from_slice(zone, &src)?;
    }
    check_consistency(&v1a, &[1, 2, 3], tracked);
    {
      // 原版拷贝构造增长路径: ZoneVector<T> v2 = v1.value();
      let v2 = v1a.clone();
      drop(v1a);
      check_consistency(&v2, &[1, 2, 3], tracked);
    }
    let v1b: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3)];
      v1b = ZoneVector::from_slice(zone, &src)?;
    }
    check_consistency(&v1b, &[1, 2, 3], tracked);

    // 原版拷贝赋值无增长: v3({4,5,6}) = v1.value()
    let mut v3: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(4), T::new(5), T::new(6)];
      v3 = ZoneVector::from_slice(zone, &src)?;
    }
    v3.assign_from(&v1b)?;
    drop(v1b);
    check_consistency(&v3, &[1, 2, 3], tracked);

    // 原版移动构造 v4(std::move(v3)) + 空 v3 断言: mem::replace 保留两者可读
    {
      let v4 = mem::replace(&mut v3, ZoneVector::new(zone));
      check_consistency(&v4, &[1, 2, 3], tracked);
    }
    check_consistency(&v3, &[], tracked);
  }
  T::check_empty();
  OK
}

/// 原版 TEST_F(ZoneVectorTest, Assign) 泛型体
fn assign_case<T: Tracked>(zone: &Zone<'_>, tracked: bool) -> Void {
  {
    // 容量足够的 assign
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    {
      let four = T::new(4);
      v.assign(2, &four)?;
    }
    check_consistency(&v, &[4, 4], tracked);
    // 此时 capacity > size
    {
      let five = T::new(5);
      v.assign(3, &five)?;
    }
    check_consistency(&v, &[5, 5, 5], tracked);
  }

  {
    // 触发扩容的 assign
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    {
      let four = T::new(4);
      v.assign(2, &four)?;
    }
    check_consistency(&v, &[4, 4], tracked);
  }

  T::check_empty();
  OK
}

/// 原版 TEST_F(ZoneVectorTest, Insert) 泛型体
fn insert_case<T: Tracked>(zone: &Zone<'_>, tracked: bool) -> Void {
  // 迭代器 (范围) 形态插入到正确位置
  {
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(2), T::new(4)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    {
      let src1 = [T::new(1)];
      let src3 = [T::new(3)];
      let src5 = [T::new(5)];
      v.insert_slice(0, &src1)?;
      v.insert_slice(2, &src3)?;
      let tail = v.size();
      v.insert_slice(tail, &src5)?;
    }
    check_consistency(&v, &[1, 2, 3, 4, 5], tracked);
  }

  // count 形态插入
  {
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(2), T::new(4)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    {
      let one = T::new(1);
      let three = T::new(3);
      let five = T::new(5);
      v.insert_repeat(0, 1, &one)?;
      v.insert_repeat(2, 1, &three)?;
      let tail = v.size();
      v.insert_repeat(tail, 1, &five)?;
    }
    check_consistency(&v, &[1, 2, 3, 4, 5], tracked);
  }

  // PrepareForInsertion 容量不足分支
  {
    let mut v: ZoneVector<'_, '_, T> = ZoneVector::new(zone);
    assert_eq!(v.capacity(), 0);
    {
      let five = T::new(5);
      v.insert_repeat(0, 1, &five)?;
    }
    check_consistency(&v, &[5], tracked);
    {
      let src = [T::new(1), T::new(2), T::new(3), T::new(4)];
      v.insert_slice(0, &src)?;
    }
    check_consistency(&v, &[1, 2, 3, 4, 5], tracked);
  }

  // 容量充足分支 case 1
  {
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3), T::new(4), T::new(5)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    v.reserve(10)?;
    assert_eq!(v.capacity(), 10);
    check_consistency(&v, &[1, 2, 3, 4, 5], tracked);
    {
      let src4 = [T::new(11), T::new(12), T::new(13), T::new(14)];
      v.insert_slice(3, &src4)?;
    }
    check_consistency(&v, &[1, 2, 3, 11, 12, 13, 14, 4, 5], tracked);
  }

  // 容量充足分支 case 2
  {
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3), T::new(4), T::new(5)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    v.reserve(10)?;
    {
      let src2 = [T::new(11), T::new(12)];
      v.insert_slice(2, &src2)?;
    }
    check_consistency(&v, &[1, 2, 11, 12, 3, 4, 5], tracked);
  }
  T::check_empty();

  // 边界: 插入数量恰等于移动数量 (原版无 CheckConsistency, 此处补验)
  {
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3), T::new(4)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    v.reserve(10)?;
    {
      let src2 = [T::new(11), T::new(12)];
      v.insert_slice(2, &src2)?;
    }
    check_consistency(&v, &[1, 2, 11, 12, 3, 4], tracked);
  }
  OK
}

/// 原版 TEST_F(ZoneVectorTest, Erase) 泛型体
fn erase_case<T: Tracked>(zone: &Zone<'_>, tracked: bool) -> Void {
  // 删除单个元素 (原版 erase 析构被删值; Rust 取回后显式 drop)
  {
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    let removed = v.erase(1);
    assert!(removed.is_some());
    drop(removed);
    check_consistency(&v, &[1, 3], tracked);
  }
  // 删除区间
  {
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3), T::new(4)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    let removed = v.erase_range(1..3);
    assert!(removed.is_some());
    drop(removed);
    check_consistency(&v, &[1, 4], tracked);
  }
  // 删除首元素
  {
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    drop(v.erase(0));
    check_consistency(&v, &[2, 3], tracked);
  }
  // 删除末元素
  {
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    drop(v.erase(2));
    check_consistency(&v, &[1, 2], tracked);
  }
  // 空区间删除 (无变化)
  {
    let mut v: ZoneVector<'_, '_, T>;
    {
      let src = [T::new(1), T::new(2), T::new(3)];
      v = ZoneVector::from_slice(zone, &src)?;
    }
    assert!(v.erase_range(0..0).is_none());
    check_consistency(&v, &[1, 2, 3], tracked);
    assert!(v.erase_range(1..1).is_none());
    check_consistency(&v, &[1, 2, 3], tracked);
    let n = v.size();
    assert!(v.erase_range(n..n).is_none());
    check_consistency(&v, &[1, 2, 3], tracked);
  }
  T::check_empty();
  OK
}

/// 原版 TEST_F(ZoneVectorTest, Basic)
#[test]
fn zone_vector_test_basic() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  basic::<Trivial>(&zone, TRACKED_TRIVIAL)?;
  basic::<CopyAssignable>(&zone, true)?;
  basic::<MoveAssignable>(&zone, true)?;
  basic::<NotAssignable>(&zone, true)?;
  OK
}

/// 原版 TEST_F(ZoneVectorTest, Assign)
#[test]
fn zone_vector_test_assign() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  assign_case::<Trivial>(&zone, TRACKED_TRIVIAL)?;
  assign_case::<CopyAssignable>(&zone, true)?;
  assign_case::<MoveAssignable>(&zone, true)?;
  assign_case::<NotAssignable>(&zone, true)?;
  OK
}

/// 原版 TEST_F(ZoneVectorTest, Insert)
#[test]
fn zone_vector_test_insert() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  insert_case::<Trivial>(&zone, TRACKED_TRIVIAL)?;
  insert_case::<CopyAssignable>(&zone, true)?;
  insert_case::<MoveAssignable>(&zone, true)?;
  insert_case::<NotAssignable>(&zone, true)?;
  OK
}

/// 原版 TEST_F(ZoneVectorTest, Erase)
#[test]
fn zone_vector_test_erase() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  erase_case::<Trivial>(&zone, TRACKED_TRIVIAL)?;
  erase_case::<CopyAssignable>(&zone, true)?;
  erase_case::<MoveAssignable>(&zone, true)?;
  erase_case::<NotAssignable>(&zone, true)?;
  OK
}
