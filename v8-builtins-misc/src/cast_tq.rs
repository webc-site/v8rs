// 由 tool/src/builtins/cast.tq.js 从 v8/src/builtins/cast.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 56 条)
pub const DECLS: &[TqDecl] = &[
  ("macro", "IsBigInt", "macro IsBigInt(HeapObject): bool"),
  (
    "macro",
    "IsConstructor",
    "macro IsConstructor(HeapObject): bool",
  ),
  (
    "macro",
    "IsCustomElementsReceiverInstanceType",
    "macro IsCustomElementsReceiverInstanceType(int32): bool",
  ),
  (
    "macro",
    "IsExtensibleMap",
    "macro IsExtensibleMap(Map): bool",
  ),
  (
    "macro",
    "IsNumberNormalized",
    "macro IsNumberNormalized(Number): bool",
  ),
  (
    "macro",
    "IsSafeInteger",
    "macro IsSafeInteger(Object): bool",
  ),
  (
    "macro",
    "IsAccessorInfo",
    "macro IsAccessorInfo(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsAccessorPair",
    "macro IsAccessorPair(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsAllocationSite",
    "macro IsAllocationSite(o: HeapObject): bool",
  ),
  ("macro", "IsCell", "macro IsCell(o: HeapObject): bool"),
  (
    "macro",
    "IsInstructionStream",
    "macro IsInstructionStream(o: HeapObject): bool",
  ),
  ("macro", "IsCode", "macro IsCode(o: HeapObject): bool"),
  ("macro", "IsContext", "macro IsContext(o: HeapObject): bool"),
  (
    "macro",
    "IsExtendedMap",
    "macro IsExtendedMap(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsJSInterceptorMap",
    "macro IsJSInterceptorMap(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsCoverageInfo",
    "macro IsCoverageInfo(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsDebugInfo",
    "macro IsDebugInfo(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsFixedDoubleArray",
    "macro IsFixedDoubleArray(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsFeedbackCell",
    "macro IsFeedbackCell(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsFeedbackVector",
    "macro IsFeedbackVector(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsHeapNumber",
    "macro IsHeapNumber(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsJSModuleNamespace",
    "macro IsJSModuleNamespace(o: Object): bool",
  ),
  (
    "macro",
    "IsJSPrototype",
    "macro IsJSPrototype(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsUninitializedHeapNumber",
    "macro IsUninitializedHeapNumber(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsNativeContext",
    "macro IsNativeContext(o: HeapObject): bool",
  ),
  ("macro", "IsNumber", "macro IsNumber(o: Object): bool"),
  (
    "macro",
    "IsPrivateSymbol",
    "macro IsPrivateSymbol(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsPromiseCapability",
    "macro IsPromiseCapability(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsPromiseFulfillReactionJobTask",
    "macro IsPromiseFulfillReactionJobTask(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsPromiseReaction",
    "macro IsPromiseReaction(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsPromiseRejectReactionJobTask",
    "macro IsPromiseRejectReactionJobTask(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsSharedFunctionInfo",
    "macro IsSharedFunctionInfo(o: HeapObject): bool",
  ),
  ("macro", "IsSymbol", "macro IsSymbol(o: HeapObject): bool"),
  ("macro", "IsTuple2", "macro IsTuple2(o: HeapObject): bool"),
  (
    "macro",
    "TaggedToHeapObject",
    "macro TaggedToHeapObject(Object): HeapObject labels CastError",
  ),
  (
    "macro",
    "TaggedToSmi",
    "macro TaggedToSmi(Object): Smi labels CastError",
  ),
  (
    "macro",
    "TaggedToPositiveSmi",
    "macro TaggedToPositiveSmi(Object): PositiveSmi labels CastError",
  ),
  (
    "macro",
    "TaggedToDirectString",
    "macro TaggedToDirectString(Object): DirectString labels CastError",
  ),
  (
    "macro",
    "HeapObjectToCallable",
    "macro HeapObjectToCallable(HeapObject): Callable labels CastError",
  ),
  (
    "macro",
    "HeapObjectToConstructor",
    "macro HeapObjectToConstructor(HeapObject): Constructor labels CastError",
  ),
  (
    "macro",
    "Cast",
    "macro Cast<A : type extends WeakHeapObject>(o: A|Object): A labels CastError",
  ),
  (
    "macro",
    "Cast",
    "macro Cast<A : type extends Object>( implicit context: Context)(o: MaybeObject): A labels CastError",
  ),
  (
    "macro",
    "Cast",
    "macro Cast<A : type extends Object>(implicit context: Context)(o: Object): A labels CastError",
  ),
  (
    "macro",
    "Cast",
    "macro Cast<A : type extends HeapObject>(o: HeapObject): A labels CastError",
  ),
  (
    "macro",
    "Cast",
    "macro Cast<T : type extends Symbol>(o: Symbol): T labels CastError",
  ),
  ("struct", "CastResult", "struct CastResult<T: type>"),
  (
    "macro",
    "CastFastJSArray",
    "macro CastFastJSArray(implicit context: Context)(o: HeapObject): CastResult<FastJSArray>labels CastError",
  ),
  (
    "macro",
    "CastFastJSArrayForRead",
    "macro CastFastJSArrayForRead(implicit context: Context)(o: HeapObject): CastResult<FastJSArrayForRead>labels CastError",
  ),
  (
    "macro",
    "Cast",
    "macro Cast<T: type>(o: String): T labels CastError",
  ),
  (
    "macro",
    "Is",
    "macro Is<A : type extends Object, B : type extends Object>( implicit context: Context)(o: B): bool",
  ),
  (
    "macro",
    "Is",
    "macro Is<A : type extends Object>( implicit context: Context)(o: MaybeObject): bool",
  ),
  (
    "macro",
    "UnsafeCast",
    "macro UnsafeCast<A : type extends Object>( implicit context: Context)(o: Object): A",
  ),
  (
    "macro",
    "UnsafeCast",
    "macro UnsafeCast<A : type extends Object>( implicit context: Context)(o: MaybeObject): A",
  ),
  (
    "macro",
    "UnsafeConstCast",
    "macro UnsafeConstCast<T: type>(r: const &T):&T",
  ),
  (
    "macro",
    "UnsafeCast",
    "macro UnsafeCast<A : type extends WeakHeapObject>(o: A|Object): A",
  ),
  (
    "macro",
    "CastOrDefault",
    "macro CastOrDefault<T: type, Arg: type, Default: type>( implicit context: Context)(x: Arg, default: Default): T|Default",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 56);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 55);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 1);
};
