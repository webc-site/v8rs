// 由 tool/src/wasm/wasm-opcodes.h.js 从 v8/src/wasm/wasm-opcodes.{h,-inl.h,cc} 生成; 修订就地改, 复跑覆盖
// Wasm 操作码全表: X-macro 机械展开 + 签名查表 + TrapReason

/// Wasm 操作码 32 位内部编码 (对应 C++ enum WasmOpcode)。
///
/// 编码规则 (见原版 wasm-opcodes.h 注释, 双射):
/// 1) 无前缀: [0x00, 0xff] 即字节本身;
/// 2) 前缀码 [0xfb00, 0xfeff]: (prefix << 8) | 低位字节;
/// 3) relaxed-simd: (0xfd << 12) | 12 位字节, 即 [0xfd100, 0xfd1ff]。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmOpcode(pub u32);

impl WasmOpcode {
  /// 原始 32 位编码
  #[inline]
  #[must_use]
  pub const fn as_u32(self) -> u32 {
    self.0
  }
}

/// 无前缀表达式操作码 (对应 C++ kExpr* 常量, FOREACH_OPCODE 展开 699 条)
#[rustfmt::skip]
pub mod expr {
  use super::WasmOpcode;
  /// unreachable
  pub const UNREACHABLE: WasmOpcode = WasmOpcode(0x0);
  /// nop
  pub const NOP: WasmOpcode = WasmOpcode(0x1);
  /// block
  pub const BLOCK: WasmOpcode = WasmOpcode(0x2);
  /// loop
  pub const LOOP: WasmOpcode = WasmOpcode(0x3);
  /// if
  pub const IF: WasmOpcode = WasmOpcode(0x4);
  /// else
  pub const ELSE: WasmOpcode = WasmOpcode(0x5);
  /// try
  pub const TRY: WasmOpcode = WasmOpcode(0x6);
  /// catch
  pub const CATCH: WasmOpcode = WasmOpcode(0x7);
  /// throw
  pub const THROW: WasmOpcode = WasmOpcode(0x8);
  /// rethrow
  pub const RETHROW: WasmOpcode = WasmOpcode(0x9);
  /// try_table
  pub const TRY_TABLE: WasmOpcode = WasmOpcode(0x1f);
  /// throw_ref
  pub const THROW_REF: WasmOpcode = WasmOpcode(0xa);
  /// end
  pub const END: WasmOpcode = WasmOpcode(0xb);
  /// br
  pub const BR: WasmOpcode = WasmOpcode(0xc);
  /// br_if
  pub const BR_IF: WasmOpcode = WasmOpcode(0xd);
  /// br_table
  pub const BR_TABLE: WasmOpcode = WasmOpcode(0xe);
  /// return
  pub const RETURN: WasmOpcode = WasmOpcode(0xf);
  /// delegate
  pub const DELEGATE: WasmOpcode = WasmOpcode(0x18);
  /// catch_all
  pub const CATCH_ALL: WasmOpcode = WasmOpcode(0x19);
  /// br_on_null
  pub const BR_ON_NULL: WasmOpcode = WasmOpcode(0xd5);
  /// br_on_non_null
  pub const BR_ON_NON_NULL: WasmOpcode = WasmOpcode(0xd6);
  /// nop_for_testing
  pub const NOP_FOR_TESTING_UNSUPPORTED_IN_LIFTOFF: WasmOpcode = WasmOpcode(0x16);
  /// call
  pub const CALL_FUNCTION: WasmOpcode = WasmOpcode(0x10);
  /// call_indirect
  pub const CALL_INDIRECT: WasmOpcode = WasmOpcode(0x11);
  /// return_call
  pub const RETURN_CALL: WasmOpcode = WasmOpcode(0x12);
  /// return_call_indirect
  pub const RETURN_CALL_INDIRECT: WasmOpcode = WasmOpcode(0x13);
  /// call_ref
  pub const CALL_REF: WasmOpcode = WasmOpcode(0x14);
  /// return_call_ref
  pub const RETURN_CALL_REF: WasmOpcode = WasmOpcode(0x15);
  /// drop
  pub const DROP: WasmOpcode = WasmOpcode(0x1a);
  /// select
  pub const SELECT: WasmOpcode = WasmOpcode(0x1b);
  /// select
  pub const SELECT_WITH_TYPE: WasmOpcode = WasmOpcode(0x1c);
  /// local.get
  pub const LOCAL_GET: WasmOpcode = WasmOpcode(0x20);
  /// local.set
  pub const LOCAL_SET: WasmOpcode = WasmOpcode(0x21);
  /// local.tee
  pub const LOCAL_TEE: WasmOpcode = WasmOpcode(0x22);
  /// global.get
  pub const GLOBAL_GET: WasmOpcode = WasmOpcode(0x23);
  /// global.set
  pub const GLOBAL_SET: WasmOpcode = WasmOpcode(0x24);
  /// table.get
  pub const TABLE_GET: WasmOpcode = WasmOpcode(0x25);
  /// table.set
  pub const TABLE_SET: WasmOpcode = WasmOpcode(0x26);
  /// i32.const
  pub const I32_CONST: WasmOpcode = WasmOpcode(0x41);
  /// i64.const
  pub const I64_CONST: WasmOpcode = WasmOpcode(0x42);
  /// f32.const
  pub const F32_CONST: WasmOpcode = WasmOpcode(0x43);
  /// f64.const
  pub const F64_CONST: WasmOpcode = WasmOpcode(0x44);
  /// ref.null
  pub const REF_NULL: WasmOpcode = WasmOpcode(0xd0);
  /// ref.is_null
  pub const REF_IS_NULL: WasmOpcode = WasmOpcode(0xd1);
  /// ref.func
  pub const REF_FUNC: WasmOpcode = WasmOpcode(0xd2);
  /// ref.as_non_null
  pub const REF_AS_NON_NULL: WasmOpcode = WasmOpcode(0xd4);
  /// ref.eq
  pub const REF_EQ: WasmOpcode = WasmOpcode(0xd3);
  /// i32.load
  pub const I32_LOAD_MEM: WasmOpcode = WasmOpcode(0x28);
  /// i64.load
  pub const I64_LOAD_MEM: WasmOpcode = WasmOpcode(0x29);
  /// f32.load
  pub const F32_LOAD_MEM: WasmOpcode = WasmOpcode(0x2a);
  /// f64.load
  pub const F64_LOAD_MEM: WasmOpcode = WasmOpcode(0x2b);
  /// i32.load8_s
  pub const I32_LOAD_MEM8_S: WasmOpcode = WasmOpcode(0x2c);
  /// i32.load8_u
  pub const I32_LOAD_MEM8_U: WasmOpcode = WasmOpcode(0x2d);
  /// i32.load16_s
  pub const I32_LOAD_MEM16_S: WasmOpcode = WasmOpcode(0x2e);
  /// i32.load16_u
  pub const I32_LOAD_MEM16_U: WasmOpcode = WasmOpcode(0x2f);
  /// i64.load8_s
  pub const I64_LOAD_MEM8_S: WasmOpcode = WasmOpcode(0x30);
  /// i64.load8_u
  pub const I64_LOAD_MEM8_U: WasmOpcode = WasmOpcode(0x31);
  /// i64.load16_s
  pub const I64_LOAD_MEM16_S: WasmOpcode = WasmOpcode(0x32);
  /// i64.load16_u
  pub const I64_LOAD_MEM16_U: WasmOpcode = WasmOpcode(0x33);
  /// i64.load32_s
  pub const I64_LOAD_MEM32_S: WasmOpcode = WasmOpcode(0x34);
  /// i64.load32_u
  pub const I64_LOAD_MEM32_U: WasmOpcode = WasmOpcode(0x35);
  /// i32.store
  pub const I32_STORE_MEM: WasmOpcode = WasmOpcode(0x36);
  /// i64.store
  pub const I64_STORE_MEM: WasmOpcode = WasmOpcode(0x37);
  /// f32.store
  pub const F32_STORE_MEM: WasmOpcode = WasmOpcode(0x38);
  /// f64.store
  pub const F64_STORE_MEM: WasmOpcode = WasmOpcode(0x39);
  /// i32.store8
  pub const I32_STORE_MEM8: WasmOpcode = WasmOpcode(0x3a);
  /// i32.store16
  pub const I32_STORE_MEM16: WasmOpcode = WasmOpcode(0x3b);
  /// i64.store8
  pub const I64_STORE_MEM8: WasmOpcode = WasmOpcode(0x3c);
  /// i64.store16
  pub const I64_STORE_MEM16: WasmOpcode = WasmOpcode(0x3d);
  /// i64.store32
  pub const I64_STORE_MEM32: WasmOpcode = WasmOpcode(0x3e);
  /// memory.size
  pub const MEMORY_SIZE: WasmOpcode = WasmOpcode(0x3f);
  /// memory.grow
  pub const MEMORY_GROW: WasmOpcode = WasmOpcode(0x40);
  /// i32.add
  pub const I32_ADD: WasmOpcode = WasmOpcode(0x6a);
  /// i32.sub
  pub const I32_SUB: WasmOpcode = WasmOpcode(0x6b);
  /// i32.mul
  pub const I32_MUL: WasmOpcode = WasmOpcode(0x6c);
  /// i64.add
  pub const I64_ADD: WasmOpcode = WasmOpcode(0x7c);
  /// i64.sub
  pub const I64_SUB: WasmOpcode = WasmOpcode(0x7d);
  /// i64.mul
  pub const I64_MUL: WasmOpcode = WasmOpcode(0x7e);
  /// i32.eqz
  pub const I32_EQZ: WasmOpcode = WasmOpcode(0x45);
  /// i32.eq
  pub const I32_EQ: WasmOpcode = WasmOpcode(0x46);
  /// i32.ne
  pub const I32_NE: WasmOpcode = WasmOpcode(0x47);
  /// i32.lt_s
  pub const I32_LT_S: WasmOpcode = WasmOpcode(0x48);
  /// i32.lt_u
  pub const I32_LT_U: WasmOpcode = WasmOpcode(0x49);
  /// i32.gt_s
  pub const I32_GT_S: WasmOpcode = WasmOpcode(0x4a);
  /// i32.gt_u
  pub const I32_GT_U: WasmOpcode = WasmOpcode(0x4b);
  /// i32.le_s
  pub const I32_LE_S: WasmOpcode = WasmOpcode(0x4c);
  /// i32.le_u
  pub const I32_LE_U: WasmOpcode = WasmOpcode(0x4d);
  /// i32.ge_s
  pub const I32_GE_S: WasmOpcode = WasmOpcode(0x4e);
  /// i32.ge_u
  pub const I32_GE_U: WasmOpcode = WasmOpcode(0x4f);
  /// i64.eqz
  pub const I64_EQZ: WasmOpcode = WasmOpcode(0x50);
  /// i64.eq
  pub const I64_EQ: WasmOpcode = WasmOpcode(0x51);
  /// i64.ne
  pub const I64_NE: WasmOpcode = WasmOpcode(0x52);
  /// i64.lt_s
  pub const I64_LT_S: WasmOpcode = WasmOpcode(0x53);
  /// i64.lt_u
  pub const I64_LT_U: WasmOpcode = WasmOpcode(0x54);
  /// i64.gt_s
  pub const I64_GT_S: WasmOpcode = WasmOpcode(0x55);
  /// i64.gt_u
  pub const I64_GT_U: WasmOpcode = WasmOpcode(0x56);
  /// i64.le_s
  pub const I64_LE_S: WasmOpcode = WasmOpcode(0x57);
  /// i64.le_u
  pub const I64_LE_U: WasmOpcode = WasmOpcode(0x58);
  /// i64.ge_s
  pub const I64_GE_S: WasmOpcode = WasmOpcode(0x59);
  /// i64.ge_u
  pub const I64_GE_U: WasmOpcode = WasmOpcode(0x5a);
  /// f32.eq
  pub const F32_EQ: WasmOpcode = WasmOpcode(0x5b);
  /// f32.ne
  pub const F32_NE: WasmOpcode = WasmOpcode(0x5c);
  /// f32.lt
  pub const F32_LT: WasmOpcode = WasmOpcode(0x5d);
  /// f32.gt
  pub const F32_GT: WasmOpcode = WasmOpcode(0x5e);
  /// f32.le
  pub const F32_LE: WasmOpcode = WasmOpcode(0x5f);
  /// f32.ge
  pub const F32_GE: WasmOpcode = WasmOpcode(0x60);
  /// f64.eq
  pub const F64_EQ: WasmOpcode = WasmOpcode(0x61);
  /// f64.ne
  pub const F64_NE: WasmOpcode = WasmOpcode(0x62);
  /// f64.lt
  pub const F64_LT: WasmOpcode = WasmOpcode(0x63);
  /// f64.gt
  pub const F64_GT: WasmOpcode = WasmOpcode(0x64);
  /// f64.le
  pub const F64_LE: WasmOpcode = WasmOpcode(0x65);
  /// f64.ge
  pub const F64_GE: WasmOpcode = WasmOpcode(0x66);
  /// i32.clz
  pub const I32_CLZ: WasmOpcode = WasmOpcode(0x67);
  /// i32.ctz
  pub const I32_CTZ: WasmOpcode = WasmOpcode(0x68);
  /// i32.popcnt
  pub const I32_POPCNT: WasmOpcode = WasmOpcode(0x69);
  /// i32.div_s
  pub const I32_DIV_S: WasmOpcode = WasmOpcode(0x6d);
  /// i32.div_u
  pub const I32_DIV_U: WasmOpcode = WasmOpcode(0x6e);
  /// i32.rem_s
  pub const I32_REM_S: WasmOpcode = WasmOpcode(0x6f);
  /// i32.rem_u
  pub const I32_REM_U: WasmOpcode = WasmOpcode(0x70);
  /// i32.and
  pub const I32_AND: WasmOpcode = WasmOpcode(0x71);
  /// i32.or
  pub const I32_IOR: WasmOpcode = WasmOpcode(0x72);
  /// i32.xor
  pub const I32_XOR: WasmOpcode = WasmOpcode(0x73);
  /// i32.shl
  pub const I32_SHL: WasmOpcode = WasmOpcode(0x74);
  /// i32.shr_s
  pub const I32_SHR_S: WasmOpcode = WasmOpcode(0x75);
  /// i32.shr_u
  pub const I32_SHR_U: WasmOpcode = WasmOpcode(0x76);
  /// i32.rotl
  pub const I32_ROL: WasmOpcode = WasmOpcode(0x77);
  /// i32.rotr
  pub const I32_ROR: WasmOpcode = WasmOpcode(0x78);
  /// i64.clz
  pub const I64_CLZ: WasmOpcode = WasmOpcode(0x79);
  /// i64.ctz
  pub const I64_CTZ: WasmOpcode = WasmOpcode(0x7a);
  /// i64.popcnt
  pub const I64_POPCNT: WasmOpcode = WasmOpcode(0x7b);
  /// i64.div_s
  pub const I64_DIV_S: WasmOpcode = WasmOpcode(0x7f);
  /// i64.div_u
  pub const I64_DIV_U: WasmOpcode = WasmOpcode(0x80);
  /// i64.rem_s
  pub const I64_REM_S: WasmOpcode = WasmOpcode(0x81);
  /// i64.rem_u
  pub const I64_REM_U: WasmOpcode = WasmOpcode(0x82);
  /// i64.and
  pub const I64_AND: WasmOpcode = WasmOpcode(0x83);
  /// i64.or
  pub const I64_IOR: WasmOpcode = WasmOpcode(0x84);
  /// i64.xor
  pub const I64_XOR: WasmOpcode = WasmOpcode(0x85);
  /// i64.shl
  pub const I64_SHL: WasmOpcode = WasmOpcode(0x86);
  /// i64.shr_s
  pub const I64_SHR_S: WasmOpcode = WasmOpcode(0x87);
  /// i64.shr_u
  pub const I64_SHR_U: WasmOpcode = WasmOpcode(0x88);
  /// i64.rotl
  pub const I64_ROL: WasmOpcode = WasmOpcode(0x89);
  /// i64.rotr
  pub const I64_ROR: WasmOpcode = WasmOpcode(0x8a);
  /// f32.abs
  pub const F32_ABS: WasmOpcode = WasmOpcode(0x8b);
  /// f32.neg
  pub const F32_NEG: WasmOpcode = WasmOpcode(0x8c);
  /// f32.ceil
  pub const F32_CEIL: WasmOpcode = WasmOpcode(0x8d);
  /// f32.floor
  pub const F32_FLOOR: WasmOpcode = WasmOpcode(0x8e);
  /// f32.trunc
  pub const F32_TRUNC: WasmOpcode = WasmOpcode(0x8f);
  /// f32.nearest
  pub const F32_NEAREST_INT: WasmOpcode = WasmOpcode(0x90);
  /// f32.sqrt
  pub const F32_SQRT: WasmOpcode = WasmOpcode(0x91);
  /// f32.add
  pub const F32_ADD: WasmOpcode = WasmOpcode(0x92);
  /// f32.sub
  pub const F32_SUB: WasmOpcode = WasmOpcode(0x93);
  /// f32.mul
  pub const F32_MUL: WasmOpcode = WasmOpcode(0x94);
  /// f32.div
  pub const F32_DIV: WasmOpcode = WasmOpcode(0x95);
  /// f32.min
  pub const F32_MIN: WasmOpcode = WasmOpcode(0x96);
  /// f32.max
  pub const F32_MAX: WasmOpcode = WasmOpcode(0x97);
  /// f32.copysign
  pub const F32_COPY_SIGN: WasmOpcode = WasmOpcode(0x98);
  /// f64.abs
  pub const F64_ABS: WasmOpcode = WasmOpcode(0x99);
  /// f64.neg
  pub const F64_NEG: WasmOpcode = WasmOpcode(0x9a);
  /// f64.ceil
  pub const F64_CEIL: WasmOpcode = WasmOpcode(0x9b);
  /// f64.floor
  pub const F64_FLOOR: WasmOpcode = WasmOpcode(0x9c);
  /// f64.trunc
  pub const F64_TRUNC: WasmOpcode = WasmOpcode(0x9d);
  /// f64.nearest
  pub const F64_NEAREST_INT: WasmOpcode = WasmOpcode(0x9e);
  /// f64.sqrt
  pub const F64_SQRT: WasmOpcode = WasmOpcode(0x9f);
  /// f64.add
  pub const F64_ADD: WasmOpcode = WasmOpcode(0xa0);
  /// f64.sub
  pub const F64_SUB: WasmOpcode = WasmOpcode(0xa1);
  /// f64.mul
  pub const F64_MUL: WasmOpcode = WasmOpcode(0xa2);
  /// f64.div
  pub const F64_DIV: WasmOpcode = WasmOpcode(0xa3);
  /// f64.min
  pub const F64_MIN: WasmOpcode = WasmOpcode(0xa4);
  /// f64.max
  pub const F64_MAX: WasmOpcode = WasmOpcode(0xa5);
  /// f64.copysign
  pub const F64_COPY_SIGN: WasmOpcode = WasmOpcode(0xa6);
  /// i32.wrap_i64
  pub const I32_CONVERT_I64: WasmOpcode = WasmOpcode(0xa7);
  /// i32.trunc_f32_s
  pub const I32_SCONVERT_F32: WasmOpcode = WasmOpcode(0xa8);
  /// i32.trunc_f32_u
  pub const I32_UCONVERT_F32: WasmOpcode = WasmOpcode(0xa9);
  /// i32.trunc_f64_s
  pub const I32_SCONVERT_F64: WasmOpcode = WasmOpcode(0xaa);
  /// i32.trunc_f64_u
  pub const I32_UCONVERT_F64: WasmOpcode = WasmOpcode(0xab);
  /// i64.extend_i32_s
  pub const I64_SCONVERT_I32: WasmOpcode = WasmOpcode(0xac);
  /// i64.extend_i32_u
  pub const I64_UCONVERT_I32: WasmOpcode = WasmOpcode(0xad);
  /// i64.trunc_f32_s
  pub const I64_SCONVERT_F32: WasmOpcode = WasmOpcode(0xae);
  /// i64.trunc_f32_u
  pub const I64_UCONVERT_F32: WasmOpcode = WasmOpcode(0xaf);
  /// i64.trunc_f64_s
  pub const I64_SCONVERT_F64: WasmOpcode = WasmOpcode(0xb0);
  /// i64.trunc_f64_u
  pub const I64_UCONVERT_F64: WasmOpcode = WasmOpcode(0xb1);
  /// f32.convert_i32_s
  pub const F32_SCONVERT_I32: WasmOpcode = WasmOpcode(0xb2);
  /// f32.convert_i32_u
  pub const F32_UCONVERT_I32: WasmOpcode = WasmOpcode(0xb3);
  /// f32.convert_i64_s
  pub const F32_SCONVERT_I64: WasmOpcode = WasmOpcode(0xb4);
  /// f32.convert_i64_u
  pub const F32_UCONVERT_I64: WasmOpcode = WasmOpcode(0xb5);
  /// f32.demote_f64
  pub const F32_CONVERT_F64: WasmOpcode = WasmOpcode(0xb6);
  /// f64.convert_i32_s
  pub const F64_SCONVERT_I32: WasmOpcode = WasmOpcode(0xb7);
  /// f64.convert_i32_u
  pub const F64_UCONVERT_I32: WasmOpcode = WasmOpcode(0xb8);
  /// f64.convert_i64_s
  pub const F64_SCONVERT_I64: WasmOpcode = WasmOpcode(0xb9);
  /// f64.convert_i64_u
  pub const F64_UCONVERT_I64: WasmOpcode = WasmOpcode(0xba);
  /// f64.promote_f32
  pub const F64_CONVERT_F32: WasmOpcode = WasmOpcode(0xbb);
  /// i32.reinterpret_f32
  pub const I32_REINTERPRET_F32: WasmOpcode = WasmOpcode(0xbc);
  /// i64.reinterpret_f64
  pub const I64_REINTERPRET_F64: WasmOpcode = WasmOpcode(0xbd);
  /// f32.reinterpret_i32
  pub const F32_REINTERPRET_I32: WasmOpcode = WasmOpcode(0xbe);
  /// f64.reinterpret_i64
  pub const F64_REINTERPRET_I64: WasmOpcode = WasmOpcode(0xbf);
  /// i32.extend8_s
  pub const I32_SEXTEND_I8: WasmOpcode = WasmOpcode(0xc0);
  /// i32.extend16_s
  pub const I32_SEXTEND_I16: WasmOpcode = WasmOpcode(0xc1);
  /// i64.extend8_s
  pub const I64_SEXTEND_I8: WasmOpcode = WasmOpcode(0xc2);
  /// i64.extend16_s
  pub const I64_SEXTEND_I16: WasmOpcode = WasmOpcode(0xc3);
  /// i64.extend32_s
  pub const I64_SEXTEND_I32: WasmOpcode = WasmOpcode(0xc4);
  /// cont.new
  pub const CONT_NEW: WasmOpcode = WasmOpcode(0xe0);
  /// cont.bind
  pub const CONT_BIND: WasmOpcode = WasmOpcode(0xe1);
  /// suspend
  pub const SUSPEND: WasmOpcode = WasmOpcode(0xe2);
  /// resume
  pub const RESUME: WasmOpcode = WasmOpcode(0xe3);
  /// resume_throw
  pub const RESUME_THROW: WasmOpcode = WasmOpcode(0xe4);
  /// resume_throw_ref
  pub const RESUME_THROW_REF: WasmOpcode = WasmOpcode(0xe5);
  /// switch
  pub const SWITCH: WasmOpcode = WasmOpcode(0xe6);
}

/// 前缀字节 (对应 C++ k*Prefix, FOREACH_PREFIX)
#[rustfmt::skip]
pub mod prefix {
  /// GC 前缀
  pub const GC: u32 = 0xfb;
  /// Numeric 前缀
  pub const NUMERIC: u32 = 0xfc;
  /// Simd 前缀
  pub const SIMD: u32 = 0xfd;
  /// Atomic 前缀
  pub const ATOMIC: u32 = 0xfe;
}

