// 由 tool/src/wasm/value-type.cc.js 从 v8/src/wasm/value-type.{h,cc} 生成; 修订就地改, 复跑覆盖
// ValueType 名称表 + null-ref 别名表 + CanonicalSig 骨架 (头侧枚举在 value_type.rs)

use crate::{
  value_type::{GenericKind, NumericKind},
  wasm_constants::ValueTypeCode,
};

/// 数值类型名表 (对应 FOREACH_NUMERIC_VALUE_TYPE: kind/log2/code/machine/short/name)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericTypeInfo {
  pub kind: NumericKind,
  /// log2Size
  pub log2_size: i8,
  /// ValueTypeCode 记号 (kI32Code 等)
  pub code: ValueTypeCode,
  /// MachineType 记号 (Int32 等, 仅存档)
  pub machine: &'static str,
  /// shortName 字符
  pub short_name: char,
  /// 打印名 ("i32"/"v128" 等)
  pub name: &'static str,
}

/// FOREACH_NUMERIC_VALUE_TYPE 展开表 (源顺序, 8 条)
#[rustfmt::skip]
pub static NUMERIC_TYPE_INFOS: &[NumericTypeInfo] = &[
  NumericTypeInfo { kind: NumericKind::I32, log2_size: 2, code: ValueTypeCode::I32, machine: "Int32", short_name: 'i', name: "i32" },
  NumericTypeInfo { kind: NumericKind::I64, log2_size: 3, code: ValueTypeCode::I64, machine: "Int64", short_name: 'l', name: "i64" },
  NumericTypeInfo { kind: NumericKind::F32, log2_size: 2, code: ValueTypeCode::F32, machine: "Float32", short_name: 'f', name: "f32" },
  NumericTypeInfo { kind: NumericKind::F64, log2_size: 3, code: ValueTypeCode::F64, machine: "Float64", short_name: 'd', name: "f64" },
  NumericTypeInfo { kind: NumericKind::S128, log2_size: 4, code: ValueTypeCode::S128, machine: "Simd128", short_name: 's', name: "v128" },
  NumericTypeInfo { kind: NumericKind::I8, log2_size: 0, code: ValueTypeCode::I8, machine: "Int8", short_name: 'b', name: "i8" },
  NumericTypeInfo { kind: NumericKind::I16, log2_size: 1, code: ValueTypeCode::I16, machine: "Int16", short_name: 'h', name: "i16" },
  NumericTypeInfo { kind: NumericKind::F16, log2_size: 1, code: ValueTypeCode::F16, machine: "Float16", short_name: 'p', name: "f16" },
];

/// 抽象类型名表 (对应 FOREACH_ABSTRACT_TYPE 展开: NONE+TOP+自有, 20 条)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbstractTypeInfo {
  pub kind: GenericKind,
  /// ValueTypeCode 记号
  pub code: &'static str,
  /// 位组合记号 (REF/FUNC/STRUCT/ARRAY/CONT, 仅存档)
  pub extra_bits: &'static str,
  /// 打印名
  pub name: &'static str,
}

