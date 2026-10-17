#!/usr/bin/env bun
// tool/src/objects/backing-store.h.js — 覆盖 v8/src/objects/backing-store.{h,cc}
// → v8-objects-rest/src/backing_store.rs
// 机械提取: WasmMemoryFlag / ResizeOrGrowResult / Flag 私有枚举 + kTypeID 锚
// 945 行 .cc 语义实现面登记跳过。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/backing-store.h");

// WasmMemoryFlag
const wasmFlag = hSrc.match(/enum class WasmMemoryFlag : uint8_t\s*\{([^}]+)\}/);
if (!wasmFlag) throw new Error("未找到 WasmMemoryFlag");
const wasmEntries = wasmFlag[1].split(",").map((s) => s.trim()).filter(Boolean);
if (wasmEntries.join(",") !== "kNotWasm,kWasmMemory32,kWasmMemory64") {
  throw new Error(`WasmMemoryFlag 条目失配: ${wasmEntries.join(",")}`);
}

// ManagedTypeId 锚
const typeId = hSrc.match(/static constexpr ManagedTypeId kTypeID = ManagedTypeId::(\w+);/);
if (!typeId || typeId[1] !== "kBackingStore") throw new Error("kTypeID 锚失配");

// ResizeOrGrowResult
const rog = hSrc.match(/enum ResizeOrGrowResult\s*\{([^}]+)\}/);
if (!rog) throw new Error("未找到 ResizeOrGrowResult");
const rogEntries = rog[1].split(",").map((s) => s.trim()).filter(Boolean);
if (rogEntries.join(",") !== "kSuccess,kFailure,kRace") {
  throw new Error(`ResizeOrGrowResult 条目失配: ${rogEntries.join(",")}`);
}

// 私有 Flag 枚举 (顺序位)
const flag = hSrc.match(/enum Flag\s*\{([\s\S]*?)\n\s*\};/);
if (!flag) throw new Error("未找到 BackingStore::Flag");
const flagEntries = flag[1].split(",").map((s) => s.trim()).filter(Boolean);
if (flagEntries.length !== 10) throw new Error(`Flag 条数 ${flagEntries.length} != 10`);

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

let rust = banner("src/objects/backing-store.h.js", "src/objects/backing-store.{h,cc}", "BackingStore 枚举与私有标志位");
rust += `//! BackingStore: ArrayBuffer/WasmMemory 后备存储描述。
//! .cc 语义实现面 (分配/GC 注册) 登记跳过。

/// Wasm 内存类别 (原版 enum class WasmMemoryFlag : uint8_t)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WasmMemoryFlag {
`;
wasmEntries.forEach((e, i) => {
  rust += `  /// ${e}\n  ${pascal(e)} = ${i},\n`;
});
rust += `}

/// 扩容结果 (原版 enum ResizeOrGrowResult)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ResizeOrGrowResult {
`;
rogEntries.forEach((e, i) => {
  rust += `  /// ${e}\n  ${pascal(e)} = ${i},\n`;
});
rust += `}

/// BackingStore::kTypeID 锚值 (ManagedTypeId::kBackingStore)。
pub const TYPE_ID: &str = "kBackingStore";

/// 私有标志位 (原版 enum Flag, 顺序位; 存于 atomic flags_ 字段)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Flag {
`;
flagEntries.forEach((e, i) => {
  rust += `  /// ${e}\n  ${pascal(e)} = 1 << ${i},\n`;
});
rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(WasmMemoryFlag::WasmMemory64 as u8 == 2);
  assert!(ResizeOrGrowResult::Race as u8 == 2);
  assert!(Flag::EmptyDeleter as u32 == 1 << 9);
};
`;

const p = writeSrc("v8-objects-rest", "backing_store.rs", rust);
console.log(`backing_store.rs: WasmMemoryFlag 3 + ResizeOrGrowResult 3 + Flag 10 位 + kTypeID 锚, 对数通过`);
