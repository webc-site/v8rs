// Sweeper 语义面 (纯规则子集)。
//
// 族: v8/src/heap/sweeper.{h,cc} + src/common/globals.h (空间计数)
// 分工: Sweeper 内部枚举 (FreeSpaceTreatmentMode/SweepingMode/SweepingScope/
//       IncludeMainThread/SafepointKind) 归 sweeper_enums.rs; 本模块挂载 ZappingMode
//       与 ShouldZapDeadObjectsOnPage 1:1 转写、kZapTagged、清扫空间/并发度纯式。
//       Sweeper/SweepingState/ConcurrentSweeper 方法 (依赖 Space/NormalPage/
//       Page metadata/MarkingBitmap 挂载点) 已登记 readme 台账, 待 heap-core 冻结后启动。

use crate::error::{Error, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ZappingMode {
  /// kNone — 不处置
  None = 0,
  /// kCreateFillers — 仅铺 filler (保守栈扫描需要)
  CreateFillers = 1,
  /// kCreateFillersAndZap — 铺 filler 并 zap (verify-heap / 对象统计需要)
  CreateFillersAndZap = 2,
}

/// 死对象处置模式选择 — ShouldZapDeadObjectsOnPage 的 1:1 转写。
/// v8: src/heap/sweeper.cc:631
#[must_use]
pub const fn should_zap_dead_objects_on_page(
  should_zap_garbage: bool,
  track_gc_object_stats: bool,
  conservative_stack_scanning: bool,
) -> ZappingMode {
  if should_zap_garbage || track_gc_object_stats {
    // verify-heap / 对象统计要求死对象仍是合法对象
    ZappingMode::CreateFillersAndZap
  } else if conservative_stack_scanning {
    // 保守栈扫描要求全死对象铺 filler, 防栈上假引用复活死对象
    ZappingMode::CreateFillers
  } else {
    ZappingMode::None
  }
}

/// 原子 zap 的填充字 (kZapTagged = kZapValue 截断到 Tagged_t; 指针压缩 → u32)。
/// v8: src/heap/sweeper.cc:54 AtomicZapBlock
pub const K_ZAP_TAGGED: u32 = 0xbeadbeef;

/// NEW_SPACE 判别值 (AllocationSpace, globals.h:1488)。
pub const K_NEW_SPACE: u8 = 1;
/// OLD_SPACE 判别值。
pub const K_OLD_SPACE: u8 = 2;
/// CODE_SPACE 判别值 (globals.h:1493)。
pub const K_CODE_SPACE: u8 = 3;
/// SHARED_SPACE 判别值 (globals.h:1494)。
pub const K_SHARED_SPACE: u8 = 4;
/// TRUSTED_SPACE 判别值 (globals.h:1495)。
pub const K_TRUSTED_SPACE: u8 = 5;
/// SHARED_TRUSTED_SPACE 判别值 (globals.h:1498)。
pub const K_SHARED_TRUSTED_SPACE: u8 = 6;
/// LO_SPACE 判别值 (globals.h:1500, 大对象段起始)。
pub const K_LO_SPACE: u8 = 8;
/// FIRST_SWEEPABLE_SPACE = NEW_SPACE (globals.h:1512)。
pub const K_FIRST_SWEEPABLE_SPACE: u8 = 1;
/// LAST_SWEEPABLE_SPACE = SHARED_TRUSTED_SPACE (globals.h:1513)。
pub const K_LAST_SWEEPABLE_SPACE: u8 = 6;
/// 可清扫空间数 (kNumberOfSweepingSpaces = LAST - FIRST + 1, sweeper.h:207)。
pub const K_NUMBER_OF_SWEEPING_SPACES: u8 = 6;
/// 主清扫空间数 (kNumberOfMajorSweepingSpaces = 上者 - 1, 剥 NEW_SPACE; cc:135)。
pub const K_NUMBER_OF_MAJOR_SWEEPING_SPACES: u8 = 5;
/// 主清扫任务上限 (MajorSweeperJob::kMaxTasks; cc:139)。
pub const K_MAX_TASKS_MAJOR: u8 = 5;
/// 次清扫任务上限 (MinorSweeperJob::kMaxTasks = 1; cc:215)。
pub const K_MAX_TASKS_MINOR: u8 = 1;
/// 每任务页数 (两 Job 的 GetMaxConcurrency 内 kPagePerTask = 2; cc:169/245)。
pub const K_PAGE_PER_TASK: u8 = 2;

/// MinorSweeperJob 的清扫空间 — kNewSpace = sticky_mark_bits ? OLD_SPACE : NEW_SPACE。
/// v8: src/heap/sweeper.cc:98
#[must_use]
pub const fn concurrent_minor_space(sticky_mark_bits: bool) -> u8 {
  if sticky_mark_bits {
    K_OLD_SPACE
  } else {
    K_NEW_SPACE
  }
}

/// GetMaxConcurrency 并发度式 — min(并发清扫器数, worker + (页数 + 每任务页数 - 1) / 每任务页数)。
/// v8: src/heap/sweeper.cc:168 MajorSweeperJob::GetMaxConcurrency (Minor 同式)
#[must_use]
pub const fn get_max_concurrency(
  concurrent_sweepers: usize,
  worker_count: usize,
  sweeping_page_count: usize,
) -> usize {
  let per_task =
    worker_count.saturating_add(sweeping_page_count.div_ceil(K_PAGE_PER_TASK as usize));
  if concurrent_sweepers < per_task {
    concurrent_sweepers
  } else {
    per_task
  }
}

/// 计算主线程清扫与后台清扫的追踪 ID 对 (trace_id_, background_trace_id_)。
/// trace_id 保证为偶数，background_trace_id 保证为奇数。
/// v8: src/heap/sweeper.cc:317-320
#[inline]
#[must_use]
pub const fn compute_sweeper_trace_ids(seed: u64) -> (u64, u64) {
  let trace_id = seed << 1;
  let background_trace_id = trace_id + 1;
  (trace_id, background_trace_id)
}

/// 内存页清扫阶段状态。
/// v8: src/heap/sweeper.h Sweeper::SweptList / SweepingList
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PageSweepingState {
  /// 未清扫 (待进入清扫队列)
  Unswept = 0,
  /// 正在清扫 (已出队由主线程或并发工作线程处理中)
  SweepingInProgress = 1,
  /// 清扫完成 (已可分配或放入已清扫空闲表)
  Swept = 2,
}

impl PageSweepingState {
  /// 该页当前是否允许分配。
  /// 只有已彻底清扫完成 (Swept) 的页才能安全分配。
  #[inline]
  #[must_use]
  pub const fn can_allocate_on_page(self) -> bool {
    matches!(self, Self::Swept)
  }
}

/// 清扫阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SweepingPhase {
  /// 空闲状态，未开始清扫
  #[default]
  Idle,
  /// 已初始化 trace ID 与模式，尚未启动
  Initialized,
  /// 正在清扫（主线程及可选的并发后台任务）
  Sweeping,
  /// 并发任务已等待合并完成，准备 finalize
  Joined,
}

