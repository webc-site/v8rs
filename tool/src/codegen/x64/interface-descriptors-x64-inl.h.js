#!/usr/bin/env bun
// tool/src/codegen/x64/interface-descriptors-x64-inl.h.js
// 机械提取 v8/src/codegen/x64/interface-descriptors-x64-inl.h:
// 默认寄存器阵列 + WriteBarrierDescriptor + 29 个描述符寄存器访问器
// → v8-codegen-x64/src/interface_descriptors.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/codegen/x64/interface-descriptors-x64-inl.h");
const flat = src.replace(/\\\r?\n/g, " ");

// 1. 默认 GP 寄存器阵列: RegisterArray(rax, rbx, rcx, rdx, rdi)
const defArr = flat.match(/DefaultRegisterArray\(\) \{[\s\S]*?RegisterArray\(([^)]*)\)/);
if (!defArr) throw new Error("缺 DefaultRegisterArray");
const defRegs = defArr[1].split(",").map((s) => s.trim()).filter(Boolean);
if (defRegs.length !== 5) throw new Error(`DefaultRegisterArray ${defRegs.length} != 5`);
if (!flat.includes("static_assert(registers.size() == kMaxBuiltinRegisterParams);"))
  throw new Error("缺 kMaxBuiltinRegisterParams 断言");

// 2. 默认 Double 阵列: xmm0..xmm6 (7)
const defDbl = flat.match(/DefaultDoubleRegisterArray\(\) \{[\s\S]*?DoubleRegisterArray\(([^)]*)\)/);
if (!defDbl) throw new Error("缺 DefaultDoubleRegisterArray");
const defDblRegs = defDbl[1].split(",").map((s) => s.trim()).filter(Boolean);
if (defDblRegs.length !== 7) throw new Error(`DefaultDoubleRegisterArray ${defDblRegs.length} != 7`);

// 3. 返回寄存器阵列
const defRet = flat.match(/DefaultReturnRegisterArray\(\) \{[\s\S]*?RegisterArray\(([^)]*)\)/);
if (!defRet) throw new Error("缺 DefaultReturnRegisterArray");
const retRegs = defRet[1].split(",").map((s) => s.trim()).filter(Boolean);
if (retRegs.length !== 3) throw new Error("DefaultReturnRegisterArray != 3");

