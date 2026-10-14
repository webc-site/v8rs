//! 审查环 round2 补强用例 (第 2 轮对抗式审查)
//!
//! 针对 5 项核心安全领域设计对抗用例:
//! 1. 内存安全与 SAFETY 标注 (ZST make 提前析构防范、Segment 头部越界防范)
//! 2. 内存对齐与溢出防护 (超大 size/align_up/checked_mul 防溢出与 OOM 判定)
//! 3. Accounting 计数准确性 (高水位与超限防护)
//! 4. Segment 生命周期 (超小段拒绝、段安全分配)
//! 5. 容器边界 (ZoneVector self-assign、insertion 溢出防护、ZoneDeque 双端交叉一致性)

use std::{
  mem,
  sync::atomic::{AtomicUsize, Ordering},
};

use aok::{OK, Void};
use v8_zone::{
  AccountingAllocator, Error, RecyclingZoneAllocator, Segment, Zone, ZoneAllocationPolicy,
  ZoneDeque, ZoneScope, ZoneVector,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 缺陷 1 防护: 原版 zone.cc:163-177 new_size > INT_MAX 判定为 OOM.
/// Rust 版超限 size (如 usize::MAX 或 MAX_NEW_SIZE+1) 必须返回 Err(OutOfMemory),
/// 绝不能在 debug 下加法溢出 panic, 更不能在 release 下 wrapping 回绕为 0 返回虚假成功.
#[test]
fn zone_test_allocate_bytes_massive_overflow_returns_oom() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test_oom");

  let r1 = zone.allocate_bytes_ref(usize::MAX);
  assert!(matches!(r1, Err(Error::OutOfMemory { .. })));

  let r2 = zone.allocate_bytes_ref(usize::MAX - 1);
  assert!(matches!(r2, Err(Error::OutOfMemory { .. })));

  let r3 = zone.allocate_bytes_ref(usize::MAX - 7);
  assert!(matches!(r3, Err(Error::OutOfMemory { .. })));

  let r4 = zone.allocate_bytes_ref(i32::MAX as usize + 1);
  assert!(matches!(r4, Err(Error::OutOfMemory { .. })));

  // allocate_array::<u8> 在 length * 1 未溢出时转调 allocate_bytes, 同样受 OOM 拦截
  let r5 = zone.allocate_array::<u8>(usize::MAX - 1);
  assert!(matches!(r5, Err(Error::OutOfMemory { .. })));
  OK
}

static ZST_DROPPED: AtomicUsize = AtomicUsize::new(0);

struct ZstDrop;

impl Drop for ZstDrop {
  fn drop(&mut self) {
    ZST_DROPPED.fetch_add(1, Ordering::Relaxed);
  }
}

/// 缺陷 2 防护: Zone::make 处理 ZST 时, 必须 mem::forget 防止实参 value
/// 在 make 函数返回时立即触发 drop 导致返回悬垂引用 (UAF).
#[test]
fn zone_test_make_zst_with_drop_does_not_drop_prematurely() -> Void {
  let dropped_before = ZST_DROPPED.load(Ordering::Relaxed);
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test_zst");

  let zst_ref = zone.make(ZstDrop)?;
  // 此时 returned reference 仍然存活, 实参绝不能已经被析构
  let dropped_after_make = ZST_DROPPED.load(Ordering::Relaxed);
  assert_eq!(
    dropped_after_make, dropped_before,
    "ZST 必须经 mem::forget 转移所有权, 不得在 make 返回时提前析构"
  );

  // 保证借用存活
  let _addr = zst_ref as *mut ZstDrop as usize;
  OK
}

/// 缺陷 3 防护: 原版 accounting-allocator.cc:135 DCHECK_LE(sizeof(Segment), count)
/// Rust 版 allocate_segment 是 pub API, 当请求 bytes < size_of::<Segment>() 时
/// 必须返回 None, 绝不能在 release 模式下写越界导致堆破坏.
#[test]
fn accounting_allocator_allocate_segment_underflow_guard() -> Void {
  let allocator = AccountingAllocator::new();

  assert!(allocator.allocate_segment(0).is_none());
  assert!(
    allocator
      .allocate_segment(mem::size_of::<Segment>() - 1)
      .is_none()
  );
  assert!(allocator.allocate_segment(8).is_none());

  // 正常合规分配
  let seg = allocator.allocate_segment(mem::size_of::<Segment>() + 64);
  assert!(seg.is_some());
  if let Some(s) = seg {
    unsafe { allocator.return_segment(s) };
  }
  OK
}

