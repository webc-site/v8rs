// 由 tool/src/builtins/arraybuffer.tq.js 从 v8/src/builtins/arraybuffer.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 10 条)
pub const DECLS: &[TqDecl] = &[
  (
    "macro",
    "ArrayBufferWasmMemorySymbol",
    "macro ArrayBufferWasmMemorySymbol(): PrivateSymbol",
  ),
  ("namespace", "arraybuffer", "namespace arraybuffer"),
  (
    "builtin",
    "ArrayBufferPrototypeGetByteLength",
    "builtin ArrayBufferPrototypeGetByteLength( js-implicit context: NativeContext, receiver: JSAny)(): Number",
  ),
  (
    "builtin",
    "ArrayBufferPrototypeGetMaxByteLength",
    "builtin ArrayBufferPrototypeGetMaxByteLength( js-implicit context: NativeContext, receiver: JSAny)(): Number",
  ),
  (
    "builtin",
    "ArrayBufferPrototypeGetResizable",
    "builtin ArrayBufferPrototypeGetResizable( js-implicit context: NativeContext, receiver: JSAny)(): Boolean",
  ),
  (
    "builtin",
    "ArrayBufferPrototypeGetDetached",
    "builtin ArrayBufferPrototypeGetDetached( js-implicit context: NativeContext, receiver: JSAny)(): Boolean",
  ),
  (
    "builtin",
    "ArrayBufferPrototypeGetImmutable",
    "builtin ArrayBufferPrototypeGetImmutable( js-implicit context: NativeContext, receiver: JSAny)(): Boolean",
  ),
  (
    "builtin",
    "SharedArrayBufferPrototypeGetMaxByteLength",
    "builtin SharedArrayBufferPrototypeGetMaxByteLength( js-implicit context: NativeContext, receiver: JSAny)(): Number",
  ),
  (
    "builtin",
    "SharedArrayBufferPrototypeGetGrowable",
    "builtin SharedArrayBufferPrototypeGetGrowable( js-implicit context: NativeContext, receiver: JSAny)(): Boolean",
  ),
  (
    "builtin",
    "ArrayBufferIsView",
    "builtin ArrayBufferIsView(arg: JSAny): Boolean",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 10);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 8);
};
