//! criterion 主评测入口 (genbench 步骤 3): 基于 AstBench trait 泛型驱动

#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

mod base;

use core::hint::black_box;

use base::{ALLOC_BATCH, AstBench, TREE_PARAMS, V8AstBench};
use criterion::{Criterion, criterion_group, criterion_main};

/// 对单个实现跑全套评测: 分配 / 建树 / 遍历 / 布局检查
fn bench_impl<I: AstBench>(c: &mut Criterion, imp: &I) {
  let name = I::NAME;

  c.bench_function(&format!("{name}/alloc_nodes"), |b| {
    b.iter(|| black_box(imp.alloc_nodes(black_box(ALLOC_BATCH))));
  });

  for (depth, branch) in TREE_PARAMS {
    let tag = format!("{depth}x{branch}");

    let id = format!("{name}/build_tree/{tag}");
    c.bench_function(&id, |b| {
      b.iter_with_large_drop(|| black_box(imp.build_tree(black_box(depth), black_box(branch))));
    });

    let tree = imp.build_tree(depth, branch);

    let id = format!("{name}/visit_tree/{tag}");
    c.bench_function(&id, |b| {
      b.iter(|| black_box(imp.visit_tree(black_box(&tree))));
    });

    let id = format!("{name}/layout_check/{tag}");
    c.bench_function(&id, |b| {
      b.iter(|| black_box(imp.layout_check(black_box(&tree))));
    });
  }
}

fn benchmark(c: &mut Criterion) {
  bench_impl(c, &V8AstBench);
  #[cfg(feature = "bench-dyn-box")]
  bench_impl(c, &base::DynBoxBench);
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
