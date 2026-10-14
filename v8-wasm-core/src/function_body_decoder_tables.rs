// 由 tool/src/wasm/function-body-decoder-impl.h.js 从 v8/src/wasm/function-body-decoder-impl.h 生成; 修订就地改, 复跑覆盖
// 函数体解码器规则区: load/store 类型表 + Control/Reachability + ATOMIC_OP_LIST

use crate::wasm_opcodes::expr::{I32_LOAD_MEM, I32_STORE_MEM, I64_LOAD_MEM32_U, I64_STORE_MEM32};

/// 解码模式 (对应 enum DecodingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodingMode {
  /// kFunctionBody
  FunctionBody,
  /// kConstantExpression
  ConstantExpression,
}

/// 控制帧种类 (对应 enum ControlKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ControlKind {
  /// kControlIf
  If = 0,
  /// kControlIfElse
  IfElse = 1,
  /// kControlBlock
  Block = 2,
  /// kControlLoop
  Loop = 3,
  /// kControlTry
  Try = 4,
  /// kControlTryTable
  TryTable = 5,
  /// kControlTryCatch
  TryCatch = 6,
  /// kControlTryCatchAll
  TryCatchAll = 7,
}

/// 可达性 (对应 enum Reachability : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Reachability {
  /// kReachable: reachable code (可达代码)
  Reachable = 0,
  /// kSpecOnlyReachable: reachable code in unreachable block (块不可达但验证正常)
  SpecOnlyReachable = 1,
  /// kUnreachable: code unreachable in its own block (多态验证)
  Unreachable = 2,
}

/// load 操作码区间硬编码表 (对应 GetLoadType 内 kLoadTypes, 14 条)
/// 区间 [kExprI32LoadMem, kExprI64LoadMem32U], 索引 = opcode - 起点
#[rustfmt::skip]
pub static LOAD_TYPES: &[&str] = &[
  "I32Load",
  "I64Load",
  "F32Load",
  "F64Load",
  "I32Load8S",
  "I32Load8U",
  "I32Load16S",
  "I32Load16U",
  "I64Load8S",
  "I64Load8U",
  "I64Load16S",
  "I64Load16U",
  "I64Load32S",
  "I64Load32U",
];

/// store 操作码区间硬编码表 (对应 GetStoreType 内 kStoreTypes, 9 条)
/// 区间 [kExprI32StoreMem, kExprI64StoreMem32]
#[rustfmt::skip]
pub static STORE_TYPES: &[&str] = &[
  "I32Store",
  "I64Store",
  "F32Store",
  "F64Store",
  "I32Store8",
  "I32Store16",
  "I64Store8",
  "I64Store16",
  "I64Store32",
];

/// load 区间端点 (kExpr 编码, 引用 wasm_opcodes)
pub const LOAD_MIN_OPCODE: u32 = I32_LOAD_MEM.as_u32();
pub const LOAD_MAX_OPCODE: u32 = I64_LOAD_MEM32_U.as_u32();
/// store 区间端点
pub const STORE_MIN_OPCODE: u32 = I32_STORE_MEM.as_u32();
pub const STORE_MAX_OPCODE: u32 = I64_STORE_MEM32.as_u32();

/// 对应 GetLoadType(opcode): 区间查表
#[inline]
#[must_use]
pub fn get_load_type(opcode: u32) -> Option<&'static str> {
  if !(LOAD_MIN_OPCODE..=LOAD_MAX_OPCODE).contains(&opcode) {
    return None;
  }
  LOAD_TYPES.get((opcode - LOAD_MIN_OPCODE) as usize).copied()
}

/// 对应 GetStoreType(opcode): 区间查表
#[inline]
#[must_use]
pub fn get_store_type(opcode: u32) -> Option<&'static str> {
  if !(STORE_MIN_OPCODE..=STORE_MAX_OPCODE).contains(&opcode) {
    return None;
  }
  STORE_TYPES
    .get((opcode - STORE_MIN_OPCODE) as usize)
    .copied()
}

