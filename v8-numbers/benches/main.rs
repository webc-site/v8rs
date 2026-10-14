//! v8-numbers criterion 主评测入口 — 基于 `base::Op` trait 泛型驱动
//!
//! 组名 = 操作 (dtoa/itos/strtod/parse-int/roundtrip), bench 名 = 实现 NAME,
//! 结果落在 criterion/<op>/<NAME>/ 下, 由 crate 根 bench.js 汇总。

#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

#[path = "base.rs"]
mod base;

use std::hint::black_box;

use base::{
  OP_DTOA, OP_ITOS, OP_PARSE_INT, OP_ROUNDTRIP, OP_STRTOD, OUT_BUF_SIZE, SAMPLE_COUNT, V8rs,
};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};

/// dtoa 族评测: Number→String (整数/小数/指数切换边界/极值样本集)
fn bench_dtoa<S: base::Op>(c: &mut Criterion) {
  if S::OPS & OP_DTOA == 0 {
    return;
  }
  let samples = base::doubles();
  let mut group = c.benchmark_group("dtoa");
  group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));
  group.bench_function(S::NAME, |b| {
    let mut imp = S::new();
    let mut out = vec![0u8; OUT_BUF_SIZE];
    b.iter(|| {
      let mut acc = 0usize;
      for &v in &samples {
        acc += imp.dtoa(black_box(v), &mut out);
      }
      black_box(acc)
    });
  });
  group.finish();
}

/// 整数格式化评测: Int32→String
fn bench_itos<S: base::Op>(c: &mut Criterion) {
  if S::OPS & OP_ITOS == 0 {
    return;
  }
  let samples = base::ints();
  let mut group = c.benchmark_group("itos");
  group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));
  group.bench_function(S::NAME, |b| {
    let mut imp = S::new();
    let mut out = vec![0u8; OUT_BUF_SIZE];
    b.iter(|| {
      let mut acc = 0usize;
      for &n in &samples {
        acc += imp.itos(black_box(n), &mut out);
      }
      black_box(acc)
    });
  });
  group.finish();
}

/// 解析评测: String→Number (strtod/parseFloat 代表样本)
fn bench_strtod<S: base::Op>(c: &mut Criterion) {
  if S::OPS & OP_STRTOD == 0 {
    return;
  }
  let samples = base::number_strings();
  let mut group = c.benchmark_group("strtod");
  group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));
  group.bench_function(S::NAME, |b| {
    let mut imp = S::new();
    b.iter(|| {
      let mut acc = 0f64;
      for s in &samples {
        acc += imp.strtod(black_box(s));
      }
      black_box(acc)
    });
  });
  group.finish();
}

/// parseInt 评测: 十进制整数字符串
fn bench_parse_int<S: base::Op>(c: &mut Criterion) {
  if S::OPS & OP_PARSE_INT == 0 {
    return;
  }
  let samples = base::int_strings();
  let mut group = c.benchmark_group("parse-int");
  group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));
  group.bench_function(S::NAME, |b| {
    let mut imp = S::new();
    b.iter(|| {
      let mut acc = 0f64;
      for s in &samples {
        acc += imp.parse_int(black_box(s));
      }
      black_box(acc)
    });
  });
  group.finish();
}

/// round-trip 评测: dtoa → strtod
fn bench_roundtrip<S: base::Op>(c: &mut Criterion) {
  if S::OPS & OP_ROUNDTRIP == 0 {
    return;
  }
  let samples = base::doubles();
  let mut group = c.benchmark_group("roundtrip");
  group.throughput(Throughput::Elements(SAMPLE_COUNT as u64));
  group.bench_function(S::NAME, |b| {
    let mut imp = S::new();
    let mut out = vec![0u8; OUT_BUF_SIZE];
    b.iter(|| {
      let mut acc = 0f64;
      for &v in &samples {
        acc += imp.roundtrip(black_box(v), &mut out);
      }
      black_box(acc)
    });
  });
  group.finish();
}

/// 注册一个实现的全部参与操作
fn register<S: base::Op>(c: &mut Criterion) {
  bench_dtoa::<S>(c);
  bench_itos::<S>(c);
  bench_strtod::<S>(c);
  bench_parse_int::<S>(c);
  bench_roundtrip::<S>(c);
}

fn numbers(c: &mut Criterion) {
  register::<V8rs>(c);
  #[cfg(feature = "bench-std")]
  register::<base::Std>(c);
  #[cfg(feature = "bench-ryu")]
  register::<base::Ryu>(c);
  #[cfg(feature = "bench-itoa")]
  register::<base::Itoa>(c);
}

criterion_group!(benches, numbers);
criterion_main!(benches);
