#!/usr/bin/env bun
// tool/src/objects/heap-object-set-map-inl.h.js — 覆盖 v8/src/objects/heap-object-set-map-inl.h
// HeapObject map 写入族 (写屏障/内存序/验证模式矩阵) + AllocationWitness 骨架
// → v8-objects-core/src/heap_object_set_map_inl.rs
// 对数: clang -E 探针, 文件切片内限定名定义逐名计数 (重载全数)
import { readV8, writeSrc, banner } from "../../lib.js";
import { spawnSync } from "node:child_process";
import { mkdtempSync, writeFileSync, readFileSync, rmSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const src = readV8("src/objects/heap-object-set-map-inl.h");

// —— clang -E 探针 (行标记切片, 只取本文件预处理产物) ————————————————
function clangProbe() {
  const dir = mkdtempSync(join(tmpdir(), "v8rs-hosm-"));
  const file = join(dir, "probe.cc");
  const outFile = join(dir, "probe.i");
  writeFileSync(file, `#include "src/objects/heap-object-set-map-inl.h"\n`);
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
    const want = /heap-object-set-map-inl\.h"/;
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

// 编译器产物侧: 本文件全部限定名定义 (含重载计数)
const clangDefs = {};
for (const m of pre.matchAll(/\b(HeapObject|AllocationWitness|PrimitiveHeapObject)::(\w+)\s*\(/g)) {
  const k = `${m[1]}::${m[2]}`;
  clangDefs[k] = (clangDefs[k] ?? 0) + 1;
}
// 期望清单 (-inl.h 逐段读出; 名字 → 重载数)
const EXPECT = {
  "HeapObject::set_map": 3, // (Isolate*,Map) / (IsolateT*,Map,ReleaseStoreTag) / 核心模板 (order+mode)
  "HeapObject::set_map_safe_transition": 2,
  "HeapObject::set_map_safe_transition_no_write_barrier": 2,
  "HeapObject::set_map_no_write_barrier": 2,
  "HeapObject::HeapObject": 2, // (ReadOnly<Map>) / (AllocationWitness, Map)
  "HeapObject::set_map_after_allocation": 1,
  "HeapObject::SetFillerMap": 1,
  "HeapObject::map_slot": 1,
  "HeapObject::set_map_word": 2, // Relaxed / Release
  "HeapObject::set_map_word_forwarded": 2,
  "HeapObject::relaxed_compare_and_swap_map_word_forwarded": 1,
  "AllocationWitness::AllocationWitness": 2,
  "AllocationWitness::WriteBarrierModeForAllocation": 1,
  "PrimitiveHeapObject::PrimitiveHeapObject": 1,
};
const extra = Object.keys(clangDefs).filter((k) => !(k in EXPECT));
if (extra.length > 0) throw new Error(`clang 侧未知定义: ${extra.join(",")}`);
const clangTotal = Object.values(clangDefs).reduce((a, b) => a + b, 0);
const expectTotal = Object.values(EXPECT).reduce((a, b) => a + b, 0);
for (const [name, n] of Object.entries(EXPECT)) {
  const got = clangDefs[name] ?? 0;
  if (got !== n) throw new Error(`clang 对数失配: ${name} 期望 ${n} 实得 ${got}`);
}
if (clangTotal !== expectTotal) throw new Error(`总定义数失配: ${clangTotal} != ${expectTotal}`);
// 分支证据: 核心 set_map 模板体 (DCHECK 在无 DEBUG 下展开为 (void)0, 取非断言证据)
if (!pre.includes("NotifyObjectLayoutChangeDone") || !pre.includes("WriteBarrier::ForValue")) {
  throw new Error("核心 set_map 模板体证据缺失");
}
// 源文本锚
for (const s of ["EmitWriteBarrier", "VerificationMode", "WriteBarrierModeForAllocation", "MapField::Relaxed_CompareAndSwap"]) {
  if (!src.includes(s)) throw new Error(`heap-object-set-map-inl.h 锚点缺失: ${s}`);
}

const body = `${banner("src/objects/heap-object-set-map-inl.h.js", "src/objects/heap-object-set-map-inl.h", `map 写入族骨架 ${expectTotal} 定义; 写屏障/GC 通知语义归 v8-heap-gc`)}
//! HeapObject 的 map 字段写入族 + AllocationWitness 分配见证。
//!
//! 写入矩阵: {默认 | safe_transition} × {带屏障 | 无屏障} ×
//! {Relaxed | Release} × {潜在布局变化 | 安全 map 迁移}; 核心模板
//! \`set_map<EmitWriteBarrier, MemoryOrder, IsolateT>\` 统一收口。
//! 写屏障 (WriteBarrier::ForValue) 与堆验证 (HeapVerifier) 归 v8-heap-gc。

use crate::heap_object::HeapObject;
use crate::map::Map;
use crate::tagged::Tagged;

/// map 写入的验证模式 (heap-object.h enum class VerificationMode)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationMode {
  /// 安全 map 迁移: 对象布局不变, 并发标记器可见。
  SafeMapTransition,
  /// 潜在布局变化: 需要完整验证。
  PotentialLayoutChange,
}

/// 是否发射写屏障 (heap-object.h enum class EmitWriteBarrier)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmitWriteBarrier {
  Yes,
  No,
}

/// 内存序标签 (原版 RelaxedStoreTag/ReleaseStoreTag 的骨架映射)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreOrder {
  Relaxed,
  Release,
}

/// 分配见证: 携带只读根 + 该分配适用的写屏障模式。
#[derive(Debug, Clone, Copy)]
pub struct AllocationWitness {
  /// 分配对象地址。
  pub object: usize,
  /// 写屏障模式 (SKIP/UPDATE)。
  pub write_barrier_mode: u8,
}

impl AllocationWitness {
  /// 按分配类型推导写屏障模式 (只读/年轻代分配跳过屏障)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h AllocationWitness::WriteBarrierModeForAllocation
  pub fn write_barrier_mode_for_allocation(_object: usize, _allocation: u8) -> u8 {
    todo!()
  }
}

impl HeapObject {
  /// 写 map 字段 (默认: 带屏障 + relaxed + 潜在布局变化)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map
  pub fn set_map_release(&mut self, _value: Tagged<Map>, _order: StoreOrder) {
    todo!()
  }

  /// 写 map 字段 (安全 map 迁移: 布局不变, 验证降级)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_safe_transition
  pub fn set_map_safe_transition(&mut self, _value: Tagged<Map>, _order: StoreOrder) {
    todo!()
  }

  /// 写 map 字段, 无写屏障 (白对象/不可变值场景)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_no_write_barrier
  pub fn set_map_no_write_barrier(&mut self, _value: Tagged<Map>, _order: StoreOrder) {
    todo!()
  }

  /// 写 map 字段 (安全迁移 + 无屏障)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_safe_transition_no_write_barrier
  pub fn set_map_safe_transition_no_write_barrier(&mut self, _value: Tagged<Map>, _order: StoreOrder) {
    todo!()
  }

  /// 分配后立即初始化 map (Heap 专用, 屏障模式显式)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_after_allocation
  pub fn set_map_after_allocation(&mut self, _value: Tagged<Map>, _emit: EmitWriteBarrier) {
    todo!()
  }

  /// 空闲空间填充 map 写入 (WritableFreeSpace 定向写)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::SetFillerMap
  pub fn set_filler_map(_writable_page: usize, _value: Tagged<Map>) {
    todo!()
  }

  /// map 槽地址 (MapField 偏移 0)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::map_slot
  pub fn map_slot(&self) -> usize {
    todo!()
  }

  /// 直写 map word (非 map 语义: GC 期 map word 复用)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_word
  pub fn set_map_word(&mut self, _map: Tagged<Map>, _order: StoreOrder) {
    todo!()
  }

  /// 直写转发地址 (GC 拷贝期前向指针)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_word_forwarded
  pub fn set_map_word_forwarded(&mut self, _target: usize, _order: StoreOrder) {
    todo!()
  }

  /// CAS 转发 map word (relaxed, 返回是否换成功)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::relaxed_compare_and_swap_map_word_forwarded
  pub fn relaxed_compare_and_swap_map_word_forwarded(&mut self, _old_map_word: u64, _new_target: usize) -> bool {
    todo!()
  }
}

/// 构造族清单 (map 写入矩阵之外的本文件构造定义)。
pub const CTOR_FAMILY: [&str; 5] = [
  "HeapObject::HeapObject(Tagged<ReadOnly<Map>>)", // 断言 map 非空, SKIP 屏障落 map
  "HeapObject::HeapObject(const AllocationWitness&, Tagged<Map>)", // 按 witness 屏障模式落 map
  "AllocationWitness::AllocationWitness(object, roots, allocation)", // 推导屏障模式
  "AllocationWitness::AllocationWitness(object, roots, write_barrier_mode)", // 显式屏障模式
  "PrimitiveHeapObject::PrimitiveHeapObject(Tagged<ReadOnly<Map>>)", // 转发基类构造
];

const _: () = {
  assert!(CTOR_FAMILY.len() == 5);
};
`;

const p = writeSrc("v8-objects-core", "heap_object_set_map_inl.rs", body);
console.log(`heap_object_set_map_inl.rs: clang 限定定义 ${clangTotal} (期望 ${expectTotal}), 逐名对数通过 → ${p}`);