/// ATOMIC_OP_LIST 展开 (V(Name, ArgType), 59 条)
#[rustfmt::skip]
pub static ATOMIC_OP_LIST: &[(&str, &str)] = &[
  ("AtomicNotify", "Uint32"),
  ("I32AtomicWait", "Uint32"),
  ("I64AtomicWait", "Uint64"),
  ("I32AtomicLoad", "Uint32"),
  ("I64AtomicLoad", "Uint64"),
  ("I32AtomicLoad8U", "Uint8"),
  ("I32AtomicLoad16U", "Uint16"),
  ("I64AtomicLoad8U", "Uint8"),
  ("I64AtomicLoad16U", "Uint16"),
  ("I64AtomicLoad32U", "Uint32"),
  ("I32AtomicAdd", "Uint32"),
  ("I32AtomicAdd8U", "Uint8"),
  ("I32AtomicAdd16U", "Uint16"),
  ("I64AtomicAdd", "Uint64"),
  ("I64AtomicAdd8U", "Uint8"),
  ("I64AtomicAdd16U", "Uint16"),
  ("I64AtomicAdd32U", "Uint32"),
  ("I32AtomicSub", "Uint32"),
  ("I64AtomicSub", "Uint64"),
  ("I32AtomicSub8U", "Uint8"),
  ("I32AtomicSub16U", "Uint16"),
  ("I64AtomicSub8U", "Uint8"),
  ("I64AtomicSub16U", "Uint16"),
  ("I64AtomicSub32U", "Uint32"),
  ("I32AtomicAnd", "Uint32"),
  ("I64AtomicAnd", "Uint64"),
  ("I32AtomicAnd8U", "Uint8"),
  ("I32AtomicAnd16U", "Uint16"),
  ("I64AtomicAnd8U", "Uint8"),
  ("I64AtomicAnd16U", "Uint16"),
  ("I64AtomicAnd32U", "Uint32"),
  ("I32AtomicOr", "Uint32"),
  ("I64AtomicOr", "Uint64"),
  ("I32AtomicOr8U", "Uint8"),
  ("I32AtomicOr16U", "Uint16"),
  ("I64AtomicOr8U", "Uint8"),
  ("I64AtomicOr16U", "Uint16"),
  ("I64AtomicOr32U", "Uint32"),
  ("I32AtomicXor", "Uint32"),
  ("I64AtomicXor", "Uint64"),
  ("I32AtomicXor8U", "Uint8"),
  ("I32AtomicXor16U", "Uint16"),
  ("I64AtomicXor8U", "Uint8"),
  ("I64AtomicXor16U", "Uint16"),
  ("I64AtomicXor32U", "Uint32"),
  ("I32AtomicExchange", "Uint32"),
  ("I64AtomicExchange", "Uint64"),
  ("I32AtomicExchange8U", "Uint8"),
  ("I32AtomicExchange16U", "Uint16"),
  ("I64AtomicExchange8U", "Uint8"),
  ("I64AtomicExchange16U", "Uint16"),
  ("I64AtomicExchange32U", "Uint32"),
  ("I32AtomicCompareExchange", "Uint32"),
  ("I64AtomicCompareExchange", "Uint64"),
  ("I32AtomicCompareExchange8U", "Uint8"),
  ("I32AtomicCompareExchange16U", "Uint16"),
  ("I64AtomicCompareExchange8U", "Uint8"),
  ("I64AtomicCompareExchange16U", "Uint16"),
  ("I64AtomicCompareExchange32U", "Uint32"),
];

// —— 提取不变式断言 ——
const _: () = {
  use crate::table_eq::str_eq;
  assert!(LOAD_TYPES.len() == 14);
  assert!(STORE_TYPES.len() == 9);
  assert!(LOAD_MAX_OPCODE - LOAD_MIN_OPCODE + 1 == 14);
  assert!(STORE_MAX_OPCODE - STORE_MIN_OPCODE + 1 == 9);
  assert!(ATOMIC_OP_LIST.len() == 59);
  assert!(str_eq(ATOMIC_OP_LIST[0].0, "AtomicNotify"));
  assert!(ControlKind::If as u8 == 0);
  assert!(ControlKind::TryCatchAll as u8 == 7);
  assert!(Reachability::Reachable as u8 == 0);
  assert!(Reachability::Unreachable as u8 == 2);
  assert!(str_eq(LOAD_TYPES[0], "I32Load"));
  assert!(str_eq(LOAD_TYPES[13], "I64Load32U"));
  assert!(str_eq(STORE_TYPES[0], "I32Store"));
};
