//! v8-base/src/small_vector.rs 集成测试 — 对应 small-vector-unittest.cc
//! 原版模板以 (kInlineCapacity, 初始长度, ...) 参数化 trivial/nontrivial 两组;
//! Rust 侧以内联容量 K=4 为主实例, 析构计数用 Rc<Cell> 承接 NonTrivial 组。
//! 登记差异/跳过:
//! - NoAssign / ImplicitLifetimeNonTrivialConstructor: C++ 类型特征与隐式生命
//!   周期构造断言, Rust 无可观察对应物 → 跳过。
//! - MoveConstruct/MoveAssign: C++ moved-from 复位由 Rust take/replace 复现。

use std::{
  cell::Cell,
  mem::{replace, size_of, take},
  rc::Rc,
};

use aok::{OK, Void};
use v8_base::small_vector::SmallVector;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 NonTrivialTestCounter: 析构计数 (Clone 复制句柄, Drop 记一次)
struct Counted {
  counter: Rc<Cell<usize>>,
}
impl Counted {
  fn new(counter: &Rc<Cell<usize>>) -> Self {
    Self {
      counter: counter.clone(),
    }
  }
}
impl Default for Counted {
  fn default() -> Self {
    // 独立零计数句柄: resize 收缩路径不构造新元素, 该实现仅为满足 trait 约束
    Self {
      counter: Rc::new(Cell::new(0)),
    }
  }
}
impl Clone for Counted {
  fn clone(&self) -> Self {
    Self {
      counter: self.counter.clone(),
    }
  }
}
impl Drop for Counted {
  fn drop(&mut self) {
    self.counter.set(self.counter.get() + 1);
  }
}

type SV<T> = SmallVector<T, 4>;

#[test]
fn simple_construct() -> Void {
  let v = SV::<u32>::new();
  assert_eq!(v.size(), 0);
  assert!(v.is_empty());
  assert!(!v.is_big());
  assert_eq!(v.capacity(), 4); // kInlineSize
  assert_eq!(SV::<u32>::K_INLINE_SIZE, 4);
  // 原版 kInlineCapacity 边界组: K=0 形态首次 push 即退化堆段
  let mut z = SmallVector::<u32, 0>::new();
  assert_eq!(z.capacity(), 0);
  z.push_back(1);
  assert!(z.is_big());
  assert_eq!(z.as_slice(), &[1]);
  OK
}

#[test]
fn construct_from_list() -> Void {
  // 原版 ConstructFromList: init_list 构造 (Rust 以 from_slice 承接)
  let v = SV::<u32>::from_slice(&[1, 2, 3]);
  assert_eq!(v.size(), 3);
  assert!(!v.is_big());
  assert_eq!(v.as_slice(), &[1, 2, 3]);
  let v = SV::<u32>::from_slice(&[1, 2, 3, 4, 5]);
  assert!(v.is_big());
  assert_eq!(v.as_slice(), &[1, 2, 3, 4, 5]);
  // 空列表
  let v = SV::<u32>::from_slice(&[]);
  assert!(v.is_empty());
  assert!(!v.is_big());
  OK
}

#[test]
fn construct_from_vector() -> Void {
  // 原版 ConstructFromVector: 由既有 SmallVector/vector 拷贝构造
  let src = SV::<u32>::from_slice(&[7, 8]);
  let big_src = SV::<u32>::from_slice(&[1, 2, 3, 4, 5]);
  let v = src.clone();
  assert_eq!(v.as_slice(), &[7, 8]);
  let w = big_src.clone();
  assert!(w.is_big());
  assert_eq!(w.as_slice(), big_src.as_slice());
  // SmallVector(size) / SmallVector(size, value) 形态
  let d = SV::<u32>::with_size(3);
  assert_eq!(d.as_slice(), &[0, 0, 0]);
  let i = SV::<u32>::with_size_init(3, &9);
  assert_eq!(i.as_slice(), &[9, 9, 9]);
  let h = SV::<u32>::with_size(6);
  assert!(h.is_big());
  OK
}

