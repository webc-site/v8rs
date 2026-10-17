// 由 tool/src/wasm/wasm-init-expr.h.js 从 v8/src/wasm/wasm-init-expr.h 生成; 修订就地改, 复跑覆盖
// WasmInitExpr 操作符枚举 (不依赖 wire bytes 的常量表达式表示)

/// 初始化表达式操作符 (对应原版 WasmInitExpr::Operator, 共 25 种)
/// 与 ConstantExpression 不同, 该表示不依赖模块字节码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InitExprOperator {
  /// kGlobalGet
  GlobalGet = 0,
  /// kI32Const
  I32Const = 1,
  /// kI64Const
  I64Const = 2,
  /// kF32Const
  F32Const = 3,
  /// kF64Const
  F64Const = 4,
  /// kS128Const
  S128Const = 5,
  /// kI32Add
  I32Add = 6,
  /// kI32Sub
  I32Sub = 7,
  /// kI32Mul
  I32Mul = 8,
  /// kI64Add
  I64Add = 9,
  /// kI64Sub
  I64Sub = 10,
  /// kI64Mul
  I64Mul = 11,
  /// kRefNullConst
  RefNullConst = 12,
  /// kRefFuncConst
  RefFuncConst = 13,
  /// kStructNew
  StructNew = 14,
  /// kStructNewDefault
  StructNewDefault = 15,
  /// kStructNewDesc
  StructNewDesc = 16,
  /// kStructNewDefaultDesc
  StructNewDefaultDesc = 17,
  /// kArrayNew
  ArrayNew = 18,
  /// kArrayNewDefault
  ArrayNewDefault = 19,
  /// kArrayNewFixed
  ArrayNewFixed = 20,
  /// kRefI31
  RefI31 = 21,
  /// kStringConst
  StringConst = 22,
  /// kAnyConvertExtern
  AnyConvertExtern = 23,
  /// kExternConvertAny
  ExternConvertAny = 24,
}

impl InitExprOperator {
  /// 操作符总数
  pub const COUNT: usize = 25;

  /// 从数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u8) -> Option<Self> {
    match val {
      0 => Some(Self::GlobalGet),
      1 => Some(Self::I32Const),
      2 => Some(Self::I64Const),
      3 => Some(Self::F32Const),
      4 => Some(Self::F64Const),
      5 => Some(Self::S128Const),
      6 => Some(Self::I32Add),
      7 => Some(Self::I32Sub),
      8 => Some(Self::I32Mul),
      9 => Some(Self::I64Add),
      10 => Some(Self::I64Sub),
      11 => Some(Self::I64Mul),
      12 => Some(Self::RefNullConst),
      13 => Some(Self::RefFuncConst),
      14 => Some(Self::StructNew),
      15 => Some(Self::StructNewDefault),
      16 => Some(Self::StructNewDesc),
      17 => Some(Self::StructNewDefaultDesc),
      18 => Some(Self::ArrayNew),
      19 => Some(Self::ArrayNewDefault),
      20 => Some(Self::ArrayNewFixed),
      21 => Some(Self::RefI31),
      22 => Some(Self::StringConst),
      23 => Some(Self::AnyConvertExtern),
      24 => Some(Self::ExternConvertAny),
      _ => None,
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(InitExprOperator::COUNT == 25);
  assert!(InitExprOperator::GlobalGet as u8 == 0);
  assert!(InitExprOperator::ExternConvertAny as u8 == 24);
};
