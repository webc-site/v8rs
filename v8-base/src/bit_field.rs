//! v8/src/base/bit-field.h 的 1:1 转写
//! 原版模板 BitField<T, shift, size, U> → Rust 泛型 + const 泛型参数
//! bitfield_detail::to_value (strong_alias 解包) → BitFieldCodec trait
//! DEFINE_BIT_FIELDS 系列 X-macro → define_bit_fields! / define_bit_fields_64! 宏机械展开

use std::{fmt, marker::PhantomData, mem::size_of};

// ---------------------------------------------------------------------------
// 基础类型约束
// ---------------------------------------------------------------------------

/// BitField 的存储基底类型 (原版 uint8_t/16/32/64 的 static_assert(is_unsigned_v<U>))
pub trait BitFieldBase: Copy + PartialEq + Eq + fmt::Debug {
  const BITS: u32;
  fn from_u64(v: u64) -> Self;
  fn to_u64(self) -> u64;
}

impl BitFieldBase for u8 {
  const BITS: u32 = 8;
  #[inline]
  fn from_u64(v: u64) -> Self {
    v as u8
  }
  #[inline]
  fn to_u64(self) -> u64 {
    self as u64
  }
}
impl BitFieldBase for u16 {
  const BITS: u32 = 16;
  #[inline]
  fn from_u64(v: u64) -> Self {
    v as u16
  }
  #[inline]
  fn to_u64(self) -> u64 {
    self as u64
  }
}
impl BitFieldBase for u32 {
  const BITS: u32 = 32;
  #[inline]
  fn from_u64(v: u64) -> Self {
    v as u32
  }
  #[inline]
  fn to_u64(self) -> u64 {
    self as u64
  }
}
impl BitFieldBase for u64 {
  const BITS: u32 = 64;
  #[inline]
  fn from_u64(v: u64) -> Self {
    v
  }
  #[inline]
  fn to_u64(self) -> u64 {
    self
  }
}

/// 可存入 BitField 的字段类型: 对应原版 static_cast 编解码路径
/// (整数/bool/枚举)。枚举实现本 trait 可用 strum 或手写。
pub trait BitFieldCodec: Copy {
  fn to_u64(self) -> u64;
  fn from_u64(v: u64) -> Self;
  /// 可信类型标记: 原版 `std::is_enum_v<T> || std::is_same_v<T, bool>` 分支
  /// (bit-field.h:78) — 可信类型 encode 仅 DCHECK, 整数走 CHECK (release 亦执行)。
  /// 默认 false 即整数语义; 枚举实现应覆写为 true。
  const TRUSTED_TYPE: bool = false;
}

macro_rules! impl_codec_ints {
  ($($t:ty),*) => {
    $(
      impl BitFieldCodec for $t {
        #[inline]
        fn to_u64(self) -> u64 { self as u64 }
        #[inline]
        fn from_u64(v: u64) -> Self { v as $t }
      }
    )*
  };
}

impl_codec_ints!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl BitFieldCodec for bool {
  /// bool 为原版可信类型 (is_same_v<T, bool>)
  const TRUSTED_TYPE: bool = true;
  #[inline]
  fn to_u64(self) -> u64 {
    self as u64
  }
  #[inline]
  fn from_u64(v: u64) -> Self {
    v != 0
  }
}

// ---------------------------------------------------------------------------
// BitField
// ---------------------------------------------------------------------------

/// 位域编解码器 (原版 BitField<T, shift, size, U>; U 默认 uint32_t 以 BitField32 别名表达)
/// C++ 类无状态, 全部为静态方法 → Rust 关联函数
pub struct BitField<T, U, const SHIFT: u8, const SIZE: u8> {
  _marker: PhantomData<(T, U)>,
}

