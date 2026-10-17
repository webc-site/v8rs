// 由 tool/src/builtins/wasm-interpreter-wrappers.tq.js 从 v8/src/builtins/wasm-interpreter-wrappers.tq 生成; 修订就地改, 复跑覆盖
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
  ("namespace", "wasm", "namespace wasm"),
  (
    "builtin",
    "JSToWasmInterpreterWrapperAsm",
    "builtin JSToWasmInterpreterWrapperAsm( RawPtr<intptr>, WasmExportedFunctionData, JSAny): JSAny",
  ),
  (
    "const",
    "kWasmS128Type",
    "const kWasmS128Type: constexpr uint32 generates 'wasm::kWasmS128.raw_bit_field()'",
  ),
  (
    "const",
    "kWrapperBufferRefParamsArray",
    "const kWrapperBufferRefParamsArray: constexpr intptr generates 'WasmInterpreterWrapperConstants::kWrapperBufferRefParamsArray'",
  ),
  (
    "macro",
    "NewInterpreterReturnSlotAllocator",
    "macro NewInterpreterReturnSlotAllocator(): ReturnSlotAllocator",
  ),
  (
    "macro",
    "LocationAllocatorForInterpreterParams",
    "macro LocationAllocatorForInterpreterParams( paramBuffer: &intptr, paramBufferSize: intptr): LocationAllocator",
  ),
  (
    "macro",
    "LocationAllocatorForWasmInterpreterReturns",
    "macro LocationAllocatorForWasmInterpreterReturns(stack: RawPtr): LocationAllocator",
  ),
  (
    "builtin",
    "JSToWasmInterpreterWrapper",
    "builtin JSToWasmInterpreterWrapper( js-implicit context: NativeContext, receiver: JSAny, target: JSFunction, dispatchHandle: DispatchHandle)(...arguments): JSAny",
  ),
  (
    "builtin",
    "JSToWasmInterpreterHandleReturns",
    "builtin JSToWasmInterpreterHandleReturns( resultArray: JSArray, wrapperBuffer: RawPtr<intptr>): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  ("kWasmS128Type", "wasm::kWasmS128.raw_bit_field()"),
  (
    "kWrapperBufferRefParamsArray",
    "WasmInterpreterWrapperConstants::kWrapperBufferRefParamsArray",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 9);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 3);
};
