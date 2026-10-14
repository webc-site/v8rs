// 由手工从 v8/src/bigint/mul-fft.cc 转写; 对应原版逐行
//! Schönhage-Strassen FFT 乘法
//! 参考 Christoph Lüders: "Fast Multiplication of Large Integers" (arXiv:1503.04955)
//! 原版 FFTContainer 内 part 间用指针别名做就地 SumDiff, 此处用 split 视图保语义

use std::ptr::eq;

use crate::{
  bigint::{
    Digit, K_DIGIT_BITS, K_FFT_INNER_THRESHOLD, K_LOG2_DIGIT_BITS, Platform, bit_length,
    digit_add2, digit_add3, digit_sub, digit_sub2,
  },
  bigint_internal::Processor,
  util::{count_trailing_zeros, round_up},
  vector_arithmetic::add_and_return_overflow,
};

/// zero_above 的默认值 (视为无限)
const ZERO_ABOVE_INF: usize = usize::MAX;

// ---------------------------------------------------------------------------
// 第 1 部分: mod F_n 算术 (F_n = 2^(K*kDigitBits) + 1, K 由数组长度隐含)
// ---------------------------------------------------------------------------

/// ModFn 辅助: 按 high 的符号加减一个 F_n
fn mod_fn_helper(x: &mut [Digit], high: i64) {
  let len = x.len();
  let k = len - 1;
  if high > 0 {
    let mut borrow = high as Digit;
    x[k] = 0;
    for xi in x[..len].iter_mut() {
      let (v, b) = digit_sub(*xi, borrow);
      *xi = v;
      borrow = b;
      if borrow == 0 {
        break;
      }
    }
  } else {
    let mut carry = (-high) as Digit;
    x[k] = 0;
    for xi in x[..len].iter_mut() {
      let (v, c) = digit_add2(*xi, carry);
      *xi = v;
      carry = c;
      if carry == 0 {
        break;
      }
    }
  }
}

/// x := x mod F_n (x 只是略大于 F_n)
fn mod_fn(x: &mut [Digit]) {
  let k = x.len() - 1;
  let mut high = x[k] as i64;
  if high == 0 {
    return;
  }
  mod_fn_helper(x, high);
  high = x[k] as i64;
  if high == 0 {
    return;
  }
  debug_assert!(high == 1 || high == -1);
  mod_fn_helper(x, high);
  high = x[k] as i64;
  if high == -1 {
    mod_fn_helper(x, high);
  }
}

/// dest := src mod F_n (src 约 2 倍宽)
fn mod_fn_double_width(dest: &mut [Digit], src: &[Digit]) {
  let k = dest.len() - 1;
  let mut borrow = 0;
  for i in 0..k {
    let (v, b) = digit_sub2(src[i], src[i + k], borrow);
    dest[i] = v;
    borrow = b;
  }
  let (v, _) = digit_sub2(0, src[2 * k], borrow);
  dest[k] = v;
  // borrow 可能非零, mod_fn 会处理
  mod_fn(dest);
}

/// 就地和差 (输入输出重叠安全): x = a+b, y = a-b, 再各做 mod F_n
fn sum_diff_inplace(x: &mut [Digit], y: &mut [Digit]) {
  let mut carry = 0;
  let mut borrow = 0;
  for i in 0..x.len() {
    let ai = x[i];
    let bi = y[i];
    let (v, c) = digit_add3(ai, bi, carry);
    x[i] = v;
    carry = c;
    let (v2, b) = digit_sub2(ai, bi, borrow);
    y[i] = v2;
    borrow = b;
  }
  mod_fn(x);
  mod_fn(y);
}

/// x = a + b 就地, y_out = a - b (b 只读); 支持输出与 a 重叠
fn sum_diff_x_inplace(x: &mut [Digit], y_out: &mut [Digit], b: &[Digit]) {
  let mut carry = 0;
  let mut borrow = 0;
  for i in 0..x.len() {
    let ai = x[i];
    let bi = b[i];
    let (v, c) = digit_add3(ai, bi, carry);
    x[i] = v;
    carry = c;
    let (v2, bk) = digit_sub2(ai, bi, borrow);
    y_out[i] = v2;
    borrow = bk;
  }
  mod_fn(x);
  mod_fn(y_out);
}

