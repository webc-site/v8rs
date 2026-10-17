#!/usr/bin/env bun
// tool/src/objects/heap-object-field-inl.h.js — 覆盖 v8/src/objects/heap-object-field-inl.h
// HeapObject 沙箱/外指/CppHeap/间接指针字段访问器 + Raw*Field 槽位构造骨架
// → v8-objects-core/src/heap_object_field_inl.rs
// 对数: clang -E 探针 (SANDBOX+EXTERNAL_CODE_SPACE 开), HeapObject:: 限定定义逐名计数
import { readV8, writeSrc, banner } from "../../lib.js";
import { spawnSync } from "node:child_process";
import { mkdtempSync, writeFileSync, readFileSync, rmSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const src = readV8("src/objects/heap-object-field-inl.h");

// —— clang -E 探针 ————————————————————————————————————————————————
// 不加 -P: 保留行标记, 按 "文件来源切片" 只取 heap-object-field-inl.h 自身的
// 预处理产物 (传递包含的 heap-object-inl.h 等其余 HeapObject:: 定义不混入)
function clangProbe() {
  const dir = mkdtempSync(join(tmpdir(), "v8rs-hofield-"));
  const file = join(dir, "probe.cc");
  const outFile = join(dir, "probe.i");
  writeFileSync(file, `#include "src/objects/heap-object-field-inl.h"\n`);
  const stubs = [
    ["third_party/googletest/src/googletest/include/gtest", "gtest_prod.h", "#define FRIEND_TEST(a, b)\n"],
    ["third_party/fp16/src/include", "fp16.h", "#pragma once\n"],
    [".", "simdutf.h", "#pragma once\n"],
  ];
  for (const [d, n, c] of stubs) {
    mkdirSync(join(dir, d), { recursive: true });
    writeFileSync(join(dir, d, n), c);
  }
  const v8root = join(import.meta.dir, "../../../v8");
  const res = spawnSync(
    "clang",
    ["-E", "-std=c++20", "-DV8_METAGEN_GENERATION_PASS", "-DV8_COMPRESS_POINTERS",
      "-DV8_INTL_SUPPORT", "-DV8_ENABLE_WEBASSEMBLY", "-DV8_ENABLE_SANDBOX", "-DV8_EXTERNAL_CODE_SPACE",
      "-I", dir, "-I", join(v8root, "include"), "-I", v8root, "-I", "/opt/homebrew/include",
      "-o", outFile, file],
    { encoding: "utf8" },
  );
  let out = null;
  if (!res.error && res.status === 0) {
    const whole = readFileSync(outFile, "utf8");
    // 行标记形如: # 24 "v8/src/objects/heap-object-field-inl.h" 2
    const want = /heap-object-field-inl\.h"/;
    const other = /^# \d+ "[^"]*"/;
    let inSlice = false;
    const parts = [];
    for (const line of whole.split("\n")) {
      if (other.test(line)) {
        inSlice = want.test(line);
        continue;
      }
      if (inSlice) parts.push(line);
    }
    out = parts.join("\n");
  }
  try { rmSync(dir, { recursive: true, force: true }); } catch { /* tmp 清理失败可忽略 */ }
  if (out === null || out.length === 0) throw new Error("clang 探针失败或文件切片为空");
  return out;
}

const pre = clangProbe();
if (pre === null) throw new Error("clang 探针失败, 无对数基准");

// 编译器产物侧: 提取全部 HeapObject:: 限定名定义 (含重载, 逐名计数)
const clangDefs = {};
for (const m of pre.matchAll(/\bHeapObject::(\w+)\s*\(/g)) {
  clangDefs[m[1]] = (clangDefs[m[1]] ?? 0) + 1;
}
// 期望清单 (从 -inl.h 逐段读出; 名字 → 重载数)
const EXPECT = {
  ReadSandboxedPointerField: 1,
  WriteSandboxedPointerField: 2, // (PtrComprCageBase) / (Isolate*)
  ReadBoundedSizeField: 1,
  WriteBoundedSizeField: 1,
  InitExternalPointerField: 2, // <tag> 模板 / 运行期 tag
  ReadExternalPointerField: 2, // <tag_range> 模板 / 运行期 tag_range
  ReadCppHeapPointerField: 2, // <lower,upper> 模板 / 运行期 tag_range
  WriteExternalPointerField: 2,
  SetupLazilyInitializedExternalPointerField: 1,
  IsLazilyInitializedExternalPointerFieldInitialized: 1,
  WriteLazilyInitializedExternalPointerField: 2,
  SetupLazilyInitializedCppHeapPointerField: 1,
  WriteLazilyInitializedCppHeapPointerField: 1,
  RawField: 1,
  RawMaybeWeakField: 1,
  RawInstructionStreamField: 1,
  RawExternalPointerField: 1,
  RawCppHeapPointerField: 1,
  RawIndirectPointerField: 1,
};
const clangTotal = Object.values(clangDefs).reduce((a, b) => a + b, 0);
const expectTotal = Object.values(EXPECT).reduce((a, b) => a + b, 0);
// 逐名比对 (clang 侧多余项 = 声明文本混入, 需为已知集合)
const extra = Object.keys(clangDefs).filter((k) => !(k in EXPECT));
if (extra.length > 0) throw new Error(`clang 侧出现未知 HeapObject:: 定义: ${extra.join(",")}`);
for (const [name, n] of Object.entries(EXPECT)) {
  const got = clangDefs[name] ?? 0;
  if (got !== n) throw new Error(`clang 对数失配: ${name} 期望 ${n} 实得 ${got}`);
}
if (clangTotal !== expectTotal) throw new Error(`总定义数失配: ${clangTotal} != ${expectTotal}`);
// 沙箱分支证据 (配置 SANDBOX 开)
if (!pre.includes("ExternalPointerHandle")) throw new Error("沙箱分支证据缺失: ExternalPointerHandle");
// 源文本锚
for (const s of ["ReadSandboxedPointerField", "WriteLazilyInitializedExternalPointerField", "RawIndirectPointerField"]) {
  if (!src.includes(s)) throw new Error(`heap-object-field-inl.h 锚点缺失: ${s}`);
}

// Rust 侧签名 (骨架; Address/cage_base → usize, 隔离沙箱句柄依赖)
const body = `${banner("src/objects/heap-object-field-inl.h.js", "src/objects/heap-object-field-inl.h", `HeapObject 字段访问器骨架 ${expectTotal} 项; 沙箱表解析语义归 v8-sandbox`)}
//! HeapObject 的字段级访问器: 沙箱指针 / BoundedSize / 外部指针 /
//! CppHeap 指针 / 间接 (受信) 指针 + Raw*Field 槽位构造。
//!
//! 目标配置 V8_ENABLE_SANDBOX 开: 外指/CppHeap/间接指针字段在沙箱下
//! 存句柄 (ExternalPointerHandle 等), 解析走 ExternalPointerTable —
//! 表依赖归 v8-sandbox, 此处只落签名骨架。

use crate::heap_object::HeapObject;

impl HeapObject {
  /// 读沙箱指针字段 (offset 寻址 + cage base 解码)。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::ReadSandboxedPointerField
  pub fn read_sandboxed_pointer_field(&self, _offset: usize, _cage_base: usize) -> usize {
    todo!()
  }

  /// 写沙箱指针字段 (cage base 编码后落槽)。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::WriteSandboxedPointerField
  pub fn write_sandboxed_pointer_field(&self, _offset: usize, _cage_base: usize, _value: usize) {
    todo!()
  }

  /// 读 BoundedSize 字段 (40 位有界尺寸, 沙箱内安全)。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::ReadBoundedSizeField
  pub fn read_bounded_size_field(&self, _offset: usize) -> usize {
    todo!()
  }

  /// 写 BoundedSize 字段。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::WriteBoundedSizeField
  pub fn write_bounded_size_field(&self, _offset: usize, _value: usize) {
    todo!()
  }

  /// 取强指针槽 (RawField) 的槽地址。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawField
  pub fn raw_field(&self, _byte_offset: i32) -> usize {
    todo!()
  }

  /// 取可能弱指针槽 (RawMaybeWeakField) 的槽地址。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawMaybeWeakField
  pub fn raw_maybe_weak_field(&self, _byte_offset: i32) -> usize {
    todo!()
  }

  /// 取指令流槽 (RawInstructionStreamField) 的槽地址。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawInstructionStreamField
  pub fn raw_instruction_stream_field(&self, _byte_offset: i32) -> usize {
    todo!()
  }

  /// 取外指槽 (RawExternalPointerField) 的槽地址 (含 tag range)。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawExternalPointerField
  pub fn raw_external_pointer_field(&self, _byte_offset: i32, _tag_range: u32) -> usize {
    todo!()
  }

  /// 取 CppHeap 指针槽 (RawCppHeapPointerField) 的槽地址。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawCppHeapPointerField
  pub fn raw_cpp_heap_pointer_field(&self, _byte_offset: i32) -> usize {
    todo!()
  }

  /// 取间接 (受信) 指针槽 (RawIndirectPointerField) 的槽地址。
  ///
  /// v8: src/objects/heap-object-field-inl.h HeapObject::RawIndirectPointerField
  pub fn raw_indirect_pointer_field(&self, _byte_offset: i32, _tag_range: u32) -> usize {
    todo!()
  }
}

/// 外指/CppHeap/间接指针字段的初始化-读写-懒初始化族 (模板形参 tag 化,
/// 运行期 tag 变体 2 套; 沙箱句柄表语义归 v8-sandbox)。
pub const FIELD_ACCESSOR_FAMILY: [&str; 13] = [
  "InitExternalPointerField<tag> | InitExternalPointerField(tag)",
  "ReadExternalPointerField<tag_range> | ReadExternalPointerField(tag_range)",
  "ReadCppHeapPointerField<lower,upper> | ReadCppHeapPointerField(tag_range)",
  "WriteExternalPointerField<tag> | WriteExternalPointerField(tag)",
  "SetupLazilyInitializedExternalPointerField",
  "IsLazilyInitializedExternalPointerFieldInitialized",
  "WriteLazilyInitializedExternalPointerField<tag> | WriteLazilyInitializedExternalPointerField(tag)",
  "SetupLazilyInitializedCppHeapPointerField",
  "WriteLazilyInitializedCppHeapPointerField",
  "WriteSandboxedPointerField (Isolate* 变体)",
  "ReadSandboxedPointerField",
  "ReadBoundedSizeField",
  "WriteBoundedSizeField",
];

const _: () = {
  assert!(FIELD_ACCESSOR_FAMILY.len() == 13);
};
`;

const p = writeSrc("v8-objects-core", "heap_object_field_inl.rs", body);
console.log(`heap_object_field_inl.rs: clang 限定定义 ${clangTotal} (期望 ${expectTotal}), 逐名对数通过 → ${p}`);
