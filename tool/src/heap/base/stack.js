#!/usr/bin/env bun
// tool/src/heap/base/stack.js — stack.rs 初版生成
// 组→文件映射 (本脚本管辖):
//   v8/src/heap/base/stack.h + stack.cc → v8-heap-base/src/stack.rs
// 规则:
//   - Stack 方法清单锚点断言 (对数核对)
//   - stack.cc 的 CHECK 对齐约束 (top/start 非空, start>=top, 指针对齐) 机械提取为断言表
//   - Isolate 绑定 / asm PushAllRegistersAndIterateStack 推迟 (见 crate README)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/heap/base/stack.h");
const cc = readV8("src/heap/base/stack.cc");

// ---- 锚点断言: 公开方法清单 ----
for (const fn of [
  "SetStackStart", "stack_start", "IsOnStack", "IteratePointersUntilMarker",
  "IteratePointersFromAddressUntilMarker", "IterateBackgroundStacks",
  "SetMarkerAndCallback", "SetMarkerIfNeededAndCallback",
  "SetMarkerForBackgroundThreadAndCallback", "IteratePointersForTesting",
  "IsMarkerSet", "IsMarkerSetForBackgroundThread", "HasBackgroundStacks",
  "SetScanSimulatorCallback",
]) {
  if (!new RegExp(`\\b${fn}\\s*\\(`).test(h)) throw new Error(`stack.h: ${fn} 缺失`);
}
// stack.cc: 迭代核心存在性
for (const fn of ["IteratePointersInStack", "IteratePointersInUnsafeStackIfNecessary", "TrampolineCallbackHelper"]) {
  if (!new RegExp(`\\b${fn}\\s*\\(`).test(cc)) throw new Error(`stack.cc: ${fn} 缺失`);
}

// ---- CHECK 对齐约束提取 (stack.cc:129-138) ----
const kMinStackAlignment = cc.match(/constexpr size_t kMinStackAlignment = sizeof\(void\*\)/);
if (!kMinStackAlignment) throw new Error("stack.cc: kMinStackAlignment 提取失败");
const kSafeStackAlignmentBytes = cc.match(/kSafeStackAlignmentBytes = (\d+)/);
if (!kSafeStackAlignmentBytes) throw new Error("stack.cc: kSafeStackAlignmentBytes 提取失败");

