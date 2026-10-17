#!/usr/bin/env bun
// tool/src/objects/fixed-primitive-array.tq.js — 覆盖 v8/src/objects/fixed-primitive-array.tq
// (差集: .h/-inl.h 归既有 fixed-primitive-array.h.js / fixed-primitive-array-inl.h.js 席)
// → v8-objects-arrays/src/primitive_array_tq.rs.skel
// 机械提取: tq 类布局 (FixedDoubleArray/ByteArray/TrustedByteArray) +
//   extern macro/operator 声明清单 + NewFixedDoubleArray 双配置布局。

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/fixed-primitive-array.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── 1. tq 类布局 ───────────────────────────────────────────────────────
const classes = [...tq.matchAll(/^extern class (\w+)(?:\s+extends\s+([\w\s]+?))?\s*\{([^}]*)\n\}/gm)]
  .map((m) => ({ name: m[1], ext: (m[2] ?? "").trim(), body: m[3] }));
must(classes.length === 3, `tq extern class 应 3, 实得 ${classes.length}: ${classes.map((c) => c.name)}`);

const EXPECT = [
  ["FixedDoubleArray", "FixedArrayBase", "values[length]: float64_or_undefined_or_hole"],
  ["ByteArray", "FixedArrayBase", "values[length]: uint8"],
  ["TrustedByteArray", "TrustedFixedArrayBase", "values[length]: uint8"],
];
for (let i = 0; i < 3; i++) {
  const c = classes[i];
  const fields = [...c.body.matchAll(/^\s{2}(\w+)\[(\w+)\]:\s*([^;]+);/gm)]
    .map((m) => ({ name: m[1], len: m[2], type: m[3].trim() }));
  must(c.name === EXPECT[i][0] && c.ext === EXPECT[i][1],
    `tq 类[${i}] ${c.name} extends ${c.ext} 期望 ${EXPECT[i][0]} extends ${EXPECT[i][1]}`);
  must(fields.length === 1 && fields[0].name === "values" && fields[0].len === "length" &&
    fields[0].type === EXPECT[i][2].replace(/^values\[length\]: /, ""),
    `${c.name} 字段失配: ${JSON.stringify(fields)}`);
}

