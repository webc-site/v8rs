//! criterion 主评测入口：基于 Target trait 泛型驱动，评测 id 由 操作名 + T::NAME 组成

#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

mod base;

use std::hint::black_box;

use base::{Target, V8Target};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

/// 评测项：操作 id + 对应 trait 方法
type Op<T> = (&'static str, fn(&mut T) -> u64);

/// 对一个实现跑全部评测项
fn run<T: Target>(c: &mut Criterion) {
  let mut t = T::default();
  let ops: [Op<T>; 3] = [
    (base::OPS[0], T::parse_suite),
    (base::OPS[1], T::char_ranges),
    (base::OPS[2], T::format),
  ];
  for (op, f) in ops {
    let mut group = c.benchmark_group(op);
    group.bench_with_input(BenchmarkId::from_parameter(T::NAME), &(), |b, _| {
      b.iter(|| black_box(f(&mut t)));
    });
    group.finish();
  }
}

fn bench_all(c: &mut Criterion) {
  run::<V8Target>(c);
  #[cfg(feature = "bench-regex-syntax")]
  run::<base::RegexSyntaxTarget>(c);
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
