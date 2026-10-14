//! 评测 id 格式 `{NAME}.{op}.{bits}` 或 `{NAME}.{op}.{radix}.{bits}` (以 `.` 分隔便于报告脚本解析)
//! div/rem 样本: 被除数 = 当前位宽, 除数 = 一半位宽 (>=2 digit, 覆盖 schoolbook/Burnikel 边界)

#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use humansize::{BINARY, format_size};

#[path = "base.rs"]
mod base;
#[cfg(feature = "bench-num-bigint")]
use base::NumBigint;
use base::{Base, K_BIT_SIZES, K_RADIXES, V8, radix_string, sample};

/// 对单个实现库跑全部指标
fn run_impl<B: Base>(c: &mut Criterion) {
  let mut imp = B::new();
  for bits in K_BIT_SIZES {
    let x = sample(bits, 1);
    let y = sample(bits, 2);
    let (a, b) = (imp.parse_digits(&x), imp.parse_digits(&y));
    let id = |op: &str| format!("{}.{op}.{bits}", B::NAME);
    c.bench_function(&id("add"), |bencher| {
      bencher.iter(|| black_box(imp.add(&a, &b)))
    });
    c.bench_function(&id("sub"), |bencher| {
      bencher.iter(|| black_box(imp.sub(&a, &b)))
    });
    c.bench_function(&id("mul"), |bencher| {
      bencher.iter(|| black_box(imp.mul(&a, &b)))
    });
    // 除法样本: 除数取一半位宽 (至少 2 digit), 64 bit 档无有效多位除法, 跳过
    if bits >= 256 {
      let p = imp.parse_digits(&sample(bits, 3));
      let q = imp.parse_digits(&sample(bits / 2, 4));
      c.bench_function(&format!("{}.div.{bits}", B::NAME), |bencher| {
        bencher.iter(|| black_box(imp.div(&p, &q)));
      });
      c.bench_function(&format!("{}.rem.{bits}", B::NAME), |bencher| {
        bencher.iter(|| black_box(imp.rem(&p, &q)));
      });
    }
    for radix in K_RADIXES {
      let s = radix_string(&x, radix);
      c.bench_function(
        &format!("{}.to_string.{radix}.{bits}", B::NAME),
        |bencher| {
          bencher.iter(|| black_box(imp.to_string(&a, radix)));
        },
      );
      c.bench_function(
        &format!("{}.from_string.{radix}.{bits}", B::NAME),
        |bencher| {
          bencher.iter(|| black_box(imp.parse_string(&s, radix)));
        },
      );
    }
  }
}

fn bench_all(c: &mut Criterion) {
  let max_bytes = K_BIT_SIZES.into_iter().max().unwrap_or(0) as u64 / 8;
  println!(
    "v8-bigint 评测: 最大操作数 {}",
    format_size(max_bytes, BINARY)
  );
  run_impl::<V8>(c);
  #[cfg(feature = "bench-num-bigint")]
  run_impl::<NumBigint>(c);
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