/// 带前缀操作码常量 (kExpr<Name> 形式的 0xfb00/0xfc00/0xfd00/0xfe00/0xfd1xx 编码)
#[rustfmt::skip]
pub mod prefixed_expr {
  use super::WasmOpcode;
  /// f32.load_f16
  pub const F32_LOAD_MEM_F16: WasmOpcode = WasmOpcode(0xfc30);
  /// f32.store_f16
  pub const F32_STORE_MEM_F16: WasmOpcode = WasmOpcode(0xfc31);
  /// v128.load
  pub const S128_LOAD_MEM: WasmOpcode = WasmOpcode(0xfd00);
  /// v128.load8x8_s
  pub const S128_LOAD8X8_S: WasmOpcode = WasmOpcode(0xfd01);
  /// v128.load8x8_u
  pub const S128_LOAD8X8_U: WasmOpcode = WasmOpcode(0xfd02);
  /// v128.load16x4_s
  pub const S128_LOAD16X4_S: WasmOpcode = WasmOpcode(0xfd03);
  /// v128.load16x4_u
  pub const S128_LOAD16X4_U: WasmOpcode = WasmOpcode(0xfd04);
  /// v128.load32x2_s
  pub const S128_LOAD32X2_S: WasmOpcode = WasmOpcode(0xfd05);
  /// v128.load32x2_u
  pub const S128_LOAD32X2_U: WasmOpcode = WasmOpcode(0xfd06);
  /// v128.load8_splat
  pub const S128_LOAD8_SPLAT: WasmOpcode = WasmOpcode(0xfd07);
  /// v128.load16_splat
  pub const S128_LOAD16_SPLAT: WasmOpcode = WasmOpcode(0xfd08);
  /// v128.load32_splat
  pub const S128_LOAD32_SPLAT: WasmOpcode = WasmOpcode(0xfd09);
  /// v128.load64_splat
  pub const S128_LOAD64_SPLAT: WasmOpcode = WasmOpcode(0xfd0a);
  /// v128.store
  pub const S128_STORE_MEM: WasmOpcode = WasmOpcode(0xfd0b);
  /// v128.load32_zero
  pub const S128_LOAD32_ZERO: WasmOpcode = WasmOpcode(0xfd5c);
  /// v128.load64_zero
  pub const S128_LOAD64_ZERO: WasmOpcode = WasmOpcode(0xfd5d);
  /// v128.load8_lane
  pub const S128_LOAD8_LANE: WasmOpcode = WasmOpcode(0xfd54);
  /// v128.load16_lane
  pub const S128_LOAD16_LANE: WasmOpcode = WasmOpcode(0xfd55);
  /// v128.load32_lane
  pub const S128_LOAD32_LANE: WasmOpcode = WasmOpcode(0xfd56);
  /// v128.load64_lane
  pub const S128_LOAD64_LANE: WasmOpcode = WasmOpcode(0xfd57);
  /// v128.store8_lane
  pub const S128_STORE8_LANE: WasmOpcode = WasmOpcode(0xfd58);
  /// v128.store16_lane
  pub const S128_STORE16_LANE: WasmOpcode = WasmOpcode(0xfd59);
  /// v128.store32_lane
  pub const S128_STORE32_LANE: WasmOpcode = WasmOpcode(0xfd5a);
  /// v128.store64_lane
  pub const S128_STORE64_LANE: WasmOpcode = WasmOpcode(0xfd5b);
  /// v128.const
  pub const S128_CONST: WasmOpcode = WasmOpcode(0xfd0c);
  /// i8x16.shuffle
  pub const I8X16_SHUFFLE: WasmOpcode = WasmOpcode(0xfd0d);
  /// i8x16.swizzle
  pub const I8X16_SWIZZLE: WasmOpcode = WasmOpcode(0xfd0e);
  /// i8x16.splat
  pub const I8X16_SPLAT: WasmOpcode = WasmOpcode(0xfd0f);
  /// i16x8.splat
  pub const I16X8_SPLAT: WasmOpcode = WasmOpcode(0xfd10);
  /// i32x4.splat
  pub const I32X4_SPLAT: WasmOpcode = WasmOpcode(0xfd11);
  /// i64x2.splat
  pub const I64X2_SPLAT: WasmOpcode = WasmOpcode(0xfd12);
  /// f32x4.splat
  pub const F32X4_SPLAT: WasmOpcode = WasmOpcode(0xfd13);
  /// f64x2.splat
  pub const F64X2_SPLAT: WasmOpcode = WasmOpcode(0xfd14);
  /// i8x16.eq
  pub const I8X16_EQ: WasmOpcode = WasmOpcode(0xfd23);
  /// i8x16.ne
  pub const I8X16_NE: WasmOpcode = WasmOpcode(0xfd24);
  /// i8x16.lt_s
  pub const I8X16_LT_S: WasmOpcode = WasmOpcode(0xfd25);
  /// i8x16.lt_u
  pub const I8X16_LT_U: WasmOpcode = WasmOpcode(0xfd26);
  /// i8x16.gt_s
  pub const I8X16_GT_S: WasmOpcode = WasmOpcode(0xfd27);
  /// i8x16.gt_u
  pub const I8X16_GT_U: WasmOpcode = WasmOpcode(0xfd28);
  /// i8x16.le_s
  pub const I8X16_LE_S: WasmOpcode = WasmOpcode(0xfd29);
  /// i8x16.le_u
  pub const I8X16_LE_U: WasmOpcode = WasmOpcode(0xfd2a);
  /// i8x16.ge_s
  pub const I8X16_GE_S: WasmOpcode = WasmOpcode(0xfd2b);
  /// i8x16.ge_u
  pub const I8X16_GE_U: WasmOpcode = WasmOpcode(0xfd2c);
  /// i16x8.eq
  pub const I16X8_EQ: WasmOpcode = WasmOpcode(0xfd2d);
  /// i16x8.ne
  pub const I16X8_NE: WasmOpcode = WasmOpcode(0xfd2e);
  /// i16x8.lt_s
  pub const I16X8_LT_S: WasmOpcode = WasmOpcode(0xfd2f);
  /// i16x8.lt_u
  pub const I16X8_LT_U: WasmOpcode = WasmOpcode(0xfd30);
  /// i16x8.gt_s
  pub const I16X8_GT_S: WasmOpcode = WasmOpcode(0xfd31);
  /// i16x8.gt_u
  pub const I16X8_GT_U: WasmOpcode = WasmOpcode(0xfd32);
  /// i16x8.le_s
  pub const I16X8_LE_S: WasmOpcode = WasmOpcode(0xfd33);
  /// i16x8.le_u
  pub const I16X8_LE_U: WasmOpcode = WasmOpcode(0xfd34);
  /// i16x8.ge_s
  pub const I16X8_GE_S: WasmOpcode = WasmOpcode(0xfd35);
  /// i16x8.ge_u
  pub const I16X8_GE_U: WasmOpcode = WasmOpcode(0xfd36);
  /// i32x4.eq
  pub const I32X4_EQ: WasmOpcode = WasmOpcode(0xfd37);
  /// i32x4.ne
  pub const I32X4_NE: WasmOpcode = WasmOpcode(0xfd38);
  /// i32x4.lt_s
  pub const I32X4_LT_S: WasmOpcode = WasmOpcode(0xfd39);
  /// i32x4.lt_u
  pub const I32X4_LT_U: WasmOpcode = WasmOpcode(0xfd3a);
  /// i32x4.gt_s
  pub const I32X4_GT_S: WasmOpcode = WasmOpcode(0xfd3b);
  /// i32x4.gt_u
  pub const I32X4_GT_U: WasmOpcode = WasmOpcode(0xfd3c);
  /// i32x4.le_s
  pub const I32X4_LE_S: WasmOpcode = WasmOpcode(0xfd3d);
  /// i32x4.le_u
  pub const I32X4_LE_U: WasmOpcode = WasmOpcode(0xfd3e);
  /// i32x4.ge_s
  pub const I32X4_GE_S: WasmOpcode = WasmOpcode(0xfd3f);
  /// i32x4.ge_u
  pub const I32X4_GE_U: WasmOpcode = WasmOpcode(0xfd40);
  /// f32x4.eq
  pub const F32X4_EQ: WasmOpcode = WasmOpcode(0xfd41);
  /// f32x4.ne
  pub const F32X4_NE: WasmOpcode = WasmOpcode(0xfd42);
  /// f32x4.lt
  pub const F32X4_LT: WasmOpcode = WasmOpcode(0xfd43);
  /// f32x4.gt
  pub const F32X4_GT: WasmOpcode = WasmOpcode(0xfd44);
  /// f32x4.le
  pub const F32X4_LE: WasmOpcode = WasmOpcode(0xfd45);
  /// f32x4.ge
  pub const F32X4_GE: WasmOpcode = WasmOpcode(0xfd46);
  /// f64x2.eq
  pub const F64X2_EQ: WasmOpcode = WasmOpcode(0xfd47);
  /// f64x2.ne
  pub const F64X2_NE: WasmOpcode = WasmOpcode(0xfd48);
  /// f64x2.lt
  pub const F64X2_LT: WasmOpcode = WasmOpcode(0xfd49);
  /// f64x2.gt
  pub const F64X2_GT: WasmOpcode = WasmOpcode(0xfd4a);
  /// f64x2.le
  pub const F64X2_LE: WasmOpcode = WasmOpcode(0xfd4b);
  /// f64x2.ge
  pub const F64X2_GE: WasmOpcode = WasmOpcode(0xfd4c);
  /// v128.not
  pub const S128_NOT: WasmOpcode = WasmOpcode(0xfd4d);
  /// v128.and
  pub const S128_AND: WasmOpcode = WasmOpcode(0xfd4e);
  /// v128.andnot
  pub const S128_AND_NOT: WasmOpcode = WasmOpcode(0xfd4f);
  /// v128.or
  pub const S128_OR: WasmOpcode = WasmOpcode(0xfd50);
  /// v128.xor
  pub const S128_XOR: WasmOpcode = WasmOpcode(0xfd51);
  /// v128.bitselect
  pub const S128_SELECT: WasmOpcode = WasmOpcode(0xfd52);
  /// v128.any_true
  pub const V128_ANY_TRUE: WasmOpcode = WasmOpcode(0xfd53);
  /// f32x4.demote_f64x2_zero
  pub const F32X4_DEMOTE_F64X2_ZERO: WasmOpcode = WasmOpcode(0xfd5e);
  /// f64x2.promote_low_f32x4
  pub const F64X2_PROMOTE_LOW_F32X4: WasmOpcode = WasmOpcode(0xfd5f);
  /// i8x16.abs
  pub const I8X16_ABS: WasmOpcode = WasmOpcode(0xfd60);
  /// i8x16.neg
  pub const I8X16_NEG: WasmOpcode = WasmOpcode(0xfd61);
  /// i8x16.popcnt
  pub const I8X16_POPCNT: WasmOpcode = WasmOpcode(0xfd62);
  /// i8x16.all_true
  pub const I8X16_ALL_TRUE: WasmOpcode = WasmOpcode(0xfd63);
  /// i8x16.bitmask
  pub const I8X16_BIT_MASK: WasmOpcode = WasmOpcode(0xfd64);
  /// i8x16.narrow_i16x8_s
  pub const I8X16_SCONVERT_I16X8: WasmOpcode = WasmOpcode(0xfd65);
  /// i8x16.narrow_i16x8_u
  pub const I8X16_UCONVERT_I16X8: WasmOpcode = WasmOpcode(0xfd66);
  /// f32x4.ceil
  pub const F32X4_CEIL: WasmOpcode = WasmOpcode(0xfd67);
  /// f32x4.floor
  pub const F32X4_FLOOR: WasmOpcode = WasmOpcode(0xfd68);
  /// f32x4.trunc
  pub const F32X4_TRUNC: WasmOpcode = WasmOpcode(0xfd69);
  /// f32x4.nearest
  pub const F32X4_NEAREST_INT: WasmOpcode = WasmOpcode(0xfd6a);
  /// i8x16.shl
  pub const I8X16_SHL: WasmOpcode = WasmOpcode(0xfd6b);
  /// i8x16.shr_s
  pub const I8X16_SHR_S: WasmOpcode = WasmOpcode(0xfd6c);
  /// i8x16.shr_u
  pub const I8X16_SHR_U: WasmOpcode = WasmOpcode(0xfd6d);
  /// i8x16.add
  pub const I8X16_ADD: WasmOpcode = WasmOpcode(0xfd6e);
  /// i8x16.add_sat_s
  pub const I8X16_ADD_SAT_S: WasmOpcode = WasmOpcode(0xfd6f);
  /// i8x16.add_sat_u
  pub const I8X16_ADD_SAT_U: WasmOpcode = WasmOpcode(0xfd70);
  /// i8x16.sub
  pub const I8X16_SUB: WasmOpcode = WasmOpcode(0xfd71);
  /// i8x16.sub_sat_s
  pub const I8X16_SUB_SAT_S: WasmOpcode = WasmOpcode(0xfd72);
  /// i8x16.sub_sat_u
  pub const I8X16_SUB_SAT_U: WasmOpcode = WasmOpcode(0xfd73);
  /// f64x2.ceil
  pub const F64X2_CEIL: WasmOpcode = WasmOpcode(0xfd74);
  /// f64x2.floor
  pub const F64X2_FLOOR: WasmOpcode = WasmOpcode(0xfd75);
  /// i8x16.min_s
  pub const I8X16_MIN_S: WasmOpcode = WasmOpcode(0xfd76);
  /// i8x16.min_u
  pub const I8X16_MIN_U: WasmOpcode = WasmOpcode(0xfd77);
  /// i8x16.max_s
  pub const I8X16_MAX_S: WasmOpcode = WasmOpcode(0xfd78);
  /// i8x16.max_u
  pub const I8X16_MAX_U: WasmOpcode = WasmOpcode(0xfd79);
  /// f64x2.trunc
  pub const F64X2_TRUNC: WasmOpcode = WasmOpcode(0xfd7a);
  /// i8x16.avgr_u
  pub const I8X16_ROUNDING_AVERAGE_U: WasmOpcode = WasmOpcode(0xfd7b);
  /// i16x8.extadd_pairwise_i8x16_s
  pub const I16X8_EXT_ADD_PAIRWISE_I8X16_S: WasmOpcode = WasmOpcode(0xfd7c);
  /// i16x8.extadd_pairwise_i8x16_u
  pub const I16X8_EXT_ADD_PAIRWISE_I8X16_U: WasmOpcode = WasmOpcode(0xfd7d);
  /// i32x4.extadd_pairwise_i16x8_s
  pub const I32X4_EXT_ADD_PAIRWISE_I16X8_S: WasmOpcode = WasmOpcode(0xfd7e);
  /// i32x4.extadd_pairwise_i16x8_u
  pub const I32X4_EXT_ADD_PAIRWISE_I16X8_U: WasmOpcode = WasmOpcode(0xfd7f);
  /// i16x8.abs
  pub const I16X8_ABS: WasmOpcode = WasmOpcode(0xfd80);
  /// i16x8.neg
  pub const I16X8_NEG: WasmOpcode = WasmOpcode(0xfd81);
  /// i16x8.q15mulr_sat_s
  pub const I16X8_Q15_MUL_RSAT_S: WasmOpcode = WasmOpcode(0xfd82);
  /// i16x8.all_true
  pub const I16X8_ALL_TRUE: WasmOpcode = WasmOpcode(0xfd83);
  /// i16x8.bitmask
  pub const I16X8_BIT_MASK: WasmOpcode = WasmOpcode(0xfd84);
  /// i16x8.narrow_i32x4_s
  pub const I16X8_SCONVERT_I32X4: WasmOpcode = WasmOpcode(0xfd85);
  /// i16x8.narrow_i32x4_u
  pub const I16X8_UCONVERT_I32X4: WasmOpcode = WasmOpcode(0xfd86);
  /// i16x8.extend_low_i8x16_s
  pub const I16X8_SCONVERT_I8X16_LOW: WasmOpcode = WasmOpcode(0xfd87);
  /// i16x8.extend_high_i8x16_s
  pub const I16X8_SCONVERT_I8X16_HIGH: WasmOpcode = WasmOpcode(0xfd88);
  /// i16x8.extend_low_i8x16_u
  pub const I16X8_UCONVERT_I8X16_LOW: WasmOpcode = WasmOpcode(0xfd89);
  /// i16x8.extend_high_i8x16_u
  pub const I16X8_UCONVERT_I8X16_HIGH: WasmOpcode = WasmOpcode(0xfd8a);
  /// i16x8.shl
  pub const I16X8_SHL: WasmOpcode = WasmOpcode(0xfd8b);
  /// i16x8.shr_s
  pub const I16X8_SHR_S: WasmOpcode = WasmOpcode(0xfd8c);
  /// i16x8.shr_u
  pub const I16X8_SHR_U: WasmOpcode = WasmOpcode(0xfd8d);
  /// i16x8.add
  pub const I16X8_ADD: WasmOpcode = WasmOpcode(0xfd8e);
  /// i16x8.add_sat_s
  pub const I16X8_ADD_SAT_S: WasmOpcode = WasmOpcode(0xfd8f);
  /// i16x8.add_sat_u
  pub const I16X8_ADD_SAT_U: WasmOpcode = WasmOpcode(0xfd90);
  /// i16x8.sub
  pub const I16X8_SUB: WasmOpcode = WasmOpcode(0xfd91);
  /// i16x8.sub_sat_s
  pub const I16X8_SUB_SAT_S: WasmOpcode = WasmOpcode(0xfd92);
  /// i16x8.sub_sat_u
  pub const I16X8_SUB_SAT_U: WasmOpcode = WasmOpcode(0xfd93);
  /// f64x2.nearest
  pub const F64X2_NEAREST_INT: WasmOpcode = WasmOpcode(0xfd94);
  /// i16x8.mul
  pub const I16X8_MUL: WasmOpcode = WasmOpcode(0xfd95);
  /// i16x8.min_s
  pub const I16X8_MIN_S: WasmOpcode = WasmOpcode(0xfd96);
  /// i16x8.min_u
  pub const I16X8_MIN_U: WasmOpcode = WasmOpcode(0xfd97);
  /// i16x8.max_s
  pub const I16X8_MAX_S: WasmOpcode = WasmOpcode(0xfd98);
  /// i16x8.max_u
  pub const I16X8_MAX_U: WasmOpcode = WasmOpcode(0xfd99);
  /// i16x8.avgr_u
  pub const I16X8_ROUNDING_AVERAGE_U: WasmOpcode = WasmOpcode(0xfd9b);
  /// i16x8.extmul_low_i8x16_s
  pub const I16X8_EXT_MUL_LOW_I8X16_S: WasmOpcode = WasmOpcode(0xfd9c);
  /// i16x8.extmul_high_i8x16_s
  pub const I16X8_EXT_MUL_HIGH_I8X16_S: WasmOpcode = WasmOpcode(0xfd9d);
  /// i16x8.extmul_low_i8x16_u
  pub const I16X8_EXT_MUL_LOW_I8X16_U: WasmOpcode = WasmOpcode(0xfd9e);
  /// i16x8.extmul_high_i8x16_u
  pub const I16X8_EXT_MUL_HIGH_I8X16_U: WasmOpcode = WasmOpcode(0xfd9f);
  /// i32x4.abs
  pub const I32X4_ABS: WasmOpcode = WasmOpcode(0xfda0);
  /// i32x4.neg
  pub const I32X4_NEG: WasmOpcode = WasmOpcode(0xfda1);
  /// i32x4.all_true
  pub const I32X4_ALL_TRUE: WasmOpcode = WasmOpcode(0xfda3);
  /// i32x4.bitmask
  pub const I32X4_BIT_MASK: WasmOpcode = WasmOpcode(0xfda4);
  /// i32x4.extend_low_i16x8_s
  pub const I32X4_SCONVERT_I16X8_LOW: WasmOpcode = WasmOpcode(0xfda7);
  /// i32x4.extend_high_i16x8_s
  pub const I32X4_SCONVERT_I16X8_HIGH: WasmOpcode = WasmOpcode(0xfda8);
  /// i32x4.extend_low_i16x8_u
  pub const I32X4_UCONVERT_I16X8_LOW: WasmOpcode = WasmOpcode(0xfda9);
  /// i32x4.extend_high_i16x8_u
  pub const I32X4_UCONVERT_I16X8_HIGH: WasmOpcode = WasmOpcode(0xfdaa);
  /// i32x4.shl
  pub const I32X4_SHL: WasmOpcode = WasmOpcode(0xfdab);
  /// i32x4.shr_s
  pub const I32X4_SHR_S: WasmOpcode = WasmOpcode(0xfdac);
  /// i32x4.shr_u
  pub const I32X4_SHR_U: WasmOpcode = WasmOpcode(0xfdad);
  /// i32x4.add
  pub const I32X4_ADD: WasmOpcode = WasmOpcode(0xfdae);
  /// i32x4.sub
  pub const I32X4_SUB: WasmOpcode = WasmOpcode(0xfdb1);
  /// i32x4.mul
  pub const I32X4_MUL: WasmOpcode = WasmOpcode(0xfdb5);
  /// i32x4.min_s
  pub const I32X4_MIN_S: WasmOpcode = WasmOpcode(0xfdb6);
  /// i32x4.min_u
  pub const I32X4_MIN_U: WasmOpcode = WasmOpcode(0xfdb7);
  /// i32x4.max_s
  pub const I32X4_MAX_S: WasmOpcode = WasmOpcode(0xfdb8);
  /// i32x4.max_u
  pub const I32X4_MAX_U: WasmOpcode = WasmOpcode(0xfdb9);
  /// i32x4.dot_i16x8_s
  pub const I32X4_DOT_I16X8_S: WasmOpcode = WasmOpcode(0xfdba);
  /// i32x4.extmul_low_i16x8_s
  pub const I32X4_EXT_MUL_LOW_I16X8_S: WasmOpcode = WasmOpcode(0xfdbc);
  /// i32x4.extmul_high_i16x8_s
  pub const I32X4_EXT_MUL_HIGH_I16X8_S: WasmOpcode = WasmOpcode(0xfdbd);
  /// i32x4.extmul_low_i16x8_u
  pub const I32X4_EXT_MUL_LOW_I16X8_U: WasmOpcode = WasmOpcode(0xfdbe);
  /// i32x4.extmul_high_i16x8_u
  pub const I32X4_EXT_MUL_HIGH_I16X8_U: WasmOpcode = WasmOpcode(0xfdbf);
  /// i64x2.abs
  pub const I64X2_ABS: WasmOpcode = WasmOpcode(0xfdc0);
  /// i64x2.neg
  pub const I64X2_NEG: WasmOpcode = WasmOpcode(0xfdc1);
  /// i64x2.all_true
  pub const I64X2_ALL_TRUE: WasmOpcode = WasmOpcode(0xfdc3);
  /// i64x2.bitmask
  pub const I64X2_BIT_MASK: WasmOpcode = WasmOpcode(0xfdc4);
  /// i64x2.extend_low_i32x4_s
  pub const I64X2_SCONVERT_I32X4_LOW: WasmOpcode = WasmOpcode(0xfdc7);
  /// i64x2.extend_high_i32x4_s
  pub const I64X2_SCONVERT_I32X4_HIGH: WasmOpcode = WasmOpcode(0xfdc8);
  /// i64x2.extend_low_i32x4_u
  pub const I64X2_UCONVERT_I32X4_LOW: WasmOpcode = WasmOpcode(0xfdc9);
  /// i64x2.extend_high_i32x4_u
  pub const I64X2_UCONVERT_I32X4_HIGH: WasmOpcode = WasmOpcode(0xfdca);
  /// i64x2.shl
  pub const I64X2_SHL: WasmOpcode = WasmOpcode(0xfdcb);
  /// i64x2.shr_s
  pub const I64X2_SHR_S: WasmOpcode = WasmOpcode(0xfdcc);
  /// i64x2.shr_u
  pub const I64X2_SHR_U: WasmOpcode = WasmOpcode(0xfdcd);
  /// i64x2.add
  pub const I64X2_ADD: WasmOpcode = WasmOpcode(0xfdce);
  /// i64x2.sub
  pub const I64X2_SUB: WasmOpcode = WasmOpcode(0xfdd1);
  /// i64x2.mul
  pub const I64X2_MUL: WasmOpcode = WasmOpcode(0xfdd5);
  /// i64x2.eq
  pub const I64X2_EQ: WasmOpcode = WasmOpcode(0xfdd6);
  /// i64x2.ne
  pub const I64X2_NE: WasmOpcode = WasmOpcode(0xfdd7);
  /// i64x2.lt_s
  pub const I64X2_LT_S: WasmOpcode = WasmOpcode(0xfdd8);
  /// i64x2.gt_s
  pub const I64X2_GT_S: WasmOpcode = WasmOpcode(0xfdd9);
  /// i64x2.le_s
  pub const I64X2_LE_S: WasmOpcode = WasmOpcode(0xfdda);
  /// i64x2.ge_s
  pub const I64X2_GE_S: WasmOpcode = WasmOpcode(0xfddb);
  /// i64x2.extmul_low_i32x4_s
  pub const I64X2_EXT_MUL_LOW_I32X4_S: WasmOpcode = WasmOpcode(0xfddc);
  /// i64x2.extmul_high_i32x4_s
  pub const I64X2_EXT_MUL_HIGH_I32X4_S: WasmOpcode = WasmOpcode(0xfddd);
  /// i64x2.extmul_low_i32x4_u
  pub const I64X2_EXT_MUL_LOW_I32X4_U: WasmOpcode = WasmOpcode(0xfdde);
  /// i64x2.extmul_high_i32x4_u
  pub const I64X2_EXT_MUL_HIGH_I32X4_U: WasmOpcode = WasmOpcode(0xfddf);
  /// f32x4.abs
  pub const F32X4_ABS: WasmOpcode = WasmOpcode(0xfde0);
  /// f32x4.neg
  pub const F32X4_NEG: WasmOpcode = WasmOpcode(0xfde1);
  /// f32x4.sqrt
  pub const F32X4_SQRT: WasmOpcode = WasmOpcode(0xfde3);
  /// f32x4.add
  pub const F32X4_ADD: WasmOpcode = WasmOpcode(0xfde4);
  /// f32x4.sub
  pub const F32X4_SUB: WasmOpcode = WasmOpcode(0xfde5);
  /// f32x4.mul
  pub const F32X4_MUL: WasmOpcode = WasmOpcode(0xfde6);
  /// f32x4.div
  pub const F32X4_DIV: WasmOpcode = WasmOpcode(0xfde7);
  /// f32x4.min
  pub const F32X4_MIN: WasmOpcode = WasmOpcode(0xfde8);
  /// f32x4.max
  pub const F32X4_MAX: WasmOpcode = WasmOpcode(0xfde9);
  /// f32x4.pmin
  pub const F32X4_PMIN: WasmOpcode = WasmOpcode(0xfdea);
  /// f32x4.pmax
  pub const F32X4_PMAX: WasmOpcode = WasmOpcode(0xfdeb);
  /// f64x2.abs
  pub const F64X2_ABS: WasmOpcode = WasmOpcode(0xfdec);
  /// f64x2.neg
  pub const F64X2_NEG: WasmOpcode = WasmOpcode(0xfded);
  /// f64x2.sqrt
  pub const F64X2_SQRT: WasmOpcode = WasmOpcode(0xfdef);
  /// f64x2.add
  pub const F64X2_ADD: WasmOpcode = WasmOpcode(0xfdf0);
  /// f64x2.sub
  pub const F64X2_SUB: WasmOpcode = WasmOpcode(0xfdf1);
  /// f64x2.mul
  pub const F64X2_MUL: WasmOpcode = WasmOpcode(0xfdf2);
  /// f64x2.div
  pub const F64X2_DIV: WasmOpcode = WasmOpcode(0xfdf3);
  /// f64x2.min
  pub const F64X2_MIN: WasmOpcode = WasmOpcode(0xfdf4);
  /// f64x2.max
  pub const F64X2_MAX: WasmOpcode = WasmOpcode(0xfdf5);
  /// f64x2.pmin
  pub const F64X2_PMIN: WasmOpcode = WasmOpcode(0xfdf6);
  /// f64x2.pmax
  pub const F64X2_PMAX: WasmOpcode = WasmOpcode(0xfdf7);
  /// i32x4.trunc_sat_f32x4_s
  pub const I32X4_SCONVERT_F32X4: WasmOpcode = WasmOpcode(0xfdf8);
  /// i32x4.trunc_sat_f32x4_u
  pub const I32X4_UCONVERT_F32X4: WasmOpcode = WasmOpcode(0xfdf9);
  /// f32x4.convert_i32x4_s
  pub const F32X4_SCONVERT_I32X4: WasmOpcode = WasmOpcode(0xfdfa);
  /// f32x4.convert_i32x4_u
  pub const F32X4_UCONVERT_I32X4: WasmOpcode = WasmOpcode(0xfdfb);
  /// i32x4.trunc_sat_f64x2_s_zero
  pub const I32X4_TRUNC_SAT_F64X2_SZERO: WasmOpcode = WasmOpcode(0xfdfc);
  /// i32x4.trunc_sat_f64x2_u_zero
  pub const I32X4_TRUNC_SAT_F64X2_UZERO: WasmOpcode = WasmOpcode(0xfdfd);
  /// f64x2.convert_low_i32x4_s
  pub const F64X2_CONVERT_LOW_I32X4_S: WasmOpcode = WasmOpcode(0xfdfe);
  /// f64x2.convert_low_i32x4_u
  pub const F64X2_CONVERT_LOW_I32X4_U: WasmOpcode = WasmOpcode(0xfdff);
  /// i8x16.relaxed_swizzle
  pub const I8X16_RELAXED_SWIZZLE: WasmOpcode = WasmOpcode(0xfd100);
  /// i32x4.relaxed_trunc_f32x4_s
  pub const I32X4_RELAXED_TRUNC_F32X4_S: WasmOpcode = WasmOpcode(0xfd101);
  /// i32x4.relaxed_trunc_f32x4_u
  pub const I32X4_RELAXED_TRUNC_F32X4_U: WasmOpcode = WasmOpcode(0xfd102);
  /// i32x4.relaxed_trunc_f64x2_s_zero
  pub const I32X4_RELAXED_TRUNC_F64X2_SZERO: WasmOpcode = WasmOpcode(0xfd103);
  /// i32x4.relaxed_trunc_f64x2_u_zero
  pub const I32X4_RELAXED_TRUNC_F64X2_UZERO: WasmOpcode = WasmOpcode(0xfd104);
  /// f32x4.qfma
  pub const F32X4_QFMA: WasmOpcode = WasmOpcode(0xfd105);
  /// f32x4.qfms
  pub const F32X4_QFMS: WasmOpcode = WasmOpcode(0xfd106);
  /// f64x2.qfma
  pub const F64X2_QFMA: WasmOpcode = WasmOpcode(0xfd107);
  /// f64x2.qfms
  pub const F64X2_QFMS: WasmOpcode = WasmOpcode(0xfd108);
  /// i8x16.relaxed_laneselect
  pub const I8X16_RELAXED_LANE_SELECT: WasmOpcode = WasmOpcode(0xfd109);
  /// i16x8.relaxed_laneselect
  pub const I16X8_RELAXED_LANE_SELECT: WasmOpcode = WasmOpcode(0xfd10a);
  /// i32x4.relaxed_laneselect
  pub const I32X4_RELAXED_LANE_SELECT: WasmOpcode = WasmOpcode(0xfd10b);
  /// i64x2.relaxed_laneselect
  pub const I64X2_RELAXED_LANE_SELECT: WasmOpcode = WasmOpcode(0xfd10c);
  /// f32x4.relaxed_min
  pub const F32X4_RELAXED_MIN: WasmOpcode = WasmOpcode(0xfd10d);
  /// f32x4.relaxed_max
  pub const F32X4_RELAXED_MAX: WasmOpcode = WasmOpcode(0xfd10e);
  /// f64x2.relaxed_min
  pub const F64X2_RELAXED_MIN: WasmOpcode = WasmOpcode(0xfd10f);
  /// f64x2.relaxed_max
  pub const F64X2_RELAXED_MAX: WasmOpcode = WasmOpcode(0xfd110);
  /// i16x8.relaxed_q15mulr_s
  pub const I16X8_RELAXED_Q15_MUL_RS: WasmOpcode = WasmOpcode(0xfd111);
  /// i16x8.dot_i8x16_i7x16_s
  pub const I16X8_DOT_I8X16_I7X16_S: WasmOpcode = WasmOpcode(0xfd112);
  /// i32x4.dot_i8x16_i7x16_add_s
  pub const I32X4_DOT_I8X16_I7X16_ADD_S: WasmOpcode = WasmOpcode(0xfd113);
  /// f16x8.splat
  pub const F16X8_SPLAT: WasmOpcode = WasmOpcode(0xfd120);
  /// f16x8.abs
  pub const F16X8_ABS: WasmOpcode = WasmOpcode(0xfd130);
  /// f16x8.neg
  pub const F16X8_NEG: WasmOpcode = WasmOpcode(0xfd131);
  /// f16x8.sqrt
  pub const F16X8_SQRT: WasmOpcode = WasmOpcode(0xfd132);
  /// f16x8.ceil
  pub const F16X8_CEIL: WasmOpcode = WasmOpcode(0xfd133);
  /// f16x8.floor
  pub const F16X8_FLOOR: WasmOpcode = WasmOpcode(0xfd134);
  /// f16x8.trunc
  pub const F16X8_TRUNC: WasmOpcode = WasmOpcode(0xfd135);
  /// f16x8.nearest
  pub const F16X8_NEAREST_INT: WasmOpcode = WasmOpcode(0xfd136);
  /// f16x8.eq
  pub const F16X8_EQ: WasmOpcode = WasmOpcode(0xfd137);
  /// f16x8.ne
  pub const F16X8_NE: WasmOpcode = WasmOpcode(0xfd138);
  /// f16x8.lt
  pub const F16X8_LT: WasmOpcode = WasmOpcode(0xfd139);
  /// f16x8.gt
  pub const F16X8_GT: WasmOpcode = WasmOpcode(0xfd13a);
  /// f16x8.le
  pub const F16X8_LE: WasmOpcode = WasmOpcode(0xfd13b);
  /// f16x8.ge
  pub const F16X8_GE: WasmOpcode = WasmOpcode(0xfd13c);
  /// f16x8.add
  pub const F16X8_ADD: WasmOpcode = WasmOpcode(0xfd13d);
  /// f16x8.sub
  pub const F16X8_SUB: WasmOpcode = WasmOpcode(0xfd13e);
  /// f16x8.mul
  pub const F16X8_MUL: WasmOpcode = WasmOpcode(0xfd13f);
  /// f16x8.div
  pub const F16X8_DIV: WasmOpcode = WasmOpcode(0xfd140);
  /// f16x8.min
  pub const F16X8_MIN: WasmOpcode = WasmOpcode(0xfd141);
  /// f16x8.max
  pub const F16X8_MAX: WasmOpcode = WasmOpcode(0xfd142);
  /// f16x8.pmin
  pub const F16X8_PMIN: WasmOpcode = WasmOpcode(0xfd143);
  /// f16x8.pmax
  pub const F16X8_PMAX: WasmOpcode = WasmOpcode(0xfd144);
  /// i16x8.trunc_sat_f16x8_s
  pub const I16X8_SCONVERT_F16X8: WasmOpcode = WasmOpcode(0xfd145);
  /// i16x8.trunc_sat_f16x8_u
  pub const I16X8_UCONVERT_F16X8: WasmOpcode = WasmOpcode(0xfd146);
  /// f16x8.convert_i16x8_s
  pub const F16X8_SCONVERT_I16X8: WasmOpcode = WasmOpcode(0xfd147);
  /// f16x8.convert_i16x8_u
  pub const F16X8_UCONVERT_I16X8: WasmOpcode = WasmOpcode(0xfd148);
  /// f16x8.demote_f32x4_zero
  pub const F16X8_DEMOTE_F32X4_ZERO: WasmOpcode = WasmOpcode(0xfd149);
  /// f16x8.demote_f64x2_zero
  pub const F16X8_DEMOTE_F64X2_ZERO: WasmOpcode = WasmOpcode(0xfd14a);
  /// f32x4.promote_low_f16x8
  pub const F32X4_PROMOTE_LOW_F16X8: WasmOpcode = WasmOpcode(0xfd14b);
  /// f16x8.madd
  pub const F16X8_QFMA: WasmOpcode = WasmOpcode(0xfd14e);
  /// f16x8.nmadd
  pub const F16X8_QFMS: WasmOpcode = WasmOpcode(0xfd14f);
  /// i8x16.extract_lane_s
  pub const I8X16_EXTRACT_LANE_S: WasmOpcode = WasmOpcode(0xfd15);
  /// i8x16.extract_lane_u
  pub const I8X16_EXTRACT_LANE_U: WasmOpcode = WasmOpcode(0xfd16);
  /// i16x8.extract_lane_s
  pub const I16X8_EXTRACT_LANE_S: WasmOpcode = WasmOpcode(0xfd18);
  /// i16x8.extract_lane_u
  pub const I16X8_EXTRACT_LANE_U: WasmOpcode = WasmOpcode(0xfd19);
  /// i32x4.extract_lane
  pub const I32X4_EXTRACT_LANE: WasmOpcode = WasmOpcode(0xfd1b);
  /// i64x2.extract_lane
  pub const I64X2_EXTRACT_LANE: WasmOpcode = WasmOpcode(0xfd1d);
  /// f32x4.extract_lane
  pub const F32X4_EXTRACT_LANE: WasmOpcode = WasmOpcode(0xfd1f);
  /// f64x2.extract_lane
  pub const F64X2_EXTRACT_LANE: WasmOpcode = WasmOpcode(0xfd21);
  /// f16x8.extract_lane
  pub const F16X8_EXTRACT_LANE: WasmOpcode = WasmOpcode(0xfd121);
  /// i8x16.replace_lane
  pub const I8X16_REPLACE_LANE: WasmOpcode = WasmOpcode(0xfd17);
  /// i16x8.replace_lane
  pub const I16X8_REPLACE_LANE: WasmOpcode = WasmOpcode(0xfd1a);
  /// i32x4.replace_lane
  pub const I32X4_REPLACE_LANE: WasmOpcode = WasmOpcode(0xfd1c);
  /// i64x2.replace_lane
  pub const I64X2_REPLACE_LANE: WasmOpcode = WasmOpcode(0xfd1e);
  /// f32x4.replace_lane
  pub const F32X4_REPLACE_LANE: WasmOpcode = WasmOpcode(0xfd20);
  /// f64x2.replace_lane
  pub const F64X2_REPLACE_LANE: WasmOpcode = WasmOpcode(0xfd22);
  /// f16x8.replace_lane
  pub const F16X8_REPLACE_LANE: WasmOpcode = WasmOpcode(0xfd122);
  /// i32.trunc_sat_f32_s
  pub const I32_SCONVERT_SAT_F32: WasmOpcode = WasmOpcode(0xfc00);
  /// i32.trunc_sat_f32_u
  pub const I32_UCONVERT_SAT_F32: WasmOpcode = WasmOpcode(0xfc01);
  /// i32.trunc_sat_f64_s
  pub const I32_SCONVERT_SAT_F64: WasmOpcode = WasmOpcode(0xfc02);
  /// i32.trunc_sat_f64_u
  pub const I32_UCONVERT_SAT_F64: WasmOpcode = WasmOpcode(0xfc03);
  /// i64.trunc_sat_f32_s
  pub const I64_SCONVERT_SAT_F32: WasmOpcode = WasmOpcode(0xfc04);
  /// i64.trunc_sat_f32_u
  pub const I64_UCONVERT_SAT_F32: WasmOpcode = WasmOpcode(0xfc05);
  /// i64.trunc_sat_f64_s
  pub const I64_SCONVERT_SAT_F64: WasmOpcode = WasmOpcode(0xfc06);
  /// i64.trunc_sat_f64_u
  pub const I64_UCONVERT_SAT_F64: WasmOpcode = WasmOpcode(0xfc07);
  /// data.drop
  pub const DATA_DROP: WasmOpcode = WasmOpcode(0xfc09);
  /// table.init
  pub const TABLE_INIT: WasmOpcode = WasmOpcode(0xfc0c);
  /// elem.drop
  pub const ELEM_DROP: WasmOpcode = WasmOpcode(0xfc0d);
  /// table.copy
  pub const TABLE_COPY: WasmOpcode = WasmOpcode(0xfc0e);
  /// table.size
  pub const TABLE_SIZE: WasmOpcode = WasmOpcode(0xfc10);
  /// i64.add128
  pub const I64_ADD128: WasmOpcode = WasmOpcode(0xfc13);
  /// i64.sub128
  pub const I64_SUB128: WasmOpcode = WasmOpcode(0xfc14);
  /// i64.mul_wide_s
  pub const I64_MUL_WIDE_S: WasmOpcode = WasmOpcode(0xfc15);
  /// i64.mul_wide_u
  pub const I64_MUL_WIDE_U: WasmOpcode = WasmOpcode(0xfc16);
  /// memory.init
  pub const MEMORY_INIT: WasmOpcode = WasmOpcode(0xfc08);
  /// memory.copy
  pub const MEMORY_COPY: WasmOpcode = WasmOpcode(0xfc0a);
  /// memory.fill
  pub const MEMORY_FILL: WasmOpcode = WasmOpcode(0xfc0b);
  /// table.grow
  pub const TABLE_GROW: WasmOpcode = WasmOpcode(0xfc0f);
  /// table.fill
  pub const TABLE_FILL: WasmOpcode = WasmOpcode(0xfc11);
  /// memory.atomic.notify
  pub const ATOMIC_NOTIFY: WasmOpcode = WasmOpcode(0xfe00);
  /// memory.atomic.wait32
  pub const I32_ATOMIC_WAIT: WasmOpcode = WasmOpcode(0xfe01);
  /// memory.atomic.wait64
  pub const I64_ATOMIC_WAIT: WasmOpcode = WasmOpcode(0xfe02);
  /// i32.atomic.load
  pub const I32_ATOMIC_LOAD: WasmOpcode = WasmOpcode(0xfe10);
  /// i64.atomic.load
  pub const I64_ATOMIC_LOAD: WasmOpcode = WasmOpcode(0xfe11);
  /// i32.atomic.load8_u
  pub const I32_ATOMIC_LOAD8_U: WasmOpcode = WasmOpcode(0xfe12);
  /// i32.atomic.load16_u
  pub const I32_ATOMIC_LOAD16_U: WasmOpcode = WasmOpcode(0xfe13);
  /// i64.atomic.load8_u
  pub const I64_ATOMIC_LOAD8_U: WasmOpcode = WasmOpcode(0xfe14);
  /// i64.atomic.load16_u
  pub const I64_ATOMIC_LOAD16_U: WasmOpcode = WasmOpcode(0xfe15);
  /// i64.atomic.load32_u
  pub const I64_ATOMIC_LOAD32_U: WasmOpcode = WasmOpcode(0xfe16);
  /// i32.atomic.store
  pub const I32_ATOMIC_STORE: WasmOpcode = WasmOpcode(0xfe17);
  /// i64.atomic.store
  pub const I64_ATOMIC_STORE: WasmOpcode = WasmOpcode(0xfe18);
  /// i32.atomic.store8
  pub const I32_ATOMIC_STORE8_U: WasmOpcode = WasmOpcode(0xfe19);
  /// i32.atomic.store16
  pub const I32_ATOMIC_STORE16_U: WasmOpcode = WasmOpcode(0xfe1a);
  /// i64.atomic.store8
  pub const I64_ATOMIC_STORE8_U: WasmOpcode = WasmOpcode(0xfe1b);
  /// i64.atomic.store16
  pub const I64_ATOMIC_STORE16_U: WasmOpcode = WasmOpcode(0xfe1c);
  /// i64.atomic.store32
  pub const I64_ATOMIC_STORE32_U: WasmOpcode = WasmOpcode(0xfe1d);
  /// i32.atomic.rmw.add
  pub const I32_ATOMIC_ADD: WasmOpcode = WasmOpcode(0xfe1e);
  /// i64.atomic.rmw.add
  pub const I64_ATOMIC_ADD: WasmOpcode = WasmOpcode(0xfe1f);
  /// i32.atomic.rmw8.add_u
  pub const I32_ATOMIC_ADD8_U: WasmOpcode = WasmOpcode(0xfe20);
  /// i32.atomic.rmw16.add_u
  pub const I32_ATOMIC_ADD16_U: WasmOpcode = WasmOpcode(0xfe21);
  /// i64.atomic.rmw8.add_u
  pub const I64_ATOMIC_ADD8_U: WasmOpcode = WasmOpcode(0xfe22);
  /// i64.atomic.rmw16.add_u
  pub const I64_ATOMIC_ADD16_U: WasmOpcode = WasmOpcode(0xfe23);
  /// i64.atomic.rmw32.add_u
  pub const I64_ATOMIC_ADD32_U: WasmOpcode = WasmOpcode(0xfe24);
  /// i32.atomic.rmw.sub
  pub const I32_ATOMIC_SUB: WasmOpcode = WasmOpcode(0xfe25);
  /// i64.atomic.rmw.sub
  pub const I64_ATOMIC_SUB: WasmOpcode = WasmOpcode(0xfe26);
  /// i32.atomic.rmw8.sub_u
  pub const I32_ATOMIC_SUB8_U: WasmOpcode = WasmOpcode(0xfe27);
  /// i32.atomic.rmw16.sub_u
  pub const I32_ATOMIC_SUB16_U: WasmOpcode = WasmOpcode(0xfe28);
  /// i64.atomic.rmw8.sub_u
  pub const I64_ATOMIC_SUB8_U: WasmOpcode = WasmOpcode(0xfe29);
  /// i64.atomic.rmw16.sub_u
  pub const I64_ATOMIC_SUB16_U: WasmOpcode = WasmOpcode(0xfe2a);
  /// i64.atomic.rmw32.sub_u
  pub const I64_ATOMIC_SUB32_U: WasmOpcode = WasmOpcode(0xfe2b);
  /// i32.atomic.rmw.and
  pub const I32_ATOMIC_AND: WasmOpcode = WasmOpcode(0xfe2c);
  /// i64.atomic.rmw.and
  pub const I64_ATOMIC_AND: WasmOpcode = WasmOpcode(0xfe2d);
  /// i32.atomic.rmw8.and_u
  pub const I32_ATOMIC_AND8_U: WasmOpcode = WasmOpcode(0xfe2e);
  /// i32.atomic.rmw16.and_u
  pub const I32_ATOMIC_AND16_U: WasmOpcode = WasmOpcode(0xfe2f);
  /// i64.atomic.rmw8.and_u
  pub const I64_ATOMIC_AND8_U: WasmOpcode = WasmOpcode(0xfe30);
  /// i64.atomic.rmw16.and_u
  pub const I64_ATOMIC_AND16_U: WasmOpcode = WasmOpcode(0xfe31);
  /// i64.atomic.rmw32.and_u
  pub const I64_ATOMIC_AND32_U: WasmOpcode = WasmOpcode(0xfe32);
  /// i32.atomic.rmw.or
  pub const I32_ATOMIC_OR: WasmOpcode = WasmOpcode(0xfe33);
  /// i64.atomic.rmw.or
  pub const I64_ATOMIC_OR: WasmOpcode = WasmOpcode(0xfe34);
  /// i32.atomic.rmw8.or_u
  pub const I32_ATOMIC_OR8_U: WasmOpcode = WasmOpcode(0xfe35);
  /// i32.atomic.rmw16.or_u
  pub const I32_ATOMIC_OR16_U: WasmOpcode = WasmOpcode(0xfe36);
  /// i64.atomic.rmw8.or_u
  pub const I64_ATOMIC_OR8_U: WasmOpcode = WasmOpcode(0xfe37);
  /// i64.atomic.rmw16.or_u
  pub const I64_ATOMIC_OR16_U: WasmOpcode = WasmOpcode(0xfe38);
  /// i64.atomic.rmw32.or_u
  pub const I64_ATOMIC_OR32_U: WasmOpcode = WasmOpcode(0xfe39);
  /// i32.atomic.rmw.xor
  pub const I32_ATOMIC_XOR: WasmOpcode = WasmOpcode(0xfe3a);
  /// i64.atomic.rmw.xor
  pub const I64_ATOMIC_XOR: WasmOpcode = WasmOpcode(0xfe3b);
  /// i32.atomic.rmw8.xor_u
  pub const I32_ATOMIC_XOR8_U: WasmOpcode = WasmOpcode(0xfe3c);
  /// i32.atomic.rmw16.xor_u
  pub const I32_ATOMIC_XOR16_U: WasmOpcode = WasmOpcode(0xfe3d);
  /// i64.atomic.rmw8.xor_u
  pub const I64_ATOMIC_XOR8_U: WasmOpcode = WasmOpcode(0xfe3e);
  /// i64.atomic.rmw16.xor_u
  pub const I64_ATOMIC_XOR16_U: WasmOpcode = WasmOpcode(0xfe3f);
  /// i64.atomic.rmw32.xor_u
  pub const I64_ATOMIC_XOR32_U: WasmOpcode = WasmOpcode(0xfe40);
  /// i32.atomic.rmw.xchg
  pub const I32_ATOMIC_EXCHANGE: WasmOpcode = WasmOpcode(0xfe41);
  /// i64.atomic.rmw.xchg
  pub const I64_ATOMIC_EXCHANGE: WasmOpcode = WasmOpcode(0xfe42);
  /// i32.atomic.rmw8.xchg_u
  pub const I32_ATOMIC_EXCHANGE8_U: WasmOpcode = WasmOpcode(0xfe43);
  /// i32.atomic.rmw16.xchg_u
  pub const I32_ATOMIC_EXCHANGE16_U: WasmOpcode = WasmOpcode(0xfe44);
  /// i64.atomic.rmw8.xchg_u
  pub const I64_ATOMIC_EXCHANGE8_U: WasmOpcode = WasmOpcode(0xfe45);
  /// i64.atomic.rmw16.xchg_u
  pub const I64_ATOMIC_EXCHANGE16_U: WasmOpcode = WasmOpcode(0xfe46);
  /// i64.atomic.rmw32.xchg_u
  pub const I64_ATOMIC_EXCHANGE32_U: WasmOpcode = WasmOpcode(0xfe47);
  /// i32.atomic.rmw.cmpxchg
  pub const I32_ATOMIC_COMPARE_EXCHANGE: WasmOpcode = WasmOpcode(0xfe48);
  /// i64.atomic.rmw.cmpxchg
  pub const I64_ATOMIC_COMPARE_EXCHANGE: WasmOpcode = WasmOpcode(0xfe49);
  /// i32.atomic.rmw8.cmpxchg_u
  pub const I32_ATOMIC_COMPARE_EXCHANGE8_U: WasmOpcode = WasmOpcode(0xfe4a);
  /// i32.atomic.rmw16.cmpxchg_u
  pub const I32_ATOMIC_COMPARE_EXCHANGE16_U: WasmOpcode = WasmOpcode(0xfe4b);
  /// i64.atomic.rmw8.cmpxchg_u
  pub const I64_ATOMIC_COMPARE_EXCHANGE8_U: WasmOpcode = WasmOpcode(0xfe4c);
  /// i64.atomic.rmw16.cmpxchg_u
  pub const I64_ATOMIC_COMPARE_EXCHANGE16_U: WasmOpcode = WasmOpcode(0xfe4d);
  /// i64.atomic.rmw32.cmpxchg_u
  pub const I64_ATOMIC_COMPARE_EXCHANGE32_U: WasmOpcode = WasmOpcode(0xfe4e);
  /// atomic.fence
  pub const ATOMIC_FENCE: WasmOpcode = WasmOpcode(0xfe03);
  /// struct.new
  pub const STRUCT_NEW: WasmOpcode = WasmOpcode(0xfb00);
  /// struct.new_default
  pub const STRUCT_NEW_DEFAULT: WasmOpcode = WasmOpcode(0xfb01);
  /// struct.get
  pub const STRUCT_GET: WasmOpcode = WasmOpcode(0xfb02);
  /// struct.get_s
  pub const STRUCT_GET_S: WasmOpcode = WasmOpcode(0xfb03);
  /// struct.get_u
  pub const STRUCT_GET_U: WasmOpcode = WasmOpcode(0xfb04);
  /// struct.set
  pub const STRUCT_SET: WasmOpcode = WasmOpcode(0xfb05);
  /// array.new
  pub const ARRAY_NEW: WasmOpcode = WasmOpcode(0xfb06);
  /// array.new_default
  pub const ARRAY_NEW_DEFAULT: WasmOpcode = WasmOpcode(0xfb07);
  /// array.new_fixed
  pub const ARRAY_NEW_FIXED: WasmOpcode = WasmOpcode(0xfb08);
  /// array.new_data
  pub const ARRAY_NEW_DATA: WasmOpcode = WasmOpcode(0xfb09);
  /// array.new_elem
  pub const ARRAY_NEW_ELEM: WasmOpcode = WasmOpcode(0xfb0a);
  /// array.get
  pub const ARRAY_GET: WasmOpcode = WasmOpcode(0xfb0b);
  /// array.get_s
  pub const ARRAY_GET_S: WasmOpcode = WasmOpcode(0xfb0c);
  /// array.get_u
  pub const ARRAY_GET_U: WasmOpcode = WasmOpcode(0xfb0d);
  /// array.set
  pub const ARRAY_SET: WasmOpcode = WasmOpcode(0xfb0e);
  /// array.len
  pub const ARRAY_LEN: WasmOpcode = WasmOpcode(0xfb0f);
  /// array.fill
  pub const ARRAY_FILL: WasmOpcode = WasmOpcode(0xfb10);
  /// array.copy
  pub const ARRAY_COPY: WasmOpcode = WasmOpcode(0xfb11);
  /// array.init_data
  pub const ARRAY_INIT_DATA: WasmOpcode = WasmOpcode(0xfb12);
  /// array.init_elem
  pub const ARRAY_INIT_ELEM: WasmOpcode = WasmOpcode(0xfb13);
  /// ref.test
  pub const REF_TEST: WasmOpcode = WasmOpcode(0xfb14);
  /// ref.test null
  pub const REF_TEST_NULL: WasmOpcode = WasmOpcode(0xfb15);
  /// ref.cast
  pub const REF_CAST: WasmOpcode = WasmOpcode(0xfb16);
  /// ref.cast null
  pub const REF_CAST_NULL: WasmOpcode = WasmOpcode(0xfb17);
  /// br_on_cast
  pub const BR_ON_CAST: WasmOpcode = WasmOpcode(0xfb18);
  /// br_on_cast_fail
  pub const BR_ON_CAST_FAIL: WasmOpcode = WasmOpcode(0xfb19);
  /// any.convert_extern
  pub const ANY_CONVERT_EXTERN: WasmOpcode = WasmOpcode(0xfb1a);
  /// extern.convert_any
  pub const EXTERN_CONVERT_ANY: WasmOpcode = WasmOpcode(0xfb1b);
  /// ref.i31
  pub const REF_I31: WasmOpcode = WasmOpcode(0xfb1c);
  /// i31.get_s
  pub const I31_GET_S: WasmOpcode = WasmOpcode(0xfb1d);
  /// i31.get_u
  pub const I31_GET_U: WasmOpcode = WasmOpcode(0xfb1e);
  /// ref.i31_shared
  pub const REF_I31_SHARED: WasmOpcode = WasmOpcode(0xfb1f);
  /// struct.new_desc
  pub const STRUCT_NEW_DESC: WasmOpcode = WasmOpcode(0xfb20);
  /// struct.new_default_desc
  pub const STRUCT_NEW_DEFAULT_DESC: WasmOpcode = WasmOpcode(0xfb21);
  /// ref.get_desc
  pub const REF_GET_DESC: WasmOpcode = WasmOpcode(0xfb22);
  /// ref.cast_desc_eq
  pub const REF_CAST_DESC_EQ: WasmOpcode = WasmOpcode(0xfb23);
  /// ref.cast_desc_eq null
  pub const REF_CAST_DESC_EQ_NULL: WasmOpcode = WasmOpcode(0xfb24);
  /// br_on_cast_desc_eq
  pub const BR_ON_CAST_DESC_EQ: WasmOpcode = WasmOpcode(0xfb25);
  /// br_on_cast_desc_eq_fail
  pub const BR_ON_CAST_DESC_EQ_FAIL: WasmOpcode = WasmOpcode(0xfb26);
  /// ref.cast_nop
  pub const REF_CAST_NOP: WasmOpcode = WasmOpcode(0xfb4c);
  /// string.new_utf8
  pub const STRING_NEW_UTF8: WasmOpcode = WasmOpcode(0xfb80);
  /// string.new_wtf16
  pub const STRING_NEW_WTF16: WasmOpcode = WasmOpcode(0xfb81);
  /// string.const
  pub const STRING_CONST: WasmOpcode = WasmOpcode(0xfb82);
  /// string.measure_utf8
  pub const STRING_MEASURE_UTF8: WasmOpcode = WasmOpcode(0xfb83);
  /// string.measure_wtf8
  pub const STRING_MEASURE_WTF8: WasmOpcode = WasmOpcode(0xfb84);
  /// string.measure_wtf16
  pub const STRING_MEASURE_WTF16: WasmOpcode = WasmOpcode(0xfb85);
  /// string.encode_utf8
  pub const STRING_ENCODE_UTF8: WasmOpcode = WasmOpcode(0xfb86);
  /// string.encode_wtf16
  pub const STRING_ENCODE_WTF16: WasmOpcode = WasmOpcode(0xfb87);
  /// string.concat
  pub const STRING_CONCAT: WasmOpcode = WasmOpcode(0xfb88);
  /// string.eq
  pub const STRING_EQ: WasmOpcode = WasmOpcode(0xfb89);
  /// string.is_usv_sequence
  pub const STRING_IS_USVSEQUENCE: WasmOpcode = WasmOpcode(0xfb8a);
  /// string.new_lossy_utf8
  pub const STRING_NEW_LOSSY_UTF8: WasmOpcode = WasmOpcode(0xfb8b);
  /// string.new_wtf8
  pub const STRING_NEW_WTF8: WasmOpcode = WasmOpcode(0xfb8c);
  /// string.encode_lossy_utf8
  pub const STRING_ENCODE_LOSSY_UTF8: WasmOpcode = WasmOpcode(0xfb8d);
  /// string.encode_wtf8
  pub const STRING_ENCODE_WTF8: WasmOpcode = WasmOpcode(0xfb8e);
  /// string.new_utf8_try
  pub const STRING_NEW_UTF8_TRY: WasmOpcode = WasmOpcode(0xfb8f);
  /// string.as_wtf8
  pub const STRING_AS_WTF8: WasmOpcode = WasmOpcode(0xfb90);
  /// stringview_wtf8.advance
  pub const STRING_VIEW_WTF8_ADVANCE: WasmOpcode = WasmOpcode(0xfb91);
  /// stringview_wtf8.encode_utf8
  pub const STRING_VIEW_WTF8_ENCODE_UTF8: WasmOpcode = WasmOpcode(0xfb92);
  /// stringview_wtf8.slice
  pub const STRING_VIEW_WTF8_SLICE: WasmOpcode = WasmOpcode(0xfb93);
  /// stringview_wtf8.encode_lossy_utf8
  pub const STRING_VIEW_WTF8_ENCODE_LOSSY_UTF8: WasmOpcode = WasmOpcode(0xfb94);
  /// stringview_wtf8.encode_wtf8
  pub const STRING_VIEW_WTF8_ENCODE_WTF8: WasmOpcode = WasmOpcode(0xfb95);
  /// string.as_wtf16
  pub const STRING_AS_WTF16: WasmOpcode = WasmOpcode(0xfb98);
  /// stringview_wtf16.length
  pub const STRING_VIEW_WTF16_LENGTH: WasmOpcode = WasmOpcode(0xfb99);
  /// stringview_wtf16.get_codeunit
  pub const STRING_VIEW_WTF16_GET_CODEUNIT: WasmOpcode = WasmOpcode(0xfb9a);
  /// stringview_wtf16.encode
  pub const STRING_VIEW_WTF16_ENCODE: WasmOpcode = WasmOpcode(0xfb9b);
  /// stringview_wtf16.slice
  pub const STRING_VIEW_WTF16_SLICE: WasmOpcode = WasmOpcode(0xfb9c);
  /// string.as_iter
  pub const STRING_AS_ITER: WasmOpcode = WasmOpcode(0xfba0);
  /// stringview_iter.next
  pub const STRING_VIEW_ITER_NEXT: WasmOpcode = WasmOpcode(0xfba1);
  /// stringview_iter.advance
  pub const STRING_VIEW_ITER_ADVANCE: WasmOpcode = WasmOpcode(0xfba2);
  /// stringview_iter.rewind
  pub const STRING_VIEW_ITER_REWIND: WasmOpcode = WasmOpcode(0xfba3);
  /// stringview_iter.slice
  pub const STRING_VIEW_ITER_SLICE: WasmOpcode = WasmOpcode(0xfba4);
  /// string.compare
  pub const STRING_COMPARE: WasmOpcode = WasmOpcode(0xfba8);
  /// string.from_code_point
  pub const STRING_FROM_CODE_POINT: WasmOpcode = WasmOpcode(0xfba9);
  /// string.hash
  pub const STRING_HASH: WasmOpcode = WasmOpcode(0xfbaa);
  /// string.new_utf8_array
  pub const STRING_NEW_UTF8_ARRAY: WasmOpcode = WasmOpcode(0xfbb0);
  /// string.new_wtf16_array
  pub const STRING_NEW_WTF16_ARRAY: WasmOpcode = WasmOpcode(0xfbb1);
  /// string.encode_utf8_array
  pub const STRING_ENCODE_UTF8_ARRAY: WasmOpcode = WasmOpcode(0xfbb2);
  /// string.encode_wtf16_array
  pub const STRING_ENCODE_WTF16_ARRAY: WasmOpcode = WasmOpcode(0xfbb3);
  /// string.new_lossy_utf8_array
  pub const STRING_NEW_LOSSY_UTF8_ARRAY: WasmOpcode = WasmOpcode(0xfbb4);
  /// string.new_wtf8_array
  pub const STRING_NEW_WTF8_ARRAY: WasmOpcode = WasmOpcode(0xfbb5);
  /// string.encode_lossy_utf8_array
  pub const STRING_ENCODE_LOSSY_UTF8_ARRAY: WasmOpcode = WasmOpcode(0xfbb6);
  /// string.encode_wtf8_array
  pub const STRING_ENCODE_WTF8_ARRAY: WasmOpcode = WasmOpcode(0xfbb7);
  /// string.new_utf8_array_try
  pub const STRING_NEW_UTF8_ARRAY_TRY: WasmOpcode = WasmOpcode(0xfbb8);
  /// pause
  pub const PAUSE: WasmOpcode = WasmOpcode(0xfe04);
  /// struct.wait
  pub const STRUCT_WAIT: WasmOpcode = WasmOpcode(0xfe05);
  /// waitqueue.notify
  pub const WAITQUEUE_NOTIFY: WasmOpcode = WasmOpcode(0xfe06);
  /// waitqueue.new
  pub const WAITQUEUE_NEW: WasmOpcode = WasmOpcode(0xfe07);
  /// array.wait
  pub const ARRAY_WAIT: WasmOpcode = WasmOpcode(0xfe08);
  /// publish
  pub const PUBLISH: WasmOpcode = WasmOpcode(0xfe0f);
  /// struct.atomic.get
  pub const STRUCT_ATOMIC_GET: WasmOpcode = WasmOpcode(0xfe5c);
  /// struct.atomic.get_s
  pub const STRUCT_ATOMIC_GET_S: WasmOpcode = WasmOpcode(0xfe5d);
  /// struct.atomic.get_u
  pub const STRUCT_ATOMIC_GET_U: WasmOpcode = WasmOpcode(0xfe5e);
  /// struct.atomic.set
  pub const STRUCT_ATOMIC_SET: WasmOpcode = WasmOpcode(0xfe5f);
  /// struct.atomic.rmw.add
  pub const STRUCT_ATOMIC_ADD: WasmOpcode = WasmOpcode(0xfe60);
  /// struct.atomic.rmw.sub
  pub const STRUCT_ATOMIC_SUB: WasmOpcode = WasmOpcode(0xfe61);
  /// struct.atomic.rmw.and
  pub const STRUCT_ATOMIC_AND: WasmOpcode = WasmOpcode(0xfe62);
  /// struct.atomic.rmw.or
  pub const STRUCT_ATOMIC_OR: WasmOpcode = WasmOpcode(0xfe63);
  /// struct.atomic.rmw.xor
  pub const STRUCT_ATOMIC_XOR: WasmOpcode = WasmOpcode(0xfe64);
  /// struct.atomic.rmw.xchg
  pub const STRUCT_ATOMIC_EXCHANGE: WasmOpcode = WasmOpcode(0xfe65);
  /// struct.atomic.rmw.cmpxchg
  pub const STRUCT_ATOMIC_COMPARE_EXCHANGE: WasmOpcode = WasmOpcode(0xfe66);
  /// array.atomic.get
  pub const ARRAY_ATOMIC_GET: WasmOpcode = WasmOpcode(0xfe67);
  /// array.atomic.get_s
  pub const ARRAY_ATOMIC_GET_S: WasmOpcode = WasmOpcode(0xfe68);
  /// array.atomic.get_u
  pub const ARRAY_ATOMIC_GET_U: WasmOpcode = WasmOpcode(0xfe69);
  /// array.atomic.set
  pub const ARRAY_ATOMIC_SET: WasmOpcode = WasmOpcode(0xfe6a);
  /// array.atomic.rmw.add
  pub const ARRAY_ATOMIC_ADD: WasmOpcode = WasmOpcode(0xfe6b);
  /// array.atomic.rmw.sub
  pub const ARRAY_ATOMIC_SUB: WasmOpcode = WasmOpcode(0xfe6c);
  /// array.atomic.rmw.and
  pub const ARRAY_ATOMIC_AND: WasmOpcode = WasmOpcode(0xfe6d);
  /// array.atomic.rmw.or
  pub const ARRAY_ATOMIC_OR: WasmOpcode = WasmOpcode(0xfe6e);
  /// array.atomic.rmw.xor
  pub const ARRAY_ATOMIC_XOR: WasmOpcode = WasmOpcode(0xfe6f);
  /// array.atomic.rmw.xchg
  pub const ARRAY_ATOMIC_EXCHANGE: WasmOpcode = WasmOpcode(0xfe70);
  /// array.atomic.rmw.cmpxchg
  pub const ARRAY_ATOMIC_COMPARE_EXCHANGE: WasmOpcode = WasmOpcode(0xfe71);
}

