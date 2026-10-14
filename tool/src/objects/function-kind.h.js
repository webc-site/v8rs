#!/usr/bin/env bun
// tool/src/objects/function-kind.h.js — 覆盖 v8/src/objects/function-kind.h
// FunctionKind 位域枚举 + 谓词族 (IsInRange 区间/switch/call 组合) 机械直译
// → v8-objects-js-func/src/function_kind.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/function-kind.h");

// —— Rust 命名辅助 ——
const pascal = (k) => k.replace(/^k/, "").replace(/^./, (c) => c.toUpperCase());
const snake = (fn) =>
  fn.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

// 1. 枚举体 (连同分组注释)
const enumMatch = hSrc.match(/enum class FunctionKind : uint8_t \{([\s\S]*?)\n\};/);
if (!enumMatch) throw new Error("未找到 FunctionKind 枚举");

const entries = [];
for (const line of enumMatch[1].split("\n")) {
  const t = line.trim();
  if (!t) continue;
  const cm = t.match(/^(k\w+),$/);
  if (cm) {
    entries.push({ raw: cm[1], name: pascal(cm[1]), comment: "" });
    continue;
  }
  if (t.startsWith("kLastFunctionKind")) continue; // 哨兵别名, 不产变体
  if (t.startsWith("//")) {
    if (entries.length === 0) entries.push({ header: [] , comment: "" });
    const slot = entries[0].header ?? (entries[entries.length - 1].tail ??= []);
    slot.push(t);
    continue;
  }
  throw new Error(`FunctionKind 枚举含未识别行: ${t}`);
}
// 收集到 last 之前的有效变体 (kInvalid 亦为变体, 保留)
const variants = entries.filter((e) => e.raw && e.raw !== "kInvalid");
const invalid = entries.find((e) => e.raw === "kInvalid");
if (!invalid) throw new Error("未找到 kInvalid 哨兵");
if (variants.length !== 28) throw new Error(`FunctionKind 变体数不符: 期望 28 (不含 kInvalid), 得 ${variants.length}`);

// 2. 位宽
const bitSizeMatch = hSrc.match(/constexpr int kFunctionKindBitSize = (\d+);/);
if (!bitSizeMatch) throw new Error("未找到 kFunctionKindBitSize");
const bitSize = Number(bitSizeMatch[1]);
if (variants.length >= 1 << bitSize) throw new Error("kFunctionKindBitSize 与变体数矛盾");

// 3. 谓词族: 逐个提取 inline bool Xxx(...)
const predicates = [];
for (const pm of hSrc.matchAll(/inline bool (Is\w+|BindsSuper)\(FunctionKind kind\) \{([\s\S]*?)\n\}/g)) {
  const name = pm[1];
  let body = pm[2].replace(/\/\/[^\n]*/g, "").replace(/\s+/g, " ").trim();
  body = body.replace(/^return /, "").replace(/;$/, "");
  // IsInRange → in_range(kind, A, B)
  const inRange = /base::IsInRange\(\s*kind,\s*FunctionKind::(\w+),\s*FunctionKind::(\w+)\s*\)/g;
  if (inRange.test(body)) body = body.replace(/base::IsInRange\(\s*kind,\s*FunctionKind::(\w+),\s*FunctionKind::(\w+)\s*\)/g, (_, a, b) => `in_range(kind, Self::${pascal(a)}, Self::${pascal(b)})`);
  // 残留 IsInRange 未匹配 → 结构变更
  if (/IsInRange/.test(body)) throw new Error(`${name} 含未归一化的 IsInRange`);
  // 其余谓词调用 IsX(kind) → kind.is_xxx()
  body = body.replace(/\b(Is\w+|BindsSuper)\(kind\)/g, (m, fn) => (fn === name ? m : `kind.${snake(fn)}()`));
  // FunctionKind::X → Self::X (去 k 前缀, 与变体名一致)
  body = body.replace(/FunctionKind::(\w+)/g, (_, v) => `Self::${pascal(v)}`);
  predicates.push({ name, rust: snake(name), kind: body.includes("in_range(") ? "range" : body.includes("switch") ? "switch" : "expr", body });
}
if (predicates.length !== 22) throw new Error(`谓词数不符: 期望 22, 得 ${predicates.length}`);

// switch 形谓词单独解析 case 集 (IsStatic)
const staticMatch = hSrc.match(/inline bool IsStatic\(FunctionKind kind\) \{([\s\S]*?)\n\}/);
if (!staticMatch) throw new Error("未找到 IsStatic switch 形谓词");
const staticCases = [...staticMatch[1].matchAll(/case FunctionKind::(k\w+):/g)].map((m) => m[1]);
if (staticCases.length !== 8) throw new Error(`IsStatic case 数不符: 期望 8, 得 ${staticCases.length}`);

