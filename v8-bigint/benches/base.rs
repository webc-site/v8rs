//! v8-bigint 自包含评测公共模块: 定义核心评测 trait `Base`、样本生成与各实现库 impl
//! 评测 id 基于 `Base::NAME` 生成; v8rs 与对比库各写一个 impl (对比库由 feature `bench-<库名>` 开关)
//! 被 `main.rs` 以 `#[path]` 引入, 不作为独立 bench 目标 (Cargo.toml `autobenches = false`)
//!
//! 指标设计 (覆盖算法切换边界, digit = 64 bit):
//! - add/sub: O(n) 逐 digit 加减
//! - mul: 位宽覆盖 schoolbook(<34 digit) / Karatsuba(34..210) / Toom-Cook(210..720) / FFT(>=720)
//!   阈值见 v8_bigint::bigint::{K_KARATSUBA_THRESHOLD, K_TOOM_THRESHOLD, K_FFT_THRESHOLD}
//! - div/rem: 覆盖 schoolbook(<57 digit) / Burnikel-Ziegler(57..13000) 边界;
//!   Barrett(>=13000 digit ≈ 832k bit) 超出 64k bit 样本规模, 本次不评测 (已在报告说明)
//! - to_string/from_string: radix 10 (通用快速路径) 与 16 (2 幂 shortcut)
//! - malachite-nz 编译过重不引入, 对比库仅 num-bigint
//!

use fastrand::Rng;
use num_bigint::{BigInt, Sign};
use v8_bigint::{
  bigint::{
    Digit, K_DIGIT_BITS, K_KARATSUBA_THRESHOLD, Platform, Status, add, add_result_length, compare,
    divide_result_length, multiply_result_length, norm, subtract, subtract_result_length,
  },
  bigint_internal::Processor,
  from_string::FromStringAccumulator,
  mul_helpers::multiply_schoolbook,
  to_string::to_string_result_length,
};

/// 核心评测操作: 所有参与评测的实现都实现本 trait
/// from_* 借 mutable self 复用内部 scratch, 是评测框架约定, 非构造语义
pub trait Base {
  /// 评测 id 前缀
  const NAME: &'static str;
  /// 本库内部的大整数表示
  type Num: Clone;

