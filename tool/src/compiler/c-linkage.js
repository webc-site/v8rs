#!/usr/bin/env bun
// v8/src/compiler/c-linkage.cc (单文件, 无 .h)
// → v8-compiler-passes/src/c_linkage.rs.skel
//
// 规则面: 各平台 PARAM_REGISTERS/FP_PARAM_REGISTERS/FP_RETURN_REGISTER/
// CALLEE_SAVE 宏表 (X-macro 展开器), BuildParameterLocations 三变体
// (Win+MIPS64 单计数 / Loong64 混合 / 通用双计数), GetSimplifiedCDescriptor
// 的 CallDescriptor 装配常量 (kCallAddress/kNoAllocate/kInvalidEntrypointTag)。
// 本仓目标 arm64: 取 ARM64 分支寄存器表为主数据。
import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/compiler/c-linkage.cc");

// —— 哨兵 ——
const sentinels = [
  [/void BuildParameterLocations\(/, "BuildParameterLocations"],
  [/CallDescriptor\* Linkage::GetSimplifiedCDescriptor\(/, "GetSimplifiedCDescriptor"],
  [/#elif V8_TARGET_ARCH_ARM64/, "ARM64 分支"],
  [/CallDescriptor::kCallAddress,  \/\/ kind/, "kCallAddress"],
  [/flags \|= CallDescriptor::kNoAllocate;/, "kNoAllocate"],
  [/kInvalidEntrypointTag,         \/\/ tag/, "kInvalidEntrypointTag"],
  [/LinkageLocation::ForCallerFrameSlot\(-1 - stack_offset, type\)/, "caller frame slot"],
  [/LinkageLocation::ForNullRegister\(/, "ForNullRegister (Loong64 特有)"],
];
for (const [re, name] of sentinels) {
  if (!re.test(cc)) throw new Error(`哨兵缺失: ${name}`);
}

// —— 平台寄存器宏表提取: #define PARAM_REGISTERS ... ——
function archBlock(arch) {
  const re = new RegExp(`#elif V8_TARGET_ARCH_${arch}\\n([\\s\\S]*?)(?=#elif V8_TARGET_ARCH|#else|#endif|\\/\\/ General)`);
  const m = cc.match(re);
  if (!m) throw new Error(`缺 ${arch} 分支`);
  return m[1];
}
function defines(block) {
  const out = {};
  for (const m of block.matchAll(/^#define (\w+) (.*)$/gm)) out[m[1]] = m[2].trim();
  return out;
}
const arm64 = defines(archBlock("ARM64"));
if (arm64.PARAM_REGISTERS !== "x0, x1, x2, x3, x4, x5, x6, x7") throw new Error(`arm64 GP 漂移: ${arm64.PARAM_REGISTERS}`);
if (arm64.FP_PARAM_REGISTERS !== "d0, d1, d2, d3, d4, d5, d6, d7") throw new Error(`arm64 FP 漂移: ${arm64.FP_PARAM_REGISTERS}`);
if (arm64.FP_RETURN_REGISTER !== "d0") throw new Error("arm64 FP_RETURN 漂移");
const x64 = defines(cc.match(/#else  \/\/ V8_TARGET_OS_WIN\n([\s\S]*?)(?=#elif|#endif)/)[1]);
if (x64.PARAM_REGISTERS !== "rdi, rsi, rdx, rcx, r8, r9") throw new Error(`x64 GP 漂移: ${x64.PARAM_REGISTERS}`);
// BuildParameterLocations 变体数: Win/MIPS64 共用 + Loong64 + 通用
const bpCount = [...cc.matchAll(/void BuildParameterLocations\(/g)].length;
if (bpCount !== 3) throw new Error(`BuildParameterLocations ${bpCount} != 3`);

const gp = arm64.PARAM_REGISTERS.split(",").map((s) => s.trim());
const fp = arm64.FP_PARAM_REGISTERS.split(",").map((s) => s.trim());

const body = `${banner("src/compiler/c-linkage.js", "src/compiler/c-linkage.cc", "Linkage::GetSimplifiedCDescriptor — 简化 C 调用约定 (平台寄存器表 + 参数定位器)")}
//! C 调用约定的 CallDescriptor 装配。
//!
//! 规则面: ARM64 寄存器表 (GP ${gp.length} + FP ${fp.length}),
//! 三种 BuildParameterLocations 变体的定位规则。
//! 语义面 (推迟): LinkageLocation/LocationSignature (linkage.h 族),
//! Register code 映射 (codegen 层)。

#![allow(dead_code)]

/// v8: src/compiler/c-linkage.cc ARM64 PARAM_REGISTERS
pub const C_PARAM_REGISTERS: &[&str] = &[${gp.map((r) => `"${r}"`).join(", ")}];
/// v8: src/compiler/c-linkage.cc ARM64 FP_PARAM_REGISTERS
pub const C_FP_PARAM_REGISTERS: &[&str] = &[${fp.map((r) => `"${r}"`).join(", ")}];
/// v8: src/compiler/c-linkage.cc ARM64 FP_RETURN_REGISTER
pub const C_FP_RETURN_REGISTER: &str = "${arm64.FP_RETURN_REGISTER}";
/// x64 SysV (对照): ${x64.PARAM_REGISTERS} / ${x64.FP_PARAM_REGISTERS}

/// v8: src/compiler/c-linkage.cc BuildParameterLocations (通用双计数版, ARM64 走此)
/// 规则: GP/FP 各自计数; 溢出 → ForCallerFrameSlot(-1 - stack_offset) 且
/// stack_offset 先增; Win-x64/MIPS64 用单计数 (i 同标), Loong64 FP 溢出可回落
/// GP (ForNullRegister 负编码)。
pub fn build_parameter_locations(
  msig: &crate::MachineSignature,
  out_locations: &mut crate::linkage::LocationSignatureBuilder,
) {
  let mut stack_offset: i32 = 0; // STACK_SHADOW_WORDS 仅 Win-x64 = 4
  let mut num_params = 0usize;
  let mut num_fp_params = 0usize;
  for i in 0..msig.parameter_count() {
    let ty = msig.get_param(i);
    let spill = if crate::is_floating_point(ty.representation()) {
      num_fp_params >= C_FP_PARAM_REGISTERS.len()
    } else {
      num_params >= C_PARAM_REGISTERS.len()
    };
    if spill {
      out_locations.add_param(crate::LinkageLocation::for_caller_frame_slot(-1 - stack_offset, ty));
      stack_offset += 1;
    } else if crate::is_floating_point(ty.representation()) {
      out_locations.add_param(crate::LinkageLocation::for_register(num_fp_params as i32, ty));
      num_fp_params += 1;
    } else {
      out_locations.add_param(crate::LinkageLocation::for_register(num_params as i32, ty));
      num_params += 1;
    }
  }
}

/// v8: src/compiler/c-linkage.cc Linkage::GetSimplifiedCDescriptor
/// 装配常量: kind=kCallAddress, tag=kInvalidEntrypointTag,
/// target=Pointer() ForAnyRegister, flags|=kNoAllocate, 名 "c-call";
/// 前置: properties ∈ {kNoThrow, kPure}, 返回数 ≤ 2,
/// 第二返回必须非浮点 (走 kReturnRegister1)。
pub fn get_simplified_c_descriptor(
  zone: &mut crate::zone::Zone,
  msig: &crate::MachineSignature,
  flags: crate::CallDescriptorFlags,
  properties: crate::operator::Properties,
) -> *mut crate::CallDescriptor {
  debug_assert!(properties == crate::operator::Properties::NoThrow || properties == crate::operator::Properties::Pure);
  debug_assert!(msig.parameter_count() <= crate::K_MAX_C_PARAMETERS);
  todo!("locations 装配 + BuildParameterLocations + CallDescriptor::new(kCallAddress, …, \\"c-call\\")")
}
`;

const p = writeSrc("v8-compiler-passes", "c_linkage.rs.skel", body);
console.log(`written ${p}\narm64 GP=${gp.length} FP=${fp.length} BuildParameterLocations变体=${bpCount}`);
