#!/usr/bin/env bun
// tool/src/base/cpu/cpu-ppc.cc.js — 族: src/base/cpu/cpu-ppc.cc
// 覆盖: /proc/self/auxv 探测 (AT_PLATFORM/AT_ICACHEBSIZE/AT_DCACHEBSIZE)
//        + power9/10/11 判别 + AIX _system_configuration 分支
// → v8-base/src/cpu_ppc.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/cpu/cpu-ppc.cc");

// —— 锚点断言 ——
if (!src.includes('base::Fopen("/proc/self/auxv", "r")')) throw new Error("缺 auxv 打开");
for (const a of ["AT_PLATFORM", "AT_ICACHEBSIZE", "AT_DCACHEBSIZE"]) {
  if (!src.includes(`case ${a}:`)) throw new Error(`缺 auxv 项 ${a}`);
}
if (!src.includes("entry.a_type == AT_NULL")) throw new Error("缺 AT_NULL 终止条件");
if (!src.includes('strcmp(auxv_cpu_type, "power11") == 0')) throw new Error("缺 power11 判别");
if (!src.includes('strcmp(auxv_cpu_type, "power10") == 0')) throw new Error("缺 power10 判别");
if (!src.includes('strcmp(auxv_cpu_type, "power9") == 0')) throw new Error("缺 power9 判别");
if (!src.includes("switch (_system_configuration.implementation)")) throw new Error("缺 AIX 分支");
if (!src.includes("#ifndef POWER_9")) throw new Error("缺 POWER_9 兜底定义");

// —— 真解析: auxv 开关项计数 ——
const auxvCases = (src.match(/case AT_\w+:/g) || []).length;
if (auxvCases !== 3) throw new Error(`auxv case ${auxvCases} != 3`);
console.log("cpu-ppc: auxv 项 3, 平台分支 2 (Linux auxv / AIX systemcfg), part 判别 3");

let rust = banner(
  "src/base/cpu/cpu-ppc.cc.js",
  "src/base/cpu/cpu-ppc.cc",
  "PPC64 探测 (auxv 缓存行 + power 代际判别)",
);

rust += `
//! v8: src/base/cpu/cpu-ppc.cc

impl Cpu {
    /// v8: cpu-ppc.cc DetectFeatures (Linux, 非模拟器)
    /// 遍历 /proc/self/auxv 的 Elf64_auxv_t:
    ///   AT_PLATFORM    → 字符串指针, "power11"/"power10"/"power9" → part_ = kPPCPower{11,10,9}
    ///   AT_ICACHEBSIZE → icache_line_size_
    ///   AT_DCACHEBSIZE → dcache_line_size_
    ///   AT_NULL        → 终止; part_ 无匹配时 -1
    /// AIX: _system_configuration.implementation ∈ {POWER_10, POWER_9} (0x40000/0x20000 兜底宏)
    #[cfg(target_arch = "powerpc64")]
    fn detect_features(&mut self) {
        // Developer: auxv 解析循环 + part 映射
    }
}
`;

writeSrc("v8-base", "cpu_ppc.rs.skel", rust);
console.log("written v8-base/src/cpu_ppc.rs.skel");
