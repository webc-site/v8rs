use aok::{OK, Void};
use v8_heap_gc::*;

#[test]
fn test_heap_layout_constants() -> Void {
  assert_eq!(K_REGULAR_PAGE_SIZE, 1 << K_PAGE_SIZE_BITS);
  assert_eq!(K_PAGE_ALIGNMENT_MASK, K_REGULAR_PAGE_SIZE - 1);
  assert_eq!(K_MAX_REGULAR_HEAP_OBJECT_SIZE, 1 << (K_PAGE_SIZE_BITS - 1));
  OK
}

#[test]
fn test_marking_bitmap_operations() -> Void {
  let mut bitmap = MarkingBitmap::new();
  assert!(bitmap.is_clean());

  let index = 1234;
  let mark_bit = bitmap.mark_bit_from_index(index);
  assert!(!mark_bit.get(&bitmap.cells));

  assert!(mark_bit.set(&mut bitmap.cells));
  assert!(mark_bit.get(&bitmap.cells));
  assert!(!bitmap.is_clean());

  assert!(mark_bit.clear(&mut bitmap.cells));
  assert!(!mark_bit.get(&bitmap.cells));
  assert!(bitmap.is_clean());

  // 范围操作
  bitmap.set_range(100, 200);
  assert!(bitmap.all_bits_set_in_range(100, 200));
  assert!(bitmap.all_bits_clear_in_range(0, 100));
  assert!(bitmap.all_bits_clear_in_range(200, 300));

  // 超界保护测试 (无 panic)
  bitmap.set_range(K_LENGTH as u32 - 10, K_LENGTH as u32 + 100);
  assert!(bitmap.all_bits_set_in_range(K_LENGTH as u32 - 10, K_LENGTH as u32));
  bitmap.clear_range(K_LENGTH as u32 - 10, K_LENGTH as u32 + 100);
  assert!(bitmap.all_bits_clear_in_range(K_LENGTH as u32 - 10, K_LENGTH as u32));

  bitmap.clear();
  assert!(bitmap.is_clean());

  // 地址换算与掩码
  let base_addr = K_REGULAR_PAGE_SIZE;
  let idx = address_to_index(base_addr + 64);
  assert_eq!(idx, (64 >> K_TAGGED_SIZE_LOG2) as u32);
  assert_eq!(idx, 16);
  let addr_offset = index_to_address_offset(idx);
  assert_eq!(addr_offset, 64);

  let cell_idx = index_to_cell(idx);
  let in_cell = index_in_cell(idx);
  let mask = index_in_cell_mask(idx);
  assert_eq!(mask, 1usize << in_cell);
  assert_eq!(cell_idx, 0);
  assert_eq!(cell_to_base(1), 256);

  OK
}

#[test]
fn test_marking_progress_tracker() -> Void {
  let mut tracker = MarkingProgressTracker::new();
  assert!(!tracker.is_enabled());
  assert_eq!(tracker.total_number_of_chunks(), 0);

  // 1 MB = 1024 * 1024 bytes, chunk = 128 * 1024 -> 8 chunks
  let test_size = 1024 * 1024;
  tracker.enable(test_size);
  assert!(tracker.is_enabled());
  assert_eq!(tracker.total_number_of_chunks(), 8);

  // 连续获取 chunk
  for i in 0..8 {
    assert_eq!(tracker.get_next_chunk_to_mark(), i);
  }
  assert_eq!(tracker.get_current_chunk_for_testing(), 8);

  tracker.reset_if_enabled();
  assert_eq!(tracker.get_current_chunk_for_testing(), 0);

  OK
}

#[test]
fn test_gc_tracer_and_strings() -> Void {
  assert_eq!(
    event_type_to_string(GcEventType::Scavenger, false),
    "Scavenge"
  );
  assert_eq!(event_type_to_string(GcEventType::Scavenger, true), "s");
  assert_eq!(
    event_type_to_string(GcEventType::MarkCompactor, false),
    "Mark-Compact"
  );
  assert_eq!(event_type_to_string(GcEventType::MarkCompactor, true), "mc");

  assert_eq!(event_state_to_string(EventState::Marking), Some("Marking"));
  assert_eq!(event_state_to_string(EventState::Atomic), Some("Atomic"));
  assert_eq!(
    event_state_to_string(EventState::Sweeping),
    Some("Sweeping")
  );
  // 原版 ToString(NOT_RUNNING) 返回 nullptr (无名), 1:1 对应 None
  assert_eq!(event_state_to_string(EventState::NotRunning), None);

  assert_eq!(
    priority_to_string(GcTracerPriority::UserBlocking),
    "UserBlocking"
  );
  assert_eq!(
    priority_to_string(GcTracerPriority::UserVisible),
    "UserVisible"
  );
  assert_eq!(
    priority_to_string(GcTracerPriority::BestEffort),
    "BestEffort"
  );

  // 吞吐与速度有界性
  let bounded = bounded_average_speed(Some(0.1));
  assert_eq!(bounded, Some(K_MIN_NON_EMPTY_SPEED_IN_BYTES_PER_MS));
  let large_bounded = bounded_average_speed(Some(1e12));
  assert_eq!(large_bounded, Some(K_MAX_SPEED_IN_BYTES_PER_MS));

  assert_eq!(ScopeId::NUMBER_OF_SCOPES, 166);
  assert_eq!(ScopeId::FIRST_INCREMENTAL_SCOPE, ScopeId::McIncremental);
  OK
}

