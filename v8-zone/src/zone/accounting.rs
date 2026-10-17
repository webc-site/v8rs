//! 原版 src/zone/accounting-allocator.h/cc 转写
//!
//! AccountingAllocator 负责段的 malloc/free 与用量记账;
//! TracingAccountingAllocator 对应本 crate 的 tracing 形态 (new_tracing)。
//! 推迟项: managed_zone_memory 路径 (依赖 Isolate/MemoryPool/VirtualMemory, L3 后回收)。

use std::{
  alloc::{Layout, alloc, dealloc},
  collections::BTreeSet,
  io::{self, Write},
  mem,
  ptr::{self, NonNull},
  sync::{
    Arc, Mutex, MutexGuard, PoisonError,
    atomic::{AtomicBool, AtomicUsize, Ordering},
  },
};

#[cfg(feature = "precise-zone-stats")]
use crate::zone::type_stats::TypeStats;
use crate::{
  internal::bits::{ALIGNMENT, MB},
  zone::{core::Zone, segment::Segment},
};

/// 原版 TracingFlags::is_zone_stats_enabled 的最小替身
/// (待回收: v8-tracing 提供全局 TracingFlags)
static ZONE_STATS_ENABLED: AtomicBool = AtomicBool::new(false);

/// 读取 zone 统计 tracing 开关
pub fn zone_stats_enabled() -> bool {
  ZONE_STATS_ENABLED.load(Ordering::Relaxed)
}

/// 设置 zone 统计 tracing 开关
pub fn set_zone_stats_enabled(enabled: bool) {
  ZONE_STATS_ENABLED.store(enabled, Ordering::Relaxed);
}

/// 原版 v8_flags.trace_zone_type_stats 的最小替身
/// (仅 V8_ENABLE_PRECISE_ZONE_STATS 有意义; 待回收: v8-flags)
#[cfg(feature = "precise-zone-stats")]
static TRACE_ZONE_TYPE_STATS: AtomicBool = AtomicBool::new(false);

/// 读取逐类型统计开关 (原版 v8_flags.trace_zone_type_stats, 默认 false)
#[cfg(feature = "precise-zone-stats")]
pub fn trace_zone_type_stats_enabled() -> bool {
  TRACE_ZONE_TYPE_STATS.load(Ordering::Relaxed)
}

/// 设置逐类型统计开关
#[cfg(feature = "precise-zone-stats")]
pub fn set_trace_zone_type_stats_enabled(enabled: bool) {
  TRACE_ZONE_TYPE_STATS.store(enabled, Ordering::Relaxed);
}

/// 活跃 zone 的统计视图: 以 Arc 原子计数共享, 对应原版读 const Zone* 字段
struct ActiveZone {
  addr: usize,
  name: Box<str>,
  segment_bytes: Arc<AtomicUsize>,
  allocation_size: Arc<AtomicUsize>,
  freed_size: Arc<AtomicUsize>,
}

/// tracing 形态的内部状态 (原版 mutex_ + active_zones_ + 流量计数)
struct ZoneTracing {
  active_zones: Mutex<BTreeSet<AddrKey>>,
  zone_views: Mutex<Vec<ActiveZone>>,
  trace_zone_stats: AtomicBool,
  memory_traffic_since_last_report: Mutex<usize>,
  tolerance: usize,
  /// 原版 type_stats_ (仅 V8_ENABLE_PRECISE_ZONE_STATS; MergeWith/Dump
  /// 对应 accounting-allocator.cc:162-175。原版在 mutex_ 临界区内变更,
  /// 此处以独立 Mutex 表达)
  #[cfg(feature = "precise-zone-stats")]
  type_stats: Mutex<TypeStats>,
}

/// 按地址排序的活跃 zone 键 (避免迭代序不确定, 见 glossary 差分约定)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct AddrKey(usize);

/// zone 段的分配/归还与用量记账
pub struct AccountingAllocator {
  current_memory_usage: AtomicUsize,
  max_memory_usage: AtomicUsize,
  tracing: Option<ZoneTracing>,
}

impl Default for AccountingAllocator {
  fn default() -> Self {
    Self::new()
  }
}

impl AccountingAllocator {
  /// 原版 AccountingAllocator()
  pub fn new() -> Self {
    AccountingAllocator {
      current_memory_usage: AtomicUsize::new(0),
      max_memory_usage: AtomicUsize::new(0),
      tracing: None,
    }
  }

