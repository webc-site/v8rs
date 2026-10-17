#!/usr/bin/env bun
// tool/src/base/cpu/cpu.h.js — 族: src/base/cpu/cpu.h + src/base/cpu/cpu.cc
// 覆盖: CPU 类 (跨架构特性查询面) — 常量 16 + 枚举 2 + 特性谓词 ~60 + 字段 ~65
//        + cc 侧 /proc/cpuinfo 探测 (StarboardDetectCPU) 与 GetInstance 惰性单例
// 注意: cpu-arm.cc / cpu-x86.cc / cpu-helper.h 归既有脚本 (cpu-arm.cc.js 等), 不重做
// → v8-base/src/cpu.rs.skel

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/base/cpu/cpu.h");
const ccSrc = readV8("src/base/cpu/cpu.cc");

// —— 锚点断言 ——
if (!hSrc.includes("class V8_BASE_EXPORT CPU final {")) throw new Error("缺 CPU 类");
if (!hSrc.includes("static const CPU& GetInstance();")) throw new Error("缺 GetInstance");
if (!hSrc.includes("void DetectFeatures();")) throw new Error("缺 DetectFeatures");
if (!ccSrc.includes('static const char PATHNAME[] = "/proc/cpuinfo";')) throw new Error("缺 cpuinfo 路径");
if (!ccSrc.includes("bool CPU::StarboardDetectCPU() {")) throw new Error("缺 Starboard 探测");
if (!ccSrc.includes("static const CPU cpu;")) throw new Error("缺惰性单例");

// —— 真解析: 常量/枚举/字段对数 ——
const consts = [...hSrc.matchAll(/^  static constexpr (int|unsigned) (k\w+) = (0x[\da-fA-F]+|\d+);/gm)].map((m) => [m[2], m[3]]);
if (consts.length !== 14) throw new Error(`constexpr 常量 ${consts.length} != 14`);
const ppcEnum = hSrc.match(/enum \{ kPPCPower9, kPPCPower10, kPPCPower11 \};/);
if (!ppcEnum) throw new Error("缺 PPC 枚举");
const rvEnumBody = hSrc.match(/enum class RV_MMU_MODE \{([\s\S]*?)\};/)[1];
const rvVariants = [...rvEnumBody.matchAll(/^\s{4}(k\w+),?$/gm)].map((m) => m[1]);
if (rvVariants.join(",") !== "kRiscvSV39,kRiscvSV48,kRiscvSV57") throw new Error(`RV_MMU_MODE ${rvVariants}`);

// 谓词 (has_*/is_*) 与字段计数
const predicates = [...hSrc.matchAll(/^  (?:unsigned )?(bool|int) (has_\w+|is_\w+|vlen|riscv_mmu)\(\) const/gm)].map((m) => m[2]);
if (predicates.length < 58) throw new Error(`谓词数 ${predicates.length} < 58`);
const fields = (hSrc.match(/^  (?:char \w+\[13\]|int|bool|unsigned|RV_MMU_MODE) \w+_ =/gm) || []).length;
console.log(`cpu: constexpr 常量 ${consts.length}, 枚举 2 (PPC 3 值 + RV_MMU_MODE 3 值), 谓词 ${predicates.length}, 字段 ${fields}`);

let rust = banner(
  "src/base/cpu/cpu.h.js",
  "src/base/cpu/cpu.h + src/base/cpu/cpu.cc",
  "处理器特性查询 (类定义跨架构, 实现分 cpu_<arch>.cc)",
);

rust += `
//! v8: src/base/cpu/cpu.h/.cc
//! 架构实现分文件: cpu-x86.cc / cpu-arm.cc (既有脚本), cpu-loong64/mips64/ppc/riscv/s390.cc (本席另脚本)

/// v8: src/base/cpu/cpu.h CPU
/// vendor_: char[13] (12 字符 + NUL); 其余数值字段见下方常量;
/// DetectFeatures() 私有探测入口 (构造时调); GetInstance: static const CPU 惰性单例
#[derive(Debug, Clone)]
pub struct Cpu {
    pub vendor: [u8; 13],
    pub stepping: i32, pub model: i32, pub ext_model: i32,
    pub family: i32, pub ext_family: i32, pub type_: i32,
    pub implementer: i32, pub architecture: i32, pub variant: i32, pub part: i32,
    pub icache_line_size: i32, pub dcache_line_size: i32,
    pub num_virtual_address_bits: i32,
    // Developer: 特性位 ~60 项 (x86/arm/mips/riscv/loong 分组), 按 has_xxx_ 字段 1:1 展开
}

impl Cpu {
    /// v8: cpu.cc CPU::CPU() — DetectFeatures() 填充
    pub fn new() -> Self;
    /// v8: cpu.cc GetInstance — static const CPU (函数级静态, 线程安全 magic static)
    pub fn get_instance() -> &'static Cpu;
}

// —— v8: src/base/cpu/cpu.h 常量表 (constexpr) ——
${consts.map(([n, v]) => `pub const ${n.replace(/^k/, "K").toUpperCase()}: i32 = ${v}; // v8: cpu.h ${n}`).join("\n")}

/// v8: cpu.h RV_MMU_MODE — riscv_mmu_ 默认 kRiscvSV48
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RvMmuMode {
    #[default]
    KRiscvSv39,
    KRiscvSv48,
    KRiscvSv57,
}

// v8: cpu.h enum { kPPCPower9, kPPCPower10, kPPCPower11 } — 匿名枚举序数 0/1/2
pub const PPC_POWER9: i32 = 0;
pub const PPC_POWER10: i32 = 1;
pub const PPC_POWER11: i32 = 2;

// v8: cpu.cc — /proc/cpuinfo 探测 (PATHNAME) 走 CPUInfo/ExtractField/HasListItem (cpu-helper.h 既有);
//     StarboardDetectCPU 仅 V8_OS_STARBOARD 构建生效, Rust 登记跳过
`;

writeSrc("v8-base", "cpu.rs.skel", rust);
console.log("written v8-base/src/cpu.rs.skel");
