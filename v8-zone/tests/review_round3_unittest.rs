//! 审查环 round3 补强用例 (第 3 轮对抗式审查)
//!
//! 针对第 3 轮审查发现的核心死角设计验证用例:
//! 1. 多线程与并发安全 (Zone / ZoneSnapshot 具有 Send, 严格禁止 Sync, 容器及分配器保持 !Send + !Sync)
//! 2. Zone::delete 极值防御 (u8 搭配 usize::MAX - 3 防范 align_up 整数加法溢出 panic)
//! 3. Zone::reset 多段复位 (多段状态下 reset 保证 position_base 与 segment_head 正确同步)
//! 4. AccountingAllocator 析构契约 (零内存泄露析构断言守恒)
//! 5. ZoneAllocationPolicy 特性 (Debug / PartialEq / Eq 等价语义)

use std::thread;

use aok::{OK, Void};
use fastrand::Rng;
use v8_zone::{
  AccountingAllocator, Zone, ZoneAllocationPolicy, ZoneAllocator, ZoneList, ZoneSnapshot,
  ZoneVector,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

fn assert_send<T: Send>() {}

/// 缺陷 1 验证: Zone 与 ZoneSnapshot 实现了 Send, 允许安全转移到后台线程执行分配
#[test]
fn zone_thread_send_transfer_and_allocate() -> Void {
  assert_send::<Zone<'_>>();
  assert_send::<ZoneSnapshot>();

  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "thread_test");

  // 转移 Zone 所有权到线程并在其中分配
  thread::scope(|s| {
    let handle = s.spawn(move || -> usize {
      let p = zone.allocate_bytes_ref(256).expect("allocate on worker");
      assert_eq!(p.len(), 256);
      zone.allocation_size()
    });

    let alloc_size = handle.join().expect("thread join");
    assert_eq!(alloc_size, 256);
  });
  // 线程结束后分配的段归还 (allocator 是 Sync)
  assert_eq!(allocator.current_memory_usage(), 0);
  OK
}

/// 缺陷 1 伴生契约验证: Zone 绝不能为 Sync, 且借用 Zone 的容器及分配器绝不能为 Send / Sync
#[test]
fn zone_concurrency_negative_bounds() -> Void {
  // 编译期静态锁 (v8-common assert_scope 先例): PhantomData 透传 Send/Sync 性质,
  // Zone 含 Cell 内部可变非线程安全 → !Sync; &'z Zone 为 !Send → 容器/分配器 !Send + !Sync
  static_assertions::assert_not_impl_all!(Zone<'_>: Sync);
  static_assertions::assert_not_impl_all!(ZoneAllocator<'_, '_, i32>: Send, Sync);
  static_assertions::assert_not_impl_all!(ZoneAllocationPolicy<'_, '_>: Send, Sync);
  OK
}

/// 缺陷 2 验证: Zone::delete 传入 count = usize::MAX - 3, T = u8
/// 必须被 MAX_NEW_SIZE 拦截, 绝不能在 debug 构建下触发 (raw_size + 7) 加法溢出 panic.
#[test]
fn zone_delete_u8_near_max_usize_no_panic() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test_delete_u8_overflow");
  let p = zone.allocate_bytes_ref(64)?;
  let ptr = p.as_mut_ptr().cast::<u8>();

  // 极端尺寸: checked_mul(1) 成功返回 usize::MAX - 3,
  // 随后 align_up 必须安全被 MAX_NEW_SIZE 拦截, 禁止 panic
  let policy = ZoneAllocationPolicy::new(&zone);
  policy.delete_array(ptr, usize::MAX - 3);
  policy.delete_array(ptr, usize::MAX - 7);
  policy.delete_array(ptr, usize::MAX - 1);

  // 正常归还 64 字节
  policy.delete_array(ptr, 64);
  OK
}

/// 缺陷 3 验证: 多段场景下 Zone::reset() 正确复位 position_base
/// 并彻底归还历史段, 最终回到 1 个段且 allocation_size 为 0.
/// (D4: reset 需独占, MaybeUninit 借用在块结束时先行释放)
#[test]
fn zone_reset_multi_segment_position_base_clean() -> Void {
  let allocator = AccountingAllocator::new();
  let mut zone = Zone::new(&allocator, "test_reset_multi");

  // 强制分配多个段 (8K -> 16K -> 32K ...)
  let seg_bytes_before = {
    let _seg1 = zone.allocate_bytes_ref(4000)?;
    let _seg2 = zone.allocate_bytes_ref(12000)?;
    let _seg3 = zone.allocate_bytes_ref(28000)?;
    let before = zone.segment_bytes_allocated();
    assert!(before >= 8 * 1024 + 16 * 1024, "必须分配了至少 2 个段");
    before
  };

  // 执行 reset
  zone.reset();

  // reset 后应当只保留最后一个段 (即分配 seg3 时的段)
  assert_eq!(
    zone.allocation_size(),
    0,
    "reset 后 allocation_size 必须为 0"
  );
  let kept_bytes = zone.segment_bytes_allocated();
  assert!(kept_bytes > 0 && kept_bytes < seg_bytes_before);

  // 在保留段上继续分配
  let _next = zone.allocate_bytes_ref(128)?;
  assert_eq!(zone.allocation_size(), 128);
  OK
}

