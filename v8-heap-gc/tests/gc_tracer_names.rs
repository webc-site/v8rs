// W11HGR2 round2 防回归: GCTracer 名字表与优先级的原版判别值 1:1 锚定
use aok::{OK, Void};
use v8_heap_gc::{
  EventState, GcEventType, GcTracerPriority, event_state_to_string, event_type_to_string,
  priority_to_string,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// v8: include/v8-isolate.h:698-708 enum class Priority { kBestEffort, kUserVisible, kUserBlocking }
// 判别值升序 0/1/2 (best→blocking), 曾倒置登记, 本例防回归。
#[test]
fn isolate_priority_discriminants_1to1() -> Void {
  assert_eq!(GcTracerPriority::BestEffort as u8, 0);
  assert_eq!(GcTracerPriority::UserVisible as u8, 1);
  assert_eq!(GcTracerPriority::UserBlocking as u8, 2);
  // v8: src/heap/gc-tracer.cc:85-95 ToString(Priority) 三名表
  assert_eq!(
    priority_to_string(GcTracerPriority::BestEffort),
    "BestEffort"
  );
  assert_eq!(
    priority_to_string(GcTracerPriority::UserVisible),
    "UserVisible"
  );
  assert_eq!(
    priority_to_string(GcTracerPriority::UserBlocking),
    "UserBlocking"
  );
  OK
}

// v8: src/heap/gc-tracer.h:189 enum class State { NOT_RUNNING, MARKING, ATOMIC, SWEEPING }
// v8: src/heap/gc-tracer.cc:72-84 ToString(State) — NOT_RUNNING 返回 nullptr (无名)。
#[test]
fn event_state_names_match_original() -> Void {
  assert_eq!(EventState::NotRunning as u8, 0);
  assert_eq!(EventState::Marking as u8, 1);
  assert_eq!(EventState::Atomic as u8, 2);
  assert_eq!(EventState::Sweeping as u8, 3);
  assert_eq!(event_state_to_string(EventState::Marking), Some("Marking"));
  assert_eq!(event_state_to_string(EventState::Atomic), Some("Atomic"));
  assert_eq!(
    event_state_to_string(EventState::Sweeping),
    Some("Sweeping")
  );
  // 原版 nullptr 面: 不得存在显示名
  assert_eq!(event_state_to_string(EventState::NotRunning), None);
  OK
}

// v8: src/heap/gc-tracer.cc:110-125 ToString(Event::Type, short_name) 双名表全枚举
#[test]
fn event_type_double_names_complete() -> Void {
  let pairs = [
    (GcEventType::Scavenger, "s", "Scavenge"),
    (GcEventType::MarkCompactor, "mc", "Mark-Compact"),
    (
      GcEventType::IncrementalMarkCompactor,
      "mc",
      "Incremental Mark-Compact",
    ),
    (GcEventType::MinorMarkSweeper, "mms", "Minor Mark-Sweep"),
    (
      GcEventType::IncrementalMinorMarkSweeper,
      "mms",
      "Incremental Minor Mark-Sweep",
    ),
    (GcEventType::Start, "st", "Start"),
  ];
  for (t, short, long) in pairs {
    assert_eq!(event_type_to_string(t, true), short);
    assert_eq!(event_type_to_string(t, false), long);
  }
  OK
}
