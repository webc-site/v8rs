// 回归用例 (轮7 W7DBGR 复审 round1 固化)
// 对齐原版 gtest: v8/test/cctest/test-debug.cc (BreakPointInfo / DebugInfo 断点族)
//   + v8/src/objects/debug-objects.h:67 (DebugExecutionMode 值面)
// 覆盖本波新填的 debug_objects 三视图纯逻辑: BreakPointInfo 三态增删、
//   DebugInfo flags 位段、DebugExecutionMode、CoverageInfo 复位、StackFrameInfo 位置。

use aok::{OK, Void};
use v8_debug::debug_objects::{
  BreakPointInfoView, BreakPointsSlot, CoverageInfoSlot, CoverageInfoView, DebugExecutionMode,
  DebugInfoView, K_CAN_BREAK_AT_ENTRY, K_DEBUG_EXECUTION_MODE, K_FLAGS_NONE, K_HAS_BREAK_INFO,
  K_HAS_COVERAGE_INFO, stack_frame_info_get_source_position,
};

// ── DebugExecutionMode 值面 (debug-objects.h:67-68 kSideEffects = kDebugExecutionMode bit5) ──
#[test]
fn test_debug_execution_mode_value() -> Void {
  assert_eq!(DebugExecutionMode::Breakpoints as u8, 0);
  assert_eq!(DebugExecutionMode::SideEffects as u8, 32);
  assert_eq!(
    DebugExecutionMode::SideEffects as u8,
    K_DEBUG_EXECUTION_MODE as u8
  );

  let mut view = DebugInfoView::default();
  assert_eq!(
    view.debug_info_debug_execution_mode(),
    DebugExecutionMode::Breakpoints
  );
  view.debug_info_set_debug_execution_mode(DebugExecutionMode::SideEffects);
  assert_eq!(
    view.debug_info_debug_execution_mode(),
    DebugExecutionMode::SideEffects
  );
  assert_ne!(view.flags, K_FLAGS_NONE);
  view.debug_info_set_debug_execution_mode(DebugExecutionMode::Breakpoints);
  assert_eq!(
    view.debug_info_debug_execution_mode(),
    DebugExecutionMode::Breakpoints
  );
  assert_eq!(view.flags & K_DEBUG_EXECUTION_MODE, 0);
  OK
}

// ── DebugInfo::IsEmpty / HasBreakInfo (debug-objects.cc:19) ──
#[test]
fn test_debug_info_is_empty_and_flags() -> Void {
  let mut view = DebugInfoView::default();
  assert!(view.debug_info_is_empty()); // flags==kNone && hints==0
  view.flags = K_HAS_BREAK_INFO;
  assert!(!view.debug_info_is_empty());
  assert!(view.debug_info_has_break_info());
  view.debug_info_clear_break_info();
  assert!(!view.debug_info_has_break_info());

  // break_at_entry 需 can_break_at_entry 前置 (debug_assert)
  view.flags = K_CAN_BREAK_AT_ENTRY;
  assert!(view.debug_info_can_break_at_entry());
  view.debug_info_set_break_at_entry();
  assert!(view.debug_info_break_at_entry());
  view.debug_info_clear_break_at_entry();
  assert!(!view.debug_info_break_at_entry());

  // coverage info 位
  view.flags |= K_HAS_COVERAGE_INFO;
  assert!(view.debug_info_has_coverage_info());
  view.debug_info_clear_coverage_info();
  assert!(!view.debug_info_has_coverage_info());
  OK
}

// ── BreakPointInfo::SetBreakPoint 三态 (debug-objects.cc:289) ──
#[test]
fn test_break_point_info_set_three_state() -> Void {
  let mut info = BreakPointInfoView::default();
  assert_eq!(info.break_points, BreakPointsSlot::Undefined);

  // undefined → Single
  info.break_point_info_set_break_point(10);
  assert_eq!(info.break_points, BreakPointsSlot::Single(10));
  // Single 同 id → 去重
  info.break_point_info_set_break_point(10);
  assert_eq!(info.break_points, BreakPointsSlot::Single(10));
  // Single 异 id → Fixed[old,new]
  info.break_point_info_set_break_point(20);
  assert_eq!(info.break_points, BreakPointsSlot::Fixed(vec![10, 20]));
  // Fixed 追加, 已存在不重复
  info.break_point_info_set_break_point(20);
  assert_eq!(info.break_points, BreakPointsSlot::Fixed(vec![10, 20]));
  info.break_point_info_set_break_point(30);
  assert_eq!(info.break_point_info_get_break_point_count(), 3);
  assert!(info.break_point_info_has_any_break_point());
  assert!(info.break_point_info_has_break_point(20));
  assert!(!info.break_point_info_has_break_point(99));
  assert_eq!(info.break_point_info_get_break_point_by_id(20), Some(1));
  assert_eq!(info.break_point_info_get_break_point_by_id(99), None);
  OK
}

