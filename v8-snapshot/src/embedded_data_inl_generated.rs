// 由 tool/src/snapshot/embedded/embedded-data-inl.h.js 从 v8/src/snapshot/embedded/embedded-data-inl.h 生成; 修订就地改, 复跑覆盖
// EmbeddedData 内联访问骨架

// 布局描述 (LayoutDescription: instruction_offset/length + metadata_offset)
// 与段指针由 embedded_data.rs 覆盖; 本文件登记内联访问面。
// 方法注册表 (7 条):
// - InstructionStartOf
// - InstructionEndOf
// - InstructionSizeOf
// - MetadataStartOf
// - InstructionStartOfBytecodeHandlers
// - InstructionEndOfBytecodeHandlers
// - PaddedInstructionSizeOf
// 关键断言锚: Builtins::kBytecodeHandlersAreSortedLast (字节码 handler 段排尾部,
// EndOfBytecodeHandlers 即 code_ + code_size_, 含尾部填充)。

pub struct EmbeddedDataInl;

// TODO(dev): 依 embedded_data.rs 补访问实现与 PadAndAlignCode。

// —— 提取不变式断言 ——
const _: () = {};
