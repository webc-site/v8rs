// MarkCompactCollector 语义面 (纯规则子集)。
//
// 族: v8/src/heap/mark-compact.{h,cc,inl.h}
// 分工: 内部枚举 (StartCompactionMode/MarkingWorklistProcessingMode/CallOrigin/
//       EphemeronResult/CollectorState) 归 `mark_compact_enums.rs`; 本模块挂载
//       Evacuator 疏散模式与 ComputeEvacuationMode 1:1 转写 + 压实启发式常量,
//       MarkCompactCollector 方法骨架 (依赖 Heap/NormalPage/Space/Visitor 对象模型)
//       已登记 readme 台账, 待 heap-core 冻结后启动。

/// 疏散模式 (Evacuator::EvacuationMode, cc:4700)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EvacuationMode {
  /// kObjectsNewToOld
  ObjectsNewToOld = 0,
  /// kPageNewToOld
  PageNewToOld = 1,
  /// kObjectsOldToOld
  ObjectsOldToOld = 2,
}

impl EvacuationMode {
  /// 追踪名 — EvacuationModeName 的字符串面。
  /// v8: src/heap/mark-compact.cc:4706 Evacuator::EvacuationModeName
  #[must_use]
  pub const fn trace_name(self) -> &'static str {
    match self {
      Self::ObjectsNewToOld => "objects-new-to-old",
      Self::PageNewToOld => "page-new-to-old",
      Self::ObjectsOldToOld => "objects-old-to-old",
    }
  }
}

/// 疏散模式选择 — ComputeEvacuationMode 的 1:1 转写。
/// 原注释: 判定序重要, will_be_promoted 必须先判 (page promotion 优先于 object evacuation)。
/// v8: src/heap/mark-compact.cc:4718 Evacuator::ComputeEvacuationMode
#[must_use]
pub const fn compute_evacuation_mode(
  will_be_promoted: bool,
  in_young_generation: bool,
) -> EvacuationMode {
  if will_be_promoted {
    EvacuationMode::PageNewToOld
  } else if in_young_generation {
    EvacuationMode::ObjectsNewToOld
  } else {
    EvacuationMode::ObjectsOldToOld
  }
}

/// 常规模式目标碎片率 % (kTargetFragmentationPercent; cc:613 ComputeEvacuationHeuristics)。
pub const K_TARGET_FRAGMENTATION_PERCENT: i32 = 70;
/// 单区 (页载荷) 目标压实时长 ms (kTargetMsPerArea, 原版 float; cc:618)。
pub const K_TARGET_MS_PER_AREA: f32 = 0.5;
/// 压测中止抽查比例 (kFraction = 0.05; cc:1566 SetUpAbortEvacuationAtAddress 与
/// cc:5202 EvacuatePagesInParallel 两处同值, 均仅 stress_compaction 路径)。
pub const K_STRESS_ABORT_FRACTION: f64 = 0.05;
/// 标记工作表处理的截止检查间隔对象数 (kDeadlineCheckInterval; cc:2295)。
pub const K_DEADLINE_CHECK_INTERVAL: usize = 128;
/// 字符串表分块上限 (kMaxStringTableChunks; cc:3136 ClearNonLiveReferences)。
pub const K_MAX_STRING_TABLE_CHUNKS: i32 = 8;
/// 字符串表分块最小尺寸 (kMinStringTableChunkSize; cc:3137)。
pub const K_MIN_STRING_TABLE_CHUNK_SIZE: i32 = 1024;
/// 指针更新并行任务上限 (kMaxPointerUpdateTasks; cc:5406)。
pub const K_MAX_POINTER_UPDATE_TASKS: usize = 8;
/// reduce-memory 模式目标碎片率 % 占位 (原值取 flag
/// compaction_target_fragmentation_percent_for_reduce_memory; cc:601)。
pub const K_TARGET_FRAGMENTATION_PERCENT_REDUCE_MEMORY: i32 = 0;
/// 常规模式疏散字节上限占位 MB (原值取 flag compaction_max_evacuated_bytes_mb; cc:614)。
pub const K_MAX_EVACUATED_BYTES_MB: usize = 1024;
/// MB 单位宽 (cc:604 各处 `* MB`)。
pub const MB: usize = 1024 * 1024;

