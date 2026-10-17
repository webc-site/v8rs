// 由 tool/src/interpreter/block-coverage-builder.h.js 从 v8/src/interpreter/block-coverage-builder.h 生成; 修订就地改, 复跑覆盖
// 块覆盖率槽位分配器骨架

/// 无覆盖槽哨兵 (原版 kNoCoverageArraySlot)
pub const NO_COVERAGE_ARRAY_SLOT: i32 = -1;

/// 原版 class BlockCoverageBuilder 成员声明 (机械 1:1 提取, 11 条)
pub const BLOCK_COVERAGE_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] BlockCoverageBuilder(Zone* zone, BytecodeArrayBuilder* builder, SourceRangeMap* source_range_map) : slots_(0, zone), builder_(builder), source_range_map_(source_range_map)",
  "[public/const] static constexpr int kNoCoverageArraySlot = -1",
  "[public/method] int AllocateBlockCoverageSlot(ZoneObject* node, SourceRangeKind kind)",
  "[public/method] int AllocateNaryBlockCoverageSlot(NaryOperation* node, size_t index)",
  "[public/method] int AllocateConditionalChainBlockCoverageSlot(ConditionalChain* node, SourceRangeKind kind, size_t index)",
  "[public/method] void IncrementBlockCounter(int coverage_array_slot)",
  "[public/method] void IncrementBlockCounter(ZoneObject* node, SourceRangeKind kind)",
  "[public/method] const ZoneVector<SourceRange>& slots() const",
  "[private/field] ZoneVector<SourceRange> slots_",
  "[private/field] BytecodeArrayBuilder* builder_",
  "[private/field] SourceRangeMap* source_range_map_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BLOCK_COVERAGE_BUILDER_DECLARATIONS.len() == 11);
  assert!(NO_COVERAGE_ARRAY_SLOT == -1);
};
