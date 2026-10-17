// 由 tool/src/objects/prototype-info.tq.js 从 v8/src/objects/prototype-info.tq 生成; 修订就地改, 复跑覆盖
// PrototypeInfo tq 布局 (clang -E prototype-info.h 对数, 压缩配置)
//! PrototypeInfo 的 Torque 侧布局镜像: 标志位段 + 字段表 + 偏移
//! (压缩 K_TAGGED_SIZE=4; 语义面/PrototypeUsers 归 prototype-info.h 侧脚本)。

/// PrototypeInfoFlags: should_be_fast 位段 (1 bit @0, SmiTagged 容器)。
pub const SHOULD_BE_FAST_SHIFT: u32 = 0;
/// should_be_fast 掩码。
pub const SHOULD_BE_FAST_MASK: u32 = 0x1;

/// 缓存 handler 数 (原版 kCachedHandlerCount; Default + WithLookupOnReceiver)。
pub const CACHED_HANDLER_COUNT: usize = 2;
/// kLoadNonExistentHandlerDefault 序值。
pub const LOAD_NON_EXISTENT_HANDLER_DEFAULT: usize = 0;
/// kLoadNonExistentHandlerWithLookupOnReceiver 序值。
pub const LOAD_NON_EXISTENT_HANDLER_WITH_LOOKUP_ON_RECEIVER: usize = 1;

/// tq extern class PrototypeInfo 字段表 (顺序 1:1; 尾数组含 extent 名)。
pub const PROTOTYPE_INFO_FIELDS: [(&str, &str); 8] = [
  ("module_namespace", "JSModuleNamespace | Undefined"),
  ("prototype_users", "WeakArrayList | Zero"),
  (
    "prototype_chain_enum_cache",
    "FixedArray | Zero | Undefined",
  ),
  ("registry_slot", "Smi"),
  ("bit_field", "SmiTagged<PrototypeInfoFlags>"),
  ("derived_maps", "WeakArrayList | Undefined"),
  (
    "prototype_shared_closure_info",
    "PrototypeSharedClosureInfo | Undefined",
  ),
  (
    "cached_handler[kPrototypeInfoCachedHandlerCount]",
    "LoadHandler | Zero",
  ),
];

/// module_namespace 偏移 (tagged)。
pub const K_MODULE_NAMESPACE_OFFSET: usize = 4;
/// prototype_users 偏移 (tagged)。
pub const K_PROTOTYPE_USERS_OFFSET: usize = 8;
/// prototype_chain_enum_cache 偏移 (tagged)。
pub const K_PROTOTYPE_CHAIN_ENUM_CACHE_OFFSET: usize = 12;
/// registry_slot 偏移 (tagged Smi)。
pub const K_REGISTRY_SLOT_OFFSET: usize = 16;
/// bit_field 偏移 (tagged Smi, 内嵌 PrototypeInfoFlags)。
pub const K_BIT_FIELD_OFFSET: usize = 20;
/// derived_maps 偏移 (tagged)。
pub const K_DERIVED_MAPS_OFFSET: usize = 24;
/// prototype_shared_closure_info 偏移 (tagged)。
pub const K_PROTOTYPE_SHARED_CLOSURE_INFO_OFFSET: usize = 28;
/// cached_handler 数组起点偏移 (tagged x kCachedHandlerCount)。
pub const K_CACHED_HANDLER_OFFSET: usize = 32;
/// 定长对象尺寸 (头 + 8 字段 + 2 缓存 handler)。
pub const K_OBJECT_SIZE: usize = 44;

// —— 提取不变式断言 ——
const _: () = {
  assert!(PROTOTYPE_INFO_FIELDS.len() == 8);
  assert!((SHOULD_BE_FAST_MASK == 0x1) && (SHOULD_BE_FAST_SHIFT == 0));
  assert!((CACHED_HANDLER_COUNT == 2) && (LOAD_NON_EXISTENT_HANDLER_WITH_LOOKUP_ON_RECEIVER == 1));
  assert!((K_CACHED_HANDLER_OFFSET == 32) && (K_OBJECT_SIZE == 44));
};