#[test]
fn copy() -> Void {
  let v = SV::<u32>::from_slice(&[1, 2, 3]);
  let w = v.clone();
  assert_eq!(w, v);
  let big = SV::<u32>::from_slice(&[1, 2, 3, 4, 5]);
  let w = big.clone();
  assert!(w.is_big());
  assert_eq!(w.as_slice(), big.as_slice());
  OK
}

#[test]
fn copy_separation() -> Void {
  // 原版 CopySeparation: 拷贝后双向独立演化
  let mut v = SV::<u32>::from_slice(&[1, 2]);
  let mut w = v.clone();
  w.push_back(3);
  assert_eq!(v.as_slice(), &[1, 2]);
  assert_eq!(w.as_slice(), &[1, 2, 3]);
  v.extend_from_slice(&[3, 4, 5]);
  assert!(v.is_big());
  let mut x = v.clone();
  x.push_back(6);
  assert_eq!(v.size(), 5);
  assert_eq!(x.size(), 6);
  OK
}

#[test]
fn move_construct() -> Void {
  // 原版 MoveConstruct: 目标接管存储, 源复位内联空态
  let mut v = SV::<u32>::from_slice(&[1, 2, 3, 4, 5]);
  assert!(v.is_big());
  let moved = take(&mut v);
  assert!(v.is_empty());
  assert!(!v.is_big());
  assert_eq!(moved.as_slice(), &[1, 2, 3, 4, 5]);
  assert!(moved.is_big());
  OK
}

#[test]
fn move_assign() -> Void {
  // 原版 MoveAssign: trg 旧存储释放, src 复位
  let mut w = SV::<u32>::from_slice(&[9]);
  let mut src = SV::<u32>::from_slice(&[1, 2, 3, 4]);
  let old = replace(&mut w, SV::new()); // 清空目标 (原版 trg 析构旧段)
  drop(old);
  w = take(&mut src);
  assert_eq!(w.as_slice(), &[1, 2, 3, 4]);
  assert!(src.is_empty());
  assert!(!src.is_big());
  // 内联段互赋: moved-from 亦可继续正常使用
  let mut b = SV::<u32>::from_slice(&[2]);
  let a = take(&mut b);
  assert_eq!(a.as_slice(), &[2]);
  assert!(b.is_empty());
  b.push_back(3);
  assert_eq!(b.as_slice(), &[3]);
  OK
}

