#!/usr/bin/env bun
// tool/src/base/sys-info.h.js — 覆盖 v8/src/base/sys-info.{h,cc}
// 解析 SysInfo 4 静态方法与每方法平台分派矩阵 (sysctl/sysconf/GetNativeSystemInfo/rlimit)
// → v8-base/src/sys_info.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/sys-info.h");
const cc = readV8("src/base/sys-info.cc");

for (const anchor of [
  [h, "V8_BASE_SYS_INFO_H_"],
  [h, "class V8_BASE_EXPORT SysInfo final {"],
  [h, "static int NumberOfProcessors();"],
  [h, "static int64_t AmountOfPhysicalMemory();"],
  [h, "static int64_t AmountOfVirtualMemory();"],
  [h, "static uintptr_t AddressSpaceEnd();"],
  [cc, "long result = sysconf(_SC_NPROCESSORS_ONLN);"],
  [cc, "long pages = sysconf(_SC_PHYS_PAGES);"],
  [cc, "int result = getrlimit(RLIMIT_DATA, &rlim);"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: 每函数的平台分支
const fnPlatforms = {};
for (const m of cc.matchAll(/^(\w[\w:<>* ]*?) SysInfo::(\w+)\(\) \{([\s\S]*?)\n\}/gm)) {
  const plats = [...m[3].matchAll(/#elif (V8_OS_\w+)|#if (V8_OS_\w+)/g)].map((x) => x[1] || x[2]);
  fnPlatforms[m[2]] = [...new Set(plats)];
}

const body = `${banner("src/base/sys-info.h.js", "src/base/sys-info.h + .cc", "系统信息静态面")}
//! v8 侧: 静态方法 ${Object.keys(fnPlatforms).length}; 平台分派矩阵见下
//! Rust 策略: std::thread::available_parallelism / sysinfo crate 或 libc 直调

pub struct SysInfo;

impl SysInfo {
    /// v8: NumberOfProcessors — 逻辑核数; sysconf(_SC_NPROCESSORS_ONLN), 失败 1
    pub fn number_of_processors() -> i32 { /* 语义: available_parallelism */ todo!() }

    /// v8: AmountOfPhysicalMemory — 物理内存字节; POSIX: _SC_PHYS_PAGES * _SC_PAGESIZE
    pub fn amount_of_physical_memory() -> i64 { todo!() }

    /// v8: AmountOfVirtualMemory — 进程虚拟内存上限; getrlimit(RLIMIT_DATA), 无限→0
    pub fn amount_of_virtual_memory() -> i64 { todo!() }

    /// v8: AddressSpaceEnd — 可寻址上界+1; WIN: lpMaximumApplicationAddress+1; 其他: usize::MAX
    /// v8 注: 不查 RLIMIT_AS (它限映射大小不限地址)
    pub fn address_space_end() -> usize { todo!() }
}

// ---- 平台分派对账 (v8 cc 解析) ----
${Object.entries(fnPlatforms).map(([fn, plats]) => `// ${fn}: ${plats.join(" / ")}`).join("\n")}
// 语义: Rust 侧按 target cfg 分派, Darwin sysctl(HW_MEMSIZE)/FreeBSD vm.stats.win32 GlobalMemoryStatusEx
// 等低频平台仅在需要时补 — Developer 决定
`;

writeSrc("v8-base", "sys_info.rs.skel", body);
console.log(`sys-info.h.js: methods=${Object.keys(fnPlatforms).length} platforms=${Object.values(fnPlatforms).flat().length}`);