/// result := (input << shift) mod F_n, digit_shift >= K (利用 2^K ≡ -1 折叠)
fn shift_mod_fn_large(
  result: &mut [Digit],
  input: &[Digit],
  mut digit_shift: usize,
  bits_shift: u32,
  k: usize,
) {
  debug_assert!(digit_shift >= k);
  digit_shift -= k;
  let mut borrow;
  if bits_shift == 0 {
    let mut carry = 1;
    for i in 0..digit_shift {
      let (v, c) = digit_add2(input[i + k - digit_shift], carry);
      result[i] = v;
      carry = c;
    }
    let (v, mut b0) = digit_sub(input[k].wrapping_add(carry), input[0]);
    result[digit_shift] = v;
    for i in digit_shift + 1..k {
      let d = input[i - digit_shift];
      let (v, b2) = digit_sub2(0, d, b0);
      result[i] = v;
      b0 = b2;
    }
    borrow = b0;
  } else {
    let mut add_carry = 1;
    let mut input_carry = input[k - digit_shift - 1] >> (K_DIGIT_BITS - bits_shift);
    for i in 0..digit_shift {
      let d = input[i + k - digit_shift];
      let summand = (d << bits_shift) | input_carry;
      let (v, c) = digit_add2(summand, add_carry);
      result[i] = v;
      add_carry = c;
      input_carry = d >> (K_DIGIT_BITS - bits_shift);
    }
    {
      // result[digit_shift] = (add_carry + iK 部分) - i0 部分
      let d = input[k];
      let i_k_part = (d << bits_shift) | input_carry;
      let mut i_k_carry = d >> (K_DIGIT_BITS - bits_shift);
      let (sum, ac) = digit_add2(add_carry, i_k_part);
      // iK_carry 不足一整个 digit, 可并入 add_carry 而不溢出
      i_k_carry += ac;
      let d0 = input[0];
      let i0_part = d0 << bits_shift;
      let (v, b) = digit_sub(sum, i0_part);
      result[digit_shift] = v;
      borrow = b;
      input_carry = d0 >> (K_DIGIT_BITS - bits_shift);
      if digit_shift + 1 < k {
        let d1 = input[1];
        let subtrahend = (d1 << bits_shift) | input_carry;
        let (v, b2) = digit_sub2(i_k_carry, subtrahend, borrow);
        result[digit_shift + 1] = v;
        borrow = b2;
        input_carry = d1 >> (K_DIGIT_BITS - bits_shift);
      }
    }
    for i in digit_shift + 2..k {
      let d = input[i - digit_shift];
      let subtrahend = (d << bits_shift) | input_carry;
      let (v, b) = digit_sub2(0, subtrahend, borrow);
      result[i] = v;
      borrow = b;
      input_carry = d >> (K_DIGIT_BITS - bits_shift);
    }
  }
  // 虚拟顶位 1 应被 borrow 消掉; 无借位说明虚拟初值给多了, 再减 2^K + 1
  result[k] = 0;
  if borrow != 1 {
    borrow = 1;
    for ri in result[..k].iter_mut() {
      let (v, b) = digit_sub(*ri, borrow);
      *ri = v;
      borrow = b;
      if borrow == 0 {
        break;
      }
    }
    if borrow != 0 {
      // 结果必为 2^K
      result[..k].fill(0);
      result[k] = 1;
    }
  }
}

