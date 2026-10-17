// main.rs — criterion 主评测入口, 基于 DateBench trait 泛型驱动
// 运行: CARGO_TARGET_DIR=target/bench-date cargo bench -p v8-date --features bench

#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

mod base;

use std::hint::black_box;

use base::{DateBench, PARSE_SAMPLES, TIME_SAMPLES};
use criterion::Criterion;

/// 对单个实现注册全部操作评测, 评测 id 基于 T::NAME 生成
fn bench_impl<T: DateBench>(c: &mut Criterion, target: &mut T) {
  let mut group = c.benchmark_group(T::NAME);

  if T::HAS_PARSE {
    group.bench_function("parse_date_time", |b| {
      let mut sink: u64 = 0;
      b.iter(|| {
        for s in PARSE_SAMPLES {
          let ms = target.parse_date_time(black_box(s));
          sink = sink.wrapping_add(black_box(ms).to_bits());
        }
        black_box(&sink);
      });
    });
  }

  group.bench_function("epoch_ms_to_ymd", |b| {
    let mut sink: u64 = 0;
    b.iter(|| {
      for t in TIME_SAMPLES {
        let (y, m, d) = target.epoch_ms_to_ymd(black_box(*t));
        sink = sink
          .wrapping_add(y as u64)
          .wrapping_add((m as u64) << 8)
          .wrapping_add((d as u64) << 16);
      }
      black_box(&sink);
    });
  });

  group.bench_function("epoch_ms_to_time_of_day", |b| {
    let mut sink: u64 = 0;
    b.iter(|| {
      for t in TIME_SAMPLES {
        let (h, mi, s, ms) = target.epoch_ms_to_time_of_day(black_box(*t));
        sink = sink
          .wrapping_add(h as u64)
          .wrapping_add((mi as u64) << 8)
          .wrapping_add((s as u64) << 16)
          .wrapping_add((ms as u64) << 24);
      }
      black_box(&sink);
    });
  });

  group.finish();
}

fn bench_all(c: &mut Criterion) {
  bench_impl(c, &mut base::V8DateBench::default());
  #[cfg(feature = "bench-chrono")]
  bench_impl(c, &mut base::ChronoDateBench);
  #[cfg(feature = "bench-std")]
  bench_impl(c, &mut base::StdSystemTimeBench);
}

criterion::criterion_group!(benches, bench_all);
criterion::criterion_main!(benches);
