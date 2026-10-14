//! tests/free_list_unittest.rs — v8-heap-core/src/free_list.rs 金标准用例 (HC-23R)。
//!
//! 原版测试勘定实测: v8/test/unittests/heap 无 free-list 专属 unittest 文件 (find 实测仅
//! cppgc/free-list-unittest.cc, 属 cppgc-internal 同族禁混入面)。直接相关可迁用例 =
//! v8/test/unittests/heap/spaces-unittest.cc 的 2 个纯逻辑 TEST_F (h:488-490 FRIEND_TEST
//! 指向的用例即此):
//!   SpacesTest.FreeListManySelectFreeListCategoryType (spaces-unittest.cc:485-534) →
//!     本档 free_list_many_select_free_list_category_type, 1:1 复刻 (EXPECT_* 非致命语义
//!     以失败计数收束承接)。
//!   SpacesTest.FreeListManyCachedFastPathSelectFastAllocationFreeListCategoryType
//!     (spaces-unittest.cc:539-583) → 本档 fast_path_select_fallback_tiny_gold, 1:1 复刻
//!     (构造侧申报: 原用例建 FreeListManyCachedFastPath; select_fast 实体住基类 h:470-483
//!     → rs 以 FreeListManyCachedFastPathBase 承接同名方法, 语义同式)。
//! 其余面按票面口径: 原版 CHECK/DCHECK 断言语义自造等价覆盖 (categories_min 表/static_assert
//! h:466-468、双链手术 cc:500-540、Free/Pick/Search cc:36-110、Reset cc:456-464、
//! TryFindNodeIn cc:147-161、SumFreeList/FreeListLength cc:553-576、工厂 cc:139-146),
//! 节点/页运行态以 FreeListEnv 窄 trait 测试替身承接。

use std::{
  collections::HashMap,
  ptr::{eq, from_ref, null},
};

use aok::{OK, Void};
use log::info;
use v8_common::{Address, K_TAGGED_SIZE, WritableFreeSpace};
use v8_flags::Flags;
use v8_heap_core::{allocation::AllocationOrigin, free_list::*};
use v8_objects_core::tagged::Tagged;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ══════════════════════════════════════════════════════════════════════════
// 测试替身 — FreeListEnv 窄 trait 的最小运行态 (节点表: ptr → (size, next);
// 页/堆面在 cc:36-110/147-161/500-540/553-576 流程内不可达, 保留 unreachable;
// W10HCR2 扩展: cc:427-428 Allocate 尾部页记账面以哨兵句柄 + 记录向量承接,
// 覆盖 fast_path_allocate_* 两用例)
// ══════════════════════════════════════════════════════════════════════════

#[derive(Default)]
struct TestEnv {
  /// 节点表: 键=Tagged.ptr() (含强引用 01 tag), 值=(块尺寸, next 节点)
  nodes: HashMap<Address, (usize, Tagged<FreeSpace>)>,
  /// cc:428 IncreaseAllocatedBytes 观测记录: (页哨兵句柄位型, bytes)。
  /// 句柄 = 节点指针位型直转 (从不解引用, 仅相等/记账断言)。
  page_increases: Vec<(Address, usize)>,
}

impl TestEnv {
  /// 以强引用位型 (tag=01) 造节点句柄; address 取偶数 (页对齐位型)
  fn tagged_at(address: Address) -> Tagged<FreeSpace> {
    Tagged::new(address | 1)
  }

  /// 登记一个自由块 (size 与 Category::Free cc:98 DCHECK 同源)
  fn add_node(&mut self, address: Address, size: usize) {
    self
      .nodes
      .insert(Self::tagged_at(address).ptr(), (size, Tagged::default()));
  }
}

