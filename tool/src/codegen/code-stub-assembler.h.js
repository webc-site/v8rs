#!/usr/bin/env bun
// tool/src/codegen/code-stub-assembler.h.js
// 机械提取 v8/src/codegen/code-stub-assembler.{h,-inl.h,cc} —
// AllocationFlag/SlackTrackingMode/ExtractFixedArrayFlag/HoleConversionMode 枚举
// + 生成宏族登记 + 方法面骨架 (22155 行 cc 为语义面, 规则 10 只取骨架)。
// 产物: v8-codegen-core/src/code_stub_assembler_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/code-stub-assembler.h";
const INL_REL = "src/codegen/code-stub-assembler-inl.h";
const CC_REL = "src/codegen/code-stub-assembler.cc";
const h = readV8(H_REL);
readV8(INL_REL); // 家族覆盖锚
readV8(CC_REL); // 家族覆盖锚

// 1. AllocationFlag : uint8_t
const afM = h.match(/enum class AllocationFlag : uint8_t\s*\{([^}]+)\};/);
if (!afM) throw new Error("未找到 AllocationFlag 枚举");
const af = [...afM[1].matchAll(/k(\w+)\s*=\s*([^,\n]+)/g)].map((m) => ({ name: m[1], expr: m[2].trim() }));
if (af.length !== 3 || af[0].expr !== "0" || af[1].expr !== "1" || af[2].expr !== "1 << 1") {
  throw new Error(`AllocationFlag 条目异常: ${JSON.stringify(af)}`);
}

// 2. SlackTrackingMode
const stmM = h.match(/enum SlackTrackingMode\s*\{([^}]+)\};/);
if (!stmM) throw new Error("未找到 SlackTrackingMode 枚举");
const stm = [...stmM[1].matchAll(/k(\w+)/g)].map((m) => m[1]);
if (stm.length !== 3) throw new Error(`SlackTrackingMode 条数不符: ${stm.length} != 3`);

// 3. ExtractFixedArrayFlag (位 + 组合)
const efaM = h.match(/enum class ExtractFixedArrayFlag\s*\{([^}]+)\};/);
if (!efaM) throw new Error("未找到 ExtractFixedArrayFlag 枚举");
const efa = [...efaM[1].matchAll(/k(\w+)\s*=\s*([^,\n]+)/g)].map((m) => ({ name: m[1], expr: m[2].trim() }));
if (efa.length !== 5) throw new Error(`ExtractFixedArrayFlag 条数不符: ${efa.length} != 5`);

// 4. HoleConversionMode
const hcmM = h.match(/enum class HoleConversionMode\s*\{([^}]+)\};/);
if (!hcmM) throw new Error("未找到 HoleConversionMode 枚举");
const hcm = [...hcmM[1].matchAll(/k(\w+)/g)].map((m) => m[1]);
if (hcm.length !== 2) throw new Error(`HoleConversionMode 条数不符: ${hcm.length} != 2`);

// 5. 生成宏族登记
const macros = [...h.matchAll(/^#define ((?:PARAMETER_BINOP|SMI_ARITHMETIC_BINOP|SMI_COMPARISON_OP|BINT_COMPARISON_OP|HEAP_CONSTANT_ACCESSOR|HEAP_CONSTANT_TEST))\(/gm)].map((m) => m[1]);
if (macros.length < 6) throw new Error(`生成宏族不足: ${macros.length}`);

// 方法面计数 (TNode 返回的公开方法 ≈ CSA 原语面)
const methodCount = [...h.matchAll(/^\s+TNode<[^>]+>\s+(\w+)\(/gm)].map((m) => m[1]);
if (methodCount.length < 400) throw new Error(`CSA 方法面异常: ${methodCount.length}`);

let rust = banner("src/codegen/code-stub-assembler.h.js", "src/codegen/code-stub-assembler.{h,-inl.h,cc}", "CSA 枚举与骨架");
rust += `
// 语义面: CSA ${methodCount.length}+ 个 TNode 原语 (cc ${readV8(CC_REL).split("\n").length} 行)
// 登记跳过, 由 Developer 按原语表分批补齐。此处产出规则枚举与宏族登记。

/// 分配标志 (原 enum class AllocationFlag : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AllocationFlag {
  None = 0,
  DoubleAlignment = 1,
  Pretenured = 1 << 1,
}

/// 位集别名 (原 base::Flags<AllocationFlag>)
pub type AllocationFlags = u8;

/// 松弛追踪模式 (原 enum SlackTrackingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SlackTrackingMode {
${stm.map((s, i) => `  /// k${s}\n  ${s} = ${i},`).join("\n")}
}

/// 数组拷贝选择标志 (原 enum class ExtractFixedArrayFlag, 位集)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtractFixedArrayFlag(pub u8);

impl ExtractFixedArrayFlag {
${efa.map((f) => `  /// 原 k${f.name} = ${f.expr}\n  pub const ${f.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}: Self = Self(${f.expr.replace(/k(\w+)/g, (_, n) => `Self::${n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}.0`)});`).join("\n")}

  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    (self.0 & other.0) == other.0
  }
}

/// 洞值转换模式 (原 enum class HoleConversionMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HoleConversionMode {
${hcm.map((s, i) => `  /// k${s}\n  ${s} = ${i},`).join("\n")}
}

// 原生成宏族 (展开为方法矩阵):
${macros.map((m) => `// - ${m}`).join("\n")}

/// 原 class CodeStubAssembler 骨架 (原语方法 ${methodCount.length} 处 TNode 签名)
pub struct CodeStubAssembler;

// —— 提取不变式断言 ——
const _: () = {
  assert!(AllocationFlag::Pretenured as u8 == 2);
  assert!(ExtractFixedArrayFlag::ALL_FIXED_ARRAYS.0 == 3);
};
`;

const p = writeSrc("v8-codegen-core", "code_stub_assembler_generated.rs", rust);
console.log(`written ${p} (枚举 4, 宏 ${macros.length}, 原语 ${methodCount.length})`);