/// 缺陷 4 验证: AccountingAllocator 零用量析构不 panic, 契约守恒
#[test]
fn accounting_allocator_clean_drop_contract() -> Void {
  let allocator = AccountingAllocator::new();
  {
    let zone = Zone::new(&allocator, "temp_zone");
    let _data = zone.allocate_bytes_ref(1024)?;
    assert!(allocator.current_memory_usage() > 0);
  }
  // zone drop 后 current_memory_usage 必须归 0
  assert_eq!(allocator.current_memory_usage(), 0);
  // allocator 在此 drop, 内部 debug_assert_eq!(0) 通过
  drop(allocator);
  OK
}

/// 缺陷 5 验证: ZoneAllocationPolicy 实现了 PartialEq, Eq, Debug
#[test]
fn zone_allocation_policy_traits() -> Void {
  let allocator = AccountingAllocator::new();
  let zone1 = Zone::new(&allocator, "policy_zone_1");
  let zone2 = Zone::new(&allocator, "policy_zone_2");

  let p1 = ZoneAllocationPolicy::new(&zone1);
  let p1_copy = p1;
  let p2 = ZoneAllocationPolicy::new(&zone2);

  // PartialEq / Eq
  assert_eq!(p1, p1_copy);
  assert_ne!(p1, p2);

  // Debug 打印含 zone 名字
  let debug_str = format!("{p1:?}");
  assert!(debug_str.contains("ZoneAllocationPolicy"));
  assert!(debug_str.contains("policy_zone_1"));
  OK
}

/// 深度回滚验证: 多段 Snapshot/Restore 彻底归还多余段并恢复用量
/// (D4: restore 需独占, 跨段分配借用在块结束时先行释放)
#[test]
fn zone_snapshot_multi_segment_restore() -> Void {
  let allocator = AccountingAllocator::new();
  let mut zone = Zone::new(&allocator, "snapshot_multi");

  let snap = {
    let _base = zone.allocate_bytes_ref(500)?;
    zone.snapshot()
  };
  let base_alloc = zone.allocation_size();
  let base_seg_bytes = zone.segment_bytes_allocated();

  // 跨段分配
  {
    let _big1 = zone.allocate_bytes_ref(16 * 1024)?;
    let _big2 = zone.allocate_bytes_ref(32 * 1024)?;
    assert!(zone.segment_bytes_allocated() > base_seg_bytes);
  }

  // 回滚
  snap.restore(&mut zone)?;
  assert_eq!(zone.allocation_size(), base_alloc);
  assert_eq!(zone.segment_bytes_allocated(), base_seg_bytes);
  assert_eq!(allocator.current_memory_usage(), base_seg_bytes);
  OK
}

/// R3-4 (round3 对抗审查) 回归: assign_from realloc 分支容量口径.
/// 原版 zone-containers.h:166-170 以 other.capacity() 精确分配
/// (AllocateArray 直配, 不走高水位倍增); 旧实现走 Grow → max(2*old_cap, min)
/// 导致容量漂移 (外部可观察)。
#[test]
fn zone_vector_assign_from_realloc_exact_capacity() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test_assign_realloc");
  let mut v: ZoneVector<'_, '_, u32> = ZoneVector::new(&zone);
  for i in 1..=3u32 {
    v.push_back(i)?;
  }
  assert_eq!(v.capacity(), 4, "高水位倍增: 2 → 4");
  let other = ZoneVector::from_slice(&zone, &[1, 2, 3, 4, 5])?;
  assert_eq!(other.capacity(), 5, "initializer 构造容量恰为 size");
  // 4 < 5 → 走 realloc 分支
  v.assign_from(&other)?;
  assert_eq!(v.size(), 5);
  assert_eq!(
    v.capacity(),
    5,
    "缺陷: realloc 分支走了高水位倍增, 原版按 other.capacity() 精确分配"
  );
  assert_eq!(v.as_slice(), &[1, 2, 3, 4, 5]);
  OK
}

