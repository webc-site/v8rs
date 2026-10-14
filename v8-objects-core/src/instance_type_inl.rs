// 由 tool/src/objects/instance-type-inl.h.js 从 v8/src/objects/instance-type-inl.h 生成; 修订就地改, 复跑覆盖
// 钉死配置: 静态根关 (V8_STATIC_ROOTS_BOOL=false); 字符串位掩码/派生谓词归并
// instance_type_checker.rs 承载 (readme/zh.md 归并登记), 本模块承接唯一 map 表与静态根面登记

//! 实例类型检查器内联面 (原版 instance-type-inl.h)
//!
//! 收编说明 (轮 6 W6O1): 原版 INSTANCE_TYPE_CHECKERS/RANGE2 在静态根关基线下
//! 退化为 `Is##type(map->instance_type())`, 16 个字符串位掩码谓词与派生谓词
//! (IsSeqString/IsConsString/IsReferenceComparable/IsNativeContextSpecific 等)
//! 已在 `instance_type_checker.rs` 以同名 snake_case 函数 1:1 落地 (对账见
//! readme/zh.md 归并登记), 本文件不重复实现。此处承接两块真实缺口:
//!
//! 1. `Map::TryGetMapRootIdxFor` / `InstanceTypeChecker::UniqueMapOfInstanceType`
//!    (instance-type-inl.h:23-108): 实例类型 → 唯一只读 map 根索引。
//!    本 crate 尚无 RootIndex 类型 (归 v8-roots, 下游依赖, 破环规则禁上引),
//!    以根索引**访问器名字符串** (原版 `k##Name##Map` 的 `Name##Map` 部分)
//!    作为 1:1 载体, v8-roots 就位后替换为强类型。
//! 2. 静态根面 (`CheckInstanceMap`/`CheckInstanceMapRange`,
//!    instance-type-inl.h:229-243): 依赖 RootIndex 与 map 地址区间快查表,
//!    登记推迟 (见 readme/zh.md)。

use crate::{instance_type::InstanceType, map::Map, tagged::Tagged};

/// `Map::TryGetMapRootIdxFor` 的 STRUCT_LIST + 显式 case + wasm 全集
/// (instance-type-inl.h:23-70): 40 STRUCT_LIST + 14 显式 + 2 WASM = 56 行。
/// 原版重复登记的 WASM_FAST_API_CALL_DATA / WASM_STRING_VIEW_ITER 两行已去重。
/// 值为 `RootIndex::k##Name##Map` 的名字载体 (见模块头收编说明)。
const UNIQUE_MAP_ROOT_INDEX_TABLE: [(InstanceType, &str); 56] = [
  // STRUCT_LIST (objects/structs.h, MAKE_CASE 展开 40 项)
  (
    InstanceType::PromiseFulfillReactionJobTask,
    "PromiseFulfillReactionJobTaskMap",
  ),
  (
    InstanceType::PromiseRejectReactionJobTask,
    "PromiseRejectReactionJobTaskMap",
  ),
  (InstanceType::AsyncResumeTask, "AsyncResumeTaskMap"),
  (InstanceType::CallableTask, "CallableTaskMap"),
  (InstanceType::CallbackTask, "CallbackTaskMap"),
  (
    InstanceType::PromiseResolveThenableJobTask,
    "PromiseResolveThenableJobTaskMap",
  ),
  (InstanceType::AccessCheckInfo, "AccessCheckInfoMap"),
  (InstanceType::AccessorPair, "AccessorPairMap"),
  (
    InstanceType::AliasedArgumentsEntry,
    "AliasedArgumentsEntryMap",
  ),
  (InstanceType::AllocationMemento, "AllocationMementoMap"),
  (
    InstanceType::ArrayBoilerplateDescription,
    "ArrayBoilerplateDescriptionMap",
  ),
  (
    InstanceType::AsyncGeneratorRequest,
    "AsyncGeneratorRequestMap",
  ),
  (InstanceType::BreakPoint, "BreakPointMap"),
  (InstanceType::BreakPointInfo, "BreakPointInfoMap"),
  (InstanceType::BytecodeWrapper, "BytecodeWrapperMap"),
  (InstanceType::CallSiteInfo, "CallSiteInfoMap"),
  (InstanceType::ClassBoilerplate, "ClassBoilerplateMap"),
  (InstanceType::ClassPositions, "ClassPositionsMap"),
  (InstanceType::CodeWrapper, "CodeWrapperMap"),
  (
    InstanceType::DebugScriptScopeInfo,
    "DebugScriptScopeInfoMap",
  ),
  (InstanceType::EnumCache, "EnumCacheMap"),
  (InstanceType::ErrorStackData, "ErrorStackDataMap"),
  (
    InstanceType::ForInEnumeratorHolder,
    "ForInEnumeratorHolderMap",
  ),
  (
    InstanceType::FunctionTemplateRareData,
    "FunctionTemplateRareDataMap",
  ),
  (InstanceType::ModuleRequest, "ModuleRequestMap"),
  (InstanceType::PromiseCapability, "PromiseCapabilityMap"),
  (InstanceType::PromiseReaction, "PromiseReactionMap"),
  (
    InstanceType::PropertyDescriptorObject,
    "PropertyDescriptorObjectMap",
  ),
  (InstanceType::PrototypeInfo, "PrototypeInfoMap"),
  (
    InstanceType::PrototypeSharedClosureInfo,
    "PrototypeSharedClosureInfoMap",
  ),
  (
    InstanceType::RegExpBoilerplateDescription,
    "RegExpBoilerplateDescriptionMap",
  ),
  (InstanceType::RegExpDataWrapper, "RegExpDataWrapperMap"),
  (InstanceType::Script, "ScriptMap"),
  (InstanceType::ScriptOrModule, "ScriptOrModuleMap"),
  (
    InstanceType::SourceTextModuleInfoEntry,
    "SourceTextModuleInfoEntryMap",
  ),
  (InstanceType::StackFrameInfo, "StackFrameInfoMap"),
  (InstanceType::StackTraceInfo, "StackTraceInfoMap"),
  (
    InstanceType::TemplateObjectDescription,
    "TemplateObjectDescriptionMap",
  ),
  (InstanceType::Tuple2, "Tuple2Map"),
  (InstanceType::WasmExceptionTag, "WasmExceptionTagMap"),
  // 显式 case (instance-type-inl.h:34-61)
  (InstanceType::DescriptorArray, "DescriptorArrayMap"),
  (
    InstanceType::OnHeapBasicBlockProfilerData,
    "OnHeapBasicBlockProfilerDataMap",
  ),
  (InstanceType::TurbofanBitsetType, "TurbofanBitsetTypeMap"),
  (InstanceType::TurbofanUnionType, "TurbofanUnionTypeMap"),
  (InstanceType::TurbofanRangeType, "TurbofanRangeTypeMap"),
  (
    InstanceType::TurbofanHeapConstantType,
    "TurbofanHeapConstantTypeMap",
  ),
  (
    InstanceType::TurbofanOtherNumberConstantType,
    "TurbofanOtherNumberConstantTypeMap",
  ),
  (
    InstanceType::TurboshaftWord32RangeType,
    "TurboshaftWord32RangeTypeMap",
  ),
  (
    InstanceType::TurboshaftWord32SetType,
    "TurboshaftWord32SetTypeMap",
  ),
  (
    InstanceType::TurboshaftWord64RangeType,
    "TurboshaftWord64RangeTypeMap",
  ),
  (
    InstanceType::TurboshaftWord64SetType,
    "TurboshaftWord64SetTypeMap",
  ),
  (
    InstanceType::TurboshaftFloat64RangeType,
    "TurboshaftFloat64RangeTypeMap",
  ),
  (
    InstanceType::TurboshaftFloat64SetType,
    "TurboshaftFloat64SetTypeMap",
  ),
  (InstanceType::SortState, "SortStateMap"),
  // V8_ENABLE_WEBASSEMBLY 分支 (钉死配置开)
  (InstanceType::WasmFastApiCallData, "WasmFastApiCallDataMap"),
  (InstanceType::WasmStringViewIter, "WasmStringViewIterMap"),
];