impl FreeListEnv for TestEnv {
  fn node_size(&self, node: Tagged<FreeSpace>) -> usize {
    self.nodes.get(&node.ptr()).map(|(s, _)| *s).unwrap_or(0)
  }
  fn node_next(&self, node: Tagged<FreeSpace>) -> Tagged<FreeSpace> {
    self
      .nodes
      .get(&node.ptr())
      .map(|(_, n)| *n)
      .unwrap_or_default()
  }
  fn node_set_next(
    &mut self,
    node: Tagged<FreeSpace>,
    _heap: *const Heap,
    _writable_free_space: &WritableFreeSpace,
    next: Tagged<FreeSpace>,
  ) {
    self.nodes.get_mut(&node.ptr()).unwrap().1 = next;
  }
  fn node_is_executable(&self, _node: Tagged<FreeSpace>) -> bool {
    false // cc:68 executable 分支在非码页链不可达; 本档链全走 ForNonExecutableMemory 通道
  }
  fn free_range_executable(&mut self, _address: Address, _size: usize) -> WritableFreeSpace {
    unreachable!("测试链无 executable 节点 (cc:71-74 面)")
  }
  fn page_from_address(&self, _isolate: *const Isolate, _address: Address) -> *mut NormalPage {
    unreachable!("本档不触 FreeList::Free 页记账面 (cc:184-195)")
  }
  fn page_from_heap_object(&self, node: Tagged<FreeSpace>) -> *mut NormalPage {
    // W10HCR2: cc:427 `NormalPage::FromHeapObject(node)` — 测试替身以节点指针位型
    // 直转哨兵句柄 (合成裸指针, 本档从不解引用, 仅记录/比较)。
    node.ptr() as *mut NormalPage
  }
  fn page_decrease_allocated_bytes(&mut self, _page: *mut NormalPage, _bytes: usize) {
    unreachable!("cc:189 面本档不触")
  }
  fn page_increase_allocated_bytes(&mut self, page: *mut NormalPage, bytes: usize) {
    // W10HCR2: cc:428/366 尾部页记账 — 记录句柄位型与字节数供断言。
    self.page_increases.push((page as Address, bytes));
  }
  fn page_add_wasted_memory(&mut self, _page: *mut NormalPage, _bytes: usize) {
    unreachable!("cc:193 面本档不触")
  }
  fn page_free_list_category(
    &mut self,
    _page: *mut NormalPage,
    _type_: FreeListCategoryType,
  ) -> *mut FreeListCategory {
    unreachable!("本档类目对象由测试栈直持, 不经页元数据取槽 (承载形申报见模块头)")
  }
  fn page_for_all_free_list_categories<F: FnMut(*mut FreeListCategory)>(
    &mut self,
    _page: *mut NormalPage,
    _callback: F,
  ) {
    unreachable!("本档不触 EvictFreeListItems (cc:485)")
  }
  fn isolate_heap(&self, _isolate: *const Isolate) -> *const Heap {
    null()
  }
}

/// 栈上类目 + 单节点入料: 造一个非空类目 (cc:93-102 头压首块, 不挂链)。
fn stocked_category(
  env: &mut TestEnv,
  cat: &mut FreeListCategory,
  type_: FreeListCategoryType,
  address: Address,
  size: usize,
  fl: &mut FreeListMany,
) {
  cat.initialize(type_);
  env.add_node(address, size);
  cat.free(
    env,
    null(),
    &WritableFreeSpace::for_non_executable_memory(address, size),
    FreeMode::DoNotLinkCategory,
    fl,
  );
  assert_eq!(size as u32, cat.available());
}

// ══════════════════════════════════════════════════════════════════════════
// 1:1 复刻 spaces-unittest.cc 两用例
// ══════════════════════════════════════════════════════════════════════════

