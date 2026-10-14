// 由 tool/src/codegen/register-configuration.h.js 从 v8/src/codegen/register-configuration.{h,cc} 生成; 修订就地改, 复跑覆盖
// RegisterConfiguration 上限/别名类别/访问器签名骨架

/// 架构无关上限: 通用寄存器 (原版 kMaxGeneralRegisters)
pub const K_MAX_GENERAL_REGISTERS: i32 = 32;
/// 架构无关上限: FP 寄存器 (原版 kMaxFPRegisters)
pub const K_MAX_FP_REGISTERS: i32 = 32;
/// 架构无关上限: 二者取大 (原版 kMaxRegisters)
pub const K_MAX_REGISTERS: i32 = if K_MAX_FP_REGISTERS > K_MAX_GENERAL_REGISTERS {
  K_MAX_FP_REGISTERS
} else {
  K_MAX_GENERAL_REGISTERS
};

/// 原版 enum class AliasingKind (common/globals.h): FP 寄存器别名策略
/// (判别值按原版声明顺序: kOverlap → kCombine → kIndependent)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AliasingKind {
  /// 同尺寸寄存器互别名 (x64 Intel 布局)
  Overlap = 0,
  /// 别名到更小尺寸的一对寄存器 (ARM 布局)
  Combine = 1,
  /// SIMD128 与其他尺寸独立 (Riscv 布局)
  Independent = 2,
}

use std::sync::LazyLock;

use gxhash::HashMap;
use parking_lot::RwLock;

/// 原版 RegisterConfiguration: 寄存器配置抽象基类。
/// 数值字段以原版私有字段名一一对应; 分配码数组以 owned Vec 承载。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterConfiguration {
  pub num_general_registers_: i32,
  pub num_float_registers_: i32,
  pub num_double_registers_: i32,
  pub num_simd128_registers_: i32,
  pub num_simd256_registers_: i32,
  pub num_allocatable_general_registers_: i32,
  pub num_allocatable_float_registers_: i32,
  pub num_allocatable_double_registers_: i32,
  pub num_allocatable_simd128_registers_: i32,
  pub num_allocatable_simd256_registers_: i32,
  pub allocatable_general_codes_mask_: i32,
  pub allocatable_float_codes_mask_: i32,
  pub allocatable_double_codes_mask_: i32,
  pub allocatable_simd128_codes_mask_: i32,
  pub allocatable_simd256_codes_mask_: i32,
  pub allocatable_general_codes_: Vec<i32>,
  pub allocatable_float_codes_: Vec<i32>,
  pub allocatable_double_codes_: Vec<i32>,
  pub allocatable_simd128_codes_: Vec<i32>,
  pub allocatable_simd256_codes_: Vec<i32>,
  pub fp_aliasing_kind_: AliasingKind,
}

/// GetAllocatable*/IsAllocatable* 通用生成宏的机械展开 (5 组同构访问器)
macro_rules! allocatable_accessors {
  ($get:ident, $is:ident, $num:ident, $mask:ident, $codes:ident) => {
    #[inline]
    #[must_use]
    pub fn $get(&self, index: usize) -> i32 {
      debug_assert!(index < self.$num() as usize);
      self.$codes[index]
    }
    #[inline]
    #[must_use]
    pub fn $is(&self, index: usize) -> bool {
      ((1i32 << index) & self.$mask) != 0
    }
  };
}

