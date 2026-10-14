// 由 tool/src/builtins/regexp-match.tq.js 从 v8/src/builtins/regexp-match.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-regexp; 聚合基线脚本另产同源模块 (并存不覆盖)

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
  ("namespace", "regexp", "namespace regexp"),
  (
    "const",
    "kATOM",
    "const kATOM: constexpr uint8 generates 'static_cast<uint8_t>(RegExpData::Type::ATOM)'",
  ),
  (
    "const",
    "kIRREGEXP",
    "const kIRREGEXP: constexpr uint8 generates 'static_cast<uint8_t>(RegExpData::Type::IRREGEXP)'",
  ),
  (
    "const",
    "kEXPERIMENTAL",
    "const kEXPERIMENTAL: constexpr uint8 generates 'static_cast<uint8_t>(RegExpData::Type::EXPERIMENTAL)'",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::RegExpMatchGlobal",
    "macro RegExpBuiltinsAssembler::RegExpMatchGlobal( implicit context: Context)(JSRegExp, String, RegExpData): JSAny",
  ),
  ("namespace", "runtime", "namespace runtime"),
  (
    "const",
    "receiver",
    "const receiver = UnsafeCast<JSReceiver>(receiver)",
  ),
  (
    "const",
    "string",
    "const string: String = ToString_Inline(string)",
  ),
  (
    "const",
    "fastRegExp",
    "const fastRegExp = Cast<FastJSRegExp>(receiver) otherwise return SlowRegExpPrototypeMatchBody(receiver, string)",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  ("kATOM", "static_cast<uint8_t>(RegExpData::Type::ATOM)"),
  (
    "kIRREGEXP",
    "static_cast<uint8_t>(RegExpData::Type::IRREGEXP)",
  ),
  (
    "kEXPERIMENTAL",
    "static_cast<uint8_t>(RegExpData::Type::EXPERIMENTAL)",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 9);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 6);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 1);
};
