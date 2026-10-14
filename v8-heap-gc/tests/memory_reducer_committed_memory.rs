// W11HGR 回归: MemoryReducer Step 的 DONE/UNINIT + MarkCompact 阈值须复现原版
// std::max(static_cast<size_t>(last * factor), last + delta) 的截断与整数域比较
// (v8/src/heap/memory-reducer.cc:181-188)。补 W10HG 未覆盖的两类边界:
// factor/delta 恰平局 (tie) 与截断值卡进 (1153433601.1 → 1153433601)。
use aok::{OK, Void};
use v8_heap_gc::{
  K_COMMITTED_MEMORY_DELTA, K_COMMITTED_MEMORY_FACTOR, MemoryReducerEvent as Event,
  MemoryReducerEventType as EventType, MemoryReducerId as Id, MemoryReducerState as State,
  memory_reducer_step as step,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const MB: usize = 1024 * 1024;
const LONG: f64 = 8000.0;
const START: f64 = 30000.0;
const MAX_GCS: i32 = 2;

fn mc_garbage_left(time_ms: f64, committed: usize) -> Event {
  Event {
    event_type: EventType::MarkCompact,
    time_ms,
    committed_memory: committed,
    next_gc_likely_to_collect_more: true,
    should_start_incremental_gc: false,
    can_start_incremental_gc: false,
  }
}

// 原版阈值整数域复算: 先各自经 static_cast<size_t> 截断, 再整数 std::max。
fn original_threshold(last: usize) -> usize {
  let by_factor = ((last as f64) * K_COMMITTED_MEMORY_FACTOR) as usize;
  let by_delta = last.saturating_add(K_COMMITTED_MEMORY_DELTA);
  if by_factor > by_delta {
    by_factor
  } else {
    by_delta
  }
}

/// factor 与 delta 两路截断后恰平局 (last=100MB, 0.1*last == delta):
/// threshold=115343360; 恰达阈值 → Wait, 差 1 → Done。
/// 若误用 f64 域 max/比较, 平局两侧微小尾差即可翻转结果, 此例钉死整数域判定。
#[test]
fn done_mark_compact_threshold_tie() -> Void {
  let last = 100 * MB;
  let threshold = original_threshold(last);
  // 平局两侧整数相等 (依赖 IEEE double 1.1 尾差同原版 C++ 一致)
  assert_eq!(threshold, last.saturating_add(K_COMMITTED_MEMORY_DELTA));
  let state0 = State::create_done(1.0, last);
  let s = step(
    &state0,
    &mc_garbage_left(2.0, threshold),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Wait);
  let s = step(
    &state0,
    &mc_garbage_left(2.0, threshold - 1),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Done);
  OK
}

/// 截断卡进边界 (last=1000MB+1): f64 积为 1153433601.1..., 截断阈值 1153433601。
/// 若比较在实数域进行, committed=1153433601 < 1153433601.1 会误判 Done;
/// 原版整数域比较必须 Wait。差 1 (1153433600) 则 Done。
#[test]
fn done_mark_compact_threshold_truncation() -> Void {
  let last = 1000 * MB + 1;
  let threshold = original_threshold(last);
  assert_eq!(threshold, 1_153_433_601);
  let state0 = State::create_done(1.0, last);
  let s = step(
    &state0,
    &mc_garbage_left(2.0, threshold),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Wait);
  let s = step(
    &state0,
    &mc_garbage_left(2.0, threshold - 1),
    LONG,
    START,
    MAX_GCS,
  );
  assert_eq!(s.id, Id::Done);
  OK
}
