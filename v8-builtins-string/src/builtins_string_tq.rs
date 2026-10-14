// 由 tool/src/builtins/builtins-string.tq.js 从 v8/src/builtins/builtins-string.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-string; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 19 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "string", "namespace string"),
  ("namespace", "runtime", "namespace runtime"),
  (
    "const",
    "kind",
    "const kind = ElementsKind::PACKED_ELEMENTS",
  ),
  (
    "const",
    "stringLength",
    "const stringLength: intptr = string.length_intptr",
  ),
  (
    "const",
    "nativeContext",
    "const nativeContext = LoadNativeContext(context)",
  ),
  (
    "const",
    "map",
    "const map: Map = LoadJSArrayElementsMap(kind, nativeContext)",
  ),
  (
    "const",
    "array",
    "const array: JSArray = AllocateJSArray(kind, map, stringLength, SmiTag(stringLength))",
  ),
  (
    "const",
    "elements",
    "const elements = UnsafeCast<FixedArray>(array.elements)",
  ),
  (
    "const",
    "encoding",
    "const encoding = UnicodeEncoding::UTF16",
  ),
  (
    "const",
    "string",
    "const string: String = ToThisString(receiver, methodName)",
  ),
  (
    "const",
    "indexNumber",
    "const indexNumber: Number = ToInteger_Inline(position)",
  ),
  (
    "const",
    "length",
    "const length: intptr = Convert<intptr>(arguments.length)",
  ),
  (
    "const",
    "length",
    "const length: intptr = Convert<intptr>(arguments.length)",
  ),
  (
    "const",
    "map",
    "const map = GetDerivedMap(target, UnsafeCast<JSReceiver>(newTarget))",
  ),
  (
    "const",
    "obj",
    "const obj = UnsafeCast<JSPrimitiveWrapper>(AllocateFastOrSlowJSObjectFromMap(map))",
  ),
  ("const", "function", "const function = GetStringFunction()"),
  (
    "const",
    "initialMap",
    "const initialMap = UnsafeCast<Map>(function.prototype_or_initial_map)",
  ),
  (
    "const",
    "obj",
    "const obj = UnsafeCast<JSPrimitiveWrapper>( AllocateFastOrSlowJSObjectFromMap(initialMap))",
  ),
  (
    "const",
    "code",
    "const code: char16 = StringCharCodeAt(receiver, position)",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 19);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 17);
};
