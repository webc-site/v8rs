// Scavenger 语义面 (纯规则子集)。
//
// 族: v8/src/heap/scavenger.{h,cc}
// 分工: 本模块挂载对象年龄、并行任务数上限/段大小、NumberOfScavengeTasks 纯式。
//       ScavengerCollector::CollectGarbage / Scavenger 拷贝器 / RootScavengeVisitor /
//       ScavengerJobTask / ScavengerWeakObjectsProcessor (依赖 Heap/Tagged/Map/
//       MutablePage/SemiSpaceNewSpace/EphemeronTable 对象模型) 已登记 readme 台账,
//       待 heap-core 冻结后启动。

use crate::heap_layout::{Address, K_TAGGED_SIZE_LOG2};

/// 对象年龄 (cc 局部 enum class ObjectAge)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ObjectAge {
  /// kOld
  Old = 0,
  /// kYoung
  Young = 1,
}

/// Scavenger 常量集 (v8: src/heap/scavenger.cc)。
pub mod scavenger_consts {
  /// v8: ScavengerWeakObjectsProcessor::kWeakObjectListSegmentSize
  pub const K_WEAK_OBJECT_LIST_SEGMENT_SIZE: usize = 64;
  /// v8: Scavenger::kScavengedObjectListSegmentSize
  pub const K_SCAVENGED_OBJECT_LIST_SEGMENT_SIZE: usize = 256;
  /// v8: EmptyChunksList 段大小 (Worklist<MutablePage*, 64>)
  pub const K_EMPTY_CHUNKS_LIST_SEGMENT_SIZE: usize = 64;
  /// v8: NumberOfScavengeTasks 内 kMaxScavengerTasks (含主线程)
  pub const K_MAX_SCAVENGER_TASKS: i32 = 8;
  /// v8: Scavenger::ShouldEagerlyProcessPromotedList — kMinSegmentSize / 2
  pub const K_PROCESS_PROMOTED_LIST_THRESHOLD: i32 =
    (K_SCAVENGED_OBJECT_LIST_SEGMENT_SIZE / 2) as i32;
  /// MB (NumberOfScavengeTasks 分母)。
  pub const K_MB: usize = 1024 * 1024;
}

/// 半空间页数 (from semi-space: 每半区页数 = capacity / page_size)。
/// 保留 K_TAGGED_SIZE_LOG2 引用以驱动模块依赖顺序检查。
const _: () = {
  assert!(K_TAGGED_SIZE_LOG2 <= 3);
};

/// NumberOfScavengeTasks: `capacity/MB + 1` 与 `worker+1` 及 `kMaxScavengerTasks`
/// 三者取 min, 再夹到 `>= 1`。若 heap 提示不能同时提升 young 与扩展 old 则回落到 1
/// (内存受限优化, 依赖 Heap::CanPromoteYoungAndExpandOldGeneration, 待 heap-core 挂载)。
/// parallel_scavenge 关闭 → 恒返回 1。
///
/// v8: src/heap/scavenger.cc:1579 NumberOfScavengeTasks
#[must_use]
pub const fn number_of_scavenge_tasks(
  parallel_scavenge: bool,
  total_capacity_bytes: usize,
  worker_threads: i32,
  can_promote_and_expand: bool,
) -> i32 {
  if !parallel_scavenge {
    return 1;
  }
  let num_scavenge_tasks = (total_capacity_bytes / scavenger_consts::K_MB) as i32 + 1;
  let num_cores = worker_threads + 1;
  let mut tasks = num_scavenge_tasks;
  if scavenger_consts::K_MAX_SCAVENGER_TASKS < tasks {
    tasks = scavenger_consts::K_MAX_SCAVENGER_TASKS;
  }
  if num_cores < tasks {
    tasks = num_cores;
  }
  if tasks < 1 {
    tasks = 1;
  }
  if !can_promote_and_expand {
    tasks = 1;
  }
  tasks
}

/// 检查对象地址是否低于 AgeMark 晋升界线。
/// 若页不低于 age_mark 则不可晋升 (false)；
/// 若 age_mark 恰好位于该页内，则比较对象地址与 age_mark；
/// 若页整体低于 age_mark，则必定低于 (true)。
///
/// v8: src/heap/new-spaces-inl.h:80 SemiSpaceNewSpace::IsAddressBelowAgeMark
#[inline]
#[must_use]
pub const fn is_address_below_age_mark(
  chunk_address: Address,
  page_size: usize,
  in_new_space_below_age_mark: bool,
  age_mark: Address,
  object_address: Address,
) -> bool {
  if !in_new_space_below_age_mark {
    return false;
  }
  let on_age_mark_page =
    chunk_address < age_mark && age_mark <= chunk_address.saturating_add(page_size as Address);
  !on_age_mark_page || object_address < age_mark
}

/// Scavenger 拷贝或晋升的策略偏好。
/// v8: src/heap/scavenger.cc:2126-2146
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ScavengePreference {
  /// 优先尝试年轻代拷贝 (SemiSpaceCopy)，若空间不足/失败再回退至老生代晋升 (Promote)
  CopyThenPromote = 0,
  /// 优先老生代晋升 (Promote)，若老生代分配失败再回退尝试年轻代拷贝 (SemiSpaceCopy)
  PromoteThenCopy = 1,
}

/// 根据年龄判断 Scavenger 目标偏好。
/// 达到晋升标准的对象走 PromoteThenCopy；年轻对象走 CopyThenPromote。
#[inline]
#[must_use]
pub const fn determine_scavenge_preference(should_be_promoted: bool) -> ScavengePreference {
  if should_be_promoted {
    ScavengePreference::PromoteThenCopy
  } else {
    ScavengePreference::CopyThenPromote
  }
}

