// 由 tool/src/heap/cppgc-internal/gc-info-table.js 从 v8/src/heap/cppgc-internal/gc-info-table.h
// + src/heap/cppgc-internal/gc-info-table.cc 生成; 修订就地改, 复跑覆盖
// GCInfo 注册表: 预留-按需扩容-发布; 声明面在 src/heap/cppgc-internal/gc-info-table.h
// 偏离登记 (详账见 readme/zh.md): 原版 OS 页粒度预留 (kNoAccess) + 只读提交推进
// (SetPermissions kRead/kReadWrite + CheckMemoryIsZeroed) → Rust Vec 倍增零值填充
// (GcInfo::default 即空回调, 零校验天然成立); FatalOutOfMemoryHandler 臂由 Rust 分配器
// fail-fast 语义承接; PageAllocator 注入参数随之省略 (Initialize 无参)。
// 溢出臂: 原版 CHECK_LT(new_index, kMaxIndex) 崩溃 → Rust 禁 panic, 返回 Error::GcInfoTableOverflow。

use std::sync::{
  OnceLock,
  atomic::{AtomicU16, Ordering},
};

use parking_lot::RwLock;

use crate::{
  error::{Error, Result},
  gc_info::{GcInfo, GcInfoIndex},
};

/// v8: src/heap/cppgc-internal/gc-info-table.h:33 GCInfoTable::kMaxIndex
pub const K_MAX_INDEX: GcInfoIndex = 1 << 14;
/// v8: src/heap/cppgc-internal/gc-info-table.h:37 GCInfoTable::kMinIndex
pub const K_MIN_INDEX: GcInfoIndex = 1;
/// v8: src/heap/cppgc-internal/gc-info-table.h:41 GCInfoTable::kInitialWantedLimit
pub const K_INITIAL_WANTED_LIMIT: GcInfoIndex = 512;

/// v8: src/heap/cppgc-internal/gc-info-table.cc:29-32 — kEntrySize 必为 2 的幂, 条目不跨提交段。
const _: () = assert!(size_of::<GcInfo>().is_power_of_two());

/// v8: src/heap/cppgc-internal/gc-info-table.h:23-87 GCInfoTable
pub struct GcInfoTable {
  /// v8: table_ (:79) — 每类一个 GCInfo; HeapObjectHeader 持索引 (index 0 为 freelist 全零项)
  table: Vec<GcInfo>,
  /// v8: current_index_ (:82) = kMinIndex — 新 GCInfo 申请用的当前索引
  current_index: GcInfoIndex,
  /// v8: limit_ (:84) = 0 — 当前已提交表上限 (不含)
  limit: GcInfoIndex,
}

impl GcInfoTable {
  /// v8: src/heap/cppgc-internal/gc-info-table.cc:57-69 GCInfoTable::GCInfoTable
  /// 原版: 整表预留 (kNoAccess) + 构造即 Resize 首次提交。
  pub fn new() -> Self {
    let mut table = Self {
      table: Vec::new(),
      current_index: K_MIN_INDEX,
      limit: 0,
    };
    table.resize();
    table
  }

  /// v8: src/heap/cppgc-internal/gc-info-table.cc:80-89 GCInfoTable::InitialTableLimit
  /// 原版 min(kMaxIndex, RoundUp(kInitialWantedLimit*kEntrySize, allocPage) / kEntrySize);
  /// Vec 形态无 OS 页对齐约束 → 直取 kInitialWantedLimit (偏离登记见 readme/zh.md)。
  const fn initial_table_limit() -> GcInfoIndex {
    if K_INITIAL_WANTED_LIMIT < K_MAX_INDEX {
      K_INITIAL_WANTED_LIMIT
    } else {
      K_MAX_INDEX
    }
  }

  /// v8: src/heap/cppgc-internal/gc-info-table.cc:75-78 GCInfoTable::MaxTableSize
  /// 原版 RoundUp(kMaxIndex*kEntrySize, allocPage); Vec 形态即容量上限 kMaxIndex 项。
  const fn max_table_size() -> usize {
    K_MAX_INDEX as usize * size_of::<GcInfo>()
  }

  /// v8: src/heap/cppgc-internal/gc-info-table.cc:132-154 GCInfoTable::RegisterNewGCInfo
  /// 锁内二次检查 registered_index → 满则 Resize → current_index_++ → 写表 → Release 发布。
  pub fn register_new_gc_info(
    &mut self,
    registered_index: &AtomicU16,
    info: GcInfo,
  ) -> Result<GcInfoIndex> {
    // 原版 MutexGuard 由调用侧 (GlobalGcInfoTable 单例互斥) 承接。
    // v8: gc-info-table.cc:140-143 — 取锁后二次检查, 他线程可能已注册。
    let val = registered_index.load(Ordering::Relaxed);
    if val != 0 {
      return Ok(val);
    }
    if self.current_index == self.limit {
      self.resize();
    }
    let new_index = self.current_index;
    // v8: gc-info-table.cc:150 CHECK_LT(new_index, kMaxIndex)
    if new_index >= K_MAX_INDEX {
      return Err(Error::GcInfoTableOverflow);
    }
    self.current_index = new_index + 1;
    self.table[new_index as usize] = info;
    registered_index.store(new_index, Ordering::Release);
    Ok(new_index)
  }

