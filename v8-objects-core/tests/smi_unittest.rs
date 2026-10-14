use aok::{OK, Void};
use v8_objects_core::{
  smi::{LargeSmi, SmallSmi, Smi},
  tagged::{Object, Tagged},
  tagged_constants::smi31,
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn test_small_smi_corner_cases() -> Void {
  // 原版 v8/test/unittests/api/smi-tagging-unittest.cc: SmallSmi 边界测试
  const INT31_MAX: i32 = smi31::MAX_VALUE;
  const INT31_MIN: i32 = smi31::MIN_VALUE;

  assert!(SmallSmi::is_valid_i64(0));

  // int8_t, uint8_t
  assert!(SmallSmi::is_valid_i64(i8::MAX as i64));
  assert!(SmallSmi::is_valid_i64(i8::MIN as i64));
  assert!(SmallSmi::is_valid_u64(u8::MAX as u64));
  assert!(SmallSmi::is_valid_u64(u8::MIN as u64));

  // int16_t, uint16_t
  assert!(SmallSmi::is_valid_i64(i16::MAX as i64));
  assert!(SmallSmi::is_valid_i64(i16::MIN as i64));
  assert!(SmallSmi::is_valid_u64(u16::MAX as u64));
  assert!(SmallSmi::is_valid_u64(u16::MIN as u64));

  // int31 边界
  assert!(SmallSmi::is_valid_i64(INT31_MAX as i64));
  assert!(SmallSmi::is_valid_i64(INT31_MIN as i64));
  assert!(SmallSmi::is_valid_u64(INT31_MAX as u64));

  // 超出 int31 范围
  assert!(!SmallSmi::is_valid_i64(INT31_MAX as i64 + 1));
  assert!(!SmallSmi::is_valid_i64(INT31_MIN as i64 - 1));
  assert!(!SmallSmi::is_valid_u64(INT31_MAX as u64 + 1));

  // 32 位极值在 31 位 Smi 中非法
  assert!(!SmallSmi::is_valid_i64(i32::MAX as i64));
  assert!(!SmallSmi::is_valid_i64(i32::MIN as i64));
  assert!(!SmallSmi::is_valid_u64(u32::MAX as u64));

  // 64 位极值非法
  assert!(!SmallSmi::is_valid_i64(i64::MAX));
  assert!(!SmallSmi::is_valid_i64(i64::MIN));
  assert!(!SmallSmi::is_valid_u64(u64::MAX));

  OK
}

#[test]
fn test_large_smi_corner_cases() -> Void {
  // 原版 v8/test/unittests/api/smi-tagging-unittest.cc: LargeSmi 边界测试
  const INT31_MAX: i32 = smi31::MAX_VALUE;
  const INT31_MIN: i32 = smi31::MIN_VALUE;

  assert!(LargeSmi::is_valid_i64(0));

  // int8_t, uint8_t
  assert!(LargeSmi::is_valid_i64(i8::MAX as i64));
  assert!(LargeSmi::is_valid_i64(i8::MIN as i64));
  assert!(LargeSmi::is_valid_u64(u8::MAX as u64));
  assert!(LargeSmi::is_valid_u64(u8::MIN as u64));

  // int16_t, uint16_t
  assert!(LargeSmi::is_valid_i64(i16::MAX as i64));
  assert!(LargeSmi::is_valid_i64(i16::MIN as i64));
  assert!(LargeSmi::is_valid_u64(u16::MAX as u64));
  assert!(LargeSmi::is_valid_u64(u16::MIN as u64));

  // int31 及其外沿全部有效
  assert!(LargeSmi::is_valid_i64(INT31_MAX as i64));
  assert!(LargeSmi::is_valid_i64(INT31_MIN as i64));
  assert!(LargeSmi::is_valid_u64(INT31_MAX as u64));
  assert!(LargeSmi::is_valid_i64(INT31_MAX as i64 + 1));
  assert!(LargeSmi::is_valid_i64(INT31_MIN as i64 - 1));
  assert!(LargeSmi::is_valid_u64(INT31_MAX as u64 + 1));

  // int32 边界均有效
  assert!(LargeSmi::is_valid_i64(i32::MAX as i64));
  assert!(LargeSmi::is_valid_i64(i32::MIN as i64));
  assert!(!LargeSmi::is_valid_u64(u32::MAX as u64));

  // 64 位极值溢出
  assert!(!LargeSmi::is_valid_i64(i64::MAX));
  assert!(!LargeSmi::is_valid_i64(i64::MIN));
  assert!(!LargeSmi::is_valid_u64(u64::MAX));

  OK
}

#[test]
fn test_smi_encoding_and_sign_extension() -> Void {
  // 原版 v8/test/unittests/objects/casting-unittest.cc: Smi 编码与符号位恢复
  let test_cases = [
    0,
    1,
    -1,
    42,
    -42,
    1024,
    -1024,
    Smi::MIN_VALUE,
    Smi::MAX_VALUE,
    Smi::MIN_VALUE_31,
    Smi::MAX_VALUE_31,
  ];

  for &val in &test_cases {
    let smi = Smi::from_int(val);

    // 检查 Smi Tag 最低位为 0
    assert!(smi.is_smi());
    assert!(!smi.is_heap_object());
    assert!(!smi.is_weak());

    // 符号位扩展解码还原
    assert_eq!(smi.value(), val);
    assert_eq!(Smi::value(smi), val);

    // 安全解包测试
    let unpacked = smi.to_smi();
    assert_eq!(unpacked, Some(smi));
    assert_eq!(smi.to_heap_object(), None);
  }

  // 验证 zero 辅助构造器
  let zero = Smi::zero();
  assert!(zero.is_smi());
  assert_eq!(zero.value(), 0);

  OK
}

#[test]
fn test_smi_31bit_pattern_conversion() -> Void {
  // 原版 Smi::From31BitPattern / To31BitPattern 互逆测试。
  // 64 位非压缩模型 (kSmiValueSize=32) 下 From31BitPattern 为恒等:
  // 输入域 [0, 2^31-1] 直接编码, 低位 31 位不变、MSB 不截断。
  for val in [0, 1, 100, 0x3FFF_FFFF, 0x7FFF_FFFF] {
    let smi = Smi::from_31bit_pattern(val);
    assert_eq!(Smi::to_31bit_pattern(smi), val as u32);
    assert_eq!(smi.value(), val, "恒等: 输入 {val} 应原样编码");
  }
  // 恒等语义边界: MSB 置位的最大输入不被符号截断 (smi.h:83-88, 移位 0 位)
  assert_eq!(Smi::from_31bit_pattern(0x7FFF_FFFF).value(), i32::MAX);

  // 显式 31 位压缩模式 API 的对偶 (from_int31/value_31): 符号位在 bit30
  assert_eq!(Smi::value_31(Smi::from_int31(0x3FFF_FFFF)), 0x3FFF_FFFF);
  assert_eq!(Smi::value_31(Smi::from_int31(-1)), -1);

  // ToUint32Smi 转换
  let pos = Smi::from_int(50);
  assert_eq!(Smi::to_uint32_smi(pos).value(), 50);

  let neg = Smi::from_int(-50);
  assert_eq!(Smi::to_uint32_smi(neg).value(), 0);

  OK
}

#[test]
fn test_smi_conversion_helpers() -> Void {
  // 原版 smi.h: ToInt/ToUInt/FromUInt/uninitialized_deserialization_value
  let smi = Smi::from_int(-123);
  let obj: Tagged<Object> = smi.cast();

  assert_eq!(Smi::to_int(obj), -123);
  assert_eq!(Smi::to_uint(Smi::from_int(777).cast()), 777);

  // FromUInt: [0, kSmiMaxValue] → Smi
  assert_eq!(Smi::from_uint(4242).value(), 4242);
  assert_eq!(Smi::from_uint(0).value(), 0);
  assert_eq!(
    Smi::from_uint(Smi::MAX_VALUE as u32).value(),
    Smi::MAX_VALUE
  );

  // 未初始化哨兵 = kNullAddress (读作 nullptr 的 0 值)
  assert_eq!(Smi::uninitialized_deserialization_value().ptr(), 0);

  OK
}

/// 参考实现: 十进制字符串的字典序比较 (对应 cctest ExpectedCompareResult)
fn expected_compare(a: i32, b: i32) -> i32 {
  let sa = a.to_string();
  let sb = b.to_string();
  if sa == sb {
    0
  } else if sa < sb {
    -1
  } else {
    1
  }
}

#[test]
fn test_smi_lexicographic_compare() -> Void {
  // 原版 cctest/test-smi-lexicographic-compare.cc: 全对组合与字符串序对照
  let mut smis: Vec<i32> = Vec::new();
  let add_signed = |x: i64, smis: &mut Vec<i32>| {
    if !Smi::is_valid(x) {
      return;
    }
    smis.push(x as i32);
    smis.push((-x) as i32);
  };

  // 10 的幂 × 尾数 × 邻域
  let mut xb: i64 = 1;
  while xb <= Smi::MAX_VALUE as i64 {
    for xf in 0..=9 {
      for xo in -1..=1 {
        add_signed(xb * xf + xo, &mut smis);
      }
    }
    xb *= 10;
  }
  // 2 的幂 × 邻域
  let mut yb: i64 = 1;
  while yb <= Smi::MAX_VALUE as i64 {
    for yo in -2..=2 {
      add_signed(yb + yo, &mut smis);
    }
    yb *= 2;
  }

  // 全对组合比对
  for &a in &smis {
    for &b in &smis {
      let actual = Smi::lexicographic_compare(Smi::from_int(a), Smi::from_int(b)).value();
      let expected = expected_compare(a, b);
      assert_eq!(actual, expected, "compare({a}, {b})");
    }
  }

  OK
}