#[rustfmt::skip]
pub static ABSTRACT_TYPE_INFOS: &[AbstractTypeInfo] = &[
  AbstractTypeInfo { kind: GenericKind::NoCont, code: "NoCont", extra_bits: "REF|CONT", name: "nocont" },
  AbstractTypeInfo { kind: GenericKind::NoExn, code: "NoExn", extra_bits: "REF", name: "noexn" },
  AbstractTypeInfo { kind: GenericKind::NoExtern, code: "NoExtern", extra_bits: "REF", name: "noextern" },
  AbstractTypeInfo { kind: GenericKind::NoFunc, code: "NoFunc", extra_bits: "REF|FUNC", name: "nofunc" },
  AbstractTypeInfo { kind: GenericKind::NoWaitqueue, code: "NoWaitqueue", extra_bits: "REF", name: "nowaitqueue" },
  AbstractTypeInfo { kind: GenericKind::None, code: "None", extra_bits: "REF", name: "none" },
  AbstractTypeInfo { kind: GenericKind::Func, code: "FuncRef", extra_bits: "REF|FUNC", name: "func" },
  AbstractTypeInfo { kind: GenericKind::Any, code: "AnyRef", extra_bits: "REF", name: "any" },
  AbstractTypeInfo { kind: GenericKind::Extern, code: "ExternRef", extra_bits: "REF", name: "extern" },
  AbstractTypeInfo { kind: GenericKind::Exn, code: "ExnRef", extra_bits: "REF", name: "exn" },
  AbstractTypeInfo { kind: GenericKind::Cont, code: "ContRef", extra_bits: "REF|CONT", name: "cont" },
  AbstractTypeInfo { kind: GenericKind::Waitqueue, code: "WaitqueueRef", extra_bits: "REF", name: "waitqueue" },
  AbstractTypeInfo { kind: GenericKind::Eq, code: "EqRef", extra_bits: "REF", name: "eq" },
  AbstractTypeInfo { kind: GenericKind::I31, code: "I31Ref", extra_bits: "REF", name: "i31" },
  AbstractTypeInfo { kind: GenericKind::Struct, code: "StructRef", extra_bits: "REF|STRUCT", name: "struct" },
  AbstractTypeInfo { kind: GenericKind::Array, code: "ArrayRef", extra_bits: "REF|ARRAY", name: "array" },
  AbstractTypeInfo { kind: GenericKind::String, code: "StringRef", extra_bits: "REF", name: "string" },
  AbstractTypeInfo { kind: GenericKind::StringViewWtf8, code: "StringViewWtf8", extra_bits: "REF", name: "stringview_wtf8" },
  AbstractTypeInfo { kind: GenericKind::StringViewWtf16, code: "StringViewWtf16", extra_bits: "REF", name: "stringview_wtf16" },
  AbstractTypeInfo { kind: GenericKind::StringViewIter, code: "StringViewIter", extra_bits: "REF", name: "stringview_iter" },
];

/// 内部类型名表 (对应 FOREACH_INTERNAL_TYPE, 4 条)
#[rustfmt::skip]
pub static INTERNAL_TYPE_INFOS: &[(&str, &str)] = &[
  ("Void", "<void>"),
  ("Top", "<top>"),
  ("Bottom", "<bot>"),
  ("ExternString", "<extern_string>"),
];

/// null-ref 系别名表 (对应 PrintGenericHeaptypeName / name() 特判: kNone 等可空类型打印为 nullXXref)
/// (generic, 共享前缀打印名, 非共享打印名)
pub static NULL_REF_NAMES: &[(GenericKind, &str, &str)] = &[
  (GenericKind::None, "(shared nullref)", "nullref"),
  (GenericKind::NoExn, "(shared nullexnref)", "nullexnref"),
  (
    GenericKind::NoExtern,
    "(shared nullexternref)",
    "nullexternref",
  ),
  (GenericKind::NoFunc, "(shared nullfuncref)", "nullfuncref"),
  (GenericKind::NoCont, "(shared nullcontref)", "nullcontref"),
];

/// 生成器侧对数: null-ref 表与抽象表一致性
const _: () = {
  use crate::table_eq::str_eq;
  assert!(NUMERIC_TYPE_INFOS.len() == 8);
  assert!(ABSTRACT_TYPE_INFOS.len() == 20);
  assert!(INTERNAL_TYPE_INFOS.len() == 4);
  assert!(NULL_REF_NAMES.len() == 5);
  assert!(str_eq(NUMERIC_TYPE_INFOS[0].name, "i32"));
  assert!(str_eq(NUMERIC_TYPE_INFOS[4].name, "v128"));
  assert!(str_eq(NUMERIC_TYPE_INFOS[7].name, "f16"));
  assert!(str_eq(ABSTRACT_TYPE_INFOS[0].name, "nocont"));
  assert!(str_eq(ABSTRACT_TYPE_INFOS[6].name, "func"));
  assert!(str_eq(ABSTRACT_TYPE_INFOS[19].name, "stringview_iter"));
  assert!(str_eq(INTERNAL_TYPE_INFOS[0].0, "Void"));
};

// 深水推迟 (readme/zh.md 台账): WasmReturnTypeFromSignature / EquivalentNumericSig /
// GetI32Sig 依赖 WasmEngine 与 TypeCanonicalizer 状态面 (canonical-types 语义轮), 骨架不入挂载面。
