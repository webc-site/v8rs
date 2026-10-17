#!/usr/bin/env bun
// tool/src/objects/bigint.h.js — 覆盖 v8/src/objects/bigint.h + bigint.tq
// BigIntBase 常量表 / 位域 (SignBits/PaddingBits/LengthBits) / digit 几何常量
// 平台分支: kSystemPointerSize 按 x64+arm64 (=8, 64 位) 展开
// → v8-objects-rest/src/bigint.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/bigint.h");

// 1. 原始常量
const maxBitsBits = Number(hSrc.match(/static const uint32_t kMaxBitsBits = (\d+);/)?.[1]);
if (maxBitsBits !== 30) throw new Error(`kMaxBitsBits 不符: ${maxBitsBits}`);

// 平台分支展开: 64 位目标 (x64/arm64)
const SYSTEM_POINTER_SIZE = 8;
const SYSTEM_POINTER_SIZE_LOG2 = 3;
const BITS_PER_BYTE = 8;
const BITS_PER_BYTE_LOG2 = 3;

// kMaxLength = ((1 << kMaxBitsBits) - 1) / (kSystemPointerSize * kBitsPerByte)
const maxLength = Math.floor(((1 << maxBitsBits) - 1) / (SYSTEM_POINTER_SIZE * BITS_PER_BYTE));
// kMaxBits = kMaxLength * kSystemPointerSize * kBitsPerByte
const maxBits = maxLength * SYSTEM_POINTER_SIZE * BITS_PER_BYTE;
// kLengthFieldBits = kMaxBitsBits - kSystemPointerSizeLog2 - kBitsPerByteLog2
const lengthFieldBits = maxBitsBits - SYSTEM_POINTER_SIZE_LOG2 - BITS_PER_BYTE_LOG2;
// kPaddingBits = 32 - 1 /* sign */ - kLengthFieldBits
const paddingBits = 32 - 1 - lengthFieldBits;
if (maxLength !== 16777215) throw new Error(`kMaxLength 对数不符: ${maxLength}, 期望 16777215`);
if (lengthFieldBits !== 24) throw new Error(`kLengthFieldBits 对数不符: ${lengthFieldBits}`);
if (paddingBits !== 7) throw new Error(`kPaddingBits 对数不符: ${paddingBits}`);

// 2. digit 几何 (digit_t = uintptr_t → 8 字节)
const digitBits = SYSTEM_POINTER_SIZE * BITS_PER_BYTE;
const halfDigitBits = digitBits / 2;
const halfDigitMask = (1n << BigInt(halfDigitBits)) - 1n;
if (digitBits !== 64) throw new Error(`kDigitBits 对数不符: ${digitBits}`);

// 3. 位域布局: SignBits(0,1) → PaddingBits(1,kPaddingBits) → LengthBits(8,kLengthFieldBits)
const signOff = 0, signSize = 1;
const padOff = signOff + signSize, padSize = paddingBits;
const lenOff = padOff + padSize, lenSize = lengthFieldBits;
if (lenOff + lenSize !== 32) throw new Error("BigIntBase 位域总宽 != 32");

// 序列化布局: LengthBitsForSerialization = SignBits::Next<uint32_t, 31>
const serLenOff = 1, serLenSize = 31;

console.log(`BigInt kMaxLength=${maxLength}, 位域 sign(0,1) pad(${padOff},${padSize}) len(${lenOff},${lenSize})`);

let rust = banner(
  "src/objects/bigint.h.js",
  "src/objects/bigint.{h,tq}",
  "BigInt 几何常量与位域布局 (64 位目标 x64/arm64 展开)",
);

rust += `
use crate::error::Error;

/// BigInt 基类常量命名空间 (原版 BigIntBase 静态常量)
pub struct BigIntBase;

#[allow(dead_code)]
impl BigIntBase {
  /// 长度字段位宽上界 (原版 kMaxBitsBits)
  pub const MAX_BITS_BITS: u32 = ${maxBitsBits};

  /// 最大 digit 长度 (原版 kMaxLength = ((1<<30)-1)/(8*8))
  pub const MAX_LENGTH: u32 = ${maxLength};

  /// 最大位长, 约 10 亿 (原版 kMaxBits = kMaxLength*8*8)
  pub const MAX_BITS: u32 = ${maxBits};

  /// 头内长度字段位宽 (原版 kLengthFieldBits = 30-3-3)
  pub const LENGTH_FIELD_BITS: u32 = ${lengthFieldBits};

  /// 头内填充位宽 (原版 kPaddingBits = 32-1-24)
  pub const PADDING_BITS: u32 = ${paddingBits};

  /// 单 digit 字节数 (原版 kDigitSize = sizeof(uintptr_t), 64 位目标)
  pub const DIGIT_SIZE: u32 = ${SYSTEM_POINTER_SIZE};

  /// 单 digit 位数 (原版 kDigitBits)
  pub const DIGIT_BITS: u32 = ${digitBits};

  /// 半 digit 位数 (原版 kHalfDigitBits)
  pub const HALF_DIGIT_BITS: u32 = ${halfDigitBits};

  /// 半 digit 掩码 (原版 kHalfDigitMask)
  pub const HALF_DIGIT_MASK: u64 = 0x${halfDigitMask.toString(16)}_u64;

  /// digit 类型别名 (原版 using digit_t = uintptr_t)
  pub type Digit = u64;

  /// 位域: 符号位 (原版 SignBits = BitField<bool, 0, 1>)
  pub const SIGN_SHIFT: u32 = ${signOff};
  pub const SIGN_MASK: u32 = 0x${(((1 << signSize) - 1) << signOff).toString(16)};

  /// 位域: 填充 (原版 PaddingBits)
  pub const PADDING_SHIFT: u32 = ${padOff};
  pub const PADDING_MASK: u32 = 0x${(((1 << padSize) - 1) << padOff).toString(16)};

  /// 位域: 长度 (原版 LengthBits)
  pub const LENGTH_SHIFT: u32 = ${lenOff};
  pub const LENGTH_MASK: u32 = 0x${((((1 << lenSize) - 1) << lenOff) >>> 0).toString(16)};

  /// 序列化专用长度域 (原版 LengthBitsForSerialization = SignBits::Next<uint32_t, 31>)
  pub const LENGTH_SERIALIZATION_SHIFT: u32 = ${serLenOff};
  pub const LENGTH_SERIALIZATION_MASK: u32 = 0x${((((1 << serLenSize) - 1) << serLenOff) >>> 0).toString(16)};

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
#[allow(dead_code)]
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
`;

const p = writeSrc("v8-objects-rest", "bigint.rs", rust);
console.log(`written ${p}`);
