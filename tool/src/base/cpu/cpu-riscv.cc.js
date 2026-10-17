#!/usr/bin/env bun
// tool/src/base/cpu/cpu-riscv.cc.js — 族: src/base/cpu/cpu-riscv.cc
// 覆盖: riscv_hwprobe syscall (9 扩展位) + isa 串回退 + mmu 模式 + vlen 内建
// → v8-base/src/cpu_riscv.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/base/cpu/cpu-riscv.cc");

// —— 锚点断言 ——
if (!src.includes("__attribute__((target(\"arch=+v\"))) static unsigned vlen_intrinsic()")) throw new Error("缺 vlen 内建");
if (!src.includes("static bool IsaHasFeature(const char* isa, const char* feature)")) throw new Error("缺 isa 解析");
if (!src.includes("riscv_hwprobe pairs[] = {{RISCV_HWPROBE_KEY_IMA_EXT_0, 0}};")) throw new Error("缺 hwprobe 探测");
if (!src.includes("syscall(__NR_riscv_hwprobe")) throw new Error("缺 syscall 调用");
if (!src.includes('char* mmu = cpu_info.ExtractField("mmu");')) throw new Error("缺 mmu 提取");

// —— 真解析: hwprobe 扩展位与特性映射, 对数 ——
const probes = [...src.matchAll(/if \(pairs\[0\]\.value & (RISCV_HWPROBE_\w+)\) \{\s*\n\s*(has_\w+|riscv_\w+) = (true|\w+);/g)].map((m) => `${m[1]}→${m[2]}`);
if (probes.length !== 9) throw new Error(`hwprobe 位 ${probes.length} != 9`);
const isaFallback = [...src.matchAll(/IsaHasFeature\(isa, "(\w+)"\)/g)].map((m) => m[1]);
if (isaFallback.join(",") !== "rv64imafdc,rv64imafdcv") throw new Error(`isa 回退 ${isaFallback}`);
const mmuModes = [...src.matchAll(/HasListItem\(mmu, "sv(\d+)"\)/g)].map((m) => `sv${m[1]}`);
if (mmuModes.join(",") !== "sv48,sv39,sv57") throw new Error(`mmu 模式 ${mmuModes}`);
console.log(`cpu-riscv: hwprobe 位 ${probes.length}, isa 回退 2 串, mmu 模式 ${mmuModes.length} (后判优先), vlen 内建 1`);

let rust = banner(
  "src/base/cpu/cpu-riscv.cc.js",
  "src/base/cpu/cpu-riscv.cc",
  "RISC-V 特性探测 (hwprobe syscall + cpuinfo isa 回退)",
);

rust += `
//! v8: src/base/cpu/cpu-riscv.cc

// v8: hwprobe 位 → 特性映射 (RISCV_HWPROBE_KEY_IMA_EXT_0)
${probes.map((p) => `// v8: ${p}`).join("\n")}

/// v8: cpu-riscv.cc IsaHasFeature — 下划线分隔 isa 串整词匹配 (如 "rv64imafdcv_zba_zbb")
fn isa_has_feature(isa: Option<&str>, feature: &str) -> bool;

impl Cpu {
    /// v8: cpu-riscv.cc DetectFeatures
    /// 主路: __NR_riscv_hwprobe syscall (Linux 6.4+), 9 扩展位见上表
    /// 回退: /proc/cpuinfo "isa" 行 — "rv64imafdc"→fpu, "rv64imafdcv"→fpu+rvv
    /// mmu: "mmu" 字段 sv48/sv39/sv57 顺序判定, 后命中者覆盖 (sv39 后于 sv48, sv57 最后)
    /// vlen: has_rvv_ 时 __riscv_vlenb()*8 (target attribute arch=+v 编译)
    #[cfg(target_arch = "riscv64")]
    fn detect_features(&mut self) {
        // Developer: syscall + 回退 + vlen 组装
    }
}
`;

writeSrc("v8-base", "cpu_riscv.rs.skel", rust);
console.log("written v8-base/src/cpu_riscv.rs.skel");