/// spaces-unittest.cc:485-534 `SpacesTest.FreeListManySelectFreeListCategoryType` 1:1:
/// 256 字节内逐 size 全检 + 各类目界值 (±8 / 与次类中点) 抽检; EXPECT_* 非致命语义以
/// 失败计数收束 (0 失配即绿)。
#[test]
fn free_list_many_select_free_list_category_type() -> Void {
  let free_list = FreeListMany::new();
  let last = free_list.base().last_category();
  let mut failures = 0usize;

  // 原 489-500: size ∈ [0, kPreciseCategoryMaxSize] 全量逐字节
  for size in 0..=K_PRECISE_CATEGORY_MAX_SIZE {
    let cat = free_list.select_free_list_category_type(size);
    let ok = if cat == 0 {
      size < CATEGORIES_MIN[1] as usize // 原 493 EXPECT_LT(size, categories_min[1])
    } else {
      // 原 496-497: size 恰落 [min[cat], min[cat+1])
      CATEGORIES_MIN[cat as usize] as usize <= size
        && size < CATEGORIES_MIN[(cat + 1) as usize] as usize
    };
    failures += usize::from(!ok);
  }

  // 原 505-533: cat ∈ (kFirstCategory, last] 界值抽检
  for cat in (K_FIRST_CATEGORY + 1)..=last {
    let mut sizes = vec![
      CATEGORIES_MIN[cat as usize] as usize - 8,
      CATEGORIES_MIN[cat as usize] as usize,
      CATEGORIES_MIN[cat as usize] as usize + 8,
    ];
    if cat != last {
      // 原 517-519 与次类中点
      sizes.push(
        (CATEGORIES_MIN[cat as usize] as usize + CATEGORIES_MIN[(cat + 1) as usize] as usize) / 2,
      );
    }
    for size in sizes {
      let selected = free_list.select_free_list_category_type(size);
      let ok = if selected == last {
        CATEGORIES_MIN[selected as usize] as usize <= size // 原 524
      } else {
        CATEGORIES_MIN[selected as usize] as usize <= size
          && size < CATEGORIES_MIN[(selected + 1) as usize] as usize // 原 527-528
      };
      failures += usize::from(!ok);
    }
  }
  assert_eq!(0, failures, "select 界值失配数 (原版 EXPECT 收束口径)");
  info!("free_list_many_select_free_list_category_type ok");
  OK
}

/// spaces-unittest.cc:539-583 `FreeListManyCachedFastPathSelectFastAllocation...` 1:1 复刻。
/// 构造侧申报: 原用例 `FreeListManyCachedFastPath free_list;` (kAllow 路径), select_fast
/// 实体住 FastPathBase (h:470-483) → 此处直建 Base::new(Allow, 默认旗标), 语义同式。
#[test]
fn fast_path_select_fallback_tiny_gold() -> Void {
  let free_list = FreeListManyCachedFastPathBase::new(SmallBlocksMode::Allow, &Flags::default());
  let last = free_list.base().last_category();
  let mut failures = 0usize;

  // 原 543-557: cat ∈ [kFirstCategory, last] 界值集
  for cat in K_FIRST_CATEGORY..=last {
    let mut sizes = vec![
      CATEGORIES_MIN[cat as usize] as usize - 8,
      CATEGORIES_MIN[cat as usize] as usize,
      CATEGORIES_MIN[cat as usize] as usize + 8,
    ];
    if cat != last {
      sizes.push(
        (CATEGORIES_MIN[cat as usize] as usize + CATEGORIES_MIN[(cat + 1) as usize] as usize) / 2,
      );
    }
    for size in sizes {
      let selected = free_list.select_fast_allocation_free_list_category_type(size);
      let ok = if size <= K_TINY_OBJECT_MAX_SIZE {
        // 原 563-565: tiny → 快路径首类目 18
        selected == K_FAST_PATH_FIRST_CATEGORY
      } else if size >= CATEGORIES_MIN[last as usize] as usize - K_FAST_PATH_OFFSET {
        // 原 566-568: 近末类上界 → 末类目
        selected == last
      } else {
        // 原 572-578: 所选类目最小值 ≥ size+1.85k 且次小类目 < size+1.85k
        CATEGORIES_MIN[selected as usize] as usize >= size + K_FAST_PATH_OFFSET
          && (CATEGORIES_MIN[(selected - 1) as usize] as usize) < size + K_FAST_PATH_OFFSET
      };
      failures += usize::from(!ok);
    }
  }
  assert_eq!(0, failures, "select_fast 界值失配数 (原版 EXPECT 收束口径)");
  info!("fast_path_select_fallback_tiny_gold ok");
  OK
}

// ══════════════════════════════════════════════════════════════════════════
// 原版 CHECK/DCHECK 语义等价覆盖 (自造口径, 见模块头申报)
// ══════════════════════════════════════════════════════════════════════════

