//! v8/test/unittests/base/threaded-list-unittest.cc 的规约迁移
//!
//! 规约清单 (fixture: 5 节点 other_next 链 + 5 节点 next 链 + 3 节点 extra 双链):
//! - Add / AddFront: 既有表与空表两路径 + Verify + LengthForTest + AtForTest
//! - DropHead: 头摘除后 first/长度
//! - Append: 拼接后端迭代器不变 (end 槽地址守恒)
//! - AppendOutOfScope: 空 scoped 表 append 后仍可继续 Add
//! - Prepend: 头部拼接顺序
//! - Clear: 长度 0 + first null
//! - MoveAssign / MoveCtor: 非空与空源两路径 (steal/move_from)
//! - Remove: 头/中/尾/清空/未找到五路径
//! - Rewind: 中点位与头位点回卷
//! - IterComp / ConstIterComp: 迭代器相等触发 (Rust 无 const 迭代器之分, 两用例合并)
//! - RemoveAt: 头/中/尾三路径 + Contains 判假 + 摘除节点 next 槽清空
//!
//! 全程 TearDown 断言: next 链 (normal list) 不被 other_next 操作触碰

use std::ptr;

use aok::{OK, Void};
use v8_base::threaded_list::{
  DefaultThreadedListTraits, ThreadedList, ThreadedListTraits, ThreadedNextSlot,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 ThreadedListTestNode: next_ + other_next_ 双链字段
struct Node {
  next_: *mut Node,
  other_next_: *mut Node,
}

impl Node {
  const fn new() -> Self {
    Self {
      next_: ptr::null_mut(),
      other_next_: ptr::null_mut(),
    }
  }
}

impl ThreadedNextSlot<Node> for Node {
  fn next_slot(&mut self) -> *mut *mut Node {
    &mut self.next_
  }
}

/// 原版 ThreadedListTestNode::OtherTraits: 经 other_next_ 字段成链
struct OtherTraits;

impl ThreadedListTraits<Node> for OtherTraits {
  unsafe fn next(t: *mut Node) -> *mut *mut Node {
    // SAFETY: 契约要求 t 为存活节点
    unsafe { ptr::from_mut(&mut (*t).other_next_) }
  }
}

type OtherList = ThreadedList<Node, OtherTraits>;
type NormalList = ThreadedList<Node, DefaultThreadedListTraits>;

const INIT_NODES: usize = 5;

/// fixture SetUp/TearDown: 返回 (主链, next 对照链, extra 链, next extra 对照链)
/// 与全部节点; TearDown 断言以闭包在用例尾部显式调用
struct Fixture {
  nodes: [Node; INIT_NODES],
  list: OtherList,
  normal_next_list: NormalList,
  extra_test_list: OtherList,
  normal_extra_test_list: NormalList,
  extra_nodes: [Node; 3],
}

/// 以 Box 固定节点地址 (返回值 move 会使已登记的裸指针失效)
fn fixture() -> Box<Fixture> {
  let mut f = Box::new(Fixture {
    nodes: [const { Node::new() }; INIT_NODES],
    list: OtherList::new(),
    normal_next_list: NormalList::new(),
    extra_test_list: OtherList::new(),
    normal_extra_test_list: NormalList::new(),
    extra_nodes: [const { Node::new() }; 3],
  });
  for i in 0..INIT_NODES {
    // SAFETY: 节点为 fixture 字段, 存活覆盖用例; next 槽未接线
    unsafe {
      f.list.add(ptr::from_ref(&f.nodes[i]).cast_mut());
      f.normal_next_list
        .add(ptr::from_ref(&f.nodes[i]).cast_mut());
    }
  }
  assert!(f.list.verify());
  assert_eq!(INIT_NODES, f.list.length_for_test());
  assert!(f.normal_next_list.verify());
  assert_eq!(INIT_NODES, f.normal_next_list.length_for_test());

  for extra in &f.extra_nodes {
    let p = ptr::from_ref(extra).cast_mut();
    // SAFETY: 同上
    unsafe {
      f.extra_test_list.add(p);
      f.normal_extra_test_list.add(p);
    }
  }
  assert_eq!(3, f.extra_test_list.length_for_test());
  assert!(f.extra_test_list.verify());
  assert_eq!(3, f.normal_extra_test_list.length_for_test());
  assert!(f.normal_extra_test_list.verify());
  f
}

/// TearDown: next 链未被触碰
fn teardown(f: &mut Fixture) {
  assert!(f.normal_next_list.verify());
  assert_eq!(INIT_NODES, f.normal_next_list.length_for_test());
  assert_eq!(
    f.normal_next_list.at_for_test(0),
    f.nodes.as_ptr().cast_mut()
  );
  assert_eq!(f.normal_next_list.at_for_test(4), unsafe {
    f.nodes.as_mut_ptr().add(4)
  });
  assert!(f.normal_extra_test_list.verify());
  assert_eq!(3, f.normal_extra_test_list.length_for_test());
  assert_eq!(
    f.normal_extra_test_list.at_for_test(0),
    f.extra_nodes.as_ptr().cast_mut()
  );
  assert_eq!(f.normal_extra_test_list.at_for_test(2), unsafe {
    f.extra_nodes.as_mut_ptr().add(2)
  });
  f.list.clear();
  f.extra_test_list.clear();
}

/// 以字段切片取地址, 避免整结构借用与 &mut 链借用冲突
fn extra_addr(nodes: &[Node; 3], i: usize) -> *mut Node {
  unsafe { nodes.as_ptr().add(i) as *mut Node }
}

fn node_addr(nodes: &[Node; INIT_NODES], i: usize) -> *mut Node {
  unsafe { nodes.as_ptr().add(i) as *mut Node }
}

/// TEST_F(ThreadedListTest, Add)
#[test]
fn threaded_list_add() -> Void {
  let mut f = fixture();
  assert_eq!(5, f.list.length_for_test());
  let mut new_node = Node::new();
  // Add to existing list
  // SAFETY: new_node 栈上存活, next 槽未接线
  unsafe { f.list.add(ptr::from_mut(&mut new_node)) };
  assert!(f.list.verify());
  assert_eq!(6, f.list.length_for_test());
  assert_eq!(f.list.at_for_test(5), ptr::from_mut(&mut new_node));

  f.list.clear();
  assert_eq!(0, f.list.length_for_test());

  let mut new_node = Node::new();
  // Add to empty list
  // SAFETY: 同上
  unsafe { f.list.add(ptr::from_mut(&mut new_node)) };
  assert!(f.list.verify());
  assert_eq!(1, f.list.length_for_test());
  assert_eq!(f.list.at_for_test(0), ptr::from_mut(&mut new_node));
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, AddFront)
#[test]
fn threaded_list_add_front() -> Void {
  let mut f = fixture();
  assert_eq!(5, f.list.length_for_test());
  let mut new_node = Node::new();
  // SAFETY: new_node 存活, next 槽未接线
  unsafe { f.list.add_front(ptr::from_mut(&mut new_node)) };
  assert!(f.list.verify());
  assert_eq!(6, f.list.length_for_test());
  assert_eq!(f.list.first(), ptr::from_mut(&mut new_node));

  f.list.clear();
  assert_eq!(0, f.list.length_for_test());

  let mut new_node = Node::new();
  // SAFETY: 同上
  unsafe { f.list.add_front(ptr::from_mut(&mut new_node)) };
  assert!(f.list.verify());
  assert_eq!(1, f.list.length_for_test());
  assert_eq!(f.list.first(), ptr::from_mut(&mut new_node));
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, DropHead)
#[test]
fn threaded_list_drop_head() -> Void {
  let mut f = fixture();
  assert_eq!(3, f.extra_test_list.length_for_test());
  assert_eq!(f.extra_test_list.first(), extra_addr(&f.extra_nodes, 0));
  f.extra_test_list.drop_head();
  assert!(f.extra_test_list.verify());
  assert_eq!(f.extra_test_list.first(), extra_addr(&f.extra_nodes, 1));
  assert_eq!(2, f.extra_test_list.length_for_test());
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, Append)
#[test]
fn threaded_list_append() -> Void {
  let mut f = fixture();
  let initial_extra_list_end = f.extra_test_list.end();
  assert_eq!(5, f.list.length_for_test());
  f.list.append(&mut f.extra_test_list);
  assert!(f.list.verify());
  assert!(f.extra_test_list.verify());
  assert!(f.extra_test_list.is_empty());
  assert_eq!(8, f.list.length_for_test());
  assert_eq!(f.list.at_for_test(4), node_addr(&f.nodes, 4));
  assert_eq!(f.list.at_for_test(5), extra_addr(&f.extra_nodes, 0));
  // 拼接后 list 的 end 槽 == 原 extra 的 end 槽
  assert!(f.list.end().same(&initial_extra_list_end));
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, AppendOutOfScope)
#[test]
fn threaded_list_append_out_of_scope() -> Void {
  let mut f = fixture();
  let mut local_extra_test_node_0 = Node::new();
  assert_eq!(5, f.list.length_for_test());
  {
    let mut scoped_extra_test_list = OtherList::new();
    f.list.append(&mut scoped_extra_test_list);
  }
  // SAFETY: 局部节点存活, next 槽未接线
  unsafe { f.list.add(ptr::from_mut(&mut local_extra_test_node_0)) };

  assert!(f.list.verify());
  assert_eq!(6, f.list.length_for_test());
  assert_eq!(f.list.at_for_test(4), node_addr(&f.nodes, 4));
  assert_eq!(
    f.list.at_for_test(5),
    ptr::from_mut(&mut local_extra_test_node_0)
  );
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, Prepend)
#[test]
fn threaded_list_prepend() -> Void {
  let mut f = fixture();
  assert_eq!(5, f.list.length_for_test());
  f.list.prepend(&mut f.extra_test_list);
  assert!(f.list.verify());
  assert!(f.extra_test_list.verify());
  assert!(f.extra_test_list.is_empty());
  assert_eq!(8, f.list.length_for_test());
  assert_eq!(f.list.first(), extra_addr(&f.extra_nodes, 0));
  assert_eq!(f.list.at_for_test(2), extra_addr(&f.extra_nodes, 2));
  assert_eq!(f.list.at_for_test(3), node_addr(&f.nodes, 0));
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, Clear)
#[test]
fn threaded_list_clear() -> Void {
  let mut f = fixture();
  assert_ne!(0, f.list.length_for_test());
  f.list.clear();
  assert_eq!(0, f.list.length_for_test());
  assert!(f.list.first().is_null());
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, MoveAssign)
#[test]
fn threaded_list_move_assign() -> Void {
  let mut f = fixture();
  let mut m_list = OtherList::new();
  assert_eq!(3, f.extra_test_list.length_for_test());
  m_list.move_from(&mut f.extra_test_list);

  assert!(m_list.verify());
  assert_eq!(m_list.first(), extra_addr(&f.extra_nodes, 0));
  assert_eq!(3, m_list.length_for_test());

  // move assign from empty list
  f.extra_test_list.clear();
  assert_eq!(0, f.extra_test_list.length_for_test());
  m_list.move_from(&mut f.extra_test_list);
  assert_eq!(0, m_list.length_for_test());
  assert!(m_list.verify());
  assert!(m_list.first().is_null());
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, MoveCtor)
#[test]
fn threaded_list_move_ctor() -> Void {
  let mut f = fixture();
  assert_eq!(3, f.extra_test_list.length_for_test());
  let mut m_list = OtherList::steal(&mut f.extra_test_list);

  assert!(m_list.verify());
  assert_eq!(3, m_list.length_for_test());
  assert_eq!(m_list.first(), extra_addr(&f.extra_nodes, 0));

  // move construct from empty list
  f.extra_test_list.clear();
  assert_eq!(0, f.extra_test_list.length_for_test());
  let mut m_list2 = OtherList::steal(&mut f.extra_test_list);
  assert_eq!(0, m_list2.length_for_test());
  assert!(m_list2.verify());
  assert!(m_list2.first().is_null());
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, Remove)
#[test]
fn threaded_list_remove() -> Void {
  let mut f = fixture();
  assert_eq!(5, f.list.length_for_test());

  // Remove first
  assert_eq!(f.list.first(), node_addr(&f.nodes, 0));
  f.list.remove(node_addr(&f.nodes, 0));
  assert!(f.list.verify());
  assert_eq!(f.list.first(), node_addr(&f.nodes, 1));
  assert_eq!(4, f.list.length_for_test());

  // Remove middle
  f.list.remove(node_addr(&f.nodes, 2));
  assert!(f.list.verify());
  assert_eq!(3, f.list.length_for_test());
  assert_eq!(f.list.first(), node_addr(&f.nodes, 1));
  assert_eq!(f.list.at_for_test(1), node_addr(&f.nodes, 3));

  // Remove last
  f.list.remove(node_addr(&f.nodes, 4));
  assert!(f.list.verify());
  assert_eq!(2, f.list.length_for_test());
  assert_eq!(f.list.first(), node_addr(&f.nodes, 1));
  assert_eq!(f.list.at_for_test(1), node_addr(&f.nodes, 3));

  // Remove rest
  f.list.remove(node_addr(&f.nodes, 1));
  f.list.remove(node_addr(&f.nodes, 3));
  assert!(f.list.verify());
  assert_eq!(0, f.list.length_for_test());

  // Remove not found
  f.list.remove(node_addr(&f.nodes, 4));
  assert!(f.list.verify());
  assert_eq!(0, f.list.length_for_test());
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, Rewind)
#[test]
fn threaded_list_rewind() -> Void {
  let mut f = fixture();
  assert_eq!(3, f.extra_test_list.length_for_test());
  {
    let mut iter = f.extra_test_list.begin();
    loop {
      let end = f.extra_test_list.end();
      if iter.same(&end) {
        break;
      }
      if iter.get() == extra_addr(&f.extra_nodes, 2) {
        f.extra_test_list.rewind(&iter);
        break;
      }
      iter.advance();
    }
  }
  assert_eq!(2, f.extra_test_list.length_for_test());
  let mut iter = f.extra_test_list.begin();
  assert_eq!(iter.get(), extra_addr(&f.extra_nodes, 0));
  iter.advance();
  assert_eq!(iter.get(), extra_addr(&f.extra_nodes, 1));

  let begin = f.extra_test_list.begin();
  f.extra_test_list.rewind(&begin);
  assert_eq!(0, f.extra_test_list.length_for_test());
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, IterComp) + ConstIterComp
/// (Rust ThreadedListBase 无 const/非 const 迭代器之分, 两用例合并)
#[test]
fn threaded_list_iter_comp() -> Void {
  let mut f = fixture();
  let mut c_list = OtherList::steal(&mut f.extra_test_list);
  let mut found_first = false;
  let mut iter = c_list.begin();
  loop {
    let end = c_list.end();
    if iter.same(&end) {
      break;
    }
    if iter.same(&c_list.begin()) {
      found_first = true;
    }
    iter.advance();
  }
  assert!(found_first);
  teardown(&mut f);
  OK
}

/// TEST_F(ThreadedListTest, RemoveAt)
#[test]
fn threaded_list_remove_at() -> Void {
  let mut f = fixture();
  let mut it = f.list.begin();

  // Removing first
  let mut to_remove = f.list.first();
  it = f.list.remove_at(&it);
  assert_eq!(to_remove, node_addr(&f.nodes, 0));
  assert_eq!(f.list.first(), node_addr(&f.nodes, 1));
  assert!(it.same(&f.list.begin()));
  assert_eq!(it.get(), node_addr(&f.nodes, 1));
  // SAFETY: to_remove 为存活节点 (fixture 字段)
  assert_eq!(unsafe { *OtherTraits::next(to_remove) }, ptr::null_mut());
  assert!(!f.list.contains(to_remove));
  assert_eq!(4, f.list.length_for_test());
  assert!(f.list.verify());

  // Removing in the middle
  it.advance();
  to_remove = it.get();
  it = f.list.remove_at(&it);
  assert_eq!(it.get(), node_addr(&f.nodes, 3));
  assert!(!f.list.contains(to_remove));
  // SAFETY: 存活节点
  assert_eq!(unsafe { *OtherTraits::next(to_remove) }, ptr::null_mut());
  assert_eq!(
    unsafe { *OtherTraits::next(node_addr(&f.nodes, 1)) },
    node_addr(&f.nodes, 3)
  );
  assert_eq!(3, f.list.length_for_test());
  assert!(f.list.verify());

  // Removing last
  it.advance();
  to_remove = it.get();
  it = f.list.remove_at(&it);
  assert!(it.same(&f.list.end()));
  assert!(!f.list.contains(to_remove));
  assert_eq!(
    unsafe { *OtherTraits::next(node_addr(&f.nodes, 4)) },
    ptr::null_mut()
  );
  assert_eq!(2, f.list.length_for_test());
  assert!(f.list.verify());
  teardown(&mut f);
  OK
}
