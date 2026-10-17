// 由 tool/src/wasm/canonical-types.h.js 从 v8/src/wasm/canonical-types.h 生成; 修订就地改, 复跑覆盖
// TypeCanonicalizer 预定义规范类型索引表 + 容量常量

use crate::{value_type::CanonicalTypeIndex, wasm_limits::K_V8_MAX_WASM_TYPES};

/// 规范类型容量上限 (对应 kMaxCanonicalTypes = 2 * kV8MaxWasmTypes)
pub const MAX_CANONICAL_TYPES: usize = 2 * K_V8_MAX_WASM_TYPES;

/// 预定义规范类型索引 (对应 TypeCanonicalizer 内 static constexpr, 连续 0..30)
#[rustfmt::skip]
pub static PREDEFINED_TYPE_INDICES: &[(&str, CanonicalTypeIndex)] = &[
  ("kPredefinedArrayI8Index", CanonicalTypeIndex::new(0)),
  ("kPredefinedArrayI8SharedIndex", CanonicalTypeIndex::new(1)),
  ("kPredefinedArrayI16Index", CanonicalTypeIndex::new(2)),
  ("kPredefinedArrayI16SharedIndex", CanonicalTypeIndex::new(3)),
  ("kPredefinedArrayExternRefIndex", CanonicalTypeIndex::new(4)),
  ("kPredefinedArrayFuncRefIndex", CanonicalTypeIndex::new(5)),
  ("kPredefinedSigIndex_e_i", CanonicalTypeIndex::new(6)),
  ("kPredefinedSigIndex_t_i", CanonicalTypeIndex::new(7)),
  ("kPredefinedSigIndex_e_r", CanonicalTypeIndex::new(8)),
  ("kPredefinedSigIndex_t_s", CanonicalTypeIndex::new(9)),
  ("kPredefinedSigIndex_e_rr", CanonicalTypeIndex::new(10)),
  ("kPredefinedSigIndex_t_ss", CanonicalTypeIndex::new(11)),
  ("kPredefinedSigIndex_e_rii", CanonicalTypeIndex::new(12)),
  ("kPredefinedSigIndex_t_sii", CanonicalTypeIndex::new(13)),
  ("kPredefinedSigIndex_i_r", CanonicalTypeIndex::new(14)),
  ("kPredefinedSigIndex_i_s", CanonicalTypeIndex::new(15)),
  ("kPredefinedSigIndex_i_ri", CanonicalTypeIndex::new(16)),
  ("kPredefinedSigIndex_i_si", CanonicalTypeIndex::new(17)),
  ("kPredefinedSigIndex_i_rr", CanonicalTypeIndex::new(18)),
  ("kPredefinedSigIndex_i_ss", CanonicalTypeIndex::new(19)),
  ("kPredefinedSigIndex_e_a16ii", CanonicalTypeIndex::new(20)),
  ("kPredefinedSigIndex_t_as16ii", CanonicalTypeIndex::new(21)),
  ("kPredefinedSigIndex_i_ra16i", CanonicalTypeIndex::new(22)),
  ("kPredefinedSigIndex_i_sas16i", CanonicalTypeIndex::new(23)),
  ("kPredefinedSigIndex_i_ra8i", CanonicalTypeIndex::new(24)),
  ("kPredefinedSigIndex_i_sas8i", CanonicalTypeIndex::new(25)),
  ("kPredefinedSigIndex_n8_r", CanonicalTypeIndex::new(26)),
  ("kPredefinedSigIndex_ns8_s", CanonicalTypeIndex::new(27)),
  ("kPredefinedSigIndex_e_a8ii", CanonicalTypeIndex::new(28)),
  ("kPredefinedSigIndex_t_as8ii", CanonicalTypeIndex::new(29)),
  ("kPredefinedSigIndex_configureAll", CanonicalTypeIndex::new(30)),
];

/// 预定义类型数量 (对应 kNumberOfPredefinedTypes)
pub const NUMBER_OF_PREDEFINED_TYPES: u32 = 31;

