// 由 tool/src/wasm/wasm-module.h.js 从 v8/src/wasm/wasm-module.h 生成; 修订就地改, 复跑覆盖
// WasmModule 相关枚举与哨兵常量

/// 内存地址类型 (对应原版 enum class AddressType : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AddressType {
  /// kI32
  I32 = 0,
  /// kI64
  I64 = 1,
}

impl AddressType {
  /// 原版 AddressTypeToStr
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
      Self::I32 => "i32",
      Self::I64 => "i64",
    }
  }
}

/// 边界检查策略 (对应原版 enum BoundsCheckStrategy : int8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum BoundsCheckStrategy {
  /// kTrapHandler — Emit trapping instructions, use the trap handler for OOB detection.
  TrapHandler = 0,
  /// kExplicitBoundsChecks — Emit explicit bounds checks.
  ExplicitBoundsChecks = 1,
  /// kNoBoundsChecks — Emit no bounds checks at all (for testing only).
  NoBoundsChecks = 2,
}

/// 元素段状态 (对应原版 WasmElemSegment::Status)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElemSegmentStatus {
  /// kStatusActive
  Active = 0,
  /// kStatusPassive — 实例化时自动复制
  Passive = 1,
  /// kStatusDeclarative — 纯声明式, 永不复制; 实例化后显式复制
  Declarative = 2,
}

/// 元素段元素形态 (对应原版 WasmElemSegment::ElementType)
/// variant 采 rust 惯例命名; 原版 kFoo 拼写以各条 doc 锚 (kFunctionIndexElements/kExpressionElements) 保留可追溯
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElementType {
  /// v8: kFunctionIndexElements
  FunctionIndex = 0,
  /// v8: kExpressionElements
  Expression = 1,
}

/// 自适应映射最大键 (对应原版 AdaptiveMap::kMaxKey; index+1 不得溢出)
pub const ADAPTIVE_MAP_MAX_KEY: u32 = 10000000;

/// 自适应映射装填因子 (对应原版 AdaptiveMap::kLoadFactor)
pub const ADAPTIVE_MAP_LOAD_FACTOR: u32 = 4;

/// 类型定义类别 (对应原版 TypeDefinition::Kind; 数值取自 RefTypeKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum TypeDefinitionKind {
  /// kFunction
  Function = 1,
  /// kStruct
  Struct = 2,
  /// kArray
  Array = 3,
  /// kCont
  Cont = 4,
}

/// 调试符号类别总数 (对应原版 WasmDebugSymbols::kNumTypes)
pub const DEBUG_SYMBOLS_NUM_TYPES: i32 = 3;

/// 调试符号类别 (对应原版 WasmDebugSymbols::Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DebugSymbolsType {
  /// SourceMap
  SourceMap = 0,
  /// EmbeddedDWARF
  EmbeddedDwarf = 1,
  /// ExternalDWARF
  ExternalDwarf = 2,
  /// None
  None = 3,
}

/// 未初始化的 Liftoff 帧大小哨兵 (原版 kUninitializedLiftoffFrameSize)
pub const UNINITIALIZED_LIFTOFF_FRAME_SIZE: u32 = 1;

/// call_ref 直接调用哨兵函数索引 (原版 kCallRef)
pub const CALL_REF_SENTINEL: u32 = 4294967295;

/// call_indirect 直接调用哨兵函数索引 (原版 kCallIndirect = kCallRef - 1)
pub const CALL_INDIRECT_SENTINEL: u32 = 4294967294;

// —— 提取不变式断言 ——
const _: () = {
  assert!(ADAPTIVE_MAP_MAX_KEY == 10_000_000);
  assert!(ADAPTIVE_MAP_LOAD_FACTOR == 4);
  assert!(CALL_REF_SENTINEL == 0xFFFF_FFFF);
  assert!(CALL_INDIRECT_SENTINEL == CALL_REF_SENTINEL - 1);
  assert!(UNINITIALIZED_LIFTOFF_FRAME_SIZE == 1);
  assert!(TypeDefinitionKind::Function as i8 == 1);
  assert!(TypeDefinitionKind::Cont as i8 == 4);
};