impl<T: BitFieldCodec, U: BitFieldBase, const SHIFT: u8, const SIZE: u8>
  BitField<T, U, SHIFT, SIZE>
{
  // static_assert 组 → 关联 const 断言 (实例化即校验)
  const ASSERTS: () = {
    assert!((SHIFT as usize) < size_of::<U>() * 8, "shift 越界为 UB");
    assert!((SIZE as usize) < size_of::<U>() * 8, "size 越界为 UB");
    assert!((SHIFT as usize + SIZE as usize) <= size_of::<U>() * 8);
    assert!(SIZE > 0);
  };

  pub const K_SHIFT: u32 = SHIFT as u32;
  pub const K_SIZE: u32 = SIZE as u32;
  pub const K_LAST_USED_BIT: u32 = SHIFT as u32 + SIZE as u32 - 1;

  #[inline]
  fn mask_u64() -> u64 {
    let () = Self::ASSERTS;
    // ((U{1} << kShift) << kSize) - (U{1} << kShift), 以 u64 环绕计算后截断
    let high = (1u64 << Self::K_SHIFT) << Self::K_SIZE;
    let low = 1u64 << Self::K_SHIFT;
    high.wrapping_sub(low)
  }

  /// kMask: 位域在 U 上的掩码
  #[inline]
  pub fn k_mask() -> U {
    U::from_u64(Self::mask_u64())
  }

  /// kNumValues = 2^size
  #[inline]
  pub fn k_num_values() -> U {
    U::from_u64(1u64 << Self::K_SIZE)
  }

  /// kMax = kNumValues - 1
  #[inline]
  pub fn k_max() -> U {
    U::from_u64((1u64 << Self::K_SIZE) - 1)
  }

  /// is_valid: 值是否可放入位域
  #[inline]
  pub fn is_valid(value: T) -> bool {
    (value.to_u64() & !Self::k_max().to_u64()) == 0
  }

  /// encode: 编码到 U
  /// 原版 bit-field.h:78-100 三分支: 枚举/bool/位宽恰好装满 → DCHECK;
  /// 其余整数 → CHECK (release 亦执行, 不静默截断污染相邻位域)
  #[inline]
  pub fn encode(value: T) -> U {
    if T::TRUSTED_TYPE || size_of::<T>() * 8 <= SIZE as usize {
      debug_assert!(Self::is_valid(value));
    } else {
      assert!(Self::is_valid(value), "位域 encode 溢出: 值超出 kMax");
    }
    U::from_u64(value.to_u64() << Self::K_SHIFT)
  }

  /// update: 更新已有 U 值中的位域
  #[inline]
  pub fn update(previous: U, value: T) -> U {
    U::from_u64((previous.to_u64() & !Self::k_mask().to_u64()) | Self::encode(value).to_u64())
  }

  /// decode: 从 U 提取位域
  #[inline]
  pub fn decode(value: U) -> T {
    T::from_u64((value.to_u64() & Self::k_mask().to_u64()) >> Self::K_SHIFT)
  }
}

/// 原版默认 U=uint32_t 的惯用别名
pub type BitField32<T, const SHIFT: u8, const SIZE: u8> = BitField<T, u32, SHIFT, SIZE>;

/// BitField8<T, shift, size> = BitField<T, shift, size, uint8_t>
pub type BitField8<T, const SHIFT: u8, const SIZE: u8> = BitField<T, u8, SHIFT, SIZE>;

/// BitField16
pub type BitField16<T, const SHIFT: u8, const SIZE: u8> = BitField<T, u16, SHIFT, SIZE>;

/// BitField64
pub type BitField64<T, const SHIFT: u8, const SIZE: u8> = BitField<T, u64, SHIFT, SIZE>;

// ---------------------------------------------------------------------------
// BitFieldUnion
// ---------------------------------------------------------------------------

/// 两个不相交线性位域的联合 (仅静态掩码)
pub struct BitFieldUnion<
  A: BitFieldSpec,
  B: BitFieldSpec,
  const A_SHIFT: u8,
  const A_SIZE: u8,
  const B_SHIFT: u8,
  const B_SIZE: u8,
> {
  _marker: PhantomData<(A, B)>,
}

/// 供 BitFieldUnion 使用的位域规格 trait (读 kShift/kSize/kMask 的抽象)
pub trait BitFieldSpec {
  fn shift() -> u32;
  fn size() -> u32;
  fn mask() -> u64;
}

impl<T: BitFieldCodec, U: BitFieldBase, const SHIFT: u8, const SIZE: u8> BitFieldSpec
  for BitField<T, U, SHIFT, SIZE>
{
  #[inline]
  fn shift() -> u32 {
    Self::K_SHIFT
  }
  #[inline]
  fn size() -> u32 {
    Self::K_SIZE
  }
  #[inline]
  fn mask() -> u64 {
    Self::k_mask().to_u64()
  }
}

impl<
  A: BitFieldSpec,
  B: BitFieldSpec,
  const A_SHIFT: u8,
  const A_SIZE: u8,
  const B_SHIFT: u8,
  const B_SIZE: u8,
> BitFieldUnion<A, B, A_SHIFT, A_SIZE, B_SHIFT, B_SIZE>
{
  /// static_assert((A::kMask & B::kMask) == 0) 的运行时校验入口
  #[inline]
  pub fn disjoint() -> bool {
    (A::mask() & B::mask()) == 0
  }
  #[inline]
  pub fn k_shift() -> u32 {
    A::shift().min(B::shift())
  }
  #[inline]
  pub fn k_mask() -> u64 {
    A::mask() | B::mask()
  }
  #[inline]
  pub fn k_size() -> u32 {
    A::size() + B::size() + (A::shift().max(B::shift()) - Self::k_shift())
  }
}

// ---------------------------------------------------------------------------
// BitSetComputer
// ---------------------------------------------------------------------------

/// 可变条目数信息的数组编解码 (原版 BitSetComputer<T, kBitsPerItem, kBitsPerWord, U>)
pub struct BitSetComputer<T, U, const BITS_PER_ITEM: u8, const BITS_PER_WORD: u8> {
  _marker: PhantomData<(T, U)>,
}

