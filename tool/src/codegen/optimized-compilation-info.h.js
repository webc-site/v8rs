#!/usr/bin/env bun
// tool/src/codegen/optimized-compilation-info.h.js
// 机械提取 v8/src/codegen/optimized-compilation-info.{h,cc} —
// FLAGS X-macro 全量展开 (21 个布尔编译开关位) + 构造签名骨架。
// 产物: v8-codegen-core/src/optimized_compilation_info.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/optimized-compilation-info.h";
const CC_REL = "src/codegen/optimized-compilation-info.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

// FLAGS(V) X-macro 展开: V(Camel, lower, bit)
const flagsM = h.match(/#define FLAGS\(V\)\s*\\\n([\s\S]*?)\n\n/);
if (!flagsM) throw new Error("未找到 FLAGS 宏");
const flags = [...flagsM[1].matchAll(/V\((\w+),\s*(\w+),\s*(\d+)\)/g)].map((m) => ({
  camel: m[1],
  lower: m[2],
  bit: Number(m[3]),
}));
if (flags.length !== 21) throw new Error(`FLAGS 条数不符: ${flags.length} != 21`);
for (let i = 0; i < flags.length; i++) if (flags[i].bit !== i) throw new Error(`位号不连续于 ${i}`);
if (!h.includes("static constexpr int kNoOptimizationId = -1;")) throw new Error("缺 kNoOptimizationId 锚");

let rust = banner("src/codegen/optimized-compilation-info.h.js", "src/codegen/optimized-compilation-info.{h,cc}", "优化编译信息位标志");
rust += `
/// 优化编译开关位标志 (原 FLAGS(V) X-macro 展开, 每位一个布尔开关)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags(u32);

impl Flags {
${flags
  .map(
    (f) => `  /// 原 k${f.camel} (位 ${f.bit}, 访问器 ${f.lower}())\n  pub const ${f.camel.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}: Self = Self(1 << ${f.bit});`,
  )
  .join("\n")}

  #[inline]
  #[must_use]
  pub const fn bits(self) -> u32 {
    self.0
  }

  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    (self.0 & other.0) == other.0
  }

  #[inline]
  #[must_use]
  pub const fn union(self, other: Self) -> Self {
    Self(self.0 | other.0)
  }
}

/// 优化 ID 缺省哨兵 (原 kNoOptimizationId)
pub const NO_OPTIMIZATION_ID: i32 = -1;

/// 原 class OptimizedCompilationInfo — 构造签名骨架:
/// - 原签名: OptimizedCompilationInfo(Zone*, Isolate*, SharedFunctionInfo, JSFunction, CodeKind, BytecodeOffset)
/// - 原签名: 测试构造 (无 OSR)
/// - 原签名: DEF_GETTER/DEF_SETTER × ${flags.length} (每开关一对, X-macro 生成)
pub struct OptimizedCompilationInfo;

// TODO(dev): 补标志生命周期与 bytecode_array 接线。

// —— 提取不变式断言 ——
const _: () = {
  assert!(Flags::FUNCTION_CONTEXT_SPECIALIZING.0 == 1);
  assert!(Flags::SHADOW_STACK_COMPLIANT_LAZY_DEOPT.0 == 1 << 20);
};
`;

const p = writeSrc("v8-codegen-core", "optimized_compilation_info.rs", rust);
console.log(`written ${p} (FLAGS ${flags.length})`);
