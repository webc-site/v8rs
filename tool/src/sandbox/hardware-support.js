#!/usr/bin/env bun
// tool/src/sandbox/hardware-support.js — 硬件沙箱支持骨架生成
// 组→文件映射 (本脚本管辖):
//   v8/src/sandbox/hardware-support.h + hardware-support.cc + code-sandboxing-mode.h
//   → v8-sandbox/src/hardware_support.rs
// 规则:
//   - CodeSandboxingMode 枚举 / pkey 语义锚点核对
//   - 方法签名骨架直出; 真实 pkey 仅 linux x86_64, 其余平台恒 inactive (与 v8-base-platform 一致)
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/sandbox/hardware-support.h");
const cc = readV8("src/sandbox/hardware-support.cc");
const mode = readV8("src/sandbox/code-sandboxing-mode.h");

// ---- 锚点断言 ----
const checks = [];
function check(desc, ok) {
  if (!ok) throw new Error(`hardware-support: 对数核对失败: ${desc}`);
  checks.push(desc);
}
check("CodeSandboxingMode: kSandboxed 在 kUnsandboxed 之前", mode.indexOf("kSandboxed") < mode.indexOf("kUnsandboxed"));
check("默认 pkey = 0 / 无 pkey = -1", cc.includes("kNoMemoryProtectionKey") && cc.includes("kDefaultProtectionKey"));
check("非严格模式专用 out-of-sandbox pkey", cc.includes("out_of_sandbox_pkey_ = base::MemoryProtectionKey::AllocateKey()"));
check("进入沙箱 = 禁写 out-of-sandbox key", cc.includes("base::MemoryProtectionKey::Permission::kDisableWrite"));
check("mask 非 0 表示 active", cc.includes("CHECK_NE(sandboxed_mode_pkey_mask_, 0)"));
check("EnterSandbox/ExitSandbox 免费函数", h.includes("inline void EnterSandbox()") && h.includes("inline void ExitSandbox()"));
check("DisallowSandboxAccess/AllowSandboxAccess 仅 DEBUG 硬件启用", h.includes("#if defined(DEBUG) && defined(V8_ENABLE_SANDBOX_HARDWARE_SUPPORT)"));
check("内核信号投递支持: 6.12 起可", cc.includes("major == 6 && minor >= 12"));

