// 由 tool/src/builtins/builtins-misc.js 生成; 来源 v8/src/builtins
// 内建名表 + 核心调度元数据 (9 内建)

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// 内建名表 (共 9 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  JSToWasmLazyDeoptContinuation,
  WasmConstructorWrapper,
  WasmFloat32ToNumber,
  WasmFloat64ToNumber,
  WasmFloat64ToString,
  WasmGetOwnProperty,
  WasmMethodWrapper,
  WasmToJsWrapperCSA,
  WasmToJsWrapperInvalidSig,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::JSToWasmLazyDeoptContinuation => "JSToWasmLazyDeoptContinuation",
      Self::WasmConstructorWrapper => "WasmConstructorWrapper",
      Self::WasmFloat32ToNumber => "WasmFloat32ToNumber",
      Self::WasmFloat64ToNumber => "WasmFloat64ToNumber",
      Self::WasmFloat64ToString => "WasmFloat64ToString",
      Self::WasmGetOwnProperty => "WasmGetOwnProperty",
      Self::WasmMethodWrapper => "WasmMethodWrapper",
      Self::WasmToJsWrapperCSA => "WasmToJsWrapperCSA",
      Self::WasmToJsWrapperInvalidSig => "WasmToJsWrapperInvalidSig",
    }
  }

  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取对应的接口描述符符号
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 是否为 Wasm 类型转换内建
  #[inline]
  pub const fn is_conversion(&self) -> bool {
    matches!(
      self,
      Self::WasmFloat32ToNumber | Self::WasmFloat64ToNumber | Self::WasmFloat64ToString
    )
  }

  /// 是否为 Wasm 包装器内建
  #[inline]
  pub const fn is_wrapper(&self) -> bool {
    matches!(
      self,
      Self::WasmConstructorWrapper
        | Self::WasmMethodWrapper
        | Self::WasmToJsWrapperCSA
        | Self::WasmToJsWrapperInvalidSig
    )
  }

  /// 是否为 Wasm 去优化延续内建
  #[inline]
  pub const fn is_deopt(&self) -> bool {
    matches!(self, Self::JSToWasmLazyDeoptContinuation)
  }
}

// 统一提供 9 项零成本内建工厂函数
pub const fn js_to_wasm_lazy_deopt_continuation() -> Builtin {
  Builtin::JSToWasmLazyDeoptContinuation
}

pub const fn wasm_constructor_wrapper() -> Builtin {
  Builtin::WasmConstructorWrapper
}

pub const fn wasm_float32_to_number() -> Builtin {
  Builtin::WasmFloat32ToNumber
}

pub const fn wasm_float64_to_number() -> Builtin {
  Builtin::WasmFloat64ToNumber
}

pub const fn wasm_float64_to_string() -> Builtin {
  Builtin::WasmFloat64ToString
}

pub const fn wasm_get_own_property() -> Builtin {
  Builtin::WasmGetOwnProperty
}

pub const fn wasm_method_wrapper() -> Builtin {
  Builtin::WasmMethodWrapper
}

pub const fn wasm_to_js_wrapper_csa() -> Builtin {
  Builtin::WasmToJsWrapperCSA
}

pub const fn wasm_to_js_wrapper_invalid_sig() -> Builtin {
  Builtin::WasmToJsWrapperInvalidSig
}

