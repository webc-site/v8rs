// 由 tool/src/compiler/backend/instruction-codes.h.js 从 v8/src/compiler/backend/arm64/instruction-codes-arm64.h + instruction-codes.h 生成; 修订就地改, 复跑覆盖
// arm64 指令码表: ArchOpcode (公共+架构) 与 AddressingMode

// 公共面见 instruction_codes.rs; kLastArchOpcode/kLastAddressingMode = 条数-1

/// 原版 ArchOpcode (arm64, 公共 216 + 架构项)
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
  /// kArm64Add
  Arm64Add = 107,
  /// kArm64Add32
  Arm64Add32 = 108,
  /// kArm64Add64_3
  Arm64Add64_3 = 109,
  /// kArm64Add128
  Arm64Add128 = 110,
  /// kArm64Sub128
  Arm64Sub128 = 111,
  /// kArm64And
  Arm64And = 112,
  /// kArm64And32
  Arm64And32 = 113,
  /// kArm64Bic
  Arm64Bic = 114,
  /// kArm64Bic32
  Arm64Bic32 = 115,
  /// kArm64Clz
  Arm64Clz = 116,
  /// kArm64Clz32
  Arm64Clz32 = 117,
  /// kArm64Cmp
  Arm64Cmp = 118,
  /// kArm64Cmp32
  Arm64Cmp32 = 119,
  /// kArm64Cmn
  Arm64Cmn = 120,
  /// kArm64Cmn32
  Arm64Cmn32 = 121,
  /// kArm64Cnt
  Arm64Cnt = 122,
  /// kArm64Cnt32
  Arm64Cnt32 = 123,
  /// kArm64Cnt64
  Arm64Cnt64 = 124,
  /// kArm64Ctz
  Arm64Ctz = 125,
  /// kArm64Ctz32
  Arm64Ctz32 = 126,
  /// kArm64Tst
  Arm64Tst = 127,
  /// kArm64Tst32
  Arm64Tst32 = 128,
  /// kArm64Or
  Arm64Or = 129,
  /// kArm64Or32
  Arm64Or32 = 130,
  /// kArm64Orn
  Arm64Orn = 131,
  /// kArm64Orn32
  Arm64Orn32 = 132,
  /// kArm64Eor
  Arm64Eor = 133,
  /// kArm64Eor32
  Arm64Eor32 = 134,
  /// kArm64Eon
  Arm64Eon = 135,
  /// kArm64Eon32
  Arm64Eon32 = 136,
  /// kArm64Sub
  Arm64Sub = 137,
  /// kArm64Sub32
  Arm64Sub32 = 138,
  /// kArm64Mul
  Arm64Mul = 139,
  /// kArm64Mul32
  Arm64Mul32 = 140,
  /// kArm64Smulh
  Arm64Smulh = 141,
  /// kArm64Smull
  Arm64Smull = 142,
  /// kArm64Smull2
  Arm64Smull2 = 143,
  /// kArm64Umull
  Arm64Umull = 144,
  /// kArm64Umulh
  Arm64Umulh = 145,
  /// kArm64Umull2
  Arm64Umull2 = 146,
  /// kArm64Madd
  Arm64Madd = 147,
  /// kArm64Madd32
  Arm64Madd32 = 148,
  /// kArm64Msub
  Arm64Msub = 149,
  /// kArm64Msub32
  Arm64Msub32 = 150,
  /// kArm64Mneg
  Arm64Mneg = 151,
  /// kArm64Mneg32
  Arm64Mneg32 = 152,
  /// kArm64Idiv
  Arm64Idiv = 153,
  /// kArm64Idiv32
  Arm64Idiv32 = 154,
  /// kArm64Udiv
  Arm64Udiv = 155,
  /// kArm64Udiv32
  Arm64Udiv32 = 156,
  /// kArm64Imod
  Arm64Imod = 157,
  /// kArm64Imod32
  Arm64Imod32 = 158,
  /// kArm64Umod
  Arm64Umod = 159,
  /// kArm64Umod32
  Arm64Umod32 = 160,
  /// kArm64Not
  Arm64Not = 161,
  /// kArm64Not32
  Arm64Not32 = 162,
  /// kArm64Lsl
  Arm64Lsl = 163,
  /// kArm64Lsl32
  Arm64Lsl32 = 164,
  /// kArm64Lsr
  Arm64Lsr = 165,
  /// kArm64Lsr32
  Arm64Lsr32 = 166,
  /// kArm64Asr
  Arm64Asr = 167,
  /// kArm64Asr32
  Arm64Asr32 = 168,
  /// kArm64Ror
  Arm64Ror = 169,
  /// kArm64Ror32
  Arm64Ror32 = 170,
  /// kArm64Mov32
  Arm64Mov32 = 171,
  /// kArm64Sxtb32
  Arm64Sxtb32 = 172,
  /// kArm64Sxth32
  Arm64Sxth32 = 173,
  /// kArm64Sxtb
  Arm64Sxtb = 174,
  /// kArm64Sxth
  Arm64Sxth = 175,
  /// kArm64Sxtw
  Arm64Sxtw = 176,
  /// kArm64Sbfx
  Arm64Sbfx = 177,
  /// kArm64Sbfx32
  Arm64Sbfx32 = 178,
  /// kArm64Ubfx
  Arm64Ubfx = 179,
  /// kArm64Ubfx32
  Arm64Ubfx32 = 180,
  /// kArm64Ubfiz
  Arm64Ubfiz = 181,
  /// kArm64Ubfiz32
  Arm64Ubfiz32 = 182,
  /// kArm64Sbfiz
  Arm64Sbfiz = 183,
  /// kArm64Bfi
  Arm64Bfi = 184,
  /// kArm64Rbit
  Arm64Rbit = 185,
  /// kArm64Rbit32
  Arm64Rbit32 = 186,
  /// kArm64Rev
  Arm64Rev = 187,
  /// kArm64Rev32
  Arm64Rev32 = 188,
  /// kArm64Smax32
  Arm64Smax32 = 189,
  /// kArm64Smax64
  Arm64Smax64 = 190,
  /// kArm64Smin32
  Arm64Smin32 = 191,
  /// kArm64Smin64
  Arm64Smin64 = 192,
  /// kArm64Umax32
  Arm64Umax32 = 193,
  /// kArm64Umax64
  Arm64Umax64 = 194,
  /// kArm64Umin32
  Arm64Umin32 = 195,
  /// kArm64Umin64
  Arm64Umin64 = 196,
  /// kArm64TestAndBranch32
  Arm64TestAndBranch32 = 197,
  /// kArm64TestAndBranch
  Arm64TestAndBranch = 198,
  /// kArm64CompareAndBranch32
  Arm64CompareAndBranch32 = 199,
  /// kArm64CompareAndBranch
  Arm64CompareAndBranch = 200,
  /// kArm64Claim
  Arm64Claim = 201,
  /// kArm64Poke
  Arm64Poke = 202,
  /// kArm64PokePair
  Arm64PokePair = 203,
  /// kArm64Peek
  Arm64Peek = 204,
  /// kArm64Float16RoundDown
  Arm64Float16RoundDown = 205,
  /// kArm64Float16RoundUp
  Arm64Float16RoundUp = 206,
  /// kArm64Float16RoundTruncate
  Arm64Float16RoundTruncate = 207,
  /// kArm64Float16RoundTiesEven
  Arm64Float16RoundTiesEven = 208,
  /// kArm64Float32Move
  Arm64Float32Move = 209,
  /// kArm64Float32MoveU32
  Arm64Float32MoveU32 = 210,
  /// kArm64Float32Cmp
  Arm64Float32Cmp = 211,
  /// kArm64Float32Add
  Arm64Float32Add = 212,
  /// kArm64Float32Sub
  Arm64Float32Sub = 213,
  /// kArm64Float32Mul
  Arm64Float32Mul = 214,
  /// kArm64Float32Div
  Arm64Float32Div = 215,
  /// kArm64Float32Abs
  Arm64Float32Abs = 216,
  /// kArm64Float32Abd
  Arm64Float32Abd = 217,
  /// kArm64Float32Neg
  Arm64Float32Neg = 218,
  /// kArm64Float32Sqrt
  Arm64Float32Sqrt = 219,
  /// kArm64Float32Fnmul
  Arm64Float32Fnmul = 220,
  /// kArm64Float32RoundDown
  Arm64Float32RoundDown = 221,
  /// kArm64Float32Max
  Arm64Float32Max = 222,
  /// kArm64Float32Min
  Arm64Float32Min = 223,
  /// kArm64Float64Cmp
  Arm64Float64Cmp = 224,
  /// kArm64Float64Add
  Arm64Float64Add = 225,
  /// kArm64Float64Sub
  Arm64Float64Sub = 226,
  /// kArm64Float64Mul
  Arm64Float64Mul = 227,
  /// kArm64Float64Div
  Arm64Float64Div = 228,
  /// kArm64Float64Mod
  Arm64Float64Mod = 229,
  /// kArm64Float64Max
  Arm64Float64Max = 230,
  /// kArm64Float64Min
  Arm64Float64Min = 231,
  /// kArm64Float64Abs
  Arm64Float64Abs = 232,
  /// kArm64Float64Abd
  Arm64Float64Abd = 233,
  /// kArm64Float64Neg
  Arm64Float64Neg = 234,
  /// kArm64Float64Sqrt
  Arm64Float64Sqrt = 235,
  /// kArm64Float64Fnmul
  Arm64Float64Fnmul = 236,
  /// kArm64Float64RoundDown
  Arm64Float64RoundDown = 237,
  /// kArm64Float32RoundUp
  Arm64Float32RoundUp = 238,
  /// kArm64Float64RoundUp
  Arm64Float64RoundUp = 239,
  /// kArm64Float64RoundTiesAway
  Arm64Float64RoundTiesAway = 240,
  /// kArm64Float32RoundTruncate
  Arm64Float32RoundTruncate = 241,
  /// kArm64Float64RoundTruncate
  Arm64Float64RoundTruncate = 242,
  /// kArm64Float32RoundTiesEven
  Arm64Float32RoundTiesEven = 243,
  /// kArm64Float64RoundTiesEven
  Arm64Float64RoundTiesEven = 244,
  /// kArm64Float64SilenceNaN
  Arm64Float64SilenceNaN = 245,
  /// kArm64Float32ToFloat64
  Arm64Float32ToFloat64 = 246,
  /// kArm64Float64ToFloat32
  Arm64Float64ToFloat32 = 247,
  /// kArm64Float64ToFloat16RawBits
  Arm64Float64ToFloat16RawBits = 248,
  /// kArm64Float16RawBitsToFloat64
  Arm64Float16RawBitsToFloat64 = 249,
  /// kArm64Float32ToInt32
  Arm64Float32ToInt32 = 250,
  /// kArm64Float64ToInt32
  Arm64Float64ToInt32 = 251,
  /// kArm64Float32ToUint32
  Arm64Float32ToUint32 = 252,
  /// kArm64Float64ToUint32
  Arm64Float64ToUint32 = 253,
  /// kArm64Float32ToInt64
  Arm64Float32ToInt64 = 254,
  /// kArm64Float64ToInt64
  Arm64Float64ToInt64 = 255,
  /// kArm64Float32ToUint64
  Arm64Float32ToUint64 = 256,
  /// kArm64Float64ToUint64
  Arm64Float64ToUint64 = 257,
  /// kArm64Int32ToFloat32
  Arm64Int32ToFloat32 = 258,
  /// kArm64Int32ToFloat64
  Arm64Int32ToFloat64 = 259,
  /// kArm64Int64ToFloat32
  Arm64Int64ToFloat32 = 260,
  /// kArm64Int64ToFloat64
  Arm64Int64ToFloat64 = 261,
  /// kArm64Uint32ToFloat32
  Arm64Uint32ToFloat32 = 262,
  /// kArm64Uint32ToFloat64
  Arm64Uint32ToFloat64 = 263,
  /// kArm64Uint64ToFloat32
  Arm64Uint64ToFloat32 = 264,
  /// kArm64Uint64ToFloat64
  Arm64Uint64ToFloat64 = 265,
  /// kArm64Float64ExtractLowWord32
  Arm64Float64ExtractLowWord32 = 266,
  /// kArm64Float64ExtractHighWord32
  Arm64Float64ExtractHighWord32 = 267,
  /// kArm64Float64InsertLowWord32
  Arm64Float64InsertLowWord32 = 268,
  /// kArm64Float64InsertHighWord32
  Arm64Float64InsertHighWord32 = 269,
  /// kArm64Float64Move
  Arm64Float64Move = 270,
  /// kArm64Float64MoveU64
  Arm64Float64MoveU64 = 271,
  /// kArm64U64MoveFloat64
  Arm64U64MoveFloat64 = 272,
  /// kArm64LdarDecompressTaggedSigned
  Arm64LdarDecompressTaggedSigned = 273,
  /// kArm64LdarDecompressTagged
  Arm64LdarDecompressTagged = 274,
  /// kArm64StlrCompressTagged
  Arm64StlrCompressTagged = 275,
  /// kArm64StrIndirectPointer
  Arm64StrIndirectPointer = 276,
  /// kArm64LdrDecodeSandboxedPointer
  Arm64LdrDecodeSandboxedPointer = 277,
  /// kArm64StrEncodeSandboxedPointer
  Arm64StrEncodeSandboxedPointer = 278,
  /// kArm64DmbIsh
  Arm64DmbIsh = 279,
  /// kArm64DsbIsb
  Arm64DsbIsb = 280,
  /// kArm64Word64AtomicAddUint64
  Arm64Word64AtomicAddUint64 = 281,
  /// kArm64Word64AtomicSubUint64
  Arm64Word64AtomicSubUint64 = 282,
  /// kArm64Word64AtomicAndUint64
  Arm64Word64AtomicAndUint64 = 283,
  /// kArm64Word64AtomicOrUint64
  Arm64Word64AtomicOrUint64 = 284,
  /// kArm64Word64AtomicXorUint64
  Arm64Word64AtomicXorUint64 = 285,
  /// kArm64Word64AtomicExchangeUint64
  Arm64Word64AtomicExchangeUint64 = 286,
  /// kArm64Word64AtomicCompareExchangeUint64
  Arm64Word64AtomicCompareExchangeUint64 = 287,
  /// kArm64Cpy
  Arm64Cpy = 288,
  /// kArm64Set
  Arm64Set = 289,
  /// kArm64Ldr
  Arm64Ldr = 290,
  /// kArm64Ldrb
  Arm64Ldrb = 291,
  /// kArm64LdrD
  Arm64LdrD = 292,
  /// kArm64Ldrh
  Arm64Ldrh = 293,
  /// kArm64LdrQ
  Arm64LdrQ = 294,
  /// kArm64LdrS
  Arm64LdrS = 295,
  /// kArm64LdrH
  Arm64LdrH = 296,
  /// kArm64Ldrsb
  Arm64Ldrsb = 297,
  /// kArm64LdrsbW
  Arm64LdrsbW = 298,
  /// kArm64Ldrsh
  Arm64Ldrsh = 299,
  /// kArm64LdrshW
  Arm64LdrshW = 300,
  /// kArm64Ldrsw
  Arm64Ldrsw = 301,
  /// kArm64LdrW
  Arm64LdrW = 302,
  /// kArm64Str
  Arm64Str = 303,
  /// kArm64StrPair
  Arm64StrPair = 304,
  /// kArm64Strb
  Arm64Strb = 305,
  /// kArm64StrD
  Arm64StrD = 306,
  /// kArm64Strh
  Arm64Strh = 307,
  /// kArm64StrQ
  Arm64StrQ = 308,
  /// kArm64StrS
  Arm64StrS = 309,
  /// kArm64StrH
  Arm64StrH = 310,
  /// kArm64StrW
  Arm64StrW = 311,
  /// kArm64StrWPair
  Arm64StrWPair = 312,
  /// kArm64LdrDecompressTaggedSigned
  Arm64LdrDecompressTaggedSigned = 313,
  /// kArm64LdrDecompressTagged
  Arm64LdrDecompressTagged = 314,
  /// kArm64LdrDecompressProtected
  Arm64LdrDecompressProtected = 315,
  /// kArm64StrCompressTagged
  Arm64StrCompressTagged = 316,
  /// kArm64Word64AtomicLoadUint64
  Arm64Word64AtomicLoadUint64 = 317,
  /// kArm64Word64AtomicStoreWord64
  Arm64Word64AtomicStoreWord64 = 318,
  /// kArm64LoadLane
  Arm64LoadLane = 319,
  /// kArm64LoadSplat
  Arm64LoadSplat = 320,
  /// kArm64StoreLane
  Arm64StoreLane = 321,
  /// kArm64S128LoadPairDeinterleave
  Arm64S128LoadPairDeinterleave = 322,
}

