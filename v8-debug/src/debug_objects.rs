// 由 tool/src/debug/debug-objects.js 从 v8/src/objects/debug-objects.{-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// 席位表差集: tq 布局/bitfield + inl 访问器 + cc 签名骨架 (位链数值归既有 debug-objects.h.js)
//! 调试对象族: BreakPoint/BreakPointInfo/DebugInfo/CoverageInfo/StackFrameInfo/
//! ErrorStackData/DebugScriptScopeInfo (debug 域)。
//!
//! 位链数值面 (掩码/偏移) 归既有 debug-objects.h.js 产物; 本文件登记 tq 布局与 .cc 方法实现。

use std::mem::size_of;

/// 原版 DebugInfo::ExecutionMode (debug-objects.h:67)。
/// kSideEffects 复用 kDebugExecutionMode 位 (bit 5, 值 32)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DebugExecutionMode {
  /// kBreakpoints = 0
  Breakpoints = 0,
  /// kSideEffects = kDebugExecutionMode (bit 5)
  SideEffects = 32,
}

// ── tq extern class 布局表 (源码序) ──
pub const BREAKPOINT_FIELDS: [(&str, &str); 2] = [("id", "Smi"), ("condition", "String")];
pub const BREAKPOINTINFO_FIELDS: [(&str, &str); 2] = [
  ("source_position", "Smi"),
  ("break_points", "FixedArray|BreakPoint|Undefined"),
];
pub const DEBUGINFO_FIELDS: [(&str, &str); 7] = [
  ("shared", "SharedFunctionInfo"),
  ("debugger_hints", "SmiTagged<DebuggerHints>"),
  ("break_points", "FixedArray"),
  ("flags", "SmiTagged<DebugInfoFlags>"),
  ("coverage_info", "CoverageInfo|Undefined"),
  ("original_bytecode_array", "ProtectedPointer<BytecodeArray>"),
  ("debug_bytecode_array", "ProtectedPointer<BytecodeArray>"),
];
pub const COVERAGEINFO_FIELDS: [(&str, &str); 2] = [
  ("slot_count", "int32"),
  ("slots[slot_count]", "CoverageInfoSlot"),
];
pub const STACKFRAMEINFO_FIELDS: [(&str, &str); 4] = [
  ("shared_or_script", "SharedFunctionInfo|Script"),
  ("function_name", "String"),
  ("flags", "SmiTagged<StackFrameInfoFlags>"),
  ("bytecode_offset_or_source_position", "Smi"),
];
pub const STACKTRACEINFO_FIELDS: [(&str, &str); 2] = [("id", "Smi"), ("frames", "FixedArray")];
pub const ERRORSTACKDATA_FIELDS: [(&str, &str); 2] = [
  (
    "raw_data_for_call_site_infos_or_formatted_stack",
    "FixedArray|JSAny",
  ),
  ("stack_trace", "StackTraceInfo"),
];
pub const DEBUGSCRIPTSCOPEINFO_FIELDS: [(&str, &str); 2] = [
  ("numeric_data", "ByteArray"),
  ("string_table", "FixedArray"),
];

// ── tq bitfield struct (位段名/类型/位宽, 源码序) ──
pub const DEBUG_INFO_FLAGS_BITS: [(&str, &str, u32); 6] = [
  ("has_break_info", "bool", 1),
  ("prepared_for_debug_execution", "bool", 1),
  ("has_coverage_info", "bool", 1),
  ("break_at_entry", "bool", 1),
  ("can_break_at_entry", "bool", 1),
  ("debug_execution_mode", "bool", 1),
];
pub const DEBUGGER_HINTS_BITS: [(&str, &str, u32); 4] = [
  ("side_effect_state", "int32", 2),
  ("debug_is_blackboxed", "bool", 1),
  ("computed_debug_is_blackboxed", "bool", 1),
  ("debugging_id", "int32", 20),
];
pub const STACK_FRAME_INFO_FLAGS_BITS: [(&str, &str, u32); 1] = [("is_constructor", "bool", 1)];

