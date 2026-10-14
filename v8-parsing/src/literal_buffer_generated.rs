// 由 tool/src/parsing/literal-buffer.h.js 从 v8/src/parsing/literal-buffer.h 生成; 修订就地改, 复跑覆盖
// LiteralBuffer 预分配尺寸与容量扩容常量与策略

/// 初始缓冲区容量 (256 字节)
pub const K_INITIAL_CAPACITY: usize = 256;

/// 缓冲区增长因子 (4 倍)
pub const K_GROWTH_FACTOR: usize = 4;

/// 最大单次增量 (1MB = 1048576 字节)
pub const K_MAX_GROWTH: usize = 1048576;

/// 计算新容量 (对应 LiteralBuffer::NewCapacity)
#[inline]
#[must_use]
pub const fn new_capacity(min_capacity: usize) -> usize {
  if min_capacity < (K_MAX_GROWTH / (K_GROWTH_FACTOR - 1)) {
    min_capacity * K_GROWTH_FACTOR
  } else {
    min_capacity + K_MAX_GROWTH
  }
}