  /// 原版 TracingAccountingAllocator(isolate); tolerance 对应
  /// v8_flags.zone_stats_tolerance (默认 1MB), trace_zone_stats 对应
  /// v8_flags.trace_zone_stats (默认 false)
  /// (待回收: v8-flags 接管这两个 flag)
  pub fn new_tracing(trace_zone_stats: bool, tolerance: usize) -> Self {
    AccountingAllocator {
      current_memory_usage: AtomicUsize::new(0),
      max_memory_usage: AtomicUsize::new(0),
      tracing: Some(ZoneTracing {
        active_zones: Mutex::new(BTreeSet::new()),
        zone_views: Mutex::new(Vec::new()),
        trace_zone_stats: AtomicBool::new(trace_zone_stats),
        memory_traffic_since_last_report: Mutex::new(0),
        tolerance,
        #[cfg(feature = "precise-zone-stats")]
        type_stats: Mutex::new(TypeStats::default()),
      }),
    }
  }

  /// 默认容差 1MB 的 tracing 分配器
  pub fn with_default_tolerance(trace_zone_stats: bool) -> Self {
    Self::new_tracing(trace_zone_stats, MB)
  }

  /// 原版 GetCurrentMemoryUsage()
  pub fn current_memory_usage(&self) -> usize {
    self.current_memory_usage.load(Ordering::Relaxed)
  }

  /// 原版 GetMaxMemoryUsage()
  pub fn max_memory_usage(&self) -> usize {
    self.max_memory_usage.load(Ordering::Relaxed)
  }

  /// 原版 AccountingAllocator::AllocateSegment; 失败返回 None (原版返回 nullptr)
  pub fn allocate_segment(&self, bytes: usize) -> Option<NonNull<Segment>> {
    if bytes < mem::size_of::<Segment>() {
      return None;
    }
    let layout = Layout::from_size_align(bytes, ALIGNMENT).ok()?;
    // SAFETY: size 非零 (ALIGNMENT 上已校验); 返回空指针即分配失败
    let memory = unsafe { alloc(layout) };
    if memory.is_null() {
      return None;
    }
    let current = self
      .current_memory_usage
      .fetch_add(bytes, Ordering::Relaxed)
      + bytes;
    let mut max = self.max_memory_usage.load(Ordering::Relaxed);
    while current > max {
      match self.max_memory_usage.compare_exchange_weak(
        max,
        current,
        Ordering::Relaxed,
        Ordering::Relaxed,
      ) {
        Ok(_) => break,
        Err(observed) => max = observed,
      }
    }
    let ptr = memory.cast::<Segment>();
    // 暴露 raw 指针 provenance: Zone 以地址整数 (usize, 原版 Address position_)
    // 持有分配位置, 后续 int→ptr 回转经本 exposed tag 取回可写权限
    // (Miri Stacked Borrows 通配符解析的官方机制; 原版 C++ 即纯地址整数运算)
    let _ = ptr.expose_provenance();
    // SAFETY: memory 为刚分配的 >= size_of::<Segment>() 的合法块, 此处放置构造头部
    unsafe { ptr.write(Segment::new(bytes)) };
    NonNull::new(ptr)
  }

  /// 原版 AccountingAllocator::ReturnSegment
  ///
  /// # Safety
  /// segment 必须由本分配器 allocate_segment 返回且尚未归还
  pub unsafe fn return_segment(&self, segment: NonNull<Segment>) {
    let seg_ptr = segment.as_ptr();
    // 原版顺序: ZapContents → 记账扣减 → ZapHeader → 归还
    Segment::zap_contents(seg_ptr);
    // SAFETY: 调用方保证段所有权, 头部字段此时仍有效
    let segment_size = unsafe { (*seg_ptr).total_size() };
    self
      .current_memory_usage
      .fetch_sub(segment_size, Ordering::Relaxed);
    Segment::zap_header(seg_ptr);
    // SAFETY: 块由 allocate_segment 以相同 layout 分配, 此处归还;
    // zap_header 后头部字段不可再读, layout 先取出
    let layout = unsafe { Layout::from_size_align_unchecked(segment_size, ALIGNMENT) };
    unsafe { dealloc(seg_ptr.cast(), layout) };
  }

  /// 原版 TraceZoneCreation: 仅在 zone_stats tracing 开启时登记
  pub(crate) fn trace_zone_creation(&self, zone: &Zone<'_>) {
    if !zone_stats_enabled() {
      return;
    }
    if let Some(tracing) = &self.tracing {
      let addr = ptr::from_ref(zone) as usize;
      lock_clean(&tracing.active_zones).insert(AddrKey(addr));
      lock_clean(&tracing.zone_views).push(ActiveZone {
        addr,
        name: zone.name().into(),
        segment_bytes: Arc::clone(zone.segment_bytes_view()),
        allocation_size: Arc::clone(zone.allocation_size_view()),
        freed_size: Arc::clone(zone.freed_size_view()),
      });
    }
  }

