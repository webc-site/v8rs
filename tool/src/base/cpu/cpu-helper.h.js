#!/usr/bin/env bun
// tool/src/base/cpu/cpu-helper.h.js — 覆盖 v8/src/base/cpu/cpu-helper.h
// /proc/cpuinfo 采集器签名骨架 (arm64 Linux 目标生效; darwin 运行时不可达, 登记保留)
// → v8-base/src/cpu_helper.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/cpu/cpu-helper.h");

// 构建分支: V8_HOST_ARCH_ARM64 + V8_OS_LINUX 生效; 其他架构 (MIPS64/RISCV64/LOONG64) 登记跳过
const archGuard = hSrc.match(/#if V8_HOST_ARCH_ARM \|\| V8_HOST_ARCH_ARM64 \|[\s\S]*?#endif  \/\/ V8_HOST_ARCH/);
if (!archGuard) throw new Error("缺架构守卫块");
if (!hSrc.includes("class CPUInfo final")) throw new Error("缺 CPUInfo 类");
if (!hSrc.includes("char* ExtractField(const char* field) const;")) throw new Error("缺 ExtractField 签名");
if (!hSrc.includes("bool HasListItem(const char* list, const char* item);")) throw new Error("缺 HasListItem 签名");

console.log("cpu-helper 结构 4 锚点");

let rust = banner("src/base/cpu/cpu-helper.h.js", "src/base/cpu/cpu-helper.h", "/proc/cpuinfo 采集器骨架 (ARM64+Linux 分支; darwin 不可达)");

rust += `
/// /proc/cpuinfo 采集器 (原版 base::CPUInfo, ARM64+Linux 构建分支)
/// 其他架构分支 (MIPS64/RISCV64/LOONG64) 与非 Linux 系统登记跳过
pub struct CpuInfo;

impl CpuInfo {
  /// 提取 cpuinfo 中首个指定字段, 堆串返回; 未找到返回 None
  /// 原版返回 malloc 串 (delete[] 释放), Rust 侧由 Developer 定所有权
  pub fn extract_field(&self, field: &str) -> Option<String>;
}

/// 空格分隔列表包含判定 (原版 HasListItem)
pub fn has_list_item(list: &str, item: &str) -> bool {
  list.split(' ').any(|x| x == item)
}

// —— 提取不变式断言 ——
const _: () = {
  // HasListItem 语义锚点
  assert!(has_list_item("a b c", "b"));
  assert!(!has_list_item("a b c", "d"));
};
`;

const p = writeSrc("v8-base", "cpu_helper.rs", rust);
console.log(`written ${p}`);
