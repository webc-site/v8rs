// 由 tool/src/objects/hole.h.js 从 v8/src/objects/hole.{h,tq} + object-list-macros.h(HOLE_LIST) 生成; 修订就地改, 复跑覆盖
// Hole 全类型表 (V8_ENABLE_TDZ_HOLE 关闭分支: TdzHole 别名 TheHole)

/// Hole 载荷大小 (原版 kPayloadSize = 64*KB, 需为页大小倍数)
pub const HOLE_PAYLOAD_SIZE: usize = 65536;

/// Hole 全类型表 (原版 HOLE_LIST 展开, 三列: 类型/只读根名/根槽名)
/// 各类型为 `V8_OBJECT class X : public Hole` 的空子类, 仅作实例类型区分
pub const HOLE_LIST: [(&str, &str, &str); 13] = [
  ("TheHole", "the_hole_value", "TheHoleValue"),
  (
    "PropertyCellHole",
    "property_cell_hole_value",
    "PropertyCellHoleValue",
  ),
  (
    "HashTableHole",
    "hash_table_hole_value",
    "HashTableHoleValue",
  ),
  ("PromiseHole", "promise_hole_value", "PromiseHoleValue"),
  (
    "DisabledTdzHole",
    "disabled_tdz_hole_value",
    "DisabledTdzHoleValue",
  ),
  ("ExceptionHole", "exception", "Exception"),
  (
    "TerminationException",
    "termination_exception",
    "TerminationException",
  ),
  (
    "UninitializedHole",
    "uninitialized_value",
    "UninitializedValue",
  ),
  ("ArgumentsMarker", "arguments_marker", "ArgumentsMarker"),
  ("OptimizedOut", "optimized_out", "OptimizedOut"),
  ("StaleRegister", "stale_register", "StaleRegister"),
  (
    "SelfReferenceMarker",
    "self_reference_marker",
    "SelfReferenceMarker",
  ),
  (
    "BasicBlockCountersMarker",
    "basic_block_counters_marker",
    "BasicBlockCountersMarker",
  ),
];

/// Hole 基类标记 (原版 V8_OBJECT class Hole : public HeapObject)
pub struct Hole;

/// 类型别名, 原 TDZ_HOLE_LIST 展开项 (V8_ENABLE_TDZ_HOLE 关闭时 TdzHole = TheHole)
pub type TdzHole = TheHole;

/// TheHole 标记类型 (tq: extern class TheHole extends Hole)
pub struct TheHole;
/// PromiseHole (tq: extern class PromiseHole extends Hole)
pub struct PromiseHole;

// —— 提取不变式断言 ——
const _: () = {
  assert!(HOLE_PAYLOAD_SIZE == 65536);
  assert!(HOLE_PAYLOAD_SIZE.is_multiple_of(4096));
  assert!(HOLE_LIST.len() == 13);
};
