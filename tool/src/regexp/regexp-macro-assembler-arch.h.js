#!/usr/bin/env bun
// tool/src/regexp/regexp-macro-assembler-arch.h.js
// 机械展开 v8/src/regexp/regexp-macro-assembler-arch.h 的目标架构分支
// (kUnalignedReadSupported + 头文件分发; x64+arm64 展开, 其余登记)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/regexp/regexp-macro-assembler-arch.h");

// 每架构分支: 架构宏 → 头文件 + kUnalignedReadSupported 取值
const branches = [...hSrc.matchAll(/#elif (V8_TARGET_ARCH_\w+)\n#include "([^"]+)"\nstatic constexpr bool kUnalignedReadSupported = (true|false);/g)];
const first = hSrc.match(/#if (V8_TARGET_ARCH_IA32)\n#include "([^"]+)"\nstatic constexpr bool kUnalignedReadSupported = (true|false);/);
if (!first) throw new Error("未找到首分支 (IA32)");
const all = [{ arch: first[1], header: first[2], unaligned: first[3] === "true" }, ...branches.map((b) => ({ arch: b[1], header: b[2], unaligned: b[3] === "true" }))];
// riscv 分支带嵌套 #ifdef (RISCV_HAS_NO_UNALIGNED 两态), 单独登记
const riscv = hSrc.match(/#elif V8_TARGET_ARCH_RISCV32 \|\| V8_TARGET_ARCH_RISCV64\n#include "([^"]+)"[\s\S]*?#endif/);
if (!riscv || !riscv[0].includes("RISCV_HAS_NO_UNALIGNED")) throw new Error("riscv 条件分支未登记");
if (all.length !== 8) throw new Error(`架构分支数异常: ${all.length}`);
// #error 兜底存在性
if (!hSrc.includes("#error Unsupported target architecture.")) {
  throw new Error("未找到兜底 #error");
}

// x64/arm64 展开, 其余登记
const targets = all.filter((b) => b.arch === "V8_TARGET_ARCH_X64" || b.arch === "V8_TARGET_ARCH_ARM64");
if (targets.length !== 2) throw new Error("x64/arm64 分支缺失");
for (const t of targets) if (!t.unaligned) throw new Error(`${t.arch} 未对齐读应为 true`);

console.log(`regexp-macro-assembler-arch.h: branches=${all.length} targets=${targets.map((t) => t.arch).join("/")}`);

let rust = banner(
  "src/regexp/regexp-macro-assembler-arch.h.js",
  "src/regexp/regexp-macro-assembler-arch.h",
  "目标架构宏汇编分发 (x64+arm64 展开; 其余架构登记)",
);

rust += `
/// 非对齐读取支持 (原版 kUnalignedReadSupported)
${targets
  .map((t) => {
    const mod = t.arch.endsWith("X64") ? "x64" : "arm64";
    return `/// ${t.arch} (源分支 include ${t.header})\npub mod ${mod} {\n  pub const UNALIGNED_READ_SUPPORTED: bool = ${t.unaligned};\n}\n`;
  })
  .join("\n")}
/// 跳过的架构登记 (硬规则 9): ${all
   .filter((b) => !targets.includes(b))
   .map((b) => b.arch.replace("V8_TARGET_ARCH_", ""))
   .join(" / ")} — 均登记于源文件, 按需再展开。
/// 语义: 其余分支取值见源; riscv32/64: RISCV_HAS_NO_UNALIGNED 定义时 false, 否则 true。

// —— 提取不变式断言 ——
const _: () = {
  assert!(x64::UNALIGNED_READ_SUPPORTED);
  assert!(arm64::UNALIGNED_READ_SUPPORTED);
};
`;

const p = writeSrc("v8-regexp-compiler", "macro_assembler_arch.rs", rust);
console.log(`written ${p}`);
