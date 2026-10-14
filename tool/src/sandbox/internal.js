#!/usr/bin/env bun
// tool/src/sandbox/internal.js — v8-sandbox 常量族生成
// 组→文件映射 (本脚本管辖):
//   v8/include/v8-internal.h (sandbox / external-pointer 常量段) → v8-sandbox/src/internal.rs
// 规则:
//   - 从 v8-internal.h 逐常量提取 (默认目标分支: 64 位桌面, 非 Android/iOS/RISCV/LOONG, 非 LA57)
//   - 交叉关系断言 (原版 static_assert 全量复刻为脚本内对数核对)
//   - 直出 Rust 常量表; 语义逻辑无 (纯常量层)
import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("include/v8-internal.h");

const defs = new Map();
defs.set("KB", 1024);
defs.set("MB", 1024 * 1024);
defs.set("GB", 1024 * 1024 * 1024);

// ---- 数值提取工具 ----
// 解析 `constexpr <T> NAME = <expr>;`; JS 位运算是 32 位, 故 `<<` 文本替换为 *2**
function evalConst(name) {
  const m = src.match(
    new RegExp(`constexpr (?:size_t|uint32_t|uint64_t|int)\\s+${name}\\s*=\\s*([^;]+);`),
  );
  if (!m) throw new Error(`internal.js: 常量 ${name} 未找到`);
  const v = evalExpr(m[1]);
  defs.set(name, v);
  return v;
}
// 多分支常量 (Android/ChromeOS/LOONG = 0 分支在前, 默认 #else 分支在后): 取末次出现
function evalConstDefaultBranch(name) {
  const re = new RegExp(`constexpr (?:size_t|uint32_t|uint64_t|int)\\s+${name}\\s*=\\s*([^;]+);`, "g");
  const ms = [...src.matchAll(re)];
  if (ms.length < 2) throw new Error(`internal.js: 常量 ${name} 应为多分支定义`);
  const first = evalExpr(ms[0][1]);
  const v = evalExpr(ms.at(-1)[1]);
  if (first !== 0) throw new Error(`internal.js: ${name} 首分支应为 0 (受限平台), 实得 ${first}`);
  if (v <= 0) throw new Error(`internal.js: ${name} 默认分支应为正值, 实得 ${v}`);
  defs.set(name, v);
  return v;
}
function evalExpr(exprRaw) {
  let e = exprRaw
    .replace(/size_t\{(\d+)\}/g, "$1")
    .replace(/(\d)U?LL?\b/g, "$1")
    .replace(/<<\s*/g, "*2**")
    .trim();
  for (const n of [...defs.keys()].sort((a, b) => b.length - a.length))
    e = e.split(n).join(`(${defs.get(n)})`);
  e = e.replace(/\bKB\b/g, "(1024)").replace(/\bMB\b/g, "(1024*1024)").replace(/\bGB\b/g, "(1024*1024*1024)");
  if (!/^[\d\s()+\-*/.*]+$/.test(e)) throw new Error(`internal.js: 无法求值: ${exprRaw} -> ${e}`);
  const v = Function(`"use strict"; return (${e});`)();
  if (!Number.isInteger(v) || v < 0) throw new Error(`internal.js: 结果非法: ${exprRaw} = ${v}`);
  return v;
}
function macroCount(name) {
  const m = src.match(new RegExp(`#define ${name}\\s+(\\d+)`));
  if (!m) throw new Error(`internal.js: 宏 ${name} 未找到`);
  return Number(m[1]);
}

// ---- 提取 ----
defs.set("V8_EMBEDDER_DATA_TAG_COUNT", macroCount("V8_EMBEDDER_DATA_TAG_COUNT"));
defs.set("V8_EXTERNAL_POINTER_TAG_COUNT", macroCount("V8_EXTERNAL_POINTER_TAG_COUNT"));

// 指针压缩 cage (v8-internal.h:167-168)
evalConst("kPtrComprCageReservationSize");
evalConst("kPtrComprCageBaseAlignment");

// sandbox 常量族 (v8-internal.h:218-330)
// kSandboxSizeLog2 多分支: 断言默认分支 40 (1 TB) 存在
if (!/constexpr size_t kSandboxSizeLog2 = 40;\s*\/\/ 1 TB/.test(src))
  throw new Error("internal.js: kSandboxSizeLog2 默认分支应为 40 (1 TB)");
defs.set("kSandboxSizeLog2", 40);
defs.set("kSandboxSize", 2 ** 40);
defs.set("kSandboxedPointerShift", 64 - defs.get("kSandboxSizeLog2"));
evalConst("kSandboxMinimumReservationSize");
evalConst("kMaxSafeBufferSizeForSandbox");
evalConst("kBoundedSizeShift");
defs.set("kBoundedSizeMask", 2 ** (64 - defs.get("kBoundedSizeShift")) - 1);
evalConst("kSandboxGuardRegionSize");
evalConstDefaultBranch("kAdditionalTrailingGuardRegionSize");
defs.set("kSandboxAlignment", defs.get("kPtrComprCageBaseAlignment"));
// kRequiresTypedArrayAccessMasks = (kAdditionalTrailingGuardRegionSize == 0), bool 特例
defs.set(
  "kRequiresTypedArrayAccessMasks",
  defs.get("kAdditionalTrailingGuardRegionSize") === 0,
);

