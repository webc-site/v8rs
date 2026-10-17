// 由 tool/src/builtins/string-towellformed.tq.js 从 v8/src/builtins/string-towellformed.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 6 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "runtime", "namespace runtime"),
  ("namespace", "string", "namespace string"),
  (
    "const",
    "methodName",
    "const methodName: constexpr string = 'String.prototype.toWellFormed'",
  ),
  ("const", "s", "const s = ToThisString(receiver, methodName)"),
  ("const", "strLen", "const strLen = s.length_uint32"),
  ("const", "flat", "const flat = Flatten(s)"),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const METHOD_NAME: &'static str = "String.prototype.toWellFormed"; // 原版: methodName

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 6);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 4);
};
