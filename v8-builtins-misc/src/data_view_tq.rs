// 由 tool/src/builtins/data-view.tq.js 从 v8/src/builtins/data-view.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 65 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "data_view", "namespace data_view"),
  (
    "const",
    "kBuiltinNameByteLength",
    "const kBuiltinNameByteLength: constexpr string = 'get DataView.prototype.byteLength'",
  ),
  (
    "const",
    "kBuiltinNameByteOffset",
    "const kBuiltinNameByteOffset: constexpr string = 'get DataView.prototype.byteOffset'",
  ),
  (
    "macro",
    "MakeDataViewGetterNameString",
    "macro MakeDataViewGetterNameString(kind: constexpr ElementsKind): String",
  ),
  (
    "macro",
    "MakeDataViewSetterNameString",
    "macro MakeDataViewSetterNameString(kind: constexpr ElementsKind): String",
  ),
  (
    "macro",
    "WasDetached",
    "macro WasDetached(view: JSArrayBufferView): bool",
  ),
  (
    "macro",
    "ValidateDataView",
    "macro ValidateDataView(context: Context, o: JSAny, method: String): JSDataViewOrRabGsabDataView",
  ),
  (
    "builtin",
    "DataViewPrototypeGetBuffer",
    "builtin DataViewPrototypeGetBuffer( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): JSArrayBuffer",
  ),
  (
    "builtin",
    "DataViewPrototypeGetByteLength",
    "builtin DataViewPrototypeGetByteLength( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): Number",
  ),
  (
    "builtin",
    "DataViewPrototypeGetByteOffset",
    "builtin DataViewPrototypeGetByteOffset( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): Number",
  ),
  (
    "macro",
    "BitcastInt32ToFloat32",
    "macro BitcastInt32ToFloat32(uint32): float32",
  ),
  (
    "macro",
    "BitcastFloat32ToInt32",
    "macro BitcastFloat32ToInt32(float32): uint32",
  ),
  (
    "macro",
    "Float64ExtractLowWord32",
    "macro Float64ExtractLowWord32(float64): uint32",
  ),
  (
    "macro",
    "Float64ExtractHighWord32",
    "macro Float64ExtractHighWord32(float64): uint32",
  ),
  (
    "macro",
    "Float64InsertLowWord32",
    "macro Float64InsertLowWord32(float64, uint32): float64",
  ),
  (
    "macro",
    "Float64InsertHighWord32",
    "macro Float64InsertHighWord32(float64, uint32): float64",
  ),
  (
    "macro",
    "DataViewBuiltinsAssembler::LoadUint8",
    "macro DataViewBuiltinsAssembler::LoadUint8(RawPtr, uintptr): uint32",
  ),
  (
    "macro",
    "DataViewBuiltinsAssembler::LoadInt8",
    "macro DataViewBuiltinsAssembler::LoadInt8(RawPtr, uintptr): int32",
  ),
  (
    "macro",
    "LoadDataView8",
    "macro LoadDataView8( buffer: JSArrayBuffer, offset: uintptr, signed: constexpr bool): Smi",
  ),
  (
    "macro",
    "LoadDataView16",
    "macro LoadDataView16( buffer: JSArrayBuffer, offset: uintptr, requestedLittleEndian: bool, signed: constexpr bool): Number",
  ),
  (
    "macro",
    "LoadDataView32",
    "macro LoadDataView32( buffer: JSArrayBuffer, offset: uintptr, requestedLittleEndian: bool, kind: constexpr ElementsKind): Number",
  ),
  (
    "macro",
    "LoadDataViewFloat16",
    "macro LoadDataViewFloat16( buffer: JSArrayBuffer, offset: uintptr, requestedLittleEndian: bool): Number",
  ),
  (
    "macro",
    "LoadDataViewFloat64",
    "macro LoadDataViewFloat64( buffer: JSArrayBuffer, offset: uintptr, requestedLittleEndian: bool): Number",
  ),
  (
    "const",
    "kZeroDigitBigInt",
    "const kZeroDigitBigInt: constexpr int31 = 0",
  ),
  (
    "const",
    "kOneDigitBigInt",
    "const kOneDigitBigInt: constexpr int31 = 1",
  ),
  (
    "const",
    "kTwoDigitBigInt",
    "const kTwoDigitBigInt: constexpr int31 = 2",
  ),
  (
    "macro",
    "MakeBigIntOn64Bit",
    "macro MakeBigIntOn64Bit( implicit context: Context)(lowWord: uint32, highWord: uint32, signed: constexpr bool): BigInt",
  ),
  (
    "macro",
    "MakeBigIntOn32Bit",
    "macro MakeBigIntOn32Bit( implicit context: Context)(lowWord: uint32, highWord: uint32, signed: constexpr bool): BigInt",
  ),
  (
    "macro",
    "MakeBigInt",
    "macro MakeBigInt( implicit context: Context)(lowWord: uint32, highWord: uint32, signed: constexpr bool): BigInt",
  ),
  (
    "macro",
    "LoadDataViewBigInt",
    "macro LoadDataViewBigInt( implicit context: Context)(buffer: JSArrayBuffer, offset: uintptr, requestedLittleEndian: bool, signed: constexpr bool): BigInt",
  ),
  (
    "macro",
    "DataViewBuiltinsAssembler::DataViewElementSize",
    "macro DataViewBuiltinsAssembler::DataViewElementSize( constexpr ElementsKind): constexpr int31",
  ),
  (
    "macro",
    "DataViewGet",
    "macro DataViewGet( context: Context, receiver: JSAny, requestIndex: JSAny, requestedLittleEndian: JSAny, kind: constexpr ElementsKind): Numeric",
  ),
  (
    "builtin",
    "DataViewPrototypeGetUint8",
    "builtin DataViewPrototypeGetUint8( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeGetInt8",
    "builtin DataViewPrototypeGetInt8( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeGetUint16",
    "builtin DataViewPrototypeGetUint16( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeGetInt16",
    "builtin DataViewPrototypeGetInt16( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeGetUint32",
    "builtin DataViewPrototypeGetUint32( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeGetInt32",
    "builtin DataViewPrototypeGetInt32( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeGetFloat16",
    "builtin DataViewPrototypeGetFloat16( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeGetFloat32",
    "builtin DataViewPrototypeGetFloat32( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeGetFloat64",
    "builtin DataViewPrototypeGetFloat64( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeGetBigUint64",
    "builtin DataViewPrototypeGetBigUint64( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeGetBigInt64",
    "builtin DataViewPrototypeGetBigInt64( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "macro",
    "ToNumber",
    "macro ToNumber(Context, JSAny): Number",
  ),
  (
    "macro",
    "TruncateFloat64ToWord32",
    "macro TruncateFloat64ToWord32(float64): uint32",
  ),
  (
    "macro",
    "DataViewBuiltinsAssembler::StoreWord8",
    "macro DataViewBuiltinsAssembler::StoreWord8(RawPtr, uintptr, uint32): void",
  ),
  (
    "macro",
    "StoreDataView8",
    "macro StoreDataView8(buffer: JSArrayBuffer, offset: uintptr, value: uint32): void",
  ),
  (
    "macro",
    "StoreDataView16",
    "macro StoreDataView16( buffer: JSArrayBuffer, offset: uintptr, value: uint32, requestedLittleEndian: bool): void",
  ),
  (
    "macro",
    "StoreDataView32",
    "macro StoreDataView32( buffer: JSArrayBuffer, offset: uintptr, value: uint32, requestedLittleEndian: bool): void",
  ),
  (
    "macro",
    "StoreDataView64",
    "macro StoreDataView64( buffer: JSArrayBuffer, offset: uintptr, lowWord: uint32, highWord: uint32, requestedLittleEndian: bool): void",
  ),
  (
    "macro",
    "DataViewBuiltinsAssembler::DataViewDecodeBigIntLength",
    "macro DataViewBuiltinsAssembler::DataViewDecodeBigIntLength(BigIntBase): uint32",
  ),
  (
    "macro",
    "DataViewBuiltinsAssembler::DataViewDecodeBigIntSign",
    "macro DataViewBuiltinsAssembler::DataViewDecodeBigIntSign(BigIntBase): uint32",
  ),
  (
    "macro",
    "StoreDataViewBigInt",
    "macro StoreDataViewBigInt( buffer: JSArrayBuffer, offset: uintptr, bigIntValue: BigInt, requestedLittleEndian: bool): void",
  ),
  (
    "macro",
    "DataViewSet",
    "macro DataViewSet( context: Context, receiver: JSAny, requestIndex: JSAny, value: JSAny, requestedLittleEndian: JSAny, kind: constexpr ElementsKind): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetUint8",
    "builtin DataViewPrototypeSetUint8( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetInt8",
    "builtin DataViewPrototypeSetInt8( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetUint16",
    "builtin DataViewPrototypeSetUint16( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetInt16",
    "builtin DataViewPrototypeSetInt16( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetUint32",
    "builtin DataViewPrototypeSetUint32( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetInt32",
    "builtin DataViewPrototypeSetInt32( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetFloat16",
    "builtin DataViewPrototypeSetFloat16( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetFloat32",
    "builtin DataViewPrototypeSetFloat32( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetFloat64",
    "builtin DataViewPrototypeSetFloat64( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetBigUint64",
    "builtin DataViewPrototypeSetBigUint64( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "DataViewPrototypeSetBigInt64",
    "builtin DataViewPrototypeSetBigInt64( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const BUILTIN_NAME_BYTE_LENGTH: &'static str = "get DataView.prototype.byteLength"; // 原版: kBuiltinNameByteLength
pub const BUILTIN_NAME_BYTE_OFFSET: &'static str = "get DataView.prototype.byteOffset"; // 原版: kBuiltinNameByteOffset
pub const ZERO_DIGIT_BIG_INT: i64 = 0; // 原版: kZeroDigitBigInt
pub const ONE_DIGIT_BIG_INT: i64 = 1; // 原版: kOneDigitBigInt
pub const TWO_DIGIT_BIG_INT: i64 = 2; // 原版: kTwoDigitBigInt

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 65);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 5);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 34);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 25);
};