/// 具名常量直出 (便于按名引用, 与原版同名)
pub mod predefined {
  use crate::value_type::CanonicalTypeIndex;
  /// kPredefinedArrayI8Index
  pub const ARRAY_I8_INDEX: CanonicalTypeIndex = CanonicalTypeIndex::new(0);
  /// kPredefinedArrayI8SharedIndex
  pub const ARRAY_I8_SHARED_INDEX: CanonicalTypeIndex = CanonicalTypeIndex::new(1);
  /// kPredefinedArrayI16Index
  pub const ARRAY_I16_INDEX: CanonicalTypeIndex = CanonicalTypeIndex::new(2);
  /// kPredefinedArrayI16SharedIndex
  pub const ARRAY_I16_SHARED_INDEX: CanonicalTypeIndex = CanonicalTypeIndex::new(3);
  /// kPredefinedArrayExternRefIndex
  pub const ARRAY_EXTERN_REF_INDEX: CanonicalTypeIndex = CanonicalTypeIndex::new(4);
  /// kPredefinedArrayFuncRefIndex
  pub const ARRAY_FUNC_REF_INDEX: CanonicalTypeIndex = CanonicalTypeIndex::new(5);
  /// kPredefinedSigIndex_e_i
  pub const SIG_INDEX_E_I: CanonicalTypeIndex = CanonicalTypeIndex::new(6);
  /// kPredefinedSigIndex_t_i
  pub const SIG_INDEX_T_I: CanonicalTypeIndex = CanonicalTypeIndex::new(7);
  /// kPredefinedSigIndex_e_r
  pub const SIG_INDEX_E_R: CanonicalTypeIndex = CanonicalTypeIndex::new(8);
  /// kPredefinedSigIndex_t_s
  pub const SIG_INDEX_T_S: CanonicalTypeIndex = CanonicalTypeIndex::new(9);
  /// kPredefinedSigIndex_e_rr
  pub const SIG_INDEX_E_RR: CanonicalTypeIndex = CanonicalTypeIndex::new(10);
  /// kPredefinedSigIndex_t_ss
  pub const SIG_INDEX_T_SS: CanonicalTypeIndex = CanonicalTypeIndex::new(11);
  /// kPredefinedSigIndex_e_rii
  pub const SIG_INDEX_E_RII: CanonicalTypeIndex = CanonicalTypeIndex::new(12);
  /// kPredefinedSigIndex_t_sii
  pub const SIG_INDEX_T_SII: CanonicalTypeIndex = CanonicalTypeIndex::new(13);
  /// kPredefinedSigIndex_i_r
  pub const SIG_INDEX_I_R: CanonicalTypeIndex = CanonicalTypeIndex::new(14);
  /// kPredefinedSigIndex_i_s
  pub const SIG_INDEX_I_S: CanonicalTypeIndex = CanonicalTypeIndex::new(15);
  /// kPredefinedSigIndex_i_ri
  pub const SIG_INDEX_I_RI: CanonicalTypeIndex = CanonicalTypeIndex::new(16);
  /// kPredefinedSigIndex_i_si
  pub const SIG_INDEX_I_SI: CanonicalTypeIndex = CanonicalTypeIndex::new(17);
  /// kPredefinedSigIndex_i_rr
  pub const SIG_INDEX_I_RR: CanonicalTypeIndex = CanonicalTypeIndex::new(18);
  /// kPredefinedSigIndex_i_ss
  pub const SIG_INDEX_I_SS: CanonicalTypeIndex = CanonicalTypeIndex::new(19);
  /// kPredefinedSigIndex_e_a16ii
  pub const SIG_INDEX_E_A16II: CanonicalTypeIndex = CanonicalTypeIndex::new(20);
  /// kPredefinedSigIndex_t_as16ii
  pub const SIG_INDEX_T_AS16II: CanonicalTypeIndex = CanonicalTypeIndex::new(21);
  /// kPredefinedSigIndex_i_ra16i
  pub const SIG_INDEX_I_RA16I: CanonicalTypeIndex = CanonicalTypeIndex::new(22);
  /// kPredefinedSigIndex_i_sas16i
  pub const SIG_INDEX_I_SAS16I: CanonicalTypeIndex = CanonicalTypeIndex::new(23);
  /// kPredefinedSigIndex_i_ra8i
  pub const SIG_INDEX_I_RA8I: CanonicalTypeIndex = CanonicalTypeIndex::new(24);
  /// kPredefinedSigIndex_i_sas8i
  pub const SIG_INDEX_I_SAS8I: CanonicalTypeIndex = CanonicalTypeIndex::new(25);
  /// kPredefinedSigIndex_n8_r
  pub const SIG_INDEX_N8_R: CanonicalTypeIndex = CanonicalTypeIndex::new(26);
  /// kPredefinedSigIndex_ns8_s
  pub const SIG_INDEX_NS8_S: CanonicalTypeIndex = CanonicalTypeIndex::new(27);
  /// kPredefinedSigIndex_e_a8ii
  pub const SIG_INDEX_E_A8II: CanonicalTypeIndex = CanonicalTypeIndex::new(28);
  /// kPredefinedSigIndex_t_as8ii
  pub const SIG_INDEX_T_AS8II: CanonicalTypeIndex = CanonicalTypeIndex::new(29);
  /// kPredefinedSigIndex_configureAll
  pub const SIG_INDEX_CONFIGURE_ALL: CanonicalTypeIndex = CanonicalTypeIndex::new(30);
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(PREDEFINED_TYPE_INDICES.len() == 31);
  assert!(NUMBER_OF_PREDEFINED_TYPES == 31);
  assert!(predefined::ARRAY_I8_INDEX.index == 0);
  assert!(predefined::ARRAY_I8_SHARED_INDEX.index == 1);
  assert!(predefined::ARRAY_I16_INDEX.index == 2);
  assert!(predefined::ARRAY_EXTERN_REF_INDEX.index == 4);
  assert!(predefined::SIG_INDEX_E_I.index == 6);
  assert!(predefined::SIG_INDEX_CONFIGURE_ALL.index == 30);
  assert!(predefined::SIG_INDEX_CONFIGURE_ALL.index + 1 == NUMBER_OF_PREDEFINED_TYPES);
};
