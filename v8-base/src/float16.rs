//! 原版 v8/src/base/float16.h + third_party/fp16 的 IEEE half 转换
//! 转换语义: round-to-nearest-even, 与 fp16_ieee_*_value 等价。

use crate::memory::{Address, read_unaligned_value, write_unaligned_value};

/// Float16 — 16 位 IEEE half, 与原版 sizeof == 2 对齐
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Float16 {
  bits: u16,
}

impl Float16 {
  #[must_use]
  pub const fn from_bits(bits: u16) -> Self {
    Self { bits }
  }

  #[must_use]
  pub const fn get_bits(self) -> u16 {
    self.bits
  }

  /// Float16::Read(Address)
  ///
  /// # Safety
  /// 地址起 2 字节必须可读。
  #[must_use]
  pub unsafe fn read(source: Address) -> Self {
    // SAFETY: 契约保证 2 字节可读
    Self::from_bits(unsafe { read_unaligned_value::<u16>(source) })
  }

  /// Float16::Write(Address)
  ///
  /// # Safety
  /// 地址起 2 字节必须可写。
  pub unsafe fn write(self, destination: Address) {
    // SAFETY: 契约保证 2 字节可写
    unsafe { write_unaligned_value(destination, self.bits) };
  }

  /// fp16_ieee_from_fp32_value — RNE 舍入
  #[must_use]
  pub fn from_float32(f32_value: f32) -> Self {
    let bits = f32_value.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let abs = bits & 0x7FFF_FFFF;
    let exp = (abs >> 23) & 0xFF;
    let mantissa = abs & 0x7F_FFFF;

    if exp == 0xFF {
      // inf / NaN
      if mantissa == 0 {
        return Self::from_bits(sign | 0x7C00);
      }
      // NaN: 保持 NaN 并静默化, 保留高 9 位 payload
      let payload = ((mantissa >> 13) & 0x3FF) as u16;
      let quiet = if payload == 0 { 0x200 } else { payload };
      return Self::from_bits(sign | 0x7C00 | quiet);
    }

    let new_exp = exp as i32 - 127 + 15;
    if new_exp >= 31 {
      // 溢出 → inf (RNE 下 |v| >= 65520+2^? 阈值由整段比较覆盖)
      return Self::from_bits(sign | 0x7C00);
    }
    if new_exp > 0 {
      // 规格化: 保留 10 位尾数, 第 11 位起为舍入位
      let mut m = (mantissa >> 13) as u16;
      let rem = mantissa & 0x1FFF;
      let half = 0x1000;
      if rem > half || (rem == half && (m & 1) == 1) {
        m += 1;
        if m == 0x400 {
          // 尾数进位 → 指数加一
          let ne = new_exp + 1;
          if ne >= 31 {
            return Self::from_bits(sign | 0x7C00);
          }
          return Self::from_bits(sign | ((ne as u16) << 10));
        }
      }
      return Self::from_bits(sign | ((new_exp as u16) << 10) | m);
    }
    // 非规格化 (含 0): 以 2^-24 为单位舍入
    if new_exp < -10 {
      return Self::from_bits(sign);
    }
    let sig = mantissa | 0x80_0000; // 24 位, 含隐藏位
    let shift = 14 - new_exp; // 1..24
    let mut q = sig >> shift;
    let r = sig & ((1u32 << shift) - 1);
    let half = 1u32 << (shift - 1);
    if r > half || (r == half && (q & 1) == 1) {
      q += 1;
    }
    Self::from_bits(sign | q as u16)
  }

  /// fp16_ieee_to_fp32_value
  #[must_use]
  pub fn to_float32(self) -> f32 {
    let bits = self.bits;
    // 符号位: half 的 bit15 映射到 float32 的 bit31 (掩码后整体左移 16)
    let sign = u32::from(bits & 0x8000) << 16;
    let exp = (bits >> 10) & 0x1F;
    let mantissa = u32::from(bits & 0x3FF);
    let f = if exp == 0 {
      if mantissa == 0 {
        sign
      } else {
        // 非规格化 → 规格化
        let mut e = -14i32;
        let mut m = mantissa;
        while m & 0x400 == 0 {
          m <<= 1;
          e -= 1;
        }
        let m = m & 0x3FF;
        sign | (((e + 127) as u32) << 23) | (m << 13)
      }
    } else if exp == 31 {
      sign | 0x7F80_0000 | (mantissa << 13)
    } else {
      sign | (u32::from(exp) + 112) << 23 | (mantissa << 13)
    };
    f32::from_bits(f)
  }
}
