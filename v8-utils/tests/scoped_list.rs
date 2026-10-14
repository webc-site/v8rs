// 上游无 scoped-list 单测; 本文件为嵌套作用域 + MergeInto 回卷语义的补测
// (对应 scoped-list.h 类文档契约: 内层存活时外层禁 Add; MergeInto 交还外层)。
use aok::{OK, Void};
use v8_utils::scoped_list::ScopedList;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// 基本作用域: Add/Rewind 截回 start, backing 可复用
#[test]
fn add_and_rewind() -> Void {
  let mut buffer: Vec<i32> = vec![7];
  {
    let mut list = ScopedList::new(&mut buffer);
    assert_eq!(list.length(), 0);
    list.add(1);
    list.add(2);
    assert_eq!(list.length(), 2);
    assert_eq!(*list.at(0), 1);
    assert_eq!(*list.at(1), 2);
    assert_eq!(list.as_slice(), &[1, 2]);
    assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
  }
  assert_eq!(buffer, vec![7]);
  // backing 复用: 第二个作用域从同一尾区开始
  {
    let mut list = ScopedList::new(&mut buffer);
    list.add_all(&[5, 6]);
    assert_eq!(list.as_slice(), &[5, 6]);
    *list.at_mut(1) = 9;
    assert_eq!(list.as_slice(), &[5, 9]);
  }
  assert_eq!(buffer, vec![7]);
  OK
}

// 嵌套作用域: 内层析构回卷 (keep=false), 外层条目不受影响
#[test]
fn nested_scope_rewinds_on_drop() -> Void {
  let mut buffer: Vec<i32> = Vec::new();
  {
    let mut outer = ScopedList::new(&mut buffer);
    outer.add(1);
    outer.scope(false, |inner| {
      inner.add(10);
      inner.add(11);
      assert_eq!(inner.length(), 2);
      assert_eq!(inner.as_slice(), &[10, 11]);
    }); // 内层回卷
    assert_eq!(outer.length(), 1);
    assert_eq!(outer.as_slice(), &[1]);
    outer.add(2); // 内层结束后外层恢复可加
    assert_eq!(outer.as_slice(), &[1, 2]);
  }
  assert!(buffer.is_empty());
  OK
}

// MergeInto (keep=true): 内层区间并入外层, 外层随后照常扩展
#[test]
fn merge_into_parent_keeps_entries() -> Void {
  let mut buffer: Vec<i32> = Vec::new();
  {
    let mut outer = ScopedList::new(&mut buffer);
    outer.add(1);
    let n = outer.scope(true, |inner| {
      inner.add(10);
      inner.add(11);
      inner.length()
    });
    assert_eq!(n, 2);
    assert_eq!(outer.length(), 3);
    assert_eq!(outer.as_slice(), &[1, 10, 11]);
    outer.add(12);
    assert_eq!(outer.as_slice(), &[1, 10, 11, 12]);
  }
  assert!(buffer.is_empty());
  OK
}

// 多级嵌套: 内内层回卷后, 已并入 (keep=true) 的内层条目仍在外层
#[test]
fn deep_nesting_with_merge_and_rewind() -> Void {
  let mut buffer: Vec<i32> = Vec::new();
  {
    let mut l1 = ScopedList::new(&mut buffer);
    l1.add(1);
    l1.scope(true, |l2| {
      l2.add(2);
      l2.scope(false, |l3| {
        l3.add(3);
      }); // l3 回卷
      assert_eq!(l2.as_slice(), &[2]);
    }); // [2] 并入 l1
    assert_eq!(l1.as_slice(), &[1, 2]);
  }
  assert!(buffer.is_empty());
  OK
}
