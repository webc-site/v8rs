// 由 tool/src/builtins/array-join.tq.js 从 v8/src/builtins/array-join.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-array; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 32 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "array", "namespace array"),
  (
    "type",
    "LoadJoinElementFn",
    "type LoadJoinElementFn = builtin(Context, JSReceiver, uintptr) => JSAny",
  ),
  (
    "const",
    "kMaxArrayLength",
    "const kMaxArrayLength: constexpr uint32 generates 'JSArray::kMaxArrayLength'",
  ),
  (
    "const",
    "kMaxBufferChunkSize",
    "const kMaxBufferChunkSize: constexpr int31 = kMaxNewSpaceFixedArrayElements",
  ),
  (
    "macro",
    "ArrayBuiltinsAssembler::CallJSArrayArrayJoinConcatToSequentialString",
    "macro ArrayBuiltinsAssembler::CallJSArrayArrayJoinConcatToSequentialString( FixedArray, intptr, String, String): String",
  ),
  (
    "builtin",
    "LoadJoinElement",
    "builtin LoadJoinElement<T : type extends ElementsKind>( context: Context, receiver: JSReceiver, k: uintptr): JSAny",
  ),
  (
    "builtin",
    "LoadJoinTypedElement",
    "builtin LoadJoinTypedElement<T : type extends ElementsKind>( context: Context, receiver: JSReceiver, k: uintptr): JSAny",
  ),
  (
    "builtin",
    "ConvertToLocaleString",
    "builtin ConvertToLocaleString( context: Context, element: JSAny, locales: JSAny, options: JSAny): String",
  ),
  (
    "macro",
    "CannotUseSameArrayAccessor",
    "macro CannotUseSameArrayAccessor<T: type>( implicit context: Context)(loadFn: LoadJoinElementFn, receiver: JSReceiver, originalMap: Map, originalLen: Number): bool",
  ),
  (
    "macro",
    "AddStringLength",
    "macro AddStringLength( implicit context: Context)(lenA: intptr, lenB: intptr): intptr",
  ),
  (
    "macro",
    "StoreAndGrowFixedArray",
    "macro StoreAndGrowFixedArray<T: type>( fixedArray: FixedArray, index: intptr, element: T): FixedArray",
  ),
  ("struct", "Buffer", "struct Buffer"),
  (
    "macro",
    "NewBuffer",
    "macro NewBuffer(len: uintptr, sep: String): Buffer",
  ),
  (
    "macro",
    "BufferJoin",
    "macro BufferJoin(implicit context: Context)(buffer: Buffer, sep: String): String",
  ),
  (
    "macro",
    "FastArrayJoin",
    "macro FastArrayJoin( implicit context: Context)(kind: constexpr ElementsKind, array: FastJSArrayForRead, sep: String, length: Smi): String labels Bailout",
  ),
  (
    "macro",
    "ArrayJoinImpl",
    "macro ArrayJoinImpl<T: type>( implicit context: Context)(receiver: JSReceiver, sep: String, lengthNumber: Number, useToLocaleString: constexpr bool, locales: JSAny, options: JSAny, initialLoadFn: LoadJoinElementFn): String",
  ),
  (
    "macro",
    "ArrayJoin",
    "macro ArrayJoin<T: type>( implicit context: Context)(useToLocaleString: constexpr bool, receiver: JSReceiver, sep: String, lenNumber: Number, locales: JSAny, options: JSAny): JSAny",
  ),
  (
    "const",
    "kMinJoinStackSize",
    "const kMinJoinStackSize: constexpr int31 generates 'JSArray::kMinJoinStackSize'",
  ),
  (
    "macro",
    "LoadJoinStack",
    "macro LoadJoinStack(implicit context: Context)(): FixedArray labels IfUninitialized",
  ),
  (
    "macro",
    "SetJoinStack",
    "macro SetJoinStack(implicit context: Context)(stack: FixedArray): void",
  ),
  (
    "builtin",
    "JoinStackPush",
    "builtin JoinStackPush( implicit context: Context)(stack: FixedArray, receiver: JSReceiver): Boolean",
  ),
  (
    "macro",
    "JoinStackPushInline",
    "macro JoinStackPushInline(implicit context: Context)(receiver: JSReceiver): bool",
  ),
  (
    "builtin",
    "JoinStackPop",
    "builtin JoinStackPop( implicit context: Context)(stack: FixedArray, receiver: JSReceiver): JSAny",
  ),
  (
    "macro",
    "JoinStackPopInline",
    "macro JoinStackPopInline(implicit context: Context)(receiver: JSReceiver): void",
  ),
  (
    "macro",
    "CycleProtectedArrayJoin",
    "macro CycleProtectedArrayJoin<T: type>( implicit context: Context)(useToLocaleString: constexpr bool, o: JSReceiver, len: Number, sep: String, locales: JSAny, options: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayPrototypeJoin",
    "builtin ArrayPrototypeJoin( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "ArrayPrototypeJoinImpl",
    "builtin ArrayPrototypeJoinImpl( implicit context: Context)(o: JSReceiver, len: Number, separator: String): JSAny",
  ),
  (
    "macro",
    "ArrayPrototypeToString_Inline",
    "macro ArrayPrototypeToString_Inline( implicit context: Context)(array: JSArrayForFastToString): String",
  ),
  (
    "builtin",
    "ArrayPrototypeToLocaleString",
    "builtin ArrayPrototypeToLocaleString( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "ArrayPrototypeToString",
    "builtin ArrayPrototypeToString( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "TypedArrayPrototypeJoin",
    "builtin TypedArrayPrototypeJoin( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "TypedArrayPrototypeToLocaleString",
    "builtin TypedArrayPrototypeToLocaleString( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  ("kMaxArrayLength", "JSArray::kMaxArrayLength"),
  ("kMinJoinStackSize", "JSArray::kMinJoinStackSize"),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 32);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 15);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 11);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 1);
};