  /// v8: src/heap/cppgc-internal/gc-info-table.h:53-58 GCInfoTable::GCInfoFromIndex
  /// 原版返回 const GCInfo&; Rust GcInfo: Copy 直接返回值 (语义等价, 免悬垂引用约束)。
  pub fn gc_info_from_index(&self, index: GcInfoIndex) -> GcInfo {
    debug_assert!((K_MIN_INDEX..K_MAX_INDEX).contains(&index));
    debug_assert!(!self.table.is_empty());
    // 越界 (index ≥ limit) 原版落进未提交段 = 崩溃路径; Rust 侧 get 兜底为全零空槽。
    self.table.get(index as usize).copied().unwrap_or_default()
  }

  /// v8: src/heap/cppgc-internal/gc-info-table.h:60 GCInfoTable::NumberOfGCInfos
  pub const fn number_of_gc_infos(&self) -> GcInfoIndex {
    self.current_index
  }

  /// v8: src/heap/cppgc-internal/gc-info-table.h:62 GCInfoTable::LimitForTesting
  pub const fn limit_for_testing(&self) -> GcInfoIndex {
    self.limit
  }

  /// v8: src/heap/cppgc-internal/gc-info-table.h:63 GCInfoTable::TableSlotForTesting
  /// 原版返回可变引用 (写臂用于只读段崩溃用例); Rust 侧只读出槽 (只读推进偏离已登记)。
  pub fn table_slot_for_testing(&self, index: GcInfoIndex) -> GcInfo {
    self.gc_info_from_index(index)
  }

  /// v8: src/heap/cppgc-internal/gc-info-table.cc:91-122 GCInfoTable::Resize
  /// new_limit = limit ? 2*limit : InitialTableLimit(); 新段 ReadWrite 提交 + 旧段转只读
  /// + CheckMemoryIsZeroed (:124-130) — Vec 形态: 零值扩容天然满足三项校验 (只读推进见台账)。
  fn resize(&mut self) {
    let new_limit = if self.limit == 0 {
      Self::initial_table_limit()
    } else {
      (2 * self.limit).min(K_MAX_INDEX)
    };
    debug_assert!(new_limit > self.limit || self.limit == K_MAX_INDEX);
    debug_assert!(new_limit <= K_MAX_INDEX);
    // v8: :98 CHECK_GE(MaxTableSize(), new_committed_size)
    debug_assert!(Self::max_table_size() >= new_limit as usize * size_of::<GcInfo>());
    self.table.resize(new_limit as usize, GcInfo::default());
    self.limit = new_limit;
  }
}

impl Default for GcInfoTable {
  fn default() -> Self {
    Self::new()
  }
}

/// v8: src/heap/cppgc-internal/gc-info-table.h:90-121 GlobalGCInfoTable — 进程级单例门面
pub struct GlobalGcInfoTable;

static GLOBAL_GC_INFO_TABLE: OnceLock<RwLock<GcInfoTable>> = OnceLock::new();

impl GlobalGcInfoTable {
  /// v8: src/heap/cppgc-internal/gc-info-table.cc:44-54 GlobalGCInfoTable::Initialize
  /// 原版可重复同参调用 (异参 CHECK_EQ allocator); Rust 形态省略参数 (见台账), 幂等初始化。
  pub fn initialize() {
    let _ = Self::get();
  }

  fn get() -> &'static RwLock<GcInfoTable> {
    GLOBAL_GC_INFO_TABLE.get_or_init(|| RwLock::new(GcInfoTable::new()))
  }

  /// v8: gc-info-table.h:102 GlobalGCInfoTable::GetMutable — 可变单例 (注册路径, 写锁执行)
  pub fn with_table_mut<R>(f: impl FnOnce(&mut GcInfoTable) -> R) -> R {
    let mut guard = Self::get().write();
    f(&mut guard)
  }

  /// 兼容已有调用
  pub fn with_table<R>(f: impl FnOnce(&mut GcInfoTable) -> R) -> R {
    Self::with_table_mut(f)
  }

  /// v8: gc-info-table.h:103 GlobalGCInfoTable::Get — 只读单例 (并发读锁)
  pub fn table<R>(f: impl FnOnce(&GcInfoTable) -> R) -> R {
    let guard = Self::get().read();
    f(&guard)
  }

  /// v8: gc-info-table.cc:132 + gc-info-table.h:102 → GetMutable().RegisterNewGCInfo
  pub fn register_new_gc_info(registered_index: &AtomicU16, info: GcInfo) -> Result<GcInfoIndex> {
    // 快速路径: 取锁前先做一次 relaxed 检查, 若已注册直接返回
    let val = registered_index.load(Ordering::Relaxed);
    if val != 0 {
      return Ok(val);
    }
    Self::with_table_mut(|t| t.register_new_gc_info(registered_index, info))
  }

  /// v8: src/heap/cppgc-internal/gc-info-table.h:105-107 GlobalGCInfoTable::GCInfoFromIndex
  pub fn gc_info_from_index(index: GcInfoIndex) -> GcInfo {
    Self::table(|t| t.gc_info_from_index(index))
  }
}