static DEFAULT_CONFIG: LazyLock<RegisterConfiguration> = LazyLock::new(|| {
  #[cfg(target_arch = "x86_64")]
  {
    // x86_64: 16 通用寄存器, 16 浮点/双精度/SIMD寄存器 (Overlap 布局)
    // 可分配通用: rax=0, rbx=3, rdx=2, rcx=1, rsi=6, rdi=7, r8=8, r9=9, r11=11, r12=12, r15=15
    const GENERAL: [i32; 11] = [0, 3, 2, 1, 6, 7, 8, 9, 11, 12, 15];
    const DOUBLE: [i32; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    RegisterConfiguration::new(
      AliasingKind::Overlap,
      16,
      16,
      0,
      0,
      GENERAL.len(),
      DOUBLE.len(),
      0,
      0,
      &GENERAL,
      &DOUBLE,
      None,
    )
  }

  #[cfg(not(target_arch = "x86_64"))]
  {
    // aarch64 及默认: 32 通用寄存器, 32 浮点/双精度寄存器 (Combine 布局)
    // 可分配通用: x0..x15, x19..x25, x27 (共 24 个)
    const GENERAL: [i32; 24] = [
      0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 19, 20, 21, 22, 23, 24, 25, 27,
    ];
    // 可分配双精度: d0..d14, d16..d28 (共 28 个, 严格递增且成对供 SIMD128 合成)
    const DOUBLE: [i32; 28] = [
      0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
      27, 28,
    ];
    RegisterConfiguration::new(
      AliasingKind::Combine,
      32,
      32,
      0,
      0,
      GENERAL.len(),
      DOUBLE.len(),
      0,
      0,
      &GENERAL,
      &DOUBLE,
      None,
    )
  }
});

static RESTRICTED_CACHE: LazyLock<RwLock<HashMap<u64, &'static RegisterConfiguration>>> =
  LazyLock::new(|| RwLock::new(HashMap::default()));

impl RegisterConfiguration {
  /// 架构无关上限: 通用寄存器
  pub const K_MAX_GENERAL_REGISTERS: i32 = K_MAX_GENERAL_REGISTERS;
  /// 架构无关上限: FP 寄存器
  pub const K_MAX_FP_REGISTERS: i32 = K_MAX_FP_REGISTERS;
  /// 架构无关上限: 二者取大
  pub const K_MAX_REGISTERS: i32 = K_MAX_REGISTERS;

  /// 原版 num_general_registers()
  #[inline]
  #[must_use]
  pub const fn num_general_registers(&self) -> i32 {
    self.num_general_registers_
  }

  /// 原版 num_float_registers()
  #[inline]
  #[must_use]
  pub const fn num_float_registers(&self) -> i32 {
    self.num_float_registers_
  }

  /// 原版 num_double_registers()
  #[inline]
  #[must_use]
  pub const fn num_double_registers(&self) -> i32 {
    self.num_double_registers_
  }

  /// 原版 num_simd128_registers()
  #[inline]
  #[must_use]
  pub const fn num_simd128_registers(&self) -> i32 {
    self.num_simd128_registers_
  }

  /// 原版 num_simd256_registers()
  #[inline]
  #[must_use]
  pub const fn num_simd256_registers(&self) -> i32 {
    self.num_simd256_registers_
  }

  /// 原版 num_allocatable_general_registers()
  #[inline]
  #[must_use]
  pub const fn num_allocatable_general_registers(&self) -> i32 {
    self.num_allocatable_general_registers_
  }

  /// 原版 num_allocatable_float_registers()
  #[inline]
  #[must_use]
  pub const fn num_allocatable_float_registers(&self) -> i32 {
    self.num_allocatable_float_registers_
  }

  /// 原版 num_allocatable_double_registers()
  #[inline]
  #[must_use]
  pub const fn num_allocatable_double_registers(&self) -> i32 {
    self.num_allocatable_double_registers_
  }

  /// 原版 num_allocatable_simd128_registers()
  #[inline]
  #[must_use]
  pub const fn num_allocatable_simd128_registers(&self) -> i32 {
    self.num_allocatable_simd128_registers_
  }

  /// 原版 num_allocatable_simd256_registers()
  #[inline]
  #[must_use]
  pub const fn num_allocatable_simd256_registers(&self) -> i32 {
    self.num_allocatable_simd256_registers_
  }

  /// 原版 fp_aliasing_kind()
  #[inline]
  #[must_use]
  pub const fn fp_aliasing_kind(&self) -> AliasingKind {
    self.fp_aliasing_kind_
  }

  allocatable_accessors!(
    get_allocatable_general_code,
    is_allocatable_general_code,
    num_allocatable_general_registers,
    allocatable_general_codes_mask_,
    allocatable_general_codes_
  );
  allocatable_accessors!(
    get_allocatable_float_code,
    is_allocatable_float_code,
    num_allocatable_float_registers,
    allocatable_float_codes_mask_,
    allocatable_float_codes_
  );
  allocatable_accessors!(
    get_allocatable_double_code,
    is_allocatable_double_code,
    num_allocatable_double_registers,
    allocatable_double_codes_mask_,
    allocatable_double_codes_
  );
  allocatable_accessors!(
    get_allocatable_simd128_code,
    is_allocatable_simd128_code,
    num_allocatable_simd128_registers,
    allocatable_simd128_codes_mask_,
    allocatable_simd128_codes_
  );

  /// 原版 allocatable_general_codes_mask()
  #[inline]
  #[must_use]
  pub const fn allocatable_general_codes_mask(&self) -> i32 {
    self.allocatable_general_codes_mask_
  }

  /// 原版 allocatable_double_codes_mask()
  #[inline]
  #[must_use]
  pub const fn allocatable_double_codes_mask(&self) -> i32 {
    self.allocatable_double_codes_mask_
  }

  /// 原版 allocatable_float_codes_mask()
  #[inline]
  #[must_use]
  pub const fn allocatable_float_codes_mask(&self) -> i32 {
    self.allocatable_float_codes_mask_
  }

  /// 原版 allocatable_simd128_codes_mask()
  #[inline]
  #[must_use]
  pub const fn allocatable_simd128_codes_mask(&self) -> i32 {
    self.allocatable_simd128_codes_mask_
  }

  /// 原版 allocatable_simd256_codes_mask()
  #[inline]
  #[must_use]
  pub const fn allocatable_simd256_codes_mask(&self) -> i32 {
    self.allocatable_simd256_codes_mask_
  }

  /// 原版 allocatable_general_codes()
  #[inline]
  #[must_use]
  pub fn allocatable_general_codes(&self) -> &[i32] {
    &self.allocatable_general_codes_
  }

  /// 原版 allocatable_float_codes()
  #[inline]
  #[must_use]
  pub fn allocatable_float_codes(&self) -> &[i32] {
    &self.allocatable_float_codes_
  }

  /// 原版 allocatable_double_codes()
  #[inline]
  #[must_use]
  pub fn allocatable_double_codes(&self) -> &[i32] {
    &self.allocatable_double_codes_
  }

  /// 原版 allocatable_simd128_codes()
  #[inline]
  #[must_use]
  pub fn allocatable_simd128_codes(&self) -> &[i32] {
    &self.allocatable_simd128_codes_
  }

  /// 原版 allocatable_simd256_codes()
  #[inline]
  #[must_use]
  pub fn allocatable_simd256_codes(&self) -> &[i32] {
    &self.allocatable_simd256_codes_
  }

  /// 测试用配置构造
  #[must_use]
  pub fn new_for_test(fp_aliasing_kind: AliasingKind) -> Self {
    Self {
      num_general_registers_: 16,
      num_float_registers_: 32,
      num_double_registers_: 16,
      num_simd128_registers_: 16,
      num_simd256_registers_: 0,
      num_allocatable_general_registers_: 14,
      num_allocatable_float_registers_: 32,
      num_allocatable_double_registers_: 16,
      num_allocatable_simd128_registers_: 16,
      num_allocatable_simd256_registers_: 0,
      allocatable_general_codes_mask_: 0x3fff,
      allocatable_float_codes_mask_: -1,
      allocatable_double_codes_mask_: 0xffff,
      allocatable_simd128_codes_mask_: 0xffff,
      allocatable_simd256_codes_mask_: 0,
      allocatable_general_codes_: (0..14).collect(),
      allocatable_float_codes_: (0..32).collect(),
      allocatable_double_codes_: (0..16).collect(),
      allocatable_simd128_codes_: (0..16).collect(),
      allocatable_simd256_codes_: Vec::new(),
      fp_aliasing_kind_: fp_aliasing_kind,
    }
  }

  /// 别名: 测试用配置构造
  #[inline]
  #[must_use]
  pub fn new_for_test_with_aliasing(fp_aliasing_kind: AliasingKind) -> Self {
    Self::new_for_test(fp_aliasing_kind)
  }

  /// 原版 register-configuration.cc RegisterConfiguration::RegisterConfiguration:
  /// 由原始参数集与分配码数组构建配置; FP/SIMD128/SIMD256 码数组与掩码按
  /// Combine / Overlap / Independent 三类别名策略推导。
  /// v8: src/codegen/register-configuration.cc:228-323
  #[allow(clippy::too_many_arguments, clippy::similar_names)]
  #[must_use]
  pub fn new(
    fp_aliasing_kind: AliasingKind,
    num_general_registers: i32,
    num_double_registers: i32,
    num_simd128_registers: i32,
    num_simd256_registers: i32,
    num_allocatable_general_registers: usize,
    num_allocatable_double_registers: usize,
    num_allocatable_simd128_registers: usize,
    num_allocatable_simd256_registers: usize,
    allocatable_general_codes: &[i32],
    allocatable_double_codes: &[i32],
    independent_allocatable_simd128_codes: Option<&[i32]>,
  ) -> Self {
    // v8: DCHECK_LE(num_general_registers_, kMaxGeneralRegisters)
    debug_assert!(
      num_general_registers <= K_MAX_GENERAL_REGISTERS,
      "原版 DCHECK_LE(kMaxGeneralRegisters)"
    );
    // v8: DCHECK_LE(num_double_registers_, kMaxFPRegisters)
    debug_assert!(
      num_double_registers <= K_MAX_FP_REGISTERS,
      "原版 DCHECK_LE(kMaxFPRegisters)"
    );
    let mut config = Self {
      num_general_registers_: num_general_registers,
      num_float_registers_: 0,
      num_double_registers_: num_double_registers,
      num_simd128_registers_: num_simd128_registers,
      num_simd256_registers_: num_simd256_registers,
      num_allocatable_general_registers_: num_allocatable_general_registers as i32,
      num_allocatable_float_registers_: 0,
      num_allocatable_double_registers_: num_allocatable_double_registers as i32,
      num_allocatable_simd128_registers_: num_allocatable_simd128_registers as i32,
      num_allocatable_simd256_registers_: num_allocatable_simd256_registers as i32,
      allocatable_general_codes_mask_: 0,
      allocatable_float_codes_mask_: 0,
      allocatable_double_codes_mask_: 0,
      allocatable_simd128_codes_mask_: 0,
      allocatable_simd256_codes_mask_: 0,
      allocatable_general_codes_: allocatable_general_codes[..num_allocatable_general_registers]
        .to_vec(),
      allocatable_float_codes_: Vec::new(),
      allocatable_double_codes_: allocatable_double_codes[..num_allocatable_double_registers]
        .to_vec(),
      allocatable_simd128_codes_: Vec::new(),
      allocatable_simd256_codes_: Vec::new(),
      fp_aliasing_kind_: fp_aliasing_kind,
    };
    // v8: 逐码置通用/双精度掩码 (1 << code; wrapping_shl 规避 bit31 调试溢出)
    for &code in &config.allocatable_general_codes_ {
      config.allocatable_general_codes_mask_ |= 1i32.wrapping_shl(code as u32);
    }
    for &code in &config.allocatable_double_codes_ {
      config.allocatable_double_codes_mask_ |= 1i32.wrapping_shl(code as u32);
    }

    match fp_aliasing_kind {
      AliasingKind::Combine => {
        // v8: register-configuration.cc:263-287 kCombine 支
        config.num_float_registers_ = if num_double_registers * 2 <= K_MAX_FP_REGISTERS {
          num_double_registers * 2
        } else {
          K_MAX_FP_REGISTERS
        };
        config.num_allocatable_float_registers_ = 0;
        for &code in &config.allocatable_double_codes_ {
          let base_code = code * 2;
          if base_code >= K_MAX_FP_REGISTERS {
            continue;
          }
          config.allocatable_float_codes_.push(base_code);
          config.allocatable_float_codes_.push(base_code + 1);
          config.num_allocatable_float_registers_ += 2;
          config.allocatable_float_codes_mask_ |= 0x3i32.wrapping_shl(base_code as u32);
        }
        config.num_simd128_registers_ = num_double_registers / 2;
        config.num_allocatable_simd128_registers_ = 0;
        // v8: 原版假定 allocatable_double_codes 严格递增 (DCHECK_GE)
        if let Some(&first) = config.allocatable_double_codes_.first() {
          let mut last_simd128_code = first / 2;
          for &code in &config.allocatable_double_codes_[1..] {
            let next_simd128_code = code / 2;
            debug_assert!(
              next_simd128_code >= last_simd128_code,
              "原版 DCHECK_GE: 分配码严格递增假设"
            );
            if last_simd128_code == next_simd128_code {
              config.allocatable_simd128_codes_.push(next_simd128_code);
              config.allocatable_simd128_codes_mask_ |= 1i32.wrapping_shl(next_simd128_code as u32);
              config.num_allocatable_simd128_registers_ += 1;
            }
            last_simd128_code = next_simd128_code;
          }
        }
      }
      AliasingKind::Overlap => {
        // v8: register-configuration.cc:288-306 kOverlap 支
        config.num_float_registers_ = num_double_registers;
        config.num_simd128_registers_ = num_double_registers;
        config.num_allocatable_float_registers_ = config.num_allocatable_double_registers_;
        config.num_allocatable_simd128_registers_ = config.num_allocatable_double_registers_;
        for &code in &config.allocatable_double_codes_ {
          config.allocatable_float_codes_.push(code);
          config.allocatable_simd128_codes_.push(code);
          // v8: #if V8_TARGET_ARCH_X64 支的 simd256 码与 double 同码
          if cfg!(target_arch = "x86_64") {
            config.allocatable_simd256_codes_.push(code);
          }
        }
        config.allocatable_float_codes_mask_ = config.allocatable_double_codes_mask_;
        config.allocatable_simd128_codes_mask_ = config.allocatable_double_codes_mask_;
        if cfg!(target_arch = "x86_64") {
          config.num_simd256_registers_ = num_double_registers;
          config.num_allocatable_simd256_registers_ = config.num_allocatable_double_registers_;
          config.allocatable_simd256_codes_mask_ = config.allocatable_double_codes_mask_;
        }
      }
      AliasingKind::Independent => {
        // v8: register-configuration.cc:307-323 kIndependent 支
        debug_assert!(
          independent_allocatable_simd128_codes.is_some(),
          "原版 DCHECK_NE(independent_allocatable_simd128_codes, nullptr)"
        );
        config.num_float_registers_ = num_double_registers;
        config.num_allocatable_float_registers_ = config.num_allocatable_double_registers_;
        for &code in &config.allocatable_double_codes_ {
          config.allocatable_float_codes_.push(code);
        }
        config.allocatable_float_codes_mask_ = config.allocatable_double_codes_mask_;
        if let Some(indep) = independent_allocatable_simd128_codes {
          let n = config.num_allocatable_simd128_registers_ as usize;
          for &code in &indep[..n] {
            config.allocatable_simd128_codes_.push(code);
            config.allocatable_simd128_codes_mask_ |= 1i32.wrapping_shl(code as u32);
          }
        }
      }
    }
    config
  }

  /// 原版 Default(): 目标架构默认配置
  #[must_use]
  pub fn default_configuration() -> &'static RegisterConfiguration {
    &DEFAULT_CONFIG
  }

  /// 原版 Poisoning(): 带保留掩码寄存器的配置 (当前架构回退至默认配置)
  #[must_use]
  pub fn poisoning() -> &'static RegisterConfiguration {
    Self::default_configuration()
  }

  /// 原版 RestrictGeneralRegisters(reglist): 根据寄存器掩码构造过滤后的配置单例
  #[must_use]
  pub fn restrict_general_registers(registers: u64) -> &'static RegisterConfiguration {
    let default_cfg = Self::default_configuration();
    if registers == u64::MAX
      || registers == (default_cfg.allocatable_general_codes_mask_ as u32 as u64)
    {
      return default_cfg;
    }

    {
      let read_lock = RESTRICTED_CACHE.read();
      if let Some(&cfg) = read_lock.get(&registers) {
        return cfg;
      }
    }

    let mut write_lock = RESTRICTED_CACHE.write();
    if let Some(&cfg) = write_lock.get(&registers) {
      return cfg;
    }

    let filtered_general: Vec<i32> = default_cfg
      .allocatable_general_codes_
      .iter()
      .copied()
      .filter(|&code| (0..64).contains(&code) && ((registers & (1u64 << (code as u32))) != 0))
      .collect();

    let cfg = Box::leak(Box::new(RegisterConfiguration::new(
      default_cfg.fp_aliasing_kind_,
      default_cfg.num_general_registers_,
      default_cfg.num_double_registers_,
      default_cfg.num_simd128_registers_,
      default_cfg.num_simd256_registers_,
      filtered_general.len(),
      default_cfg.allocatable_double_codes_.len(),
      0,
      0,
      &filtered_general,
      &default_cfg.allocatable_double_codes_,
      None,
    )));
    write_lock.insert(registers, cfg);
    cfg
  }

  /// 原版 register-configuration.cc GetAliases: kCombine 别名族的 FP 表示序运算。
  /// rep/other_rep 为 MachineRepresentation 判别值 (kFloat32=17 / kFloat64=18 / kSimd128=19)。
  pub fn get_aliases(&self, rep: u8, index: i32, other_rep: u8, alias_base_index: &mut i32) -> i32 {
    debug_assert!(self.fp_aliasing_kind_ == AliasingKind::Combine);
    if rep == other_rep {
      *alias_base_index = index;
      return 1;
    }
    let rep_int = rep as i32;
    let other_rep_int = other_rep as i32;
    if rep_int > other_rep_int {
      let shift = (rep_int - other_rep_int) as u32;
      let base_index = index << shift;
      if base_index >= K_MAX_FP_REGISTERS {
        // 别名索引会越出 FP 寄存器范围。
        return 0;
      }
      *alias_base_index = base_index;
      return 1 << shift;
    }
    let shift = (other_rep_int - rep_int) as u32;
    *alias_base_index = index >> shift;
    1
  }

  /// 原版 register-configuration.cc AreAliases: 两寄存器是否互为别名
  #[must_use]
  pub fn are_aliases(&self, rep: u8, index: i32, other_rep: u8, other_index: i32) -> bool {
    debug_assert!(self.fp_aliasing_kind_ == AliasingKind::Combine);
    if rep == other_rep {
      return index == other_index;
    }
    let rep_int = rep as i32;
    let other_rep_int = other_rep as i32;
    if rep_int > other_rep_int {
      let shift = (rep_int - other_rep_int) as u32;
      return index == other_index >> shift;
    }
    let shift = (other_rep_int - rep_int) as u32;
    index >> shift == other_index
  }
}

// —— 提取不变式断言 ——
/// 原版 MachineRepresentation 判别值 (仅断言 FP/SIMD 连续段所需)
/// v8: src/codegen/machine-type.h enum class MachineRepresentation
const K_REP_FLOAT32: u8 = 17;
/// v8: src/codegen/machine-type.h MachineRepresentation::kFloat64
const K_REP_FLOAT64: u8 = 18;
/// v8: src/codegen/machine-type.h MachineRepresentation::kSimd128
const K_REP_SIMD128: u8 = 19;
/// v8: src/codegen/machine-type.h MachineRepresentation::kSimd256
const K_REP_SIMD256: u8 = 20;

const _: () = {
  assert!(K_MAX_GENERAL_REGISTERS == 32 && K_MAX_FP_REGISTERS == 32);
  assert!(K_MAX_REGISTERS == 32);
  // v8: register-configuration.cc static_assert: kFloat32/kFloat64/kSimd128/kSimd256 连续
  assert!(K_REP_FLOAT64 == K_REP_FLOAT32 + 1);
  assert!(K_REP_SIMD128 == K_REP_FLOAT64 + 1);
  assert!(K_REP_SIMD256 == K_REP_SIMD128 + 1);
};