/// 原版 kLastArchOpcode (条数 - 1)
pub const LAST_ARCH_OPCODE: u16 = 322;

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
  ArchOpcode::Arm64Ldr,
  ArchOpcode::Arm64Ldrb,
  ArchOpcode::Arm64LdrD,
  ArchOpcode::Arm64Ldrh,
  ArchOpcode::Arm64LdrQ,
  ArchOpcode::Arm64LdrS,
  ArchOpcode::Arm64LdrH,
  ArchOpcode::Arm64Ldrsb,
  ArchOpcode::Arm64LdrsbW,
  ArchOpcode::Arm64Ldrsh,
  ArchOpcode::Arm64LdrshW,
  ArchOpcode::Arm64Ldrsw,
  ArchOpcode::Arm64LdrW,
  ArchOpcode::Arm64Str,
  ArchOpcode::Arm64StrPair,
  ArchOpcode::Arm64Strb,
  ArchOpcode::Arm64StrD,
  ArchOpcode::Arm64Strh,
  ArchOpcode::Arm64StrQ,
  ArchOpcode::Arm64StrS,
  ArchOpcode::Arm64StrH,
  ArchOpcode::Arm64StrW,
  ArchOpcode::Arm64StrWPair,
  ArchOpcode::Arm64LdrDecompressTaggedSigned,
  ArchOpcode::Arm64LdrDecompressTagged,
  ArchOpcode::Arm64LdrDecompressProtected,
  ArchOpcode::Arm64StrCompressTagged,
  ArchOpcode::Arm64Word64AtomicLoadUint64,
  ArchOpcode::Arm64Word64AtomicStoreWord64,
  ArchOpcode::Arm64LoadLane,
  ArchOpcode::Arm64LoadSplat,
  ArchOpcode::Arm64StoreLane,
  ArchOpcode::Arm64S128LoadPairDeinterleave,
];

