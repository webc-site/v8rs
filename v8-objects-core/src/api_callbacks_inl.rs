// 由 tool/src/objects/api-callbacks-inl.h.js 从 v8/src/objects/api-callbacks-inl.h (+ api-callbacks.h 位段核对) 生成; 修订就地改, 复跑覆盖
// 回调访问器宏展开登记; 位域本体在 v8-objects-rest/api_callbacks.rs

//! API 回调内联访问器 (原版 api-callbacks-inl.h)
//!
//! 原版布局契约: `Internals::kCallbackInfoDataOffset` 与 AccessorInfo::data_ /
//! InterceptorInfo::data_ 的 offsetof 全等 (Api 免类型判断直读 data 槽).

use std::ptr::write_bytes;

use crate::error::{Error, Result};

/// 偏移契约: data_ 槽在两个类上偏移一致 (原版 static_assert ×2)
pub const CALLBACK_INFO_DATA_OFFSET_OWNERS: [&str; 2] = ["AccessorInfo", "InterceptorInfo"];

/// InterceptorInfo 惰性回调访问器清单 (LAZY_*_CHECKED2 展开, 16 项:
/// named 7 + indexed 9; 原版按 named/indexed 复用同一物理槽)
pub const INTERCEPTOR_CALLBACKS: [(&str, &str, &str, bool); 16] = [
  (
    "named_getter",
    "getter_",
    "kApiNamedPropertyGetterCallbackTag",
    true,
  ),
  (
    "named_setter",
    "setter_",
    "kApiNamedPropertySetterCallbackTag",
    true,
  ),
  (
    "named_query",
    "query_",
    "kApiNamedPropertyQueryCallbackTag",
    true,
  ),
  (
    "named_descriptor",
    "descriptor_",
    "kApiNamedPropertyDescriptorCallbackTag",
    true,
  ),
  (
    "named_deleter",
    "deleter_",
    "kApiNamedPropertyDeleterCallbackTag",
    true,
  ),
  (
    "named_enumerator",
    "enumerator_",
    "kApiNamedPropertyEnumeratorCallbackTag",
    true,
  ),
  (
    "named_definer",
    "definer_",
    "kApiNamedPropertyDefinerCallbackTag",
    true,
  ),
  (
    "indexed_getter",
    "getter_",
    "kApiIndexedPropertyGetterCallbackTag",
    false,
  ),
  (
    "indexed_setter",
    "setter_",
    "kApiIndexedPropertySetterCallbackTag",
    false,
  ),
  (
    "indexed_query",
    "query_",
    "kApiIndexedPropertyQueryCallbackTag",
    false,
  ),
  (
    "indexed_descriptor",
    "descriptor_",
    "kApiIndexedPropertyDescriptorCallbackTag",
    false,
  ),
  (
    "indexed_deleter",
    "deleter_",
    "kApiIndexedPropertyDeleterCallbackTag",
    false,
  ),
  (
    "indexed_enumerator",
    "enumerator_",
    "kApiIndexedPropertyEnumeratorCallbackTag",
    false,
  ),
  (
    "indexed_definer",
    "definer_",
    "kApiIndexedPropertyDefinerCallbackTag",
    false,
  ),
  (
    "indexed_index_of",
    "index_of_",
    "kApiIndexedPropertyIndexOfCallbackTag",
    false,
  ),
  (
    "indexed_iterable_to_list",
    "iterable_to_list_",
    "kApiIndexedPropertyIterableToListCallbackTag",
    false,
  ),
];

/// InterceptorInfo::has_* 派生谓词 (INTERCEPTOR_INFO_HAS_GETTER X-macro 展开, 7 项)
pub const INTERCEPTOR_HAS_PREDICATES: [&str; 7] = [
  "has_getter",
  "has_setter",
  "has_query",
  "has_descriptor",
  "has_deleter",
  "has_definer",
  "has_enumerator",
];

