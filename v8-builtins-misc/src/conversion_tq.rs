// 由 tool/src/builtins/conversion.tq.js 从 v8/src/builtins/conversion.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 10 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "runtime", "namespace runtime"),
  ("namespace", "conversion", "namespace conversion"),
  ("const", "smi", "const smi = SmiFromInt32(input)"),
  ("const", "untagged", "const untagged = SmiToInt32(smi)"),
  (
    "const",
    "exoticToPrimitive",
    "const exoticToPrimitive: JSAny = GetInterestingProperty(context, input, ToPrimitiveSymbolConstant()) otherwise OrdinaryToPrimitive",
  ),
  (
    "const",
    "result",
    "const result: JSAny = Call(context, exoticToPrimitive, input, hint)",
  ),
  (
    "const",
    "exoticToPrimitive",
    "const exoticToPrimitive: JSAny = TryGetExoticToPrimitive(input) otherwise return OrdinaryToPrimitive_Number_Inline(input)",
  ),
  (
    "const",
    "exoticToPrimitive",
    "const exoticToPrimitive: JSAny = TryGetExoticToPrimitive(input) otherwise return OrdinaryToPrimitive_Number_Inline(input)",
  ),
  (
    "const",
    "exoticToPrimitive",
    "const exoticToPrimitive: JSAny = TryGetExoticToPrimitive(input) otherwise return OrdinaryToPrimitive_String_Inline(input)",
  ),
  (
    "const",
    "method",
    "const method: JSAny = GetProperty(input, name)",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 10);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 8);
};