// ── 2. extern 声明清单 ─────────────────────────────────────────────────
const externMacros = [...tq.matchAll(/^[ \t]*extern macro ([\w:]+)\(/gm)].map((m) => m[1]);
const externMacroDecls = externMacros.length; // 声明次数 (含重载)
const externMacroNames = [...new Set(externMacros)];
const operators = [...tq.matchAll(/^extern operator '([\w.\[\]=]+)' macro (\w+)/gm)]
  .map((m) => `${m[1]} → ${m[2]}`);
must(externMacroDecls === 6 && externMacroNames.length === 5,
  `extern macro 应 6 次声明 5 个名, 实得 ${externMacroDecls} 次: ${externMacros}`);
must(operators.length === 3, `extern operator 应 3, 实得 ${operators.length}: ${operators}`);
for (const need of ["CodeStubAssembler::AllocateByteArray", "StoreFixedDoubleArrayUndefined",
  "AllocateZeroedFixedDoubleArray", "FillFixedDoubleArrayWithZero", "AllocateFixedDoubleArrayWithHoles"])
  must(externMacroNames.includes(need), `缺 extern macro ${need}`);
must(operators.includes(".values[]= → StoreFixedDoubleArrayElement") &&
  operators.includes(".values[] → LoadFixedDoubleArrayElement"), "operator 映射锚失配");
// V8_ENABLE_UNDEFINED_DOUBLE 分支
must(tq.includes("@if(V8_ENABLE_UNDEFINED_DOUBLE)"), "V8_ENABLE_UNDEFINED_DOUBLE 分支锚缺失");
// StoreFixedDoubleArrayElement 重载 (intptr / Smi 双下标)
const storeCount = (tq.match(/extern operator '\.values\[\]=' macro StoreFixedDoubleArrayElement/g) ?? []).length;
must(storeCount === 2, `StoreFixedDoubleArrayElement 重载应 2 (intptr/Smi), 实得 ${storeCount}`);

// ── 3. tq 自有 macro / 双配置布局 ──────────────────────────────────────
const macros = [...tq.matchAll(/^(?:operator '[\w\[\]]+=' )?macro (\w+)\(/gm)].map((m) => m[1]);
must(JSON.stringify(macros) === JSON.stringify(["StoreFixedDoubleArrayDirect", "ExtractFixedDoubleArray"]),
  `tq macro 应 2 (StoreFixedDoubleArrayDirect/ExtractFixedDoubleArray), 实得 ${JSON.stringify(macros)}`);
must(tq.includes("@if(TAGGED_SIZE_8_BYTES)") && tq.includes("@ifnot(TAGGED_SIZE_8_BYTES)"),
  "NewFixedDoubleArray 双配置分支锚缺失");
must(tq.includes("if (length == 0) return kEmptyFixedArray;"), "空数组快捷路径锚缺失");
must(tq.includes("if (length > kFixedDoubleArrayMaxLength) deferred {"), "长度上限检查锚缺失");
// 8 字节配置变体含 optional_padding
const ifBlock = tq.slice(tq.indexOf("@if(TAGGED_SIZE_8_BYTES)"));
must(ifBlock.slice(0, 600).includes("optional_padding: 0,"), "8 字节变体 optional_padding 锚缺失");

// ── 4. 生成 ────────────────────────────────────────────────────────────
const body = `${banner("src/objects/fixed-primitive-array.tq.js", "src/objects/fixed-primitive-array.tq", "差集 (h/-inl.h 归既有两席): tq 布局 + extern 声明清单")}
//! 差集骨架: 定长原始数组 tq 布局。
//! 配置: 64 位非压缩 (kTaggedSize=8 → FixedArrayBase 头 = map(8)+length(4)+padding(4) = 16;
//!   TrustedFixedArrayBase 同构)。

/// tq 布局表 (1:1)。
pub const PRIMITIVE_ARRAY_TQ_LAYOUT: &[(&str, &str, &str, &str)] = &[
  ("FixedDoubleArray", "FixedArrayBase", "values[length]", "float64_or_undefined_or_hole"),
  ("ByteArray", "FixedArrayBase", "values[length]", "uint8"),
  ("TrustedByteArray", "TrustedFixedArrayBase", "values[length]", "uint8"),
];

/// extern operator 映射 (${operators.length} 条)。
pub const PRIMITIVE_ARRAY_TQ_OPERATORS: &[&str] = &[
${operators.map((o) => `  "${o}",`).join("\n")}
];

/// extern macro 声明 (${externMacroDecls} 次, 5 个名; AllocateFixedDoubleArrayWithHoles 带 AllocationFlag 重载)。
pub const PRIMITIVE_ARRAY_TQ_EXTERN_MACROS: &[&str] = &[
${externMacroNames.map((m) => `  "${m}",`).join("\n")}
];

/// v8: src/objects/fixed-primitive-array.tq NewFixedDoubleArray<Iterator> —
///   length==0 → kEmptyFixedArray; > kFixedDoubleArrayMaxLength → FatalProcessOutOfMemory;
///   TAGGED_SIZE_8_BYTES 变体多 optional_padding: 0 (本配置登记即 8 字节分支)。
/// v8: src/objects/fixed-primitive-array.tq ExtractFixedDoubleArray —
///   容量截取 + 尾部 kDoubleHole 填充 (IteratorSequence + ConstantIterator)。
`;

const p = writeSrc("v8-objects-arrays", "primitive_array_tq.rs.skel", body);
console.log(
  `fixed-primitive-array.tq.js: ${p} — tq 类 ${classes.length}、operator ${operators.length}、extern macro ${externMacros.length}、` +
    `自有 macro ${macros.length}、断言 ${checks} 项通过`,
);
