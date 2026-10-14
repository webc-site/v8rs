#!/usr/bin/env bun
// tool/src/roots/static-roots.h.js
// 机械提取 v8/src/roots/static-roots.h + 4 个变体 (intl/nointl × wasm/nowasm):
// StaticReadOnlyRoot 常量表与 StaticReadOnlyRootsPointerTable 数组 1:1 展开

import { readV8, writeSrc, banner } from "../../lib.js";

const VARIANTS = [
  { file: "src/roots/static-roots-intl-wasm.h", mod: "intl_wasm", constexpr: 1046, table: 1044 },
  { file: "src/roots/static-roots-intl-nowasm.h", mod: "intl_nowasm", constexpr: 1028, table: 1026 },
  { file: "src/roots/static-roots-nointl-wasm.h", mod: "nointl_wasm", constexpr: 904, table: 902 },
  { file: "src/roots/static-roots-nointl-nowasm.h", mod: "nointl_nowasm", constexpr: 886, table: 884 },
];

// 平台分支展开 (规则: x64/arm64 默认取 #ifdef 分支; #else 分支剔除并在此登记):
// static-roots-* 仅 V8_ENABLE_TDZ_HOLE 一处 #ifdef — 启用 TDZ hole, kDisabledTdzHoleValue 弃用
function stripElse(src) {
  const out = [];
  let skip = false;
  for (const line of src.split("\n")) {
    if (/^#ifdef/.test(line)) {
      out.push("// [分支展开] " + line.trim());
      continue;
    }
    if (/^#else/.test(line)) {
      skip = true;
      continue;
    }
    if (/^#endif/.test(line)) {
      skip = false;
      continue;
    }
    if (!skip) out.push(line);
  }
  return out.join("\n");
}

// kUndefinedValue → UNDEFINED_VALUE; kempty_string → EMPTY_STRING
function screaming(kName) {
  return kName.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
}

// 表达式里的 kXxx 引用 → SCREAMING 常量引用
function convExpr(expr) {
  return expr.replace(/\bk([A-Za-z_]\w*)\b/g, (_, n) => screaming("k" + n));
}

function parseVariant(file) {
  const src = stripElse(readV8(file));
  const consts = [];
  const re = /\bstatic constexpr Tagged_t\s+(k\w+)\s*=\s*([^;]+);/g;
  let m;
  while ((m = re.exec(src)) !== null) {
    consts.push({ raw: m[1], rust: screaming(m[1]), expr: convExpr(m[2].replace(/\s+/g, " ").trim()) });
  }
  const tm = src.match(
    /static constexpr std::array<Tagged_t,\s*(\d+)>\s*StaticReadOnlyRootsPointerTable\s*=\s*\{([^;]*)\};/,
  );
  if (!tm) throw new Error(`${file} 未找到 StaticReadOnlyRootsPointerTable`);
  const tableSize = parseInt(tm[1], 10);
  const table = [...tm[2].matchAll(/StaticReadOnlyRoot::(k\w+)/g)].map((x) => x[1]);
  return { consts, table, tableSize };
}

let rust = banner(
  "src/roots/static-roots.h.js",
  "src/roots/static-roots.h + static-roots-{intl,nointl}-{wasm,nowasm}.h",
  "静态只读根地址表 (tools/dev/gen-static-roots.py 产物, 此处再机械转写)",
);

rust += `
/// 静态根布局选择矩阵 (原版 static-roots.h 的 #if 展开):
///   V8_STATIC_ROOTS_BOOL 开启时, 按 (V8_INTL_SUPPORT, V8_ENABLE_WEBASSEMBLY) 选变体
///   — x64/arm64 指针压缩默认构建取 intl_wasm; 其余变体按下表模块登记
/// 注意: Tagged_t = u32 (指针压缩 32 位堆内偏移)

`;

for (const v of VARIANTS) {
  const { consts, table, tableSize } = parseVariant(v.file);
  if (consts.length !== v.constexpr) {
    throw new Error(`${v.file} constexpr 应 ${v.constexpr}, 实得 ${consts.length}`);
  }
  if (table.length !== tableSize) {
    throw new Error(`${v.file} 表项应 ${tableSize}, 实得 ${table.length}`);
  }
  if (tableSize !== v.table) throw new Error(`${v.file} 表容量应 ${v.table}, 实得 ${tableSize}`);
  // 首项对数: 各变体必须以 undefined/null 起头
  if (consts[0].raw !== "kUndefinedValue" || consts[1].raw !== "kNullValue") {
    throw new Error(`${v.file} 首两项漂移: ${consts[0].raw},${consts[1].raw}`);
  }

  rust += `/// 变体: INTL_SUPPORT 开 + WebAssembly 开 (${v.file})
pub mod ${v.mod} {
  /// StaticReadOnlyRoot 常量 (原版 struct StaticReadOnlyRoot, 共 ${consts.length} 项)
  #[rustfmt::skip]
  pub mod roots {
`;
  for (const c of consts) {
    rust += `    pub const ${c.rust}: u32 = ${c.expr};\n`;
  }
  rust += `  }

  /// 只读根指针表 (原版 StaticReadOnlyRootsPointerTable, 共 ${tableSize} 项)
  #[rustfmt::skip]
  pub static STATIC_READ_ONLY_ROOTS_POINTER_TABLE: [u32; ${tableSize}] = [
`;
  for (let i = 0; i < table.length; i += 4) {
    rust += `    ${table.slice(i, i + 4).map((n) => roots_mod(n, v.mod)).join(", ")},\n`;
  }
  rust += `  ];
}

`;
}

function roots_mod(kName, mod) {
  const ref = screaming(kName);
  return `roots::${ref}`;
}

rust += `/// 默认变体选择 (原版 #if 分支: INTL + WASM 为 x64/arm64 默认)
#[allow(unused_imports)]
pub use intl_wasm::{roots as default_roots, STATIC_READ_ONLY_ROOTS_POINTER_TABLE as DEFAULT_TABLE};
`;

const p = writeSrc("v8-roots", "static_roots_generated.rs", rust);
console.log(`written ${p}; 变体=${VARIANTS.length}, 最大表=${VARIANTS[0].table}`);