/// 疏散启发式选择 — ComputeEvacuationHeuristics 的纯规则面 1:1 转写。
/// 原函数取 `heap_->ShouldReduceMemory()`/`ShouldOptimizeForMemoryUsage()` 与 tracer 速度,
/// 此处按最小参数原则以标量入参承载三态与估速, 保留判定序 (reduce > optimize > 常规)。
/// v8: src/heap/mark-compact.cc:596 MarkCompactCollector::ComputeEvacuationHeuristics
#[must_use]
pub const fn compute_evacuation_heuristics(
  area_size: usize,
  reduce_memory: bool,
  optimize_memory: bool,
  compaction_speed_bps_ms: Option<f64>,
) -> (i32, usize) {
  // reduce / optimize 两态由外部 flag 直接给定, 深水面 flag 值此处以常量占位登记。
  if reduce_memory || optimize_memory {
    (
      K_TARGET_FRAGMENTATION_PERCENT_REDUCE_MEMORY,
      K_MAX_EVACUATED_BYTES_MB * MB,
    )
  } else {
    // 常规 (延迟敏感) 模式: 有足够速度样本则按 trace 估算, 否则回落 70%。
    let target = match compaction_speed_bps_ms {
      Some(speed) => {
        // estimated_ms_per_area = 1 + area_size / speed
        let estimated_ms_per_area = 1.0 + (area_size as f64) / speed;
        let mut percent =
          (100.0 - 100.0 * K_TARGET_MS_PER_AREA as f64 / estimated_ms_per_area) as i32;
        // 下界夹到 reduce-memory 目标 (cc:637)
        if percent < K_TARGET_FRAGMENTATION_PERCENT_REDUCE_MEMORY {
          percent = K_TARGET_FRAGMENTATION_PERCENT_REDUCE_MEMORY;
        }
        percent
      }
      None => K_TARGET_FRAGMENTATION_PERCENT,
    };
    (target, K_MAX_EVACUATED_BYTES_MB * MB)
  }
}

/// 计算碎片空闲字节阈值 — target_fragmentation_percent * (area_size / 100)。
/// v8: src/heap/mark-compact.cc:675
#[inline]
#[must_use]
pub const fn compute_free_bytes_threshold(
  area_size: usize,
  target_fragmentation_percent: i32,
) -> usize {
  if target_fragmentation_percent <= 0 {
    0
  } else {
    (target_fragmentation_percent as usize) * (area_size / 100)
  }
}

/// 预估疏散活对象所需新页数 — ceil(total_live_bytes / area_size)。
/// v8: src/heap/mark-compact.cc:779
#[inline]
#[must_use]
pub const fn estimated_new_pages(total_live_bytes: usize, area_size: usize) -> usize {
  if area_size == 0 {
    0
  } else {
    total_live_bytes.div_ceil(area_size)
  }
}

/// 待评估的疏散候选页
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvacuationCandidatePage {
  /// 页/块标识（如地址或页索引）
  pub page_id: usize,
  /// 页内活对象分配字节数 (allocated_bytes / live_bytes)
  pub live_bytes: usize,
}

impl EvacuationCandidatePage {
  /// 创建候选页
  #[inline]
  #[must_use]
  pub const fn new(page_id: usize, live_bytes: usize) -> Self {
    Self {
      page_id,
      live_bytes,
    }
  }
}

/// 疏散候选页选择结果
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EvacuationSelectionResult {
  /// 选中的疏散候选页标识列表（按活字节从小到大排序）
  pub selected_pages: Vec<usize>,
  /// 选中的总活字节数
  pub total_live_bytes: usize,
  /// 容纳被疏散活对象预估所需新页数
  pub estimated_new_pages: usize,
  /// 预估净释放页数 (selected_pages.len() - estimated_new_pages)
  pub estimated_released_pages: usize,
}

