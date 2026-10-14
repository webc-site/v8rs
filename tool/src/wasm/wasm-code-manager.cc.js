#!/usr/bin/env bun
// tool/src/wasm/wasm-code-manager.cc.js
// 机械提取 v8/src/wasm/wasm-code-manager.cc 的代码尺寸估算因子表、
// 代码查找缓存、分配区域哨兵与防线断言。
// 状态机枚举/常量在 v8-wasm-core/src/wasm_code_manager_consts.rs (wasm-code-manager.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/wasm-code-manager.cc");

// ---------- 1. WasmCode 尺寸防线 ----------
if (!src.includes("static_assert(sizeof(WasmCode) <= 128);")) {
  throw new Error("sizeof(WasmCode) 防线缺失");
}

// ---------- 2. 参数槽位上限 ----------
const maxSlots = src.match(/static constexpr int kMaxSlotsPerParam = (\d+);/);
if (!maxSlots || maxSlots[1] !== "4") throw new Error("kMaxSlotsPerParam != 4");

// ---------- 3. 各架构代码尺寸因子表 ----------
const archOrder = ["x64", "ia32", "arm", "arm64", "other"];
const factors = [...src.matchAll(
  /constexpr size_t kTurbofanFunctionOverhead = (\d+);\s*constexpr size_t kTurbofanCodeSizeMultiplier = (\d+);\s*constexpr size_t kLiftoffFunctionOverhead = (\d+);\s*constexpr size_t kLiftoffCodeSizeMultiplier = (\d+);/g,
)].map((m) => [Number(m[1]), Number(m[2]), Number(m[3]), Number(m[4])]);
if (factors.length !== 5) throw new Error(`架构因子组 ${factors.length} != 5`);
if (JSON.stringify(factors[0]) !== JSON.stringify([24, 3, 56, 4])) throw new Error("x64 因子漂移");
if (JSON.stringify(factors[3]) !== JSON.stringify([40, 3, 68, 4])) throw new Error("arm64 因子漂移");
if (JSON.stringify(factors[4]) !== JSON.stringify([44, 4, 96, 5])) throw new Error("兜底因子漂移");

// ---------- 4. 估算公式 ----------
if (!src.includes("return kLiftoffFunctionOverhead + kCodeAlignment / 2 +\n         body_size * kLiftoffCodeSizeMultiplier;")) {
  throw new Error("Liftoff 估算公式缺失");
}

// ---------- 5. 无限制区域哨兵 ----------
if (!src.includes("constexpr base::AddressRegion kUnrestrictedRegion{\n    kNullAddress, std::numeric_limits<size_t>::max()};")) {
  throw new Error("kUnrestrictedRegion 哨兵缺失");
}

// ---------- 6. 分段保留分割开关 (32 位) ----------
const splitFlags = [...src.matchAll(/constexpr bool kNeedsToSplitRangeByReservations = (true|false);/g)].map((m) => m[1]);
if (splitFlags.join(",") !== "true,false") throw new Error("分段分割双值缺失");

// ---------- 7. 代码查找缓存: 2 的幂, hash & (size-1) ----------
if (!src.includes("static_assert(base::bits::IsPowerOfTwo(kWasmCodeLookupCacheSize));")) {
  throw new Error("缓存大小 2 的幂断言缺失");
}
if (!src.includes("uint32_t index = hash & (kWasmCodeLookupCacheSize - 1);")) {
  throw new Error("缓存索引公式缺失");
}

// ---------- 8. 分层预算数组元素宽 ----------
if (!src.includes("static_assert(sizeof(*tiering_budgets_.get()) == sizeof(uint32_t));")) {
  throw new Error("分层预算元素宽断言缺失");
}

// ---------- 9. 执行层序断言 ----------
if (!src.includes("static_assert(ExecutionTier::kNone < ExecutionTier::kLiftoff &&")) {
  throw new Error("执行层序断言缺失");
}

// ---------- 10. GC 对象序断言 (code_allocator_ 在 GC 指针前) ----------
if (!src.includes("static_assert(offsetof(NativeModule, code_allocator_) >")) {
  throw new Error("NativeModule 字段序断言缺失");
}

// ---------- 11. 源码翻译注释标记 ----------
if (!src.includes('constexpr char pc_offset[] = "pc-offset";')) {
  throw new Error("源码翻译标记 pc-offset 缺失");
}

console.log(`wasm-code-manager: 架构因子=${factors.length} 分割开关=${splitFlags.length}`);

// ---------- 生成 Rust ----------
const ARCH_RUST = ["X64", "IA32", "ARM", "ARM64", "OTHER"];
let factorTable = "";
for (let i = 0; i < factors.length; i++) {
  const [tf, tm, lf, lm] = factors[i];
  factorTable += `    ("${ARCH_RUST[i]}", CodeSizeFactor { turbofan_overhead: ${tf}, turbofan_multiplier: ${tm}, liftoff_overhead: ${lf}, liftoff_multiplier: ${lm} }),\n`;
}