/// 来自 wasm 相关 .tq 的声明元数据 (共 191 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("extern macro", "Allocate"),
  ("extern macro", "Allocate"),
  ("extern macro", "Retain"),
  ("macro", "NumberToInt32"),
  ("macro", "NumberToUint32"),
  ("extern macro", "WasmCrossInstanceCallSymbolConstant"),
  ("extern macro", "WasmBuiltinsAssembler"),
  ("extern macro", "WasmBuiltinsAssembler"),
  ("extern macro", "WasmBuiltinsAssembler"),
  ("extern macro", "WasmBuiltinsAssembler"),
  ("extern macro", "WasmBuiltinsAssembler"),
  ("extern macro", "WasmBuiltinsAssembler"),
  ("extern macro", "WasmBuiltinsAssembler"),
  ("extern macro", "WasmBuiltinsAssembler"),
  ("extern macro", "WasmBuiltinsAssembler"),
  ("extern macro", "SwitchToTheCentralStackIfNeeded"),
  ("extern macro", "SwitchFromTheCentralStack"),
  ("macro", "LoadContextFromFrame"),
  ("builtin", "WasmInt32ToHeapNumber"),
  ("builtin", "WasmInt32ToSharedHeapNumber"),
  ("builtin", "WasmFuncRefToJS"),
  ("builtin", "WasmTaggedNonSmiToInt32"),
  ("builtin", "WasmTaggedToFloat64"),
  ("builtin", "WasmTaggedToFloat32"),
  ("builtin", "WasmMemoryGrow"),
  ("builtin", "WasmTableInit"),
  ("builtin", "WasmTableCopy"),
  ("builtin", "WasmTableFill"),
  ("builtin", "WasmTableGrow"),
  ("builtin", "WasmTableGet"),
  ("builtin", "WasmTableSet"),
  ("builtin", "WasmTableGetFuncRef"),
  ("builtin", "WasmFunctionTableGet"),
  ("builtin", "WasmTableSetFuncRef"),
  ("builtin", "WasmRefFunc"),
  ("builtin", "WasmInternalFunctionCreateExternal"),
  ("builtin", "WasmAllocateZeroedFixedArray"),
  ("builtin", "WasmAllocateFixedArray"),
  ("builtin", "WasmLiftoffDeoptFinish"),
  ("builtin", "WasmThrow"),
  ("builtin", "WasmRethrow"),
  ("builtin", "WasmThrowRef"),
  ("builtin", "WasmRethrowExplicitContext"),
  ("builtin", "WasmTriggerTierUp"),
  ("builtin", "WasmGrowableStackGuard"),
  ("builtin", "WasmStackGuard"),
  ("builtin", "WasmStackGuardLoop"),
  ("builtin", "WasmStackOverflow"),
  ("builtin", "WasmTraceGlobal"),
  ("builtin", "WasmTraceMemory"),
  ("builtin", "WasmTraceEnter"),
  ("builtin", "WasmTraceExit"),
  ("builtin", "WasmConfigureAllPrototypesOpt"),
  ("builtin", "WasmAllocateJSArray"),
  ("builtin", "WasmAllocateStructWithRtt"),
  ("builtin", "WasmAllocateDescriptorStruct"),
  ("builtin", "WasmAllocateSharedStructWithRtt"),
  ("builtin", "WasmAllocateArray_Uninitialized"),
  ("builtin", "WasmAllocateSharedArray_Uninitialized"),
  ("builtin", "WasmArrayNewSegment"),
  ("builtin", "WasmArrayInitSegment"),
  ("builtin", "WasmArrayCopy"),
  ("builtin", "WasmUint32ToNumber"),
  ("builtin", "UintPtr53ToNumber"),
  ("macro", "UintPtrToNumberRounding"),
  ("builtin", "WasmI32AtomicWait"),
  ("builtin", "WasmI64AtomicWait"),
  ("builtin", "WasmManagedObjectWait32"),
  ("builtin", "WasmManagedObjectWait64"),
  ("builtin", "WasmManagedObjectWaitRef"),
  ("builtin", "WasmWaitqueueNew"),
  ("macro", "UpdateCallRefOrIndirectIC"),
  ("struct", "TargetAndImplicitArg"),
  ("builtin", "CallRefIC"),
  ("builtin", "CallIndirectIC"),
  ("builtin", "WasmTrap"),
  ("builtin", "ThrowWasmTrapUnreachable"),
  ("builtin", "WasmPropagateException"),
  ("builtin", "ThrowWasmTrapMemOutOfBounds"),
  ("builtin", "ThrowWasmTrapUnalignedAccess"),
  ("builtin", "ThrowWasmTrapDivByZero"),
  ("builtin", "ThrowWasmTrapDivUnrepresentable"),
  ("builtin", "ThrowWasmTrapRemByZero"),
  ("builtin", "ThrowWasmTrapFloatUnrepresentable"),
  ("builtin", "ThrowWasmTrapFuncSigMismatch"),
  ("builtin", "ThrowWasmTrapNullFunc"),
  ("builtin", "ThrowWasmTrapDataSegmentOutOfBounds"),
  ("builtin", "ThrowWasmTrapElementSegmentOutOfBounds"),
  ("builtin", "ThrowWasmTrapTableOutOfBounds"),
  ("builtin", "ThrowWasmTrapRethrowNull"),
  ("builtin", "ThrowWasmTrapNullDereference"),
  ("builtin", "ThrowWasmTrapIllegalCast"),
  ("builtin", "ThrowWasmTrapArrayOutOfBounds"),
  ("builtin", "ThrowWasmTrapArrayTooLarge"),
  ("builtin", "ThrowWasmTrapStringOffsetOutOfBounds"),
  ("builtin", "WasmTypeAssertionFailed"),
  ("builtin", "ThrowWasmTrapResume"),
  ("builtin", "ThrowWasmTrapSuspend"),
  ("builtin", "ThrowWasmTrapSwitch"),
  ("macro", "GetRefAt"),
  ("extern macro", "LoadPointerFromRootRegister"),
  ("builtin", "WasmStringNewWtf8"),
  ("builtin", "WasmStringNewWtf8Array"),
  ("builtin", "WasmStringNewWtf16"),
  ("struct", "TwoByteToOneByteIterator"),
  ("macro", "StringFromTwoByteSlice"),
  ("builtin", "WasmStringNewWtf16Array"),
  ("builtin", "WasmStringFromDataSegment"),
  ("builtin", "WasmStringAsWtf16"),
  ("builtin", "WasmStringConst"),
  ("builtin", "WasmStringMeasureUtf8"),
  ("builtin", "WasmStringMeasureWtf8"),
  ("builtin", "WasmStringEncodeWtf8"),
  ("builtin", "WasmStringEncodeWtf8Array"),
  ("builtin", "WasmStringToUtf8Array"),
  ("builtin", "WasmStringEncodeWtf16"),
  ("builtin", "WasmStringEncodeWtf16Array"),
  ("builtin", "ThrowToLowerCaseCalledOnNull"),
  ("builtin", "ThrowIndexOfCalledOnNull"),
  ("builtin", "ThrowDataViewTypeError"),
  ("builtin", "ThrowDataViewDetachedError"),
  ("builtin", "ThrowDataViewOutOfBounds"),
  ("builtin", "WasmStringConcat"),
  ("builtin", "WasmStringAdd_NoMapCheck_Shared"),
  ("builtin", "WasmStringEqual"),
  ("builtin", "WasmStringIsUSVSequence"),
  ("builtin", "WasmStringAsWtf8"),
  ("macro", "IsWtf8CodepointStart"),
  ("macro", "AlignWtf8PositionForward"),
  ("macro", "AlignWtf8PositionBackward"),
  ("builtin", "WasmStringViewWtf8Advance"),
  ("struct", "NewPositionAndBytesWritten"),
  ("builtin", "WasmStringViewWtf8Encode"),
  ("builtin", "WasmStringViewWtf8Slice"),
  ("builtin", "WasmStringViewWtf16Encode"),
  ("builtin", "WasmStringSliceShared"),
  ("extern macro", "WasmStringViewIterMapConstant"),
  ("builtin", "WasmStringAsIter"),
  ("builtin", "WasmStringAsIter"),
  ("macro", "IsLeadSurrogate"),
  ("macro", "IsTrailSurrogate"),
  ("macro", "CombineSurrogatePair"),
  ("builtin", "WasmStringCodePointAt"),
  ("builtin", "WasmStringViewIterNext"),
  ("builtin", "WasmStringViewIterAdvance"),
  ("builtin", "WasmStringViewIterRewind"),
  ("builtin", "WasmStringViewIterSlice"),
  ("builtin", "WasmIntToString"),
  ("builtin", "WasmStringToDouble"),
  ("builtin", "WasmStringFromCodePoint"),
  ("builtin", "WasmStringHash"),
  ("builtin", "WasmAnyConvertExtern"),
  ("builtin", "WasmAnyConvertExternShared"),
  ("extern macro", "CallOrConstructBuiltinsAssembler"),
  ("builtin", "WasmFastApiCallTypeCheckAndUpdateIC"),
  ("builtin", "WasmStringIndexOf"),
  ("builtin", "WasmLiftoffIsEqRefUnshared"),
  ("builtin", "WasmLiftoffIsArrayRefUnshared"),
  ("builtin", "WasmLiftoffIsStructRefUnshared"),
  ("builtin", "WasmLiftoffCastEqRefUnshared"),
  ("builtin", "WasmLiftoffCastArrayRefUnshared"),
  ("builtin", "WasmLiftoffCastStructRefUnshared"),
  ("struct", "WasmToJSResult"),
  ("extern macro", "StackAlignmentInBytes"),
  ("macro", "HandleF32Returns"),
  ("extern macro", "UniqueIntPtrConstant"),
  ("macro", "Bitcast"),
  ("extern macro", "BitcastFloat32ToInt32"),
  ("extern macro", "BitcastInt32ToFloat32"),
  ("macro", "RefCast"),
  ("macro", "TruncateBigIntToI64"),
  ("struct", "Int64AsInt32Pair"),
  ("extern macro", "BigIntToRawBytes"),
  ("extern macro", "GenericJSToWasmWrapperParamBuffer"),
  ("extern macro", "AllocateBuffer"),
  ("extern macro", "DeallocateBuffer"),
  ("struct", "ReturnSlotAllocator"),
  ("macro", "NewReturnSlotAllocator"),
  ("struct", "LocationAllocator"),
  ("macro", "LocationAllocatorForParams"),
  ("macro", "LocationAllocatorForReturns"),
  ("macro", "JSToWasmObject"),
  ("macro", "HandleF32Params"),
  ("macro", "JSToWasmWrapperHelper"),
  ("macro", "WasmToJSObject"),
  ("builtin", "JSToWasmHandleReturns"),
  ("macro", "Trap"),
  ("macro", "NewInterpreterReturnSlotAllocator"),
  ("macro", "LocationAllocatorForInterpreterParams"),
  ("macro", "LocationAllocatorForWasmInterpreterReturns"),
  ("builtin", "JSToWasmInterpreterHandleReturns"),
];
