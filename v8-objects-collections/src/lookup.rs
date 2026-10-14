// 由 tool/src/objects/lookup.cc.js 从 v8/src/objects/lookup.{cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// 属性查找迭代器方法面 (配置/状态枚举归 v8-objects-rest/lookup.rs)

//! LookupIterator / ConcurrentLookupIterator 的 .cc + -inl.h 方法面。
//!
//! - 配置位/状态机枚举已由 lookup.h.js 提取至 v8-objects-rest/src/lookup.rs
//! - 查找分派: 元素 (IsElement) / 拦截器 / 特殊 receiver / 常规 holder 四路
//! - 字典 map 走 NameDictionary 慢路径; 快 map 走描述符 + 字段索引

// ── LookupIterator 方法面 (46 项) ────────────────────────
/// LookupIterator 全部 .cc 定义 (对数基准)。
pub const LOOKUP_ITERATOR_METHODS: [&str; 46] = [
  "ApplyTransitionToDataProperty",
  "CanStayConst",
  "CompareAndSwapDataValue",
  "Delete",
  "DictCanStayConst",
  "FetchValue",
  "GetAccessorIndex",
  "GetAccessors",
  "GetDataValue",
  "GetFieldDescriptorIndex",
  "GetFieldIndex",
  "GetHolderForApi",
  "GetInterceptorForFailedAccessCheck",
  "GetPropertyCell",
  "GetReceiverMap",
  "GetRootForNonJSReceiver",
  "GetStringPropertyValue",
  "HasAccess",
  "HasInternalMarkerProperty",
  "HolderIsReceiver",
  "HolderIsReceiverOrHiddenPrototype",
  "InternalUpdateProtector",
  "LookupCachedProperty",
  "LookupInRegularHolder",
  "LookupInSpecialHolder",
  "Next",
  "NextHolder",
  "NextInternal",
  "NotFound",
  "PrepareForDataProperty",
  "PrepareTransitionToDataProperty",
  "RecheckTypedArrayBounds",
  "ReconfigureDataProperty",
  "ReloadPropertyInformation",
  "RestartInternal",
  "SkipInterceptor",
  "Start",
  "SwapDataValue",
  "TransitionToAccessorPair",
  "TransitionToAccessorProperty",
  "TryGetOwnChar",
  "TryGetOwnConstantElement",
  "TryGetOwnCowElement",
  "TryGetPropertyCell",
  "TryLookupCachedProperty",
  "WriteDataValue",
];

// ── ConcurrentLookupIterator 方法面 (4 项) ─────────────────
/// ConcurrentLookupIterator 全部 .cc 定义 (无锁并发读路径)。
pub const CONCURRENT_LOOKUP_ITERATOR_METHODS: [&str; 4] = [
  "TryGetOwnChar",
  "TryGetOwnConstantElement",
  "TryGetOwnCowElement",
  "TryGetPropertyCell",
];

// ── 分派规则登记 ───────────────────────────────────────────────────────
/// 四路分派: 元素/拦截器/特殊 receiver/常规 holder。
pub const DISPATCH_RULES: [&str; 4] = [
  "IsElement(holder) → 元素路径",
  "拦截器 (SkipInterceptor 判定) → API 拦截",
  "IsSpecialReceiverMap → LookupInSpecialHolder",
  "常规 → LookupInRegularHolder (字典/描述符二分)",
];

