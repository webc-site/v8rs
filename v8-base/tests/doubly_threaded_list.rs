//! v8/test/unittests/base/doubly-threaded-list-unittest.cc 的规约迁移
//!
//! 规约清单:
//! - BasicTest: PushFront 建链 → 头/next/prev 全链断言; Remove front/middle/
//!   back/only 四路径的指针联动与空表判定
//! - IteratorTest: 正向迭代计数与序; RemoveAt 逐个删空后全部节点
//!   prev_/next_ 归 nullptr

use std::ptr;

use aok::{OK, Void};
use v8_base::doubly_threaded_list::{
  DefaultDoublyThreadedListTraits, DoublyThreadedList, DoublyThreadedSlots,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 unittest 内嵌 struct Elem { val; Elem** prev_; Elem* next_; }
struct Elem {
  val: i32,
  /// prev_ 字段值 = 前驱 next_ 槽地址 (或表头槽地址)
  prev_: *mut *mut Elem,
  next_: *mut Elem,
}

impl Elem {
  fn new(val: i32) -> Self {
    Self {
      val,
      prev_: ptr::null_mut(),
      next_: ptr::null_mut(),
    }
  }
  /// 原版 operator==: val + prev_ + next_ 全等
  fn eq(&self, other: &Self) -> bool {
    self.val == other.val && self.prev_ == other.prev_ && self.next_ == other.next_
  }
}

impl DoublyThreadedSlots<*mut Elem> for Elem {
  fn prev_slot(&mut self) -> *mut *mut *mut Elem {
    &mut self.prev_
  }
  fn next_slot(&mut self) -> *mut *mut Elem {
    &mut self.next_
  }
}

type ElemList = DoublyThreadedList<*mut Elem, DefaultDoublyThreadedListTraits>;

fn addr(e: &Elem) -> *mut Elem {
  ptr::from_ref(e).cast_mut()
}

/// TEST_F(DoublyThreadedListTest, BasicTest)
#[test]
fn doubly_threaded_list_basic_test() -> Void {
  let mut list = ElemList::new();
  let e1 = Elem::new(1);
  let e2 = Elem::new(1);
  let e3 = Elem::new(1);
  let e4 = Elem::new(1);

  list.push_front(addr(&e1));
  // EXPECT_EQ(**(list.begin()), e1)
  assert!(unsafe { (*list.begin().get()).eq(&e1) });

  list.push_front(addr(&e2));
  list.push_front(addr(&e3));
  list.push_front(addr(&e4));
  assert!(unsafe { (*list.begin().get()).eq(&e4) });
  // SAFETY: 节点均在链上且存活 (以下所有解引用同此契约)
  unsafe {
    assert!((*e4.next_).eq(&e3));
    assert!((*e3.next_).eq(&e2));
    assert!((*e2.next_).eq(&e1));
    assert_eq!(e1.next_, ptr::null_mut());
    assert_eq!(*e1.prev_, addr(&e1));
    assert_eq!(*e2.prev_, addr(&e2));
    assert_eq!(*e3.prev_, addr(&e3));
    assert_eq!(*e4.prev_, addr(&e4));
  }

  // Removing front
  list.remove(addr(&e4));
  assert!(unsafe { (*list.begin().get()).eq(&e3) });
  unsafe {
    assert_eq!(*e3.prev_, addr(&e3));
    assert_eq!(e4.next_, ptr::null_mut());
    assert_eq!(e4.prev_, ptr::null_mut());
  }

  // Removing middle
  list.remove(addr(&e2));
  unsafe {
    assert!((*e3.next_).eq(&e1));
    assert_eq!(e2.prev_, ptr::null_mut());
    assert_eq!(e2.next_, ptr::null_mut());
    assert_eq!(e1.prev_, &raw mut (*addr(&e3)).next_);
    assert_eq!(*e1.prev_, addr(&e1));
  }

  // Removing back (纯字段比较, 无解引用)
  list.remove(addr(&e1));
  assert_eq!(e3.next_, ptr::null_mut());
  assert_eq!(e1.prev_, ptr::null_mut());
  assert_eq!(e1.next_, ptr::null_mut());
  assert!(unsafe { (*list.begin().get()).eq(&e3) });

  // Removing only item (纯字段比较)
  list.remove(addr(&e3));
  assert_eq!(e3.prev_, ptr::null_mut());
  assert_eq!(e3.next_, ptr::null_mut());
  assert!(list.is_empty());
  OK
}

/// TEST_F(DoublyThreadedListTest, IteratorTest)
#[test]
fn doubly_threaded_list_iterator_test() -> Void {
  let mut list = ElemList::new();
  let e1 = Elem::new(1);
  let e2 = Elem::new(1);
  let e3 = Elem::new(1);
  let e4 = Elem::new(1);

  list.push_front(addr(&e1));
  list.push_front(addr(&e2));
  list.push_front(addr(&e3));
  list.push_front(addr(&e4));

  // 计数 4 (range-for)
  let count = list.begin().count();
  assert_eq!(4, count);

  // 顺序 e4 → e3 → e2 → e1 → end
  let mut it = list.begin();
  assert!(unsafe { (*it.get()).eq(&e4) });
  it.advance();
  assert!(unsafe { (*it.get()).eq(&e3) });
  it.advance();
  assert!(unsafe { (*it.get()).eq(&e2) });
  it.advance();
  assert!(unsafe { (*it.get()).eq(&e1) });
  it.advance();
  assert!(!it.has_next());

  // RemoveAt 逐个删空
  let mut it = list.begin();
  assert!(unsafe { (*it.get()).eq(&e4) });
  it = list.remove_at(&it);
  assert!(unsafe { (*it.get()).eq(&e3) });
  it.advance();
  assert!(unsafe { (*it.get()).eq(&e2) });
  it = list.remove_at(&it);
  assert!(unsafe { (*it.get()).eq(&e1) });
  unsafe { assert!((*e3.next_).eq(&e1)) };
  it = list.remove_at(&it);
  assert!(!it.has_next());
  assert_eq!(e3.next_, ptr::null_mut());
  let it2 = list.begin();
  let _ = list.remove_at(&it2);

  assert!(list.is_empty());
  // 纯字段比较 (原版删空后 prev_/next_ 归 nullptr 断言)
  assert_eq!(e1.next_, ptr::null_mut());
  assert_eq!(e2.next_, ptr::null_mut());
  assert_eq!(e3.next_, ptr::null_mut());
  assert_eq!(e4.next_, ptr::null_mut());
  assert_eq!(e1.prev_, ptr::null_mut());
  assert_eq!(e2.prev_, ptr::null_mut());
  assert_eq!(e3.prev_, ptr::null_mut());
  assert_eq!(e4.prev_, ptr::null_mut());
  OK
}
