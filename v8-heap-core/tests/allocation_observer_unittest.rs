//! v8-heap-core allocation 族 + allocation-observer 族金标准单测。
//!
//! 迁移源 (实测本仓): v8/test/unittests/heap/allocation-observer-unittest.cc (160 行)。
//! AddAndRemoveUnusedObservers (unittest.cc:24-45) 与 Step (unittest.cc:78-108) 逐数值/回调断言 1:1 迁移。
//! RecursiveAdd (unittest.cc:127-142) 与 RecursiveRemove (unittest.cc:161-175) 推迟登记 (见下)。
//! 另附 AllocationResult / AllocationStats 语义金标准 (源自 allocation-result.h/allocation-stats.h)。
//!
//! 推迟登记 (双行号 + 原因, 禁假绿)。
//! RecursiveAdd unittest.cc:127-142 ↔ 本仓 allocation_observer.rs Invoke pending_added_ 区 (cc:118-130)。
//! RecursiveRemove unittest.cc:161-175 ↔ 本仓 Invoke pending_removed_ 区 (cc:134-155)。
//! 原因: 二者依赖观察者 Step() 内经回指 AllocationCounter* 反向调用 counter 的 Add/Remove (C++ 重入)。
//! 本 crate 观察者为禁 dyn 手动 fn 指针虚表 (ObserverRef), Step 无 counter 句柄, 且借用检查禁止迭代
//! observers_ 期对外借 &mut counter — 重入路径不可安全迁移 (pending 队列承接逻辑本身已 1:1 落地,
//! 仅触发入口不可达)。故不伪造该二例的通过, 如实申报为推迟面。

use core::mem::size_of;
use std::cell::Cell;

