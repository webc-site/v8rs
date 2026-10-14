#!/usr/bin/env bun
// tool/src/maglev/arm64/maglev-assembler-arm64-inl.h.js
// 机械提取 v8/src/maglev/arm64/{maglev-assembler-arm64-inl.h, maglev-assembler-arm64.cc} 的规则代码:
//   ConditionForNaN 常量 / ShiftFromScale 缩放表 / MapCompare 临时数 / 栈槽展开常数 /
//   MaglevAssembler 方法签名骨架 (1:1) / 条件编译块展开登记 (arm64 默认配置)
// 权威数值来源: v8/src/codegen/arm64/constants-arm64.h 的 enum Condition

import { readV8, writeSrc, banner } from "../../../lib.js";

const GEN = "src/maglev/arm64/maglev-assembler-arm64-inl.h.js";
const die = (m) => {
  throw new Error(`[${GEN}] ${m}`);
};

const INL_REL = "src/maglev/arm64/maglev-assembler-arm64-inl.h";
const CC_REL = "src/maglev/arm64/maglev-assembler-arm64.cc";
const CONST_REL = "src/codegen/arm64/constants-arm64.h";
const inl = readV8(INL_REL);
const cc = readV8(CC_REL);
const constH = readV8(CONST_REL);

// —— 通用: 解析 C 枚举 `name = value` 条目 (标识符别名查表, 复合表达式跳过) ————
function parseCEnum(src, headerRe, what) {
  const m = src.match(headerRe);
  if (!m) die(`未找到枚举: ${what}`);
  const out = new Map();
  const itemRe = /([A-Za-z_]\w*)\s*=\s*([^,]+?),/g;
  let it;
  while ((it = itemRe.exec(m[1])) !== null) {
    const raw = it[2].trim();
    if (/^-?\d+$/.test(raw)) out.set(it[1], parseInt(raw, 10));
    else if (out.has(raw)) out.set(it[1], out.get(raw));
  }
  return out;
}