/// categories_min 表 24 值金标准 (h:328-330) + static_assert h:466-468 运行侧复证
/// (编译侧由 rs const 门承接, 本档双跑对账)。
#[test]
fn categories_min_table_and_static_assert_gold() -> Void {
  let expect = [
    24, 32, 48, 64, 80, 96, 112, 128, 144, 160, 176, 192, 208, 224, 240, 256, 512, 1024, 2048,
    4096, 8192, 16384, 32768, 65536,
  ];
  assert_eq!(24, expect.len());
  assert_eq!(24, K_NUMBER_OF_CATEGORIES);
  for (i, v) in expect.iter().enumerate() {
    assert_eq!(*v, CATEGORIES_MIN[i], "categories_min[{i}] (h:328-330)");
  }
  // h:466 static_assert(categories_min[kFastPathFirstCategory] == kFastPathStart)
  assert_eq!(18, K_FAST_PATH_FIRST_CATEGORY);
  assert_eq!(2048, K_FAST_PATH_START);
  assert_eq!(CATEGORIES_MIN[18] as usize, K_FAST_PATH_START);
  // h:467-468 static_assert(categories_min[kFastPathFallBackTiny] == kTinyObjectMaxSize * 2)
  assert_eq!(15, K_FAST_PATH_FALL_BACK_TINY);
  assert_eq!(128, K_TINY_OBJECT_MAX_SIZE);
  assert_eq!(CATEGORIES_MIN[15] as usize, K_TINY_OBJECT_MAX_SIZE * 2);
  // h:462 kFastPathOffset = 2048-128 = 1920 (1.85k 超配, h:420-431 类注释)
  assert_eq!(1920, K_FAST_PATH_OFFSET);
  assert_eq!(
    K_FAST_PATH_START - K_TINY_OBJECT_MAX_SIZE,
    K_FAST_PATH_OFFSET
  );
  // h:310 kMinBlockSize = 3*kTaggedSize (符号式对照, 值随压缩配置)
  assert_eq!(3 * K_TAGGED_SIZE, K_MIN_BLOCK_SIZE);
  // h:317 kPreciseCategoryMaxSize = 256
  assert_eq!(256, K_PRECISE_CATEGORY_MAX_SIZE);
  // h:314 kMaxBlockSize = kRegularPageSize
  assert_eq!(v8_common::K_REGULAR_PAGE_SIZE, K_MAX_BLOCK_SIZE);
  // h:38-39 哨兵
  assert_eq!(0, K_FIRST_CATEGORY);
  assert_eq!(-1, K_INVALID_CATEGORY);
  info!("categories_min_table_and_static_assert_gold ok");
  OK
}

/// 双链手术金标准 (cc:500-517 AddCategory 空拒/头插/入账 + cc:519-540 RemoveCategory
/// 挂链扣账/摘链/清 prev-next; DCHECK_NE cc:506 态由两次连续同槽 add 的链头让位体现)。
#[test]
fn add_remove_category_link_surgery_gold() -> Void {
  let mut env = TestEnv::default();
  let mut fl = FreeListMany::new();
  let mut c0 = FreeListCategory::default();
  c0.initialize(5);

  // cc:505: 空类目 (top null) 拒绝入链, 槽不动
  assert!(!fl.add_category(&mut c0));
  assert!(fl.base().top(5).is_null());

  // 入料后头插 c0 (cc:508-516): 槽指 c0, owner.available += c0.available() (cc:514)
  env.add_node(0x2000, 64);
  c0.free(
    &mut env,
    null(),
    &WritableFreeSpace::for_non_executable_memory(0x2000, 64),
    FreeMode::DoNotLinkCategory,
    &mut fl,
  );
  assert_eq!(64, c0.available());
  assert!(fl.add_category(&mut c0));
  assert!(eq(from_ref(&c0), fl.base().top(5)));
  assert_eq!(64, fl.base().available());

  // 头插 c1 (链上 c0): c1.next=c0, c0.prev=c1, 槽=c1 (cc:508-513 双链)
  let mut c1 = FreeListCategory::default();
  stocked_category(&mut env, &mut c1, 5, 0x2100, 32, &mut fl);
  assert!(fl.add_category(&mut c1));
  assert!(eq(from_ref(&c1), fl.base().top(5)));
  assert!(eq(from_ref(&c0), c1.next()));
  assert!(eq(from_ref(&c1), c0.prev()));
  assert_eq!(96, fl.base().available());

  // cc:519-540 摘 c1: 挂链态回扣 available; 头让位 c0; prev/next 清空
  fl.remove_category(&mut c1);
  assert!(eq(from_ref(&c0), fl.base().top(5)));
  assert_eq!(64, fl.base().available());
  assert!(c1.prev().is_null() && c1.next().is_null());
  assert!(!c1.is_linked(&fl)); // 槽已让位 + prev/next 空 (inl:17-20)
  info!("add_remove_category_link_surgery_gold ok");
  OK
}

