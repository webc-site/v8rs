#!/usr/bin/env bun
// tool/src/base/platform/platform-linux.js — 覆盖 v8/src/base/platform/platform-linux.{h,cc}
// linux 面: /proc/self/maps 解析 (MemoryRegion + SignalSafeMapsParser) + OS 9 方法
// → v8-base-platform/src/platform_linux.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/platform/platform-linux.h");
const ccSrc = readV8("src/base/platform/platform-linux.cc");

// 1. MemoryRegion 结构面: 8 字段 + 路径容量
const hAnchors = [
  "struct V8_BASE_EXPORT MemoryRegion {",
  "uintptr_t start;",
  "uintptr_t end;",
  "char raw_permissions[5];",
  "PagePermissions permissions;",
  "uintptr_t offset;",
  "dev_t dev;",
  "ino_t inode;",
  "static constexpr size_t kMaxPathnameSize = 128;",
  "char pathname[kMaxPathnameSize];",
  "class V8_BASE_EXPORT SignalSafeMapsParser {",
  "static constexpr size_t kBufferSize = 1024;",
  "std::optional<MemoryRegion> Next();",
  "bool ReadChar(char* out);",
  "bool ReadHex(uintptr_t* out_val, char* out_delim);",
  "bool ReadDecimal(uintptr_t* out_val, char* out_delim);",
  "V8_BASE_EXPORT std::vector<OS::SharedLibraryAddress> GetSharedLibraryAddresses(\n    FILE* fp);",
];
for (const a of hAnchors) {
  if (!hSrc.includes(a)) throw new Error(`platform-linux.h 缺锚点: ${a}`);
}
const pathSize = Number(hSrc.match(/kMaxPathnameSize = (\d+);/)?.[1]);
const bufSize = Number(hSrc.match(/kBufferSize = (\d+);/)?.[1]);
if (pathSize !== 128 || bufSize !== 1024) throw new Error("kMaxPathnameSize/kBufferSize 值不符");

// 2. cc OS 面: 9 唯一方法 (信号安全解析 + mremap 重映射)
const osFns = new Set([...ccSrc.matchAll(/OS::(\w+)\(/g)].map((m) => m[1]));
if (osFns.size !== 9) throw new Error(`OS:: 唯一方法数不符: ${osFns.size}, 期望 9`);
const ccAnchors = [
  "TimezoneCache* OS::CreateTimezoneCache() {",
  "void OS::SignalCodeMovingGC() {",
  "void* addr = mmap(OS::GetRandomMmapAddr(), size, PROT_READ | PROT_EXEC,",
  "FILE* f = fopen(OS::GetGCFakeMMapFile(), \"w+\");",
  "void* OS::RemapShared(void* old_address, void* new_address, size_t size) {",
  "mremap(old_address, 0, size, MREMAP_FIXED | MREMAP_MAYMOVE, new_address);",
  "bool OS::RemapPages(const void* address, size_t size, void* new_address,",
];
for (const a of ccAnchors) {
  if (!ccSrc.includes(a)) throw new Error(`platform-linux.cc 缺锚点: ${a}`);
}
// GetFirstFreeMemoryRangeWithin: /proc/self/maps 扫描找空闲区
if (!ccSrc.includes("std::optional<OS::MemoryRange> OS::GetFirstFreeMemoryRangeWithin(")) throw new Error("GetFirstFreeMemoryRangeWithin 缺失");

// 3. 测试通道: GetSharedLibraryAddresses(fp) 重载 (供伪造 maps)
const fpOverload = ccSrc.match(/std::vector<OS::SharedLibraryAddress> GetSharedLibraryAddresses\(FILE\* fp\) \{/);
if (!fpOverload) throw new Error("fp 重载缺失");

console.log(`platform-linux: OS:: ${osFns.size}, MemoryRegion 字段 8, 路径容量 ${pathSize}, 缓冲 ${bufSize}`);

let rust = banner("src/base/platform/platform-linux.js", "src/base/platform/platform-linux.{h,cc}", "linux 平台面 (/proc/self/maps 信号安全解析 + mremap; 非目标, 登记骨架)");

rust += `
/// /proc/PID/maps 记录 (原版 struct MemoryRegion)
/// v8: src/base/platform/platform-linux.h MemoryRegion
#[derive(Debug, Clone)]
pub struct MemoryRegion {
  pub start: usize,
  pub end: usize,
  /// 原版 char raw_permissions[5] ("rwxp" + NUL)
  pub raw_permissions: [u8; 5],
  pub permissions: crate::platform::PagePermissions,
  pub offset: usize,
  pub dev: u64,
  pub inode: u64,
  pub pathname: [u8; ${pathSize}],
}

/// 路径容量 (原版 kMaxPathnameSize)
/// v8: src/base/platform/platform-linux.h MemoryRegion::kMaxPathnameSize
pub const MAX_PATHNAME_SIZE: usize = ${pathSize};
/// 读缓冲 (原版 SignalSafeMapsParser::kBufferSize)
/// v8: src/base/platform/platform-linux.h SignalSafeMapsParser::kBufferSize
pub const PARSER_BUFFER_SIZE: usize = ${bufSize};

/// 信号安全 maps 解析器 (原版 class SignalSafeMapsParser)
/// v8: src/base/platform/platform-linux.h SignalSafeMapsParser
/// 只用 open/read/close; Next() 逐行取 MemoryRegion;
/// ReadHex/ReadDecimal 借中间字符定位分隔符 — 供信号处理器内查映射

// —— OS 9 方法登记 (linux 非目标, 语义留锚) ——
/// v8: src/base/platform/platform-linux.cc OS::CreateTimezoneCache
/// v8: src/base/platform/platform-linux.cc OS::SignalCodeMovingGC
/// 写 GC 假 mmap 文件 + mmap(PROT_READ|PROT_EXEC) 触发剖析器记录代码段移动
/// v8: src/base/platform/platform-linux.cc OS::AdjustSchedulingParams — 空体
/// v8: src/base/platform/platform-linux.cc OS::GetFirstFreeMemoryRangeWithin
/// 扫 /proc/self/maps 找 [boundary_start, boundary_end) 内首个 ≥minimum_size 空洞
/// v8: src/base/platform/platform-linux.cc OS::GetSharedLibraryAddresses
/// 包装 GetSharedLibraryAddresses(fp) 直读 /proc/self/maps
/// v8: src/base/platform/platform-linux.cc GetSharedLibraryAddresses(FILE* fp)
/// 测试通道: fp 伪造 maps 内容 (Visible for testing)
/// v8: src/base/platform/platform-linux.cc OS::RemapShared
/// mremap(old, 0, size, MREMAP_FIXED | MREMAP_MAYMOVE, new) 共享区搬迁
/// v8: src/base/platform/platform-linux.cc OS::RemapPages
/// mremap 原地重映射 (对应 darwin 的 mach_vm_remap)
/// v8: src/base/platform/platform-linux.cc OS::GetGCFakeMMapFile / GetRandomMmapAddr
/// posix 基座转发

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(MAX_PATHNAME_SIZE, 128);
  assert_eq!(PARSER_BUFFER_SIZE, 1024);
  // raw_permissions 定长 5 = 'r','w','x',共享/私有,NUL
  assert_eq!(core::mem::size_of::<[u8; 5]>(), 5);
};
`;

const p = writeSrc("v8-base-platform", "platform_linux.rs.skel", rust);
console.log(`written ${p}`);
