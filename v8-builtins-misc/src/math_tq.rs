// 由 tool/src/builtins/math.tq.js 从 v8/src/builtins/math.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 83 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "math", "namespace math"),
  (
    "macro",
    "ReduceToSmiOrFloat64",
    "macro ReduceToSmiOrFloat64(implicit context: Context)(x: JSAny): never labels SmiResult(Smi), Float64Result(float64)",
  ),
  (
    "macro",
    "IsIntPtrAbsWithOverflowSupported",
    "macro IsIntPtrAbsWithOverflowSupported(): constexpr bool",
  ),
  (
    "macro",
    "TrySmiAdd",
    "macro TrySmiAdd(Smi, Smi): Smi labels Overflow",
  ),
  (
    "macro",
    "TrySmiSub",
    "macro TrySmiSub(Smi, Smi): Smi labels Overflow",
  ),
  (
    "macro",
    "TrySmiAbs",
    "macro TrySmiAbs(Smi): Smi labels Overflow",
  ),
  ("macro", "Float64Abs", "macro Float64Abs(float64): float64"),
  (
    "const",
    "kSmiMaxValuePlusOne",
    "const kSmiMaxValuePlusOne: constexpr float64 generates '0.0 - kSmiMinValue'",
  ),
  (
    "builtin",
    "MathAbs",
    "builtin MathAbs( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Ceil",
    "macro Float64Ceil(float64): float64",
  ),
  (
    "builtin",
    "MathCeil",
    "builtin MathCeil( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Floor",
    "macro Float64Floor(float64): float64",
  ),
  (
    "builtin",
    "MathFloor",
    "builtin MathFloor( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Round",
    "macro Float64Round(float64): float64",
  ),
  (
    "builtin",
    "MathRound",
    "builtin MathRound( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Trunc",
    "macro Float64Trunc(float64): float64",
  ),
  (
    "builtin",
    "MathTrunc",
    "builtin MathTrunc( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Pow",
    "macro Float64Pow(float64, float64): float64",
  ),
  (
    "macro",
    "TruncateTaggedToFloat64",
    "macro TruncateTaggedToFloat64(implicit context: Context)(JSAny): float64",
  ),
  (
    "macro",
    "MathPowImpl",
    "macro MathPowImpl(implicit context: Context)(base: JSAny, exponent: JSAny): Number",
  ),
  (
    "builtin",
    "MathPow",
    "builtin MathPow( js-implicit context: NativeContext)(base: JSAny, exponent: JSAny): Number",
  ),
  (
    "macro",
    "Float64Max",
    "macro Float64Max(float64, float64): float64",
  ),
  (
    "builtin",
    "MathMax",
    "builtin MathMax( js-implicit context: NativeContext)(...arguments): Number",
  ),
  (
    "macro",
    "Float64Min",
    "macro Float64Min(float64, float64): float64",
  ),
  (
    "builtin",
    "MathMin",
    "builtin MathMin( js-implicit context: NativeContext)(...arguments): Number",
  ),
  (
    "macro",
    "Float64Acos",
    "macro Float64Acos(float64): float64",
  ),
  (
    "builtin",
    "MathAcos",
    "builtin MathAcos( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Acosh",
    "macro Float64Acosh(float64): float64",
  ),
  (
    "builtin",
    "MathAcosh",
    "builtin MathAcosh( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Asin",
    "macro Float64Asin(float64): float64",
  ),
  (
    "builtin",
    "MathAsin",
    "builtin MathAsin( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Asinh",
    "macro Float64Asinh(float64): float64",
  ),
  (
    "builtin",
    "MathAsinh",
    "builtin MathAsinh( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Atan",
    "macro Float64Atan(float64): float64",
  ),
  (
    "builtin",
    "MathAtan",
    "builtin MathAtan( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Atan2",
    "macro Float64Atan2(float64, float64): float64",
  ),
  (
    "builtin",
    "MathAtan2",
    "builtin MathAtan2( js-implicit context: NativeContext)(y: JSAny, x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Atanh",
    "macro Float64Atanh(float64): float64",
  ),
  (
    "builtin",
    "MathAtanh",
    "builtin MathAtanh( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Cbrt",
    "macro Float64Cbrt(float64): float64",
  ),
  (
    "builtin",
    "MathCbrt",
    "builtin MathCbrt( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  ("macro", "Word32Clz", "macro Word32Clz(int32): int32"),
  (
    "builtin",
    "MathClz32",
    "builtin MathClz32( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  ("macro", "Float64Cos", "macro Float64Cos(float64): float64"),
  (
    "builtin",
    "MathCos",
    "builtin MathCos( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Cosh",
    "macro Float64Cosh(float64): float64",
  ),
  (
    "builtin",
    "MathCosh",
    "builtin MathCosh( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  ("macro", "Float64Exp", "macro Float64Exp(float64): float64"),
  (
    "builtin",
    "MathExp",
    "builtin MathExp( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Expm1",
    "macro Float64Expm1(float64): float64",
  ),
  (
    "builtin",
    "MathExpm1",
    "builtin MathExpm1( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "builtin",
    "MathFround",
    "builtin MathFround( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "builtin",
    "MathF16round",
    "builtin MathF16round( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "builtin",
    "MathImul",
    "builtin MathImul( js-implicit context: NativeContext)(x: JSAny, y: JSAny): Number",
  ),
  ("macro", "Float64Log", "macro Float64Log(float64): float64"),
  (
    "builtin",
    "MathLog",
    "builtin MathLog( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Log1p",
    "macro Float64Log1p(float64): float64",
  ),
  (
    "builtin",
    "MathLog1p",
    "builtin MathLog1p( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Log10",
    "macro Float64Log10(float64): float64",
  ),
  (
    "builtin",
    "MathLog10",
    "builtin MathLog10( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Log2",
    "macro Float64Log2(float64): float64",
  ),
  (
    "builtin",
    "MathLog2",
    "builtin MathLog2( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  ("macro", "Float64Sin", "macro Float64Sin(float64): float64"),
  (
    "builtin",
    "MathSin",
    "builtin MathSin( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "builtin",
    "MathSign",
    "builtin MathSign( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Sinh",
    "macro Float64Sinh(float64): float64",
  ),
  (
    "builtin",
    "MathSinh",
    "builtin MathSinh( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Sqrt",
    "macro Float64Sqrt(float64): float64",
  ),
  (
    "builtin",
    "MathSqrt",
    "builtin MathSqrt( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  ("macro", "Float64Tan", "macro Float64Tan(float64): float64"),
  (
    "builtin",
    "MathTan",
    "builtin MathTan( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "Float64Tanh",
    "macro Float64Tanh(float64): float64",
  ),
  (
    "builtin",
    "MathTanh",
    "builtin MathTanh( js-implicit context: NativeContext)(x: JSAny): Number",
  ),
  (
    "macro",
    "FastMathHypot",
    "macro FastMathHypot( implicit context: Context)(arguments: Arguments): Number labels Slow",
  ),
  (
    "builtin",
    "MathHypot",
    "builtin MathHypot( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): Number",
  ),
  ("struct", "XorShift128State", "struct XorShift128State"),
  (
    "macro",
    "LoadMathRandomState",
    "macro LoadMathRandomState(ByteArray): XorShift128State",
  ),
  (
    "macro",
    "StoreMathRandomState",
    "macro StoreMathRandomState(ByteArray, XorShift128State): void",
  ),
  (
    "macro",
    "InitializeAndMaybeRefillMathRandom",
    "macro InitializeAndMaybeRefillMathRandom(NativeContext): Smi",
  ),
  ("struct", "XorShift128Result", "struct XorShift128Result"),
  (
    "macro",
    "RandomToDouble",
    "macro RandomToDouble(random: uint64): float64",
  ),
  (
    "macro",
    "XorShift128",
    "macro XorShift128(s0In: uint64, s1In: uint64): XorShift128Result",
  ),
  (
    "builtin",
    "MathRandom",
    "builtin MathRandom( js-implicit context: NativeContext, receiver: JSAny)(): Number",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[("kSmiMaxValuePlusOne", "0.0 - kSmiMinValue")];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 83);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 43);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 36);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 2);
};