  fn new() -> Self;
  /// digit 样本 → 本库表示
  fn parse_digits(&mut self, digits: &[Digit]) -> Self::Num;
  fn add(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num;
  /// 非负减法, 实现方自行保证结果非负 (取绝对差)
  fn sub(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num;
  fn mul(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num;
  /// 商 (要求 a > b)
  fn div(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num;
  /// 余数 (要求 a > b)
  fn rem(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num;
  fn to_string(&mut self, a: &Self::Num, radix: u32) -> Vec<u8>;
  fn parse_string(&mut self, s: &[u8], radix: u32) -> Self::Num;
}

/// 评测位宽: 64/256/1k/4k/16k/64k bit, 即 1/4/16/64/256/1024 digit
/// 恰好夹住 Karatsuba(34)/Toom(210)/FFT(720) 与 Burnikel(57) 的切换边界
pub const K_BIT_SIZES: [u32; 6] = [64, 256, 1024, 4096, 16384, 65536];
/// 进制样本: 10 走通用路径, 16 走 2 幂 shortcut
pub const K_RADIXES: [u32; 2] = [10, 16];

/// 固定种子基值, 保证全体系确定性可复现
const K_SEED: u64 = 2026_1011;

/// digit 序列 → 小端字节序列 (num-bigint 互转的公共件)
pub fn digits_to_le_bytes(digits: &[Digit]) -> Vec<u8> {
  digits
    .iter()
    .copied()
    .flat_map(Digit::to_le_bytes)
    .collect()
}

/// 确定性伪随机 digit 样本 (fastrand 固定 seed), msd 置高位保证已规格化 (无前导零 digit)
pub fn sample(bits: u32, salt: u64) -> Vec<Digit> {
  let mut rng = Rng::with_seed(K_SEED ^ (salt << 32) ^ bits as u64);
  let mut digits: Vec<Digit> = (0..(bits / K_DIGIT_BITS) as usize)
    .map(|_| rng.u64(..))
    .collect();
  let top = digits.len() - 1;
  digits[top] |= 1 << (K_DIGIT_BITS - 1);
  digits
}

/// 用 num-bigint (dev 依赖, 仅作样本工具) 生成各实现共享的规范进制字符串
pub fn radix_string(digits: &[Digit], radix: u32) -> Vec<u8> {
  BigInt::from_bytes_le(Sign::Plus, &digits_to_le_bytes(digits))
    .to_str_radix(radix)
    .into_bytes()
}

// ---------------------------------------------------------------------------
// v8rs 实现
// ---------------------------------------------------------------------------

/// v8rs 默认平台: 永不中断
#[derive(Clone, Copy, Default)]
struct NeverPlatform;

impl Platform for NeverPlatform {}

/// v8rs 实现: 数值即规格化 digit 序列 (非负)
pub struct V8 {
  proc: Processor<NeverPlatform>,
}

impl Base for V8 {
  const NAME: &'static str = "v8rs";
  type Num = Vec<Digit>;

  fn new() -> Self {
    Self {
      proc: Processor::new(NeverPlatform),
    }
  }

  fn parse_digits(&mut self, digits: &[Digit]) -> Self::Num {
    norm(digits).to_vec()
  }

  fn add(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num {
    let mut z = vec![0; add_result_length(a.len() as u32, b.len() as u32) as usize];
    add(&mut z, a, b);
    norm(&z).to_vec()
  }

  fn sub(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num {
    let (x, y) = if compare(a, b) >= 0 { (a, b) } else { (b, a) };
    let mut z = vec![0; subtract_result_length(x.len() as u32, y.len() as u32) as usize];
    subtract(&mut z, x, y);
    norm(&z).to_vec()
  }

  fn mul(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num {
    let (x, y) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut z = vec![0; multiply_result_length(x, y) as usize];
    // 复刻内部调度以覆盖 schoolbook 小尺寸段 (公开入口 multiply_large 不含该分支)
    if (y.len() as u32) < K_KARATSUBA_THRESHOLD {
      multiply_schoolbook(&mut z, x, y);
    } else {
      self.proc.multiply_large(&mut z, x, y);
    }
    norm(&z).to_vec()
  }

  fn div(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num {
    let mut q = vec![0; divide_result_length(a, b) as usize];
    let _: Status = self.proc.divide_large(&mut q, a, b);
    norm(&q).to_vec()
  }

  fn rem(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num {
    let mut r = vec![0; b.len()];
    let _: Status = self.proc.modulo_large(&mut r, a, b);
    norm(&r).to_vec()
  }

  fn to_string(&mut self, a: &Self::Num, radix: u32) -> Vec<u8> {
    let mut out = vec![0u8; to_string_result_length(a, radix, false) as usize];
    let _: (Status, u32) = self.proc.write_string(&mut out, a, radix, false);
    // write_string 尾部补零, 截去得到实际字符
    let end = out.iter().position(|&c| c == 0).unwrap_or(out.len());
    out[..end].to_vec()
  }

  fn parse_string(&mut self, s: &[u8], radix: u32) -> Self::Num {
    let mut acc = FromStringAccumulator::new(u32::MAX);
    acc.parse(s, radix as Digit);
    let mut z = vec![0; acc.result_length() as usize];
    self.proc.parse_string(&mut z, &mut acc);
    norm(&z).to_vec()
  }
}

// ---------------------------------------------------------------------------
// num-bigint 对比实现 (feature `bench-num-bigint` 单独开启)
// ---------------------------------------------------------------------------

#[cfg(feature = "bench-num-bigint")]
pub struct NumBigint;

#[cfg(feature = "bench-num-bigint")]
impl Base for NumBigint {
  const NAME: &'static str = "num-bigint";
  type Num = num_bigint::BigInt;

  fn new() -> Self {
    Self
  }

  fn parse_digits(&mut self, digits: &[Digit]) -> Self::Num {
    BigInt::from_bytes_le(Sign::Plus, &digits_to_le_bytes(digits))
  }

  fn add(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num {
    a + b
  }

  fn sub(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num {
    if a >= b { a - b } else { b - a }
  }

  fn mul(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num {
    a * b
  }

  fn div(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num {
    a / b
  }

  fn rem(&mut self, a: &Self::Num, b: &Self::Num) -> Self::Num {
    a % b
  }

  fn to_string(&mut self, a: &Self::Num, radix: u32) -> Vec<u8> {
    a.to_str_radix(radix).into_bytes()
  }

  fn parse_string(&mut self, s: &[u8], radix: u32) -> Self::Num {
    // 样本由 radix_string 自生成, 必可解析; 失败兜底为零而非 panic
    BigInt::parse_bytes(s, radix).unwrap_or(BigInt::ZERO)
  }
}