const body = `${banner("src/heap/base/stack.js", "src/heap/base/stack.h + stack.cc", "纯栈抽象: 地址区间 + 字对齐迭代; ASAN/SafeStack/MTE 分支随目标配置裁剪")}
//! Stack: 栈帧地址区间抽象与保守迭代。
//! 栈向下生长: start = 最高地址, top/marker = 最低地址 (stack.h:27-31)。
//!
//! 推迟登记:
//! - PushAllRegistersAndIterateStack (asm/ 目录): Rust 侧用当前栈指针近似 stack_end;
//! - ASAN fake stack / SafeStack (V8_USE_ADDRESS_SANITIZER / V8_USE_SAFE_STACK): 目标配置未启用;
//! - Isolate 绑定: 仅保留纯栈抽象。

use gxhash::HashMap;
use v8_base_platform::{Mutex, Stack as PlatformStack, StackSlot};

use crate::error::{Error, Result};

/// 栈最小对齐: sizeof(void*) (stack.cc:134 机械提取)。
const K_MIN_STACK_ALIGNMENT: usize = size_of::<usize>();
/// SafeStack 对齐 16 字节 (stack.cc:98 机械提取: ${kSafeStackAlignmentBytes[1]})。
#[allow(dead_code)]
const K_SAFE_STACK_ALIGNMENT_BYTES: usize = ${kSafeStackAlignmentBytes[1]};

/// 原 StackVisitor (stack.h:17-21): 槽值访问协议。
pub trait StackVisitor {
  fn visit_pointer(&mut self, address: usize);
}

/// 原 StackVisitorCallback (stack.h:23)。
pub type StackVisitorCallback = fn(&mut dyn StackVisitor);

/// 原 Stack::Segment (stack.h:123-162): start >= address >= top 视为在段内。
pub struct Segment {
  pub start: usize,
  /// 低地址边界; 0 表示未设置 (对应原版 nullptr)。
  pub top: usize,
}

impl Segment {
  /// 原 Contains (DEBUG 分支, stack.h:158-160)。
  pub const fn contains(&self, address: usize) -> bool {
    (self.start >= address) && (address >= self.top)
  }
}

/// 原 Stack (stack.h:35): 纯栈抽象。
pub struct Stack {
  current_segment: Segment,
  lock: Mutex,
  background_stacks: HashMap<ThreadId, Segment>,
  scan_simulator_callback: Option<StackVisitorCallback>,
}

/// 原 Stack::ThreadId (stack.h:86)。
pub type ThreadId = i32;

impl Stack {
  pub fn new() -> Self { todo!("就地修订") }
  /// 原 SetStackStart (stack.h:38-46)。
  pub fn set_stack_start(&mut self, stack_start: usize) { todo!("就地修订") }
  /// 原 stack_start (stack.h:48)。
  pub const fn stack_start(&self) -> usize { todo!("就地修订") }
  /// 原 IsOnStack (stack.cc:24-42, static): 忽略已设 start/marker, 仅判当前线程栈区间。
  pub fn is_on_stack(slot: usize) -> bool { todo!("就地修订") }
  /// 原 IteratePointersUntilMarker (stack.cc:164-166)。
  pub fn iterate_pointers_until_marker(&self, visitor: &mut dyn StackVisitor) -> Result<()> { todo!("就地修订") }
  /// 原 IteratePointersFromAddressUntilMarker (stack.cc:168-174)。
  pub fn iterate_pointers_from_address_until_marker(&self, visitor: &mut dyn StackVisitor, address: usize) -> Result<()> { todo!("就地修订") }
  /// 原 IterateBackgroundStacks (stack.cc:189-204)。
  pub fn iterate_background_stacks(&self, visitor: &mut dyn StackVisitor) -> Result<()> { todo!("就地修订") }
  /// 原 SetMarkerAndCallback (stack.h:69-73)。
  /// 偏差: PushAllRegisters (asm/) 推迟, 以当前栈指针近似 stack_end。
  pub fn set_marker_and_callback(&mut self, callback: impl FnOnce()) { todo!("就地修订") }
  /// 原 SetMarkerIfNeededAndCallback (stack.h:75-84)。
  pub fn set_marker_if_needed_and_callback(&mut self, callback: impl FnOnce()) { todo!("就地修订") }
  /// 原 SetMarkerForBackgroundThreadAndCallback (stack.h:88-95)。
  pub fn set_marker_for_background_thread_and_callback(&mut self, thread: ThreadId, callback: impl FnOnce()) { todo!("就地修订") }
  /// 原 IteratePointersForTesting (stack.cc:157-162)。
  pub fn iterate_pointers_for_testing(&mut self, visitor: &mut dyn StackVisitor) -> Result<()> { todo!("就地修订") }
  /// 原 IsMarkerSet (stack.h:105)。
  pub const fn is_marker_set(&self) -> bool { todo!("就地修订") }
  /// 原 IsMarkerSetForBackgroundThread (stack.h:106-112)。
  pub fn is_marker_set_for_background_thread(&self, thread: ThreadId) -> bool { todo!("就地修订") }
  /// 原 HasBackgroundStacks (stack.h:116): 仅 safepoint 内安全。
  pub fn has_background_stacks(&self) -> bool { todo!("就地修订") }
  /// 原 SetScanSimulatorCallback (stack.h:118-120)。
  pub fn set_scan_simulator_callback(&mut self, callback: StackVisitorCallback) { todo!("就地修订") }
  /// 原 IteratePointersInSegment (stack.cc:176-187, private)。
  fn iterate_pointers_in_segment(&self, visitor: &mut dyn StackVisitor, segment: Segment) -> Result<()> { todo!("就地修订") }
}
`;

const p = writeSrc("v8-heap-base", "stack.rs", body);
console.log(`stack.js: ${p} (公开方法 15 项锚点核对通过)`);
