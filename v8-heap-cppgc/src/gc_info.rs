// 由 tool/src/heap/cppgc-internal/gc-info.js 从 v8/src/heap/cppgc-internal/gc-info.cc 生成;
// 型面 (GCInfoIndex/kMaxGCInfoIndex/kMinGCInfoIndex/GCInfo/GetHiddenName/EnsureGCInfoIndexTrait)
// 声明在 include/cppgc/internal/gc-info.h, 随本件同挂载; 修订就地改, 复跑覆盖。
// 推迟台账: GCInfoTrait<T> (gc-info.h:144-173) 与 GCInfoFolding<T,ParentMost> (:178-215) 为
// 编译期 T 模板面, 待 allocation/object-allocator 批次承接; GCInfoTableSection 与
// CPPGC_ENABLE_OBJECT_SECTION_GCINFO 分支 (:42-79, gc-info.cc 全件外层 #if) 非目标配置, 不实现
// (见 readme/zh.md)。

use core::ffi::c_void;
use std::sync::atomic::AtomicU16;

use crate::{
  error::Result,
  gc_info_table::GlobalGcInfoTable,
  name_trait::{
    HeapObjectName, HeapObjectNameForUnnamedObject, K_HIDDEN_NAME, K_NO_NAME_DEDUCIBLE,
    NameCallback,
  },
  trace_trait::TraceCallback,
  visitor::Visitor,
};

/// v8: include/cppgc/internal/gc-info.h:20 GCInfoIndex (uint16_t)
pub type GcInfoIndex = u16;
/// v8: include/cppgc/internal/gc-info.h:21 kMaxGCInfoIndex
pub const K_MAX_GC_INFO_INDEX: GcInfoIndex = 1 << 14;
/// v8: include/cppgc/internal/gc-info.h:22 kMinGCInfoIndex
pub const K_MIN_GC_INFO_INDEX: GcInfoIndex = 1;

/// v8: include/cppgc/internal/finalizer-trait.h:15 FinalizationCallback — `void (*)(void*)`
/// 原版可空 (nullptr = 无需析构), Rust 侧以 Option 承接 (见 readme/zh.md 台账)。
pub type FinalizationCallback = unsafe fn(*mut c_void);

/// v8: include/cppgc/internal/gc-info.h:24-33 GCInfo — 每类一个回调三元组 + 补齐字。
/// `padding` 为原版第 4 字 (`size_t padding = 0`), 使 sizeof(GCInfo) 为 2 的幂,
/// 从而条目不跨 GCInfoTable 的 OS 页提交段边界 (gc-info-table.cc:24-32 注释)。
#[derive(Clone, Copy)]
pub struct GcInfo {
  /// 原版 finalize — 可空 (空 = 非终结对象)。
  pub finalize: Option<FinalizationCallback>,
  /// 原版 trace。
  pub trace: TraceCallback,
  /// 原版 name。
  pub name: NameCallback,
  /// 原版 padding — 仅占位, 无运行期语义。
  pub padding: usize,
}

/// v8: gc-info.h:32 padding 字 + 3 指针 = 4 字 (64 位)。
const _: () = assert!(size_of::<GcInfo>() == 4 * size_of::<usize>());
/// v8: src/heap/cppgc-internal/gc-info-table.cc:30-32 static_assert(bits::IsPowerOfTwo(kEntrySize))
const _: () = assert!(size_of::<GcInfo>().is_power_of_two());

impl GcInfo {
  /// v8: include/cppgc/internal/gc-info.h:25-27 GCInfo(finalize, trace, name) — padding 隐 0。
  pub const fn new(
    finalize: Option<FinalizationCallback>,
    trace: TraceCallback,
    name: NameCallback,
  ) -> Self {
    Self {
      finalize,
      trace,
      name,
      padding: 0,
    }
  }
}

