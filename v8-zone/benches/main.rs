//! criterion 主评测入口: 基于 base.rs 的 BenchOps trait 泛型驱动
//!
//! 每组评测以操作命名, 评测 id 以实现 NAME 生成, 参数编码进 id
//! (size/align/n/depth), 供 bench.js 解析换算 allocs/s。

#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

#[path = "base.rs"]
mod base;

use std::time::Duration;

#[cfg(feature = "bench-bumpalo")]
use base::BumpBench;
#[cfg(feature = "bench-std")]
use base::StdBench;
use base::{
  ALIGNED_N, ALIGNED_SIZE, ALIGNS, ALLOC_N, BenchOps, DEPTHS, RESET_N, RESET_SIZE, SCOPE_N,
  SCOPE_SIZE, SIZES, ZoneBench,
};
use criterion::{BenchmarkId, Criterion, Throughput};

/// 把一类实现的全部操作注册进 criterion
fn add<T: BenchOps>(c: &mut Criterion) {
  let mut ops = T::new();
  let name = T::NAME;

  let mut g = c.benchmark_group("alloc");
  for &size in &SIZES {
    g.throughput(Throughput::Bytes((size * ALLOC_N) as u64));
    let id = BenchmarkId::new(name, format!("size{size}_n{ALLOC_N}"));
    g.bench_with_input(id, &size, |b, &size| b.iter(|| ops.alloc(ALLOC_N, size)));
  }
  g.finish();

  let mut g = c.benchmark_group("aligned");
  for &align in &ALIGNS {
    let per = (ALIGNED_SIZE + align) * ALIGNED_N;
    g.throughput(Throughput::Bytes(per as u64));
    let id = BenchmarkId::new(
      name,
      format!("align{align}_size{ALIGNED_SIZE}_n{ALIGNED_N}"),
    );
    g.bench_with_input(id, &align, |b, &align| {
      b.iter(|| ops.aligned(ALIGNED_N, ALIGNED_SIZE, align))
    });
  }
  g.finish();

  let mut g = c.benchmark_group("scoped");
  for &depth in &DEPTHS {
    let per = depth * SCOPE_N * SCOPE_SIZE;
    g.throughput(Throughput::Bytes(per as u64));
    let id = BenchmarkId::new(name, format!("depth{depth}_size{SCOPE_SIZE}_n{SCOPE_N}"));
    g.bench_with_input(id, &depth, |b, &depth| {
      b.iter(|| ops.scoped(depth, SCOPE_N, SCOPE_SIZE))
    });
  }
  g.finish();

  let mut g = c.benchmark_group("reset");
  g.throughput(Throughput::Bytes((RESET_SIZE * RESET_N) as u64));
  let id = BenchmarkId::new(name, format!("size{RESET_SIZE}_n{RESET_N}"));
  g.bench_with_input(id, &RESET_SIZE, |b, &size| {
    b.iter(|| ops.reset(RESET_N, size))
  });
  g.finish();
}

fn main() {
  let mut c = Criterion::default()
    .warm_up_time(Duration::from_secs(1))
    .measurement_time(Duration::from_secs(3))
    .configure_from_args();
  add::<ZoneBench>(&mut c);
  #[cfg(feature = "bench-bumpalo")]
  add::<BumpBench>(&mut c);
  #[cfg(feature = "bench-std")]
  add::<StdBench>(&mut c);
  c.final_summary();
}