/// 疏散候选页选择算法 — CollectEvacuationCandidates 的纯逻辑 1:1 转写。
///
/// 算法步骤:
/// 1. 若非全量必缩模式，先过滤出空闲字节 >= 阈值 (area_size - live_bytes >= free_bytes_threshold) 的候选页；
/// 2. 按 live_bytes 由小到大排序（空闲率高的优先疏散，空穴最容易压实）；
/// 3. 在 max_evacuated_bytes 配额内贪心纳入候选；
/// 4. 计算预估所需新页数与净释放页数，若净释放页数为 0 且非 compact_on_every_full_gc，
///    则放弃本次疏散以避免无收益的「压缩-扩展」反复震荡。
///
/// v8: src/heap/mark-compact.cc:649-789
#[must_use]
pub fn select_evacuation_candidates(
  area_size: usize,
  pages: impl IntoIterator<Item = EvacuationCandidatePage>,
  target_fragmentation_percent: i32,
  max_evacuated_bytes: usize,
  compact_on_every_full_gc: bool,
) -> EvacuationSelectionResult {
  if area_size == 0 {
    return EvacuationSelectionResult::default();
  }

  let free_bytes_threshold = compute_free_bytes_threshold(area_size, target_fragmentation_percent);

  // 1. 过滤：仅当在标准路径时，检查 free_bytes >= free_bytes_threshold
  let mut eligible: Vec<EvacuationCandidatePage> = pages
    .into_iter()
    .filter(|p| {
      if compact_on_every_full_gc {
        true
      } else {
        area_size.saturating_sub(p.live_bytes) >= free_bytes_threshold
      }
    })
    .collect();

  // 2. 升序排序：活对象最少（空闲空间最多）的页排在前面
  eligible.sort_unstable_by_key(|p| p.live_bytes);

  // 3. 配额累计
  let mut selected_count = 0usize;
  let mut total_live_bytes = 0usize;
  for p in &eligible {
    if compact_on_every_full_gc
      || (total_live_bytes.saturating_add(p.live_bytes) <= max_evacuated_bytes)
    {
      selected_count += 1;
      total_live_bytes = total_live_bytes.saturating_add(p.live_bytes);
    } else {
      break;
    }
  }

  // 4. 净释放页评估与防震荡机制
  let new_pages = estimated_new_pages(total_live_bytes, area_size);
  let released_pages = selected_count.saturating_sub(new_pages);

  if released_pages == 0 && !compact_on_every_full_gc {
    // 释放页数为 0 说明压实后页数未净减，规避无意义的搬移开销
    EvacuationSelectionResult::default()
  } else {
    let selected_pages = eligible[..selected_count]
      .iter()
      .map(|p| p.page_id)
      .collect();
    EvacuationSelectionResult {
      selected_pages,
      total_live_bytes,
      estimated_new_pages: new_pages,
      estimated_released_pages: released_pages,
    }
  }
}

// —— 编译期不变式 (对齐 upstream static_assert) ——
const _: () = {
  assert!(K_TARGET_FRAGMENTATION_PERCENT == 70);
  assert!(K_DEADLINE_CHECK_INTERVAL == 128);
  assert!(K_MAX_POINTER_UPDATE_TASKS == 8);
  // 判定序不变式: promotion 优先
  assert!(matches!(
    compute_evacuation_mode(true, true),
    EvacuationMode::PageNewToOld
  ));
  assert!(matches!(
    compute_evacuation_mode(true, false),
    EvacuationMode::PageNewToOld
  ));
  assert!(matches!(
    compute_evacuation_mode(false, true),
    EvacuationMode::ObjectsNewToOld
  ));
  assert!(matches!(
    compute_evacuation_mode(false, false),
    EvacuationMode::ObjectsOldToOld
  ));
  assert!(EvacuationMode::ObjectsNewToOld as u8 == 0);
  assert!(EvacuationMode::PageNewToOld as u8 == 1);
  assert!(EvacuationMode::ObjectsOldToOld as u8 == 2);
  // 疏散启发式判定序: reduce/optimize 优先于常规
  assert!(
    compute_evacuation_heuristics(0, true, false, None).0
      == K_TARGET_FRAGMENTATION_PERCENT_REDUCE_MEMORY
  );
  // 常规无估速 → 回落 70%
  assert!(
    compute_evacuation_heuristics(4096, false, false, None).0 == K_TARGET_FRAGMENTATION_PERCENT
  );
};

