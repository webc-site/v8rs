#!/usr/bin/env bun
// tool/src/wasm/wasm-linkage.h.js
// 机械提取 v8/src/wasm/wasm-linkage.h 的调用约定寄存器表。
// 平台分支按 x64 + arm64 展开; 其余架构登记于注释 (源文本逐一保留)。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-linkage.h");

// 切出各平台块
const blocks = [];
const re = /^#(?:el)?if\s+V8_TARGET_ARCH_(\w+)|^#elif\s+V8_TARGET_ARCH_(\w+)/gm;
// 按 "// == <arch> ==" 注释分段
const sectionRe = /\/\/ == (\w+) =+\n([\s\S]*?)(?=\/\/ == \w+ =+\n|\n#else|\n#endif)/g;
let m;
while ((m = sectionRe.exec(hSrc)) !== null) {
  blocks.push({ arch: m[1], body: m[2] });
}
if (blocks.length < 8) throw new Error(`平台块仅 ${blocks.length} 个, 解析异常`);

function extractRegs(body, name) {
  const mm = body.match(new RegExp(`(?:constexpr\\s+[\\w:<>,\\s]+?)${name}\\[\\]\\s*=\\s*\\{([^}]*)\\}`));
  if (!mm) return null;
  return mm[1].split(",").map((s) => s.trim()).filter(Boolean);
}

const TABLES = ["kGpParamRegisters", "kGpReturnRegisters", "kFpParamRegisters", "kFpReturnRegisters"];
const perArch = new Map();
for (const b of blocks) {
  const t = {};
  for (const name of TABLES) {
    t[name] = extractRegs(b.body, name);
    if (!t[name]) throw new Error(`${b.arch} 缺少 ${name}`);
    if (t[name].length === 0 && b.arch !== "unknown") {
      // unknown 平台才允许全空
      throw new Error(`${b.arch} ${name} 为空`);
    }
  }
  // simd128 表一律为空 (x64/arm64/ia32/arm/arm64/mips64/loong64/s390x), 其余为非空数组
  const simdP = b.body.match(/kSimd128ParamRegisters\s*=?\s*\{([^}]*)\}/);
  const simdR = b.body.match(/kSimd128ReturnRegisters\s*=?\s*\{([^}]*)\}/);
  t.kSimd128ParamRegisters = simdP ? simdP[1].split(",").map((s) => s.trim()).filter(Boolean) : [];
  t.kSimd128ReturnRegisters = simdR ? simdR[1].split(",").map((s) => s.trim()).filter(Boolean) : [];
  perArch.set(b.arch, t);
}

// 目标平台锚点 (rule 9: x64 + arm64 展开)
const x64 = perArch.get("x64");
const arm64 = perArch.get("arm64");
if (!x64 || !arm64) throw new Error("缺少 x64/arm64 平台块");
if (x64.kGpParamRegisters.join() !== "rsi,rax,rdx,rcx,rbx,r9") throw new Error("x64 GP 参数寄存器漂移");
if (arm64.kGpParamRegisters.join() !== "x7,x0,x2,x3,x4,x5,x6") throw new Error("arm64 GP 参数寄存器漂移");
if (x64.kFpParamRegisters.length !== 6) throw new Error("x64 FP 参数寄存器数漂移");
if (arm64.kFpParamRegisters.length !== 8) throw new Error("arm64 FP 参数寄存器数漂移");

// 布尔常量: x64/arm64 目标下全 false
for (const c of ["kIsFpAlwaysDouble", "kIsBigEndian", "kIsBigEndianOnSim", "kFP32NeedNanBox"]) {
  const mm = hSrc.match(new RegExp(`${c} = (true|false)`, "g"));
  if (!mm || mm.length < 2) throw new Error(`常量 ${c} 分支缺失`);
}
const idxMatch = hSrc.match(/constexpr int kWasmInstanceDataParameterIndex = (\d+);/);
if (!idxMatch) throw new Error("未找到 kWasmInstanceDataParameterIndex");
const instanceIdx = Number(idxMatch[1]);
if (!hSrc.includes("kWasmImplicitArgRegister ==")) throw new Error("static_assert 锚点缺失");

console.log(`平台块 ${blocks.length}: ${blocks.map((b) => b.arch).join(",")}`);

