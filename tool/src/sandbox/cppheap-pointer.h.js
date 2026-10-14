#!/usr/bin/env bun
// tool/src/sandbox/cppheap-pointer.h.js
// 机械提取 v8/src/sandbox/cppheap-pointer.{h,-inl.h} — CppHeap 指针句柄面。
// Tag 枚举本体在 indirect-pointer-tag.h (已覆盖面), 此处登记句柄/别名/签名骨架。
// 产物: v8-sandbox/src/cppheap_pointer.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/sandbox/cppheap-pointer.h";
const INL_REL = "src/sandbox/cppheap-pointer-inl.h";
const h = readV8(H_REL);
const inl = readV8(INL_REL);

const sigs = [...h.matchAll(/^V8_INLINE\s+(?:[\w:<>&*]+\s+)*?(\w+)\(/gm)].map((m) => m[1]);
if (sigs.length < 3) throw new Error(`cppheap-pointer.h 函数面不足: ${sigs.length}`);
const inlSigs = [...inl.matchAll(/^V8_INLINE\s+(?:[\w:<>&*]+\s+)*?(\w+)\(/gm)].map((m) => m[1]);
if (!h.includes("CppHeapPointerHandle") && !h.includes("CppHeapPointer_t")) {
  throw new Error("缺 CppHeapPointerHandle/CppHeapPointer_t 锚");
}

let rust = banner("src/sandbox/cppheap-pointer.h.js", "src/sandbox/cppheap-pointer.{h,-inl.h}", "CppHeap 指针句柄骨架");
rust += `
// Tag 枚举 (CppHeapPointerTag) 与 tagging 常量在 src/sandbox/indirect-pointer-tag.h,
// 由既有 tags/tables 产物覆盖, 此处不重复登记。

/// 原 CppHeapPointerHandle (外部队表索引)
pub type CppHeapPointerHandle = u32;

/// 原 CppHeapPointer_t — 沙箱关闭时退化为裸指针, 开启时为句柄
pub type CppHeapPointerT = usize;

// 原版头文件函数面 (${sigs.length} 条):
${sigs.map((s) => `// - ${s}`).join("\n")}
// 原版 -inl.h 函数面 (${inlSigs.length} 条):
${inlSigs.map((s) => `// - ${s}`).join("\n")}

// TODO(dev): 依函数面补读写路径实现 (Tag 校验/表访问)。

// —— 提取不变式断言 ——
const _: () = {
  let _h: CppHeapPointerHandle = 0;
  let _p: CppHeapPointerT = 0;
};
`;

const p = writeSrc("v8-sandbox", "cppheap_pointer.rs", rust);
console.log(`written ${p} (h ${sigs.length}, inl ${inlSigs.length})`);
