// 由 tool/src/regexp/regexp-bytecode-generator.h.js 从 v8/src/regexp/regexp-bytecode-generator.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// 字节码生成器缓冲常量与哨兵

/// 字节码缓冲初始字节数 (原版 kInitialBufferSizeInBytes = 1 * KB)
pub const INITIAL_BUFFER_SIZE_IN_BYTES: i32 = 1024;

/// 单次扩容上限字节数 (原版 kMaxBufferGrowthInBytes = 1 * MB)
pub const MAX_BUFFER_GROWTH_IN_BYTES: usize = 1024 * 1024;

/// 无效 pc 哨兵 (原版 kInvalidPC)
pub const INVALID_PC: i32 = -1;

/// 字节码生成器 (对应原版 RegExpBytecodeGenerator; 语义实现面登记跳过,
/// 位于 v8/src/regexp/regexp-bytecode-generator.cc)
pub struct BytecodeGenerator;

// —— 提取不变式断言 ——
const _: () = {
  assert!(INITIAL_BUFFER_SIZE_IN_BYTES == 1024);
  assert!(INVALID_PC == -1);
};
