// 由 tool/src/heap/gc-tracer.cc.js 从 v8/src/heap/gc-tracer.{cc,-inl.h} 收编; 修订就地改, 复跑覆盖
//! GC 追踪规则面: 事件类型/状态名表 (ToString 双名)、isolate 优先级名、平均速度界。
//! 事件名以 gc_tracer 枚举承载, 免字符串入参误达不可达臂。

use crate::gc_tracer::{EventState, EventType};

/// isolate 优先级 (对应 v8::Isolate::Priority; 判别值 1:1: kBestEffort=0 < kUserVisible=1 < kUserBlocking=2)。
/// v8: include/v8-isolate.h:698-708 enum class Priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Priority {
  /// kBestEffort (最低优先级)
  BestEffort = 0,
  /// kUserVisible
  UserVisible = 1,
  /// kUserBlocking (最高优先级)
  UserBlocking = 2,
}

/// 事件类型名 (对应 ToString(Event::Type, short_name) 双名)。
/// v8: src/heap/gc-tracer.cc ToString(Event::Type)
#[must_use]
pub const fn event_type_to_string(type_: EventType, short_name: bool) -> &'static str {
  match type_ {
    EventType::Scavenger => {
      if short_name {
        "s"
      } else {
        "Scavenge"
      }
    }
    EventType::MarkCompactor => {
      if short_name {
        "mc"
      } else {
        "Mark-Compact"
      }
    }
    EventType::IncrementalMarkCompactor => {
      if short_name {
        "mc"
      } else {
        "Incremental Mark-Compact"
      }
    }
    EventType::MinorMarkSweeper => {
      if short_name {
        "mms"
      } else {
        "Minor Mark-Sweep"
      }
    }
    EventType::IncrementalMinorMarkSweeper => {
      if short_name {
        "mms"
      } else {
        "Incremental Minor Mark-Sweep"
      }
    }
    EventType::Start => {
      if short_name {
        "st"
      } else {
        "Start"
      }
    }
  }
}

/// 事件状态名 (对应 ToString(Event::State); 原版 NOT_RUNNING 返回 nullptr, 即无名,
/// 此处 1:1 以 None 承载, 禁臆造显示名)。
/// v8: src/heap/gc-tracer.cc:72-84 ToString(GCTracer::Event::State)
#[must_use]
pub const fn event_state_to_string(state: EventState) -> Option<&'static str> {
  match state {
    EventState::Marking => Some("Marking"),
    EventState::Atomic => Some("Atomic"),
    EventState::Sweeping => Some("Sweeping"),
    EventState::NotRunning => None,
  }
}

/// isolate 优先级名 (对应 ToString(Priority))。
/// v8: src/heap/gc-tracer.cc ToString(Priority)
#[must_use]
pub const fn priority_to_string(priority: Priority) -> &'static str {
  match priority {
    Priority::UserBlocking => "UserBlocking",
    Priority::UserVisible => "UserVisible",
    Priority::BestEffort => "BestEffort",
  }
}

/// 非空平均速度下限 B/ms (kMinNonEmptySpeedInBytesPerMs)。
/// v8: src/heap/gc-tracer.cc BoundedAverageSpeed
pub const K_MIN_NON_EMPTY_SPEED_IN_BYTES_PER_MS: f64 = 1.0;
/// 速度上限 B/ms (1 GB/ms; kMaxSpeedInBytesPerMs)。
/// v8: src/heap/gc-tracer.cc kMaxSpeedInBytesPerMs
pub const K_MAX_SPEED_IN_BYTES_PER_MS: f64 = 1024.0 * 1024.0 * 1024.0;

/// BoundedAverageSpeed: 环形缓冲均值夹在 [1, GB] 内; 空样本回落 None。
/// v8: src/heap/gc-tracer.cc BoundedAverageSpeed
#[must_use]
pub fn bounded_average_speed(raw: Option<f64>) -> Option<f64> {
  raw.map(|v| {
    v.clamp(
      K_MIN_NON_EMPTY_SPEED_IN_BYTES_PER_MS,
      K_MAX_SPEED_IN_BYTES_PER_MS,
    )
  })
}

/// BoundedThroughput: 平滑吞吐上限 GB/ms。
/// v8: src/heap/gc-tracer.cc BoundedThroughput
#[must_use]
pub const fn bounded_throughput(raw: f64) -> f64 {
  if raw > K_MAX_SPEED_IN_BYTES_PER_MS {
    K_MAX_SPEED_IN_BYTES_PER_MS
  } else {
    raw
  }
}

// —— 编译期不变式 ——
const _: () = {
  // v8::Isolate::Priority 判别值 1:1 (include/v8-isolate.h:698-708, 隐式 0/1/2 升序)
  assert!(Priority::BestEffort as u8 == 0);
  assert!(Priority::UserVisible as u8 == 1);
  assert!(Priority::UserBlocking as u8 == 2);
};
// (f64 边界仅测试; const 上下文中 `==`/cast 不稳定, 见 tests::name_table_tests::bounded_speed)

#[cfg(test)]
mod name_table_tests {
  use super::*;

  #[test]
  fn event_type_names() {
    assert_eq!(
      event_type_to_string(EventType::Scavenger, false),
      "Scavenge"
    );
    assert_eq!(event_type_to_string(EventType::Scavenger, true), "s");
    assert_eq!(
      event_type_to_string(EventType::MarkCompactor, false),
      "Mark-Compact"
    );
    assert_eq!(
      event_type_to_string(EventType::IncrementalMarkCompactor, true),
      "mc"
    );
    assert_eq!(
      event_type_to_string(EventType::IncrementalMinorMarkSweeper, false),
      "Incremental Minor Mark-Sweep"
    );
    assert_eq!(event_type_to_string(EventType::Start, true), "st");
  }

  #[test]
  fn event_state_names() {
    assert_eq!(event_state_to_string(EventState::Marking), Some("Marking"));
    assert_eq!(
      event_state_to_string(EventState::Sweeping),
      Some("Sweeping")
    );
    assert_eq!(event_state_to_string(EventState::Atomic), Some("Atomic"));
    // 原版 NOT_RUNNING → nullptr (无名), 1:1 对应 None
    assert_eq!(event_state_to_string(EventState::NotRunning), None);
  }

  #[test]
  fn priority_names() {
    assert_eq!(priority_to_string(Priority::BestEffort), "BestEffort");
    assert_eq!(priority_to_string(Priority::UserBlocking), "UserBlocking");
    assert_eq!(priority_to_string(Priority::UserVisible), "UserVisible");
  }

  #[test]
  fn bounded_speed() {
    assert_eq!(bounded_average_speed(Some(0.5)), Some(1.0));
    assert_eq!(
      bounded_average_speed(Some(1e12)),
      Some(K_MAX_SPEED_IN_BYTES_PER_MS)
    );
    assert_eq!(bounded_average_speed(None), None);
    assert_eq!(bounded_throughput(2e12), K_MAX_SPEED_IN_BYTES_PER_MS);
    assert_eq!(bounded_throughput(123.0), 123.0);
  }
}