/// result := input * 2^power_of_two mod (2^K + 1); 性能关键路径
/// zero_above: 已知 input 在该下标之上全零
fn shift_mod_fn(
  result: &mut [Digit],
  input: &[Digit],
  power_of_two: u32,
  k: usize,
  zero_above: usize,
) {
  let mut digit_shift = (power_of_two / K_DIGIT_BITS) as usize;
  let bits_shift = power_of_two % K_DIGIT_BITS;
  // x * 2^(2K+m) == x * 2^m (mod 2^K+1), 循环减比取模快
  while digit_shift >= 2 * k {
    digit_shift -= 2 * k;
  }
  if digit_shift >= k {
    return shift_mod_fn_large(result, input, digit_shift, bits_shift, k);
  }
  let mut borrow;
  if bits_shift == 0 {
    // 单趟: 先拷 [i1..iX-1] 到 result 高位, 再从虚拟零减 [iX..iK]
    let mut i = 1;
    let mut b0 = 0;
    let mut cap = (k - digit_shift).min(zero_above);
    while i < cap {
      result[i + digit_shift] = input[i];
      i += 1;
    }
    while i < k - digit_shift {
      result[i + digit_shift] = 0;
      i += 1;
    }
    cap = k.min(zero_above);
    while i < cap {
      let d = input[i];
      let (v, b) = digit_sub2(0, d, b0);
      result[i + digit_shift - k] = v;
      b0 = b;
      i += 1;
    }
    while i < k {
      let (v, b) = digit_sub(0, b0);
      result[i + digit_shift - k] = v;
      b0 = b;
      i += 1;
    }
    let (v, b) = digit_sub2(input[0], input[k], b0);
    result[digit_shift] = v;
    borrow = b;
  } else {
    // 同流程但带 bits_shift
    let mut carry = 0;
    let mut i = 0;
    let mut b1 = 0;
    let mut cap = (k - digit_shift).min(zero_above);
    while i < cap {
      let d = input[i];
      result[i + digit_shift] = (d << bits_shift) | carry;
      carry = d >> (K_DIGIT_BITS - bits_shift);
      i += 1;
    }
    while i < k - digit_shift {
      result[i + digit_shift] = carry;
      carry = 0;
      i += 1;
    }
    cap = k.min(zero_above);
    while i < cap {
      let d = input[i];
      let (v, b) = digit_sub2(0, (d << bits_shift) | carry, b1);
      result[i + digit_shift - k] = v;
      b1 = b;
      carry = d >> (K_DIGIT_BITS - bits_shift);
      i += 1;
    }
    if i < k {
      let (v, b) = digit_sub2(0, carry, b1);
      result[i + digit_shift - k] = v;
      b1 = b;
      carry = 0;
      i += 1;
    }
    while i < k {
      let (v, b) = digit_sub(0, b1);
      result[i + digit_shift - k] = v;
      b1 = b;
      i += 1;
    }
    let d = input[k];
    let (v, b) = digit_sub2(result[digit_shift], (d << bits_shift) | carry, b1);
    result[digit_shift] = v;
    borrow = b;
    debug_assert!((d >> (K_DIGIT_BITS - bits_shift)) == 0);
  }
  result[k] = 0;
  let mut i = digit_shift + 1;
  while i <= k && borrow > 0 {
    let (v, b) = digit_sub(result[i], borrow);
    result[i] = v;
    borrow = b;
    i += 1;
  }
  if borrow > 0 {
    // 下溢说明减多了, 加回 2^K + 1
    let mut carry = 1;
    for rj in result[..=k].iter_mut() {
      let (v, c) = digit_add2(*rj, carry);
      *rj = v;
      carry = c;
      if carry == 0 {
        break;
      }
    }
    let (v, _) = digit_add2(result[k], 1);
    result[k] = v;
  }
}

// ---------------------------------------------------------------------------
// 第 2 部分: 参数选择
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default)]
struct Parameters {
  m: i32,
  k: u32,
  n: u32,
  s: u32,
  r: u32,
}

/// 主层参数 (输入 digit 长 N 与 m)
fn compute_parameters(n_digits: u32, m: i32, params: &mut Parameters) {
  let n_total = n_digits * K_DIGIT_BITS;
  let n = 1u32 << m;
  let nhalf = n >> 1;
  let mut s = (n_total + n - 1) >> m; // ceil(N/n), bit 域
  s = round_up(s, K_DIGIT_BITS);
  let mut k = m as u32 + 2 * s + 1; // K 至少这么大 (bit 域)...
  k = round_up(k, nhalf); // ...且为 n/2 的倍数
  let mut r = k >> (m - 1); // 第几个倍数

  // 递归要能推进: 超过递归阈值时 K 补到 8 的倍数, 否则补到 kDigitBits 的倍数
  let threshold = if k + 1 >= K_FFT_INNER_THRESHOLD * K_DIGIT_BITS {
    3 + K_LOG2_DIGIT_BITS
  } else {
    K_LOG2_DIGIT_BITS
  };
  let mut k_tz = count_trailing_zeros(k);
  while k_tz < threshold {
    k += 1 << k_tz;
    r = k >> (m - 1);
    k_tz = count_trailing_zeros(k);
  }

  debug_assert!(k.is_multiple_of(K_DIGIT_BITS));
  debug_assert!(s.is_multiple_of(K_DIGIT_BITS));
  params.k = k / K_DIGIT_BITS;
  params.s = s / K_DIGIT_BITS;
  params.n = n;
  params.r = r;
}

