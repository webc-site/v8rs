//! v8-numbers 核心评测操作 — trait 定义 + 各实现
//!
//! 评测 id 基于 `Op::NAME` 生成 (criterion 组名 = 操作名, bench 名 = NAME)。
//! 对比库 (todo 10.2): ryu / itoa / std format/parse 作对照; zmij 已是
//! v8-numbers 内部内核, 不单独重复引入。

#[cfg(feature = "bench-std")]
use std::str::from_utf8;

use fastrand::Rng;
use v8_numbers::conversions::{
  ConversionFlag, DOUBLE_TO_STRING_MIN_BUFFER_SIZE, double_to_string_view, int_to_string_view,
  string_to_double, string_to_int,
};

/// 每次迭代喂入的样本条数 (固定数据集)
pub const SAMPLE_COUNT: usize = 1000;

/// 输出缓冲容量 — std Display 无指数形态, 5e-324 展开可达 ~330 字节
pub const OUT_BUF_SIZE: usize = 512;

// 参与评测的操作位掩码
pub const OP_DTOA: u32 = 1 << 0;
pub const OP_ITOS: u32 = 1 << 1;
pub const OP_STRTOD: u32 = 1 << 2;
pub const OP_PARSE_INT: u32 = 1 << 3;
pub const OP_ROUNDTRIP: u32 = 1 << 4;

/// 本 crate 核心评测操作族 — Number↔String 双向转换
///
/// 实现方只需覆写自己参与的操作, 用 `OPS` 声明; 未覆写的默认体返回
/// 中性值 (0 / NaN), 不会被评测框架调用。
pub trait Op: Sized {
  /// 评测 id 名 (criterion bench 名)
  const NAME: &'static str;
  /// 参与的操作位集合
  const OPS: u32;

  fn new() -> Self;

  /// Number→String (ECMAScript 9.8.1, 整数/小数/指数切换)
  fn dtoa(&mut self, v: f64, out: &mut [u8]) -> usize {
    let _ = (v, out);
    0
  }

  /// Int32→String (整数快速路径)
  fn itos(&mut self, n: i32, out: &mut [u8]) -> usize {
    let _ = (n, out);
    0
  }

  /// String→Number (ECMAScript 9.3.1 / parseFloat)
  fn strtod(&mut self, s: &[u8]) -> f64 {
    let _ = s;
    f64::NAN
  }

  /// parseInt (十进制)
  fn parse_int(&mut self, s: &[u8]) -> f64 {
    let _ = s;
    f64::NAN
  }

  /// round-trip: dtoa 后立即 strtod
  fn roundtrip(&mut self, v: f64, out: &mut [u8]) -> f64 {
    let n = self.dtoa(v, out);
    self.strtod(&out[..n])
  }
}

/// 把库自管的输出拷贝进调用方缓冲, 返回写入长度 (仅 bench-std 对照路径消费)
#[cfg(feature = "bench-std")]
fn emit(out: &mut [u8], s: &[u8]) -> usize {
  let n = s.len().min(out.len());
  out[..n].copy_from_slice(&s[..n]);
  n
}

// ---------------------------------------------------------------------------
// 固定确定性数据集
// ---------------------------------------------------------------------------

/// 1e3 条 double 样本: 覆盖各量级 (1e-320..1e308)、整数/小数/指数切换
/// 边界 (decimal_point = 21 / -6 / 0) 与极值 (f64::MAX、最小规格化/非规格化)
pub fn doubles() -> Vec<f64> {
  let mut rng = Rng::with_seed(0x52d4_7b8f_0eab_cdef);
  let mut out = Vec::with_capacity(SAMPLE_COUNT);

  // 手工构造: ES 9.8.1 格式分支切换边界 + 极值
  out.extend_from_slice(&[
    1e-7,
    1.0000001e-7,
    9.99999e-7,
    1e-6,
    1.5e-6,
    0.1,
    1.0 / 3.0,
    2.0 / 3.0,
    1234.5678,
    -0.5,
    -1e-9,
    1e15,
    9007199254740992.0,
    9999999999999999.0,
    1e21,
    1e21 + 1024.0,
    -1e22,
    1e100,
    1e-100,
    f64::MAX,
    f64::MIN,
    f64::MIN_POSITIVE,
    2.2250738585072014e-308,
    5e-324,
    -5e-324,
  ]);

  // 非规格化随机样本
  for _ in 0..50 {
    let v = f64::from_bits(rng.u64(..) & 0x000f_ffff_ffff_ffff);
    if v != 0.0 {
      out.push(v);
    }
  }

  // 量级扫描: 随机尾数 × 10^k, k ∈ [-300, 300]
  while out.len() < SAMPLE_COUNT {
    let k = rng.u32(0..601) as i32 - 300;
    let v = rng.f64() * 10f64.powi(k);
    if v.is_finite() && v != 0.0 {
      out.push(v);
    }
  }
  out
}

