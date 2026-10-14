#!/usr/bin/env bun
// tool/src/maglev/x64/maglev-assembler-x64-inl.h.js
// 机械提取 v8/src/maglev/x64/maglev-assembler-x64-inl.h 的规则代码:
//   Operation→x64 Condition 映射表 / ScaleFactorFromInt 缩放表 / ConditionForNaN 常量 /
//   MaglevAssembler 方法签名骨架 (1:1) / 条件编译块展开登记 (x64 默认配置)
// 权威数值来源: v8/src/codegen/x64/assembler-x64.h 的 enum Condition / enum ScaleFactor

import { readV8, writeSrc, banner } from "../../../lib.js";

const GEN = "src/maglev/x64/maglev-assembler-x64-inl.h.js";
const die = (m) => {
  throw new Error(`[${GEN}] ${m}`);
};

const INL_REL = "src/maglev/x64/maglev-assembler-x64-inl.h";
const ASM_H_REL = "src/codegen/x64/assembler-x64.h";
const inl = readV8(INL_REL);
const asmH = readV8(ASM_H_REL);

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
    // 复合表达式 (如 kSpecialCondition) 不入表
  }
  return out;
}

// —— 1. x64 Condition 枚举 (数值权威) ————
const conditions = parseCEnum(
  asmH,
  /enum Condition\s*:\s*int\s*\{([\s\S]*?)\};/,
  "enum Condition",
);
for (const [name, val] of [
  ["equal", 4],
  ["below", 2],
  ["below_equal", 6],
  ["above", 7],
  ["above_equal", 3],
  ["parity_even", 10],
]) {
  if (conditions.get(name) !== val) {
    die(`Condition.${name} 期望 ${val}, 实得 ${conditions.get(name)}`);
  }
}
if (conditions.size < 16) die(`Condition 条目 ${conditions.size} < 16`);

// —— 2. x64 ScaleFactor 枚举 ————
const scaleFactors = parseCEnum(
  asmH,
  /enum ScaleFactor\s*:\s*int8_t\s*\{([\s\S]*?)\};/,
  "enum ScaleFactor",
);
for (const [name, val] of [
  ["times_1", 0],
  ["times_2", 1],
  ["times_4", 2],
  ["times_8", 3],
]) {
  if (scaleFactors.get(name) !== val) {
    die(`ScaleFactor.${name} 期望 ${val}, 实得 ${scaleFactors.get(name)}`);
  }
}

// —— 3. ConditionForFloat64: Operation → Condition 映射 (constexpr switch) ————
const cfMatch = inl.match(
  /constexpr Condition ConditionForFloat64\(Operation operation\)\s*\{([\s\S]*?)\n\}/,
);
if (!cfMatch) die("未找到 ConditionForFloat64");
const cfCases = [...cfMatch[1].matchAll(/case Operation::(\w+)/g)].map(
  (m) => m[1],
);
const cfReturns = [...cfMatch[1].matchAll(/return (\w+);/g)].map((m) => m[1]);
if (cfCases.length !== 6) die(`ConditionForFloat64 case ${cfCases.length} != 6`);
if (cfReturns.length !== 5)
  die(`ConditionForFloat64 return ${cfReturns.length} != 5`);
// case 标签与 return 按序对齐: [kEqual,kStrictEqual]→equal, kLessThan→below, ...
const cfGroups = [
  ["kEqual", "kStrictEqual"],
  ["kLessThan"],
  ["kLessThanOrEqual"],
  ["kGreaterThan"],
  ["kGreaterThanOrEqual"],
];
const condForFloat64 = [];
cfReturns.forEach((cond, gi) => {
  const group = cfGroups[gi];
  if (!group) die(`ConditionForFloat64 第 ${gi} 组缺失`);
  for (const op of group) {
    const cv = conditions.get(cond);
    if (cv === undefined) die(`Condition ${cond} 无数值`);
    condForFloat64.push([op, cond, cv]);
  }
});
if (condForFloat64.length !== 6) die("ConditionForFloat64 对齐后 != 6 条");

