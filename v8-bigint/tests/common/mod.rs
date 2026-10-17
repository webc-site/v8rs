// 由手工从 v8/test/bigint/bigint-shell.cc 的辅助部分转写; 对应原版逐行
//! 测试公共件: 确定性 RNG, 随机 BigInt 生成, 十六进制格式化

use v8_bigint::{
  bigint::{DefaultPlatform, Digit, K_DIGIT_BITS, bit_length, count_leading_zeros, norm},
  bigint_internal::Processor,
};

/// xorshift128 + MurmurHash3 种子 (与原版 RNG 一致, 保证采样可复现)
pub struct Rng {
  state0: u64,
  state1: u64,
}

impl Rng {
  pub fn new(seed: i64) -> Self {
    let s0 = murmur_hash3(seed as u64);
    let s1 = murmur_hash3(!s0);
    assert!(s0 != 0 || s1 != 0);
    Self {
      state0: s0,
      state1: s1,
    }
  }

  pub fn next_u64(&mut self) -> u64 {
    xorshift128(&mut self.state0, &mut self.state1);
    self.state0.wrapping_add(self.state1)
  }
}

fn xorshift128(s0: &mut u64, s1: &mut u64) {
  let mut t1 = *s0;
  let s0v = *s1;
  *s0 = s0v;
  t1 ^= t1 << 23;
  t1 ^= t1 >> 17;
  t1 ^= s0v;
  t1 ^= s0v >> 26;
  *s1 = t1;
}

fn murmur_hash3(mut h: u64) -> u64 {
  h ^= h >> 33;
  h = h.wrapping_mul(0xFF51AFD7ED558CCD);
  h ^= h >> 33;
  h = h.wrapping_mul(0xC4CEB9FE1A85EC53);
  h ^= h >> 33;
  h
}

/// 每位独立随机的 digit 填充 (MSD 不为零)
pub fn generate_random_bits(rng: &mut Rng, z: &mut [Digit]) {
  for d in z.iter_mut() {
    *d = rng.next_u64();
  }
  // MSD 不允许为零
  while z[z.len() - 1] == 0 {
    z[z.len() - 1] = rng.next_u64();
  }
}

/// 易暴露 bug 的模式混合: 全随机 / 2 的幂 / 全 1 / 0-1 交替 (原版 4 种 mode)
pub fn generate_random(rng: &mut Rng, len: usize) -> Vec<Digit> {
  let mut z = vec![0; len];
  if len == 0 {
    return z;
  }
  let mode = (rng.next_u64() & 3) as u8;
  if mode == 0 {
    generate_random_bits(rng, &mut z);
    return z;
  }
  if mode == 1 {
    // 2 的幂, 孤立 1 位在 MSD
    let bit_in_msd = (rng.next_u64() % K_DIGIT_BITS as u64) as u32;
    z[len - 1] = 1 << bit_in_msd;
    return z;
  }
  // mode 2/3: MSD 低位连续 1
  let bits_in_msd = (rng.next_u64() % K_DIGIT_BITS as u64) as u32;
  let mut msd = (1u64 << bits_in_msd) - 1;
  if msd == 0 {
    msd = Digit::MAX;
  }
  z[len - 1] = msd;
  if mode == 2 {
    // 非 MSD 位全 1
    z[..len - 1].fill(Digit::MAX);
  } else {
    // mode 3: 每 digit 全 0 或全 1
    let mut random = 0u64;
    let mut random_bits = 0;
    for d in z[..len - 1].iter_mut() {
      if random_bits == 0 {
        random = rng.next_u64();
        random_bits = 64;
      }
      *d = if random & 1 != 0 { Digit::MAX } else { 0 };
      random >>= 1;
      random_bits -= 1;
    }
  }
  z
}

/// 随机 radix 进制数字串 (拒绝采样)
pub fn generate_random_string(rng: &mut Rng, len: usize, radix: u32) -> Vec<u8> {
  const CONVERSION_CHARS: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
  let mut out = Vec::with_capacity(len);
  let char_bits = bit_length(radix - 1);
  let char_mask = (1u64 << char_bits) - 1;
  let mut random = 0u64;
  let mut available_bits = 0i32;
  while out.len() < len {
    loop {
      if available_bits < char_bits as i32 {
        random = rng.next_u64();
        available_bits = 64;
      }
      let next_char = (random & char_mask) as u32;
      random >>= char_bits;
      available_bits -= char_bits as i32;
      if next_char < radix {
        out.push(CONVERSION_CHARS[next_char as usize]);
        break;
      }
    }
  }
  out
}

/// 十六进制格式化 (超长截断显示)
pub fn format_hex(x: &[Digit]) -> String {
  const CONVERSION_CHARS: &[u8; 16] = b"0123456789abcdef";
  let x = norm(x);
  if x.is_empty() {
    return "0".to_string();
  }
  let mut msd = x[x.len() - 1];
  let msd_leading_zeros = count_leading_zeros(msd);
  let bit_length_val = x.len() as u64 * K_DIGIT_BITS as u64 - msd_leading_zeros as u64;
  let chars = bit_length_val.div_ceil(4);
  if chars > 100000 {
    return format!("<BigInt with {bit_length_val} bits>");
  }
  let mut result = vec![b'?'; chars as usize];
  let mut pos = result.len() - 1;
  for &d in &x[..x.len() - 1] {
    let mut dd = d;
    for _ in 0..K_DIGIT_BITS / 4 {
      result[pos] = CONVERSION_CHARS[(dd & 15) as usize];
      pos -= 1;
      dd >>= 4;
    }
  }
  while msd != 0 {
    result[pos] = CONVERSION_CHARS[(msd & 15) as usize];
    msd >>= 4;
    if pos == 0 {
      debug_assert!(msd == 0);
      break;
    }
    pos -= 1;
  }
  String::from_utf8(result).unwrap_or_default()
}

/// 构造默认处理器
pub fn processor() -> Processor<DefaultPlatform> {
  Processor::new(DefaultPlatform)
}