/// 原版 AddressingMode (None + TARGET_ADDRESSING_MODE_LIST)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[rustfmt::skip]
pub enum AddressingMode {
  /// kMode_None
  ModeNone = 0,
  /// kMode_MRI
  ModeMRI = 1,
  /// kMode_MRR
  ModeMRR = 2,
  /// kMode_Operand2_R_LSL_I
  ModeOperand2RLslI = 3,
  /// kMode_Operand2_R_LSR_I
  ModeOperand2RLsrI = 4,
  /// kMode_Operand2_R_ASR_I
  ModeOperand2RAsrI = 5,
  /// kMode_Operand2_R_ROR_I
  ModeOperand2RRorI = 6,
  /// kMode_Operand2_R_UXTB
  ModeOperand2RUxtb = 7,
  /// kMode_Operand2_R_UXTH
  ModeOperand2RUxth = 8,
  /// kMode_Operand2_R_SXTB
  ModeOperand2RSxtb = 9,
  /// kMode_Operand2_R_SXTH
  ModeOperand2RSxth = 10,
  /// kMode_Operand2_R_SXTW
  ModeOperand2RSxtw = 11,
  /// kMode_Root
  ModeRoot = 12,
}

/// 原版 kLastAddressingMode (条数 - 1)
pub const LAST_ADDRESSING_MODE: u16 = 12;

// —— 提取不变式断言 ——
const _: () = {
  assert!(LAST_ARCH_OPCODE as u32 >= 100);
  assert!(ArchOpcode::ArchJmp as u16 >= 8);
  assert!(AddressingMode::ModeNone as u16 == 0);
  assert!(!HAS_MEMORY_ACCESS_MODE.is_empty());
};