/// 节点链面金标准: Category::Free 头压挂链/记账 (cc:93-110)、PickNodeFromList 单步
/// (cc:36-50)、SearchForNodeInList 中摘保持前驱链 (cc:52-91)、TryFindNodeIn 空类目
/// 自动摘链 (cc:147-161)、SumFreeList/FreeListLength/IterateNodesForTesting (cc:553-576/h:85)。
#[test]
fn node_chain_free_pick_search_gold() -> Void {
  let mut env = TestEnv::default();
  let mut fl = FreeListMany::new();
  let mut cat = FreeListCategory::default();
  cat.initialize(3);

  // 三节点 A(0x3000,64) B(0x3080,16) C(0x30C0,48): 逆序 Free → 链 A→B→C
  for (addr, size) in [(0x3000usize, 64usize), (0x3080, 16), (0x30C0, 48)] {
    env.add_node(addr, size);
  }
  for (addr, size) in [(0x30C0usize, 48usize), (0x3080, 16), (0x3000, 64)] {
    cat.free(
      &mut env,
      null(),
      &WritableFreeSpace::for_non_executable_memory(addr, size),
      FreeMode::LinkCategory,
      &mut fl,
    );
  }
  // cc:107: 首块 Free 即触发 add_category (此前未挂链)
  assert!(eq(from_ref(&cat), fl.base().top(3)));
  // cc:104-105: 挂链后各块走 increase_available → owner 累加
  assert_eq!(128, fl.base().available());
  assert!(TestEnv::tagged_at(0x3000) == cat.top());
  assert_eq!(128, cat.available());
  // cc:99 SetNext(old_top): A.next=B, B.next=C, C.next=∅
  assert!(TestEnv::tagged_at(0x3080) == env.node_next(TestEnv::tagged_at(0x3000)));
  assert!(TestEnv::tagged_at(0x30C0) == env.node_next(TestEnv::tagged_at(0x3080)));
  assert!(env.node_next(TestEnv::tagged_at(0x30C0)).is_null());
  assert_eq!(128, cat.sum_free_list(&env)); // cc:553-567 走链尺寸和
  assert_eq!(3, cat.free_list_length(&env)); // cc:568-576 节点数

  // cc:36-50 Pick: 顶够大→弹出; 不够大→node_size=0 返回空, 链不动 (cc:43-44)
  let mut node_size = 0usize;
  let n = cat.pick_node_from_list(&mut env, 40, &mut node_size);
  assert!(TestEnv::tagged_at(0x3000) == n);
  assert_eq!(64, node_size); // cc:47
  assert_eq!(64, cat.available()); // cc:48 128-64
  assert!(TestEnv::tagged_at(0x3080) == cat.top());
  let before_top = cat.top();
  node_size = 7;
  let n = cat.pick_node_from_list(&mut env, 40, &mut node_size);
  assert!(n.is_null()); // B(16) < 40
  assert_eq!(0, node_size);
  assert!(before_top == cat.top());

  // cc:52-91 Search: 走链首个够大 = C(48); B 改链越过 C (cc:67-83 前驱保持)
  let mut n = cat.search_for_node_in_list(&mut env, null(), 40, &mut node_size);
  assert!(TestEnv::tagged_at(0x30C0) == n);
  assert_eq!(48, node_size);
  assert_eq!(16, cat.available()); // cc:63 扣减 64-48
  assert!(env.node_next(TestEnv::tagged_at(0x3080)).is_null()); // cc:75 B.next = C.next
  assert!(cat.is_linked(&fl));

  // h:85-91 IterateNodesForTesting: 现链 = B 单节点
  let mut seen = Vec::new();
  cat.iterate_nodes_for_testing(&env, |node| seen.push(node.ptr()));
  assert_eq!(vec![0x3081usize], seen);

  // cc:147-161 TryFindNodeIn: 弹出 B → 类目空 → 自动 RemoveCategory 摘链; owner 扣账
  n = fl.try_find_node_in(&mut env, 3, 16, &mut node_size);
  assert!(TestEnv::tagged_at(0x3080) == n);
  assert_eq!(16, node_size);
  assert!(fl.base().top(3).is_null()); // cc:157-158
  assert_eq!(112, fl.base().available()); // 128 - 16 (cc:153)

  // cc:150: 空槽 try → 直接空返回
  let mut ns = 9usize;
  assert!(fl.try_find_node_in(&mut env, 3, 1, &mut ns).is_null());
  assert_eq!(9, ns); // 未触碰
  info!("node_chain_free_pick_search_gold ok");
  OK
}