/// 递归内层参数
fn compute_parameters_inner(n_digits: u32, params: &mut Parameters) {
  let max_m = count_trailing_zeros(n_digits) as i32;
  let mut m = bit_length(n_digits) as i32 - 4; // 不让 s 太小
  m = max_m.min(m);
  let n_total = n_digits * K_DIGIT_BITS;
  let n = 1u32 << m;
  // 内层不能向上取整 s: N = n*s 固定
  let s = n_total >> m;
  debug_assert!(n_total == s * n);
  let mut k = m as u32 + 2 * s + 1;
  k = round_up(k, n);
  k = round_up(k, K_DIGIT_BITS);
  params.r = k >> m;
  debug_assert!(k.is_multiple_of(K_DIGIT_BITS));
  debug_assert!(s.is_multiple_of(K_DIGIT_BITS));
  params.k = k / K_DIGIT_BITS;
  params.s = s / K_DIGIT_BITS;
  params.n = n;
  params.m = m;
}

fn predict_inner_k(n_digits: u32) -> u32 {
  let mut params = Parameters::default();
  compute_parameters_inner(n_digits, &mut params);
  params.k
}

/// m 是否应减一 (看 K/s 随 m-1, m-2 的变化趋势; 阈值由实测选定)
fn should_decrement_m(current: &Parameters, next: &Parameters, after_next: &Parameters) -> bool {
  // K == 64 时表现尤其好
  if current.k == 64 && next.k >= 112 {
    return false;
  }
  // s 太小永远不划算
  if current.s < 6 {
    return true;
  }
  // 时间大致由 K * n 决定; 减 m 则 n 减半而 K 常变大, 看 (再) 下一层的放大倍数
  let factor = after_next.k as f64 / current.k as f64;
  if (current.s == 6 && factor < 3.85)
    || (current.s == 7 && factor < 3.73)
    || (current.s == 8 && factor < 3.55)
    || (current.s == 9 && factor < 3.50)
    || factor < 3.4
  {
    return true;
  }
  // K 刚低于递归阈值时确保递归, 除非内层 K 过大
  if current.k >= 160 && current.k < 250 && predict_inner_k(next.k) < 28 {
    return true;
  }
  false
}

/// 为输入 digit 长 N 选参数, 返回 m
fn get_parameters(n_digits: u32, params: &mut Parameters) -> i32 {
  let mut max_m = bit_length(n_digits) as i32 - 3; // 更大的 m 让 s 太小
  max_m = max_m.max(K_LOG2_DIGIT_BITS as i32); // 更小的 m 破坏后续逻辑
  let mut m = max_m;
  let mut current = Parameters::default();
  compute_parameters(n_digits, m, &mut current);
  let mut next = Parameters::default();
  compute_parameters(n_digits, m - 1, &mut next);
  while m > 2 {
    let mut after_next = Parameters::default();
    compute_parameters(n_digits, m - 2, &mut after_next);
    if should_decrement_m(&current, &next, &after_next) {
      m -= 1;
      current = next;
      next = after_next;
    } else {
      break;
    }
  }
  *params = current;
  m
}

// ---------------------------------------------------------------------------
// 第 3 部分: FFT 变换 (storage 为 n 个长 length 的 part 连排)
// ---------------------------------------------------------------------------

/// 切出同一存储中两个不相邻 part 的可变引用 (first 下标 < second 下标)
fn two_parts_mut(
  storage: &mut [Digit],
  length: usize,
  first: usize,
  second: usize,
) -> (&mut [Digit], &mut [Digit]) {
  debug_assert!(first < second);
  debug_assert!(second * length <= storage.len());
  let (lo, hi) = storage.split_at_mut(second * length);
  (
    &mut lo[first * length..(first + 1) * length],
    &mut hi[..length],
  )
}

