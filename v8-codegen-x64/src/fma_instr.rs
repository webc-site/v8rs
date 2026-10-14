// 由 tool/src/codegen/x64/fma-instr.h.js 从 v8/src/codegen/x64/fma-instr.h 生成; 修订就地改, 复跑覆盖
// FMA 指令编码表 (X-macro 机械展开)

// 二元组: (指令助记名, 编码序列: 66,0F,38,W标,opcode)。
// W0/W1 为 VEX.W 位标。

/// FMA 标量双精度 (12 条)
#[rustfmt::skip]
pub const FMA_SD_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("vfmadd132sd", &["66", "0F", "38", "W1", "99"]),
  ("vfmadd213sd", &["66", "0F", "38", "W1", "a9"]),
  ("vfmadd231sd", &["66", "0F", "38", "W1", "b9"]),
  ("vfmsub132sd", &["66", "0F", "38", "W1", "9b"]),
  ("vfmsub213sd", &["66", "0F", "38", "W1", "ab"]),
  ("vfmsub231sd", &["66", "0F", "38", "W1", "bb"]),
  ("vfnmadd132sd", &["66", "0F", "38", "W1", "9d"]),
  ("vfnmadd213sd", &["66", "0F", "38", "W1", "ad"]),
  ("vfnmadd231sd", &["66", "0F", "38", "W1", "bd"]),
  ("vfnmsub132sd", &["66", "0F", "38", "W1", "9f"]),
  ("vfnmsub213sd", &["66", "0F", "38", "W1", "af"]),
  ("vfnmsub231sd", &["66", "0F", "38", "W1", "bf"]),
];

/// FMA 标量单精度 (12 条)
#[rustfmt::skip]
pub const FMA_SS_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("vfmadd132ss", &["66", "0F", "38", "W0", "99"]),
  ("vfmadd213ss", &["66", "0F", "38", "W0", "a9"]),
  ("vfmadd231ss", &["66", "0F", "38", "W0", "b9"]),
  ("vfmsub132ss", &["66", "0F", "38", "W0", "9b"]),
  ("vfmsub213ss", &["66", "0F", "38", "W0", "ab"]),
  ("vfmsub231ss", &["66", "0F", "38", "W0", "bb"]),
  ("vfnmadd132ss", &["66", "0F", "38", "W0", "9d"]),
  ("vfnmadd213ss", &["66", "0F", "38", "W0", "ad"]),
  ("vfnmadd231ss", &["66", "0F", "38", "W0", "bd"]),
  ("vfnmsub132ss", &["66", "0F", "38", "W0", "9f"]),
  ("vfnmsub213ss", &["66", "0F", "38", "W0", "af"]),
  ("vfnmsub231ss", &["66", "0F", "38", "W0", "bf"]),
];

/// FMA 打包单精度 (6 条)
#[rustfmt::skip]
pub const FMA_PS_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("vfmadd132ps", &["66", "0F", "38", "W0", "98"]),
  ("vfmadd213ps", &["66", "0F", "38", "W0", "a8"]),
  ("vfmadd231ps", &["66", "0F", "38", "W0", "b8"]),
  ("vfnmadd132ps", &["66", "0F", "38", "W0", "9c"]),
  ("vfnmadd213ps", &["66", "0F", "38", "W0", "ac"]),
  ("vfnmadd231ps", &["66", "0F", "38", "W0", "bc"]),
];

/// FMA 打包双精度 (6 条)
#[rustfmt::skip]
pub const FMA_PD_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("vfmadd132pd", &["66", "0F", "38", "W1", "98"]),
  ("vfmadd213pd", &["66", "0F", "38", "W1", "a8"]),
  ("vfmadd231pd", &["66", "0F", "38", "W1", "b8"]),
  ("vfnmadd132pd", &["66", "0F", "38", "W1", "9c"]),
  ("vfnmadd213pd", &["66", "0F", "38", "W1", "ac"]),
  ("vfnmadd231pd", &["66", "0F", "38", "W1", "bc"]),
];

