//! v8-base/src/random_number_generator.rs 集成测试
//! — 1:1 对应 utils/random-number-generator-unittest.cc
//!
//! 原版用例 (TEST_P 九种子 INT_MIN/-1/0/1/42/100/1234567890/987654321/INT_MAX):
//! NextIntWithMaxValue / NextBooleanReturnsFalseOrTrue /
//! NextDoubleReturnsValueBetween0And1 / NextSample{0,1,Max,Half,MoreThanHalf,
//! LessThanHalf} + Slow 变体 / NextSampleSlowExcluded{,Max1,Max2} /
//! NextSampleInvalidParam / NextSampleSlowInvalidParam{1,2}。
//! 差异登记: 原版 ASSERT_DEATH_IF_SUPPORTED (CHECK abort) → Rust 禁 panic,
//! 断言等价契约改为 Err 返回 (规约: 非法入参必须失败, 不得静默产出)。
//! kMaxRuns = 12345 守恒。

use std::collections::HashSet;

use aok::{OK, Void};
use v8_base::random_number_generator::{RandomNumberGenerator, murmur_hash3};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// kMaxRuns (原版 12345)
const MAX_RUNS: usize = 12345;

/// INSTANTIATE_TEST_SUITE_P 九种子
const SEEDS: [i64; 9] = [
  i32::MIN as i64,
  -1,
  0,
  1,
  42,
  100,
  1234567890,
  987654321,
  i32::MAX as i64,
];

/// CheckSample: 尺寸 / 互异 / 全部 < max
fn check_sample(mut sample: Vec<u64>, max: u64, size: usize) {
  assert_eq!(sample.len(), size);
  sample.sort_unstable();
  assert!(!sample.windows(2).any(|w| w[0] == w[1]), "值须互异");
  assert!(sample.iter().all(|&x| x < max));
}

/// CheckSlowSample: 附加排除集断言
fn check_slow_sample(sample: Vec<u64>, max: u64, size: usize, excluded: &HashSet<u64>) {
  check_sample(sample.clone(), max, size);
  assert!(sample.iter().all(|x| !excluded.contains(x)));
}

/// TestNextSample
fn test_next_sample(rng: &mut RandomNumberGenerator, max: u64, size: usize, slow: bool) {
  let sample = if slow {
    rng.next_sample_slow(max, size, &HashSet::new()).unwrap()
  } else {
    rng.next_sample(max, size).unwrap()
  };
  check_sample(sample, max, size);
}

/// NextIntWithMaxValue (九种子)
#[test]
fn next_int_with_max_value() -> Void {
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for max in 1..=MAX_RUNS as i32 {
      let n = rng.next_bounded_int(max);
      assert!(n >= 0);
      assert!(n < max);
    }
  }
  OK
}

/// NextBooleanReturnsFalseOrTrue
#[test]
fn next_boolean_returns_false_or_true() -> Void {
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      let _b = rng.next_bool(); // bool 类型本身保证 false|true
    }
  }
  OK
}

/// NextDoubleReturnsValueBetween0And1
#[test]
fn next_double_returns_value_between_0_and_1() -> Void {
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      let d = rng.next_double();
      assert!((0.0..1.0).contains(&d));
    }
  }
  OK
}

/// NextSampleInvalidParam (原版 ASSERT_DEATH → Err 契约)
#[test]
fn next_sample_invalid_param() -> Void {
  let mut rng = RandomNumberGenerator::with_seed(123);
  assert!(rng.next_sample(10, 11).is_err());
  OK
}

/// NextSampleSlowInvalidParam1
#[test]
fn next_sample_slow_invalid_param_1() -> Void {
  let mut rng = RandomNumberGenerator::with_seed(123);
  assert!(rng.next_sample_slow(10, 11, &HashSet::new()).is_err());
  OK
}

/// NextSampleSlowInvalidParam2
#[test]
fn next_sample_slow_invalid_param_2() -> Void {
  let mut rng = RandomNumberGenerator::with_seed(123);
  let excluded: HashSet<u64> = [0, 2, 3].into_iter().collect();
  assert!(rng.next_sample_slow(5, 3, &excluded).is_err());
  OK
}

/// NextSample0 + NextSampleSlow0
#[test]
fn next_sample_0() -> Void {
  let m = 1;
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    test_next_sample(&mut rng, m, 0, false);
    test_next_sample(&mut rng, m, 0, true);
  }
  OK
}

/// NextSample1 + NextSampleSlow1
#[test]
fn next_sample_1() -> Void {
  let m = 10;
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      test_next_sample(&mut rng, m, 1, false);
    }
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      test_next_sample(&mut rng, m, 1, true);
    }
  }
  OK
}