/// 缺陷 4 防护: 原版 zone-containers.h:131 DCHECK_NE(this, &other)
/// ZoneVector::assign_from 自赋值时必须短路, 严禁 drop_in_place 之后从同一位置 clone (UAF/double-drop).
/// (miri 下 ignore: 用例经裸指针自造 &mut 与 & 别名, 调用点本身即调用方侧
/// Stacked Borrows UB — R2-4 守卫防的是函数体内二次解引用, 常规构建完整覆盖;
/// Rust 无 sound 的"同一对象 &mut+&"构造形态, 对应原版 DCHECK 契约)
#[test]
#[cfg_attr(miri, ignore)]
fn zone_vector_assign_self_safe() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test_self_assign");
  let mut v: ZoneVector<'_, '_, i32> = ZoneVector::new(&zone);
  v.push_back(10)?;
  v.push_back(20)?;
  v.push_back(30)?;

  // 自赋值
  let raw_ptr = &v as *const ZoneVector<'_, '_, i32>;
  unsafe {
    v.assign_from(&*raw_ptr)?;
  }

  assert_eq!(v.size(), 3);
  assert_eq!(v.at(0), &10);
  assert_eq!(v.at(1), &20);
  assert_eq!(v.at(2), &30);
  OK
}

/// 缺陷 5 防护: 原版 zone-containers.h:510 CHECK(max - size() >= count)
/// ZoneVector 扩容与插入时当 old_size + count 溢出必须返回 Err(SizeOverflow),
/// 不得发生整数回绕越界.
#[test]
fn zone_vector_prepare_for_insertion_overflow_guard() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test_ins_overflow");
  let mut v: ZoneVector<'_, '_, u32> = ZoneVector::new(&zone);
  v.push_back(42)?;

  let r = v.insert_repeat(0, usize::MAX, &1);
  assert!(matches!(r, Err(Error::SizeOverflow)));
  OK
}

/// 缺陷 6 防护: RecyclingZoneAllocator::deallocate 乘法溢出防护.
#[test]
fn recycling_zone_allocator_deallocate_large_size_overflow_guard() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test_recyc_overflow");
  let za = RecyclingZoneAllocator::<u64>::new(&zone);
  let p = za.allocate(4)?;
  // 传入极端 n, 不得触发整数溢出 panic
  unsafe { za.deallocate(p, usize::MAX) };
  unsafe { za.deallocate(p, 4) };
  OK
}

/// 缺陷 7 防护: ZoneAllocationPolicy::delete_array 乘法溢出防护.
#[test]
fn zone_delete_large_count_overflow_guard() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test_delete_overflow");
  let p = zone.allocate_bytes_ref(16)?;
  let policy = ZoneAllocationPolicy::new(&zone);
  // 极端 count 归还仅安全跳过, 不得 panic 溢出
  policy.delete_array(p.as_mut_ptr().cast::<u64>(), usize::MAX);
  policy.delete_array(p.as_mut_ptr().cast::<u64>(), 2);
  OK
}

/// 边界测试: ZoneDeque 双端交替操作一致性与正确性
#[test]
fn zone_deque_interleaved_push_pop_consistency() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test_deque");
  let mut deque: ZoneDeque<'_, '_, i32> = ZoneDeque::new(&zone);

  // 交替 push_front / push_back
  deque.push_front(2)?;
  deque.push_back(3)?;
  deque.push_front(1)?;
  deque.push_back(4)?;
  assert_eq!(deque.size(), 4);
  assert_eq!(*deque.front().unwrap(), 1);
  assert_eq!(*deque.back().unwrap(), 4);
  assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3, 4]);

  // 交替 pop_front / pop_back 触发双栈反向搬运
  assert_eq!(deque.pop_back()?.unwrap(), 4);
  assert_eq!(deque.pop_front()?.unwrap(), 1);
  assert_eq!(deque.pop_back()?.unwrap(), 3);
  assert_eq!(deque.pop_front()?.unwrap(), 2);
  assert!(deque.is_empty());
  assert_eq!(deque.pop_front()?, None);
  assert_eq!(deque.pop_back()?, None);
  OK
}

/// 边界测试: ZoneScope 作用域回滚与生命周期安全 (D4: scope 持 &mut Zone)
#[test]
fn zone_scope_rollback_and_subsequent_allocation() -> Void {
  let allocator = AccountingAllocator::new();
  let mut zone = Zone::new(&allocator, "test_scope");
  let (base_addr, usage_base) = {
    let base_alloc = zone.allocate_bytes_ref(128)?;
    (base_alloc.as_mut_ptr() as usize, zone.allocation_size())
  };

  {
    let mut scope = ZoneScope::new(&mut zone);
    let _nested = scope.zone().allocate_bytes_ref(1024)?;
    assert!(scope.zone().allocation_size() > usage_base);
  }

  // scope 析构后回滚到 usage_base
  assert_eq!(zone.allocation_size(), usage_base);

  // 随后继续分配
  let next_alloc = zone.allocate_bytes_ref(64)?;
  assert_eq!(zone.allocation_size(), usage_base + 64);
  assert_ne!(base_addr, next_alloc.as_mut_ptr() as usize);
  OK
}