#[test]
fn test_incremental_marking_heuristics() -> Void {
  // Step origin 最大持续时间测试
  assert_eq!(
    get_max_duration_ms(StepOrigin::Task),
    K_MAX_STEP_SIZE_ON_TASK_MS
  );
  assert_eq!(
    get_max_duration_ms(StepOrigin::V8),
    K_MAX_STEP_SIZE_ON_ALLOCATION_MS
  );

  // 激活阈值逻辑 (8MB/8MB 阈值)
  assert!(is_below_activation_thresholds(1024, 1024, false));
  assert!(!is_below_activation_thresholds(
    10 * 1024 * 1024,
    1024,
    false
  ));
  OK
}

#[test]
fn test_memory_reducer_state_flow() -> Void {
  let uninit = MemoryReducerState::create_uninitialized();
  assert_eq!(uninit.id, MemoryReducerId::Uninit);

  let pg_event = MemoryReducerEvent {
    event_type: MemoryReducerEventType::PossibleGarbage,
    time_ms: 1000.0,
    committed_memory: 50 * 1024 * 1024,
    next_gc_likely_to_collect_more: false,
    should_start_incremental_gc: false,
    can_start_incremental_gc: false,
  };

  let wait_state = memory_reducer_step(&uninit, &pg_event, 2000.0, 8000.0, 4);
  assert_eq!(wait_state.id, MemoryReducerId::Wait);
  assert_eq!(wait_state.next_gc_start_ms, 9000.0);

  let mut timer_event = MemoryReducerEvent {
    event_type: MemoryReducerEventType::Timer,
    time_ms: 10000.0,
    committed_memory: 50 * 1024 * 1024,
    next_gc_likely_to_collect_more: false,
    should_start_incremental_gc: true,
    can_start_incremental_gc: true,
  };

  let run_state = memory_reducer_step(&wait_state, &timer_event, 2000.0, 8000.0, 4);
  assert_eq!(run_state.id, MemoryReducerId::Run);
  assert_eq!(run_state.started_gcs, 1);

  // 完成一轮 GC 进入 wait，记录 last_gc_time_ms
  let mc_event = MemoryReducerEvent {
    event_type: MemoryReducerEventType::MarkCompact,
    time_ms: 10050.0,
    committed_memory: 40 * 1024 * 1024,
    next_gc_likely_to_collect_more: true,
    should_start_incremental_gc: false,
    can_start_incremental_gc: false,
  };
  let wait_state2 = memory_reducer_step(&run_state, &mc_event, 2000.0, 8000.0, 4);
  assert_eq!(wait_state2.id, MemoryReducerId::Wait);
  assert_eq!(wait_state2.last_gc_time_ms, 10050.0);

  // 看门狗超时测试 (kWatchdogDelayMs = 100000)
  timer_event.time_ms = 200000.0;
  assert!(memory_reducer_watchdog_gc(&wait_state2, &timer_event));
  OK
}

#[test]
fn test_sweeper_concurrency_and_zapping() -> Void {
  // get_max_concurrency: min(concurrent_sweepers, worker + ceil(pages / K_PAGE_PER_TASK))
  let max_workers = get_max_concurrency(8, 2, 100);
  assert!(max_workers <= 8);

  assert_eq!(
    should_zap_dead_objects_on_page(false, false, false),
    ZappingMode::None
  );
  assert_eq!(
    should_zap_dead_objects_on_page(true, false, false),
    ZappingMode::CreateFillersAndZap
  );
  assert_eq!(
    should_zap_dead_objects_on_page(false, false, true),
    ZappingMode::CreateFillers
  );
  OK
}

#[test]
fn test_scavenger_task_computation() -> Void {
  // 并行关闭时恒为 1
  assert_eq!(
    number_of_scavenge_tasks(false, 32 * 1024 * 1024, 8, true),
    1
  );
  // 内存受限时恒为 1
  assert_eq!(
    number_of_scavenge_tasks(true, 16 * 1024 * 1024, 4, false),
    1
  );
  // 16MB 半区 + 4 worker -> min(17, 8, 5) = 5
  assert_eq!(number_of_scavenge_tasks(true, 16 * 1024 * 1024, 4, true), 5);
  // 128MB + 32 worker -> min(129, 8, 33) = 8 (受限于 kMaxScavengerTasks)
  assert_eq!(
    number_of_scavenge_tasks(true, 128 * 1024 * 1024, 32, true),
    8
  );
  OK
}