/// 正向 FFT (DIF: 输出按位反转序, 恰好是反向 DIT 需要的输入)
fn fft_shuffled(
  storage: &mut [Digit],
  temp: &mut [Digit],
  start: usize,
  len: usize,
  omega: i32,
  length: usize,
  k: usize,
) {
  debug_assert!(len.is_multiple_of(2));
  debug_assert!((start + len) * length <= storage.len());
  let half = len / 2;
  {
    let (pa, pb) = two_parts_mut(storage, length, start, start + half);
    sum_diff_inplace(pa, pb);
  }
  for idx in 1..half {
    {
      // part idx 累加半区对位; 差写入 temp
      let (pa, pb) = two_parts_mut(storage, length, start + idx, start + half + idx);
      sum_diff_x_inplace(pa, &mut temp[..length], pb);
    }
    // 差加权后写回半区对位
    let w = omega * idx as i32;
    let pb = &mut storage[(start + half + idx) * length..(start + half + idx + 1) * length];
    shift_mod_fn(pb, &temp[..length], w as u32, k, ZERO_ABOVE_INF);
  }
  fft_recurse(storage, temp, start, half, omega, length, k);
}

/// 正向 FFT 的递归步 (覆盖 [start, start + 2*half))
fn fft_recurse(
  storage: &mut [Digit],
  temp: &mut [Digit],
  start: usize,
  half: usize,
  omega: i32,
  length: usize,
  k: usize,
) {
  if half > 1 {
    fft_shuffled(storage, temp, start, half, 2 * omega, length, k);
    fft_shuffled(storage, temp, start + half, half, 2 * omega, length, k);
  }
}

/// 反向 FFT (DIT: 位反转输入 → 正常序输出)
fn backward_fft_threadsafe(
  storage: &mut [Digit],
  temp: &mut [Digit],
  start: usize,
  len: usize,
  omega: i32,
  length: usize,
  k: usize,
) {
  debug_assert!(len.is_multiple_of(2));
  let half = len / 2;
  // half == 2 不递归: 逐点乘已做了一层反向 FFT
  if half > 2 {
    backward_fft_threadsafe(storage, temp, start, half, 2 * omega, length, k);
    backward_fft_threadsafe(storage, temp, start + half, half, 2 * omega, length, k);
  }
  {
    let (pa, pb) = two_parts_mut(storage, length, start, start + half);
    sum_diff_inplace(pa, pb);
  }
  for idx in 1..half {
    let w = omega * (len - idx) as i32;
    {
      let pb = &mut storage[(start + half + idx) * length..(start + half + idx + 1) * length];
      shift_mod_fn(&mut temp[..length], pb, w as u32, k, ZERO_ABOVE_INF);
    }
    let (pa, pb) = two_parts_mut(storage, length, start + idx, start + half + idx);
    sum_diff_x_inplace(pa, pb, &temp[..length]);
  }
}

/// 是否应把 part 解释为负数 (数值 >= threshold * F_n / 2)
fn should_be_negative(x: &[Digit], threshold: Digit, s: usize) -> bool {
  if x[2 * s] >= threshold {
    return true;
  }
  for &d in &x[2 * s + 1..] {
    if d > 0 {
      return true;
    }
  }
  false
}

/// FFT 容器: n 个长 K+1 的 part + 2*(K+1) 临时区 (处理器由调用方传入, 便于双容器并存)
struct FftContainer {
  n: usize,
  k: usize,
  length: usize,
  storage: Vec<Digit>,
  temp: Vec<Digit>,
}

impl FftContainer {
  fn new(n: usize, k: usize) -> Self {
    let length = k + 1;
    Self {
      n,
      k,
      length,
      storage: vec![0; length * n],
      temp: vec![0; length * 2],
    }
  }

