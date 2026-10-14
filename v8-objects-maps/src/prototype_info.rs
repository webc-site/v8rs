// 由 tool/src/objects/prototype-info.h.js 从 v8/src/objects/prototype-info.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// PrototypeInfo 常量/位段/字段表 + -inl 签名清单 (clang -E 对数)
//! PrototypeInfo: 原型 map 元数据容器 (访问器 + 派生 map 缓存 + 注册槽)。
//! tq 布局镜像归 crate::prototype_info_tq; 本模块落 C++ 侧规则代码。

/// 缓存 handler 槽位 (原版 enum CachedHandlerIndex)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum CachedHandlerIndex {
  /// 无访问检查、不在 receiver 上查找 (kLoadNonExistentHandlerDefault = 0)。
  LoadNonExistentHandlerDefault = 0,
  /// 需在 receiver 上查找 (kLoadNonExistentHandlerWithLookupOnReceiver = 1)。
  LoadNonExistentHandlerWithLookupOnReceiver = 1,
}

/// 未注册标记 (原版 PrototypeInfo::UNREGISTERED)。
pub const UNREGISTERED: i32 = -1;

/// ShouldBeFastBit: bit_field_ 的 should_be_fast 位 (1 bit @0, uint32 容器)。
pub const SHOULD_BE_FAST_BIT_SHIFT: u32 = 0;
/// ShouldBeFastBit 位宽。
pub const SHOULD_BE_FAST_BIT_BITS: u32 = 1;
/// ShouldBeFastBit 掩码。
pub const SHOULD_BE_FAST_BIT_MASK: u32 = 0x1;

/// 原型用户登记表槽位 (原版 PrototypeUsers, WeakArrayList 语义)。
pub mod prototype_users {
  /// 空槽链头槽位。
  pub const K_EMPTY_SLOT_INDEX: usize = 0;
  /// 首个可用槽位。
  pub const K_FIRST_INDEX: usize = 1;
  /// 无空槽标记值 (原版 kNoEmptySlotsMarker = Smi(0))。
  pub const K_NO_EMPTY_SLOTS_MARKER: i32 = 0;
}

/// C++ 成员字段表 (TaggedMember 声明序 1:1)。
pub const PROTOTYPE_INFO_MEMBERS: [(&str, &str); 8] = [
  ("module_namespace_", "JSModuleNamespace | Undefined"),
  ("prototype_users_", "WeakArrayList | Zero"),
  (
    "prototype_chain_enum_cache_",
    "FixedArray | Zero | Undefined",
  ),
  ("registry_slot_", "Smi"),
  ("bit_field_", "Smi"),
  ("derived_maps_", "WeakArrayList | Undefined"),
  (
    "prototype_shared_closure_info_",
    "PrototypeSharedClosureInfo | Undefined",
  ),
  ("cached_handler_", "LoadHandler | Zero"),
];

// —— -inl.h 签名清单 (28 项含重载与宏展开, pp 定义计数对数; 语义面留 Developer 修订) ——
// PrototypeInfo::PrototypeInfo
// PrototypeInfo::module_namespace
// PrototypeInfo::set_module_namespace
// PrototypeInfo::prototype_users
// PrototypeInfo::set_prototype_users
// PrototypeInfo::prototype_chain_enum_cache
// PrototypeInfo::set_prototype_chain_enum_cache
// PrototypeInfo::registry_slot
// PrototypeInfo::set_registry_slot
// PrototypeInfo::bit_field
// PrototypeInfo::set_bit_field
// PrototypeInfo::derived_maps
// PrototypeInfo::derived_maps
// PrototypeInfo::set_derived_maps
// PrototypeInfo::set_derived_maps
// PrototypeInfo::prototype_shared_closure_info
// PrototypeInfo::set_prototype_shared_closure_info
// PrototypeInfo::cached_handler
// PrototypeInfo::set_cached_handler
// PrototypeInfo::ObjectCreateMap
// PrototypeInfo::ObjectCreateMap
// PrototypeInfo::SetObjectCreateMap
// PrototypeInfo::GetDerivedMap
// PrototypeInfo::AddDerivedMap
// PrototypeInfo::should_be_fast_map
// PrototypeUsers::MarkSlotEmpty
// PrototypeUsers::empty_slot_index
// PrototypeUsers::set_empty_slot_index

// —— 提取不变式断言 ——
const _: () = {
  assert!(CachedHandlerIndex::LoadNonExistentHandlerDefault as usize == 0);
  assert!(CachedHandlerIndex::LoadNonExistentHandlerWithLookupOnReceiver as usize == 1);
  assert!(UNREGISTERED == -1);
  assert!((SHOULD_BE_FAST_BIT_MASK == 0x1) && (SHOULD_BE_FAST_BIT_SHIFT == 0));
  assert!(PROTOTYPE_INFO_MEMBERS.len() == 8);
  assert!(prototype_users::K_FIRST_INDEX == 1);
};
