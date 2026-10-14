// 由 tool/src/base/platform/platform.h.js 从 v8/src/base/platform/platform.h 生成; 修订就地改, 复跑覆盖
// 平台层枚举与常量 (POSIX 分支, darwin x64/arm64 目标)

/// 内存保护权限 (原版 enum class MemoryPermission)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MemoryPermission {
  /// kNoAccess — 不可访问 (PROT_NONE)
  NoAccess = 0,
  /// kRead — 只读
  Read = 1,
  /// kReadWrite — 读写
  ReadWrite = 2,
  /// kReadWriteExecute — 读写执行
  ReadWriteExecute = 3,
  /// kReadExecute — 读执行
  ReadExecute = 4,
  /// kNoAccessWillJitLater — 预留 JIT (w^x 过渡, 原版 TODO 注明移除)
  NoAccessWillJitLater = 5,
}

/// 线程优先级 (原版 enum class Priority)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Priority {
  /// kBestEffort
  BestEffort = 0,
  /// kUserVisible
  UserVisible = 1,
  /// kUserBlocking
  UserBlocking = 2,
  /// kDefault — 保持不变
  Default = 3,
}

/// 栈回溯帧文件模式 (原版 StackFrame::FileMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StackFileMode {
  /// kReadOnly
  ReadOnly = 0,
  /// kReadWrite
  ReadWrite = 1,
}

/// 栈回溯错误哨兵 (原版 kStackWalkError)
pub const STACK_WALK_ERROR: i32 = -1;
/// 栈回溯名字最大长 (原版 kStackWalkMaxNameLen)
pub const STACK_WALK_MAX_NAME_LEN: usize = 256;
/// 栈回溯文本最大长 (原版 kStackWalkMaxTextLen)
pub const STACK_WALK_MAX_TEXT_LEN: usize = 256;

/// GC 假 mmap 时间单位 (原版 msPerSecond)
pub const MS_PER_SECOND: i32 = 1000;

/// 线程名最大长 (原版 kMaxThreadNameLength)
pub const MAX_THREAD_NAME_LENGTH: usize = 16;

/// POSIX 分支线程本地存储键 (原版 using LocalStorageKey = pthread_key_t)
pub type LocalStorageKey = libc::pthread_key_t;

// —— 签名骨架 (原版 v8::base::OS 静态面, 供 Developer 补语义) ——
// Allocate/Deallocate/AllocateShared/DeallocateShared/HasLazyCommits
// Sleep(TimeDelta) / Abort() [noreturn] / DebugBreak()
// GetGCFakeMMapFile / CreateSharedMemoryHandleForTesting
// RemapPage / HasMemoryProtectionKeySupport 等签名见原版 260-450 行
pub const OS_SIGNATURES: [&str; 10] = [
  "allocate", "deallocate", "allocate_shared", "deallocate_shared",
  "has_lazy_commits", "sleep", "abort", "debug_break",
  "get_gc_fake_mmap_file", "stack_walk",
];

// —— TLS 分支登记 (rule 9: 按目标展开) ——
// darwin x64: __APPLE__ && V8_HOST_ARCH_X64 → fast TLS 走 pthread tsd 动态段 (原版 110-130 行)
// arm64: fast TLS 不可用 (寄存器基址跨版本不稳定) → LocalStorageKey 兜底
// win32/ia32 TIB 内联分支 (__readfsdword, kTibInlineTlsOffset=0xE10) — 非目标, 登记跳过
pub const FAST_TLS_DARWIN_X64: bool = true;
pub const FAST_TLS_ARM64: bool = false;

// —— 提取不变式断言 ——
const _: () = {
  assert!(STACK_WALK_ERROR == -1);
  assert!(MS_PER_SECOND == 1000);
  assert!(MAX_THREAD_NAME_LENGTH == 16);
  assert!(!FAST_TLS_ARM64);
};