/// InterceptorInfo 标志位 (BOOL_ACCESSORS/BOOL_GETTER 展开, bool 位段 6 个)
pub const INTERCEPTOR_FLAG_BITS: [&str; 6] = [
  "can_intercept_symbols",
  "non_masking",
  "has_no_side_effect",
  "has_new_callbacks_signature",
  "has_dont_delete_property",
  "is_named",
];

/// AccessorInfo 位段访问器 (BIT_FIELD_ACCESSORS 展开 + 手写 setter_side_effect_type)
pub const ACCESSOR_INFO_BIT_FIELDS: [&str; 5] = [
  "replace_on_access",
  "is_sloppy",
  "getter_side_effect_type",
  "initial_property_attributes",
  "setter_side_effect_type",
];

/// AccessorInfo 普通回调 (REDIRECTED/EXTERNAL 展开各 1 对)
pub const ACCESSOR_INFO_CALLBACKS: [&str; 2] = ["getter", "setter"];

/// 副作用类型枚举 (原版 enum class SideEffectType : int32)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SideEffectType {
  HasSideEffect = 0,
  HasNoSideEffect = 1,
  HasSideEffectToReceiver = 2,
}

/// AccessorInfo::has_getter — 惰性 getter 槽非空 (原版 getter(isolate) != kNullAddress)
#[inline]
pub const fn accessor_info_has_getter(getter_address: usize) -> bool {
  getter_address != 0
}

/// AccessorInfo::has_setter — 惰性 setter 槽非空
#[inline]
pub const fn accessor_info_has_setter(setter_address: usize) -> bool {
  setter_address != 0
}

/// AccessorInfo::setter_side_effect_type — 禁止 kHasNoSideEffect (原版 CHECK_NE)
#[inline]
pub fn accessor_info_setter_side_effect_type(flags: &mut u32, value: SideEffectType) -> Result<()> {
  if value == SideEffectType::HasNoSideEffect {
    return Err(Error::InvalidSetterSideEffectType);
  }
  const SETTER_SIDE_EFFECT_TYPE_SHIFT: u32 = 4;
  const SETTER_SIDE_EFFECT_TYPE_MASK: u32 = 0x30;
  *flags = (*flags & !SETTER_SIDE_EFFECT_TYPE_MASK)
    | (((value as u32) << SETTER_SIDE_EFFECT_TYPE_SHIFT) & SETTER_SIDE_EFFECT_TYPE_MASK);
  Ok(())
}

/// 序列化前摘除回调重定向 (仅模拟器构建有效, 原版 CHECK(USE_SIMULATOR_BOOL))
#[inline]
pub fn remove_callback_redirection_for_serialization(
  is_named: bool,
  getter_redirection: &mut usize,
) {
  if v8_common::USE_SIMULATOR && is_named {
    *getter_redirection = 0;
  }
}

/// 反序列化后恢复回调重定向
#[inline]
pub fn restore_callback_redirection_after_deserialization(
  is_named: bool,
  getter_redirection: &mut usize,
  original_target: usize,
) {
  if v8_common::USE_SIMULATOR && is_named {
    *getter_redirection = original_target;
  }
}

/// 清零可选填充 (保证快照确定性)
///
/// # Safety
/// 调用方需确保 `address + padding_offset` 至 `address + padding_offset + padding_size` 为有效可写内存。
#[inline]
pub unsafe fn clear_padding(address: usize, padding_offset: usize, padding_size: usize) {
  if padding_size == 0 {
    return;
  }
  unsafe {
    let ptr = (address + padding_offset) as *mut u8;
    write_bytes(ptr, 0, padding_size);
  }
}

/// 取 Api 回调的持有者: JSGlobalObject → 对应 JSGlobalProxy (原版 GetHolderForApi)
#[inline]
pub fn get_holder_for_api(
  holder: usize,
  is_global_object: bool,
  global_proxy_fn: Option<fn(usize) -> usize>,
) -> usize {
  if is_global_object && let Some(to_proxy) = global_proxy_fn {
    return to_proxy(holder);
  }
  holder
}