/// 操作码签名种类 (对应 C++ enum WasmOpcodeSig, kSigEnum_*; FOREACH_SIGNATURE 展开 59 + None)
/// variant 逐字保留原版签名短名 (S_S/V_IS..), 与 wasm-opcodes.h FOREACH_SIGNATURE 对位检索, 不改写
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WasmOpcodeSig {
  /// 无签名 (kSigEnum_None)
  None = 0,
  /// s_s: (S128) -> S128
  S_S = 1,
  /// s_f: (F32) -> S128
  S_F = 2,
  /// s_d: (F64) -> S128
  S_D = 3,
  /// s_ss: (S128, S128) -> S128
  S_SS = 4,
  /// s_i: (I32) -> S128
  S_I = 5,
  /// s_l: (I64) -> S128
  S_L = 6,
  /// s_si: (S128, I32) -> S128
  S_SI = 7,
  /// i_s: (S128) -> I32
  I_S = 8,
  /// v_is: (I32, S128) -> Void
  V_IS = 9,
  /// s_sss: (S128, S128, S128) -> S128
  S_SSS = 10,
  /// s_is: (I32, S128) -> S128
  S_IS = 11,
  /// d_d: (F64) -> F64
  D_D = 12,
  /// d_dd: (F64, F64) -> F64
  D_DD = 13,
  /// d_f: (F32) -> F64
  D_F = 14,
  /// d_i: (I32) -> F64
  D_I = 15,
  /// d_id: (I32, F64) -> F64
  D_ID = 16,
  /// d_l: (I64) -> F64
  D_L = 17,
  /// f_d: (F64) -> F32
  F_D = 18,
  /// f_f: (F32) -> F32
  F_F = 19,
  /// f_ff: (F32, F32) -> F32
  F_FF = 20,
  /// f_i: (I32) -> F32
  F_I = 21,
  /// f_if: (I32, F32) -> F32
  F_IF = 22,
  /// f_l: (I64) -> F32
  F_L = 23,
  /// i_a: (AnyRef) -> I32
  I_A = 24,
  /// i_ci: (FuncRef, I32) -> I32
  I_CI = 25,
  /// i_d: (F64) -> I32
  I_D = 26,
  /// i_dd: (F64, F64) -> I32
  I_DD = 27,
  /// i_f: (F32) -> I32
  I_F = 28,
  /// i_ff: (F32, F32) -> I32
  I_FF = 29,
  /// i_i: (I32) -> I32
  I_I = 30,
  /// i_ii: (I32, I32) -> I32
  I_II = 31,
  /// i_iii: (I32, I32, I32) -> I32
  I_III = 32,
  /// i_iil: (I32, I32, I64) -> I32
  I_IIL = 33,
  /// i_ill: (I32, I64, I64) -> I32
  I_ILL = 34,
  /// i_l: (I64) -> I32
  I_L = 35,
  /// i_li: (I64, I32) -> I32
  I_LI = 36,
  /// i_lii: (I64, I32, I32) -> I32
  I_LII = 37,
  /// i_lil: (I64, I32, I64) -> I32
  I_LIL = 38,
  /// i_lll: (I64, I64, I64) -> I32
  I_LLL = 39,
  /// i_ll: (I64, I64) -> I32
  I_LL = 40,
  /// i_qq: (EqRef, EqRef) -> I32
  I_QQ = 41,
  /// i_v: () -> I32
  I_V = 42,
  /// l_d: (F64) -> I64
  L_D = 43,
  /// l_f: (F32) -> I64
  L_F = 44,
  /// l_i: (I32) -> I64
  L_I = 45,
  /// l_il: (I32, I64) -> I64
  L_IL = 46,
  /// l_ill: (I32, I64, I64) -> I64
  L_ILL = 47,
  /// l_l: (I64) -> I64
  L_L = 48,
  /// l_ll: (I64, I64) -> I64
  L_LL = 49,
  /// l_lll: (I64, I64, I64) -> I64
  L_LLL = 50,
  /// v_id: (I32, F64) -> Void
  V_ID = 51,
  /// v_if: (I32, F32) -> Void
  V_IF = 52,
  /// v_i: (I32) -> Void
  V_I = 53,
  /// v_ii: (I32, I32) -> Void
  V_II = 54,
  /// v_iii: (I32, I32, I32) -> Void
  V_III = 55,
  /// v_il: (I32, I64) -> Void
  V_IL = 56,
  /// v_li: (I64, I32) -> Void
  V_LI = 57,
  /// v_ll: (I64, I64) -> Void
  V_LL = 58,
  /// v_v: () -> Void
  V_V = 59,
}