impl<T: BitFieldCodec, U: BitFieldBase, const BITS_PER_ITEM: u8, const BITS_PER_WORD: u8>
  BitSetComputer<T, U, BITS_PER_ITEM, BITS_PER_WORD>
{
  pub const K_ITEMS_PER_WORD: usize = (BITS_PER_WORD / BITS_PER_ITEM) as usize;
  pub const K_MASK: u64 = (1u64 << BITS_PER_ITEM) - 1;

  /// 容纳 items 条信息需要的数组元素数
  #[inline]
  pub fn word_count(items: usize) -> usize {
    if items == 0 {
      return 0;
    }
    (items - 1) / Self::K_ITEMS_PER_WORD + 1
  }

  /// item 对应的数组下标
  #[inline]
  pub fn index(base_index: usize, item: usize) -> usize {
    base_index + item / Self::K_ITEMS_PER_WORD
  }

  /// 提取 data 中 item 的信息
  #[inline]
  pub fn decode(data: U, item: usize) -> T {
    T::from_u64((data.to_u64() >> Self::shift(item)) & Self::K_MASK)
  }

  /// 将 value 编码进 previous 的 item 位置
  #[inline]
  pub fn encode(previous: U, item: usize, value: T) -> U {
    let shift_value = Self::shift(item);
    let set_bits = value.to_u64() << shift_value;
    U::from_u64((previous.to_u64() & !(Self::K_MASK << shift_value)) | set_bits)
  }

  #[inline]
  pub fn shift(item: usize) -> u32 {
    (item % Self::K_ITEMS_PER_WORD) as u32 * BITS_PER_ITEM as u32
  }
}

// ---------------------------------------------------------------------------
// DEFINE_BIT_FIELDS / DEFINE_BIT_FIELDS_64 X-macro 机械展开
// ---------------------------------------------------------------------------

/// 内部递归展开器: 逐个字段生成类型别名并累加 shift
/// ($root 为 BitField; size 64 版通过 $u=u64 实现, 原版 DEFINE_BIT_FIELD_64_TYPE
/// 使用 BitField64 别名, 语义等价)
#[doc(hidden)]
#[macro_export]
macro_rules! __define_bit_fields_seq {
  (@done $u:ty, $root:ident, $count:ident, @acc [$($acc:tt)*]) => {
    /// 位域总位数 (原版 Ranges::kBitsCount)
    pub const $count: u32 = 0 $($acc)*;
  };
  (@step $u:ty, $root:ident, $count:ident, @acc [$($acc:tt)*],
   $name:ident : $t:ty , $size:literal ; $($rest:tt)*) => {

    pub type $name = $crate::bit_field::$root<$t, $u, { 0 $($acc)* }, $size>;
    $crate::__define_bit_fields_seq!(
      @step $u, $root, $count, @acc [$($acc)* + $size], $($rest)*);
  };
  (@step $u:ty, $root:ident, $count:ident, @acc [$($acc:tt)*] $(,)?) => {
    $crate::__define_bit_fields_seq!(@done $u, $root, $count, @acc [$($acc)*]);
  };
}

/// 原版 DEFINE_BIT_FIELDS(LIST_MACRO): 连续位域序列定义
/// 用法: `define_bit_fields!(u32, K_BITS_COUNT, { Name: Type, Size; ... });`
/// 生成 `Name = BitField<Type, u32, {累加shift}, Size>` 别名与总位数常量
/// 原版 k##Name##Start/End 枚举需标识符拼接 (paste! 宏), 本轮未引入依赖,
/// 上层可从别名的 K_SHIFT/K_LAST_USED_BIT 读取 (语义等价)
#[macro_export]
macro_rules! define_bit_fields {
  ($u:ty, $count:ident, { $($name:ident : $t:ty , $size:literal);* $(;)? }) => {
    $crate::__define_bit_fields_seq!(@step $u, BitField, $count, @acc [],
      $($name : $t , $size ;)*);
  };
}

/// 原版 DEFINE_BIT_FIELDS_64(LIST_MACRO): 64 位存储版本
#[macro_export]
macro_rules! define_bit_fields_64 {
  ($count:ident, { $($name:ident : $t:ty , $size:literal);* $(;)? }) => {
    $crate::__define_bit_fields_seq!(@step u64, BitField, $count, @acc [],
      $($name : $t , $size ;)*);
  };
}

#[cfg(test)]
mod examples {

  pub type MutableMode = u32;

  // 原版注释中的 MAP_BIT_FIELD1 展开示例:
  //   V(IsAbcBit, bool, 1) V(IsBcdBit, bool, 1) V(CdeBits, int, 5) V(DefBits, MutableMode, 1)
  define_bit_fields!(u32, K_BITS_COUNT, {
    IsAbcBit: bool, 1;
    IsBcdBit: bool, 1;
    CdeBits: u32, 5;
    DefBits: MutableMode, 1;
  });

  #[test]
  fn generated_aliases() {
    // shift 顺序累加: 0,1,2..7,7
    assert_eq!(IsAbcBit::k_mask(), 0b1);
    assert_eq!(IsBcdBit::k_mask(), 0b10);
    assert_eq!(CdeBits::k_mask(), 0b111_1100);
    assert_eq!(DefBits::k_mask(), 0b1000_0000);
    assert_eq!(IsAbcBit::K_LAST_USED_BIT, 0);
    assert_eq!(K_BITS_COUNT, 8);
    let v = CdeBits::encode(3);
    assert_eq!(CdeBits::decode(v), 3);
  }
}