/// Reset 族金标准 (cc:456-464 FreeList::Reset: 逐类目 Reset(cc:30-34) + 全槽清 +
/// wasted/available 归零 (cc:462-463))。
#[test]
fn reset_clears_all_categories_gold() -> Void {
  let mut env = TestEnv::default();
  let mut fl = FreeListMany::new();
  let mut c0 = FreeListCategory::default();
  stocked_category(&mut env, &mut c0, 2, 0x4000, 48, &mut fl);
  let mut c1 = FreeListCategory::default();
  stocked_category(&mut env, &mut c1, 2, 0x4100, 32, &mut fl);
  assert!(fl.add_category(&mut c0));
  assert!(fl.add_category(&mut c1));
  assert_eq!(80, fl.base().available());
  fl.increase_wasted_bytes(8); // h:183 (cc:462 清零对象)

  fl.reset(); // cc:456-464
  assert_eq!(0, fl.base().available());
  assert_eq!(0, fl.base().wasted_bytes());
  assert!(fl.base().top(2).is_null());
  assert!(c0.top().is_null() && c0.available() == 0); // cc:32-33
  assert!(c1.top().is_null() && c1.available() == 0);
  assert!(!c0.is_linked(&fl) && !c1.is_linked(&fl));
  info!("reset_clears_all_categories_gold ok");
  OK
}

/// Unlink/Relink 记账金标准 (cc:22-28 Unlink: 挂链且顶非空才回扣 owner 账, 清 prev/next;
/// cc:126-129 Relink: DCHECK(!is_linked) 前置 + AddCategory 重挂)。
#[test]
fn unlink_relink_accounting_gold() -> Void {
  let mut env = TestEnv::default();
  let mut fl = FreeListMany::new();

  // Unlink 面: 挂链态回扣账; 槽仍指本类目 (原版 cc:22-28 不动槽, 槽清归 cc:459-461 数组路径)
  let mut cat = FreeListCategory::default();
  stocked_category(&mut env, &mut cat, 7, 0x5000, 40, &mut fl);
  assert!(fl.add_category(&mut cat));
  assert_eq!(40, fl.base().available());
  cat.unlink(&mut fl);
  assert_eq!(0, fl.base().available());
  assert!(cat.prev().is_null() && cat.next().is_null());

  // Relink 面: 全新未挂链类目 (type 9 空槽)。先走未挂链 remove: is_linked false →
  // 不扣账不挪链 (cc:523-531 双 false 分支)
  let mut cat2 = FreeListCategory::default();
  stocked_category(&mut env, &mut cat2, 9, 0x5100, 24, &mut fl);
  assert!(!cat2.is_linked(&fl)); // prev/next 空 + 槽空 (inl:17-20 三条件全 false)
  fl.remove_category(&mut cat2);
  assert_eq!(0, fl.base().available());
  assert!(fl.base().top(9).is_null());
  // cc:126-129 Relink: DCHECK(!is_linked) 前置 + AddCategory 重挂入账 (cc:514)
  cat2.relink(&mut fl);
  assert!(eq(from_ref(&cat2), fl.base().top(9)));
  assert_eq!(24, fl.base().available());
  // 挂链态 remove: 回扣账 + 头槽让位于 next (cc:524-531)
  fl.remove_category(&mut cat2);
  assert_eq!(0, fl.base().available());
  assert!(fl.base().top(9).is_null());
  info!("unlink_relink_accounting_gold ok");
  OK
}

