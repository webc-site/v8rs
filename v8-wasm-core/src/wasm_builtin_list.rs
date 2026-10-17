// 由 tool/src/wasm/wasm-builtin-list.h.js 从 v8/src/wasm/wasm-builtin-list.h 生成; 修订就地改, 复跑覆盖
// wasm builtin 全表: 远跳表槽位序 (陷阱段对齐 compiler::TrapId) + 非跳表 builtin

use strum::VariantArray;

/// 跳表槽位条目 (陷阱 + 普通 builtin, 顺序 = 原版 WASM_BUILTINS_WITH_JUMP_TABLE_SLOT 展开)
/// 注: 前 21 项为 FOREACH_WASM_TRAPREASON (VTRAP), 与 compiler::TrapId 序号对齐, 顺序敏感。
/// variant 采 rust 惯例命名; 原版 V(NAME) 名 (含下划线形态) 以 name() 字符串值 + 各条 /// v8: doc 锚保留可追溯
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VariantArray)]
#[repr(u8)]
pub enum WasmFarJumpBuiltin {
  /// VTRAP(TrapUnreachable) → Builtin::kThrowWasmUnreachable
  ThrowWasmUnreachable = 0,
  /// VTRAP(TrapMemOutOfBounds) → Builtin::kThrowWasmMemOutOfBounds
  ThrowWasmMemOutOfBounds = 1,
  /// VTRAP(TrapUnalignedAccess) → Builtin::kThrowWasmUnalignedAccess
  ThrowWasmUnalignedAccess = 2,
  /// VTRAP(TrapDivByZero) → Builtin::kThrowWasmDivByZero
  ThrowWasmDivByZero = 3,
  /// VTRAP(TrapDivUnrepresentable) → Builtin::kThrowWasmDivUnrepresentable
  ThrowWasmDivUnrepresentable = 4,
  /// VTRAP(TrapRemByZero) → Builtin::kThrowWasmRemByZero
  ThrowWasmRemByZero = 5,
  /// VTRAP(TrapFloatUnrepresentable) → Builtin::kThrowWasmFloatUnrepresentable
  ThrowWasmFloatUnrepresentable = 6,
  /// VTRAP(TrapNullFunc) → Builtin::kThrowWasmNullFunc
  ThrowWasmNullFunc = 7,
  /// VTRAP(TrapFuncSigMismatch) → Builtin::kThrowWasmFuncSigMismatch
  ThrowWasmFuncSigMismatch = 8,
  /// VTRAP(TrapDataSegmentOutOfBounds) → Builtin::kThrowWasmDataSegmentOutOfBounds
  ThrowWasmDataSegmentOutOfBounds = 9,
  /// VTRAP(TrapElementSegmentOutOfBounds) → Builtin::kThrowWasmElementSegmentOutOfBounds
  ThrowWasmElementSegmentOutOfBounds = 10,
  /// VTRAP(TrapTableOutOfBounds) → Builtin::kThrowWasmTableOutOfBounds
  ThrowWasmTableOutOfBounds = 11,
  /// VTRAP(TrapRethrowNull) → Builtin::kThrowWasmRethrowNull
  ThrowWasmRethrowNull = 12,
  /// VTRAP(TrapNullDereference) → Builtin::kThrowWasmNullDereference
  ThrowWasmNullDereference = 13,
  /// VTRAP(TrapIllegalCast) → Builtin::kThrowWasmIllegalCast
  ThrowWasmIllegalCast = 14,
  /// VTRAP(TrapArrayOutOfBounds) → Builtin::kThrowWasmArrayOutOfBounds
  ThrowWasmArrayOutOfBounds = 15,
  /// VTRAP(TrapArrayTooLarge) → Builtin::kThrowWasmArrayTooLarge
  ThrowWasmArrayTooLarge = 16,
  /// VTRAP(TrapResume) → Builtin::kThrowWasmResume
  ThrowWasmResume = 17,
  /// VTRAP(TrapSuspend) → Builtin::kThrowWasmSuspend
  ThrowWasmSuspend = 18,
  /// VTRAP(TrapSwitch) → Builtin::kThrowWasmSwitch
  ThrowWasmSwitch = 19,
  /// VTRAP(TrapStringOffsetOutOfBounds) → Builtin::kThrowWasmStringOffsetOutOfBounds
  ThrowWasmStringOffsetOutOfBounds = 20,
  /// V(WasmCompileLazy)
  WasmCompileLazy = 21,
  /// V(WasmTriggerTierUp)
  WasmTriggerTierUp = 22,
  /// V(WasmLiftoffFrameSetup)
  WasmLiftoffFrameSetup = 23,
  /// V(WasmDebugBreak)
  WasmDebugBreak = 24,
  /// V(WasmTrapHandlerThrowTrap)
  WasmTrapHandlerThrowTrap = 25,
  /// V(WasmInt32ToHeapNumber)
  WasmInt32ToHeapNumber = 26,
  /// V(WasmInt32ToSharedHeapNumber)
  WasmInt32ToSharedHeapNumber = 27,
  /// V(WasmFloat64ToString)
  WasmFloat64ToString = 28,
  /// V(WasmStringToDouble)
  WasmStringToDouble = 29,
  /// V(WasmIntToString)
  WasmIntToString = 30,
  /// V(WasmTaggedNonSmiToInt32)
  WasmTaggedNonSmiToInt32 = 31,
  /// V(WasmFloat32ToNumber)
  WasmFloat32ToNumber = 32,
  /// V(WasmFloat64ToNumber)
  WasmFloat64ToNumber = 33,
  /// V(WasmTaggedToFloat64)
  WasmTaggedToFloat64 = 34,
  /// V(WasmAllocateJSArray)
  WasmAllocateJSArray = 35,
  /// V(WasmI32AtomicWait)
  WasmI32AtomicWait = 36,
  /// V(WasmI64AtomicWait)
  WasmI64AtomicWait = 37,
  /// V(WasmGetOwnProperty)
  WasmGetOwnProperty = 38,
  /// V(WasmRefFunc)
  WasmRefFunc = 39,
  /// V(WasmInternalFunctionCreateExternal)
  WasmInternalFunctionCreateExternal = 40,
  /// V(WasmMemoryGrow)
  WasmMemoryGrow = 41,
  /// V(WasmTableInit)
  WasmTableInit = 42,
  /// V(WasmTableCopy)
  WasmTableCopy = 43,
  /// V(WasmTableFill)
  WasmTableFill = 44,
  /// V(WasmTableGrow)
  WasmTableGrow = 45,
  /// V(WasmTableGet)
  WasmTableGet = 46,
  /// V(WasmTableSet)
  WasmTableSet = 47,
  /// V(WasmTableGetFuncRef)
  WasmTableGetFuncRef = 48,
  /// V(WasmTableSetFuncRef)
  WasmTableSetFuncRef = 49,
  /// V(WasmFunctionTableGet)
  WasmFunctionTableGet = 50,
  /// V(WasmStackGuard)
  WasmStackGuard = 51,
  /// V(WasmStackGuardLoop)
  WasmStackGuardLoop = 52,
  /// V(WasmGrowableStackGuard)
  WasmGrowableStackGuard = 53,
  /// V(WasmStackOverflow)
  WasmStackOverflow = 54,
  /// V(WasmAllocateFixedArray)
  WasmAllocateFixedArray = 55,
  /// V(WasmThrow)
  WasmThrow = 56,
  /// V(WasmRethrow)
  WasmRethrow = 57,
  /// V(WasmThrowRef)
  WasmThrowRef = 58,
  /// V(WasmRethrowExplicitContext)
  WasmRethrowExplicitContext = 59,
  /// V(WasmHandleStackOverflow)
  WasmHandleStackOverflow = 60,
  /// V(WasmReturnFromSegment)
  WasmReturnFromSegment = 61,
  /// V(WasmTraceEnter)
  WasmTraceEnter = 62,
  /// V(WasmTraceExit)
  WasmTraceExit = 63,
  /// V(WasmTraceMemory)
  WasmTraceMemory = 64,
  /// V(WasmTraceGlobal)
  WasmTraceGlobal = 65,
  /// V(BigIntToI32Pair)
  BigIntToI32Pair = 66,
  /// V(BigIntToI64)
  BigIntToI64 = 67,
  /// V(CallRefIC)
  CallRefIC = 68,
  /// V(CallIndirectIC)
  CallIndirectIC = 69,
  /// V(DoubleToI)
  DoubleToI = 70,
  /// V(I32PairToBigInt)
  I32PairToBigInt = 71,
  /// V(I64ToBigInt)
  I64ToBigInt = 72,
  /// V(RecordWriteSaveFP)
  RecordWriteSaveFP = 73,
  /// V(RecordWriteIgnoreFP)
  RecordWriteIgnoreFP = 74,
  /// V(ThrowDataViewTypeError)
  ThrowDataViewTypeError = 75,
  /// V(ThrowDataViewDetachedError)
  ThrowDataViewDetachedError = 76,
  /// V(ThrowDataViewOutOfBounds)
  ThrowDataViewOutOfBounds = 77,
  /// V(ThrowIndexOfCalledOnNull)
  ThrowIndexOfCalledOnNull = 78,
  /// V(ThrowToLowerCaseCalledOnNull)
  ThrowToLowerCaseCalledOnNull = 79,
  /// v8: V(WasmAllocateArray_Uninitialized)
  WasmAllocateArrayUninitialized = 80,
  /// v8: V(WasmAllocateSharedArray_Uninitialized)
  WasmAllocateSharedArrayUninitialized = 81,
  /// V(WasmArrayCopy)
  WasmArrayCopy = 82,
  /// V(WasmArrayNewSegment)
  WasmArrayNewSegment = 83,
  /// V(WasmArrayInitSegment)
  WasmArrayInitSegment = 84,
  /// V(WasmAllocateStructWithRtt)
  WasmAllocateStructWithRtt = 85,
  /// V(WasmAllocateDescriptorStruct)
  WasmAllocateDescriptorStruct = 86,
  /// V(WasmAllocateSharedStructWithRtt)
  WasmAllocateSharedStructWithRtt = 87,
  /// V(WasmWaitqueueNew)
  WasmWaitqueueNew = 88,
  /// V(WasmManagedObjectWait32)
  WasmManagedObjectWait32 = 89,
  /// V(WasmManagedObjectWait64)
  WasmManagedObjectWait64 = 90,
  /// V(WasmManagedObjectWaitRef)
  WasmManagedObjectWaitRef = 91,
  /// V(WasmConfigureAllPrototypesOpt)
  WasmConfigureAllPrototypesOpt = 92,
  /// V(WasmOnStackReplace)
  WasmOnStackReplace = 93,
  /// V(WasmReject)
  WasmReject = 94,
  /// V(WasmStringNewWtf8)
  WasmStringNewWtf8 = 95,
  /// V(WasmStringNewWtf16)
  WasmStringNewWtf16 = 96,
  /// V(WasmStringConst)
  WasmStringConst = 97,
  /// V(WasmStringMeasureUtf8)
  WasmStringMeasureUtf8 = 98,
  /// V(WasmStringMeasureWtf8)
  WasmStringMeasureWtf8 = 99,
  /// V(WasmStringEncodeWtf8)
  WasmStringEncodeWtf8 = 100,
  /// V(WasmStringEncodeWtf16)
  WasmStringEncodeWtf16 = 101,
  /// V(WasmStringConcat)
  WasmStringConcat = 102,
  /// V(WasmStringEqual)
  WasmStringEqual = 103,
  /// V(WasmStringIsUSVSequence)
  WasmStringIsUSVSequence = 104,
  /// V(WasmStringAsWtf16)
  WasmStringAsWtf16 = 105,
  /// V(WasmStringViewWtf16GetCodeUnit)
  WasmStringViewWtf16GetCodeUnit = 106,
  /// V(WasmStringCodePointAt)
  WasmStringCodePointAt = 107,
  /// V(WasmStringViewWtf16Encode)
  WasmStringViewWtf16Encode = 108,
  /// V(WasmStringViewWtf16Slice)
  WasmStringViewWtf16Slice = 109,
  /// V(WasmStringSliceShared)
  WasmStringSliceShared = 110,
  /// V(WasmStringNewWtf8Array)
  WasmStringNewWtf8Array = 111,
  /// V(WasmStringNewWtf16Array)
  WasmStringNewWtf16Array = 112,
  /// V(WasmStringEncodeWtf8Array)
  WasmStringEncodeWtf8Array = 113,
  /// V(WasmStringToUtf8Array)
  WasmStringToUtf8Array = 114,
  /// V(WasmStringEncodeWtf16Array)
  WasmStringEncodeWtf16Array = 115,
  /// V(WasmStringAsWtf8)
  WasmStringAsWtf8 = 116,
  /// V(WasmStringViewWtf8Advance)
  WasmStringViewWtf8Advance = 117,
  /// V(WasmStringViewWtf8Encode)
  WasmStringViewWtf8Encode = 118,
  /// V(WasmStringViewWtf8Slice)
  WasmStringViewWtf8Slice = 119,
  /// V(WasmStringAsIter)
  WasmStringAsIter = 120,
  /// V(WasmStringViewIterNext)
  WasmStringViewIterNext = 121,
  /// V(WasmStringViewIterAdvance)
  WasmStringViewIterAdvance = 122,
  /// V(WasmStringViewIterRewind)
  WasmStringViewIterRewind = 123,
  /// V(WasmStringViewIterSlice)
  WasmStringViewIterSlice = 124,
  /// V(WasmStringCompare)
  WasmStringCompare = 125,
  /// V(WasmStringIndexOf)
  WasmStringIndexOf = 126,
  /// V(WasmStringFromCodePoint)
  WasmStringFromCodePoint = 127,
  /// V(WasmStringHash)
  WasmStringHash = 128,
  /// V(WasmAnyConvertExtern)
  WasmAnyConvertExtern = 129,
  /// V(WasmAnyConvertExternShared)
  WasmAnyConvertExternShared = 130,
  /// V(WasmStringFromDataSegment)
  WasmStringFromDataSegment = 131,
  /// v8: V(WasmStringAdd_NoMapCheck)
  WasmStringAddNoMapCheck = 132,
  /// v8: V(WasmStringAdd_NoMapCheck_Shared)
  WasmStringAddNoMapCheckShared = 133,
  /// V(DebugPrintWord32)
  DebugPrintWord32 = 134,
  /// V(DebugPrintWord64)
  DebugPrintWord64 = 135,
  /// V(DebugPrintFloat32)
  DebugPrintFloat32 = 136,
  /// V(DebugPrintFloat64)
  DebugPrintFloat64 = 137,
  /// V(DebugPrintObject)
  DebugPrintObject = 138,
  /// V(WasmFastApiCallTypeCheckAndUpdateIC)
  WasmFastApiCallTypeCheckAndUpdateIC = 139,
  /// V(WasmFXResume)
  WasmFXResume = 140,
  /// V(WasmFXResumeThrow)
  WasmFXResumeThrow = 141,
  /// V(WasmFXResumeThrowRef)
  WasmFXResumeThrowRef = 142,
  /// V(WasmFXSuspend)
  WasmFXSuspend = 143,
  /// V(WasmFXSwitch)
  WasmFXSwitch = 144,
  /// v8: V(DeoptimizationEntry_Eager)
  DeoptimizationEntryEager = 145,
  /// V(WasmLiftoffDeoptFinish)
  WasmLiftoffDeoptFinish = 146,
  /// V(WasmPropagateException)
  WasmPropagateException = 147,
  /// V(WasmLiftoffIsEqRefUnshared)
  WasmLiftoffIsEqRefUnshared = 148,
  /// V(WasmLiftoffIsArrayRefUnshared)
  WasmLiftoffIsArrayRefUnshared = 149,
  /// V(WasmLiftoffIsStructRefUnshared)
  WasmLiftoffIsStructRefUnshared = 150,
  /// V(WasmLiftoffCastEqRefUnshared)
  WasmLiftoffCastEqRefUnshared = 151,
  /// V(WasmLiftoffCastArrayRefUnshared)
  WasmLiftoffCastArrayRefUnshared = 152,
  /// V(WasmLiftoffCastStructRefUnshared)
  WasmLiftoffCastStructRefUnshared = 153,
  /// V(WasmTypeAssertionFailed)
  WasmTypeAssertionFailed = 154,
}

