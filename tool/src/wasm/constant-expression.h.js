#!/usr/bin/env bun
// tool/src/wasm/constant-expression.h.js
// 机械提取 v8/src/wasm/constant-expression.h: ConstantExpression 位域与 Kind 枚举

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/constant-expression.h");

const kindMatch = hSrc.match(/enum class Kind\s*\{([^}]*)\};/);
if (!kindMatch) throw new Error("未找到 Kind 枚举");
const kinds = kindMatch[1]
  .split(",")
  .map((s) => s.trim())
  .filter((s) => s && !s.startsWith("//") && !s.startsWith("*"));
// 过滤 kLastKind 别名
const kindList = kinds.filter((k) => !k.includes("kLastKind"));
const lastAlias = kinds.find((k) => k.includes("kLastKind"));
if (!lastAlias) throw new Error("缺少 kLastKind 别名");
if (kindList.length !== 5) throw new Error(`Kind ${kindList.length} 项, 期望 5`);
if (kindList[0] !== "kEmpty" || kindList[kindList.length - 1] !== "kWireBytesRef") {
  throw new Error("Kind 锚点漂移");
}

const bits = {};
for (const [, k, v] of hSrc.matchAll(/static constexpr int k(\w+Bits) = (\d+);/g)) {
  bits[k] = Number(v);
}
for (const k of ["ValueBits", "LengthBits", "OffsetBits", "KindBits"]) {
  if (!(k in bits)) throw new Error(`位宽缺失: k${k}`);
}
// 布局: Offset(0..30) Length(30..60) Kind(60..63); 或 Value(0..32) Kind(60..63)
const lengthShift = bits.OffsetBits;
const kindShift = bits.OffsetBits + bits.LengthBits;
if (!hSrc.includes("kV8MaxWasmModuleSize <= LengthField::kMax + 1")) throw new Error("Length 容量断言缺失");
if (!hSrc.includes("sizeof(ConstantExpression) <= 8")) throw new Error("8 字节断言缺失");

console.log(`Kind ${kindList.length}, 位域 value=${bits.ValueBits}/offset=${bits.OffsetBits}/length=${bits.LengthBits}/kind=${bits.KindBits}`);

let rust = banner(
  "src/wasm/constant-expression.h.js",
  "src/wasm/constant-expression.h",
  "ConstantExpression 64 位位域编码 (Kind 枚举 + 字段布局)",
);

const camel = (s) => s.replace(/^k/, "").split("_").map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join("");

rust += `
/// 常量表达式种类 (对应 enum class Kind; kLastKind = kWireBytesRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Kind {
`;

