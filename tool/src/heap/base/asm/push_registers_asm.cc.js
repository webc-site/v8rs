#!/usr/bin/env bun
// tool/src/heap/base/asm/push_registers_asm.cc.js
// 机械提取 v8/src/heap/base/asm/{arm,arm64,ia32,loong64,mips64,ppc,riscv,s390,x64,zos}/
// push_registers_asm.cc (10 架构同 stem 打包一脚本):
// 每架构 ① 符号 PushAllRegistersAndIterateStack 锚 ② 回调调用锚 ③ 指令行计数对数
// (GNU asm 计 `^    "  ` 行; zos 计内联 asm 串) ④ x64/arm64 保存指令定点锚。
// Rust 侧产 per-arch cfg 门面 extern "C" 声明骨架 (asm 体本身留 Developer 手写)。

import { readV8, writeSrc, banner } from "../../../../lib.js";

// 逐架构完整字面量路径 (规范: readV8 实参禁模板串)
const SOURCES = {
  arm: readV8("src/heap/base/asm/arm/push_registers_asm.cc"),
  arm64: readV8("src/heap/base/asm/arm64/push_registers_asm.cc"),
  ia32: readV8("src/heap/base/asm/ia32/push_registers_asm.cc"),
  loong64: readV8("src/heap/base/asm/loong64/push_registers_asm.cc"),
  mips64: readV8("src/heap/base/asm/mips64/push_registers_asm.cc"),
  ppc: readV8("src/heap/base/asm/ppc/push_registers_asm.cc"),
  riscv: readV8("src/heap/base/asm/riscv/push_registers_asm.cc"),
  s390: readV8("src/heap/base/asm/s390/push_registers_asm.cc"),
  x64: readV8("src/heap/base/asm/x64/push_registers_asm.cc"),
  zos: readV8("src/heap/base/asm/zos/push_registers_asm.cc"),
};

// 回调调用指令锚 (每架构至少命中一个)
const CALL_OPS = {
  arm: "blx", arm64: "blr", ia32: "call", loong64: "jirl", mips64: "jalr",
  ppc: "blr", riscv: "jalr", s390: "basr", x64: "call", zos: "basr",
};

const results = {};
for (const arch of Object.keys(SOURCES)) {
  const src = SOURCES[arch];
  if (!src.includes("PushAllRegistersAndIterateStack")) throw new Error(`${arch}: 符号缺失`);
  if (!src.includes(CALL_OPS[arch])) throw new Error(`${arch}: 回调调用 ${CALL_OPS[arch]} 缺失`);
  if (arch === "zos") {
    // zos: C++ 函数体内联 asm 串 (lg/lgr/basr/nopr)
    results[arch] = (src.match(/" (?:lg|lgr|basr|nopr) /g) || []).length;
  } else {
    results[arch] = (src.match(/^ {4}" {2}/gm) || []).length;
  }
  if (results[arch] === 0) throw new Error(`${arch}: 指令行 0`);
}
// 定点锚: x64 = push rbp + dummy + rbx/r12/r13/r14/r15 (7 push) / arm64 = stp ×6
const x64Push = (SOURCES.x64.match(/^ {4}" {2}push /gm) || []).length;
const arm64Stp = (SOURCES.arm64.match(/^ {4}" {2}stp /gm) || []).length;
if (x64Push !== 7) throw new Error(`x64 push ${x64Push}, 期望 7`);
if (arm64Stp !== 6) throw new Error(`arm64 stp ${arm64Stp}, 期望 6`);
console.log(`push_registers_asm 10 架构指令行: ${Object.entries(results).map(([a, n]) => `${a}=${n}`).join(" ")} (x64 push=${x64Push}, arm64 stp=${arm64Stp})`);

let rust = banner(
  "src/heap/base/asm/push_registers_asm.cc.js",
  "src/heap/base/asm/*/push_registers_asm.cc (10 架构)",
  "保存全部被调者保存寄存器并迭代栈: extern C 声明骨架",
);

rust += `
/// 对应 extern "C" void PushAllRegistersAndIterateStack(
///   Stack* stack, void* argument, Stack::IterateStackCallback callback);
/// 语义: 压入全部 callee-saved 寄存器 → 以当前 sp 为栈标记回调 → 弹出恢复。
/// 实现是每架构手写内联汇编, 不能机械转写; Rust 侧按架构 cfg 挂外部符号
/// (todo 五: 先 x64 + arm64, 其余架构后置)。
/// v8: src/heap/base/asm/x64/push_registers_asm.cc PushAllRegistersAndIterateStack
pub type IterateStackCallback = unsafe extern "C" fn(stack: *mut u8, argument: *mut u8, sp: *const u8);

#[cfg(target_arch = "x86_64")]
extern "C" {
  /// x64: push rbp + 对齐 dummy + rbx/r12/r13/r14/r15, call *%r8 回调
  /// (指令行 ${results.x64}, 其中 push ${x64Push})
  #[link_name = "PushAllRegistersAndIterateStack"]
  pub fn push_all_registers_and_iterate_stack(
    stack: *mut u8,
    argument: *mut u8,
    callback: IterateStackCallback,
  );
}

#[cfg(target_arch = "aarch64")]
extern "C" {
  /// arm64: stp fp,lr + x19-x28 五对, blr 回调 (指令行 ${results.arm64}, 其中 stp ${arm64Stp})
  #[link_name = "PushAllRegistersAndIterateStack"]
  pub fn push_all_registers_and_iterate_stack(
    stack: *mut u8,
    argument: *mut u8,
    callback: IterateStackCallback,
  );
}

// 其余 8 架构台账 (asm 指令行数; 后端启用时按同型 extern "C" 挂符号):
${Object.entries(results).filter(([a]) => a !== "x64" && a !== "arm64").map(([a, n]) => `//   ${a}: ${n}`).join("\n")}

// —— 提取不变式断言 ——
const _: () = {
  // 回调签名三参 (Stack*, argument, sp) — 与函数指针占位对齐
  assert!(core::mem::size_of::<IterateStackCallback>() == core::mem::size_of::<*mut u8>());
};
`;

const p = writeSrc("v8-heap-core", "push_registers_asm.rs.skel", rust);
console.log(`written ${p}`);