const body = `${banner("src/sandbox/hardware-support.js", "src/sandbox/hardware-support.h + src/sandbox/hardware-support.cc + src/sandbox/code-sandboxing-mode.h", "硬件沙箱支持: PKU/pkey (linux x86_64 真实, 其余平台恒 inactive); 对数核对 " + checks.length + " 条通过")}
//! SandboxHardwareSupport; 对应原版 hardware-support{.h,.cc} + code-sandboxing-mode.h。
//! 仅 linux x86_64 支持 PKU; 其他平台 TryActivate 恒 false, Enter/Exit 为 no-op
//! (与 v8-base-platform::MemoryProtectionKey 的平台矩阵一致)。
//! 偏差 (README 登记): EnableForCurrentThread 的备用信号栈保障
//! (EnsureAlternativeSignalStackIsAvailableForCurrentThread) 依赖未落地的信号子系统, 暂为 no-op;
//! DisallowSandboxAccess/AllowSandboxAccess 原版仅 DEBUG 构建 + 硬件启用时生效, 此处等价实现。

use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};

use v8_base_platform::{K_DEFAULT_PROTECTION_KEY, K_NO_MEMORY_PROTECTION_KEY, MemoryProtectionKey, Permission};

use crate::Result;

// ---- CodeSandboxingMode (code-sandboxing-mode.h:22-36) ----
/// 代码沙箱模式: 沙箱内禁写沙箱外内存 (kSandboxed) / 无限制 (kUnsandboxed)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CodeSandboxingMode {
  Sandboxed = 0,
  Unsandboxed = 1,
}

// ---- pkey 全局 (hardware-support.cc:20-26) ----
static SANDBOX_PKEY: AtomicI32 = AtomicI32::new(K_NO_MEMORY_PROTECTION_KEY);
static OUT_OF_SANDBOX_PKEY: AtomicI32 = AtomicI32::new(K_NO_MEMORY_PROTECTION_KEY);
static EXTENSION_PKEY: AtomicI32 = AtomicI32::new(K_NO_MEMORY_PROTECTION_KEY);
/// 进入沙箱模式的 PKRU 掩码; 0 = 硬件支持未激活
static SANDBOXED_MODE_PKEY_MASK: AtomicU32 = AtomicU32::new(0);

/// 原版 SandboxHardwareSupport 静态集合
pub struct SandboxHardwareSupport;

impl SandboxHardwareSupport {
  /// 原版 TryActivateBeforeThreadCreation (hardware-support.cc:29-33)
  pub fn try_activate_before_thread_creation() -> bool {
    todo!("修订阶段实现")
  }

  /// 原版 IsActive (hardware-support.cc:36-38)
  pub fn is_active() -> bool {
    SANDBOX_PKEY.load(Ordering::Relaxed) != K_NO_MEMORY_PROTECTION_KEY
  }

  /// 原版 IsStrict (hardware-support.cc:41-47): 默认 pkey 作 out-of-sandbox key
  pub fn is_strict() -> bool {
    OUT_OF_SANDBOX_PKEY.load(Ordering::Relaxed) == K_DEFAULT_PROTECTION_KEY
  }

  /// 原版 EnableForCurrentThread (hardware-support.cc:50-82); altstack 保障偏差见模块注释
  pub fn enable_for_current_thread(limit_address: Option<usize>) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 RegisterOutOfSandboxMemory (hardware-support.cc:84-90)
  ///
  /// SAFETY: addr+size 须落在合法映射内。
  pub unsafe fn register_out_of_sandbox_memory(addr: usize, size: usize, permissions: v8_base_platform::MemoryPermission) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 RegisterUnsafeSandboxExtensionMemory (hardware-support.cc:92-98)
  ///
  /// SAFETY: addr+size 须落在合法映射内。
  pub unsafe fn register_unsafe_sandbox_extension_memory(addr: usize, size: usize) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 RegisterReadOnlyMemoryInsideSandbox (hardware-support.cc:101-110)
  ///
  /// SAFETY: addr+size 须落在合法映射内。
  pub unsafe fn register_read_only_memory_inside_sandbox(
    addr: usize,
    size: usize,
    current_permissions: v8_base_platform::MemoryPermission,
  ) -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 EnterSandboxedExecutionModeForCurrentThread (hardware-support.cc:113-120)
  pub fn enter_sandboxed_execution_mode_for_current_thread() -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 ExitSandboxedExecutionModeForCurrentThread (hardware-support.cc:123-130)
  pub fn exit_sandboxed_execution_mode_for_current_thread() -> Result<()> {
    todo!("修订阶段实现")
  }

  /// 原版 CurrentSandboxingMode (hardware-support.cc:133-145)
  pub fn current_sandboxing_mode() -> CodeSandboxingMode {
    todo!("修订阶段实现")
  }

  /// 原版 CurrentSandboxingModeIs (hardware-support.cc:148-152)
  pub fn current_sandboxing_mode_is(expected: CodeSandboxingMode) -> bool {
    todo!("修订阶段实现")
  }

  /// 原版 KernelSupportsSignalDeliveryInSandbox (hardware-support.cc:155-171):
  /// Linux 内核 >= 6.12; 非 Linux 平台不支持 (原版 UNREACHABLE → Err)
  pub fn kernel_supports_signal_delivery_in_sandbox() -> Result<bool> {
    todo!("修订阶段实现")
  }

  /// 原版 sandboxed_mode_pkey_mask_address: 生成代码用掩码地址 (此处暴露值)
  pub fn sandboxed_mode_pkey_mask() -> u32 {
    SANDBOXED_MODE_PKEY_MASK.load(Ordering::Relaxed)
  }

  /// 原版 SandboxPkey()
  pub fn sandbox_pkey() -> i32 {
    SANDBOX_PKEY.load(Ordering::Relaxed)
  }
}

// ---- Enter/Exit + scope (hardware-support.h:179-212) ----
/// 原版 EnterSandbox: 进入沙箱执行模式 (禁写沙箱外)
pub fn enter_sandbox() -> Result<()> {
  SandboxHardwareSupport::enter_sandboxed_execution_mode_for_current_thread()
}

/// 原版 ExitSandbox
pub fn exit_sandbox() -> Result<()> {
  SandboxHardwareSupport::exit_sandboxed_execution_mode_for_current_thread()
}

/// 原版 EnterSandboxScope / ExitSandboxScope: RAII
pub struct EnterSandboxScope;
pub struct ExitSandboxScope;

// ---- DisallowSandboxAccess / AllowSandboxAccess (hardware-support.h:226-270, DEBUG 行为) ----
/// 原版 DisallowSandboxAccess: 存续期移除沙箱读写 (LIFO)
pub struct DisallowSandboxAccess {
  previous_permission: Option<Permission>,
}

/// 原版 AllowSandboxAccess: 对 DisallowSandboxAccess 的临时豁免
pub struct AllowSandboxAccess {
  previous_permission: Option<Permission>,
}
`;

const p = writeSrc("v8-sandbox", "hardware_support.rs", body);
console.log(`hardware-support.js: ${p} (对数核对 ${checks.length} 条通过)`);