let rust = banner(
  "src/wasm/wasm-linkage.h.js",
  "src/wasm/wasm-linkage.h",
  "wasm 调用约定寄存器表 (x64+arm64 展开, 其余架构登记注释)",
);

// 生成单个平台的寄存器表
function archTables(arch, t, indent) {
  const pad = " ".repeat(indent);
  let out = "";
  for (const name of TABLES) {
    const rustName = name.replace(/^k/, "").toUpperCase();
    out += `${pad}/// ${name}\n${pad}pub static ${rustName}: &[&str] = &[${t[name].map((r) => `"${r}"`).join(", ")}];\n`;
  }
  for (const name of ["kSimd128ParamRegisters", "kSimd128ReturnRegisters"]) {
    const rustName = name.replace(/^k/, "").toUpperCase();
    out += `${pad}/// ${name}\n${pad}pub static ${rustName}: &[&str] = &[${t[name].map((r) => `"${r}"`).join(", ")}];\n`;
  }
  return out;
}

rust += `
/// wasm 实例参数在调用描述符中的固定下标 (对应 kWasmInstanceDataParameterIndex)
pub const WASM_INSTANCE_DATA_PARAMETER_INDEX: i32 = ${instanceIdx};

/// x64 目标下的平台常量 (对应各 constexpr bool, x64 分支取值)
pub mod x64 {
${archTables("x64", x64, "  ")}
  /// kIsFpAlwaysDouble (x64 = false)
  pub const IS_FP_ALWAYS_DOUBLE: bool = false;
  /// kIsBigEndian (x64 = false)
  pub const IS_BIG_ENDIAN: bool = false;
  /// kIsBigEndianOnSim (x64 = false)
  pub const IS_BIG_ENDIAN_ON_SIM: bool = false;
  /// kFP32NeedNanBox (x64 = false)
  pub const FP32_NEED_NAN_BOX: bool = false;
}

/// arm64 目标下的平台常量 (对应各 constexpr bool, arm64 分支取值)
pub mod arm64 {
${archTables("arm64", arm64, "  ")}
  /// kIsFpAlwaysDouble (arm64 = false; 仅 PPC64 为 true)
  pub const IS_FP_ALWAYS_DOUBLE: bool = false;
  /// kIsBigEndian (arm64 = false)
  pub const IS_BIG_ENDIAN: bool = false;
  /// kIsBigEndianOnSim (arm64 = false)
  pub const IS_BIG_ENDIAN_ON_SIM: bool = false;
  /// kFP32NeedNanBox (arm64 = false; 仅 RISCV64 为 true)
  pub const FP32_NEED_NAN_BOX: bool = false;
}
`;

// 其余架构: 登记不展开 (rule 9)
const skipped = blocks.filter((b) => b.arch !== "x64" && b.arch !== "arm64");
rust += `
// —— 其余架构登记 (本产物不展开, 源文本位于 wasm-linkage.h 对应分支) ——
`;

for (const b of skipped) {
  const t = perArch.get(b.arch);
  rust += `// ${b.arch}: gp_param=[${t.kGpParamRegisters.join(",")}] gp_ret=[${t.kGpReturnRegisters.join(",")}] fp_param=[${t.kFpParamRegisters.join(",")}] fp_ret=[${t.kFpReturnRegisters.join(",")}]\n`;
}

