// 由 tool/src/builtins/typed-array-createtypedarray.tq.js 从 v8/src/builtins/typed-array-createtypedarray.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 23 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "typed_array", "namespace typed_array"),
  (
    "builtin",
    "IterableToListConvertHoles",
    "builtin IterableToListConvertHoles(Context, Object, Callable): JSArray",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::AllocateEmptyOnHeapBuffer",
    "macro TypedArrayBuiltinsAssembler::AllocateEmptyOnHeapBuffer( implicit context: Context)(): JSArrayBuffer",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::GetDefaultConstructor",
    "macro TypedArrayBuiltinsAssembler::GetDefaultConstructor( implicit context: Context)(JSTypedArray): JSFunction",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::SetupTypedArrayEmbedderFields",
    "macro TypedArrayBuiltinsAssembler::SetupTypedArrayEmbedderFields( JSTypedArray): void",
  ),
  (
    "const",
    "kFloat16ArrayFeature",
    "const kFloat16ArrayFeature: constexpr UseCounterFeature generates 'v8::Isolate::kFloat16Array'",
  ),
  (
    "const",
    "kBigIntFeature",
    "const kBigIntFeature: constexpr UseCounterFeature generates 'v8::Isolate::kBigInt'",
  ),
  (
    "macro",
    "TypedArrayBuiltinsAssembler::IsTrackArrayBufferViewsEnabled",
    "macro TypedArrayBuiltinsAssembler::IsTrackArrayBufferViewsEnabled(): bool",
  ),
  (
    "macro",
    "AttachView",
    "macro AttachView(buffer: JSArrayBuffer, view: JSArrayBufferView): void",
  ),
  (
    "macro",
    "AllocateTypedArray",
    "macro AllocateTypedArray( implicit context: Context)(isOnHeap: constexpr bool, map: Map, buffer: JSArrayBuffer, byteOffset: uintptr, byteLength: uintptr, isLengthTracking: bool): JSTypedArray",
  ),
  (
    "macro",
    "TypedArrayInitialize",
    "macro TypedArrayInitialize( implicit context: Context)(initialize: constexpr bool, map: Map, length: uintptr, elementsInfo: typed_array::TypedArrayElementsInfo): JSTypedArray labels IfRangeError",
  ),
  (
    "macro",
    "ConstructByLength",
    "macro ConstructByLength( implicit context: Context)(target: JSFunction, newTarget: JSReceiver, lengthObj: JSAny): JSTypedArray",
  ),
  (
    "macro",
    "ConstructByArrayLike",
    "macro ConstructByArrayLike( implicit context: Context)(map: Map, arrayLike: HeapObject, length: uintptr, elementsInfo: typed_array::TypedArrayElementsInfo): JSTypedArray",
  ),
  (
    "macro",
    "ConstructByIterable",
    "macro ConstructByIterable( implicit context: Context)(iterable: JSReceiver, iteratorFn: Callable): never labels IfConstructByArrayLike(JSArray, uintptr)",
  ),
  (
    "macro",
    "ConstructByTypedArray",
    "macro ConstructByTypedArray( implicit context: Context)(srcTypedArray: JSTypedArray): never labels IfConstructByArrayLike(JSTypedArray, uintptr)",
  ),
  (
    "macro",
    "ConstructByArrayBuffer",
    "macro ConstructByArrayBuffer( implicit context: Context)(target: JSFunction, newTarget: JSReceiver, buffer: JSArrayBuffer, byteOffset: JSAny, length: JSAny): JSTypedArray",
  ),
  (
    "macro",
    "TypedArrayCreateByLength",
    "macro TypedArrayCreateByLength( implicit context: Context)(constructor: Constructor, length: Number, methodName: constexpr string, accessMode: constexpr TypedArrayAccessMode): JSTypedArray",
  ),
  (
    "macro",
    "ConstructByJSReceiver",
    "macro ConstructByJSReceiver( implicit context: Context)(obj: JSReceiver): never labels IfConstructByArrayLike(JSReceiver, uintptr), IfIteratorNotCallable(JSAny)",
  ),
  (
    "builtin",
    "CreateTypedArray",
    "builtin CreateTypedArray( context: Context, target: JSFunction, newTarget: JSReceiver, arg1: JSAny, arg2: JSAny, arg3: JSAny): JSTypedArray",
  ),
  (
    "macro",
    "TypedArraySpeciesCreateMaybeOOB",
    "macro TypedArraySpeciesCreateMaybeOOB( implicit context: Context)(methodName: constexpr string, numArgs: constexpr int31, exemplar: JSTypedArray, arg0: JSAny, arg1: JSAny, arg2: JSAny, accessMode: constexpr TypedArrayAccessMode): JSTypedArr…",
  ),
  (
    "macro",
    "TypedArraySpeciesCreateByLength",
    "macro TypedArraySpeciesCreateByLength( implicit context: Context)(methodName: constexpr string, exemplar: JSTypedArray, length: uintptr, accessMode: constexpr TypedArrayAccessMode): JSTypedArray",
  ),
  (
    "macro",
    "TypedArraySpeciesCreateByBuffer",
    "macro TypedArraySpeciesCreateByBuffer( implicit context: Context)(methodName: constexpr string, exemplar: JSTypedArray, buffer: JSArrayBuffer, beginByteOffset: uintptr, newLength: NumberOrUndefined, accessMode: constexpr TypedArrayAccessMod…",
  ),
  (
    "macro",
    "TypedArrayCreateSameType",
    "macro TypedArrayCreateSameType( implicit context: Context)(exemplar: JSTypedArray, newLength: uintptr): JSTypedArray",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  ("kFloat16ArrayFeature", "v8::Isolate::kFloat16Array"),
  ("kBigIntFeature", "v8::Isolate::kBigInt"),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 23);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 18);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
};
