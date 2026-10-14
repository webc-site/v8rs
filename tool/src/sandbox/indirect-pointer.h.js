#!/usr/bin/env bun
// tool/src/sandbox/indirect-pointer.h.js
// 机械提取 v8/src/sandbox/indirect-pointer.{h,-inl.h} — 间接指针读写函数面。
// Tag 枚举与表常量在 indirect-pointer-tag.h (既有覆盖面)。
// 产物: v8-sandbox/src/indirect_pointer.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/indirect-pointer.h";
const INL_REL = "src/sandbox/indirect-pointer-inl.h";
const h = readV8(H_REL);
const inl = readV8(INL_REL);

// 函数面: 顶层声明 (V8_INLINE/void/Address/Tagged 开头的原型)
const collect = (src) =>
  [...src.matchAll(/^(?:V8_INLINE\s+)?(?:Address|void|Tagged<[^>]+>|IndirectPointerHandle|TrustedObject)\s+(\w+)\(/gm)].map((m) => m[1]);
const hFns = collect(h);
const inlFns = collect(inl);
if (hFns.length < 3) throw new Error(`indirect-pointer.h 函数面不足: ${hFns.length}`);
if (inlFns.length < 4) throw new Error(`indirect-pointer-inl.h 函数面不足: ${inlFns.length}`);
if (!h.includes("IndirectPointerTag tag")) throw new Error("缺 IndirectPointerTag 参数锚");
if (!inl.includes("kNullIndirectPointerHandle")) throw new Error("inl 缺空句柄锚");

let rust = banner("src/sandbox/indirect-pointer.h.js", "src/sandbox/indirect-pointer.{h,-inl.h}", "间接指针函数面骨架");
rust += `
// IndirectPointerTag/句柄常量在 src/sandbox/indirect-pointer-tag.h (既有覆盖面)。

/// 原 IndirectPointerHandle (uint32_t 表索引)
pub type IndirectPointerHandle = u32;

// 原版头文件函数面 (${hFns.length} 条):
${[...new Set(hFns)].map((s) => `// - ${s}`).join("\n")}
// 原版 -inl.h 函数面 (${inlFns.length} 条):
${[...new Set(inlFns)].map((s) => `// - ${s}`).join("\n")}

// TODO(dev): 补 ReadIndirectPointerField/WriteIndirectPointerField 沙箱开闭双路径。

// —— 提取不变式断言 ——
const _: () = {
  let _h: IndirectPointerHandle = 0;
};
`;

const p = writeSrc("v8-sandbox", "indirect_pointer.rs", rust);
console.log(`written ${p} (h ${hFns.length}, inl ${inlFns.length})`);
