//! v8-common criterion 主评测入口 — 基于 BenchOp trait 泛型驱动
//!
//! 运行: `cargo bench -p v8-common --features bench`
//! 输出: `<CARGO_TARGET_DIR>/criterion/<实现名>/<操作>/…` (estimates.json 等)

#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

use std::env::var;

use fastrand::Rng;
mod base;

use std::{hint::black_box, path::PathBuf, time::Duration};

#[cfg(feature = "bench-gxhash")]
use base::Gxhash;
use base::{BenchOp, CAGE_BASE, Std, V8Common};
use criterion::Criterion;
use v8_common::V8HeapCompressionScheme;

/// 输入规模 — 标签/压缩/对齐池 + 散列建表/探针池
const POOL: usize = 4096;
const HASH_KEYS: usize = 2048;
const HASH_PROBES: usize = 512;

/// 各操作的输入池
struct Inputs {
  tags: Vec<usize>,
  ptrs: Vec<usize>,
  aligns: Vec<usize>,
  keys: Vec<usize>,
  probes: Vec<usize>,
}

/// 固定种子生成确定性输入 (fastrand: 测试构造数据约定)
fn gen_inputs() -> Inputs {
  let mut rng = Rng::with_seed(0xBE4C_2026);
  // tag 输入: smi(偶)/heap(&3==1)/weak(&3==3) 三段混合
  let tags: Vec<usize> = (0..POOL)
    .map(|i| (rng.u64(..0x1_0000_0000_0000) as usize) & !3 | (i % 4))
    .collect();
  // 压缩输入: cage 内带 heap tag 指针 + 偶数 Smi (满足 compress_object 校验域)
  let ptrs: Vec<usize> = (0..POOL)
    .map(|i| CAGE_BASE | ((rng.u32(..) as usize & !3) | (i % 2)))
    .collect();
  // 对齐输入: 未对齐小尺寸地址
  let aligns: Vec<usize> = (0..POOL).map(|_| rng.usize(0..1 << 20)).collect();
  // 散列建表键: 4KB 步长、带 heap tag 的 cage 内地址 (object_hash = 右移 2)
  let keys: Vec<usize> = (0..HASH_KEYS)
    .map(|i| CAGE_BASE + (i + 1) * 0x1000 + 1)
    .collect();
  // 探针: 半命中 (原键) 半缺失 (键 +4, 散列槽不同)
  let probes: Vec<usize> = (0..HASH_PROBES)
    .map(|i| {
      if i % 2 == 0 {
        keys[i % HASH_KEYS]
      } else {
        keys[i % HASH_KEYS] + 4
      }
    })
    .collect();
  Inputs {
    tags,
    ptrs,
    aligns,
    keys,
    probes,
  }
}

/// 泛型驱动: 一个实现 = 一个 benchmark_group, 评测 id = NAME/操作名
fn bench_impl<T: BenchOp>(c: &mut Criterion, op: &T, inputs: &Inputs) {
  let mut g = c.benchmark_group(T::NAME);
  g.bench_function("tag_classify", |b| {
    b.iter(|| op.tag_classify(black_box(&inputs.tags)))
  });
  g.bench_function("compress_roundtrip", |b| {
    b.iter(|| op.compress_roundtrip(black_box(&inputs.ptrs)))
  });
  g.bench_function("align_ops", |b| {
    b.iter(|| op.align_ops(black_box(&inputs.aligns)))
  });
  g.bench_function("hash_probe", |b| {
    b.iter(|| op.hash_probe(black_box(&inputs.keys), black_box(&inputs.probes)))
  });
  g.finish();
}

fn main() {
  // 进程级 cage base 初始化 (原版 InitBase, 幂等; bench 单进程无竞争)
  let _ = V8HeapCompressionScheme::init_base(CAGE_BASE);
  let inputs = gen_inputs();

  // 输入池占用一次性打印 (humansize), 非计时路径
  let pool_bytes = POOL * size_of::<usize>() * 3 + (HASH_KEYS + HASH_PROBES) * size_of::<usize>();
  println!(
    "bench inputs pool: {}",
    humansize::format_size(pool_bytes, humansize::BINARY)
  );

  // criterion 产物落 CARGO_TARGET_DIR/criterion, 与并发隔离目录一致
  let target = var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
  let out_dir = PathBuf::from(target).join("criterion");
  let mut c = Criterion::default()
    .sample_size(20)
    .warm_up_time(Duration::from_millis(200))
    .measurement_time(Duration::from_millis(500))
    .output_directory(&out_dir)
    .configure_from_args();

  bench_impl(&mut c, &V8Common, &inputs);
  bench_impl(&mut c, &Std, &inputs);
  #[cfg(feature = "bench-gxhash")]
  bench_impl(&mut c, &Gxhash, &inputs);
  c.final_summary();
}
