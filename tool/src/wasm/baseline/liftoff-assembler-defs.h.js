#!/usr/bin/env bun
// tool/src/wasm/baseline/liftoff-assembler-defs.h.js
// 机械提取 v8/src/wasm/baseline/liftoff-assembler-defs.h:
// 各架构 Liftoff 汇编缓存寄存器表。按 x64 + arm64 展开, 其余架构登记注释。

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/baseline/liftoff-assembler-defs.h");

// 按 #if/#elif V8_TARGET_ARCH_X 分块, 以 #elif/#else/#endif 收尾
const blocks = [];
const lines = hSrc.split("\n");
let cur = null;
for (const line of lines) {
  const start = line.match(/^#(?:if|elif)\s+V8_TARGET_ARCH_(\w+)/);
  const end = /^(#elif|#else|#endif)/.test(line);
  if (start) {
    cur = { arch: start[1], body: [] };
    blocks.push(cur);
    continue;
  }
  if (end) {
    cur = null;
    continue;
  }
  if (cur) cur.body.push(line);
}
if (blocks.length < 8) throw new Error(`架构块仅 ${blocks.length} 个`);

function extractList(body, name) {
  const m = body.join("\n").match(new RegExp(`${name}\\s*=\\s*\\{([^}]*)\\}`, "s"));
  if (!m) return null;
  return m[1].replace(/\/\/[^\n]*/g, "").split(",").map((s) => s.trim()).filter(Boolean);
}

const perArch = new Map();
for (const b of blocks) {
  const gp = extractList(b.body, "kLiftoffAssemblerGpCacheRegs");
  const fp = extractList(b.body, "kLiftoffAssemblerFpCacheRegs");
  const simd = extractList(b.body, "kLiftoffAssemblerSimd128CacheRegs");
  const setup = b.body.join("\n").match(/kLiftoffFrameSetupFunctionReg\s*=\s*(\w+);/);
  if (gp === null || fp === null || simd === null || !setup) {
    throw new Error(`${b.arch} 块提取不全: gp=${!!gp} fp=${!!fp} simd=${!!simd} setup=${!!setup}`);
  }
  perArch.set(b.arch, { gp, fp, simd, setup: setup[1] });
}

const x64 = perArch.get("X64");
const arm64 = perArch.get("ARM64");
if (!x64 || !arm64) throw new Error("缺少 x64/arm64 块");
if (x64.gp.join() !== "rax,rcx,rdx,rbx,rsi,rdi,r8,r9,r12,r15") throw new Error("x64 GP 缓存寄存器漂移");
if (x64.gp.length !== 10) throw new Error("x64 GP 数漂移");
if (x64.fp.length !== 8) throw new Error("x64 FP 数漂移");
if (x64.setup !== "r12") throw new Error("x64 frame-setup 寄存器漂移");
if (arm64.setup !== "x18" && arm64.setup !== "x0") {
  // arm64 的 frame-setup 寄存器以原文为准, 记录并核对存在性
  console.log(`注意: arm64 kLiftoffFrameSetupFunctionReg = ${arm64.setup}`);
}
if (arm64.gp.length < 10) throw new Error("arm64 GP 数异常");

console.log(`架构块 ${blocks.length}: ${blocks.map((b) => b.arch).join(",")}`);

let rust = banner(
  "src/wasm/baseline/liftoff-assembler-defs.h.js",
  "src/wasm/baseline/liftoff-assembler-defs.h",
  "Liftoff 缓存寄存器表 (x64+arm64 展开, 其余架构登记注释)",
);

function emitArch(mod, t) {
  return `/// ${mod} 目标 (V8_TARGET_ARCH_${mod === "x64" ? "X64" : "ARM64"})
pub mod ${mod} {
  /// kLiftoffAssemblerGpCacheRegs (RegList)
  pub static GP_CACHE_REGS: &[&str] = &[${t.gp.map((r) => `"${r}"`).join(", ")}];
  /// kLiftoffAssemblerFpCacheRegs (DoubleRegList)
  pub static FP_CACHE_REGS: &[&str] = &[${t.fp.map((r) => `"${r}"`).join(", ")}];
  /// kLiftoffAssemblerSimd128CacheRegs (Simd128RegList)
  pub static SIMD128_CACHE_REGS: &[&str] = &[${t.simd.map((r) => `"${r}"`).join(", ")}];
  /// kLiftoffFrameSetupFunctionReg (WasmLiftoffFrameSetup builtin 用)
  pub const FRAME_SETUP_FUNCTION_REG: &str = "${t.setup}";
}
`;
}

rust += `
${emitArch("x64", x64)}
${emitArch("arm64", arm64)}
// —— 其余架构登记 (源文本见 liftoff-assembler-defs.h 对应分支, 本产物不展开) ——
`;

for (const b of blocks) {
  if (b.arch === "X64" || b.arch === "ARM64") continue;
  const t = perArch.get(b.arch);
  rust += `// ${b.arch}: gp=[${t.gp.join(",")}] fp=[${t.fp.join(",")}] simd=[${t.simd.join(",")}] setup=${t.setup}\n`;
}

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(x64::GP_CACHE_REGS.len() == 10);
  assert!(x64::FP_CACHE_REGS.len() == 8);
  assert!(x64::SIMD128_CACHE_REGS.is_empty());
  assert!(x64::GP_CACHE_REGS.contains(&"r15"));
  assert!(!x64::GP_CACHE_REGS.contains(&"r13")); // r13 = kRootRegister
  assert!(!x64::GP_CACHE_REGS.contains(&"r10")); // r10 = kScratchRegister
  assert!(!x64::GP_CACHE_REGS.contains(&"r14")); // r14 = kPtrComprCageBaseRegister
  assert!(!x64::FP_CACHE_REGS.contains(&"xmm15"));
  assert!(!arm64::GP_CACHE_REGS.is_empty());
  assert!(!arm64::FP_CACHE_REGS.is_empty());
};
`;
;

const p = writeSrc("v8-wasm-liftoff", "liftoff_assembler_defs.rs", rust);
console.log(`written ${p}`);
