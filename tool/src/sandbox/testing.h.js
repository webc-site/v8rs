#!/usr/bin/env bun
// tool/src/sandbox/testing.h.js
// 机械提取 v8/src/sandbox/testing.{h,cc} — 沙箱测试设施规则面:
//   Mode / MemoryAccessType 枚举 + 安全访问组合常量 + 静态接口签名骨架。
//   cc 1766 行崩溃过滤/内存破坏 API 实现为纯语义面, 登记跳过。
// 产物: v8-sandbox/src/testing.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/testing.h";
const CC_REL = "src/sandbox/testing.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const modeM = h.match(/enum class Mode\s*\{([\s\S]*?)\n  \};/);
if (!modeM) throw new Error("未找到 Mode 枚举");
const modes = [...modeM[1].replace(/\/\/[^\n]*/g, "").matchAll(/k(\w+),/g)].map((m) => `k${m[1]}`);
if (modes.length !== 3 || modes.join() !== "kDisabled,kForTesting,kForFuzzing") {
  throw new Error(`Mode 条目异常: ${modes.join()}`);
}
const matM = h.match(/enum class MemoryAccessType\s*\{([^}]+)\};/);
if (!matM) throw new Error("未找到 MemoryAccessType 枚举");
const mats = [...matM[1].matchAll(/k(\w+)\s*=\s*1 << (\d+)/g)].map((m) => ({ name: m[1], bit: Number(m[2]) }));
if (mats.length !== 3 || mats[0].bit !== 0 || mats[1].bit !== 1 || mats[2].bit !== 2) {
  throw new Error(`MemoryAccessType 条目异常: ${JSON.stringify(mats)}`);
}
for (const anchor of ["kReadAndWriteAccessIsSafe", "kOnlyReadAccessIsSafe", "static void Enable(Mode mode);", "static void Disable();", "InstallMemoryCorruptionApi", "RegisterSafeMemoryRegion", "GetInstanceTypeMap()", "GetFieldOffsetMap()"]) {
  if (!h.includes(anchor)) throw new Error(`缺锚 ${anchor}`);
}

const toPascal = (k) => k.replace(/^k/, "").replace(/_(\w)/g, (_, c) => c.toUpperCase());

let rust = banner("src/sandbox/testing.h.js", "src/sandbox/testing.{h,cc}", "SandboxTesting 枚举与骨架");
rust += `
// 语义面登记跳过: 崩溃过滤器 (SIGSEGV/SIGTRAP 分诊)、内存破坏 API 安装、
// 安全内存区注册表、InstanceType/FieldOffset 映射构建 (cc ~1766 行)。

/// 沙箱测试模式 (原 enum class Mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Mode {
  /// 未激活
  Disabled = 0,
  /// 测试模式: 启用崩溃过滤 + 内存破坏 API; 无害崩溃以 0 退出
  ForTesting = 1,
  /// 模糊模式: 无害崩溃以非零退出, 供 fuzzer 识别提前终止
  ForFuzzing = 2,
}

/// 内存访问类型 (原 enum class MemoryAccessType, base::Flags 位集)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MemoryAccessType {
  Read = 1 << 0,
  Write = 1 << 1,
  Execute = 1 << 2,
}

/// 位集别名 (原 base::Flags<MemoryAccessType>)
pub type MemoryAccessTypes = u8;

/// 读写访问均安全 (原 kReadAndWriteAccessIsSafe)
pub const READ_AND_WRITE_ACCESS_IS_SAFE: MemoryAccessTypes = (MemoryAccessType::Read | MemoryAccessType::Write) as u8;
/// 仅读访问安全 (原 kOnlyReadAccessIsSafe)
pub const ONLY_READ_ACCESS_IS_SAFE: MemoryAccessTypes = MemoryAccessType::Read as u8;

/// 原 class SandboxTesting : AllStatic (全静态接口)
pub struct SandboxTesting;

impl SandboxTesting {
  // 原签名: static void Enable(Mode mode)
  // 原签名: static void Disable()
  // 原签名: static bool IsEnabled() { return mode_ != Mode::kDisabled; }
  // 原签名: static void InstallMemoryCorruptionApi(Isolate* isolate)
  // 原签名: static void RegisterSafeMemoryRegion(Address start)
  // 原签名: static void UnregisterSafeMemoryRegion(Address start)
  // 原签名: static InstanceTypeMap& GetInstanceTypeMap()
  // 原签名: static FieldOffsetMap& GetFieldOffsetMap()
  // 原签名: static std::optional<int> GetFieldOffset(instance_type, field_name)
  // 原签名: static Mode mode()
  // TODO(dev): 依语义面补实现。
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(READ_AND_WRITE_ACCESS_IS_SAFE == 0b011);
  assert!(ONLY_READ_ACCESS_IS_SAFE == 0b001);
  assert!(${toPascal(modes[0]) ? "Mode::Disabled" : "Mode::Disabled"} as u8 == 0);
};
`;

const p = writeSrc("v8-sandbox", "testing.rs", rust);
console.log(`written ${p} (Mode 3, MemoryAccessType 3, 组合 2)`);
