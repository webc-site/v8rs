// 由 tool/src/compiler/backend/instruction-codes.h.js 从 v8/src/compiler/backend/x64/instruction-codes-x64.h + instruction-codes.h 生成; 修订就地改, 复跑覆盖
// x64 指令码表: ArchOpcode (公共+架构) 与 AddressingMode

// 公共面见 instruction_codes.rs; kLastArchOpcode/kLastAddressingMode = 条数-1

/// 原版 ArchOpcode (x64, 公共 394 + 架构项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[rustfmt::skip]
pub enum ArchOpcode {
  /// kArchTailCallCodeObject
  ArchTailCallCodeObject = 0,
  /// kArchTailCallAddress
  ArchTailCallAddress = 1,
  /// kArchCallCodeObject
  ArchCallCodeObject = 2,
  /// kArchCallJSFunction
  ArchCallJSFunction = 3,
  /// kArchCallCFunction
  ArchCallCFunction = 4,
  /// kArchCallBuiltinPointer
  ArchCallBuiltinPointer = 5,
  /// kArchPrepareCallCFunction
  ArchPrepareCallCFunction = 6,
  /// kArchSaveCallerRegisters
  ArchSaveCallerRegisters = 7,
  /// kArchRestoreCallerRegisters
  ArchRestoreCallerRegisters = 8,
  /// kArchPrepareTailCall
  ArchPrepareTailCall = 9,
  /// kArchJmp
  ArchJmp = 10,
  /// kArchBinarySearchSwitch
  ArchBinarySearchSwitch = 11,
  /// kArchTableSwitch
  ArchTableSwitch = 12,
  /// kArchNop
  ArchNop = 13,
  /// kArchPause
  ArchPause = 14,
  /// kArchAbortCSADcheck
  ArchAbortCSADcheck = 15,
  /// kArchDebugBreak
  ArchDebugBreak = 16,
  /// kArchComment
  ArchComment = 17,
  /// kArchDeoptimize
  ArchDeoptimize = 18,
  /// kArchRet
  ArchRet = 19,
  /// kArchFramePointer
  ArchFramePointer = 20,
  /// kArchRootPointer
  ArchRootPointer = 21,
  /// kArchParentFramePointer
  ArchParentFramePointer = 22,
  /// kArchTruncateDoubleToI
  ArchTruncateDoubleToI = 23,
  /// kArchStackSlot
  ArchStackSlot = 24,
  /// kArchStackPointerGreaterThan
  ArchStackPointerGreaterThan = 25,
  /// kArchStackCheckOffset
  ArchStackCheckOffset = 26,
  /// kIeee754Float64Acos
  Ieee754Float64Acos = 27,
  /// kIeee754Float64Acosh
  Ieee754Float64Acosh = 28,
  /// kIeee754Float64Asin
  Ieee754Float64Asin = 29,
  /// kIeee754Float64Asinh
  Ieee754Float64Asinh = 30,
  /// kIeee754Float64Atan
  Ieee754Float64Atan = 31,
  /// kIeee754Float64Atanh
  Ieee754Float64Atanh = 32,
  /// kIeee754Float64Atan2
  Ieee754Float64Atan2 = 33,
  /// kIeee754Float64Cbrt
  Ieee754Float64Cbrt = 34,
  /// kIeee754Float64Cos
  Ieee754Float64Cos = 35,
  /// kIeee754Float64Cosh
  Ieee754Float64Cosh = 36,
  /// kIeee754Float64Exp
  Ieee754Float64Exp = 37,
  /// kIeee754Float64Expm1
  Ieee754Float64Expm1 = 38,
  /// kIeee754Float64Log
  Ieee754Float64Log = 39,
  /// kIeee754Float64Log1p
  Ieee754Float64Log1p = 40,
  /// kIeee754Float64Log10
  Ieee754Float64Log10 = 41,
  /// kIeee754Float64Log2
  Ieee754Float64Log2 = 42,
  /// kIeee754Float64Pow
  Ieee754Float64Pow = 43,
  /// kIeee754Float64Sin
  Ieee754Float64Sin = 44,
  /// kIeee754Float64Sinh
  Ieee754Float64Sinh = 45,
  /// kIeee754Float64Tan
  Ieee754Float64Tan = 46,
  /// kIeee754Float64Tanh
  Ieee754Float64Tanh = 47,
  /// kArchTailCallWasm
  ArchTailCallWasm = 48,
  /// kArchTailCallWasmIndirect
  ArchTailCallWasmIndirect = 49,
  /// kArchCallWasmFunction
  ArchCallWasmFunction = 50,
  /// kArchCallWasmFunctionIndirect
  ArchCallWasmFunctionIndirect = 51,
  /// kArchTrap
  ArchTrap = 52,
  /// kArchStackPointer
  ArchStackPointer = 53,
  /// kArchSetStackPointer
  ArchSetStackPointer = 54,
  /// kAtomicExchangeInt8
  AtomicExchangeInt8 = 55,
  /// kAtomicExchangeUint8
  AtomicExchangeUint8 = 56,
  /// kAtomicExchangeInt16
  AtomicExchangeInt16 = 57,
  /// kAtomicExchangeUint16
  AtomicExchangeUint16 = 58,
  /// kAtomicExchangeWord32
  AtomicExchangeWord32 = 59,
  /// kAtomicExchangeWithWriteBarrier
  AtomicExchangeWithWriteBarrier = 60,
  /// kAtomicCompareExchangeInt8
  AtomicCompareExchangeInt8 = 61,
  /// kAtomicCompareExchangeUint8
  AtomicCompareExchangeUint8 = 62,
  /// kAtomicCompareExchangeInt16
  AtomicCompareExchangeInt16 = 63,
  /// kAtomicCompareExchangeUint16
  AtomicCompareExchangeUint16 = 64,
  /// kAtomicCompareExchangeWord32
  AtomicCompareExchangeWord32 = 65,
  /// kAtomicCompareExchangeWithWriteBarrier
  AtomicCompareExchangeWithWriteBarrier = 66,
  /// kAtomicAddInt8
  AtomicAddInt8 = 67,
  /// kAtomicAddUint8
  AtomicAddUint8 = 68,
  /// kAtomicAddInt16
  AtomicAddInt16 = 69,
  /// kAtomicAddUint16
  AtomicAddUint16 = 70,
  /// kAtomicAddWord32
  AtomicAddWord32 = 71,
  /// kAtomicSubInt8
  AtomicSubInt8 = 72,
  /// kAtomicSubUint8
  AtomicSubUint8 = 73,
  /// kAtomicSubInt16
  AtomicSubInt16 = 74,
  /// kAtomicSubUint16
  AtomicSubUint16 = 75,
  /// kAtomicSubWord32
  AtomicSubWord32 = 76,
  /// kAtomicAndInt8
  AtomicAndInt8 = 77,
  /// kAtomicAndUint8
  AtomicAndUint8 = 78,
  /// kAtomicAndInt16
  AtomicAndInt16 = 79,
  /// kAtomicAndUint16
  AtomicAndUint16 = 80,
  /// kAtomicAndWord32
  AtomicAndWord32 = 81,
  /// kAtomicOrInt8
  AtomicOrInt8 = 82,
  /// kAtomicOrUint8
  AtomicOrUint8 = 83,
  /// kAtomicOrInt16
  AtomicOrInt16 = 84,
  /// kAtomicOrUint16
  AtomicOrUint16 = 85,
  /// kAtomicOrWord32
  AtomicOrWord32 = 86,
  /// kAtomicXorInt8
  AtomicXorInt8 = 87,
  /// kAtomicXorUint8
  AtomicXorUint8 = 88,
  /// kAtomicXorInt16
  AtomicXorInt16 = 89,
  /// kAtomicXorUint16
  AtomicXorUint16 = 90,
  /// kAtomicXorWord32
  AtomicXorWord32 = 91,
  /// kArchStoreWithWriteBarrier
  ArchStoreWithWriteBarrier = 92,
  /// kArchAtomicStoreWithWriteBarrier
  ArchAtomicStoreWithWriteBarrier = 93,
  /// kArchStoreIndirectWithWriteBarrier
  ArchStoreIndirectWithWriteBarrier = 94,
  /// kArchStoreSkippedWriteBarrier
  ArchStoreSkippedWriteBarrier = 95,
  /// kArchAtomicStoreSkippedWriteBarrier
  ArchAtomicStoreSkippedWriteBarrier = 96,
  /// kArchStoreIndirectSkippedWriteBarrier
  ArchStoreIndirectSkippedWriteBarrier = 97,
  /// kAtomicLoadInt8
  AtomicLoadInt8 = 98,
  /// kAtomicLoadUint8
  AtomicLoadUint8 = 99,
  /// kAtomicLoadInt16
  AtomicLoadInt16 = 100,
  /// kAtomicLoadUint16
  AtomicLoadUint16 = 101,
  /// kAtomicLoadWord32
  AtomicLoadWord32 = 102,
  /// kAtomicStoreWord8
  AtomicStoreWord8 = 103,
  /// kAtomicStoreWord16
  AtomicStoreWord16 = 104,
  /// kAtomicStoreWord32
  AtomicStoreWord32 = 105,
  /// kArchLoadTrustedPointer
  ArchLoadTrustedPointer = 106,
  /// kX64Add
  X64Add = 107,
  /// kX64Add64_3
  X64Add64_3 = 108,
  /// kX64Add128
  X64Add128 = 109,
  /// kX64Add32
  X64Add32 = 110,
  /// kX64Sub128
  X64Sub128 = 111,
  /// kX64And
  X64And = 112,
  /// kX64And32
  X64And32 = 113,
  /// kX64Cmp
  X64Cmp = 114,
  /// kX64Cmp32
  X64Cmp32 = 115,
  /// kX64Cmp16
  X64Cmp16 = 116,
  /// kX64Cmp8
  X64Cmp8 = 117,
  /// kX64Test
  X64Test = 118,
  /// kX64Test32
  X64Test32 = 119,
  /// kX64Test16
  X64Test16 = 120,
  /// kX64Test8
  X64Test8 = 121,
  /// kX64Or
  X64Or = 122,
  /// kX64Or32
  X64Or32 = 123,
  /// kX64Xor
  X64Xor = 124,
  /// kX64Xor32
  X64Xor32 = 125,
  /// kX64Sub
  X64Sub = 126,
  /// kX64Sub32
  X64Sub32 = 127,
  /// kX64Imul
  X64Imul = 128,
  /// kX64ImulWide
  X64ImulWide = 129,
  /// kX64UmulWide
  X64UmulWide = 130,
  /// kX64Imul32
  X64Imul32 = 131,
  /// kX64ImulHigh32
  X64ImulHigh32 = 132,
  /// kX64UmulHigh32
  X64UmulHigh32 = 133,
  /// kX64Idiv
  X64Idiv = 134,
  /// kX64Idiv32
  X64Idiv32 = 135,
  /// kX64Udiv
  X64Udiv = 136,
  /// kX64Udiv32
  X64Udiv32 = 137,
  /// kX64Not
  X64Not = 138,
  /// kX64Not32
  X64Not32 = 139,
  /// kX64Neg
  X64Neg = 140,
  /// kX64Neg32
  X64Neg32 = 141,
  /// kX64Shl
  X64Shl = 142,
  /// kX64Shl32
  X64Shl32 = 143,
  /// kX64Shr
  X64Shr = 144,
  /// kX64Shr32
  X64Shr32 = 145,
  /// kX64Sar
  X64Sar = 146,
  /// kX64Sar32
  X64Sar32 = 147,
  /// kX64Rol
  X64Rol = 148,
  /// kX64Rol32
  X64Rol32 = 149,
  /// kX64Ror
  X64Ror = 150,
  /// kX64Ror32
  X64Ror32 = 151,
  /// kX64Lzcnt
  X64Lzcnt = 152,
  /// kX64Lzcnt32
  X64Lzcnt32 = 153,
  /// kX64Tzcnt
  X64Tzcnt = 154,
  /// kX64Tzcnt32
  X64Tzcnt32 = 155,
  /// kX64Popcnt
  X64Popcnt = 156,
  /// kX64Popcnt32
  X64Popcnt32 = 157,
  /// kX64Bswap
  X64Bswap = 158,
  /// kX64Bswap32
  X64Bswap32 = 159,
  /// kX64MFence
  X64MFence = 160,
  /// kX64LFence
  X64LFence = 161,
  /// kSSEFloat32Cmp
  SSEFloat32Cmp = 162,
  /// kSSEFloat32Sqrt
  SSEFloat32Sqrt = 163,
  /// kSSEFloat32ToFloat64
  SSEFloat32ToFloat64 = 164,
  /// kSSEFloat32ToInt32
  SSEFloat32ToInt32 = 165,
  /// kSSEFloat32ToUint32
  SSEFloat32ToUint32 = 166,
  /// kSSEFloat32Round
  SSEFloat32Round = 167,
  /// kSSEFloat64Cmp
  SSEFloat64Cmp = 168,
  /// kSSEFloat64Mod
  SSEFloat64Mod = 169,
  /// kSSEFloat64Sqrt
  SSEFloat64Sqrt = 170,
  /// kSSEFloat64Round
  SSEFloat64Round = 171,
  /// kSSEFloat32Max
  SSEFloat32Max = 172,
  /// kSSEFloat64Max
  SSEFloat64Max = 173,
  /// kSSEFloat32Min
  SSEFloat32Min = 174,
  /// kSSEFloat64Min
  SSEFloat64Min = 175,
  /// kSSEFloat64ToFloat32
  SSEFloat64ToFloat32 = 176,
  /// kSSEFloat64ToFloat16RawBits
  SSEFloat64ToFloat16RawBits = 177,
  /// kSSEFloat16RawBitsToFloat64
  SSEFloat16RawBitsToFloat64 = 178,
  /// kSSEFloat64ToInt32
  SSEFloat64ToInt32 = 179,
  /// kSSEFloat64ToUint32
  SSEFloat64ToUint32 = 180,
  /// kSSEFloat32ToInt64
  SSEFloat32ToInt64 = 181,
  /// kSSEFloat64ToInt64
  SSEFloat64ToInt64 = 182,
  /// kSSEFloat32ToUint64
  SSEFloat32ToUint64 = 183,
  /// kSSEFloat64ToUint64
  SSEFloat64ToUint64 = 184,
  /// kSSEInt32ToFloat64
  SSEInt32ToFloat64 = 185,
  /// kSSEInt32ToFloat32
  SSEInt32ToFloat32 = 186,
  /// kSSEInt64ToFloat32
  SSEInt64ToFloat32 = 187,
  /// kSSEInt64ToFloat64
  SSEInt64ToFloat64 = 188,
  /// kSSEUint64ToFloat32
  SSEUint64ToFloat32 = 189,
  /// kSSEUint64ToFloat64
  SSEUint64ToFloat64 = 190,
  /// kSSEUint32ToFloat64
  SSEUint32ToFloat64 = 191,
  /// kSSEUint32ToFloat32
  SSEUint32ToFloat32 = 192,
  /// kSSEFloat64ExtractLowWord32
  SSEFloat64ExtractLowWord32 = 193,
  /// kSSEFloat64ExtractHighWord32
  SSEFloat64ExtractHighWord32 = 194,
  /// kSSEFloat64InsertLowWord32
  SSEFloat64InsertLowWord32 = 195,
  /// kSSEFloat64InsertHighWord32
  SSEFloat64InsertHighWord32 = 196,
  /// kSSEFloat64LoadLowWord32
  SSEFloat64LoadLowWord32 = 197,
  /// kSSEFloat64SilenceNaN
  SSEFloat64SilenceNaN = 198,
  /// kAVXFloat32Cmp
  AVXFloat32Cmp = 199,
  /// kAVXFloat64Cmp
  AVXFloat64Cmp = 200,
  /// kX64Float64Abs
  X64Float64Abs = 201,
  /// kX64Float64Neg
  X64Float64Neg = 202,
  /// kX64Float32Abs
  X64Float32Abs = 203,
  /// kX64Float32Neg
  X64Float32Neg = 204,
  /// kX64MovqStoreIndirectPointer
  X64MovqStoreIndirectPointer = 205,
  /// kX64MovqEncodeSandboxedPointer
  X64MovqEncodeSandboxedPointer = 206,
  /// kX64MovqDecodeSandboxedPointer
  X64MovqDecodeSandboxedPointer = 207,
  /// kX64BitcastFI
  X64BitcastFI = 208,
  /// kX64BitcastDL
  X64BitcastDL = 209,
  /// kX64BitcastIF
  X64BitcastIF = 210,
  /// kX64BitcastLD
  X64BitcastLD = 211,
  /// kX64Lea32
  X64Lea32 = 212,
  /// kX64Lea
  X64Lea = 213,
  /// kX64Dec32
  X64Dec32 = 214,
  /// kX64Inc32
  X64Inc32 = 215,
  /// kX64Push
  X64Push = 216,
  /// kX64Poke
  X64Poke = 217,
  /// kX64Peek
  X64Peek = 218,
  /// kX64Cvttps2dq
  X64Cvttps2dq = 219,
  /// kX64Cvttpd2dq
  X64Cvttpd2dq = 220,
  /// kX64I32x4TruncF64x2UZero
  X64I32x4TruncF64x2UZero = 221,
  /// kX64I32x4TruncF32x4U
  X64I32x4TruncF32x4U = 222,
  /// kX64I32x8TruncF32x8U
  X64I32x8TruncF32x8U = 223,
  /// kX64FSplat
  X64FSplat = 224,
  /// kX64FExtractLane
  X64FExtractLane = 225,
  /// kX64FReplaceLane
  X64FReplaceLane = 226,
  /// kX64FAbs
  X64FAbs = 227,
  /// kX64FNeg
  X64FNeg = 228,
  /// kX64FSqrt
  X64FSqrt = 229,
  /// kX64FAdd
  X64FAdd = 230,
  /// kX64FSub
  X64FSub = 231,
  /// kX64FMul
  X64FMul = 232,
  /// kX64FDiv
  X64FDiv = 233,
  /// kX64FMin
  X64FMin = 234,
  /// kX64FMax
  X64FMax = 235,
  /// kX64FEq
  X64FEq = 236,
  /// kX64FNe
  X64FNe = 237,
  /// kX64FLt
  X64FLt = 238,
  /// kX64FLe
  X64FLe = 239,
  /// kX64F64x2Qfma
  X64F64x2Qfma = 240,
  /// kX64F64x2Qfms
  X64F64x2Qfms = 241,
  /// kX64Minpd
  X64Minpd = 242,
  /// kX64Maxpd
  X64Maxpd = 243,
  /// kX64F64x2Round
  X64F64x2Round = 244,
  /// kX64F64x2ConvertLowI32x4S
  X64F64x2ConvertLowI32x4S = 245,
  /// kX64F64x4ConvertI32x4S
  X64F64x4ConvertI32x4S = 246,
  /// kX64F64x2ConvertLowI32x4U
  X64F64x2ConvertLowI32x4U = 247,
  /// kX64F32x4SConvertI32x4
  X64F32x4SConvertI32x4 = 248,
  /// kX64F32x8SConvertI32x8
  X64F32x8SConvertI32x8 = 249,
  /// kX64F32x4UConvertI32x4
  X64F32x4UConvertI32x4 = 250,
  /// kX64F32x8UConvertI32x8
  X64F32x8UConvertI32x8 = 251,
  /// kX64F32x4Qfma
  X64F32x4Qfma = 252,
  /// kX64F32x4Qfms
  X64F32x4Qfms = 253,
  /// kX64Minps
  X64Minps = 254,
  /// kX64Maxps
  X64Maxps = 255,
  /// kX64F32x4Round
  X64F32x4Round = 256,
  /// kX64F32x4DemoteF64x2Zero
  X64F32x4DemoteF64x2Zero = 257,
  /// kX64F32x4DemoteF64x4
  X64F32x4DemoteF64x4 = 258,
  /// kX64F16x8Round
  X64F16x8Round = 259,
  /// kX64I16x8SConvertF16x8
  X64I16x8SConvertF16x8 = 260,
  /// kX64I16x8UConvertF16x8
  X64I16x8UConvertF16x8 = 261,
  /// kX64F16x8SConvertI16x8
  X64F16x8SConvertI16x8 = 262,
  /// kX64F16x8UConvertI16x8
  X64F16x8UConvertI16x8 = 263,
  /// kX64F16x8DemoteF32x4Zero
  X64F16x8DemoteF32x4Zero = 264,
  /// kX64F16x8DemoteF64x2Zero
  X64F16x8DemoteF64x2Zero = 265,
  /// kX64F32x4PromoteLowF16x8
  X64F32x4PromoteLowF16x8 = 266,
  /// kX64F16x8Qfma
  X64F16x8Qfma = 267,
  /// kX64F16x8Qfms
  X64F16x8Qfms = 268,
  /// kX64Minph
  X64Minph = 269,
  /// kX64Maxph
  X64Maxph = 270,
  /// kX64ISplat
  X64ISplat = 271,
  /// kX64IExtractLane
  X64IExtractLane = 272,
  /// kX64IAbs
  X64IAbs = 273,
  /// kX64INeg
  X64INeg = 274,
  /// kX64IBitMask
  X64IBitMask = 275,
  /// kX64IShl
  X64IShl = 276,
  /// kX64IShrS
  X64IShrS = 277,
  /// kX64IAdd
  X64IAdd = 278,
  /// kX64ISub
  X64ISub = 279,
  /// kX64IMul
  X64IMul = 280,
  /// kX64IEq
  X64IEq = 281,
  /// kX64IGtS
  X64IGtS = 282,
  /// kX64IGeS
  X64IGeS = 283,
  /// kX64INe
  X64INe = 284,
  /// kX64IShrU
  X64IShrU = 285,
  /// kX64I64x2ExtMulLowI32x4S
  X64I64x2ExtMulLowI32x4S = 286,
  /// kX64I64x2ExtMulHighI32x4S
  X64I64x2ExtMulHighI32x4S = 287,
  /// kX64I64x2ExtMulLowI32x4U
  X64I64x2ExtMulLowI32x4U = 288,
  /// kX64I64x2ExtMulHighI32x4U
  X64I64x2ExtMulHighI32x4U = 289,
  /// kX64I64x2SConvertI32x4Low
  X64I64x2SConvertI32x4Low = 290,
  /// kX64I64x2SConvertI32x4High
  X64I64x2SConvertI32x4High = 291,
  /// kX64I64x4SConvertI32x4
  X64I64x4SConvertI32x4 = 292,
  /// kX64I64x2UConvertI32x4Low
  X64I64x2UConvertI32x4Low = 293,
  /// kX64I64x2UConvertI32x4High
  X64I64x2UConvertI32x4High = 294,
  /// kX64I64x4UConvertI32x4
  X64I64x4UConvertI32x4 = 295,
  /// kX64I32x4SConvertF32x4
  X64I32x4SConvertF32x4 = 296,
  /// kX64I32x8SConvertF32x8
  X64I32x8SConvertF32x8 = 297,
  /// kX64I32x4SConvertI16x8Low
  X64I32x4SConvertI16x8Low = 298,
  /// kX64I32x4SConvertI16x8High
  X64I32x4SConvertI16x8High = 299,
  /// kX64I32x8SConvertI16x8
  X64I32x8SConvertI16x8 = 300,
  /// kX64IMinS
  X64IMinS = 301,
  /// kX64IMaxS
  X64IMaxS = 302,
  /// kX64I32x4UConvertF32x4
  X64I32x4UConvertF32x4 = 303,
  /// kX64I32x8UConvertF32x8
  X64I32x8UConvertF32x8 = 304,
  /// kX64I32x4UConvertI16x8Low
  X64I32x4UConvertI16x8Low = 305,
  /// kX64I32x4UConvertI16x8High
  X64I32x4UConvertI16x8High = 306,
  /// kX64I32x8UConvertI16x8
  X64I32x8UConvertI16x8 = 307,
  /// kX64IMinU
  X64IMinU = 308,
  /// kX64IMaxU
  X64IMaxU = 309,
  /// kX64IGtU
  X64IGtU = 310,
  /// kX64IGeU
  X64IGeU = 311,
  /// kX64I32x4DotI16x8S
  X64I32x4DotI16x8S = 312,
  /// kX64I32x8DotI16x16S
  X64I32x8DotI16x16S = 313,
  /// kX64I32x4DotI8x16I7x16AddS
  X64I32x4DotI8x16I7x16AddS = 314,
  /// kX64I32x4ExtMulLowI16x8S
  X64I32x4ExtMulLowI16x8S = 315,
  /// kX64I32x4ExtMulHighI16x8S
  X64I32x4ExtMulHighI16x8S = 316,
  /// kX64I32x4ExtMulLowI16x8U
  X64I32x4ExtMulLowI16x8U = 317,
  /// kX64I32x4ExtMulHighI16x8U
  X64I32x4ExtMulHighI16x8U = 318,
  /// kX64I32x4ExtAddPairwiseI16x8S
  X64I32x4ExtAddPairwiseI16x8S = 319,
  /// kX64I32x8ExtAddPairwiseI16x16S
  X64I32x8ExtAddPairwiseI16x16S = 320,
  /// kX64I32x4ExtAddPairwiseI16x8U
  X64I32x4ExtAddPairwiseI16x8U = 321,
  /// kX64I32x8ExtAddPairwiseI16x16U
  X64I32x8ExtAddPairwiseI16x16U = 322,
  /// kX64I32x4TruncSatF64x2SZero
  X64I32x4TruncSatF64x2SZero = 323,
  /// kX64I32x4TruncSatF64x2UZero
  X64I32x4TruncSatF64x2UZero = 324,
  /// kX64I32X4ShiftZeroExtendI8x16
  X64I32X4ShiftZeroExtendI8x16 = 325,
  /// kX64IExtractLaneS
  X64IExtractLaneS = 326,
  /// kX64I16x8SConvertI8x16Low
  X64I16x8SConvertI8x16Low = 327,
  /// kX64I16x8SConvertI8x16High
  X64I16x8SConvertI8x16High = 328,
  /// kX64I16x16SConvertI8x16
  X64I16x16SConvertI8x16 = 329,
  /// kX64I16x8SConvertI32x4
  X64I16x8SConvertI32x4 = 330,
  /// kX64I16x16SConvertI32x8
  X64I16x16SConvertI32x8 = 331,
  /// kX64IAddSatS
  X64IAddSatS = 332,
  /// kX64ISubSatS
  X64ISubSatS = 333,
  /// kX64I16x8UConvertI8x16Low
  X64I16x8UConvertI8x16Low = 334,
  /// kX64I16x8UConvertI8x16High
  X64I16x8UConvertI8x16High = 335,
  /// kX64I16x16UConvertI8x16
  X64I16x16UConvertI8x16 = 336,
  /// kX64I16x8UConvertI32x4
  X64I16x8UConvertI32x4 = 337,
  /// kX64I16x16UConvertI32x8
  X64I16x16UConvertI32x8 = 338,
  /// kX64IAddSatU
  X64IAddSatU = 339,
  /// kX64ISubSatU
  X64ISubSatU = 340,
  /// kX64IRoundingAverageU
  X64IRoundingAverageU = 341,
  /// kX64I16x8ExtMulLowI8x16S
  X64I16x8ExtMulLowI8x16S = 342,
  /// kX64I16x8ExtMulHighI8x16S
  X64I16x8ExtMulHighI8x16S = 343,
  /// kX64I16x8ExtMulLowI8x16U
  X64I16x8ExtMulLowI8x16U = 344,
  /// kX64I16x8ExtMulHighI8x16U
  X64I16x8ExtMulHighI8x16U = 345,
  /// kX64I16x8ExtAddPairwiseI8x16S
  X64I16x8ExtAddPairwiseI8x16S = 346,
  /// kX64I16x16ExtAddPairwiseI8x32S
  X64I16x16ExtAddPairwiseI8x32S = 347,
  /// kX64I16x8ExtAddPairwiseI8x16U
  X64I16x8ExtAddPairwiseI8x16U = 348,
  /// kX64I16x16ExtAddPairwiseI8x32U
  X64I16x16ExtAddPairwiseI8x32U = 349,
  /// kX64I16x8Q15MulRSatS
  X64I16x8Q15MulRSatS = 350,
  /// kX64I16x8RelaxedQ15MulRS
  X64I16x8RelaxedQ15MulRS = 351,
  /// kX64I16x8DotI8x16I7x16S
  X64I16x8DotI8x16I7x16S = 352,
  /// kX64I8x16SConvertI16x8
  X64I8x16SConvertI16x8 = 353,
  /// kX64I8x32SConvertI16x16
  X64I8x32SConvertI16x16 = 354,
  /// kX64I8x16UConvertI16x8
  X64I8x16UConvertI16x8 = 355,
  /// kX64I8x32UConvertI16x16
  X64I8x32UConvertI16x16 = 356,
  /// kX64S128Const
  X64S128Const = 357,
  /// kX64S256Const
  X64S256Const = 358,
  /// kX64SZero
  X64SZero = 359,
  /// kX64SAllOnes
  X64SAllOnes = 360,
  /// kX64SNot
  X64SNot = 361,
  /// kX64SAnd
  X64SAnd = 362,
  /// kX64SOr
  X64SOr = 363,
  /// kX64SXor
  X64SXor = 364,
  /// kX64SSelect
  X64SSelect = 365,
  /// kX64SAndNot
  X64SAndNot = 366,
  /// kX64I8x16Swizzle
  X64I8x16Swizzle = 367,
  /// kX64I8x16Shuffle
  X64I8x16Shuffle = 368,
  /// kX64Vpshufd
  X64Vpshufd = 369,
  /// kX64I8x16Popcnt
  X64I8x16Popcnt = 370,
  /// kX64Shufps
  X64Shufps = 371,
  /// kX64S32x4Rotate
  X64S32x4Rotate = 372,
  /// kX64S32x4Swizzle
  X64S32x4Swizzle = 373,
  /// kX64S32x4Shuffle
  X64S32x4Shuffle = 374,
  /// kX64S16x8Blend
  X64S16x8Blend = 375,
  /// kX64S16x8HalfShuffle1
  X64S16x8HalfShuffle1 = 376,
  /// kX64S16x8HalfShuffle2
  X64S16x8HalfShuffle2 = 377,
  /// kX64S8x16Alignr
  X64S8x16Alignr = 378,
  /// kX64S16x8Dup
  X64S16x8Dup = 379,
  /// kX64S8x16Dup
  X64S8x16Dup = 380,
  /// kX64S16x8UnzipHigh
  X64S16x8UnzipHigh = 381,
  /// kX64S16x8UnzipLow
  X64S16x8UnzipLow = 382,
  /// kX64S8x16UnzipHigh
  X64S8x16UnzipHigh = 383,
  /// kX64S8x16UnzipLow
  X64S8x16UnzipLow = 384,
  /// kX64S64x2UnpackHigh
  X64S64x2UnpackHigh = 385,
  /// kX64S32x4UnpackHigh
  X64S32x4UnpackHigh = 386,
  /// kX64S16x8UnpackHigh
  X64S16x8UnpackHigh = 387,
  /// kX64S8x16UnpackHigh
  X64S8x16UnpackHigh = 388,
  /// kX64S32x8UnpackHigh
  X64S32x8UnpackHigh = 389,
  /// kX64S64x2UnpackLow
  X64S64x2UnpackLow = 390,
  /// kX64S32x4UnpackLow
  X64S32x4UnpackLow = 391,
  /// kX64S16x8UnpackLow
  X64S16x8UnpackLow = 392,
  /// kX64S8x16UnpackLow
  X64S8x16UnpackLow = 393,
  /// kX64S32x8UnpackLow
  X64S32x8UnpackLow = 394,
  /// kX64S8x16TransposeLow
  X64S8x16TransposeLow = 395,
  /// kX64S8x16TransposeHigh
  X64S8x16TransposeHigh = 396,
  /// kX64S8x8Reverse
  X64S8x8Reverse = 397,
  /// kX64S8x4Reverse
  X64S8x4Reverse = 398,
  /// kX64S8x2Reverse
  X64S8x2Reverse = 399,
  /// kX64V128AnyTrue
  X64V128AnyTrue = 400,
  /// kX64IAllTrue
  X64IAllTrue = 401,
  /// kX64Blendvpd
  X64Blendvpd = 402,
  /// kX64Blendvps
  X64Blendvps = 403,
  /// kX64Pblendvb
  X64Pblendvb = 404,
  /// kX64I64x4ExtMulI32x4S
  X64I64x4ExtMulI32x4S = 405,
  /// kX64I64x4ExtMulI32x4U
  X64I64x4ExtMulI32x4U = 406,
  /// kX64I32x8ExtMulI16x8S
  X64I32x8ExtMulI16x8S = 407,
  /// kX64I32x8ExtMulI16x8U
  X64I32x8ExtMulI16x8U = 408,
  /// kX64I16x16ExtMulI8x16S
  X64I16x16ExtMulI8x16S = 409,
  /// kX64I16x16ExtMulI8x16U
  X64I16x16ExtMulI8x16U = 410,
  /// kX64TraceInstruction
  X64TraceInstruction = 411,
  /// kX64F32x8Pmin
  X64F32x8Pmin = 412,
  /// kX64F32x8Pmax
  X64F32x8Pmax = 413,
  /// kX64F64x4Pmin
  X64F64x4Pmin = 414,
  /// kX64F64x4Pmax
  X64F64x4Pmax = 415,
  /// kX64ExtractF128
  X64ExtractF128 = 416,
  /// kX64F32x8Qfma
  X64F32x8Qfma = 417,
  /// kX64F32x8Qfms
  X64F32x8Qfms = 418,
  /// kX64F64x4Qfma
  X64F64x4Qfma = 419,
  /// kX64F64x4Qfms
  X64F64x4Qfms = 420,
  /// kX64InsertI128
  X64InsertI128 = 421,
  /// kX64I32x8DotI8x32I7x32AddS
  X64I32x8DotI8x32I7x32AddS = 422,
  /// kX64I16x16DotI8x32I7x32S
  X64I16x16DotI8x32I7x32S = 423,
  /// kX64F32x8Round
  X64F32x8Round = 424,
  /// kX64F64x2PromoteLowF32x4
  X64F64x2PromoteLowF32x4 = 425,
  /// kX64Movb
  X64Movb = 426,
  /// kX64Movdqu
  X64Movdqu = 427,
  /// kX64Movl
  X64Movl = 428,
  /// kX64Movq
  X64Movq = 429,
  /// kX64Movsd
  X64Movsd = 430,
  /// kX64Movss
  X64Movss = 431,
  /// kX64Movsh
  X64Movsh = 432,
  /// kX64Movsxbl
  X64Movsxbl = 433,
  /// kX64Movsxbq
  X64Movsxbq = 434,
  /// kX64Movsxlq
  X64Movsxlq = 435,
  /// kX64Movsxwl
  X64Movsxwl = 436,
  /// kX64Movsxwq
  X64Movsxwq = 437,
  /// kX64Movw
  X64Movw = 438,
  /// kX64Movzxbl
  X64Movzxbl = 439,
  /// kX64Movzxbq
  X64Movzxbq = 440,
  /// kX64Movzxwl
  X64Movzxwl = 441,
  /// kX64Movzxwq
  X64Movzxwq = 442,
  /// kX64Pextrb
  X64Pextrb = 443,
  /// kX64Pextrw
  X64Pextrw = 444,
  /// kX64Pinsrb
  X64Pinsrb = 445,
  /// kX64Pinsrd
  X64Pinsrd = 446,
  /// kX64Pinsrq
  X64Pinsrq = 447,
  /// kX64Pinsrw
  X64Pinsrw = 448,
  /// kX64S128Load16Splat
  X64S128Load16Splat = 449,
  /// kX64S128Load16x4S
  X64S128Load16x4S = 450,
  /// kX64S128Load16x4U
  X64S128Load16x4U = 451,
  /// kX64S128Load32Splat
  X64S128Load32Splat = 452,
  /// kX64S128Load32x2S
  X64S128Load32x2S = 453,
  /// kX64S128Load32x2U
  X64S128Load32x2U = 454,
  /// kX64S128Load64Splat
  X64S128Load64Splat = 455,
  /// kX64S128Load8Splat
  X64S128Load8Splat = 456,
  /// kX64S128Load8x8S
  X64S128Load8x8S = 457,
  /// kX64S128Load8x8U
  X64S128Load8x8U = 458,
  /// kX64S128Store32Lane
  X64S128Store32Lane = 459,
  /// kX64S128Store64Lane
  X64S128Store64Lane = 460,
  /// kX64Word64AtomicStoreWord64
  X64Word64AtomicStoreWord64 = 461,
  /// kX64Word64AtomicAddUint64
  X64Word64AtomicAddUint64 = 462,
  /// kX64Word64AtomicSubUint64
  X64Word64AtomicSubUint64 = 463,
  /// kX64Word64AtomicAndUint64
  X64Word64AtomicAndUint64 = 464,
  /// kX64Word64AtomicOrUint64
  X64Word64AtomicOrUint64 = 465,
  /// kX64Word64AtomicXorUint64
  X64Word64AtomicXorUint64 = 466,
  /// kX64Word64AtomicExchangeUint64
  X64Word64AtomicExchangeUint64 = 467,
  /// kX64Word64AtomicCompareExchangeUint64
  X64Word64AtomicCompareExchangeUint64 = 468,
  /// kX64Movdqu256
  X64Movdqu256 = 469,
  /// kX64MovqDecompressTaggedSigned
  X64MovqDecompressTaggedSigned = 470,
  /// kX64MovqDecompressTagged
  X64MovqDecompressTagged = 471,
  /// kX64MovqCompressTagged
  X64MovqCompressTagged = 472,
  /// kX64MovqDecompressProtected
  X64MovqDecompressProtected = 473,
  /// kX64S256Load8Splat
  X64S256Load8Splat = 474,
  /// kX64S256Load16Splat
  X64S256Load16Splat = 475,
  /// kX64S256Load32Splat
  X64S256Load32Splat = 476,
  /// kX64S256Load64Splat
  X64S256Load64Splat = 477,
  /// kX64S256Load8x16S
  X64S256Load8x16S = 478,
  /// kX64S256Load8x16U
  X64S256Load8x16U = 479,
  /// kX64S256Load8x8U
  X64S256Load8x8U = 480,
  /// kX64S256Load16x8S
  X64S256Load16x8S = 481,
  /// kX64S256Load16x8U
  X64S256Load16x8U = 482,
  /// kX64S256Load32x4S
  X64S256Load32x4S = 483,
  /// kX64S256Load32x4U
  X64S256Load32x4U = 484,
  /// kSSEFloat32Add
  SSEFloat32Add = 485,
  /// kSSEFloat32Sub
  SSEFloat32Sub = 486,
  /// kSSEFloat32Mul
  SSEFloat32Mul = 487,
  /// kSSEFloat32Div
  SSEFloat32Div = 488,
  /// kSSEFloat64Add
  SSEFloat64Add = 489,
  /// kSSEFloat64Sub
  SSEFloat64Sub = 490,
  /// kSSEFloat64Mul
  SSEFloat64Mul = 491,
  /// kSSEFloat64Div
  SSEFloat64Div = 492,
  /// kAVXFloat32Add
  AVXFloat32Add = 493,
  /// kAVXFloat32Sub
  AVXFloat32Sub = 494,
  /// kAVXFloat32Mul
  AVXFloat32Mul = 495,
  /// kAVXFloat32Div
  AVXFloat32Div = 496,
  /// kAVXFloat64Add
  AVXFloat64Add = 497,
  /// kAVXFloat64Sub
  AVXFloat64Sub = 498,
  /// kAVXFloat64Mul
  AVXFloat64Mul = 499,
  /// kAVXFloat64Div
  AVXFloat64Div = 500,
}

