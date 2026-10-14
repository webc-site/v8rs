//! v8-base/src/intrusive_set.rs 集成测试 — 对应 v8/src/base/intrusive-set.h
//!
//! 上游 v8/test/unittests 无 intrusive-set 测试文件, 本文件按头文件语义推导断言:
//! - Add: DCHECK(!Contains(x)); Index(x) = size; push_back
//! - Remove: DCHECK(Contains(x)) + DCHECK_EQ(x, elements_[index]); swap-pop
//!   (Index(elements_.back()) = index; elements_[index] = back; pop_back)
//! - iterator: 迭代中 Add 的新元素会被访问; 移除当前元素时被换入的元素会被访问
//!   (原版以 last_index_location_ 识别, Rust 侧以索引 Cell 地址等价承接)

use std::ptr;

use aok::{OK, Void};
use v8_base::intrusive_set::{
  GetIndex, IntrusiveContainer, IntrusiveSet, IntrusiveSetIndex, K_NOT_IN_SET,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版元素: 自带 IntrusiveSetIndex 槽 (此处以堆上 Item 承接)
struct Item {
  #[expect(dead_code)]
  val: i32,
  index: IntrusiveSetIndex,
}

/// 句柄类型即原版 T (指针样值, Copy + 地址相等比较)
type Handle = *mut Item;

/// 原版 GetIntrusiveSetIndex 函子
#[derive(Clone, Copy)]
struct ItemIndex;

impl GetIndex<Handle> for ItemIndex {
  fn index_of<'a>(&self, x: &Handle) -> &'a IntrusiveSetIndex {
    // SAFETY: 原版契约 — 句柄在其被移除且迭代结束前指向存活的 Item
    unsafe { &(*(*x)).index }
  }
}

/// 测试夹具: 堆上 n 个 Item + 其句柄 (地址稳定, 与 C++ 栈/堆数组等价)
struct Fixture {
  /// 持有节点存活 (测试期间不读)
  _items: Vec<Item>,
  handles: Vec<Handle>,
}

impl Fixture {
  fn new(n: i32) -> Self {
    // collect 后 Vec 不再变动, 元素地址稳定 (等价原版栈/堆数组)
    let mut items: Vec<Item> = (0..n)
      .map(|i| Item {
        val: i,
        index: IntrusiveSetIndex::new(),
      })
      .collect();
    let handles: Vec<Handle> = items.iter_mut().map(ptr::from_mut).collect();
    Self {
      _items: items,
      handles,
    }
  }

  fn set(&self) -> IntrusiveSet<Handle, ItemIndex, Vec<Handle>> {
    IntrusiveSet::new(Vec::new(), ItemIndex)
  }

  fn index_of(&self, h: Handle) -> usize {
    ItemIndex.index_of(&h).get()
  }
}

/// 原版 Add / Contains 与索引登记
#[test]
fn intrusive_set_add_contains() -> Void {
  let fx = Fixture::new(4);
  let set = fx.set();
  assert_eq!(0, set.size());
  assert!(!set.contains(fx.handles[0]));
  assert_eq!(K_NOT_IN_SET, fx.index_of(fx.handles[0]));

  for (i, &h) in fx.handles.iter().enumerate().take(4) {
    set.add(h);
    assert!(set.contains(h));
    assert_eq!(i, set.index_slot(&h).get());
    assert_eq!(i + 1, set.size());
  }
  // 重复 Add 由 DCHECK(!Contains) 拦截 (debug 构建); 此处校验快照顺序
  assert_eq!(fx.handles[..], set.snapshot()[..]);
  OK
}

/// 原版 Remove: swap-pop (末元素回填到被删槽位)
#[test]
fn intrusive_set_remove_swap_pop() -> Void {
  let fx = Fixture::new(4);
  let set = fx.set();
  for &h in fx.handles.iter().take(4) {
    set.add(h);
  }

  // 移除中间元素 (index 1) → elements.back() (index 3) 回填到 1
  set.remove(fx.handles[1]);
  assert!(!set.contains(fx.handles[1]));
  assert_eq!(K_NOT_IN_SET, fx.index_of(fx.handles[1]));
  assert_eq!(3, set.size());
  assert_eq!(1, fx.index_of(fx.handles[3]));
  // swap-pop: 末元素 h3 回填到 index 1 → 顺序 [h0, h3, h2]
  assert_eq!(
    [fx.handles[0], fx.handles[3], fx.handles[2]],
    set.snapshot().as_slice()
  );

  // 移除首元素 (index 0) → 现末元素 (handles[2]) 回填到 0 (容器为 [h0, h3, h2])
  set.remove(fx.handles[0]);
  assert!(!set.contains(fx.handles[0]));
  assert_eq!(2, set.size());
  assert_eq!(0, fx.index_of(fx.handles[2]));
  assert_eq!(1, fx.index_of(fx.handles[3]));
  assert_eq!([fx.handles[2], fx.handles[3]], set.snapshot().as_slice());

  // 移除当前末元素自身 (与原版本身 swap-pop 到自身槽位)
  set.remove(fx.handles[3]);
  assert_eq!(1, set.size());
  assert_eq!([fx.handles[2]], set.snapshot().as_slice());
  assert_eq!(0, fx.index_of(fx.handles[2]));

  set.remove(fx.handles[2]);
  assert_eq!(0, set.size());
  assert!(set.snapshot().is_empty());
  for i in 0..4 {
    assert!(!set.contains(fx.handles[i]));
    assert_eq!(K_NOT_IN_SET, fx.index_of(fx.handles[i]));
  }
  OK
}

/// 原版迭代器: 访问全部元素, 顺序与容器一致
#[test]
fn intrusive_set_iter_visits_all() -> Void {
  let fx = Fixture::new(6);
  let set = fx.set();
  for &h in fx.handles.iter() {
    set.add(h);
  }
  let visited: Vec<Handle> = set.iter().collect();
  assert_eq!(set.snapshot(), visited);
  assert_eq!(6, visited.len());
  for &h in fx.handles.iter() {
    assert!(visited.contains(&h));
  }

  // 空集合迭代为空
  let empty = fx.set();
  assert_eq!(0, empty.iter().count());
  OK
}

/// 原版注释: 迭代期间 Add 的元素会被本次迭代访问
#[test]
fn intrusive_set_iter_add_during_iteration() -> Void {
  let fx = Fixture::new(6);
  let set = fx.set();
  for &h in fx.handles.iter().take(3) {
    set.add(h);
  }

  let mut visited = Vec::new();
  for x in set.iter() {
    visited.push(x);
    // 首次访问时追加剩余 3 个元素 (原版: 新元素纳入本次迭代)
    if visited.len() == 1 {
      for &h in fx.handles.iter().skip(3) {
        set.add(h);
      }
    }
  }
  assert_eq!(6, visited.len());
  for &h in fx.handles.iter() {
    assert!(visited.contains(&h));
  }
  assert_eq!(6, set.size());
  OK
}

/// 原版注释: 迭代中移除"当前"元素合法 — 被换入的元素仍会被访问
#[test]
fn intrusive_set_iter_remove_current_element() -> Void {
  let fx = Fixture::new(5);
  let set = fx.set();
  for &h in fx.handles.iter() {
    set.add(h);
  }

  let mut visited = Vec::new();
  for x in set.iter() {
    visited.push(x);
    set.remove(x);
  }
  // 每个元素恰访问一次, 且移除后集合为空
  assert_eq!(5, visited.len());
  let uniq: Vec<Handle> = {
    let mut v = visited.clone();
    v.sort_unstable();
    v.dedup();
    v
  };
  assert_eq!(5, uniq.len());
  assert_eq!(0, set.size());
  for &h in fx.handles.iter() {
    assert!(!set.contains(h));
  }
  OK
}

/// 原版 IntrusiveSetIndex: value = kNotInSet 起, 引用读写
#[test]
fn intrusive_set_index_surface() -> Void {
  let idx = IntrusiveSetIndex::new();
  assert_eq!(K_NOT_IN_SET, idx.get());
  assert_eq!(K_NOT_IN_SET, usize::MAX);
  idx.set(3);
  assert_eq!(3, idx.get());
  // addr 稳定 (迭代器以索引内存地址识别 swap-pop)
  let a = idx.addr();
  assert_eq!(a, idx.addr());
  assert!(!a.is_null());
  // Default 与 new 同义
  assert_eq!(K_NOT_IN_SET, IntrusiveSetIndex::default().get());
  // 同一槽的地址互异
  let other = IntrusiveSetIndex::new();
  assert_ne!(a, other.addr());
  OK
}

/// 原版 Container 模板参数面 (std::vector 的 size/push_back/pop_back/back/[])
#[test]
fn intrusive_set_container_vec_surface() -> Void {
  let mut v: Vec<u32> = Vec::new();
  assert_eq!(0, v.ilen());
  assert_eq!(None, v.iget(0));
  assert_eq!(None, v.ipop());

  v.ipush(7);
  v.ipush(8);
  assert_eq!(2, v.ilen());
  assert_eq!(Some(&7), v.iget(0));
  assert_eq!(Some(&8), v.iget(1));
  assert_eq!(None, v.iget(2));

  assert!(v.iset(1, 9));
  assert_eq!(Some(&9), v.iget(1));
  assert!(!v.iset(5, 0));

  assert_eq!(Some(9), v.ipop());
  assert_eq!(1, v.ilen());
  assert_eq!(Some(7), v.ipop());
  assert_eq!(0, v.ilen());
  OK
}