// 4. WriteBarrierDescriptor::registers() 双分支
const wb = flat.match(
  /WriteBarrierDescriptor::registers\(\) \{[\s\S]*?#ifdef V8_TARGET_OS_WIN\s*return RegisterArray\(([^)]*)\);\s*#else\s*return RegisterArray\(([^)]*)\);/,
);
if (!wb) throw new Error("缺 WriteBarrierDescriptor");
const wbWin = wb[1].split(",").map((s) => s.trim()).filter(Boolean);
const wbNix = wb[2].split(",").map((s) => s.trim()).filter(Boolean);
if (wbWin.length !== 7 || wbNix.length !== 6) throw new Error("WriteBarrierDescriptor 分支数异常");

// 5. 描述符寄存器访问器: constexpr Register X::Y() { return REG; } (允许跨行)
const accessors = [...flat.matchAll(
  /constexpr Register\s+(\w+)::\s*(\w+)\(\)\s*\{\s*return\s+([\w\[\]0-9]+);\s*\}/g,
)].map(([, desc, role, reg]) => ({ desc, role, reg }));
// 对数: 原版侧独立计数 (grep -c "constexpr Register" = 39)
if (accessors.length !== 39) throw new Error(`访问器 ${accessors.length} != 39`);
// 抽查
const find = (d, r) => accessors.find((a) => a.desc === d && a.role === r);
if (!find("LoadDescriptor", "ReceiverRegister") || find("LoadDescriptor", "ReceiverRegister").reg !== "rdx")
  throw new Error("LoadDescriptor::ReceiverRegister 抽查失败");
if (!find("LoadDescriptor", "NameRegister") || find("LoadDescriptor", "NameRegister").reg !== "rcx")
  throw new Error("LoadDescriptor::NameRegister 抽查失败");

// kInterpreterAccumulatorRegister = rax (x64, register-x64.h 定义)
const regEnum = (reg) => (reg === "kInterpreterAccumulatorRegister" ? "Rax" : reg.toUpperCase());

let rust = banner(
  "src/codegen/x64/interface-descriptors-x64-inl.h.js",
  "src/codegen/x64/interface-descriptors-x64-inl.h",
  "x64 调用描述符寄存器分配表",
);

rust += `
/// kMaxBuiltinRegisterParams (原版 static_assert 锚定 GP 默认阵列大小)
pub const K_MAX_BUILTIN_REGISTER_PARAMS: usize = ${defRegs.length};

/// 原版 CallInterfaceDescriptor::DefaultRegisterArray(): rax, rbx, rcx, rdx, rdi
pub const DEFAULT_REGISTER_ARRAY: [&str; ${defRegs.length}] = [${defRegs.map((r) => JSON.stringify(r)).join(", ")}];

/// 原版 DefaultDoubleRegisterArray(): xmm0..xmm6
pub const DEFAULT_DOUBLE_REGISTER_ARRAY: [&str; ${defDblRegs.length}] = [${defDblRegs.map((r) => JSON.stringify(r)).join(", ")}];

/// 原版 DefaultReturnRegisterArray(): kReturnRegister0/1/2
pub const DEFAULT_RETURN_REGISTER_ARRAY: [&str; ${retRegs.length}] = [${retRegs.map((r) => JSON.stringify(r)).join(", ")}];

/// 原版 DefaultReturnDoubleRegisterArray(): kFPReturnRegister0 + 2 个 no_dreg 填充
pub const DEFAULT_RETURN_DOUBLE_REGISTER_ARRAY: [&str; 3] = ["kFPReturnRegister0", "no_dreg", "no_dreg"];

/// 原版 WriteBarrierDescriptor::registers() — 非 WIN 分支 (本目标生效): 6 寄存器
pub const WRITE_BARRIER_REGISTERS: [&str; ${wbNix.length}] = [${wbNix.map((r) => JSON.stringify(r)).join(", ")}];
/// 原版 WriteBarrierDescriptor::registers() — V8_TARGET_OS_WIN 分支: 7 寄存器
pub const WRITE_BARRIER_REGISTERS_WIN: [&str; ${wbWin.length}] = [${wbWin.map((r) => JSON.stringify(r)).join(", ")}];

/// 描述符寄存器访问器总表: (描述符, 角色名, 寄存器名)。
/// 寄存器名保持原版字面 (rax/rcx/...; kInterpreterAccumulatorRegister = rax,
/// kCArgRegs[i] = C 调用参数寄存器组)。
#[rustfmt::skip]
pub const DESCRIPTOR_REGISTERS: &[(&str, &str, &str)] = &[
${accessors.map((a) => `  (${JSON.stringify(a.desc)}, ${JSON.stringify(a.role)}, ${JSON.stringify(a.reg)}),`).join("\n")}
];

/// 按 (描述符, 角色) 查寄存器
#[inline]
#[must_use]
pub fn descriptor_register(desc: &str, role: &str) -> Option<&'static str> {
  DESCRIPTOR_REGISTERS
    .iter()
    .find(|(d, r, _)| *d == desc && *r == role)
    .map(|(_, _, reg)| *reg)
}

// TSAN 分支 (V8_IS_TSAN): TSANStoreDescriptor / TSANLoadDescriptor::registers()
// 本目标不启用 TSAN — 登记: 未提取 (2 个描述符)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_BUILTIN_REGISTER_PARAMS == 5);
  assert!(DEFAULT_REGISTER_ARRAY == ["rax", "rbx", "rcx", "rdx", "rdi"]);
  assert!(DEFAULT_DOUBLE_REGISTER_ARRAY.len() == 7);
  assert!(DESCRIPTOR_REGISTERS.len() == ${accessors.length});
  assert!(descriptor_register("LoadDescriptor", "ReceiverRegister") == Some("rdx"));
  assert!(descriptor_register("LoadDescriptor", "NameRegister") == Some("rcx"));
  assert!(descriptor_register("LoadDescriptor", "SlotRegister") == Some("rax"));
  assert!(WRITE_BARRIER_REGISTERS.len() == ${wbNix.length});
  assert!(WRITE_BARRIER_REGISTERS_WIN.len() == ${wbWin.length});
};
`;

const p = writeSrc("v8-codegen-x64", "interface_descriptors.rs", rust);
console.log(
  `interface-descriptors-x64-inl: 默认阵列 4 / WriteBarrier 双分支 / 访问器 ${accessors.length} → ${p}`,
);