  /// 原版 TraceZoneDestruction
  pub(crate) fn trace_zone_destruction(&self, zone: &Zone<'_>) {
    if !zone_stats_enabled() {
      return;
    }
    if let Some(tracing) = &self.tracing {
      // 原版顺序: 先并账逐类型统计, 再报流量 (Dump 尚含将亡 zone),
      // 再从活跃集移除; 活跃集清空后打印类型统计总表
      // (accounting-allocator.cc:160-175)
      #[cfg(feature = "precise-zone-stats")]
      if trace_zone_type_stats_enabled() {
        lock_clean(&tracing.type_stats).merge_with(&zone.type_stats());
      }
      self.update_memory_traffic_and_report(zone.segment_bytes_allocated());
      let addr = ptr::from_ref(zone) as usize;
      lock_clean(&tracing.active_zones).remove(&AddrKey(addr));
      let mut views = lock_clean(&tracing.zone_views);
      if let Some(i) = views.iter().position(|z| z.addr == addr) {
        views.swap_remove(i);
      }
      drop(views);
      #[cfg(feature = "precise-zone-stats")]
      if trace_zone_type_stats_enabled() && lock_clean(&tracing.active_zones).is_empty() {
        lock_clean(&tracing.type_stats).dump();
      }
    }
  }

  /// 原版 TraceAllocateSegment
  pub(crate) fn trace_allocate_segment(&self, segment: &Segment) {
    if !zone_stats_enabled() {
      return;
    }
    if self.tracing.is_some() {
      self.update_memory_traffic_and_report(segment.total_size());
    }
  }

  /// 原版 UpdateMemoryTrafficAndReportMemoryUsage
  fn update_memory_traffic_and_report(&self, memory_traffic_delta: usize) {
    let Some(tracing) = &self.tracing else { return };
    let by_flag = tracing.trace_zone_stats.load(Ordering::Relaxed);
    if !by_flag && !zone_stats_enabled() {
      return;
    }
    let mut traffic = lock_clean(&tracing.memory_traffic_since_last_report);
    *traffic += memory_traffic_delta;
    if *traffic < tracing.tolerance {
      return;
    }
    *traffic = 0;
    drop(traffic);
    let trace_str = self.dump(true);
    if by_flag {
      // 原版 PrintF 输出 stdout; L0 不引 log 依赖, 直写标准输出, 写失败静默
      let mut out = io::stdout();
      let _ = writeln!(
        out,
        "{{\"type\": \"v8-zone-trace\", \"stats\": {trace_str}}}"
      );
    }
  }

  /// 原版 TracingAccountingAllocator::Dump
  fn dump(&self, dump_details: bool) -> String {
    let Some(tracing) = &self.tracing else {
      return "{\"isolate\": null, \"allocated\": 0, \"used\": 0, \"freed\": 0}".to_string();
    };
    let views = lock_clean(&tracing.zone_views);
    let mut out = String::from("{\"isolate\": null, ");
    let mut total_allocated = 0usize;
    let mut total_used = 0usize;
    let mut total_freed = 0usize;
    if dump_details {
      out.push_str("\"zones\": [");
      for (i, zone) in views.iter().enumerate() {
        if i > 0 {
          out.push_str(", ");
        }
        let allocated = zone.segment_bytes.load(Ordering::Relaxed);
        let used = zone.allocation_size.load(Ordering::Relaxed);
        let freed = zone.freed_size.load(Ordering::Relaxed);
        total_allocated += allocated;
        total_used += used;
        total_freed += freed;
        out.push_str(&format!(
          "{{\"name\": \"{}\", \"allocated\": {allocated}, \"used\": {used}, \"freed\": {freed}}}",
          zone.name
        ));
      }
      out.push_str("], ");
    } else {
      for zone in views.iter() {
        total_allocated += zone.segment_bytes.load(Ordering::Relaxed);
        total_used += zone.allocation_size.load(Ordering::Relaxed);
        total_freed += zone.freed_size.load(Ordering::Relaxed);
      }
    }
    out.push_str(&format!(
      "\"allocated\": {total_allocated}, \"used\": {total_used}, \"freed\": {total_freed}}}"
    ));
    out
  }
}

impl Drop for AccountingAllocator {
  /// 原版 ~AccountingAllocator = default; 显式空实现仅固定释放顺序
  fn drop(&mut self) {}
}

/// 临界区无 panic 路径, 中毒视为不可达, 直接取内层
fn lock_clean<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
  m.lock().unwrap_or_else(PoisonError::into_inner)
}
