#!/usr/bin/env bun
// tool/src/codegen/arm64/interface-descriptors-arm64-inl.h.js
// 机械提取 v8/src/codegen/arm64/interface-descriptors-arm64-inl.h:
// 默认寄存器阵列 + WriteBarrierDescriptor + 38 个描述符寄存器访问器
// → v8-codegen-arm64/src/interface_descriptors.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/codegen/arm64/interface-descriptors-arm64-inl.h");
const flat = src.replace(/\\\r?\n/g, " ");

// 1. 默认阵列
const defArr = flat.match(/DefaultRegisterArray\(\) \{[\s\S]*?RegisterArray\(([^)]*)\)/);
if (!defArr) throw new Error("缺 DefaultRegisterArray");
const defRegs = defArr[1].split(",").map((s) => s.trim()).filter(Boolean);
if (defRegs.join(",") !== "x0,x1,x2,x3,x4") throw new Error(`GP 默认阵列异常: ${defRegs}`);

const defDbl = flat.match(/DefaultDoubleRegisterArray\(\) \{[\s\S]*?DoubleRegisterArray\(([^)]*)\)/);
if (!defDbl) throw new Error("缺 DefaultDoubleRegisterArray");
const defDblRegs = defDbl[1].split(",").map((s) => s.trim()).filter(Boolean);
if (defDblRegs.length !== 7) throw new Error("FP 默认阵列 != 7");

const defRet = flat.match(/DefaultReturnRegisterArray\(\) \{[\s\S]*?RegisterArray\(([^)]*)\)/);
if (!defRet) throw new Error("缺 DefaultReturnRegisterArray");
const retRegs = defRet[1].split(",").map((s) => s.trim()).filter(Boolean);
if (retRegs.length !== 3) throw new Error("返回阵列 != 3");

// 2. WriteBarrierDescriptor (arm64 单分支: 8 寄存器, x7 为 padding)
const wb = flat.match(/WriteBarrierDescriptor::registers\(\) \{[\s\S]*?return RegisterArray\(([^)]*)\);/);
if (!wb) throw new Error("缺 WriteBarrierDescriptor");
const wbRegs = wb[1].split(",").map((s) => s.trim()).filter(Boolean);
if (wbRegs.length !== 8) throw new Error(`WriteBarrier ${wbRegs.length} != 8`);
if (!flat.includes("Remove x7 which is just there for padding"))
  throw new Error("缺 x7 padding 注释锚点");

// 3. 访问器 (允许跨行/::折行)
const accessors = [...flat.matchAll(
  /constexpr Register\s+(\w+)::\s*(\w+)\(\)\s*\{\s*return\s+([\w\[\]0-9]+);\s*\}/g,
)].map(([, desc, role, reg]) => ({ desc, role, reg }));
// 对数: grep -c "constexpr Register" = 38
if (accessors.length !== 38) throw new Error(`访问器 ${accessors.length} != 38`);
const find = (d, r) => accessors.find((a) => a.desc === d && a.role === r);
if (find("LoadDescriptor", "ReceiverRegister")?.reg !== "x1") throw new Error("Load Receiver 抽查失败");
if (find("LoadDescriptor", "NameRegister")?.reg !== "x2") throw new Error("Load Name 抽查失败");

let rust = banner(
  "src/codegen/arm64/interface-descriptors-arm64-inl.h.js",
  "src/codegen/arm64/interface-descriptors-arm64-inl.h",
  "arm64 调用描述符寄存器分配表",
);

rust += `
/// kMaxBuiltinRegisterParams (原版 static_assert 锚定 GP 默认阵列大小)
pub const K_MAX_BUILTIN_REGISTER_PARAMS: usize = ${defRegs.length};

/// 原版 DefaultRegisterArray(): x0..x4
pub const DEFAULT_REGISTER_ARRAY: [&str; 5] = [${defRegs.map((r) => JSON.stringify(r)).join(", ")}];

/// 原版 DefaultDoubleRegisterArray(): d0..d6
pub const DEFAULT_DOUBLE_REGISTER_ARRAY: [&str; 7] = [${defDblRegs.map((r) => JSON.stringify(r)).join(", ")}];

/// 原版 DefaultReturnRegisterArray(): kReturnRegister0/1/2
pub const DEFAULT_RETURN_REGISTER_ARRAY: [&str; 3] = [${retRegs.map((r) => JSON.stringify(r)).join(", ")}];

/// 原版 DefaultReturnDoubleRegisterArray(): kFPReturnRegister0 + no_dreg 填充
pub const DEFAULT_RETURN_DOUBLE_REGISTER_ARRAY: [&str; 3] = ["kFPReturnRegister0", "no_dreg", "no_dreg"];

/// 原版 WriteBarrierDescriptor::registers() (arm64 单分支; x7 仅为 padding, 待移除 — TODO leszeks)
pub const WRITE_BARRIER_REGISTERS: [&str; 8] = [${wbRegs.map((r) => JSON.stringify(r)).join(", ")}];

/// 描述符寄存器访问器总表: (描述符, 角色名, 寄存器名)
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

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_BUILTIN_REGISTER_PARAMS == 5);
  assert!(DEFAULT_REGISTER_ARRAY == ["x0", "x1", "x2", "x3", "x4"]);
  assert!(DESCRIPTOR_REGISTERS.len() == 38);
  assert!(descriptor_register("LoadDescriptor", "ReceiverRegister") == Some("x1"));
  assert!(descriptor_register("LoadDescriptor", "NameRegister") == Some("x2"));
  assert!(descriptor_register("LoadDescriptor", "SlotRegister") == Some("x0"));
  assert!(WRITE_BARRIER_REGISTERS.len() == 8);
  assert!(WRITE_BARRIER_REGISTERS[0] == "x1" && WRITE_BARRIER_REGISTERS[7] == "x7");
};
`;

const p = writeSrc("v8-codegen-arm64", "interface_descriptors.rs", rust);
console.log(`interface-descriptors-arm64-inl: 默认阵列 4 / WriteBarrier 8 / 访问器 ${accessors.length} → ${p}`);
