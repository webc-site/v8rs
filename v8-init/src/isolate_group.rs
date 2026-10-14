// 由 tool/src/init/isolate-group.js 从 v8/src/init/isolate-group.{h,cc} 生成; 修订就地改, 复跑覆盖
// 隔离组: 指针压缩笼共享单元

// IsolateGroup: 一组共享同一指针压缩笼 (pointer cage) 的 Isolate。
// 压缩关闭时退化为全局单组; JS 对象只能在同组 Isolate 间传递 (API 用户责任)。

/// BasePageTableEntry 哨兵: 元数据可被任意 Isolate 读 (RO/共享页) 时,
/// isolate_ 字段置此值 (原版 constexpr = -1, uintptr 语义)。
pub const READ_ONLY_OR_SHARED_ENTRY_ISOLATE_SENTINEL: u64 = 0xffffffffffffffff;

/// 判定给定 entry 是否为只读或共享页哨兵
pub const fn is_read_only_or_shared_sentinel(entry: u64) -> bool {
  entry == READ_ONLY_OR_SHARED_ENTRY_ISOLATE_SENTINEL
}

/// 能否新建隔离组 = COMPRESS_POINTERS_IN_MULTIPLE_CAGES_BOOL:
/// 目标配置 (x64+arm64, 默认单共享笼) 展开 false; 多笼构建 true (构建期决定, 此处登记)。
pub const CAN_CREATE_NEW_GROUPS: bool = false;

// 公开静态 API 签名骨架 (原版 isolate-group.h):
// - `static AcquireDefault()`
// - `static New()`
// - `static InitializeOncePerProcess()`
// - `static TearDownOncePerProcess()`
// - `static current()`
// - `static void set_current(IsolateGroup* group)`
// - `static current()`
// - `static void set_current(IsolateGroup* group)`
// - `static current()`
// - `static GetDefault()`
// - `static ReleaseDefault()`
// - `static current_non_inlined()`
// - `static void set_current_non_inlined(IsolateGroup* group)`

// cc 实现 (机械提取 IsolateGroup:: 方法名, 共 16 个):
// current_non_inlined, set_current_non_inlined, BasePageTableEntry, default_isolate_group_, Initialize, Initialize, Initialize, InitializeOncePerProcess, TearDownOncePerProcess, Release, SetupReadOnlyHeap, AddIsolate, RemoveIsolate, GetIsolateCount, New, ReleaseDefault