/// 搬移竞争结果 — 对应 Scavenger::TryMigrateObject 中的 CAS 语义。
/// 当多个线程竞争复制同一对象时，CAS 成功的线程负责真正复制内存并登记 forwarding 地址；
/// CAS 失败的线程放弃已分配空间，改用对手设置的已有转发地址。
/// v8: src/heap/scavenger.cc:1976-1985
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationOutcome {
  /// 首次复制成功，已分配 target 地址
  Success { target_address: Address },
  /// 发生多线程竞争已被其他线程先行迁移，返回现有转发目标
  AlreadyMigrated { existing_target: Address },
}

/// Scavenger 任务统计与本地工作清单状态。
/// v8: src/heap/scavenger.cc:442-443, 1931-1938
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ScavengerTaskStats {
  /// 本任务复制到年轻代的字节总数
  pub copied_size: usize,
  /// 本任务晋升到老生代的字节总数
  pub promoted_size: usize,
}

impl ScavengerTaskStats {
  /// 新建统计计数器
  #[inline]
  #[must_use]
  pub const fn new() -> Self {
    Self {
      copied_size: 0,
      promoted_size: 0,
    }
  }

  /// 记录年轻代复制
  #[inline]
  pub fn record_copied(&mut self, size: usize) {
    self.copied_size += size;
  }

  /// 记录老生代晋升
  #[inline]
  pub fn record_promoted(&mut self, size: usize) {
    self.promoted_size += size;
  }

  /// 是否应优先处理晋升工作清单。
  /// 当晋升队列分段长度超过门限 (kProcessPromotedListThreshold = 128) 时立即就地处理。
  /// v8: src/heap/scavenger.cc:1931 Scavenger::ShouldEagerlyProcessPromotedList
  #[inline]
  #[must_use]
  pub const fn should_eagerly_process_promoted_list(promoted_segment_size: usize) -> bool {
    promoted_segment_size >= scavenger_consts::K_PROCESS_PROMOTED_LIST_THRESHOLD as usize
  }
}

// —— 编译期不变式 ——
const _: () = {
  assert!(ObjectAge::Old as u8 == 0);
  assert!(ObjectAge::Young as u8 == 1);
  assert!(ScavengePreference::CopyThenPromote as u8 == 0);
  assert!(ScavengePreference::PromoteThenCopy as u8 == 1);
  assert!(scavenger_consts::K_MAX_SCAVENGER_TASKS == 8);
  assert!(scavenger_consts::K_PROCESS_PROMOTED_LIST_THRESHOLD == 128);
  // 并行关闭 → 恒 1
  assert!(number_of_scavenge_tasks(false, 32 * 1024 * 1024, 8, true) == 1);
  // 16MB 半区 + 4 worker → min(17, 8, 5) = 5
  assert!(number_of_scavenge_tasks(true, 16 * 1024 * 1024, 4, true) == 5);
  // 内存受限 → 回落 1
  assert!(number_of_scavenge_tasks(true, 16 * 1024 * 1024, 4, false) == 1);
  // 上限受 kMaxScavengerTasks 夹: 128MB + 32 worker → min(129, 8, 33) = 8
  assert!(number_of_scavenge_tasks(true, 128 * 1024 * 1024, 32, true) == 8);

  // age mark 纯式常量断言:
  // 1. 不在 below age mark 区间恒为 false
  assert!(!is_address_below_age_mark(
    0x1000, 4096, false, 0x2000, 0x1500
  ));
  // 2. 在 age mark 页且地址小于 age mark -> true
  assert!(is_address_below_age_mark(
    0x1000, 4096, true, 0x1800, 0x1500
  ));
  // 3. 在 age mark 页且地址大于等于 age mark -> false
  assert!(!is_address_below_age_mark(
    0x1000, 4096, true, 0x1800, 0x1900
  ));
  // 4. 整页在 age mark 之下 -> true
  assert!(is_address_below_age_mark(
    0x1000, 4096, true, 0x5000, 0x1500
  ));
};

#[cfg(test)]
mod scavenger_tests {
  use super::*;

  #[test]
  fn test_scavenge_preference_and_age_mark() {
    assert_eq!(
      determine_scavenge_preference(true),
      ScavengePreference::PromoteThenCopy
    );
    assert_eq!(
      determine_scavenge_preference(false),
      ScavengePreference::CopyThenPromote
    );

    // Below age mark:
    let page_addr = 0x20000;
    let page_size = 256 * 1024;
    let age_mark = page_addr + 128 * 1024;
    // Object before age mark on the boundary page
    assert!(is_address_below_age_mark(
      page_addr,
      page_size,
      true,
      age_mark,
      page_addr + 64 * 1024
    ));
    // Object after age mark on the boundary page
    assert!(!is_address_below_age_mark(
      page_addr,
      page_size,
      true,
      age_mark,
      page_addr + 192 * 1024
    ));
  }

  #[test]
  fn test_scavenger_stats_and_threshold() {
    let mut stats = ScavengerTaskStats::new();
    assert_eq!(stats.copied_size, 0);
    assert_eq!(stats.promoted_size, 0);

    stats.record_copied(1024);
    stats.record_promoted(2048);
    assert_eq!(stats.copied_size, 1024);
    assert_eq!(stats.promoted_size, 2048);

    assert!(!ScavengerTaskStats::should_eagerly_process_promoted_list(
      127
    ));
    assert!(ScavengerTaskStats::should_eagerly_process_promoted_list(
      128
    ));
    assert!(ScavengerTaskStats::should_eagerly_process_promoted_list(
      256
    ));
  }
}
