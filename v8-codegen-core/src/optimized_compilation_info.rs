// 由 tool/src/codegen/optimized-compilation-info.h.js 从 v8/src/codegen/optimized-compilation-info.{h,cc} 生成; 修订就地改, 复跑覆盖
// 优化编译信息位标志

/// 优化编译开关位标志 (原 FLAGS(V) X-macro 展开, 每位一个布尔开关)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags(u32);

impl Flags {
  /// 原 kFunctionContextSpecializing (位 0, 访问器 function_context_specializing())
  pub const FUNCTION_CONTEXT_SPECIALIZING: Self = Self(1 << 0);
  /// 原 kInlining (位 1, 访问器 inlining())
  pub const INLINING: Self = Self(1 << 1);
  /// 原 kDisableFutureOptimization (位 2, 访问器 disable_future_optimization())
  pub const DISABLE_FUTURE_OPTIMIZATION: Self = Self(1 << 2);
  /// 原 kSplitting (位 3, 访问器 splitting())
  pub const SPLITTING: Self = Self(1 << 3);
  /// 原 kSourcePositions (位 4, 访问器 source_positions())
  pub const SOURCE_POSITIONS: Self = Self(1 << 4);
  /// 原 kBailoutOnUninitialized (位 5, 访问器 bailout_on_uninitialized())
  pub const BAILOUT_ON_UNINITIALIZED: Self = Self(1 << 5);
  /// 原 kLoopPeeling (位 6, 访问器 loop_peeling())
  pub const LOOP_PEELING: Self = Self(1 << 6);
  /// 原 kSwitchJumpTable (位 7, 访问器 switch_jump_table())
  pub const SWITCH_JUMP_TABLE: Self = Self(1 << 7);
  /// 原 kCalledWithCodeStartRegister (位 8, 访问器 called_with_code_start_register())
  pub const CALLED_WITH_CODE_START_REGISTER: Self = Self(1 << 8);
  /// 原 kAllocationFolding (位 9, 访问器 allocation_folding())
  pub const ALLOCATION_FOLDING: Self = Self(1 << 9);
  /// 原 kAnalyzeEnvironmentLiveness (位 10, 访问器 analyze_environment_liveness())
  pub const ANALYZE_ENVIRONMENT_LIVENESS: Self = Self(1 << 10);
  /// 原 kTraceTurboJson (位 11, 访问器 trace_turbo_json())
  pub const TRACE_TURBO_JSON: Self = Self(1 << 11);
  /// 原 kTraceTurboGraph (位 12, 访问器 trace_turbo_graph())
  pub const TRACE_TURBO_GRAPH: Self = Self(1 << 12);
  /// 原 kTraceTurboScheduled (位 13, 访问器 trace_turbo_scheduled())
  pub const TRACE_TURBO_SCHEDULED: Self = Self(1 << 13);
  /// 原 kTraceTurboAllocation (位 14, 访问器 trace_turbo_allocation())
  pub const TRACE_TURBO_ALLOCATION: Self = Self(1 << 14);
  /// 原 kTraceHeapBroker (位 15, 访问器 trace_heap_broker())
  pub const TRACE_HEAP_BROKER: Self = Self(1 << 15);
  /// 原 kDiscardResultForTesting (位 16, 访问器 discard_result_for_testing())
  pub const DISCARD_RESULT_FOR_TESTING: Self = Self(1 << 16);
  /// 原 kInlineJSWasmCalls (位 17, 访问器 inline_js_wasm_calls())
  pub const INLINE_JSWASM_CALLS: Self = Self(1 << 17);
  /// 原 kTurboshaftTraceReduction (位 18, 访问器 turboshaft_trace_reduction())
  pub const TURBOSHAFT_TRACE_REDUCTION: Self = Self(1 << 18);
  /// 原 kCouldNotInlineAllCandidates (位 19, 访问器 could_not_inline_all_candidates())
  pub const COULD_NOT_INLINE_ALL_CANDIDATES: Self = Self(1 << 19);
  /// 原 kShadowStackCompliantLazyDeopt (位 20, 访问器 shadow_stack_compliant_lazy_deopt())
  pub const SHADOW_STACK_COMPLIANT_LAZY_DEOPT: Self = Self(1 << 20);

  #[inline]
  #[must_use]
  pub const fn bits(self) -> u32 {
    self.0
  }

  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    (self.0 & other.0) == other.0
  }

  #[inline]
  #[must_use]
  pub const fn union(self, other: Self) -> Self {
    Self(self.0 | other.0)
  }
}

/// 优化 ID 缺省哨兵 (原 kNoOptimizationId)
pub const NO_OPTIMIZATION_ID: i32 = -1;

/// 原 class OptimizedCompilationInfo — 构造签名骨架:
/// - 原签名: OptimizedCompilationInfo(Zone*, Isolate*, SharedFunctionInfo, JSFunction, CodeKind, BytecodeOffset)
/// - 原签名: 测试构造 (无 OSR)
/// - 原签名: DEF_GETTER/DEF_SETTER × 21 (每开关一对, X-macro 生成)
pub struct OptimizedCompilationInfo;

// TODO(dev): 补标志生命周期与 bytecode_array 接线。

// —— 提取不变式断言 ——
const _: () = {
  assert!(Flags::FUNCTION_CONTEXT_SPECIALIZING.0 == 1);
  assert!(Flags::SHADOW_STACK_COMPLIANT_LAZY_DEOPT.0 == 1 << 20);
};