  /// 读入 X 分块并做正向 FFT (theta 非 0 时逐块加权)
  fn start_default(&mut self, x: &[Digit], mut chunk_size: usize, theta: i32, omega: i32) {
    let mut len = x.len();
    let mut pos = 0;
    let mut current_theta = 0i32;
    let mut i = 0;
    while i < self.n && len > 0 {
      chunk_size = chunk_size.min(len);
      // 内层调用的 X.len == n*chunk_size+1: 外层 mod Fn 归一后顶位可能为 1, 并入末块
      if i == self.n - 1 && len == chunk_size + 1 {
        debug_assert!(x[self.n * chunk_size] <= 1);
        debug_assert!(self.length > chunk_size);
        chunk_size += 1;
      }
      let length = self.length;
      let k = self.k;
      let part = &mut self.storage[i * length..(i + 1) * length];
      if current_theta != 0 {
        // 乘以 theta^i 并 mod 2^K+1 (theta 以移位量传入)
        self.temp[..chunk_size].copy_from_slice(&x[pos..pos + chunk_size]);
        self.temp[chunk_size..length].fill(0);
        shift_mod_fn(
          part,
          &self.temp[..length],
          current_theta as u32,
          k,
          chunk_size,
        );
      } else {
        part[..chunk_size].copy_from_slice(&x[pos..pos + chunk_size]);
        part[chunk_size..length].fill(0);
      }
      pos += chunk_size;
      len -= chunk_size;
      i += 1;
      current_theta += theta;
    }
    debug_assert!(len == 0);
    while i < self.n {
      let length = self.length;
      self.storage[i * length..(i + 1) * length].fill(0);
      i += 1;
    }
    let (length, k, n) = (self.length, self.k, self.n);
    let (storage, temp) = (&mut self.storage, &mut self.temp);
    fft_shuffled(storage, temp, 0, n, omega, length, k);
  }

  /// 约一半 part 为零填充时的优化入口
  fn start(&mut self, x: &[Digit], mut chunk_size: usize, theta: i32, omega: i32) {
    let len = x.len();
    if len > self.n * chunk_size / 2 {
      return self.start_default(x, chunk_size, theta, omega);
    }
    debug_assert!(theta == 0);
    let length = self.length;
    let nhalf = self.n / 2;
    // 展开首轮: 同一份数据填 part 0 与 part nhalf
    chunk_size = chunk_size.min(len);
    {
      let (lo, hi) = self.storage.split_at_mut(nhalf * length);
      let p0 = &mut lo[..length];
      let pn = &mut hi[..length];
      p0[..chunk_size].copy_from_slice(&x[..chunk_size]);
      p0[chunk_size..length].fill(0);
      pn[..chunk_size].copy_from_slice(&x[..chunk_size]);
      pn[chunk_size..length].fill(0);
    }
    let mut pos = chunk_size;
    let mut len = len - chunk_size;
    let mut i = 1;
    while i < nhalf && len > 0 {
      chunk_size = chunk_size.min(len);
      let (lo, hi) = self.storage.split_at_mut((i + nhalf) * length);
      let pi = &mut lo[i * length..(i + 1) * length];
      pi[..chunk_size].copy_from_slice(&x[pos..pos + chunk_size]);
      pi[chunk_size..length].fill(0);
      let pi_half = &mut hi[..length];
      let w = omega * i as i32;
      shift_mod_fn(pi_half, pi, w as u32, self.k, chunk_size);
      pos += chunk_size;
      len -= chunk_size;
      i += 1;
    }
    while i < nhalf {
      let (lo, hi) = self.storage.split_at_mut((i + nhalf) * length);
      lo[i * length..(i + 1) * length].fill(0);
      hi[..length].fill(0);
      i += 1;
    }
    let (storage, temp) = (&mut self.storage, &mut self.temp);
    fft_recurse(storage, temp, 0, nhalf, omega, length, self.k);
  }

  /// 反向 FFT
  fn backward_fft(&mut self, start: usize, len: usize, omega: i32) {
    let (length, k) = (self.length, self.k);
    let (storage, temp) = (&mut self.storage, &mut self.temp);
    backward_fft_threadsafe(storage, temp, start, len, omega, length, k);
  }

  /// 反向 FFT 后按 2^(-shift) 加权重组进 Z
  fn normalize_and_recombine(&mut self, omega: i32, m: i32, z: &mut [Digit], chunk_size: usize) {
    z.fill(0);
    let mut z_index = 0;
    let shift = self.n as i32 * omega - m;
    for i in 0..self.n {
      let (length, k) = (self.length, self.k);
      {
        let part = &self.storage[i * length..(i + 1) * length];
        shift_mod_fn(&mut self.temp, part, shift as u32, k, ZERO_ABOVE_INF);
      }
      let mut carry = 0;
      let mut zi = z_index;
      let mut j = 0;
      while j < length && zi < z.len() {
        let (v, c) = digit_add3(z[zi], self.temp[j], carry);
        z[zi] = v;
        carry = c;
        j += 1;
        zi += 1;
      }
      if carry != 0 {
        z[zi] = carry;
      }
      z_index += chunk_size;
    }
  }

