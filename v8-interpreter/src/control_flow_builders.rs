// 由 tool/src/interpreter/control-flow-builders.h.js 从 v8/src/interpreter/control-flow-builders.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 原版 class/struct ControlFlowBuilder 成员声明 (机械 1:1 提取, 6 条)
pub const CONTROL_FLOW_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] explicit ControlFlowBuilder(BytecodeArrayBuilder* builder) : builder_(builder)",
  "[public/method] ControlFlowBuilder(const ControlFlowBuilder&) = delete",
  "[public/method] ControlFlowBuilder& operator=(const ControlFlowBuilder&) = delete",
  "[public/method] virtual ~ControlFlowBuilder() = default",
  "[protected/method] BytecodeArrayBuilder* builder() const",
  "[private/field] BytecodeArrayBuilder* builder_",
];

/// 原版 class/struct BreakableControlFlowBuilder 成员声明 (机械 1:1 提取, 15 条)
pub const BREAKABLE_CONTROL_FLOW_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] BreakableControlFlowBuilder(BytecodeArrayBuilder* builder, BlockCoverageBuilder* block_coverage_builder, AstNode* node) : ControlFlowBuilder(builder), break_labels_(builder->zone()), node_(node), block_coverage_builder_(block_coverage_builder)",
  "[public/method] ~BreakableControlFlowBuilder() override",
  "[public/method] void Break()",
  "[public/method] void BreakIfTrue(BytecodeArrayBuilder::ToBooleanMode mode)",
  "[public/method] void BreakIfForInDone(Register index, Register cache_length)",
  "[public/method] BytecodeLabels* break_labels()",
  "[protected/method] void EmitJump(BytecodeLabels* labels)",
  "[protected/method] void EmitJumpIfTrue(BytecodeArrayBuilder::ToBooleanMode mode, BytecodeLabels* labels)",
  "[protected/method] void EmitJumpIfFalse(BytecodeArrayBuilder::ToBooleanMode mode, BytecodeLabels* labels)",
  "[protected/method] void EmitJumpIfUndefined(BytecodeLabels* labels)",
  "[protected/method] void EmitJumpIfForInDone(BytecodeLabels* labels, Register index, Register cache_length)",
  "[protected/method] void BindBreakTarget()",
  "[protected/field] BytecodeLabels break_labels_",
  "[protected/field] AstNode* node_",
  "[protected/field] BlockCoverageBuilder* block_coverage_builder_",
];

/// 原版 class/struct BlockBuilder 成员声明 (机械 1:1 提取, 1 条)
pub const BLOCK_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] BlockBuilder(BytecodeArrayBuilder* builder, BlockCoverageBuilder* block_coverage_builder, BreakableStatement* statement) : BreakableControlFlowBuilder(builder, block_coverage_builder, statement)",
];

/// 原版 class/struct LoopBuilder 成员声明 (机械 1:1 提取, 16 条)
pub const LOOP_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] LoopBuilder(BytecodeArrayBuilder* builder, BlockCoverageBuilder* block_coverage_builder, AstNode* node, FeedbackVectorSpec* feedback_vector_spec) : BreakableControlFlowBuilder(builder, block_coverage_builder, node), continue_labels_(builder->zone()), end_labels_(builder->zone()), feedback_vector_spec_(feedback_vector_spec)",
  "[public/method] ~LoopBuilder() override",
  "[public/method] void LoopHeader()",
  "[public/method] void LoopBody()",
  "[public/method] void JumpToHeader(int loop_depth, LoopBuilder* const parent_loop)",
  "[public/method] void BindContinueTarget()",
  "[public/method] void Continue()",
  "[public/method] void ContinueIfUndefined()",
  "[private/method] void JumpToLoopEnd()",
  "[private/method] void BindLoopEnd()",
  "[private/field] BytecodeLoopHeader loop_header_",
  "[private/field] BytecodeLabels continue_labels_",
  "[private/field] BytecodeLabels end_labels_",
  "[private/field] int block_coverage_body_slot_",
  "[private/field] int source_position_",
  "[private/field] FeedbackVectorSpec* const feedback_vector_spec_",
];

/// 原版 class/struct SwitchBuilder 成员声明 (机械 1:1 提取, 14 条)
pub const SWITCH_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] SwitchBuilder(BytecodeArrayBuilder* builder, BlockCoverageBuilder* block_coverage_builder, SwitchStatement* statement, int number_of_cases, BytecodeJumpTable* jump_table) : BreakableControlFlowBuilder(builder, block_coverage_builder, statement), case_sites_(builder->zone()), default_(builder->zone()), fall_through_(builder->zone()), jump_table_(jump_table)",
  "[public/method] ~SwitchBuilder() override",
  "[public/method] void BindCaseTargetForJumpTable(int case_value, CaseClause* clause)",
  "[public/method] void BindCaseTargetForCompareJump(int index, CaseClause* clause)",
  "[public/method] void JumpToCaseIfTrue(BytecodeArrayBuilder::ToBooleanMode mode, int index)",
  "[public/method] void EmitJumpTableIfExists(int min_case, int max_case, std::map<int, CaseClause*>& covered_cases)",
  "[public/method] void BindDefault(CaseClause* clause)",
  "[public/method] void JumpToDefault()",
  "[public/method] void JumpToFallThroughIfFalse()",
  "[private/field] ZoneVector<BytecodeLabel> case_sites_",
  "[private/field] BytecodeLabels default_",
  "[private/field] BytecodeLabels fall_through_",
  "[private/field] BytecodeJumpTable* jump_table_",
  "[private/method] void BuildBlockCoverage(CaseClause* clause)",
];