/// 类目初值/Initialize 金标准 (h:110/114 字段初值 + h:46-51 Initialize 置型清账不动 top_)。
#[test]
fn category_init_and_sentinels_gold() -> Void {
  let cat = FreeListCategory::default();
  assert_eq!(K_INVALID_CATEGORY, cat.type_()); // h:110 初值
  assert_eq!(0, cat.available()); // h:114
  assert!(cat.top().is_null() && cat.is_empty()); // h:79
  assert!(cat.prev().is_null() && cat.next().is_null()); // h:119-120
  let mut cat = cat;
  cat.set_top(TestEnv::tagged_at(0x6000));
  cat.initialize(9);
  assert_eq!(9, cat.type_());
  assert_eq!(0, cat.available());
  assert!(!cat.top().is_null()); // h:46-51 实测口径: Initialize 不清 top_ (实体 h:46-51 四行)

  // FreeListBase ctor (cc:134-137): last = n-1; categories 未分配态; available/wasted 零
  let base = FreeListBase::new(24, 12);
  assert_eq!(23, base.last_category());
  assert_eq!(24, base.number_of_categories());
  assert_eq!(12, base.min_block_size());
  assert_eq!(0, base.available());
  assert_eq!(0, base.wasted_bytes());
  // wasted 原子面 (h:183-188 relaxed 加减)
  let mut base = base;
  base.increase_available_bytes(16);
  base.decrease_available_bytes(4);
  assert_eq!(12, base.available());
  info!("category_init_and_sentinels_gold ok");
  OK
}

/// 工厂面金标准 (cc:139/143 CreateFreeList/ForNewSpace + h:445-450 kProhibit 最小块
/// 抬升, v8_flags 参数注入申报面)。
#[test]
fn factory_and_prohibit_mode_gold() -> Void {
  let list = create_free_list(); // cc:139-141 → FreeListManyCachedOrigin (kAllow 链)
  assert_eq!(23, list.base().last_category());
  assert_eq!(3 * K_TAGGED_SIZE, list.base().min_block_size());

  // ForNewSpace (cc:143-146) kProhibit: minor_ms 默认关 → kFastPathStart (h:448-450)
  let flags_off = Flags::default();
  assert!(!flags_off.minor_ms());
  let list = create_free_list_for_new_space(&flags_off);
  assert_eq!(K_FAST_PATH_START, list.base().min_block_size());

  // minor_ms 开且 min_lab_size_kb>0 → 取 KB 值 (h:447 分支)
  let mut flags_on = Flags::default();
  flags_on.set_minor_ms(true).unwrap();
  flags_on.set_minor_ms_min_lab_size_kb(8).unwrap();
  let list = create_free_list_for_new_space(&flags_on);
  assert_eq!(8 * v8_common::KB, list.base().min_block_size());

  // kAllow 对照: 不抬升
  let allow = FreeListManyCachedFastPathBase::new(SmallBlocksMode::Allow, &flags_on);
  assert_eq!(3 * K_TAGGED_SIZE, allow.base().min_block_size());
  info!("factory_and_prohibit_mode_gold ok");
  OK
}