// ── BreakPointInfo::ClearBreakPoint 三态 (debug-objects.cc:253) ──
#[test]
fn test_break_point_info_clear_three_state() -> Void {
  // Single 命中 → Undefined; 未命中保持
  let mut a = BreakPointInfoView {
    break_points: BreakPointsSlot::Single(5),
    ..Default::default()
  };
  a.break_point_info_clear_break_point(6);
  assert_eq!(a.break_points, BreakPointsSlot::Single(5));
  a.break_point_info_clear_break_point(5);
  assert_eq!(a.break_points, BreakPointsSlot::Undefined);

  // Fixed 去除一项
  let mut b = BreakPointInfoView {
    break_points: BreakPointsSlot::Fixed(vec![1, 2, 3]),
    ..Default::default()
  };
  b.break_point_info_clear_break_point(9);
  assert_eq!(b.break_points, BreakPointsSlot::Fixed(vec![1, 2, 3]));
  b.break_point_info_clear_break_point(2);
  assert_eq!(b.break_points, BreakPointsSlot::Fixed(vec![1, 3]));

  // 注意: 原版去除至剩 1 项仍是长度 1 的 FixedArray, 本层归一化为 Single。
  // 该「表示收敛」为忠实性偏差, 以断言锁定当前行为, 供后续对齐时复核。
  let mut c = BreakPointInfoView {
    break_points: BreakPointsSlot::Fixed(vec![7, 8]),
    ..Default::default()
  };
  c.break_point_info_clear_break_point(8);
  assert_eq!(c.break_points, BreakPointsSlot::Single(7));
  // 去除至空
  let mut d = BreakPointInfoView {
    break_points: BreakPointsSlot::Fixed(vec![9]),
    ..Default::default()
  };
  d.break_point_info_clear_break_point(9);
  assert_eq!(d.break_points, BreakPointsSlot::Undefined);
  assert!(!d.break_point_info_has_any_break_point());
  assert!(!d.break_point_info_has_break_point(9));
  OK
}

// ── DebugInfo 通过注入闭包委托 BreakPointInfo (debug-objects.cc:122/138) ──
#[test]
fn test_debug_info_delegates_break_point() -> Void {
  let mut info = BreakPointInfoView::default();
  info.break_point_info_set_break_point(1);
  // 上层 DebugInfo 持有 break_info 位后委托清单点
  let mut dbg = DebugInfoView {
    flags: K_HAS_BREAK_INFO,
    ..Default::default()
  };
  dbg.debug_info_clear_break_point(&mut info, 1);
  assert_eq!(info.break_points, BreakPointsSlot::Undefined);
  assert_eq!(info.break_point_info_get_break_point_count(), 0);
  assert_eq!(dbg.debug_info_get_break_point_count([&info]), 0);
  info.break_point_info_set_break_point(42);
  assert_eq!(dbg.debug_info_get_break_point_count([&info]), 1);
  // HasBreakPoint 以闭包注入外部查找
  let has = dbg.debug_info_has_break_point(|| true);
  assert!(has);
  OK
}

// ── CoverageInfo::ResetBlockCount (debug-objects.cc) ──
#[test]
fn test_coverage_info_reset_and_print() -> Void {
  let mut cov = CoverageInfoView {
    slots: vec![
      CoverageInfoSlot {
        start_source_position: 0,
        end_source_position: 5,
        block_count: 3,
        padding: 0,
      },
      CoverageInfoSlot {
        start_source_position: 6,
        end_source_position: 9,
        block_count: 7,
        padding: 0,
      },
    ],
  };
  cov.coverage_info_reset_block_count();
  assert!(cov.slots.iter().all(|s| s.block_count == 0));
  let text = cov.coverage_info_coverage_info_print();
  assert!(text.starts_with("CoverageInfo {"));
  assert!(text.contains("slot[1]"));
  OK
}

// ── StackFrameInfo::GetSourcePosition (debug-objects.cc) ──
#[test]
fn test_stack_frame_info_source_position() -> Void {
  assert_eq!(stack_frame_info_get_source_position(true, 128), 128);
  assert_eq!(stack_frame_info_get_source_position(false, 128), -1);
  OK
}