const rust = `${banner(
  "src/wasm/wasm-code-manager.cc.js",
  "src/wasm/wasm-code-manager.cc",
  "代码尺寸估算因子与代码管理防线 (状态机在 wasm_code_manager_consts.rs)",
)}
//! WasmCode / NativeModule 行为规则面。

/// WasmCode 结构尺寸上限 (缓存友好)
/// v8: src/wasm/wasm-code-manager.cc static_assert(sizeof(WasmCode) <= 128)
pub const WASM_CODE_MAX_SIZE: usize = 128;

/// 每参数最大栈槽位 (32 位平台 S128 占 4 槽)
/// v8: src/wasm/wasm-code-manager.cc kMaxSlotsPerParam
pub const K_MAX_SLOTS_PER_PARAM: i32 = ${maxSlots[1]};

/// 代码尺寸估算因子 (按架构; 由 --trace-wasm-compilation-times 基准得出)
/// v8: src/wasm/wasm-code-manager.cc kTurbofan/kLiftoff FunctionOverhead/CodeSizeMultiplier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeSizeFactor {
    pub turbofan_overhead: usize,
    pub turbofan_multiplier: usize,
    pub liftoff_overhead: usize,
    pub liftoff_multiplier: usize,
}

#[rustfmt::skip]
pub static CODE_SIZE_FACTORS: &[(&str, CodeSizeFactor)] = &[
${factorTable}];

/// Liftoff 代码尺寸估算: overhead + kCodeAlignment/2 + body * multiplier
/// v8: src/wasm/wasm-code-manager.cc WasmCodeManager::EstimateLiftoffCodeSize
pub fn estimate_liftoff_code_size(body_size: usize, f: CodeSizeFactor, code_alignment: usize) -> usize {
    f.liftoff_overhead + code_alignment / 2 + body_size * f.liftoff_multiplier
}

/// Turbofan 代码尺寸估算: 同构公式换用 turbofan 因子
/// v8: src/wasm/wasm-code-manager.cc WasmCodeManager::EstimateTurbofanCodeSize
pub fn estimate_turbofan_code_size(body_size: usize, f: CodeSizeFactor, code_alignment: usize) -> usize {
    f.turbofan_overhead + code_alignment / 2 + body_size * f.turbofan_multiplier
}

/// 区域分配"无限制"哨兵: (空地址, size_t 最大值)
/// v8: src/wasm/wasm-code-manager.cc kUnrestrictedRegion
pub const UNRESTRICTED_REGION_SENTINEL: &str = "(kNullAddress, size_t::MAX)";

/// 32 位平台需按保留区切分地址区间 (64 位直接整段)
/// v8: src/wasm/wasm-code-manager.cc kNeedsToSplitRangeByReservations
pub const SPLIT_RANGE_BY_RESERVATIONS_32BIT: bool = true;
pub const SPLIT_RANGE_BY_RESERVATIONS_64BIT: bool = false;

/// 代码查找缓存: 大小为 2 的幂, 索引 = hash & (size-1)
/// v8: src/wasm/wasm-code-manager.cc WasmCodeLookupCache::GetCacheEntry
pub const CODE_LOOKUP_CACHE_SIZE_IS_POW2: bool = true;
pub const CODE_LOOKUP_CACHE_INDEX: &str = "hash & (kWasmCodeLookupCacheSize - 1)";

/// 分层预算数组元素为 u32
/// v8: src/wasm/wasm-code-manager.cc static_assert(sizeof(*tiering_budgets_.get()) == sizeof(uint32_t))
pub const TIERING_BUDGET_ELEM_WIDTH: usize = 4;

/// 执行层序: kNone < kLiftoff (其余层按枚举序递增)
/// v8: src/wasm/wasm-code-manager.cc static_assert(ExecutionTier::kNone < kLiftoff)
pub const EXECUTION_TIER_ORDER: &str = "None < Liftoff < ...";

/// 源码翻译注释标记 (DebugSideTable / 译者注释用)
/// v8: src/wasm/wasm-code-manager.cc pc_offset / source_offset / translation_index
pub const PC_OFFSET_MARK: &str = "pc-offset";
pub const SOURCE_OFFSET_MARK: &str = " source-offset";
pub const TRANSLATION_INDEX_MARK: &str = " translation-index";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(WASM_CODE_MAX_SIZE, 128);
    assert_eq!(K_MAX_SLOTS_PER_PARAM, 4);
    assert_eq!(CODE_SIZE_FACTORS.len(), 5);
    assert_eq!(CODE_SIZE_FACTORS[0].1.turbofan_overhead, 24);
    assert_eq!(CODE_SIZE_FACTORS[0].1.liftoff_multiplier, 4);
    assert_eq!(CODE_SIZE_FACTORS[1].1.liftoff_overhead, 48);
    assert_eq!(CODE_SIZE_FACTORS[2].1.liftoff_multiplier, 5);
    assert_eq!(CODE_SIZE_FACTORS[3].1.liftoff_overhead, 68);
    assert_eq!(CODE_SIZE_FACTORS[4].1.turbofan_multiplier, 4);
    let f = CODE_SIZE_FACTORS[0].1;
    assert_eq!(estimate_liftoff_code_size(100, f, 64), 56 + 32 + 400);
    assert_eq!(estimate_turbofan_code_size(100, f, 64), 24 + 32 + 300);
};
`;

const p = writeSrc("v8-wasm-objects", "wasm_code_manager_cc.rs", rust);
console.log(`written ${p}`);