/// v8: free-list.cc:375-436 — W10HCR2 面2 金标准: FastPathBase::Allocate 四段编排的
/// **part2 tiny 回退命中**面 (cc:392-406)。原版 part2 循环复用**共享变量 type**
/// (cc:397 `for (type = next_nonempty_category[kFastPathFallBackTiny]; ...)`), 命中后
/// 尾部 cc:425-426 `categories_[type]`/`UpdateCacheAfterRemoval(type)` 读到 part2 类目。
/// 缺陷回归锚: 先行实现以独立局部 `let mut t` 承接 → type_ 滞留 part1 退出哨兵 24 →
/// 修复前本例须在 top(24) 处越界 panic (categories_ 表长 24); 修复后三断言全过。
#[test]
fn fast_path_allocate_tiny_fallback_part2_gold() -> Void {
  let mut env = TestEnv::default();
  let mut owner = FreeListMany::new(); // 入料宿主 (Category::Free 形参具体型, DoNotLink 不挂本链)
  let mut fl = FreeListManyCachedFastPathBase::new(SmallBlocksMode::Allow, &Flags::default());

  // 入料: 类目 16 ((256,512] 档) 单节点 300 字节; add_category 走 cached 覆写
  // (cc:272-285) → UpdateCacheAfterAddition(16): cache[0..=16]=16, cache[17..=24]=24 哨兵。
  let mut c16 = FreeListCategory::default();
  stocked_category(&mut env, &mut c16, 16, 0x7000, 300, &mut owner);
  assert!(fl.add_category(&mut c16));
  assert_eq!(300, fl.base().available());

  // 请求 64 字节 (tiny ≤ kTinyObjectMaxSize=128):
  // part1 (cc:382-390): first_category = select_fast(64) = 18, cache[18] = 24 > last(23)
  //   → 循环零轮退出, node 空, type_ 残值 24;
  // part2 (cc:392-406): kAllow + node 空 + 64 ≤ 128 → type_ = cache[15] = 16 < 18 →
  //   TryFindNodeIn(16, 64) 命中 → 弹出 (类目空自动摘链扣账 cc:147-161) → break。
  let mut node_size = 0usize;
  let node = fl.allocate(
    &mut env,
    null(),
    64,
    &mut node_size,
    AllocationOrigin::Runtime,
  );
  assert!(
    TestEnv::tagged_at(0x7000) == node,
    "cc:400 part2 命中须返回入料节点"
  );
  assert_eq!(300, node_size, "cc:400 弹出块尺寸");
  assert_eq!(
    0,
    fl.base().available(),
    "cc:153 类目空自动摘链后 owner 扣账归零"
  );
  assert!(
    c16.top().is_null() && c16.available() == 0,
    "cc:104-108 弹出后类目清空"
  );
  // 尾部 (cc:425-428): type_ 共享变量 = 16 (part2 类目, 非残留哨兵 24 → 修复前
  // categories_[24] 越界) → top(16) 空 → UpdateCacheAfterRemoval(16) 幂等再写 ≡ 原版
  // 双重回写语义 (try_find_node_in 内摘链已写一次) → 页记账一次。
  assert_eq!(
    1,
    env.page_increases.len(),
    "cc:427-428 IncreaseAllocatedBytes 一次"
  );
  assert_eq!((0x7001, 300), env.page_increases[0], "页哨兵句柄与记账字节");
  info!("fast_path_allocate_tiny_fallback_part2_gold ok");
  OK
}

/// v8: free-list.cc:375-436 对照面 — **part1 直接命中** (cc:382-390) 全缓存跳跃链路:
/// 请求 6000 → first=select_fast=20, cache[20]=21 (类目 21 入料回写) → TryFindNodeIn(21)
/// 命中即 break, part2/末段不进入; 尾部 type_=21 页记账。修复前后同形 (回归护栏:
/// 证明 part2 修复未扰动 part1 路径)。
#[test]
fn fast_path_allocate_part1_direct_hit_gold() -> Void {
  let mut env = TestEnv::default();
  let mut owner = FreeListMany::new();
  let mut fl = FreeListManyCachedFastPathBase::new(SmallBlocksMode::Allow, &Flags::default());

  // 入料: 9000 字节 → select_free_list_category_type(9000): (8192,16384] → 类目 21。
  let mut c21 = FreeListCategory::default();
  stocked_category(&mut env, &mut c21, 21, 0x8000, 9000, &mut owner);
  assert!(fl.add_category(&mut c21)); // cache[0..=21]=21, cache[22..=24]=24
  assert_eq!(9000, fl.base().available());

  // 请求 6000: part1 first = select_fast(6000): 6000+1920=7920 ≤ min[20]=8192 → 20;
  // type_ = cache[20] = 21 ≤ 23 → TryFindNodeIn(21, 6000): 9000 顶块 ≥ 6000 → 命中。
  let mut node_size = 0usize;
  let node = fl.allocate(
    &mut env,
    null(),
    6000,
    &mut node_size,
    AllocationOrigin::Runtime,
  );
  assert!(TestEnv::tagged_at(0x8000) == node, "cc:388 part1 命中节点");
  assert_eq!(9000, node_size);
  assert_eq!(0, fl.base().available());
  assert_eq!(1, env.page_increases.len());
  assert_eq!((0x8001, 9000), env.page_increases[0]);
  info!("fast_path_allocate_part1_direct_hit_gold ok");
  OK
}
