// 由 tool/src/heap/base/stack.js 从 v8/src/heap/base/stack.h + stack.cc 生成; 修订就地改, 复跑覆盖
// 纯栈抽象: 地址区间 + 字对齐迭代; ASAN/SafeStack/MTE 分支随目标配置裁剪

//! Stack: 栈帧地址区间抽象与保守迭代。
//! 栈向下生长: start = 最高地址, top/marker = 最低地址 (stack.h:27-31)。
//!
//! 推迟登记:
//! - PushAllRegistersAndIterateStack (asm/ 目录): Rust 侧用当前栈指针向下字对齐近似 stack_end;
//! - ASAN fake stack / SafeStack (V8_USE_ADDRESS_SANITIZER / V8_USE_SAFE_STACK): 目标配置未启用;
//! - Isolate 绑定: 仅保留纯栈抽象。

use std::ptr;

use gxhash::HashMap;
use v8_base_platform::{Mutex, Stack as PlatformStack};

use crate::{
  error::{Error, Result},
  memory_tagging::SuspendTagCheckingScope,
};

/// 栈最小对齐: sizeof(void*) (stack.cc:134 机械提取)。
const K_MIN_STACK_ALIGNMENT: usize = size_of::<usize>();

/// 原 StackVisitor (stack.h:17-21): 槽值访问协议。
pub trait StackVisitor {
  fn visit_pointer(&mut self, address: usize);
}

/// 原 StackVisitorCallback (stack.h:23)。
pub type StackVisitorCallback = fn(&mut dyn StackVisitor);

/// 原 Stack::Segment (stack.h:123-162): start >= address >= top 视为在段内。
#[derive(Clone, Copy)]
pub struct Segment {
  pub start: usize,
  /// 低地址边界; 0 表示未设置 (对应原版 nullptr)。
  pub top: usize,
}

impl Segment {
  /// 原 Segment(const void* stack_start, const void* stack_top) (stack.h:146-155)。
  pub const fn new(start: usize, top: usize) -> Self {
    Self { start, top }
  }

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

/// 当前线程栈端近似: 栈指针向下对齐 (保守缩小扫描区, 原版由 asm 压寄存器后给出)。
fn current_stack_top_aligned() -> usize {
  PlatformStack::get_current_stack_position().value() & !(K_MIN_STACK_ALIGNMENT - 1)
}

impl Stack {
  pub fn new() -> Self {
    Self {
      current_segment: Segment::new(0, 0),
      lock: Mutex::new(),
      background_stacks: HashMap::default(),
      scan_simulator_callback: None,
    }
  }

  /// 原 SetStackStart (stack.h:38-46)。
  pub fn set_stack_start(&mut self, stack_start: usize) {
    self.current_segment.start = stack_start;
  }

  /// 原 stack_start (stack.h:48)。
  pub const fn stack_start(&self) -> usize {
    self.current_segment.start
  }

  /// 原 IsOnStack (stack.cc:24-42, static): 忽略已设 start/marker, 仅判当前线程栈区间。
  pub fn is_on_stack(slot: usize) -> bool {
    // ASAN fake stack / SafeStack 分支按目标配置裁剪。
    let top = PlatformStack::get_current_stack_position().value();
    let start = PlatformStack::get_stack_start().value();
    top <= slot && slot <= start
  }

  /// 原 IteratePointersUntilMarker (stack.cc:164-166)。
  pub fn iterate_pointers_until_marker(&self, visitor: &mut dyn StackVisitor) -> Result<()> {
    self.iterate_pointers_in_segment(visitor, self.current_segment)
  }

  /// 原 IteratePointersFromAddressUntilMarker (stack.cc:168-174)。
  pub fn iterate_pointers_from_address_until_marker(
    &self,
    visitor: &mut dyn StackVisitor,
    address: usize,
  ) -> Result<()> {
    debug_assert!(address != 0);
    debug_assert!(self.current_segment.contains(address));
    let segment = Segment::new(address, self.current_segment.top);
    self.iterate_pointers_in_segment(visitor, segment)
  }

  /// 原 IterateBackgroundStacks (stack.cc:189-204)。
  pub fn iterate_background_stacks(&self, visitor: &mut dyn StackVisitor) -> Result<()> {
    // 快照段集合 (原版依赖 safepoint 无锁读; Rust 侧短临界区拷贝等价承载)。
    let segments: Vec<Segment> = {
      let _g = self.lock.lock_guard();
      self.background_stacks.values().copied().collect()
    };
    // MTE 挂起域: 扫栈期间停止标签检查 (非 linux+arm64 为 no-op)。
    let _mte = SuspendTagCheckingScope::new();
    for segment in segments {
      self.iterate_words(visitor, segment)?;
    }
    Ok(())
  }

  /// 原 SetMarkerAndCallback (stack.h:69-73)。
  /// 偏差: PushAllRegisters (asm/) 推迟, 以当前栈指针近似 stack_end。
  pub fn set_marker_and_callback(&mut self, callback: impl FnOnce()) {
    let previous_segment = self.current_segment;
    self.current_segment.top = current_stack_top_aligned();
    callback();
    self.current_segment = previous_segment;
  }