/// FMA 组合表 = SD + SS + PS + PD (36 条)
#[rustfmt::skip]
pub const FMA_INSTRUCTION_LIST: &[(&str, &[&str])] = &[
  ("vfmadd132sd", &["66", "0F", "38", "W1", "99"]),
  ("vfmadd213sd", &["66", "0F", "38", "W1", "a9"]),
  ("vfmadd231sd", &["66", "0F", "38", "W1", "b9"]),
  ("vfmsub132sd", &["66", "0F", "38", "W1", "9b"]),
  ("vfmsub213sd", &["66", "0F", "38", "W1", "ab"]),
  ("vfmsub231sd", &["66", "0F", "38", "W1", "bb"]),
  ("vfnmadd132sd", &["66", "0F", "38", "W1", "9d"]),
  ("vfnmadd213sd", &["66", "0F", "38", "W1", "ad"]),
  ("vfnmadd231sd", &["66", "0F", "38", "W1", "bd"]),
  ("vfnmsub132sd", &["66", "0F", "38", "W1", "9f"]),
  ("vfnmsub213sd", &["66", "0F", "38", "W1", "af"]),
  ("vfnmsub231sd", &["66", "0F", "38", "W1", "bf"]),
  ("vfmadd132ss", &["66", "0F", "38", "W0", "99"]),
  ("vfmadd213ss", &["66", "0F", "38", "W0", "a9"]),
  ("vfmadd231ss", &["66", "0F", "38", "W0", "b9"]),
  ("vfmsub132ss", &["66", "0F", "38", "W0", "9b"]),
  ("vfmsub213ss", &["66", "0F", "38", "W0", "ab"]),
  ("vfmsub231ss", &["66", "0F", "38", "W0", "bb"]),
  ("vfnmadd132ss", &["66", "0F", "38", "W0", "9d"]),
  ("vfnmadd213ss", &["66", "0F", "38", "W0", "ad"]),
  ("vfnmadd231ss", &["66", "0F", "38", "W0", "bd"]),
  ("vfnmsub132ss", &["66", "0F", "38", "W0", "9f"]),
  ("vfnmsub213ss", &["66", "0F", "38", "W0", "af"]),
  ("vfnmsub231ss", &["66", "0F", "38", "W0", "bf"]),
  ("vfmadd132ps", &["66", "0F", "38", "W0", "98"]),
  ("vfmadd213ps", &["66", "0F", "38", "W0", "a8"]),
  ("vfmadd231ps", &["66", "0F", "38", "W0", "b8"]),
  ("vfnmadd132ps", &["66", "0F", "38", "W0", "9c"]),
  ("vfnmadd213ps", &["66", "0F", "38", "W0", "ac"]),
  ("vfnmadd231ps", &["66", "0F", "38", "W0", "bc"]),
  ("vfmadd132pd", &["66", "0F", "38", "W1", "98"]),
  ("vfmadd213pd", &["66", "0F", "38", "W1", "a8"]),
  ("vfmadd231pd", &["66", "0F", "38", "W1", "b8"]),
  ("vfnmadd132pd", &["66", "0F", "38", "W1", "9c"]),
  ("vfnmadd213pd", &["66", "0F", "38", "W1", "ac"]),
  ("vfnmadd231pd", &["66", "0F", "38", "W1", "bc"]),
];


// —— 提取不变式断言 ——
const _: () = {
  assert!(FMA_SD_INSTRUCTION_LIST.len() == 12);
  assert!(FMA_SS_INSTRUCTION_LIST.len() == 12);
  assert!(FMA_PS_INSTRUCTION_LIST.len() == 6);
  assert!(FMA_PD_INSTRUCTION_LIST.len() == 6);
  assert!(FMA_INSTRUCTION_LIST.len() == 36);
  assert!(FMA_SD_INSTRUCTION_LIST[0].0 == "vfmadd132sd");
  assert!(FMA_INSTRUCTION_LIST.last().unwrap().0 == "vfnmsub231pd");
};
