// 由 tool/src/objects/instance-types-gen.h.js 从 v8/src/objects/instance-types-gen.h 生成; 修订就地改, 复跑覆盖
// 实例类型生成器选择开关
//! 生成器选择: metagen (tools/metagen/) 或 Torque 产物。
//! 默认 V8_USE_METAGEN=1 (原版 #if !defined 回退值)。
//! 本 crate 的实例类型值由 instance_type 模块 (metagen 求解器复刻) 提供。

/// metagen 路径开关 (原版默认 1; 与 crate::instance_type 的求解器来源一致)。
pub const V8_USE_METAGEN: bool = true;

/// 收割遍 (V8_METAGEN_GENERATION_PASS) 下定义为 no-op 的供给宏名单。
pub const HARVEST_PASS_NOOP_MACROS: [&str; 5] = [
  "TORQUE_ASSIGNED_INSTANCE_TYPES",
  "TORQUE_ASSIGNED_INSTANCE_TYPE_LIST",
  "INSTANCE_TYPE_LIST_SINGLE",
  "INSTANCE_TYPE_LIST_MULTIPLE",
  "INSTANCE_TYPE_LIST_RANGE",
];

/// Torque 路径附加供给头 (metagen 路径内嵌于 instance-types.h)。
pub const TORQUE_EXTRA_INCLUDES: [&str; 2] = [
  "torque-generated/instance-type-checker-lists.h",
  "torque-generated/debug-reader-classes-list.h",
];

/// 诊断派发宏 (metagen 由 Print/Verify 声明推导, Torque 路径由 debug-reader 列表别名)。
pub const DIAGNOSTIC_DISPATCH_MACRO: &str = "HEAP_OBJECT_DIAGNOSTIC_DISPATCH_LIST";

// —— 提取不变式断言 ——
const _: () = {
  assert!(V8_USE_METAGEN);
  assert!(HARVEST_PASS_NOOP_MACROS.len() == 5);
  assert!(TORQUE_EXTRA_INCLUDES.len() == 2);
};
