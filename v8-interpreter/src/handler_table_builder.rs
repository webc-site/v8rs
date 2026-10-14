// 由 tool/src/interpreter/handler-table-builder.h.js 从 v8/src/interpreter/handler-table-builder.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 异常表条目 (原版 struct Entry 字段面)
#[derive(Debug, Clone, Copy)]
pub struct HandlerEntrySkeleton {
  /// try 区起始字节码偏移
  pub offset_start: i32,
  /// try 区结束字节码偏移
  pub offset_end: i32,
  /// handler 目标字节码偏移
  pub offset_target: i32,
  /// handler 上下文寄存器索引 (原版 Register, index 编码)
  pub context_register: i32,
  /// catch 预测 (原版 HandlerTable::CatchPrediction, u8 透传)
  pub catch_prediction: u8,
  /// 是否已丢弃
  pub dropped: bool,
}

/// 原版 class/struct HandlerTableBuilder 成员声明 (机械 1:1 提取, 12 条)
pub const HANDLER_TABLE_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] explicit HandlerTableBuilder(Zone* zone)",
  "[public/method] HandlerTableBuilder(const HandlerTableBuilder&) = delete",
  "[public/method] HandlerTableBuilder& operator=(const HandlerTableBuilder&) = delete",
  "[public/method] int NewHandlerEntry()",
  "[public/method] void SetTryRegionStart(int handler_id, size_t offset)",
  "[public/method] void SetTryRegionEnd(int handler_id, size_t offset)",
  "[public/method] void SetHandlerTarget(int handler_id, size_t offset)",
  "[public/method] void SetPrediction(int handler_id, HandlerTable::CatchPrediction prediction)",
  "[public/method] void SetContextRegister(int handler_id, Register reg)",
  "[public/method] void DropHandlerEntry(int handler_id)",
  "[private/field] struct Entry",
  "[private/field] ZoneVector<Entry> entries_",
];

/// 原版 class/struct Entry 成员声明 (机械 1:1 提取, 6 条)
pub const HANDLER_TABLE_ENTRY_DECLARATIONS: &[&str] = &[
  "[private/field] int offset_start",
  "[private/field] // Bytecode offset starting try-region. int offset_end",
  "[private/field] // Bytecode offset ending try-region. int offset_target",
  "[private/field] // Bytecode offset of handler target. Register context",
  "[private/field] // Register holding context for handler. HandlerTable::CatchPrediction catch_prediction_",
  "[private/field] bool dropped",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(HANDLER_TABLE_ENTRY_DECLARATIONS.len() == 6);
};
