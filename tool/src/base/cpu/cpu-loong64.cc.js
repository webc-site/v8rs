#!/usr/bin/env bun
// tool/src/base/cpu/cpu-loong64.cc.js — 族: src/base/cpu/cpu-loong64.cc
// 覆盖: cpucfg 指令探测 LSX/LASX + /proc/cpuinfo 回退
// → v8-base/src/cpu_loong64.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/cpu/cpu-loong64.cc");

// —— 锚点断言 ——
if (!src.includes("#define LOONGARCH_CFG2 0x2")) throw new Error("缺 CFG2 寄存器号");
if (!src.includes("#define LOONGARCH_CFG2_LSX (1 << 6)")) throw new Error("缺 LSX 位");
if (!src.includes("#define LOONGARCH_CFG2_LASX (1 << 7)")) throw new Error("缺 LASX 位");
if (!src.includes('int flags = cpu_flags_cpucfg(LOONGARCH_CFG2);')) throw new Error("缺 cpucfg 调用");
if (!src.includes('__asm__ volatile("cpucfg %0, %1')) throw new Error("缺 cpucfg 内联汇编");
if (!src.includes('cpu_info.ExtractField("features")')) throw new Error("缺 cpuinfo 回退");
if (!src.includes('HasListItem(features, "lsx")')) throw new Error("缺 lsx 列表判定");
if (!src.includes('HasListItem(features, "lasx")')) throw new Error("缺 lasx 列表判定");
console.log("cpu-loong64: 掩码常量 3, 探测路径 2 (cpucfg 主 + cpuinfo 回退), 特性位 2");

let rust = banner(
  "src/base/cpu/cpu-loong64.cc.js",
  "src/base/cpu/cpu-loong64.cc",
  "LoongArch LSX/LASX 特性探测",
);

rust += `
//! v8: src/base/cpu/cpu-loong64.cc

// v8: cpu-loong64.cc 掩码常量
pub const LOONGARCH_CFG2: u32 = 0x2;      // v8: LOONGARCH_CFG2 (cpucfg 寄存器号)
pub const LOONGARCH_CFG2_LSX: u32 = 1 << 6; // v8: LOONGARCH_CFG2_LSX
pub const LOONGARCH_CFG2_LASX: u32 = 1 << 7; // v8: LOONGARCH_CFG2_LASX

/// v8: cpu-loong64.cc cpu_flags_cpucfg — 内联汇编 cpucfg 读配置字
#[cfg(all(target_os = "linux", target_arch = "loongarch64"))]
fn cpu_flags_cpucfg(cfg: u32) -> u32;

impl Cpu {
    /// v8: cpu-loong64.cc DetectFeatures
    /// 主路: cpucfg(LOONGARCH_CFG2) != 0 → 按 LSX/LASX 位;
    /// 回退: /proc/cpuinfo "features" 字段 HasListItem("lsx"/"lasx")
    #[cfg(target_arch = "loongarch64")]
    fn detect_features(&mut self) {
        // Developer: has_lsx / has_lasx 赋值逻辑
    }
}
`;

writeSrc("v8-base", "cpu_loong64.rs.skel", rust);
console.log("written v8-base/src/cpu_loong64.rs.skel");