/// 签名种类总数 (含 None, 对应 kSigEnum 计数)
pub const WASM_OPCODE_SIG_COUNT: usize = 60;

/// 签名的具体类型布局 (对应 kTypes_##name[] / FunctionSig 常量表)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigDef {
  /// 返回类型 (kTypes[0]; Void 表示无返回)
  pub ret: SigType,
  /// 参数类型 (kTypes[1..])
  pub params: &'static [SigType],
}

/// 签名位置出现的值类型记号 (对应 kWasmI32 等; 仅限签名表用到的子集)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SigType {
  I32,
  I64,
  F32,
  F64,
  S128,
  AnyRef,
  FuncRef,
  EqRef,
  Void,
}

impl WasmOpcodeSig {
  /// 查签名布局 (对应 kTypes_##name 常量与 kCachedSigs)
  #[inline]
  #[must_use]
  pub const fn sig_def(self) -> SigDef {
    match self {
      Self::S_S => SigDef {
        ret: SigType::S128,
        params: &[SigType::S128],
      },
      Self::S_F => SigDef {
        ret: SigType::S128,
        params: &[SigType::F32],
      },
      Self::S_D => SigDef {
        ret: SigType::S128,
        params: &[SigType::F64],
      },
      Self::S_SS => SigDef {
        ret: SigType::S128,
        params: &[SigType::S128, SigType::S128],
      },
      Self::S_I => SigDef {
        ret: SigType::S128,
        params: &[SigType::I32],
      },
      Self::S_L => SigDef {
        ret: SigType::S128,
        params: &[SigType::I64],
      },
      Self::S_SI => SigDef {
        ret: SigType::S128,
        params: &[SigType::S128, SigType::I32],
      },
      Self::I_S => SigDef {
        ret: SigType::I32,
        params: &[SigType::S128],
      },
      Self::V_IS => SigDef {
        ret: SigType::Void,
        params: &[SigType::I32, SigType::S128],
      },
      Self::S_SSS => SigDef {
        ret: SigType::S128,
        params: &[SigType::S128, SigType::S128, SigType::S128],
      },
      Self::S_IS => SigDef {
        ret: SigType::S128,
        params: &[SigType::I32, SigType::S128],
      },
      Self::D_D => SigDef {
        ret: SigType::F64,
        params: &[SigType::F64],
      },
      Self::D_DD => SigDef {
        ret: SigType::F64,
        params: &[SigType::F64, SigType::F64],
      },
      Self::D_F => SigDef {
        ret: SigType::F64,
        params: &[SigType::F32],
      },
      Self::D_I => SigDef {
        ret: SigType::F64,
        params: &[SigType::I32],
      },
      Self::D_ID => SigDef {
        ret: SigType::F64,
        params: &[SigType::I32, SigType::F64],
      },
      Self::D_L => SigDef {
        ret: SigType::F64,
        params: &[SigType::I64],
      },
      Self::F_D => SigDef {
        ret: SigType::F32,
        params: &[SigType::F64],
      },
      Self::F_F => SigDef {
        ret: SigType::F32,
        params: &[SigType::F32],
      },
      Self::F_FF => SigDef {
        ret: SigType::F32,
        params: &[SigType::F32, SigType::F32],
      },
      Self::F_I => SigDef {
        ret: SigType::F32,
        params: &[SigType::I32],
      },
      Self::F_IF => SigDef {
        ret: SigType::F32,
        params: &[SigType::I32, SigType::F32],
      },
      Self::F_L => SigDef {
        ret: SigType::F32,
        params: &[SigType::I64],
      },
      Self::I_A => SigDef {
        ret: SigType::I32,
        params: &[SigType::AnyRef],
      },
      Self::I_CI => SigDef {
        ret: SigType::I32,
        params: &[SigType::FuncRef, SigType::I32],
      },
      Self::I_D => SigDef {
        ret: SigType::I32,
        params: &[SigType::F64],
      },
      Self::I_DD => SigDef {
        ret: SigType::I32,
        params: &[SigType::F64, SigType::F64],
      },
      Self::I_F => SigDef {
        ret: SigType::I32,
        params: &[SigType::F32],
      },
      Self::I_FF => SigDef {
        ret: SigType::I32,
        params: &[SigType::F32, SigType::F32],
      },
      Self::I_I => SigDef {
        ret: SigType::I32,
        params: &[SigType::I32],
      },
      Self::I_II => SigDef {
        ret: SigType::I32,
        params: &[SigType::I32, SigType::I32],
      },
      Self::I_III => SigDef {
        ret: SigType::I32,
        params: &[SigType::I32, SigType::I32, SigType::I32],
      },
      Self::I_IIL => SigDef {
        ret: SigType::I32,
        params: &[SigType::I32, SigType::I32, SigType::I64],
      },
      Self::I_ILL => SigDef {
        ret: SigType::I32,
        params: &[SigType::I32, SigType::I64, SigType::I64],
      },
      Self::I_L => SigDef {
        ret: SigType::I32,
        params: &[SigType::I64],
      },
      Self::I_LI => SigDef {
        ret: SigType::I32,
        params: &[SigType::I64, SigType::I32],
      },
      Self::I_LII => SigDef {
        ret: SigType::I32,
        params: &[SigType::I64, SigType::I32, SigType::I32],
      },
      Self::I_LIL => SigDef {
        ret: SigType::I32,
        params: &[SigType::I64, SigType::I32, SigType::I64],
      },
      Self::I_LLL => SigDef {
        ret: SigType::I32,
        params: &[SigType::I64, SigType::I64, SigType::I64],
      },
      Self::I_LL => SigDef {
        ret: SigType::I32,
        params: &[SigType::I64, SigType::I64],
      },
      Self::I_QQ => SigDef {
        ret: SigType::I32,
        params: &[SigType::EqRef, SigType::EqRef],
      },
      Self::I_V => SigDef {
        ret: SigType::I32,
        params: &[],
      },
      Self::L_D => SigDef {
        ret: SigType::I64,
        params: &[SigType::F64],
      },
      Self::L_F => SigDef {
        ret: SigType::I64,
        params: &[SigType::F32],
      },
      Self::L_I => SigDef {
        ret: SigType::I64,
        params: &[SigType::I32],
      },
      Self::L_IL => SigDef {
        ret: SigType::I64,
        params: &[SigType::I32, SigType::I64],
      },
      Self::L_ILL => SigDef {
        ret: SigType::I64,
        params: &[SigType::I32, SigType::I64, SigType::I64],
      },
      Self::L_L => SigDef {
        ret: SigType::I64,
        params: &[SigType::I64],
      },
      Self::L_LL => SigDef {
        ret: SigType::I64,
        params: &[SigType::I64, SigType::I64],
      },
      Self::L_LLL => SigDef {
        ret: SigType::I64,
        params: &[SigType::I64, SigType::I64, SigType::I64],
      },
      Self::V_ID => SigDef {
        ret: SigType::Void,
        params: &[SigType::I32, SigType::F64],
      },
      Self::V_IF => SigDef {
        ret: SigType::Void,
        params: &[SigType::I32, SigType::F32],
      },
      Self::V_I => SigDef {
        ret: SigType::Void,
        params: &[SigType::I32],
      },
      Self::V_II => SigDef {
        ret: SigType::Void,
        params: &[SigType::I32, SigType::I32],
      },
      Self::V_III => SigDef {
        ret: SigType::Void,
        params: &[SigType::I32, SigType::I32, SigType::I32],
      },
      Self::V_IL => SigDef {
        ret: SigType::Void,
        params: &[SigType::I32, SigType::I64],
      },
      Self::V_LI => SigDef {
        ret: SigType::Void,
        params: &[SigType::I64, SigType::I32],
      },
      Self::V_LL => SigDef {
        ret: SigType::Void,
        params: &[SigType::I64, SigType::I64],
      },
      Self::V_V => SigDef {
        ret: SigType::Void,
        params: &[],
      },
      Self::None => SigDef {
        ret: SigType::Void,
        params: &[],
      },
    }
  }
}

/// 单条操作码元信息 (FOREACH_OPCODE 展开, 源顺序)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpcodeDef {
  /// C++ kExpr 名称后缀, 如 "I32LoadMem"
  pub name: &'static str,
  /// 32 位内部编码
  pub opcode: u32,
  /// wat 文本名, 如 "i32.load"
  pub wat: &'static str,
  /// mem32 签名 (atomic 族) / 通用签名; "_" 表示解码器特判, 无表内签名
  pub sig: WasmOpcodeSig,
  /// mem64 签名 (仅 FOREACH_ATOMIC_OPCODE 第 5 参)
  pub sig64: WasmOpcodeSig,
  /// 前缀字节; 0 = 无前缀
  pub prefix: u32,
}

const SIG_NONE: WasmOpcodeSig = WasmOpcodeSig::None;