for (const [i, k] of kindList.entries()) {
  rust += `  /// ${k}\n  ${camel(k)} = ${i},\n`;
}
rust += `}

impl Kind {
  /// 对应 kLastKind
  pub const LAST_KIND: Self = Self::WireBytesRef;
}

/// 位域宽度 (对应 kValueBits/kOffsetBits/kLengthBits/kKindBits)
pub const VALUE_BITS: i32 = ${bits.ValueBits};
pub const OFFSET_BITS: i32 = ${bits.OffsetBits};
pub const LENGTH_BITS: i32 = ${bits.LengthBits};
pub const KIND_BITS: i32 = ${bits.KindBits};

/// ConstantExpression: 64 位内联编码。
/// 两种布局: WireBytes 时 offset(0..30)+length(30..60)+kind(60..63);
/// 其余 value(0..32)+kind(60..63)。整体不超过 8 字节。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantExpression {
  bit_field_: u64,
}

impl ConstantExpression {
  /// 布局常量
  pub const LENGTH_SHIFT: i32 = ${lengthShift};
  pub const KIND_SHIFT: i32 = ${kindShift};

  /// 空表达式 (kind == kEmpty, bit_field 全零)
  #[must_use]
  pub const fn new() -> Self {
    Self { bit_field_: 0 }
  }

  /// 对应 I32Const(value)
  #[must_use]
  pub const fn i32_const(value: i32) -> Self {
    Self {
      bit_field_: ((value as u32) as u64) | ((Kind::I32Const as u64) << Self::KIND_SHIFT),
    }
  }

  /// 对应 RefFunc(index)
  #[must_use]
  pub const fn ref_func(index: u32) -> Self {
    Self { bit_field_: (index as u64) | ((Kind::RefFunc as u64) << Self::KIND_SHIFT) }
  }

  /// 对应 RefNull(heap_type_bits); 入参为 HeapType 原始位域
  #[must_use]
  pub const fn ref_null(heap_type_bits: u32) -> Self {
    Self {
      bit_field_: (heap_type_bits as u64) | ((Kind::RefNull as u64) << Self::KIND_SHIFT),
    }
  }

  /// 对应 WireBytes(offset, length)
  #[must_use]
  pub const fn wire_bytes(offset: u32, length: u32) -> Self {
    Self {
      bit_field_: (offset as u64)
        | ((length as u64) << Self::LENGTH_SHIFT)
        | ((Kind::WireBytesRef as u64) << Self::KIND_SHIFT),
    }
  }

  /// 对应 kind()
  #[must_use]
  pub const fn kind(&self) -> Kind {
    let k = ((self.bit_field_ >> Self::KIND_SHIFT) & ((1 << KIND_BITS) - 1)) as u8;
    if k == 0 {
      Kind::Empty
    } else if k == 1 {
      Kind::I32Const
    } else if k == 2 {
      Kind::RefNull
    } else if k == 3 {
      Kind::RefFunc
    } else {
      Kind::WireBytesRef
    }
  }

  /// 对应 is_set()
  #[must_use]
  pub const fn is_set(&self) -> bool {
    !matches!(self.kind(), Kind::Empty)
  }

  /// 对应 index() (kind == RefFunc)
  #[must_use]
  pub const fn index(&self) -> u32 {
    self.bit_field_ as u32
  }

  /// 对应 i32_value() (kind == I32Const)
  #[must_use]
  pub const fn i32_value(&self) -> i32 {
    self.bit_field_ as u32 as i32
  }

  /// 对应 wire_bytes_ref() -> (offset, length) (kind == WireBytesRef)
  #[must_use]
  pub const fn wire_bytes_ref(&self) -> (u32, u32) {
    (
      (self.bit_field_ & ((1 << OFFSET_BITS) - 1)) as u32,
      ((self.bit_field_ >> Self::LENGTH_SHIFT) & ((1 << LENGTH_BITS) - 1)) as u32,
    )
  }
}

impl Default for ConstantExpression {
  fn default() -> Self {
    Self::new()
  }
}

/// 求值结果 (对应 using ValueOrError = std::variant<WasmValue, MessageTemplate>)
pub type ValueOrError = Result<crate::wasm_value::WasmValue, ()>;

/// 求值常量表达式 (对应 EvaluateConstantExpression, 骨架)
pub fn evaluate_constant_expression(
  _expr: ConstantExpression,
  _expected: crate::value_type::ValueType,
) -> ValueOrError {
  unimplemented!("骨架: 参照原版 constant-expression.cc EvaluateConstantExpression 修订")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(size_of::<ConstantExpression>() <= 8);
  assert!(ConstantExpression::new().kind() == Kind::Empty);
  assert!(!ConstantExpression::new().is_set());
  assert!(ConstantExpression::i32_const(-1).kind() == Kind::I32Const);
  assert!(ConstantExpression::i32_const(-1).i32_value() == -1);
  assert!(ConstantExpression::ref_func(7).kind() == Kind::RefFunc);
  assert!(ConstantExpression::ref_func(7).index() == 7);
  assert!(ConstantExpression::wire_bytes(0x1234, 0x5678).kind() == Kind::WireBytesRef);
  assert!(ConstantExpression::wire_bytes(0x1234, 0x5678).wire_bytes_ref() == (0x1234, 0x5678));
  assert!((1u64 << KIND_SHIFT) > (1u64 << 32));
};
`;
;

const p = writeSrc("v8-wasm-core", "constant_expression.rs", rust);
console.log(`written ${p}`);