/// R3-5 (round3 对抗审查) 回归: ZoneVector(size, Zone*) 默认构造容量口径.
/// 原版 zone-containers.h:64-69 直接 AllocateArray(size), 容量恰为 size;
/// 旧实现经 resize→EnsureCapacity→Grow, size=1 被 kMinCapacity=2 抬到 2。
#[test]
fn zone_vector_default_ctor_exact_capacity() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test_default_ctor");
  let v = ZoneVector::<u32>::with_default(&zone, 1)?;
  assert_eq!(v.size(), 1);
  assert_eq!(
    v.capacity(),
    1,
    "缺陷: with_default(1) 容量被 kMinCapacity 抬到 2, 原版恰为 1"
  );
  let v3 = ZoneVector::<u32>::with_default(&zone, 3)?;
  assert_eq!(v3.capacity(), 3, "原版 ctor 分配恰 size");
  assert_eq!(v3.as_slice(), &[0, 0, 0]);
  OK
}

/// R3-6 (round3 对抗审查) 泄漏压力: 固定 seed 伪随机 alloc/snapshot/restore/
/// reset 序列, 逐迭代核对 分配记账 与 运行时段计数, 收尾断言 allocator 归零。
/// (glossary 差分约定: fastrand 显式播种, 禁真随机)
#[test]
fn zone_stress_random_alloc_restore_reset_no_leak() -> Void {
  let mut rng = Rng::with_seed(0x2026_1011);
  let allocator = AccountingAllocator::new();
  let mut zone = Zone::new(&allocator, "stress");
  // (快照, 期望 allocation_size) 栈; LIFO 恢复纪律保证 snapshot 可达
  let mut snaps: Vec<(ZoneSnapshot, usize)> = Vec::new();
  let mut expected = 0usize;
  // align_up(n, 8) 本地口径 (internal::bits 不导出)
  let round8 = |n: usize| (n + 7) & !7;

  for i in 0..2000 {
    match rng.usize(0..100) {
      // 60%: 分配 0..=512 字节 (0 走原版 zone.h:66-69 零长分支)
      0..=59 => {
        let n = rng.usize(0..=512);
        // 借用随语句结束 (let _ 立即弃置, NLL 终结借用)
        let _ = zone.allocate_bytes_ref(n)?;
        expected += round8(n);
      }
      // 10%: 快照入栈
      60..=69 => {
        snaps.push((zone.snapshot(), expected));
      }
      // 15%: LIFO 回滚 (栈空退化为分配 0)
      70..=84 => {
        if let Some((snap, exp)) = snaps.pop() {
          snap.restore(&mut zone)?;
          expected = exp;
        }
      }
      // 5%: reset — 保留段语义, 此前快照全部作废
      85..=89 => {
        zone.reset();
        snaps.clear();
        expected = 0;
      }
      // 10%: 副 zone 建段/归还, 校验 allocator 总账在丢弃后回落
      _ => {
        let other = Zone::new(&allocator, "stress_side");
        let _ = other.allocate_bytes_ref(3000 + rng.usize(0..64))?;
        assert!(other.segment_bytes_allocated() > 0);
        drop(other);
      }
    }
    // 逐迭代不变量: 记账口径一致 + allocator 总量 == 本 zone 段总量 (无泄漏)
    assert_eq!(
      zone.allocation_size(),
      expected,
      "iter {i}: allocation_size 记账漂移"
    );
    assert_eq!(
      allocator.current_memory_usage(),
      zone.segment_bytes_allocated(),
      "iter {i}: allocator 段总账与 zone 段计数不一致 (段泄漏或双归还)"
    );
    assert!(allocator.max_memory_usage() >= allocator.current_memory_usage());
  }

  // 混合容器分配/释放路径 (Grow → DeleteArray 旧块) 后总账仍守恒
  {
    let mut v: ZoneVector<'_, '_, u64> = ZoneVector::new(&zone);
    for k in 0..500u64 {
      v.push_back(k)?;
    }
    assert_eq!(v.size(), 500);
    let mut l: ZoneList<'_, '_, u64> = ZoneList::new(4, &zone)?;
    for k in 0..300u64 {
      l.add(&k)?;
    }
    l.rewind(10);
  }
  assert_eq!(
    allocator.current_memory_usage(),
    zone.segment_bytes_allocated()
  );

  // 收尾: zone Drop → DeleteAll, allocator 归零 (无运行时泄漏)
  drop(snaps);
  drop(zone);
  assert_eq!(
    allocator.current_memory_usage(),
    0,
    "zone 析构后 allocator 段总账必须归零"
  );
  assert!(allocator.max_memory_usage() > 0, "压力序列必须真实建过段");
  OK
}