// 外部指针表 (v8-internal.h:333-368, 默认分支 512MB / shift 6)
if (!/constexpr size_t kExternalPointerTableReservationSize = 512 \* MB;/.test(src))
  throw new Error("internal.js: EPT 预留默认分支应为 512 MB");
if (!/constexpr uint32_t kExternalPointerIndexShift = 6;/.test(src))
  throw new Error("internal.js: kExternalPointerIndexShift 默认分支应为 6");
defs.set("kExternalPointerTableReservationSize", 512 * 1024 * 1024);
defs.set("kExternalPointerIndexShift", 6);
evalConst("kExternalPointerTableEntrySize");
evalConst("kExternalPointerTableEntrySizeLog2");
defs.set(
  "kMaxExternalPointers",
  defs.get("kExternalPointerTableReservationSize") / defs.get("kExternalPointerTableEntrySize"),
);

// tag 位布局 (v8-internal.h:377-393): 非 LA57 分支为默认; LA57 仅注释
const tagBits = {
  markBit: 2n ** 48n,
  tagShift: 49n,
  tagMask: 0x00fe000000000000n,
  tagAndMark: 0x00ff000000000000n,
  payload: 0xff00ffffffffffffn,
};
if (!src.includes("0xfe00000000000000ULL") || !src.includes("0x00fe000000000000ULL"))
  throw new Error("internal.js: tag 掩码字面量缺失");
const shiftedTagMask = tagBits.tagMask >> tagBits.tagShift;

// ---- static_assert 复刻 (对数核对) ----
const checks = [];
function check(desc, ok) {
  if (!ok) throw new Error(`internal.js: 对数核对失败: ${desc}`);
  checks.push(desc);
}
check("kSandboxedPointerShift == 24 (64-40)", defs.get("kSandboxedPointerShift") === 24);
check(
  "kSandboxMinimumReservationSize > kPtrComprCageReservationSize",
  defs.get("kSandboxMinimumReservationSize") > defs.get("kPtrComprCageReservationSize"),
);
check(
  "2**(64-kBoundedSizeShift) == kMaxSafeBufferSizeForSandbox+1",
  2 ** (64 - defs.get("kBoundedSizeShift")) === defs.get("kMaxSafeBufferSizeForSandbox") + 1,
);
check(
  "kSandboxGuardRegionSize % kSandboxAlignment == 0",
  defs.get("kSandboxGuardRegionSize") % defs.get("kSandboxAlignment") === 0,
);
check(
  "kMaxSafeBufferSizeForSandbox <= kSandboxGuardRegionSize",
  defs.get("kMaxSafeBufferSizeForSandbox") <= defs.get("kSandboxGuardRegionSize"),
);
check(
  "2**(32-kExternalPointerIndexShift) == kMaxExternalPointers",
  2 ** (32 - defs.get("kExternalPointerIndexShift")) === defs.get("kMaxExternalPointers"),
);
check(
  "kAdditionalTrailingGuardRegionSize == 288GB - kSandboxGuardRegionSize",
  defs.get("kAdditionalTrailingGuardRegionSize") === 288 * 1024 ** 3 - defs.get("kSandboxGuardRegionSize"),
);
check("shiftedTagMask << tagShift == tagMask", (shiftedTagMask << tagBits.tagShift) === tagBits.tagMask);
check("V8_EMBEDDER_DATA_TAG_COUNT == 15", defs.get("V8_EMBEDDER_DATA_TAG_COUNT") === 15);
check("V8_EXTERNAL_POINTER_TAG_COUNT == 40", defs.get("V8_EXTERNAL_POINTER_TAG_COUNT") === 40);
check("kExternalPointerTableEntrySizeLog2 == log2(entry size)", 2 ** defs.get("kExternalPointerTableEntrySizeLog2") === defs.get("kExternalPointerTableEntrySize"));

