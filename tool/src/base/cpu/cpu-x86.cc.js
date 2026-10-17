#!/usr/bin/env bun
// tool/src/base/cpu/cpu-x86.cc.js — 覆盖 v8/src/base/cpu/cpu-x86.cc
// x64 (IA32/X64) CPUID 特征位表机械提取: has_x_ = (reg & MASK) != 0
// → v8-base/src/cpu_x86.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const ccSrc = readV8("src/base/cpu/cpu-x86.cc");

// 1. 特征位表: has_<name>_ = (cpu_infoN[K] & 0xMASK) != 0
const feats = [...ccSrc.matchAll(/has_(\w+)_ = \(cpu_info(\d*)\[(\d)\] & (0x[0-9a-fA-F]+|\(1 << \d\)|1 << \d+)\) != 0;/g)].map((m) => ({
  name: m[1], src: `cpu_info${m[2]}`, idx: Number(m[3]), mask: m[4],
}));
// 锚点: 首项 fpu (leaf1 EDX bit0), 末项 cetss (leaf7 ECX bit7)
if (feats.length < 20) throw new Error(`特征位数异常: ${feats.length}`);
if (feats[0].name !== "fpu" || feats[0].mask !== "0x00000001") throw new Error("fpu 首项锚点不符");
if (!feats.some((f) => f.name === "cetss")) throw new Error("缺 cetss 位");
if (feats[feats.length - 1].name !== "non_stop_time_stamp_counter") throw new Error("末项锚点不符 (non_stop_tsc)");

// 寄存器来源分组 (leaf 归属由上下文决定, 机械登记)
const srcGroups = {};
for (const f of feats) (srcGroups[f.src] ??= []).push(f.name);
const expectedSrcs = ["cpu_info", "cpu_info70", "cpu_info71"]; // cpu_info24 (avx10_1) 为比较式, 归 NON_MECHANICAL
for (const s of expectedSrcs) {
  if (!srcGroups[s]) throw new Error(`缺寄存器来源组 ${s}`);
}

// 2. 身份字段公式 (stepping/model/family/type/ext_model)
const idFields = [...ccSrc.matchAll(/\n    (\w+)_ = [( ]*cpu_info\[0\][^;\n]*;/g)].map((m) => m[1]);
const idAll = [...new Set(idFields)];
for (const need of ["stepping", "model", "family", "type", "ext_model"]) {
  if (!idAll.includes(need)) throw new Error(`缺身份字段 ${need}`);
}

// 3. 非机械段登记 (avx10_1 比较 / Intel JCC erratum 型号表)
const nonMech = [
  "has_avx10_1_ = (cpu_info24[1] & 0xff) >= 1 (比较式, 非位测)",
  "has_intel_jcc_erratum_ (型号/stepping 表, 语义判定)",
];
if (!ccSrc.includes("has_intel_jcc_erratum_")) throw new Error("缺 JCC erratum 段锚点");

console.log(`cpu-x86 特征位 ${feats.length} (来源组 ${Object.keys(srcGroups).length}), 身份字段 ${idAll.length}`);

let rust = banner("src/base/cpu/cpu-x86.cc.js", "src/base/cpu/cpu-x86.cc", "x64 CPUID 特征位表 (V8_HOST_ARCH_X64 分支; IA32/MSVC/PIC 汇编变体登记不产码)");

rust += `
/// CPUID 特征位表 (原版 has_<name>_ 位测, 列: 特征|寄存器|下标|掩码)
/// 寄存器来源: cpu_info=leaf1 EAX 页, cpu_info70=leaf7, cpu_info71=leaf7 子页1, cpu_info24=leaf0x24
pub const CPUID_FEATURES: [(&str, &str, u32, u32); ${feats.length}] = [
`;
for (const f of feats) {
  rust += `  ("${f.name}", "${f.src}", ${f.idx}, ${f.mask}),\n`;
}
rust += `];

/// 身份字段 (原版 DetectFeatures 内公式, EAX 分段提取)
/// stepping=EAX[3:0], model=EAX[7:4]+EAX[19:16]<<4, family=EAX[11:8], type=EAX[13:12], ext_model=EAX[19:16]
pub const ID_FIELDS: [&str; ${idAll.length}] = [
${idAll.map((n) => `  "${n}",`).join("\n")}
];

/// 按表测位 (对 CPUID 结果页应用单条特征)
#[inline]
#[must_use]
pub fn has_feature(regs: &[(u32; 4)], feature: &str) -> bool {
  let Some((_, src, idx, mask)) = CPUID_FEATURES.iter().find(|f| f.0 == feature) else {
    return false;
  };
  let page = match *src {
    "cpu_info" => 0,
    "cpu_info70" => 1,
    "cpu_info71" => 2,
    "cpu_info24" => 3,
    _ => return false,
  };
  regs.get(page).is_some_and(|r| r[*idx as usize] & mask != 0)
}

/// 非机械段登记 (rule: 不编造, 语义归 Developer)
/// ${nonMech.join(" / ")}
pub const NON_MECHANICAL: [&str; 2] = ["avx10_1_compare", "intel_jcc_erratum_table"];

// —— 提取不变式断言 ——
const _: () = {
  assert!(CPUID_FEATURES.len() == ${feats.length});
  assert!(CPUID_FEATURES[0].0 == "fpu");
  assert!(CPUID_FEATURES[0].3 == 0x1);
  assert!(ID_FIELDS.contains(&"stepping"));
};
`;

const p = writeSrc("v8-base", "cpu_x86.rs", rust);
console.log(`written ${p}`);