/// NextSampleMax + NextSampleSlowMax
#[test]
fn next_sample_max() -> Void {
  let m = 10;
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      test_next_sample(&mut rng, m, m as usize, false);
    }
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      test_next_sample(&mut rng, m, m as usize, true);
    }
  }
  OK
}

/// NextSampleHalf + NextSampleSlowHalf
#[test]
fn next_sample_half() -> Void {
  let (n, m) = (5_usize, 10_u64);
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      test_next_sample(&mut rng, m, n, false);
    }
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      test_next_sample(&mut rng, m, n, true);
    }
  }
  OK
}

/// NextSampleMoreThanHalf + NextSampleSlowMoreThanHalf
#[test]
fn next_sample_more_than_half() -> Void {
  let (n, m) = (90_usize, 100_u64);
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      test_next_sample(&mut rng, m, n, false);
    }
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      test_next_sample(&mut rng, m, n, true);
    }
  }
  OK
}

/// NextSampleLessThanHalf + NextSampleSlowLessThanHalf
#[test]
fn next_sample_less_than_half() -> Void {
  let (n, m) = (10_usize, 100_u64);
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      test_next_sample(&mut rng, m, n, false);
    }
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      test_next_sample(&mut rng, m, n, true);
    }
  }
  OK
}

/// NextSampleSlowExcluded
#[test]
fn next_sample_slow_excluded() -> Void {
  let (n, m) = (2_usize, 10_u64);
  let excluded: HashSet<u64> = [2, 4, 5, 9].into_iter().collect();
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      let sample = rng.next_sample_slow(m, n, &excluded).unwrap();
      check_slow_sample(sample, m, n, &excluded);
    }
  }
  OK
}

/// NextSampleSlowExcludedMax1
#[test]
fn next_sample_slow_excluded_max_1() -> Void {
  let (n, m) = (1_usize, 5_u64);
  let excluded: HashSet<u64> = [0, 2, 3, 4].into_iter().collect();
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      let sample = rng.next_sample_slow(m, n, &excluded).unwrap();
      check_slow_sample(sample, m, n, &excluded);
    }
  }
  OK
}

/// NextSampleSlowExcludedMax2
#[test]
fn next_sample_slow_excluded_max_2() -> Void {
  let (n, m) = (7_usize, 10_u64);
  let excluded: HashSet<u64> = [0, 4, 8].into_iter().collect();
  for seed in SEEDS {
    let mut rng = RandomNumberGenerator::with_seed(seed);
    for _ in 0..MAX_RUNS {
      let sample = rng.next_sample_slow(m, n, &excluded).unwrap();
      check_slow_sample(sample, m, n, &excluded);
    }
  }
  OK
}

/// 算法一致性补充: 同 seed 同流 (类注释红线) + MurmurHash3(0) = 0 金标 +
/// xorshift128+ 直接公式金标 (seed 链首步)
#[test]
fn seed_stream_determinism_and_golden() -> Void {
  let mut a = RandomNumberGenerator::with_seed(42);
  let mut b = RandomNumberGenerator::with_seed(42);
  for _ in 0..1000 {
    assert_eq!(a.next_int64(), b.next_int64());
  }
  // MurmurHash3(0) = 0 (双射不动点)
  assert_eq!(murmur_hash3(0), 0);
  // with_seed(0): state0 = MH3(0) = 0; state1 = MH3(!0)
  let mut r = RandomNumberGenerator::with_seed(0);
  // 手工步进 xorshift128+ 与 next_int64 比对
  let (mut s0, mut s1) = (0_u64, murmur_hash3(!0_u64));
  for _ in 0..100 {
    let expect = RandomNumberGenerator::xor_shift128(&mut s0, &mut s1) as i64;
    assert_eq!(r.next_int64(), expect);
  }
  // ToDouble 金标: 0 → 0.0; 2^53-1 → (2^53-1)/2^53
  assert_eq!(RandomNumberGenerator::to_double(0), 0.0);
  // to_double(u64::MAX) = (2^53-1)/2^53 = 1 - 2^-53: 指数 1022, 尾数全 1, 位模式精确
  assert_eq!(
    RandomNumberGenerator::to_double(u64::MAX),
    f64::from_bits((1022_u64 << 52) | ((1_u64 << 52) - 1))
  );
  // 次小步进: to_double(1<<11) = 1/2^53 = 2^-53 (精确幂, 指数 970)
  assert_eq!(
    RandomNumberGenerator::to_double(1_u64 << 11),
    f64::from_bits(970_u64 << 52)
  );
  // initial_seed 保留
  assert_eq!(
    RandomNumberGenerator::with_seed(-98765).initial_seed(),
    -98765
  );
  OK
}