#[cfg(test)]
mod heuristics_tests {
  use super::*;

  #[test]
  fn regular_fallback_without_speed() {
    let (pct, bytes) = compute_evacuation_heuristics(4096, false, false, None);
    assert_eq!(pct, K_TARGET_FRAGMENTATION_PERCENT);
    assert_eq!(bytes, K_MAX_EVACUATED_BYTES_MB * MB);
  }

  #[test]
  fn trace_based_target() {
    // speed 使 estimated_ms_per_area = 1 + area/speed; 取 area=1MB, speed=2MB/ms
    // → ms_per_area = 1.5, percent = 100 - 100*0.5/1.5 ≈ 66 (>= reduce 0 不夹)
    let (pct, _) = compute_evacuation_heuristics(MB, false, false, Some(2.0 * MB as f64));
    assert_eq!(pct, 66);
  }

  #[test]
  fn reduce_memory_wins_over_regular() {
    let (pct, _) = compute_evacuation_heuristics(MB, true, true, Some(1.0e9));
    assert_eq!(pct, K_TARGET_FRAGMENTATION_PERCENT_REDUCE_MEMORY);
  }

  #[test]
  fn candidate_selection_filters_and_sorts() {
    let area_size = 256 * 1024;
    // target fragmentation 50% -> free threshold = 128KB (live <= 128KB qualifies)
    let pages = [
      EvacuationCandidatePage::new(1, 200 * 1024), // live 200KB -> free 56KB < 128KB -> filtered out
      EvacuationCandidatePage::new(2, 80 * 1024),  // live 80KB -> free 176KB >= 128KB -> eligible
      EvacuationCandidatePage::new(3, 40 * 1024),  // live 40KB -> free 216KB >= 128KB -> eligible
    ];
    let res = select_evacuation_candidates(area_size, pages, 50, 1024 * 1024, false);
    // Page 3 has 40KB, page 2 has 80KB. Sorted: [3, 2].
    // Total live = 120KB. New pages = ceil(120 / 256) = 1.
    // Released = 2 - 1 = 1 page.
    assert_eq!(res.selected_pages, vec![3, 2]);
    assert_eq!(res.total_live_bytes, 120 * 1024);
    assert_eq!(res.estimated_new_pages, 1);
    assert_eq!(res.estimated_released_pages, 1);
  }

  #[test]
  fn candidate_selection_anti_expansion_abort() {
    let area_size = 256 * 1024;
    // 1 page eligible with 100KB live. New pages needed = ceil(100/256) = 1.
    // Released = 1 - 1 = 0. In regular mode, this aborts to avoid compact->expand cycle.
    let pages = [EvacuationCandidatePage::new(1, 100 * 1024)];
    let res = select_evacuation_candidates(area_size, pages, 10, 1024 * 1024, false);
    assert_eq!(res.selected_pages.len(), 0);
    assert_eq!(res.estimated_released_pages, 0);

    // If compact_on_every_full_gc is true, it proceeds anyway.
    let res_force = select_evacuation_candidates(area_size, pages, 10, 1024 * 1024, true);
    assert_eq!(res_force.selected_pages, vec![1]);
    assert_eq!(res_force.total_live_bytes, 100 * 1024);
  }

  #[test]
  fn candidate_selection_respects_quota() {
    let area_size = 256 * 1024;
    let pages = [
      EvacuationCandidatePage::new(1, 10 * 1024),
      EvacuationCandidatePage::new(2, 20 * 1024),
      EvacuationCandidatePage::new(3, 30 * 1024),
    ];
    // Quota only allows 40KB -> picks page 1 (10KB) and page 2 (20KB), stops before page 3 (30KB)
    let res = select_evacuation_candidates(area_size, pages, 10, 40 * 1024, false);
    assert_eq!(res.selected_pages, vec![1, 2]);
    assert_eq!(res.total_live_bytes, 30 * 1024);
    assert_eq!(res.estimated_new_pages, 1);
    assert_eq!(res.estimated_released_pages, 1);
  }
}
