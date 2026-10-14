// 由 tool/src/objects/sort-state.h.js 从 v8/src/objects/sort-state.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// SortState 布局字段表 (13 槽)
//! SortState: Array.prototype.sort 的 Torque 侧状态对象。
//! 字段全部由 Torque 生成代码访问 (原版无 C++ 访问器); 偏移由布局引擎计算,
//! 此处登记字段清单与类型 1:1。

/// SortState 布局字段表 (sort-state.h 成员声明顺序, 共 13 槽)。
pub const SORT_STATE_FIELDS: [(&str, &str); 13] = [
  ("receiver_", "JSReceiver"),
  ("initial_receiver_map_", "Map"),
  ("initial_receiver_length_", "Number"),
  ("user_cmp_fn_", "UnionOf<JSCallable, Undefined>"),
  ("is_reset_to_generic_", "Boolean"),
  ("min_gallop_", "Smi"),
  ("pending_runs_size_", "Smi"),
  ("pending_runs_", "FixedArray"),
  ("pending_powers_", "FixedArray"),
  ("work_array_", "FixedArray"),
  ("temp_array_", "FixedArray"),
  ("sort_length_", "Smi"),
  ("number_of_undefined_", "Smi"),
];

/// 字段槽总数。
pub const FIELD_COUNT: usize = 13;

// —— 提取不变式断言 ——
const _: () = {
  assert!(FIELD_COUNT == 13);
  assert!(SORT_STATE_FIELDS[0] == ("receiver_", "JSReceiver"));
  assert!(SORT_STATE_FIELDS[FIELD_COUNT - 1].0 == "number_of_undefined_");
  assert!(SORT_STATE_FIELDS.iter().all(|(_, t)| !t.is_empty()));
};
