//! fast_hash 金标准散列值对账 — 逐例断言具体数值, 锁定折叠方向与权重
//! v8: src/compiler/turboshaft/fast-hash.h 分发表 + 17 倍混合 (变参/两参/range 三种路径)

use aok::{OK, Void};
use log::info;
use v8_turboshaft::{
  FastHash, fast_hash_combine, fast_hash_combine2, fast_hash_pair, fast_hash_range,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 金标准: 变参 fast_hash_combine 权重式 Σ 17^i * hash(v_i), 首元素权重 1
/// 原版递归 C(v0..v_{k-1}) = 17 * C(v1..v_{k-1}) + hash(v0) 展开, 手算具体数值
#[test]
fn fast_hash_combine_golden_weights() -> Void {
  info!("> fast_hash_combine 权重式具体数值金标准");
  // 空集: combine() = 0 (原版 fast_hash_combine() return 0u)
  assert_eq!(fast_hash_combine(&[]), 0);
  // 单元素: combine(v) 模板路径 = combine(0, hash(v)) = 17*0 + hv = hv
  assert_eq!(fast_hash_combine(&[9]), 9);
  // 2 元素: hv0 + 17*hv1 — 3 + 17*5 = 3 + 85 = 88
  assert_eq!(fast_hash_combine(&[3, 5]), 88);
  // 3 元素: hv0 + 17*hv1 + 289*hv2 — 3 + 85 + 2023 = 2111
  assert_eq!(fast_hash_combine(&[3, 5, 7]), 2111);
  // 4 元素: + 4913*hv3 — 2111 + 4913*11 = 2111 + 54043 = 56154
  assert_eq!(fast_hash_combine(&[3, 5, 7, 11]), 56154);
  // 顺序敏感 (首末元素权重不同): [7,5,3] vs [3,5,7] 不等
  assert_eq!(fast_hash_combine(&[7, 5, 3]), 7 + 17 * 5 + 289 * 3);
  assert_ne!(fast_hash_combine(&[3, 5, 7]), fast_hash_combine(&[7, 5, 3]));
  OK
}

/// 金标准: 两参 combine2 = 17 * acc + value (原版非模板重载)
/// 关键鉴别: combine2(a,b) != combine(&[a,b]) 当 a!=b — 折叠方向不同
#[test]
fn fast_hash_combine2_golden_vs_combine_fold() -> Void {
  info!("> combine2 vs combine 折叠方向金标准鉴别");
  // 17 * acc + value
  assert_eq!(fast_hash_combine2(0, 7), 7);
  assert_eq!(fast_hash_combine2(7, 0), 119); // 17*7 + 0
  assert_eq!(fast_hash_combine2(7, 3), 122); // 17*7 + 3
  assert_eq!(fast_hash_combine2(3, 7), 58); // 17*3 + 7
  // combine 首权重 1, 逐位 17 后移 → combine(&[a,b]) = a + 17*b
  assert_eq!(fast_hash_combine(&[7, 3]), 7 + 17 * 3); // 58
  // combine2(a,b) 与 combine(&[a,b]) 交叉等式 (锁定 fold 方向不反向)
  assert_eq!(fast_hash_combine2(7, 3), fast_hash_combine(&[3, 7])); // 122
  assert_eq!(fast_hash_combine2(3, 7), fast_hash_combine(&[7, 3])); // 58
  OK
}

/// 金标准: fast_hash_range 左折叠 combine2(acc, *v) 逐元素
/// 关键鉴别: range 与变参 combine 折叠方向相反, 同输入结果不同
#[test]
fn fast_hash_range_golden_left_fold() -> Void {
  info!("> fast_hash_range 左折叠金标准");
  // 空 → 0
  assert_eq!(fast_hash_range(&[]), 0);
  // 单元素: combine2(0, v) = v
  assert_eq!(fast_hash_range(&[42]), 42);
  // 1,2: acc=0 → 17*0+1=1 → 17*1+2 = 19
  assert_eq!(fast_hash_range(&[1, 2]), 19);
  // 1,2,3: → 17*19+3 = 326
  assert_eq!(fast_hash_range(&[1, 2, 3]), 326);
  // 3,5: range = 17*3 + 5 = 56; combine = 3 + 17*5 = 88 — 严格不等
  assert_eq!(fast_hash_range(&[3, 5]), 56);
  assert_eq!(fast_hash_combine(&[3, 5]), 88);
  assert_ne!(fast_hash_range(&[3, 5]), fast_hash_combine(&[3, 5]));
  // 4 元素 1,2,3,4: 17*326+4 = 5546
  assert_eq!(fast_hash_range(&[1, 2, 3, 4]), 5546);
  OK
}

/// 金标准: pair = combine(&[first, second]) 走模板变参路径
/// 交叉对: pair(a,b) = a + 17*b = combine2(b,a) — 用于对合检验
#[test]
fn fast_hash_pair_golden() -> Void {
  info!("> fast_hash_pair 金标准");
  assert_eq!(fast_hash_pair(3, 5), 3 + 17 * 5); // 88
  assert_eq!(fast_hash_pair(5, 3), 5 + 17 * 3); // 56
  assert_eq!(fast_hash_pair(0, 0), 0);
  assert_eq!(fast_hash_pair(1, 0), 1);
  assert_eq!(fast_hash_pair(0, 1), 17);
  // 与 combine2 交叉等式 (锁定 first/second 顺序)
  assert_eq!(fast_hash_pair(3, 5), fast_hash_combine2(5, 3));
  assert_eq!(fast_hash_pair(7, 3), fast_hash_combine2(3, 7));
  OK
}

/// 金标准: 分发表直传与 optional 位移
/// u64/u32/usize 恒等, Option(v) = (hash(v) << 1) + 1, None = 0
#[test]
fn fast_hash_dispatch_golden() -> Void {
  info!("> fast_hash 分发表恒等与 optional 位移金标准");
  // u64 恒等 (主模板 size_t 直传)
  assert_eq!(0u64.fast_hash(), 0);
  assert_eq!(1u64.fast_hash(), 1);
  assert_eq!(12345u64.fast_hash(), 12345);
  assert_eq!(u64::MAX.fast_hash(), u64::MAX);
  // u32 上转 u64 (零扩展)
  assert_eq!(0u32.fast_hash(), 0);
  assert_eq!(1u32.fast_hash(), 1);
  assert_eq!(u32::MAX.fast_hash(), 0xFFFF_FFFFu64);
  // usize 走 size_t 路径
  assert_eq!(1usize.fast_hash(), 1u64);
  assert_eq!(usize::MAX.fast_hash(), usize::MAX as u64);
  // Option<u64>: 原版 has_value ? (fast_hash(*v) << 1) + 1 : 0
  assert_eq!(None::<u64>.fast_hash(), 0);
  assert_eq!(Some(0u64).fast_hash(), 1);
  assert_eq!(Some(1u64).fast_hash(), 3); // (1<<1)+1
  assert_eq!(Some(7u64).fast_hash(), 15); // (7<<1)+1
  assert_eq!(
    Some(u32::MAX as u64).fast_hash(),
    (((u32::MAX as u64) << 1) + 1)
  );
  OK
}
