// 由 tool/src/wasm/baseline/liftoff-assembler.h.js 从 v8/src/wasm/baseline/liftoff-assembler.h 生成; 修订就地改, 复跑覆盖
// Liftoff 汇编器帧布局常量与控制流枚举

/// 栈槽宽 (原版 kStackSlotSize)
pub const STACK_SLOT_SIZE: i32 = 8;

/// 内联局部类别槽位数 (原版 kInlineLocalKinds; 超出转堆数组)
pub const INLINE_LOCAL_KINDS: u32 = 16;

/// 无缓存内存起始下标记 (原版 kNoCachedMemIndex)
pub const NO_CACHED_MEM_INDEX: i32 = -1;

/// 寄存器溢出目标 (对应原版 CacheState::SpillLocation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SpillLocation {
  /// kTopOfStack — push 方式溢出到栈顶
  TopOfStack = 0,
  /// kStackSlots — 写回各自栈槽
  StackSlots = 1,
}

/// 分支时局部处理 (对应原版 enum IgnoreLocals : bool)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IgnoreLocals {
  /// kIncludeLocals (0)
  IncludeLocals = 0,
  /// kIgnoreLocals (1)
  IgnoreLocals = 1,
}

/// 栈合并方向 (对应原版 JumpDirection)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JumpDirection {
  /// kForwardJump
  ForwardJump = 0,
  /// kBackwardJump
  BackwardJump = 1,
}

/// 字节序选择 (对应原版 Endianness)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Endianness {
  /// kNative
  Native = 0,
  /// kLittle
  Little = 1,
}

/// Smi 检查分支 (对应原版 SmiCheckMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SmiCheckMode {
  /// kJumpOnSmi
  JumpOnSmi = 0,
  /// kJumpOnNotSmi
  JumpOnNotSmi = 1,
}

/// 是否支持立即数存储 (原版 kSupportsStoreConst; 按架构两分支)
pub const SUPPORTS_STORE_CONST: bool = true;
/// 不支持立即数存储的架构登记 (源内另一分支取值)
pub const SUPPORTS_STORE_CONST_ARCH_BRANCH: bool = false;

// —— 提取不变式断言 ——
const _: () = {
  assert!(STACK_SLOT_SIZE == 8);
  assert!(INLINE_LOCAL_KINDS == 16);
  assert!(NO_CACHED_MEM_INDEX == -1);
  assert!(SUPPORTS_STORE_CONST);
};
