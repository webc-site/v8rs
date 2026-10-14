// 由 tool/src/wasm/wasm-disassembler.h.js 从 v8/src/wasm/wasm-disassembler.h + wasm-disassembler-impl.h 生成; 修订就地改, 复跑覆盖
// 反汇编器行为配置常量

/// 类型段是否跳过函数类型打印 (原版 kSkipFunctionTypesInTypeSection; true 为 DevTools legacy 行为)
pub const SKIP_FUNCTION_TYPES_IN_TYPE_SECTION: bool = true;

/// 是否打印索引注释 (原版 kIndicesAsComments = NamesProvider::kIndexAsComment)
pub const INDICES_AS_COMMENTS: bool = true;

/// 是否跳过数据段名打印 (原版 kSkipDataSegmentNames)
pub const SKIP_DATA_SEGMENT_NAMES: bool = true;

/// 反汇编行缓冲宽 (原版 MultiLineStringBuilder::kBufferSize; 恰容一个 uint64)
pub const BUFFER_SIZE: usize = 20;

/// 无效递归组哨兵 (原版 RecGroup::kInvalid = ~0u)
pub const INVALID_REC_GROUP: u32 = u32::MAX;

/// 函数头打印策略 (对应原版 FunctionHeader : bool)
pub type FunctionHeader = bool;

/// kSkipHeader
pub const SKIP_HEADER: FunctionHeader = false;
/// kPrintHeader
pub const PRINT_HEADER: FunctionHeader = true;

// —— 提取不变式断言 ——
const _: () = {
  assert!(BUFFER_SIZE == 20);
  assert!(INVALID_REC_GROUP == 0xFFFF_FFFF);
  assert!(SKIP_FUNCTION_TYPES_IN_TYPE_SECTION);
  assert!(SKIP_DATA_SEGMENT_NAMES);
  assert!(PRINT_HEADER);
  assert!(!SKIP_HEADER);
};