  /// 反向 FFT 后附 theta^(-k) 反加权重组进 Z (内层递归用)
  fn counter_weight_and_recombine(&mut self, theta: i32, m: i32, z: &mut [Digit], s: usize) {
    z.fill(0);
    let mut z_index = 0;
    for k in 0..self.n {
      let mut shift = -theta * k as i32 - m;
      if shift < 0 {
        shift += 2 * self.n as i32 * theta;
      }
      debug_assert!(shift >= 0);
      let (length, big_k) = (self.length, self.k);
      {
        let part = &self.storage[k * length..(k + 1) * length];
        shift_mod_fn(&mut self.temp, part, shift as u32, k, ZERO_ABOVE_INF);
      }
      let remaining_z = z.len() - z_index;
      if should_be_negative(&self.temp[..length], (k + 1) as Digit, s) {
        // 先减 F_n 再并入: Z + (X - F_n) == Z - (F_n - X)
        let mut borrow_z = 0;
        let mut borrow_fn;
        {
          // i == 0
          let (d, b) = digit_sub(1, self.temp[0]);
          borrow_fn = b;
          let (v, b2) = digit_sub2(z[z_index], d, borrow_z);
          z[z_index] = v;
          borrow_z = b2;
        }
        let mut i = 1;
        while i < big_k && i < remaining_z {
          let (d, b) = digit_sub2(0, self.temp[i], borrow_fn);
          borrow_fn = b;
          let (v, b2) = digit_sub2(z[z_index + i], d, borrow_z);
          z[z_index + i] = v;
          borrow_z = b2;
          i += 1;
        }
        debug_assert!(i == big_k && big_k == length - 1);
        while i < length && i < remaining_z {
          let (d, b) = digit_sub2(1, self.temp[i], borrow_fn);
          borrow_fn = b;
          let (v, b2) = digit_sub2(z[z_index + i], d, borrow_z);
          z[z_index + i] = v;
          borrow_z = b2;
          i += 1;
        }
        debug_assert!(borrow_fn == 0);
        while borrow_z > 0 && i < remaining_z {
          let (v, b) = digit_sub(z[z_index + i], borrow_z);
          z[z_index + i] = v;
          borrow_z = b;
          i += 1;
        }
      } else {
        let mut carry = 0;
        let mut i = 0;
        while i < length && i < remaining_z {
          let (v, c) = digit_add3(z[z_index + i], self.temp[i], carry);
          z[z_index + i] = v;
          carry = c;
          i += 1;
        }
        while carry > 0 && i < remaining_z {
          let (v, c) = digit_add2(z[z_index + i], carry);
          z[z_index + i] = v;
          carry = c;
          i += 1;
        }
        // carry 非 0 也无妨 (Z 之前可能为 "负")
      }
      z_index += s;
    }
  }
}

impl<P: Platform> Processor<P> {
  /// 逐点乘: a 与 other 两个已变换容器 (other 为 None 表示平方)
  fn fft_pointwise(&mut self, a: &mut FftContainer, other: Option<&mut FftContainer>) {
    // (K & 3) == 0 保证内层 FFT 至少分 4 块
    let use_fft = a.length as u32 >= K_FFT_INNER_THRESHOLD && (a.k & 3) == 0;
    let mut params = Parameters::default();
    if use_fft {
      compute_parameters_inner(a.k as u32, &mut params);
    }
    let mut result = vec![0; 2 * a.length];
    for i in 0..a.n {
      let length = a.length;
      let a_part = &a.storage[i * length..(i + 1) * length];
      let b_part = match other.as_ref() {
        Some(o) => &o.storage[i * length..(i + 1) * length],
        None => a_part,
      };
      if use_fft {
        multiply_fft_inner(self, &mut result, a_part, b_part, &params);
      } else {
        self.multiply(&mut result, a_part, b_part);
      }
      if self.should_terminate() {
        return;
      }
      let part = &mut a.storage[i * length..(i + 1) * length];
      mod_fn_double_width(part, &result);
      // 顺带做一层反向 FFT (缓存友好)
      if i & 1 == 1 {
        let (prev, cur) = two_parts_mut(&mut a.storage, length, i - 1, i);
        sum_diff_inplace(prev, cur);
      }
    }
  }
}