// ---- 生成 ----
const D = defs;
const body = `${banner("src/sandbox/internal.js", "include/v8-internal.h (sandbox/external-pointer 常量段)", "目标分支: 64 位桌面 (非 Android/iOS/RISCV/LOONG, 非 LA57); 原版 static_assert 已在脚本内对数核对")}
//! 沙箱常量层; 对应 v8/include/v8-internal.h 的 V8_ENABLE_SANDBOX 段。
//! 生成脚本内置对数核对 ${checks.length} 条全部通过。

/// 原版 v8-internal.h:41-43
pub const KB: usize = 1024;
pub const MB: usize = KB * 1024;
pub const GB: usize = MB * 1024;

/// 原版 Address (uintptr_t); 术语表映射 usize
pub type Address = usize;

// ---- 指针压缩 cage (v8-internal.h:167-168) ----
pub const K_PTR_COMPR_CAGE_RESERVATION_SIZE: usize = ${D.get("kPtrComprCageReservationSize")}; // 4 GB
pub const K_PTR_COMPR_CAGE_BASE_ALIGNMENT: usize = ${D.get("kPtrComprCageBaseAlignment")}; // 4 GB

// ---- 沙箱尺寸/对齐 (v8-internal.h:218-258) ----
/// 1 TB; Android/ChromeOS-ARM64/iOS/RISCV/LOONG 分支 (37/34) 见原版, 目标取默认
pub const K_SANDBOX_SIZE_LOG2: u32 = ${D.get("kSandboxSizeLog2")};
pub const K_SANDBOX_SIZE: usize = ${D.get("kSandboxSize")};
pub const K_SANDBOX_ALIGNMENT: usize = ${D.get("kSandboxAlignment")}; // = kPtrComprCageBaseAlignment
/// 沙箱内指针编码右移量 = 64 - kSandboxSizeLog2 = 24
pub const K_SANDBOXED_POINTER_SHIFT: u32 = ${D.get("kSandboxedPointerShift")};
/// 部分预留沙箱最小预留 = 8 GB
pub const K_SANDBOX_MINIMUM_RESERVATION_SIZE: usize = ${D.get("kSandboxMinimumReservationSize")};
/// 沙箱内最大安全缓冲 = 32 GB - 1
pub const K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX: usize = ${D.get("kMaxSafeBufferSizeForSandbox")};

// ---- 有界大小编码 (v8-internal.h:282-293) ----
pub const K_BOUNDED_SIZE_SHIFT: u32 = ${D.get("kBoundedSizeShift")};
/// (1 << (64 - 29)) - 1 = 2^35 - 1
pub const K_BOUNDED_SIZE_MASK: u64 = ${D.get("kBoundedSizeMask")};

// ---- 保护区域 (v8-internal.h:295-329) ----
/// 前后 guard = 32GB + (32GB-1 + 1) = 64 GB
pub const K_SANDBOX_GUARD_REGION_SIZE: usize = ${D.get("kSandboxGuardRegionSize")};
/// 追加尾部 guard = 288GB - 64GB = 224 GB (Android/ChromeOS-ARM64/LOONG 为 0)
pub const K_ADDITIONAL_TRAILING_GUARD_REGION_SIZE: usize = ${D.get("kAdditionalTrailingGuardRegionSize")};
pub const K_REQUIRES_TYPED_ARRAY_ACCESS_MASKS: bool = ${D.get("kRequiresTypedArrayAccessMasks")};

// ---- 外部指针表 (v8-internal.h:333-368, 默认分支) ----
/// Android 256MB/7 与 iOS 128MB/8 分支见原版, 目标取默认
pub const K_EXTERNAL_POINTER_TABLE_RESERVATION_SIZE: usize = ${D.get("kExternalPointerTableReservationSize")};
pub const K_EXTERNAL_POINTER_INDEX_SHIFT: u32 = ${D.get("kExternalPointerIndexShift")};
pub const K_EXTERNAL_POINTER_TABLE_ENTRY_SIZE: usize = ${D.get("kExternalPointerTableEntrySize")};
pub const K_EXTERNAL_POINTER_TABLE_ENTRY_SIZE_LOG2: u32 = ${D.get("kExternalPointerTableEntrySizeLog2")};
/// = 2**(32-6), 保证移位后索引必在表内
pub const K_MAX_EXTERNAL_POINTERS: usize = ${D.get("kMaxExternalPointers")};

// ---- 外部指针 tag 位布局 (v8-internal.h:377-393, 非 LA57 分支) ----
/// LA57 分支 (x64 LA57): mark=1<<56, shift=57, tag=0xfe00000000000000, payload=0x00ffffffffffffff
pub const K_EXTERNAL_POINTER_MARK_BIT: u64 = ${tagBits.markBit}; // 1<<48
pub const K_EXTERNAL_POINTER_TAG_SHIFT: u64 = ${tagBits.tagShift}; // 49
pub const K_EXTERNAL_POINTER_TAG_MASK: u64 = ${tagBits.tagMask}; // 0x00fe000000000000
pub const K_EXTERNAL_POINTER_TAG_AND_MARKBIT_MASK: u64 = ${tagBits.tagAndMark}; // 0x00ff000000000000
pub const K_EXTERNAL_POINTER_PAYLOAD_MASK: u64 = ${tagBits.payload}; // 0xff00ffffffffffff
/// tagMask >> tagShift = 0x7f (7 位 tag)
pub const K_EXTERNAL_POINTER_SHIFTED_TAG_MASK: u64 = ${shiftedTagMask};

// ---- tag 数量宏 (v8-internal.h:469-474) ----
pub const V8_EMBEDDER_DATA_TAG_COUNT: u32 = ${D.get("V8_EMBEDDER_DATA_TAG_COUNT")};
pub const V8_EXTERNAL_POINTER_TAG_COUNT: u32 = ${D.get("V8_EXTERNAL_POINTER_TAG_COUNT")};
`;

const p = writeSrc("v8-sandbox", "internal.rs", body);
console.log(`internal.js: ${p} (常量 ${D.size} 个, 对数核对 ${checks.length} 条通过)`);
