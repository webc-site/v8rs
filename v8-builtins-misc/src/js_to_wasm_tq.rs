// 由 tool/src/builtins/js-to-wasm.tq.js 从 v8/src/builtins/js-to-wasm.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 41 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "runtime", "namespace runtime"),
  ("namespace", "wasm", "namespace wasm"),
  ("const", "bigint", "const bigint = ToBigInt(context, input)"),
  (
    "const",
    "digit",
    "const digit = bigint::LoadBigIntDigit(bigint, 0)",
  ),
  ("macro", "AllocStack", "macro AllocStack(): void"),
  ("macro", "AllocGP", "macro AllocGP(): void"),
  ("macro", "AllocFP32", "macro AllocFP32(): void"),
  ("macro", "AllocFP64", "macro AllocFP64(): void"),
  ("macro", "GetSize", "macro GetSize(): intptr"),
  ("macro", "StartRefs", "macro StartRefs(): void"),
  ("macro", "GetStackSlot", "macro GetStackSlot(): &intptr"),
  ("macro", "GetGPSlot", "macro GetGPSlot(): &intptr"),
  ("macro", "GetFP32Slot", "macro GetFP32Slot(): &intptr"),
  (
    "macro",
    "GetRemainingFPRegs",
    "macro GetRemainingFPRegs(): intptr",
  ),
  ("macro", "GetFP64Slot", "macro GetFP64Slot(): &intptr"),
  ("macro", "StartRefs", "macro StartRefs(): void"),
  ("macro", "GetStackEnd", "macro GetStackEnd(): RawPtr"),
  (
    "macro",
    "GetAlignedStackEnd",
    "macro GetAlignedStackEnd(alignment: intptr): RawPtr",
  ),
  (
    "macro",
    "CheckBufferOverflow",
    "macro CheckBufferOverflow(): void",
  ),
  (
    "const",
    "isGeneric",
    "const isGeneric = (targetType & kValueTypeHasIndexBit) == 0",
  ),
  (
    "const",
    "functionData",
    "const functionData = target.shared_function_info.wasm_exported_function_data",
  ),
  (
    "const",
    "switchStack",
    "const switchStack = promise == Promise::kPromise || promise == Promise::kStressSwitch",
  ),
  (
    "const",
    "internalFunction",
    "const internalFunction = functionData.internal",
  ),
  ("const", "sig", "const sig = internalFunction.sig"),
  (
    "const",
    "implicitArg",
    "const implicitArg: WasmImportData|WasmTrustedInstanceData = internalFunction.implicit_arg",
  ),
  (
    "const",
    "paramCount",
    "const paramCount = *GetRefAt<intptr>(sig, kWasmParameterCountOffset)",
  ),
  (
    "const",
    "returnCount",
    "const returnCount = *GetRefAt<intptr>(sig, kWasmReturnCountOffset)",
  ),
  (
    "const",
    "reps",
    "const reps = *GetRefAt<RawPtr>(sig, kWasmSigTypesOffset)",
  ),
  (
    "const",
    "sigTypes",
    "const sigTypes = torque_internal::unsafe::NewOffHeapConstSlice( %RawDownCast<RawPtr<uint32>>(reps), paramCount + returnCount)",
  ),
  (
    "const",
    "paramTypes",
    "const paramTypes = Subslice(sigTypes, returnCount, paramCount) otherwise unreachable",
  ),
  (
    "const",
    "kMaxOnStackParamCount",
    "const kMaxOnStackParamCount = 10",
  ),
  (
    "const",
    "paramStart",
    "const paramStart = paramBuffer.GCUnsafeRawPtr()",
  ),
  (
    "const",
    "paramEnd",
    "const paramEnd = locationAllocator.GetStackEnd()",
  ),
  (
    "const",
    "callTarget",
    "const callTarget = functionData.internal.raw_call_target",
  ),
  (
    "const",
    "wrapperBuffer",
    "const wrapperBuffer = %RawDownCast<RawPtr<intptr>>( StackSlotPtr(kWrapperBufferSize, torque_internal::SizeOf<intptr>()))",
  ),
  (
    "const",
    "returnCount",
    "const returnCount = *GetRefAt<int32>( wrapperBuffer, kWrapperBufferReturnCount)",
  ),
  (
    "const",
    "fixedArray",
    "const fixedArray: FixedArray = UnsafeCast<FixedArray>(resultArray.elements)",
  ),
  (
    "const",
    "returnBuffer",
    "const returnBuffer = *GetRefAt<RawPtr>( wrapperBuffer, kWrapperBufferStackReturnBufferStart)",
  ),
  (
    "const",
    "reps",
    "const reps = *GetRefAt<RawPtr>( wrapperBuffer, kWrapperBufferSigRepresentationArray)",
  ),
  (
    "const",
    "retTypes",
    "const retTypes = torque_internal::unsafe::NewOffHeapConstSlice( %RawDownCast<RawPtr<uint32>>(reps), Convert<intptr>(returnCount))",
  ),
  (
    "const",
    "hasRefReturns",
    "const hasRefReturns = *GetRefAt<bool>( wrapperBuffer, kWrapperBufferRefReturnCount)",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 41);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 24);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 15);
};