/// 原版 class/struct TryCatchBuilder 成员声明 (机械 1:1 提取, 10 条)
pub const TRY_CATCH_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] TryCatchBuilder(BytecodeArrayBuilder* builder, BlockCoverageBuilder* block_coverage_builder, TryCatchStatement* statement, HandlerTable::CatchPrediction catch_prediction) : ControlFlowBuilder(builder), handler_id_(builder->NewHandlerEntry()), catch_prediction_(catch_prediction), block_coverage_builder_(block_coverage_builder), statement_(statement)",
  "[public/method] ~TryCatchBuilder() override",
  "[public/method] void BeginTry(Register context)",
  "[public/method] void EndTry(bool emit_catch = true)",
  "[public/method] void EndCatch()",
  "[private/field] int handler_id_",
  "[private/field] HandlerTable::CatchPrediction catch_prediction_",
  "[private/field] BytecodeLabel exit_",
  "[private/field] BlockCoverageBuilder* block_coverage_builder_",
  "[private/field] TryCatchStatement* statement_",
];

/// 原版 class/struct TryFinallyBuilder 成员声明 (机械 1:1 提取, 14 条)
pub const TRY_FINALLY_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] TryFinallyBuilder(BytecodeArrayBuilder* builder, BlockCoverageBuilder* block_coverage_builder, TryFinallyStatement* statement, HandlerTable::CatchPrediction catch_prediction) : ControlFlowBuilder(builder), handler_id_(builder->NewHandlerEntry()), catch_prediction_(catch_prediction), finalization_sites_(builder->zone()), block_coverage_builder_(block_coverage_builder), statement_(statement)",
  "[public/method] ~TryFinallyBuilder() override",
  "[public/method] void BeginTry(Register context)",
  "[public/method] void LeaveTry()",
  "[public/method] void EndTry()",
  "[public/method] void BeginHandler()",
  "[public/method] void BeginFinally()",
  "[public/method] void EndFinally()",
  "[private/field] int handler_id_",
  "[private/field] HandlerTable::CatchPrediction catch_prediction_",
  "[private/field] BytecodeLabel handler_",
  "[private/field] BytecodeLabels finalization_sites_",
  "[private/field] BlockCoverageBuilder* block_coverage_builder_",
  "[private/field] TryFinallyStatement* statement_",
];

/// 原版 class/struct ConditionalChainControlFlowBuilder 成员声明 (机械 1:1 提取, 16 条)
pub const CONDITIONAL_CHAIN_CFB_DECLARATIONS: &[&str] = &[
  "[public/method] ConditionalChainControlFlowBuilder( BytecodeArrayBuilder* builder, BlockCoverageBuilder* block_coverage_builder, AstNode* node, size_t then_count) : ControlFlowBuilder(builder), end_labels_(builder->zone()), then_count_(then_count), then_labels_list_(static_cast<int>(then_count_), builder->zone()), else_labels_list_(static_cast<int>(then_count_), builder->zone()), block_coverage_then_slots_(then_count_, builder->zone()), block_coverage_else_slots_(then_count_, builder->zone()), block_coverage_builder_(block_coverage_builder)",
  "[public/method] ~ConditionalChainControlFlowBuilder() override",
  "[public/method] BytecodeLabels* then_labels_at(size_t index)",
  "[public/method] BytecodeLabels* else_labels_at(size_t index)",
  "[public/method] int block_coverage_then_slot_at(size_t index) const",
  "[public/method] int block_coverage_else_slot_at(size_t index) const",
  "[public/method] void ThenAt(size_t index)",
  "[public/method] void ElseAt(size_t index)",
  "[public/method] void JumpToEnd()",
  "[private/field] BytecodeLabels end_labels_",
  "[private/field] size_t then_count_",
  "[private/field] ZonePtrList<BytecodeLabels> then_labels_list_",
  "[private/field] ZonePtrList<BytecodeLabels> else_labels_list_",
  "[private/field] ZoneVector<int> block_coverage_then_slots_",
  "[private/field] ZoneVector<int> block_coverage_else_slots_",
  "[private/field] BlockCoverageBuilder* block_coverage_builder_",
];

/// 原版 class/struct ConditionalControlFlowBuilder 成员声明 (机械 1:1 提取, 14 条)
pub const CONDITIONAL_CFB_DECLARATIONS: &[&str] = &[
  "[public/method] ConditionalControlFlowBuilder(BytecodeArrayBuilder* builder, BlockCoverageBuilder* block_coverage_builder, AstNode* node) : ControlFlowBuilder(builder), end_labels_(builder->zone()), then_labels_(builder->zone()), else_labels_(builder->zone()), node_(node), block_coverage_builder_(block_coverage_builder)",
  "[public/method] ~ConditionalControlFlowBuilder() override",
  "[public/method] BytecodeLabels* then_labels()",
  "[public/method] BytecodeLabels* else_labels()",
  "[public/method] void Then()",
  "[public/method] void Else()",
  "[public/method] void JumpToEnd()",
  "[private/field] BytecodeLabels end_labels_",
  "[private/field] BytecodeLabels then_labels_",
  "[private/field] BytecodeLabels else_labels_",
  "[private/field] AstNode* node_",
  "[private/field] int block_coverage_then_slot_",
  "[private/field] int block_coverage_else_slot_",
  "[private/field] BlockCoverageBuilder* block_coverage_builder_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(CONTROL_FLOW_BUILDER_DECLARATIONS.len() == 6);
};