/// 原版 kLastArchOpcode (条数 - 1)
pub const LAST_ARCH_OPCODE: u16 = 500;

/// 原版 HasMemoryAccessMode 集合 (COMMON+TARGET memory-access 列表)
pub const HAS_MEMORY_ACCESS_MODE: &[ArchOpcode] = &[
  ArchOpcode::AtomicExchangeInt8,
  ArchOpcode::AtomicExchangeUint8,
  ArchOpcode::AtomicExchangeInt16,
  ArchOpcode::AtomicExchangeUint16,
  ArchOpcode::AtomicExchangeWord32,
  ArchOpcode::AtomicExchangeWithWriteBarrier,
  ArchOpcode::AtomicCompareExchangeInt8,
  ArchOpcode::AtomicCompareExchangeUint8,
  ArchOpcode::AtomicCompareExchangeInt16,
  ArchOpcode::AtomicCompareExchangeUint16,
  ArchOpcode::AtomicCompareExchangeWord32,
  ArchOpcode::AtomicCompareExchangeWithWriteBarrier,
  ArchOpcode::AtomicAddInt8,
  ArchOpcode::AtomicAddUint8,
  ArchOpcode::AtomicAddInt16,
  ArchOpcode::AtomicAddUint16,
  ArchOpcode::AtomicAddWord32,
  ArchOpcode::AtomicSubInt8,
  ArchOpcode::AtomicSubUint8,
  ArchOpcode::AtomicSubInt16,
  ArchOpcode::AtomicSubUint16,
  ArchOpcode::AtomicSubWord32,
  ArchOpcode::AtomicAndInt8,
  ArchOpcode::AtomicAndUint8,
  ArchOpcode::AtomicAndInt16,
  ArchOpcode::AtomicAndUint16,
  ArchOpcode::AtomicAndWord32,
  ArchOpcode::AtomicOrInt8,
  ArchOpcode::AtomicOrUint8,
  ArchOpcode::AtomicOrInt16,
  ArchOpcode::AtomicOrUint16,
  ArchOpcode::AtomicOrWord32,
  ArchOpcode::AtomicXorInt8,
  ArchOpcode::AtomicXorUint8,
  ArchOpcode::AtomicXorInt16,
  ArchOpcode::AtomicXorUint16,
  ArchOpcode::AtomicXorWord32,
  ArchOpcode::ArchStoreWithWriteBarrier,
  ArchOpcode::ArchAtomicStoreWithWriteBarrier,
  ArchOpcode::ArchStoreIndirectWithWriteBarrier,
  ArchOpcode::ArchStoreSkippedWriteBarrier,
  ArchOpcode::ArchAtomicStoreSkippedWriteBarrier,
  ArchOpcode::ArchStoreIndirectSkippedWriteBarrier,
  ArchOpcode::AtomicLoadInt8,
  ArchOpcode::AtomicLoadUint8,
  ArchOpcode::AtomicLoadInt16,
  ArchOpcode::AtomicLoadUint16,
  ArchOpcode::AtomicLoadWord32,
  ArchOpcode::AtomicStoreWord8,
  ArchOpcode::AtomicStoreWord16,
  ArchOpcode::AtomicStoreWord32,
  ArchOpcode::ArchLoadTrustedPointer,
  ArchOpcode::X64F64x2PromoteLowF32x4,
  ArchOpcode::X64Movb,
  ArchOpcode::X64Movdqu,
  ArchOpcode::X64Movl,
  ArchOpcode::X64Movq,
  ArchOpcode::X64Movsd,
  ArchOpcode::X64Movss,
  ArchOpcode::X64Movsh,
  ArchOpcode::X64Movsxbl,
  ArchOpcode::X64Movsxbq,
  ArchOpcode::X64Movsxlq,
  ArchOpcode::X64Movsxwl,
  ArchOpcode::X64Movsxwq,
  ArchOpcode::X64Movw,
  ArchOpcode::X64Movzxbl,
  ArchOpcode::X64Movzxbq,
  ArchOpcode::X64Movzxwl,
  ArchOpcode::X64Movzxwq,
  ArchOpcode::X64Pextrb,
  ArchOpcode::X64Pextrw,
  ArchOpcode::X64Pinsrb,
  ArchOpcode::X64Pinsrd,
  ArchOpcode::X64Pinsrq,
  ArchOpcode::X64Pinsrw,
  ArchOpcode::X64S128Load16Splat,
  ArchOpcode::X64S128Load16x4S,
  ArchOpcode::X64S128Load16x4U,
  ArchOpcode::X64S128Load32Splat,
  ArchOpcode::X64S128Load32x2S,
  ArchOpcode::X64S128Load32x2U,
  ArchOpcode::X64S128Load64Splat,
  ArchOpcode::X64S128Load8Splat,
  ArchOpcode::X64S128Load8x8S,
  ArchOpcode::X64S128Load8x8U,
  ArchOpcode::X64S128Store32Lane,
  ArchOpcode::X64S128Store64Lane,
  ArchOpcode::X64Word64AtomicStoreWord64,
  ArchOpcode::X64Word64AtomicAddUint64,
  ArchOpcode::X64Word64AtomicSubUint64,
  ArchOpcode::X64Word64AtomicAndUint64,
  ArchOpcode::X64Word64AtomicOrUint64,
  ArchOpcode::X64Word64AtomicXorUint64,
  ArchOpcode::X64Word64AtomicExchangeUint64,
  ArchOpcode::X64Word64AtomicCompareExchangeUint64,
  ArchOpcode::X64Movdqu256,
  ArchOpcode::X64MovqDecompressTaggedSigned,
  ArchOpcode::X64MovqDecompressTagged,
  ArchOpcode::X64MovqCompressTagged,
  ArchOpcode::X64MovqDecompressProtected,
  ArchOpcode::X64S256Load8Splat,
  ArchOpcode::X64S256Load16Splat,
  ArchOpcode::X64S256Load32Splat,
  ArchOpcode::X64S256Load64Splat,
  ArchOpcode::X64S256Load8x16S,
  ArchOpcode::X64S256Load8x16U,
  ArchOpcode::X64S256Load8x8U,
  ArchOpcode::X64S256Load16x8S,
  ArchOpcode::X64S256Load16x8U,
  ArchOpcode::X64S256Load32x4S,
  ArchOpcode::X64S256Load32x4U,
  ArchOpcode::SSEFloat32Add,
  ArchOpcode::SSEFloat32Sub,
  ArchOpcode::SSEFloat32Mul,
  ArchOpcode::SSEFloat32Div,
  ArchOpcode::SSEFloat64Add,
  ArchOpcode::SSEFloat64Sub,
  ArchOpcode::SSEFloat64Mul,
  ArchOpcode::SSEFloat64Div,
  ArchOpcode::AVXFloat32Add,
  ArchOpcode::AVXFloat32Sub,
  ArchOpcode::AVXFloat32Mul,
  ArchOpcode::AVXFloat32Div,
  ArchOpcode::AVXFloat64Add,
  ArchOpcode::AVXFloat64Sub,
  ArchOpcode::AVXFloat64Mul,
  ArchOpcode::AVXFloat64Div,
];