/// 并发任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SweepingJobStatus {
  /// 无并发任务或任务已终止
  #[default]
  None,
  /// 并发任务正在后台运行
  Active,
  /// 并发任务已暂停（Cancel 后待 Resume）
  Paused,
}

/// SweepingState 并发清除状态机 — 对应 Sweeper::SweepingState<scope>。
/// 封装清扫初始化、启动、并发任务调度、暂停、恢复、等待合并及完成全生命周期状态。
/// v8: src/heap/sweeper.h:275-321, src/heap/sweeper.cc:281-393
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SweepingStateMachine {
  phase: SweepingPhase,
  job_status: SweepingJobStatus,
  in_progress: bool,
  should_reduce_memory: bool,
  trace_id: u64,
  background_trace_id: u64,
}

impl SweepingStateMachine {
  /// 创建空闲状态机
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self {
      phase: SweepingPhase::Idle,
      job_status: SweepingJobStatus::None,
      in_progress: false,
      should_reduce_memory: false,
      trace_id: 0,
      background_trace_id: 0,
    }
  }

  /// 初始化清扫状态。
  /// v8: Sweeper::SweepingState::InitializeSweeping (cc:308)
  pub fn initialize(&mut self, seed: u64, should_reduce_memory: bool) -> Result<()> {
    if self.phase != SweepingPhase::Idle || self.in_progress || self.has_valid_job() {
      return Err(Error::InvalidSweepingState {
        expected: "Idle",
        actual: self.phase_name(),
      });
    }
    let (t_id, bg_id) = compute_sweeper_trace_ids(seed);
    self.trace_id = t_id;
    self.background_trace_id = bg_id;
    self.should_reduce_memory = should_reduce_memory;
    self.phase = SweepingPhase::Initialized;
    Ok(())
  }

  /// 启动主清扫（标记进入进行中状态）。
  /// v8: Sweeper::SweepingState::StartSweeping (cc:324)
  pub fn start_sweeping(&mut self) -> Result<()> {
    if self.phase != SweepingPhase::Initialized {
      return Err(Error::InvalidSweepingState {
        expected: "Initialized",
        actual: self.phase_name(),
      });
    }
    self.in_progress = true;
    self.phase = SweepingPhase::Sweeping;
    Ok(())
  }

  /// 启动后台并发清扫任务。
  /// 若 concurrent_enabled 为 true 则登记活跃 job。
  /// v8: Sweeper::SweepingState::StartConcurrentSweeping (cc:334)
  pub fn start_concurrent_sweeping(&mut self, concurrent_enabled: bool) -> Result<bool> {
    if self.phase != SweepingPhase::Sweeping || !self.in_progress {
      return Err(Error::InvalidSweepingState {
        expected: "Sweeping",
        actual: self.phase_name(),
      });
    }
    if concurrent_enabled {
      self.job_status = SweepingJobStatus::Active;
      Ok(true)
    } else {
      Ok(false)
    }
  }

  /// 暂停并发清扫任务（取消 job handle）。
  /// v8: Sweeper::SweepingState::Pause (cc:379)
  pub fn pause(&mut self) -> bool {
    if self.job_status == SweepingJobStatus::Active {
      self.job_status = SweepingJobStatus::Paused;
      true
    } else {
      false
    }
  }

  /// 停止并发清扫任务（取消 job handle 并清空 job 状态）。
  /// v8: Sweeper::SweepingState::StopConcurrentSweeping (cc:303)
  pub fn stop_concurrent_sweeping(&mut self) -> bool {
    if self.has_valid_job() {
      self.job_status = SweepingJobStatus::None;
      true
    } else {
      false
    }
  }

  /// 恢复并发清扫任务。
  /// v8: Sweeper::SweepingState::Resume (cc:388)
  pub fn resume(&mut self) -> Result<bool> {
    if !self.in_progress {
      return Err(Error::InvalidSweepingState {
        expected: "Sweeping (in_progress)",
        actual: self.phase_name(),
      });
    }
    if self.job_status == SweepingJobStatus::Paused || self.job_status == SweepingJobStatus::None {
      self.job_status = SweepingJobStatus::Active;
      Ok(true)
    } else {
      Ok(false)
    }
  }

  /// 等待并发任务结束（合并清扫）。
  /// v8: Sweeper::SweepingState::JoinSweeping (cc:363)
  pub fn join_sweeping(&mut self) -> Result<()> {
    if !self.in_progress {
      return Err(Error::InvalidSweepingState {
        expected: "in_progress",
        actual: self.phase_name(),
      });
    }
    self.job_status = SweepingJobStatus::None;
    self.phase = SweepingPhase::Joined;
    Ok(())
  }

  /// 完成清扫，重置为 Idle。
  /// v8: Sweeper::SweepingState::FinishSweeping (cc:369)
  pub fn finish_sweeping(&mut self) -> Result<()> {
    if !self.in_progress || self.has_valid_job() {
      return Err(Error::InvalidSweepingState {
        expected: "Joined with no active job",
        actual: self.phase_name(),
      });
    }
    self.phase = SweepingPhase::Idle;
    self.in_progress = false;
    self.job_status = SweepingJobStatus::None;
    self.should_reduce_memory = false;
    self.trace_id = 0;
    self.background_trace_id = 0;
    Ok(())
  }

  #[inline]
  #[must_use]
  pub const fn phase(&self) -> SweepingPhase {
    self.phase
  }

  #[inline]
  #[must_use]
  pub const fn is_in_progress(&self) -> bool {
    self.in_progress
  }

  #[inline]
  #[must_use]
  pub const fn has_valid_job(&self) -> bool {
    matches!(
      self.job_status,
      SweepingJobStatus::Active | SweepingJobStatus::Paused
    )
  }

  #[inline]
  #[must_use]
  pub const fn has_active_job(&self) -> bool {
    matches!(self.job_status, SweepingJobStatus::Active)
  }

  #[inline]
  #[must_use]
  pub const fn should_reduce_memory(&self) -> bool {
    self.should_reduce_memory
  }

  #[inline]
  #[must_use]
  pub const fn trace_id(&self) -> u64 {
    self.trace_id
  }

  #[inline]
  #[must_use]
  pub const fn background_trace_id(&self) -> u64 {
    self.background_trace_id
  }

  const fn phase_name(&self) -> &'static str {
    match self.phase {
      SweepingPhase::Idle => "Idle",
      SweepingPhase::Initialized => "Initialized",
      SweepingPhase::Sweeping => "Sweeping",
      SweepingPhase::Joined => "Joined",
    }
  }
}