/// 跳表条目总数 (21 陷阱 + 134 builtin, 对应 WASM_BUILTINS_WITH_JUMP_TABLE_SLOT 展开长度)
pub const WASM_FAR_JUMP_BUILTIN_COUNT: usize = 155;

/// 陷阱段条数 (FOREACH_WASM_TRAPREASON)
pub const TRAP_SEGMENT_COUNT: usize = 21;

/// 非跳表 builtin (对应 WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT, 6 条)
#[rustfmt::skip]
pub static WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT: &[&str] = &[
  "WasmAllocateInYoungGeneration",
  "WasmAllocateInOldGeneration",
  "WasmAllocateInSharedHeap",
  "WasmJSStringEqual",
  "WasmToJsWrapperInvalidSig",
  "WasmTrap",
];

impl WasmFarJumpBuiltin {
  /// builtin 名 (对应 V(NAME) 原文)
  #[must_use]
  pub const fn builtin_name(self) -> &'static str {
    match self {
      Self::ThrowWasmUnreachable => "ThrowWasmUnreachable",
      Self::ThrowWasmMemOutOfBounds => "ThrowWasmMemOutOfBounds",
      Self::ThrowWasmUnalignedAccess => "ThrowWasmUnalignedAccess",
      Self::ThrowWasmDivByZero => "ThrowWasmDivByZero",
      Self::ThrowWasmDivUnrepresentable => "ThrowWasmDivUnrepresentable",
      Self::ThrowWasmRemByZero => "ThrowWasmRemByZero",
      Self::ThrowWasmFloatUnrepresentable => "ThrowWasmFloatUnrepresentable",
      Self::ThrowWasmNullFunc => "ThrowWasmNullFunc",
      Self::ThrowWasmFuncSigMismatch => "ThrowWasmFuncSigMismatch",
      Self::ThrowWasmDataSegmentOutOfBounds => "ThrowWasmDataSegmentOutOfBounds",
      Self::ThrowWasmElementSegmentOutOfBounds => "ThrowWasmElementSegmentOutOfBounds",
      Self::ThrowWasmTableOutOfBounds => "ThrowWasmTableOutOfBounds",
      Self::ThrowWasmRethrowNull => "ThrowWasmRethrowNull",
      Self::ThrowWasmNullDereference => "ThrowWasmNullDereference",
      Self::ThrowWasmIllegalCast => "ThrowWasmIllegalCast",
      Self::ThrowWasmArrayOutOfBounds => "ThrowWasmArrayOutOfBounds",
      Self::ThrowWasmArrayTooLarge => "ThrowWasmArrayTooLarge",
      Self::ThrowWasmResume => "ThrowWasmResume",
      Self::ThrowWasmSuspend => "ThrowWasmSuspend",
      Self::ThrowWasmSwitch => "ThrowWasmSwitch",
      Self::ThrowWasmStringOffsetOutOfBounds => "ThrowWasmStringOffsetOutOfBounds",
      Self::WasmCompileLazy => "WasmCompileLazy",
      Self::WasmTriggerTierUp => "WasmTriggerTierUp",
      Self::WasmLiftoffFrameSetup => "WasmLiftoffFrameSetup",
      Self::WasmDebugBreak => "WasmDebugBreak",
      Self::WasmTrapHandlerThrowTrap => "WasmTrapHandlerThrowTrap",
      Self::WasmInt32ToHeapNumber => "WasmInt32ToHeapNumber",
      Self::WasmInt32ToSharedHeapNumber => "WasmInt32ToSharedHeapNumber",
      Self::WasmFloat64ToString => "WasmFloat64ToString",
      Self::WasmStringToDouble => "WasmStringToDouble",
      Self::WasmIntToString => "WasmIntToString",
      Self::WasmTaggedNonSmiToInt32 => "WasmTaggedNonSmiToInt32",
      Self::WasmFloat32ToNumber => "WasmFloat32ToNumber",
      Self::WasmFloat64ToNumber => "WasmFloat64ToNumber",
      Self::WasmTaggedToFloat64 => "WasmTaggedToFloat64",
      Self::WasmAllocateJSArray => "WasmAllocateJSArray",
      Self::WasmI32AtomicWait => "WasmI32AtomicWait",
      Self::WasmI64AtomicWait => "WasmI64AtomicWait",
      Self::WasmGetOwnProperty => "WasmGetOwnProperty",
      Self::WasmRefFunc => "WasmRefFunc",
      Self::WasmInternalFunctionCreateExternal => "WasmInternalFunctionCreateExternal",
      Self::WasmMemoryGrow => "WasmMemoryGrow",
      Self::WasmTableInit => "WasmTableInit",
      Self::WasmTableCopy => "WasmTableCopy",
      Self::WasmTableFill => "WasmTableFill",
      Self::WasmTableGrow => "WasmTableGrow",
      Self::WasmTableGet => "WasmTableGet",
      Self::WasmTableSet => "WasmTableSet",
      Self::WasmTableGetFuncRef => "WasmTableGetFuncRef",
      Self::WasmTableSetFuncRef => "WasmTableSetFuncRef",
      Self::WasmFunctionTableGet => "WasmFunctionTableGet",
      Self::WasmStackGuard => "WasmStackGuard",
      Self::WasmStackGuardLoop => "WasmStackGuardLoop",
      Self::WasmGrowableStackGuard => "WasmGrowableStackGuard",
      Self::WasmStackOverflow => "WasmStackOverflow",
      Self::WasmAllocateFixedArray => "WasmAllocateFixedArray",
      Self::WasmThrow => "WasmThrow",
      Self::WasmRethrow => "WasmRethrow",
      Self::WasmThrowRef => "WasmThrowRef",
      Self::WasmRethrowExplicitContext => "WasmRethrowExplicitContext",
      Self::WasmHandleStackOverflow => "WasmHandleStackOverflow",
      Self::WasmReturnFromSegment => "WasmReturnFromSegment",
      Self::WasmTraceEnter => "WasmTraceEnter",
      Self::WasmTraceExit => "WasmTraceExit",
      Self::WasmTraceMemory => "WasmTraceMemory",
      Self::WasmTraceGlobal => "WasmTraceGlobal",
      Self::BigIntToI32Pair => "BigIntToI32Pair",
      Self::BigIntToI64 => "BigIntToI64",
      Self::CallRefIC => "CallRefIC",
      Self::CallIndirectIC => "CallIndirectIC",
      Self::DoubleToI => "DoubleToI",
      Self::I32PairToBigInt => "I32PairToBigInt",
      Self::I64ToBigInt => "I64ToBigInt",
      Self::RecordWriteSaveFP => "RecordWriteSaveFP",
      Self::RecordWriteIgnoreFP => "RecordWriteIgnoreFP",
      Self::ThrowDataViewTypeError => "ThrowDataViewTypeError",
      Self::ThrowDataViewDetachedError => "ThrowDataViewDetachedError",
      Self::ThrowDataViewOutOfBounds => "ThrowDataViewOutOfBounds",
      Self::ThrowIndexOfCalledOnNull => "ThrowIndexOfCalledOnNull",
      Self::ThrowToLowerCaseCalledOnNull => "ThrowToLowerCaseCalledOnNull",
      Self::WasmAllocateArrayUninitialized => "WasmAllocateArray_Uninitialized",
      Self::WasmAllocateSharedArrayUninitialized => "WasmAllocateSharedArray_Uninitialized",
      Self::WasmArrayCopy => "WasmArrayCopy",
      Self::WasmArrayNewSegment => "WasmArrayNewSegment",
      Self::WasmArrayInitSegment => "WasmArrayInitSegment",
      Self::WasmAllocateStructWithRtt => "WasmAllocateStructWithRtt",
      Self::WasmAllocateDescriptorStruct => "WasmAllocateDescriptorStruct",
      Self::WasmAllocateSharedStructWithRtt => "WasmAllocateSharedStructWithRtt",
      Self::WasmWaitqueueNew => "WasmWaitqueueNew",
      Self::WasmManagedObjectWait32 => "WasmManagedObjectWait32",
      Self::WasmManagedObjectWait64 => "WasmManagedObjectWait64",
      Self::WasmManagedObjectWaitRef => "WasmManagedObjectWaitRef",
      Self::WasmConfigureAllPrototypesOpt => "WasmConfigureAllPrototypesOpt",
      Self::WasmOnStackReplace => "WasmOnStackReplace",
      Self::WasmReject => "WasmReject",
      Self::WasmStringNewWtf8 => "WasmStringNewWtf8",
      Self::WasmStringNewWtf16 => "WasmStringNewWtf16",
      Self::WasmStringConst => "WasmStringConst",
      Self::WasmStringMeasureUtf8 => "WasmStringMeasureUtf8",
      Self::WasmStringMeasureWtf8 => "WasmStringMeasureWtf8",
      Self::WasmStringEncodeWtf8 => "WasmStringEncodeWtf8",
      Self::WasmStringEncodeWtf16 => "WasmStringEncodeWtf16",
      Self::WasmStringConcat => "WasmStringConcat",
      Self::WasmStringEqual => "WasmStringEqual",
      Self::WasmStringIsUSVSequence => "WasmStringIsUSVSequence",
      Self::WasmStringAsWtf16 => "WasmStringAsWtf16",
      Self::WasmStringViewWtf16GetCodeUnit => "WasmStringViewWtf16GetCodeUnit",
      Self::WasmStringCodePointAt => "WasmStringCodePointAt",
      Self::WasmStringViewWtf16Encode => "WasmStringViewWtf16Encode",
      Self::WasmStringViewWtf16Slice => "WasmStringViewWtf16Slice",
      Self::WasmStringSliceShared => "WasmStringSliceShared",
      Self::WasmStringNewWtf8Array => "WasmStringNewWtf8Array",
      Self::WasmStringNewWtf16Array => "WasmStringNewWtf16Array",
      Self::WasmStringEncodeWtf8Array => "WasmStringEncodeWtf8Array",
      Self::WasmStringToUtf8Array => "WasmStringToUtf8Array",
      Self::WasmStringEncodeWtf16Array => "WasmStringEncodeWtf16Array",
      Self::WasmStringAsWtf8 => "WasmStringAsWtf8",
      Self::WasmStringViewWtf8Advance => "WasmStringViewWtf8Advance",
      Self::WasmStringViewWtf8Encode => "WasmStringViewWtf8Encode",
      Self::WasmStringViewWtf8Slice => "WasmStringViewWtf8Slice",
      Self::WasmStringAsIter => "WasmStringAsIter",
      Self::WasmStringViewIterNext => "WasmStringViewIterNext",
      Self::WasmStringViewIterAdvance => "WasmStringViewIterAdvance",
      Self::WasmStringViewIterRewind => "WasmStringViewIterRewind",
      Self::WasmStringViewIterSlice => "WasmStringViewIterSlice",
      Self::WasmStringCompare => "WasmStringCompare",
      Self::WasmStringIndexOf => "WasmStringIndexOf",
      Self::WasmStringFromCodePoint => "WasmStringFromCodePoint",
      Self::WasmStringHash => "WasmStringHash",
      Self::WasmAnyConvertExtern => "WasmAnyConvertExtern",
      Self::WasmAnyConvertExternShared => "WasmAnyConvertExternShared",
      Self::WasmStringFromDataSegment => "WasmStringFromDataSegment",
      Self::WasmStringAddNoMapCheck => "WasmStringAdd_NoMapCheck",
      Self::WasmStringAddNoMapCheckShared => "WasmStringAdd_NoMapCheck_Shared",
      Self::DebugPrintWord32 => "DebugPrintWord32",
      Self::DebugPrintWord64 => "DebugPrintWord64",
      Self::DebugPrintFloat32 => "DebugPrintFloat32",
      Self::DebugPrintFloat64 => "DebugPrintFloat64",
      Self::DebugPrintObject => "DebugPrintObject",
      Self::WasmFastApiCallTypeCheckAndUpdateIC => "WasmFastApiCallTypeCheckAndUpdateIC",
      Self::WasmFXResume => "WasmFXResume",
      Self::WasmFXResumeThrow => "WasmFXResumeThrow",
      Self::WasmFXResumeThrowRef => "WasmFXResumeThrowRef",
      Self::WasmFXSuspend => "WasmFXSuspend",
      Self::WasmFXSwitch => "WasmFXSwitch",
      Self::DeoptimizationEntryEager => "DeoptimizationEntry_Eager",
      Self::WasmLiftoffDeoptFinish => "WasmLiftoffDeoptFinish",
      Self::WasmPropagateException => "WasmPropagateException",
      Self::WasmLiftoffIsEqRefUnshared => "WasmLiftoffIsEqRefUnshared",
      Self::WasmLiftoffIsArrayRefUnshared => "WasmLiftoffIsArrayRefUnshared",
      Self::WasmLiftoffIsStructRefUnshared => "WasmLiftoffIsStructRefUnshared",
      Self::WasmLiftoffCastEqRefUnshared => "WasmLiftoffCastEqRefUnshared",
      Self::WasmLiftoffCastArrayRefUnshared => "WasmLiftoffCastArrayRefUnshared",
      Self::WasmLiftoffCastStructRefUnshared => "WasmLiftoffCastStructRefUnshared",
      Self::WasmTypeAssertionFailed => "WasmTypeAssertionFailed",
    }
  }

  /// 全表序 (对位原版 detail::InitBuiltinToFarJumpTableIndex: 槽位序 = 展开序)
  #[must_use]
  pub const fn all() -> &'static [Self] {
    Self::VARIANTS
  }

  /// 是否陷阱段 (VTRAP)
  #[must_use]
  pub const fn is_trap(self) -> bool {
    (self as usize) < TRAP_SEGMENT_COUNT
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(WASM_FAR_JUMP_BUILTIN_COUNT == 155);
  assert!(TRAP_SEGMENT_COUNT == 21);
  assert!((WasmFarJumpBuiltin::ThrowWasmUnreachable as usize) == 0);
  assert!((WasmFarJumpBuiltin::WasmCompileLazy as usize) == 21);
  assert!((WasmFarJumpBuiltin::WasmCompileLazy as usize) == TRAP_SEGMENT_COUNT);
  assert!(WasmFarJumpBuiltin::ThrowWasmUnreachable.is_trap());
  assert!(!WasmFarJumpBuiltin::WasmCompileLazy.is_trap());
  assert!(WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT.len() == 6);
};