#[rustfmt::skip]
pub static OPCODES: &[OpcodeDef] = &[
  OpcodeDef { name: "Unreachable", opcode: 0x0, wat: "unreachable", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Nop", opcode: 0x1, wat: "nop", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Block", opcode: 0x2, wat: "block", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Loop", opcode: 0x3, wat: "loop", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "If", opcode: 0x4, wat: "if", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Else", opcode: 0x5, wat: "else", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Try", opcode: 0x6, wat: "try", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Catch", opcode: 0x7, wat: "catch", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Throw", opcode: 0x8, wat: "throw", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Rethrow", opcode: 0x9, wat: "rethrow", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "TryTable", opcode: 0x1f, wat: "try_table", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "ThrowRef", opcode: 0xa, wat: "throw_ref", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "End", opcode: 0xb, wat: "end", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Br", opcode: 0xc, wat: "br", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "BrIf", opcode: 0xd, wat: "br_if", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "BrTable", opcode: 0xe, wat: "br_table", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Return", opcode: 0xf, wat: "return", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Delegate", opcode: 0x18, wat: "delegate", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "CatchAll", opcode: 0x19, wat: "catch_all", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "BrOnNull", opcode: 0xd5, wat: "br_on_null", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "BrOnNonNull", opcode: 0xd6, wat: "br_on_non_null", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "NopForTestingUnsupportedInLiftoff", opcode: 0x16, wat: "nop_for_testing", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "CallFunction", opcode: 0x10, wat: "call", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "CallIndirect", opcode: 0x11, wat: "call_indirect", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "ReturnCall", opcode: 0x12, wat: "return_call", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "ReturnCallIndirect", opcode: 0x13, wat: "return_call_indirect", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "CallRef", opcode: 0x14, wat: "call_ref", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "ReturnCallRef", opcode: 0x15, wat: "return_call_ref", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Drop", opcode: 0x1a, wat: "drop", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Select", opcode: 0x1b, wat: "select", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "SelectWithType", opcode: 0x1c, wat: "select", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "LocalGet", opcode: 0x20, wat: "local.get", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "LocalSet", opcode: 0x21, wat: "local.set", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "LocalTee", opcode: 0x22, wat: "local.tee", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "GlobalGet", opcode: 0x23, wat: "global.get", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "GlobalSet", opcode: 0x24, wat: "global.set", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "TableGet", opcode: 0x25, wat: "table.get", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "TableSet", opcode: 0x26, wat: "table.set", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Const", opcode: 0x41, wat: "i32.const", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Const", opcode: 0x42, wat: "i64.const", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Const", opcode: 0x43, wat: "f32.const", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Const", opcode: 0x44, wat: "f64.const", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "RefNull", opcode: 0xd0, wat: "ref.null", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "RefIsNull", opcode: 0xd1, wat: "ref.is_null", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "RefFunc", opcode: 0xd2, wat: "ref.func", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "RefAsNonNull", opcode: 0xd4, wat: "ref.as_non_null", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "RefEq", opcode: 0xd3, wat: "ref.eq", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32LoadMem", opcode: 0x28, wat: "i32.load", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LoadMem", opcode: 0x29, wat: "i64.load", sig: WasmOpcodeSig::L_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32LoadMem", opcode: 0x2a, wat: "f32.load", sig: WasmOpcodeSig::F_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64LoadMem", opcode: 0x2b, wat: "f64.load", sig: WasmOpcodeSig::D_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32LoadMem8S", opcode: 0x2c, wat: "i32.load8_s", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32LoadMem8U", opcode: 0x2d, wat: "i32.load8_u", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32LoadMem16S", opcode: 0x2e, wat: "i32.load16_s", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32LoadMem16U", opcode: 0x2f, wat: "i32.load16_u", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LoadMem8S", opcode: 0x30, wat: "i64.load8_s", sig: WasmOpcodeSig::L_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LoadMem8U", opcode: 0x31, wat: "i64.load8_u", sig: WasmOpcodeSig::L_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LoadMem16S", opcode: 0x32, wat: "i64.load16_s", sig: WasmOpcodeSig::L_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LoadMem16U", opcode: 0x33, wat: "i64.load16_u", sig: WasmOpcodeSig::L_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LoadMem32S", opcode: 0x34, wat: "i64.load32_s", sig: WasmOpcodeSig::L_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LoadMem32U", opcode: 0x35, wat: "i64.load32_u", sig: WasmOpcodeSig::L_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32LoadMemF16", opcode: 0xfc30, wat: "f32.load_f16", sig: WasmOpcodeSig::F_I, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I32StoreMem", opcode: 0x36, wat: "i32.store", sig: WasmOpcodeSig::V_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64StoreMem", opcode: 0x37, wat: "i64.store", sig: WasmOpcodeSig::V_IL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32StoreMem", opcode: 0x38, wat: "f32.store", sig: WasmOpcodeSig::V_IF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64StoreMem", opcode: 0x39, wat: "f64.store", sig: WasmOpcodeSig::V_ID, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32StoreMem8", opcode: 0x3a, wat: "i32.store8", sig: WasmOpcodeSig::V_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32StoreMem16", opcode: 0x3b, wat: "i32.store16", sig: WasmOpcodeSig::V_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64StoreMem8", opcode: 0x3c, wat: "i64.store8", sig: WasmOpcodeSig::V_IL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64StoreMem16", opcode: 0x3d, wat: "i64.store16", sig: WasmOpcodeSig::V_IL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64StoreMem32", opcode: 0x3e, wat: "i64.store32", sig: WasmOpcodeSig::V_IL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32StoreMemF16", opcode: 0xfc31, wat: "f32.store_f16", sig: WasmOpcodeSig::V_IF, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "MemorySize", opcode: 0x3f, wat: "memory.size", sig: WasmOpcodeSig::I_V, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "MemoryGrow", opcode: 0x40, wat: "memory.grow", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Add", opcode: 0x6a, wat: "i32.add", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Sub", opcode: 0x6b, wat: "i32.sub", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Mul", opcode: 0x6c, wat: "i32.mul", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Add", opcode: 0x7c, wat: "i64.add", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Sub", opcode: 0x7d, wat: "i64.sub", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Mul", opcode: 0x7e, wat: "i64.mul", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Eqz", opcode: 0x45, wat: "i32.eqz", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Eq", opcode: 0x46, wat: "i32.eq", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Ne", opcode: 0x47, wat: "i32.ne", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32LtS", opcode: 0x48, wat: "i32.lt_s", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32LtU", opcode: 0x49, wat: "i32.lt_u", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32GtS", opcode: 0x4a, wat: "i32.gt_s", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32GtU", opcode: 0x4b, wat: "i32.gt_u", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32LeS", opcode: 0x4c, wat: "i32.le_s", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32LeU", opcode: 0x4d, wat: "i32.le_u", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32GeS", opcode: 0x4e, wat: "i32.ge_s", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32GeU", opcode: 0x4f, wat: "i32.ge_u", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Eqz", opcode: 0x50, wat: "i64.eqz", sig: WasmOpcodeSig::I_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Eq", opcode: 0x51, wat: "i64.eq", sig: WasmOpcodeSig::I_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Ne", opcode: 0x52, wat: "i64.ne", sig: WasmOpcodeSig::I_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LtS", opcode: 0x53, wat: "i64.lt_s", sig: WasmOpcodeSig::I_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LtU", opcode: 0x54, wat: "i64.lt_u", sig: WasmOpcodeSig::I_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64GtS", opcode: 0x55, wat: "i64.gt_s", sig: WasmOpcodeSig::I_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64GtU", opcode: 0x56, wat: "i64.gt_u", sig: WasmOpcodeSig::I_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LeS", opcode: 0x57, wat: "i64.le_s", sig: WasmOpcodeSig::I_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64LeU", opcode: 0x58, wat: "i64.le_u", sig: WasmOpcodeSig::I_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64GeS", opcode: 0x59, wat: "i64.ge_s", sig: WasmOpcodeSig::I_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64GeU", opcode: 0x5a, wat: "i64.ge_u", sig: WasmOpcodeSig::I_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Eq", opcode: 0x5b, wat: "f32.eq", sig: WasmOpcodeSig::I_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Ne", opcode: 0x5c, wat: "f32.ne", sig: WasmOpcodeSig::I_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Lt", opcode: 0x5d, wat: "f32.lt", sig: WasmOpcodeSig::I_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Gt", opcode: 0x5e, wat: "f32.gt", sig: WasmOpcodeSig::I_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Le", opcode: 0x5f, wat: "f32.le", sig: WasmOpcodeSig::I_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Ge", opcode: 0x60, wat: "f32.ge", sig: WasmOpcodeSig::I_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Eq", opcode: 0x61, wat: "f64.eq", sig: WasmOpcodeSig::I_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Ne", opcode: 0x62, wat: "f64.ne", sig: WasmOpcodeSig::I_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Lt", opcode: 0x63, wat: "f64.lt", sig: WasmOpcodeSig::I_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Gt", opcode: 0x64, wat: "f64.gt", sig: WasmOpcodeSig::I_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Le", opcode: 0x65, wat: "f64.le", sig: WasmOpcodeSig::I_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Ge", opcode: 0x66, wat: "f64.ge", sig: WasmOpcodeSig::I_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Clz", opcode: 0x67, wat: "i32.clz", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Ctz", opcode: 0x68, wat: "i32.ctz", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Popcnt", opcode: 0x69, wat: "i32.popcnt", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32DivS", opcode: 0x6d, wat: "i32.div_s", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32DivU", opcode: 0x6e, wat: "i32.div_u", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32RemS", opcode: 0x6f, wat: "i32.rem_s", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32RemU", opcode: 0x70, wat: "i32.rem_u", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32And", opcode: 0x71, wat: "i32.and", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Ior", opcode: 0x72, wat: "i32.or", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Xor", opcode: 0x73, wat: "i32.xor", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Shl", opcode: 0x74, wat: "i32.shl", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32ShrS", opcode: 0x75, wat: "i32.shr_s", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32ShrU", opcode: 0x76, wat: "i32.shr_u", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Rol", opcode: 0x77, wat: "i32.rotl", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32Ror", opcode: 0x78, wat: "i32.rotr", sig: WasmOpcodeSig::I_II, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Clz", opcode: 0x79, wat: "i64.clz", sig: WasmOpcodeSig::L_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Ctz", opcode: 0x7a, wat: "i64.ctz", sig: WasmOpcodeSig::L_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Popcnt", opcode: 0x7b, wat: "i64.popcnt", sig: WasmOpcodeSig::L_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64DivS", opcode: 0x7f, wat: "i64.div_s", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64DivU", opcode: 0x80, wat: "i64.div_u", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64RemS", opcode: 0x81, wat: "i64.rem_s", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64RemU", opcode: 0x82, wat: "i64.rem_u", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64And", opcode: 0x83, wat: "i64.and", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Ior", opcode: 0x84, wat: "i64.or", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Xor", opcode: 0x85, wat: "i64.xor", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Shl", opcode: 0x86, wat: "i64.shl", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64ShrS", opcode: 0x87, wat: "i64.shr_s", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64ShrU", opcode: 0x88, wat: "i64.shr_u", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Rol", opcode: 0x89, wat: "i64.rotl", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64Ror", opcode: 0x8a, wat: "i64.rotr", sig: WasmOpcodeSig::L_LL, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Abs", opcode: 0x8b, wat: "f32.abs", sig: WasmOpcodeSig::F_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Neg", opcode: 0x8c, wat: "f32.neg", sig: WasmOpcodeSig::F_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Ceil", opcode: 0x8d, wat: "f32.ceil", sig: WasmOpcodeSig::F_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Floor", opcode: 0x8e, wat: "f32.floor", sig: WasmOpcodeSig::F_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Trunc", opcode: 0x8f, wat: "f32.trunc", sig: WasmOpcodeSig::F_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32NearestInt", opcode: 0x90, wat: "f32.nearest", sig: WasmOpcodeSig::F_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Sqrt", opcode: 0x91, wat: "f32.sqrt", sig: WasmOpcodeSig::F_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Add", opcode: 0x92, wat: "f32.add", sig: WasmOpcodeSig::F_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Sub", opcode: 0x93, wat: "f32.sub", sig: WasmOpcodeSig::F_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Mul", opcode: 0x94, wat: "f32.mul", sig: WasmOpcodeSig::F_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Div", opcode: 0x95, wat: "f32.div", sig: WasmOpcodeSig::F_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Min", opcode: 0x96, wat: "f32.min", sig: WasmOpcodeSig::F_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32Max", opcode: 0x97, wat: "f32.max", sig: WasmOpcodeSig::F_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32CopySign", opcode: 0x98, wat: "f32.copysign", sig: WasmOpcodeSig::F_FF, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Abs", opcode: 0x99, wat: "f64.abs", sig: WasmOpcodeSig::D_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Neg", opcode: 0x9a, wat: "f64.neg", sig: WasmOpcodeSig::D_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Ceil", opcode: 0x9b, wat: "f64.ceil", sig: WasmOpcodeSig::D_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Floor", opcode: 0x9c, wat: "f64.floor", sig: WasmOpcodeSig::D_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Trunc", opcode: 0x9d, wat: "f64.trunc", sig: WasmOpcodeSig::D_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64NearestInt", opcode: 0x9e, wat: "f64.nearest", sig: WasmOpcodeSig::D_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Sqrt", opcode: 0x9f, wat: "f64.sqrt", sig: WasmOpcodeSig::D_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Add", opcode: 0xa0, wat: "f64.add", sig: WasmOpcodeSig::D_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Sub", opcode: 0xa1, wat: "f64.sub", sig: WasmOpcodeSig::D_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Mul", opcode: 0xa2, wat: "f64.mul", sig: WasmOpcodeSig::D_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Div", opcode: 0xa3, wat: "f64.div", sig: WasmOpcodeSig::D_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Min", opcode: 0xa4, wat: "f64.min", sig: WasmOpcodeSig::D_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64Max", opcode: 0xa5, wat: "f64.max", sig: WasmOpcodeSig::D_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64CopySign", opcode: 0xa6, wat: "f64.copysign", sig: WasmOpcodeSig::D_DD, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32ConvertI64", opcode: 0xa7, wat: "i32.wrap_i64", sig: WasmOpcodeSig::I_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32SConvertF32", opcode: 0xa8, wat: "i32.trunc_f32_s", sig: WasmOpcodeSig::I_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32UConvertF32", opcode: 0xa9, wat: "i32.trunc_f32_u", sig: WasmOpcodeSig::I_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32SConvertF64", opcode: 0xaa, wat: "i32.trunc_f64_s", sig: WasmOpcodeSig::I_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32UConvertF64", opcode: 0xab, wat: "i32.trunc_f64_u", sig: WasmOpcodeSig::I_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64SConvertI32", opcode: 0xac, wat: "i64.extend_i32_s", sig: WasmOpcodeSig::L_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64UConvertI32", opcode: 0xad, wat: "i64.extend_i32_u", sig: WasmOpcodeSig::L_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64SConvertF32", opcode: 0xae, wat: "i64.trunc_f32_s", sig: WasmOpcodeSig::L_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64UConvertF32", opcode: 0xaf, wat: "i64.trunc_f32_u", sig: WasmOpcodeSig::L_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64SConvertF64", opcode: 0xb0, wat: "i64.trunc_f64_s", sig: WasmOpcodeSig::L_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64UConvertF64", opcode: 0xb1, wat: "i64.trunc_f64_u", sig: WasmOpcodeSig::L_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32SConvertI32", opcode: 0xb2, wat: "f32.convert_i32_s", sig: WasmOpcodeSig::F_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32UConvertI32", opcode: 0xb3, wat: "f32.convert_i32_u", sig: WasmOpcodeSig::F_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32SConvertI64", opcode: 0xb4, wat: "f32.convert_i64_s", sig: WasmOpcodeSig::F_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32UConvertI64", opcode: 0xb5, wat: "f32.convert_i64_u", sig: WasmOpcodeSig::F_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32ConvertF64", opcode: 0xb6, wat: "f32.demote_f64", sig: WasmOpcodeSig::F_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64SConvertI32", opcode: 0xb7, wat: "f64.convert_i32_s", sig: WasmOpcodeSig::D_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64UConvertI32", opcode: 0xb8, wat: "f64.convert_i32_u", sig: WasmOpcodeSig::D_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64SConvertI64", opcode: 0xb9, wat: "f64.convert_i64_s", sig: WasmOpcodeSig::D_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64UConvertI64", opcode: 0xba, wat: "f64.convert_i64_u", sig: WasmOpcodeSig::D_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64ConvertF32", opcode: 0xbb, wat: "f64.promote_f32", sig: WasmOpcodeSig::D_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32ReinterpretF32", opcode: 0xbc, wat: "i32.reinterpret_f32", sig: WasmOpcodeSig::I_F, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64ReinterpretF64", opcode: 0xbd, wat: "i64.reinterpret_f64", sig: WasmOpcodeSig::L_D, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F32ReinterpretI32", opcode: 0xbe, wat: "f32.reinterpret_i32", sig: WasmOpcodeSig::F_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "F64ReinterpretI64", opcode: 0xbf, wat: "f64.reinterpret_i64", sig: WasmOpcodeSig::D_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32SExtendI8", opcode: 0xc0, wat: "i32.extend8_s", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I32SExtendI16", opcode: 0xc1, wat: "i32.extend16_s", sig: WasmOpcodeSig::I_I, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64SExtendI8", opcode: 0xc2, wat: "i64.extend8_s", sig: WasmOpcodeSig::L_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64SExtendI16", opcode: 0xc3, wat: "i64.extend16_s", sig: WasmOpcodeSig::L_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "I64SExtendI32", opcode: 0xc4, wat: "i64.extend32_s", sig: WasmOpcodeSig::L_L, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "ContNew", opcode: 0xe0, wat: "cont.new", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "ContBind", opcode: 0xe1, wat: "cont.bind", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Suspend", opcode: 0xe2, wat: "suspend", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Resume", opcode: 0xe3, wat: "resume", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "ResumeThrow", opcode: 0xe4, wat: "resume_throw", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "ResumeThrowRef", opcode: 0xe5, wat: "resume_throw_ref", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "Switch", opcode: 0xe6, wat: "switch", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0x0 },
  OpcodeDef { name: "S128LoadMem", opcode: 0xfd00, wat: "v128.load", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load8x8S", opcode: 0xfd01, wat: "v128.load8x8_s", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load8x8U", opcode: 0xfd02, wat: "v128.load8x8_u", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load16x4S", opcode: 0xfd03, wat: "v128.load16x4_s", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load16x4U", opcode: 0xfd04, wat: "v128.load16x4_u", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load32x2S", opcode: 0xfd05, wat: "v128.load32x2_s", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load32x2U", opcode: 0xfd06, wat: "v128.load32x2_u", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load8Splat", opcode: 0xfd07, wat: "v128.load8_splat", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load16Splat", opcode: 0xfd08, wat: "v128.load16_splat", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load32Splat", opcode: 0xfd09, wat: "v128.load32_splat", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load64Splat", opcode: 0xfd0a, wat: "v128.load64_splat", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128StoreMem", opcode: 0xfd0b, wat: "v128.store", sig: WasmOpcodeSig::V_IS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load32Zero", opcode: 0xfd5c, wat: "v128.load32_zero", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load64Zero", opcode: 0xfd5d, wat: "v128.load64_zero", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load8Lane", opcode: 0xfd54, wat: "v128.load8_lane", sig: WasmOpcodeSig::S_IS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load16Lane", opcode: 0xfd55, wat: "v128.load16_lane", sig: WasmOpcodeSig::S_IS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load32Lane", opcode: 0xfd56, wat: "v128.load32_lane", sig: WasmOpcodeSig::S_IS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Load64Lane", opcode: 0xfd57, wat: "v128.load64_lane", sig: WasmOpcodeSig::S_IS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Store8Lane", opcode: 0xfd58, wat: "v128.store8_lane", sig: WasmOpcodeSig::V_IS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Store16Lane", opcode: 0xfd59, wat: "v128.store16_lane", sig: WasmOpcodeSig::V_IS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Store32Lane", opcode: 0xfd5a, wat: "v128.store32_lane", sig: WasmOpcodeSig::V_IS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Store64Lane", opcode: 0xfd5b, wat: "v128.store64_lane", sig: WasmOpcodeSig::V_IS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Const", opcode: 0xfd0c, wat: "v128.const", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Shuffle", opcode: 0xfd0d, wat: "i8x16.shuffle", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Swizzle", opcode: 0xfd0e, wat: "i8x16.swizzle", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Splat", opcode: 0xfd0f, wat: "i8x16.splat", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8Splat", opcode: 0xfd10, wat: "i16x8.splat", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4Splat", opcode: 0xfd11, wat: "i32x4.splat", sig: WasmOpcodeSig::S_I, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2Splat", opcode: 0xfd12, wat: "i64x2.splat", sig: WasmOpcodeSig::S_L, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Splat", opcode: 0xfd13, wat: "f32x4.splat", sig: WasmOpcodeSig::S_F, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Splat", opcode: 0xfd14, wat: "f64x2.splat", sig: WasmOpcodeSig::S_D, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Eq", opcode: 0xfd23, wat: "i8x16.eq", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Ne", opcode: 0xfd24, wat: "i8x16.ne", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16LtS", opcode: 0xfd25, wat: "i8x16.lt_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16LtU", opcode: 0xfd26, wat: "i8x16.lt_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16GtS", opcode: 0xfd27, wat: "i8x16.gt_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16GtU", opcode: 0xfd28, wat: "i8x16.gt_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16LeS", opcode: 0xfd29, wat: "i8x16.le_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16LeU", opcode: 0xfd2a, wat: "i8x16.le_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16GeS", opcode: 0xfd2b, wat: "i8x16.ge_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16GeU", opcode: 0xfd2c, wat: "i8x16.ge_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8Eq", opcode: 0xfd2d, wat: "i16x8.eq", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8Ne", opcode: 0xfd2e, wat: "i16x8.ne", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8LtS", opcode: 0xfd2f, wat: "i16x8.lt_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8LtU", opcode: 0xfd30, wat: "i16x8.lt_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8GtS", opcode: 0xfd31, wat: "i16x8.gt_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8GtU", opcode: 0xfd32, wat: "i16x8.gt_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8LeS", opcode: 0xfd33, wat: "i16x8.le_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8LeU", opcode: 0xfd34, wat: "i16x8.le_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8GeS", opcode: 0xfd35, wat: "i16x8.ge_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8GeU", opcode: 0xfd36, wat: "i16x8.ge_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4Eq", opcode: 0xfd37, wat: "i32x4.eq", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4Ne", opcode: 0xfd38, wat: "i32x4.ne", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4LtS", opcode: 0xfd39, wat: "i32x4.lt_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4LtU", opcode: 0xfd3a, wat: "i32x4.lt_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4GtS", opcode: 0xfd3b, wat: "i32x4.gt_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4GtU", opcode: 0xfd3c, wat: "i32x4.gt_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4LeS", opcode: 0xfd3d, wat: "i32x4.le_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4LeU", opcode: 0xfd3e, wat: "i32x4.le_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4GeS", opcode: 0xfd3f, wat: "i32x4.ge_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4GeU", opcode: 0xfd40, wat: "i32x4.ge_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Eq", opcode: 0xfd41, wat: "f32x4.eq", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Ne", opcode: 0xfd42, wat: "f32x4.ne", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Lt", opcode: 0xfd43, wat: "f32x4.lt", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Gt", opcode: 0xfd44, wat: "f32x4.gt", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Le", opcode: 0xfd45, wat: "f32x4.le", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Ge", opcode: 0xfd46, wat: "f32x4.ge", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Eq", opcode: 0xfd47, wat: "f64x2.eq", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Ne", opcode: 0xfd48, wat: "f64x2.ne", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Lt", opcode: 0xfd49, wat: "f64x2.lt", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Gt", opcode: 0xfd4a, wat: "f64x2.gt", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Le", opcode: 0xfd4b, wat: "f64x2.le", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Ge", opcode: 0xfd4c, wat: "f64x2.ge", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Not", opcode: 0xfd4d, wat: "v128.not", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128And", opcode: 0xfd4e, wat: "v128.and", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128AndNot", opcode: 0xfd4f, wat: "v128.andnot", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Or", opcode: 0xfd50, wat: "v128.or", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Xor", opcode: 0xfd51, wat: "v128.xor", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "S128Select", opcode: 0xfd52, wat: "v128.bitselect", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "V128AnyTrue", opcode: 0xfd53, wat: "v128.any_true", sig: WasmOpcodeSig::I_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4DemoteF64x2Zero", opcode: 0xfd5e, wat: "f32x4.demote_f64x2_zero", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2PromoteLowF32x4", opcode: 0xfd5f, wat: "f64x2.promote_low_f32x4", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Abs", opcode: 0xfd60, wat: "i8x16.abs", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Neg", opcode: 0xfd61, wat: "i8x16.neg", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Popcnt", opcode: 0xfd62, wat: "i8x16.popcnt", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16AllTrue", opcode: 0xfd63, wat: "i8x16.all_true", sig: WasmOpcodeSig::I_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16BitMask", opcode: 0xfd64, wat: "i8x16.bitmask", sig: WasmOpcodeSig::I_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16SConvertI16x8", opcode: 0xfd65, wat: "i8x16.narrow_i16x8_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16UConvertI16x8", opcode: 0xfd66, wat: "i8x16.narrow_i16x8_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Ceil", opcode: 0xfd67, wat: "f32x4.ceil", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Floor", opcode: 0xfd68, wat: "f32x4.floor", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Trunc", opcode: 0xfd69, wat: "f32x4.trunc", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4NearestInt", opcode: 0xfd6a, wat: "f32x4.nearest", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Shl", opcode: 0xfd6b, wat: "i8x16.shl", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16ShrS", opcode: 0xfd6c, wat: "i8x16.shr_s", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16ShrU", opcode: 0xfd6d, wat: "i8x16.shr_u", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Add", opcode: 0xfd6e, wat: "i8x16.add", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16AddSatS", opcode: 0xfd6f, wat: "i8x16.add_sat_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16AddSatU", opcode: 0xfd70, wat: "i8x16.add_sat_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16Sub", opcode: 0xfd71, wat: "i8x16.sub", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16SubSatS", opcode: 0xfd72, wat: "i8x16.sub_sat_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16SubSatU", opcode: 0xfd73, wat: "i8x16.sub_sat_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Ceil", opcode: 0xfd74, wat: "f64x2.ceil", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Floor", opcode: 0xfd75, wat: "f64x2.floor", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16MinS", opcode: 0xfd76, wat: "i8x16.min_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16MinU", opcode: 0xfd77, wat: "i8x16.min_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16MaxS", opcode: 0xfd78, wat: "i8x16.max_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16MaxU", opcode: 0xfd79, wat: "i8x16.max_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Trunc", opcode: 0xfd7a, wat: "f64x2.trunc", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16RoundingAverageU", opcode: 0xfd7b, wat: "i8x16.avgr_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ExtAddPairwiseI8x16S", opcode: 0xfd7c, wat: "i16x8.extadd_pairwise_i8x16_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ExtAddPairwiseI8x16U", opcode: 0xfd7d, wat: "i16x8.extadd_pairwise_i8x16_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4ExtAddPairwiseI16x8S", opcode: 0xfd7e, wat: "i32x4.extadd_pairwise_i16x8_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4ExtAddPairwiseI16x8U", opcode: 0xfd7f, wat: "i32x4.extadd_pairwise_i16x8_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8Abs", opcode: 0xfd80, wat: "i16x8.abs", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8Neg", opcode: 0xfd81, wat: "i16x8.neg", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8Q15MulRSatS", opcode: 0xfd82, wat: "i16x8.q15mulr_sat_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8AllTrue", opcode: 0xfd83, wat: "i16x8.all_true", sig: WasmOpcodeSig::I_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8BitMask", opcode: 0xfd84, wat: "i16x8.bitmask", sig: WasmOpcodeSig::I_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8SConvertI32x4", opcode: 0xfd85, wat: "i16x8.narrow_i32x4_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8UConvertI32x4", opcode: 0xfd86, wat: "i16x8.narrow_i32x4_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8SConvertI8x16Low", opcode: 0xfd87, wat: "i16x8.extend_low_i8x16_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8SConvertI8x16High", opcode: 0xfd88, wat: "i16x8.extend_high_i8x16_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8UConvertI8x16Low", opcode: 0xfd89, wat: "i16x8.extend_low_i8x16_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8UConvertI8x16High", opcode: 0xfd8a, wat: "i16x8.extend_high_i8x16_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8Shl", opcode: 0xfd8b, wat: "i16x8.shl", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ShrS", opcode: 0xfd8c, wat: "i16x8.shr_s", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ShrU", opcode: 0xfd8d, wat: "i16x8.shr_u", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8Add", opcode: 0xfd8e, wat: "i16x8.add", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8AddSatS", opcode: 0xfd8f, wat: "i16x8.add_sat_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8AddSatU", opcode: 0xfd90, wat: "i16x8.add_sat_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8Sub", opcode: 0xfd91, wat: "i16x8.sub", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8SubSatS", opcode: 0xfd92, wat: "i16x8.sub_sat_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8SubSatU", opcode: 0xfd93, wat: "i16x8.sub_sat_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2NearestInt", opcode: 0xfd94, wat: "f64x2.nearest", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8Mul", opcode: 0xfd95, wat: "i16x8.mul", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8MinS", opcode: 0xfd96, wat: "i16x8.min_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8MinU", opcode: 0xfd97, wat: "i16x8.min_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8MaxS", opcode: 0xfd98, wat: "i16x8.max_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8MaxU", opcode: 0xfd99, wat: "i16x8.max_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8RoundingAverageU", opcode: 0xfd9b, wat: "i16x8.avgr_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ExtMulLowI8x16S", opcode: 0xfd9c, wat: "i16x8.extmul_low_i8x16_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ExtMulHighI8x16S", opcode: 0xfd9d, wat: "i16x8.extmul_high_i8x16_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ExtMulLowI8x16U", opcode: 0xfd9e, wat: "i16x8.extmul_low_i8x16_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ExtMulHighI8x16U", opcode: 0xfd9f, wat: "i16x8.extmul_high_i8x16_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4Abs", opcode: 0xfda0, wat: "i32x4.abs", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4Neg", opcode: 0xfda1, wat: "i32x4.neg", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4AllTrue", opcode: 0xfda3, wat: "i32x4.all_true", sig: WasmOpcodeSig::I_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4BitMask", opcode: 0xfda4, wat: "i32x4.bitmask", sig: WasmOpcodeSig::I_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4SConvertI16x8Low", opcode: 0xfda7, wat: "i32x4.extend_low_i16x8_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4SConvertI16x8High", opcode: 0xfda8, wat: "i32x4.extend_high_i16x8_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4UConvertI16x8Low", opcode: 0xfda9, wat: "i32x4.extend_low_i16x8_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4UConvertI16x8High", opcode: 0xfdaa, wat: "i32x4.extend_high_i16x8_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4Shl", opcode: 0xfdab, wat: "i32x4.shl", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4ShrS", opcode: 0xfdac, wat: "i32x4.shr_s", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4ShrU", opcode: 0xfdad, wat: "i32x4.shr_u", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4Add", opcode: 0xfdae, wat: "i32x4.add", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4Sub", opcode: 0xfdb1, wat: "i32x4.sub", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4Mul", opcode: 0xfdb5, wat: "i32x4.mul", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4MinS", opcode: 0xfdb6, wat: "i32x4.min_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4MinU", opcode: 0xfdb7, wat: "i32x4.min_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4MaxS", opcode: 0xfdb8, wat: "i32x4.max_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4MaxU", opcode: 0xfdb9, wat: "i32x4.max_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4DotI16x8S", opcode: 0xfdba, wat: "i32x4.dot_i16x8_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4ExtMulLowI16x8S", opcode: 0xfdbc, wat: "i32x4.extmul_low_i16x8_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4ExtMulHighI16x8S", opcode: 0xfdbd, wat: "i32x4.extmul_high_i16x8_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4ExtMulLowI16x8U", opcode: 0xfdbe, wat: "i32x4.extmul_low_i16x8_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4ExtMulHighI16x8U", opcode: 0xfdbf, wat: "i32x4.extmul_high_i16x8_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2Abs", opcode: 0xfdc0, wat: "i64x2.abs", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2Neg", opcode: 0xfdc1, wat: "i64x2.neg", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2AllTrue", opcode: 0xfdc3, wat: "i64x2.all_true", sig: WasmOpcodeSig::I_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2BitMask", opcode: 0xfdc4, wat: "i64x2.bitmask", sig: WasmOpcodeSig::I_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2SConvertI32x4Low", opcode: 0xfdc7, wat: "i64x2.extend_low_i32x4_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2SConvertI32x4High", opcode: 0xfdc8, wat: "i64x2.extend_high_i32x4_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2UConvertI32x4Low", opcode: 0xfdc9, wat: "i64x2.extend_low_i32x4_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2UConvertI32x4High", opcode: 0xfdca, wat: "i64x2.extend_high_i32x4_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2Shl", opcode: 0xfdcb, wat: "i64x2.shl", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2ShrS", opcode: 0xfdcc, wat: "i64x2.shr_s", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2ShrU", opcode: 0xfdcd, wat: "i64x2.shr_u", sig: WasmOpcodeSig::S_SI, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2Add", opcode: 0xfdce, wat: "i64x2.add", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2Sub", opcode: 0xfdd1, wat: "i64x2.sub", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2Mul", opcode: 0xfdd5, wat: "i64x2.mul", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2Eq", opcode: 0xfdd6, wat: "i64x2.eq", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2Ne", opcode: 0xfdd7, wat: "i64x2.ne", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2LtS", opcode: 0xfdd8, wat: "i64x2.lt_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2GtS", opcode: 0xfdd9, wat: "i64x2.gt_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2LeS", opcode: 0xfdda, wat: "i64x2.le_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2GeS", opcode: 0xfddb, wat: "i64x2.ge_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2ExtMulLowI32x4S", opcode: 0xfddc, wat: "i64x2.extmul_low_i32x4_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2ExtMulHighI32x4S", opcode: 0xfddd, wat: "i64x2.extmul_high_i32x4_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2ExtMulLowI32x4U", opcode: 0xfdde, wat: "i64x2.extmul_low_i32x4_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2ExtMulHighI32x4U", opcode: 0xfddf, wat: "i64x2.extmul_high_i32x4_u", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Abs", opcode: 0xfde0, wat: "f32x4.abs", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Neg", opcode: 0xfde1, wat: "f32x4.neg", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Sqrt", opcode: 0xfde3, wat: "f32x4.sqrt", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Add", opcode: 0xfde4, wat: "f32x4.add", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Sub", opcode: 0xfde5, wat: "f32x4.sub", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Mul", opcode: 0xfde6, wat: "f32x4.mul", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Div", opcode: 0xfde7, wat: "f32x4.div", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Min", opcode: 0xfde8, wat: "f32x4.min", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Max", opcode: 0xfde9, wat: "f32x4.max", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Pmin", opcode: 0xfdea, wat: "f32x4.pmin", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Pmax", opcode: 0xfdeb, wat: "f32x4.pmax", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Abs", opcode: 0xfdec, wat: "f64x2.abs", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Neg", opcode: 0xfded, wat: "f64x2.neg", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Sqrt", opcode: 0xfdef, wat: "f64x2.sqrt", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Add", opcode: 0xfdf0, wat: "f64x2.add", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Sub", opcode: 0xfdf1, wat: "f64x2.sub", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Mul", opcode: 0xfdf2, wat: "f64x2.mul", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Div", opcode: 0xfdf3, wat: "f64x2.div", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Min", opcode: 0xfdf4, wat: "f64x2.min", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Max", opcode: 0xfdf5, wat: "f64x2.max", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Pmin", opcode: 0xfdf6, wat: "f64x2.pmin", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Pmax", opcode: 0xfdf7, wat: "f64x2.pmax", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4SConvertF32x4", opcode: 0xfdf8, wat: "i32x4.trunc_sat_f32x4_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4UConvertF32x4", opcode: 0xfdf9, wat: "i32x4.trunc_sat_f32x4_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4SConvertI32x4", opcode: 0xfdfa, wat: "f32x4.convert_i32x4_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4UConvertI32x4", opcode: 0xfdfb, wat: "f32x4.convert_i32x4_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4TruncSatF64x2SZero", opcode: 0xfdfc, wat: "i32x4.trunc_sat_f64x2_s_zero", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4TruncSatF64x2UZero", opcode: 0xfdfd, wat: "i32x4.trunc_sat_f64x2_u_zero", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2ConvertLowI32x4S", opcode: 0xfdfe, wat: "f64x2.convert_low_i32x4_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2ConvertLowI32x4U", opcode: 0xfdff, wat: "f64x2.convert_low_i32x4_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16RelaxedSwizzle", opcode: 0xfd100, wat: "i8x16.relaxed_swizzle", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4RelaxedTruncF32x4S", opcode: 0xfd101, wat: "i32x4.relaxed_trunc_f32x4_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4RelaxedTruncF32x4U", opcode: 0xfd102, wat: "i32x4.relaxed_trunc_f32x4_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4RelaxedTruncF64x2SZero", opcode: 0xfd103, wat: "i32x4.relaxed_trunc_f64x2_s_zero", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4RelaxedTruncF64x2UZero", opcode: 0xfd104, wat: "i32x4.relaxed_trunc_f64x2_u_zero", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Qfma", opcode: 0xfd105, wat: "f32x4.qfma", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4Qfms", opcode: 0xfd106, wat: "f32x4.qfms", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Qfma", opcode: 0xfd107, wat: "f64x2.qfma", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2Qfms", opcode: 0xfd108, wat: "f64x2.qfms", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16RelaxedLaneSelect", opcode: 0xfd109, wat: "i8x16.relaxed_laneselect", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8RelaxedLaneSelect", opcode: 0xfd10a, wat: "i16x8.relaxed_laneselect", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4RelaxedLaneSelect", opcode: 0xfd10b, wat: "i32x4.relaxed_laneselect", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2RelaxedLaneSelect", opcode: 0xfd10c, wat: "i64x2.relaxed_laneselect", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4RelaxedMin", opcode: 0xfd10d, wat: "f32x4.relaxed_min", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4RelaxedMax", opcode: 0xfd10e, wat: "f32x4.relaxed_max", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2RelaxedMin", opcode: 0xfd10f, wat: "f64x2.relaxed_min", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2RelaxedMax", opcode: 0xfd110, wat: "f64x2.relaxed_max", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8RelaxedQ15MulRS", opcode: 0xfd111, wat: "i16x8.relaxed_q15mulr_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8DotI8x16I7x16S", opcode: 0xfd112, wat: "i16x8.dot_i8x16_i7x16_s", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4DotI8x16I7x16AddS", opcode: 0xfd113, wat: "i32x4.dot_i8x16_i7x16_add_s", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Splat", opcode: 0xfd120, wat: "f16x8.splat", sig: WasmOpcodeSig::S_F, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Abs", opcode: 0xfd130, wat: "f16x8.abs", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Neg", opcode: 0xfd131, wat: "f16x8.neg", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Sqrt", opcode: 0xfd132, wat: "f16x8.sqrt", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Ceil", opcode: 0xfd133, wat: "f16x8.ceil", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Floor", opcode: 0xfd134, wat: "f16x8.floor", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Trunc", opcode: 0xfd135, wat: "f16x8.trunc", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8NearestInt", opcode: 0xfd136, wat: "f16x8.nearest", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Eq", opcode: 0xfd137, wat: "f16x8.eq", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Ne", opcode: 0xfd138, wat: "f16x8.ne", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Lt", opcode: 0xfd139, wat: "f16x8.lt", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Gt", opcode: 0xfd13a, wat: "f16x8.gt", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Le", opcode: 0xfd13b, wat: "f16x8.le", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Ge", opcode: 0xfd13c, wat: "f16x8.ge", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Add", opcode: 0xfd13d, wat: "f16x8.add", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Sub", opcode: 0xfd13e, wat: "f16x8.sub", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Mul", opcode: 0xfd13f, wat: "f16x8.mul", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Div", opcode: 0xfd140, wat: "f16x8.div", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Min", opcode: 0xfd141, wat: "f16x8.min", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Max", opcode: 0xfd142, wat: "f16x8.max", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Pmin", opcode: 0xfd143, wat: "f16x8.pmin", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Pmax", opcode: 0xfd144, wat: "f16x8.pmax", sig: WasmOpcodeSig::S_SS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8SConvertF16x8", opcode: 0xfd145, wat: "i16x8.trunc_sat_f16x8_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8UConvertF16x8", opcode: 0xfd146, wat: "i16x8.trunc_sat_f16x8_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8SConvertI16x8", opcode: 0xfd147, wat: "f16x8.convert_i16x8_s", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8UConvertI16x8", opcode: 0xfd148, wat: "f16x8.convert_i16x8_u", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8DemoteF32x4Zero", opcode: 0xfd149, wat: "f16x8.demote_f32x4_zero", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8DemoteF64x2Zero", opcode: 0xfd14a, wat: "f16x8.demote_f64x2_zero", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4PromoteLowF16x8", opcode: 0xfd14b, wat: "f32x4.promote_low_f16x8", sig: WasmOpcodeSig::S_S, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Qfma", opcode: 0xfd14e, wat: "f16x8.madd", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8Qfms", opcode: 0xfd14f, wat: "f16x8.nmadd", sig: WasmOpcodeSig::S_SSS, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16ExtractLaneS", opcode: 0xfd15, wat: "i8x16.extract_lane_s", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16ExtractLaneU", opcode: 0xfd16, wat: "i8x16.extract_lane_u", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ExtractLaneS", opcode: 0xfd18, wat: "i16x8.extract_lane_s", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ExtractLaneU", opcode: 0xfd19, wat: "i16x8.extract_lane_u", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4ExtractLane", opcode: 0xfd1b, wat: "i32x4.extract_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2ExtractLane", opcode: 0xfd1d, wat: "i64x2.extract_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4ExtractLane", opcode: 0xfd1f, wat: "f32x4.extract_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2ExtractLane", opcode: 0xfd21, wat: "f64x2.extract_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8ExtractLane", opcode: 0xfd121, wat: "f16x8.extract_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I8x16ReplaceLane", opcode: 0xfd17, wat: "i8x16.replace_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I16x8ReplaceLane", opcode: 0xfd1a, wat: "i16x8.replace_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32x4ReplaceLane", opcode: 0xfd1c, wat: "i32x4.replace_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I64x2ReplaceLane", opcode: 0xfd1e, wat: "i64x2.replace_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F32x4ReplaceLane", opcode: 0xfd20, wat: "f32x4.replace_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F64x2ReplaceLane", opcode: 0xfd22, wat: "f64x2.replace_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "F16x8ReplaceLane", opcode: 0xfd122, wat: "f16x8.replace_lane", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfd },
  OpcodeDef { name: "I32SConvertSatF32", opcode: 0xfc00, wat: "i32.trunc_sat_f32_s", sig: WasmOpcodeSig::I_F, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I32UConvertSatF32", opcode: 0xfc01, wat: "i32.trunc_sat_f32_u", sig: WasmOpcodeSig::I_F, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I32SConvertSatF64", opcode: 0xfc02, wat: "i32.trunc_sat_f64_s", sig: WasmOpcodeSig::I_D, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I32UConvertSatF64", opcode: 0xfc03, wat: "i32.trunc_sat_f64_u", sig: WasmOpcodeSig::I_D, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I64SConvertSatF32", opcode: 0xfc04, wat: "i64.trunc_sat_f32_s", sig: WasmOpcodeSig::L_F, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I64UConvertSatF32", opcode: 0xfc05, wat: "i64.trunc_sat_f32_u", sig: WasmOpcodeSig::L_F, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I64SConvertSatF64", opcode: 0xfc06, wat: "i64.trunc_sat_f64_s", sig: WasmOpcodeSig::L_D, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I64UConvertSatF64", opcode: 0xfc07, wat: "i64.trunc_sat_f64_u", sig: WasmOpcodeSig::L_D, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "DataDrop", opcode: 0xfc09, wat: "data.drop", sig: WasmOpcodeSig::V_V, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "TableInit", opcode: 0xfc0c, wat: "table.init", sig: WasmOpcodeSig::V_III, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "ElemDrop", opcode: 0xfc0d, wat: "elem.drop", sig: WasmOpcodeSig::V_V, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "TableCopy", opcode: 0xfc0e, wat: "table.copy", sig: WasmOpcodeSig::V_III, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "TableSize", opcode: 0xfc10, wat: "table.size", sig: WasmOpcodeSig::I_V, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I64Add128", opcode: 0xfc13, wat: "i64.add128", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I64Sub128", opcode: 0xfc14, wat: "i64.sub128", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I64MulWideS", opcode: 0xfc15, wat: "i64.mul_wide_s", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "I64MulWideU", opcode: 0xfc16, wat: "i64.mul_wide_u", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "MemoryInit", opcode: 0xfc08, wat: "memory.init", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "MemoryCopy", opcode: 0xfc0a, wat: "memory.copy", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "MemoryFill", opcode: 0xfc0b, wat: "memory.fill", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "TableGrow", opcode: 0xfc0f, wat: "table.grow", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "TableFill", opcode: 0xfc11, wat: "table.fill", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfc },
  OpcodeDef { name: "AtomicNotify", opcode: 0xfe00, wat: "memory.atomic.notify", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicWait", opcode: 0xfe01, wat: "memory.atomic.wait32", sig: WasmOpcodeSig::I_IIL, sig64: WasmOpcodeSig::I_LIL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicWait", opcode: 0xfe02, wat: "memory.atomic.wait64", sig: WasmOpcodeSig::I_ILL, sig64: WasmOpcodeSig::I_LLL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicLoad", opcode: 0xfe10, wat: "i32.atomic.load", sig: WasmOpcodeSig::I_I, sig64: WasmOpcodeSig::I_L, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicLoad", opcode: 0xfe11, wat: "i64.atomic.load", sig: WasmOpcodeSig::L_I, sig64: WasmOpcodeSig::L_L, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicLoad8U", opcode: 0xfe12, wat: "i32.atomic.load8_u", sig: WasmOpcodeSig::I_I, sig64: WasmOpcodeSig::I_L, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicLoad16U", opcode: 0xfe13, wat: "i32.atomic.load16_u", sig: WasmOpcodeSig::I_I, sig64: WasmOpcodeSig::I_L, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicLoad8U", opcode: 0xfe14, wat: "i64.atomic.load8_u", sig: WasmOpcodeSig::L_I, sig64: WasmOpcodeSig::L_L, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicLoad16U", opcode: 0xfe15, wat: "i64.atomic.load16_u", sig: WasmOpcodeSig::L_I, sig64: WasmOpcodeSig::L_L, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicLoad32U", opcode: 0xfe16, wat: "i64.atomic.load32_u", sig: WasmOpcodeSig::L_I, sig64: WasmOpcodeSig::L_L, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicStore", opcode: 0xfe17, wat: "i32.atomic.store", sig: WasmOpcodeSig::V_II, sig64: WasmOpcodeSig::V_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicStore", opcode: 0xfe18, wat: "i64.atomic.store", sig: WasmOpcodeSig::V_IL, sig64: WasmOpcodeSig::V_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicStore8U", opcode: 0xfe19, wat: "i32.atomic.store8", sig: WasmOpcodeSig::V_II, sig64: WasmOpcodeSig::V_LI, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicStore16U", opcode: 0xfe1a, wat: "i32.atomic.store16", sig: WasmOpcodeSig::V_II, sig64: WasmOpcodeSig::V_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicStore8U", opcode: 0xfe1b, wat: "i64.atomic.store8", sig: WasmOpcodeSig::V_IL, sig64: WasmOpcodeSig::V_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicStore16U", opcode: 0xfe1c, wat: "i64.atomic.store16", sig: WasmOpcodeSig::V_IL, sig64: WasmOpcodeSig::V_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicStore32U", opcode: 0xfe1d, wat: "i64.atomic.store32", sig: WasmOpcodeSig::V_IL, sig64: WasmOpcodeSig::V_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicAdd", opcode: 0xfe1e, wat: "i32.atomic.rmw.add", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicAdd", opcode: 0xfe1f, wat: "i64.atomic.rmw.add", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicAdd8U", opcode: 0xfe20, wat: "i32.atomic.rmw8.add_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicAdd16U", opcode: 0xfe21, wat: "i32.atomic.rmw16.add_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicAdd8U", opcode: 0xfe22, wat: "i64.atomic.rmw8.add_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicAdd16U", opcode: 0xfe23, wat: "i64.atomic.rmw16.add_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicAdd32U", opcode: 0xfe24, wat: "i64.atomic.rmw32.add_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicSub", opcode: 0xfe25, wat: "i32.atomic.rmw.sub", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicSub", opcode: 0xfe26, wat: "i64.atomic.rmw.sub", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicSub8U", opcode: 0xfe27, wat: "i32.atomic.rmw8.sub_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicSub16U", opcode: 0xfe28, wat: "i32.atomic.rmw16.sub_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicSub8U", opcode: 0xfe29, wat: "i64.atomic.rmw8.sub_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicSub16U", opcode: 0xfe2a, wat: "i64.atomic.rmw16.sub_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicSub32U", opcode: 0xfe2b, wat: "i64.atomic.rmw32.sub_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicAnd", opcode: 0xfe2c, wat: "i32.atomic.rmw.and", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicAnd", opcode: 0xfe2d, wat: "i64.atomic.rmw.and", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicAnd8U", opcode: 0xfe2e, wat: "i32.atomic.rmw8.and_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicAnd16U", opcode: 0xfe2f, wat: "i32.atomic.rmw16.and_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicAnd8U", opcode: 0xfe30, wat: "i64.atomic.rmw8.and_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicAnd16U", opcode: 0xfe31, wat: "i64.atomic.rmw16.and_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicAnd32U", opcode: 0xfe32, wat: "i64.atomic.rmw32.and_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicOr", opcode: 0xfe33, wat: "i32.atomic.rmw.or", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicOr", opcode: 0xfe34, wat: "i64.atomic.rmw.or", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicOr8U", opcode: 0xfe35, wat: "i32.atomic.rmw8.or_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicOr16U", opcode: 0xfe36, wat: "i32.atomic.rmw16.or_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicOr8U", opcode: 0xfe37, wat: "i64.atomic.rmw8.or_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicOr16U", opcode: 0xfe38, wat: "i64.atomic.rmw16.or_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicOr32U", opcode: 0xfe39, wat: "i64.atomic.rmw32.or_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicXor", opcode: 0xfe3a, wat: "i32.atomic.rmw.xor", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicXor", opcode: 0xfe3b, wat: "i64.atomic.rmw.xor", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicXor8U", opcode: 0xfe3c, wat: "i32.atomic.rmw8.xor_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicXor16U", opcode: 0xfe3d, wat: "i32.atomic.rmw16.xor_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicXor8U", opcode: 0xfe3e, wat: "i64.atomic.rmw8.xor_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicXor16U", opcode: 0xfe3f, wat: "i64.atomic.rmw16.xor_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicXor32U", opcode: 0xfe40, wat: "i64.atomic.rmw32.xor_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicExchange", opcode: 0xfe41, wat: "i32.atomic.rmw.xchg", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicExchange", opcode: 0xfe42, wat: "i64.atomic.rmw.xchg", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicExchange8U", opcode: 0xfe43, wat: "i32.atomic.rmw8.xchg_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicExchange16U", opcode: 0xfe44, wat: "i32.atomic.rmw16.xchg_u", sig: WasmOpcodeSig::I_II, sig64: WasmOpcodeSig::I_LI, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicExchange8U", opcode: 0xfe45, wat: "i64.atomic.rmw8.xchg_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicExchange16U", opcode: 0xfe46, wat: "i64.atomic.rmw16.xchg_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicExchange32U", opcode: 0xfe47, wat: "i64.atomic.rmw32.xchg_u", sig: WasmOpcodeSig::L_IL, sig64: WasmOpcodeSig::L_LL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicCompareExchange", opcode: 0xfe48, wat: "i32.atomic.rmw.cmpxchg", sig: WasmOpcodeSig::I_III, sig64: WasmOpcodeSig::I_LII, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicCompareExchange", opcode: 0xfe49, wat: "i64.atomic.rmw.cmpxchg", sig: WasmOpcodeSig::L_ILL, sig64: WasmOpcodeSig::L_LLL, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicCompareExchange8U", opcode: 0xfe4a, wat: "i32.atomic.rmw8.cmpxchg_u", sig: WasmOpcodeSig::I_III, sig64: WasmOpcodeSig::I_LII, prefix: 0xfe },
  OpcodeDef { name: "I32AtomicCompareExchange16U", opcode: 0xfe4b, wat: "i32.atomic.rmw16.cmpxchg_u", sig: WasmOpcodeSig::I_III, sig64: WasmOpcodeSig::I_LII, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicCompareExchange8U", opcode: 0xfe4c, wat: "i64.atomic.rmw8.cmpxchg_u", sig: WasmOpcodeSig::L_ILL, sig64: WasmOpcodeSig::L_LLL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicCompareExchange16U", opcode: 0xfe4d, wat: "i64.atomic.rmw16.cmpxchg_u", sig: WasmOpcodeSig::L_ILL, sig64: WasmOpcodeSig::L_LLL, prefix: 0xfe },
  OpcodeDef { name: "I64AtomicCompareExchange32U", opcode: 0xfe4e, wat: "i64.atomic.rmw32.cmpxchg_u", sig: WasmOpcodeSig::L_ILL, sig64: WasmOpcodeSig::L_LLL, prefix: 0xfe },
  OpcodeDef { name: "AtomicFence", opcode: 0xfe03, wat: "atomic.fence", sig: WasmOpcodeSig::V_V, sig64: WasmOpcodeSig::V_V, prefix: 0xfe },
  OpcodeDef { name: "StructNew", opcode: 0xfb00, wat: "struct.new", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StructNewDefault", opcode: 0xfb01, wat: "struct.new_default", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StructGet", opcode: 0xfb02, wat: "struct.get", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StructGetS", opcode: 0xfb03, wat: "struct.get_s", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StructGetU", opcode: 0xfb04, wat: "struct.get_u", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StructSet", opcode: 0xfb05, wat: "struct.set", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayNew", opcode: 0xfb06, wat: "array.new", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayNewDefault", opcode: 0xfb07, wat: "array.new_default", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayNewFixed", opcode: 0xfb08, wat: "array.new_fixed", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayNewData", opcode: 0xfb09, wat: "array.new_data", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayNewElem", opcode: 0xfb0a, wat: "array.new_elem", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayGet", opcode: 0xfb0b, wat: "array.get", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayGetS", opcode: 0xfb0c, wat: "array.get_s", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayGetU", opcode: 0xfb0d, wat: "array.get_u", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArraySet", opcode: 0xfb0e, wat: "array.set", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayLen", opcode: 0xfb0f, wat: "array.len", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayFill", opcode: 0xfb10, wat: "array.fill", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayCopy", opcode: 0xfb11, wat: "array.copy", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayInitData", opcode: 0xfb12, wat: "array.init_data", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ArrayInitElem", opcode: 0xfb13, wat: "array.init_elem", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "RefTest", opcode: 0xfb14, wat: "ref.test", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "RefTestNull", opcode: 0xfb15, wat: "ref.test null", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "RefCast", opcode: 0xfb16, wat: "ref.cast", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "RefCastNull", opcode: 0xfb17, wat: "ref.cast null", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "BrOnCast", opcode: 0xfb18, wat: "br_on_cast", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "BrOnCastFail", opcode: 0xfb19, wat: "br_on_cast_fail", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "AnyConvertExtern", opcode: 0xfb1a, wat: "any.convert_extern", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "ExternConvertAny", opcode: 0xfb1b, wat: "extern.convert_any", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "RefI31", opcode: 0xfb1c, wat: "ref.i31", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "I31GetS", opcode: 0xfb1d, wat: "i31.get_s", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "I31GetU", opcode: 0xfb1e, wat: "i31.get_u", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "RefI31Shared", opcode: 0xfb1f, wat: "ref.i31_shared", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StructNewDesc", opcode: 0xfb20, wat: "struct.new_desc", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StructNewDefaultDesc", opcode: 0xfb21, wat: "struct.new_default_desc", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "RefGetDesc", opcode: 0xfb22, wat: "ref.get_desc", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "RefCastDescEq", opcode: 0xfb23, wat: "ref.cast_desc_eq", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "RefCastDescEqNull", opcode: 0xfb24, wat: "ref.cast_desc_eq null", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "BrOnCastDescEq", opcode: 0xfb25, wat: "br_on_cast_desc_eq", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "BrOnCastDescEqFail", opcode: 0xfb26, wat: "br_on_cast_desc_eq_fail", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "RefCastNop", opcode: 0xfb4c, wat: "ref.cast_nop", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringNewUtf8", opcode: 0xfb80, wat: "string.new_utf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringNewWtf16", opcode: 0xfb81, wat: "string.new_wtf16", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringConst", opcode: 0xfb82, wat: "string.const", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringMeasureUtf8", opcode: 0xfb83, wat: "string.measure_utf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringMeasureWtf8", opcode: 0xfb84, wat: "string.measure_wtf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringMeasureWtf16", opcode: 0xfb85, wat: "string.measure_wtf16", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringEncodeUtf8", opcode: 0xfb86, wat: "string.encode_utf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringEncodeWtf16", opcode: 0xfb87, wat: "string.encode_wtf16", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringConcat", opcode: 0xfb88, wat: "string.concat", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringEq", opcode: 0xfb89, wat: "string.eq", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringIsUSVSequence", opcode: 0xfb8a, wat: "string.is_usv_sequence", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringNewLossyUtf8", opcode: 0xfb8b, wat: "string.new_lossy_utf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringNewWtf8", opcode: 0xfb8c, wat: "string.new_wtf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringEncodeLossyUtf8", opcode: 0xfb8d, wat: "string.encode_lossy_utf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringEncodeWtf8", opcode: 0xfb8e, wat: "string.encode_wtf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringNewUtf8Try", opcode: 0xfb8f, wat: "string.new_utf8_try", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringAsWtf8", opcode: 0xfb90, wat: "string.as_wtf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewWtf8Advance", opcode: 0xfb91, wat: "stringview_wtf8.advance", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewWtf8EncodeUtf8", opcode: 0xfb92, wat: "stringview_wtf8.encode_utf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewWtf8Slice", opcode: 0xfb93, wat: "stringview_wtf8.slice", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewWtf8EncodeLossyUtf8", opcode: 0xfb94, wat: "stringview_wtf8.encode_lossy_utf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewWtf8EncodeWtf8", opcode: 0xfb95, wat: "stringview_wtf8.encode_wtf8", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringAsWtf16", opcode: 0xfb98, wat: "string.as_wtf16", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewWtf16Length", opcode: 0xfb99, wat: "stringview_wtf16.length", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewWtf16GetCodeunit", opcode: 0xfb9a, wat: "stringview_wtf16.get_codeunit", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewWtf16Encode", opcode: 0xfb9b, wat: "stringview_wtf16.encode", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewWtf16Slice", opcode: 0xfb9c, wat: "stringview_wtf16.slice", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringAsIter", opcode: 0xfba0, wat: "string.as_iter", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewIterNext", opcode: 0xfba1, wat: "stringview_iter.next", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewIterAdvance", opcode: 0xfba2, wat: "stringview_iter.advance", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewIterRewind", opcode: 0xfba3, wat: "stringview_iter.rewind", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringViewIterSlice", opcode: 0xfba4, wat: "stringview_iter.slice", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringCompare", opcode: 0xfba8, wat: "string.compare", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringFromCodePoint", opcode: 0xfba9, wat: "string.from_code_point", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringHash", opcode: 0xfbaa, wat: "string.hash", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringNewUtf8Array", opcode: 0xfbb0, wat: "string.new_utf8_array", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringNewWtf16Array", opcode: 0xfbb1, wat: "string.new_wtf16_array", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringEncodeUtf8Array", opcode: 0xfbb2, wat: "string.encode_utf8_array", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringEncodeWtf16Array", opcode: 0xfbb3, wat: "string.encode_wtf16_array", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringNewLossyUtf8Array", opcode: 0xfbb4, wat: "string.new_lossy_utf8_array", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringNewWtf8Array", opcode: 0xfbb5, wat: "string.new_wtf8_array", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringEncodeLossyUtf8Array", opcode: 0xfbb6, wat: "string.encode_lossy_utf8_array", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringEncodeWtf8Array", opcode: 0xfbb7, wat: "string.encode_wtf8_array", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "StringNewUtf8ArrayTry", opcode: 0xfbb8, wat: "string.new_utf8_array_try", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfb },
  OpcodeDef { name: "Pause", opcode: 0xfe04, wat: "pause", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructWait", opcode: 0xfe05, wat: "struct.wait", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "WaitqueueNotify", opcode: 0xfe06, wat: "waitqueue.notify", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "WaitqueueNew", opcode: 0xfe07, wat: "waitqueue.new", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayWait", opcode: 0xfe08, wat: "array.wait", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "Publish", opcode: 0xfe0f, wat: "publish", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicGet", opcode: 0xfe5c, wat: "struct.atomic.get", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicGetS", opcode: 0xfe5d, wat: "struct.atomic.get_s", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicGetU", opcode: 0xfe5e, wat: "struct.atomic.get_u", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicSet", opcode: 0xfe5f, wat: "struct.atomic.set", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicAdd", opcode: 0xfe60, wat: "struct.atomic.rmw.add", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicSub", opcode: 0xfe61, wat: "struct.atomic.rmw.sub", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicAnd", opcode: 0xfe62, wat: "struct.atomic.rmw.and", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicOr", opcode: 0xfe63, wat: "struct.atomic.rmw.or", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicXor", opcode: 0xfe64, wat: "struct.atomic.rmw.xor", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicExchange", opcode: 0xfe65, wat: "struct.atomic.rmw.xchg", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "StructAtomicCompareExchange", opcode: 0xfe66, wat: "struct.atomic.rmw.cmpxchg", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicGet", opcode: 0xfe67, wat: "array.atomic.get", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicGetS", opcode: 0xfe68, wat: "array.atomic.get_s", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicGetU", opcode: 0xfe69, wat: "array.atomic.get_u", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicSet", opcode: 0xfe6a, wat: "array.atomic.set", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicAdd", opcode: 0xfe6b, wat: "array.atomic.rmw.add", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicSub", opcode: 0xfe6c, wat: "array.atomic.rmw.sub", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicAnd", opcode: 0xfe6d, wat: "array.atomic.rmw.and", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicOr", opcode: 0xfe6e, wat: "array.atomic.rmw.or", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicXor", opcode: 0xfe6f, wat: "array.atomic.rmw.xor", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicExchange", opcode: 0xfe70, wat: "array.atomic.rmw.xchg", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
  OpcodeDef { name: "ArrayAtomicCompareExchange", opcode: 0xfe71, wat: "array.atomic.rmw.cmpxchg", sig: SIG_NONE, sig64: SIG_NONE, prefix: 0xfe },
];

/// 无前缀短操作码签名表 (对应 impl::kShortSigTable)
#[rustfmt::skip]
pub static SHORT_SIG_TABLE: [WasmOpcodeSig; 256] = [
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::I_I, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II,
  WasmOpcodeSig::I_L, WasmOpcodeSig::I_LL, WasmOpcodeSig::I_LL, WasmOpcodeSig::I_LL, WasmOpcodeSig::I_LL, WasmOpcodeSig::I_LL, WasmOpcodeSig::I_LL, WasmOpcodeSig::I_LL, WasmOpcodeSig::I_LL, WasmOpcodeSig::I_LL, WasmOpcodeSig::I_LL, WasmOpcodeSig::I_FF, WasmOpcodeSig::I_FF, WasmOpcodeSig::I_FF, WasmOpcodeSig::I_FF, WasmOpcodeSig::I_FF,
  WasmOpcodeSig::I_FF, WasmOpcodeSig::I_DD, WasmOpcodeSig::I_DD, WasmOpcodeSig::I_DD, WasmOpcodeSig::I_DD, WasmOpcodeSig::I_DD, WasmOpcodeSig::I_DD, WasmOpcodeSig::I_I, WasmOpcodeSig::I_I, WasmOpcodeSig::I_I, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II,
  WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::L_L, WasmOpcodeSig::L_L, WasmOpcodeSig::L_L, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL,
  WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::F_F, WasmOpcodeSig::F_F, WasmOpcodeSig::F_F, WasmOpcodeSig::F_F, WasmOpcodeSig::F_F,
  WasmOpcodeSig::F_F, WasmOpcodeSig::F_F, WasmOpcodeSig::F_FF, WasmOpcodeSig::F_FF, WasmOpcodeSig::F_FF, WasmOpcodeSig::F_FF, WasmOpcodeSig::F_FF, WasmOpcodeSig::F_FF, WasmOpcodeSig::F_FF, WasmOpcodeSig::D_D, WasmOpcodeSig::D_D, WasmOpcodeSig::D_D, WasmOpcodeSig::D_D, WasmOpcodeSig::D_D, WasmOpcodeSig::D_D, WasmOpcodeSig::D_D,
  WasmOpcodeSig::D_DD, WasmOpcodeSig::D_DD, WasmOpcodeSig::D_DD, WasmOpcodeSig::D_DD, WasmOpcodeSig::D_DD, WasmOpcodeSig::D_DD, WasmOpcodeSig::D_DD, WasmOpcodeSig::I_L, WasmOpcodeSig::I_F, WasmOpcodeSig::I_F, WasmOpcodeSig::I_D, WasmOpcodeSig::I_D, WasmOpcodeSig::L_I, WasmOpcodeSig::L_I, WasmOpcodeSig::L_F, WasmOpcodeSig::L_F,
  WasmOpcodeSig::L_D, WasmOpcodeSig::L_D, WasmOpcodeSig::F_I, WasmOpcodeSig::F_I, WasmOpcodeSig::F_L, WasmOpcodeSig::F_L, WasmOpcodeSig::F_D, WasmOpcodeSig::D_I, WasmOpcodeSig::D_I, WasmOpcodeSig::D_L, WasmOpcodeSig::D_L, WasmOpcodeSig::D_F, WasmOpcodeSig::I_F, WasmOpcodeSig::L_D, WasmOpcodeSig::F_I, WasmOpcodeSig::D_L,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
];

/// SIMD MVP 表达式签名表 (对应 impl::kSimdExprSigTable; 取 MVP_0+MEM+MEM_1_OPERAND)
#[rustfmt::skip]
pub static SIMD_EXPR_SIG_TABLE: [WasmOpcodeSig; 256] = [
  WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::V_IS, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_I,
  WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_L, WasmOpcodeSig::S_F, WasmOpcodeSig::S_D, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS,
  WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS,
  WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS,
  WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SSS, WasmOpcodeSig::I_S, WasmOpcodeSig::S_IS, WasmOpcodeSig::S_IS, WasmOpcodeSig::S_IS, WasmOpcodeSig::S_IS, WasmOpcodeSig::V_IS, WasmOpcodeSig::V_IS, WasmOpcodeSig::V_IS, WasmOpcodeSig::V_IS, WasmOpcodeSig::S_I, WasmOpcodeSig::S_I, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S,
  WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::I_S, WasmOpcodeSig::I_S, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS,
  WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S,
  WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SS, WasmOpcodeSig::I_S, WasmOpcodeSig::I_S, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS,
  WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::None, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS,
  WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::None, WasmOpcodeSig::I_S, WasmOpcodeSig::I_S, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SS, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::S_SS, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::None, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS,
  WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::None, WasmOpcodeSig::I_S, WasmOpcodeSig::I_S, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SI, WasmOpcodeSig::S_SS, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::S_SS, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS,
  WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::None, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::None, WasmOpcodeSig::S_S,
  WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S,
];

/// Relaxed SIMD 表达式签名表 (对应 impl::kRelaxedSimdExprSigTable)
#[rustfmt::skip]
pub static RELAXED_SIMD_EXPR_SIG_TABLE: [WasmOpcodeSig; 256] = [
  WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SSS, WasmOpcodeSig::S_SSS, WasmOpcodeSig::S_SSS, WasmOpcodeSig::S_SSS, WasmOpcodeSig::S_SSS, WasmOpcodeSig::S_SSS, WasmOpcodeSig::S_SSS, WasmOpcodeSig::S_SSS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS,
  WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SSS, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::S_F, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS,
  WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_SS, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::S_S, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::S_SSS, WasmOpcodeSig::S_SSS,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
];

/// Atomic 表达式签名表 mem32 (对应 impl::kAtomicExprSigTableMem32)
#[rustfmt::skip]
pub static ATOMIC_EXPR_SIG_TABLE_MEM32: [WasmOpcodeSig; 256] = [
  WasmOpcodeSig::I_II, WasmOpcodeSig::I_IIL, WasmOpcodeSig::I_ILL, WasmOpcodeSig::V_V, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::I_I, WasmOpcodeSig::L_I, WasmOpcodeSig::I_I, WasmOpcodeSig::I_I, WasmOpcodeSig::L_I, WasmOpcodeSig::L_I, WasmOpcodeSig::L_I, WasmOpcodeSig::V_II, WasmOpcodeSig::V_IL, WasmOpcodeSig::V_II, WasmOpcodeSig::V_II, WasmOpcodeSig::V_IL, WasmOpcodeSig::V_IL, WasmOpcodeSig::V_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL,
  WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II,
  WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL,
  WasmOpcodeSig::L_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL, WasmOpcodeSig::I_II, WasmOpcodeSig::I_II, WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL, WasmOpcodeSig::L_IL, WasmOpcodeSig::I_III, WasmOpcodeSig::L_ILL, WasmOpcodeSig::I_III, WasmOpcodeSig::I_III, WasmOpcodeSig::L_ILL, WasmOpcodeSig::L_ILL, WasmOpcodeSig::L_ILL, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
];

/// Atomic 表达式签名表 mem64 (对应 impl::kAtomicExprSigTableMem64)
#[rustfmt::skip]
pub static ATOMIC_EXPR_SIG_TABLE_MEM64: [WasmOpcodeSig; 256] = [
  WasmOpcodeSig::I_LI, WasmOpcodeSig::I_LIL, WasmOpcodeSig::I_LLL, WasmOpcodeSig::V_V, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::I_L, WasmOpcodeSig::L_L, WasmOpcodeSig::I_L, WasmOpcodeSig::I_L, WasmOpcodeSig::L_L, WasmOpcodeSig::L_L, WasmOpcodeSig::L_L, WasmOpcodeSig::V_LI, WasmOpcodeSig::V_LL, WasmOpcodeSig::V_LI, WasmOpcodeSig::V_LI, WasmOpcodeSig::V_LL, WasmOpcodeSig::V_LL, WasmOpcodeSig::V_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL,
  WasmOpcodeSig::I_LI, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::I_LI,
  WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL,
  WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LI, WasmOpcodeSig::I_LI, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::L_LL, WasmOpcodeSig::I_LII, WasmOpcodeSig::L_LLL, WasmOpcodeSig::I_LII, WasmOpcodeSig::I_LII, WasmOpcodeSig::L_LLL, WasmOpcodeSig::L_LLL, WasmOpcodeSig::L_LLL, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
];

/// Numeric 前缀签名表 (对应 impl::kNumericExprSigTable; 仅 WITH_SIG 子集)
#[rustfmt::skip]
pub static NUMERIC_EXPR_SIG_TABLE: [WasmOpcodeSig; 256] = [
  WasmOpcodeSig::I_F, WasmOpcodeSig::I_F, WasmOpcodeSig::I_D, WasmOpcodeSig::I_D, WasmOpcodeSig::L_F, WasmOpcodeSig::L_F, WasmOpcodeSig::L_D, WasmOpcodeSig::L_D, WasmOpcodeSig::None, WasmOpcodeSig::V_V, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::V_III, WasmOpcodeSig::V_V, WasmOpcodeSig::V_III, WasmOpcodeSig::None,
  WasmOpcodeSig::I_V, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
  WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None, WasmOpcodeSig::None,
];

/// TrapReason: wasm 陷阱原因 (对应 C++ enum TrapReason; 列表源 src/common/globals.h FOREACH_WASM_TRAPREASON)
/// variant 采 rust 惯例命名; 原版 kTrapX 名以各条 /// v8: doc 锚保留可追溯, 判别值 (=序号) 与原版一致
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TrapReason {
  /// v8: kTrapUnreachable
  Unreachable = 0,
  /// v8: kTrapMemOutOfBounds
  MemOutOfBounds = 1,
  /// v8: kTrapUnalignedAccess
  UnalignedAccess = 2,
  /// v8: kTrapDivByZero
  DivByZero = 3,
  /// v8: kTrapDivUnrepresentable
  DivUnrepresentable = 4,
  /// v8: kTrapRemByZero
  RemByZero = 5,
  /// v8: kTrapFloatUnrepresentable
  FloatUnrepresentable = 6,
  /// v8: kTrapNullFunc
  NullFunc = 7,
  /// v8: kTrapFuncSigMismatch
  FuncSigMismatch = 8,
  /// v8: kTrapDataSegmentOutOfBounds
  DataSegmentOutOfBounds = 9,
  /// v8: kTrapElementSegmentOutOfBounds
  ElementSegmentOutOfBounds = 10,
  /// v8: kTrapTableOutOfBounds
  TableOutOfBounds = 11,
  /// v8: kTrapRethrowNull
  RethrowNull = 12,
  /// v8: kTrapNullDereference
  NullDereference = 13,
  /// v8: kTrapIllegalCast
  IllegalCast = 14,
  /// v8: kTrapArrayOutOfBounds
  ArrayOutOfBounds = 15,
  /// v8: kTrapArrayTooLarge
  ArrayTooLarge = 16,
  /// v8: kTrapResume
  Resume = 17,
  /// v8: kTrapSuspend
  Suspend = 18,
  /// v8: kTrapSwitch
  Switch = 19,
  /// v8: kTrapStringOffsetOutOfBounds
  StringOffsetOutOfBounds = 20,
}

/// kTrapCount
pub const TRAP_COUNT: u8 = 21;

/// WasmOpcodes 静态方法族 (对应 class WasmOpcodes, 规则直译)
impl WasmOpcode {
  /// 操作码名称 (对应 WasmOpcodes::OpcodeName; 未知名返回 "invalid opcode")
  #[inline]
  #[must_use]
  pub const fn opcode_name(self) -> &'static str {
    match self.0 {
      0x0 => "Unreachable",
      0x1 => "Nop",
      0x2 => "Block",
      0x3 => "Loop",
      0x4 => "If",
      0x5 => "Else",
      0x6 => "Try",
      0x7 => "Catch",
      0x8 => "Throw",
      0x9 => "Rethrow",
      0xa => "ThrowRef",
      0xb => "End",
      0xc => "Br",
      0xd => "BrIf",
      0xe => "BrTable",
      0xf => "Return",
      0x10 => "CallFunction",
      0x11 => "CallIndirect",
      0x12 => "ReturnCall",
      0x13 => "ReturnCallIndirect",
      0x14 => "CallRef",
      0x15 => "ReturnCallRef",
      0x16 => "NopForTestingUnsupportedInLiftoff",
      0x18 => "Delegate",
      0x19 => "CatchAll",
      0x1a => "Drop",
      0x1b => "Select",
      0x1c => "SelectWithType",
      0x1f => "TryTable",
      0x20 => "LocalGet",
      0x21 => "LocalSet",
      0x22 => "LocalTee",
      0x23 => "GlobalGet",
      0x24 => "GlobalSet",
      0x25 => "TableGet",
      0x26 => "TableSet",
      0x28 => "I32LoadMem",
      0x29 => "I64LoadMem",
      0x2a => "F32LoadMem",
      0x2b => "F64LoadMem",
      0x2c => "I32LoadMem8S",
      0x2d => "I32LoadMem8U",
      0x2e => "I32LoadMem16S",
      0x2f => "I32LoadMem16U",
      0x30 => "I64LoadMem8S",
      0x31 => "I64LoadMem8U",
      0x32 => "I64LoadMem16S",
      0x33 => "I64LoadMem16U",
      0x34 => "I64LoadMem32S",
      0x35 => "I64LoadMem32U",
      0x36 => "I32StoreMem",
      0x37 => "I64StoreMem",
      0x38 => "F32StoreMem",
      0x39 => "F64StoreMem",
      0x3a => "I32StoreMem8",
      0x3b => "I32StoreMem16",
      0x3c => "I64StoreMem8",
      0x3d => "I64StoreMem16",
      0x3e => "I64StoreMem32",
      0x3f => "MemorySize",
      0x40 => "MemoryGrow",
      0x41 => "I32Const",
      0x42 => "I64Const",
      0x43 => "F32Const",
      0x44 => "F64Const",
      0x45 => "I32Eqz",
      0x46 => "I32Eq",
      0x47 => "I32Ne",
      0x48 => "I32LtS",
      0x49 => "I32LtU",
      0x4a => "I32GtS",
      0x4b => "I32GtU",
      0x4c => "I32LeS",
      0x4d => "I32LeU",
      0x4e => "I32GeS",
      0x4f => "I32GeU",
      0x50 => "I64Eqz",
      0x51 => "I64Eq",
      0x52 => "I64Ne",
      0x53 => "I64LtS",
      0x54 => "I64LtU",
      0x55 => "I64GtS",
      0x56 => "I64GtU",
      0x57 => "I64LeS",
      0x58 => "I64LeU",
      0x59 => "I64GeS",
      0x5a => "I64GeU",
      0x5b => "F32Eq",
      0x5c => "F32Ne",
      0x5d => "F32Lt",
      0x5e => "F32Gt",
      0x5f => "F32Le",
      0x60 => "F32Ge",
      0x61 => "F64Eq",
      0x62 => "F64Ne",
      0x63 => "F64Lt",
      0x64 => "F64Gt",
      0x65 => "F64Le",
      0x66 => "F64Ge",
      0x67 => "I32Clz",
      0x68 => "I32Ctz",
      0x69 => "I32Popcnt",
      0x6a => "I32Add",
      0x6b => "I32Sub",
      0x6c => "I32Mul",
      0x6d => "I32DivS",
      0x6e => "I32DivU",
      0x6f => "I32RemS",
      0x70 => "I32RemU",
      0x71 => "I32And",
      0x72 => "I32Ior",
      0x73 => "I32Xor",
      0x74 => "I32Shl",
      0x75 => "I32ShrS",
      0x76 => "I32ShrU",
      0x77 => "I32Rol",
      0x78 => "I32Ror",
      0x79 => "I64Clz",
      0x7a => "I64Ctz",
      0x7b => "I64Popcnt",
      0x7c => "I64Add",
      0x7d => "I64Sub",
      0x7e => "I64Mul",
      0x7f => "I64DivS",
      0x80 => "I64DivU",
      0x81 => "I64RemS",
      0x82 => "I64RemU",
      0x83 => "I64And",
      0x84 => "I64Ior",
      0x85 => "I64Xor",
      0x86 => "I64Shl",
      0x87 => "I64ShrS",
      0x88 => "I64ShrU",
      0x89 => "I64Rol",
      0x8a => "I64Ror",
      0x8b => "F32Abs",
      0x8c => "F32Neg",
      0x8d => "F32Ceil",
      0x8e => "F32Floor",
      0x8f => "F32Trunc",
      0x90 => "F32NearestInt",
      0x91 => "F32Sqrt",
      0x92 => "F32Add",
      0x93 => "F32Sub",
      0x94 => "F32Mul",
      0x95 => "F32Div",
      0x96 => "F32Min",
      0x97 => "F32Max",
      0x98 => "F32CopySign",
      0x99 => "F64Abs",
      0x9a => "F64Neg",
      0x9b => "F64Ceil",
      0x9c => "F64Floor",
      0x9d => "F64Trunc",
      0x9e => "F64NearestInt",
      0x9f => "F64Sqrt",
      0xa0 => "F64Add",
      0xa1 => "F64Sub",
      0xa2 => "F64Mul",
      0xa3 => "F64Div",
      0xa4 => "F64Min",
      0xa5 => "F64Max",
      0xa6 => "F64CopySign",
      0xa7 => "I32ConvertI64",
      0xa8 => "I32SConvertF32",
      0xa9 => "I32UConvertF32",
      0xaa => "I32SConvertF64",
      0xab => "I32UConvertF64",
      0xac => "I64SConvertI32",
      0xad => "I64UConvertI32",
      0xae => "I64SConvertF32",
      0xaf => "I64UConvertF32",
      0xb0 => "I64SConvertF64",
      0xb1 => "I64UConvertF64",
      0xb2 => "F32SConvertI32",
      0xb3 => "F32UConvertI32",
      0xb4 => "F32SConvertI64",
      0xb5 => "F32UConvertI64",
      0xb6 => "F32ConvertF64",
      0xb7 => "F64SConvertI32",
      0xb8 => "F64UConvertI32",
      0xb9 => "F64SConvertI64",
      0xba => "F64UConvertI64",
      0xbb => "F64ConvertF32",
      0xbc => "I32ReinterpretF32",
      0xbd => "I64ReinterpretF64",
      0xbe => "F32ReinterpretI32",
      0xbf => "F64ReinterpretI64",
      0xc0 => "I32SExtendI8",
      0xc1 => "I32SExtendI16",
      0xc2 => "I64SExtendI8",
      0xc3 => "I64SExtendI16",
      0xc4 => "I64SExtendI32",
      0xd0 => "RefNull",
      0xd1 => "RefIsNull",
      0xd2 => "RefFunc",
      0xd3 => "RefEq",
      0xd4 => "RefAsNonNull",
      0xd5 => "BrOnNull",
      0xd6 => "BrOnNonNull",
      0xe0 => "ContNew",
      0xe1 => "ContBind",
      0xe2 => "Suspend",
      0xe3 => "Resume",
      0xe4 => "ResumeThrow",
      0xe5 => "ResumeThrowRef",
      0xe6 => "Switch",
      0xfb00 => "StructNew",
      0xfb01 => "StructNewDefault",
      0xfb02 => "StructGet",
      0xfb03 => "StructGetS",
      0xfb04 => "StructGetU",
      0xfb05 => "StructSet",
      0xfb06 => "ArrayNew",
      0xfb07 => "ArrayNewDefault",
      0xfb08 => "ArrayNewFixed",
      0xfb09 => "ArrayNewData",
      0xfb0a => "ArrayNewElem",
      0xfb0b => "ArrayGet",
      0xfb0c => "ArrayGetS",
      0xfb0d => "ArrayGetU",
      0xfb0e => "ArraySet",
      0xfb0f => "ArrayLen",
      0xfb10 => "ArrayFill",
      0xfb11 => "ArrayCopy",
      0xfb12 => "ArrayInitData",
      0xfb13 => "ArrayInitElem",
      0xfb14 => "RefTest",
      0xfb15 => "RefTestNull",
      0xfb16 => "RefCast",
      0xfb17 => "RefCastNull",
      0xfb18 => "BrOnCast",
      0xfb19 => "BrOnCastFail",
      0xfb1a => "AnyConvertExtern",
      0xfb1b => "ExternConvertAny",
      0xfb1c => "RefI31",
      0xfb1d => "I31GetS",
      0xfb1e => "I31GetU",
      0xfb1f => "RefI31Shared",
      0xfb20 => "StructNewDesc",
      0xfb21 => "StructNewDefaultDesc",
      0xfb22 => "RefGetDesc",
      0xfb23 => "RefCastDescEq",
      0xfb24 => "RefCastDescEqNull",
      0xfb25 => "BrOnCastDescEq",
      0xfb26 => "BrOnCastDescEqFail",
      0xfb4c => "RefCastNop",
      0xfb80 => "StringNewUtf8",
      0xfb81 => "StringNewWtf16",
      0xfb82 => "StringConst",
      0xfb83 => "StringMeasureUtf8",
      0xfb84 => "StringMeasureWtf8",
      0xfb85 => "StringMeasureWtf16",
      0xfb86 => "StringEncodeUtf8",
      0xfb87 => "StringEncodeWtf16",
      0xfb88 => "StringConcat",
      0xfb89 => "StringEq",
      0xfb8a => "StringIsUSVSequence",
      0xfb8b => "StringNewLossyUtf8",
      0xfb8c => "StringNewWtf8",
      0xfb8d => "StringEncodeLossyUtf8",
      0xfb8e => "StringEncodeWtf8",
      0xfb8f => "StringNewUtf8Try",
      0xfb90 => "StringAsWtf8",
      0xfb91 => "StringViewWtf8Advance",
      0xfb92 => "StringViewWtf8EncodeUtf8",
      0xfb93 => "StringViewWtf8Slice",
      0xfb94 => "StringViewWtf8EncodeLossyUtf8",
      0xfb95 => "StringViewWtf8EncodeWtf8",
      0xfb98 => "StringAsWtf16",
      0xfb99 => "StringViewWtf16Length",
      0xfb9a => "StringViewWtf16GetCodeunit",
      0xfb9b => "StringViewWtf16Encode",
      0xfb9c => "StringViewWtf16Slice",
      0xfba0 => "StringAsIter",
      0xfba1 => "StringViewIterNext",
      0xfba2 => "StringViewIterAdvance",
      0xfba3 => "StringViewIterRewind",
      0xfba4 => "StringViewIterSlice",
      0xfba8 => "StringCompare",
      0xfba9 => "StringFromCodePoint",
      0xfbaa => "StringHash",
      0xfbb0 => "StringNewUtf8Array",
      0xfbb1 => "StringNewWtf16Array",
      0xfbb2 => "StringEncodeUtf8Array",
      0xfbb3 => "StringEncodeWtf16Array",
      0xfbb4 => "StringNewLossyUtf8Array",
      0xfbb5 => "StringNewWtf8Array",
      0xfbb6 => "StringEncodeLossyUtf8Array",
      0xfbb7 => "StringEncodeWtf8Array",
      0xfbb8 => "StringNewUtf8ArrayTry",
      0xfc00 => "I32SConvertSatF32",
      0xfc01 => "I32UConvertSatF32",
      0xfc02 => "I32SConvertSatF64",
      0xfc03 => "I32UConvertSatF64",
      0xfc04 => "I64SConvertSatF32",
      0xfc05 => "I64UConvertSatF32",
      0xfc06 => "I64SConvertSatF64",
      0xfc07 => "I64UConvertSatF64",
      0xfc08 => "MemoryInit",
      0xfc09 => "DataDrop",
      0xfc0a => "MemoryCopy",
      0xfc0b => "MemoryFill",
      0xfc0c => "TableInit",
      0xfc0d => "ElemDrop",
      0xfc0e => "TableCopy",
      0xfc0f => "TableGrow",
      0xfc10 => "TableSize",
      0xfc11 => "TableFill",
      0xfc13 => "I64Add128",
      0xfc14 => "I64Sub128",
      0xfc15 => "I64MulWideS",
      0xfc16 => "I64MulWideU",
      0xfc30 => "F32LoadMemF16",
      0xfc31 => "F32StoreMemF16",
      0xfd00 => "S128LoadMem",
      0xfd01 => "S128Load8x8S",
      0xfd02 => "S128Load8x8U",
      0xfd03 => "S128Load16x4S",
      0xfd04 => "S128Load16x4U",
      0xfd05 => "S128Load32x2S",
      0xfd06 => "S128Load32x2U",
      0xfd07 => "S128Load8Splat",
      0xfd08 => "S128Load16Splat",
      0xfd09 => "S128Load32Splat",
      0xfd0a => "S128Load64Splat",
      0xfd0b => "S128StoreMem",
      0xfd0c => "S128Const",
      0xfd0d => "I8x16Shuffle",
      0xfd0e => "I8x16Swizzle",
      0xfd0f => "I8x16Splat",
      0xfd10 => "I16x8Splat",
      0xfd11 => "I32x4Splat",
      0xfd12 => "I64x2Splat",
      0xfd13 => "F32x4Splat",
      0xfd14 => "F64x2Splat",
      0xfd15 => "I8x16ExtractLaneS",
      0xfd16 => "I8x16ExtractLaneU",
      0xfd17 => "I8x16ReplaceLane",
      0xfd18 => "I16x8ExtractLaneS",
      0xfd19 => "I16x8ExtractLaneU",
      0xfd1a => "I16x8ReplaceLane",
      0xfd1b => "I32x4ExtractLane",
      0xfd1c => "I32x4ReplaceLane",
      0xfd1d => "I64x2ExtractLane",
      0xfd1e => "I64x2ReplaceLane",
      0xfd1f => "F32x4ExtractLane",
      0xfd20 => "F32x4ReplaceLane",
      0xfd21 => "F64x2ExtractLane",
      0xfd22 => "F64x2ReplaceLane",
      0xfd23 => "I8x16Eq",
      0xfd24 => "I8x16Ne",
      0xfd25 => "I8x16LtS",
      0xfd26 => "I8x16LtU",
      0xfd27 => "I8x16GtS",
      0xfd28 => "I8x16GtU",
      0xfd29 => "I8x16LeS",
      0xfd2a => "I8x16LeU",
      0xfd2b => "I8x16GeS",
      0xfd2c => "I8x16GeU",
      0xfd2d => "I16x8Eq",
      0xfd2e => "I16x8Ne",
      0xfd2f => "I16x8LtS",
      0xfd30 => "I16x8LtU",
      0xfd31 => "I16x8GtS",
      0xfd32 => "I16x8GtU",
      0xfd33 => "I16x8LeS",
      0xfd34 => "I16x8LeU",
      0xfd35 => "I16x8GeS",
      0xfd36 => "I16x8GeU",
      0xfd37 => "I32x4Eq",
      0xfd38 => "I32x4Ne",
      0xfd39 => "I32x4LtS",
      0xfd3a => "I32x4LtU",
      0xfd3b => "I32x4GtS",
      0xfd3c => "I32x4GtU",
      0xfd3d => "I32x4LeS",
      0xfd3e => "I32x4LeU",
      0xfd3f => "I32x4GeS",
      0xfd40 => "I32x4GeU",
      0xfd41 => "F32x4Eq",
      0xfd42 => "F32x4Ne",
      0xfd43 => "F32x4Lt",
      0xfd44 => "F32x4Gt",
      0xfd45 => "F32x4Le",
      0xfd46 => "F32x4Ge",
      0xfd47 => "F64x2Eq",
      0xfd48 => "F64x2Ne",
      0xfd49 => "F64x2Lt",
      0xfd4a => "F64x2Gt",
      0xfd4b => "F64x2Le",
      0xfd4c => "F64x2Ge",
      0xfd4d => "S128Not",
      0xfd4e => "S128And",
      0xfd4f => "S128AndNot",
      0xfd50 => "S128Or",
      0xfd51 => "S128Xor",
      0xfd52 => "S128Select",
      0xfd53 => "V128AnyTrue",
      0xfd54 => "S128Load8Lane",
      0xfd55 => "S128Load16Lane",
      0xfd56 => "S128Load32Lane",
      0xfd57 => "S128Load64Lane",
      0xfd58 => "S128Store8Lane",
      0xfd59 => "S128Store16Lane",
      0xfd5a => "S128Store32Lane",
      0xfd5b => "S128Store64Lane",
      0xfd5c => "S128Load32Zero",
      0xfd5d => "S128Load64Zero",
      0xfd5e => "F32x4DemoteF64x2Zero",
      0xfd5f => "F64x2PromoteLowF32x4",
      0xfd60 => "I8x16Abs",
      0xfd61 => "I8x16Neg",
      0xfd62 => "I8x16Popcnt",
      0xfd63 => "I8x16AllTrue",
      0xfd64 => "I8x16BitMask",
      0xfd65 => "I8x16SConvertI16x8",
      0xfd66 => "I8x16UConvertI16x8",
      0xfd67 => "F32x4Ceil",
      0xfd68 => "F32x4Floor",
      0xfd69 => "F32x4Trunc",
      0xfd6a => "F32x4NearestInt",
      0xfd6b => "I8x16Shl",
      0xfd6c => "I8x16ShrS",
      0xfd6d => "I8x16ShrU",
      0xfd6e => "I8x16Add",
      0xfd6f => "I8x16AddSatS",
      0xfd70 => "I8x16AddSatU",
      0xfd71 => "I8x16Sub",
      0xfd72 => "I8x16SubSatS",
      0xfd73 => "I8x16SubSatU",
      0xfd74 => "F64x2Ceil",
      0xfd75 => "F64x2Floor",
      0xfd76 => "I8x16MinS",
      0xfd77 => "I8x16MinU",
      0xfd78 => "I8x16MaxS",
      0xfd79 => "I8x16MaxU",
      0xfd7a => "F64x2Trunc",
      0xfd7b => "I8x16RoundingAverageU",
      0xfd7c => "I16x8ExtAddPairwiseI8x16S",
      0xfd7d => "I16x8ExtAddPairwiseI8x16U",
      0xfd7e => "I32x4ExtAddPairwiseI16x8S",
      0xfd7f => "I32x4ExtAddPairwiseI16x8U",
      0xfd80 => "I16x8Abs",
      0xfd81 => "I16x8Neg",
      0xfd82 => "I16x8Q15MulRSatS",
      0xfd83 => "I16x8AllTrue",
      0xfd84 => "I16x8BitMask",
      0xfd85 => "I16x8SConvertI32x4",
      0xfd86 => "I16x8UConvertI32x4",
      0xfd87 => "I16x8SConvertI8x16Low",
      0xfd88 => "I16x8SConvertI8x16High",
      0xfd89 => "I16x8UConvertI8x16Low",
      0xfd8a => "I16x8UConvertI8x16High",
      0xfd8b => "I16x8Shl",
      0xfd8c => "I16x8ShrS",
      0xfd8d => "I16x8ShrU",
      0xfd8e => "I16x8Add",
      0xfd8f => "I16x8AddSatS",
      0xfd90 => "I16x8AddSatU",
      0xfd91 => "I16x8Sub",
      0xfd92 => "I16x8SubSatS",
      0xfd93 => "I16x8SubSatU",
      0xfd94 => "F64x2NearestInt",
      0xfd95 => "I16x8Mul",
      0xfd96 => "I16x8MinS",
      0xfd97 => "I16x8MinU",
      0xfd98 => "I16x8MaxS",
      0xfd99 => "I16x8MaxU",
      0xfd9b => "I16x8RoundingAverageU",
      0xfd9c => "I16x8ExtMulLowI8x16S",
      0xfd9d => "I16x8ExtMulHighI8x16S",
      0xfd9e => "I16x8ExtMulLowI8x16U",
      0xfd9f => "I16x8ExtMulHighI8x16U",
      0xfda0 => "I32x4Abs",
      0xfda1 => "I32x4Neg",
      0xfda3 => "I32x4AllTrue",
      0xfda4 => "I32x4BitMask",
      0xfda7 => "I32x4SConvertI16x8Low",
      0xfda8 => "I32x4SConvertI16x8High",
      0xfda9 => "I32x4UConvertI16x8Low",
      0xfdaa => "I32x4UConvertI16x8High",
      0xfdab => "I32x4Shl",
      0xfdac => "I32x4ShrS",
      0xfdad => "I32x4ShrU",
      0xfdae => "I32x4Add",
      0xfdb1 => "I32x4Sub",
      0xfdb5 => "I32x4Mul",
      0xfdb6 => "I32x4MinS",
      0xfdb7 => "I32x4MinU",
      0xfdb8 => "I32x4MaxS",
      0xfdb9 => "I32x4MaxU",
      0xfdba => "I32x4DotI16x8S",
      0xfdbc => "I32x4ExtMulLowI16x8S",
      0xfdbd => "I32x4ExtMulHighI16x8S",
      0xfdbe => "I32x4ExtMulLowI16x8U",
      0xfdbf => "I32x4ExtMulHighI16x8U",
      0xfdc0 => "I64x2Abs",
      0xfdc1 => "I64x2Neg",
      0xfdc3 => "I64x2AllTrue",
      0xfdc4 => "I64x2BitMask",
      0xfdc7 => "I64x2SConvertI32x4Low",
      0xfdc8 => "I64x2SConvertI32x4High",
      0xfdc9 => "I64x2UConvertI32x4Low",
      0xfdca => "I64x2UConvertI32x4High",
      0xfdcb => "I64x2Shl",
      0xfdcc => "I64x2ShrS",
      0xfdcd => "I64x2ShrU",
      0xfdce => "I64x2Add",
      0xfdd1 => "I64x2Sub",
      0xfdd5 => "I64x2Mul",
      0xfdd6 => "I64x2Eq",
      0xfdd7 => "I64x2Ne",
      0xfdd8 => "I64x2LtS",
      0xfdd9 => "I64x2GtS",
      0xfdda => "I64x2LeS",
      0xfddb => "I64x2GeS",
      0xfddc => "I64x2ExtMulLowI32x4S",
      0xfddd => "I64x2ExtMulHighI32x4S",
      0xfdde => "I64x2ExtMulLowI32x4U",
      0xfddf => "I64x2ExtMulHighI32x4U",
      0xfde0 => "F32x4Abs",
      0xfde1 => "F32x4Neg",
      0xfde3 => "F32x4Sqrt",
      0xfde4 => "F32x4Add",
      0xfde5 => "F32x4Sub",
      0xfde6 => "F32x4Mul",
      0xfde7 => "F32x4Div",
      0xfde8 => "F32x4Min",
      0xfde9 => "F32x4Max",
      0xfdea => "F32x4Pmin",
      0xfdeb => "F32x4Pmax",
      0xfdec => "F64x2Abs",
      0xfded => "F64x2Neg",
      0xfdef => "F64x2Sqrt",
      0xfdf0 => "F64x2Add",
      0xfdf1 => "F64x2Sub",
      0xfdf2 => "F64x2Mul",
      0xfdf3 => "F64x2Div",
      0xfdf4 => "F64x2Min",
      0xfdf5 => "F64x2Max",
      0xfdf6 => "F64x2Pmin",
      0xfdf7 => "F64x2Pmax",
      0xfdf8 => "I32x4SConvertF32x4",
      0xfdf9 => "I32x4UConvertF32x4",
      0xfdfa => "F32x4SConvertI32x4",
      0xfdfb => "F32x4UConvertI32x4",
      0xfdfc => "I32x4TruncSatF64x2SZero",
      0xfdfd => "I32x4TruncSatF64x2UZero",
      0xfdfe => "F64x2ConvertLowI32x4S",
      0xfdff => "F64x2ConvertLowI32x4U",
      0xfe00 => "AtomicNotify",
      0xfe01 => "I32AtomicWait",
      0xfe02 => "I64AtomicWait",
      0xfe03 => "AtomicFence",
      0xfe04 => "Pause",
      0xfe05 => "StructWait",
      0xfe06 => "WaitqueueNotify",
      0xfe07 => "WaitqueueNew",
      0xfe08 => "ArrayWait",
      0xfe0f => "Publish",
      0xfe10 => "I32AtomicLoad",
      0xfe11 => "I64AtomicLoad",
      0xfe12 => "I32AtomicLoad8U",
      0xfe13 => "I32AtomicLoad16U",
      0xfe14 => "I64AtomicLoad8U",
      0xfe15 => "I64AtomicLoad16U",
      0xfe16 => "I64AtomicLoad32U",
      0xfe17 => "I32AtomicStore",
      0xfe18 => "I64AtomicStore",
      0xfe19 => "I32AtomicStore8U",
      0xfe1a => "I32AtomicStore16U",
      0xfe1b => "I64AtomicStore8U",
      0xfe1c => "I64AtomicStore16U",
      0xfe1d => "I64AtomicStore32U",
      0xfe1e => "I32AtomicAdd",
      0xfe1f => "I64AtomicAdd",
      0xfe20 => "I32AtomicAdd8U",
      0xfe21 => "I32AtomicAdd16U",
      0xfe22 => "I64AtomicAdd8U",
      0xfe23 => "I64AtomicAdd16U",
      0xfe24 => "I64AtomicAdd32U",
      0xfe25 => "I32AtomicSub",
      0xfe26 => "I64AtomicSub",
      0xfe27 => "I32AtomicSub8U",
      0xfe28 => "I32AtomicSub16U",
      0xfe29 => "I64AtomicSub8U",
      0xfe2a => "I64AtomicSub16U",
      0xfe2b => "I64AtomicSub32U",
      0xfe2c => "I32AtomicAnd",
      0xfe2d => "I64AtomicAnd",
      0xfe2e => "I32AtomicAnd8U",
      0xfe2f => "I32AtomicAnd16U",
      0xfe30 => "I64AtomicAnd8U",
      0xfe31 => "I64AtomicAnd16U",
      0xfe32 => "I64AtomicAnd32U",
      0xfe33 => "I32AtomicOr",
      0xfe34 => "I64AtomicOr",
      0xfe35 => "I32AtomicOr8U",
      0xfe36 => "I32AtomicOr16U",
      0xfe37 => "I64AtomicOr8U",
      0xfe38 => "I64AtomicOr16U",
      0xfe39 => "I64AtomicOr32U",
      0xfe3a => "I32AtomicXor",
      0xfe3b => "I64AtomicXor",
      0xfe3c => "I32AtomicXor8U",
      0xfe3d => "I32AtomicXor16U",
      0xfe3e => "I64AtomicXor8U",
      0xfe3f => "I64AtomicXor16U",
      0xfe40 => "I64AtomicXor32U",
      0xfe41 => "I32AtomicExchange",
      0xfe42 => "I64AtomicExchange",
      0xfe43 => "I32AtomicExchange8U",
      0xfe44 => "I32AtomicExchange16U",
      0xfe45 => "I64AtomicExchange8U",
      0xfe46 => "I64AtomicExchange16U",
      0xfe47 => "I64AtomicExchange32U",
      0xfe48 => "I32AtomicCompareExchange",
      0xfe49 => "I64AtomicCompareExchange",
      0xfe4a => "I32AtomicCompareExchange8U",
      0xfe4b => "I32AtomicCompareExchange16U",
      0xfe4c => "I64AtomicCompareExchange8U",
      0xfe4d => "I64AtomicCompareExchange16U",
      0xfe4e => "I64AtomicCompareExchange32U",
      0xfe5c => "StructAtomicGet",
      0xfe5d => "StructAtomicGetS",
      0xfe5e => "StructAtomicGetU",
      0xfe5f => "StructAtomicSet",
      0xfe60 => "StructAtomicAdd",
      0xfe61 => "StructAtomicSub",
      0xfe62 => "StructAtomicAnd",
      0xfe63 => "StructAtomicOr",
      0xfe64 => "StructAtomicXor",
      0xfe65 => "StructAtomicExchange",
      0xfe66 => "StructAtomicCompareExchange",
      0xfe67 => "ArrayAtomicGet",
      0xfe68 => "ArrayAtomicGetS",
      0xfe69 => "ArrayAtomicGetU",
      0xfe6a => "ArrayAtomicSet",
      0xfe6b => "ArrayAtomicAdd",
      0xfe6c => "ArrayAtomicSub",
      0xfe6d => "ArrayAtomicAnd",
      0xfe6e => "ArrayAtomicOr",
      0xfe6f => "ArrayAtomicXor",
      0xfe70 => "ArrayAtomicExchange",
      0xfe71 => "ArrayAtomicCompareExchange",
      0xfd100 => "I8x16RelaxedSwizzle",
      0xfd101 => "I32x4RelaxedTruncF32x4S",
      0xfd102 => "I32x4RelaxedTruncF32x4U",
      0xfd103 => "I32x4RelaxedTruncF64x2SZero",
      0xfd104 => "I32x4RelaxedTruncF64x2UZero",
      0xfd105 => "F32x4Qfma",
      0xfd106 => "F32x4Qfms",
      0xfd107 => "F64x2Qfma",
      0xfd108 => "F64x2Qfms",
      0xfd109 => "I8x16RelaxedLaneSelect",
      0xfd10a => "I16x8RelaxedLaneSelect",
      0xfd10b => "I32x4RelaxedLaneSelect",
      0xfd10c => "I64x2RelaxedLaneSelect",
      0xfd10d => "F32x4RelaxedMin",
      0xfd10e => "F32x4RelaxedMax",
      0xfd10f => "F64x2RelaxedMin",
      0xfd110 => "F64x2RelaxedMax",
      0xfd111 => "I16x8RelaxedQ15MulRS",
      0xfd112 => "I16x8DotI8x16I7x16S",
      0xfd113 => "I32x4DotI8x16I7x16AddS",
      0xfd120 => "F16x8Splat",
      0xfd121 => "F16x8ExtractLane",
      0xfd122 => "F16x8ReplaceLane",
      0xfd130 => "F16x8Abs",
      0xfd131 => "F16x8Neg",
      0xfd132 => "F16x8Sqrt",
      0xfd133 => "F16x8Ceil",
      0xfd134 => "F16x8Floor",
      0xfd135 => "F16x8Trunc",
      0xfd136 => "F16x8NearestInt",
      0xfd137 => "F16x8Eq",
      0xfd138 => "F16x8Ne",
      0xfd139 => "F16x8Lt",
      0xfd13a => "F16x8Gt",
      0xfd13b => "F16x8Le",
      0xfd13c => "F16x8Ge",
      0xfd13d => "F16x8Add",
      0xfd13e => "F16x8Sub",
      0xfd13f => "F16x8Mul",
      0xfd140 => "F16x8Div",
      0xfd141 => "F16x8Min",
      0xfd142 => "F16x8Max",
      0xfd143 => "F16x8Pmin",
      0xfd144 => "F16x8Pmax",
      0xfd145 => "I16x8SConvertF16x8",
      0xfd146 => "I16x8UConvertF16x8",
      0xfd147 => "F16x8SConvertI16x8",
      0xfd148 => "F16x8UConvertI16x8",
      0xfd149 => "F16x8DemoteF32x4Zero",
      0xfd14a => "F16x8DemoteF64x2Zero",
      0xfd14b => "F32x4PromoteLowF16x8",
      0xfd14e => "F16x8Qfma",
      0xfd14f => "F16x8Qfms",
      _ => "invalid opcode",
    }
  }

  /// 提取前缀字节 (对应 WasmOpcodes::ExtractPrefix)
  #[inline]
  #[must_use]
  pub const fn extract_prefix(self) -> u8 {
    if self.0 > 0xffff {
      (self.0 >> 12) as u8
    } else {
      (self.0 >> 8) as u8
    }
  }

  /// 是否前缀操作码 (对应 WasmOpcodes::IsPrefixOpcode)
  #[inline]
  #[must_use]
  pub const fn is_prefix_opcode(self) -> bool {
    // 原版 FOREACH_PREFIX 四前缀 0xfb(gc)/0xfc(numeric)/0xfd(simd)/0xfe(atomic) 连续
    matches!(self.0, 0xfb..=0xfe)
  }

  /// 是否可断点操作码 (对应 WasmOpcodes::IsBreakable; Block/Try/Catch/Loop/Else 不可断)
  #[inline]
  #[must_use]
  pub const fn is_breakable(self) -> bool {
    !matches!(self.0, 0x02..=0x03 | 0x05..=0x07)
  }

  /// 是否 relaxed-simd 操作码 (对应 WasmOpcodes::IsRelaxedSimdOpcode)
  #[inline]
  #[must_use]
  pub const fn is_relaxed_simd_opcode(self) -> bool {
    (self.0 & 0xfff00) == 0xfd100
  }

  /// 是否 FP16 SIMD 操作码 (对应 WasmOpcodes::IsFP16SimdOpcode)
  #[inline]
  #[must_use]
  pub const fn is_fp16_simd_opcode(self) -> bool {
    let v = self.0;
    (v >= prefixed_expr::F16X8_SPLAT.as_u32() && v <= prefixed_expr::F16X8_REPLACE_LANE.as_u32())
      || (v >= prefixed_expr::F16X8_ABS.as_u32() && v <= prefixed_expr::F16X8_QFMS.as_u32())
  }

  /// 是否 atomic read-modify-write 操作码 (对应 WasmOpcodes::IsAtomicRmwOpcode)
  #[inline]
  #[must_use]
  pub const fn is_atomic_rmw_opcode(self) -> bool {
    self.0 >= prefixed_expr::I32_ATOMIC_ADD.as_u32()
      && self.0 <= prefixed_expr::I64_ATOMIC_COMPARE_EXCHANGE32_U.as_u32()
  }

  /// 查操作码签名 (对应 WasmOpcodes::Signature; None 表示无表内签名)
  #[inline]
  #[must_use]
  pub const fn signature(self) -> WasmOpcodeSig {
    match self.extract_prefix() as u32 {
      0 => SHORT_SIG_TABLE[(self.0 & 0xff) as usize],
      p if p == prefix::SIMD => {
        if self.0 <= 0xfdff {
          SIMD_EXPR_SIG_TABLE[(self.0 & 0xff) as usize]
        } else if self.is_relaxed_simd_opcode() {
          RELAXED_SIMD_EXPR_SIG_TABLE[(self.0 & 0xff) as usize]
        } else {
          WasmOpcodeSig::None
        }
      }
      p if p == prefix::NUMERIC => NUMERIC_EXPR_SIG_TABLE[(self.0 & 0xff) as usize],
      _ => WasmOpcodeSig::None,
    }
  }

  /// atomic 操作码按 memory64 选签名 (对应 WasmOpcodes::SignatureForAtomicOp)
  #[inline]
  #[must_use]
  pub const fn signature_for_atomic_op(self, is_memory64: bool) -> WasmOpcodeSig {
    if is_memory64 {
      ATOMIC_EXPR_SIG_TABLE_MEM64[(self.0 & 0xff) as usize]
    } else {
      ATOMIC_EXPR_SIG_TABLE_MEM32[(self.0 & 0xff) as usize]
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(OPCODES.len() == 699);
  assert!(WASM_OPCODE_SIG_COUNT == 60);
  assert!(TRAP_COUNT == 21);
  assert!(expr::UNREACHABLE.as_u32() == 0x00);
  assert!(expr::NOP.as_u32() == 0x01);
  assert!(expr::END.as_u32() == 0x0b);
  assert!(prefixed_expr::I64_ATOMIC_COMPARE_EXCHANGE32_U.as_u32() == 0xfe4e);
  assert!(prefixed_expr::S128_CONST.as_u32() == 0xfd0c);
  assert!(prefixed_expr::I8X16_RELAXED_SWIZZLE.as_u32() == 0xfd100);
  assert!(WasmOpcode(0xfd5c).extract_prefix() == 0xfd);
  assert!(WasmOpcode(0x45).extract_prefix() == 0x00);
  assert!(!WasmOpcode(0x02).is_breakable());
  assert!(WasmOpcode(0x45).is_breakable());
  assert!(WasmOpcode(0xfd110).is_relaxed_simd_opcode());
  assert!(!WasmOpcode(0xfd0c).is_relaxed_simd_opcode());
  assert!(WasmOpcode(0xfd121).is_fp16_simd_opcode());
  assert!(!WasmOpcode(0xfd11f).is_fp16_simd_opcode());
  assert!(WasmOpcode(0xfe1e).is_atomic_rmw_opcode());
  assert!(!WasmOpcode(0xfe10).is_atomic_rmw_opcode());
  assert!(matches!(WasmOpcode(0x45).signature(), WasmOpcodeSig::I_I));
  assert!(matches!(WasmOpcode(0xfd00).signature(), WasmOpcodeSig::S_I));
  assert!(matches!(WasmOpcode(0xfc00).signature(), WasmOpcodeSig::I_F));
  assert!(matches!(
    WasmOpcode(0xfe10).signature_for_atomic_op(false),
    WasmOpcodeSig::I_I
  ));
  assert!(matches!(
    WasmOpcode(0xfe10).signature_for_atomic_op(true),
    WasmOpcodeSig::I_L
  ));
  assert!(TrapReason::MemOutOfBounds as u8 == 1);
};

/// 打印签名为短记号串 (对应 operator<<(ostream&, const FunctionSig&), 骨架)
pub fn write_sig_short(sb: &mut String, sig: &SigDef) {
  if sig.ret == SigType::Void {
    sb.push('v');
  } else {
    push_short(sb, sig.ret);
  }
  sb.push('_');
  if sig.params.is_empty() {
    sb.push('v');
  }
  for p in sig.params {
    push_short(sb, *p);
  }
}

fn push_short(sb: &mut String, t: SigType) {
  let c = match t {
    SigType::I32 => 'i',
    SigType::I64 => 'l',
    SigType::F32 => 'f',
    SigType::F64 => 'd',
    SigType::S128 => 's',
    _ => '?',
  };
  sb.push(c);
}

/// JS 兼容性判定 (对应 IsJSCompatibleSignature, 骨架: 语义待 Developer 修订)
pub fn is_js_compatible_signature(_sig: &SigDef) -> bool {
  unimplemented!("骨架: 参照原版 wasm-opcodes.cc IsJSCompatibleSignature 修订")
}