// ── 主流程签名骨架 (就地修订; 参数为 Address/索引占位) ─────────────────
/// 启动查找 (Start: 首次 LookupInHolder + NotFound 预判)。
///
/// v8: src/objects/lookup.cc LookupIterator::Start
pub fn start(_it: u64) {
  todo!("L4: 需 LookupIterator 全状态机 (receiver/name/state) + JSReceiver Map 描述符/字典分派")
}
/// 推进到下一原型层 (Next: 代理跳层 + NextInternal)。
///
/// v8: src/objects/lookup.cc LookupIterator::Next
pub fn next(_it: u64) {
  todo!("L4: 需 PrototypeIterator 遍历 + JSReceiver/代理跳层 (全 receiver 对象模型)")
}
/// 常规 holder 查找 (LookupInRegularHolder: 字典/描述符二分)。
///
/// v8: src/objects/lookup.cc LookupIterator::LookupInRegularHolder
pub fn lookup_in_regular_holder(_map: u64, _holder: u64) -> u8 {
  todo!("L4: 需 Map 描述符二分 (NameDictionary 慢路径依赖本 crate collections 面 + 全对象模型)")
}
/// 特殊 receiver 查找 (LookupInSpecialHolder: 代理/拦截器/字符串包装)。
///
/// v8: src/objects/lookup.cc LookupIterator::LookupInSpecialHolder
pub fn lookup_in_special_holder(_map: u64, _holder: u64) -> u8 {
  todo!("L4: 需特殊 receiver 分派 (JSProxy/拦截器/JSString/TypedArray 全对象模型)")
}
/// 数据属性赋值准备 (PrepareForDataProperty: 常量性/表示法保持判定)。
///
/// v8: src/objects/lookup.cc LookupIterator::PrepareForDataProperty
pub fn prepare_for_data_property(_it: u64, _value: u64) {
  todo!("L4: 需 Map 迁移/常量性判定 + Factory 表示法转换 (全对象模型 + Isolate)")
}
/// 重配置数据属性 (ReconfigureDataProperty: 属性属性/表示法变更)。
///
/// v8: src/objects/lookup.cc LookupIterator::ReconfigureDataProperty
pub fn reconfigure_data_property(_it: u64, _value: u64, _attributes: u8) {
  todo!("L4: 需 Map 描述符/字典属性重配置 + Factory 迁移路径 (全对象模型)")
}
/// 删除属性 (Delete: 字典直删 / 快属性转字典)。
///
/// v8: src/objects/lookup.cc LookupIterator::Delete
pub fn delete(_it: u64) {
  todo!("L4: 需快属性→字典转换 + NameDictionary 直删 (全 receiver + Isolate 对象模型)")
}
/// 取当前值 (FetchValue: 字段/常量/访问器三路)。
///
/// v8: src/objects/lookup.cc LookupIterator::FetchValue
pub fn fetch_value(_it: u64) -> u64 {
  todo!("L4: 需字段索引/PropertyCell 常量/访问器求值三路 (全 receiver + Isolate)")
}
/// 写当前值 (WriteDataValue: inobject/outofobject 字段写 + 写屏障)。
///
/// v8: src/objects/lookup.cc LookupIterator::WriteDataValue
pub fn write_data_value(_it: u64, _value: u64, _store_barrier: u8) {
  todo!("L4: 需 JSObject inobject/outofobject 字段写 + 写屏障 (全 receiver 对象模型)")
}

// lookup-inl.h 定义清单 (对数基准, 19 项):
// LookupIterator::LookupIterator
// LookupIterator::name
// LookupIterator::name_for_transition
// LookupIterator::GetName
// LookupIterator::GetKey
// LookupIterator::IsElement
// LookupIterator::IsAnyPrivateName
// LookupIterator::is_dictionary_holder
// LookupIterator::transition_map
// LookupIterator::transition_cell
// LookupIterator::GetHolder
// LookupIterator::ExtendingNonExtensible
// LookupIterator::IsCacheableTransition
// LookupIterator::UpdateProtector
// LookupIterator::descriptor_number
// LookupIterator::dictionary_entry
// LookupIterator::ComputeConfiguration
// LookupIterator::GetStoreTarget
// LookupIterator::GetInterceptor

// —— 提取不变式断言 ——
const _: () = {
  assert!(LOOKUP_ITERATOR_METHODS.len() == 46);
  assert!(CONCURRENT_LOOKUP_ITERATOR_METHODS.len() == 4);
  assert!(LOOKUP_ITERATOR_METHODS.contains(&"ApplyTransitionToDataProperty"));
  assert!(DISPATCH_RULES.len() == 4);
};