// —— 通用: 续行折叠 + 带守卫栈的定义签名扫描 ————
function includeGuardOf(src) {
  const m = src.match(/^#ifndef\s+([A-Za-z_]\w*)\n#define\s+\1\b/m);
  return m ? m[1] : null;
}
function foldContinuations(src) {
  return src.replace(/,\n[ \t]+/g, ", ").replace(/\(\n[ \t]+/g, "(");
}
function scanDefsWithGuards(src, ownerRe, owner) {
  const folded = foldContinuations(src);
  const flines = folded.split("\n");
  const flineIdx = [];
  {
    let acc = 0;
    for (const l of flines) {
      flineIdx.push(acc);
      acc += l.length + 1;
    }
  }
  const lineOf = (off) => {
    let lo = 0,
      hi = flineIdx.length - 1;
    while (lo < hi) {
      const mid = (lo + hi + 1) >> 1;
      if (flineIdx[mid] <= off) lo = mid;
      else hi = mid - 1;
    }
    return lo;
  };
  const ig = includeGuardOf(src);
  const lineStart = [];
  {
    let stack = [];
    for (const l of flines) {
      lineStart.push(stack.slice());
      const t = l.trim();
      if (/^#\s*if/.test(t)) {
        const id = (t.match(/[A-Za-z_]\w*/g) || [])
          .filter((w) => !["if", "ifdef", "ifndef", "defined"].includes(w))
          .pop();
        if (id && id !== ig) stack.push(id);
      } else if (/^#\s*endif/.test(t) && stack.length) {
        stack.pop();
      }
    }
  }
  const defs = [];
  const re = new RegExp(ownerRe.source, "gm");
  let m;
  while ((m = re.exec(folded)) !== null) {
    const li = lineOf(m.index);
    const stack = lineStart[li] || [];
    const guard = stack.length ? stack[stack.length - 1] : null;
    const sigRaw = m[0].slice(0, m[0].length - 1).replace(/\s+/g, " ").trim();
    let depth = 0,
      params = 0,
      hasAny = m[2].trim().length > 0;
    if (hasAny) {
      params = 1;
      for (const ch of m[2]) {
        if (ch === "<") depth++;
        else if (ch === ">") depth--;
        else if (ch === "," && depth === 0) params++;
      }
    }
    defs.push({
      name: m[2],
      params: hasAny ? params : 0,
      guard,
      sig: sigRaw,
    });
  }
  if (!defs.length) die(`${owner} 未提取到任何定义`);
  return defs;
}

function conditionalBlocks(src, dropGuard) {
  const out = new Map();
  for (const l of src.split("\n")) {
    const m = l.trim().match(/^#\s*(?:if|ifdef|ifndef)\s+(.*)$/);
    if (!m) continue;
    const id = (m[1].match(/[A-Za-z_]\w*/g) || [])
      .filter((w) => !["defined", "if", "ifdef", "ifndef"].includes(w))
      .pop();
    if (!id || id === dropGuard) continue;
    out.set(id, (out.get(id) || 0) + 1);
  }
  return out;
}

// —— 1. arm64 Condition 枚举 (数值权威: constants-arm64.h) ————
const armConditions = parseCEnum(
  constH,
  /enum Condition\s*:\s*int\s*\{([\s\S]*?)\n\};/,
  "enum Condition (constants-arm64.h)",
);
for (const [name, val] of [
  ["eq", 0],
  ["ne", 1],
  ["hs", 2],
  ["vs", 6],
  ["al", 14],
]) {
  if (armConditions.get(name) !== val) {
    die(`Condition.${name} 期望 ${val}, 实得 ${armConditions.get(name)}`);
  }
}

// —— 2. ConditionForFloat64 = 平台通用委托; ConditionForNaN = vs ————
const cfMatch = inl.match(
  /constexpr Condition ConditionForFloat64\(Operation operation\)\s*\{([\s\S]*?)\n\}/,
);
if (!cfMatch) die("未找到 ConditionForFloat64");
if (!/return ConditionFor\(operation\);/.test(cfMatch[1])) {
  die("ConditionForFloat64 并非委托 ConditionFor, 需改表提取");
}
const nanMatch = inl.match(/constexpr Condition ConditionForNaN\(\)\s*\{\s*return (\w+);/);
if (!nanMatch) die("未找到 ConditionForNaN");
const nanName = nanMatch[1];
const nanVal = armConditions.get(nanName);
if (nanName !== "vs" || nanVal !== 6) {
  die(`ConditionForNaN 期望 vs=6, 实得 ${nanName}=${nanVal}`);
}

// —— 3. ShiftFromScale: int → LSL 移位量 ————
const shMatch = inl.match(/inline int ShiftFromScale\(int n\)\s*\{([\s\S]*?)\n\}/);
if (!shMatch) die("未找到 ShiftFromScale");
const shCases = [...shMatch[1].matchAll(/case (\d+):\s*return (\d+);/g)];
if (shCases.length !== 4) die(`ShiftFromScale case ${shCases.length} != 4`);
const shiftFromScale = shCases.map((m) => [parseInt(m[1], 10), parseInt(m[2], 10)]);

// —— 4. MapCompare::TemporaryCount 恒 1 (arm64: 临时寄存器总是要 1 个) ————
const tcMatch = cc.match(
  /int MapCompare::TemporaryCount\(size_t map_count\)\s*\{\s*return (\d+);/,
) || inl.match(/int MapCompare::TemporaryCount\(size_t map_count\)\s*\{\s*return (\d+);/);
if (!tcMatch) die("未找到 MapCompare::TemporaryCount");
if (tcMatch[1] !== "1") die(`TemporaryCount 期望 1, 实得 ${tcMatch[1]}`);

// —— 5. Prologue 栈槽初始化循环展开常数 (arm64 .cc) ————
const unrollCtx = cc.match(
  /\/\/ Magic value\. Experimentally, an unroll size of (\d+) doesn't seem any[\s\S]*?const int kLoopUnrollSize = (\d+);/,
);
if (!unrollCtx) die("未找到 kLoopUnrollSize");
if (unrollCtx[1] !== unrollCtx[2]) die("kLoopUnrollSize 注释与值不一致");

// —— 6. 条件编译块分布 (arm64 默认配置: COMPRESS_POINTERS=true, SANDBOX=true;
//      V8_STATIC_ROOTS 未定义→false; V8_ENABLE_UNDEFINED_DOUBLE 未启用;
//      V8_VERIFY_WRITE_BARRIERS 未定义→false) ————
const inlBlocks = conditionalBlocks(inl, includeGuardOf(inl));
const ccBlocks = conditionalBlocks(cc, null);
const expectInlBlocks = [
  ["V8_ENABLE_SANDBOX", 3, true],
  ["V8_COMPRESS_POINTERS", 1, true],
  ["V8_STATIC_ROOTS_BOOL", 1, false],
  ["V8_ENABLE_UNDEFINED_DOUBLE", 1, false],
];
const expectCcBlocks = [
  ["V8_VERIFY_WRITE_BARRIERS", 1, false],
  ["V8_STATIC_ROOTS_BOOL", 3, false],
];
for (const [id, n] of expectInlBlocks) {
  if (inlBlocks.get(id) !== n) die(`-inl 条件块 ${id} ${inlBlocks.get(id)} != ${n}`);
}
for (const [id, n] of expectCcBlocks) {
  if (ccBlocks.get(id) !== n) die(`.cc 条件块 ${id} ${ccBlocks.get(id)} != ${n}`);
}

// —— 7. 签名骨架 ————
const inlDefs = scanDefsWithGuards(
  inl,
  /^((?:template\s*<[^\n>]*>\s*)?(?:inline\s+)?[A-Za-z_][\w:<>,&* \t]*?)MaglevAssembler::(\w+)\s*\(([^{};]*)\)\s*\{/,
  "MaglevAssembler(-inl)",
);
if (inlDefs.length !== 185) die(`-inl MaglevAssembler 定义 ${inlDefs.length} != 185`);
const inlUnique = new Set(inlDefs.map((d) => d.name));
if (inlUnique.size !== 143) die(`-inl 唯一名 ${inlUnique.size} != 143`);
const guardCounts = {};
for (const d of inlDefs) {
  if (d.guard) guardCounts[d.guard] = (guardCounts[d.guard] || 0) + 1;
}
if (guardCounts.V8_ENABLE_SANDBOX !== 1) die(`SANDBOX 守卫方法 ${guardCounts.V8_ENABLE_SANDBOX} != 1`);
if (guardCounts.V8_STATIC_ROOTS_BOOL !== 3) die(`STATIC_ROOTS 守卫方法 ${guardCounts.V8_STATIC_ROOTS_BOOL} != 3`);
if (guardCounts.V8_ENABLE_UNDEFINED_DOUBLE !== 3) die(`UNDEFINED_DOUBLE 守卫方法 ${guardCounts.V8_ENABLE_UNDEFINED_DOUBLE} != 3`);

const mapCompareDefs = scanDefsWithGuards(
  inl,
  /^((?:inline\s+)?[A-Za-z_][\w:<>,&* \t]*?)MapCompare::(\w+)\s*\(([^{};]*)\)\s*\{/,
  "MapCompare",
);
if (mapCompareDefs.length !== 4) die(`MapCompare 定义 ${mapCompareDefs.length} != 4`);

const ccDefs = scanDefsWithGuards(
  cc,
  /^((?:template\s*<[^\n>]*>\s*)?(?:inline\s+)?[A-Za-z_][\w:<>,&* \t]*?)MaglevAssembler::(\w+)\s*\(([^{};]*)\)\s*\{/,
  "MaglevAssembler(.cc)",
);
if (ccDefs.length !== 15) die(`.cc MaglevAssembler 定义 ${ccDefs.length} != 15`);
const ccUnique = new Set(ccDefs.map((d) => d.name));
if (ccUnique.size !== 14) die(`.cc 唯一名 ${ccUnique.size} != 14`);

// .cc 自由函数 (SubSizeAndTagObject ×2 重载 + AllocateRaw 模板)
const ccFreeDefs = scanDefsWithGuards(
  cc,
  /^((?:template\s*<[^\n>]*>\s*)?(?:inline\s+)?[A-Za-z_][\w:<>,&* \t]*?)(SubSizeAndTagObject|AllocateRaw)\s*\(([^{};]*)\)\s*\{/,
  "cc 自由函数",
);
const ccFreeNames = ccFreeDefs.map((d) => d.name).sort().join(",");
if (ccFreeNames !== "AllocateRaw,SubSizeAndTagObject,SubSizeAndTagObject") {
  die(`.cc 自由函数异常: ${ccFreeNames}`);
}

// —— 8. 结构锚点: -inl 的 MaglevAssembler:: 全部出现次数分解 ————
// 185 定义 + 1 类声明(class TemporaryRegisterScope) + 8 局部 temps 声明 + 3 形参声明
const inlOcc = (inl.match(/MaglevAssembler::/g) || []).length;
const tempsLocals = (inl.match(/MaglevAssembler::TemporaryRegisterScope \w+\(/g) || []).length;
const tempsParams = (inl.match(/MaglevAssembler::TemporaryRegisterScope\*/g) || []).length;
if (inlOcc !== 185 + 1 + tempsLocals + tempsParams) {
  die(`-inl 出现次数 ${inlOcc} != 185 defs + 1 class + ${tempsLocals} locals + ${tempsParams} params`);
}
if (tempsLocals !== 8) die(`-inl 局部 temps 声明 ${tempsLocals} != 8`);
if (tempsParams !== 3) die(`-inl 形参 temps 声明 ${tempsParams} != 3`);

// —— 生成 Rust 模块 ————
const lit = (s) => JSON.stringify(s);
function emitSigs(defs) {
  return defs
    .map(
      (d) =>
        `  MethodSig { name: ${lit(d.name)}, params: ${d.params}, guard: ${d.guard ? `Some(${lit(d.guard)})` : "None"}, sig: ${lit(d.sig)} },`,
    )
    .join("\n");
}
function emitBlocks(rows) {
  return rows
    .map(([id, n, on]) => `  (${lit(id)}, ${n}, ${on ? "true" : "false"}),`)
    .join("\n");
}

let rust = banner(
  GEN,
  `${INL_REL} + ${CC_REL} (+ ${CONST_REL} Condition 数值)`,
  "arm64 MaglevAssembler: NaN 条件/缩放表/签名骨架 (机械基线)",
);

rust += `
//! 产物为规则代码基线: 表格与签名清单 1:1 对应原版;
//! 方法体语义 (Ldr/Fcmp 等) 不在此展开, 待 Developer 就地实现。
//! ConditionForFloat64 原版为平台通用委托 (return ConditionFor(operation)),
//! 无 arm64 特化表, 故不产映射表。

// —— 条件编译块展开登记 (arm64 默认配置) ——
// COMPRESS_POINTERS=true, V8_ENABLE_SANDBOX=true (v8-common/src/globals/config.rs);
// V8_STATIC_ROOTS 未定义 → V8_STATIC_ROOTS_BOOL=false (include/v8config.h);
// V8_ENABLE_UNDEFINED_DOUBLE 未启用 (config.rs V8_UNDEFINED_DOUBLE=false);
// V8_VERIFY_WRITE_BARRIERS 未定义 → false (flag-definitions.h 门)。
#[rustfmt::skip]
pub static INL_CONDITIONAL_BLOCKS: &[(&str, usize, bool)] = &[
${emitBlocks(expectInlBlocks)}
];

/// .cc 体内条件编译块 (均在函数体内部, 非方法级守卫)
#[rustfmt::skip]
pub static CC_CONDITIONAL_BLOCKS: &[(&str, usize, bool)] = &[
${emitBlocks(expectCcBlocks)}
];

/// ConditionForNaN: arm64 浮点无序比较置溢出位 (原版 constexpr ConditionForNaN)
pub const CONDITION_FOR_NAN: (&str, i8) = (${lit(nanName)}, ${nanVal});

/// 原版 ShiftFromScale(n): {1,2,4,8} → LSL 移位量 (MemOperand 缩放)
#[inline]
#[must_use]
pub const fn shift_from_scale(n: i32) -> Option<u32> {
  match n {
`;
for (const [n, v] of shiftFromScale) {
  rust += `    ${n} => Some(${v}),\n`;
}
rust += `    _ => None,
  }
}

/// 原版 MapCompare::TemporaryCount: arm64 恒返回 1 (x64 为 map_count==1 ? 0 : 1)
pub const MAP_COMPARE_TEMPORARY_COUNT: usize = ${tcMatch[1]};

/// 原版 kLoopUnrollSize: Prologue 栈槽初始化循环展开宽度 (arm64 .cc 实验常数)
pub const LOOP_UNROLL_SIZE: i32 = ${unrollCtx[2]};

/// 方法签名骨架: name / 参数个数 / 外层条件编译守卫 / C++ 原文签名 (折叠空白)
#[derive(Debug, Clone, Copy)]
pub struct MethodSig {
  pub name: &'static str,
  pub params: u8,
  pub guard: Option<&'static str>,
  pub sig: &'static str,
}

/// MaglevAssembler 内联方法定义 (-inl.h, 185 处, 143 个唯一名)
#[rustfmt::skip]
pub static INL_METHODS: &[MethodSig] = &[
${emitSigs(inlDefs)}
];

/// MaglevAssembler 方法定义 (.cc, 15 处, 14 个唯一名: Allocate ×2 等)
#[rustfmt::skip]
pub static CC_METHODS: &[MethodSig] = &[
${emitSigs(ccDefs)}
];

/// MapCompare 架构内联实现 (4: 构造/Generate/GetMap/TemporaryCount;
/// TemporaryCount 见 MAP_COMPARE_TEMPORARY_COUNT)
#[rustfmt::skip]
pub static MAP_COMPARE_METHODS: &[MethodSig] = &[
${emitSigs(mapCompareDefs)}
];

/// .cc 自由函数 (SubSizeAndTagObject ×2 重载 + AllocateRaw 模板)
#[rustfmt::skip]
pub static CC_FREE_FUNCTIONS: &[MethodSig] = &[
${emitSigs(ccFreeDefs)}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(CONDITION_FOR_NAN.1 == 6);
  assert!(INL_CONDITIONAL_BLOCKS.len() == 4);
  assert!(CC_CONDITIONAL_BLOCKS.len() == 2);
  assert!(INL_METHODS.len() == 185);
  assert!(CC_METHODS.len() == 15);
  assert!(MAP_COMPARE_METHODS.len() == 4);
  assert!(CC_FREE_FUNCTIONS.len() == 3);
  assert!(MAP_COMPARE_TEMPORARY_COUNT == 1);
  assert!(LOOP_UNROLL_SIZE == 8);
};

// 守卫方法计数 (slice::iter 非 const 稳定, 放测试环)
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn guard_counts() {
    assert_eq!(
      INL_METHODS.iter().filter(|d| d.guard == Some("V8_STATIC_ROOTS_BOOL")).count(),
      3
    );
    assert_eq!(
      INL_METHODS.iter().filter(|d| d.guard == Some("V8_ENABLE_UNDEFINED_DOUBLE")).count(),
      3
    );
  }
}
`;

const p = writeSrc("v8-maglev-backend", "arm64/maglev_assembler_arm64.rs", rust);
console.log(
  `OK ${GEN}: nan=${nanName}:${nanVal} shift=${shiftFromScale.length} inl=${inlDefs.length}(${inlUnique.size}名) cc=${ccDefs.length}(${ccUnique.size}名) free=${ccFreeDefs.length} mapcmp=${mapCompareDefs.length} unroll=${unrollCtx[2]} → ${p}`,
);
