//! 审查环 round1 补强用例 (对抗审查, 不改动 src/)
//!
//! 断言规约先行 (todo.md 7.3), 每条对应原版 zone.h/cc、zone-containers.h、
//! zone-list-inl.h 的既有行为; 红用例 = 已证实缺陷, 详见 readme/zh.md
//! "审查环 round1" 小节。

use std::sync::atomic::{AtomicUsize, Ordering};

use aok::{OK, Void};
use v8_zone::{AccountingAllocator, Error, Zone, ZoneList, ZoneScope, ZoneVector};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 zone.h:120 CHECK_LT(length, size_max/sizeof(T)) → Rust Err(SizeOverflow)
#[test]
fn zone_test_allocate_array_length_overflow_guard() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let r = zone.allocate_array::<u64>(usize::MAX / 4 + 1);
  assert!(
    matches!(r, Err(Error::SizeOverflow)),
    "溢出必须映射 Err, 不得静默小分配"
  );
  OK
}

/// 原版 zone.cc:143-199 高水位扩段公式: 首段夹到 kMinimumSegmentSize=8K;
/// 请求 20K 时 nwso=20480+2*8192=36864 >= 32K → new = max(32K, SE+20480)=32K
/// (Rust 段为精确大小, 原版 total_size >= 请求, 公式口径一致)
#[test]
fn zone_test_expand_growth_formula_exact() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let _a = zone.allocate_bytes_ref(8)?;
  assert_eq!(
    zone.segment_bytes_allocated(),
    8 * 1024,
    "kMinimumSegmentSize clamp"
  );
  let _b = zone.allocate_bytes_ref(20 * 1024)?;
  assert_eq!(
    zone.segment_bytes_allocated(),
    8 * 1024 + 32 * 1024,
    "kMaximumSegmentSize clamp 高水位公式"
  );
  OK
}

/// 原版非 ASan 构建: Seal 仅置位 (zone.h:166), CHECK(!sealed_) 只在 AsanNew
/// (zone.cc:43)。外部行为 1:1 → seal 后分配仍必须可用 (本 crate 无 ASan 路径)
#[test]
fn zone_test_seal_does_not_block_allocation_non_asan() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  zone.seal();
  assert!(zone.is_sealed());
  let _p = zone.allocate_bytes_ref(8)?;
  OK
}

/// 原版 zone.h:66-69: 已有段时 Allocate(0) 返回 zone 内 position_ 真址
/// (0 不大于余量, 走 AllocateUnchecked), 不是段外哨兵。零长分配的返回值应
/// 满足 Contains() (zone.cc:96-105)。
#[test]
fn zone_test_allocate_zero_bytes_returns_in_zone_position() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let _a = zone.allocate_bytes_ref(8)?;
  let p = zone.allocate_bytes_ref(0)?.as_ptr();
  assert!(
    zone.contains(p as *const ()),
    "原版: 零长分配返回 zone 内 position, 非段外哨兵"
  );
  OK
}

/// 原版 ZoneSnapshot::Restore (zone.cc:219-229) 传错 zone 时 CHECK_NOT_NULL
/// 终止进程, 不存在"Err 返回后 zone 继续运行"的形态。Rust 收紧为 Err 后,
/// 错误路径不得留下已归还段但仍挂在 segment_head 上的悬垂链 (原子性契约)。
/// D3 修复验证: Err 前零段归还; 修复后 zone2 正常 drop 不再需要 forget。
#[test]
fn zone_test_restore_wrong_zone_error_must_not_release_segments() -> Void {
  let allocator = AccountingAllocator::new();
  let zone1 = Zone::new(&allocator, "z1");
  let mut zone2 = Zone::new(&allocator, "z2");
  let snapshot = {
    let _a = zone1.allocate_bytes_ref(64)?;
    zone1.snapshot()
  };
  let usage_before = {
    let _b = zone2.allocate_bytes_ref(64)?;
    allocator.current_memory_usage()
  };
  let result = snapshot.restore(&mut zone2);
  assert!(matches!(result, Err(Error::WrongZone)));
  assert_eq!(
    allocator.current_memory_usage(),
    usage_before,
    "缺陷: Err 返回前已把 zone2 的段归还 allocator, 而 zone2.segment_head 仍指向已释放段 (后续 drop → 双重释放)"
  );
  OK
}

/// 缺陷 H1 (Soundness) 修复验证: ZoneScope 现持 &mut Zone, 作用域内经
/// scope.zone() 交出的借用生命周期被约束在作用域内; "回滚后同地址再交
/// &'z mut 与仍活借用别名" 的旧缺陷形态在类型层面不可表达 (编译期拒绝,
/// ZoneScope::new(&mut zone) 与存活借用互斥)。
/// 运行时面: 回滚精确到快照 position, 作用域前分配不受影响;
/// 作用域内地址按原版 ZoneScope 语义复用 (借用已亡, 无别名, 安全)。
#[test]
fn zone_scope_restore_must_not_reissue_address_while_mut_borrow_live() -> Void {
  let allocator = AccountingAllocator::new();
  let mut zone = Zone::new(&allocator, "test");
  let base_addr = {
    let base = zone.allocate_bytes_ref(8)?;
    base.as_mut_ptr() as usize
  };
  let inside_addr = {
    let mut scope = ZoneScope::new(&mut zone);
    let tmp = scope.zone().allocate_bytes_ref(64)?;
    tmp.as_mut_ptr() as usize
  }; // tmp 借用与作用域同尽 → 回滚后复用不再别名
  assert_ne!(base_addr, inside_addr);
  let after = zone.allocate_bytes_ref(64)?;
  assert_eq!(
    inside_addr,
    after.as_mut_ptr() as usize,
    "回滚后地址复用恢复原版 ZoneScope 语义 (作用域内借用已亡, 不构成别名)"
  );
  assert_ne!(base_addr, after.as_mut_ptr() as usize);
  OK
}

