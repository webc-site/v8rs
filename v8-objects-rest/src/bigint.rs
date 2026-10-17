// 由 tool/src/objects/bigint.h.js 从 v8/src/objects/bigint.{h,tq} 生成; 修订就地改, 复跑覆盖
// BigInt 几何常量与位域布局 (64 位目标 x64/arm64 展开)

use crate::error::Error;

/// BigInt 基类常量命名空间 (原版 BigIntBase 静态常量)
pub struct BigIntBase;

impl BigIntBase {
  /// 长度字段位宽上界 (原版 kMaxBitsBits)
  pub const MAX_BITS_BITS: u32 = 30;

  /// 最大 digit 长度 (原版 kMaxLength = ((1<<30)-1)/(8*8))
  pub const MAX_LENGTH: u32 = 16777215;

  /// 最大位长, 约 10 亿 (原版 kMaxBits = kMaxLength*8*8)
  pub const MAX_BITS: u32 = 1073741760;

  /// 头内长度字段位宽 (原版 kLengthFieldBits = 30-3-3)
  pub const LENGTH_FIELD_BITS: u32 = 24;

  /// 头内填充位宽 (原版 kPaddingBits = 32-1-24)
  pub const PADDING_BITS: u32 = 7;

  /// 单 digit 字节数 (原版 kDigitSize = sizeof(uintptr_t), 64 位目标)
  pub const DIGIT_SIZE: u32 = 8;

  /// 单 digit 位数 (原版 kDigitBits)
  pub const DIGIT_BITS: u32 = 64;

  /// 半 digit 位数 (原版 kHalfDigitBits)
  pub const HALF_DIGIT_BITS: u32 = 32;

  /// 半 digit 掩码 (原版 kHalfDigitMask)
  pub const HALF_DIGIT_MASK: u64 = 0xffffffff_u64;

  /// digit 类型别名 (原版 using digit_t = uintptr_t)
  pub type Digit = u64;

  /// 位域: 符号位 (原版 SignBits = BitField<bool, 0, 1>)
  pub const SIGN_SHIFT: u32 = 0;
  pub const SIGN_MASK: u32 = 0x1;

  /// 位域: 填充 (原版 PaddingBits)
  pub const PADDING_SHIFT: u32 = 1;
  pub const PADDING_MASK: u32 = 0xfe;

  /// 位域: 长度 (原版 LengthBits)
  pub const LENGTH_SHIFT: u32 = 8;
  pub const LENGTH_MASK: u32 = 0xffffff00;

  /// 序列化专用长度域 (原版 LengthBitsForSerialization = SignBits::Next<uint32_t, 31>)
  pub const LENGTH_SERIALIZATION_SHIFT: u32 = 1;
  pub const LENGTH_SERIALIZATION_MASK: u32 = 0xfffffffe;

  /// 对象实例大小公式: sizeof(BigInt) + length * kDigitSize (原版 AllocatedSize)
  /// header_bytes 由调用方按实际布局传入 (原版为 sizeof(BigInt))
  #[inline]
  #[must_use]
  pub const fn allocated_size(header_bytes: u32, length: u32) -> u32 {
    header_bytes + length * Self::DIGIT_SIZE
  }
}

// —— 签名骨架 (原版 BigInt/FreshlyAllocatedBigInt 公开接口, 供 Developer 补语义) ——
// V8_OBJECT class FreshlyAllocatedBigInt : public BigIntBase (快照反序列化专用分配形态)
// V8_OBJECT class BigInt : public BigIntBase
//   static Tagged<BigInt> Zero(...); One(...); FromBit(...); FromInt(...);
//   static MaybeHandle<BigInt> FromString(...); ToString(...);
//   static uint32_t DigitLength(...); bool ToBoolean(...);
//   class BodyDescriptor;

fn _signatures_placeholder(_: &Error) {}

// —— 提取不变式断言 ——
const _: () = {
  assert!(BigIntBase::MAX_LENGTH == 16777215);
  assert!(BigIntBase::MAX_BITS == 1073741760);
  assert!(BigIntBase::LENGTH_FIELD_BITS == 24);
  assert!(BigIntBase::PADDING_BITS == 7);
  assert!(BigIntBase::LENGTH_SHIFT + BigIntBase::LENGTH_FIELD_BITS == 32);
  assert!(BigIntBase::DIGIT_BITS == 64);
  assert!(BigIntBase::HALF_DIGIT_BITS == 32);
};
