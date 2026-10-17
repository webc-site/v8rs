#!/usr/bin/env bun
// tool/src/objects/backing-store.cc.js — 覆盖 v8/src/objects/backing-store.cc
// 组→文件映射 (本脚本管辖): 席位表差集 (.h 布局/常量归既有 tool/src/objects/backing-store.h.js)
// → v8-objects-js-async/src/backing_store_cc.rs
// 机械提取: AllocationStatus 枚举 + kFullGuardSize32 + .cc 方法签名骨架 (todo! 就地修订)

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/backing-store.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[backing-store.cc.js] ${m}`); };

// ── 1. AllocationStatus 枚举 (匿名 namespace, UMA 上报) ──────────────────
const st = cc.match(/enum class AllocationStatus \{([\s\S]*?)\n\};/);
must(st, "cc 缺 AllocationStatus 枚举");
// 逐行去注释后提取条目 (避开注释中 k 开头词, 如 "unknown")
const statusEntries = st[1].split("\n")
  .flatMap((l) => l.replace(/\/\/.*$/, "").match(/(k\w+)/g) || []);
must(new Set(statusEntries).size === statusEntries.length, "AllocationStatus 条目重复");
must(statusEntries.length === 4, `AllocationStatus 条数 ${statusEntries.length} != 4`);
must(statusEntries.join(",") === "kSuccess,kSuccessAfterRetry,kAddressSpaceLimitReachedFailure,kOtherFailure",
  `AllocationStatus 序漂移: ${statusEntries.join(",")}`);

// ── 2. kFullGuardSize32 (64 位 + wasm 构建分支: 8GB) ─────────────────────
const guard = cc.match(/#if V8_ENABLE_WEBASSEMBLY && V8_TARGET_ARCH_64_BIT\s*\nconstexpr size_t kFullGuardSize32 = uint64_t\{8\} \* GB;/);
must(guard, "cc 缺 kFullGuardSize32 = 8 * GB (64bit+wasm 守卫)");
must(cc.includes("static_assert(kFullGuardSize32 >= size_t{4} * GB);"), "缺 kFullGuardSize32 >= 4GB 断言");

// ── 3. backing_store_id 起始值 ───────────────────────────────────────────
must(cc.includes("std::atomic<uint32_t> next_backing_store_id_{1};"), "next_backing_store_id_ 起始值 != 1");

// ── 4. 关键语义锚 ────────────────────────────────────────────────────────
for (const s of [
  "BackingStore::GetWasmReservationSize(HasGuardRegions has_guard_regions,",
  "wasm::kMaxMemory64Size + AllocatePageSize()",
  "wasm::kWasmPageSize % AllocatePageSize()",
  "DCHECK_IMPLIES(is_wasm_memory64 && has_guard_regions,",
  "DCHECK_IMPLIES(resizable, !custom_deleter);",
  "RecordStatus(isolate, AllocationStatus::kOtherFailure);",
  "RecordStatus(isolate, did_retry ? AllocationStatus::kSuccessAfterRetry",
]) must(cc.includes(s), `cc 缺语义锚 ${s}`);

// ── 5. .cc 方法签名骨架 (全局提取 BackingStore::name(, 唯一化) ────────────
const ccSigs = [...cc.matchAll(/BackingStore::(\w+)\(/g)].map((m) => m[1]);
const unique = [...new Set(ccSigs)];
must(unique.length >= 18, `.cc BackingStore 方法数 ${unique.length} < 18`);
for (const s of ["GetWasmReservationSize", "AllocateWasmMemory", "GrowWasmMemoryInPlace", "CopyWasmMemory",
  "ResizeInPlace", "GrowInPlace", "WrapAllocation", "EmptyBackingStore", "BroadcastSharedWasmMemoryGrow"]) {
  must(unique.includes(s), `.cc 缺方法 ${s}`);
}

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
const pascal = (s) => s.replace(/^k/, "").replace(/(^|_)(\w)/g, (_, a, b) => b.toUpperCase());

let rust = banner("src/objects/backing-store.cc.js", "src/objects/backing-store.cc",
  "席位表差集: .cc 分配状态/保留区公式 + 签名骨架 (.h 布局归既有 backing-store.h.js)");
rust += `//! BackingStore .cc 差集 (ArrayBuffer/WasmMemory 后备存储实现面)。
//!
//! 布局与类型常量归既有 backing-store.h.js; 本文件登记分配结果枚举、
//! wasm 保留区公式锚与 .cc 方法骨架。

// ── AllocationStatus (匿名 ns, UMA wasm_memory_allocation_result 上报) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AllocationStatus {
`;
statusEntries.forEach((n, i) => { rust += `  /// ${n}\n  ${pascal(n)} = ${i},\n`; });
rust += `}

impl AllocationStatus {
  /// 状态总数
  pub const COUNT: usize = ${statusEntries.length};
}

// ── wasm 内存保留区 (64 位 + V8_ENABLE_WEBASSEMBLY 分支) ──
/// 32 位 wasm 内存的完整 guard 区大小 (原版 kFullGuardSize32 = uint64{8} * GB)。
pub const K_FULL_GUARD_SIZE32: usize = 8 * 1024 * 1024 * 1024;

/// backing_store_id 计数器起始值 (原版 next_backing_store_id_{1})。
pub const K_NEXT_BACKING_STORE_ID_INIT: u32 = 1;

// ── GetWasmReservationSize 公式锚 (cc; memory64 + guard = kMaxMemory64Size + 页大小) ──
//   has_guard_regions: byte_capacity 上限 4GB (wasm32) / kMaxMemory64Size (memory64),
//   保留大小 = kMaxMemory64Size + AllocatePageSize() (带 guard)
//            = kMaxMemory64Size (无 guard); wasm32 全 guard = kFullGuardSize32

// ── .cc 方法签名骨架 (${unique.length} 个唯一方法, 源码序; 语义就地修订) ──
`;
for (const m of unique) {
  rust += `/// 原版 BackingStore::${m} (backing-store.cc)。
pub fn ${snake(m)}() {
  todo!("就地修订: backing-store.cc ${m}")
}

`;
}
rust += `// ── 提取不变式断言 ──
const _: () = {
  assert!(AllocationStatus::COUNT == 4);
  assert!(AllocationStatus::Success as u8 == 0);
  assert!(AllocationStatus::OtherFailure as u8 == 3);
  assert!(K_FULL_GUARD_SIZE32 >= 4 * 1024 * 1024 * 1024);
  assert!(K_NEXT_BACKING_STORE_ID_INIT == 1);
};
`;

const p = writeSrc("v8-objects-js-async", "backing_store_cc.rs", rust);
console.log(`backing-store.cc.js: ${p} — AllocationStatus ${statusEntries.length} + kFullGuardSize32 8GB + .cc 唯一方法 ${unique.length}, 断言 ${checks} 项全过`);
