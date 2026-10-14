// 由 tool/src/wasm/interpreter/instruction-handlers.h.js 从 v8/src/wasm/interpreter/instruction-handlers.h 生成; 修订就地改, 复跑覆盖
// InstructionHandler 枚举 (X-macro 全展开; 缺省配置: 无 BOUNDS_CHECKS / 无 TRACING)

/// 指令处理函数枚举 (对应原版 enum InstructionHandler : uint16_t)
/// 展开自 FOREACH_INSTR_HANDLER 宏链; 对数锚: clang -E 展开 1690 项
/// 配置: V8_DRUMBRAKE_BOUNDS_CHECKS 关, V8_ENABLE_DRUMBRAKE_TRACING 关
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum InstructionHandler {
  /// k_r2r_I32LoadMem8S
  R2rI32LoadMem8S = 0,
  /// k_r2r_I32LoadMem8U
  R2rI32LoadMem8U = 1,
  /// k_r2r_I32LoadMem16S
  R2rI32LoadMem16S = 2,
  /// k_r2r_I32LoadMem16U
  R2rI32LoadMem16U = 3,
  /// k_r2r_I64LoadMem8S
  R2rI64LoadMem8S = 4,
  /// k_r2r_I64LoadMem8U
  R2rI64LoadMem8U = 5,
  /// k_r2r_I64LoadMem16S
  R2rI64LoadMem16S = 6,
  /// k_r2r_I64LoadMem16U
  R2rI64LoadMem16U = 7,
  /// k_r2r_I64LoadMem32S
  R2rI64LoadMem32S = 8,
  /// k_r2r_I64LoadMem32U
  R2rI64LoadMem32U = 9,
  /// k_r2r_I32LoadMem
  R2rI32LoadMem = 10,
  /// k_r2r_I64LoadMem
  R2rI64LoadMem = 11,
  /// k_r2r_F32LoadMem
  R2rF32LoadMem = 12,
  /// k_r2r_F64LoadMem
  R2rF64LoadMem = 13,
  /// k_r2s_I32LoadMem8S
  R2sI32LoadMem8S = 14,
  /// k_r2s_I32LoadMem8U
  R2sI32LoadMem8U = 15,
  /// k_r2s_I32LoadMem16S
  R2sI32LoadMem16S = 16,
  /// k_r2s_I32LoadMem16U
  R2sI32LoadMem16U = 17,
  /// k_r2s_I64LoadMem8S
  R2sI64LoadMem8S = 18,
  /// k_r2s_I64LoadMem8U
  R2sI64LoadMem8U = 19,
  /// k_r2s_I64LoadMem16S
  R2sI64LoadMem16S = 20,
  /// k_r2s_I64LoadMem16U
  R2sI64LoadMem16U = 21,
  /// k_r2s_I64LoadMem32S
  R2sI64LoadMem32S = 22,
  /// k_r2s_I64LoadMem32U
  R2sI64LoadMem32U = 23,
  /// k_r2s_I32LoadMem
  R2sI32LoadMem = 24,
  /// k_r2s_I64LoadMem
  R2sI64LoadMem = 25,
  /// k_r2s_F32LoadMem
  R2sF32LoadMem = 26,
  /// k_r2s_F64LoadMem
  R2sF64LoadMem = 27,
  /// k_s2r_I32LoadMem8S
  S2rI32LoadMem8S = 28,
  /// k_s2r_I32LoadMem8U
  S2rI32LoadMem8U = 29,
  /// k_s2r_I32LoadMem16S
  S2rI32LoadMem16S = 30,
  /// k_s2r_I32LoadMem16U
  S2rI32LoadMem16U = 31,
  /// k_s2r_I64LoadMem8S
  S2rI64LoadMem8S = 32,
  /// k_s2r_I64LoadMem8U
  S2rI64LoadMem8U = 33,
  /// k_s2r_I64LoadMem16S
  S2rI64LoadMem16S = 34,
  /// k_s2r_I64LoadMem16U
  S2rI64LoadMem16U = 35,
  /// k_s2r_I64LoadMem32S
  S2rI64LoadMem32S = 36,
  /// k_s2r_I64LoadMem32U
  S2rI64LoadMem32U = 37,
  /// k_s2r_I32LoadMem
  S2rI32LoadMem = 38,
  /// k_s2r_I64LoadMem
  S2rI64LoadMem = 39,
  /// k_s2r_F32LoadMem
  S2rF32LoadMem = 40,
  /// k_s2r_F64LoadMem
  S2rF64LoadMem = 41,
  /// k_s2s_I32LoadMem8S
  S2sI32LoadMem8S = 42,
  /// k_s2s_I32LoadMem8U
  S2sI32LoadMem8U = 43,
  /// k_s2s_I32LoadMem16S
  S2sI32LoadMem16S = 44,
  /// k_s2s_I32LoadMem16U
  S2sI32LoadMem16U = 45,
  /// k_s2s_I64LoadMem8S
  S2sI64LoadMem8S = 46,
  /// k_s2s_I64LoadMem8U
  S2sI64LoadMem8U = 47,
  /// k_s2s_I64LoadMem16S
  S2sI64LoadMem16S = 48,
  /// k_s2s_I64LoadMem16U
  S2sI64LoadMem16U = 49,
  /// k_s2s_I64LoadMem32S
  S2sI64LoadMem32S = 50,
  /// k_s2s_I64LoadMem32U
  S2sI64LoadMem32U = 51,
  /// k_s2s_I32LoadMem
  S2sI32LoadMem = 52,
  /// k_s2s_I64LoadMem
  S2sI64LoadMem = 53,
  /// k_s2s_F32LoadMem
  S2sF32LoadMem = 54,
  /// k_s2s_F64LoadMem
  S2sF64LoadMem = 55,
  /// k_r2s_I32StoreMem8
  R2sI32StoreMem8 = 56,
  /// k_r2s_I32StoreMem16
  R2sI32StoreMem16 = 57,
  /// k_r2s_I64StoreMem8
  R2sI64StoreMem8 = 58,
  /// k_r2s_I64StoreMem16
  R2sI64StoreMem16 = 59,
  /// k_r2s_I64StoreMem32
  R2sI64StoreMem32 = 60,
  /// k_r2s_I32StoreMem
  R2sI32StoreMem = 61,
  /// k_r2s_I64StoreMem
  R2sI64StoreMem = 62,
  /// k_r2s_F32StoreMem
  R2sF32StoreMem = 63,
  /// k_r2s_F64StoreMem
  R2sF64StoreMem = 64,
  /// k_s2s_I32StoreMem8
  S2sI32StoreMem8 = 65,
  /// k_s2s_I32StoreMem16
  S2sI32StoreMem16 = 66,
  /// k_s2s_I64StoreMem8
  S2sI64StoreMem8 = 67,
  /// k_s2s_I64StoreMem16
  S2sI64StoreMem16 = 68,
  /// k_s2s_I64StoreMem32
  S2sI64StoreMem32 = 69,
  /// k_s2s_I32StoreMem
  S2sI32StoreMem = 70,
  /// k_s2s_I64StoreMem
  S2sI64StoreMem = 71,
  /// k_s2s_F32StoreMem
  S2sF32StoreMem = 72,
  /// k_s2s_F64StoreMem
  S2sF64StoreMem = 73,
  /// k_s2s_I32LoadMem8S_LocalSet
  S2sI32LoadMem8SLocalSet = 74,
  /// k_s2s_I32LoadMem8U_LocalSet
  S2sI32LoadMem8ULocalSet = 75,
  /// k_s2s_I32LoadMem16S_LocalSet
  S2sI32LoadMem16SLocalSet = 76,
  /// k_s2s_I32LoadMem16U_LocalSet
  S2sI32LoadMem16ULocalSet = 77,
  /// k_s2s_I64LoadMem8S_LocalSet
  S2sI64LoadMem8SLocalSet = 78,
  /// k_s2s_I64LoadMem8U_LocalSet
  S2sI64LoadMem8ULocalSet = 79,
  /// k_s2s_I64LoadMem16S_LocalSet
  S2sI64LoadMem16SLocalSet = 80,
  /// k_s2s_I64LoadMem16U_LocalSet
  S2sI64LoadMem16ULocalSet = 81,
  /// k_s2s_I64LoadMem32S_LocalSet
  S2sI64LoadMem32SLocalSet = 82,
  /// k_s2s_I64LoadMem32U_LocalSet
  S2sI64LoadMem32ULocalSet = 83,
  /// k_s2s_I32LoadMem_LocalSet
  S2sI32LoadMemLocalSet = 84,
  /// k_s2s_I64LoadMem_LocalSet
  S2sI64LoadMemLocalSet = 85,
  /// k_s2s_F32LoadMem_LocalSet
  S2sF32LoadMemLocalSet = 86,
  /// k_s2s_F64LoadMem_LocalSet
  S2sF64LoadMemLocalSet = 87,
  /// k_r2s_I32LoadStoreMem
  R2sI32LoadStoreMem = 88,
  /// k_r2s_I64LoadStoreMem
  R2sI64LoadStoreMem = 89,
  /// k_r2s_F32LoadStoreMem
  R2sF32LoadStoreMem = 90,
  /// k_r2s_F64LoadStoreMem
  R2sF64LoadStoreMem = 91,
  /// k_s2s_I32LoadStoreMem
  S2sI32LoadStoreMem = 92,
  /// k_s2s_I64LoadStoreMem
  S2sI64LoadStoreMem = 93,
  /// k_s2s_F32LoadStoreMem
  S2sF32LoadStoreMem = 94,
  /// k_s2s_F64LoadStoreMem
  S2sF64LoadStoreMem = 95,
  /// k_s2r_I32GlobalGet
  S2rI32GlobalGet = 96,
  /// k_s2r_I64GlobalGet
  S2rI64GlobalGet = 97,
  /// k_s2r_F32GlobalGet
  S2rF32GlobalGet = 98,
  /// k_s2r_F64GlobalGet
  S2rF64GlobalGet = 99,
  /// k_s2s_I32GlobalGet
  S2sI32GlobalGet = 100,
  /// k_s2s_I64GlobalGet
  S2sI64GlobalGet = 101,
  /// k_s2s_F32GlobalGet
  S2sF32GlobalGet = 102,
  /// k_s2s_F64GlobalGet
  S2sF64GlobalGet = 103,
  /// k_s2s_S128GlobalGet
  S2sS128GlobalGet = 104,
  /// k_s2s_RefGlobalGet
  S2sRefGlobalGet = 105,
  /// k_r2s_I32GlobalSet
  R2sI32GlobalSet = 106,
  /// k_r2s_I64GlobalSet
  R2sI64GlobalSet = 107,
  /// k_r2s_F32GlobalSet
  R2sF32GlobalSet = 108,
  /// k_r2s_F64GlobalSet
  R2sF64GlobalSet = 109,
  /// k_s2s_I32GlobalSet
  S2sI32GlobalSet = 110,
  /// k_s2s_I64GlobalSet
  S2sI64GlobalSet = 111,
  /// k_s2s_F32GlobalSet
  S2sF32GlobalSet = 112,
  /// k_s2s_F64GlobalSet
  S2sF64GlobalSet = 113,
  /// k_s2s_S128GlobalSet
  S2sS128GlobalSet = 114,
  /// k_s2s_RefGlobalSet
  S2sRefGlobalSet = 115,
  /// k_r2s_I32Drop
  R2sI32Drop = 116,
  /// k_r2s_I64Drop
  R2sI64Drop = 117,
  /// k_r2s_F32Drop
  R2sF32Drop = 118,
  /// k_r2s_F64Drop
  R2sF64Drop = 119,
  /// k_r2s_RefDrop
  R2sRefDrop = 120,
  /// k_s2s_I32Drop
  S2sI32Drop = 121,
  /// k_s2s_I64Drop
  S2sI64Drop = 122,
  /// k_s2s_F32Drop
  S2sF32Drop = 123,
  /// k_s2s_F64Drop
  S2sF64Drop = 124,
  /// k_s2s_S128Drop
  S2sS128Drop = 125,
  /// k_s2s_RefDrop
  S2sRefDrop = 126,
  /// k_r2r_I32Select
  R2rI32Select = 127,
  /// k_r2r_I64Select
  R2rI64Select = 128,
  /// k_r2r_F32Select
  R2rF32Select = 129,
  /// k_r2r_F64Select
  R2rF64Select = 130,
  /// k_r2s_I32Select
  R2sI32Select = 131,
  /// k_r2s_I64Select
  R2sI64Select = 132,
  /// k_r2s_F32Select
  R2sF32Select = 133,
  /// k_r2s_F64Select
  R2sF64Select = 134,
  /// k_r2s_S128Select
  R2sS128Select = 135,
  /// k_r2s_RefSelect
  R2sRefSelect = 136,
  /// k_s2r_I32Select
  S2rI32Select = 137,
  /// k_s2r_I64Select
  S2rI64Select = 138,
  /// k_s2r_F32Select
  S2rF32Select = 139,
  /// k_s2r_F64Select
  S2rF64Select = 140,
  /// k_s2s_I32Select
  S2sI32Select = 141,
  /// k_s2s_I64Select
  S2sI64Select = 142,
  /// k_s2s_F32Select
  S2sF32Select = 143,
  /// k_s2s_F64Select
  S2sF64Select = 144,
  /// k_s2s_S128Select
  S2sS128Select = 145,
  /// k_s2s_RefSelect
  S2sRefSelect = 146,
  /// k_r2r_I32Add
  R2rI32Add = 147,
  /// k_r2r_I32Sub
  R2rI32Sub = 148,
  /// k_r2r_I32Mul
  R2rI32Mul = 149,
  /// k_r2r_I32And
  R2rI32And = 150,
  /// k_r2r_I32Ior
  R2rI32Ior = 151,
  /// k_r2r_I32Xor
  R2rI32Xor = 152,
  /// k_r2r_I64Add
  R2rI64Add = 153,
  /// k_r2r_I64Sub
  R2rI64Sub = 154,
  /// k_r2r_I64Mul
  R2rI64Mul = 155,
  /// k_r2r_I64And
  R2rI64And = 156,
  /// k_r2r_I64Ior
  R2rI64Ior = 157,
  /// k_r2r_I64Xor
  R2rI64Xor = 158,
  /// k_r2r_F32Add
  R2rF32Add = 159,
  /// k_r2r_F32Sub
  R2rF32Sub = 160,
  /// k_r2r_F32Mul
  R2rF32Mul = 161,
  /// k_r2r_F64Add
  R2rF64Add = 162,
  /// k_r2r_F64Sub
  R2rF64Sub = 163,
  /// k_r2r_F64Mul
  R2rF64Mul = 164,
  /// k_r2r_I32DivS
  R2rI32DivS = 165,
  /// k_r2r_I64DivS
  R2rI64DivS = 166,
  /// k_r2r_I32DivU
  R2rI32DivU = 167,
  /// k_r2r_I64DivU
  R2rI64DivU = 168,
  /// k_r2r_F32Div
  R2rF32Div = 169,
  /// k_r2r_F64Div
  R2rF64Div = 170,
  /// k_r2r_I32RemS
  R2rI32RemS = 171,
  /// k_r2r_I64RemS
  R2rI64RemS = 172,
  /// k_r2r_I32RemU
  R2rI32RemU = 173,
  /// k_r2r_I64RemU
  R2rI64RemU = 174,
  /// k_r2s_I32Add
  R2sI32Add = 175,
  /// k_r2s_I32Sub
  R2sI32Sub = 176,
  /// k_r2s_I32Mul
  R2sI32Mul = 177,
  /// k_r2s_I32And
  R2sI32And = 178,
  /// k_r2s_I32Ior
  R2sI32Ior = 179,
  /// k_r2s_I32Xor
  R2sI32Xor = 180,
  /// k_r2s_I64Add
  R2sI64Add = 181,
  /// k_r2s_I64Sub
  R2sI64Sub = 182,
  /// k_r2s_I64Mul
  R2sI64Mul = 183,
  /// k_r2s_I64And
  R2sI64And = 184,
  /// k_r2s_I64Ior
  R2sI64Ior = 185,
  /// k_r2s_I64Xor
  R2sI64Xor = 186,
  /// k_r2s_F32Add
  R2sF32Add = 187,
  /// k_r2s_F32Sub
  R2sF32Sub = 188,
  /// k_r2s_F32Mul
  R2sF32Mul = 189,
  /// k_r2s_F64Add
  R2sF64Add = 190,
  /// k_r2s_F64Sub
  R2sF64Sub = 191,
  /// k_r2s_F64Mul
  R2sF64Mul = 192,
  /// k_r2s_I32DivS
  R2sI32DivS = 193,
  /// k_r2s_I64DivS
  R2sI64DivS = 194,
  /// k_r2s_I32DivU
  R2sI32DivU = 195,
  /// k_r2s_I64DivU
  R2sI64DivU = 196,
  /// k_r2s_F32Div
  R2sF32Div = 197,
  /// k_r2s_F64Div
  R2sF64Div = 198,
  /// k_r2s_I32RemS
  R2sI32RemS = 199,
  /// k_r2s_I64RemS
  R2sI64RemS = 200,
  /// k_r2s_I32RemU
  R2sI32RemU = 201,
  /// k_r2s_I64RemU
  R2sI64RemU = 202,
  /// k_s2r_I32Add
  S2rI32Add = 203,
  /// k_s2r_I32Sub
  S2rI32Sub = 204,
  /// k_s2r_I32Mul
  S2rI32Mul = 205,
  /// k_s2r_I32And
  S2rI32And = 206,
  /// k_s2r_I32Ior
  S2rI32Ior = 207,
  /// k_s2r_I32Xor
  S2rI32Xor = 208,
  /// k_s2r_I64Add
  S2rI64Add = 209,
  /// k_s2r_I64Sub
  S2rI64Sub = 210,
  /// k_s2r_I64Mul
  S2rI64Mul = 211,
  /// k_s2r_I64And
  S2rI64And = 212,
  /// k_s2r_I64Ior
  S2rI64Ior = 213,
  /// k_s2r_I64Xor
  S2rI64Xor = 214,
  /// k_s2r_F32Add
  S2rF32Add = 215,
  /// k_s2r_F32Sub
  S2rF32Sub = 216,
  /// k_s2r_F32Mul
  S2rF32Mul = 217,
  /// k_s2r_F64Add
  S2rF64Add = 218,
  /// k_s2r_F64Sub
  S2rF64Sub = 219,
  /// k_s2r_F64Mul
  S2rF64Mul = 220,
  /// k_s2r_I32DivS
  S2rI32DivS = 221,
  /// k_s2r_I64DivS
  S2rI64DivS = 222,
  /// k_s2r_I32DivU
  S2rI32DivU = 223,
  /// k_s2r_I64DivU
  S2rI64DivU = 224,
  /// k_s2r_F32Div
  S2rF32Div = 225,
  /// k_s2r_F64Div
  S2rF64Div = 226,
  /// k_s2r_I32RemS
  S2rI32RemS = 227,
  /// k_s2r_I64RemS
  S2rI64RemS = 228,
  /// k_s2r_I32RemU
  S2rI32RemU = 229,
  /// k_s2r_I64RemU
  S2rI64RemU = 230,
  /// k_s2s_I32Add
  S2sI32Add = 231,
  /// k_s2s_I32Sub
  S2sI32Sub = 232,
  /// k_s2s_I32Mul
  S2sI32Mul = 233,
  /// k_s2s_I32And
  S2sI32And = 234,
  /// k_s2s_I32Ior
  S2sI32Ior = 235,
  /// k_s2s_I32Xor
  S2sI32Xor = 236,
  /// k_s2s_I64Add
  S2sI64Add = 237,
  /// k_s2s_I64Sub
  S2sI64Sub = 238,
  /// k_s2s_I64Mul
  S2sI64Mul = 239,
  /// k_s2s_I64And
  S2sI64And = 240,
  /// k_s2s_I64Ior
  S2sI64Ior = 241,
  /// k_s2s_I64Xor
  S2sI64Xor = 242,
  /// k_s2s_F32Add
  S2sF32Add = 243,
  /// k_s2s_F32Sub
  S2sF32Sub = 244,
  /// k_s2s_F32Mul
  S2sF32Mul = 245,
  /// k_s2s_F64Add
  S2sF64Add = 246,
  /// k_s2s_F64Sub
  S2sF64Sub = 247,
  /// k_s2s_F64Mul
  S2sF64Mul = 248,
  /// k_s2s_I32DivS
  S2sI32DivS = 249,
  /// k_s2s_I64DivS
  S2sI64DivS = 250,
  /// k_s2s_I32DivU
  S2sI32DivU = 251,
  /// k_s2s_I64DivU
  S2sI64DivU = 252,
  /// k_s2s_F32Div
  S2sF32Div = 253,
  /// k_s2s_F64Div
  S2sF64Div = 254,
  /// k_s2s_I32RemS
  S2sI32RemS = 255,
  /// k_s2s_I64RemS
  S2sI64RemS = 256,
  /// k_s2s_I32RemU
  S2sI32RemU = 257,
  /// k_s2s_I64RemU
  S2sI64RemU = 258,
  /// k_r2r_I32Eq
  R2rI32Eq = 259,
  /// k_r2r_I32Ne
  R2rI32Ne = 260,
  /// k_r2r_I32LtU
  R2rI32LtU = 261,
  /// k_r2r_I32LeU
  R2rI32LeU = 262,
  /// k_r2r_I32GtU
  R2rI32GtU = 263,
  /// k_r2r_I32GeU
  R2rI32GeU = 264,
  /// k_r2r_I32LtS
  R2rI32LtS = 265,
  /// k_r2r_I32LeS
  R2rI32LeS = 266,
  /// k_r2r_I32GtS
  R2rI32GtS = 267,
  /// k_r2r_I32GeS
  R2rI32GeS = 268,
  /// k_r2r_I64Eq
  R2rI64Eq = 269,
  /// k_r2r_I64Ne
  R2rI64Ne = 270,
  /// k_r2r_I64LtU
  R2rI64LtU = 271,
  /// k_r2r_I64LeU
  R2rI64LeU = 272,
  /// k_r2r_I64GtU
  R2rI64GtU = 273,
  /// k_r2r_I64GeU
  R2rI64GeU = 274,
  /// k_r2r_I64LtS
  R2rI64LtS = 275,
  /// k_r2r_I64LeS
  R2rI64LeS = 276,
  /// k_r2r_I64GtS
  R2rI64GtS = 277,
  /// k_r2r_I64GeS
  R2rI64GeS = 278,
  /// k_r2r_F32Eq
  R2rF32Eq = 279,
  /// k_r2r_F32Ne
  R2rF32Ne = 280,
  /// k_r2r_F32Lt
  R2rF32Lt = 281,
  /// k_r2r_F32Le
  R2rF32Le = 282,
  /// k_r2r_F32Gt
  R2rF32Gt = 283,
  /// k_r2r_F32Ge
  R2rF32Ge = 284,
  /// k_r2r_F64Eq
  R2rF64Eq = 285,
  /// k_r2r_F64Ne
  R2rF64Ne = 286,
  /// k_r2r_F64Lt
  R2rF64Lt = 287,
  /// k_r2r_F64Le
  R2rF64Le = 288,
  /// k_r2r_F64Gt
  R2rF64Gt = 289,
  /// k_r2r_F64Ge
  R2rF64Ge = 290,
  /// k_r2s_I32Eq
  R2sI32Eq = 291,
  /// k_r2s_I32Ne
  R2sI32Ne = 292,
  /// k_r2s_I32LtU
  R2sI32LtU = 293,
  /// k_r2s_I32LeU
  R2sI32LeU = 294,
  /// k_r2s_I32GtU
  R2sI32GtU = 295,
  /// k_r2s_I32GeU
  R2sI32GeU = 296,
  /// k_r2s_I32LtS
  R2sI32LtS = 297,
  /// k_r2s_I32LeS
  R2sI32LeS = 298,
  /// k_r2s_I32GtS
  R2sI32GtS = 299,
  /// k_r2s_I32GeS
  R2sI32GeS = 300,
  /// k_r2s_I64Eq
  R2sI64Eq = 301,
  /// k_r2s_I64Ne
  R2sI64Ne = 302,
  /// k_r2s_I64LtU
  R2sI64LtU = 303,
  /// k_r2s_I64LeU
  R2sI64LeU = 304,
  /// k_r2s_I64GtU
  R2sI64GtU = 305,
  /// k_r2s_I64GeU
  R2sI64GeU = 306,
  /// k_r2s_I64LtS
  R2sI64LtS = 307,
  /// k_r2s_I64LeS
  R2sI64LeS = 308,
  /// k_r2s_I64GtS
  R2sI64GtS = 309,
  /// k_r2s_I64GeS
  R2sI64GeS = 310,
  /// k_r2s_F32Eq
  R2sF32Eq = 311,
  /// k_r2s_F32Ne
  R2sF32Ne = 312,
  /// k_r2s_F32Lt
  R2sF32Lt = 313,
  /// k_r2s_F32Le
  R2sF32Le = 314,
  /// k_r2s_F32Gt
  R2sF32Gt = 315,
  /// k_r2s_F32Ge
  R2sF32Ge = 316,
  /// k_r2s_F64Eq
  R2sF64Eq = 317,
  /// k_r2s_F64Ne
  R2sF64Ne = 318,
  /// k_r2s_F64Lt
  R2sF64Lt = 319,
  /// k_r2s_F64Le
  R2sF64Le = 320,
  /// k_r2s_F64Gt
  R2sF64Gt = 321,
  /// k_r2s_F64Ge
  R2sF64Ge = 322,
  /// k_s2r_I32Eq
  S2rI32Eq = 323,
  /// k_s2r_I32Ne
  S2rI32Ne = 324,
  /// k_s2r_I32LtU
  S2rI32LtU = 325,
  /// k_s2r_I32LeU
  S2rI32LeU = 326,
  /// k_s2r_I32GtU
  S2rI32GtU = 327,
  /// k_s2r_I32GeU
  S2rI32GeU = 328,
  /// k_s2r_I32LtS
  S2rI32LtS = 329,
  /// k_s2r_I32LeS
  S2rI32LeS = 330,
  /// k_s2r_I32GtS
  S2rI32GtS = 331,
  /// k_s2r_I32GeS
  S2rI32GeS = 332,
  /// k_s2r_I64Eq
  S2rI64Eq = 333,
  /// k_s2r_I64Ne
  S2rI64Ne = 334,
  /// k_s2r_I64LtU
  S2rI64LtU = 335,
  /// k_s2r_I64LeU
  S2rI64LeU = 336,
  /// k_s2r_I64GtU
  S2rI64GtU = 337,
  /// k_s2r_I64GeU
  S2rI64GeU = 338,
  /// k_s2r_I64LtS
  S2rI64LtS = 339,
  /// k_s2r_I64LeS
  S2rI64LeS = 340,
  /// k_s2r_I64GtS
  S2rI64GtS = 341,
  /// k_s2r_I64GeS
  S2rI64GeS = 342,
  /// k_s2r_F32Eq
  S2rF32Eq = 343,
  /// k_s2r_F32Ne
  S2rF32Ne = 344,
  /// k_s2r_F32Lt
  S2rF32Lt = 345,
  /// k_s2r_F32Le
  S2rF32Le = 346,
  /// k_s2r_F32Gt
  S2rF32Gt = 347,
  /// k_s2r_F32Ge
  S2rF32Ge = 348,
  /// k_s2r_F64Eq
  S2rF64Eq = 349,
  /// k_s2r_F64Ne
  S2rF64Ne = 350,
  /// k_s2r_F64Lt
  S2rF64Lt = 351,
  /// k_s2r_F64Le
  S2rF64Le = 352,
  /// k_s2r_F64Gt
  S2rF64Gt = 353,
  /// k_s2r_F64Ge
  S2rF64Ge = 354,
  /// k_s2s_I32Eq
  S2sI32Eq = 355,
  /// k_s2s_I32Ne
  S2sI32Ne = 356,
  /// k_s2s_I32LtU
  S2sI32LtU = 357,
  /// k_s2s_I32LeU
  S2sI32LeU = 358,
  /// k_s2s_I32GtU
  S2sI32GtU = 359,
  /// k_s2s_I32GeU
  S2sI32GeU = 360,
  /// k_s2s_I32LtS
  S2sI32LtS = 361,
  /// k_s2s_I32LeS
  S2sI32LeS = 362,
  /// k_s2s_I32GtS
  S2sI32GtS = 363,
  /// k_s2s_I32GeS
  S2sI32GeS = 364,
  /// k_s2s_I64Eq
  S2sI64Eq = 365,
  /// k_s2s_I64Ne
  S2sI64Ne = 366,
  /// k_s2s_I64LtU
  S2sI64LtU = 367,
  /// k_s2s_I64LeU
  S2sI64LeU = 368,
  /// k_s2s_I64GtU
  S2sI64GtU = 369,
  /// k_s2s_I64GeU
  S2sI64GeU = 370,
  /// k_s2s_I64LtS
  S2sI64LtS = 371,
  /// k_s2s_I64LeS
  S2sI64LeS = 372,
  /// k_s2s_I64GtS
  S2sI64GtS = 373,
  /// k_s2s_I64GeS
  S2sI64GeS = 374,
  /// k_s2s_F32Eq
  S2sF32Eq = 375,
  /// k_s2s_F32Ne
  S2sF32Ne = 376,
  /// k_s2s_F32Lt
  S2sF32Lt = 377,
  /// k_s2s_F32Le
  S2sF32Le = 378,
  /// k_s2s_F32Gt
  S2sF32Gt = 379,
  /// k_s2s_F32Ge
  S2sF32Ge = 380,
  /// k_s2s_F64Eq
  S2sF64Eq = 381,
  /// k_s2s_F64Ne
  S2sF64Ne = 382,
  /// k_s2s_F64Lt
  S2sF64Lt = 383,
  /// k_s2s_F64Le
  S2sF64Le = 384,
  /// k_s2s_F64Gt
  S2sF64Gt = 385,
  /// k_s2s_F64Ge
  S2sF64Ge = 386,
  /// k_r2r_I32Shl
  R2rI32Shl = 387,
  /// k_r2r_I32ShrU
  R2rI32ShrU = 388,
  /// k_r2r_I32ShrS
  R2rI32ShrS = 389,
  /// k_r2r_I64Shl
  R2rI64Shl = 390,
  /// k_r2r_I64ShrU
  R2rI64ShrU = 391,
  /// k_r2r_I64ShrS
  R2rI64ShrS = 392,
  /// k_r2r_I32Rol
  R2rI32Rol = 393,
  /// k_r2r_I32Ror
  R2rI32Ror = 394,
  /// k_r2r_I64Rol
  R2rI64Rol = 395,
  /// k_r2r_I64Ror
  R2rI64Ror = 396,
  /// k_r2r_F32Min
  R2rF32Min = 397,
  /// k_r2r_F32Max
  R2rF32Max = 398,
  /// k_r2r_F64Min
  R2rF64Min = 399,
  /// k_r2r_F64Max
  R2rF64Max = 400,
  /// k_r2r_F32CopySign
  R2rF32CopySign = 401,
  /// k_r2r_F64CopySign
  R2rF64CopySign = 402,
  /// k_r2s_I32Shl
  R2sI32Shl = 403,
  /// k_r2s_I32ShrU
  R2sI32ShrU = 404,
  /// k_r2s_I32ShrS
  R2sI32ShrS = 405,
  /// k_r2s_I64Shl
  R2sI64Shl = 406,
  /// k_r2s_I64ShrU
  R2sI64ShrU = 407,
  /// k_r2s_I64ShrS
  R2sI64ShrS = 408,
  /// k_r2s_I32Rol
  R2sI32Rol = 409,
  /// k_r2s_I32Ror
  R2sI32Ror = 410,
  /// k_r2s_I64Rol
  R2sI64Rol = 411,
  /// k_r2s_I64Ror
  R2sI64Ror = 412,
  /// k_r2s_F32Min
  R2sF32Min = 413,
  /// k_r2s_F32Max
  R2sF32Max = 414,
  /// k_r2s_F64Min
  R2sF64Min = 415,
  /// k_r2s_F64Max
  R2sF64Max = 416,
  /// k_r2s_F32CopySign
  R2sF32CopySign = 417,
  /// k_r2s_F64CopySign
  R2sF64CopySign = 418,
  /// k_s2r_I32Shl
  S2rI32Shl = 419,
  /// k_s2r_I32ShrU
  S2rI32ShrU = 420,
  /// k_s2r_I32ShrS
  S2rI32ShrS = 421,
  /// k_s2r_I64Shl
  S2rI64Shl = 422,
  /// k_s2r_I64ShrU
  S2rI64ShrU = 423,
  /// k_s2r_I64ShrS
  S2rI64ShrS = 424,
  /// k_s2r_I32Rol
  S2rI32Rol = 425,
  /// k_s2r_I32Ror
  S2rI32Ror = 426,
  /// k_s2r_I64Rol
  S2rI64Rol = 427,
  /// k_s2r_I64Ror
  S2rI64Ror = 428,
  /// k_s2r_F32Min
  S2rF32Min = 429,
  /// k_s2r_F32Max
  S2rF32Max = 430,
  /// k_s2r_F64Min
  S2rF64Min = 431,
  /// k_s2r_F64Max
  S2rF64Max = 432,
  /// k_s2r_F32CopySign
  S2rF32CopySign = 433,
  /// k_s2r_F64CopySign
  S2rF64CopySign = 434,
  /// k_s2s_I32Shl
  S2sI32Shl = 435,
  /// k_s2s_I32ShrU
  S2sI32ShrU = 436,
  /// k_s2s_I32ShrS
  S2sI32ShrS = 437,
  /// k_s2s_I64Shl
  S2sI64Shl = 438,
  /// k_s2s_I64ShrU
  S2sI64ShrU = 439,
  /// k_s2s_I64ShrS
  S2sI64ShrS = 440,
  /// k_s2s_I32Rol
  S2sI32Rol = 441,
  /// k_s2s_I32Ror
  S2sI32Ror = 442,
  /// k_s2s_I64Rol
  S2sI64Rol = 443,
  /// k_s2s_I64Ror
  S2sI64Ror = 444,
  /// k_s2s_F32Min
  S2sF32Min = 445,
  /// k_s2s_F32Max
  S2sF32Max = 446,
  /// k_s2s_F64Min
  S2sF64Min = 447,
  /// k_s2s_F64Max
  S2sF64Max = 448,
  /// k_s2s_F32CopySign
  S2sF32CopySign = 449,
  /// k_s2s_F64CopySign
  S2sF64CopySign = 450,
  /// k_r2r_F32Abs
  R2rF32Abs = 451,
  /// k_r2r_F32Neg
  R2rF32Neg = 452,
  /// k_r2r_F32Ceil
  R2rF32Ceil = 453,
  /// k_r2r_F32Floor
  R2rF32Floor = 454,
  /// k_r2r_F32Trunc
  R2rF32Trunc = 455,
  /// k_r2r_F32NearestInt
  R2rF32NearestInt = 456,
  /// k_r2r_F32Sqrt
  R2rF32Sqrt = 457,
  /// k_r2r_F64Abs
  R2rF64Abs = 458,
  /// k_r2r_F64Neg
  R2rF64Neg = 459,
  /// k_r2r_F64Ceil
  R2rF64Ceil = 460,
  /// k_r2r_F64Floor
  R2rF64Floor = 461,
  /// k_r2r_F64Trunc
  R2rF64Trunc = 462,
  /// k_r2r_F64NearestInt
  R2rF64NearestInt = 463,
  /// k_r2r_F64Sqrt
  R2rF64Sqrt = 464,
  /// k_r2s_F32Abs
  R2sF32Abs = 465,
  /// k_r2s_F32Neg
  R2sF32Neg = 466,
  /// k_r2s_F32Ceil
  R2sF32Ceil = 467,
  /// k_r2s_F32Floor
  R2sF32Floor = 468,
  /// k_r2s_F32Trunc
  R2sF32Trunc = 469,
  /// k_r2s_F32NearestInt
  R2sF32NearestInt = 470,
  /// k_r2s_F32Sqrt
  R2sF32Sqrt = 471,
  /// k_r2s_F64Abs
  R2sF64Abs = 472,
  /// k_r2s_F64Neg
  R2sF64Neg = 473,
  /// k_r2s_F64Ceil
  R2sF64Ceil = 474,
  /// k_r2s_F64Floor
  R2sF64Floor = 475,
  /// k_r2s_F64Trunc
  R2sF64Trunc = 476,
  /// k_r2s_F64NearestInt
  R2sF64NearestInt = 477,
  /// k_r2s_F64Sqrt
  R2sF64Sqrt = 478,
  /// k_s2r_F32Abs
  S2rF32Abs = 479,
  /// k_s2r_F32Neg
  S2rF32Neg = 480,
  /// k_s2r_F32Ceil
  S2rF32Ceil = 481,
  /// k_s2r_F32Floor
  S2rF32Floor = 482,
  /// k_s2r_F32Trunc
  S2rF32Trunc = 483,
  /// k_s2r_F32NearestInt
  S2rF32NearestInt = 484,
  /// k_s2r_F32Sqrt
  S2rF32Sqrt = 485,
  /// k_s2r_F64Abs
  S2rF64Abs = 486,
  /// k_s2r_F64Neg
  S2rF64Neg = 487,
  /// k_s2r_F64Ceil
  S2rF64Ceil = 488,
  /// k_s2r_F64Floor
  S2rF64Floor = 489,
  /// k_s2r_F64Trunc
  S2rF64Trunc = 490,
  /// k_s2r_F64NearestInt
  S2rF64NearestInt = 491,
  /// k_s2r_F64Sqrt
  S2rF64Sqrt = 492,
  /// k_s2s_F32Abs
  S2sF32Abs = 493,
  /// k_s2s_F32Neg
  S2sF32Neg = 494,
  /// k_s2s_F32Ceil
  S2sF32Ceil = 495,
  /// k_s2s_F32Floor
  S2sF32Floor = 496,
  /// k_s2s_F32Trunc
  S2sF32Trunc = 497,
  /// k_s2s_F32NearestInt
  S2sF32NearestInt = 498,
  /// k_s2s_F32Sqrt
  S2sF32Sqrt = 499,
  /// k_s2s_F64Abs
  S2sF64Abs = 500,
  /// k_s2s_F64Neg
  S2sF64Neg = 501,
  /// k_s2s_F64Ceil
  S2sF64Ceil = 502,
  /// k_s2s_F64Floor
  S2sF64Floor = 503,
  /// k_s2s_F64Trunc
  S2sF64Trunc = 504,
  /// k_s2s_F64NearestInt
  S2sF64NearestInt = 505,
  /// k_s2s_F64Sqrt
  S2sF64Sqrt = 506,
  /// k_r2r_I32ConvertI64
  R2rI32ConvertI64 = 507,
  /// k_r2r_I64SConvertF32
  R2rI64SConvertF32 = 508,
  /// k_r2r_I64SConvertF64
  R2rI64SConvertF64 = 509,
  /// k_r2r_I64UConvertF32
  R2rI64UConvertF32 = 510,
  /// k_r2r_I64UConvertF64
  R2rI64UConvertF64 = 511,
  /// k_r2r_I32SConvertF32
  R2rI32SConvertF32 = 512,
  /// k_r2r_I32UConvertF32
  R2rI32UConvertF32 = 513,
  /// k_r2r_I32SConvertF64
  R2rI32SConvertF64 = 514,
  /// k_r2r_I32UConvertF64
  R2rI32UConvertF64 = 515,
  /// k_r2r_I64SConvertI32
  R2rI64SConvertI32 = 516,
  /// k_r2r_I64UConvertI32
  R2rI64UConvertI32 = 517,
  /// k_r2r_F32SConvertI32
  R2rF32SConvertI32 = 518,
  /// k_r2r_F32UConvertI32
  R2rF32UConvertI32 = 519,
  /// k_r2r_F32SConvertI64
  R2rF32SConvertI64 = 520,
  /// k_r2r_F32UConvertI64
  R2rF32UConvertI64 = 521,
  /// k_r2r_F32ConvertF64
  R2rF32ConvertF64 = 522,
  /// k_r2r_F64SConvertI32
  R2rF64SConvertI32 = 523,
  /// k_r2r_F64UConvertI32
  R2rF64UConvertI32 = 524,
  /// k_r2r_F64SConvertI64
  R2rF64SConvertI64 = 525,
  /// k_r2r_F64UConvertI64
  R2rF64UConvertI64 = 526,
  /// k_r2r_F64ConvertF32
  R2rF64ConvertF32 = 527,
  /// k_r2s_I32ConvertI64
  R2sI32ConvertI64 = 528,
  /// k_r2s_I64SConvertF32
  R2sI64SConvertF32 = 529,
  /// k_r2s_I64SConvertF64
  R2sI64SConvertF64 = 530,
  /// k_r2s_I64UConvertF32
  R2sI64UConvertF32 = 531,
  /// k_r2s_I64UConvertF64
  R2sI64UConvertF64 = 532,
  /// k_r2s_I32SConvertF32
  R2sI32SConvertF32 = 533,
  /// k_r2s_I32UConvertF32
  R2sI32UConvertF32 = 534,
  /// k_r2s_I32SConvertF64
  R2sI32SConvertF64 = 535,
  /// k_r2s_I32UConvertF64
  R2sI32UConvertF64 = 536,
  /// k_r2s_I64SConvertI32
  R2sI64SConvertI32 = 537,
  /// k_r2s_I64UConvertI32
  R2sI64UConvertI32 = 538,
  /// k_r2s_F32SConvertI32
  R2sF32SConvertI32 = 539,
  /// k_r2s_F32UConvertI32
  R2sF32UConvertI32 = 540,
  /// k_r2s_F32SConvertI64
  R2sF32SConvertI64 = 541,
  /// k_r2s_F32UConvertI64
  R2sF32UConvertI64 = 542,
  /// k_r2s_F32ConvertF64
  R2sF32ConvertF64 = 543,
  /// k_r2s_F64SConvertI32
  R2sF64SConvertI32 = 544,
  /// k_r2s_F64UConvertI32
  R2sF64UConvertI32 = 545,
  /// k_r2s_F64SConvertI64
  R2sF64SConvertI64 = 546,
  /// k_r2s_F64UConvertI64
  R2sF64UConvertI64 = 547,
  /// k_r2s_F64ConvertF32
  R2sF64ConvertF32 = 548,
  /// k_s2r_I32ConvertI64
  S2rI32ConvertI64 = 549,
  /// k_s2r_I64SConvertF32
  S2rI64SConvertF32 = 550,
  /// k_s2r_I64SConvertF64
  S2rI64SConvertF64 = 551,
  /// k_s2r_I64UConvertF32
  S2rI64UConvertF32 = 552,
  /// k_s2r_I64UConvertF64
  S2rI64UConvertF64 = 553,
  /// k_s2r_I32SConvertF32
  S2rI32SConvertF32 = 554,
  /// k_s2r_I32UConvertF32
  S2rI32UConvertF32 = 555,
  /// k_s2r_I32SConvertF64
  S2rI32SConvertF64 = 556,
  /// k_s2r_I32UConvertF64
  S2rI32UConvertF64 = 557,
  /// k_s2r_I64SConvertI32
  S2rI64SConvertI32 = 558,
  /// k_s2r_I64UConvertI32
  S2rI64UConvertI32 = 559,
  /// k_s2r_F32SConvertI32
  S2rF32SConvertI32 = 560,
  /// k_s2r_F32UConvertI32
  S2rF32UConvertI32 = 561,
  /// k_s2r_F32SConvertI64
  S2rF32SConvertI64 = 562,
  /// k_s2r_F32UConvertI64
  S2rF32UConvertI64 = 563,
  /// k_s2r_F32ConvertF64
  S2rF32ConvertF64 = 564,
  /// k_s2r_F64SConvertI32
  S2rF64SConvertI32 = 565,
  /// k_s2r_F64UConvertI32
  S2rF64UConvertI32 = 566,
  /// k_s2r_F64SConvertI64
  S2rF64SConvertI64 = 567,
  /// k_s2r_F64UConvertI64
  S2rF64UConvertI64 = 568,
  /// k_s2r_F64ConvertF32
  S2rF64ConvertF32 = 569,
  /// k_s2s_I32ConvertI64
  S2sI32ConvertI64 = 570,
  /// k_s2s_I64SConvertF32
  S2sI64SConvertF32 = 571,
  /// k_s2s_I64SConvertF64
  S2sI64SConvertF64 = 572,
  /// k_s2s_I64UConvertF32
  S2sI64UConvertF32 = 573,
  /// k_s2s_I64UConvertF64
  S2sI64UConvertF64 = 574,
  /// k_s2s_I32SConvertF32
  S2sI32SConvertF32 = 575,
  /// k_s2s_I32UConvertF32
  S2sI32UConvertF32 = 576,
  /// k_s2s_I32SConvertF64
  S2sI32SConvertF64 = 577,
  /// k_s2s_I32UConvertF64
  S2sI32UConvertF64 = 578,
  /// k_s2s_I64SConvertI32
  S2sI64SConvertI32 = 579,
  /// k_s2s_I64UConvertI32
  S2sI64UConvertI32 = 580,
  /// k_s2s_F32SConvertI32
  S2sF32SConvertI32 = 581,
  /// k_s2s_F32UConvertI32
  S2sF32UConvertI32 = 582,
  /// k_s2s_F32SConvertI64
  S2sF32SConvertI64 = 583,
  /// k_s2s_F32UConvertI64
  S2sF32UConvertI64 = 584,
  /// k_s2s_F32ConvertF64
  S2sF32ConvertF64 = 585,
  /// k_s2s_F64SConvertI32
  S2sF64SConvertI32 = 586,
  /// k_s2s_F64UConvertI32
  S2sF64UConvertI32 = 587,
  /// k_s2s_F64SConvertI64
  S2sF64SConvertI64 = 588,
  /// k_s2s_F64UConvertI64
  S2sF64UConvertI64 = 589,
  /// k_s2s_F64ConvertF32
  S2sF64ConvertF32 = 590,
  /// k_r2r_F32ReinterpretI32
  R2rF32ReinterpretI32 = 591,
  /// k_r2r_F64ReinterpretI64
  R2rF64ReinterpretI64 = 592,
  /// k_r2r_I32ReinterpretF32
  R2rI32ReinterpretF32 = 593,
  /// k_r2r_I64ReinterpretF64
  R2rI64ReinterpretF64 = 594,
  /// k_r2s_F32ReinterpretI32
  R2sF32ReinterpretI32 = 595,
  /// k_r2s_F64ReinterpretI64
  R2sF64ReinterpretI64 = 596,
  /// k_r2s_I32ReinterpretF32
  R2sI32ReinterpretF32 = 597,
  /// k_r2s_I64ReinterpretF64
  R2sI64ReinterpretF64 = 598,
  /// k_s2r_F32ReinterpretI32
  S2rF32ReinterpretI32 = 599,
  /// k_s2r_F64ReinterpretI64
  S2rF64ReinterpretI64 = 600,
  /// k_s2r_I32ReinterpretF32
  S2rI32ReinterpretF32 = 601,
  /// k_s2r_I64ReinterpretF64
  S2rI64ReinterpretF64 = 602,
  /// k_s2s_F32ReinterpretI32
  S2sF32ReinterpretI32 = 603,
  /// k_s2s_F64ReinterpretI64
  S2sF64ReinterpretI64 = 604,
  /// k_s2s_I32ReinterpretF32
  S2sI32ReinterpretF32 = 605,
  /// k_s2s_I64ReinterpretF64
  S2sI64ReinterpretF64 = 606,
  /// k_r2r_I32Clz
  R2rI32Clz = 607,
  /// k_r2r_I32Ctz
  R2rI32Ctz = 608,
  /// k_r2r_I32Popcnt
  R2rI32Popcnt = 609,
  /// k_r2r_I32Eqz
  R2rI32Eqz = 610,
  /// k_r2r_I64Clz
  R2rI64Clz = 611,
  /// k_r2r_I64Ctz
  R2rI64Ctz = 612,
  /// k_r2r_I64Popcnt
  R2rI64Popcnt = 613,
  /// k_r2r_I64Eqz
  R2rI64Eqz = 614,
  /// k_r2s_I32Clz
  R2sI32Clz = 615,
  /// k_r2s_I32Ctz
  R2sI32Ctz = 616,
  /// k_r2s_I32Popcnt
  R2sI32Popcnt = 617,
  /// k_r2s_I32Eqz
  R2sI32Eqz = 618,
  /// k_r2s_I64Clz
  R2sI64Clz = 619,
  /// k_r2s_I64Ctz
  R2sI64Ctz = 620,
  /// k_r2s_I64Popcnt
  R2sI64Popcnt = 621,
  /// k_r2s_I64Eqz
  R2sI64Eqz = 622,
  /// k_s2r_I32Clz
  S2rI32Clz = 623,
  /// k_s2r_I32Ctz
  S2rI32Ctz = 624,
  /// k_s2r_I32Popcnt
  S2rI32Popcnt = 625,
  /// k_s2r_I32Eqz
  S2rI32Eqz = 626,
  /// k_s2r_I64Clz
  S2rI64Clz = 627,
  /// k_s2r_I64Ctz
  S2rI64Ctz = 628,
  /// k_s2r_I64Popcnt
  S2rI64Popcnt = 629,
  /// k_s2r_I64Eqz
  S2rI64Eqz = 630,
  /// k_s2s_I32Clz
  S2sI32Clz = 631,
  /// k_s2s_I32Ctz
  S2sI32Ctz = 632,
  /// k_s2s_I32Popcnt
  S2sI32Popcnt = 633,
  /// k_s2s_I32Eqz
  S2sI32Eqz = 634,
  /// k_s2s_I64Clz
  S2sI64Clz = 635,
  /// k_s2s_I64Ctz
  S2sI64Ctz = 636,
  /// k_s2s_I64Popcnt
  S2sI64Popcnt = 637,
  /// k_s2s_I64Eqz
  S2sI64Eqz = 638,
  /// k_r2r_I32SExtendI8
  R2rI32SExtendI8 = 639,
  /// k_r2r_I32SExtendI16
  R2rI32SExtendI16 = 640,
  /// k_r2r_I64SExtendI8
  R2rI64SExtendI8 = 641,
  /// k_r2r_I64SExtendI16
  R2rI64SExtendI16 = 642,
  /// k_r2r_I64SExtendI32
  R2rI64SExtendI32 = 643,
  /// k_r2s_I32SExtendI8
  R2sI32SExtendI8 = 644,
  /// k_r2s_I32SExtendI16
  R2sI32SExtendI16 = 645,
  /// k_r2s_I64SExtendI8
  R2sI64SExtendI8 = 646,
  /// k_r2s_I64SExtendI16
  R2sI64SExtendI16 = 647,
  /// k_r2s_I64SExtendI32
  R2sI64SExtendI32 = 648,
  /// k_s2r_I32SExtendI8
  S2rI32SExtendI8 = 649,
  /// k_s2r_I32SExtendI16
  S2rI32SExtendI16 = 650,
  /// k_s2r_I64SExtendI8
  S2rI64SExtendI8 = 651,
  /// k_s2r_I64SExtendI16
  S2rI64SExtendI16 = 652,
  /// k_s2r_I64SExtendI32
  S2rI64SExtendI32 = 653,
  /// k_s2s_I32SExtendI8
  S2sI32SExtendI8 = 654,
  /// k_s2s_I32SExtendI16
  S2sI32SExtendI16 = 655,
  /// k_s2s_I64SExtendI8
  S2sI64SExtendI8 = 656,
  /// k_s2s_I64SExtendI16
  S2sI64SExtendI16 = 657,
  /// k_s2s_I64SExtendI32
  S2sI64SExtendI32 = 658,
  /// k_r2r_I32SConvertSatF32
  R2rI32SConvertSatF32 = 659,
  /// k_r2r_I32UConvertSatF32
  R2rI32UConvertSatF32 = 660,
  /// k_r2r_I32SConvertSatF64
  R2rI32SConvertSatF64 = 661,
  /// k_r2r_I32UConvertSatF64
  R2rI32UConvertSatF64 = 662,
  /// k_r2r_I64SConvertSatF32
  R2rI64SConvertSatF32 = 663,
  /// k_r2r_I64UConvertSatF32
  R2rI64UConvertSatF32 = 664,
  /// k_r2r_I64SConvertSatF64
  R2rI64SConvertSatF64 = 665,
  /// k_r2r_I64UConvertSatF64
  R2rI64UConvertSatF64 = 666,
  /// k_r2s_I32SConvertSatF32
  R2sI32SConvertSatF32 = 667,
  /// k_r2s_I32UConvertSatF32
  R2sI32UConvertSatF32 = 668,
  /// k_r2s_I32SConvertSatF64
  R2sI32SConvertSatF64 = 669,
  /// k_r2s_I32UConvertSatF64
  R2sI32UConvertSatF64 = 670,
  /// k_r2s_I64SConvertSatF32
  R2sI64SConvertSatF32 = 671,
  /// k_r2s_I64UConvertSatF32
  R2sI64UConvertSatF32 = 672,
  /// k_r2s_I64SConvertSatF64
  R2sI64SConvertSatF64 = 673,
  /// k_r2s_I64UConvertSatF64
  R2sI64UConvertSatF64 = 674,
  /// k_s2r_I32SConvertSatF32
  S2rI32SConvertSatF32 = 675,
  /// k_s2r_I32UConvertSatF32
  S2rI32UConvertSatF32 = 676,
  /// k_s2r_I32SConvertSatF64
  S2rI32SConvertSatF64 = 677,
  /// k_s2r_I32UConvertSatF64
  S2rI32UConvertSatF64 = 678,
  /// k_s2r_I64SConvertSatF32
  S2rI64SConvertSatF32 = 679,
  /// k_s2r_I64UConvertSatF32
  S2rI64UConvertSatF32 = 680,
  /// k_s2r_I64SConvertSatF64
  S2rI64SConvertSatF64 = 681,
  /// k_s2r_I64UConvertSatF64
  S2rI64UConvertSatF64 = 682,
  /// k_s2s_I32SConvertSatF32
  S2sI32SConvertSatF32 = 683,
  /// k_s2s_I32UConvertSatF32
  S2sI32UConvertSatF32 = 684,
  /// k_s2s_I32SConvertSatF64
  S2sI32SConvertSatF64 = 685,
  /// k_s2s_I32UConvertSatF64
  S2sI32UConvertSatF64 = 686,
  /// k_s2s_I64SConvertSatF32
  S2sI64SConvertSatF32 = 687,
  /// k_s2s_I64UConvertSatF32
  S2sI64UConvertSatF32 = 688,
  /// k_s2s_I64SConvertSatF64
  S2sI64SConvertSatF64 = 689,
  /// k_s2s_I64UConvertSatF64
  S2sI64UConvertSatF64 = 690,
  /// k_s2s_MemoryGrow
  S2sMemoryGrow = 691,
  /// k_s2s_Memory64Grow
  S2sMemory64Grow = 692,
  /// k_s2s_MemorySize
  S2sMemorySize = 693,
  /// k_s2s_Memory64Size
  S2sMemory64Size = 694,
  /// k_s2s_Return
  S2sReturn = 695,
  /// k_s2s_Branch
  S2sBranch = 696,
  /// k_r2s_BranchIf
  R2sBranchIf = 697,
  /// k_s2s_BranchIf
  S2sBranchIf = 698,
  /// k_r2s_BranchIfWithParams
  R2sBranchIfWithParams = 699,
  /// k_s2s_BranchIfWithParams
  S2sBranchIfWithParams = 700,
  /// k_r2s_If
  R2sIf = 701,
  /// k_s2s_If
  S2sIf = 702,
  /// k_s2s_Else
  S2sElse = 703,
  /// k_s2s_CallFunction
  S2sCallFunction = 704,
  /// k_s2s_ReturnCall
  S2sReturnCall = 705,
  /// k_s2s_CallImportedFunction
  S2sCallImportedFunction = 706,
  /// k_s2s_ReturnCallImportedFunction
  S2sReturnCallImportedFunction = 707,
  /// k_s2s_CallIndirect
  S2sCallIndirect = 708,
  /// k_s2s_CallIndirect64
  S2sCallIndirect64 = 709,
  /// k_s2s_ReturnCallIndirect
  S2sReturnCallIndirect = 710,
  /// k_s2s_ReturnCallIndirect64
  S2sReturnCallIndirect64 = 711,
  /// k_r2s_BrTable
  R2sBrTable = 712,
  /// k_s2s_BrTable
  S2sBrTable = 713,
  /// k_s2s_CopySlotMulti
  S2sCopySlotMulti = 714,
  /// k_s2s_CopySlot_ll
  S2sCopySlotll = 715,
  /// k_s2s_CopySlot_lq
  S2sCopySlotlq = 716,
  /// k_s2s_CopySlot_ql
  S2sCopySlotql = 717,
  /// k_s2s_CopySlot_qq
  S2sCopySlotqq = 718,
  /// k_s2s_CopySlot32
  S2sCopySlot32 = 719,
  /// k_s2s_CopySlot32x2
  S2sCopySlot32x2 = 720,
  /// k_s2s_CopySlot64
  S2sCopySlot64 = 721,
  /// k_s2s_CopySlot64x2
  S2sCopySlot64x2 = 722,
  /// k_s2s_CopySlot128
  S2sCopySlot128 = 723,
  /// k_s2s_CopySlotRef
  S2sCopySlotRef = 724,
  /// k_s2s_PreserveCopySlot32
  S2sPreserveCopySlot32 = 725,
  /// k_s2s_PreserveCopySlot64
  S2sPreserveCopySlot64 = 726,
  /// k_s2s_PreserveCopySlot128
  S2sPreserveCopySlot128 = 727,
  /// k_s2s_PreserveCopySlotRef
  S2sPreserveCopySlotRef = 728,
  /// k_r2s_CopyR0ToSlot32
  R2sCopyR0ToSlot32 = 729,
  /// k_r2s_CopyR0ToSlot64
  R2sCopyR0ToSlot64 = 730,
  /// k_r2s_CopyFp0ToSlot32
  R2sCopyFp0ToSlot32 = 731,
  /// k_r2s_CopyFp0ToSlot64
  R2sCopyFp0ToSlot64 = 732,
  /// k_r2s_PreserveCopyR0ToSlot32
  R2sPreserveCopyR0ToSlot32 = 733,
  /// k_r2s_PreserveCopyR0ToSlot64
  R2sPreserveCopyR0ToSlot64 = 734,
  /// k_r2s_PreserveCopyFp0ToSlot32
  R2sPreserveCopyFp0ToSlot32 = 735,
  /// k_r2s_PreserveCopyFp0ToSlot64
  R2sPreserveCopyFp0ToSlot64 = 736,
  /// k_s2s_RefNull
  S2sRefNull = 737,
  /// k_s2s_RefIsNull
  S2sRefIsNull = 738,
  /// k_s2s_RefFunc
  S2sRefFunc = 739,
  /// k_s2s_RefEq
  S2sRefEq = 740,
  /// k_s2s_MemoryInit
  S2sMemoryInit = 741,
  /// k_s2s_Memory64Init
  S2sMemory64Init = 742,
  /// k_s2s_DataDrop
  S2sDataDrop = 743,
  /// k_s2s_MemoryCopy
  S2sMemoryCopy = 744,
  /// k_s2s_MemoryCopy_Mem64
  S2sMemoryCopyMem64 = 745,
  /// k_s2s_MemoryCopy_Idx64_Idx32
  S2sMemoryCopyIdx64Idx32 = 746,
  /// k_s2s_MemoryCopy_Idx32_Idx64
  S2sMemoryCopyIdx32Idx64 = 747,
  /// k_s2s_MemoryFill
  S2sMemoryFill = 748,
  /// k_s2s_Memory64Fill
  S2sMemory64Fill = 749,
  /// k_s2s_TableGet
  S2sTableGet = 750,
  /// k_s2s_Table64Get
  S2sTable64Get = 751,
  /// k_s2s_TableSet
  S2sTableSet = 752,
  /// k_s2s_Table64Set
  S2sTable64Set = 753,
  /// k_s2s_TableInit
  S2sTableInit = 754,
  /// k_s2s_Table64Init
  S2sTable64Init = 755,
  /// k_s2s_ElemDrop
  S2sElemDrop = 756,
  /// k_s2s_TableCopy
  S2sTableCopy = 757,
  /// k_s2s_Table64Copy_32_64_32
  S2sTable64Copy326432 = 758,
  /// k_s2s_Table64Copy_64_32_32
  S2sTable64Copy643232 = 759,
  /// k_s2s_Table64Copy_64_64_64
  S2sTable64Copy646464 = 760,
  /// k_s2s_TableGrow
  S2sTableGrow = 761,
  /// k_s2s_Table64Grow
  S2sTable64Grow = 762,
  /// k_s2s_TableSize
  S2sTableSize = 763,
  /// k_s2s_Table64Size
  S2sTable64Size = 764,
  /// k_s2s_TableFill
  S2sTableFill = 765,
  /// k_s2s_Table64Fill
  S2sTable64Fill = 766,
  /// k_s2s_Unreachable
  S2sUnreachable = 767,
  /// k_s2s_Unwind
  S2sUnwind = 768,
  /// k_s2s_OnLoopBegin
  S2sOnLoopBegin = 769,
  /// k_s2s_OnLoopBeginNoRefSlots
  S2sOnLoopBeginNoRefSlots = 770,
  /// k_s2s_Throw
  S2sThrow = 771,
  /// k_s2s_Rethrow
  S2sRethrow = 772,
  /// k_s2s_Catch
  S2sCatch = 773,
  /// k_s2s_AtomicFence
  S2sAtomicFence = 774,
  /// k_s2s_SimdF64x2Splat
  S2sSimdF64x2Splat = 775,
  /// k_s2s_SimdF32x4Splat
  S2sSimdF32x4Splat = 776,
  /// k_s2s_SimdI64x2Splat
  S2sSimdI64x2Splat = 777,
  /// k_s2s_SimdI32x4Splat
  S2sSimdI32x4Splat = 778,
  /// k_s2s_SimdI16x8Splat
  S2sSimdI16x8Splat = 779,
  /// k_s2s_SimdI8x16Splat
  S2sSimdI8x16Splat = 780,
  /// k_s2s_SimdF64x2ExtractLane
  S2sSimdF64x2ExtractLane = 781,
  /// k_s2s_SimdF32x4ExtractLane
  S2sSimdF32x4ExtractLane = 782,
  /// k_s2s_SimdI64x2ExtractLane
  S2sSimdI64x2ExtractLane = 783,
  /// k_s2s_SimdI32x4ExtractLane
  S2sSimdI32x4ExtractLane = 784,
  /// k_s2s_SimdI16x8ExtractLaneS
  S2sSimdI16x8ExtractLaneS = 785,
  /// k_s2s_SimdI16x8ExtractLaneU
  S2sSimdI16x8ExtractLaneU = 786,
  /// k_s2s_SimdI8x16ExtractLaneS
  S2sSimdI8x16ExtractLaneS = 787,
  /// k_s2s_SimdI8x16ExtractLaneU
  S2sSimdI8x16ExtractLaneU = 788,
  /// k_s2s_SimdF64x2Add
  S2sSimdF64x2Add = 789,
  /// k_s2s_SimdF64x2Sub
  S2sSimdF64x2Sub = 790,
  /// k_s2s_SimdF64x2Mul
  S2sSimdF64x2Mul = 791,
  /// k_s2s_SimdF64x2Div
  S2sSimdF64x2Div = 792,
  /// k_s2s_SimdF64x2Min
  S2sSimdF64x2Min = 793,
  /// k_s2s_SimdF64x2Max
  S2sSimdF64x2Max = 794,
  /// k_s2s_SimdF64x2Pmin
  S2sSimdF64x2Pmin = 795,
  /// k_s2s_SimdF64x2Pmax
  S2sSimdF64x2Pmax = 796,
  /// k_s2s_SimdF32x4RelaxedMin
  S2sSimdF32x4RelaxedMin = 797,
  /// k_s2s_SimdF32x4RelaxedMax
  S2sSimdF32x4RelaxedMax = 798,
  /// k_s2s_SimdF64x2RelaxedMin
  S2sSimdF64x2RelaxedMin = 799,
  /// k_s2s_SimdF64x2RelaxedMax
  S2sSimdF64x2RelaxedMax = 800,
  /// k_s2s_SimdF32x4Add
  S2sSimdF32x4Add = 801,
  /// k_s2s_SimdF32x4Sub
  S2sSimdF32x4Sub = 802,
  /// k_s2s_SimdF32x4Mul
  S2sSimdF32x4Mul = 803,
  /// k_s2s_SimdF32x4Div
  S2sSimdF32x4Div = 804,
  /// k_s2s_SimdF32x4Min
  S2sSimdF32x4Min = 805,
  /// k_s2s_SimdF32x4Max
  S2sSimdF32x4Max = 806,
  /// k_s2s_SimdF32x4Pmin
  S2sSimdF32x4Pmin = 807,
  /// k_s2s_SimdF32x4Pmax
  S2sSimdF32x4Pmax = 808,
  /// k_s2s_SimdI64x2Add
  S2sSimdI64x2Add = 809,
  /// k_s2s_SimdI64x2Sub
  S2sSimdI64x2Sub = 810,
  /// k_s2s_SimdI64x2Mul
  S2sSimdI64x2Mul = 811,
  /// k_s2s_SimdI32x4Add
  S2sSimdI32x4Add = 812,
  /// k_s2s_SimdI32x4Sub
  S2sSimdI32x4Sub = 813,
  /// k_s2s_SimdI32x4Mul
  S2sSimdI32x4Mul = 814,
  /// k_s2s_SimdI32x4MinS
  S2sSimdI32x4MinS = 815,
  /// k_s2s_SimdI32x4MinU
  S2sSimdI32x4MinU = 816,
  /// k_s2s_SimdI32x4MaxS
  S2sSimdI32x4MaxS = 817,
  /// k_s2s_SimdI32x4MaxU
  S2sSimdI32x4MaxU = 818,
  /// k_s2s_SimdS128And
  S2sSimdS128And = 819,
  /// k_s2s_SimdS128Or
  S2sSimdS128Or = 820,
  /// k_s2s_SimdS128Xor
  S2sSimdS128Xor = 821,
  /// k_s2s_SimdS128AndNot
  S2sSimdS128AndNot = 822,
  /// k_s2s_SimdI16x8Add
  S2sSimdI16x8Add = 823,
  /// k_s2s_SimdI16x8Sub
  S2sSimdI16x8Sub = 824,
  /// k_s2s_SimdI16x8Mul
  S2sSimdI16x8Mul = 825,
  /// k_s2s_SimdI16x8MinS
  S2sSimdI16x8MinS = 826,
  /// k_s2s_SimdI16x8MinU
  S2sSimdI16x8MinU = 827,
  /// k_s2s_SimdI16x8MaxS
  S2sSimdI16x8MaxS = 828,
  /// k_s2s_SimdI16x8MaxU
  S2sSimdI16x8MaxU = 829,
  /// k_s2s_SimdI16x8AddSatS
  S2sSimdI16x8AddSatS = 830,
  /// k_s2s_SimdI16x8AddSatU
  S2sSimdI16x8AddSatU = 831,
  /// k_s2s_SimdI16x8SubSatS
  S2sSimdI16x8SubSatS = 832,
  /// k_s2s_SimdI16x8SubSatU
  S2sSimdI16x8SubSatU = 833,
  /// k_s2s_SimdI16x8RoundingAverageU
  S2sSimdI16x8RoundingAverageU = 834,
  /// k_s2s_SimdI16x8Q15MulRSatS
  S2sSimdI16x8Q15MulRSatS = 835,
  /// k_s2s_SimdI16x8RelaxedQ15MulRS
  S2sSimdI16x8RelaxedQ15MulRS = 836,
  /// k_s2s_SimdI8x16Add
  S2sSimdI8x16Add = 837,
  /// k_s2s_SimdI8x16Sub
  S2sSimdI8x16Sub = 838,
  /// k_s2s_SimdI8x16MinS
  S2sSimdI8x16MinS = 839,
  /// k_s2s_SimdI8x16MinU
  S2sSimdI8x16MinU = 840,
  /// k_s2s_SimdI8x16MaxS
  S2sSimdI8x16MaxS = 841,
  /// k_s2s_SimdI8x16MaxU
  S2sSimdI8x16MaxU = 842,
  /// k_s2s_SimdI8x16AddSatS
  S2sSimdI8x16AddSatS = 843,
  /// k_s2s_SimdI8x16AddSatU
  S2sSimdI8x16AddSatU = 844,
  /// k_s2s_SimdI8x16SubSatS
  S2sSimdI8x16SubSatS = 845,
  /// k_s2s_SimdI8x16SubSatU
  S2sSimdI8x16SubSatU = 846,
  /// k_s2s_SimdI8x16RoundingAverageU
  S2sSimdI8x16RoundingAverageU = 847,
  /// k_s2s_SimdF64x2Abs
  S2sSimdF64x2Abs = 848,
  /// k_s2s_SimdF64x2Neg
  S2sSimdF64x2Neg = 849,
  /// k_s2s_SimdF64x2Sqrt
  S2sSimdF64x2Sqrt = 850,
  /// k_s2s_SimdF64x2Ceil
  S2sSimdF64x2Ceil = 851,
  /// k_s2s_SimdF64x2Floor
  S2sSimdF64x2Floor = 852,
  /// k_s2s_SimdF64x2Trunc
  S2sSimdF64x2Trunc = 853,
  /// k_s2s_SimdF64x2NearestInt
  S2sSimdF64x2NearestInt = 854,
  /// k_s2s_SimdF32x4Abs
  S2sSimdF32x4Abs = 855,
  /// k_s2s_SimdF32x4Neg
  S2sSimdF32x4Neg = 856,
  /// k_s2s_SimdF32x4Sqrt
  S2sSimdF32x4Sqrt = 857,
  /// k_s2s_SimdF32x4Ceil
  S2sSimdF32x4Ceil = 858,
  /// k_s2s_SimdF32x4Floor
  S2sSimdF32x4Floor = 859,
  /// k_s2s_SimdF32x4Trunc
  S2sSimdF32x4Trunc = 860,
  /// k_s2s_SimdF32x4NearestInt
  S2sSimdF32x4NearestInt = 861,
  /// k_s2s_SimdI64x2Neg
  S2sSimdI64x2Neg = 862,
  /// k_s2s_SimdI32x4Neg
  S2sSimdI32x4Neg = 863,
  /// k_s2s_SimdI64x2Abs
  S2sSimdI64x2Abs = 864,
  /// k_s2s_SimdI32x4Abs
  S2sSimdI32x4Abs = 865,
  /// k_s2s_SimdS128Not
  S2sSimdS128Not = 866,
  /// k_s2s_SimdI16x8Neg
  S2sSimdI16x8Neg = 867,
  /// k_s2s_SimdI16x8Abs
  S2sSimdI16x8Abs = 868,
  /// k_s2s_SimdI8x16Neg
  S2sSimdI8x16Neg = 869,
  /// k_s2s_SimdI8x16Abs
  S2sSimdI8x16Abs = 870,
  /// k_s2s_SimdI8x16Popcnt
  S2sSimdI8x16Popcnt = 871,
  /// k_s2s_SimdI8x16BitMask
  S2sSimdI8x16BitMask = 872,
  /// k_s2s_SimdI16x8BitMask
  S2sSimdI16x8BitMask = 873,
  /// k_s2s_SimdI32x4BitMask
  S2sSimdI32x4BitMask = 874,
  /// k_s2s_SimdI64x2BitMask
  S2sSimdI64x2BitMask = 875,
  /// k_s2s_SimdF64x2Eq
  S2sSimdF64x2Eq = 876,
  /// k_s2s_SimdF64x2Ne
  S2sSimdF64x2Ne = 877,
  /// k_s2s_SimdF64x2Gt
  S2sSimdF64x2Gt = 878,
  /// k_s2s_SimdF64x2Ge
  S2sSimdF64x2Ge = 879,
  /// k_s2s_SimdF64x2Lt
  S2sSimdF64x2Lt = 880,
  /// k_s2s_SimdF64x2Le
  S2sSimdF64x2Le = 881,
  /// k_s2s_SimdF32x4Eq
  S2sSimdF32x4Eq = 882,
  /// k_s2s_SimdF32x4Ne
  S2sSimdF32x4Ne = 883,
  /// k_s2s_SimdF32x4Gt
  S2sSimdF32x4Gt = 884,
  /// k_s2s_SimdF32x4Ge
  S2sSimdF32x4Ge = 885,
  /// k_s2s_SimdF32x4Lt
  S2sSimdF32x4Lt = 886,
  /// k_s2s_SimdF32x4Le
  S2sSimdF32x4Le = 887,
  /// k_s2s_SimdI64x2Eq
  S2sSimdI64x2Eq = 888,
  /// k_s2s_SimdI64x2Ne
  S2sSimdI64x2Ne = 889,
  /// k_s2s_SimdI64x2LtS
  S2sSimdI64x2LtS = 890,
  /// k_s2s_SimdI64x2GtS
  S2sSimdI64x2GtS = 891,
  /// k_s2s_SimdI64x2LeS
  S2sSimdI64x2LeS = 892,
  /// k_s2s_SimdI64x2GeS
  S2sSimdI64x2GeS = 893,
  /// k_s2s_SimdI32x4Eq
  S2sSimdI32x4Eq = 894,
  /// k_s2s_SimdI32x4Ne
  S2sSimdI32x4Ne = 895,
  /// k_s2s_SimdI32x4GtS
  S2sSimdI32x4GtS = 896,
  /// k_s2s_SimdI32x4GeS
  S2sSimdI32x4GeS = 897,
  /// k_s2s_SimdI32x4LtS
  S2sSimdI32x4LtS = 898,
  /// k_s2s_SimdI32x4LeS
  S2sSimdI32x4LeS = 899,
  /// k_s2s_SimdI32x4GtU
  S2sSimdI32x4GtU = 900,
  /// k_s2s_SimdI32x4GeU
  S2sSimdI32x4GeU = 901,
  /// k_s2s_SimdI32x4LtU
  S2sSimdI32x4LtU = 902,
  /// k_s2s_SimdI32x4LeU
  S2sSimdI32x4LeU = 903,
  /// k_s2s_SimdI16x8Eq
  S2sSimdI16x8Eq = 904,
  /// k_s2s_SimdI16x8Ne
  S2sSimdI16x8Ne = 905,
  /// k_s2s_SimdI16x8GtS
  S2sSimdI16x8GtS = 906,
  /// k_s2s_SimdI16x8GeS
  S2sSimdI16x8GeS = 907,
  /// k_s2s_SimdI16x8LtS
  S2sSimdI16x8LtS = 908,
  /// k_s2s_SimdI16x8LeS
  S2sSimdI16x8LeS = 909,
  /// k_s2s_SimdI16x8GtU
  S2sSimdI16x8GtU = 910,
  /// k_s2s_SimdI16x8GeU
  S2sSimdI16x8GeU = 911,
  /// k_s2s_SimdI16x8LtU
  S2sSimdI16x8LtU = 912,
  /// k_s2s_SimdI16x8LeU
  S2sSimdI16x8LeU = 913,
  /// k_s2s_SimdI8x16Eq
  S2sSimdI8x16Eq = 914,
  /// k_s2s_SimdI8x16Ne
  S2sSimdI8x16Ne = 915,
  /// k_s2s_SimdI8x16GtS
  S2sSimdI8x16GtS = 916,
  /// k_s2s_SimdI8x16GeS
  S2sSimdI8x16GeS = 917,
  /// k_s2s_SimdI8x16LtS
  S2sSimdI8x16LtS = 918,
  /// k_s2s_SimdI8x16LeS
  S2sSimdI8x16LeS = 919,
  /// k_s2s_SimdI8x16GtU
  S2sSimdI8x16GtU = 920,
  /// k_s2s_SimdI8x16GeU
  S2sSimdI8x16GeU = 921,
  /// k_s2s_SimdI8x16LtU
  S2sSimdI8x16LtU = 922,
  /// k_s2s_SimdI8x16LeU
  S2sSimdI8x16LeU = 923,
  /// k_s2s_SimdF64x2ReplaceLane
  S2sSimdF64x2ReplaceLane = 924,
  /// k_s2s_SimdF32x4ReplaceLane
  S2sSimdF32x4ReplaceLane = 925,
  /// k_s2s_SimdI64x2ReplaceLane
  S2sSimdI64x2ReplaceLane = 926,
  /// k_s2s_SimdI32x4ReplaceLane
  S2sSimdI32x4ReplaceLane = 927,
  /// k_s2s_SimdI16x8ReplaceLane
  S2sSimdI16x8ReplaceLane = 928,
  /// k_s2s_SimdI8x16ReplaceLane
  S2sSimdI8x16ReplaceLane = 929,
  /// k_s2s_SimdS128LoadMem
  S2sSimdS128LoadMem = 930,
  /// k_s2s_SimdS128LoadMem_Mem64
  S2sSimdS128LoadMemMem64 = 931,
  /// k_s2s_SimdS128LoadMem_MultiMem
  S2sSimdS128LoadMemMultiMem = 932,
  /// k_s2s_SimdS128LoadMem_MultiMem64
  S2sSimdS128LoadMemMultiMem64 = 933,
  /// k_s2s_SimdS128StoreMem
  S2sSimdS128StoreMem = 934,
  /// k_s2s_SimdS128StoreMem_Mem64
  S2sSimdS128StoreMemMem64 = 935,
  /// k_s2s_SimdS128StoreMem_MultiMem
  S2sSimdS128StoreMemMultiMem = 936,
  /// k_s2s_SimdS128StoreMem_MultiMem64
  S2sSimdS128StoreMemMultiMem64 = 937,
  /// k_s2s_SimdI64x2Shl
  S2sSimdI64x2Shl = 938,
  /// k_s2s_SimdI64x2ShrS
  S2sSimdI64x2ShrS = 939,
  /// k_s2s_SimdI64x2ShrU
  S2sSimdI64x2ShrU = 940,
  /// k_s2s_SimdI32x4Shl
  S2sSimdI32x4Shl = 941,
  /// k_s2s_SimdI32x4ShrS
  S2sSimdI32x4ShrS = 942,
  /// k_s2s_SimdI32x4ShrU
  S2sSimdI32x4ShrU = 943,
  /// k_s2s_SimdI16x8Shl
  S2sSimdI16x8Shl = 944,
  /// k_s2s_SimdI16x8ShrS
  S2sSimdI16x8ShrS = 945,
  /// k_s2s_SimdI16x8ShrU
  S2sSimdI16x8ShrU = 946,
  /// k_s2s_SimdI8x16Shl
  S2sSimdI8x16Shl = 947,
  /// k_s2s_SimdI8x16ShrS
  S2sSimdI8x16ShrS = 948,
  /// k_s2s_SimdI8x16ShrU
  S2sSimdI8x16ShrU = 949,
  /// k_s2s_SimdI16x8ExtMulLowI8x16S
  S2sSimdI16x8ExtMulLowI8x16S = 950,
  /// k_s2s_SimdI16x8ExtMulHighI8x16S
  S2sSimdI16x8ExtMulHighI8x16S = 951,
  /// k_s2s_SimdI16x8ExtMulLowI8x16U
  S2sSimdI16x8ExtMulLowI8x16U = 952,
  /// k_s2s_SimdI16x8ExtMulHighI8x16U
  S2sSimdI16x8ExtMulHighI8x16U = 953,
  /// k_s2s_SimdI32x4ExtMulLowI16x8S
  S2sSimdI32x4ExtMulLowI16x8S = 954,
  /// k_s2s_SimdI32x4ExtMulHighI16x8S
  S2sSimdI32x4ExtMulHighI16x8S = 955,
  /// k_s2s_SimdI32x4ExtMulLowI16x8U
  S2sSimdI32x4ExtMulLowI16x8U = 956,
  /// k_s2s_SimdI32x4ExtMulHighI16x8U
  S2sSimdI32x4ExtMulHighI16x8U = 957,
  /// k_s2s_SimdI64x2ExtMulLowI32x4S
  S2sSimdI64x2ExtMulLowI32x4S = 958,
  /// k_s2s_SimdI64x2ExtMulHighI32x4S
  S2sSimdI64x2ExtMulHighI32x4S = 959,
  /// k_s2s_SimdI64x2ExtMulLowI32x4U
  S2sSimdI64x2ExtMulLowI32x4U = 960,
  /// k_s2s_SimdI64x2ExtMulHighI32x4U
  S2sSimdI64x2ExtMulHighI32x4U = 961,
  /// k_s2s_SimdF32x4SConvertI32x4
  S2sSimdF32x4SConvertI32x4 = 962,
  /// k_s2s_SimdF32x4UConvertI32x4
  S2sSimdF32x4UConvertI32x4 = 963,
  /// k_s2s_SimdI32x4SConvertF32x4
  S2sSimdI32x4SConvertF32x4 = 964,
  /// k_s2s_SimdI32x4UConvertF32x4
  S2sSimdI32x4UConvertF32x4 = 965,
  /// k_s2s_SimdI32x4RelaxedTruncF32x4S
  S2sSimdI32x4RelaxedTruncF32x4S = 966,
  /// k_s2s_SimdI32x4RelaxedTruncF32x4U
  S2sSimdI32x4RelaxedTruncF32x4U = 967,
  /// k_s2s_SimdI64x2SConvertI32x4Low
  S2sSimdI64x2SConvertI32x4Low = 968,
  /// k_s2s_SimdI64x2SConvertI32x4High
  S2sSimdI64x2SConvertI32x4High = 969,
  /// k_s2s_SimdI64x2UConvertI32x4Low
  S2sSimdI64x2UConvertI32x4Low = 970,
  /// k_s2s_SimdI64x2UConvertI32x4High
  S2sSimdI64x2UConvertI32x4High = 971,
  /// k_s2s_SimdI32x4SConvertI16x8High
  S2sSimdI32x4SConvertI16x8High = 972,
  /// k_s2s_SimdI32x4UConvertI16x8High
  S2sSimdI32x4UConvertI16x8High = 973,
  /// k_s2s_SimdI32x4SConvertI16x8Low
  S2sSimdI32x4SConvertI16x8Low = 974,
  /// k_s2s_SimdI32x4UConvertI16x8Low
  S2sSimdI32x4UConvertI16x8Low = 975,
  /// k_s2s_SimdI16x8SConvertI8x16High
  S2sSimdI16x8SConvertI8x16High = 976,
  /// k_s2s_SimdI16x8UConvertI8x16High
  S2sSimdI16x8UConvertI8x16High = 977,
  /// k_s2s_SimdI16x8SConvertI8x16Low
  S2sSimdI16x8SConvertI8x16Low = 978,
  /// k_s2s_SimdI16x8UConvertI8x16Low
  S2sSimdI16x8UConvertI8x16Low = 979,
  /// k_s2s_SimdF64x2ConvertLowI32x4S
  S2sSimdF64x2ConvertLowI32x4S = 980,
  /// k_s2s_SimdF64x2ConvertLowI32x4U
  S2sSimdF64x2ConvertLowI32x4U = 981,
  /// k_s2s_SimdI32x4TruncSatF64x2SZero
  S2sSimdI32x4TruncSatF64x2SZero = 982,
  /// k_s2s_SimdI32x4TruncSatF64x2UZero
  S2sSimdI32x4TruncSatF64x2UZero = 983,
  /// k_s2s_SimdI32x4RelaxedTruncF64x2SZero
  S2sSimdI32x4RelaxedTruncF64x2SZero = 984,
  /// k_s2s_SimdI32x4RelaxedTruncF64x2UZero
  S2sSimdI32x4RelaxedTruncF64x2UZero = 985,
  /// k_s2s_SimdF32x4DemoteF64x2Zero
  S2sSimdF32x4DemoteF64x2Zero = 986,
  /// k_s2s_SimdF64x2PromoteLowF32x4
  S2sSimdF64x2PromoteLowF32x4 = 987,
  /// k_s2s_SimdI16x8SConvertI32x4
  S2sSimdI16x8SConvertI32x4 = 988,
  /// k_s2s_SimdI16x8UConvertI32x4
  S2sSimdI16x8UConvertI32x4 = 989,
  /// k_s2s_SimdI8x16SConvertI16x8
  S2sSimdI8x16SConvertI16x8 = 990,
  /// k_s2s_SimdI8x16UConvertI16x8
  S2sSimdI8x16UConvertI16x8 = 991,
  /// k_s2s_SimdI8x16RelaxedLaneSelect
  S2sSimdI8x16RelaxedLaneSelect = 992,
  /// k_s2s_SimdI16x8RelaxedLaneSelect
  S2sSimdI16x8RelaxedLaneSelect = 993,
  /// k_s2s_SimdI32x4RelaxedLaneSelect
  S2sSimdI32x4RelaxedLaneSelect = 994,
  /// k_s2s_SimdI64x2RelaxedLaneSelect
  S2sSimdI64x2RelaxedLaneSelect = 995,
  /// k_s2s_SimdS128Select
  S2sSimdS128Select = 996,
  /// k_s2s_SimdI32x4DotI16x8S
  S2sSimdI32x4DotI16x8S = 997,
  /// k_s2s_SimdI16x8DotI8x16I7x16S
  S2sSimdI16x8DotI8x16I7x16S = 998,
  /// k_s2s_SimdI32x4DotI8x16I7x16AddS
  S2sSimdI32x4DotI8x16I7x16AddS = 999,
  /// k_s2s_SimdI8x16RelaxedSwizzle
  S2sSimdI8x16RelaxedSwizzle = 1000,
  /// k_s2s_SimdI8x16Swizzle
  S2sSimdI8x16Swizzle = 1001,
  /// k_s2s_SimdV128AnyTrue
  S2sSimdV128AnyTrue = 1002,
  /// k_s2s_SimdI8x16Shuffle
  S2sSimdI8x16Shuffle = 1003,
  /// k_s2s_SimdI64x2AllTrue
  S2sSimdI64x2AllTrue = 1004,
  /// k_s2s_SimdI32x4AllTrue
  S2sSimdI32x4AllTrue = 1005,
  /// k_s2s_SimdI16x8AllTrue
  S2sSimdI16x8AllTrue = 1006,
  /// k_s2s_SimdI8x16AllTrue
  S2sSimdI8x16AllTrue = 1007,
  /// k_s2s_SimdF32x4Qfma
  S2sSimdF32x4Qfma = 1008,
  /// k_s2s_SimdF32x4Qfms
  S2sSimdF32x4Qfms = 1009,
  /// k_s2s_SimdF64x2Qfma
  S2sSimdF64x2Qfma = 1010,
  /// k_s2s_SimdF64x2Qfms
  S2sSimdF64x2Qfms = 1011,
  /// k_s2s_SimdS128Load8Splat
  S2sSimdS128Load8Splat = 1012,
  /// k_s2s_SimdS128Load8Splat_Mem64
  S2sSimdS128Load8SplatMem64 = 1013,
  /// k_s2s_SimdS128Load8Splat_MultiMem
  S2sSimdS128Load8SplatMultiMem = 1014,
  /// k_s2s_SimdS128Load8Splat_MultiMem64
  S2sSimdS128Load8SplatMultiMem64 = 1015,
  /// k_s2s_SimdS128Load16Splat
  S2sSimdS128Load16Splat = 1016,
  /// k_s2s_SimdS128Load16Splat_Mem64
  S2sSimdS128Load16SplatMem64 = 1017,
  /// k_s2s_SimdS128Load16Splat_MultiMem
  S2sSimdS128Load16SplatMultiMem = 1018,
  /// k_s2s_SimdS128Load16Splat_MultiMem64
  S2sSimdS128Load16SplatMultiMem64 = 1019,
  /// k_s2s_SimdS128Load32Splat
  S2sSimdS128Load32Splat = 1020,
  /// k_s2s_SimdS128Load32Splat_Mem64
  S2sSimdS128Load32SplatMem64 = 1021,
  /// k_s2s_SimdS128Load32Splat_MultiMem
  S2sSimdS128Load32SplatMultiMem = 1022,
  /// k_s2s_SimdS128Load32Splat_MultiMem64
  S2sSimdS128Load32SplatMultiMem64 = 1023,
  /// k_s2s_SimdS128Load64Splat
  S2sSimdS128Load64Splat = 1024,
  /// k_s2s_SimdS128Load64Splat_Mem64
  S2sSimdS128Load64SplatMem64 = 1025,
  /// k_s2s_SimdS128Load64Splat_MultiMem
  S2sSimdS128Load64SplatMultiMem = 1026,
  /// k_s2s_SimdS128Load64Splat_MultiMem64
  S2sSimdS128Load64SplatMultiMem64 = 1027,
  /// k_s2s_SimdS128Load8x8S
  S2sSimdS128Load8x8S = 1028,
  /// k_s2s_SimdS128Load8x8S_Mem64
  S2sSimdS128Load8x8SMem64 = 1029,
  /// k_s2s_SimdS128Load8x8S_MultiMem
  S2sSimdS128Load8x8SMultiMem = 1030,
  /// k_s2s_SimdS128Load8x8S_MultiMem64
  S2sSimdS128Load8x8SMultiMem64 = 1031,
  /// k_s2s_SimdS128Load8x8U
  S2sSimdS128Load8x8U = 1032,
  /// k_s2s_SimdS128Load8x8U_Mem64
  S2sSimdS128Load8x8UMem64 = 1033,
  /// k_s2s_SimdS128Load8x8U_MultiMem
  S2sSimdS128Load8x8UMultiMem = 1034,
  /// k_s2s_SimdS128Load8x8U_MultiMem64
  S2sSimdS128Load8x8UMultiMem64 = 1035,
  /// k_s2s_SimdS128Load16x4S
  S2sSimdS128Load16x4S = 1036,
  /// k_s2s_SimdS128Load16x4S_Mem64
  S2sSimdS128Load16x4SMem64 = 1037,
  /// k_s2s_SimdS128Load16x4S_MultiMem
  S2sSimdS128Load16x4SMultiMem = 1038,
  /// k_s2s_SimdS128Load16x4S_MultiMem64
  S2sSimdS128Load16x4SMultiMem64 = 1039,
  /// k_s2s_SimdS128Load16x4U
  S2sSimdS128Load16x4U = 1040,
  /// k_s2s_SimdS128Load16x4U_Mem64
  S2sSimdS128Load16x4UMem64 = 1041,
  /// k_s2s_SimdS128Load16x4U_MultiMem
  S2sSimdS128Load16x4UMultiMem = 1042,
  /// k_s2s_SimdS128Load16x4U_MultiMem64
  S2sSimdS128Load16x4UMultiMem64 = 1043,
  /// k_s2s_SimdS128Load32x2S
  S2sSimdS128Load32x2S = 1044,
  /// k_s2s_SimdS128Load32x2S_Mem64
  S2sSimdS128Load32x2SMem64 = 1045,
  /// k_s2s_SimdS128Load32x2S_MultiMem
  S2sSimdS128Load32x2SMultiMem = 1046,
  /// k_s2s_SimdS128Load32x2S_MultiMem64
  S2sSimdS128Load32x2SMultiMem64 = 1047,
  /// k_s2s_SimdS128Load32x2U
  S2sSimdS128Load32x2U = 1048,
  /// k_s2s_SimdS128Load32x2U_Mem64
  S2sSimdS128Load32x2UMem64 = 1049,
  /// k_s2s_SimdS128Load32x2U_MultiMem
  S2sSimdS128Load32x2UMultiMem = 1050,
  /// k_s2s_SimdS128Load32x2U_MultiMem64
  S2sSimdS128Load32x2UMultiMem64 = 1051,
  /// k_s2s_SimdS128Load32Zero
  S2sSimdS128Load32Zero = 1052,
  /// k_s2s_SimdS128Load32Zero_Mem64
  S2sSimdS128Load32ZeroMem64 = 1053,
  /// k_s2s_SimdS128Load32Zero_MultiMem
  S2sSimdS128Load32ZeroMultiMem = 1054,
  /// k_s2s_SimdS128Load32Zero_MultiMem64
  S2sSimdS128Load32ZeroMultiMem64 = 1055,
  /// k_s2s_SimdS128Load64Zero
  S2sSimdS128Load64Zero = 1056,
  /// k_s2s_SimdS128Load64Zero_Mem64
  S2sSimdS128Load64ZeroMem64 = 1057,
  /// k_s2s_SimdS128Load64Zero_MultiMem
  S2sSimdS128Load64ZeroMultiMem = 1058,
  /// k_s2s_SimdS128Load64Zero_MultiMem64
  S2sSimdS128Load64ZeroMultiMem64 = 1059,
  /// k_s2s_SimdS128Load8Lane
  S2sSimdS128Load8Lane = 1060,
  /// k_s2s_SimdS128Load8Lane_Mem64
  S2sSimdS128Load8LaneMem64 = 1061,
  /// k_s2s_SimdS128Load8Lane_MultiMem
  S2sSimdS128Load8LaneMultiMem = 1062,
  /// k_s2s_SimdS128Load8Lane_MultiMem64
  S2sSimdS128Load8LaneMultiMem64 = 1063,
  /// k_s2s_SimdS128Load16Lane
  S2sSimdS128Load16Lane = 1064,
  /// k_s2s_SimdS128Load16Lane_Mem64
  S2sSimdS128Load16LaneMem64 = 1065,
  /// k_s2s_SimdS128Load16Lane_MultiMem
  S2sSimdS128Load16LaneMultiMem = 1066,
  /// k_s2s_SimdS128Load16Lane_MultiMem64
  S2sSimdS128Load16LaneMultiMem64 = 1067,
  /// k_s2s_SimdS128Load32Lane
  S2sSimdS128Load32Lane = 1068,
  /// k_s2s_SimdS128Load32Lane_Mem64
  S2sSimdS128Load32LaneMem64 = 1069,
  /// k_s2s_SimdS128Load32Lane_MultiMem
  S2sSimdS128Load32LaneMultiMem = 1070,
  /// k_s2s_SimdS128Load32Lane_MultiMem64
  S2sSimdS128Load32LaneMultiMem64 = 1071,
  /// k_s2s_SimdS128Load64Lane
  S2sSimdS128Load64Lane = 1072,
  /// k_s2s_SimdS128Load64Lane_Mem64
  S2sSimdS128Load64LaneMem64 = 1073,
  /// k_s2s_SimdS128Load64Lane_MultiMem
  S2sSimdS128Load64LaneMultiMem = 1074,
  /// k_s2s_SimdS128Load64Lane_MultiMem64
  S2sSimdS128Load64LaneMultiMem64 = 1075,
  /// k_s2s_SimdS128Store8Lane
  S2sSimdS128Store8Lane = 1076,
  /// k_s2s_SimdS128Store8Lane_Mem64
  S2sSimdS128Store8LaneMem64 = 1077,
  /// k_s2s_SimdS128Store8Lane_MultiMem
  S2sSimdS128Store8LaneMultiMem = 1078,
  /// k_s2s_SimdS128Store8Lane_MultiMem64
  S2sSimdS128Store8LaneMultiMem64 = 1079,
  /// k_s2s_SimdS128Store16Lane
  S2sSimdS128Store16Lane = 1080,
  /// k_s2s_SimdS128Store16Lane_Mem64
  S2sSimdS128Store16LaneMem64 = 1081,
  /// k_s2s_SimdS128Store16Lane_MultiMem
  S2sSimdS128Store16LaneMultiMem = 1082,
  /// k_s2s_SimdS128Store16Lane_MultiMem64
  S2sSimdS128Store16LaneMultiMem64 = 1083,
  /// k_s2s_SimdS128Store32Lane
  S2sSimdS128Store32Lane = 1084,
  /// k_s2s_SimdS128Store32Lane_Mem64
  S2sSimdS128Store32LaneMem64 = 1085,
  /// k_s2s_SimdS128Store32Lane_MultiMem
  S2sSimdS128Store32LaneMultiMem = 1086,
  /// k_s2s_SimdS128Store32Lane_MultiMem64
  S2sSimdS128Store32LaneMultiMem64 = 1087,
  /// k_s2s_SimdS128Store64Lane
  S2sSimdS128Store64Lane = 1088,
  /// k_s2s_SimdS128Store64Lane_Mem64
  S2sSimdS128Store64LaneMem64 = 1089,
  /// k_s2s_SimdS128Store64Lane_MultiMem
  S2sSimdS128Store64LaneMultiMem = 1090,
  /// k_s2s_SimdS128Store64Lane_MultiMem64
  S2sSimdS128Store64LaneMultiMem64 = 1091,
  /// k_s2s_SimdI32x4ExtAddPairwiseI16x8S
  S2sSimdI32x4ExtAddPairwiseI16x8S = 1092,
  /// k_s2s_SimdI32x4ExtAddPairwiseI16x8U
  S2sSimdI32x4ExtAddPairwiseI16x8U = 1093,
  /// k_s2s_SimdI16x8ExtAddPairwiseI8x16S
  S2sSimdI16x8ExtAddPairwiseI8x16S = 1094,
  /// k_s2s_SimdI16x8ExtAddPairwiseI8x16U
  S2sSimdI16x8ExtAddPairwiseI8x16U = 1095,
  /// k_s2s_BranchOnNull
  S2sBranchOnNull = 1096,
  /// k_s2s_BranchOnNullWithParams
  S2sBranchOnNullWithParams = 1097,
  /// k_s2s_BranchOnNonNull
  S2sBranchOnNonNull = 1098,
  /// k_s2s_BranchOnNonNullWithParams
  S2sBranchOnNonNullWithParams = 1099,
  /// k_s2s_BranchOnCast
  S2sBranchOnCast = 1100,
  /// k_s2s_BranchOnCastFail
  S2sBranchOnCastFail = 1101,
  /// k_s2s_StructNew
  S2sStructNew = 1102,
  /// k_s2s_StructNewDefault
  S2sStructNewDefault = 1103,
  /// k_s2s_I8SStructGet
  S2sI8SStructGet = 1104,
  /// k_s2s_I8UStructGet
  S2sI8UStructGet = 1105,
  /// k_s2s_I16SStructGet
  S2sI16SStructGet = 1106,
  /// k_s2s_I16UStructGet
  S2sI16UStructGet = 1107,
  /// k_s2s_I32StructGet
  S2sI32StructGet = 1108,
  /// k_s2s_I64StructGet
  S2sI64StructGet = 1109,
  /// k_s2s_F32StructGet
  S2sF32StructGet = 1110,
  /// k_s2s_F64StructGet
  S2sF64StructGet = 1111,
  /// k_s2s_S128StructGet
  S2sS128StructGet = 1112,
  /// k_s2s_RefStructGet
  S2sRefStructGet = 1113,
  /// k_s2s_I8StructSet
  S2sI8StructSet = 1114,
  /// k_s2s_I16StructSet
  S2sI16StructSet = 1115,
  /// k_s2s_I32StructSet
  S2sI32StructSet = 1116,
  /// k_s2s_I64StructSet
  S2sI64StructSet = 1117,
  /// k_s2s_F32StructSet
  S2sF32StructSet = 1118,
  /// k_s2s_F64StructSet
  S2sF64StructSet = 1119,
  /// k_s2s_S128StructSet
  S2sS128StructSet = 1120,
  /// k_s2s_RefStructSet
  S2sRefStructSet = 1121,
  /// k_s2s_I8ArrayNew
  S2sI8ArrayNew = 1122,
  /// k_s2s_I16ArrayNew
  S2sI16ArrayNew = 1123,
  /// k_s2s_I32ArrayNew
  S2sI32ArrayNew = 1124,
  /// k_s2s_I64ArrayNew
  S2sI64ArrayNew = 1125,
  /// k_s2s_F32ArrayNew
  S2sF32ArrayNew = 1126,
  /// k_s2s_F64ArrayNew
  S2sF64ArrayNew = 1127,
  /// k_s2s_S128ArrayNew
  S2sS128ArrayNew = 1128,
  /// k_s2s_RefArrayNew
  S2sRefArrayNew = 1129,
  /// k_s2s_ArrayNewDefault
  S2sArrayNewDefault = 1130,
  /// k_s2s_ArrayNewFixed
  S2sArrayNewFixed = 1131,
  /// k_s2s_ArrayNewData
  S2sArrayNewData = 1132,
  /// k_s2s_ArrayNewElem
  S2sArrayNewElem = 1133,
  /// k_s2s_ArrayInitData
  S2sArrayInitData = 1134,
  /// k_s2s_ArrayInitElem
  S2sArrayInitElem = 1135,
  /// k_s2s_ArrayLen
  S2sArrayLen = 1136,
  /// k_s2s_ArrayCopy
  S2sArrayCopy = 1137,
  /// k_s2s_I8SArrayGet
  S2sI8SArrayGet = 1138,
  /// k_s2s_I8UArrayGet
  S2sI8UArrayGet = 1139,
  /// k_s2s_I16SArrayGet
  S2sI16SArrayGet = 1140,
  /// k_s2s_I16UArrayGet
  S2sI16UArrayGet = 1141,
  /// k_s2s_I32ArrayGet
  S2sI32ArrayGet = 1142,
  /// k_s2s_I64ArrayGet
  S2sI64ArrayGet = 1143,
  /// k_s2s_F32ArrayGet
  S2sF32ArrayGet = 1144,
  /// k_s2s_F64ArrayGet
  S2sF64ArrayGet = 1145,
  /// k_s2s_S128ArrayGet
  S2sS128ArrayGet = 1146,
  /// k_s2s_RefArrayGet
  S2sRefArrayGet = 1147,
  /// k_s2s_I8ArraySet
  S2sI8ArraySet = 1148,
  /// k_s2s_I16ArraySet
  S2sI16ArraySet = 1149,
  /// k_s2s_I32ArraySet
  S2sI32ArraySet = 1150,
  /// k_s2s_I64ArraySet
  S2sI64ArraySet = 1151,
  /// k_s2s_F32ArraySet
  S2sF32ArraySet = 1152,
  /// k_s2s_F64ArraySet
  S2sF64ArraySet = 1153,
  /// k_s2s_S128ArraySet
  S2sS128ArraySet = 1154,
  /// k_s2s_RefArraySet
  S2sRefArraySet = 1155,
  /// k_s2s_I8ArrayFill
  S2sI8ArrayFill = 1156,
  /// k_s2s_I16ArrayFill
  S2sI16ArrayFill = 1157,
  /// k_s2s_I32ArrayFill
  S2sI32ArrayFill = 1158,
  /// k_s2s_I64ArrayFill
  S2sI64ArrayFill = 1159,
  /// k_s2s_F32ArrayFill
  S2sF32ArrayFill = 1160,
  /// k_s2s_F64ArrayFill
  S2sF64ArrayFill = 1161,
  /// k_s2s_S128ArrayFill
  S2sS128ArrayFill = 1162,
  /// k_s2s_RefArrayFill
  S2sRefArrayFill = 1163,
  /// k_s2s_RefI31
  S2sRefI31 = 1164,
  /// k_s2s_I31GetS
  S2sI31GetS = 1165,
  /// k_s2s_I31GetU
  S2sI31GetU = 1166,
  /// k_s2s_RefCast
  S2sRefCast = 1167,
  /// k_s2s_RefCastNull
  S2sRefCastNull = 1168,
  /// k_s2s_RefTest
  S2sRefTest = 1169,
  /// k_s2s_RefTestNull
  S2sRefTestNull = 1170,
  /// k_s2s_RefAsNonNull
  S2sRefAsNonNull = 1171,
  /// k_s2s_CallRef
  S2sCallRef = 1172,
  /// k_s2s_ReturnCallRef
  S2sReturnCallRef = 1173,
  /// k_s2s_AnyConvertExtern
  S2sAnyConvertExtern = 1174,
  /// k_s2s_ExternConvertAny
  S2sExternConvertAny = 1175,
  /// k_s2s_AssertNullTypecheck
  S2sAssertNullTypecheck = 1176,
  /// k_s2s_AssertNotNullTypecheck
  S2sAssertNotNullTypecheck = 1177,
  /// k_s2s_TrapIllegalCast
  S2sTrapIllegalCast = 1178,
  /// k_s2s_RefTestSucceeds
  S2sRefTestSucceeds = 1179,
  /// k_s2s_RefTestFails
  S2sRefTestFails = 1180,
  /// k_s2s_RefIsNonNull
  S2sRefIsNonNull = 1181,
  /// k_r2r_I32LoadMem8S_MultiMem64
  R2rI32LoadMem8SMultiMem64 = 1182,
  /// k_r2r_I32LoadMem8S_MultiMem
  R2rI32LoadMem8SMultiMem = 1183,
  /// k_r2r_I32LoadMem8S_Mem64
  R2rI32LoadMem8SMem64 = 1184,
  /// k_r2r_I32LoadMem8U_MultiMem64
  R2rI32LoadMem8UMultiMem64 = 1185,
  /// k_r2r_I32LoadMem8U_MultiMem
  R2rI32LoadMem8UMultiMem = 1186,
  /// k_r2r_I32LoadMem8U_Mem64
  R2rI32LoadMem8UMem64 = 1187,
  /// k_r2r_I32LoadMem16S_MultiMem64
  R2rI32LoadMem16SMultiMem64 = 1188,
  /// k_r2r_I32LoadMem16S_MultiMem
  R2rI32LoadMem16SMultiMem = 1189,
  /// k_r2r_I32LoadMem16S_Mem64
  R2rI32LoadMem16SMem64 = 1190,
  /// k_r2r_I32LoadMem16U_MultiMem64
  R2rI32LoadMem16UMultiMem64 = 1191,
  /// k_r2r_I32LoadMem16U_MultiMem
  R2rI32LoadMem16UMultiMem = 1192,
  /// k_r2r_I32LoadMem16U_Mem64
  R2rI32LoadMem16UMem64 = 1193,
  /// k_r2r_I64LoadMem8S_MultiMem64
  R2rI64LoadMem8SMultiMem64 = 1194,
  /// k_r2r_I64LoadMem8S_MultiMem
  R2rI64LoadMem8SMultiMem = 1195,
  /// k_r2r_I64LoadMem8S_Mem64
  R2rI64LoadMem8SMem64 = 1196,
  /// k_r2r_I64LoadMem8U_MultiMem64
  R2rI64LoadMem8UMultiMem64 = 1197,
  /// k_r2r_I64LoadMem8U_MultiMem
  R2rI64LoadMem8UMultiMem = 1198,
  /// k_r2r_I64LoadMem8U_Mem64
  R2rI64LoadMem8UMem64 = 1199,
  /// k_r2r_I64LoadMem16S_MultiMem64
  R2rI64LoadMem16SMultiMem64 = 1200,
  /// k_r2r_I64LoadMem16S_MultiMem
  R2rI64LoadMem16SMultiMem = 1201,
  /// k_r2r_I64LoadMem16S_Mem64
  R2rI64LoadMem16SMem64 = 1202,
  /// k_r2r_I64LoadMem16U_MultiMem64
  R2rI64LoadMem16UMultiMem64 = 1203,
  /// k_r2r_I64LoadMem16U_MultiMem
  R2rI64LoadMem16UMultiMem = 1204,
  /// k_r2r_I64LoadMem16U_Mem64
  R2rI64LoadMem16UMem64 = 1205,
  /// k_r2r_I64LoadMem32S_MultiMem64
  R2rI64LoadMem32SMultiMem64 = 1206,
  /// k_r2r_I64LoadMem32S_MultiMem
  R2rI64LoadMem32SMultiMem = 1207,
  /// k_r2r_I64LoadMem32S_Mem64
  R2rI64LoadMem32SMem64 = 1208,
  /// k_r2r_I64LoadMem32U_MultiMem64
  R2rI64LoadMem32UMultiMem64 = 1209,
  /// k_r2r_I64LoadMem32U_MultiMem
  R2rI64LoadMem32UMultiMem = 1210,
  /// k_r2r_I64LoadMem32U_Mem64
  R2rI64LoadMem32UMem64 = 1211,
  /// k_r2r_I32LoadMem_MultiMem64
  R2rI32LoadMemMultiMem64 = 1212,
  /// k_r2r_I32LoadMem_MultiMem
  R2rI32LoadMemMultiMem = 1213,
  /// k_r2r_I32LoadMem_Mem64
  R2rI32LoadMemMem64 = 1214,
  /// k_r2r_I64LoadMem_MultiMem64
  R2rI64LoadMemMultiMem64 = 1215,
  /// k_r2r_I64LoadMem_MultiMem
  R2rI64LoadMemMultiMem = 1216,
  /// k_r2r_I64LoadMem_Mem64
  R2rI64LoadMemMem64 = 1217,
  /// k_r2r_F32LoadMem_MultiMem64
  R2rF32LoadMemMultiMem64 = 1218,
  /// k_r2r_F32LoadMem_MultiMem
  R2rF32LoadMemMultiMem = 1219,
  /// k_r2r_F32LoadMem_Mem64
  R2rF32LoadMemMem64 = 1220,
  /// k_r2r_F64LoadMem_MultiMem64
  R2rF64LoadMemMultiMem64 = 1221,
  /// k_r2r_F64LoadMem_MultiMem
  R2rF64LoadMemMultiMem = 1222,
  /// k_r2r_F64LoadMem_Mem64
  R2rF64LoadMemMem64 = 1223,
  /// k_r2s_I32LoadMem8S_MultiMem64
  R2sI32LoadMem8SMultiMem64 = 1224,
  /// k_r2s_I32LoadMem8S_MultiMem
  R2sI32LoadMem8SMultiMem = 1225,
  /// k_r2s_I32LoadMem8S_Mem64
  R2sI32LoadMem8SMem64 = 1226,
  /// k_r2s_I32LoadMem8U_MultiMem64
  R2sI32LoadMem8UMultiMem64 = 1227,
  /// k_r2s_I32LoadMem8U_MultiMem
  R2sI32LoadMem8UMultiMem = 1228,
  /// k_r2s_I32LoadMem8U_Mem64
  R2sI32LoadMem8UMem64 = 1229,
  /// k_r2s_I32LoadMem16S_MultiMem64
  R2sI32LoadMem16SMultiMem64 = 1230,
  /// k_r2s_I32LoadMem16S_MultiMem
  R2sI32LoadMem16SMultiMem = 1231,
  /// k_r2s_I32LoadMem16S_Mem64
  R2sI32LoadMem16SMem64 = 1232,
  /// k_r2s_I32LoadMem16U_MultiMem64
  R2sI32LoadMem16UMultiMem64 = 1233,
  /// k_r2s_I32LoadMem16U_MultiMem
  R2sI32LoadMem16UMultiMem = 1234,
  /// k_r2s_I32LoadMem16U_Mem64
  R2sI32LoadMem16UMem64 = 1235,
  /// k_r2s_I64LoadMem8S_MultiMem64
  R2sI64LoadMem8SMultiMem64 = 1236,
  /// k_r2s_I64LoadMem8S_MultiMem
  R2sI64LoadMem8SMultiMem = 1237,
  /// k_r2s_I64LoadMem8S_Mem64
  R2sI64LoadMem8SMem64 = 1238,
  /// k_r2s_I64LoadMem8U_MultiMem64
  R2sI64LoadMem8UMultiMem64 = 1239,
  /// k_r2s_I64LoadMem8U_MultiMem
  R2sI64LoadMem8UMultiMem = 1240,
  /// k_r2s_I64LoadMem8U_Mem64
  R2sI64LoadMem8UMem64 = 1241,
  /// k_r2s_I64LoadMem16S_MultiMem64
  R2sI64LoadMem16SMultiMem64 = 1242,
  /// k_r2s_I64LoadMem16S_MultiMem
  R2sI64LoadMem16SMultiMem = 1243,
  /// k_r2s_I64LoadMem16S_Mem64
  R2sI64LoadMem16SMem64 = 1244,
  /// k_r2s_I64LoadMem16U_MultiMem64
  R2sI64LoadMem16UMultiMem64 = 1245,
  /// k_r2s_I64LoadMem16U_MultiMem
  R2sI64LoadMem16UMultiMem = 1246,
  /// k_r2s_I64LoadMem16U_Mem64
  R2sI64LoadMem16UMem64 = 1247,
  /// k_r2s_I64LoadMem32S_MultiMem64
  R2sI64LoadMem32SMultiMem64 = 1248,
  /// k_r2s_I64LoadMem32S_MultiMem
  R2sI64LoadMem32SMultiMem = 1249,
  /// k_r2s_I64LoadMem32S_Mem64
  R2sI64LoadMem32SMem64 = 1250,
  /// k_r2s_I64LoadMem32U_MultiMem64
  R2sI64LoadMem32UMultiMem64 = 1251,
  /// k_r2s_I64LoadMem32U_MultiMem
  R2sI64LoadMem32UMultiMem = 1252,
  /// k_r2s_I64LoadMem32U_Mem64
  R2sI64LoadMem32UMem64 = 1253,
  /// k_r2s_I32LoadMem_MultiMem64
  R2sI32LoadMemMultiMem64 = 1254,
  /// k_r2s_I32LoadMem_MultiMem
  R2sI32LoadMemMultiMem = 1255,
  /// k_r2s_I32LoadMem_Mem64
  R2sI32LoadMemMem64 = 1256,
  /// k_r2s_I64LoadMem_MultiMem64
  R2sI64LoadMemMultiMem64 = 1257,
  /// k_r2s_I64LoadMem_MultiMem
  R2sI64LoadMemMultiMem = 1258,
  /// k_r2s_I64LoadMem_Mem64
  R2sI64LoadMemMem64 = 1259,
  /// k_r2s_F32LoadMem_MultiMem64
  R2sF32LoadMemMultiMem64 = 1260,
  /// k_r2s_F32LoadMem_MultiMem
  R2sF32LoadMemMultiMem = 1261,
  /// k_r2s_F32LoadMem_Mem64
  R2sF32LoadMemMem64 = 1262,
  /// k_r2s_F64LoadMem_MultiMem64
  R2sF64LoadMemMultiMem64 = 1263,
  /// k_r2s_F64LoadMem_MultiMem
  R2sF64LoadMemMultiMem = 1264,
  /// k_r2s_F64LoadMem_Mem64
  R2sF64LoadMemMem64 = 1265,
  /// k_s2r_I32LoadMem8S_MultiMem64
  S2rI32LoadMem8SMultiMem64 = 1266,
  /// k_s2r_I32LoadMem8S_MultiMem
  S2rI32LoadMem8SMultiMem = 1267,
  /// k_s2r_I32LoadMem8S_Mem64
  S2rI32LoadMem8SMem64 = 1268,
  /// k_s2r_I32LoadMem8U_MultiMem64
  S2rI32LoadMem8UMultiMem64 = 1269,
  /// k_s2r_I32LoadMem8U_MultiMem
  S2rI32LoadMem8UMultiMem = 1270,
  /// k_s2r_I32LoadMem8U_Mem64
  S2rI32LoadMem8UMem64 = 1271,
  /// k_s2r_I32LoadMem16S_MultiMem64
  S2rI32LoadMem16SMultiMem64 = 1272,
  /// k_s2r_I32LoadMem16S_MultiMem
  S2rI32LoadMem16SMultiMem = 1273,
  /// k_s2r_I32LoadMem16S_Mem64
  S2rI32LoadMem16SMem64 = 1274,
  /// k_s2r_I32LoadMem16U_MultiMem64
  S2rI32LoadMem16UMultiMem64 = 1275,
  /// k_s2r_I32LoadMem16U_MultiMem
  S2rI32LoadMem16UMultiMem = 1276,
  /// k_s2r_I32LoadMem16U_Mem64
  S2rI32LoadMem16UMem64 = 1277,
  /// k_s2r_I64LoadMem8S_MultiMem64
  S2rI64LoadMem8SMultiMem64 = 1278,
  /// k_s2r_I64LoadMem8S_MultiMem
  S2rI64LoadMem8SMultiMem = 1279,
  /// k_s2r_I64LoadMem8S_Mem64
  S2rI64LoadMem8SMem64 = 1280,
  /// k_s2r_I64LoadMem8U_MultiMem64
  S2rI64LoadMem8UMultiMem64 = 1281,
  /// k_s2r_I64LoadMem8U_MultiMem
  S2rI64LoadMem8UMultiMem = 1282,
  /// k_s2r_I64LoadMem8U_Mem64
  S2rI64LoadMem8UMem64 = 1283,
  /// k_s2r_I64LoadMem16S_MultiMem64
  S2rI64LoadMem16SMultiMem64 = 1284,
  /// k_s2r_I64LoadMem16S_MultiMem
  S2rI64LoadMem16SMultiMem = 1285,
  /// k_s2r_I64LoadMem16S_Mem64
  S2rI64LoadMem16SMem64 = 1286,
  /// k_s2r_I64LoadMem16U_MultiMem64
  S2rI64LoadMem16UMultiMem64 = 1287,
  /// k_s2r_I64LoadMem16U_MultiMem
  S2rI64LoadMem16UMultiMem = 1288,
  /// k_s2r_I64LoadMem16U_Mem64
  S2rI64LoadMem16UMem64 = 1289,
  /// k_s2r_I64LoadMem32S_MultiMem64
  S2rI64LoadMem32SMultiMem64 = 1290,
  /// k_s2r_I64LoadMem32S_MultiMem
  S2rI64LoadMem32SMultiMem = 1291,
  /// k_s2r_I64LoadMem32S_Mem64
  S2rI64LoadMem32SMem64 = 1292,
  /// k_s2r_I64LoadMem32U_MultiMem64
  S2rI64LoadMem32UMultiMem64 = 1293,
  /// k_s2r_I64LoadMem32U_MultiMem
  S2rI64LoadMem32UMultiMem = 1294,
  /// k_s2r_I64LoadMem32U_Mem64
  S2rI64LoadMem32UMem64 = 1295,
  /// k_s2r_I32LoadMem_MultiMem64
  S2rI32LoadMemMultiMem64 = 1296,
  /// k_s2r_I32LoadMem_MultiMem
  S2rI32LoadMemMultiMem = 1297,
  /// k_s2r_I32LoadMem_Mem64
  S2rI32LoadMemMem64 = 1298,
  /// k_s2r_I64LoadMem_MultiMem64
  S2rI64LoadMemMultiMem64 = 1299,
  /// k_s2r_I64LoadMem_MultiMem
  S2rI64LoadMemMultiMem = 1300,
  /// k_s2r_I64LoadMem_Mem64
  S2rI64LoadMemMem64 = 1301,
  /// k_s2r_F32LoadMem_MultiMem64
  S2rF32LoadMemMultiMem64 = 1302,
  /// k_s2r_F32LoadMem_MultiMem
  S2rF32LoadMemMultiMem = 1303,
  /// k_s2r_F32LoadMem_Mem64
  S2rF32LoadMemMem64 = 1304,
  /// k_s2r_F64LoadMem_MultiMem64
  S2rF64LoadMemMultiMem64 = 1305,
  /// k_s2r_F64LoadMem_MultiMem
  S2rF64LoadMemMultiMem = 1306,
  /// k_s2r_F64LoadMem_Mem64
  S2rF64LoadMemMem64 = 1307,
  /// k_s2s_I32LoadMem8S_MultiMem64
  S2sI32LoadMem8SMultiMem64 = 1308,
  /// k_s2s_I32LoadMem8S_MultiMem
  S2sI32LoadMem8SMultiMem = 1309,
  /// k_s2s_I32LoadMem8S_Mem64
  S2sI32LoadMem8SMem64 = 1310,
  /// k_s2s_I32LoadMem8U_MultiMem64
  S2sI32LoadMem8UMultiMem64 = 1311,
  /// k_s2s_I32LoadMem8U_MultiMem
  S2sI32LoadMem8UMultiMem = 1312,
  /// k_s2s_I32LoadMem8U_Mem64
  S2sI32LoadMem8UMem64 = 1313,
  /// k_s2s_I32LoadMem16S_MultiMem64
  S2sI32LoadMem16SMultiMem64 = 1314,
  /// k_s2s_I32LoadMem16S_MultiMem
  S2sI32LoadMem16SMultiMem = 1315,
  /// k_s2s_I32LoadMem16S_Mem64
  S2sI32LoadMem16SMem64 = 1316,
  /// k_s2s_I32LoadMem16U_MultiMem64
  S2sI32LoadMem16UMultiMem64 = 1317,
  /// k_s2s_I32LoadMem16U_MultiMem
  S2sI32LoadMem16UMultiMem = 1318,
  /// k_s2s_I32LoadMem16U_Mem64
  S2sI32LoadMem16UMem64 = 1319,
  /// k_s2s_I64LoadMem8S_MultiMem64
  S2sI64LoadMem8SMultiMem64 = 1320,
  /// k_s2s_I64LoadMem8S_MultiMem
  S2sI64LoadMem8SMultiMem = 1321,
  /// k_s2s_I64LoadMem8S_Mem64
  S2sI64LoadMem8SMem64 = 1322,
  /// k_s2s_I64LoadMem8U_MultiMem64
  S2sI64LoadMem8UMultiMem64 = 1323,
  /// k_s2s_I64LoadMem8U_MultiMem
  S2sI64LoadMem8UMultiMem = 1324,
  /// k_s2s_I64LoadMem8U_Mem64
  S2sI64LoadMem8UMem64 = 1325,
  /// k_s2s_I64LoadMem16S_MultiMem64
  S2sI64LoadMem16SMultiMem64 = 1326,
  /// k_s2s_I64LoadMem16S_MultiMem
  S2sI64LoadMem16SMultiMem = 1327,
  /// k_s2s_I64LoadMem16S_Mem64
  S2sI64LoadMem16SMem64 = 1328,
  /// k_s2s_I64LoadMem16U_MultiMem64
  S2sI64LoadMem16UMultiMem64 = 1329,
  /// k_s2s_I64LoadMem16U_MultiMem
  S2sI64LoadMem16UMultiMem = 1330,
  /// k_s2s_I64LoadMem16U_Mem64
  S2sI64LoadMem16UMem64 = 1331,
  /// k_s2s_I64LoadMem32S_MultiMem64
  S2sI64LoadMem32SMultiMem64 = 1332,
  /// k_s2s_I64LoadMem32S_MultiMem
  S2sI64LoadMem32SMultiMem = 1333,
  /// k_s2s_I64LoadMem32S_Mem64
  S2sI64LoadMem32SMem64 = 1334,
  /// k_s2s_I64LoadMem32U_MultiMem64
  S2sI64LoadMem32UMultiMem64 = 1335,
  /// k_s2s_I64LoadMem32U_MultiMem
  S2sI64LoadMem32UMultiMem = 1336,
  /// k_s2s_I64LoadMem32U_Mem64
  S2sI64LoadMem32UMem64 = 1337,
  /// k_s2s_I32LoadMem_MultiMem64
  S2sI32LoadMemMultiMem64 = 1338,
  /// k_s2s_I32LoadMem_MultiMem
  S2sI32LoadMemMultiMem = 1339,
  /// k_s2s_I32LoadMem_Mem64
  S2sI32LoadMemMem64 = 1340,
  /// k_s2s_I64LoadMem_MultiMem64
  S2sI64LoadMemMultiMem64 = 1341,
  /// k_s2s_I64LoadMem_MultiMem
  S2sI64LoadMemMultiMem = 1342,
  /// k_s2s_I64LoadMem_Mem64
  S2sI64LoadMemMem64 = 1343,
  /// k_s2s_F32LoadMem_MultiMem64
  S2sF32LoadMemMultiMem64 = 1344,
  /// k_s2s_F32LoadMem_MultiMem
  S2sF32LoadMemMultiMem = 1345,
  /// k_s2s_F32LoadMem_Mem64
  S2sF32LoadMemMem64 = 1346,
  /// k_s2s_F64LoadMem_MultiMem64
  S2sF64LoadMemMultiMem64 = 1347,
  /// k_s2s_F64LoadMem_MultiMem
  S2sF64LoadMemMultiMem = 1348,
  /// k_s2s_F64LoadMem_Mem64
  S2sF64LoadMemMem64 = 1349,
  /// k_r2s_I32StoreMem8_MultiMem64
  R2sI32StoreMem8MultiMem64 = 1350,
  /// k_r2s_I32StoreMem8_MultiMem
  R2sI32StoreMem8MultiMem = 1351,
  /// k_r2s_I32StoreMem8_Mem64
  R2sI32StoreMem8Mem64 = 1352,
  /// k_r2s_I32StoreMem16_MultiMem64
  R2sI32StoreMem16MultiMem64 = 1353,
  /// k_r2s_I32StoreMem16_MultiMem
  R2sI32StoreMem16MultiMem = 1354,
  /// k_r2s_I32StoreMem16_Mem64
  R2sI32StoreMem16Mem64 = 1355,
  /// k_r2s_I64StoreMem8_MultiMem64
  R2sI64StoreMem8MultiMem64 = 1356,
  /// k_r2s_I64StoreMem8_MultiMem
  R2sI64StoreMem8MultiMem = 1357,
  /// k_r2s_I64StoreMem8_Mem64
  R2sI64StoreMem8Mem64 = 1358,
  /// k_r2s_I64StoreMem16_MultiMem64
  R2sI64StoreMem16MultiMem64 = 1359,
  /// k_r2s_I64StoreMem16_MultiMem
  R2sI64StoreMem16MultiMem = 1360,
  /// k_r2s_I64StoreMem16_Mem64
  R2sI64StoreMem16Mem64 = 1361,
  /// k_r2s_I64StoreMem32_MultiMem64
  R2sI64StoreMem32MultiMem64 = 1362,
  /// k_r2s_I64StoreMem32_MultiMem
  R2sI64StoreMem32MultiMem = 1363,
  /// k_r2s_I64StoreMem32_Mem64
  R2sI64StoreMem32Mem64 = 1364,
  /// k_r2s_I32StoreMem_MultiMem64
  R2sI32StoreMemMultiMem64 = 1365,
  /// k_r2s_I32StoreMem_MultiMem
  R2sI32StoreMemMultiMem = 1366,
  /// k_r2s_I32StoreMem_Mem64
  R2sI32StoreMemMem64 = 1367,
  /// k_r2s_I64StoreMem_MultiMem64
  R2sI64StoreMemMultiMem64 = 1368,
  /// k_r2s_I64StoreMem_MultiMem
  R2sI64StoreMemMultiMem = 1369,
  /// k_r2s_I64StoreMem_Mem64
  R2sI64StoreMemMem64 = 1370,
  /// k_r2s_F32StoreMem_MultiMem64
  R2sF32StoreMemMultiMem64 = 1371,
  /// k_r2s_F32StoreMem_MultiMem
  R2sF32StoreMemMultiMem = 1372,
  /// k_r2s_F32StoreMem_Mem64
  R2sF32StoreMemMem64 = 1373,
  /// k_r2s_F64StoreMem_MultiMem64
  R2sF64StoreMemMultiMem64 = 1374,
  /// k_r2s_F64StoreMem_MultiMem
  R2sF64StoreMemMultiMem = 1375,
  /// k_r2s_F64StoreMem_Mem64
  R2sF64StoreMemMem64 = 1376,
  /// k_s2s_I32StoreMem8_MultiMem64
  S2sI32StoreMem8MultiMem64 = 1377,
  /// k_s2s_I32StoreMem8_MultiMem
  S2sI32StoreMem8MultiMem = 1378,
  /// k_s2s_I32StoreMem8_Mem64
  S2sI32StoreMem8Mem64 = 1379,
  /// k_s2s_I32StoreMem16_MultiMem64
  S2sI32StoreMem16MultiMem64 = 1380,
  /// k_s2s_I32StoreMem16_MultiMem
  S2sI32StoreMem16MultiMem = 1381,
  /// k_s2s_I32StoreMem16_Mem64
  S2sI32StoreMem16Mem64 = 1382,
  /// k_s2s_I64StoreMem8_MultiMem64
  S2sI64StoreMem8MultiMem64 = 1383,
  /// k_s2s_I64StoreMem8_MultiMem
  S2sI64StoreMem8MultiMem = 1384,
  /// k_s2s_I64StoreMem8_Mem64
  S2sI64StoreMem8Mem64 = 1385,
  /// k_s2s_I64StoreMem16_MultiMem64
  S2sI64StoreMem16MultiMem64 = 1386,
  /// k_s2s_I64StoreMem16_MultiMem
  S2sI64StoreMem16MultiMem = 1387,
  /// k_s2s_I64StoreMem16_Mem64
  S2sI64StoreMem16Mem64 = 1388,
  /// k_s2s_I64StoreMem32_MultiMem64
  S2sI64StoreMem32MultiMem64 = 1389,
  /// k_s2s_I64StoreMem32_MultiMem
  S2sI64StoreMem32MultiMem = 1390,
  /// k_s2s_I64StoreMem32_Mem64
  S2sI64StoreMem32Mem64 = 1391,
  /// k_s2s_I32StoreMem_MultiMem64
  S2sI32StoreMemMultiMem64 = 1392,
  /// k_s2s_I32StoreMem_MultiMem
  S2sI32StoreMemMultiMem = 1393,
  /// k_s2s_I32StoreMem_Mem64
  S2sI32StoreMemMem64 = 1394,
  /// k_s2s_I64StoreMem_MultiMem64
  S2sI64StoreMemMultiMem64 = 1395,
  /// k_s2s_I64StoreMem_MultiMem
  S2sI64StoreMemMultiMem = 1396,
  /// k_s2s_I64StoreMem_Mem64
  S2sI64StoreMemMem64 = 1397,
  /// k_s2s_F32StoreMem_MultiMem64
  S2sF32StoreMemMultiMem64 = 1398,
  /// k_s2s_F32StoreMem_MultiMem
  S2sF32StoreMemMultiMem = 1399,
  /// k_s2s_F32StoreMem_Mem64
  S2sF32StoreMemMem64 = 1400,
  /// k_s2s_F64StoreMem_MultiMem64
  S2sF64StoreMemMultiMem64 = 1401,
  /// k_s2s_F64StoreMem_MultiMem
  S2sF64StoreMemMultiMem = 1402,
  /// k_s2s_F64StoreMem_Mem64
  S2sF64StoreMemMem64 = 1403,
  /// k_s2s_I32LoadMem8S_LocalSet_Mem64
  S2sI32LoadMem8SLocalSetMem64 = 1404,
  /// k_s2s_I32LoadMem8U_LocalSet_Mem64
  S2sI32LoadMem8ULocalSetMem64 = 1405,
  /// k_s2s_I32LoadMem16S_LocalSet_Mem64
  S2sI32LoadMem16SLocalSetMem64 = 1406,
  /// k_s2s_I32LoadMem16U_LocalSet_Mem64
  S2sI32LoadMem16ULocalSetMem64 = 1407,
  /// k_s2s_I64LoadMem8S_LocalSet_Mem64
  S2sI64LoadMem8SLocalSetMem64 = 1408,
  /// k_s2s_I64LoadMem8U_LocalSet_Mem64
  S2sI64LoadMem8ULocalSetMem64 = 1409,
  /// k_s2s_I64LoadMem16S_LocalSet_Mem64
  S2sI64LoadMem16SLocalSetMem64 = 1410,
  /// k_s2s_I64LoadMem16U_LocalSet_Mem64
  S2sI64LoadMem16ULocalSetMem64 = 1411,
  /// k_s2s_I64LoadMem32S_LocalSet_Mem64
  S2sI64LoadMem32SLocalSetMem64 = 1412,
  /// k_s2s_I64LoadMem32U_LocalSet_Mem64
  S2sI64LoadMem32ULocalSetMem64 = 1413,
  /// k_s2s_I32LoadMem_LocalSet_Mem64
  S2sI32LoadMemLocalSetMem64 = 1414,
  /// k_s2s_I64LoadMem_LocalSet_Mem64
  S2sI64LoadMemLocalSetMem64 = 1415,
  /// k_s2s_F32LoadMem_LocalSet_Mem64
  S2sF32LoadMemLocalSetMem64 = 1416,
  /// k_s2s_F64LoadMem_LocalSet_Mem64
  S2sF64LoadMemLocalSetMem64 = 1417,
  /// k_r2s_I32LoadStoreMem_Mem64
  R2sI32LoadStoreMemMem64 = 1418,
  /// k_r2s_I64LoadStoreMem_Mem64
  R2sI64LoadStoreMemMem64 = 1419,
  /// k_r2s_F32LoadStoreMem_Mem64
  R2sF32LoadStoreMemMem64 = 1420,
  /// k_r2s_F64LoadStoreMem_Mem64
  R2sF64LoadStoreMemMem64 = 1421,
  /// k_s2s_I32LoadStoreMem_Mem64
  S2sI32LoadStoreMemMem64 = 1422,
  /// k_s2s_I64LoadStoreMem_Mem64
  S2sI64LoadStoreMemMem64 = 1423,
  /// k_s2s_F32LoadStoreMem_Mem64
  S2sF32LoadStoreMemMem64 = 1424,
  /// k_s2s_F64LoadStoreMem_Mem64
  S2sF64LoadStoreMemMem64 = 1425,
  /// k_s2s_AtomicNotify_MultiMem64
  S2sAtomicNotifyMultiMem64 = 1426,
  /// k_s2s_AtomicNotify_MultiMem
  S2sAtomicNotifyMultiMem = 1427,
  /// k_s2s_AtomicNotify_Mem64
  S2sAtomicNotifyMem64 = 1428,
  /// k_s2s_I32AtomicWait_MultiMem64
  S2sI32AtomicWaitMultiMem64 = 1429,
  /// k_s2s_I32AtomicWait_MultiMem
  S2sI32AtomicWaitMultiMem = 1430,
  /// k_s2s_I32AtomicWait_Mem64
  S2sI32AtomicWaitMem64 = 1431,
  /// k_s2s_I64AtomicWait_MultiMem64
  S2sI64AtomicWaitMultiMem64 = 1432,
  /// k_s2s_I64AtomicWait_MultiMem
  S2sI64AtomicWaitMultiMem = 1433,
  /// k_s2s_I64AtomicWait_Mem64
  S2sI64AtomicWaitMem64 = 1434,
  /// k_s2s_I32AtomicAdd_MultiMem64
  S2sI32AtomicAddMultiMem64 = 1435,
  /// k_s2s_I32AtomicAdd_MultiMem
  S2sI32AtomicAddMultiMem = 1436,
  /// k_s2s_I32AtomicAdd_Mem64
  S2sI32AtomicAddMem64 = 1437,
  /// k_s2s_I32AtomicAdd8U_MultiMem64
  S2sI32AtomicAdd8UMultiMem64 = 1438,
  /// k_s2s_I32AtomicAdd8U_MultiMem
  S2sI32AtomicAdd8UMultiMem = 1439,
  /// k_s2s_I32AtomicAdd8U_Mem64
  S2sI32AtomicAdd8UMem64 = 1440,
  /// k_s2s_I32AtomicAdd16U_MultiMem64
  S2sI32AtomicAdd16UMultiMem64 = 1441,
  /// k_s2s_I32AtomicAdd16U_MultiMem
  S2sI32AtomicAdd16UMultiMem = 1442,
  /// k_s2s_I32AtomicAdd16U_Mem64
  S2sI32AtomicAdd16UMem64 = 1443,
  /// k_s2s_I32AtomicSub_MultiMem64
  S2sI32AtomicSubMultiMem64 = 1444,
  /// k_s2s_I32AtomicSub_MultiMem
  S2sI32AtomicSubMultiMem = 1445,
  /// k_s2s_I32AtomicSub_Mem64
  S2sI32AtomicSubMem64 = 1446,
  /// k_s2s_I32AtomicSub8U_MultiMem64
  S2sI32AtomicSub8UMultiMem64 = 1447,
  /// k_s2s_I32AtomicSub8U_MultiMem
  S2sI32AtomicSub8UMultiMem = 1448,
  /// k_s2s_I32AtomicSub8U_Mem64
  S2sI32AtomicSub8UMem64 = 1449,
  /// k_s2s_I32AtomicSub16U_MultiMem64
  S2sI32AtomicSub16UMultiMem64 = 1450,
  /// k_s2s_I32AtomicSub16U_MultiMem
  S2sI32AtomicSub16UMultiMem = 1451,
  /// k_s2s_I32AtomicSub16U_Mem64
  S2sI32AtomicSub16UMem64 = 1452,
  /// k_s2s_I32AtomicAnd_MultiMem64
  S2sI32AtomicAndMultiMem64 = 1453,
  /// k_s2s_I32AtomicAnd_MultiMem
  S2sI32AtomicAndMultiMem = 1454,
  /// k_s2s_I32AtomicAnd_Mem64
  S2sI32AtomicAndMem64 = 1455,
  /// k_s2s_I32AtomicAnd8U_MultiMem64
  S2sI32AtomicAnd8UMultiMem64 = 1456,
  /// k_s2s_I32AtomicAnd8U_MultiMem
  S2sI32AtomicAnd8UMultiMem = 1457,
  /// k_s2s_I32AtomicAnd8U_Mem64
  S2sI32AtomicAnd8UMem64 = 1458,
  /// k_s2s_I32AtomicAnd16U_MultiMem64
  S2sI32AtomicAnd16UMultiMem64 = 1459,
  /// k_s2s_I32AtomicAnd16U_MultiMem
  S2sI32AtomicAnd16UMultiMem = 1460,
  /// k_s2s_I32AtomicAnd16U_Mem64
  S2sI32AtomicAnd16UMem64 = 1461,
  /// k_s2s_I32AtomicOr_MultiMem64
  S2sI32AtomicOrMultiMem64 = 1462,
  /// k_s2s_I32AtomicOr_MultiMem
  S2sI32AtomicOrMultiMem = 1463,
  /// k_s2s_I32AtomicOr_Mem64
  S2sI32AtomicOrMem64 = 1464,
  /// k_s2s_I32AtomicOr8U_MultiMem64
  S2sI32AtomicOr8UMultiMem64 = 1465,
  /// k_s2s_I32AtomicOr8U_MultiMem
  S2sI32AtomicOr8UMultiMem = 1466,
  /// k_s2s_I32AtomicOr8U_Mem64
  S2sI32AtomicOr8UMem64 = 1467,
  /// k_s2s_I32AtomicOr16U_MultiMem64
  S2sI32AtomicOr16UMultiMem64 = 1468,
  /// k_s2s_I32AtomicOr16U_MultiMem
  S2sI32AtomicOr16UMultiMem = 1469,
  /// k_s2s_I32AtomicOr16U_Mem64
  S2sI32AtomicOr16UMem64 = 1470,
  /// k_s2s_I32AtomicXor_MultiMem64
  S2sI32AtomicXorMultiMem64 = 1471,
  /// k_s2s_I32AtomicXor_MultiMem
  S2sI32AtomicXorMultiMem = 1472,
  /// k_s2s_I32AtomicXor_Mem64
  S2sI32AtomicXorMem64 = 1473,
  /// k_s2s_I32AtomicXor8U_MultiMem64
  S2sI32AtomicXor8UMultiMem64 = 1474,
  /// k_s2s_I32AtomicXor8U_MultiMem
  S2sI32AtomicXor8UMultiMem = 1475,
  /// k_s2s_I32AtomicXor8U_Mem64
  S2sI32AtomicXor8UMem64 = 1476,
  /// k_s2s_I32AtomicXor16U_MultiMem64
  S2sI32AtomicXor16UMultiMem64 = 1477,
  /// k_s2s_I32AtomicXor16U_MultiMem
  S2sI32AtomicXor16UMultiMem = 1478,
  /// k_s2s_I32AtomicXor16U_Mem64
  S2sI32AtomicXor16UMem64 = 1479,
  /// k_s2s_I32AtomicExchange_MultiMem64
  S2sI32AtomicExchangeMultiMem64 = 1480,
  /// k_s2s_I32AtomicExchange_MultiMem
  S2sI32AtomicExchangeMultiMem = 1481,
  /// k_s2s_I32AtomicExchange_Mem64
  S2sI32AtomicExchangeMem64 = 1482,
  /// k_s2s_I32AtomicExchange8U_MultiMem64
  S2sI32AtomicExchange8UMultiMem64 = 1483,
  /// k_s2s_I32AtomicExchange8U_MultiMem
  S2sI32AtomicExchange8UMultiMem = 1484,
  /// k_s2s_I32AtomicExchange8U_Mem64
  S2sI32AtomicExchange8UMem64 = 1485,
  /// k_s2s_I32AtomicExchange16U_MultiMem64
  S2sI32AtomicExchange16UMultiMem64 = 1486,
  /// k_s2s_I32AtomicExchange16U_MultiMem
  S2sI32AtomicExchange16UMultiMem = 1487,
  /// k_s2s_I32AtomicExchange16U_Mem64
  S2sI32AtomicExchange16UMem64 = 1488,
  /// k_s2s_I64AtomicAdd_MultiMem64
  S2sI64AtomicAddMultiMem64 = 1489,
  /// k_s2s_I64AtomicAdd_MultiMem
  S2sI64AtomicAddMultiMem = 1490,
  /// k_s2s_I64AtomicAdd_Mem64
  S2sI64AtomicAddMem64 = 1491,
  /// k_s2s_I64AtomicAdd8U_MultiMem64
  S2sI64AtomicAdd8UMultiMem64 = 1492,
  /// k_s2s_I64AtomicAdd8U_MultiMem
  S2sI64AtomicAdd8UMultiMem = 1493,
  /// k_s2s_I64AtomicAdd8U_Mem64
  S2sI64AtomicAdd8UMem64 = 1494,
  /// k_s2s_I64AtomicAdd16U_MultiMem64
  S2sI64AtomicAdd16UMultiMem64 = 1495,
  /// k_s2s_I64AtomicAdd16U_MultiMem
  S2sI64AtomicAdd16UMultiMem = 1496,
  /// k_s2s_I64AtomicAdd16U_Mem64
  S2sI64AtomicAdd16UMem64 = 1497,
  /// k_s2s_I64AtomicAdd32U_MultiMem64
  S2sI64AtomicAdd32UMultiMem64 = 1498,
  /// k_s2s_I64AtomicAdd32U_MultiMem
  S2sI64AtomicAdd32UMultiMem = 1499,
  /// k_s2s_I64AtomicAdd32U_Mem64
  S2sI64AtomicAdd32UMem64 = 1500,
  /// k_s2s_I64AtomicSub_MultiMem64
  S2sI64AtomicSubMultiMem64 = 1501,
  /// k_s2s_I64AtomicSub_MultiMem
  S2sI64AtomicSubMultiMem = 1502,
  /// k_s2s_I64AtomicSub_Mem64
  S2sI64AtomicSubMem64 = 1503,
  /// k_s2s_I64AtomicSub8U_MultiMem64
  S2sI64AtomicSub8UMultiMem64 = 1504,
  /// k_s2s_I64AtomicSub8U_MultiMem
  S2sI64AtomicSub8UMultiMem = 1505,
  /// k_s2s_I64AtomicSub8U_Mem64
  S2sI64AtomicSub8UMem64 = 1506,
  /// k_s2s_I64AtomicSub16U_MultiMem64
  S2sI64AtomicSub16UMultiMem64 = 1507,
  /// k_s2s_I64AtomicSub16U_MultiMem
  S2sI64AtomicSub16UMultiMem = 1508,
  /// k_s2s_I64AtomicSub16U_Mem64
  S2sI64AtomicSub16UMem64 = 1509,
  /// k_s2s_I64AtomicSub32U_MultiMem64
  S2sI64AtomicSub32UMultiMem64 = 1510,
  /// k_s2s_I64AtomicSub32U_MultiMem
  S2sI64AtomicSub32UMultiMem = 1511,
  /// k_s2s_I64AtomicSub32U_Mem64
  S2sI64AtomicSub32UMem64 = 1512,
  /// k_s2s_I64AtomicAnd_MultiMem64
  S2sI64AtomicAndMultiMem64 = 1513,
  /// k_s2s_I64AtomicAnd_MultiMem
  S2sI64AtomicAndMultiMem = 1514,
  /// k_s2s_I64AtomicAnd_Mem64
  S2sI64AtomicAndMem64 = 1515,
  /// k_s2s_I64AtomicAnd8U_MultiMem64
  S2sI64AtomicAnd8UMultiMem64 = 1516,
  /// k_s2s_I64AtomicAnd8U_MultiMem
  S2sI64AtomicAnd8UMultiMem = 1517,
  /// k_s2s_I64AtomicAnd8U_Mem64
  S2sI64AtomicAnd8UMem64 = 1518,
  /// k_s2s_I64AtomicAnd16U_MultiMem64
  S2sI64AtomicAnd16UMultiMem64 = 1519,
  /// k_s2s_I64AtomicAnd16U_MultiMem
  S2sI64AtomicAnd16UMultiMem = 1520,
  /// k_s2s_I64AtomicAnd16U_Mem64
  S2sI64AtomicAnd16UMem64 = 1521,
  /// k_s2s_I64AtomicAnd32U_MultiMem64
  S2sI64AtomicAnd32UMultiMem64 = 1522,
  /// k_s2s_I64AtomicAnd32U_MultiMem
  S2sI64AtomicAnd32UMultiMem = 1523,
  /// k_s2s_I64AtomicAnd32U_Mem64
  S2sI64AtomicAnd32UMem64 = 1524,
  /// k_s2s_I64AtomicOr_MultiMem64
  S2sI64AtomicOrMultiMem64 = 1525,
  /// k_s2s_I64AtomicOr_MultiMem
  S2sI64AtomicOrMultiMem = 1526,
  /// k_s2s_I64AtomicOr_Mem64
  S2sI64AtomicOrMem64 = 1527,
  /// k_s2s_I64AtomicOr8U_MultiMem64
  S2sI64AtomicOr8UMultiMem64 = 1528,
  /// k_s2s_I64AtomicOr8U_MultiMem
  S2sI64AtomicOr8UMultiMem = 1529,
  /// k_s2s_I64AtomicOr8U_Mem64
  S2sI64AtomicOr8UMem64 = 1530,
  /// k_s2s_I64AtomicOr16U_MultiMem64
  S2sI64AtomicOr16UMultiMem64 = 1531,
  /// k_s2s_I64AtomicOr16U_MultiMem
  S2sI64AtomicOr16UMultiMem = 1532,
  /// k_s2s_I64AtomicOr16U_Mem64
  S2sI64AtomicOr16UMem64 = 1533,
  /// k_s2s_I64AtomicOr32U_MultiMem64
  S2sI64AtomicOr32UMultiMem64 = 1534,
  /// k_s2s_I64AtomicOr32U_MultiMem
  S2sI64AtomicOr32UMultiMem = 1535,
  /// k_s2s_I64AtomicOr32U_Mem64
  S2sI64AtomicOr32UMem64 = 1536,
  /// k_s2s_I64AtomicXor_MultiMem64
  S2sI64AtomicXorMultiMem64 = 1537,
  /// k_s2s_I64AtomicXor_MultiMem
  S2sI64AtomicXorMultiMem = 1538,
  /// k_s2s_I64AtomicXor_Mem64
  S2sI64AtomicXorMem64 = 1539,
  /// k_s2s_I64AtomicXor8U_MultiMem64
  S2sI64AtomicXor8UMultiMem64 = 1540,
  /// k_s2s_I64AtomicXor8U_MultiMem
  S2sI64AtomicXor8UMultiMem = 1541,
  /// k_s2s_I64AtomicXor8U_Mem64
  S2sI64AtomicXor8UMem64 = 1542,
  /// k_s2s_I64AtomicXor16U_MultiMem64
  S2sI64AtomicXor16UMultiMem64 = 1543,
  /// k_s2s_I64AtomicXor16U_MultiMem
  S2sI64AtomicXor16UMultiMem = 1544,
  /// k_s2s_I64AtomicXor16U_Mem64
  S2sI64AtomicXor16UMem64 = 1545,
  /// k_s2s_I64AtomicXor32U_MultiMem64
  S2sI64AtomicXor32UMultiMem64 = 1546,
  /// k_s2s_I64AtomicXor32U_MultiMem
  S2sI64AtomicXor32UMultiMem = 1547,
  /// k_s2s_I64AtomicXor32U_Mem64
  S2sI64AtomicXor32UMem64 = 1548,
  /// k_s2s_I64AtomicExchange_MultiMem64
  S2sI64AtomicExchangeMultiMem64 = 1549,
  /// k_s2s_I64AtomicExchange_MultiMem
  S2sI64AtomicExchangeMultiMem = 1550,
  /// k_s2s_I64AtomicExchange_Mem64
  S2sI64AtomicExchangeMem64 = 1551,
  /// k_s2s_I64AtomicExchange8U_MultiMem64
  S2sI64AtomicExchange8UMultiMem64 = 1552,
  /// k_s2s_I64AtomicExchange8U_MultiMem
  S2sI64AtomicExchange8UMultiMem = 1553,
  /// k_s2s_I64AtomicExchange8U_Mem64
  S2sI64AtomicExchange8UMem64 = 1554,
  /// k_s2s_I64AtomicExchange16U_MultiMem64
  S2sI64AtomicExchange16UMultiMem64 = 1555,
  /// k_s2s_I64AtomicExchange16U_MultiMem
  S2sI64AtomicExchange16UMultiMem = 1556,
  /// k_s2s_I64AtomicExchange16U_Mem64
  S2sI64AtomicExchange16UMem64 = 1557,
  /// k_s2s_I64AtomicExchange32U_MultiMem64
  S2sI64AtomicExchange32UMultiMem64 = 1558,
  /// k_s2s_I64AtomicExchange32U_MultiMem
  S2sI64AtomicExchange32UMultiMem = 1559,
  /// k_s2s_I64AtomicExchange32U_Mem64
  S2sI64AtomicExchange32UMem64 = 1560,
  /// k_s2s_I32AtomicCompareExchange_MultiMem64
  S2sI32AtomicCompareExchangeMultiMem64 = 1561,
  /// k_s2s_I32AtomicCompareExchange_MultiMem
  S2sI32AtomicCompareExchangeMultiMem = 1562,
  /// k_s2s_I32AtomicCompareExchange_Mem64
  S2sI32AtomicCompareExchangeMem64 = 1563,
  /// k_s2s_I32AtomicCompareExchange8U_MultiMem64
  S2sI32AtomicCompareExchange8UMultiMem64 = 1564,
  /// k_s2s_I32AtomicCompareExchange8U_MultiMem
  S2sI32AtomicCompareExchange8UMultiMem = 1565,
  /// k_s2s_I32AtomicCompareExchange8U_Mem64
  S2sI32AtomicCompareExchange8UMem64 = 1566,
  /// k_s2s_I32AtomicCompareExchange16U_MultiMem64
  S2sI32AtomicCompareExchange16UMultiMem64 = 1567,
  /// k_s2s_I32AtomicCompareExchange16U_MultiMem
  S2sI32AtomicCompareExchange16UMultiMem = 1568,
  /// k_s2s_I32AtomicCompareExchange16U_Mem64
  S2sI32AtomicCompareExchange16UMem64 = 1569,
  /// k_s2s_I64AtomicCompareExchange_MultiMem64
  S2sI64AtomicCompareExchangeMultiMem64 = 1570,
  /// k_s2s_I64AtomicCompareExchange_MultiMem
  S2sI64AtomicCompareExchangeMultiMem = 1571,
  /// k_s2s_I64AtomicCompareExchange_Mem64
  S2sI64AtomicCompareExchangeMem64 = 1572,
  /// k_s2s_I64AtomicCompareExchange8U_MultiMem64
  S2sI64AtomicCompareExchange8UMultiMem64 = 1573,
  /// k_s2s_I64AtomicCompareExchange8U_MultiMem
  S2sI64AtomicCompareExchange8UMultiMem = 1574,
  /// k_s2s_I64AtomicCompareExchange8U_Mem64
  S2sI64AtomicCompareExchange8UMem64 = 1575,
  /// k_s2s_I64AtomicCompareExchange16U_MultiMem64
  S2sI64AtomicCompareExchange16UMultiMem64 = 1576,
  /// k_s2s_I64AtomicCompareExchange16U_MultiMem
  S2sI64AtomicCompareExchange16UMultiMem = 1577,
  /// k_s2s_I64AtomicCompareExchange16U_Mem64
  S2sI64AtomicCompareExchange16UMem64 = 1578,
  /// k_s2s_I64AtomicCompareExchange32U_MultiMem64
  S2sI64AtomicCompareExchange32UMultiMem64 = 1579,
  /// k_s2s_I64AtomicCompareExchange32U_MultiMem
  S2sI64AtomicCompareExchange32UMultiMem = 1580,
  /// k_s2s_I64AtomicCompareExchange32U_Mem64
  S2sI64AtomicCompareExchange32UMem64 = 1581,
  /// k_s2s_I32AtomicLoad_MultiMem64
  S2sI32AtomicLoadMultiMem64 = 1582,
  /// k_s2s_I32AtomicLoad_MultiMem
  S2sI32AtomicLoadMultiMem = 1583,
  /// k_s2s_I32AtomicLoad_Mem64
  S2sI32AtomicLoadMem64 = 1584,
  /// k_s2s_I32AtomicLoad8U_MultiMem64
  S2sI32AtomicLoad8UMultiMem64 = 1585,
  /// k_s2s_I32AtomicLoad8U_MultiMem
  S2sI32AtomicLoad8UMultiMem = 1586,
  /// k_s2s_I32AtomicLoad8U_Mem64
  S2sI32AtomicLoad8UMem64 = 1587,
  /// k_s2s_I32AtomicLoad16U_MultiMem64
  S2sI32AtomicLoad16UMultiMem64 = 1588,
  /// k_s2s_I32AtomicLoad16U_MultiMem
  S2sI32AtomicLoad16UMultiMem = 1589,
  /// k_s2s_I32AtomicLoad16U_Mem64
  S2sI32AtomicLoad16UMem64 = 1590,
  /// k_s2s_I64AtomicLoad_MultiMem64
  S2sI64AtomicLoadMultiMem64 = 1591,
  /// k_s2s_I64AtomicLoad_MultiMem
  S2sI64AtomicLoadMultiMem = 1592,
  /// k_s2s_I64AtomicLoad_Mem64
  S2sI64AtomicLoadMem64 = 1593,
  /// k_s2s_I64AtomicLoad8U_MultiMem64
  S2sI64AtomicLoad8UMultiMem64 = 1594,
  /// k_s2s_I64AtomicLoad8U_MultiMem
  S2sI64AtomicLoad8UMultiMem = 1595,
  /// k_s2s_I64AtomicLoad8U_Mem64
  S2sI64AtomicLoad8UMem64 = 1596,
  /// k_s2s_I64AtomicLoad16U_MultiMem64
  S2sI64AtomicLoad16UMultiMem64 = 1597,
  /// k_s2s_I64AtomicLoad16U_MultiMem
  S2sI64AtomicLoad16UMultiMem = 1598,
  /// k_s2s_I64AtomicLoad16U_Mem64
  S2sI64AtomicLoad16UMem64 = 1599,
  /// k_s2s_I64AtomicLoad32U_MultiMem64
  S2sI64AtomicLoad32UMultiMem64 = 1600,
  /// k_s2s_I64AtomicLoad32U_MultiMem
  S2sI64AtomicLoad32UMultiMem = 1601,
  /// k_s2s_I64AtomicLoad32U_Mem64
  S2sI64AtomicLoad32UMem64 = 1602,
  /// k_s2s_I32AtomicStore_MultiMem64
  S2sI32AtomicStoreMultiMem64 = 1603,
  /// k_s2s_I32AtomicStore_MultiMem
  S2sI32AtomicStoreMultiMem = 1604,
  /// k_s2s_I32AtomicStore_Mem64
  S2sI32AtomicStoreMem64 = 1605,
  /// k_s2s_I32AtomicStore8U_MultiMem64
  S2sI32AtomicStore8UMultiMem64 = 1606,
  /// k_s2s_I32AtomicStore8U_MultiMem
  S2sI32AtomicStore8UMultiMem = 1607,
  /// k_s2s_I32AtomicStore8U_Mem64
  S2sI32AtomicStore8UMem64 = 1608,
  /// k_s2s_I32AtomicStore16U_MultiMem64
  S2sI32AtomicStore16UMultiMem64 = 1609,
  /// k_s2s_I32AtomicStore16U_MultiMem
  S2sI32AtomicStore16UMultiMem = 1610,
  /// k_s2s_I32AtomicStore16U_Mem64
  S2sI32AtomicStore16UMem64 = 1611,
  /// k_s2s_I64AtomicStore_MultiMem64
  S2sI64AtomicStoreMultiMem64 = 1612,
  /// k_s2s_I64AtomicStore_MultiMem
  S2sI64AtomicStoreMultiMem = 1613,
  /// k_s2s_I64AtomicStore_Mem64
  S2sI64AtomicStoreMem64 = 1614,
  /// k_s2s_I64AtomicStore8U_MultiMem64
  S2sI64AtomicStore8UMultiMem64 = 1615,
  /// k_s2s_I64AtomicStore8U_MultiMem
  S2sI64AtomicStore8UMultiMem = 1616,
  /// k_s2s_I64AtomicStore8U_Mem64
  S2sI64AtomicStore8UMem64 = 1617,
  /// k_s2s_I64AtomicStore16U_MultiMem64
  S2sI64AtomicStore16UMultiMem64 = 1618,
  /// k_s2s_I64AtomicStore16U_MultiMem
  S2sI64AtomicStore16UMultiMem = 1619,
  /// k_s2s_I64AtomicStore16U_Mem64
  S2sI64AtomicStore16UMem64 = 1620,
  /// k_s2s_I64AtomicStore32U_MultiMem64
  S2sI64AtomicStore32UMultiMem64 = 1621,
  /// k_s2s_I64AtomicStore32U_MultiMem
  S2sI64AtomicStore32UMultiMem = 1622,
  /// k_s2s_I64AtomicStore32U_Mem64
  S2sI64AtomicStore32UMem64 = 1623,
  /// k_s2s_AtomicNotify
  S2sAtomicNotify = 1624,
  /// k_s2s_I32AtomicWait
  S2sI32AtomicWait = 1625,
  /// k_s2s_I64AtomicWait
  S2sI64AtomicWait = 1626,
  /// k_s2s_I32AtomicAdd
  S2sI32AtomicAdd = 1627,
  /// k_s2s_I32AtomicAdd8U
  S2sI32AtomicAdd8U = 1628,
  /// k_s2s_I32AtomicAdd16U
  S2sI32AtomicAdd16U = 1629,
  /// k_s2s_I32AtomicSub
  S2sI32AtomicSub = 1630,
  /// k_s2s_I32AtomicSub8U
  S2sI32AtomicSub8U = 1631,
  /// k_s2s_I32AtomicSub16U
  S2sI32AtomicSub16U = 1632,
  /// k_s2s_I32AtomicAnd
  S2sI32AtomicAnd = 1633,
  /// k_s2s_I32AtomicAnd8U
  S2sI32AtomicAnd8U = 1634,
  /// k_s2s_I32AtomicAnd16U
  S2sI32AtomicAnd16U = 1635,
  /// k_s2s_I32AtomicOr
  S2sI32AtomicOr = 1636,
  /// k_s2s_I32AtomicOr8U
  S2sI32AtomicOr8U = 1637,
  /// k_s2s_I32AtomicOr16U
  S2sI32AtomicOr16U = 1638,
  /// k_s2s_I32AtomicXor
  S2sI32AtomicXor = 1639,
  /// k_s2s_I32AtomicXor8U
  S2sI32AtomicXor8U = 1640,
  /// k_s2s_I32AtomicXor16U
  S2sI32AtomicXor16U = 1641,
  /// k_s2s_I32AtomicExchange
  S2sI32AtomicExchange = 1642,
  /// k_s2s_I32AtomicExchange8U
  S2sI32AtomicExchange8U = 1643,
  /// k_s2s_I32AtomicExchange16U
  S2sI32AtomicExchange16U = 1644,
  /// k_s2s_I64AtomicAdd
  S2sI64AtomicAdd = 1645,
  /// k_s2s_I64AtomicAdd8U
  S2sI64AtomicAdd8U = 1646,
  /// k_s2s_I64AtomicAdd16U
  S2sI64AtomicAdd16U = 1647,
  /// k_s2s_I64AtomicAdd32U
  S2sI64AtomicAdd32U = 1648,
  /// k_s2s_I64AtomicSub
  S2sI64AtomicSub = 1649,
  /// k_s2s_I64AtomicSub8U
  S2sI64AtomicSub8U = 1650,
  /// k_s2s_I64AtomicSub16U
  S2sI64AtomicSub16U = 1651,
  /// k_s2s_I64AtomicSub32U
  S2sI64AtomicSub32U = 1652,
  /// k_s2s_I64AtomicAnd
  S2sI64AtomicAnd = 1653,
  /// k_s2s_I64AtomicAnd8U
  S2sI64AtomicAnd8U = 1654,
  /// k_s2s_I64AtomicAnd16U
  S2sI64AtomicAnd16U = 1655,
  /// k_s2s_I64AtomicAnd32U
  S2sI64AtomicAnd32U = 1656,
  /// k_s2s_I64AtomicOr
  S2sI64AtomicOr = 1657,
  /// k_s2s_I64AtomicOr8U
  S2sI64AtomicOr8U = 1658,
  /// k_s2s_I64AtomicOr16U
  S2sI64AtomicOr16U = 1659,
  /// k_s2s_I64AtomicOr32U
  S2sI64AtomicOr32U = 1660,
  /// k_s2s_I64AtomicXor
  S2sI64AtomicXor = 1661,
  /// k_s2s_I64AtomicXor8U
  S2sI64AtomicXor8U = 1662,
  /// k_s2s_I64AtomicXor16U
  S2sI64AtomicXor16U = 1663,
  /// k_s2s_I64AtomicXor32U
  S2sI64AtomicXor32U = 1664,
  /// k_s2s_I64AtomicExchange
  S2sI64AtomicExchange = 1665,
  /// k_s2s_I64AtomicExchange8U
  S2sI64AtomicExchange8U = 1666,
  /// k_s2s_I64AtomicExchange16U
  S2sI64AtomicExchange16U = 1667,
  /// k_s2s_I64AtomicExchange32U
  S2sI64AtomicExchange32U = 1668,
  /// k_s2s_I32AtomicCompareExchange
  S2sI32AtomicCompareExchange = 1669,
  /// k_s2s_I32AtomicCompareExchange8U
  S2sI32AtomicCompareExchange8U = 1670,
  /// k_s2s_I32AtomicCompareExchange16U
  S2sI32AtomicCompareExchange16U = 1671,
  /// k_s2s_I64AtomicCompareExchange
  S2sI64AtomicCompareExchange = 1672,
  /// k_s2s_I64AtomicCompareExchange8U
  S2sI64AtomicCompareExchange8U = 1673,
  /// k_s2s_I64AtomicCompareExchange16U
  S2sI64AtomicCompareExchange16U = 1674,
  /// k_s2s_I64AtomicCompareExchange32U
  S2sI64AtomicCompareExchange32U = 1675,
  /// k_s2s_I32AtomicLoad
  S2sI32AtomicLoad = 1676,
  /// k_s2s_I32AtomicLoad8U
  S2sI32AtomicLoad8U = 1677,
  /// k_s2s_I32AtomicLoad16U
  S2sI32AtomicLoad16U = 1678,
  /// k_s2s_I64AtomicLoad
  S2sI64AtomicLoad = 1679,
  /// k_s2s_I64AtomicLoad8U
  S2sI64AtomicLoad8U = 1680,
  /// k_s2s_I64AtomicLoad16U
  S2sI64AtomicLoad16U = 1681,
  /// k_s2s_I64AtomicLoad32U
  S2sI64AtomicLoad32U = 1682,
  /// k_s2s_I32AtomicStore
  S2sI32AtomicStore = 1683,
  /// k_s2s_I32AtomicStore8U
  S2sI32AtomicStore8U = 1684,
  /// k_s2s_I32AtomicStore16U
  S2sI32AtomicStore16U = 1685,
  /// k_s2s_I64AtomicStore
  S2sI64AtomicStore = 1686,
  /// k_s2s_I64AtomicStore8U
  S2sI64AtomicStore8U = 1687,
  /// k_s2s_I64AtomicStore16U
  S2sI64AtomicStore16U = 1688,
  /// k_s2s_I64AtomicStore32U
  S2sI64AtomicStore32U = 1689,
}

