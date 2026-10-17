// 由 tool/src/objects/property-cell.tq.js 从 v8/src/objects/property-cell.tq 生成; 修订就地改, 复跑覆盖
// PropertyCell tq 布局 (clang -E property-cell.h 对数, 压缩配置)
//! PropertyCell 的 Torque 侧布局镜像: 4 字段表 + 偏移 (压缩 K_TAGGED_SIZE=4)。
//! 语义面 (PropertyCellType 状态机/依赖代码失效) 归 property-cell.h 侧脚本。

/// tq extern class PropertyCell 字段表 (顺序 1:1)。
pub const PROPERTY_CELL_FIELDS: [(&str, &str); 4] = [
  ("name", "Name"),
  ("property_details_raw", "Smi"),
  ("value", "Object"),
  ("dependent_code", "DependentCode"),
];

/// name 偏移 (tagged)。
pub const K_NAME_OFFSET: usize = 4;
/// property_details_raw 偏移 (tagged Smi)。
pub const K_PROPERTY_DETAILS_RAW_OFFSET: usize = 8;
/// value 偏移 (tagged)。
pub const K_VALUE_OFFSET: usize = 12;
/// dependent_code 偏移 (tagged)。
pub const K_DEPENDENT_CODE_OFFSET: usize = 16;
/// 对象头尺寸 (字段后第一个字节)。
pub const K_HEADER_SIZE: usize = 20;

// —— 提取不变式断言 ——
const _: () = {
  assert!(PROPERTY_CELL_FIELDS.len() == 4);
  assert!((K_NAME_OFFSET == 4) && (K_VALUE_OFFSET == 12) && (K_DEPENDENT_CODE_OFFSET == 16));
  assert!(K_HEADER_SIZE == 20);
};