// —— 4. ConditionForNaN ————
const nanMatch = inl.match(/constexpr Condition ConditionForNaN\(\)\s*\{\s*return (\w+);/);
if (!nanMatch) die("未找到 ConditionForNaN");
const nanName = nanMatch[1];
const nanVal = conditions.get(nanName);
if (nanName !== "parity_even" || nanVal !== 10) {
  die(`ConditionForNaN 期望 parity_even=10, 实得 ${nanName}=${nanVal}`);
}

// —— 5. ScaleFactorFromInt: int → ScaleFactor ————
const sfMatch = inl.match(
  /inline ScaleFactor ScaleFactorFromInt\(int n\)\s*\{([\s\S]*?)\n\}/,
);
if (!sfMatch) die("未找到 ScaleFactorFromInt");
const sfCases = [...sfMatch[1].matchAll(/case (\d+):\s*return (\w+);/g)];
if (sfCases.length !== 4) die(`ScaleFactorFromInt case ${sfCases.length} != 4`);
const scaleFromInt = sfCases.map((m) => {
  const n = parseInt(m[1], 10);
  const sfv = scaleFactors.get(m[2]);
  if (sfv === undefined) die(`ScaleFactor ${m[2]} 无数值`);
  return [n, m[2], sfv];
});

// —— 6. 条件编译块分布 (x64 默认配置: COMPRESS_POINTERS=true, V8_ENABLE_SANDBOX=true;
//      V8_STATIC_ROOTS 未定义→false; V8_ENABLE_UNDEFINED_DOUBLE 未启用; V8_DUMPLING 未启用) ————
const blockCounts = new Map();
for (const l of inl.split("\n")) {
  const m = l.trim().match(/^#\s*(?:if|ifdef|ifndef)\s+(.*)$/);
  if (!m) continue;
  const id = (m[1].match(/[A-Za-z_]\w*/g) || [])
    .filter((w) => !["defined", "if", "ifdef", "ifndef"].includes(w))
    .pop();
  if (id) blockCounts.set(id, (blockCounts.get(id) || 0) + 1);
}
const expectBlocks = [
  // [宏, 块数, 按 x64 默认配置是否启用]
  ["V8_ENABLE_SANDBOX", 3, true],
  ["V8_COMPRESS_POINTERS", 1, true],
  ["V8_STATIC_ROOTS_BOOL", 1, false],
  ["V8_ENABLE_UNDEFINED_DOUBLE", 1, false],
  ["V8_DUMPLING", 1, false],
];
for (const [id, n, on] of expectBlocks) {
  if (blockCounts.get(id) !== n) die(`条件块 ${id} ${blockCounts.get(id)} != ${n}`);
}
if (blockCounts.size !== 6) die(`条件宏种类 ${blockCounts.size} != 6 (含 include guard)`);

// —— 7. 方法定义签名骨架 (含外层守卫宏跟踪) ————
const GUARD_ON = { V8_ENABLE_SANDBOX: true, V8_COMPRESS_POINTERS: true };
// 文件头 include guard (#ifndef X + #define X) 不算特性守卫
function includeGuardOf(src) {
  const m = src.match(/^#ifndef\s+([A-Za-z_]\w*)\n#define\s+\1\b/m);
  return m ? m[1] : null;
}
// 定义头扫描前处理: 续行折叠成单行 (",\n  " / "(\n  " → 连接), 守卫指令行保留
function foldContinuations(src) {
  return src.replace(/,\n[ \t]+/g, ", ").replace(/\(\n[ \t]+/g, "(");
}
function scanDefsWithGuards(src, ownerRe, owner) {
  const folded = foldContinuations(src);
  // 折叠后仍按原行序跟踪守卫 (#if/#endif 相对顺序不变), 直接在折叠文本上建栈
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
  // 守卫栈: lineStart[i] = 该行开始时的栈 (include guard 不入栈)
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
    // 顶层逗号分割参数
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

const asmDefs = scanDefsWithGuards(
  inl,
  /^((?:template\s*<[^\n>]*>\s*)?(?:inline\s+)?[A-Za-z_][\w:<>,&* \t]*?)MaglevAssembler::(\w+)\s*\(([^{};]*)\)\s*\{/,
  "MaglevAssembler",
);
if (asmDefs.length !== 183) die(`MaglevAssembler 定义 ${asmDefs.length} != 183`);
const uniqueNames = new Set(asmDefs.map((d) => d.name));
if (uniqueNames.size !== 143) die(`MaglevAssembler 唯一名 ${uniqueNames.size} != 143`);
const guarded = asmDefs.filter((d) => d.guard);
const guardSandbox = guarded.filter((d) => d.guard === "V8_ENABLE_SANDBOX");
const guardStaticRoots = guarded.filter((d) => d.guard === "V8_STATIC_ROOTS_BOOL");
const guardUndefined = guarded.filter((d) => d.guard === "V8_ENABLE_UNDEFINED_DOUBLE");
if (guardSandbox.length !== 1) die(`SANDBOX 守卫方法 ${guardSandbox.length} != 1`);
if (guardStaticRoots.length !== 3) die(`STATIC_ROOTS 守卫方法 ${guardStaticRoots.length} != 3`);
if (guardUndefined.length !== 3) die(`UNDEFINED_DOUBLE 守卫方法 ${guardUndefined.length} != 3`);

const mapCompareDefs = scanDefsWithGuards(
  inl,
  /^((?:inline\s+)?[A-Za-z_][\w:<>,&* \t]*?)MapCompare::(\w+)\s*\(([^{};]*)\)\s*\{/,
  "MapCompare",
);
if (mapCompareDefs.length !== 4) die(`MapCompare 定义 ${mapCompareDefs.length} != 4`);

// 文件级自由函数 (ConditionForFloat64 / ConditionForNaN / ScaleFactorFromInt)
const freeFns = [
  "constexpr Condition ConditionForFloat64(Operation operation)",
  "constexpr Condition ConditionForNaN()",
  "inline ScaleFactor ScaleFactorFromInt(int n)",
];

// —— 类结构登记 (TemporaryRegisterScope, 类体内方法按锚点核对) ————
const scopeAnchors = [
  "struct SavedData",
  "explicit TemporaryRegisterScope(MaglevAssembler* masm)",
  "Register AcquireScratch()",
  "DoubleRegister AcquireScratchDouble()",
  "void IncludeScratch(Register reg)",
  "SavedData CopyForDefer()",
  "void ResetToDefaultImpl()",
];
for (const a of scopeAnchors) {
  if (!inl.includes(a)) die(`TemporaryRegisterScope 锚缺失: ${a}`);
}

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

let rust = banner(
  GEN,
  `${INL_REL} + ${ASM_H_REL} (Condition/ScaleFactor 数值)`,
  "x64 MaglevAssembler 内联层: 条件映射/缩放表/签名骨架 (机械基线)",
);

rust += `
//! 产物为规则代码基线: 表格与签名清单 1:1 对应原版;
//! 方法体语义 (movq/Ucomisd 等) 不在此展开, 待 Developer 就地实现。

// —— 条件编译块展开登记 (x64 默认配置) ————
// COMPRESS_POINTERS=true, V8_ENABLE_SANDBOX=true (v8-common/src/globals/config.rs);
// V8_STATIC_ROOTS 未定义 → V8_STATIC_ROOTS_BOOL=false (include/v8config.h);
// V8_ENABLE_UNDEFINED_DOUBLE 未启用 (config.rs V8_UNDEFINED_DOUBLE=false);
// V8_DUMPLING 本仓非构建开关, 未启用。
#[rustfmt::skip]
pub static CONDITIONAL_BLOCKS: &[(&str, usize, bool)] = &[
`;
for (const [id, n, on] of expectBlocks) {
  rust += `  (${lit(id)}, ${n}, ${on ? "true" : "false"}),\n`;
}
rust += `];

/// ConditionForFloat64: Operation → x64 Condition (名 + 数值, 数值取自 assembler-x64.h)
#[rustfmt::skip]
pub static CONDITION_FOR_FLOAT64: &[(&str, &str, i8)] = &[
`;
for (const [op, cond, val] of condForFloat64) {
  rust += `  (${lit(op)}, ${lit(cond)}, ${val}),\n`;
}
rust += `];

/// ConditionForNaN: x64 浮点无序比较走奇偶标志位 (原版 constexpr ConditionForNaN)
pub const CONDITION_FOR_NAN: (&str, i8) = (${lit(nanName)}, ${nanVal});

/// ScaleFactor 枚举 (assembler-x64.h): 缩放因子 = log2(步长)
#[rustfmt::skip]
pub static SCALE_FACTOR: &[(&str, i32)] = &[
`;
for (const [name, val] of scaleFactors) {
  rust += `  (${lit(name)}, ${val}),\n`;
}
rust += `];

/// 原版 ScaleFactorFromInt(n): {1,2,4,8} → 缩放因子
#[inline]
#[must_use]
pub const fn scale_factor_from_int(n: i32) -> Option<u32> {
  match n {
`;
for (const [n, , v] of scaleFromInt) {
  rust += `    ${n} => Some(${v}),\n`;
}
rust += `    _ => None,
  }
}

/// 方法签名骨架: name / 参数个数 / 外层条件编译守卫 / C++ 原文签名 (折叠空白)
#[derive(Debug, Clone, Copy)]
pub struct MethodSig {
  pub name: &'static str,
  pub params: u8,
  pub guard: Option<&'static str>,
  pub sig: &'static str,
}

/// MaglevAssembler 内联方法定义 (183 处, 按源文件顺序; 含 143 个唯一名)
#[rustfmt::skip]
pub static ASM_METHODS: &[MethodSig] = &[
${emitSigs(asmDefs)}
];

/// MapCompare 架构内联实现 (4: 构造/Generate/GetMap/TemporaryCount;
/// TemporaryCount 规则: map_count==1 → 0, 否则 1)
#[rustfmt::skip]
pub static MAP_COMPARE_METHODS: &[MethodSig] = &[
${emitSigs(mapCompareDefs)}
];

/// 文件级自由函数签名 (规则函数, 映射表见上)
#[rustfmt::skip]
pub static FREE_FUNCTIONS: &[&str] = &[
`;

for (const f of freeFns) rust += `  ${lit(f)},\n`;
rust += `];

// —— 类结构登记 ————
// MaglevAssembler::TemporaryRegisterScope (Base=TemporaryRegisterScopeBase):
//   SavedData{has_scratch_register_, has_double_scratch_register_};
//   AcquireScratch→kScratchRegister, AcquireScratchDouble→kScratchDoubleReg;
//   ResetToDefaultImpl 双布尔复位。${scopeAnchors.length} 个结构锚点全过。

// —— 提取不变式断言 ——
const _: () = {
  assert!(CONDITION_FOR_FLOAT64.len() == 6);
  assert!(CONDITION_FOR_FLOAT64[0].2 == 4);
  assert!(CONDITION_FOR_FLOAT64[1].2 == 4);
  assert!(CONDITION_FOR_NAN.1 == 10);
  assert!(SCALE_FACTOR.len() == ${scaleFactors.size});
  assert!(ASM_METHODS.len() == 183);
  assert!(MAP_COMPARE_METHODS.len() == 4);
  assert!(FREE_FUNCTIONS.len() == ${freeFns.length});
  assert!(CONDITIONAL_BLOCKS.len() == ${expectBlocks.length});
};

// 守卫方法计数 (slice::iter 非 const 稳定, 放测试环)
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn guard_counts() {
    assert_eq!(
      ASM_METHODS.iter().filter(|d| d.guard == Some("V8_STATIC_ROOTS_BOOL")).count(),
      3
    );
    assert_eq!(
      ASM_METHODS.iter().filter(|d| d.guard == Some("V8_ENABLE_UNDEFINED_DOUBLE")).count(),
      3
    );
  }
}
`;

const p = writeSrc("v8-maglev-backend", "x64/maglev_assembler_x64_inl.rs", rust);
console.log(
  `OK ${GEN}: cond_map=${condForFloat64.length} nan=${nanName}:${nanVal} scale=${scaleFactors.size} asm=${asmDefs.length}(${uniqueNames.size}名) mapcmp=${mapCompareDefs.length} 条件块=${blockCounts.size - 1}(不含guard) → ${p}`,
);
