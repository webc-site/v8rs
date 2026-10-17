#!/usr/bin/env bun
// tool/src/wasm/well-known-imports.h.js
// 机械提取 v8/src/wasm/well-known-imports.h 的 WellKnownImport 枚举 (81 项)
// 与 WellKnownImportsList::UpdateResult 常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/well-known-imports.h");

// 1. 提取 enum class WellKnownImport : uint8_t
const enumMatch = hSrc.match(/enum class WellKnownImport : uint8_t \{([\s\S]*?)\n\};/);
if (!enumMatch) throw new Error("未找到 WellKnownImport 枚举");

// 逐行解析: 名称 [= 别名] ; 忽略注释与空行
const aliases = new Map(); // name -> 引用的别名名
const entries = [];
for (const rawLine of enumMatch[1].split("\n")) {
  const line = rawLine.replace(/\/\/.*$/, "").trim();
  if (!line) continue;
  const m = line.match(/^(k[A-Za-z0-9]+)(?:\s*=\s*(k[A-Za-z0-9]+))?[,]?$/);
  if (!m) throw new Error(`无法解析枚举行: "${rawLine}"`);
  entries.push({ name: m[1], alias: m[2] ?? null });
}

const EXPECTED = 81;
if (entries.length !== EXPECTED) {
  throw new Error(`WellKnownImport 条数不符: 提取 ${entries.length}, 期望 ${EXPECTED}`);
}

// 解析显式值: 顺序编号, 遇 "= 别名" 跳到别名值
const valueOf = new Map();
let next = 0;
for (const e of entries) {
  if (e.alias !== null) {
    if (!valueOf.has(e.alias)) {
      throw new Error(`别名 ${e.name} = ${e.alias} 先于其定义出现`);
    }
    next = valueOf.get(e.alias);
  }
  valueOf.set(e.name, next);
  e.value = next;
  next += 1;
}

// 锚点常量 (原版侧语义: kStringCast = kFirstCompileTimeImport,
// kLastCompileTimeImport = kConfigureAllPrototypes)
const firstCompile = valueOf.get("kFirstCompileTimeImport");
const lastCompile = valueOf.get("kLastCompileTimeImport");
const stringCast = valueOf.get("kStringCast");
const configureAll = valueOf.get("kConfigureAllPrototypes");
if (stringCast !== firstCompile || configureAll !== lastCompile) {
  throw new Error("Compile-time import 区间锚点不符");
}
const uninstantiated = valueOf.get("kUninstantiated");
if (uninstantiated !== 0) {
  throw new Error("kUninstantiated 应为 0 (原版 static_assert)");
}

// 变体命名: 去 k 前缀的 PascalCase (对齐 crate 既有约定)
function toPascal(name) {
  return name.replace(/^k/, "");
}
// 模块级常量命名: SCREAMING_SNAKE_CASE
function toScreamingSnake(name) {
  return name
    .replace(/^k/, "")
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .toUpperCase();
}

console.log(`成功提取 ${entries.length} 个 WellKnownImport (firstCompile=${firstCompile}, lastCompile=${lastCompile})`);

// 2. 生成 Rust
let rust = banner(
  "src/wasm/well-known-imports.h.js",
  "src/wasm/well-known-imports.h",
  "WellKnownImport 枚举与 WellKnownImportsList 状态常量",
);

rust += `
/// 已知的 well-known 导入函数类别 (对应原版 enum class WellKnownImport : uint8_t, 共 ${entries.length} 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum WellKnownImport {
`;

for (const e of entries) {
  const note = e.alias !== null ? ` (= ${e.alias})` : "";
  rust += `  /// ${e.name}${note}\n  ${toPascal(e.name)} = ${e.value},\n`;
}

rust += `}

impl WellKnownImport {
  /// 枚举总数
  pub const COUNT: usize = ${entries.length};

  /// 首个编译期内建导入 (原版 kFirstCompileTimeImport)
  pub const FIRST_COMPILE_TIME_IMPORT: Self = Self::${toPascal(entries.find((e) => e.name === "kFirstCompileTimeImport").name)};

  /// 末个编译期内建导入 (原版 kLastCompileTimeImport)
  pub const LAST_COMPILE_TIME_IMPORT: Self = Self::${toPascal(entries.find((e) => e.name === "kLastCompileTimeImport").name)};

  /// 是否为编译期内建导入 (原版 IsCompileTimeImport)
  #[inline]
  #[must_use]
  pub const fn is_compile_time_import(self) -> bool {
    (self as u8) >= Self::FIRST_COMPILE_TIME_IMPORT as u8
      && (self as u8) <= Self::LAST_COMPILE_TIME_IMPORT as u8
  }

  /// 从数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    match val {
`;

for (const e of entries) {
  rust += `      ${e.value} => Some(Self::${toPascal(e.name)}),\n`;
}

rust += `      _ => None,
    }
  }
}

/// WellKnownImportsList::UpdateResult — 更新结果 (原版 base::StrongAlias<UpdateResultTag, bool>)
pub type UpdateResult = bool;

/// 更新发现不兼容项 (原版 kFoundIncompatibility)
pub const FOUND_INCOMPATIBILITY: UpdateResult = false;

/// 更新成功 (原版 kOK)
pub const UPDATE_OK: UpdateResult = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(WellKnownImport::COUNT == ${entries.length});
  assert!(WellKnownImport::Uninstantiated as u8 == 0);
  assert!(WellKnownImport::StringCast as u8 == WellKnownImport::FIRST_COMPILE_TIME_IMPORT as u8);
  assert!(WellKnownImport::ConfigureAllPrototypes as u8 == WellKnownImport::LAST_COMPILE_TIME_IMPORT as u8);
  assert!(!WellKnownImport::Uninstantiated.is_compile_time_import());
  assert!(WellKnownImport::StringCast.is_compile_time_import());
  assert!(WellKnownImport::ConfigureAllPrototypes.is_compile_time_import());
  assert!(WellKnownImport::FastApiCall.is_compile_time_import() == false);
};
`;

const p = writeSrc("v8-wasm-core", "well_known_imports.rs", rust);
console.log(`written ${p}`);
