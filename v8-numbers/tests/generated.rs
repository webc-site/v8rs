//! 5 个新实现的 generated 模块回归与覆盖测试

use aok::{OK, Void};
use v8_numbers::{
  conversions::inl_generated::{conv_consts, conversions_inl_skel},
  hash_seed_inl_generated::hash_seed_inl_skel,
  ieee754_generated::{PowBranch, pow_skel},
  integer_literal_inl_generated::{IntegerLiteralSkel, integer_literal_inl_skel},
  math_random_generated::{MathRandomMachine, NativeContextRandomState, math_random_skel},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_conversions_inl_basics() -> Void {
  // fast_d2ui 转换与界
  assert_eq!(0, conversions_inl_skel::fast_d2ui(0.0));
  assert_eq!(1234, conversions_inl_skel::fast_d2ui(1234.0));
  assert_eq!(
    conv_consts::INTEGER_INDEFINITE,
    conversions_inl_skel::fast_d2ui(conv_consts::TWO_POW_52)
  );
  assert_eq!(
    conv_consts::INTEGER_INDEFINITE,
    conversions_inl_skel::fast_d2ui(f64::INFINITY)
  );
  assert_eq!(
    conv_consts::INTEGER_INDEFINITE,
    conversions_inl_skel::fast_d2ui(f64::NAN)
  );

  // fast_d2i
  assert_eq!(42, conversions_inl_skel::fast_d2i(42.0));
  assert_eq!(-100, conversions_inl_skel::fast_d2i(-100.0));

  // double_to_float16
  let f16_zero = conversions_inl_skel::double_to_float16(0.0);
  assert_eq!(0, f16_zero);
  let f16_inf = conversions_inl_skel::double_to_float16(f64::INFINITY);
  assert_eq!(conv_consts::FP16_INFINITY, f16_inf);
  let f16_nan = conversions_inl_skel::double_to_float16(f64::NAN);
  assert_eq!(conv_consts::FP16_Q_NAN, f16_nan);

  // double_to_float32
  assert_eq!(0.0f32, conversions_inl_skel::double_to_float32(0.0));
  assert_eq!(42.5f32, conversions_inl_skel::double_to_float32(42.5));
  assert_eq!(f32::INFINITY, conversions_inl_skel::double_to_float32(1e39));
  assert_eq!(
    f32::NEG_INFINITY,
    conversions_inl_skel::double_to_float32(-1e39)
  );

  // double_to_integer
  assert_eq!(0.0, conversions_inl_skel::double_to_integer(f64::NAN));
  assert_eq!(0.0, conversions_inl_skel::double_to_integer(-0.0));
  assert!(!conversions_inl_skel::double_to_integer(-0.0).is_sign_negative());
  assert_eq!(3.0, conversions_inl_skel::double_to_integer(3.7));
  assert_eq!(-3.0, conversions_inl_skel::double_to_integer(-3.7));

  // double_to_int32 & double_to_uint32
  assert_eq!(123, conversions_inl_skel::double_to_int32(123.0));
  assert_eq!(-123, conversions_inl_skel::double_to_int32(-123.0));
  assert_eq!(123, conversions_inl_skel::double_to_uint32(123.0));
  assert_eq!(0xFFFF_FF85, conversions_inl_skel::double_to_uint32(-123.0));

  // WebIDL 64
  assert_eq!(
    9_007_199_254_740_991,
    conversions_inl_skel::double_to_webidl_int64(9_007_199_254_740_991.0)
  );
  assert_eq!(
    9_007_199_254_740_991,
    conversions_inl_skel::double_to_webidl_uint64(9_007_199_254_740_991.0)
  );

  // Smi / Int32 / Uint32 判定与提取
  assert!(conversions_inl_skel::is_smi_double(100.0));
  assert!(!conversions_inl_skel::is_smi_double(-0.0));
  assert!(!conversions_inl_skel::is_smi_double(1e15));
  assert_eq!(
    Some(100),
    conversions_inl_skel::double_to_smi_integer(100.0)
  );
  assert!(conversions_inl_skel::is_int32_double(100.0));
  assert!(conversions_inl_skel::is_uint32_double(100.0));
  assert!(!conversions_inl_skel::is_uint32_double(-100.0));

  // double_to_uint32_if_equal_to_self
  assert_eq!(
    Some(12345),
    conversions_inl_skel::double_to_uint32_if_equal_to_self(12345.0)
  );
  assert_eq!(
    None,
    conversions_inl_skel::double_to_uint32_if_equal_to_self(12345.5)
  );

  // double_to_string & string_to_double
  assert_eq!("42", conversions_inl_skel::double_to_string(42.0));
  assert_eq!("-3.25", conversions_inl_skel::double_to_string(-3.25));
  assert_eq!(42.0, conversions_inl_skel::string_to_double("42"));
  assert_eq!(-3.25, conversions_inl_skel::string_to_double("-3.25"));

  // 零拷贝 double_to_string_view
  let mut str_buf = [0u8; 100];
  let view = conversions_inl_skel::double_to_string_view(123.456, &mut str_buf);
  assert_eq!("123.456", view);

  // is_minus_zero
  assert!(conversions_inl_skel::is_minus_zero(-0.0));
  assert!(!conversions_inl_skel::is_minus_zero(0.0));
  assert!(!conversions_inl_skel::is_minus_zero(1.0));

  // try_double_to_size
  assert_eq!(Some(100), conversions_inl_skel::try_double_to_size(100.0));
  assert_eq!(None, conversions_inl_skel::try_double_to_size(-1.0));
  assert_eq!(None, conversions_inl_skel::try_double_to_size(f64::NAN));

  OK
}

#[test]
fn test_hash_seed_inl_mixing() -> Void {
  let default_ptr = hash_seed_inl_skel::default_seed();
  assert_ne!(0, default_ptr);

  assert_eq!(0, hash_seed_inl_skel::seed(default_ptr));
  assert_eq!(0, hash_seed_inl_skel::seed(0));
  let sec = hash_seed_inl_skel::secret(default_ptr);
  assert!(!sec.is_null());
  assert_eq!(0x2d35_8dcc_aa6c_78a5, unsafe { *sec });

  // isolate / local_isolate / read_only_roots 委托
  assert_eq!(default_ptr, hash_seed_inl_skel::from_isolate(default_ptr));
  assert_eq!(
    default_ptr,
    hash_seed_inl_skel::from_local_isolate(default_ptr)
  );
  assert_eq!(
    default_ptr,
    hash_seed_inl_skel::from_read_only_roots(default_ptr)
  );
  assert_eq!(default_ptr, hash_seed_inl_skel::from_read_only_roots(0));

  // 乘数与模逆
  let m1 = hash_seed_inl_skel::m1(default_ptr);
  let m1_inv = hash_seed_inl_skel::m1_inv(default_ptr);
  assert_eq!(1, m1 & 1);
  assert_eq!(1, (m1.wrapping_mul(m1_inv)) & 0x00FF_FFFF);

  let m2 = hash_seed_inl_skel::m2(default_ptr);
  let m2_inv = hash_seed_inl_skel::m2_inv(default_ptr);
  assert_eq!(1, m2 & 1);
  assert_eq!(1, (m2.wrapping_mul(m2_inv)) & 0x00FF_FFFF);

  let m3 = hash_seed_inl_skel::m3(default_ptr);
  let m3_inv = hash_seed_inl_skel::m3_inv(default_ptr);
  assert_eq!(1, m3 & 1);
  assert_eq!(1, (m3.wrapping_mul(m3_inv)) & 0x00FF_FFFF);

  // 数组索引哈希加盐与解盐互逆性检验 (双向 100% 还原)
  let test_indices = [
    0u32,
    1,
    2,
    7,
    42,
    100,
    1024,
    65535,
    123456,
    0x00AB_CDEF,
    0x00FF_FFFF,
  ];
  for &idx in &test_indices {
    let seeded = hash_seed_inl_skel::seed_array_index_value(idx, default_ptr);
    assert!(seeded <= 0x00FF_FFFF);
    let unseeded = hash_seed_inl_skel::unseed_array_index_value(seeded, default_ptr);
    assert_eq!(idx, unseeded, "index={idx} roundtrip mismatch");
  }

  // secrets 安全切片借用
  let sec_slice = hash_seed_inl_skel::secrets(default_ptr);
  assert_eq!(0x2d35_8dcc_aa6c_78a5, sec_slice[0]);

  // HashSeed 结构体包装视图与单向/逆向运算一致性
  let hs = hash_seed_inl_skel::HashSeed::default();
  assert_eq!(0, hs.seed());
  assert_eq!(0x2d35_8dcc_aa6c_78a5, hs.secrets()[0]);
  assert_eq!(hs.m1(), hash_seed_inl_skel::m1(default_ptr));
  assert_eq!(hs.m2(), hash_seed_inl_skel::m2(default_ptr));
  assert_eq!(hs.m3(), hash_seed_inl_skel::m3(default_ptr));
  assert_eq!(
    12345,
    hs.unseed_array_index_value(hs.seed_array_index_value(12345))
  );

  OK
}

#[test]
fn test_math_random_machine() -> Void {
  // 单步 xorshift128+
  let mut s0 = 0x1234_5678_9ABC_DEF0;
  let mut s1 = 0xFEDC_BA98_7654_3210;
  let val1 = math_random_skel::xorshift128(&mut s0, &mut s1);
  let val2 = math_random_skel::xorshift128(&mut s0, &mut s1);
  assert_ne!(val1, val2);

  // next_double 在 [0, 1) 区间内
  let d = math_random_skel::next_double(&mut s0, &mut s1);
  assert!((0.0..1.0).contains(&d));

  // MathRandomMachine 状态机与重置
  let mut rng = MathRandomMachine::new();
  assert!(rng.state.is_zero());
  let r1 = rng.next_double();
  assert!((0.0..1.0).contains(&r1));
  assert!(!rng.state.is_zero());

  rng.reset();
  assert!(rng.state.is_zero());
  assert_eq!(0, rng.index);

  // Context 模拟结构
  let mut ctx = NativeContextRandomState::new();
  let ctx_ptr = &mut ctx as *mut NativeContextRandomState as usize;
  math_random_skel::initialize_context(1, ctx_ptr);
  assert!(ctx.state.is_zero());

  math_random_skel::initialize_and_maybe_refill_cache(12345, ctx_ptr);
  assert!(!ctx.state.is_zero());

  math_random_skel::reset_context(ctx_ptr);
  assert!(ctx.state.is_zero());
  assert_eq!(0, ctx.index);

  // 缓存填充与消费 (32位构型路径 / 独立状态机)
  let mut cached_machine = MathRandomMachine::new();
  assert_eq!(0, cached_machine.index);
  let val_c1 = cached_machine.next_cached_double();
  assert!((0.0..1.0).contains(&val_c1));
  assert_eq!(63, cached_machine.index);
  let val_c2 = cached_machine.next_cached_double();
  assert!((0.0..1.0).contains(&val_c2));
  assert_eq!(62, cached_machine.index);

  cached_machine.refill_cache();
  assert_eq!(64, cached_machine.index);

  OK
}

#[test]
fn test_integer_literal_inl_skel() -> Void {
  // 零归一
  let z1 = IntegerLiteralSkel::new(true, 0);
  assert!(!z1.is_negative());
  assert_eq!(0, z1.absolute_value());

  let pos = IntegerLiteralSkel::new(false, 42);
  let neg = IntegerLiteralSkel::new(true, 42);

  // to_string & Display
  assert_eq!("42", integer_literal_inl_skel::to_string(&pos));
  assert_eq!("-42", integer_literal_inl_skel::to_string(&neg));
  assert_eq!("42", format!("{pos}"));
  assert_eq!("-42", format!("{neg}"));

  // eq & compare
  assert!(integer_literal_inl_skel::eq(&pos, &pos));
  assert!(!integer_literal_inl_skel::eq(&pos, &neg));
  assert!(pos > neg);
  assert!(neg < pos);

  // shl
  let shifted = integer_literal_inl_skel::shl(&pos, &IntegerLiteralSkel::from(2i32));
  assert_eq!(168, shifted.absolute_value());
  assert!(!shifted.is_negative());

  // add: 同号 / 异号 / 归零
  let a = IntegerLiteralSkel::from(100i64);
  let b = IntegerLiteralSkel::from(200i64);
  assert_eq!(
    IntegerLiteralSkel::from(300i64),
    integer_literal_inl_skel::add(&a, &b)
  );

  let neg_b = IntegerLiteralSkel::from(-200i64);
  assert_eq!(
    IntegerLiteralSkel::from(-100i64),
    integer_literal_inl_skel::add(&a, &neg_b)
  );

  let neg_a = IntegerLiteralSkel::from(-100i64);
  let zero_sum = integer_literal_inl_skel::add(&a, &neg_a);
  assert_eq!(0, zero_sum.absolute_value());
  assert!(!zero_sum.is_negative());

  // bit_or
  let x = IntegerLiteralSkel::from(0b1010u64);
  let y = IntegerLiteralSkel::from(0b0101u64);
  assert_eq!(
    IntegerLiteralSkel::from(0b1111u64),
    integer_literal_inl_skel::bit_or(&x, &y)
  );

  // 表示范围与转换
  assert!(pos.is_representable_as_i32());
  assert!(pos.is_representable_as_u32());
  assert!(pos.is_representable_as_i64());
  assert!(pos.is_representable_as_u64());
  assert_eq!(Some(42), pos.try_to_i32());
  assert_eq!(Some(42), pos.try_to_u32());
  assert_eq!(Some(42), pos.try_to_i64());
  assert_eq!(Some(42), pos.try_to_u64());

  assert!(neg.is_representable_as_i32());
  assert!(!neg.is_representable_as_u32());
  assert_eq!(Some(-42), neg.try_to_i32());
  assert_eq!(None, neg.try_to_u32());

  let over_i32 = IntegerLiteralSkel::from(3_000_000_000u64);
  assert!(!over_i32.is_representable_as_i32());
  assert!(over_i32.is_representable_as_u32());
  assert_eq!(None, over_i32.try_to_i32());
  assert_eq!(Some(3_000_000_000), over_i32.try_to_u32());

  // 运算符重载语法糖 (+, <<, |)
  assert_eq!(IntegerLiteralSkel::from(300i64), a + b);
  assert_eq!(
    IntegerLiteralSkel::from(168i64),
    pos << IntegerLiteralSkel::from(2i32)
  );
  assert_eq!(IntegerLiteralSkel::from(0b1111u64), x | y);

  // 边界极限值严格断言
  let i32_min_exact = IntegerLiteralSkel::new(true, 1u64 << 31);
  assert!(i32_min_exact.is_representable_as_i32());
  assert_eq!(Some(i32::MIN), i32_min_exact.try_to_i32());

  let i32_min_overflow = IntegerLiteralSkel::new(true, (1u64 << 31) + 1);
  assert!(!i32_min_overflow.is_representable_as_i32());
  assert_eq!(None, i32_min_overflow.try_to_i32());

  let i64_min_exact = IntegerLiteralSkel::new(true, 1u64 << 63);
  assert!(i64_min_exact.is_representable_as_i64());
  assert_eq!(Some(i64::MIN), i64_min_exact.try_to_i64());

  let i64_max_exact = IntegerLiteralSkel::new(false, i64::MAX as u64);
  assert!(i64_max_exact.is_representable_as_i64());
  assert_eq!(Some(i64::MAX), i64_max_exact.try_to_i64());

  OK
}

#[test]
fn test_ieee754_pow_skel() -> Void {
  // 分支分类与 ECMAScript 历史规范 (±1, ±∞) = NaN
  let (res1, branch1) = pow_skel::pow_with_branch(1.0, f64::INFINITY);
  assert!(res1.is_nan());
  assert_eq!(PowBranch::BaseOneInfiniteExponent, branch1);

  let (res2, branch2) = pow_skel::pow_with_branch(-1.0, f64::NEG_INFINITY);
  assert!(res2.is_nan());
  assert_eq!(PowBranch::BaseOneInfiniteExponent, branch2);

  // 指数 NaN
  let (res_nan, branch_nan) = pow_skel::pow_with_branch(2.0, f64::NAN);
  assert!(res_nan.is_nan());
  assert_eq!(PowBranch::ExponentNaN, branch_nan);

  // y == 2.0 (Square)
  let (sq, b_sq) = pow_skel::pow_with_branch(5.0, 2.0);
  assert_eq!(25.0, sq);
  assert_eq!(PowBranch::Square, b_sq);

  // y == 0.5 (Sqrt)
  let (sqrt_val, b_sqrt) = pow_skel::pow_with_branch(16.0, 0.5);
  assert_eq!(4.0, sqrt_val);
  assert_eq!(PowBranch::Sqrt, b_sqrt);

  // -0 ** 0.5 必须为 +0
  let (zero_sqrt, _) = pow_skel::pow_with_branch(-0.0, 0.5);
  assert_eq!(0.0, zero_sqrt);
  assert!(!pow_skel::is_minus_zero(zero_sqrt));

  // -Inf ** 0.5 必须为 +Inf
  let (inf_sqrt, _) = pow_skel::pow_with_branch(f64::NEG_INFINITY, 0.5);
  assert_eq!(f64::INFINITY, inf_sqrt);

  // 正常乘方
  let (p, b_p) = pow_skel::pow_with_branch(2.0, 10.0);
  assert_eq!(1024.0, p);
  assert_eq!(PowBranch::LegacyPow, b_p);

  // 底数 NaN (BaseNaN 分支)
  let (res_base_nan, branch_base_nan) = pow_skel::pow_with_branch(f64::NAN, 3.0);
  assert!(res_base_nan.is_nan());
  assert_eq!(PowBranch::BaseNaN, branch_base_nan);

  // quiet_nan 发生器与负零判定
  assert!(pow_skel::quiet_nan().is_nan());
  assert!(pow_skel::is_minus_zero(-0.0));
  assert!(!pow_skel::is_minus_zero(0.0));

  OK
}
