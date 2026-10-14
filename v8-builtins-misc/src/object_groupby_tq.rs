// 由 tool/src/builtins/object-groupby.tq.js 从 v8/src/builtins/object-groupby.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 9 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "collections", "namespace collections"),
  (
    "macro",
    "CollectionsBuiltinsAssembler::AddValueToKeyedGroup",
    "macro CollectionsBuiltinsAssembler::AddValueToKeyedGroup( implicit context: Context)(OrderedHashMap, JSAny, Object, String): OrderedHashMap",
  ),
  (
    "macro",
    "CollectionsBuiltinsAssembler::NormalizeNumberKey",
    "macro CollectionsBuiltinsAssembler::NormalizeNumberKey(JSAny): JSAny",
  ),
  (
    "const",
    "fastIteratorResultMap",
    "const fastIteratorResultMap = GetIteratorResultMap()",
  ),
  (
    "const",
    "iteratorRecord",
    "const iteratorRecord = iterator::GetIterator(items)",
  ),
  (
    "const",
    "callbackfn",
    "const callbackfn = Cast<Callable>(callback) otherwise ThrowTypeError(MessageTemplate::kCalledNonCallable, callback)",
  ),
  (
    "const",
    "groups",
    "const groups: OrderedHashMap = GroupByImpl( items, callback, /* coerceToProperty */ True, 'Object.groupBy')",
  ),
  (
    "const",
    "nullProtoMap",
    "const nullProtoMap = LoadSlowObjectWithNullPrototypeMap(context)",
  ),
  (
    "const",
    "obj",
    "const obj = AllocateJSObjectFromMap(nullProtoMap, properties)",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 9);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 6);
};