  /// 原 SetMarkerIfNeededAndCallback (stack.h:75-84)。
  pub fn set_marker_if_needed_and_callback(&mut self, callback: impl FnOnce()) {
    if !self.is_marker_set() {
      self.set_marker_and_callback(callback);
    } else {
      debug_assert!(is_on_current_stack(self.current_segment.top));
      callback();
    }
  }

  /// 原 SetMarkerForBackgroundThreadAndCallback (stack.h:88-95) +
  /// SetMarkerForBackgroundThreadAndCallbackImpl (stack.h:186-219):
  /// 前段压后台栈 → 回调 → 有先前段则复原, 否则注销。
  pub fn set_marker_for_background_thread_and_callback(
    &mut self,
    thread: ThreadId,
    callback: impl FnOnce(),
  ) {
    let previous_segment = self
      .background_stacks
      .get(&thread)
      .copied()
      .unwrap_or(Segment::new(0, 0));
    self.background_stacks.insert(
      thread,
      Segment::new(
        PlatformStack::get_stack_start().value(),
        current_stack_top_aligned(),
      ),
    );
    callback();
    if previous_segment.top != 0 {
      self.background_stacks.insert(thread, previous_segment);
    } else {
      self.background_stacks.remove(&thread);
    }
  }

  /// 原 IteratePointersForTesting (stack.cc:157-162):
  /// 设 marker 后扫当前段 + 后台段, 结束复原 marker。
  pub fn iterate_pointers_for_testing(&mut self, visitor: &mut dyn StackVisitor) -> Result<()> {
    let previous_segment = self.current_segment;
    self.current_segment.top = current_stack_top_aligned();
    let result = self
      .iterate_pointers_until_marker(visitor)
      .and_then(|_| self.iterate_background_stacks(visitor));
    self.current_segment = previous_segment;
    result
  }

  /// 原 IsMarkerSet (stack.h:105)。
  pub const fn is_marker_set(&self) -> bool {
    self.current_segment.top != 0
  }

  /// 原 IsMarkerSetForBackgroundThread (stack.h:106-112)。
  pub fn is_marker_set_for_background_thread(&self, thread: ThreadId) -> bool {
    let _g = self.lock.lock_guard();
    match self.background_stacks.get(&thread) {
      None => false,
      Some(segment) => {
        debug_assert!(segment.top != 0);
        true
      }
    }
  }

  /// 原 HasBackgroundStacks (stack.h:116): 仅 safepoint 内安全。
  pub fn has_background_stacks(&self) -> bool {
    let _g = self.lock.lock_guard();
    !self.background_stacks.is_empty()
  }

  /// 原 SetScanSimulatorCallback (stack.h:118-120)。
  pub fn set_scan_simulator_callback(&mut self, callback: StackVisitorCallback) {
    self.scan_simulator_callback = Some(callback);
  }

  /// 原 IteratePointersInSegment (stack.cc:176-187, private):
  /// MTE 挂起 + 逐字迭代 + 扫描模拟回调。
  fn iterate_pointers_in_segment(
    &self,
    visitor: &mut dyn StackVisitor,
    segment: Segment,
  ) -> Result<()> {
    let _mte = SuspendTagCheckingScope::new();
    self.iterate_words(visitor, segment)?;
    if let Some(callback) = self.scan_simulator_callback {
      callback(visitor);
    }
    Ok(())
  }

  /// 原 IteratePointersInStack (stack.cc:129-152): CHECK 族 → Err。
  fn iterate_words(&self, visitor: &mut dyn StackVisitor, segment: Segment) -> Result<()> {
    if segment.top == 0 {
      return Err(Error::InvalidStackSegment { what: "top 空" });
    }
    if segment.start == 0 {
      return Err(Error::InvalidStackSegment { what: "start 空" });
    }
    if segment.start < segment.top {
      return Err(Error::InvalidStackSegment {
        what: "start < top",
      });
    }
    if segment.top & (K_MIN_STACK_ALIGNMENT - 1) != 0 {
      return Err(Error::InvalidStackSegment {
        what: "top 未按字对齐",
      });
    }
    if segment.start & (K_MIN_STACK_ALIGNMENT - 1) != 0 {
      return Err(Error::InvalidStackSegment {
        what: "start 未按字对齐",
      });
    }
    let mut current = segment.top;
    while current < segment.start {
      // SAFETY: 上方校验已证 current 字对齐且落在 [top, start) 栈区间内。
      let address = unsafe { ptr::read(current as *const usize) };
      if address != 0 {
        visitor.visit_pointer(address);
      }
      current += K_MIN_STACK_ALIGNMENT;
    }
    Ok(())
  }
}

impl Default for Stack {
  fn default() -> Self {
    Self::new()
  }
}

/// 原 IsOnCurrentStack (stack.cc:172-179, DEBUG 辅助): 提升为常驻 API
/// (供 marker 校验与测试断言复用)。
pub fn is_on_current_stack(ptr: usize) -> bool {
  debug_assert!(ptr != 0);
  let start = PlatformStack::get_stack_start().value();
  let top = PlatformStack::get_current_stack_position().value();
  ptr <= start && ptr >= top
}
