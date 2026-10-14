// 由 tool/src/wasm/wasm-code-manager.cc.js 从 v8/src/wasm/wasm-code-manager.cc 生成; 修订就地改, 复跑覆盖
// 代码尺寸估算因子与代码管理防线 (状态机在 wasm_code_manager_consts.rs)

//! WasmCode / NativeModule 行为规则面。

/// WasmCode 结构尺寸上限 (缓存友好)
/// v8: src/wasm/wasm-code-manager.cc static_assert(sizeof(WasmCode) <= 128)
pub const WASM_CODE_MAX_SIZE: usize = 128;

/// 每参数最大栈槽位 (32 位平台 S128 占 4 槽)
/// v8: src/wasm/wasm-code-manager.cc kMaxSlotsPerParam
pub const K_MAX_SLOTS_PER_PARAM: i32 = 4;

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
    ("X64", CodeSizeFactor { turbofan_overhead: 24, turbofan_multiplier: 3, liftoff_overhead: 56, liftoff_multiplier: 4 }),
    ("IA32", CodeSizeFactor { turbofan_overhead: 20, turbofan_multiplier: 3, liftoff_overhead: 48, liftoff_multiplier: 3 }),
    ("ARM", CodeSizeFactor { turbofan_overhead: 44, turbofan_multiplier: 3, liftoff_overhead: 96, liftoff_multiplier: 5 }),
    ("ARM64", CodeSizeFactor { turbofan_overhead: 40, turbofan_multiplier: 3, liftoff_overhead: 68, liftoff_multiplier: 4 }),
    ("OTHER", CodeSizeFactor { turbofan_overhead: 44, turbofan_multiplier: 4, liftoff_overhead: 96, liftoff_multiplier: 5 }),
];

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