#[test]
fn iterators() -> Void {
  let v = SV::<u32>::from_slice(&[1, 2, 3]);
  assert_eq!(v.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
  assert_eq!(v.riter().copied().collect::<Vec<_>>(), vec![3, 2, 1]);
  // 原版 begin()/end() 指针算术: data() 即首元素指针
  assert_eq!(v.data(), v.as_slice().as_ptr());
  // &/&mut 的 IntoIterator 形态
  assert_eq!(v.as_slice().iter().copied().sum::<u32>(), 6);
  let mut m = SV::<u32>::from_slice(&[1, 2]);
  for x in &mut m {
    *x *= 10;
  }
  assert_eq!(m.as_slice(), &[10, 20]);
  let sum: u32 = (&m).into_iter().sum();
  assert_eq!(sum, 30);
  OK
}

#[test]
fn accessors() -> Void {
  let mut v = SV::<i32>::from_slice(&[1, -2, 3]);
  assert_eq!(v.front(), Some(&1));
  assert_eq!(v.back(), Some(&3));
  assert_eq!(v.at(1), Some(&-2));
  if let Some(x) = v.at_mut(1) {
    *x = 20;
  }
  assert_eq!(v.as_slice(), &[1, 20, 3]);
  let empty = SV::<i32>::new();
  assert_eq!(empty.front(), None); // 原版空表 front() 为 DCHECK 路径
  assert_eq!(empty.back(), None);
  assert_eq!(empty.at(0), None);
  OK
}

#[test]
fn push_pop() -> Void {
  // 原版 PushPop: 跨内联边界增长 + 尾部弹出
  let mut v = SV::<u32>::new();
  for i in 0..4 {
    v.push_back(i);
  }
  assert!(!v.is_big());
  v.push_back(4); // 触发 Grow: bit_ceil(max(0, 2*4)) = 8
  assert!(v.is_big());
  assert_eq!(v.capacity(), 8);
  assert_eq!(v.as_slice(), &[0, 1, 2, 3, 4]);
  v.pop_back(1);
  assert_eq!(v.as_slice(), &[0, 1, 2, 3]);
  v.pop_back(2);
  assert_eq!(v.as_slice(), &[0, 1]);
  v.pop_back(2);
  assert!(v.is_empty());
  // 增长策略: bit_ceil(max(min_capacity, 2*capacity)) (原版 Grow)
  let mut w = SV::<u32>::new();
  w.reserve(5);
  assert_eq!(w.capacity(), 8); // max(5, 8) → 8
  w.reserve(9);
  assert_eq!(w.capacity(), 16); // max(9, 16) → 16
  OK
}

#[test]
fn emplace_pop() -> Void {
  // 原版 EmplacePop: emplace_back 原位构造 (Rust 直收值) 与 pop 析构时机
  let counter = Rc::new(Cell::new(0usize));
  let mut v = SV::<Counted>::new();
  v.emplace_back(Counted::new(&counter));
  v.push_back(Counted::new(&counter));
  assert_eq!(v.size(), 2);
  assert_eq!(counter.get(), 0);
  v.pop_back(1);
  assert_eq!(counter.get(), 1);
  OK
}

#[test]
fn insert_simple() -> Void {
  let mut v = SV::<u32>::from_slice(&[1, 3]);
  v.insert_one(1, 2);
  assert_eq!(v.as_slice(), &[1, 2, 3]);
  // 头插与尾插 (pos == end)
  v.insert_one(0, 0);
  assert_eq!(v.as_slice(), &[0, 1, 2, 3]);
  v.insert_one(4, 4);
  assert!(v.is_big());
  assert_eq!(v.as_slice(), &[0, 1, 2, 3, 4]);
  OK
}

#[test]
fn insert_multiple() -> Void {
  let mut v = SV::<u32>::from_slice(&[1, 5]);
  v.insert_at(1, 3, &0);
  assert_eq!(v.as_slice(), &[1, 0, 0, 0, 5]);
  let mut v = SV::<u32>::from_slice(&[1]);
  v.insert_at(1, 4, &2);
  assert_eq!(v.as_slice(), &[1, 2, 2, 2, 2]);
  OK
}

#[test]
fn insert_list() -> Void {
  // 原版 insert(pos, init_list)
  let mut v = SV::<u32>::from_slice(&[1, 4]);
  v.insert_slice_at(1, &[2, 3]);
  assert_eq!(v.as_slice(), &[1, 2, 3, 4]);
  assert!(!v.is_big());
  OK
}

#[test]
fn insert_iter() -> Void {
  // 原版 insert(pos, first, last) 迭代器域 (Rust 以迭代器物化切片承接)
  let mut v = SV::<u32>::from_slice(&[1, 2, 3, 4]);
  let src: Vec<u32> = (0..3).collect();
  v.insert_slice_at(0, &src);
  assert!(v.is_big());
  assert_eq!(v.as_slice(), &[0, 1, 2, 1, 2, 3, 4]);
  // 空区间插入不动表
  let len = v.size();
  v.insert_slice_at(2, &[]);
  assert_eq!(v.size(), len);
  OK
}

#[test]
fn erase() -> Void {
  // 原版 Erase: erase(begin, end) 全表清空
  let mut v = SV::<u32>::from_slice(&[1, 2, 3]);
  let end = v.erase(0, 3);
  assert_eq!(v.as_slice(), &[] as &[u32]);
  assert_eq!(end, 0);
  // 非平凡元素全段析构
  let counter = Rc::new(Cell::new(0usize));
  let mut v: SV<Counted> = SV::new();
  for _ in 0..3 {
    v.push_back(Counted::new(&counter));
  }
  v.erase(0, 3);
  assert_eq!(counter.get(), 3);
  OK
}

#[test]
fn erase_one() -> Void {
  let mut v = SV::<u32>::from_slice(&[1, 2, 3, 4]);
  let pos = v.erase_one(1);
  assert_eq!(v.as_slice(), &[1, 3, 4]);
  assert_eq!(pos, 1); // 返回新 end 位置 (原版 end_ 前移)
  let mut v = SV::<u32>::from_slice(&[1]);
  assert_eq!(v.erase_one(0), 0);
  assert!(v.is_empty());
  OK
}

#[test]
fn erase_range() -> Void {
  let mut v = SV::<u32>::from_slice(&[1, 2, 3, 4, 5]);
  v.erase(1, 4);
  assert_eq!(v.as_slice(), &[1, 5]);
  let mut v = SV::<u32>::from_slice(&[1, 2]);
  assert_eq!(v.erase(1, 2), 1);
  assert_eq!(v.as_slice(), &[1]);
  // 中段擦除析构 [1,4) 三个元素
  let counter = Rc::new(Cell::new(0usize));
  let mut v: SV<Counted> = SV::new();
  for _ in 0..5 {
    v.push_back(Counted::new(&counter));
  }
  v.erase(1, 4);
  assert_eq!(counter.get(), 3);
  assert_eq!(v.size(), 2);
  OK
}

#[test]
fn resize() -> Void {
  // 原版 Resize: 增长 default 填充 / 收缩尾部析构
  let mut v = SV::<u32>::with_size(2);
  v.resize(6);
  assert!(v.is_big());
  assert_eq!(v.as_slice(), &[0, 0, 0, 0, 0, 0]);
  v.resize(3);
  assert_eq!(v.as_slice(), &[0, 0, 0]);
  OK
}

#[test]
fn resize_init() -> Void {
  // 原版 ResizeInit: resize(new_size, initial_value)
  let mut v = SV::<u32>::new();
  v.resize_with_value(5, &7);
  assert!(v.is_big());
  assert_eq!(v.as_slice(), &[7, 7, 7, 7, 7]);
  v.resize_with_value(2, &1);
  assert_eq!(v.as_slice(), &[7, 7]);
  // 收缩析构计数 (NonTrivial 组)
  let counter = Rc::new(Cell::new(0usize));
  let template = Counted::new(&counter);
  let mut v: SV<Counted> = SV::with_size_init(3, &template);
  assert_eq!(counter.get(), 0); // 模板未消耗, 表内 3 份克隆存活
  v.resize(1);
  assert_eq!(counter.get(), 2); // 析构表内 2 份
  drop(template);
  assert_eq!(counter.get(), 3);
  OK
}

#[test]
fn reserve() -> Void {
  // 原版 Reserve: 只增不减; new_capacity <= capacity 时不动
  let mut v = SV::<u32>::from_slice(&[1]);
  v.reserve(3);
  assert_eq!(v.capacity(), 4);
  assert!(!v.is_big());
  v.reserve(4);
  assert_eq!(v.capacity(), 4);
  v.reserve(5);
  assert!(v.is_big());
  assert_eq!(v.capacity(), 8);
  // reserve 不影响 size
  assert_eq!(v.as_slice(), &[1]);
  OK
}

#[test]
fn clear_trivial() -> Void {
  let mut v = SV::<u32>::from_slice(&[1, 2, 3]);
  v.clear();
  assert!(v.is_empty());
  assert_eq!(v.as_slice(), &[] as &[u32]);
  assert!(!v.is_big());
  v.push_back(1);
  assert_eq!(v.as_slice(), &[1]);
  OK
}

#[test]
fn clear_nontrivial() -> Void {
  // 原版 ClearNonTrivial: 全量析构且只析构一次
  let counter = Rc::new(Cell::new(0usize));
  let mut v: SV<Counted> = SV::new();
  for _ in 0..5 {
    v.push_back(Counted::new(&counter));
  }
  assert!(v.is_big());
  v.clear();
  assert_eq!(counter.get(), 5);
  assert!(v.is_empty());
  v.clear(); // 幂等
  assert_eq!(counter.get(), 5);
  OK
}

#[test]
fn shrink_to_fit() -> Void {
  // 元素数回落到 K 内时可收缩回内联段 (原版 ShrinkToFit 语义对齐)
  let mut v = SV::<u32>::from_slice(&[1, 2, 3, 4, 5]);
  assert!(v.is_big());
  v.pop_back(1);
  v.shrink_to_fit();
  assert!(!v.is_big());
  assert_eq!(v.as_slice(), &[1, 2, 3, 4]);
  // into_vec 消耗自身移交堆段
  let owned = v.into_vec();
  assert_eq!(owned, vec![1, 2, 3, 4]);
  // 元素数仍超 K 时不收缩
  let mut v = SV::<u32>::from_slice(&[1, 2, 3, 4, 5]);
  v.shrink_to_fit();
  assert!(v.is_big());
  // 内联段调用为 no-op
  let mut v = SV::<u32>::from_slice(&[1]);
  v.shrink_to_fit();
  assert!(!v.is_big());
  OK
}

#[test]
fn to_vector() -> Void {
  // 原版 ToVector 语义: Vector/ConstVector 视图与表共享存储
  let mut v = SV::<u32>::from_slice(&[1, 2, 3]);
  assert_eq!(v.as_vector().as_slice(), &[1, 2, 3]);
  {
    let mut mv = v.as_mut_vector();
    mv.as_mut_slice()[0] = 9;
  }
  assert_eq!(v.as_slice(), &[9, 2, 3]);
  // 空表视图
  let e = SV::<u32>::new();
  assert!(e.as_vector().is_empty());
  OK
}

#[test]
fn stack() -> Void {
  // 原版 Stack: IsOnStack(&vec, vec.begin()) — 内联段数据地址落在表对象内
  let v = SV::<u32>::from_slice(&[1, 2]);
  let base = &v as *const SV<u32> as usize;
  let p = v.data() as usize;
  assert!(p >= base && p < base + size_of::<SV<u32>>());
  // 堆段数据落在表对象之外
  let big = SV::<u32>::from_slice(&[1, 2, 3, 4, 5]);
  let base = &big as *const SV<u32> as usize;
  let p = big.data() as usize;
  assert!(p < base || p >= base + size_of::<SV<u32>>());
  OK
}

#[test]
fn resize_no_init() -> Void {
  // 原版 ResizeNoInit 仅限 kHasTrivialElement; Rust unsafe 契约承接
  let mut v = SV::<u32>::from_slice(&[1, 2]);
  unsafe {
    v.resize_no_init(4);
    v.as_mut_slice()[2] = 3;
    v.as_mut_slice()[3] = 4;
  }
  assert_eq!(v.as_slice(), &[1, 2, 3, 4]);
  unsafe {
    v.resize_no_init(1);
  }
  assert_eq!(v.as_slice(), &[1]);
  OK
}

#[test]
fn implicit_lifetime_trivial_constructor() -> Void {
  // 原版 ImplicitLifetimeNonTrivialConstructor 的平凡侧: u32 数组直接构造
  // (非平凡构造组登记跳过, 见文件头)
  let mut v = SV::<u32>::new();
  v.extend_from_slice(&[1, 2, 3]);
  assert_eq!(v.as_slice(), &[1, 2, 3]);
  let w = SV::<u32>::from_slice(v.as_slice());
  assert_eq!(w.as_slice(), v.as_slice());
  // ZST 全链路可用 (堆段悬垂指针形态)
  let mut z = SV::<()>::new();
  for _ in 0..8 {
    z.push_back(());
  }
  assert!(z.is_big());
  assert_eq!(z.size(), 8);
  z.pop_back(8);
  assert!(z.is_empty());
  OK
}

#[test]
fn equality_and_debug() -> Void {
  let a = SV::<u32>::from_slice(&[1, 2, 3]);
  let b = SV::<u32>::from_slice(&[1, 2, 3]);
  let c = SV::<u32>::with_size(3);
  assert_eq!(a, b);
  assert_ne!(a, c);
  assert_eq!(format!("{a:?}"), "[1, 2, 3]");
  // 内联/堆段混比按元素
  let mut d = SV::<u32>::new();
  d.extend_from_slice(&[1, 2, 3]);
  d.insert_at(3, 0, &0);
  assert_eq!(a, d);
  OK
}