/// 原版未提交槽为全零内存 (回调 nullptr); Rust 非空 fn 指针无法表承 null,
/// 以空操作回调填充同一槽位 — 已注册区之外无任何调用面 (越界读在原版同样是崩溃路径)。
impl Default for GcInfo {
  fn default() -> Self {
    Self {
      finalize: None,
      trace: empty_trace,
      name: empty_name,
      padding: 0,
    }
  }
}

/// 空槽 trace 回调 (见 Default 注释)。
fn empty_trace(_visitor: *mut Visitor, _object: *const c_void) {}

/// 空槽 name 回调 (见 Default 注释)。
fn empty_name(_object: *const c_void, _mode: HeapObjectNameForUnnamedObject) -> HeapObjectName {
  HeapObjectName {
    value: K_NO_NAME_DEDUCIBLE,
    name_was_hidden: false,
  }
}

/// v8: include/cppgc/internal/gc-info.h:35-40 GetHiddenName — 隐名 NameCallback 常量臂。
pub fn get_hidden_name(
  _object: *const c_void,
  name_retrieval_mode: HeapObjectNameForUnnamedObject,
) -> HeapObjectName {
  HeapObjectName {
    value: K_HIDDEN_NAME,
    name_was_hidden: name_retrieval_mode == HeapObjectNameForUnnamedObject::UseHiddenName,
  }
}

/// v8: include/cppgc/internal/gc-info.h:83-106 EnsureGCInfoIndexTrait —
/// 4 个 EnsureGCInfoIndex 重载 (trace [+ finalize] [+ name] 组合), 全部委托
/// GlobalGCInfoTable::GetMutable().RegisterNewGCInfo。
/// 原版按 FinalizerTrait::HasFinalizer() 与 NameTrait::HasNonHiddenName() 编译期四选一
/// (DISPATCH 宏 gc-info.h:108-138); Rust 无编译期布尔特化 → 4 个显名入口, 由调用方按 T 的
/// trait 事实选择 (偏离登记见 readme/zh.md)。
pub struct EnsureGcInfoIndexTrait;

impl EnsureGcInfoIndexTrait {
  /// v8: src/heap/cppgc-internal/gc-info.cc:15-21 EnsureGCInfoIndex(trace, finalization, name)
  pub fn ensure_gc_info_index(
    registered_index: &AtomicU16,
    trace_callback: TraceCallback,
    finalization_callback: FinalizationCallback,
    name_callback: NameCallback,
  ) -> Result<GcInfoIndex> {
    GlobalGcInfoTable::register_new_gc_info(
      registered_index,
      GcInfo::new(Some(finalization_callback), trace_callback, name_callback),
    )
  }

  /// v8: src/heap/cppgc-internal/gc-info.cc:24-30 EnsureGCInfoIndex(trace, finalization) — 隐名
  pub fn ensure_gc_info_index_with_finalization(
    registered_index: &AtomicU16,
    trace_callback: TraceCallback,
    finalization_callback: FinalizationCallback,
  ) -> Result<GcInfoIndex> {
    Self::ensure_gc_info_index(
      registered_index,
      trace_callback,
      finalization_callback,
      get_hidden_name,
    )
  }

  /// v8: src/heap/cppgc-internal/gc-info.cc:33-38 EnsureGCInfoIndex(trace, name) — 无析构
  pub fn ensure_gc_info_index_with_name(
    registered_index: &AtomicU16,
    trace_callback: TraceCallback,
    name_callback: NameCallback,
  ) -> Result<GcInfoIndex> {
    GlobalGcInfoTable::register_new_gc_info(
      registered_index,
      GcInfo::new(None, trace_callback, name_callback),
    )
  }

  /// v8: src/heap/cppgc-internal/gc-info.cc:41-45 EnsureGCInfoIndex(trace) — 无析构 + 隐名
  pub fn ensure_gc_info_index_trace_only(
    registered_index: &AtomicU16,
    trace_callback: TraceCallback,
  ) -> Result<GcInfoIndex> {
    Self::ensure_gc_info_index_with_name(registered_index, trace_callback, get_hidden_name)
  }
}