use aok::{OK, Void};
use v8_heap_core::{
  allocation::{AllocationOrigin, AllocationResult, AllocationStats, BasePage},
  allocation_observer::{AllocationCounter, AllocationObserver, ObserverRef},
};
use v8_objects_core::{
  heap_object::HeapObject,
  tagged::{Object, Tagged},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 unittest.cc:15-21 UnusedObserver — Step 内 CHECK(false) (从不被调用)。
/// 迁移为记录被调次数, 测试末尾断言为 0 (禁 panic, 等价语义)。
struct UnusedObserver {
  step_size: isize,
  invoked: Cell<usize>,
}

impl UnusedObserver {
  fn new(step_size: isize) -> Self {
    Self {
      step_size,
      invoked: Cell::new(0),
    }
  }
}

impl AllocationObserver for UnusedObserver {
  fn step(&mut self, _bytes_allocated: i32, _soon_object: usize, _size: usize) {
    // 原版 cc:19 CHECK(false) — 本例不应触发; 记录以便测试断言未触发。
    self.invoked.set(self.invoked.get() + 1);
  }

  fn step_size(&self) -> isize {
    self.step_size
  }
}

/// 原版 unittest.cc:48-75 VerifyStepObserver — 校验每次 Step 的 bytes/size 并计数。
struct VerifyStepObserver {
  step_size: isize,
  invocations: usize,
  do_not_invoke: bool,
  expected_bytes_allocated: i32,
  expected_size: usize,
}

impl VerifyStepObserver {
  fn new(step_size: isize) -> Self {
    Self {
      step_size,
      invocations: 0,
      do_not_invoke: false,
      expected_bytes_allocated: 0,
      expected_size: 0,
    }
  }

  /// 原版 cc:61 ExpectNoInvocation。
  fn expect_no_invocation(&mut self) {
    self.do_not_invoke = true;
  }

  /// 原版 cc:62-66 Expect。
  fn expect(&mut self, expected_bytes_allocated: i32, expected_size: usize) {
    self.do_not_invoke = false;
    self.expected_bytes_allocated = expected_bytes_allocated;
    self.expected_size = expected_size;
  }
}

impl AllocationObserver for VerifyStepObserver {
  fn step(&mut self, bytes_allocated: i32, _soon_object: usize, size: usize) {
    // 原版 cc:54 CHECK(!do_not_invoke_)
    assert!(!self.do_not_invoke, "原版 cc:54 CHECK(!do_not_invoke_)");
    self.invocations += 1;
    // 原版 cc:57-58 CHECK_EQ(expected_bytes_allocated_, bytes_allocated); CHECK_EQ(expected_size_, size)
    assert_eq!(self.expected_bytes_allocated, bytes_allocated, "原版 cc:57");
    assert_eq!(self.expected_size, size, "原版 cc:58");
  }

  fn step_size(&self) -> isize {
    self.step_size
  }
}

/// 原版 unittest.cc:24-45 AddAndRemoveUnusedObservers — 逐值断言 1:1。
#[test]
fn add_and_remove_unused_observers() -> Void {
  let mut counter = AllocationCounter::default();
  // cc:26 CHECK_EQ(SIZE_MAX, counter.NextBytes())
  assert_eq!(usize::MAX, counter.next_bytes());

  let mut observer100 = UnusedObserver::new(100);
  let mut observer200 = UnusedObserver::new(200);

  // cc:31-32
  counter.add_allocation_observer(ObserverRef::new(&mut observer200));
  assert_eq!(counter.next_bytes(), 200);

  // cc:34-35
  counter.add_allocation_observer(ObserverRef::new(&mut observer100));
  assert_eq!(counter.next_bytes(), 100);

  // cc:37-38
  counter.advance_allocation_observers(90);
  assert_eq!(counter.next_bytes(), 10);

  // cc:40-41
  counter.remove_allocation_observer(ObserverRef::new(&mut observer100));
  assert_eq!(counter.next_bytes(), 110);

  // cc:43-44
  counter.remove_allocation_observer(ObserverRef::new(&mut observer200));
  assert_eq!(usize::MAX, counter.next_bytes());

  // 两个观察者均不应被 Step 触发 (原版 UnusedObserver::Step CHECK(false))
  assert_eq!(observer100.invoked.get(), 0);
  assert_eq!(observer200.invoked.get(), 0);

  OK
}

/// 原版 unittest.cc:78-108 Step — 回调 + NextBytes 逐值断言 1:1。
#[test]
fn step_invokes_observers() -> Void {
  let mut counter = AllocationCounter::default();
  assert_eq!(usize::MAX, counter.next_bytes());
  // cc:81 const Address kSomeObjectAddress = 8
  const K_SOME_OBJECT_ADDRESS: usize = 8;

  let mut observer100 = VerifyStepObserver::new(100);
  let mut observer200 = VerifyStepObserver::new(200);

  counter.add_allocation_observer(ObserverRef::new(&mut observer100));
  counter.add_allocation_observer(ObserverRef::new(&mut observer200));

  // cc:89-90 期望
  observer100.expect(90, 8);
  observer200.expect_no_invocation();

  // cc:92-93
  counter.advance_allocation_observers(90);
  counter.invoke_allocation_observers(K_SOME_OBJECT_ADDRESS, 8, 10);
  // cc:94-97
  assert_eq!(observer100.invocations, 1);
  assert_eq!(observer200.invocations, 0);
  assert_eq!(
    counter.next_bytes(),
    10 /* aligned */ + 100 /* smallest step */
  );

  // cc:99-100 期望
  observer100.expect(90, 16);
  observer200.expect(180, 16);

  // cc:102-107
  counter.advance_allocation_observers(90);
  counter.invoke_allocation_observers(K_SOME_OBJECT_ADDRESS, 16, 20);
  assert_eq!(observer100.invocations, 2);
  assert_eq!(observer200.invocations, 1);
  assert_eq!(
    counter.next_bytes(),
    20 /* aligned */ + 100 /* smallest step */
  );

  OK
}

/// AllocationOrigin 金标准 (原版 allocation-result.h:15-22): 6 项逐名逐值。
#[test]
fn allocation_origin_gold() -> Void {
  // 判别式即 ABI (原版 h:16-18)
  assert_eq!(AllocationOrigin::GeneratedCode as i32, 0);
  assert_eq!(AllocationOrigin::Runtime as i32, 1);
  assert_eq!(AllocationOrigin::Gc as i32, 2);
  // 别名常量 (原版 h:19-21)
  assert_eq!(
    AllocationOrigin::K_FIRST_ALLOCATION_ORIGIN,
    AllocationOrigin::GeneratedCode
  );
  assert_eq!(
    AllocationOrigin::K_LAST_ALLOCATION_ORIGIN,
    AllocationOrigin::Gc
  );
  assert_eq!(AllocationOrigin::K_NUMBER_OF_ALLOCATION_ORIGINS, 3);
  OK
}

/// AllocationResult 金标准 (原版 allocation-result.h:26-71)。
#[test]
fn allocation_result_gold() -> Void {
  // 布局静态断言承接原版 h:71 (编译期已 assert; 此处运行期复验)
  assert_eq!(size_of::<AllocationResult>(), 8);

  // Failure h:28 / IsFailure h:38 / operator bool h:62
  let failure = AllocationResult::failure();
  assert!(failure.is_failure());
  assert!(!failure.ok());

  // FromObject h:30 (非空 tag=01 堆对象指针)
  let object: Tagged<HeapObject> = Tagged::new(0x1001);
  let result = AllocationResult::from_object(object);
  assert!(!result.is_failure());
  assert!(result.ok());

  // ToObject h:52 回取同一 tagged
  assert_eq!(result.to_object().ptr(), object.ptr());
  // ToAddress h:57 = object_.address()
  assert_eq!(result.to_address(), 0x1000);

  // To<T> h:40-45: 成功写 obj 返回 true
  let mut obj: Tagged<Object> = Tagged::NULL;
  assert!(result.to(&mut obj));
  assert_eq!(obj.ptr(), object.ptr());

  // To 在失败结果上返回 false
  let mut obj2: Tagged<Object> = Tagged::NULL;
  assert!(!failure.to(&mut obj2));

  // ToObjectChecked h:47 (成功路径; 失败路径为 v8_fatal abort, 测试不触发)
  assert_eq!(result.to_object_checked().ptr(), object.ptr());
  OK
}

/// 伪造页指针: BasePage 为不透明前向声明, 仅作用账目 map 的键 (从不解引用)。
#[inline]
fn fake_page(addr: usize) -> *const BasePage {
  addr as *const BasePage
}

/// AllocationStats 金标准 (原版 allocation-stats.h:24-115): 容量/大小/max 追踪/清零/页账目。
#[test]
fn allocation_stats_gold() -> Void {
  let mut stats = AllocationStats::new();
  // ctor→Clear h:26/39: 全零
  assert_eq!(stats.capacity(), 0);
  assert_eq!(stats.max_capacity(), 0);
  assert_eq!(stats.size(), 0);

  // IncreaseCapacity h:90: 刷新 max_capacity_
  stats.increase_capacity(100);
  assert_eq!(stats.capacity(), 100);
  assert_eq!(stats.max_capacity(), 100);
  stats.increase_capacity(50);
  assert_eq!(stats.capacity(), 150);
  assert_eq!(stats.max_capacity(), 150);
  // DecreaseCapacity h:84: 不降 max
  stats.decrease_capacity(30);
  assert_eq!(stats.capacity(), 120);
  assert_eq!(stats.max_capacity(), 150);

  // IncreaseAllocatedBytes h:62 / Size h:55
  let page_a = fake_page(0xA00);
  let page_b = fake_page(0xB00);
  stats.increase_allocated_bytes(40, page_a);
  stats.increase_allocated_bytes(60, page_b);
  assert_eq!(stats.size(), 100);

  // DecreaseAllocatedBytes h:75
  stats.decrease_allocated_bytes(25, page_a);
  assert_eq!(stats.size(), 75);

  // ClearSize h:45: 仅清 size
  stats.clear_size();
  assert_eq!(stats.size(), 0);
  assert_eq!(stats.capacity(), 120);

  // Clear h:39: 全清 (含 max)
  stats.clear();
  assert_eq!(stats.capacity(), 0);
  assert_eq!(stats.max_capacity(), 0);
  assert_eq!(stats.size(), 0);

  // operator=/assign h:28 逐项拷贝
  let mut other = AllocationStats::new();
  other.increase_capacity(77);
  other.increase_allocated_bytes(11, page_a);
  let mut copy = AllocationStats::new();
  copy.assign(&other);
  assert_eq!(copy.capacity(), 77);
  assert_eq!(copy.max_capacity(), 77);
  assert_eq!(copy.size(), 11);

  OK
}

/// AllocationStats DEBUG-only 页账目 (原版 h:57 AllocatedOnPage, #ifdef DEBUG)。
#[cfg(debug_assertions)]
#[test]
fn allocation_stats_debug_page_account() -> Void {
  let mut stats = AllocationStats::new();
  let page = fake_page(0xC00);
  stats.increase_allocated_bytes(32, page);
  stats.increase_allocated_bytes(8, page);
  // h:70-71 累加
  assert_eq!(stats.allocated_on_page(page), Some(40));
  stats.decrease_allocated_bytes(10, page);
  // h:79-80 递减
  assert_eq!(stats.allocated_on_page(page), Some(30));
  // 无账页 → None (原版 at() 抛, 本仓禁 panic → Option)
  assert_eq!(stats.allocated_on_page(fake_page(0xDEAD)), None);
  OK
}
