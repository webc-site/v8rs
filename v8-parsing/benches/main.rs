//! criterion 主评测入口: 基于 ParseBench trait 泛型驱动
//!
//! 评测 id 结构: <实现 NAME>/<操作>/<样本类别 或 参数>, 由 benches/base.rs 的 trait 定义.
//! 单独评测对比项: cargo bench -p v8-parsing --features bench-naive
//! 全量评测: cargo bench -p v8-parsing --features bench

#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use humansize::{BINARY, format_size};

mod base;

#[cfg(feature = "bench-naive")]
use base::SimpleTokenizer;
use base::{LITERAL_REPS, LITERAL_UNITS, LOOKAHEAD_ROUNDS, ParseBench, V8Scanner, samples};

/// 体积标签: humansize 二进制单位, 空格换 '_' 保证评测 id 可解析
fn size_label(bytes: usize) -> String {
  format_size(bytes, BINARY).replace(' ', "_")
}

/// 对单个参与方建立全部评测组 (评测 id 基于 T::NAME 生成)
fn bench_impl<T: ParseBench>(c: &mut Criterion) {
  let sample_set = samples();

  let mut group = c.benchmark_group(format!("{}/scan_tokens", T::NAME));
  for (class, src) in &sample_set {
    if *class == "regexp" {
      continue;
    }
    group.throughput(Throughput::Bytes(src.len() as u64));
    group.bench_function(BenchmarkId::new(*class, size_label(src.len())), |b| {
      b.iter(|| T::scan_tokens(black_box(src)))
    });
  }
  group.finish();

  let mut group = c.benchmark_group(format!("{}/scan_regexp", T::NAME));
  for (class, src) in &sample_set {
    if *class != "regexp" {
      continue;
    }
    group.throughput(Throughput::Bytes(src.len() as u64));
    group.bench_function(BenchmarkId::new(*class, size_label(src.len())), |b| {
      b.iter(|| T::scan_regexp(black_box(src)))
    });
  }
  group.finish();

  // lookahead 命中路径基于 small 样本 (token 密集, 缓存恒命中)
  let small_src = &sample_set.first().map(|s| s.1.clone()).unwrap_or_default();
  let mut group = c.benchmark_group(format!("{}/lookahead", T::NAME));
  group.throughput(Throughput::Elements((LOOKAHEAD_ROUNDS * 2) as u64));
  group.bench_function("hit_path", |b| {
    b.iter(|| T::lookahead(black_box(small_src), black_box(LOOKAHEAD_ROUNDS)))
  });
  group.finish();

  let mut group = c.benchmark_group(format!("{}/literal_fill", T::NAME));
  group.throughput(Throughput::Elements((LITERAL_UNITS * LITERAL_REPS) as u64));
  group.bench_function("latin1", |b| {
    b.iter(|| T::literal_fill(black_box(LITERAL_UNITS), black_box(LITERAL_REPS)))
  });
  group.finish();
}

/// 参与方注册: v8rs 实现恒在, 对比实现按 feature bench-naive 加入
fn bench_parsing(c: &mut Criterion) {
  bench_impl::<V8Scanner>(c);
  #[cfg(feature = "bench-naive")]
  bench_impl::<SimpleTokenizer>(c);
}

criterion_group!(parsing, bench_parsing);
criterion_main!(parsing);