// 4. FunctionKind2String 字符串表
const strBody = hSrc.match(/inline const char\* FunctionKind2String\(FunctionKind kind\) \{([\s\S]*?)\n  \}/);
if (!strBody) throw new Error("未找到 FunctionKind2String");
const strMap = new Map();
for (const m of strBody[1].matchAll(/case FunctionKind::(k\w+):\s*return "([^"]*)";/g)) strMap.set(m[1], m[2]);
if (strMap.size !== 29) throw new Error(`FunctionKind2String 条数不符: 期望 29, 得 ${strMap.size}`);
for (const v of variants) if (!strMap.has(v.raw)) throw new Error(`2String 缺 ${v.raw}`);

console.log(`FunctionKind ${variants.length}+1 变体, 谓词 ${predicates.length}, 字符串表 ${strMap.size}`);

// 5. 生成 Rust
let rust = banner("src/objects/function-kind.h.js", "src/objects/function-kind.h", "FunctionKind 位域枚举与谓词族");

rust += `
/// 函数种类 (原版 enum class FunctionKind : uint8_t, 分组注释保留原版结构)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FunctionKind {
`;

// 分组注释按原版顺序回填
const variantOrder = [...enumMatch[1].matchAll(/^(k\w+),$/gm)].map((m) => m[1]);
const commentByVar = new Map();
{
  let pending = [];
  for (const line of enumMatch[1].split("\n")) {
    const t = line.trim();
    if (t.startsWith("//")) { pending.push(t); continue; }
    const cm = t.match(/^(k\w+),$/);
    if (cm) { if (!cm[1].startsWith("kLast")) commentByVar.set(cm[1], pending.splice(0)); }
  }
}

for (const raw of variantOrder) {
  if (raw === "kLastFunctionKind" || raw === "kInvalid") continue;
  const cs = commentByVar.get(raw) ?? [];
  for (const c of cs) rust += `  ${c}\n`;
  rust += `  ${pascal(raw)} = ${variantOrder.indexOf(raw)},\n`;
}
for (const c of (commentByVar.get("kInvalid") ?? [])) rust += `  ${c}\n`;
rust += `  /// kInvalid 哨兵\n  Invalid = ${variantOrder.indexOf("kInvalid")},\n`;
rust += `}

impl FunctionKind {
  /// 位宽 (原版 kFunctionKindBitSize)
  pub const BIT_SIZE: u32 = ${bitSize};
  /// 原版 kLastFunctionKind 锚点
  pub const LAST: Self = Self::${pascal("kClassStaticInitializerFunctionPrecededByMember")};

  /// 区间包含 (原版 base::IsInRange, const 可内联)
  #[inline]
  #[must_use]
  pub const fn in_range(kind: Self, lo: Self, hi: Self) -> bool {
    kind as u8 >= lo as u8 && kind as u8 <= hi as u8
  }
`;

for (const pr of predicates) {
  if (pr.name === "IsStatic") {
    rust += `  /// 原版 IsStatic (switch 展开为 matches!)\n  #[inline]\n  #[must_use]\n  pub const fn ${pr.rust}(self) -> bool {\n    matches!(self, ${staticCases.map((c) => `Self::${pascal(c)}`).join(" | ")})\n  }\n\n`;
    continue;
  }
  rust += `  /// 原版 ${pr.name}\n  #[inline]\n  #[must_use]\n  pub const fn ${pr.rust}(self) -> bool {\n    ${pr.body}\n  }\n\n`;
}

rust += `  /// 原版 FunctionKind2String
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
`;
for (const raw of variantOrder) {
  if (raw === "kLastFunctionKind") continue;
  rust += `      Self::${pascal(raw)} => "${strMap.get(raw)}",\n`;
}
rust += `    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FunctionKind::LAST as u8 < (1 << FunctionKind::BIT_SIZE));
  assert!(FunctionKind::in_range(FunctionKind::ArrowFunction, FunctionKind::ArrowFunction, FunctionKind::AsyncArrowFunction));
  assert!(FunctionKind::NormalFunction.is_constructable());
  assert!(!FunctionKind::ArrowFunction.is_constructable());
  assert!(FunctionKind::StaticConciseMethod.is_static());
  assert!(!FunctionKind::ConciseMethod.is_static());
};
`;

const p = writeSrc("v8-objects-js-func", "function_kind.rs", rust);
console.log(`written ${p}`);
