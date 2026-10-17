// v8: test/unittests/codegen/register-configuration-unittest.cc 1:1 移植 (BasicProperties / CombineAliasing)

use std::ptr::eq;

use aok::{OK, Void};
use v8_codegen_core::{
  AliasingKind, K_MAX_FP_REGISTERS, MachineRepresentation, RegisterConfiguration,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const K_FLOAT32: u8 = MachineRepresentation::Float32 as u8;
const K_FLOAT64: u8 = MachineRepresentation::Float64 as u8;
const K_SIMD128: u8 = MachineRepresentation::Simd128 as u8;

/// 原版 RegisterConfigurationUnitTest.BasicProperties:
/// Overlap 类别下各数量/掩码/分配码访问器逐项核对
#[test]
fn basic_properties() -> Void {
  let k_num_general_regs = 3i32;
  let k_num_double_regs = 4i32;
  let k_n_alloc_general = 2usize;
  let k_n_alloc_double = 2usize;
  let general_codes = [1i32, 2];
  let double_codes = [2i32, 3];

  let test = RegisterConfiguration::new(
    AliasingKind::Overlap,
    k_num_general_regs,
    k_num_double_regs,
    0,
    0,
    k_n_alloc_general,
    k_n_alloc_double,
    0,
    0,
    &general_codes,
    &double_codes,
    None,
  );

  assert_eq!(test.num_general_registers(), k_num_general_regs);
  assert_eq!(test.num_double_registers(), k_num_double_regs);
  assert_eq!(
    test.num_allocatable_general_registers() as usize,
    k_n_alloc_general
  );
  assert_eq!(
    test.num_allocatable_double_registers() as usize,
    k_n_alloc_double
  );
  assert_eq!(
    test.num_allocatable_float_registers(),
    test.num_allocatable_double_registers()
  );
  assert_eq!(
    test.num_allocatable_simd128_registers(),
    test.num_allocatable_double_registers()
  );
  // v8: #if V8_TARGET_ARCH_X64 断言面 (本机 arm64 时跳过, 与原版构建期分支等价)
  if cfg!(target_arch = "x86_64") {
    assert_eq!(
      test.num_allocatable_simd256_registers(),
      test.num_allocatable_double_registers()
    );
  }

  assert_eq!(
    test.allocatable_general_codes_mask(),
    (1 << general_codes[0]) | (1 << general_codes[1])
  );
  assert_eq!(test.get_allocatable_general_code(0), general_codes[0]);
  assert_eq!(test.get_allocatable_general_code(1), general_codes[1]);
  assert_eq!(
    test.allocatable_double_codes_mask(),
    (1 << double_codes[0]) | (1 << double_codes[1])
  );
  assert_eq!(test.get_allocatable_float_code(0), double_codes[0]);
  assert_eq!(test.get_allocatable_double_code(0), double_codes[0]);
  assert_eq!(test.get_allocatable_simd128_code(0), double_codes[0]);
  assert_eq!(test.get_allocatable_float_code(1), double_codes[1]);
  assert_eq!(test.get_allocatable_double_code(1), double_codes[1]);
  assert_eq!(test.get_allocatable_simd128_code(1), double_codes[1]);

  OK
}

/// 原版 RegisterConfigurationUnitTest.CombineAliasing:
/// Combine 类别下 float/simd128 码合成与 AreAliases/GetAliases 全断言面
#[allow(clippy::similar_names)]
#[test]
fn combine_aliasing() -> Void {
  let general_codes = [1i32, 2];
  // reg 16 不应别名到不存在的 float 32/33 (原版注释)
  let double_codes = [2i32, 3, 16];

  let test = RegisterConfiguration::new(
    AliasingKind::Combine,
    3,
    4,
    0,
    0,
    2,
    double_codes.len(),
    0,
    0,
    &general_codes,
    &double_codes,
    None,
  );

  // 3 个可分配 double, 但仅 2 个可与 float 成对别名 → 4 个可分配 float
  assert_eq!(test.num_allocatable_float_registers(), 4);
  // float 成对合成自 double
  assert_eq!(test.get_allocatable_float_code(0), double_codes[0] * 2);
  assert_eq!(test.get_allocatable_float_code(1), double_codes[0] * 2 + 1);
  assert_eq!(test.get_allocatable_float_code(2), double_codes[1] * 2);
  assert_eq!(test.get_allocatable_float_code(3), double_codes[1] * 2 + 1);
  // 3 个可分配 double, 但仅 2 个可合成 1 个 SIMD128
  assert_eq!(test.num_allocatable_simd128_registers(), 1);
  assert_eq!(test.get_allocatable_simd128_code(0), double_codes[0] / 2);

  // 寄存器与自身别名
  assert!(test.are_aliases(K_FLOAT32, 0, K_FLOAT32, 0));
  assert!(test.are_aliases(K_FLOAT64, 0, K_FLOAT64, 0));
  assert!(test.are_aliases(K_SIMD128, 0, K_SIMD128, 0));
  // 同尺寸互不别名
  assert!(!test.are_aliases(K_FLOAT32, 1, K_FLOAT32, 0));
  assert!(!test.are_aliases(K_FLOAT64, 1, K_FLOAT64, 0));
  assert!(!test.are_aliases(K_SIMD128, 1, K_SIMD128, 0));
  // float 两两合成别名 double(index/2), 四个合成别名 simd128(index/4)
  assert!(test.are_aliases(K_FLOAT32, 0, K_FLOAT64, 0));
  assert!(test.are_aliases(K_FLOAT32, 1, K_FLOAT64, 0));
  assert!(test.are_aliases(K_FLOAT32, 0, K_SIMD128, 0));
  assert!(test.are_aliases(K_FLOAT32, 1, K_SIMD128, 0));
  assert!(test.are_aliases(K_FLOAT32, 2, K_SIMD128, 0));
  assert!(test.are_aliases(K_FLOAT32, 3, K_SIMD128, 0));
  assert!(test.are_aliases(K_FLOAT64, 0, K_FLOAT32, 0));
  assert!(test.are_aliases(K_FLOAT64, 0, K_FLOAT32, 1));
  assert!(test.are_aliases(K_SIMD128, 0, K_FLOAT32, 0));
  assert!(test.are_aliases(K_SIMD128, 0, K_FLOAT32, 1));
  assert!(test.are_aliases(K_SIMD128, 0, K_FLOAT32, 2));
  assert!(test.are_aliases(K_SIMD128, 0, K_FLOAT32, 3));

  assert!(!test.are_aliases(K_FLOAT32, 0, K_FLOAT64, 1));
  assert!(!test.are_aliases(K_FLOAT32, 1, K_FLOAT64, 1));
  assert!(!test.are_aliases(K_FLOAT32, 0, K_SIMD128, 1));
  assert!(!test.are_aliases(K_FLOAT32, 1, K_SIMD128, 1));
  assert!(!test.are_aliases(K_FLOAT64, 0, K_SIMD128, 1));
  assert!(!test.are_aliases(K_FLOAT64, 1, K_SIMD128, 1));

  assert!(test.are_aliases(K_FLOAT64, 0, K_FLOAT32, 1));
  assert!(test.are_aliases(K_FLOAT64, 1, K_FLOAT32, 2));
  assert!(test.are_aliases(K_FLOAT64, 1, K_FLOAT32, 3));
  assert!(test.are_aliases(K_FLOAT64, 2, K_FLOAT32, 4));
  assert!(test.are_aliases(K_FLOAT64, 2, K_FLOAT32, 5));

  assert!(test.are_aliases(K_SIMD128, 0, K_FLOAT64, 1));
  // v8: 原版 kMaxSimd128Registers 场景下 1/2 号 simd128 面; 本基线 Combine 派生
  //     simd128 可分配数为 1, 高位 simd128 索引仍按表示序运算核对别名关系
  assert!(test.are_aliases(K_SIMD128, 1, K_FLOAT64, 2));
  assert!(test.are_aliases(K_SIMD128, 1, K_FLOAT64, 3));
  assert!(test.are_aliases(K_SIMD128, 2, K_FLOAT64, 4));
  assert!(test.are_aliases(K_SIMD128, 2, K_FLOAT64, 5));

  let mut alias_base_index = -1i32;
  assert_eq!(
    test.get_aliases(K_FLOAT32, 0, K_FLOAT32, &mut alias_base_index),
    1
  );
  assert_eq!(alias_base_index, 0);
  assert_eq!(
    test.get_aliases(K_FLOAT64, 1, K_FLOAT64, &mut alias_base_index),
    1
  );
  assert_eq!(alias_base_index, 1);
  assert_eq!(
    test.get_aliases(K_FLOAT32, 0, K_FLOAT64, &mut alias_base_index),
    1
  );
  assert_eq!(alias_base_index, 0);
  assert_eq!(
    test.get_aliases(K_FLOAT32, 1, K_FLOAT64, &mut alias_base_index),
    1
  );
  assert_eq!(
    test.get_aliases(K_FLOAT32, 2, K_FLOAT64, &mut alias_base_index),
    1
  );
  assert_eq!(alias_base_index, 1);
  assert_eq!(
    test.get_aliases(K_FLOAT32, 3, K_FLOAT64, &mut alias_base_index),
    1
  );
  assert_eq!(alias_base_index, 1);
  assert_eq!(
    test.get_aliases(K_FLOAT64, 0, K_FLOAT32, &mut alias_base_index),
    2
  );
  assert_eq!(alias_base_index, 0);
  assert_eq!(
    test.get_aliases(K_FLOAT64, 1, K_FLOAT32, &mut alias_base_index),
    2
  );
  assert_eq!(alias_base_index, 2);

  // 非可分配码仍别名
  assert_eq!(
    test.get_aliases(K_FLOAT64, 2, K_FLOAT32, &mut alias_base_index),
    2
  );
  assert_eq!(alias_base_index, 4);
  // 高位 double 不别名不存在的 float 寄存器
  assert_eq!(
    test.get_aliases(
      K_FLOAT64,
      K_MAX_FP_REGISTERS / 2,
      K_FLOAT32,
      &mut alias_base_index
    ),
    0
  );
  assert_eq!(
    test.get_aliases(
      K_FLOAT64,
      K_MAX_FP_REGISTERS / 2 + 1,
      K_FLOAT32,
      &mut alias_base_index
    ),
    0
  );
  assert_eq!(
    test.get_aliases(
      K_FLOAT64,
      K_MAX_FP_REGISTERS - 1,
      K_FLOAT32,
      &mut alias_base_index
    ),
    0
  );

  OK
}

/// 默认配置、沙箱保留配置与受限通用寄存器配置测试
#[test]
fn default_and_restricted_configurations() -> Void {
  let default_cfg = RegisterConfiguration::default_configuration();
  assert!(default_cfg.num_general_registers() > 0);
  assert!(default_cfg.num_allocatable_general_registers() > 0);

  let poisoning_cfg = RegisterConfiguration::poisoning();
  assert!(poisoning_cfg.num_general_registers() > 0);

  // 全掩码应直通默认配置单例
  let restricted_all = RegisterConfiguration::restrict_general_registers(u64::MAX);
  assert_eq!(
    restricted_all.num_allocatable_general_registers(),
    default_cfg.num_allocatable_general_registers()
  );

  // 部分掩码过滤 (仅保留前两个可分配通用寄存器)
  let code0 = default_cfg.get_allocatable_general_code(0);
  let code1 = default_cfg.get_allocatable_general_code(1);
  let mask = (1u64 << code0) | (1u64 << code1);
  let restricted = RegisterConfiguration::restrict_general_registers(mask);
  assert_eq!(restricted.num_allocatable_general_registers(), 2);
  assert_eq!(restricted.get_allocatable_general_code(0), code0);
  assert_eq!(restricted.get_allocatable_general_code(1), code1);

  // 缓存命中验证：相同掩码返回同一静态单例引用
  let cached = RegisterConfiguration::restrict_general_registers(mask);
  assert!(eq(restricted, cached));

  // 空掩码
  let restricted_empty = RegisterConfiguration::restrict_general_registers(0);
  assert_eq!(restricted_empty.num_allocatable_general_registers(), 0);

  OK
}
