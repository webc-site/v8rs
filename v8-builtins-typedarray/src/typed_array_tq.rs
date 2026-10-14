// 由 tool/src/builtins/typed-array.tq.js 从 v8/src/builtins/typed-array.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-typedarray; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 53 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "typed_array", "namespace typed_array"),
  (
    "type",
    "Uint8Elements",
    "type Uint8Elements extends ElementsKind",
  ),
  (
    "type",
    "Int8Elements",
    "type Int8Elements extends ElementsKind",
  ),
  (
    "type",
    "Uint16Elements",
    "type Uint16Elements extends ElementsKind",
  ),
  (
    "type",
    "Int16Elements",
    "type Int16Elements extends ElementsKind",
  ),
  (
    "type",
    "Uint32Elements",
    "type Uint32Elements extends ElementsKind",
  ),
  (
    "type",
    "Int32Elements",
    "type Int32Elements extends ElementsKind",
  ),
  (
    "type",
    "Float16Elements",
    "type Float16Elements extends ElementsKind",
  ),
  (
    "type",
    "Float32Elements",
    "type Float32Elements extends ElementsKind",
  ),
  (
    "type",
    "Float64Elements",
    "type Float64Elements extends ElementsKind",
  ),
  (
    "type",
    "Uint8ClampedElements",
    "type Uint8ClampedElements extends ElementsKind",
  ),
  (
    "type",
    "BigUint64Elements",
    "type BigUint64Elements extends ElementsKind",
  ),
  (
    "type",
    "BigInt64Elements",
    "type BigInt64Elements extends ElementsKind",
  ),
  (
    "type",
    "RabGsabUint8Elements",
    "type RabGsabUint8Elements extends ElementsKind",
  ),
  (
    "struct",
    "TypedArrayElementsInfo",
    "struct TypedArrayElementsInfo",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::CallCMemcpy",
    "macro TypedArrayBuiltinsAssembler::CallCMemcpy(RawPtr, RawPtr, uintptr): void",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::CallCMemmove",
    "macro TypedArrayBuiltinsAssembler::CallCMemmove( RawPtr, RawPtr, uintptr): void",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::CallCMemset",
    "macro TypedArrayBuiltinsAssembler::CallCMemset(RawPtr, intptr, uintptr): void",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::CallCRelaxedMemcpy",
    "macro TypedArrayBuiltinsAssembler::CallCRelaxedMemcpy( RawPtr, RawPtr, uintptr): void",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::CallCRelaxedMemmove",
    "macro TypedArrayBuiltinsAssembler::CallCRelaxedMemmove( RawPtr, RawPtr, uintptr): void",
  ),
  (
    "macro",
    "GetTypedArrayBuffer",
    "macro GetTypedArrayBuffer(implicit context: Context)(JSTypedArray): JSArrayBuffer",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::GetTypedArrayElementsInfo",
    "macro TypedArrayBuiltinsAssembler::GetTypedArrayElementsInfo( JSTypedArray): TypedArrayElementsInfo",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::GetTypedArrayElementsInfo",
    "macro TypedArrayBuiltinsAssembler::GetTypedArrayElementsInfo(Map): TypedArrayElementsInfo",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::IsUint8ElementsKind",
    "macro TypedArrayBuiltinsAssembler::IsUint8ElementsKind(ElementsKind): bool",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::IsBigInt64ElementsKind",
    "macro TypedArrayBuiltinsAssembler::IsBigInt64ElementsKind(ElementsKind): bool",
  ),
  (
    "macro",
    "LoadFixedTypedArrayElementAsTagged",
    "macro LoadFixedTypedArrayElementAsTagged( RawPtr, uintptr, constexpr ElementsKind): Numeric",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::StoreJSTypedArrayElementFromNumeric",
    "macro TypedArrayBuiltinsAssembler::StoreJSTypedArrayElementFromNumeric( Context, JSTypedArray, uintptr, Numeric, constexpr ElementsKind): void",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::StoreJSTypedArrayElementFromTagged",
    "macro TypedArrayBuiltinsAssembler::StoreJSTypedArrayElementFromTagged( Context, JSTypedArray, uintptr, JSAny, constexpr ElementsKind): void labels IfDetached",
  ),
  (
    "macro",
    "LoadJSTypedArrayLengthAndValidate",
    "macro LoadJSTypedArrayLengthAndValidate( JSTypedArray, constexpr TypedArrayAccessMode): uintptr labels Fail",
  ),
  (
    "type",
    "LoadNumericFn",
    "type LoadNumericFn = builtin(JSTypedArray, uintptr) => Numeric",
  ),
  (
    "type",
    "StoreNumericFn",
    "type StoreNumericFn = builtin(Context, JSTypedArray, uintptr, Numeric) => Smi",
  ),
  (
    "type",
    "StoreJSAnyFn",
    "type StoreJSAnyFn = builtin(Context, JSTypedArray, uintptr, JSAny) => Smi",
  ),
  ("const", "kStoreSucceded", "const kStoreSucceded: Smi = 0"),
  (
    "const",
    "kStoreFailureArrayDetachedOrImmutableOrOutOfBounds",
    "const kStoreFailureArrayDetachedOrImmutableOrOutOfBounds: Smi = 1",
  ),
  ("struct", "TypedArrayAccessor", "struct TypedArrayAccessor"),
  (
    "macro",
    "GetTypedArrayAccessor",
    "macro GetTypedArrayAccessor<T : type extends ElementsKind>(): TypedArrayAccessor",
  ),
  (
    "macro",
    "GetTypedArrayAccessor",
    "macro GetTypedArrayAccessor(elementsKindParam: ElementsKind): TypedArrayAccessor",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::SetJSTypedArrayOnHeapDataPtr",
    "macro TypedArrayBuiltinsAssembler::SetJSTypedArrayOnHeapDataPtr( JSTypedArray, ByteArray, uintptr): void",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::SetJSTypedArrayOffHeapDataPtr",
    "macro TypedArrayBuiltinsAssembler::SetJSTypedArrayOffHeapDataPtr( JSTypedArray, RawPtr, uintptr): void",
  ),
  (
    "macro",
    "IsJSArrayBufferViewValid",
    "macro IsJSArrayBufferViewValid( JSArrayBufferView, constexpr TypedArrayAccessMode): never labels Valid, Fail",
  ),
  (
    "macro",
    "IsJSArrayBufferViewDetachedOrOutOfBounds",
    "macro IsJSArrayBufferViewDetachedOrOutOfBounds(JSArrayBufferView): never labels DetachedOrOutOfBounds, NotDetachedNorOutOfBounds",
  ),
  (
    "macro",
    "IsJSArrayBufferViewDetachedOrOutOfBoundsBoolean",
    "macro IsJSArrayBufferViewDetachedOrOutOfBoundsBoolean( JSArrayBufferView): bool",
  ),
  (
    "macro",
    "EnsureValid",
    "macro EnsureValid(array: JSTypedArray, mode: constexpr TypedArrayAccessMode): AttachedJSTypedArray labels Fail",
  ),
  (
    "struct",
    "ValidJSTypedArrayAndLength",
    "struct ValidJSTypedArrayAndLength",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::PartiallyValidateTypedArrayMaybeOOB",
    "macro TypedArrayBuiltinsAssembler::PartiallyValidateTypedArrayMaybeOOB( Context, JSAny, constexpr string, constexpr TypedArrayAccessMode): JSTypedArray",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::ValidateTypedArrayAndGetLength",
    "macro TypedArrayBuiltinsAssembler::ValidateTypedArrayAndGetLength( Context, JSAny, constexpr string, constexpr TypedArrayAccessMode): uintptr",
  ),
  (
    "macro",
    "EnsureValidAndReadLength",
    "macro EnsureValidAndReadLength( array: JSTypedArray, mode: constexpr TypedArrayAccessMode): ValidJSTypedArrayAndLength labels Fail",
  ),
  (
    "struct",
    "AttachedJSTypedArrayWitness",
    "struct AttachedJSTypedArrayWitness",
  ),
  (
    "macro",
    "NewAttachedJSTypedArrayWitness",
    "macro NewAttachedJSTypedArrayWitness(array: AttachedJSTypedArray): AttachedJSTypedArrayWitness",
  ),
  (
    "macro",
    "KindForArrayType",
    "macro KindForArrayType<T : type extends ElementsKind>(): constexpr ElementsKind",
  ),
  (
    "builtin",
    "LoadTypedElement",
    "builtin LoadTypedElement<T : type extends ElementsKind>( array: JSTypedArray, index: uintptr): Numeric",
  ),
  (
    "builtin",
    "StoreTypedElementNumeric",
    "builtin StoreTypedElementNumeric<T : type extends ElementsKind>( context: Context, typedArray: JSTypedArray, index: uintptr, value: Numeric): Smi",
  ),
  (
    "builtin",
    "StoreTypedElementJSAny",
    "builtin StoreTypedElementJSAny<T : type extends ElementsKind>( context: Context, typedArray: JSTypedArray, index: uintptr, value: JSAny): Smi",
  ),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const STORE_SUCCEDED: i64 = 0; // 原版: kStoreSucceded
pub const STORE_FAILURE_ARRAY_DETACHED_OR_IMMUTABLE_OR_OUT_OF_BOUNDS: i64 = 1; // 原版: kStoreFailureArrayDetachedOrImmutableOrOutOfBounds

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 53);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 16);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 27);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 3);
};
