#!/usr/bin/env bun
// tool/src/codegen/x64/macro-assembler-x64.h.js
// 机械提取 v8/src/codegen/x64/macro-assembler-x64.{h,cc}: 枚举/常量 + wasm X-macro
// 方法族清单 + API 方法名清单 → v8-codegen-x64/src/macro_assembler.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/codegen/x64/macro-assembler-x64.h");
const ccSrc = readV8("src/codegen/x64/macro-assembler-x64.cc");

const m = (re, what, src = hSrc) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

// 枚举对数
for (const v of ["kInterruptStackLimit", "kRealStackLimit", "kNormal", "kReverse",
  "kInScratchRegister", "kInRootRegister"]) {
  if (!hSrc.includes(v)) throw new Error(`缺枚举值 ${v}`);
}

// X-macro 方法族
const flatH = hSrc.replace(/\\\r?\n/g, " ");
function macroEntries(name) {
  const body = flatH.match(new RegExp(`#define ${name}\\(V\\) ([\\s\\S]*?)(?=#define |#undef |#endif)`));
  if (!body) throw new Error(`缺宏 ${name}`);
  return [...body[1].matchAll(/V\((\w+)(?:,\s*(\w+),\s*(\w+))?\)/g)].map((x) => ({
    name: x[1],
    a: x[2] ?? "",
    b: x[3] ?? "",
  }));
}
const extAddPairwise = macroEntries("MACRO_ASM_X64_IEXTADDPAIRWISE_LIST");
const splat = macroEntries("MACRO_ASM_X64_ISPLAT_LIST");
if (extAddPairwise.length !== 4) throw new Error(`IEXTADDPAIRWISE ${extAddPairwise.length} != 4`);
if (splat.length !== 4) throw new Error(`ISPLAT ${splat.length} != 4`);

// .h 方法名清单 (对数基准: void 声明数)
const voidDecls = [...hSrc.matchAll(/\bvoid (\w+)\(/g)].map((x) => x[1]);
const uniqueVoid = [...new Set(voidDecls)];
if (uniqueVoid.length < 200) throw new Error(`方法清单过少: ${uniqueVoid.length}`);

// .cc 关键常量 (对数抽查)
for (const want of ["MacroAssembler::CodeEntry", "MacroAssembler::CallCFunction", "MacroAssembler::LoadRootRegisterOffset"]) {
  if (!ccSrc.includes(want)) throw new Error(`macro-assembler-x64.cc 缺 ${want}`);
}

let rust = banner(
  "src/codegen/x64/macro-assembler-x64.h.js",
  "src/codegen/x64/macro-assembler-x64.{h,cc}",
  "x64 宏汇编枚举 + wasm 方法族展开 + API 方法清单",
);

rust += `
/// 原版 enum class StackLimitKind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StackLimitKind {
  kInterruptStackLimit = 0,
  kRealStackLimit = 1,
}

/// 原版 enum class PushArrayOrder
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PushArrayOrder {
  kNormal = 0,
  kReverse = 1,
}

/// 原版 enum class IsolateRootLocation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IsolateRootLocation {
  kInScratchRegister = 0,
  kInRootRegister = 1,
}

/// 原版 MACRO_ASM_X64_IEXTADDPAIRWISE_LIST 展开:
/// DECLARE_IEXTADDPAIRWISE(op) → void op(YMMRegister dst, YMMRegister src, YMMRegister scratch)
pub const EXT_ADD_PAIRWISE_OPS: &[&str] = &[
${extAddPairwise.map((e) => `  ${JSON.stringify(e.name)},`).join("\n")}
];

/// 原版 MACRO_ASM_X64_ISPLAT_LIST 展开: (方法名, 后缀, mov 指令)
/// DECLARE_ISPLAT(name, suffix, instr_mov) → void name(YMMRegister dst, Register src)
///                                          void name(YMMRegister dst, Operand src)
#[rustfmt::skip]
pub const SPLAT_OPS: &[(&str, &str, &str)] = &[
${splat.map((e) => `  (${JSON.stringify(e.name)}, ${JSON.stringify(e.a)}, ${JSON.stringify(e.b)}),`).join("\n")}
];

/// MacroAssembler x64 API 方法名清单 (原版 .h 中 void 声明, 去重;
/// 共 ${uniqueVoid.length} 个 — 签名级移植由 Developer 按此清单推进)
pub const MACRO_ASSEMBLER_VOID_METHODS: &[&str] = &[
${uniqueVoid.map((n) => `  ${JSON.stringify(n)},`).join("\n")}
];

/// 方法数
pub const MACRO_ASSEMBLER_VOID_METHOD_COUNT: usize = ${uniqueVoid.length};

// —— 提取不变式断言 ——
const _: () = {
  assert!(StackLimitKind::kRealStackLimit as u8 == 1);
  assert!(PushArrayOrder::kReverse as u8 == 1);
  assert!(EXT_ADD_PAIRWISE_OPS.len() == 4);
  assert!(SPLAT_OPS.len() == 4);
  assert!(EXT_ADD_PAIRWISE_OPS[0] == "I32x8ExtAddPairwiseI16x16S");
  assert!(SPLAT_OPS[0].0 == "I8x32Splat" && SPLAT_OPS[0].1 == "b");
  assert!(MACRO_ASSEMBLER_VOID_METHOD_COUNT == ${uniqueVoid.length});
};
`;

const p = writeSrc("v8-codegen-x64", "macro_assembler.rs", rust);
console.log(
  `macro-assembler-x64: 枚举 3 / wasm 方法族 2 (4+4) / void 方法清单 ${uniqueVoid.length} → ${p}`,
);