const _: () = assert!(UNIQUE_MAP_ROOT_INDEX_TABLE.len() == 56);

/// 实例类型 → 唯一只读 map 根索引名 (miss = None)。
///
/// 原版 `InstanceTypeChecker::UniqueMapOfInstanceType`
/// (instance-type-inl.h:96-108): 先按 `INSTANCE_TYPE_CHECKERS_SINGLE` 模板
/// 链查 `UniqueMapOfInstanceTypeCheck<type>()`, 未命中兜底
/// `Map::TryGetMapRootIdxFor`。本表已合并两路 (SINGLE 桶生成项与
/// STRUCT_LIST/显式项重叠部分原样共表, 行为全等)。
///
/// v8: src/objects/instance-type-inl.h InstanceTypeChecker::UniqueMapOfInstanceType
#[inline]
#[must_use]
pub const fn unique_map_of_instance_type(ty: InstanceType) -> Option<&'static str> {
  let mut i = 0;
  while i < UNIQUE_MAP_ROOT_INDEX_TABLE.len() {
    let (key, name) = UNIQUE_MAP_ROOT_INDEX_TABLE[i];
    if key.raw() == ty.raw() {
      return Some(name);
    }
    i += 1;
  }
  None
}

// ===== 静态根面 (钉死基线 V8_STATIC_ROOTS_BOOL=false, 登记不展开) =====
/// 静态根开关下 map 地址区间快查表条目数 (kUniqueMapRangeOfInstanceTypeRangeList, 原版 9)
pub const STATIC_ROOTS_RANGE_LIST_LEN: usize = 9;

/// 字符串 map 地址分段 (kUniqueMapRangeOfStringType, 11 段; 静态根开时生效)
pub const STRING_MAP_RANGES: [&str; 11] = [
  "SeqString",
  "InternalizedString",
  "ExternalString",
  "UncachedExternalString",
  "ConsString",
  "SlicedString",
  "ThinString",
  "SharedSeqString",
  "SharedExternalString",
  "DirectString",
  "IndirectString",
];

/// map 一致性检查 (静态根开: 压缩值比 StaticReadOnlyRootsPointerTable; 关: 退化为 instance_type 比较)
///
/// v8: src/objects/instance-type-inl.h InstanceTypeChecker::CheckInstanceMap
#[inline]
pub fn check_instance_map(expected: InstanceType, map: Tagged<Map>) -> bool {
  // 静态根关闭基线：退化为 instance_type 比较
  unsafe { map.as_ref().instance_type() == expected }
}
