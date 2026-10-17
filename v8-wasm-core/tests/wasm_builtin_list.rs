//! wasm builtin 155 全表序锁定: 陷阱段 21 + 跳表 134 + 非跳表 6
//! v8: src/wasm/wasm-builtin-list.h WASM_BUILTINS_WITH_JUMP_TABLE_SLOT 展开序
//! v8: src/wasm/wasm-builtin-list.h detail::InitBuiltinToFarJumpTableIndex (槽位序 = 展开序)
use aok::{OK, Void};
use v8_wasm_core::wasm_builtin_list::{
  TRAP_SEGMENT_COUNT, WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT, WASM_FAR_JUMP_BUILTIN_COUNT,
  WasmFarJumpBuiltin,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 陷阱段 (前 21): VTRAP(TrapX) → ThrowWasmX, 与 FOREACH_WASM_TRAPREASON 序对位
const TRAPS: [&str; 21] = [
  "ThrowWasmUnreachable",
  "ThrowWasmMemOutOfBounds",
  "ThrowWasmUnalignedAccess",
  "ThrowWasmDivByZero",
  "ThrowWasmDivUnrepresentable",
  "ThrowWasmRemByZero",
  "ThrowWasmFloatUnrepresentable",
  "ThrowWasmNullFunc",
  "ThrowWasmFuncSigMismatch",
  "ThrowWasmDataSegmentOutOfBounds",
  "ThrowWasmElementSegmentOutOfBounds",
  "ThrowWasmTableOutOfBounds",
  "ThrowWasmRethrowNull",
  "ThrowWasmNullDereference",
  "ThrowWasmIllegalCast",
  "ThrowWasmArrayOutOfBounds",
  "ThrowWasmArrayTooLarge",
  "ThrowWasmResume",
  "ThrowWasmSuspend",
  "ThrowWasmSwitch",
  "ThrowWasmStringOffsetOutOfBounds",
];

/// 跳表段 (21..155): V(NAME) 逐字, 顺序 = 原版展开序
const JUMP_TABLE: [&str; 134] = [
  "WasmCompileLazy",
  "WasmTriggerTierUp",
  "WasmLiftoffFrameSetup",
  "WasmDebugBreak",
  "WasmTrapHandlerThrowTrap",
  "WasmInt32ToHeapNumber",
  "WasmInt32ToSharedHeapNumber",
  "WasmFloat64ToString",
  "WasmStringToDouble",
  "WasmIntToString",
  "WasmTaggedNonSmiToInt32",
  "WasmFloat32ToNumber",
  "WasmFloat64ToNumber",
  "WasmTaggedToFloat64",
  "WasmAllocateJSArray",
  "WasmI32AtomicWait",
  "WasmI64AtomicWait",
  "WasmGetOwnProperty",
  "WasmRefFunc",
  "WasmInternalFunctionCreateExternal",
  "WasmMemoryGrow",
  "WasmTableInit",
  "WasmTableCopy",
  "WasmTableFill",
  "WasmTableGrow",
  "WasmTableGet",
  "WasmTableSet",
  "WasmTableGetFuncRef",
  "WasmTableSetFuncRef",
  "WasmFunctionTableGet",
  "WasmStackGuard",
  "WasmStackGuardLoop",
  "WasmGrowableStackGuard",
  "WasmStackOverflow",
  "WasmAllocateFixedArray",
  "WasmThrow",
  "WasmRethrow",
  "WasmThrowRef",
  "WasmRethrowExplicitContext",
  "WasmHandleStackOverflow",
  "WasmReturnFromSegment",
  "WasmTraceEnter",
  "WasmTraceExit",
  "WasmTraceMemory",
  "WasmTraceGlobal",
  "BigIntToI32Pair",
  "BigIntToI64",
  "CallRefIC",
  "CallIndirectIC",
  "DoubleToI",
  "I32PairToBigInt",
  "I64ToBigInt",
  "RecordWriteSaveFP",
  "RecordWriteIgnoreFP",
  "ThrowDataViewTypeError",
  "ThrowDataViewDetachedError",
  "ThrowDataViewOutOfBounds",
  "ThrowIndexOfCalledOnNull",
  "ThrowToLowerCaseCalledOnNull",
  "WasmAllocateArray_Uninitialized",
  "WasmAllocateSharedArray_Uninitialized",
  "WasmArrayCopy",
  "WasmArrayNewSegment",
  "WasmArrayInitSegment",
  "WasmAllocateStructWithRtt",
  "WasmAllocateDescriptorStruct",
  "WasmAllocateSharedStructWithRtt",
  "WasmWaitqueueNew",
  "WasmManagedObjectWait32",
  "WasmManagedObjectWait64",
  "WasmManagedObjectWaitRef",
  "WasmConfigureAllPrototypesOpt",
  "WasmOnStackReplace",
  "WasmReject",
  "WasmStringNewWtf8",
  "WasmStringNewWtf16",
  "WasmStringConst",
  "WasmStringMeasureUtf8",
  "WasmStringMeasureWtf8",
  "WasmStringEncodeWtf8",
  "WasmStringEncodeWtf16",
  "WasmStringConcat",
  "WasmStringEqual",
  "WasmStringIsUSVSequence",
  "WasmStringAsWtf16",
  "WasmStringViewWtf16GetCodeUnit",
  "WasmStringCodePointAt",
  "WasmStringViewWtf16Encode",
  "WasmStringViewWtf16Slice",
  "WasmStringSliceShared",
  "WasmStringNewWtf8Array",
  "WasmStringNewWtf16Array",
  "WasmStringEncodeWtf8Array",
  "WasmStringToUtf8Array",
  "WasmStringEncodeWtf16Array",
  "WasmStringAsWtf8",
  "WasmStringViewWtf8Advance",
  "WasmStringViewWtf8Encode",
  "WasmStringViewWtf8Slice",
  "WasmStringAsIter",
  "WasmStringViewIterNext",
  "WasmStringViewIterAdvance",
  "WasmStringViewIterRewind",
  "WasmStringViewIterSlice",
  "WasmStringCompare",
  "WasmStringIndexOf",
  "WasmStringFromCodePoint",
  "WasmStringHash",
  "WasmAnyConvertExtern",
  "WasmAnyConvertExternShared",
  "WasmStringFromDataSegment",
  "WasmStringAdd_NoMapCheck",
  "WasmStringAdd_NoMapCheck_Shared",
  "DebugPrintWord32",
  "DebugPrintWord64",
  "DebugPrintFloat32",
  "DebugPrintFloat64",
  "DebugPrintObject",
  "WasmFastApiCallTypeCheckAndUpdateIC",
  "WasmFXResume",
  "WasmFXResumeThrow",
  "WasmFXResumeThrowRef",
  "WasmFXSuspend",
  "WasmFXSwitch",
  "DeoptimizationEntry_Eager",
  "WasmLiftoffDeoptFinish",
  "WasmPropagateException",
  "WasmLiftoffIsEqRefUnshared",
  "WasmLiftoffIsArrayRefUnshared",
  "WasmLiftoffIsStructRefUnshared",
  "WasmLiftoffCastEqRefUnshared",
  "WasmLiftoffCastArrayRefUnshared",
  "WasmLiftoffCastStructRefUnshared",
  "WasmTypeAssertionFailed",
];

/// 155 全表序锁定: builtin_name 逐一等值 + 总数 + 陷阱段边界
#[test]
fn far_jump_table_order() -> Void {
  assert_eq!(WASM_FAR_JUMP_BUILTIN_COUNT, 155);
  assert_eq!(TRAP_SEGMENT_COUNT, 21);
  let all = WasmFarJumpBuiltin::all();
  assert_eq!(all.len(), 155);
  for (i, name) in TRAPS.iter().enumerate() {
    assert_eq!(all[i].builtin_name(), *name, "陷阱段槽 {i}");
    assert!(all[i].is_trap(), "槽 {i} 应属陷阱段");
  }
  for (i, name) in JUMP_TABLE.iter().enumerate() {
    let slot = i + 21;
    assert_eq!(all[slot].builtin_name(), *name, "跳表段槽 {slot}");
    assert!(!all[slot].is_trap(), "槽 {slot} 不应属陷阱段");
  }
  OK
}

/// 判别值 = 展开槽位序 (对位 detail::InitBuiltinToFarJumpTableIndex: 陷阱段判别值=compiler::TrapId 序号)
#[test]
fn far_jump_discriminant_is_slot() -> Void {
  let all = WasmFarJumpBuiltin::all();
  for (i, &b) in all.iter().enumerate() {
    assert_eq!(b as u8 as usize, i, "槽 {i} 判别值须等于展开序");
  }
  OK
}

/// 非跳表 builtin 6 条 (WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT)
#[test]
fn without_jump_table_slot() -> Void {
  assert_eq!(
    WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT,
    [
      "WasmAllocateInYoungGeneration",
      "WasmAllocateInOldGeneration",
      "WasmAllocateInSharedHeap",
      "WasmJSStringEqual",
      "WasmToJsWrapperInvalidSig",
      "WasmTrap"
    ]
  );
  OK
}

/// u8 溢出对账: 原版 kBuiltinToFarJumpTableIndex 为 array<uint8_t>, next_index++ 超 255 会静默回绕;
/// Rust repr(u8) 编译期拒超载。锁 155 ≤ 256 边界, 防后续列表追加至 256 时静默破对位
#[test]
fn far_jump_u8_overflow_guard() -> Void {
  let all = WasmFarJumpBuiltin::all();
  assert!(
    all.len() <= u8::MAX as usize + 1,
    "跳表槽数不得超 u8 表达域 (原版 uint8_t next_index)"
  );
  assert_eq!(
    all.last().copied(),
    Some(WasmFarJumpBuiltin::WasmTypeAssertionFailed)
  );
  assert_eq!(
    WasmFarJumpBuiltin::WasmTypeAssertionFailed as u8,
    (all.len() - 1) as u8
  );
  OK
}

/// 重复 key 对账: 原版 DEF_INIT_LOOKUP 若同名 builtin 在宏列表重复展开,
/// result[k##NAME] 被二次写覆盖 → 双射破坏; Rust 侧以 builtin_name 全 155 值唯一性锁定
#[test]
fn far_jump_no_duplicate_builtin_name() -> Void {
  let all = WasmFarJumpBuiltin::all();
  let mut seen: Vec<&str> = all.iter().map(|b| b.builtin_name()).collect();
  let total = seen.len();
  seen.sort_unstable();
  seen.dedup();
  assert_eq!(
    seen.len(),
    total,
    "155 条 builtin_name 须两两互异 (对位原版重复 key 覆盖破坏双射)"
  );
  // 陷阱段判别值序 = globals.h FOREACH_WASM_TRAPREASON 展开序 (逐字复核)
  assert_eq!(
    TRAPS[20],
    WasmFarJumpBuiltin::ThrowWasmStringOffsetOutOfBounds.builtin_name()
  );
  OK
}