impl InstructionHandler {
  /// 处理函数总数 (原版 kInstructionCount)
  pub const COUNT: u16 = 1690;

  /// 从数值反查 (超表 large-id 以取模落表, 见原版 ReadFnId)
  #[inline]
  #[must_use]
  pub const fn from_repr(val: u16) -> Option<Self> {
    match val {
      0 => Some(Self::R2rI32LoadMem8S),
      1 => Some(Self::R2rI32LoadMem8U),
      2 => Some(Self::R2rI32LoadMem16S),
      3 => Some(Self::R2rI32LoadMem16U),
      4 => Some(Self::R2rI64LoadMem8S),
      5 => Some(Self::R2rI64LoadMem8U),
      6 => Some(Self::R2rI64LoadMem16S),
      7 => Some(Self::R2rI64LoadMem16U),
      8 => Some(Self::R2rI64LoadMem32S),
      9 => Some(Self::R2rI64LoadMem32U),
      10 => Some(Self::R2rI32LoadMem),
      11 => Some(Self::R2rI64LoadMem),
      12 => Some(Self::R2rF32LoadMem),
      13 => Some(Self::R2rF64LoadMem),
      14 => Some(Self::R2sI32LoadMem8S),
      15 => Some(Self::R2sI32LoadMem8U),
      16 => Some(Self::R2sI32LoadMem16S),
      17 => Some(Self::R2sI32LoadMem16U),
      18 => Some(Self::R2sI64LoadMem8S),
      19 => Some(Self::R2sI64LoadMem8U),
      20 => Some(Self::R2sI64LoadMem16S),
      21 => Some(Self::R2sI64LoadMem16U),
      22 => Some(Self::R2sI64LoadMem32S),
      23 => Some(Self::R2sI64LoadMem32U),
      24 => Some(Self::R2sI32LoadMem),
      25 => Some(Self::R2sI64LoadMem),
      26 => Some(Self::R2sF32LoadMem),
      27 => Some(Self::R2sF64LoadMem),
      28 => Some(Self::S2rI32LoadMem8S),
      29 => Some(Self::S2rI32LoadMem8U),
      30 => Some(Self::S2rI32LoadMem16S),
      31 => Some(Self::S2rI32LoadMem16U),
      32 => Some(Self::S2rI64LoadMem8S),
      33 => Some(Self::S2rI64LoadMem8U),
      34 => Some(Self::S2rI64LoadMem16S),
      35 => Some(Self::S2rI64LoadMem16U),
      36 => Some(Self::S2rI64LoadMem32S),
      37 => Some(Self::S2rI64LoadMem32U),
      38 => Some(Self::S2rI32LoadMem),
      39 => Some(Self::S2rI64LoadMem),
      40 => Some(Self::S2rF32LoadMem),
      41 => Some(Self::S2rF64LoadMem),
      42 => Some(Self::S2sI32LoadMem8S),
      43 => Some(Self::S2sI32LoadMem8U),
      44 => Some(Self::S2sI32LoadMem16S),
      45 => Some(Self::S2sI32LoadMem16U),
      46 => Some(Self::S2sI64LoadMem8S),
      47 => Some(Self::S2sI64LoadMem8U),
      48 => Some(Self::S2sI64LoadMem16S),
      49 => Some(Self::S2sI64LoadMem16U),
      50 => Some(Self::S2sI64LoadMem32S),
      51 => Some(Self::S2sI64LoadMem32U),
      52 => Some(Self::S2sI32LoadMem),
      53 => Some(Self::S2sI64LoadMem),
      54 => Some(Self::S2sF32LoadMem),
      55 => Some(Self::S2sF64LoadMem),
      56 => Some(Self::R2sI32StoreMem8),
      57 => Some(Self::R2sI32StoreMem16),
      58 => Some(Self::R2sI64StoreMem8),
      59 => Some(Self::R2sI64StoreMem16),
      60 => Some(Self::R2sI64StoreMem32),
      61 => Some(Self::R2sI32StoreMem),
      62 => Some(Self::R2sI64StoreMem),
      63 => Some(Self::R2sF32StoreMem),
      64 => Some(Self::R2sF64StoreMem),
      65 => Some(Self::S2sI32StoreMem8),
      66 => Some(Self::S2sI32StoreMem16),
      67 => Some(Self::S2sI64StoreMem8),
      68 => Some(Self::S2sI64StoreMem16),
      69 => Some(Self::S2sI64StoreMem32),
      70 => Some(Self::S2sI32StoreMem),
      71 => Some(Self::S2sI64StoreMem),
      72 => Some(Self::S2sF32StoreMem),
      73 => Some(Self::S2sF64StoreMem),
      74 => Some(Self::S2sI32LoadMem8SLocalSet),
      75 => Some(Self::S2sI32LoadMem8ULocalSet),
      76 => Some(Self::S2sI32LoadMem16SLocalSet),
      77 => Some(Self::S2sI32LoadMem16ULocalSet),
      78 => Some(Self::S2sI64LoadMem8SLocalSet),
      79 => Some(Self::S2sI64LoadMem8ULocalSet),
      80 => Some(Self::S2sI64LoadMem16SLocalSet),
      81 => Some(Self::S2sI64LoadMem16ULocalSet),
      82 => Some(Self::S2sI64LoadMem32SLocalSet),
      83 => Some(Self::S2sI64LoadMem32ULocalSet),
      84 => Some(Self::S2sI32LoadMemLocalSet),
      85 => Some(Self::S2sI64LoadMemLocalSet),
      86 => Some(Self::S2sF32LoadMemLocalSet),
      87 => Some(Self::S2sF64LoadMemLocalSet),
      88 => Some(Self::R2sI32LoadStoreMem),
      89 => Some(Self::R2sI64LoadStoreMem),
      90 => Some(Self::R2sF32LoadStoreMem),
      91 => Some(Self::R2sF64LoadStoreMem),
      92 => Some(Self::S2sI32LoadStoreMem),
      93 => Some(Self::S2sI64LoadStoreMem),
      94 => Some(Self::S2sF32LoadStoreMem),
      95 => Some(Self::S2sF64LoadStoreMem),
      96 => Some(Self::S2rI32GlobalGet),
      97 => Some(Self::S2rI64GlobalGet),
      98 => Some(Self::S2rF32GlobalGet),
      99 => Some(Self::S2rF64GlobalGet),
      100 => Some(Self::S2sI32GlobalGet),
      101 => Some(Self::S2sI64GlobalGet),
      102 => Some(Self::S2sF32GlobalGet),
      103 => Some(Self::S2sF64GlobalGet),
      104 => Some(Self::S2sS128GlobalGet),
      105 => Some(Self::S2sRefGlobalGet),
      106 => Some(Self::R2sI32GlobalSet),
      107 => Some(Self::R2sI64GlobalSet),
      108 => Some(Self::R2sF32GlobalSet),
      109 => Some(Self::R2sF64GlobalSet),
      110 => Some(Self::S2sI32GlobalSet),
      111 => Some(Self::S2sI64GlobalSet),
      112 => Some(Self::S2sF32GlobalSet),
      113 => Some(Self::S2sF64GlobalSet),
      114 => Some(Self::S2sS128GlobalSet),
      115 => Some(Self::S2sRefGlobalSet),
      116 => Some(Self::R2sI32Drop),
      117 => Some(Self::R2sI64Drop),
      118 => Some(Self::R2sF32Drop),
      119 => Some(Self::R2sF64Drop),
      120 => Some(Self::R2sRefDrop),
      121 => Some(Self::S2sI32Drop),
      122 => Some(Self::S2sI64Drop),
      123 => Some(Self::S2sF32Drop),
      124 => Some(Self::S2sF64Drop),
      125 => Some(Self::S2sS128Drop),
      126 => Some(Self::S2sRefDrop),
      127 => Some(Self::R2rI32Select),
      128 => Some(Self::R2rI64Select),
      129 => Some(Self::R2rF32Select),
      130 => Some(Self::R2rF64Select),
      131 => Some(Self::R2sI32Select),
      132 => Some(Self::R2sI64Select),
      133 => Some(Self::R2sF32Select),
      134 => Some(Self::R2sF64Select),
      135 => Some(Self::R2sS128Select),
      136 => Some(Self::R2sRefSelect),
      137 => Some(Self::S2rI32Select),
      138 => Some(Self::S2rI64Select),
      139 => Some(Self::S2rF32Select),
      140 => Some(Self::S2rF64Select),
      141 => Some(Self::S2sI32Select),
      142 => Some(Self::S2sI64Select),
      143 => Some(Self::S2sF32Select),
      144 => Some(Self::S2sF64Select),
      145 => Some(Self::S2sS128Select),
      146 => Some(Self::S2sRefSelect),
      147 => Some(Self::R2rI32Add),
      148 => Some(Self::R2rI32Sub),
      149 => Some(Self::R2rI32Mul),
      150 => Some(Self::R2rI32And),
      151 => Some(Self::R2rI32Ior),
      152 => Some(Self::R2rI32Xor),
      153 => Some(Self::R2rI64Add),
      154 => Some(Self::R2rI64Sub),
      155 => Some(Self::R2rI64Mul),
      156 => Some(Self::R2rI64And),
      157 => Some(Self::R2rI64Ior),
      158 => Some(Self::R2rI64Xor),
      159 => Some(Self::R2rF32Add),
      160 => Some(Self::R2rF32Sub),
      161 => Some(Self::R2rF32Mul),
      162 => Some(Self::R2rF64Add),
      163 => Some(Self::R2rF64Sub),
      164 => Some(Self::R2rF64Mul),
      165 => Some(Self::R2rI32DivS),
      166 => Some(Self::R2rI64DivS),
      167 => Some(Self::R2rI32DivU),
      168 => Some(Self::R2rI64DivU),
      169 => Some(Self::R2rF32Div),
      170 => Some(Self::R2rF64Div),
      171 => Some(Self::R2rI32RemS),
      172 => Some(Self::R2rI64RemS),
      173 => Some(Self::R2rI32RemU),
      174 => Some(Self::R2rI64RemU),
      175 => Some(Self::R2sI32Add),
      176 => Some(Self::R2sI32Sub),
      177 => Some(Self::R2sI32Mul),
      178 => Some(Self::R2sI32And),
      179 => Some(Self::R2sI32Ior),
      180 => Some(Self::R2sI32Xor),
      181 => Some(Self::R2sI64Add),
      182 => Some(Self::R2sI64Sub),
      183 => Some(Self::R2sI64Mul),
      184 => Some(Self::R2sI64And),
      185 => Some(Self::R2sI64Ior),
      186 => Some(Self::R2sI64Xor),
      187 => Some(Self::R2sF32Add),
      188 => Some(Self::R2sF32Sub),
      189 => Some(Self::R2sF32Mul),
      190 => Some(Self::R2sF64Add),
      191 => Some(Self::R2sF64Sub),
      192 => Some(Self::R2sF64Mul),
      193 => Some(Self::R2sI32DivS),
      194 => Some(Self::R2sI64DivS),
      195 => Some(Self::R2sI32DivU),
      196 => Some(Self::R2sI64DivU),
      197 => Some(Self::R2sF32Div),
      198 => Some(Self::R2sF64Div),
      199 => Some(Self::R2sI32RemS),
      200 => Some(Self::R2sI64RemS),
      201 => Some(Self::R2sI32RemU),
      202 => Some(Self::R2sI64RemU),
      203 => Some(Self::S2rI32Add),
      204 => Some(Self::S2rI32Sub),
      205 => Some(Self::S2rI32Mul),
      206 => Some(Self::S2rI32And),
      207 => Some(Self::S2rI32Ior),
      208 => Some(Self::S2rI32Xor),
      209 => Some(Self::S2rI64Add),
      210 => Some(Self::S2rI64Sub),
      211 => Some(Self::S2rI64Mul),
      212 => Some(Self::S2rI64And),
      213 => Some(Self::S2rI64Ior),
      214 => Some(Self::S2rI64Xor),
      215 => Some(Self::S2rF32Add),
      216 => Some(Self::S2rF32Sub),
      217 => Some(Self::S2rF32Mul),
      218 => Some(Self::S2rF64Add),
      219 => Some(Self::S2rF64Sub),
      220 => Some(Self::S2rF64Mul),
      221 => Some(Self::S2rI32DivS),
      222 => Some(Self::S2rI64DivS),
      223 => Some(Self::S2rI32DivU),
      224 => Some(Self::S2rI64DivU),
      225 => Some(Self::S2rF32Div),
      226 => Some(Self::S2rF64Div),
      227 => Some(Self::S2rI32RemS),
      228 => Some(Self::S2rI64RemS),
      229 => Some(Self::S2rI32RemU),
      230 => Some(Self::S2rI64RemU),
      231 => Some(Self::S2sI32Add),
      232 => Some(Self::S2sI32Sub),
      233 => Some(Self::S2sI32Mul),
      234 => Some(Self::S2sI32And),
      235 => Some(Self::S2sI32Ior),
      236 => Some(Self::S2sI32Xor),
      237 => Some(Self::S2sI64Add),
      238 => Some(Self::S2sI64Sub),
      239 => Some(Self::S2sI64Mul),
      240 => Some(Self::S2sI64And),
      241 => Some(Self::S2sI64Ior),
      242 => Some(Self::S2sI64Xor),
      243 => Some(Self::S2sF32Add),
      244 => Some(Self::S2sF32Sub),
      245 => Some(Self::S2sF32Mul),
      246 => Some(Self::S2sF64Add),
      247 => Some(Self::S2sF64Sub),
      248 => Some(Self::S2sF64Mul),
      249 => Some(Self::S2sI32DivS),
      250 => Some(Self::S2sI64DivS),
      251 => Some(Self::S2sI32DivU),
      252 => Some(Self::S2sI64DivU),
      253 => Some(Self::S2sF32Div),
      254 => Some(Self::S2sF64Div),
      255 => Some(Self::S2sI32RemS),
      256 => Some(Self::S2sI64RemS),
      257 => Some(Self::S2sI32RemU),
      258 => Some(Self::S2sI64RemU),
      259 => Some(Self::R2rI32Eq),
      260 => Some(Self::R2rI32Ne),
      261 => Some(Self::R2rI32LtU),
      262 => Some(Self::R2rI32LeU),
      263 => Some(Self::R2rI32GtU),
      264 => Some(Self::R2rI32GeU),
      265 => Some(Self::R2rI32LtS),
      266 => Some(Self::R2rI32LeS),
      267 => Some(Self::R2rI32GtS),
      268 => Some(Self::R2rI32GeS),
      269 => Some(Self::R2rI64Eq),
      270 => Some(Self::R2rI64Ne),
      271 => Some(Self::R2rI64LtU),
      272 => Some(Self::R2rI64LeU),
      273 => Some(Self::R2rI64GtU),
      274 => Some(Self::R2rI64GeU),
      275 => Some(Self::R2rI64LtS),
      276 => Some(Self::R2rI64LeS),
      277 => Some(Self::R2rI64GtS),
      278 => Some(Self::R2rI64GeS),
      279 => Some(Self::R2rF32Eq),
      280 => Some(Self::R2rF32Ne),
      281 => Some(Self::R2rF32Lt),
      282 => Some(Self::R2rF32Le),
      283 => Some(Self::R2rF32Gt),
      284 => Some(Self::R2rF32Ge),
      285 => Some(Self::R2rF64Eq),
      286 => Some(Self::R2rF64Ne),
      287 => Some(Self::R2rF64Lt),
      288 => Some(Self::R2rF64Le),
      289 => Some(Self::R2rF64Gt),
      290 => Some(Self::R2rF64Ge),
      291 => Some(Self::R2sI32Eq),
      292 => Some(Self::R2sI32Ne),
      293 => Some(Self::R2sI32LtU),
      294 => Some(Self::R2sI32LeU),
      295 => Some(Self::R2sI32GtU),
      296 => Some(Self::R2sI32GeU),
      297 => Some(Self::R2sI32LtS),
      298 => Some(Self::R2sI32LeS),
      299 => Some(Self::R2sI32GtS),
      300 => Some(Self::R2sI32GeS),
      301 => Some(Self::R2sI64Eq),
      302 => Some(Self::R2sI64Ne),
      303 => Some(Self::R2sI64LtU),
      304 => Some(Self::R2sI64LeU),
      305 => Some(Self::R2sI64GtU),
      306 => Some(Self::R2sI64GeU),
      307 => Some(Self::R2sI64LtS),
      308 => Some(Self::R2sI64LeS),
      309 => Some(Self::R2sI64GtS),
      310 => Some(Self::R2sI64GeS),
      311 => Some(Self::R2sF32Eq),
      312 => Some(Self::R2sF32Ne),
      313 => Some(Self::R2sF32Lt),
      314 => Some(Self::R2sF32Le),
      315 => Some(Self::R2sF32Gt),
      316 => Some(Self::R2sF32Ge),
      317 => Some(Self::R2sF64Eq),
      318 => Some(Self::R2sF64Ne),
      319 => Some(Self::R2sF64Lt),
      320 => Some(Self::R2sF64Le),
      321 => Some(Self::R2sF64Gt),
      322 => Some(Self::R2sF64Ge),
      323 => Some(Self::S2rI32Eq),
      324 => Some(Self::S2rI32Ne),
      325 => Some(Self::S2rI32LtU),
      326 => Some(Self::S2rI32LeU),
      327 => Some(Self::S2rI32GtU),
      328 => Some(Self::S2rI32GeU),
      329 => Some(Self::S2rI32LtS),
      330 => Some(Self::S2rI32LeS),
      331 => Some(Self::S2rI32GtS),
      332 => Some(Self::S2rI32GeS),
      333 => Some(Self::S2rI64Eq),
      334 => Some(Self::S2rI64Ne),
      335 => Some(Self::S2rI64LtU),
      336 => Some(Self::S2rI64LeU),
      337 => Some(Self::S2rI64GtU),
      338 => Some(Self::S2rI64GeU),
      339 => Some(Self::S2rI64LtS),
      340 => Some(Self::S2rI64LeS),
      341 => Some(Self::S2rI64GtS),
      342 => Some(Self::S2rI64GeS),
      343 => Some(Self::S2rF32Eq),
      344 => Some(Self::S2rF32Ne),
      345 => Some(Self::S2rF32Lt),
      346 => Some(Self::S2rF32Le),
      347 => Some(Self::S2rF32Gt),
      348 => Some(Self::S2rF32Ge),
      349 => Some(Self::S2rF64Eq),
      350 => Some(Self::S2rF64Ne),
      351 => Some(Self::S2rF64Lt),
      352 => Some(Self::S2rF64Le),
      353 => Some(Self::S2rF64Gt),
      354 => Some(Self::S2rF64Ge),
      355 => Some(Self::S2sI32Eq),
      356 => Some(Self::S2sI32Ne),
      357 => Some(Self::S2sI32LtU),
      358 => Some(Self::S2sI32LeU),
      359 => Some(Self::S2sI32GtU),
      360 => Some(Self::S2sI32GeU),
      361 => Some(Self::S2sI32LtS),
      362 => Some(Self::S2sI32LeS),
      363 => Some(Self::S2sI32GtS),
      364 => Some(Self::S2sI32GeS),
      365 => Some(Self::S2sI64Eq),
      366 => Some(Self::S2sI64Ne),
      367 => Some(Self::S2sI64LtU),
      368 => Some(Self::S2sI64LeU),
      369 => Some(Self::S2sI64GtU),
      370 => Some(Self::S2sI64GeU),
      371 => Some(Self::S2sI64LtS),
      372 => Some(Self::S2sI64LeS),
      373 => Some(Self::S2sI64GtS),
      374 => Some(Self::S2sI64GeS),
      375 => Some(Self::S2sF32Eq),
      376 => Some(Self::S2sF32Ne),
      377 => Some(Self::S2sF32Lt),
      378 => Some(Self::S2sF32Le),
      379 => Some(Self::S2sF32Gt),
      380 => Some(Self::S2sF32Ge),
      381 => Some(Self::S2sF64Eq),
      382 => Some(Self::S2sF64Ne),
      383 => Some(Self::S2sF64Lt),
      384 => Some(Self::S2sF64Le),
      385 => Some(Self::S2sF64Gt),
      386 => Some(Self::S2sF64Ge),
      387 => Some(Self::R2rI32Shl),
      388 => Some(Self::R2rI32ShrU),
      389 => Some(Self::R2rI32ShrS),
      390 => Some(Self::R2rI64Shl),
      391 => Some(Self::R2rI64ShrU),
      392 => Some(Self::R2rI64ShrS),
      393 => Some(Self::R2rI32Rol),
      394 => Some(Self::R2rI32Ror),
      395 => Some(Self::R2rI64Rol),
      396 => Some(Self::R2rI64Ror),
      397 => Some(Self::R2rF32Min),
      398 => Some(Self::R2rF32Max),
      399 => Some(Self::R2rF64Min),
      400 => Some(Self::R2rF64Max),
      401 => Some(Self::R2rF32CopySign),
      402 => Some(Self::R2rF64CopySign),
      403 => Some(Self::R2sI32Shl),
      404 => Some(Self::R2sI32ShrU),
      405 => Some(Self::R2sI32ShrS),
      406 => Some(Self::R2sI64Shl),
      407 => Some(Self::R2sI64ShrU),
      408 => Some(Self::R2sI64ShrS),
      409 => Some(Self::R2sI32Rol),
      410 => Some(Self::R2sI32Ror),
      411 => Some(Self::R2sI64Rol),
      412 => Some(Self::R2sI64Ror),
      413 => Some(Self::R2sF32Min),
      414 => Some(Self::R2sF32Max),
      415 => Some(Self::R2sF64Min),
      416 => Some(Self::R2sF64Max),
      417 => Some(Self::R2sF32CopySign),
      418 => Some(Self::R2sF64CopySign),
      419 => Some(Self::S2rI32Shl),
      420 => Some(Self::S2rI32ShrU),
      421 => Some(Self::S2rI32ShrS),
      422 => Some(Self::S2rI64Shl),
      423 => Some(Self::S2rI64ShrU),
      424 => Some(Self::S2rI64ShrS),
      425 => Some(Self::S2rI32Rol),
      426 => Some(Self::S2rI32Ror),
      427 => Some(Self::S2rI64Rol),
      428 => Some(Self::S2rI64Ror),
      429 => Some(Self::S2rF32Min),
      430 => Some(Self::S2rF32Max),
      431 => Some(Self::S2rF64Min),
      432 => Some(Self::S2rF64Max),
      433 => Some(Self::S2rF32CopySign),
      434 => Some(Self::S2rF64CopySign),
      435 => Some(Self::S2sI32Shl),
      436 => Some(Self::S2sI32ShrU),
      437 => Some(Self::S2sI32ShrS),
      438 => Some(Self::S2sI64Shl),
      439 => Some(Self::S2sI64ShrU),
      440 => Some(Self::S2sI64ShrS),
      441 => Some(Self::S2sI32Rol),
      442 => Some(Self::S2sI32Ror),
      443 => Some(Self::S2sI64Rol),
      444 => Some(Self::S2sI64Ror),
      445 => Some(Self::S2sF32Min),
      446 => Some(Self::S2sF32Max),
      447 => Some(Self::S2sF64Min),
      448 => Some(Self::S2sF64Max),
      449 => Some(Self::S2sF32CopySign),
      450 => Some(Self::S2sF64CopySign),
      451 => Some(Self::R2rF32Abs),
      452 => Some(Self::R2rF32Neg),
      453 => Some(Self::R2rF32Ceil),
      454 => Some(Self::R2rF32Floor),
      455 => Some(Self::R2rF32Trunc),
      456 => Some(Self::R2rF32NearestInt),
      457 => Some(Self::R2rF32Sqrt),
      458 => Some(Self::R2rF64Abs),
      459 => Some(Self::R2rF64Neg),
      460 => Some(Self::R2rF64Ceil),
      461 => Some(Self::R2rF64Floor),
      462 => Some(Self::R2rF64Trunc),
      463 => Some(Self::R2rF64NearestInt),
      464 => Some(Self::R2rF64Sqrt),
      465 => Some(Self::R2sF32Abs),
      466 => Some(Self::R2sF32Neg),
      467 => Some(Self::R2sF32Ceil),
      468 => Some(Self::R2sF32Floor),
      469 => Some(Self::R2sF32Trunc),
      470 => Some(Self::R2sF32NearestInt),
      471 => Some(Self::R2sF32Sqrt),
      472 => Some(Self::R2sF64Abs),
      473 => Some(Self::R2sF64Neg),
      474 => Some(Self::R2sF64Ceil),
      475 => Some(Self::R2sF64Floor),
      476 => Some(Self::R2sF64Trunc),
      477 => Some(Self::R2sF64NearestInt),
      478 => Some(Self::R2sF64Sqrt),
      479 => Some(Self::S2rF32Abs),
      480 => Some(Self::S2rF32Neg),
      481 => Some(Self::S2rF32Ceil),
      482 => Some(Self::S2rF32Floor),
      483 => Some(Self::S2rF32Trunc),
      484 => Some(Self::S2rF32NearestInt),
      485 => Some(Self::S2rF32Sqrt),
      486 => Some(Self::S2rF64Abs),
      487 => Some(Self::S2rF64Neg),
      488 => Some(Self::S2rF64Ceil),
      489 => Some(Self::S2rF64Floor),
      490 => Some(Self::S2rF64Trunc),
      491 => Some(Self::S2rF64NearestInt),
      492 => Some(Self::S2rF64Sqrt),
      493 => Some(Self::S2sF32Abs),
      494 => Some(Self::S2sF32Neg),
      495 => Some(Self::S2sF32Ceil),
      496 => Some(Self::S2sF32Floor),
      497 => Some(Self::S2sF32Trunc),
      498 => Some(Self::S2sF32NearestInt),
      499 => Some(Self::S2sF32Sqrt),
      500 => Some(Self::S2sF64Abs),
      501 => Some(Self::S2sF64Neg),
      502 => Some(Self::S2sF64Ceil),
      503 => Some(Self::S2sF64Floor),
      504 => Some(Self::S2sF64Trunc),
      505 => Some(Self::S2sF64NearestInt),
      506 => Some(Self::S2sF64Sqrt),
      507 => Some(Self::R2rI32ConvertI64),
      508 => Some(Self::R2rI64SConvertF32),
      509 => Some(Self::R2rI64SConvertF64),
      510 => Some(Self::R2rI64UConvertF32),
      511 => Some(Self::R2rI64UConvertF64),
      512 => Some(Self::R2rI32SConvertF32),
      513 => Some(Self::R2rI32UConvertF32),
      514 => Some(Self::R2rI32SConvertF64),
      515 => Some(Self::R2rI32UConvertF64),
      516 => Some(Self::R2rI64SConvertI32),
      517 => Some(Self::R2rI64UConvertI32),
      518 => Some(Self::R2rF32SConvertI32),
      519 => Some(Self::R2rF32UConvertI32),
      520 => Some(Self::R2rF32SConvertI64),
      521 => Some(Self::R2rF32UConvertI64),
      522 => Some(Self::R2rF32ConvertF64),
      523 => Some(Self::R2rF64SConvertI32),
      524 => Some(Self::R2rF64UConvertI32),
      525 => Some(Self::R2rF64SConvertI64),
      526 => Some(Self::R2rF64UConvertI64),
      527 => Some(Self::R2rF64ConvertF32),
      528 => Some(Self::R2sI32ConvertI64),
      529 => Some(Self::R2sI64SConvertF32),
      530 => Some(Self::R2sI64SConvertF64),
      531 => Some(Self::R2sI64UConvertF32),
      532 => Some(Self::R2sI64UConvertF64),
      533 => Some(Self::R2sI32SConvertF32),
      534 => Some(Self::R2sI32UConvertF32),
      535 => Some(Self::R2sI32SConvertF64),
      536 => Some(Self::R2sI32UConvertF64),
      537 => Some(Self::R2sI64SConvertI32),
      538 => Some(Self::R2sI64UConvertI32),
      539 => Some(Self::R2sF32SConvertI32),
      540 => Some(Self::R2sF32UConvertI32),
      541 => Some(Self::R2sF32SConvertI64),
      542 => Some(Self::R2sF32UConvertI64),
      543 => Some(Self::R2sF32ConvertF64),
      544 => Some(Self::R2sF64SConvertI32),
      545 => Some(Self::R2sF64UConvertI32),
      546 => Some(Self::R2sF64SConvertI64),
      547 => Some(Self::R2sF64UConvertI64),
      548 => Some(Self::R2sF64ConvertF32),
      549 => Some(Self::S2rI32ConvertI64),
      550 => Some(Self::S2rI64SConvertF32),
      551 => Some(Self::S2rI64SConvertF64),
      552 => Some(Self::S2rI64UConvertF32),
      553 => Some(Self::S2rI64UConvertF64),
      554 => Some(Self::S2rI32SConvertF32),
      555 => Some(Self::S2rI32UConvertF32),
      556 => Some(Self::S2rI32SConvertF64),
      557 => Some(Self::S2rI32UConvertF64),
      558 => Some(Self::S2rI64SConvertI32),
      559 => Some(Self::S2rI64UConvertI32),
      560 => Some(Self::S2rF32SConvertI32),
      561 => Some(Self::S2rF32UConvertI32),
      562 => Some(Self::S2rF32SConvertI64),
      563 => Some(Self::S2rF32UConvertI64),
      564 => Some(Self::S2rF32ConvertF64),
      565 => Some(Self::S2rF64SConvertI32),
      566 => Some(Self::S2rF64UConvertI32),
      567 => Some(Self::S2rF64SConvertI64),
      568 => Some(Self::S2rF64UConvertI64),
      569 => Some(Self::S2rF64ConvertF32),
      570 => Some(Self::S2sI32ConvertI64),
      571 => Some(Self::S2sI64SConvertF32),
      572 => Some(Self::S2sI64SConvertF64),
      573 => Some(Self::S2sI64UConvertF32),
      574 => Some(Self::S2sI64UConvertF64),
      575 => Some(Self::S2sI32SConvertF32),
      576 => Some(Self::S2sI32UConvertF32),
      577 => Some(Self::S2sI32SConvertF64),
      578 => Some(Self::S2sI32UConvertF64),
      579 => Some(Self::S2sI64SConvertI32),
      580 => Some(Self::S2sI64UConvertI32),
      581 => Some(Self::S2sF32SConvertI32),
      582 => Some(Self::S2sF32UConvertI32),
      583 => Some(Self::S2sF32SConvertI64),
      584 => Some(Self::S2sF32UConvertI64),
      585 => Some(Self::S2sF32ConvertF64),
      586 => Some(Self::S2sF64SConvertI32),
      587 => Some(Self::S2sF64UConvertI32),
      588 => Some(Self::S2sF64SConvertI64),
      589 => Some(Self::S2sF64UConvertI64),
      590 => Some(Self::S2sF64ConvertF32),
      591 => Some(Self::R2rF32ReinterpretI32),
      592 => Some(Self::R2rF64ReinterpretI64),
      593 => Some(Self::R2rI32ReinterpretF32),
      594 => Some(Self::R2rI64ReinterpretF64),
      595 => Some(Self::R2sF32ReinterpretI32),
      596 => Some(Self::R2sF64ReinterpretI64),
      597 => Some(Self::R2sI32ReinterpretF32),
      598 => Some(Self::R2sI64ReinterpretF64),
      599 => Some(Self::S2rF32ReinterpretI32),
      600 => Some(Self::S2rF64ReinterpretI64),
      601 => Some(Self::S2rI32ReinterpretF32),
      602 => Some(Self::S2rI64ReinterpretF64),
      603 => Some(Self::S2sF32ReinterpretI32),
      604 => Some(Self::S2sF64ReinterpretI64),
      605 => Some(Self::S2sI32ReinterpretF32),
      606 => Some(Self::S2sI64ReinterpretF64),
      607 => Some(Self::R2rI32Clz),
      608 => Some(Self::R2rI32Ctz),
      609 => Some(Self::R2rI32Popcnt),
      610 => Some(Self::R2rI32Eqz),
      611 => Some(Self::R2rI64Clz),
      612 => Some(Self::R2rI64Ctz),
      613 => Some(Self::R2rI64Popcnt),
      614 => Some(Self::R2rI64Eqz),
      615 => Some(Self::R2sI32Clz),
      616 => Some(Self::R2sI32Ctz),
      617 => Some(Self::R2sI32Popcnt),
      618 => Some(Self::R2sI32Eqz),
      619 => Some(Self::R2sI64Clz),
      620 => Some(Self::R2sI64Ctz),
      621 => Some(Self::R2sI64Popcnt),
      622 => Some(Self::R2sI64Eqz),
      623 => Some(Self::S2rI32Clz),
      624 => Some(Self::S2rI32Ctz),
      625 => Some(Self::S2rI32Popcnt),
      626 => Some(Self::S2rI32Eqz),
      627 => Some(Self::S2rI64Clz),
      628 => Some(Self::S2rI64Ctz),
      629 => Some(Self::S2rI64Popcnt),
      630 => Some(Self::S2rI64Eqz),
      631 => Some(Self::S2sI32Clz),
      632 => Some(Self::S2sI32Ctz),
      633 => Some(Self::S2sI32Popcnt),
      634 => Some(Self::S2sI32Eqz),
      635 => Some(Self::S2sI64Clz),
      636 => Some(Self::S2sI64Ctz),
      637 => Some(Self::S2sI64Popcnt),
      638 => Some(Self::S2sI64Eqz),
      639 => Some(Self::R2rI32SExtendI8),
      640 => Some(Self::R2rI32SExtendI16),
      641 => Some(Self::R2rI64SExtendI8),
      642 => Some(Self::R2rI64SExtendI16),
      643 => Some(Self::R2rI64SExtendI32),
      644 => Some(Self::R2sI32SExtendI8),
      645 => Some(Self::R2sI32SExtendI16),
      646 => Some(Self::R2sI64SExtendI8),
      647 => Some(Self::R2sI64SExtendI16),
      648 => Some(Self::R2sI64SExtendI32),
      649 => Some(Self::S2rI32SExtendI8),
      650 => Some(Self::S2rI32SExtendI16),
      651 => Some(Self::S2rI64SExtendI8),
      652 => Some(Self::S2rI64SExtendI16),
      653 => Some(Self::S2rI64SExtendI32),
      654 => Some(Self::S2sI32SExtendI8),
      655 => Some(Self::S2sI32SExtendI16),
      656 => Some(Self::S2sI64SExtendI8),
      657 => Some(Self::S2sI64SExtendI16),
      658 => Some(Self::S2sI64SExtendI32),
      659 => Some(Self::R2rI32SConvertSatF32),
      660 => Some(Self::R2rI32UConvertSatF32),
      661 => Some(Self::R2rI32SConvertSatF64),
      662 => Some(Self::R2rI32UConvertSatF64),
      663 => Some(Self::R2rI64SConvertSatF32),
      664 => Some(Self::R2rI64UConvertSatF32),
      665 => Some(Self::R2rI64SConvertSatF64),
      666 => Some(Self::R2rI64UConvertSatF64),
      667 => Some(Self::R2sI32SConvertSatF32),
      668 => Some(Self::R2sI32UConvertSatF32),
      669 => Some(Self::R2sI32SConvertSatF64),
      670 => Some(Self::R2sI32UConvertSatF64),
      671 => Some(Self::R2sI64SConvertSatF32),
      672 => Some(Self::R2sI64UConvertSatF32),
      673 => Some(Self::R2sI64SConvertSatF64),
      674 => Some(Self::R2sI64UConvertSatF64),
      675 => Some(Self::S2rI32SConvertSatF32),
      676 => Some(Self::S2rI32UConvertSatF32),
      677 => Some(Self::S2rI32SConvertSatF64),
      678 => Some(Self::S2rI32UConvertSatF64),
      679 => Some(Self::S2rI64SConvertSatF32),
      680 => Some(Self::S2rI64UConvertSatF32),
      681 => Some(Self::S2rI64SConvertSatF64),
      682 => Some(Self::S2rI64UConvertSatF64),
      683 => Some(Self::S2sI32SConvertSatF32),
      684 => Some(Self::S2sI32UConvertSatF32),
      685 => Some(Self::S2sI32SConvertSatF64),
      686 => Some(Self::S2sI32UConvertSatF64),
      687 => Some(Self::S2sI64SConvertSatF32),
      688 => Some(Self::S2sI64UConvertSatF32),
      689 => Some(Self::S2sI64SConvertSatF64),
      690 => Some(Self::S2sI64UConvertSatF64),
      691 => Some(Self::S2sMemoryGrow),
      692 => Some(Self::S2sMemory64Grow),
      693 => Some(Self::S2sMemorySize),
      694 => Some(Self::S2sMemory64Size),
      695 => Some(Self::S2sReturn),
      696 => Some(Self::S2sBranch),
      697 => Some(Self::R2sBranchIf),
      698 => Some(Self::S2sBranchIf),
      699 => Some(Self::R2sBranchIfWithParams),
      700 => Some(Self::S2sBranchIfWithParams),
      701 => Some(Self::R2sIf),
      702 => Some(Self::S2sIf),
      703 => Some(Self::S2sElse),
      704 => Some(Self::S2sCallFunction),
      705 => Some(Self::S2sReturnCall),
      706 => Some(Self::S2sCallImportedFunction),
      707 => Some(Self::S2sReturnCallImportedFunction),
      708 => Some(Self::S2sCallIndirect),
      709 => Some(Self::S2sCallIndirect64),
      710 => Some(Self::S2sReturnCallIndirect),
      711 => Some(Self::S2sReturnCallIndirect64),
      712 => Some(Self::R2sBrTable),
      713 => Some(Self::S2sBrTable),
      714 => Some(Self::S2sCopySlotMulti),
      715 => Some(Self::S2sCopySlotll),
      716 => Some(Self::S2sCopySlotlq),
      717 => Some(Self::S2sCopySlotql),
      718 => Some(Self::S2sCopySlotqq),
      719 => Some(Self::S2sCopySlot32),
      720 => Some(Self::S2sCopySlot32x2),
      721 => Some(Self::S2sCopySlot64),
      722 => Some(Self::S2sCopySlot64x2),
      723 => Some(Self::S2sCopySlot128),
      724 => Some(Self::S2sCopySlotRef),
      725 => Some(Self::S2sPreserveCopySlot32),
      726 => Some(Self::S2sPreserveCopySlot64),
      727 => Some(Self::S2sPreserveCopySlot128),
      728 => Some(Self::S2sPreserveCopySlotRef),
      729 => Some(Self::R2sCopyR0ToSlot32),
      730 => Some(Self::R2sCopyR0ToSlot64),
      731 => Some(Self::R2sCopyFp0ToSlot32),
      732 => Some(Self::R2sCopyFp0ToSlot64),
      733 => Some(Self::R2sPreserveCopyR0ToSlot32),
      734 => Some(Self::R2sPreserveCopyR0ToSlot64),
      735 => Some(Self::R2sPreserveCopyFp0ToSlot32),
      736 => Some(Self::R2sPreserveCopyFp0ToSlot64),
      737 => Some(Self::S2sRefNull),
      738 => Some(Self::S2sRefIsNull),
      739 => Some(Self::S2sRefFunc),
      740 => Some(Self::S2sRefEq),
      741 => Some(Self::S2sMemoryInit),
      742 => Some(Self::S2sMemory64Init),
      743 => Some(Self::S2sDataDrop),
      744 => Some(Self::S2sMemoryCopy),
      745 => Some(Self::S2sMemoryCopyMem64),
      746 => Some(Self::S2sMemoryCopyIdx64Idx32),
      747 => Some(Self::S2sMemoryCopyIdx32Idx64),
      748 => Some(Self::S2sMemoryFill),
      749 => Some(Self::S2sMemory64Fill),
      750 => Some(Self::S2sTableGet),
      751 => Some(Self::S2sTable64Get),
      752 => Some(Self::S2sTableSet),
      753 => Some(Self::S2sTable64Set),
      754 => Some(Self::S2sTableInit),
      755 => Some(Self::S2sTable64Init),
      756 => Some(Self::S2sElemDrop),
      757 => Some(Self::S2sTableCopy),
      758 => Some(Self::S2sTable64Copy326432),
      759 => Some(Self::S2sTable64Copy643232),
      760 => Some(Self::S2sTable64Copy646464),
      761 => Some(Self::S2sTableGrow),
      762 => Some(Self::S2sTable64Grow),
      763 => Some(Self::S2sTableSize),
      764 => Some(Self::S2sTable64Size),
      765 => Some(Self::S2sTableFill),
      766 => Some(Self::S2sTable64Fill),
      767 => Some(Self::S2sUnreachable),
      768 => Some(Self::S2sUnwind),
      769 => Some(Self::S2sOnLoopBegin),
      770 => Some(Self::S2sOnLoopBeginNoRefSlots),
      771 => Some(Self::S2sThrow),
      772 => Some(Self::S2sRethrow),
      773 => Some(Self::S2sCatch),
      774 => Some(Self::S2sAtomicFence),
      775 => Some(Self::S2sSimdF64x2Splat),
      776 => Some(Self::S2sSimdF32x4Splat),
      777 => Some(Self::S2sSimdI64x2Splat),
      778 => Some(Self::S2sSimdI32x4Splat),
      779 => Some(Self::S2sSimdI16x8Splat),
      780 => Some(Self::S2sSimdI8x16Splat),
      781 => Some(Self::S2sSimdF64x2ExtractLane),
      782 => Some(Self::S2sSimdF32x4ExtractLane),
      783 => Some(Self::S2sSimdI64x2ExtractLane),
      784 => Some(Self::S2sSimdI32x4ExtractLane),
      785 => Some(Self::S2sSimdI16x8ExtractLaneS),
      786 => Some(Self::S2sSimdI16x8ExtractLaneU),
      787 => Some(Self::S2sSimdI8x16ExtractLaneS),
      788 => Some(Self::S2sSimdI8x16ExtractLaneU),
      789 => Some(Self::S2sSimdF64x2Add),
      790 => Some(Self::S2sSimdF64x2Sub),
      791 => Some(Self::S2sSimdF64x2Mul),
      792 => Some(Self::S2sSimdF64x2Div),
      793 => Some(Self::S2sSimdF64x2Min),
      794 => Some(Self::S2sSimdF64x2Max),
      795 => Some(Self::S2sSimdF64x2Pmin),
      796 => Some(Self::S2sSimdF64x2Pmax),
      797 => Some(Self::S2sSimdF32x4RelaxedMin),
      798 => Some(Self::S2sSimdF32x4RelaxedMax),
      799 => Some(Self::S2sSimdF64x2RelaxedMin),
      800 => Some(Self::S2sSimdF64x2RelaxedMax),
      801 => Some(Self::S2sSimdF32x4Add),
      802 => Some(Self::S2sSimdF32x4Sub),
      803 => Some(Self::S2sSimdF32x4Mul),
      804 => Some(Self::S2sSimdF32x4Div),
      805 => Some(Self::S2sSimdF32x4Min),
      806 => Some(Self::S2sSimdF32x4Max),
      807 => Some(Self::S2sSimdF32x4Pmin),
      808 => Some(Self::S2sSimdF32x4Pmax),
      809 => Some(Self::S2sSimdI64x2Add),
      810 => Some(Self::S2sSimdI64x2Sub),
      811 => Some(Self::S2sSimdI64x2Mul),
      812 => Some(Self::S2sSimdI32x4Add),
      813 => Some(Self::S2sSimdI32x4Sub),
      814 => Some(Self::S2sSimdI32x4Mul),
      815 => Some(Self::S2sSimdI32x4MinS),
      816 => Some(Self::S2sSimdI32x4MinU),
      817 => Some(Self::S2sSimdI32x4MaxS),
      818 => Some(Self::S2sSimdI32x4MaxU),
      819 => Some(Self::S2sSimdS128And),
      820 => Some(Self::S2sSimdS128Or),
      821 => Some(Self::S2sSimdS128Xor),
      822 => Some(Self::S2sSimdS128AndNot),
      823 => Some(Self::S2sSimdI16x8Add),
      824 => Some(Self::S2sSimdI16x8Sub),
      825 => Some(Self::S2sSimdI16x8Mul),
      826 => Some(Self::S2sSimdI16x8MinS),
      827 => Some(Self::S2sSimdI16x8MinU),
      828 => Some(Self::S2sSimdI16x8MaxS),
      829 => Some(Self::S2sSimdI16x8MaxU),
      830 => Some(Self::S2sSimdI16x8AddSatS),
      831 => Some(Self::S2sSimdI16x8AddSatU),
      832 => Some(Self::S2sSimdI16x8SubSatS),
      833 => Some(Self::S2sSimdI16x8SubSatU),
      834 => Some(Self::S2sSimdI16x8RoundingAverageU),
      835 => Some(Self::S2sSimdI16x8Q15MulRSatS),
      836 => Some(Self::S2sSimdI16x8RelaxedQ15MulRS),
      837 => Some(Self::S2sSimdI8x16Add),
      838 => Some(Self::S2sSimdI8x16Sub),
      839 => Some(Self::S2sSimdI8x16MinS),
      840 => Some(Self::S2sSimdI8x16MinU),
      841 => Some(Self::S2sSimdI8x16MaxS),
      842 => Some(Self::S2sSimdI8x16MaxU),
      843 => Some(Self::S2sSimdI8x16AddSatS),
      844 => Some(Self::S2sSimdI8x16AddSatU),
      845 => Some(Self::S2sSimdI8x16SubSatS),
      846 => Some(Self::S2sSimdI8x16SubSatU),
      847 => Some(Self::S2sSimdI8x16RoundingAverageU),
      848 => Some(Self::S2sSimdF64x2Abs),
      849 => Some(Self::S2sSimdF64x2Neg),
      850 => Some(Self::S2sSimdF64x2Sqrt),
      851 => Some(Self::S2sSimdF64x2Ceil),
      852 => Some(Self::S2sSimdF64x2Floor),
      853 => Some(Self::S2sSimdF64x2Trunc),
      854 => Some(Self::S2sSimdF64x2NearestInt),
      855 => Some(Self::S2sSimdF32x4Abs),
      856 => Some(Self::S2sSimdF32x4Neg),
      857 => Some(Self::S2sSimdF32x4Sqrt),
      858 => Some(Self::S2sSimdF32x4Ceil),
      859 => Some(Self::S2sSimdF32x4Floor),
      860 => Some(Self::S2sSimdF32x4Trunc),
      861 => Some(Self::S2sSimdF32x4NearestInt),
      862 => Some(Self::S2sSimdI64x2Neg),
      863 => Some(Self::S2sSimdI32x4Neg),
      864 => Some(Self::S2sSimdI64x2Abs),
      865 => Some(Self::S2sSimdI32x4Abs),
      866 => Some(Self::S2sSimdS128Not),
      867 => Some(Self::S2sSimdI16x8Neg),
      868 => Some(Self::S2sSimdI16x8Abs),
      869 => Some(Self::S2sSimdI8x16Neg),
      870 => Some(Self::S2sSimdI8x16Abs),
      871 => Some(Self::S2sSimdI8x16Popcnt),
      872 => Some(Self::S2sSimdI8x16BitMask),
      873 => Some(Self::S2sSimdI16x8BitMask),
      874 => Some(Self::S2sSimdI32x4BitMask),
      875 => Some(Self::S2sSimdI64x2BitMask),
      876 => Some(Self::S2sSimdF64x2Eq),
      877 => Some(Self::S2sSimdF64x2Ne),
      878 => Some(Self::S2sSimdF64x2Gt),
      879 => Some(Self::S2sSimdF64x2Ge),
      880 => Some(Self::S2sSimdF64x2Lt),
      881 => Some(Self::S2sSimdF64x2Le),
      882 => Some(Self::S2sSimdF32x4Eq),
      883 => Some(Self::S2sSimdF32x4Ne),
      884 => Some(Self::S2sSimdF32x4Gt),
      885 => Some(Self::S2sSimdF32x4Ge),
      886 => Some(Self::S2sSimdF32x4Lt),
      887 => Some(Self::S2sSimdF32x4Le),
      888 => Some(Self::S2sSimdI64x2Eq),
      889 => Some(Self::S2sSimdI64x2Ne),
      890 => Some(Self::S2sSimdI64x2LtS),
      891 => Some(Self::S2sSimdI64x2GtS),
      892 => Some(Self::S2sSimdI64x2LeS),
      893 => Some(Self::S2sSimdI64x2GeS),
      894 => Some(Self::S2sSimdI32x4Eq),
      895 => Some(Self::S2sSimdI32x4Ne),
      896 => Some(Self::S2sSimdI32x4GtS),
      897 => Some(Self::S2sSimdI32x4GeS),
      898 => Some(Self::S2sSimdI32x4LtS),
      899 => Some(Self::S2sSimdI32x4LeS),
      900 => Some(Self::S2sSimdI32x4GtU),
      901 => Some(Self::S2sSimdI32x4GeU),
      902 => Some(Self::S2sSimdI32x4LtU),
      903 => Some(Self::S2sSimdI32x4LeU),
      904 => Some(Self::S2sSimdI16x8Eq),
      905 => Some(Self::S2sSimdI16x8Ne),
      906 => Some(Self::S2sSimdI16x8GtS),
      907 => Some(Self::S2sSimdI16x8GeS),
      908 => Some(Self::S2sSimdI16x8LtS),
      909 => Some(Self::S2sSimdI16x8LeS),
      910 => Some(Self::S2sSimdI16x8GtU),
      911 => Some(Self::S2sSimdI16x8GeU),
      912 => Some(Self::S2sSimdI16x8LtU),
      913 => Some(Self::S2sSimdI16x8LeU),
      914 => Some(Self::S2sSimdI8x16Eq),
      915 => Some(Self::S2sSimdI8x16Ne),
      916 => Some(Self::S2sSimdI8x16GtS),
      917 => Some(Self::S2sSimdI8x16GeS),
      918 => Some(Self::S2sSimdI8x16LtS),
      919 => Some(Self::S2sSimdI8x16LeS),
      920 => Some(Self::S2sSimdI8x16GtU),
      921 => Some(Self::S2sSimdI8x16GeU),
      922 => Some(Self::S2sSimdI8x16LtU),
      923 => Some(Self::S2sSimdI8x16LeU),
      924 => Some(Self::S2sSimdF64x2ReplaceLane),
      925 => Some(Self::S2sSimdF32x4ReplaceLane),
      926 => Some(Self::S2sSimdI64x2ReplaceLane),
      927 => Some(Self::S2sSimdI32x4ReplaceLane),
      928 => Some(Self::S2sSimdI16x8ReplaceLane),
      929 => Some(Self::S2sSimdI8x16ReplaceLane),
      930 => Some(Self::S2sSimdS128LoadMem),
      931 => Some(Self::S2sSimdS128LoadMemMem64),
      932 => Some(Self::S2sSimdS128LoadMemMultiMem),
      933 => Some(Self::S2sSimdS128LoadMemMultiMem64),
      934 => Some(Self::S2sSimdS128StoreMem),
      935 => Some(Self::S2sSimdS128StoreMemMem64),
      936 => Some(Self::S2sSimdS128StoreMemMultiMem),
      937 => Some(Self::S2sSimdS128StoreMemMultiMem64),
      938 => Some(Self::S2sSimdI64x2Shl),
      939 => Some(Self::S2sSimdI64x2ShrS),
      940 => Some(Self::S2sSimdI64x2ShrU),
      941 => Some(Self::S2sSimdI32x4Shl),
      942 => Some(Self::S2sSimdI32x4ShrS),
      943 => Some(Self::S2sSimdI32x4ShrU),
      944 => Some(Self::S2sSimdI16x8Shl),
      945 => Some(Self::S2sSimdI16x8ShrS),
      946 => Some(Self::S2sSimdI16x8ShrU),
      947 => Some(Self::S2sSimdI8x16Shl),
      948 => Some(Self::S2sSimdI8x16ShrS),
      949 => Some(Self::S2sSimdI8x16ShrU),
      950 => Some(Self::S2sSimdI16x8ExtMulLowI8x16S),
      951 => Some(Self::S2sSimdI16x8ExtMulHighI8x16S),
      952 => Some(Self::S2sSimdI16x8ExtMulLowI8x16U),
      953 => Some(Self::S2sSimdI16x8ExtMulHighI8x16U),
      954 => Some(Self::S2sSimdI32x4ExtMulLowI16x8S),
      955 => Some(Self::S2sSimdI32x4ExtMulHighI16x8S),
      956 => Some(Self::S2sSimdI32x4ExtMulLowI16x8U),
      957 => Some(Self::S2sSimdI32x4ExtMulHighI16x8U),
      958 => Some(Self::S2sSimdI64x2ExtMulLowI32x4S),
      959 => Some(Self::S2sSimdI64x2ExtMulHighI32x4S),
      960 => Some(Self::S2sSimdI64x2ExtMulLowI32x4U),
      961 => Some(Self::S2sSimdI64x2ExtMulHighI32x4U),
      962 => Some(Self::S2sSimdF32x4SConvertI32x4),
      963 => Some(Self::S2sSimdF32x4UConvertI32x4),
      964 => Some(Self::S2sSimdI32x4SConvertF32x4),
      965 => Some(Self::S2sSimdI32x4UConvertF32x4),
      966 => Some(Self::S2sSimdI32x4RelaxedTruncF32x4S),
      967 => Some(Self::S2sSimdI32x4RelaxedTruncF32x4U),
      968 => Some(Self::S2sSimdI64x2SConvertI32x4Low),
      969 => Some(Self::S2sSimdI64x2SConvertI32x4High),
      970 => Some(Self::S2sSimdI64x2UConvertI32x4Low),
      971 => Some(Self::S2sSimdI64x2UConvertI32x4High),
      972 => Some(Self::S2sSimdI32x4SConvertI16x8High),
      973 => Some(Self::S2sSimdI32x4UConvertI16x8High),
      974 => Some(Self::S2sSimdI32x4SConvertI16x8Low),
      975 => Some(Self::S2sSimdI32x4UConvertI16x8Low),
      976 => Some(Self::S2sSimdI16x8SConvertI8x16High),
      977 => Some(Self::S2sSimdI16x8UConvertI8x16High),
      978 => Some(Self::S2sSimdI16x8SConvertI8x16Low),
      979 => Some(Self::S2sSimdI16x8UConvertI8x16Low),
      980 => Some(Self::S2sSimdF64x2ConvertLowI32x4S),
      981 => Some(Self::S2sSimdF64x2ConvertLowI32x4U),
      982 => Some(Self::S2sSimdI32x4TruncSatF64x2SZero),
      983 => Some(Self::S2sSimdI32x4TruncSatF64x2UZero),
      984 => Some(Self::S2sSimdI32x4RelaxedTruncF64x2SZero),
      985 => Some(Self::S2sSimdI32x4RelaxedTruncF64x2UZero),
      986 => Some(Self::S2sSimdF32x4DemoteF64x2Zero),
      987 => Some(Self::S2sSimdF64x2PromoteLowF32x4),
      988 => Some(Self::S2sSimdI16x8SConvertI32x4),
      989 => Some(Self::S2sSimdI16x8UConvertI32x4),
      990 => Some(Self::S2sSimdI8x16SConvertI16x8),
      991 => Some(Self::S2sSimdI8x16UConvertI16x8),
      992 => Some(Self::S2sSimdI8x16RelaxedLaneSelect),
      993 => Some(Self::S2sSimdI16x8RelaxedLaneSelect),
      994 => Some(Self::S2sSimdI32x4RelaxedLaneSelect),
      995 => Some(Self::S2sSimdI64x2RelaxedLaneSelect),
      996 => Some(Self::S2sSimdS128Select),
      997 => Some(Self::S2sSimdI32x4DotI16x8S),
      998 => Some(Self::S2sSimdI16x8DotI8x16I7x16S),
      999 => Some(Self::S2sSimdI32x4DotI8x16I7x16AddS),
      1000 => Some(Self::S2sSimdI8x16RelaxedSwizzle),
      1001 => Some(Self::S2sSimdI8x16Swizzle),
      1002 => Some(Self::S2sSimdV128AnyTrue),
      1003 => Some(Self::S2sSimdI8x16Shuffle),
      1004 => Some(Self::S2sSimdI64x2AllTrue),
      1005 => Some(Self::S2sSimdI32x4AllTrue),
      1006 => Some(Self::S2sSimdI16x8AllTrue),
      1007 => Some(Self::S2sSimdI8x16AllTrue),
      1008 => Some(Self::S2sSimdF32x4Qfma),
      1009 => Some(Self::S2sSimdF32x4Qfms),
      1010 => Some(Self::S2sSimdF64x2Qfma),
      1011 => Some(Self::S2sSimdF64x2Qfms),
      1012 => Some(Self::S2sSimdS128Load8Splat),
      1013 => Some(Self::S2sSimdS128Load8SplatMem64),
      1014 => Some(Self::S2sSimdS128Load8SplatMultiMem),
      1015 => Some(Self::S2sSimdS128Load8SplatMultiMem64),
      1016 => Some(Self::S2sSimdS128Load16Splat),
      1017 => Some(Self::S2sSimdS128Load16SplatMem64),
      1018 => Some(Self::S2sSimdS128Load16SplatMultiMem),
      1019 => Some(Self::S2sSimdS128Load16SplatMultiMem64),
      1020 => Some(Self::S2sSimdS128Load32Splat),
      1021 => Some(Self::S2sSimdS128Load32SplatMem64),
      1022 => Some(Self::S2sSimdS128Load32SplatMultiMem),
      1023 => Some(Self::S2sSimdS128Load32SplatMultiMem64),
      1024 => Some(Self::S2sSimdS128Load64Splat),
      1025 => Some(Self::S2sSimdS128Load64SplatMem64),
      1026 => Some(Self::S2sSimdS128Load64SplatMultiMem),
      1027 => Some(Self::S2sSimdS128Load64SplatMultiMem64),
      1028 => Some(Self::S2sSimdS128Load8x8S),
      1029 => Some(Self::S2sSimdS128Load8x8SMem64),
      1030 => Some(Self::S2sSimdS128Load8x8SMultiMem),
      1031 => Some(Self::S2sSimdS128Load8x8SMultiMem64),
      1032 => Some(Self::S2sSimdS128Load8x8U),
      1033 => Some(Self::S2sSimdS128Load8x8UMem64),
      1034 => Some(Self::S2sSimdS128Load8x8UMultiMem),
      1035 => Some(Self::S2sSimdS128Load8x8UMultiMem64),
      1036 => Some(Self::S2sSimdS128Load16x4S),
      1037 => Some(Self::S2sSimdS128Load16x4SMem64),
      1038 => Some(Self::S2sSimdS128Load16x4SMultiMem),
      1039 => Some(Self::S2sSimdS128Load16x4SMultiMem64),
      1040 => Some(Self::S2sSimdS128Load16x4U),
      1041 => Some(Self::S2sSimdS128Load16x4UMem64),
      1042 => Some(Self::S2sSimdS128Load16x4UMultiMem),
      1043 => Some(Self::S2sSimdS128Load16x4UMultiMem64),
      1044 => Some(Self::S2sSimdS128Load32x2S),
      1045 => Some(Self::S2sSimdS128Load32x2SMem64),
      1046 => Some(Self::S2sSimdS128Load32x2SMultiMem),
      1047 => Some(Self::S2sSimdS128Load32x2SMultiMem64),
      1048 => Some(Self::S2sSimdS128Load32x2U),
      1049 => Some(Self::S2sSimdS128Load32x2UMem64),
      1050 => Some(Self::S2sSimdS128Load32x2UMultiMem),
      1051 => Some(Self::S2sSimdS128Load32x2UMultiMem64),
      1052 => Some(Self::S2sSimdS128Load32Zero),
      1053 => Some(Self::S2sSimdS128Load32ZeroMem64),
      1054 => Some(Self::S2sSimdS128Load32ZeroMultiMem),
      1055 => Some(Self::S2sSimdS128Load32ZeroMultiMem64),
      1056 => Some(Self::S2sSimdS128Load64Zero),
      1057 => Some(Self::S2sSimdS128Load64ZeroMem64),
      1058 => Some(Self::S2sSimdS128Load64ZeroMultiMem),
      1059 => Some(Self::S2sSimdS128Load64ZeroMultiMem64),
      1060 => Some(Self::S2sSimdS128Load8Lane),
      1061 => Some(Self::S2sSimdS128Load8LaneMem64),
      1062 => Some(Self::S2sSimdS128Load8LaneMultiMem),
      1063 => Some(Self::S2sSimdS128Load8LaneMultiMem64),
      1064 => Some(Self::S2sSimdS128Load16Lane),
      1065 => Some(Self::S2sSimdS128Load16LaneMem64),
      1066 => Some(Self::S2sSimdS128Load16LaneMultiMem),
      1067 => Some(Self::S2sSimdS128Load16LaneMultiMem64),
      1068 => Some(Self::S2sSimdS128Load32Lane),
      1069 => Some(Self::S2sSimdS128Load32LaneMem64),
      1070 => Some(Self::S2sSimdS128Load32LaneMultiMem),
      1071 => Some(Self::S2sSimdS128Load32LaneMultiMem64),
      1072 => Some(Self::S2sSimdS128Load64Lane),
      1073 => Some(Self::S2sSimdS128Load64LaneMem64),
      1074 => Some(Self::S2sSimdS128Load64LaneMultiMem),
      1075 => Some(Self::S2sSimdS128Load64LaneMultiMem64),
      1076 => Some(Self::S2sSimdS128Store8Lane),
      1077 => Some(Self::S2sSimdS128Store8LaneMem64),
      1078 => Some(Self::S2sSimdS128Store8LaneMultiMem),
      1079 => Some(Self::S2sSimdS128Store8LaneMultiMem64),
      1080 => Some(Self::S2sSimdS128Store16Lane),
      1081 => Some(Self::S2sSimdS128Store16LaneMem64),
      1082 => Some(Self::S2sSimdS128Store16LaneMultiMem),
      1083 => Some(Self::S2sSimdS128Store16LaneMultiMem64),
      1084 => Some(Self::S2sSimdS128Store32Lane),
      1085 => Some(Self::S2sSimdS128Store32LaneMem64),
      1086 => Some(Self::S2sSimdS128Store32LaneMultiMem),
      1087 => Some(Self::S2sSimdS128Store32LaneMultiMem64),
      1088 => Some(Self::S2sSimdS128Store64Lane),
      1089 => Some(Self::S2sSimdS128Store64LaneMem64),
      1090 => Some(Self::S2sSimdS128Store64LaneMultiMem),
      1091 => Some(Self::S2sSimdS128Store64LaneMultiMem64),
      1092 => Some(Self::S2sSimdI32x4ExtAddPairwiseI16x8S),
      1093 => Some(Self::S2sSimdI32x4ExtAddPairwiseI16x8U),
      1094 => Some(Self::S2sSimdI16x8ExtAddPairwiseI8x16S),
      1095 => Some(Self::S2sSimdI16x8ExtAddPairwiseI8x16U),
      1096 => Some(Self::S2sBranchOnNull),
      1097 => Some(Self::S2sBranchOnNullWithParams),
      1098 => Some(Self::S2sBranchOnNonNull),
      1099 => Some(Self::S2sBranchOnNonNullWithParams),
      1100 => Some(Self::S2sBranchOnCast),
      1101 => Some(Self::S2sBranchOnCastFail),
      1102 => Some(Self::S2sStructNew),
      1103 => Some(Self::S2sStructNewDefault),
      1104 => Some(Self::S2sI8SStructGet),
      1105 => Some(Self::S2sI8UStructGet),
      1106 => Some(Self::S2sI16SStructGet),
      1107 => Some(Self::S2sI16UStructGet),
      1108 => Some(Self::S2sI32StructGet),
      1109 => Some(Self::S2sI64StructGet),
      1110 => Some(Self::S2sF32StructGet),
      1111 => Some(Self::S2sF64StructGet),
      1112 => Some(Self::S2sS128StructGet),
      1113 => Some(Self::S2sRefStructGet),
      1114 => Some(Self::S2sI8StructSet),
      1115 => Some(Self::S2sI16StructSet),
      1116 => Some(Self::S2sI32StructSet),
      1117 => Some(Self::S2sI64StructSet),
      1118 => Some(Self::S2sF32StructSet),
      1119 => Some(Self::S2sF64StructSet),
      1120 => Some(Self::S2sS128StructSet),
      1121 => Some(Self::S2sRefStructSet),
      1122 => Some(Self::S2sI8ArrayNew),
      1123 => Some(Self::S2sI16ArrayNew),
      1124 => Some(Self::S2sI32ArrayNew),
      1125 => Some(Self::S2sI64ArrayNew),
      1126 => Some(Self::S2sF32ArrayNew),
      1127 => Some(Self::S2sF64ArrayNew),
      1128 => Some(Self::S2sS128ArrayNew),
      1129 => Some(Self::S2sRefArrayNew),
      1130 => Some(Self::S2sArrayNewDefault),
      1131 => Some(Self::S2sArrayNewFixed),
      1132 => Some(Self::S2sArrayNewData),
      1133 => Some(Self::S2sArrayNewElem),
      1134 => Some(Self::S2sArrayInitData),
      1135 => Some(Self::S2sArrayInitElem),
      1136 => Some(Self::S2sArrayLen),
      1137 => Some(Self::S2sArrayCopy),
      1138 => Some(Self::S2sI8SArrayGet),
      1139 => Some(Self::S2sI8UArrayGet),
      1140 => Some(Self::S2sI16SArrayGet),
      1141 => Some(Self::S2sI16UArrayGet),
      1142 => Some(Self::S2sI32ArrayGet),
      1143 => Some(Self::S2sI64ArrayGet),
      1144 => Some(Self::S2sF32ArrayGet),
      1145 => Some(Self::S2sF64ArrayGet),
      1146 => Some(Self::S2sS128ArrayGet),
      1147 => Some(Self::S2sRefArrayGet),
      1148 => Some(Self::S2sI8ArraySet),
      1149 => Some(Self::S2sI16ArraySet),
      1150 => Some(Self::S2sI32ArraySet),
      1151 => Some(Self::S2sI64ArraySet),
      1152 => Some(Self::S2sF32ArraySet),
      1153 => Some(Self::S2sF64ArraySet),
      1154 => Some(Self::S2sS128ArraySet),
      1155 => Some(Self::S2sRefArraySet),
      1156 => Some(Self::S2sI8ArrayFill),
      1157 => Some(Self::S2sI16ArrayFill),
      1158 => Some(Self::S2sI32ArrayFill),
      1159 => Some(Self::S2sI64ArrayFill),
      1160 => Some(Self::S2sF32ArrayFill),
      1161 => Some(Self::S2sF64ArrayFill),
      1162 => Some(Self::S2sS128ArrayFill),
      1163 => Some(Self::S2sRefArrayFill),
      1164 => Some(Self::S2sRefI31),
      1165 => Some(Self::S2sI31GetS),
      1166 => Some(Self::S2sI31GetU),
      1167 => Some(Self::S2sRefCast),
      1168 => Some(Self::S2sRefCastNull),
      1169 => Some(Self::S2sRefTest),
      1170 => Some(Self::S2sRefTestNull),
      1171 => Some(Self::S2sRefAsNonNull),
      1172 => Some(Self::S2sCallRef),
      1173 => Some(Self::S2sReturnCallRef),
      1174 => Some(Self::S2sAnyConvertExtern),
      1175 => Some(Self::S2sExternConvertAny),
      1176 => Some(Self::S2sAssertNullTypecheck),
      1177 => Some(Self::S2sAssertNotNullTypecheck),
      1178 => Some(Self::S2sTrapIllegalCast),
      1179 => Some(Self::S2sRefTestSucceeds),
      1180 => Some(Self::S2sRefTestFails),
      1181 => Some(Self::S2sRefIsNonNull),
      1182 => Some(Self::R2rI32LoadMem8SMultiMem64),
      1183 => Some(Self::R2rI32LoadMem8SMultiMem),
      1184 => Some(Self::R2rI32LoadMem8SMem64),
      1185 => Some(Self::R2rI32LoadMem8UMultiMem64),
      1186 => Some(Self::R2rI32LoadMem8UMultiMem),
      1187 => Some(Self::R2rI32LoadMem8UMem64),
      1188 => Some(Self::R2rI32LoadMem16SMultiMem64),
      1189 => Some(Self::R2rI32LoadMem16SMultiMem),
      1190 => Some(Self::R2rI32LoadMem16SMem64),
      1191 => Some(Self::R2rI32LoadMem16UMultiMem64),
      1192 => Some(Self::R2rI32LoadMem16UMultiMem),
      1193 => Some(Self::R2rI32LoadMem16UMem64),
      1194 => Some(Self::R2rI64LoadMem8SMultiMem64),
      1195 => Some(Self::R2rI64LoadMem8SMultiMem),
      1196 => Some(Self::R2rI64LoadMem8SMem64),
      1197 => Some(Self::R2rI64LoadMem8UMultiMem64),
      1198 => Some(Self::R2rI64LoadMem8UMultiMem),
      1199 => Some(Self::R2rI64LoadMem8UMem64),
      1200 => Some(Self::R2rI64LoadMem16SMultiMem64),
      1201 => Some(Self::R2rI64LoadMem16SMultiMem),
      1202 => Some(Self::R2rI64LoadMem16SMem64),
      1203 => Some(Self::R2rI64LoadMem16UMultiMem64),
      1204 => Some(Self::R2rI64LoadMem16UMultiMem),
      1205 => Some(Self::R2rI64LoadMem16UMem64),
      1206 => Some(Self::R2rI64LoadMem32SMultiMem64),
      1207 => Some(Self::R2rI64LoadMem32SMultiMem),
      1208 => Some(Self::R2rI64LoadMem32SMem64),
      1209 => Some(Self::R2rI64LoadMem32UMultiMem64),
      1210 => Some(Self::R2rI64LoadMem32UMultiMem),
      1211 => Some(Self::R2rI64LoadMem32UMem64),
      1212 => Some(Self::R2rI32LoadMemMultiMem64),
      1213 => Some(Self::R2rI32LoadMemMultiMem),
      1214 => Some(Self::R2rI32LoadMemMem64),
      1215 => Some(Self::R2rI64LoadMemMultiMem64),
      1216 => Some(Self::R2rI64LoadMemMultiMem),
      1217 => Some(Self::R2rI64LoadMemMem64),
      1218 => Some(Self::R2rF32LoadMemMultiMem64),
      1219 => Some(Self::R2rF32LoadMemMultiMem),
      1220 => Some(Self::R2rF32LoadMemMem64),
      1221 => Some(Self::R2rF64LoadMemMultiMem64),
      1222 => Some(Self::R2rF64LoadMemMultiMem),
      1223 => Some(Self::R2rF64LoadMemMem64),
      1224 => Some(Self::R2sI32LoadMem8SMultiMem64),
      1225 => Some(Self::R2sI32LoadMem8SMultiMem),
      1226 => Some(Self::R2sI32LoadMem8SMem64),
      1227 => Some(Self::R2sI32LoadMem8UMultiMem64),
      1228 => Some(Self::R2sI32LoadMem8UMultiMem),
      1229 => Some(Self::R2sI32LoadMem8UMem64),
      1230 => Some(Self::R2sI32LoadMem16SMultiMem64),
      1231 => Some(Self::R2sI32LoadMem16SMultiMem),
      1232 => Some(Self::R2sI32LoadMem16SMem64),
      1233 => Some(Self::R2sI32LoadMem16UMultiMem64),
      1234 => Some(Self::R2sI32LoadMem16UMultiMem),
      1235 => Some(Self::R2sI32LoadMem16UMem64),
      1236 => Some(Self::R2sI64LoadMem8SMultiMem64),
      1237 => Some(Self::R2sI64LoadMem8SMultiMem),
      1238 => Some(Self::R2sI64LoadMem8SMem64),
      1239 => Some(Self::R2sI64LoadMem8UMultiMem64),
      1240 => Some(Self::R2sI64LoadMem8UMultiMem),
      1241 => Some(Self::R2sI64LoadMem8UMem64),
      1242 => Some(Self::R2sI64LoadMem16SMultiMem64),
      1243 => Some(Self::R2sI64LoadMem16SMultiMem),
      1244 => Some(Self::R2sI64LoadMem16SMem64),
      1245 => Some(Self::R2sI64LoadMem16UMultiMem64),
      1246 => Some(Self::R2sI64LoadMem16UMultiMem),
      1247 => Some(Self::R2sI64LoadMem16UMem64),
      1248 => Some(Self::R2sI64LoadMem32SMultiMem64),
      1249 => Some(Self::R2sI64LoadMem32SMultiMem),
      1250 => Some(Self::R2sI64LoadMem32SMem64),
      1251 => Some(Self::R2sI64LoadMem32UMultiMem64),
      1252 => Some(Self::R2sI64LoadMem32UMultiMem),
      1253 => Some(Self::R2sI64LoadMem32UMem64),
      1254 => Some(Self::R2sI32LoadMemMultiMem64),
      1255 => Some(Self::R2sI32LoadMemMultiMem),
      1256 => Some(Self::R2sI32LoadMemMem64),
      1257 => Some(Self::R2sI64LoadMemMultiMem64),
      1258 => Some(Self::R2sI64LoadMemMultiMem),
      1259 => Some(Self::R2sI64LoadMemMem64),
      1260 => Some(Self::R2sF32LoadMemMultiMem64),
      1261 => Some(Self::R2sF32LoadMemMultiMem),
      1262 => Some(Self::R2sF32LoadMemMem64),
      1263 => Some(Self::R2sF64LoadMemMultiMem64),
      1264 => Some(Self::R2sF64LoadMemMultiMem),
      1265 => Some(Self::R2sF64LoadMemMem64),
      1266 => Some(Self::S2rI32LoadMem8SMultiMem64),
      1267 => Some(Self::S2rI32LoadMem8SMultiMem),
      1268 => Some(Self::S2rI32LoadMem8SMem64),
      1269 => Some(Self::S2rI32LoadMem8UMultiMem64),
      1270 => Some(Self::S2rI32LoadMem8UMultiMem),
      1271 => Some(Self::S2rI32LoadMem8UMem64),
      1272 => Some(Self::S2rI32LoadMem16SMultiMem64),
      1273 => Some(Self::S2rI32LoadMem16SMultiMem),
      1274 => Some(Self::S2rI32LoadMem16SMem64),
      1275 => Some(Self::S2rI32LoadMem16UMultiMem64),
      1276 => Some(Self::S2rI32LoadMem16UMultiMem),
      1277 => Some(Self::S2rI32LoadMem16UMem64),
      1278 => Some(Self::S2rI64LoadMem8SMultiMem64),
      1279 => Some(Self::S2rI64LoadMem8SMultiMem),
      1280 => Some(Self::S2rI64LoadMem8SMem64),
      1281 => Some(Self::S2rI64LoadMem8UMultiMem64),
      1282 => Some(Self::S2rI64LoadMem8UMultiMem),
      1283 => Some(Self::S2rI64LoadMem8UMem64),
      1284 => Some(Self::S2rI64LoadMem16SMultiMem64),
      1285 => Some(Self::S2rI64LoadMem16SMultiMem),
      1286 => Some(Self::S2rI64LoadMem16SMem64),
      1287 => Some(Self::S2rI64LoadMem16UMultiMem64),
      1288 => Some(Self::S2rI64LoadMem16UMultiMem),
      1289 => Some(Self::S2rI64LoadMem16UMem64),
      1290 => Some(Self::S2rI64LoadMem32SMultiMem64),
      1291 => Some(Self::S2rI64LoadMem32SMultiMem),
      1292 => Some(Self::S2rI64LoadMem32SMem64),
      1293 => Some(Self::S2rI64LoadMem32UMultiMem64),
      1294 => Some(Self::S2rI64LoadMem32UMultiMem),
      1295 => Some(Self::S2rI64LoadMem32UMem64),
      1296 => Some(Self::S2rI32LoadMemMultiMem64),
      1297 => Some(Self::S2rI32LoadMemMultiMem),
      1298 => Some(Self::S2rI32LoadMemMem64),
      1299 => Some(Self::S2rI64LoadMemMultiMem64),
      1300 => Some(Self::S2rI64LoadMemMultiMem),
      1301 => Some(Self::S2rI64LoadMemMem64),
      1302 => Some(Self::S2rF32LoadMemMultiMem64),
      1303 => Some(Self::S2rF32LoadMemMultiMem),
      1304 => Some(Self::S2rF32LoadMemMem64),
      1305 => Some(Self::S2rF64LoadMemMultiMem64),
      1306 => Some(Self::S2rF64LoadMemMultiMem),
      1307 => Some(Self::S2rF64LoadMemMem64),
      1308 => Some(Self::S2sI32LoadMem8SMultiMem64),
      1309 => Some(Self::S2sI32LoadMem8SMultiMem),
      1310 => Some(Self::S2sI32LoadMem8SMem64),
      1311 => Some(Self::S2sI32LoadMem8UMultiMem64),
      1312 => Some(Self::S2sI32LoadMem8UMultiMem),
      1313 => Some(Self::S2sI32LoadMem8UMem64),
      1314 => Some(Self::S2sI32LoadMem16SMultiMem64),
      1315 => Some(Self::S2sI32LoadMem16SMultiMem),
      1316 => Some(Self::S2sI32LoadMem16SMem64),
      1317 => Some(Self::S2sI32LoadMem16UMultiMem64),
      1318 => Some(Self::S2sI32LoadMem16UMultiMem),
      1319 => Some(Self::S2sI32LoadMem16UMem64),
      1320 => Some(Self::S2sI64LoadMem8SMultiMem64),
      1321 => Some(Self::S2sI64LoadMem8SMultiMem),
      1322 => Some(Self::S2sI64LoadMem8SMem64),
      1323 => Some(Self::S2sI64LoadMem8UMultiMem64),
      1324 => Some(Self::S2sI64LoadMem8UMultiMem),
      1325 => Some(Self::S2sI64LoadMem8UMem64),
      1326 => Some(Self::S2sI64LoadMem16SMultiMem64),
      1327 => Some(Self::S2sI64LoadMem16SMultiMem),
      1328 => Some(Self::S2sI64LoadMem16SMem64),
      1329 => Some(Self::S2sI64LoadMem16UMultiMem64),
      1330 => Some(Self::S2sI64LoadMem16UMultiMem),
      1331 => Some(Self::S2sI64LoadMem16UMem64),
      1332 => Some(Self::S2sI64LoadMem32SMultiMem64),
      1333 => Some(Self::S2sI64LoadMem32SMultiMem),
      1334 => Some(Self::S2sI64LoadMem32SMem64),
      1335 => Some(Self::S2sI64LoadMem32UMultiMem64),
      1336 => Some(Self::S2sI64LoadMem32UMultiMem),
      1337 => Some(Self::S2sI64LoadMem32UMem64),
      1338 => Some(Self::S2sI32LoadMemMultiMem64),
      1339 => Some(Self::S2sI32LoadMemMultiMem),
      1340 => Some(Self::S2sI32LoadMemMem64),
      1341 => Some(Self::S2sI64LoadMemMultiMem64),
      1342 => Some(Self::S2sI64LoadMemMultiMem),
      1343 => Some(Self::S2sI64LoadMemMem64),
      1344 => Some(Self::S2sF32LoadMemMultiMem64),
      1345 => Some(Self::S2sF32LoadMemMultiMem),
      1346 => Some(Self::S2sF32LoadMemMem64),
      1347 => Some(Self::S2sF64LoadMemMultiMem64),
      1348 => Some(Self::S2sF64LoadMemMultiMem),
      1349 => Some(Self::S2sF64LoadMemMem64),
      1350 => Some(Self::R2sI32StoreMem8MultiMem64),
      1351 => Some(Self::R2sI32StoreMem8MultiMem),
      1352 => Some(Self::R2sI32StoreMem8Mem64),
      1353 => Some(Self::R2sI32StoreMem16MultiMem64),
      1354 => Some(Self::R2sI32StoreMem16MultiMem),
      1355 => Some(Self::R2sI32StoreMem16Mem64),
      1356 => Some(Self::R2sI64StoreMem8MultiMem64),
      1357 => Some(Self::R2sI64StoreMem8MultiMem),
      1358 => Some(Self::R2sI64StoreMem8Mem64),
      1359 => Some(Self::R2sI64StoreMem16MultiMem64),
      1360 => Some(Self::R2sI64StoreMem16MultiMem),
      1361 => Some(Self::R2sI64StoreMem16Mem64),
      1362 => Some(Self::R2sI64StoreMem32MultiMem64),
      1363 => Some(Self::R2sI64StoreMem32MultiMem),
      1364 => Some(Self::R2sI64StoreMem32Mem64),
      1365 => Some(Self::R2sI32StoreMemMultiMem64),
      1366 => Some(Self::R2sI32StoreMemMultiMem),
      1367 => Some(Self::R2sI32StoreMemMem64),
      1368 => Some(Self::R2sI64StoreMemMultiMem64),
      1369 => Some(Self::R2sI64StoreMemMultiMem),
      1370 => Some(Self::R2sI64StoreMemMem64),
      1371 => Some(Self::R2sF32StoreMemMultiMem64),
      1372 => Some(Self::R2sF32StoreMemMultiMem),
      1373 => Some(Self::R2sF32StoreMemMem64),
      1374 => Some(Self::R2sF64StoreMemMultiMem64),
      1375 => Some(Self::R2sF64StoreMemMultiMem),
      1376 => Some(Self::R2sF64StoreMemMem64),
      1377 => Some(Self::S2sI32StoreMem8MultiMem64),
      1378 => Some(Self::S2sI32StoreMem8MultiMem),
      1379 => Some(Self::S2sI32StoreMem8Mem64),
      1380 => Some(Self::S2sI32StoreMem16MultiMem64),
      1381 => Some(Self::S2sI32StoreMem16MultiMem),
      1382 => Some(Self::S2sI32StoreMem16Mem64),
      1383 => Some(Self::S2sI64StoreMem8MultiMem64),
      1384 => Some(Self::S2sI64StoreMem8MultiMem),
      1385 => Some(Self::S2sI64StoreMem8Mem64),
      1386 => Some(Self::S2sI64StoreMem16MultiMem64),
      1387 => Some(Self::S2sI64StoreMem16MultiMem),
      1388 => Some(Self::S2sI64StoreMem16Mem64),
      1389 => Some(Self::S2sI64StoreMem32MultiMem64),
      1390 => Some(Self::S2sI64StoreMem32MultiMem),
      1391 => Some(Self::S2sI64StoreMem32Mem64),
      1392 => Some(Self::S2sI32StoreMemMultiMem64),
      1393 => Some(Self::S2sI32StoreMemMultiMem),
      1394 => Some(Self::S2sI32StoreMemMem64),
      1395 => Some(Self::S2sI64StoreMemMultiMem64),
      1396 => Some(Self::S2sI64StoreMemMultiMem),
      1397 => Some(Self::S2sI64StoreMemMem64),
      1398 => Some(Self::S2sF32StoreMemMultiMem64),
      1399 => Some(Self::S2sF32StoreMemMultiMem),
      1400 => Some(Self::S2sF32StoreMemMem64),
      1401 => Some(Self::S2sF64StoreMemMultiMem64),
      1402 => Some(Self::S2sF64StoreMemMultiMem),
      1403 => Some(Self::S2sF64StoreMemMem64),
      1404 => Some(Self::S2sI32LoadMem8SLocalSetMem64),
      1405 => Some(Self::S2sI32LoadMem8ULocalSetMem64),
      1406 => Some(Self::S2sI32LoadMem16SLocalSetMem64),
      1407 => Some(Self::S2sI32LoadMem16ULocalSetMem64),
      1408 => Some(Self::S2sI64LoadMem8SLocalSetMem64),
      1409 => Some(Self::S2sI64LoadMem8ULocalSetMem64),
      1410 => Some(Self::S2sI64LoadMem16SLocalSetMem64),
      1411 => Some(Self::S2sI64LoadMem16ULocalSetMem64),
      1412 => Some(Self::S2sI64LoadMem32SLocalSetMem64),
      1413 => Some(Self::S2sI64LoadMem32ULocalSetMem64),
      1414 => Some(Self::S2sI32LoadMemLocalSetMem64),
      1415 => Some(Self::S2sI64LoadMemLocalSetMem64),
      1416 => Some(Self::S2sF32LoadMemLocalSetMem64),
      1417 => Some(Self::S2sF64LoadMemLocalSetMem64),
      1418 => Some(Self::R2sI32LoadStoreMemMem64),
      1419 => Some(Self::R2sI64LoadStoreMemMem64),
      1420 => Some(Self::R2sF32LoadStoreMemMem64),
      1421 => Some(Self::R2sF64LoadStoreMemMem64),
      1422 => Some(Self::S2sI32LoadStoreMemMem64),
      1423 => Some(Self::S2sI64LoadStoreMemMem64),
      1424 => Some(Self::S2sF32LoadStoreMemMem64),
      1425 => Some(Self::S2sF64LoadStoreMemMem64),
      1426 => Some(Self::S2sAtomicNotifyMultiMem64),
      1427 => Some(Self::S2sAtomicNotifyMultiMem),
      1428 => Some(Self::S2sAtomicNotifyMem64),
      1429 => Some(Self::S2sI32AtomicWaitMultiMem64),
      1430 => Some(Self::S2sI32AtomicWaitMultiMem),
      1431 => Some(Self::S2sI32AtomicWaitMem64),
      1432 => Some(Self::S2sI64AtomicWaitMultiMem64),
      1433 => Some(Self::S2sI64AtomicWaitMultiMem),
      1434 => Some(Self::S2sI64AtomicWaitMem64),
      1435 => Some(Self::S2sI32AtomicAddMultiMem64),
      1436 => Some(Self::S2sI32AtomicAddMultiMem),
      1437 => Some(Self::S2sI32AtomicAddMem64),
      1438 => Some(Self::S2sI32AtomicAdd8UMultiMem64),
      1439 => Some(Self::S2sI32AtomicAdd8UMultiMem),
      1440 => Some(Self::S2sI32AtomicAdd8UMem64),
      1441 => Some(Self::S2sI32AtomicAdd16UMultiMem64),
      1442 => Some(Self::S2sI32AtomicAdd16UMultiMem),
      1443 => Some(Self::S2sI32AtomicAdd16UMem64),
      1444 => Some(Self::S2sI32AtomicSubMultiMem64),
      1445 => Some(Self::S2sI32AtomicSubMultiMem),
      1446 => Some(Self::S2sI32AtomicSubMem64),
      1447 => Some(Self::S2sI32AtomicSub8UMultiMem64),
      1448 => Some(Self::S2sI32AtomicSub8UMultiMem),
      1449 => Some(Self::S2sI32AtomicSub8UMem64),
      1450 => Some(Self::S2sI32AtomicSub16UMultiMem64),
      1451 => Some(Self::S2sI32AtomicSub16UMultiMem),
      1452 => Some(Self::S2sI32AtomicSub16UMem64),
      1453 => Some(Self::S2sI32AtomicAndMultiMem64),
      1454 => Some(Self::S2sI32AtomicAndMultiMem),
      1455 => Some(Self::S2sI32AtomicAndMem64),
      1456 => Some(Self::S2sI32AtomicAnd8UMultiMem64),
      1457 => Some(Self::S2sI32AtomicAnd8UMultiMem),
      1458 => Some(Self::S2sI32AtomicAnd8UMem64),
      1459 => Some(Self::S2sI32AtomicAnd16UMultiMem64),
      1460 => Some(Self::S2sI32AtomicAnd16UMultiMem),
      1461 => Some(Self::S2sI32AtomicAnd16UMem64),
      1462 => Some(Self::S2sI32AtomicOrMultiMem64),
      1463 => Some(Self::S2sI32AtomicOrMultiMem),
      1464 => Some(Self::S2sI32AtomicOrMem64),
      1465 => Some(Self::S2sI32AtomicOr8UMultiMem64),
      1466 => Some(Self::S2sI32AtomicOr8UMultiMem),
      1467 => Some(Self::S2sI32AtomicOr8UMem64),
      1468 => Some(Self::S2sI32AtomicOr16UMultiMem64),
      1469 => Some(Self::S2sI32AtomicOr16UMultiMem),
      1470 => Some(Self::S2sI32AtomicOr16UMem64),
      1471 => Some(Self::S2sI32AtomicXorMultiMem64),
      1472 => Some(Self::S2sI32AtomicXorMultiMem),
      1473 => Some(Self::S2sI32AtomicXorMem64),
      1474 => Some(Self::S2sI32AtomicXor8UMultiMem64),
      1475 => Some(Self::S2sI32AtomicXor8UMultiMem),
      1476 => Some(Self::S2sI32AtomicXor8UMem64),
      1477 => Some(Self::S2sI32AtomicXor16UMultiMem64),
      1478 => Some(Self::S2sI32AtomicXor16UMultiMem),
      1479 => Some(Self::S2sI32AtomicXor16UMem64),
      1480 => Some(Self::S2sI32AtomicExchangeMultiMem64),
      1481 => Some(Self::S2sI32AtomicExchangeMultiMem),
      1482 => Some(Self::S2sI32AtomicExchangeMem64),
      1483 => Some(Self::S2sI32AtomicExchange8UMultiMem64),
      1484 => Some(Self::S2sI32AtomicExchange8UMultiMem),
      1485 => Some(Self::S2sI32AtomicExchange8UMem64),
      1486 => Some(Self::S2sI32AtomicExchange16UMultiMem64),
      1487 => Some(Self::S2sI32AtomicExchange16UMultiMem),
      1488 => Some(Self::S2sI32AtomicExchange16UMem64),
      1489 => Some(Self::S2sI64AtomicAddMultiMem64),
      1490 => Some(Self::S2sI64AtomicAddMultiMem),
      1491 => Some(Self::S2sI64AtomicAddMem64),
      1492 => Some(Self::S2sI64AtomicAdd8UMultiMem64),
      1493 => Some(Self::S2sI64AtomicAdd8UMultiMem),
      1494 => Some(Self::S2sI64AtomicAdd8UMem64),
      1495 => Some(Self::S2sI64AtomicAdd16UMultiMem64),
      1496 => Some(Self::S2sI64AtomicAdd16UMultiMem),
      1497 => Some(Self::S2sI64AtomicAdd16UMem64),
      1498 => Some(Self::S2sI64AtomicAdd32UMultiMem64),
      1499 => Some(Self::S2sI64AtomicAdd32UMultiMem),
      1500 => Some(Self::S2sI64AtomicAdd32UMem64),
      1501 => Some(Self::S2sI64AtomicSubMultiMem64),
      1502 => Some(Self::S2sI64AtomicSubMultiMem),
      1503 => Some(Self::S2sI64AtomicSubMem64),
      1504 => Some(Self::S2sI64AtomicSub8UMultiMem64),
      1505 => Some(Self::S2sI64AtomicSub8UMultiMem),
      1506 => Some(Self::S2sI64AtomicSub8UMem64),
      1507 => Some(Self::S2sI64AtomicSub16UMultiMem64),
      1508 => Some(Self::S2sI64AtomicSub16UMultiMem),
      1509 => Some(Self::S2sI64AtomicSub16UMem64),
      1510 => Some(Self::S2sI64AtomicSub32UMultiMem64),
      1511 => Some(Self::S2sI64AtomicSub32UMultiMem),
      1512 => Some(Self::S2sI64AtomicSub32UMem64),
      1513 => Some(Self::S2sI64AtomicAndMultiMem64),
      1514 => Some(Self::S2sI64AtomicAndMultiMem),
      1515 => Some(Self::S2sI64AtomicAndMem64),
      1516 => Some(Self::S2sI64AtomicAnd8UMultiMem64),
      1517 => Some(Self::S2sI64AtomicAnd8UMultiMem),
      1518 => Some(Self::S2sI64AtomicAnd8UMem64),
      1519 => Some(Self::S2sI64AtomicAnd16UMultiMem64),
      1520 => Some(Self::S2sI64AtomicAnd16UMultiMem),
      1521 => Some(Self::S2sI64AtomicAnd16UMem64),
      1522 => Some(Self::S2sI64AtomicAnd32UMultiMem64),
      1523 => Some(Self::S2sI64AtomicAnd32UMultiMem),
      1524 => Some(Self::S2sI64AtomicAnd32UMem64),
      1525 => Some(Self::S2sI64AtomicOrMultiMem64),
      1526 => Some(Self::S2sI64AtomicOrMultiMem),
      1527 => Some(Self::S2sI64AtomicOrMem64),
      1528 => Some(Self::S2sI64AtomicOr8UMultiMem64),
      1529 => Some(Self::S2sI64AtomicOr8UMultiMem),
      1530 => Some(Self::S2sI64AtomicOr8UMem64),
      1531 => Some(Self::S2sI64AtomicOr16UMultiMem64),
      1532 => Some(Self::S2sI64AtomicOr16UMultiMem),
      1533 => Some(Self::S2sI64AtomicOr16UMem64),
      1534 => Some(Self::S2sI64AtomicOr32UMultiMem64),
      1535 => Some(Self::S2sI64AtomicOr32UMultiMem),
      1536 => Some(Self::S2sI64AtomicOr32UMem64),
      1537 => Some(Self::S2sI64AtomicXorMultiMem64),
      1538 => Some(Self::S2sI64AtomicXorMultiMem),
      1539 => Some(Self::S2sI64AtomicXorMem64),
      1540 => Some(Self::S2sI64AtomicXor8UMultiMem64),
      1541 => Some(Self::S2sI64AtomicXor8UMultiMem),
      1542 => Some(Self::S2sI64AtomicXor8UMem64),
      1543 => Some(Self::S2sI64AtomicXor16UMultiMem64),
      1544 => Some(Self::S2sI64AtomicXor16UMultiMem),
      1545 => Some(Self::S2sI64AtomicXor16UMem64),
      1546 => Some(Self::S2sI64AtomicXor32UMultiMem64),
      1547 => Some(Self::S2sI64AtomicXor32UMultiMem),
      1548 => Some(Self::S2sI64AtomicXor32UMem64),
      1549 => Some(Self::S2sI64AtomicExchangeMultiMem64),
      1550 => Some(Self::S2sI64AtomicExchangeMultiMem),
      1551 => Some(Self::S2sI64AtomicExchangeMem64),
      1552 => Some(Self::S2sI64AtomicExchange8UMultiMem64),
      1553 => Some(Self::S2sI64AtomicExchange8UMultiMem),
      1554 => Some(Self::S2sI64AtomicExchange8UMem64),
      1555 => Some(Self::S2sI64AtomicExchange16UMultiMem64),
      1556 => Some(Self::S2sI64AtomicExchange16UMultiMem),
      1557 => Some(Self::S2sI64AtomicExchange16UMem64),
      1558 => Some(Self::S2sI64AtomicExchange32UMultiMem64),
      1559 => Some(Self::S2sI64AtomicExchange32UMultiMem),
      1560 => Some(Self::S2sI64AtomicExchange32UMem64),
      1561 => Some(Self::S2sI32AtomicCompareExchangeMultiMem64),
      1562 => Some(Self::S2sI32AtomicCompareExchangeMultiMem),
      1563 => Some(Self::S2sI32AtomicCompareExchangeMem64),
      1564 => Some(Self::S2sI32AtomicCompareExchange8UMultiMem64),
      1565 => Some(Self::S2sI32AtomicCompareExchange8UMultiMem),
      1566 => Some(Self::S2sI32AtomicCompareExchange8UMem64),
      1567 => Some(Self::S2sI32AtomicCompareExchange16UMultiMem64),
      1568 => Some(Self::S2sI32AtomicCompareExchange16UMultiMem),
      1569 => Some(Self::S2sI32AtomicCompareExchange16UMem64),
      1570 => Some(Self::S2sI64AtomicCompareExchangeMultiMem64),
      1571 => Some(Self::S2sI64AtomicCompareExchangeMultiMem),
      1572 => Some(Self::S2sI64AtomicCompareExchangeMem64),
      1573 => Some(Self::S2sI64AtomicCompareExchange8UMultiMem64),
      1574 => Some(Self::S2sI64AtomicCompareExchange8UMultiMem),
      1575 => Some(Self::S2sI64AtomicCompareExchange8UMem64),
      1576 => Some(Self::S2sI64AtomicCompareExchange16UMultiMem64),
      1577 => Some(Self::S2sI64AtomicCompareExchange16UMultiMem),
      1578 => Some(Self::S2sI64AtomicCompareExchange16UMem64),
      1579 => Some(Self::S2sI64AtomicCompareExchange32UMultiMem64),
      1580 => Some(Self::S2sI64AtomicCompareExchange32UMultiMem),
      1581 => Some(Self::S2sI64AtomicCompareExchange32UMem64),
      1582 => Some(Self::S2sI32AtomicLoadMultiMem64),
      1583 => Some(Self::S2sI32AtomicLoadMultiMem),
      1584 => Some(Self::S2sI32AtomicLoadMem64),
      1585 => Some(Self::S2sI32AtomicLoad8UMultiMem64),
      1586 => Some(Self::S2sI32AtomicLoad8UMultiMem),
      1587 => Some(Self::S2sI32AtomicLoad8UMem64),
      1588 => Some(Self::S2sI32AtomicLoad16UMultiMem64),
      1589 => Some(Self::S2sI32AtomicLoad16UMultiMem),
      1590 => Some(Self::S2sI32AtomicLoad16UMem64),
      1591 => Some(Self::S2sI64AtomicLoadMultiMem64),
      1592 => Some(Self::S2sI64AtomicLoadMultiMem),
      1593 => Some(Self::S2sI64AtomicLoadMem64),
      1594 => Some(Self::S2sI64AtomicLoad8UMultiMem64),
      1595 => Some(Self::S2sI64AtomicLoad8UMultiMem),
      1596 => Some(Self::S2sI64AtomicLoad8UMem64),
      1597 => Some(Self::S2sI64AtomicLoad16UMultiMem64),
      1598 => Some(Self::S2sI64AtomicLoad16UMultiMem),
      1599 => Some(Self::S2sI64AtomicLoad16UMem64),
      1600 => Some(Self::S2sI64AtomicLoad32UMultiMem64),
      1601 => Some(Self::S2sI64AtomicLoad32UMultiMem),
      1602 => Some(Self::S2sI64AtomicLoad32UMem64),
      1603 => Some(Self::S2sI32AtomicStoreMultiMem64),
      1604 => Some(Self::S2sI32AtomicStoreMultiMem),
      1605 => Some(Self::S2sI32AtomicStoreMem64),
      1606 => Some(Self::S2sI32AtomicStore8UMultiMem64),
      1607 => Some(Self::S2sI32AtomicStore8UMultiMem),
      1608 => Some(Self::S2sI32AtomicStore8UMem64),
      1609 => Some(Self::S2sI32AtomicStore16UMultiMem64),
      1610 => Some(Self::S2sI32AtomicStore16UMultiMem),
      1611 => Some(Self::S2sI32AtomicStore16UMem64),
      1612 => Some(Self::S2sI64AtomicStoreMultiMem64),
      1613 => Some(Self::S2sI64AtomicStoreMultiMem),
      1614 => Some(Self::S2sI64AtomicStoreMem64),
      1615 => Some(Self::S2sI64AtomicStore8UMultiMem64),
      1616 => Some(Self::S2sI64AtomicStore8UMultiMem),
      1617 => Some(Self::S2sI64AtomicStore8UMem64),
      1618 => Some(Self::S2sI64AtomicStore16UMultiMem64),
      1619 => Some(Self::S2sI64AtomicStore16UMultiMem),
      1620 => Some(Self::S2sI64AtomicStore16UMem64),
      1621 => Some(Self::S2sI64AtomicStore32UMultiMem64),
      1622 => Some(Self::S2sI64AtomicStore32UMultiMem),
      1623 => Some(Self::S2sI64AtomicStore32UMem64),
      1624 => Some(Self::S2sAtomicNotify),
      1625 => Some(Self::S2sI32AtomicWait),
      1626 => Some(Self::S2sI64AtomicWait),
      1627 => Some(Self::S2sI32AtomicAdd),
      1628 => Some(Self::S2sI32AtomicAdd8U),
      1629 => Some(Self::S2sI32AtomicAdd16U),
      1630 => Some(Self::S2sI32AtomicSub),
      1631 => Some(Self::S2sI32AtomicSub8U),
      1632 => Some(Self::S2sI32AtomicSub16U),
      1633 => Some(Self::S2sI32AtomicAnd),
      1634 => Some(Self::S2sI32AtomicAnd8U),
      1635 => Some(Self::S2sI32AtomicAnd16U),
      1636 => Some(Self::S2sI32AtomicOr),
      1637 => Some(Self::S2sI32AtomicOr8U),
      1638 => Some(Self::S2sI32AtomicOr16U),
      1639 => Some(Self::S2sI32AtomicXor),
      1640 => Some(Self::S2sI32AtomicXor8U),
      1641 => Some(Self::S2sI32AtomicXor16U),
      1642 => Some(Self::S2sI32AtomicExchange),
      1643 => Some(Self::S2sI32AtomicExchange8U),
      1644 => Some(Self::S2sI32AtomicExchange16U),
      1645 => Some(Self::S2sI64AtomicAdd),
      1646 => Some(Self::S2sI64AtomicAdd8U),
      1647 => Some(Self::S2sI64AtomicAdd16U),
      1648 => Some(Self::S2sI64AtomicAdd32U),
      1649 => Some(Self::S2sI64AtomicSub),
      1650 => Some(Self::S2sI64AtomicSub8U),
      1651 => Some(Self::S2sI64AtomicSub16U),
      1652 => Some(Self::S2sI64AtomicSub32U),
      1653 => Some(Self::S2sI64AtomicAnd),
      1654 => Some(Self::S2sI64AtomicAnd8U),
      1655 => Some(Self::S2sI64AtomicAnd16U),
      1656 => Some(Self::S2sI64AtomicAnd32U),
      1657 => Some(Self::S2sI64AtomicOr),
      1658 => Some(Self::S2sI64AtomicOr8U),
      1659 => Some(Self::S2sI64AtomicOr16U),
      1660 => Some(Self::S2sI64AtomicOr32U),
      1661 => Some(Self::S2sI64AtomicXor),
      1662 => Some(Self::S2sI64AtomicXor8U),
      1663 => Some(Self::S2sI64AtomicXor16U),
      1664 => Some(Self::S2sI64AtomicXor32U),
      1665 => Some(Self::S2sI64AtomicExchange),
      1666 => Some(Self::S2sI64AtomicExchange8U),
      1667 => Some(Self::S2sI64AtomicExchange16U),
      1668 => Some(Self::S2sI64AtomicExchange32U),
      1669 => Some(Self::S2sI32AtomicCompareExchange),
      1670 => Some(Self::S2sI32AtomicCompareExchange8U),
      1671 => Some(Self::S2sI32AtomicCompareExchange16U),
      1672 => Some(Self::S2sI64AtomicCompareExchange),
      1673 => Some(Self::S2sI64AtomicCompareExchange8U),
      1674 => Some(Self::S2sI64AtomicCompareExchange16U),
      1675 => Some(Self::S2sI64AtomicCompareExchange32U),
      1676 => Some(Self::S2sI32AtomicLoad),
      1677 => Some(Self::S2sI32AtomicLoad8U),
      1678 => Some(Self::S2sI32AtomicLoad16U),
      1679 => Some(Self::S2sI64AtomicLoad),
      1680 => Some(Self::S2sI64AtomicLoad8U),
      1681 => Some(Self::S2sI64AtomicLoad16U),
      1682 => Some(Self::S2sI64AtomicLoad32U),
      1683 => Some(Self::S2sI32AtomicStore),
      1684 => Some(Self::S2sI32AtomicStore8U),
      1685 => Some(Self::S2sI32AtomicStore16U),
      1686 => Some(Self::S2sI64AtomicStore),
      1687 => Some(Self::S2sI64AtomicStore8U),
      1688 => Some(Self::S2sI64AtomicStore16U),
      1689 => Some(Self::S2sI64AtomicStore32U),
      _ => None,
    }
  }
}

/// 处理函数尺寸类别 (对应原版 enum InstrHandlerSize)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InstrHandlerSize {
  /// Large = 0 (false)
  Large = 0,
  /// Small = 1 (true)
  Small = 1,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(InstructionHandler::COUNT == 1690);
};
