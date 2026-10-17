// 由 tool/src/wasm/wasm-value.h.js 从 v8/src/wasm/wasm-value.h 生成; 修订就地改, 复跑覆盖
// WasmValue: 带类型信息的 wasm 值 (FOREACH_PRIMITIVE_WASMVAL_TYPE 机械展开)

use crate::value_type::ValueKind;

/// 原始类型条目 (对应 FOREACH_PRIMITIVE_WASMVAL_TYPE, 12 条)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimitiveWasmvalType {
  /// 方法名片段, 如 "i32" / "f32_boxed" (to_<name> 系列)
  pub name: &'static str,
  /// 对应 wasm 类型记号 (kWasmI32 等)
  pub localtype: ValueKind,
  /// C++ 载体类型 (int32_t / float / Simd128 等, 仅存档)
  pub ctype: &'static str,
}

/// FOREACH_PRIMITIVE_WASMVAL_TYPE 展开表 (源顺序)
pub static PRIMITIVE_WASMVAL_TYPES: &[PrimitiveWasmvalType] = &[
  PrimitiveWasmvalType {
    name: "i8",
    localtype: ValueKind::I8,
    ctype: "int8_t",
  },
  PrimitiveWasmvalType {
    name: "i16",
    localtype: ValueKind::I16,
    ctype: "int16_t",
  },
  PrimitiveWasmvalType {
    name: "i32",
    localtype: ValueKind::I32,
    ctype: "int32_t",
  },
  PrimitiveWasmvalType {
    name: "u32",
    localtype: ValueKind::I32,
    ctype: "uint32_t",
  },
  PrimitiveWasmvalType {
    name: "i64",
    localtype: ValueKind::I64,
    ctype: "int64_t",
  },
  PrimitiveWasmvalType {
    name: "u64",
    localtype: ValueKind::I64,
    ctype: "uint64_t",
  },
  PrimitiveWasmvalType {
    name: "f16",
    localtype: ValueKind::F16,
    ctype: "uint16_t",
  },
  PrimitiveWasmvalType {
    name: "f32",
    localtype: ValueKind::F32,
    ctype: "float",
  },
  PrimitiveWasmvalType {
    name: "f32_boxed",
    localtype: ValueKind::F32,
    ctype: "Float32",
  },
  PrimitiveWasmvalType {
    name: "f64",
    localtype: ValueKind::F64,
    ctype: "double",
  },
  PrimitiveWasmvalType {
    name: "f64_boxed",
    localtype: ValueKind::F64,
    ctype: "Float64",
  },
  PrimitiveWasmvalType {
    name: "s128",
    localtype: ValueKind::S128,
    ctype: "Simd128",
  },
];

/// 条目总数
pub const PRIMITIVE_WASMVAL_TYPE_COUNT: usize = 12;

/// WasmValue: 16 字节位模式 + 类型标签 (对应 class WasmValue, 字段布局直译)
#[derive(Clone, Copy)]
pub struct WasmValue {
  /// 类型 (CanonicalValueType; 骨架期以 ValueKind 代替, 语义修订时替换)
  pub type_: ValueKind,
  /// 位模式 (uint8_t bit_pattern_[16])
  pub bit_pattern_: [u8; 16],
}

/// 小端读位模式前 4/8 字节 (对应 ReadUnalignedValue; 布局保证无 panic)
const fn le_u32(b: &[u8; 16]) -> u32 {
  u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}