/// 原版 AddressingMode (None + TARGET_ADDRESSING_MODE_LIST)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[rustfmt::skip]
pub enum AddressingMode {
  /// kMode_None
  ModeNone = 0,
  /// kMode_MR
  ModeMR = 1,
  /// kMode_MRI
  ModeMRI = 2,
  /// kMode_MR1
  ModeMR1 = 3,
  /// kMode_MR2
  ModeMR2 = 4,
  /// kMode_MR4
  ModeMR4 = 5,
  /// kMode_MR8
  ModeMR8 = 6,
  /// kMode_MR1I
  ModeMR1I = 7,
  /// kMode_MR2I
  ModeMR2I = 8,
  /// kMode_MR4I
  ModeMR4I = 9,
  /// kMode_MR8I
  ModeMR8I = 10,
  /// kMode_M1
  ModeM1 = 11,
  /// kMode_M2
  ModeM2 = 12,
  /// kMode_M4
  ModeM4 = 13,
  /// kMode_M8
  ModeM8 = 14,
  /// kMode_M1I
  ModeM1I = 15,
  /// kMode_M2I
  ModeM2I = 16,
  /// kMode_M4I
  ModeM4I = 17,
  /// kMode_M8I
  ModeM8I = 18,
  /// kMode_Root
  ModeRoot = 19,
  /// kMode_MCR
  ModeMCR = 20,
  /// kMode_MCRI
  ModeMCRI = 21,
}

/// 原版 kLastAddressingMode (条数 - 1)
pub const LAST_ADDRESSING_MODE: u16 = 21;

// —— 提取不变式断言 ——
const _: () = {
  assert!(LAST_ARCH_OPCODE as u32 >= 100);
  assert!(ArchOpcode::ArchJmp as u16 >= 8);
  assert!(AddressingMode::ModeNone as u16 == 0);
  assert!(!HAS_MEMORY_ACCESS_MODE.is_empty());
};