#[test]
fn test_evacuation_candidate_selection_integration() -> Void {
  let area_size = 256 * 1024; // 256KB
  let pages = vec![
    EvacuationCandidatePage::new(101, 30 * 1024),
    EvacuationCandidatePage::new(102, 50 * 1024),
    EvacuationCandidatePage::new(103, 10 * 1024),
    EvacuationCandidatePage::new(104, 250 * 1024), // only 6KB free -> filtered out at 50% target
  ];
  let res = select_evacuation_candidates(area_size, pages, 50, 200 * 1024, false);
  // Eligible pages sorted: 103 (10KB), 101 (30KB), 102 (50KB)
  // Total live = 90KB. New pages = ceil(90/256) = 1.
  assert_eq!(res.selected_pages, vec![103, 101, 102]);
  assert_eq!(res.total_live_bytes, 90 * 1024);
  assert_eq!(res.estimated_new_pages, 1);
  assert_eq!(res.estimated_released_pages, 2);

  // 释放页数为 0 时防震荡中止测试
  let single_page = vec![EvacuationCandidatePage::new(201, 100 * 1024)];
  let res_aborted =
    select_evacuation_candidates(area_size, single_page.clone(), 10, 1024 * 1024, false);
  assert_eq!(res_aborted.selected_pages.len(), 0);
  assert_eq!(res_aborted.estimated_released_pages, 0);

  // compact_on_every_full_gc 强制开启时绕过中止
  let res_forced = select_evacuation_candidates(area_size, single_page, 10, 1024 * 1024, true);
  assert_eq!(res_forced.selected_pages, vec![201]);

  // 负值门限防护
  assert_eq!(compute_free_bytes_threshold(area_size, -10), 0);
  assert_eq!(compute_free_bytes_threshold(area_size, 0), 0);
  OK
}

#[test]
fn test_scavenger_copy_and_promotion_integration() -> Void {
  let chunk_addr = 0x40000;
  let page_sz = 256 * 1024;
  let age_mark = chunk_addr + 128 * 1024;

  // Below age mark check
  assert!(is_address_below_age_mark(
    chunk_addr,
    page_sz,
    true,
    age_mark,
    chunk_addr + 64 * 1024
  ));
  assert!(!is_address_below_age_mark(
    chunk_addr,
    page_sz,
    true,
    age_mark,
    chunk_addr + 130 * 1024
  ));

  // Preference check
  assert_eq!(
    determine_scavenge_preference(true),
    ScavengePreference::PromoteThenCopy
  );
  assert_eq!(
    determine_scavenge_preference(false),
    ScavengePreference::CopyThenPromote
  );

  // Stats check
  let mut stats = ScavengerTaskStats::new();
  stats.record_copied(4096);
  stats.record_promoted(8192);
  assert_eq!(stats.copied_size, 4096);
  assert_eq!(stats.promoted_size, 8192);
  assert!(ScavengerTaskStats::should_eagerly_process_promoted_list(
    scavenger_consts::K_PROCESS_PROMOTED_LIST_THRESHOLD as usize
  ));
  OK
}

#[test]
fn test_sweeper_state_machine_integration() -> Void {
  let mut sm = SweepingStateMachine::new();
  assert_eq!(sm.phase(), SweepingPhase::Idle);
  assert!(!sm.is_in_progress());

  let seed = 0xabcdef;
  sm.initialize(seed, true)?;
  assert_eq!(sm.phase(), SweepingPhase::Initialized);
  assert_eq!(sm.trace_id(), seed << 1);
  assert_eq!(sm.background_trace_id(), (seed << 1) + 1);

  sm.start_sweeping()?;
  assert!(sm.is_in_progress());
  assert_eq!(sm.phase(), SweepingPhase::Sweeping);

  let job_started = sm.start_concurrent_sweeping(true)?;
  assert!(job_started);
  assert!(sm.has_active_job());

  assert!(sm.pause());
  assert!(!sm.has_active_job());
  assert!(sm.has_valid_job());

  let resumed = sm.resume()?;
  assert!(resumed);
  assert!(sm.has_active_job());

  // 停止并发任务
  assert!(sm.stop_concurrent_sweeping());
  assert!(!sm.has_valid_job());

  sm.join_sweeping()?;
  assert_eq!(sm.phase(), SweepingPhase::Joined);

  sm.finish_sweeping()?;
  assert_eq!(sm.phase(), SweepingPhase::Idle);
  assert!(!sm.is_in_progress());

  // Page sweeping state check
  assert!(!PageSweepingState::Unswept.can_allocate_on_page());
  assert!(!PageSweepingState::SweepingInProgress.can_allocate_on_page());
  assert!(PageSweepingState::Swept.can_allocate_on_page());
  OK
}