const fn le_u64(b: &[u8; 16]) -> u64 {
  u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

impl WasmValue {
  /// 默认: kWasmVoid + 全零位模式
  #[must_use]
  pub const fn new() -> Self {
    Self {
      type_: ValueKind::Void,
      bit_pattern_: [0; 16],
    }
  }

  /// 构造: 写入类型标签 + 位模式前 n 字节 (对应 WriteUnalignedValue 小端直写)
  const fn with_bits(kind: ValueKind, bytes: &[u8]) -> Self {
    let mut s = Self {
      type_: kind,
      bit_pattern_: [0u8; 16],
    };
    let mut i = 0;
    while i < bytes.len() {
      s.bit_pattern_[i] = bytes[i];
      i += 1;
    }
    s
  }

  /// 对应 int8_t 构造 (V(i8, kWasmI8, int8_t))
  #[must_use]
  pub fn from_i8(v: i8) -> Self {
    Self::with_bits(ValueKind::I8, &v.to_le_bytes())
  }
  /// 对应 to_i8() (DCHECK_EQ(kWasmI8, type_))
  #[must_use]
  pub fn to_i8(&self) -> i8 {
    debug_assert_eq!(ValueKind::I8, self.type_);
    self.to_i8_unchecked()
  }
  /// 对应 to_i8_unchecked()
  #[must_use]
  pub const fn to_i8_unchecked(&self) -> i8 {
    self.bit_pattern_[0] as i8
  }

  /// 对应 int16_t 构造 (V(i16, kWasmI16, int16_t))
  #[must_use]
  pub fn from_i16(v: i16) -> Self {
    Self::with_bits(ValueKind::I16, &v.to_le_bytes())
  }
  /// 对应 to_i16()
  #[must_use]
  pub fn to_i16(&self) -> i16 {
    debug_assert_eq!(ValueKind::I16, self.type_);
    self.to_i16_unchecked()
  }
  /// 对应 to_i16_unchecked()
  #[must_use]
  pub const fn to_i16_unchecked(&self) -> i16 {
    u16::from_le_bytes([self.bit_pattern_[0], self.bit_pattern_[1]]) as i16
  }

  /// 对应 int32_t 构造 (V(i32, kWasmI32, int32_t))
  #[must_use]
  pub fn from_i32(v: i32) -> Self {
    Self::with_bits(ValueKind::I32, &v.to_le_bytes())
  }
  /// 对应 to_i32()
  #[must_use]
  pub fn to_i32(&self) -> i32 {
    debug_assert_eq!(ValueKind::I32, self.type_);
    self.to_i32_unchecked()
  }
  /// 对应 to_i32_unchecked()
  #[must_use]
  pub const fn to_i32_unchecked(&self) -> i32 {
    le_u32(&self.bit_pattern_) as i32
  }

  /// 对应 uint32_t 构造 (V(u32, kWasmI32, uint32_t); 与 i32 共享类型标签)
  #[must_use]
  pub fn from_u32(v: u32) -> Self {
    Self::with_bits(ValueKind::I32, &v.to_le_bytes())
  }
  /// 对应 to_u32()
  #[must_use]
  pub fn to_u32(&self) -> u32 {
    debug_assert_eq!(ValueKind::I32, self.type_);
    self.to_u32_unchecked()
  }
  /// 对应 to_u32_unchecked()
  #[must_use]
  pub const fn to_u32_unchecked(&self) -> u32 {
    le_u32(&self.bit_pattern_)
  }

  /// 对应 int64_t 构造 (V(i64, kWasmI64, int64_t))
  #[must_use]
  pub fn from_i64(v: i64) -> Self {
    Self::with_bits(ValueKind::I64, &v.to_le_bytes())
  }
  /// 对应 to_i64()
  #[must_use]
  pub fn to_i64(&self) -> i64 {
    debug_assert_eq!(ValueKind::I64, self.type_);
    self.to_i64_unchecked()
  }
  /// 对应 to_i64_unchecked()
  #[must_use]
  pub const fn to_i64_unchecked(&self) -> i64 {
    le_u64(&self.bit_pattern_) as i64
  }

  /// 对应 uint64_t 构造 (V(u64, kWasmI64, uint64_t); 与 i64 共享类型标签)
  #[must_use]
  pub fn from_u64(v: u64) -> Self {
    Self::with_bits(ValueKind::I64, &v.to_le_bytes())
  }
  /// 对应 to_u64()
  #[must_use]
  pub fn to_u64(&self) -> u64 {
    debug_assert_eq!(ValueKind::I64, self.type_);
    self.to_u64_unchecked()
  }
  /// 对应 to_u64_unchecked()
  #[must_use]
  pub const fn to_u64_unchecked(&self) -> u64 {
    le_u64(&self.bit_pattern_)
  }

  /// 对应 uint16_t 构造 (V(f16, kWasmF16, uint16_t); f16 以位模式承载)
  #[must_use]
  pub fn from_f16(v: u16) -> Self {
    Self::with_bits(ValueKind::F16, &v.to_le_bytes())
  }
  /// 对应 to_f16()
  #[must_use]
  pub fn to_f16(&self) -> u16 {
    debug_assert_eq!(ValueKind::F16, self.type_);
    self.to_f16_unchecked()
  }
  /// 对应 to_f16_unchecked()
  #[must_use]
  pub const fn to_f16_unchecked(&self) -> u16 {
    u16::from_le_bytes([self.bit_pattern_[0], self.bit_pattern_[1]])
  }

  /// 对应 float 构造 (V(f32, kWasmF32, float))
  #[must_use]
  pub fn from_f32(v: f32) -> Self {
    Self::with_bits(ValueKind::F32, &v.to_le_bytes())
  }
  /// 对应 to_f32()
  #[must_use]
  pub fn to_f32(&self) -> f32 {
    debug_assert_eq!(ValueKind::F32, self.type_);
    self.to_f32_unchecked()
  }
  /// 对应 to_f32_unchecked()
  #[must_use]
  pub const fn to_f32_unchecked(&self) -> f32 {
    f32::from_bits(le_u32(&self.bit_pattern_))
  }

  /// 对应 Float32 构造 (V(f32_boxed, kWasmF32, Float32); 载体同为 4 字节位模式)
  #[must_use]
  pub fn from_f32_boxed(v: f32) -> Self {
    Self::with_bits(ValueKind::F32, &v.to_le_bytes())
  }
  /// 对应 to_f32_boxed()
  #[must_use]
  pub fn to_f32_boxed(&self) -> f32 {
    debug_assert_eq!(ValueKind::F32, self.type_);
    self.to_f32_boxed_unchecked()
  }
  /// 对应 to_f32_boxed_unchecked()
  #[must_use]
  pub const fn to_f32_boxed_unchecked(&self) -> f32 {
    f32::from_bits(le_u32(&self.bit_pattern_))
  }

  /// 对应 double 构造 (V(f64, kWasmF64, double))
  #[must_use]
  pub fn from_f64(v: f64) -> Self {
    Self::with_bits(ValueKind::F64, &v.to_le_bytes())
  }
  /// 对应 to_f64()
  #[must_use]
  pub fn to_f64(&self) -> f64 {
    debug_assert_eq!(ValueKind::F64, self.type_);
    self.to_f64_unchecked()
  }
  /// 对应 to_f64_unchecked()
  #[must_use]
  pub const fn to_f64_unchecked(&self) -> f64 {
    f64::from_bits(le_u64(&self.bit_pattern_))
  }

  /// 对应 Float64 构造 (V(f64_boxed, kWasmF64, Float64); 载体同为 8 字节位模式)
  #[must_use]
  pub fn from_f64_boxed(v: f64) -> Self {
    Self::with_bits(ValueKind::F64, &v.to_le_bytes())
  }
  /// 对应 to_f64_boxed()
  #[must_use]
  pub fn to_f64_boxed(&self) -> f64 {
    debug_assert_eq!(ValueKind::F64, self.type_);
    self.to_f64_boxed_unchecked()
  }
  /// 对应 to_f64_boxed_unchecked()
  #[must_use]
  pub const fn to_f64_boxed_unchecked(&self) -> f64 {
    f64::from_bits(le_u64(&self.bit_pattern_))
  }

  /// 对应 Simd128 构造 (V(s128, kWasmS128, Simd128); 全 16 字节)
  #[must_use]
  pub const fn from_s128(v: [u8; 16]) -> Self {
    Self {
      type_: ValueKind::S128,
      bit_pattern_: v,
    }
  }
  /// 对应 to_s128()
  #[must_use]
  pub fn to_s128(&self) -> [u8; 16] {
    debug_assert_eq!(ValueKind::S128, self.type_);
    self.to_s128_unchecked()
  }
  /// 对应 to_s128_unchecked()
  #[must_use]
  pub const fn to_s128_unchecked(&self) -> [u8; 16] {
    self.bit_pattern_
  }
}

impl Default for WasmValue {
  fn default() -> Self {
    Self::new()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  use crate::table_eq::str_eq;
  assert!(PRIMITIVE_WASMVAL_TYPES.len() == 12);
  assert!(str_eq(PRIMITIVE_WASMVAL_TYPES[0].name, "i8"));
  assert!(str_eq(PRIMITIVE_WASMVAL_TYPES[11].name, "s128"));
  assert!(size_of::<WasmValue>() >= 16);
};