// —— 编译期不变式 (对齐 upstream static_assert / 纯式) ——
const _: () = {
  assert!(K_ZAP_TAGGED == 0xbeadbeef);
  // AllocationSpace 判别值 1:1 (globals.h:1488-1504, 枚举连续; 原版无 CODE_MAP_SPACE)
  assert!(K_NEW_SPACE + 1 == K_OLD_SPACE);
  assert!(K_OLD_SPACE + 1 == K_CODE_SPACE);
  assert!(K_CODE_SPACE + 1 == K_SHARED_SPACE);
  assert!(K_SHARED_SPACE + 1 == K_TRUSTED_SPACE);
  assert!(K_TRUSTED_SPACE + 1 == K_SHARED_TRUSTED_SPACE);
  assert!(K_SHARED_TRUSTED_SPACE + 2 == K_LO_SPACE);
  assert!(K_FIRST_SWEEPABLE_SPACE == K_NEW_SPACE);
  assert!(K_LAST_SWEEPABLE_SPACE == K_SHARED_TRUSTED_SPACE);
  assert!(K_NUMBER_OF_SWEEPING_SPACES == K_LAST_SWEEPABLE_SPACE - K_FIRST_SWEEPABLE_SPACE + 1);
  assert!(K_NUMBER_OF_MAJOR_SWEEPING_SPACES == K_NUMBER_OF_SWEEPING_SPACES - 1);
  assert!(matches!(
    should_zap_dead_objects_on_page(true, false, false),
    ZappingMode::CreateFillersAndZap
  ));
  assert!(matches!(
    should_zap_dead_objects_on_page(false, true, false),
    ZappingMode::CreateFillersAndZap
  ));
  assert!(matches!(
    should_zap_dead_objects_on_page(false, false, true),
    ZappingMode::CreateFillers
  ));
  assert!(matches!(
    should_zap_dead_objects_on_page(false, false, false),
    ZappingMode::None
  ));
  assert!(concurrent_minor_space(true) == K_OLD_SPACE);
  assert!(concurrent_minor_space(false) == K_NEW_SPACE);
  // 空页: 并发度 = min(sweepers, worker), 因向上取整 0/2 = 0
  assert!(get_max_concurrency(8, 4, 0) == 4);
  // 4 页: 每 2 页 1 任务 → 4 + 2 = 6; 上者受 sweepers 限制
  assert!(get_max_concurrency(8, 4, 4) == 6);
  assert!(get_max_concurrency(2, 4, 4) == 2);
  assert!(PageSweepingState::Unswept as u8 == 0);
  assert!(PageSweepingState::Swept as u8 == 2);
  assert!(!PageSweepingState::Unswept.can_allocate_on_page());
  assert!(!PageSweepingState::SweepingInProgress.can_allocate_on_page());
  assert!(PageSweepingState::Swept.can_allocate_on_page());
  // trace id: 偶数与奇数对
  assert!((compute_sweeper_trace_ids(42).0 & 1) == 0);
  assert!((compute_sweeper_trace_ids(42).1 & 1) == 1);
};

