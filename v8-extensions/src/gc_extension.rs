// 由 tool/src/extensions/gc-extension.h.js 从 v8/src/extensions/gc-extension.{h,cc} 生成; 修订就地改, 复跑覆盖
// GC 扩展: 选项枚举 + 选项字符串表 + 签名骨架

//! GC 扩展 — 原版 namespace v8::internal::GCExtension
//!
//! 语义: 注入 gc(options) — 按 type/execution/flavor/filename 选项袋触发 GC;
//! 异步执行返回 Promise (AsyncGC 可取消任务), 同步返回 undefined。

/// GC 扩展注册名
pub const GC_EXTENSION_NAME: &str = "v8/gc";

/// 栈上源码缓冲容量 (原版 char buffer_[50])
pub const GC_BUFFER_SIZE: usize = 50;

/// 默认堆快照文件名 (原版 kDefaultHeapSnapshotFileName)
pub const DEFAULT_HEAP_SNAPSHOT_FILENAME: &str = "heap.heapsnapshot";

/// GC 类型 (原版 enum class GCType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum GcType {
  /// 原版 kMinor
  Minor = 0,
  /// 原版 kMajor
  Major = 1,
  /// 原版 kMajorWithSnapshot
  MajorWithSnapshot = 2,
}

impl GcType {
  #[inline]
  #[must_use]
  pub const fn is_minor(self) -> bool {
    matches!(self, Self::Minor)
  }

  #[inline]
  #[must_use]
  pub const fn is_major(self) -> bool {
    matches!(self, Self::Major)
  }

  #[inline]
  #[must_use]
  pub const fn is_major_with_snapshot(self) -> bool {
    matches!(self, Self::MajorWithSnapshot)
  }
}

/// 执行时机 (原版 enum class ExecutionType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum ExecutionType {
  /// 原版 kAsync
  Async = 0,
  /// 原版 kSync
  Sync = 1,
}

impl ExecutionType {
  #[inline]
  #[must_use]
  pub const fn is_async(self) -> bool {
    matches!(self, Self::Async)
  }

  #[inline]
  #[must_use]
  pub const fn is_sync(self) -> bool {
    matches!(self, Self::Sync)
  }
}

/// GC 风味 (原版 enum class Flavor)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum Flavor {
  /// 原版 kRegular
  Regular = 0,
  /// 原版 kLastResort
  LastResort = 1,
}

impl Flavor {
  #[inline]
  #[must_use]
  pub const fn is_regular(self) -> bool {
    matches!(self, Self::Regular)
  }

  #[inline]
  #[must_use]
  pub const fn is_last_resort(self) -> bool {
    matches!(self, Self::LastResort)
  }
}

/// 选项字符串表 (原版 ParseType/ParseExecution/ParseFlavor 的 StrictEquals 字面量, 1:1)
pub const OPTION_STRINGS: [(&str, &str); 7] = [
  ("type", "minor"),
  ("type", "major"),
  ("type", "major-snapshot"),
  ("execution", "async"),
  ("execution", "sync"),
  ("flavor", "regular"),
  ("flavor", "last-resort"),
];

/// InvokeGC 分派动作表 (原版 switch(GCType) × switch(Flavor), 1:1)
pub const INVOKE_ACTIONS: [(&str, &str); 4] = [
  (
    "Minor",
    "CollectGarbage(NEW_SPACE, kTesting, kGCCallbackFlagForced)",
  ),
  (
    "Major+Regular",
    "PreciseCollectAllGarbage(kNoFlags, kTesting, kGCCallbackFlagForced)",
  ),
  ("Major+LastResort", "CollectAllAvailableGarbage(kTesting)"),
  (
    "MajorWithSnapshot",
    "PreciseCollectAllGarbage + TakeSnapshotToFile(filename)",
  ),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(GcType::Minor as u8 == 0);
  assert!(GcType::Major as u8 == 1);
  assert!(GcType::MajorWithSnapshot as u8 == 2);
  assert!(ExecutionType::Async as u8 == 0);
  assert!(ExecutionType::Sync as u8 == 1);
  assert!(Flavor::Regular as u8 == 0);
  assert!(Flavor::LastResort as u8 == 1);
};

// —— 签名骨架 (cc 定义 + h 声明, 1:1) ——
// GCOptions: { type: GcType, execution: ExecutionType, flavor: Flavor, filename: String }
//   GetDefault() = (Major, Sync, Regular, heap_snapshot_path || 默认名)
//   GetDefaultForTruthyWithoutOptionsBag() = (Minor, Sync, Regular, …)
// 匿名 namespace:
//   ReadProperty(isolate, ctx, object, key) -> MaybeLocal<String>
//   ParseType(isolate, maybe_type, options, found_options_object)
//   ParseExecution(isolate, maybe_execution, options, found_options_object)
//   ParseFlavor(isolate, maybe_flavor, options, found_options_object)
//   Parse(isolate, info) -> Maybe<GCOptions> — TryCatch 保异常, filename 仅 MajorWithSnapshot 生效
//   InvokeGC(isolate, gc_options) — EmbedderStackStateScope 按 sync/async 选 StackState
//   AsyncGC : CancelableTask — RunInternal 内 InvokeGC + resolver->Resolve(undefined)
// 扩展本体:
//   GetNativeFunctionTemplate(isolate, name) — ConstructorBehavior::kThrow
//   GC(info) — 0 参直接 InvokeGC(GetDefault); 异步走 PostNonNestableTask(AsyncGC)

// 平台分支台账:
//   无平台 #if; AsyncGC 析构的 kCanceled 分支依赖 Isolate 拆卸语义 — 语义承接点, 非本模块落地