/// 内层 (递归) FFT 乘法主函数
fn multiply_fft_inner<P: Platform>(
  processor: &mut Processor<P>,
  z: &mut [Digit],
  x: &[Digit],
  y: &[Digit],
  params: &Parameters,
) {
  let omega = 2 * params.r as i32; // 实为 2^(2r)
  let theta = params.r as i32; // 实为 2^r

  let mut a = FftContainer::new(params.n as usize, params.k as usize);
  a.start_default(x, params.s as usize, theta, omega);
  let mut b = FftContainer::new(params.n as usize, params.k as usize);
  b.start_default(y, params.s as usize, theta, omega);

  processor.fft_pointwise(&mut a, Some(&mut b));
  if processor.should_terminate() {
    return;
  }

  a.backward_fft(0, params.n as usize, omega);
  a.counter_weight_and_recombine(theta, params.m, z, params.s as usize);
}

// ---------------------------------------------------------------------------
// 第 4 部分: 总装
// ---------------------------------------------------------------------------

impl<P: Platform> Processor<P> {
  /// FFT 乘法入口
  pub fn multiply_fft(&mut self, z: &mut [Digit], x: &[Digit], y: &[Digit]) {
    // 非对称输入的分块阈值
    const K_ASYMMETRIC_CHUNKING_THRESHOLD: usize = 100;

    let mut params = Parameters::default();
    if eq(x.as_ptr(), y.as_ptr()) && x.len() == y.len() {
      // 平方
      let m = get_parameters((x.len() * 2) as u32, &mut params);
      let omega = params.r as i32; // 实为 2^r
      let mut a = FftContainer::new(params.n as usize, params.k as usize);
      a.start(x, params.s as usize, 0, omega);
      self.fft_pointwise(&mut a, None);
      if self.should_terminate() {
        return;
      }
      a.backward_fft(0, params.n as usize, omega);
      a.normalize_and_recombine(omega, m, z, params.s as usize);
    } else if x.len() > y.len() * K_ASYMMETRIC_CHUNKING_THRESHOLD {
      // 尺寸悬殊: 分块 (同 MultiplyToomCook)
      let k = y.len();
      let m = get_parameters((k * 2) as u32, &mut params);
      let omega = params.r as i32;
      // b 只初始化一次, a 逐块复用
      let mut b = FftContainer::new(params.n as usize, params.k as usize);
      b.start(y, params.s as usize, 0, omega);
      let mut a = FftContainer::new(params.n as usize, params.k as usize);
      // 首块展开初始化 Z
      let x0 = &x[..k];
      a.start(x0, params.s as usize, 0, omega);
      self.fft_pointwise(&mut a, Some(&mut b));
      if self.should_terminate() {
        return;
      }
      a.backward_fft(0, params.n as usize, omega);
      a.normalize_and_recombine(omega, m, z, params.s as usize);
      // 其余块
      let mut t = vec![0; 2 * k];
      let mut i = k;
      while i < x.len() {
        let xi = &x[i..(i + k).min(x.len())];
        a.start(xi, params.s as usize, 0, omega);
        self.fft_pointwise(&mut a, Some(&mut b));
        if self.should_terminate() {
          return;
        }
        a.backward_fft(0, params.n as usize, omega);
        a.normalize_and_recombine(omega, m, &mut t, params.s as usize);
        add_and_return_overflow(&mut z[i..], &t); // 不会溢出
        i += k;
      }
    } else {
      // 尺寸相近: 一次算完
      let m = get_parameters((x.len() + y.len()) as u32, &mut params);
      let omega = params.r as i32;
      let mut a = FftContainer::new(params.n as usize, params.k as usize);
      a.start(x, params.s as usize, 0, omega);
      let mut b = FftContainer::new(params.n as usize, params.k as usize);
      b.start(y, params.s as usize, 0, omega);
      self.fft_pointwise(&mut a, Some(&mut b));
      if self.should_terminate() {
        return;
      }
      a.backward_fft(0, params.n as usize, omega);
      a.normalize_and_recombine(omega, m, z, params.s as usize);
    }
  }
}
