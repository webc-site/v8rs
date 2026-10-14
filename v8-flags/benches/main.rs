// criterion 主评测入口: 基于 base::FlagBench trait 泛型驱动
// 每个 (操作 × 实现) 一个 criterion 组/函数, 评测 id 的 function 段取 T::NAME

#[global_allocator]
static ALLOC: rusty_alloc_api::RustyAlloc = rusty_alloc_api::RustyAlloc;

use v8_flags::FlagType;
mod base;

use std::{hint::black_box, time::Duration};

#[cfg(feature = "bench-hashmap")]
use base::StdHashMap;
use base::{FlagBench, V8rsPhf};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use humansize::{BINARY, format_size};

/// 单次迭代内的查找批量
const BATCH: usize = 64;

/// 查找命中样本: 生成表前 BATCH 个主 flag 名 (归一化形态)
fn hit_names() -> Vec<String> {
  v8_flags::FLAGS
    .iter()
    .map(|d| d.name.replace('_', "-"))
    .take(BATCH)
    .collect()
}

/// 查找未命中样本: 必然不存在的构造名
fn miss_names() -> Vec<String> {
  (0..BATCH)
    .map(|i| format!("bench-missing-flag-{i}"))
    .collect()
}

/// 解析用例 (组名, 输入串): 覆盖 --bool / --no-a / --a=b / 字符串值 /
/// 多 flag 空格分隔 / 未知 flag 错误路径
const PARSE_CASES: &[(&str, &str)] = &[
  ("parse_bool", "--use-strict"),
  ("parse_neg", "--no-expose-gc"),
  ("parse_kv", "--stack-size=1234"),
  ("parse_str_kv", "--icu-datetime-compat-lang=en-US"),
  (
    "parse_multi",
    "--lite-mode --no-trace-temporal --gc-interval=100 --max-old-space-size=2048",
  ),
  ("parse_unknown", "--not-a-real-v8-flag"),
];

/// 批量 set_from_string 用例: 前 BATCH 个 bool 主 flag 一次性设置
fn batch_input() -> String {
  v8_flags::FLAGS
    .iter()
    .filter(|d| matches!(d.ty, FlagType::Bool))
    .take(BATCH)
    .map(|d| format!("--{}", d.name.replace('_', "-")))
    .collect::<Vec<_>>()
    .join(" ")
}

/// 按名查找 (命中 + 未命中), 每迭代批量 BATCH 次
fn bench_lookup<T: FlagBench>(c: &mut Criterion) {
  let t = T::default();
  let hits = black_box(hit_names());
  let misses = black_box(miss_names());

  let mut g = c.benchmark_group("lookup_hit");
  g.bench_function(T::NAME, |b| {
    b.iter(|| {
      let mut acc = 0u64;
      for n in &hits {
        acc += t.lookup(n).is_some() as u64;
      }
      black_box(acc)
    })
  });
  g.finish();

  let mut g = c.benchmark_group("lookup_miss");
  g.bench_function(T::NAME, |b| {
    b.iter(|| {
      let mut acc = 0u64;
      for n in &misses {
        acc += t.lookup(n).is_some() as u64;
      }
      black_box(acc)
    })
  });
  g.finish();
}

/// flag 字符串解析各用例; 每迭代新建实现实例 (fresh 状态, 不计构建成本轮换)
fn bench_parse<T: FlagBench>(c: &mut Criterion) {
  for (group, input) in PARSE_CASES {
    let mut g = c.benchmark_group(*group);
    g.bench_function(T::NAME, |b| {
      b.iter_batched(
        T::default,
        |mut t| black_box(t.parse(input)).is_ok(),
        BatchSize::SmallInput,
      )
    });
    g.finish();
  }
}

/// 批量 set_from_string (一次 64 个 bool flag)
fn bench_batch_set<T: FlagBench>(c: &mut Criterion) {
  let input = black_box(batch_input());
  let mut g = c.benchmark_group("batch_set");
  g.bench_function(T::NAME, |b| {
    b.iter_batched(
      T::default,
      |mut t| black_box(t.parse(&input)).is_ok(),
      BatchSize::SmallInput,
    )
  });
  g.finish();
}

/// 默认值重置: 先弄脏再测 reset
fn bench_reset<T: FlagBench>(c: &mut Criterion) {
  let dirty = "--use-strict --stack-size=999 --no-expose-gc".to_string();
  let mut g = c.benchmark_group("reset_default");
  g.bench_function(T::NAME, |b| {
    b.iter_batched(
      || {
        let mut t = T::default();
        let _ = t.parse(&dirty);
        t
      },
      |mut t| {
        t.reset();
        black_box(&t);
      },
      BatchSize::SmallInput,
    )
  });
  g.finish();
}

/// 对全部已启用实现跑全部操作
fn run_all(c: &mut Criterion) {
  #[cfg(feature = "bench-hashmap")]
  let impls = [V8rsPhf::NAME, StdHashMap::NAME];
  #[cfg(not(feature = "bench-hashmap"))]
  let impls = [V8rsPhf::NAME];
  let input_bytes: usize =
    PARSE_CASES.iter().map(|(_, s)| s.len()).sum::<usize>() + batch_input().len();
  println!(
    "v8-flags bench: impls=[{}] inputs={}",
    impls.join(","),
    format_size(input_bytes as u64, BINARY)
  );

  bench_lookup::<V8rsPhf>(c);
  bench_parse::<V8rsPhf>(c);
  bench_batch_set::<V8rsPhf>(c);
  bench_reset::<V8rsPhf>(c);

  // 仅查找参与对比: 解析/批量/重置无对比库 (todo 10.2 红线, flag 语义自研),
  // std-hashmap 上下文含自建大表, 其 drop 会计入批量例程污染测量, 不跑这些组
  #[cfg(feature = "bench-hashmap")]
  bench_lookup::<StdHashMap>(c);
}

criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(3));
  targets = run_all
}
criterion_main!(benches);
