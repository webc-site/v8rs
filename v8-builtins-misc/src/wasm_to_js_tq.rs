// 由 tool/src/builtins/wasm-to-js.tq.js 从 v8/src/builtins/wasm-to-js.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 24 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "runtime", "namespace runtime"),
  ("namespace", "wasm", "namespace wasm"),
  (
    "const",
    "oldSP",
    "const oldSP = SwitchToTheCentralStackIfNeeded()",
  ),
  (
    "const",
    "sigSlot",
    "const sigSlot = LoadFramePointer() + kSignatureOffset",
  ),
  (
    "const",
    "alignment",
    "const alignment: intptr = StackAlignmentInBytes() / torque_internal::SizeOf<intptr>()",
  ),
  ("const", "numFixedSlots", "const numFixedSlots = alignment"),
  (
    "const",
    "returnCount",
    "const returnCount = *torque_internal::unsafe::NewOffHeapReference( %RawDownCast<RawPtr<intptr>>(data.sig + 0))",
  ),
  (
    "const",
    "paramCount",
    "const paramCount = *torque_internal::unsafe::NewOffHeapReference( %RawDownCast<RawPtr<intptr>>( data.sig + torque_internal::SizeOf<intptr>()))",
  ),
  (
    "const",
    "valueTypesStorage",
    "const valueTypesStorage = *torque_internal::unsafe::NewOffHeapReference( %RawDownCast<RawPtr<RawPtr<uint32>>>( data.sig + 2 * torque_internal::SizeOf<intptr>()))",
  ),
  (
    "const",
    "signatureValueTypes",
    "const signatureValueTypes = torque_internal::unsafe::NewOffHeapConstSlice<uint32>( valueTypesStorage, paramCount + returnCount)",
  ),
  (
    "const",
    "returnTypes",
    "const returnTypes = Subslice(signatureValueTypes, 0, returnCount) otherwise unreachable",
  ),
  (
    "const",
    "paramTypes",
    "const paramTypes = Subslice(signatureValueTypes, returnCount, paramCount) otherwise unreachable",
  ),
  (
    "const",
    "numStackParams",
    "const numStackParams = paramCount + 1",
  ),
  (
    "const",
    "outParams",
    "const outParams = WasmAllocateZeroedFixedArray(numStackParams)",
  ),
  (
    "const",
    "stackParamStart",
    "const stackParamStart = LoadFramePointer() + (2 + numFixedSlots) * torque_internal::SizeOf<intptr>()",
  ),
  (
    "const",
    "inParams",
    "const inParams = torque_internal::unsafe::NewOffHeapReference( %RawDownCast<RawPtr<intptr>>(stackParamStart))",
  ),
  ("const", "target", "const target = data.callable"),
  ("const", "context", "const context = data.native_context"),
  (
    "const",
    "result",
    "const result = CallVarargs( context, target, 0, Convert<int32>(numStackParams), outParams)",
  ),
  (
    "const",
    "gpRegSlots",
    "const gpRegSlots = %RawDownCast<RawPtr<intptr>>(StackSlotPtr( 2 * torque_internal::SizeOf<intptr>(), torque_internal::SizeOf<intptr>()))",
  ),
  (
    "const",
    "fpRegSlots",
    "const fpRegSlots = %RawDownCast<RawPtr<float64>>(StackSlotPtr( 2 * torque_internal::SizeOf<float64>(), torque_internal::SizeOf<float64>()))",
  ),
  (
    "const",
    "stackSlots",
    "const stackSlots = locationAllocator.GetAlignedStackEnd(StackAlignmentInBytes())",
  ),
  (
    "const",
    "popCount",
    "const popCount = (Convert<intptr>(stackSlots) - Convert<intptr>(stackParamStart)) / torque_internal::SizeOf<intptr>() + numFixedSlots",
  ),
  (
    "const",
    "wasmToJSResult",
    "const wasmToJSResult = WasmToJSResult",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 24);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 22);
};
