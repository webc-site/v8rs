//! Oilpan/cppgc 核心对象模型、元数据表与位图集成测试
//! 对应原版 v8/src/heap/cppgc-internal/* 与 include/cppgc/* 规范

use core::ffi::c_void;
use std::{ptr::null, sync::atomic::AtomicU16};

use aok::{OK, Void};
use v8_heap_cppgc::{
  AccessMode, AllocationObserverImpl, BaseObjectSizeTrait, CompactionWorklists,
  EnsureGcInfoIndexTrait, GcCycle, GcCycleType, GlobalGcInfoTable, HeapObjectHeader,
  HeapObjectNameForUnnamedObject, K_ALLOCATION_GRANULARITY, K_ALLOCATION_MASK, K_BITMAP_SIZE,
  K_FREE_LIST_ENTRY_SIZE, K_GB, K_HIDDEN_NAME, K_KB, K_LARGE_OBJECT_SIZE_THRESHOLD,
  K_MAX_GC_INFO_INDEX, K_MAX_INDEX, K_MB, K_MIN_GC_INFO_INDEX, K_MIN_INDEX, K_NO_NAME_DEDUCIBLE,
  K_PAGE_BASE_MASK, K_PAGE_OFFSET_MASK, K_PAGE_SIZE, K_PAGE_SIZE_LOG2, K_RESERVED_FOR_BITMAP,
  K_SLOT_SIZE, LivenessBrokerFactory, MainThreadIncrementalMark, MainThreadIncrementalSweep,
  MetricRecorder, NameProvider, NameTrait, NameTraitBase, ObjectStartBitmap, ObjectView,
  PlatformAwareObjectStartBitmap, ProcessHeapStatistics, ProcessHeapStatisticsUpdater,
  SingleThreadedHandle, Visitor, VisitorFactory, check_memory_is_inaccessible,
  check_memory_is_zero, is_heap_object_alive_impl, set_memory_inaccessible,
  supports_cpp_class_names_as_object_names,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_globals_and_constants() -> Void {
  const {
    // 页尺寸与位移对齐约束
    assert!(K_PAGE_SIZE == 1 << K_PAGE_SIZE_LOG2);
    assert!(K_PAGE_OFFSET_MASK == K_PAGE_SIZE - 1);
    assert!(K_PAGE_BASE_MASK == !K_PAGE_OFFSET_MASK);
    assert!(K_PAGE_OFFSET_MASK & K_PAGE_BASE_MASK == 0);
    assert!(K_PAGE_OFFSET_MASK | K_PAGE_BASE_MASK == usize::MAX);

    // 分配粒度与掩码不变式
    assert!(K_ALLOCATION_MASK == K_ALLOCATION_GRANULARITY - 1);
    assert!(K_ALLOCATION_GRANULARITY.is_power_of_two());

    // 起跑位图与页跨度完整覆盖 (每 bit 表 1 分配粒度)
    assert!(K_BITMAP_SIZE * 8 * K_ALLOCATION_GRANULARITY == K_PAGE_SIZE);
    assert!(K_RESERVED_FOR_BITMAP == K_BITMAP_SIZE);

    // 阈值与容量约束
    assert!(K_LARGE_OBJECT_SIZE_THRESHOLD == K_PAGE_SIZE / 2);
    assert!(K_FREE_LIST_ENTRY_SIZE == 2 * K_SLOT_SIZE);
    assert!(K_MIN_INDEX == K_MIN_GC_INFO_INDEX);
    assert!(K_MAX_INDEX == K_MAX_GC_INFO_INDEX);
    assert!(K_MAX_INDEX > K_MIN_INDEX);

    // 标量单位换算
    assert!(K_MB == K_KB * 1024);
    assert!(K_GB == K_MB * 1024);
  };

  assert!(supports_cpp_class_names_as_object_names());
  OK
}

#[test]
fn test_heap_object_header() -> Void {
  assert_eq!(size_of::<HeapObjectHeader>(), K_ALLOCATION_GRANULARITY);

  let mut header = HeapObjectHeader::new(32, 1);
  assert_eq!(header.allocated_size(AccessMode::NonAtomic), 32);
  assert_eq!(header.get_gc_info_index(AccessMode::NonAtomic), 1);
  assert!(!header.is_marked(AccessMode::NonAtomic));
  assert!(header.is_young(AccessMode::NonAtomic));
  assert!(header.is_in_construction(AccessMode::NonAtomic));

  header.mark_as_fully_constructed();
  assert!(!header.is_in_construction(AccessMode::NonAtomic));

  // 标记
  header.mark_non_atomic();
  assert!(header.is_marked(AccessMode::NonAtomic));
  assert!(!header.is_young(AccessMode::NonAtomic));

  // 原子标记 (已经标记过，返回 false)
  assert!(!header.try_mark_atomic());

  // 清除标记
  header.unmark(AccessMode::NonAtomic);
  assert!(!header.is_marked(AccessMode::NonAtomic));
  assert!(header.try_mark_atomic());
  assert!(header.is_marked(AccessMode::Atomic));
  OK
}

#[test]
fn test_object_start_bitmap() -> Void {
  let mut bitmap = ObjectStartBitmap::new();

  // 模拟对齐地址 (按分配粒度 8 字节对齐)
  let addr1 = (K_PAGE_SIZE * 2 + 64) as *const u8;
  let addr2 = (K_PAGE_SIZE * 2 + 128) as *const u8;

  assert!(!bitmap.check_bit(AccessMode::NonAtomic, addr1));
  assert!(!bitmap.check_bit(AccessMode::NonAtomic, addr2));

  bitmap.set_bit(AccessMode::NonAtomic, addr1);
  assert!(bitmap.check_bit(AccessMode::NonAtomic, addr1));
  assert!(!bitmap.check_bit(AccessMode::NonAtomic, addr2));

  bitmap.set_bit(AccessMode::NonAtomic, addr2);
  assert!(bitmap.check_bit(AccessMode::NonAtomic, addr2));

  bitmap.clear_bit(AccessMode::NonAtomic, addr1);
  assert!(!bitmap.check_bit(AccessMode::NonAtomic, addr1));
  assert!(bitmap.check_bit(AccessMode::NonAtomic, addr2));

  // 迭代 (检查页内偏移)
  let mut found = Vec::new();
  bitmap.iterate(|addr| {
    found.push(addr as usize & K_PAGE_OFFSET_MASK);
  });
  assert_eq!(found.len(), 1);
  assert_eq!(found[0], 128);

  // find_header 测试: 在同一页基底内查找对象头
  let page_base = (&bitmap as *const _ as usize) & K_PAGE_BASE_MASK;
  let addr_in_page = (page_base + 64) as *const u8;
  bitmap.set_bit(AccessMode::NonAtomic, addr_in_page);
  // 从对象中部 (+72 字节) 向前寻址对象头
  let mid_addr = (page_base + 72) as *const u8;
  // SAFETY: addr_in_page 与 mid_addr 在 bitmap 所在页基内。
  let h_ptr = unsafe { bitmap.find_header(AccessMode::NonAtomic, mid_addr) };
  assert_eq!(h_ptr as usize, page_base + 64);

  // 未置位低格回退越界返回空指针
  let before_addr = page_base as *const u8;
  // SAFETY: before_addr 在同一页基内。
  let h_null = unsafe { bitmap.find_header(AccessMode::NonAtomic, before_addr) };
  assert!(h_null.is_null());

  // PlatformAware 适配层测试
  let mut p_bitmap = PlatformAwareObjectStartBitmap {
    bitmap: ObjectStartBitmap::new(),
  };
  p_bitmap.set_bit(AccessMode::NonAtomic, addr1);
  assert!(p_bitmap.bitmap.check_bit(AccessMode::NonAtomic, addr1));
  p_bitmap.clear_bit(AccessMode::NonAtomic, addr1);
  assert!(!p_bitmap.bitmap.check_bit(AccessMode::NonAtomic, addr1));
  assert!(!PlatformAwareObjectStartBitmap::should_force_non_atomic(
    AccessMode::Atomic
  ));

  bitmap.clear();
  assert!(!bitmap.check_bit(AccessMode::NonAtomic, addr2));
  OK
}

#[test]
fn test_gc_info_and_table() -> Void {
  static INDEX_CELL: AtomicU16 = AtomicU16::new(0);

  unsafe fn dummy_trace(_visitor: *mut v8_heap_cppgc::Visitor, _obj: *const c_void) {}

  let idx1 = EnsureGcInfoIndexTrait::ensure_gc_info_index_trace_only(&INDEX_CELL, dummy_trace)?;
  assert_eq!(idx1, 1);

  // 二次注册幂等
  let idx2 = EnsureGcInfoIndexTrait::ensure_gc_info_index_trace_only(&INDEX_CELL, dummy_trace)?;
  assert_eq!(idx2, 1);

  // 全局表查询
  let info = GlobalGcInfoTable::gc_info_from_index(1);
  let name = (info.name)(null(), HeapObjectNameForUnnamedObject::UseHiddenName);
  assert_eq!(name.value, K_HIDDEN_NAME);
  assert!(name.name_was_hidden);

  let name_unhidden = (info.name)(
    null(),
    HeapObjectNameForUnnamedObject::UseClassNameIfSupported,
  );
  assert_eq!(name_unhidden.value, K_HIDDEN_NAME);
  assert!(!name_unhidden.name_was_hidden);

  // 表状态量与槽位测试
  GlobalGcInfoTable::table(|t| {
    assert!(t.number_of_gc_infos() >= 1);
    assert!(t.limit_for_testing() >= 512);
    let s = t.table_slot_for_testing(1);
    assert!((s.name)(null(), HeapObjectNameForUnnamedObject::UseHiddenName).name_was_hidden);
  });

  OK
}

#[test]
fn test_task_handle() -> Void {
  // 空句柄 (default): 非活跃
  let empty = SingleThreadedHandle::default();
  assert!(!empty.is_active());
  // 空句柄 CancelIfNonEmpty 无操作
  empty.cancel_if_non_empty();

  // 非空句柄: 活跃 → 取消 → 不活跃
  let h = SingleThreadedHandle::new_non_empty(v8_heap_cppgc::NonEmptyTag);
  let h_clone = h.clone();
  assert!(h.is_active());
  assert!(h_clone.is_active());
  assert!(!h.is_canceled());
  assert!(!h_clone.is_canceled());

  h.cancel_if_non_empty();
  assert!(!h.is_active());
  assert!(!h_clone.is_active());
  assert!(h.is_canceled());
  assert!(h_clone.is_canceled());
  OK
}

#[test]
fn test_object_view_and_liveness() -> Void {
  let broker = LivenessBrokerFactory::create();

  // 存活: 标记前后 (纯栈分配对象头借用, 零堆内存泄漏)
  let mut hl = HeapObjectHeader::new(32, 1);
  let payload = hl.object_start().cast_const().cast::<c_void>();
  // SAFETY: payload 由活对象头紧邻之后派生, 前置为有效头。
  assert!(!unsafe { is_heap_object_alive_impl(payload) });
  assert!(!unsafe { broker.is_heap_object_alive_impl(payload) });
  hl.mark_non_atomic();
  // SAFETY: 同上。
  assert!(unsafe { is_heap_object_alive_impl(payload) });
  assert!(unsafe { broker.is_heap_object_alive_impl(payload) });

  // 视图: 独立头 (生命周期参数 'a 零拷贝借用)
  let hv = HeapObjectHeader::new(40, 1);
  let view = ObjectView::new(&hv, AccessMode::NonAtomic);
  assert_eq!(view.size(), 40 - K_ALLOCATION_GRANULARITY);
  assert_eq!(view.end() as usize - view.start() as usize, view.size());
  assert_eq!(view.start(), hv.object_start());
  OK
}

#[test]
fn test_base_object_size_trait() -> Void {
  let h = HeapObjectHeader::new(32, 1);
  let payload = h.object_start().cast_const().cast::<c_void>();
  // SAFETY: payload 为活对象头之后派生的对象起点。
  assert_eq!(
    unsafe { BaseObjectSizeTrait::get_object_size_for_garbage_collected(payload) },
    24
  );
  OK
}

#[test]
fn test_memory_zap() -> Void {
  let mut buf = [0xff_u8; 8];
  // SAFETY: buf 为 8 字节可写有效内存。
  unsafe { set_memory_inaccessible(buf.as_mut_ptr(), buf.len()) };
  assert!(buf.iter().all(|&b| b == 0));
  // SAFETY: buf 已被置零。
  unsafe { check_memory_is_zero(buf.as_ptr(), buf.len()) };
  unsafe { check_memory_is_inaccessible(null(), 0) };
  OK
}

#[test]
fn test_compaction_worklists() -> Void {
  let mut cw = CompactionWorklists::default();
  cw.movable_slots_worklist().push(null());
  cw.movable_slots_worklist().push(null());
  assert_eq!(cw.movable_slots_worklist_ref().len(), 2);
  cw.clear_for_testing();
  assert!(cw.movable_slots_worklist_ref().is_empty());
  OK
}

#[test]
fn test_process_heap_statistics() -> Void {
  let s0 = ProcessHeapStatistics::total_allocated_space();
  let o0 = ProcessHeapStatistics::total_allocated_object_size();
  let (s_cur, o_cur) = ProcessHeapStatistics::current();
  assert_eq!(s0, s_cur);
  assert_eq!(o0, o_cur);

  ProcessHeapStatisticsUpdater::increase_total_allocated_object_size(64);
  assert_eq!(
    ProcessHeapStatistics::total_allocated_object_size() - o0,
    64
  );
  ProcessHeapStatisticsUpdater::decrease_total_allocated_object_size(64);
  assert_eq!(ProcessHeapStatistics::total_allocated_object_size(), o0);

  // AllocationObserverImpl 转发与清算
  let mut obs = AllocationObserverImpl::default();
  obs.allocated_object_size_increased(32);
  assert_eq!(
    ProcessHeapStatistics::total_allocated_object_size() - o0,
    32
  );
  obs.allocated_size_increased(128);
  assert_eq!(ProcessHeapStatistics::total_allocated_space() - s0, 128);
  obs.allocated_object_size_decreased(16);
  assert_eq!(
    ProcessHeapStatistics::total_allocated_object_size() - o0,
    16
  );
  obs.allocated_size_decreased(64);
  assert_eq!(ProcessHeapStatistics::total_allocated_space() - s0, 64);
  obs.reset_allocated_object_size(0);
  assert_eq!(ProcessHeapStatistics::total_allocated_object_size(), o0);
  obs.allocated_size_decreased(64);
  assert_eq!(ProcessHeapStatistics::total_allocated_space(), s0);
  OK
}

#[test]
fn test_name_trait() -> Void {
  // 1. 无名对象推导: 类名模式 vs 隐藏名模式
  struct MyTestObject;
  let unnamed = NameTrait::<MyTestObject>::get_name_unnamed(
    null(),
    HeapObjectNameForUnnamedObject::UseClassNameIfSupported,
  );
  assert!(unnamed.value.contains("MyTestObject"));
  assert!(!unnamed.name_was_hidden);

  let hidden = NameTrait::<MyTestObject>::get_name_unnamed(
    null(),
    HeapObjectNameForUnnamedObject::UseHiddenName,
  );
  assert_eq!(hidden.value, K_HIDDEN_NAME);
  assert!(hidden.name_was_hidden);

  // 2. 有名 NameProvider
  struct CustomNamed;
  impl NameProvider for CustomNamed {
    fn get_human_readable_name(&self) -> &'static str {
      "CustomNameForTest"
    }
  }
  let custom = CustomNamed;
  // SAFETY: custom 为有效引用。
  let named = unsafe {
    NameTrait::<CustomNamed>::get_name(
      &custom as *const _ as *const c_void,
      HeapObjectNameForUnnamedObject::UseClassNameIfSupported,
    )
  };
  assert_eq!(named.value, "CustomNameForTest");
  assert!(!named.name_was_hidden);

  // 3. 从类型签名解析名 (零分配零拷贝)
  let parsed =
    NameTraitBase::get_name_from_type_signature(Some("void func() [T = ExtractedClass]"));
  assert_eq!(parsed.value, "ExtractedClass");
  assert!(!parsed.name_was_hidden);

  // 4. 空指针与非合法签名回退
  let no_sig = NameTraitBase::get_name_from_type_signature(None);
  assert_eq!(no_sig.value, K_NO_NAME_DEDUCIBLE);
  assert!(!no_sig.name_was_hidden);

  let bad_sig = NameTraitBase::get_name_from_type_signature(Some("not a signature"));
  assert_eq!(bad_sig.value, K_NO_NAME_DEDUCIBLE);
  assert!(!bad_sig.name_was_hidden);

  OK
}

#[test]
fn test_visitor() -> Void {
  let key = VisitorFactory::create_key();
  let visitor = Visitor::new_with_key(key);
  let _k = visitor.key();
  OK
}

#[test]
fn test_metric_recorder_types() -> Void {
  let cycle = GcCycle::default();
  assert_eq!(cycle.type_, GcCycleType::Major);
  assert!(cycle.total.weak_duration_us.is_none());
  let mark = MainThreadIncrementalMark::default();
  assert!(mark.duration_us.is_none());
  let sweep = MainThreadIncrementalSweep::default();
  assert!(sweep.duration_us.is_none());

  struct Recorder;
  impl MetricRecorder for Recorder {}
  let mut r = Recorder;
  r.add_main_thread_event_gc_cycle(&cycle);
  r.add_main_thread_event_incremental_mark(&mark);
  r.add_main_thread_event_incremental_sweep(&sweep);
  OK
}
