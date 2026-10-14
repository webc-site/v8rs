// 由 tool/src/wasm/constant-expression.h.js 从 v8/src/wasm/constant-expression.h 生成; 修订就地改, 复跑覆盖
// ConstantExpression 64 位位域编码 (Kind 枚举 + 字段布局)

use crate::wasm_value::WasmValue;

/// 常量表达式种类 (对应 enum class Kind; kLastKind = kWireBytesRef)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Kind {
  /// kEmpty
  Empty = 0,
  /// kI32Const
  I32Const = 1,
  /// kRefNull
  RefNull = 2,
  /// kRefFunc
  RefFunc = 3,
  /// kWireBytesRef
  WireBytesRef = 4,
}

impl Kind {
  /// 对应 kLastKind
  pub const LAST_KIND: Self = Self::WireBytesRef;
}

/// 位域宽度 (对应 kValueBits/kOffsetBits/kLengthBits/kKindBits)
pub const VALUE_BITS: i32 = 32;
pub const OFFSET_BITS: i32 = 30;
pub const LENGTH_BITS: i32 = 30;
pub const KIND_BITS: i32 = 3;

/// ConstantExpression: 64 位内联编码。
/// 两种布局: WireBytes 时 offset(0..30)+length(30..60)+kind(60..63);
/// 其余 value(0..32)+kind(60..63)。整体不超过 8 字节。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantExpression {
  bit_field_: u64,
}

impl ConstantExpression {
  /// 布局常量
  pub const LENGTH_SHIFT: i32 = 30;
  pub const KIND_SHIFT: i32 = 60;

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
    Self {
      bit_field_: (index as u64) | ((Kind::RefFunc as u64) << Self::KIND_SHIFT),
    }
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
pub type ValueOrError = Result<WasmValue, ()>;

// 深水推迟 (readme/zh.md 台账): EvaluateConstantExpression 依赖 WasmEngine/isolate 堆面
// (constant-expression.cc), 骨架不入挂载面。

// —— 提取不变式断言 ——
const _: () = {
  assert!(size_of::<ConstantExpression>() <= 8);
  assert!(matches!(ConstantExpression::new().kind(), Kind::Empty));
  assert!(!ConstantExpression::new().is_set());
  assert!(matches!(
    ConstantExpression::i32_const(-1).kind(),
    Kind::I32Const
  ));
  assert!(ConstantExpression::i32_const(-1).i32_value() == -1);
  assert!(matches!(
    ConstantExpression::ref_func(7).kind(),
    Kind::RefFunc
  ));
  assert!(ConstantExpression::ref_func(7).index() == 7);
  assert!(matches!(
    ConstantExpression::wire_bytes(0x1234, 0x5678).kind(),
    Kind::WireBytesRef
  ));
  let wr = ConstantExpression::wire_bytes(0x1234, 0x5678).wire_bytes_ref();
  assert!(wr.0 == 0x1234);
  assert!(wr.1 == 0x5678);
  assert!((1u64 << ConstantExpression::KIND_SHIFT) > (1u64 << 32));
};
