#!/usr/bin/env bun
// tool/src/base/platform/platform.h.js — 覆盖 v8/src/base/platform/platform.h
// 平台层枚举与常量: MemoryPermission / Priority / StackWalk / TLS 分支登记
// → v8-base-platform/src/platform.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/platform.h");

// 1. MemoryPermission 枚举
const memBody = hSrc.match(/enum class MemoryPermission \{([^}]*)\};/);
if (!memBody) throw new Error("未找到 MemoryPermission 枚举");
const mems = [...memBody[1].matchAll(/\bk(\w+)/g)].map((m) => m[1]);
if (mems.length !== 6) throw new Error(`MemoryPermission 条数不符: ${mems.length}, 期望 6`);
if (mems[0] !== "NoAccess" || mems[5] !== "NoAccessWillJitLater") throw new Error("MemoryPermission 锚点不符");

// 2. Priority 枚举
const priBody = hSrc.match(/enum class Priority \{([^}]*)\};/);
if (!priBody) throw new Error("未找到 Priority 枚举");
const pris = [...priBody[1].matchAll(/\bk(\w+)/g)].map((m) => m[1]);
if (pris.join(",") !== "BestEffort,UserVisible,UserBlocking,Default") throw new Error(`Priority 锚点不符: ${pris}`);

// 3. StackWalk 常量与 StackFrame::FileMode
const stackErr = Number(hSrc.match(/static const int kStackWalkError = (-?\d+);/)?.[1]);
const nameLen = Number(hSrc.match(/kStackWalkMaxNameLen = (\d+);/)?.[1]);
const textLen = Number(hSrc.match(/kStackWalkMaxTextLen = (\d+);/)?.[1]);
const fileMode = [...hSrc.match(/enum class FileMode \{([^}]*)\}/)[1].matchAll(/k(\w+)/g)].map((m) => m[1]);
if (stackErr !== -1 || nameLen !== 256 || textLen !== 256) throw new Error("StackWalk 常量对数不符");
if (fileMode.join(",") !== "ReadOnly,ReadWrite") throw new Error("FileMode 锚点不符");

// 4. 其余散常量
const msPerSecond = Number(hSrc.match(/static const int msPerSecond = (\d+);/)?.[1]);
const threadNameLen = Number(hSrc.match(/static const int kMaxThreadNameLength = (\d+);/)?.[1]);
if (msPerSecond !== 1000 || threadNameLen !== 16) throw new Error("msPerSecond/kMaxThreadNameLength 对数不符");

// 5. TLS 分支登记 (x64+arm64 目标):
//    - darwin x64: __APPLE__ && V8_HOST_ARCH_X64 分支 (tsd_inline 动态获取)
//    - arm64: 不做 fast TLS (寄存器基址不稳定, 原版注明), 走 LocalStorageKey 兜底
//    - win32/ia32 TIB 分支与目标无关, 登记不产码
const tlsAppleX64 = hSrc.includes("defined(__APPLE__) && (V8_HOST_ARCH_IA32 || V8_HOST_ARCH_X64)");
const tlsArm64Note = hSrc.includes("Not possible on ARM64");
if (!tlsAppleX64 || !tlsArm64Note) throw new Error("TLS 分支锚点缺失");

console.log(`platform.h: MemoryPermission ${mems.length}, Priority ${pris.length}, StackWalk/FileMode 常量齐`);

let rust = banner("src/base/platform/platform.h.js", "src/base/platform/platform.h", "平台层枚举与常量 (POSIX 分支, darwin x64/arm64 目标)");

rust += `
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
pub const STACK_WALK_MAX_NAME_LEN: usize = ${nameLen};
/// 栈回溯文本最大长 (原版 kStackWalkMaxTextLen)
pub const STACK_WALK_MAX_TEXT_LEN: usize = ${textLen};

/// GC 假 mmap 时间单位 (原版 msPerSecond)
pub const MS_PER_SECOND: i32 = ${msPerSecond};

/// 线程名最大长 (原版 kMaxThreadNameLength)
pub const MAX_THREAD_NAME_LENGTH: usize = ${threadNameLen};

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
`;

const p = writeSrc("v8-base-platform", "platform.rs", rust);
console.log(`written ${p}`);
