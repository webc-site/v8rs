#!/usr/bin/env bun
// tool/src/base/cpu/cpu-mips64.cc.js — 族: src/base/cpu/cpu-mips64.cc
// 覆盖: /proc/cpuinfo 探测 FPU 与 MSA
// → v8-base/src/cpu_mips64.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/cpu/cpu-mips64.cc");

// —— 锚点断言 ——
if (!src.includes('char* cpu_model = cpu_info.ExtractField("cpu model");')) throw new Error("缺 cpu model 提取");
if (!src.includes('has_fpu_ = HasListItem(cpu_model, "FPU");')) throw new Error("缺 FPU 判定");
if (!src.includes('char* ASEs = cpu_info.ExtractField("ASEs implemented");')) throw new Error("缺 ASEs 提取");
if (!src.includes('has_msa_ = HasListItem(ASEs, "msa");')) throw new Error("缺 MSA 判定");
if (!src.includes("#if V8_HOST_ARCH_MIPS64")) throw new Error("缺架构守卫");
console.log("cpu-mips64: /proc/cpuinfo 字段 2 (cpu model / ASEs implemented), 特性位 2 (FPU/MSA)");

let rust = banner(
  "src/base/cpu/cpu-mips64.cc.js",
  "src/base/cpu/cpu-mips64.cc",
  "MIPS64 FPU/MSA 探测 (纯 cpuinfo, 无指令级探测)",
);

rust += `
//! v8: src/base/cpu/cpu-mips64.cc

impl Cpu {
    /// v8: cpu-mips64.cc DetectFeatures — Linux /proc/cpuinfo:
    ///   has_fpu = "cpu model" 字段含 "FPU"
    ///   has_msa = "ASEs implemented" 字段含 "msa"
    /// 注: 原版注释 — MIPS 无统一探测设施, 各 OS 自行提供
    #[cfg(target_arch = "mips64")]
    fn detect_features(&mut self) {
        // Developer: CpuInfo::extract_field + has_list_item 组合
    }
}
`;

writeSrc("v8-base", "cpu_mips64.rs.skel", rust);
console.log("written v8-base/src/cpu_mips64.rs.skel");