#[cfg(test)]
mod sweeper_tests {
  use super::*;

  #[test]
  fn test_sweeping_state_machine_lifecycle() {
    let mut sm = SweepingStateMachine::new();
    assert_eq!(sm.phase(), SweepingPhase::Idle);
    assert!(!sm.is_in_progress());

    // 1. 初始化
    sm.initialize(12345, true).unwrap();
    assert_eq!(sm.phase(), SweepingPhase::Initialized);
    assert!(sm.should_reduce_memory());
    assert_eq!(sm.trace_id(), 12345 << 1);
    assert_eq!(sm.background_trace_id(), (12345 << 1) + 1);

    // 重复初始化应报错
    assert!(sm.initialize(999, false).is_err());

    // 2. 启动清扫
    sm.start_sweeping().unwrap();
    assert_eq!(sm.phase(), SweepingPhase::Sweeping);
    assert!(sm.is_in_progress());

    // 3. 启动并发任务
    assert!(sm.start_concurrent_sweeping(true).unwrap());
    assert!(sm.has_valid_job());
    assert!(sm.has_active_job());

    // 4. 暂停与恢复
    assert!(sm.pause());
    assert!(sm.has_valid_job());
    assert!(!sm.has_active_job());

    assert!(sm.resume().unwrap());
    assert!(sm.has_active_job());

    // 4b. 停止并发清扫
    assert!(sm.stop_concurrent_sweeping());
    assert!(!sm.has_valid_job());
    assert!(!sm.has_active_job());
    assert!(!sm.stop_concurrent_sweeping()); // 第二次调用返回 false

    // 5. 合并与完成
    sm.join_sweeping().unwrap();
    assert_eq!(sm.phase(), SweepingPhase::Joined);
    assert!(!sm.has_valid_job());

    sm.finish_sweeping().unwrap();
    assert_eq!(sm.phase(), SweepingPhase::Idle);
    assert!(!sm.is_in_progress());
  }

  #[test]
  fn test_sweeping_state_machine_invalid_transitions() {
    let mut sm = SweepingStateMachine::new();
    // 未初始化直接启动 -> 报错
    assert!(sm.start_sweeping().is_err());
    // 未启动清扫直接启动并发 -> 报错
    assert!(sm.start_concurrent_sweeping(true).is_err());
  }
}