// ── DebugInfoFlags 位掩码 (源码位序: bit 0..5) ──
pub const K_HAS_BREAK_INFO: u32 = 1 << 0;
pub const K_PREPARED_FOR_DEBUG_EXECUTION: u32 = 1 << 1;
pub const K_HAS_COVERAGE_INFO: u32 = 1 << 2;
pub const K_BREAK_AT_ENTRY: u32 = 1 << 3;
pub const K_CAN_BREAK_AT_ENTRY: u32 = 1 << 4;
pub const K_DEBUG_EXECUTION_MODE: u32 = 1 << 5;
pub const K_FLAGS_NONE: u32 = 0;

// ── 原版 DebugInfo 值面 (flags + debugger_hints + coverage_info 存在位) ──
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DebugInfoView {
  /// `flags()` (DebugInfoFlags)
  pub flags: u32,
  /// `debugger_hints()` (DebuggerHints 位域, 原版整体 32 位)
  pub debugger_hints: u32,
  /// `HasInstrumentedBytecodeArray()` 判定投影 (原基于 debug_bytecode_array != undefined)
  pub has_instrumented_bytecode: bool,
}

impl DebugInfoView {
  /// v8: src/objects/debug-objects.cc DebugInfo::IsEmpty
  /// — flags 位段与 debugger_hints 全零即空。
  #[must_use]
  pub fn debug_info_is_empty(&self) -> bool {
    self.flags == K_FLAGS_NONE && self.debugger_hints == 0
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::HasBreakInfo
  #[must_use]
  pub fn debug_info_has_break_info(&self) -> bool {
    self.flags & K_HAS_BREAK_INFO != 0
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::DebugExecutionMode
  #[must_use]
  pub fn debug_info_debug_execution_mode(&self) -> DebugExecutionMode {
    if self.flags & K_DEBUG_EXECUTION_MODE != 0 {
      DebugExecutionMode::SideEffects
    } else {
      DebugExecutionMode::Breakpoints
    }
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::SetDebugExecutionMode
  pub fn debug_info_set_debug_execution_mode(&mut self, mode: DebugExecutionMode) {
    if mode == DebugExecutionMode::SideEffects {
      self.flags |= K_DEBUG_EXECUTION_MODE;
    } else {
      self.flags &= !K_DEBUG_EXECUTION_MODE;
    }
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::ClearBreakInfo
  /// — 若已 instrumented bytecode 则由上层 RedirectActiveFunctions 处理 (推迟),
  /// 本层仅复位 has_break_info 位并将 break_points 表清空。
  pub fn debug_info_clear_break_info(&mut self) {
    self.flags &= !K_HAS_BREAK_INFO;
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::SetBreakAtEntry
  /// — DCHECK(CanBreakAtEntry()) 前置; 置 break_at_entry 位。
  pub fn debug_info_set_break_at_entry(&mut self) {
    debug_assert!(self.debug_info_can_break_at_entry());
    self.flags |= K_BREAK_AT_ENTRY;
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::ClearBreakAtEntry
  pub fn debug_info_clear_break_at_entry(&mut self) {
    debug_assert!(self.debug_info_can_break_at_entry());
    self.flags &= !K_BREAK_AT_ENTRY;
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::BreakAtEntry
  #[must_use]
  pub fn debug_info_break_at_entry(&self) -> bool {
    self.flags & K_BREAK_AT_ENTRY != 0
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::CanBreakAtEntry
  #[must_use]
  pub fn debug_info_can_break_at_entry(&self) -> bool {
    self.flags & K_CAN_BREAK_AT_ENTRY != 0
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::HasCoverageInfo
  #[must_use]
  pub fn debug_info_has_coverage_info(&self) -> bool {
    self.flags & K_HAS_COVERAGE_INFO != 0
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::ClearCoverageInfo
  /// — 若已 HasCoverageInfo, 清 coverage_info 字段与 has_coverage_info 位;
  /// 原版 undefined_value 写入由上层持有者表达。
  pub fn debug_info_clear_coverage_info(&mut self) {
    if self.debug_info_has_coverage_info() {
      self.flags &= !K_HAS_COVERAGE_INFO;
    }
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::HasBreakPoint
  /// — 判定 source_position 是否有 break point; 需要 break_points FixedArray 视图,
  /// 本层通过 `break_info_at` 参数注入外部查找结果。
  #[must_use]
  pub fn debug_info_has_break_point(&self, break_info_at: impl FnOnce() -> bool) -> bool {
    debug_assert!(self.debug_info_has_break_info());
    break_info_at()
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::GetBreakPointInfo
  /// — 从 break_points FixedArray 中按 source_position 查找;
  /// heap-agnostic 层以 `lookup` 闭包注入 (原版 FixedArray::get + IsUndefined)。
  #[must_use]
  pub fn debug_info_get_break_point_info(
    &self,
    lookup: impl FnOnce() -> Option<BreakPointInfoView>,
  ) -> Option<BreakPointInfoView> {
    lookup()
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::ClearBreakPoint
  /// — 若 HasBreakInfo, 定位 BreakPointInfo 后清对应 BreakPoint。
  pub fn debug_info_clear_break_point(
    &mut self,
    info: &mut BreakPointInfoView,
    break_point_id: i32,
  ) {
    debug_assert!(self.debug_info_has_break_info());
    info.break_point_info_clear_break_point(break_point_id);
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::SetBreakPoint
  /// — 若 HasBreakInfo, 定位 BreakPointInfo 后加入 BreakPoint。
  /// 原版若 info 不存在则新建 (需 Factory), 本层通过闭包注入。
  pub fn debug_info_set_break_point(&mut self, info: &mut BreakPointInfoView, break_point_id: i32) {
    debug_assert!(self.debug_info_has_break_info());
    info.break_point_info_set_break_point(break_point_id);
  }

  /// v8: src/objects/debug-objects.cc DebugInfo::GetBreakPointCount
  /// — 统计 debug_info 中所有 BreakPointInfo 的断点总数。
  #[must_use]
  pub fn debug_info_get_break_point_count<'a>(
    &self,
    break_point_infos: impl IntoIterator<Item = &'a BreakPointInfoView>,
  ) -> usize {
    debug_assert!(self.debug_info_has_break_info());
    break_point_infos
      .into_iter()
      .map(|info| info.break_point_info_get_break_point_count())
      .sum()
  }
}

// ── BreakPointInfo 值面 ──
/// 原版 break_points 槽: undefined / 单 BreakPoint / FixedArray 三态。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum BreakPointsSlot {
  #[default]
  Undefined,
  Single(i32),
  Fixed(Vec<i32>),
}

/// 原版 BreakPointInfo (source_position + break_points)。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BreakPointInfoView {
  /// `source_position()` (Smi)
  pub source_position: i32,
  /// `break_points()` 三态
  pub break_points: BreakPointsSlot,
}

impl BreakPointInfoView {
  /// v8: src/objects/debug-objects.cc BreakPointInfo::ClearBreakPoint
  /// — 三态: undefined → no-op; 单点同 id → 清 undefined; FixedArray → 收缩去除。
  pub fn break_point_info_clear_break_point(&mut self, break_point_id: i32) {
    match &mut self.break_points {
      BreakPointsSlot::Undefined => {}
      BreakPointsSlot::Single(id) => {
        if *id == break_point_id {
          self.break_points = BreakPointsSlot::Undefined;
        }
      }
      BreakPointsSlot::Fixed(vec) => {
        if let Some(pos) = vec.iter().position(|&x| x == break_point_id) {
          vec.remove(pos);
          if vec.len() == 1 {
            self.break_points = BreakPointsSlot::Single(vec[0]);
          } else if vec.is_empty() {
            self.break_points = BreakPointsSlot::Undefined;
          }
        }
      }
    }
  }

  /// v8: src/objects/debug-objects.cc BreakPointInfo::SetBreakPoint
  /// — 三态扩展: undefined → Single; Single(different) → Fixed[old,new];
  /// Fixed 追加 (先检查重复)。
  pub fn break_point_info_set_break_point(&mut self, break_point_id: i32) {
    match &mut self.break_points {
      BreakPointsSlot::Undefined => {
        self.break_points = BreakPointsSlot::Single(break_point_id);
      }
      BreakPointsSlot::Single(id) => {
        if *id != break_point_id {
          self.break_points = BreakPointsSlot::Fixed(vec![*id, break_point_id]);
        }
      }
      BreakPointsSlot::Fixed(vec) => {
        if !vec.contains(&break_point_id) {
          vec.push(break_point_id);
        }
      }
    }
  }

  /// v8: src/objects/debug-objects.cc BreakPointInfo::HasBreakPoint
  /// — 判定指定 id 的断点是否存在 (原版按 BreakPoint::id() 比较)。
  #[must_use]
  pub fn break_point_info_has_break_point(&self, break_point_id: i32) -> bool {
    match &self.break_points {
      BreakPointsSlot::Undefined => false,
      BreakPointsSlot::Single(id) => *id == break_point_id,
      BreakPointsSlot::Fixed(v) => v.contains(&break_point_id),
    }
  }

  /// 检查是否包含任意断点 (原版 !IsUndefined 检查)。
  #[must_use]
  pub fn break_point_info_has_any_break_point(&self) -> bool {
    match &self.break_points {
      BreakPointsSlot::Undefined => false,
      BreakPointsSlot::Single(_) => true,
      BreakPointsSlot::Fixed(v) => !v.is_empty(),
    }
  }

  /// v8: src/objects/debug-objects.cc BreakPointInfo::GetBreakPointById
  /// — 命中返回对应 id 的索引 (`Some`); 未命中 `None`。
  #[must_use]
  pub fn break_point_info_get_break_point_by_id(&self, id: i32) -> Option<usize> {
    match &self.break_points {
      BreakPointsSlot::Undefined => None,
      BreakPointsSlot::Single(x) if *x == id => Some(0),
      BreakPointsSlot::Single(_) => None,
      BreakPointsSlot::Fixed(v) => v.iter().position(|&x| x == id),
    }
  }

  /// v8: src/objects/debug-objects.cc BreakPointInfo::GetBreakPointCount
  /// — 统计断点总数: undefined 为 0; Single 为 1; Fixed 为 array.len()。
  #[must_use]
  pub fn break_point_info_get_break_point_count(&self) -> usize {
    match &self.break_points {
      BreakPointsSlot::Undefined => 0,
      BreakPointsSlot::Single(_) => 1,
      BreakPointsSlot::Fixed(v) => v.len(),
    }
  }
}

// ── CoverageInfoSlot (tq @export struct, 4 × int32) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct CoverageInfoSlot {
  /// start_source_position
  pub start_source_position: i32,
  /// end_source_position
  pub end_source_position: i32,
  /// block_count
  pub block_count: i32,
  /// padding (凑齐 4 槽)
  pub padding: i32,
}

/// 原版 CoverageInfo (slot_count + slots[])。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CoverageInfoView {
  pub slots: Vec<CoverageInfoSlot>,
}

impl CoverageInfoView {
  /// v8: src/objects/debug-objects.cc CoverageInfo::ResetBlockCount
  /// — 将所有 slot 的 block_count 复位为 0。
  pub fn coverage_info_reset_block_count(&mut self) {
    for slot in &mut self.slots {
      slot.block_count = 0;
    }
  }

  /// v8: src/objects/debug-objects.cc CoverageInfo::CoverageInfoPrint
  /// — 原版 stderr 打印; Rust 层返回可读字符串。
  #[must_use]
  pub fn coverage_info_coverage_info_print(&self) -> String {
    let mut s = String::from("CoverageInfo {\n");
    for (i, slot) in self.slots.iter().enumerate() {
      s.push_str(&format!(
        "  slot[{}]: start={}, end={}, count={}\n",
        i, slot.start_source_position, slot.end_source_position, slot.block_count
      ));
    }
    s.push('}');
    s
  }
}

// ── StackFrameInfo (tq extern class) ──
/// 原版 StackFrameInfo::GetSourcePosition — 仅当 shared_or_script 为 Script 时返回
/// bytecode_offset_or_source_position, 否则返回 -1 (`kNoSourcePosition`)。
#[must_use]
pub fn stack_frame_info_get_source_position(
  is_script: bool,
  bytecode_offset_or_source_position: i32,
) -> i32 {
  if is_script {
    bytecode_offset_or_source_position
  } else {
    -1
  }
}

// ── 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较) ──
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(BREAKPOINT_FIELDS.len() == 2);
  assert!(DEBUGINFO_FIELDS.len() == 7);
  assert!(str_eq(DEBUGINFO_FIELDS[6].0, "debug_bytecode_array"));
  assert!(str_eq(COVERAGEINFO_FIELDS[1].0, "slots[slot_count]"));
  assert!(DEBUG_INFO_FLAGS_BITS.len() == 6);
  assert!(DEBUGGER_HINTS_BITS.len() == 4);
  assert!(DEBUGGER_HINTS_BITS[3].2 == 20);
  assert!(STACK_FRAME_INFO_FLAGS_BITS.len() == 1);
  assert!(size_of::<CoverageInfoSlot>() == 16);
};