/// 缺陷 H1 第二路径修复验证: reset() 收紧 &mut self (zone.cc:69-93),
/// 保留段起点复用与借出 &'z mut 引用在类型层面互斥 (编译期拒绝)。
/// 运行时面: reset 回卷 position 到保留段起点 (原版 zone.cc:89 语义),
/// 后续分配复用段头地址 (借用已亡, 无别名, 安全)。
#[test]
fn zone_reset_must_not_reissue_address_while_mut_borrow_live() -> Void {
  let allocator = AccountingAllocator::new();
  let mut zone = Zone::new(&allocator, "test");
  let a_addr = {
    let a = zone.make_array_with::<u32>(4, &1)?;
    let _tmp = zone.make_array_with::<u32>(4, &2)?;
    a.as_mut_ptr() as usize
  }; // a/_tmp 借用随块结束
  zone.reset();
  let b = zone.make_array_with::<u32>(4, &3)?;
  assert_eq!(
    a_addr,
    b.as_mut_ptr() as usize,
    "reset 回卷保留段起点, 首地址复用为原版语义 (借用已亡, 不构成别名)"
  );
  OK
}

/// 带 Drop 的跟踪元素: 记录构造与析构次数 (对应原版 LiveSet 思想,
/// zone-vector-unittest.cc:12-43)
static CREATED: AtomicUsize = AtomicUsize::new(0);
static DROPPED: AtomicUsize = AtomicUsize::new(0);

struct Tracked(u32);

impl Tracked {
  fn new(id: u32) -> Tracked {
    CREATED.fetch_add(1, Ordering::Relaxed);
    Tracked(id)
  }
}

impl Clone for Tracked {
  fn clone(&self) -> Tracked {
    CREATED.fetch_add(1, Ordering::Relaxed);
    Tracked(self.0)
  }
}

impl Drop for Tracked {
  fn drop(&mut self) {
    DROPPED.fetch_add(1, Ordering::Relaxed);
  }
}

/// 原版 ZoneList::InsertAt (zone-list-inl.h:97-104): Add 尾插 + 前移走
/// 赋值, 全程不调用任何元素的析构。Rust 版右移链对被 move 空的槽位再
/// drop_in_place → 同值二次析构 (double drop, UB)。
/// 规约: insert_at 之后、列表元素销毁之前, DROPPED 增量必须为 0。
/// D2 修复验证: 右移改为 ptr::copy (memmove) + ptr::write, 零析构。
#[test]
fn zone_list_insert_at_must_not_double_drop_moved_elements() -> Void {
  let created0 = CREATED.load(Ordering::Relaxed);
  let dropped0 = DROPPED.load(Ordering::Relaxed);
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut list: ZoneList<'_, '_, Tracked> = ZoneList::new(4, &zone)?;
  let a = Tracked::new(1);
  let b = Tracked::new(2);
  list.add(&a)?;
  list.add(&b)?;
  let e = Tracked::new(3);
  list.insert_at(0, &e)?;
  // 原版此刻不应有任何析构 (插入不删除元素, 临时量 e/a/b 仍活)
  let extra_drops = DROPPED.load(Ordering::Relaxed) - dropped0;
  assert_eq!(
    extra_drops, 0,
    "缺陷: insert_at 对被移空的槽位二次 drop_in_place (C++ 仅赋值, 不析构)"
  );
  assert_eq!(list.length(), 3);
  assert_eq!(list.at(0).0, 3);
  assert_eq!(list.at(1).0, 1);
  assert_eq!(list.at(2).0, 2);
  // 收尾: rewind 析构全部活元素, 总析构 == 总构造 - 仍活外部值 (a,b,e)
  list.rewind(0);
  let created = CREATED.load(Ordering::Relaxed) - created0;
  let dropped = DROPPED.load(Ordering::Relaxed) - dropped0;
  assert_eq!(
    dropped,
    created - 3,
    "a/b/e 本体仍在作用域内, 不应被计入析构"
  );
  OK
}

/// 原版 ZoneVector 拷贝构造 (zone-containers.h:116-118 → operator=
/// zone-containers.h:153-156): 新存储取 other.capacity(), 复制后容量保持。
/// 规约: clone().capacity() == 源.capacity()。
#[test]
fn zone_vector_copy_ctor_must_preserve_capacity() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut v: ZoneVector<'_, '_, u32> = ZoneVector::new(&zone);
  for i in 1..=3u32 {
    v.push_back(i)?;
  }
  assert_eq!(v.capacity(), 4, "高水位倍增: 1→2→4 (zone-containers.h:410)");
  let c = v.clone();
  assert_eq!(
    c.capacity(),
    v.capacity(),
    "缺陷: Clone 以 size 精确分配, 原版以 other.capacity() 分配"
  );
  OK
}

/// 原版填充构造 ZoneVector(size, def, zone) (zone-containers.h:79-83)
/// 直接 AllocateArray(size): capacity 恰为 size。Rust with_value 走
/// assign→grow, size=1 时被 kMinCapacity=2 抬到 2 → 容量口径漂移。
#[test]
fn zone_vector_fill_ctor_capacity_must_be_exact() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let v = ZoneVector::with_value(&zone, 1, &7u32)?;
  assert_eq!(v.size(), 1);
  assert_eq!(v.capacity(), 1, "原版 ctor 分配恰 size, 不经倍增下限");
  OK
}