/// i32 样本: 位数 1..10 覆盖 + 边界值 + 随机
pub fn ints() -> Vec<i32> {
  let mut rng = Rng::with_seed(0x9e37_79b9_85eb_ca8b);
  let mut out = vec![
    0,
    1,
    -1,
    9,
    10,
    99,
    100,
    999_999,
    1_000_000,
    1 << 30,
    -(1 << 30),
    i32::MIN,
    i32::MAX,
  ];
  while out.len() < SAMPLE_COUNT {
    out.push(rng.i32(..));
  }
  out
}

/// String→Number 样本: 由 v8-numbers 自身 dtoa 产物构成 (确定性,
/// ECMAScript 十进制/指数形态), 保证各实现均可解析
pub fn number_strings() -> Vec<Vec<u8>> {
  let mut buf = [0u8; DOUBLE_TO_STRING_MIN_BUFFER_SIZE];
  doubles()
    .iter()
    .map(|&v| double_to_string_view(v, &mut buf).as_bytes().to_vec())
    .collect()
}

/// parseInt 样本: 十进制整数字符串 (含负数、各位数)
pub fn int_strings() -> Vec<Vec<u8>> {
  let mut buf = [0u8; DOUBLE_TO_STRING_MIN_BUFFER_SIZE];
  ints()
    .iter()
    .map(|&n| int_to_string_view(n, &mut buf).as_bytes().to_vec())
    .collect()
}

// ---------------------------------------------------------------------------
// v8-numbers 实现 (被测主体: zmij 内核 + ECMAScript 格式层)
// ---------------------------------------------------------------------------

/// 被测主体 — v8-numbers 全操作参与
pub struct V8rs;

impl Op for V8rs {
  const NAME: &'static str = "v8-numbers";
  const OPS: u32 = OP_DTOA | OP_ITOS | OP_STRTOD | OP_PARSE_INT | OP_ROUNDTRIP;

  fn new() -> Self {
    V8rs
  }

  fn dtoa(&mut self, v: f64, out: &mut [u8]) -> usize {
    double_to_string_view(v, out).len()
  }

  fn itos(&mut self, n: i32, out: &mut [u8]) -> usize {
    int_to_string_view(n, out).len()
  }

  fn strtod(&mut self, s: &[u8]) -> f64 {
    string_to_double(s, ConversionFlag::NoConversionFlag, f64::NAN)
  }

  fn parse_int(&mut self, s: &[u8]) -> f64 {
    string_to_int(s, 10)
  }
}

// ---------------------------------------------------------------------------
// std format/parse 对照
// ---------------------------------------------------------------------------

/// std 对照 — format!/to_string/parse::<f64> 路径 (非 ECMAScript 格式,
/// 仅作速度参照)
#[cfg(feature = "bench-std")]
pub struct Std;

#[cfg(feature = "bench-std")]
impl Op for Std {
  const NAME: &'static str = "std";
  const OPS: u32 = OP_DTOA | OP_ITOS | OP_STRTOD | OP_PARSE_INT | OP_ROUNDTRIP;

  fn new() -> Self {
    Std
  }

  fn dtoa(&mut self, v: f64, out: &mut [u8]) -> usize {
    emit(out, v.to_string().as_bytes())
  }

  fn itos(&mut self, n: i32, out: &mut [u8]) -> usize {
    emit(out, n.to_string().as_bytes())
  }

  fn strtod(&mut self, s: &[u8]) -> f64 {
    from_utf8(s).map_or(f64::NAN, |t| t.parse().unwrap_or(f64::NAN))
  }

  fn parse_int(&mut self, s: &[u8]) -> f64 {
    self.strtod(s)
  }
}

// ---------------------------------------------------------------------------
// ryu 对照 (最短十进制格式化内核)
// ---------------------------------------------------------------------------

/// ryu 对照 — 仅格式化方向 (ryu 主线无解析)
#[cfg(feature = "bench-ryu")]
pub struct Ryu {
  buf: ryu::Buffer,
}

#[cfg(feature = "bench-ryu")]
impl Op for Ryu {
  const NAME: &'static str = "ryu";
  const OPS: u32 = OP_DTOA;

  fn new() -> Self {
    Ryu {
      buf: ryu::Buffer::new(),
    }
  }

  fn dtoa(&mut self, v: f64, out: &mut [u8]) -> usize {
    emit(out, self.buf.format(v).as_bytes())
  }
}

// ---------------------------------------------------------------------------
// itoa 对照 (整数格式化内核)
// ---------------------------------------------------------------------------

/// itoa 对照 — 仅整数格式化方向
#[cfg(feature = "bench-itoa")]
pub struct Itoa {
  buf: itoa::Buffer,
}

#[cfg(feature = "bench-itoa")]
impl Op for Itoa {
  const NAME: &'static str = "itoa";
  const OPS: u32 = OP_ITOS;

  fn new() -> Self {
    Itoa {
      buf: itoa::Buffer::new(),
    }
  }

  fn itos(&mut self, n: i32, out: &mut [u8]) -> usize {
    emit(out, self.buf.format(n).as_bytes())
  }
}
