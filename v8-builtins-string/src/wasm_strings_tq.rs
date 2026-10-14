// 由 tool/src/builtins/wasm-strings.tq.js 从 v8/src/builtins/wasm-strings.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 33 条)
pub const DECLS: &[TqDecl] = &[
  (
    "macro",
    "Trap",
    "macro Trap(context: Context, error: constexpr MessageTemplate): never",
  ),
  (
    "builtin",
    "WebAssemblyStringCast",
    "builtin WebAssemblyStringCast( js-implicit context: Context)(arg: JSAny): String",
  ),
  (
    "builtin",
    "WebAssemblyStringTest",
    "builtin WebAssemblyStringTest( js-implicit context: Context)(arg: JSAny): Smi",
  ),
  (
    "const",
    "kWtf16Unshared",
    "const kWtf16Unshared: constexpr int31 generates 'UnicodeConfig::kWtf16Unshared().raw_as_int()'",
  ),
  (
    "const",
    "kWtf16Shared",
    "const kWtf16Shared: constexpr int31 generates 'UnicodeConfig::kWtf16Shared().raw_as_int()'",
  ),
  (
    "builtin",
    "WebAssemblyStringFromWtf16Array",
    "builtin WebAssemblyStringFromWtf16Array( js-implicit context: Context)(arrayArg: JSAny, startArg: JSAny, endArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringFromWtf16ArrayShared",
    "builtin WebAssemblyStringFromWtf16ArrayShared( js-implicit context: Context)(arrayArg: JSAny, startArg: JSAny, endArg: JSAny): JSAny",
  ),
  (
    "const",
    "kLossyUtf8",
    "const kLossyUtf8: constexpr int31 generates 'unibrow::Utf8Variant::kLossyUtf8'",
  ),
  (
    "const",
    "kLossyUtf8Unshared",
    "const kLossyUtf8Unshared: constexpr int31 generates 'UnicodeConfig::kLossyUtf8Unshared().raw_as_int()'",
  ),
  (
    "const",
    "kLossyUtf8Shared",
    "const kLossyUtf8Shared: constexpr int31 generates 'UnicodeConfig::kLossyUtf8Shared().raw_as_int()'",
  ),
  (
    "builtin",
    "WebAssemblyStringFromUtf8Array",
    "builtin WebAssemblyStringFromUtf8Array( js-implicit context: Context)(arrayArg: JSAny, startArg: JSAny, endArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringFromUtf8ArrayShared",
    "builtin WebAssemblyStringFromUtf8ArrayShared( js-implicit context: Context)(arrayArg: JSAny, startArg: JSAny, endArg: JSAny): JSAny",
  ),
  (
    "macro",
    "WebAssemblyStringIntoUtf8ArrayImpl",
    "macro WebAssemblyStringIntoUtf8ArrayImpl( context: Context, stringArg: JSAny, arrayArg: JSAny, startArg: JSAny, shared: Smi): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringIntoUtf8Array",
    "builtin WebAssemblyStringIntoUtf8Array( js-implicit context: Context)(stringArg: JSAny, arrayArg: JSAny, startArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringIntoUtf8ArrayShared",
    "builtin WebAssemblyStringIntoUtf8ArrayShared( js-implicit context: Context)(stringArg: JSAny, arrayArg: JSAny, startArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringToUtf8Array",
    "builtin WebAssemblyStringToUtf8Array( js-implicit context: Context)(stringArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringToUtf8ArrayShared",
    "builtin WebAssemblyStringToUtf8ArrayShared( js-implicit context: Context)(stringArg: JSAny): JSAny",
  ),
  (
    "macro",
    "WebAssemblyStringToWtf16ArrayImpl",
    "macro WebAssemblyStringToWtf16ArrayImpl( context: Context, stringArg: JSAny, arrayArg: JSAny, startArg: JSAny, shared: Smi): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringToWtf16Array",
    "builtin WebAssemblyStringToWtf16Array( js-implicit context: Context)(stringArg: JSAny, arrayArg: JSAny, startArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringToWtf16ArrayShared",
    "builtin WebAssemblyStringToWtf16ArrayShared( js-implicit context: Context)(stringArg: JSAny, arrayArg: JSAny, startArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringFromCharCode",
    "builtin WebAssemblyStringFromCharCode( js-implicit context: Context)(codeArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringFromCharCodeShared",
    "builtin WebAssemblyStringFromCharCodeShared( js-implicit context: Context)(codeArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringFromCodePoint",
    "builtin WebAssemblyStringFromCodePoint( js-implicit context: Context)(codeArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringFromCodePointShared",
    "builtin WebAssemblyStringFromCodePointShared( js-implicit context: Context)(codeArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringCodePointAt",
    "builtin WebAssemblyStringCodePointAt( js-implicit context: Context)(stringArg: JSAny, indexArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringCharCodeAt",
    "builtin WebAssemblyStringCharCodeAt( js-implicit context: Context)(stringArg: JSAny, indexArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringLength",
    "builtin WebAssemblyStringLength( js-implicit context: Context)(stringArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringMeasureUtf8",
    "builtin WebAssemblyStringMeasureUtf8( js-implicit context: Context)(stringArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringConcat",
    "builtin WebAssemblyStringConcat( js-implicit context: Context)(firstArg: JSAny, secondArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringSubstring",
    "builtin WebAssemblyStringSubstring( js-implicit context: Context)(stringArg: JSAny, startArg: JSAny, endArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringEquals",
    "builtin WebAssemblyStringEquals( js-implicit context: Context)(a: JSAny, b: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyStringCompare",
    "builtin WebAssemblyStringCompare( js-implicit context: Context)(firstArg: JSAny, secondArg: JSAny): JSAny",
  ),
  (
    "builtin",
    "WebAssemblyConfigureAllPrototypes",
    "builtin WebAssemblyConfigureAllPrototypes( js-implicit context: Context)(prototypes: JSAny, functions: JSAny, data: JSAny, constructors: JSAny): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  (
    "kWtf16Unshared",
    "UnicodeConfig::kWtf16Unshared().raw_as_int()",
  ),
  ("kWtf16Shared", "UnicodeConfig::kWtf16Shared().raw_as_int()"),
  ("kLossyUtf8", "unibrow::Utf8Variant::kLossyUtf8"),
  (
    "kLossyUtf8Unshared",
    "UnicodeConfig::kLossyUtf8Unshared().raw_as_int()",
  ),
  (
    "kLossyUtf8Shared",
    "UnicodeConfig::kLossyUtf8Shared().raw_as_int()",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 33);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 25);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 5);
};
