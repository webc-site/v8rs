// 由 tool/src/objects/managed-type-id.h.js 从 v8/src/objects/managed-type-id.h 生成; 修订就地改, 复跑覆盖
// ManagedTypeId 名称表 (枚举 1:1)
//! ManagedTypeId 名称表: C++ 侧 `ToString(ManagedTypeId)` switch 的 1:1 移植。
//! 枚举定义在 include/v8-internal.h; 共 20 项。

/// ManagedTypeId 每个变体的显示名 (原版 ToString)。
#[rustfmt::skip]
pub const MANAGED_TYPE_ID_NAMES: [(&str, &str); 20] = [
  ("kBackingStore", "BackingStore"),
  ("kTestDeleteCounter", "TestDeleteCounter"),
  ("kTestDeleteNative", "TestDeleteNative"),
  ("kWasmStreaming", "WasmStreaming"),
  ("kWasmFuncData", "WasmFuncData"),
  ("kWasmManagedData", "WasmManagedData"),
  ("kWasmNativeModule", "WasmNativeModule"),
  ("kIcuBreakIteratorWithText", "IcuBreakIteratorWithText"),
  ("kTemporalDuration", "TemporalDuration"),
  ("kTemporalInstant", "TemporalInstant"),
  ("kTemporalPlainDate", "TemporalPlainDate"),
  ("kTemporalPlainTime", "TemporalPlainTime"),
  ("kTemporalPlainDateTime", "TemporalPlainDateTime"),
  ("kTemporalPlainYearMonth", "TemporalPlainYearMonth"),
  ("kTemporalPlainMonthDay", "TemporalPlainMonthDay"),
  ("kTemporalZonedDateTime", "TemporalZonedDateTime"),
  ("kDisplayNamesInternal", "DisplayNamesInternal"),
  ("kD8Worker", "D8Worker"),
  ("kD8ModuleEmbedderData", "D8ModuleEmbedderData"),
  ("kD8AsyncHooksWrap", "D8AsyncHooksWrap"),
];

/// 按 k 名查显示名。
#[inline]
#[must_use]
pub fn to_string(type_id: &str) -> Option<&'static str> {
  MANAGED_TYPE_ID_NAMES.iter().find(|(k, _)| *k == type_id).map(|(_, n)| *n)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(MANAGED_TYPE_ID_NAMES.len() == 20);
  assert!(MANAGED_TYPE_ID_NAMES[0] == ("kBackingStore", "BackingStore"));
  assert!(MANAGED_TYPE_ID_NAMES[19] == ("kD8AsyncHooksWrap", "D8AsyncHooksWrap"));
};