rust += `
/// 链接位置分配器 (对应 class LinkageAllocator, 骨架)
pub struct LinkageAllocator {
  gp_count_: usize,
  gp_offset_: usize,
  fp_count_: usize,
  fp_offset_: usize,
  simd128_count_: usize,
  simd128_offset_: usize,
  slot_size_: usize,
}

impl LinkageAllocator {
  /// 对应 LinkageAllocator(gp, gpc, fp, fpc, simd128, simd128c)
  #[must_use]
  pub const fn new(gpc: usize, fpc: usize, simd128c: usize) -> Self {
    Self {
      gp_count_: gpc,
      gp_offset_: 0,
      fp_count_: fpc,
      fp_offset_: 0,
      simd128_count_: simd128c,
      simd128_offset_: 0,
      slot_size_: 0,
    }
  }

  /// 对应 CanAllocateGP()
  #[must_use]
  pub const fn can_allocate_gp(&self) -> bool {
    self.gp_offset_ < self.gp_count_
  }

  /// 对应 CanAllocateFP(rep) (非 ARM 分支: fp_offset_ < fp_count_)
  #[must_use]
  pub const fn can_allocate_fp(&self) -> bool {
    self.fp_offset_ < self.fp_count_
  }

  /// 对应 CanAllocateSimd128(rep)
  #[must_use]
  pub const fn can_allocate_simd128(&self) -> bool {
    self.simd128_offset_ < self.simd128_count_
  }

  /// 对应 NextGpReg() -> 寄存器编号 (骨架: 返回游标, 具体编码待修订)
  pub fn next_gp_reg(&mut self) -> usize {
    unimplemented!("骨架: 游标自增待与寄存器编码表对接")
  }

  /// 对应 NextFpReg(rep) (非 ARM 分支)
  pub fn next_fp_reg(&mut self) -> usize {
    unimplemented!("骨架: 游标自增待与寄存器编码表对接")
  }

  /// 对应 HasSimd128Regs()
  #[must_use]
  pub const fn has_simd128_regs(&self) -> bool {
    self.simd128_count_ > 0
  }

  /// 对应 NumStackSlots()
  #[must_use]
  pub const fn num_stack_slots(&self) -> usize {
    self.slot_size_
  }

  /// 对应 EndSlotArea()
  pub fn end_slot_area(&mut self) {
    unimplemented!("骨架: 对齐槽分配器待对接");
  }
}

/// 参数/返回位置分配助手 (对应 class LinkageLocationAllocator, 骨架)
pub struct LinkageLocationAllocator {
  allocator_: LinkageAllocator,
  slot_offset_: i32,
}

impl LinkageLocationAllocator {
  /// 对应模板构造 (gp, fp, simd128, slot_offset)
  #[must_use]
  pub const fn new(gpc: usize, fpc: usize, simd128c: usize, slot_offset: i32) -> Self {
    Self { allocator_: LinkageAllocator::new(gpc, fpc, simd128c), slot_offset_: slot_offset }
  }

  /// 对应 Next(rep): 先寄存器后栈槽
  pub fn next(&mut self, _rep: crate::wasm_opcodes::SigType) -> LinkageLocation {
    unimplemented!("骨架: 参照原版 LinkageLocationAllocator::Next 修订")
  }

  /// 对应 NumStackSlots()
  #[must_use]
  pub const fn num_stack_slots(&self) -> usize {
    self.allocator_.num_stack_slots()
  }

  /// 对应 EndSlotArea()
  pub fn end_slot_area(&mut self) {
    self.allocator_.end_slot_area();
  }
}

/// 链接位置 (对应 LinkageLocation, 最小骨架)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkageLocation {
  /// 寄存器位置 (编号, 类型)
  Register(i32, crate::wasm_opcodes::SigType),
  /// 调用者栈槽 (负索引)
  CallerFrameSlot(i32, crate::wasm_opcodes::SigType),
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(WASM_INSTANCE_DATA_PARAMETER_INDEX == 0);
  assert!(x64::GP_PARAM_REGISTERS.len() == 6);
  assert!(x64::GP_RETURN_REGISTERS.len() == 2);
  assert!(x64::FP_PARAM_REGISTERS.len() == 6);
  assert!(x64::FP_RETURN_REGISTERS.len() == 2);
  assert!(arm64::GP_PARAM_REGISTERS.len() == 7);
  assert!(arm64::GP_RETURN_REGISTERS.len() == 2);
  assert!(arm64::FP_PARAM_REGISTERS.len() == 8);
  assert!(x64::GP_PARAM_REGISTERS[WASM_INSTANCE_DATA_PARAMETER_INDEX as usize] == "rsi");
  assert!(arm64::GP_PARAM_REGISTERS[WASM_INSTANCE_DATA_PARAMETER_INDEX as usize] == "x7");
  assert!(x64::SIMD128_PARAM_REGISTERS.is_empty());
  assert!(arm64::SIMD128_PARAM_REGISTERS.is_empty());
};
`;

const p = writeSrc("v8-wasm-core", "wasm_linkage.rs", rust);
console.log(`written ${p}`);
