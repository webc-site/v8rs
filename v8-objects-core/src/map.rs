// 由 tool/src/objects/map.h.js 从 v8/src/objects/map.h + map-inl.h 生成; 修订就地改, 复跑覆盖
// visitor id 三列表 + Bits1/2/3/Ex 位段 + 布局常量机械展开; transition 深层推迟归 v8-objects-maps (脚本头登记)

//! Map: 描述堆对象形状的元对象 (实例大小 / 位域标志 / 属性描述结构)。
//!
//! 偏移常量按压缩指针默认配置 (K_TAGGED_SIZE=4) 对齐原版堆布局
//! (snapshot 兼容前提); 8 字节 tagged 配置经 v8_common::globals::TAGGED_SIZE_8_BYTES
//! 常量分支。Rust 结构体为逻辑布局 (Tagged 为 usize 宽), 槽位读写走
//! tagged_field 机制。

use std::mem::transmute;

use v8_common::{ExtendedMapKind, K_TAGGED_SIZE, K_TAGGED_SIZE_LOG2, TAGGED_SIZE_8_BYTES};

use crate::{
  error,
  error::Error,
  heap_object::HeapObject,
  instance_type::InstanceType,
  instance_type_checker::{
    is_always_shared_space_js_object, is_js_object, is_maybe_read_only_js_object,
  },
  property_details::{DESCRIPTOR_INDEX_BIT_COUNT, MAX_NUMBER_OF_DESCRIPTORS},
  tagged::{Object, Tagged},
};

// ── VisitorId (map.h:34-160, 三列表 X-macro 机械展开) ─────────────────────
/// 堆访客 id: 同 id 的对象被堆访客以相同方式处理。
/// data-only 对象的 id 必须排在其它 id 之前 (kDataOnlyVisitorIdCount 快查依据)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum VisitorId {
  VisitBigInt,
  VisitCoverageInfo,
  VisitFeedbackMetadata,
  VisitFiller,
  VisitHeapNumber,
  VisitHashSeedWrapper,
  VisitUninitializedHeapNumber,
  VisitHole,
  VisitSeqOneByteString,
  VisitSeqTwoByteString,
  VisitTurbofanBitsetType,
  VisitTurbofanOtherNumberConstantType,
  VisitTurbofanRangeType,
  VisitTurboshaftFloat64RangeType,
  VisitTurboshaftFloat64SetType,
  VisitTurboshaftWord32RangeType,
  VisitTurboshaftWord32SetType,
  VisitTurboshaftWord64RangeType,
  VisitTurboshaftWord64SetType,
  VisitWasmNull,
  /// data-only 访客 id 数 (map.h:154 kDataOnlyVisitorIdCount)。
  DataOnlyVisitorIdCount,
  VisitAccessorInfo,
  VisitAllocationSite,
  VisitBytecodeWrapper,
  VisitCallSiteInfo,
  VisitCell,
  VisitCodeWrapper,
  VisitConsString,
  VisitContext,
  VisitContextCell,
  VisitCppHeapExternalObject,
  VisitCppGCManagedBase,
  VisitDataHandler,
  VisitDescriptorArray,
  VisitDoubleStringCache,
  VisitEmbedderDataArray,
  VisitEphemeronHashTable,
  VisitExternalString,
  VisitFeedbackCell,
  VisitFeedbackVector,
  VisitForeign,
  VisitFreeSpace,
  VisitFunctionTemplateInfo,
  VisitInterceptorInfo,
  VisitJSApiObject,
  VisitJSArrayBuffer,
  VisitJSDataViewOrRabGsabDataView,
  VisitJSDate,
  VisitJSExternalObject,
  VisitJSFinalizationRegistry,
  VisitJSFunction,
  VisitJSGlobalProxy,
  VisitJSObject,
  VisitJSObjectFast,
  VisitJSProxy,
  VisitJSRegExp,
  VisitJSSynchronizationPrimitive,
  VisitJSTypedArray,
  VisitJSWeakCollection,
  VisitJSWeakRef,
  VisitMap,
  VisitMegaDomHandler,
  VisitNativeContext,
  VisitOddball,
  VisitOnHeapBasicBlockProfilerData,
  VisitPreparseData,
  VisitPropertyArray,
  VisitPropertyCell,
  VisitPrototypeInfo,
  VisitPrototypeSharedClosureInfo,
  VisitRegExpBoilerplateDescription,
  VisitRegExpDataWrapper,
  VisitScopeInfo,
  VisitSharedFunctionInfo,
  VisitShortcutCandidate,
  VisitSlicedString,
  VisitSmallOrderedHashMap,
  VisitSmallOrderedHashSet,
  VisitSmallOrderedNameDictionary,
  VisitSortState,
  VisitSourceTextModule,
  VisitStruct,
  VisitSwissNameDictionary,
  VisitSymbol,
  VisitTurbofanHeapConstantType,
  VisitTurbofanUnionType,
  VisitSyntheticModule,
  VisitThinString,
  VisitTransitionArray,
  VisitWasmArray,
  VisitWasmCustomMap,
  VisitWasmFuncRef,
  VisitWasmGlobalObject,
  VisitWasmInstanceObject,
  VisitWasmMemoryObject,
  VisitWasmResumeData,
  VisitWasmStruct,
  VisitWasmSuspendingObject,
  VisitWasmContinuationObject,
  VisitWasmFastApiCallData,
  VisitWasmStackObject,
  VisitWasmStringViewIter,
  VisitWasmTableObject,
  VisitWasmTagObject,
  VisitWasmTypeInfo,
  VisitWeakCell,
  VisitArrayList,
  VisitByteArray,
  VisitClosureFeedbackCellArray,
  VisitFixedArray,
  VisitFixedDoubleArray,
  VisitObjectBoilerplateDescription,
  VisitRegExpMatchInfo,
  VisitScriptContextTable,
  VisitSloppyArgumentsElements,
  VisitWeakArrayList,
  VisitWeakFixedArray,
  VisitWeakHomomorphicFixedArray,
  VisitBytecodeArray,
  VisitCode,
  VisitDebugInfo,
  VisitInstructionStream,
  VisitInterpreterData,
  VisitUncompiledDataWithPreparseData,
  VisitUncompiledDataWithoutPreparseData,
  VisitUncompiledDataWithPreparseDataAndJob,
  VisitUncompiledDataWithoutPreparseDataWithJob,
  VisitSharedFunctionInfoWrapper,
  VisitProtectedFixedArray,
  VisitProtectedWeakFixedArray,
  VisitTrustedByteArray,
  VisitTrustedFixedArray,
  VisitTrustedForeign,
  VisitTrustedWeakFixedArray,
  VisitAtomRegExpData,
  VisitIrRegExpData,
  VisitRegExpData,
  VisitWasmImportData,
  VisitWasmCapiFunctionData,
  VisitWasmDispatchTable,
  VisitWasmDispatchTableForImports,
  VisitWasmExportedFunctionData,
  VisitWasmInternalFunction,
  VisitWasmTrustedInstanceData,
  VisitWasmSuspenderObject,
  /// 访客 id 总数 (map.h:157 kVisitorIdCount)。
  VisitorIdCount,
}

const _: () = assert!((VisitorId::VisitorIdCount as usize) < 256);

/// 对象字段类别 (map.h:164-167)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectFields {
  /// 纯数据对象 (无指针)。
  DataOnly,
  /// 可能含指针。
  MaybePointers,
}

/// 由访客 id 推对象字段类别 (map.h:1051-1055)。
#[inline]
#[must_use]
pub const fn object_fields_from(visitor_id: VisitorId) -> ObjectFields {
  if (visitor_id as u8) < (VisitorId::DataOnlyVisitorIdCount as u8) {
    ObjectFields::DataOnly
  } else {
    ObjectFields::MaybePointers
  }
}

// ── Bits1 (map.h:338-356; bit_field, u8) ─────────────────────────────────
// 编译器把 is_callable|is_undetectable 与 has_named_interceptor|
// is_access_check_needed 各当一个掩码用 (typeof / special receiver 检查),
// 保持每对位相邻 → arm64 逻辑立即数可编码。常量在 impl Map 内关联承载。

// ── Bits2 (map.h:373-385; bit_field2, u8) ────────────────────────────────

// ── Bits3 (map.h:391-421; bit_field3, u32) ───────────────────────────────

// ── BitsEx (map.h:1269-1277; bit_field_ex, u8; ExtendedMap 专用) ─────────
pub const MAP_KIND_SHIFT: u8 = 0;
pub const MAP_KIND_SIZE: u8 = 3;
pub const MAP_KIND_MASK: u8 = (1u8 << 3) - 1;
pub const MAP_SIZE_IN_WORDS_SHIFT: u8 = 3;
pub const MAP_SIZE_IN_WORDS_SIZE: u8 = 5;
pub const MAP_SIZE_IN_WORDS_MASK: u8 = ((1u8 << 5) - 1) << 3;

// 编译期布局对数 (对齐原版 static_assert 组, map.h:424-429, 446)
const _: () = assert!(Map::ENUM_LENGTH_SIZE == DESCRIPTOR_INDEX_BIT_COUNT);
const _: () = assert!(Map::NUMBER_OF_OWN_DESCRIPTORS_SIZE == DESCRIPTOR_INDEX_BIT_COUNT);
const _: () = assert!((1 << Map::NUMBER_OF_OWN_DESCRIPTORS_SIZE) > MAX_NUMBER_OF_DESCRIPTORS);
const _: () = assert!(SLACK_TRACKING_COUNTER_START < (1 << Map::CONSTRUCTION_COUNTER_SIZE));

/// 描述符索引哨兵 (property-details.h, 位宽与 Bits3 对齐锚点)。
pub use crate::property_details::INVALID_ENUM_CACHE_SENTINEL;

// ── slack tracking 常量 (map.h:443-479) ──────────────────────────────────
/// inobject slack tracking 起始计数。
pub const SLACK_TRACKING_COUNTER_START: u32 = 7;
/// slack tracking 结束计数。
pub const SLACK_TRACKING_COUNTER_END: u32 = 1;
/// 不进行 slack tracking 的计数哨兵。
pub const NO_SLACK_TRACKING: u32 = 0;
/// 宽裕分配计数 (start - end + 1)。
pub const GENEROUS_ALLOCATION_COUNT: u32 =
  SLACK_TRACKING_COUNTER_START - SLACK_TRACKING_COUNTER_END + 1;

// ── 布局常量 (对齐原版堆布局; 压缩指针默认 K_TAGGED_SIZE=4) ────────────────
/// Map 指针槽偏移 (heap 对象首字段)。
pub const K_MAP_OFFSET: usize = 0;
/// instance_size_in_words_ 字节偏移 (第一个 int 字段首字节)。
pub const K_INSTANCE_SIZE_IN_WORDS_OFFSET: usize = K_TAGGED_SIZE;
/// inobject_properties_start_or_constructor_function_index_ 偏移。
pub const K_INOBJECT_PROPERTIES_START_OR_CONSTRUCTOR_FUNCTION_INDEX_OFFSET: usize =
  K_TAGGED_SIZE + 1;
/// used_or_unused_instance_size_in_words_ 偏移。
pub const K_USED_OR_UNUSED_INSTANCE_SIZE_IN_WORDS_OFFSET: usize = K_TAGGED_SIZE + 2;
/// visitor_id_ 偏移。
pub const K_VISITOR_ID_OFFSET: usize = K_TAGGED_SIZE + 3;
/// instance_type_ 偏移 (== Internals::kMapInstanceTypeOffset = kTaggedSize + 4)。
pub const K_INSTANCE_TYPE_OFFSET: usize = K_TAGGED_SIZE + 4;
/// bit_field_ (Bits1) 偏移。
pub const K_BIT_FIELD_OFFSET: usize = K_TAGGED_SIZE + 6;
/// bit_field2_ (Bits2) 偏移。
pub const K_BIT_FIELD2_OFFSET: usize = K_TAGGED_SIZE + 7;
/// bit_field3_ (Bits3) 偏移。
pub const K_BIT_FIELD3_OFFSET: usize = K_TAGGED_SIZE + 8;
/// bit_field 段末字节偏移 (map.h:1251-1252 kBitFieldOffsetEnd)。
pub const K_BIT_FIELD_OFFSET_END: usize = K_BIT_FIELD_OFFSET;
/// 第一个 tagged 指针字段 (prototype_) 偏移;
/// 8 字节 tagged 配置下 bit_field3 后有 4 字节 optional padding (map.h:1231)。
pub const K_PROTOTYPE_OFFSET: usize = if TAGGED_SIZE_8_BYTES {
  K_TAGGED_SIZE + 16
} else {
  K_TAGGED_SIZE + 12
};
/// constructor_or_back_pointer_or_native_context_ 偏移。
pub const K_CONSTRUCTOR_OR_BACK_POINTER_OR_NATIVE_CONTEXT_OFFSET: usize =
  K_PROTOTYPE_OFFSET + K_TAGGED_SIZE;
/// instance_descriptors_ 偏移 (非 wasm: DescriptorArray)。
pub const K_INSTANCE_DESCRIPTORS_OFFSET: usize =
  K_CONSTRUCTOR_OR_BACK_POINTER_OR_NATIVE_CONTEXT_OFFSET + K_TAGGED_SIZE;
/// dependent_code_ 偏移 (非 wasm)。
pub const K_DEPENDENT_CODE_OFFSET: usize = K_INSTANCE_DESCRIPTORS_OFFSET + K_TAGGED_SIZE;
/// prototype_validity_cell_ 偏移。
pub const K_PROTOTYPE_VALIDITY_CELL_OFFSET: usize = K_DEPENDENT_CODE_OFFSET + K_TAGGED_SIZE;
/// transitions_or_prototype_info_ 偏移 (最后一个字段)。
pub const K_TRANSITIONS_OR_PROTOTYPE_INFO_OFFSET: usize =
  K_PROTOTYPE_VALIDITY_CELL_OFFSET + K_TAGGED_SIZE;
/// 弱字段段末 (map.h:1219 kEndOfWeakFieldsOffset = sizeof(Map))。
pub const K_END_OF_WEAK_FIELDS_OFFSET: usize =
  K_TRANSITIONS_OR_PROTOTYPE_INFO_OFFSET + K_TAGGED_SIZE;
/// Map 头大小 (map.h:1220)。
pub const K_HEADER_SIZE: usize = K_END_OF_WEAK_FIELDS_OFFSET;
/// Map 自身大小 (map.h:1221)。
pub const K_SIZE: usize = K_END_OF_WEAK_FIELDS_OFFSET;

const _: () = assert!(K_INSTANCE_TYPE_OFFSET == K_TAGGED_SIZE + 4); // Internals 锚点 (原版 1*kApiTaggedSize + kApiInt32Size)
const _: () = assert!(K_SIZE == K_PROTOTYPE_OFFSET + 6 * K_TAGGED_SIZE); // 头 + 6 指针字段
const _: () = assert!(!TAGGED_SIZE_8_BYTES && K_SIZE == 40); // 压缩默认锚点

/// 变长对象实例尺寸哨兵 (words = 0)。
pub const K_VARIABLE_SIZE_SENTINEL_IN_WORDS: u8 = 0;

/// JSObject::kFieldsAdded (js-objects.h:982, 借位登记: JS 对象族 crate 落地后迁回)。
pub const JS_OBJECT_FIELDS_ADDED: i32 = 3;

/// 对象形状描述元对象 (Map)
///
/// 描述堆对象的实例类型、内存大小、位域标志及属性字典结构。
/// 字段序与原版一致 (map.h:1223-1246); tagged 指针字段以 Tagged<Object>
/// 逻辑占位, 具体类型窄化视图归后续对象族 crate。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Map {
  /// 继承自 HeapObject，指向 Map 自身的元 Map
  pub header: HeapObject,
  /// 实例大小 (单位: word; 0 = kVariableSizeSentinel) (map.h:1223)
  pub instance_size_in_words: u8,
  /// JSObject map: 对象内属性起始 word 偏移; 原始类型 map: 构造函数在
  /// native context 中的序号 (map.h:1224)
  pub inobject_properties_start_or_constructor_function_index: u8,
  /// 见 used_or_unused_instance_size_in_words (map.h:1225; 双义编码, 详见
  /// unused_property_fields)
  pub used_or_unused_instance_size_in_words: u8,
  /// 堆访客 id (map.h:1226)
  pub visitor_id: u8,
  /// 实例类型 (map.h:1227; 原版存储为 u16)
  pub instance_type: InstanceType,
  /// 位域 1 (Bits1; map.h:1228)
  pub bit_field: u8,
  /// 位域 2 (Bits2; map.h:1229)
  pub bit_field2: u8,
  /// 位域 3 (Bits3; map.h:1230)
  pub bit_field3: u32,
  /// 隐式原型对象 (JSPrototype; map.h:1234)
  pub prototype: Tagged<Object>,
  /// 构造函数 / 回指针 / native context 三重叠槽 (map.h:1235)
  pub constructor_or_back_pointer_or_native_context: Tagged<Object>,
  /// 实例描述符数组 (DescriptorArray; map.h:1240)
  pub instance_descriptors: Tagged<Object>,
  /// 依赖代码表 (DependentCode; map.h:1241)
  pub dependent_code: Tagged<Object>,
  /// 原型链有效性 cell (UnionOf<Zero, Cell>; map.h:1243)
  pub prototype_validity_cell: Tagged<Object>,
  /// transitions / prototype_info 重叠槽 (map.h:1244; 类型派发归 v8-objects-maps)
  pub transitions_or_prototype_info: Tagged<Object>,
}

impl Map {
  // ── Bits1/2/3 位段常量 (关联承载; 见文件头部布局登记) ────────────────────
  pub const IS_CALLABLE_SHIFT: u8 = 0;
  pub const IS_CALLABLE_SIZE: u8 = 1;
  pub const IS_CALLABLE_MASK: u8 = (1u8 << 1) - 1;
  pub const IS_UNDETECTABLE_SHIFT: u8 = 1;
  pub const IS_UNDETECTABLE_SIZE: u8 = 1;
  pub const IS_UNDETECTABLE_MASK: u8 = ((1u8 << 1) - 1) << 1;
  pub const HAS_NAMED_INTERCEPTOR_SHIFT: u8 = 2;
  pub const HAS_NAMED_INTERCEPTOR_SIZE: u8 = 1;
  pub const HAS_NAMED_INTERCEPTOR_MASK: u8 = ((1u8 << 1) - 1) << 2;
  pub const IS_ACCESS_CHECK_NEEDED_SHIFT: u8 = 3;
  pub const IS_ACCESS_CHECK_NEEDED_SIZE: u8 = 1;
  pub const IS_ACCESS_CHECK_NEEDED_MASK: u8 = ((1u8 << 1) - 1) << 3;
  pub const HAS_INDEXED_INTERCEPTOR_SHIFT: u8 = 4;
  pub const HAS_INDEXED_INTERCEPTOR_SIZE: u8 = 1;
  pub const HAS_INDEXED_INTERCEPTOR_MASK: u8 = ((1u8 << 1) - 1) << 4;
  pub const IS_CONSTRUCTOR_SHIFT: u8 = 5;
  pub const IS_CONSTRUCTOR_SIZE: u8 = 1;
  pub const IS_CONSTRUCTOR_MASK: u8 = ((1u8 << 1) - 1) << 5;
  pub const IS_EXTENDED_MAP_SHIFT: u8 = 6;
  pub const IS_EXTENDED_MAP_SIZE: u8 = 1;
  pub const IS_EXTENDED_MAP_MASK: u8 = ((1u8 << 1) - 1) << 6;
  pub const NEW_TARGET_IS_BASE_SHIFT: u8 = 0;
  pub const NEW_TARGET_IS_BASE_SIZE: u8 = 1;
  pub const NEW_TARGET_IS_BASE_MASK: u8 = (1u8 << 1) - 1;
  pub const IS_IMMUTABLE_PROTOTYPE_SHIFT: u8 = 1;
  pub const IS_IMMUTABLE_PROTOTYPE_SIZE: u8 = 1;
  pub const IS_IMMUTABLE_PROTOTYPE_MASK: u8 = ((1u8 << 1) - 1) << 1;
  pub const ELEMENTS_KIND_SHIFT: u8 = 2;
  pub const ELEMENTS_KIND_SIZE: u8 = 6;
  pub const ELEMENTS_KIND_MASK: u8 = ((1u8 << 6) - 1) << 2;
  pub const ENUM_LENGTH_SHIFT: u32 = 0;
  pub const ENUM_LENGTH_SIZE: u32 = 10;
  pub const ENUM_LENGTH_MASK: u32 = (1u32 << 10) - 1;
  pub const NUMBER_OF_OWN_DESCRIPTORS_SHIFT: u32 = 10;
  pub const NUMBER_OF_OWN_DESCRIPTORS_SIZE: u32 = 10;
  pub const NUMBER_OF_OWN_DESCRIPTORS_MASK: u32 = ((1u32 << 10) - 1) << 10;
  pub const IS_PROTOTYPE_MAP_SHIFT: u32 = 20;
  pub const IS_PROTOTYPE_MAP_SIZE: u32 = 1;
  pub const IS_PROTOTYPE_MAP_MASK: u32 = ((1u32 << 1) - 1) << 20;
  pub const IS_DICTIONARY_MAP_SHIFT: u32 = 21;
  pub const IS_DICTIONARY_MAP_SIZE: u32 = 1;
  pub const IS_DICTIONARY_MAP_MASK: u32 = ((1u32 << 1) - 1) << 21;
  pub const OWNS_DESCRIPTORS_SHIFT: u32 = 22;
  pub const OWNS_DESCRIPTORS_SIZE: u32 = 1;
  pub const OWNS_DESCRIPTORS_MASK: u32 = ((1u32 << 1) - 1) << 22;
  pub const IS_IN_RETAINED_MAP_LIST_SHIFT: u32 = 23;
  pub const IS_IN_RETAINED_MAP_LIST_SIZE: u32 = 1;
  pub const IS_IN_RETAINED_MAP_LIST_MASK: u32 = ((1u32 << 1) - 1) << 23;
  pub const IS_DEPRECATED_SHIFT: u32 = 24;
  pub const IS_DEPRECATED_SIZE: u32 = 1;
  pub const IS_DEPRECATED_MASK: u32 = ((1u32 << 1) - 1) << 24;
  pub const IS_UNSTABLE_SHIFT: u32 = 25;
  pub const IS_UNSTABLE_SIZE: u32 = 1;
  pub const IS_UNSTABLE_MASK: u32 = ((1u32 << 1) - 1) << 25;
  pub const IS_MIGRATION_TARGET_SHIFT: u32 = 26;
  pub const IS_MIGRATION_TARGET_SIZE: u32 = 1;
  pub const IS_MIGRATION_TARGET_MASK: u32 = ((1u32 << 1) - 1) << 26;
  pub const IS_EXTENSIBLE_SHIFT: u32 = 27;
  pub const IS_EXTENSIBLE_SIZE: u32 = 1;
  pub const IS_EXTENSIBLE_MASK: u32 = ((1u32 << 1) - 1) << 27;
  pub const MAY_HAVE_INTERESTING_PROPERTIES_SHIFT: u32 = 28;
  pub const MAY_HAVE_INTERESTING_PROPERTIES_SIZE: u32 = 1;
  pub const MAY_HAVE_INTERESTING_PROPERTIES_MASK: u32 = ((1u32 << 1) - 1) << 28;
  pub const CONSTRUCTION_COUNTER_SHIFT: u32 = 29;
  pub const CONSTRUCTION_COUNTER_SIZE: u32 = 3;
  pub const CONSTRUCTION_COUNTER_MASK: u32 = ((1u32 << 3) - 1) << 29;

  /// 构造 Map 实例 (instance_size 单位: 字节, 须 word 对齐; 0 = 变长哨兵)
  #[inline]
  pub const fn new(
    meta_map: Tagged<Map>,
    instance_type: InstanceType,
    instance_size: usize,
  ) -> Self {
    debug_assert!(instance_size == 0 || instance_size.is_multiple_of(K_TAGGED_SIZE));
    Self {
      header: HeapObject::new(meta_map),
      instance_size_in_words: (instance_size >> K_TAGGED_SIZE_LOG2) as u8,
      inobject_properties_start_or_constructor_function_index: 0,
      used_or_unused_instance_size_in_words: 0,
      visitor_id: 0,
      instance_type,
      bit_field: 0,
      bit_field2: 0,
      bit_field3: 0,
      prototype: Tagged::NULL,
      constructor_or_back_pointer_or_native_context: Tagged::NULL,
      instance_descriptors: Tagged::NULL,
      dependent_code: Tagged::NULL,
      prototype_validity_cell: Tagged::NULL,
      transitions_or_prototype_info: Tagged::NULL,
    }
  }

  // ── 实例尺寸 (map-inl.h:400-419) ────────────────────────────────────────

  /// 实例大小 (单位: word)。
  #[inline]
  pub const fn instance_size_in_words(&self) -> u8 {
    self.instance_size_in_words
  }

  /// 设置实例大小 (单位: word)。
  #[inline]
  pub const fn set_instance_size_in_words(&mut self, value: u8) {
    self.instance_size_in_words = value;
  }

  /// 获取实例分配字节大小 (words << kTaggedSizeLog2; 0 = 变长哨兵)。
  #[inline]
  pub const fn instance_size(&self) -> usize {
    (self.instance_size_in_words as usize) << K_TAGGED_SIZE_LOG2
  }

  /// 设置实例分配字节大小 (须 word 对齐, map-inl.h:413 CHECK 对齐)。
  #[inline]
  pub const fn set_instance_size(&mut self, size_in_bytes: usize) {
    debug_assert!(size_in_bytes.is_multiple_of(K_TAGGED_SIZE));
    debug_assert!(size_in_bytes >> K_TAGGED_SIZE_LOG2 <= u8::MAX as usize);
    self.instance_size_in_words = (size_in_bytes >> K_TAGGED_SIZE_LOG2) as u8;
  }

  /// Map 自身占用大小 (非 extended 分支; = K_SIZE)。
  #[inline]
  pub const fn size(&self) -> usize {
    K_SIZE
  }

  /// Map 自身实际分配大小 (map-inl.h:495)。extended map 的尺寸经
  /// BitsEx 叠读, 视图归 v8-objects-maps — 此处落 Err 登记。
  #[inline]
  pub fn allocated_size(&self) -> error::Result<usize> {
    if self.is_extended_map() {
      Err(Error::Unsupported(
        "AllocatedSize: extended map 尺寸归 v8-objects-maps",
      ))
    } else {
      Ok(K_SIZE)
    }
  }

  /// 获取实例类型
  #[inline]
  pub const fn instance_type(&self) -> InstanceType {
    self.instance_type
  }

  /// 设置实例类型
  #[inline]
  pub fn set_instance_type(&mut self, instance_type: InstanceType) {
    self.instance_type = instance_type;
  }

  // ── visitor id (map-inl.h:391-398) ──────────────────────────────────────

  /// 堆访客 id (越界字节 → None; 原版 static_cast 无检查, 存入前经
  /// set_visitor_id 校验)。
  #[inline]
  #[must_use]
  pub const fn visitor_id(&self) -> Option<VisitorId> {
    visitor_id_from_u8(self.visitor_id)
  }

  /// 设置堆访客 id (原版 CHECK id < 256 → debug_assert)。
  #[inline]
  pub fn set_visitor_id(&mut self, id: VisitorId) {
    debug_assert!((id as u8 as usize) < 256);
    self.visitor_id = id as u8;
  }

  // ── inobject 属性 / 构造函数序号双义槽 (map-inl.h:421-483) ───────────────

  /// 双义槽原始字节。
  #[inline]
  pub const fn inobject_properties_start_or_constructor_function_index(&self) -> u8 {
    self.inobject_properties_start_or_constructor_function_index
  }

  /// 设置双义槽原始字节。
  #[inline]
  pub const fn set_inobject_properties_start_or_constructor_function_index(&mut self, value: u8) {
    self.inobject_properties_start_or_constructor_function_index = value;
  }

  /// 对象内属性区起始 word 偏移 (JSObject map; 原版 DCHECK IsJSObjectMap
  /// 由调用方保证)。
  #[inline]
  pub const fn get_in_object_properties_start_in_words(&self) -> u8 {
    self.inobject_properties_start_or_constructor_function_index()
  }

  /// 设置对象内属性区起始 word 偏移。
  #[inline]
  pub const fn set_in_object_properties_start_in_words(&mut self, value: u8) {
    self.set_inobject_properties_start_or_constructor_function_index(value);
  }

  /// 对象内属性数 (JSObject map; instance_size - start)。
  #[inline]
  pub const fn get_in_object_properties(&self) -> i32 {
    self.instance_size_in_words as i32 - self.get_in_object_properties_start_in_words() as i32
  }

  /// 字段序号是否落在对象内 (field_index < inobject 数)。
  #[inline]
  pub const fn is_field_in_object(&self, field_index: i32) -> bool {
    field_index < self.get_in_object_properties()
  }

  /// 对象内第 index 个属性的 byte 偏移 ((start + index) * kTaggedSize)。
  #[inline]
  pub const fn get_in_object_property_offset(&self, index: i32) -> i32 {
    (self.get_in_object_properties_start_in_words() as i32 + index) * K_TAGGED_SIZE as i32
  }

  /// 无对象包装器哨兵 (原始类型 map; map.h:284 kNoConstructorFunctionIndex)。
  pub const NO_CONSTRUCTOR_FUNCTION_INDEX: i32 = 0;

  /// 构造函数在 native context 中的序号 (原始类型 map; wasm 分支原版特判,
  /// 此处由调用方保证原始类型 map 前置)。
  #[inline]
  pub const fn get_constructor_function_index(&self) -> i32 {
    self.inobject_properties_start_or_constructor_function_index() as i32
  }

  /// 设置构造函数序号 (原始类型 map)。
  #[inline]
  pub const fn set_constructor_function_index(&mut self, value: i32) {
    debug_assert!(value >= 0 && value <= u8::MAX as i32);
    self.set_inobject_properties_start_or_constructor_function_index(value as u8);
  }

  // ── used/unused 记账 (map-inl.h:507-632) ────────────────────────────────
  // 该字节编码双义: 值 >= JSObject::kFieldsAdded(3) 时为 "已用到的 word 偏移"
  // (unused = instance_size - value, 全部属性在对象内); 否则为属性数组中的
  // slack 槽位数 (unused = value)。

  /// 双义记账字节原始值。
  #[inline]
  pub const fn used_or_unused_instance_size_in_words(&self) -> u8 {
    self.used_or_unused_instance_size_in_words
  }

  /// 设置双义记账字节原始值 (原版 CHECK <= 255 → u8 天然满足)。
  #[inline]
  pub const fn set_used_or_unused_instance_size_in_words(&mut self, value: u8) {
    self.used_or_unused_instance_size_in_words = value;
  }

  /// 可用未用属性槽数 (对象内或属性数组; map-inl.h:507)。
  #[inline]
  pub const fn unused_property_fields(&self) -> i32 {
    let value = self.used_or_unused_instance_size_in_words() as i32;
    if value >= JS_OBJECT_FIELDS_ADDED {
      self.instance_size_in_words() as i32 - value
    } else {
      value
    }
  }

  /// 对象内未用属性槽数 (map-inl.h:524; 外置属性时恒 0)。
  #[inline]
  pub const fn unused_in_object_properties(&self) -> i32 {
    let value = self.used_or_unused_instance_size_in_words() as i32;
    if value >= JS_OBJECT_FIELDS_ADDED {
      self.instance_size_in_words() as i32 - value
    } else {
      0
    }
  }

  /// 是否有外置属性 (map-inl.h:448)。
  #[inline]
  pub const fn has_out_of_object_properties(&self) -> bool {
    (self.used_or_unused_instance_size_in_words() as i32) < JS_OBJECT_FIELDS_ADDED
  }

  /// 已用实例大小 (含对象头; map-inl.h:548)。
  #[inline]
  pub const fn used_instance_size(&self) -> usize {
    let words = self.used_or_unused_instance_size_in_words() as i32;
    if words < JS_OBJECT_FIELDS_ADDED {
      // 对象内属性全用完, 字节在计属性数组 slack → 全实例大小。
      return self.instance_size();
    }
    (words as usize) << K_TAGGED_SIZE_LOG2
  }

  /// 设置对象内未用槽数 (map-inl.h:561; 非 JSObject map 须为 0)。
  #[inline]
  pub const fn set_in_object_unused_property_fields(&mut self, value: i32) {
    debug_assert!(value >= 0);
    let used_inobject_properties = self.get_in_object_properties() - value;
    self.set_used_or_unused_instance_size_in_words(
      (self.get_in_object_property_offset(used_inobject_properties) / K_TAGGED_SIZE as i32) as u8,
    );
    debug_assert!(self.unused_property_fields() == value);
  }

  /// 设置属性数组未用槽数 (map-inl.h:576; value < kFieldsAdded)。
  #[inline]
  pub const fn set_out_of_object_unused_property_fields(&mut self, value: i32) {
    debug_assert!(value < JS_OBJECT_FIELDS_ADDED && value >= 0);
    self.set_used_or_unused_instance_size_in_words(value as u8);
    debug_assert!(self.unused_property_fields() == value);
  }

  /// 复制未用槽数记账 (map-inl.h:585)。
  #[inline]
  pub const fn copy_unused_property_fields(&mut self, map: &Map) {
    self.set_used_or_unused_instance_size_in_words(map.used_or_unused_instance_size_in_words());
  }

  /// 复制未用槽数记账并按实例大小差校正 (map-inl.h:591)。
  #[inline]
  pub const fn copy_unused_property_fields_adjusted_for_instance_size(&mut self, map: &Map) {
    let mut value = map.used_or_unused_instance_size_in_words() as i32;
    if value >= JS_OBJECT_FIELDS_ADDED {
      // 对象内未用字段: 校正偏移使其匹配到对象末端的距离。
      value += self.instance_size_in_words() as i32 - map.instance_size_in_words() as i32;
    }
    self.set_used_or_unused_instance_size_in_words(value as u8);
  }

  /// 记账一次新增属性字段 (map-inl.h:602)。
  #[inline]
  pub const fn account_added_property_field(&mut self) {
    let value = self.used_or_unused_instance_size_in_words() as i32;
    if value >= JS_OBJECT_FIELDS_ADDED {
      if value == self.instance_size_in_words() as i32 {
        // 对象内已满 → 转外置, 属性数组 slack 从 0 起算再扣 1。
        self.account_added_out_of_object_property_field(0);
      } else {
        // 属性加在对象内, 计数 +1。
        self.set_used_or_unused_instance_size_in_words((value + 1) as u8);
      }
    } else {
      self.account_added_out_of_object_property_field(value);
    }
  }

  /// 记账一次新增外置属性字段 (map-inl.h:623)。
  #[inline]
  pub const fn account_added_out_of_object_property_field(
    &mut self,
    unused_in_property_array: i32,
  ) {
    let mut unused = unused_in_property_array - 1;
    if unused < 0 {
      unused += JS_OBJECT_FIELDS_ADDED;
    }
    debug_assert!(unused < JS_OBJECT_FIELDS_ADDED && unused >= 0);
    self.set_used_or_unused_instance_size_in_words(unused as u8);
  }

  // ── slack tracking (map.h:443-493; map-inl.h:1247; map.cc:2662) ─────────

  /// slack tracking 是否进行中 (counter != kNoSlackTracking)。
  #[inline]
  pub const fn is_inobject_slack_tracking_in_progress(&self) -> bool {
    self.construction_counter() != NO_SLACK_TRACKING
  }

  /// 启动 slack tracking: 计数器置 kSlackTrackingCounterStart
  /// (map.cc:2662; 无未用槽则不启动)。
  #[inline]
  pub fn start_inobject_slack_tracking(&mut self) {
    debug_assert!(!self.is_inobject_slack_tracking_in_progress());
    if self.unused_property_fields() == 0 {
      return;
    }
    self.set_construction_counter(SLACK_TRACKING_COUNTER_START);
  }

  /// 按数组尺寸取宽裕 slack (map-inl.h:1263 SlackForArraySize)。
  #[inline]
  #[must_use]
  pub const fn slack_for_array_size(old_size: i32, size_limit: i32) -> i32 {
    let max_slack = size_limit - old_size;
    debug_assert!(max_slack >= 0);
    if old_size < 4 {
      debug_assert!(max_slack >= 1);
      return 1;
    }
    if max_slack < old_size / 4 {
      max_slack
    } else {
      old_size / 4
    }
  }

  /// 按 slack 折减后的实例大小 (map-inl.h:1273)。
  #[inline]
  #[must_use]
  pub const fn instance_size_from_slack(&self, slack: i32) -> i32 {
    self.instance_size() as i32 - slack * K_TAGGED_SIZE as i32
  }

  // ── 布尔语义助手 ─────────────────────────────────────────────────────────

  /// 是否被弃用的原型 map (is_prototype_map && !owns_descriptors;
  /// map-inl.h:708)。
  #[inline]
  pub const fn is_abandoned_prototype_map(&self) -> bool {
    self.is_prototype_map() && !self.owns_descriptors()
  }

  /// 是否稳定 (非 unstable; map-inl.h:882)。
  #[inline]
  pub const fn is_stable(&self) -> bool {
    !self.is_unstable()
  }

  /// 标记为不稳定 (map-inl.h:877 mark_unstable)。
  #[inline]
  pub fn mark_unstable(&mut self) {
    self.set_is_unstable(true);
  }

  /// 是否可参与 transition (map-inl.h:906-918 返回式三谓词合取):
  /// 仅 JSObject 及其子类型, 且排除只读包装对象 (JSExternalObject/JSMessageObject)
  /// 与共享空间固定形状对象 (JSSharedArray/JSSharedStruct)。
  /// 原版两条 DCHECK_IMPLIES 依赖 HeapLayout 空间位置 (推迟), 但返回式纯 instance_type。
  #[inline]
  pub const fn can_transition(&self) -> bool {
    let ty = self.instance_type;
    is_js_object(ty) && !is_maybe_read_only_js_object(ty) && !is_always_shared_space_js_object(ty)
  }

  /// 该实例类型可否持有可 transition 的快 elements kind
  /// (map-inl.h:275: JS_ARRAY / JS_PRIMITIVE_WRAPPER / JS_ARGUMENTS_OBJECT)。
  #[inline]
  #[must_use]
  pub const fn can_have_fast_transitionable_elements_kind_of(instance_type: InstanceType) -> bool {
    matches!(
      instance_type,
      InstanceType::JSArray | InstanceType::JSPrimitiveWrapper | InstanceType::JSArgumentsObject
    )
  }

  /// 本 map 实例类型可否持有可 transition 的快 elements kind。
  #[inline]
  pub const fn can_have_fast_transitionable_elements_kind(&self) -> bool {
    Self::can_have_fast_transitionable_elements_kind_of(self.instance_type())
  }

  // ── Bits1 访问器 (机械展开) ──────────────────────────────────────────────

  #[inline]
  pub const fn is_callable(&self) -> bool {
    (self.bit_field & Self::IS_CALLABLE_MASK) != 0
  }

  #[inline]
  pub fn set_is_callable(&mut self, value: bool) {
    self.bit_field =
      (self.bit_field & !Self::IS_CALLABLE_MASK) | ((value as u8) << Self::IS_CALLABLE_SHIFT);
  }

  #[inline]
  pub const fn is_undetectable(&self) -> bool {
    (self.bit_field & Self::IS_UNDETECTABLE_MASK) != 0
  }

  #[inline]
  pub fn set_is_undetectable(&mut self, value: bool) {
    self.bit_field = (self.bit_field & !Self::IS_UNDETECTABLE_MASK)
      | ((value as u8) << Self::IS_UNDETECTABLE_SHIFT);
  }

  #[inline]
  pub const fn has_named_interceptor(&self) -> bool {
    (self.bit_field & Self::HAS_NAMED_INTERCEPTOR_MASK) != 0
  }

  #[inline]
  pub fn set_has_named_interceptor(&mut self, value: bool) {
    self.bit_field = (self.bit_field & !Self::HAS_NAMED_INTERCEPTOR_MASK)
      | ((value as u8) << Self::HAS_NAMED_INTERCEPTOR_SHIFT);
  }

  #[inline]
  pub const fn is_access_check_needed(&self) -> bool {
    (self.bit_field & Self::IS_ACCESS_CHECK_NEEDED_MASK) != 0
  }

  #[inline]
  pub fn set_is_access_check_needed(&mut self, value: bool) {
    self.bit_field = (self.bit_field & !Self::IS_ACCESS_CHECK_NEEDED_MASK)
      | ((value as u8) << Self::IS_ACCESS_CHECK_NEEDED_SHIFT);
  }

  #[inline]
  pub const fn has_indexed_interceptor(&self) -> bool {
    (self.bit_field & Self::HAS_INDEXED_INTERCEPTOR_MASK) != 0
  }

  #[inline]
  pub fn set_has_indexed_interceptor(&mut self, value: bool) {
    self.bit_field = (self.bit_field & !Self::HAS_INDEXED_INTERCEPTOR_MASK)
      | ((value as u8) << Self::HAS_INDEXED_INTERCEPTOR_SHIFT);
  }

  #[inline]
  pub const fn is_constructor(&self) -> bool {
    (self.bit_field & Self::IS_CONSTRUCTOR_MASK) != 0
  }

  #[inline]
  pub fn set_is_constructor(&mut self, value: bool) {
    self.bit_field =
      (self.bit_field & !Self::IS_CONSTRUCTOR_MASK) | ((value as u8) << Self::IS_CONSTRUCTOR_SHIFT);
  }

  #[inline]
  pub const fn is_extended_map(&self) -> bool {
    (self.bit_field & Self::IS_EXTENDED_MAP_MASK) != 0
  }

  #[inline]
  pub fn set_is_extended_map(&mut self, value: bool) {
    self.bit_field = (self.bit_field & !Self::IS_EXTENDED_MAP_MASK)
      | ((value as u8) << Self::IS_EXTENDED_MAP_SHIFT);
  }

  // ── Bits2 访问器 (机械展开) ──────────────────────────────────────────────

  #[inline]
  pub const fn new_target_is_base(&self) -> bool {
    (self.bit_field2 & Self::NEW_TARGET_IS_BASE_MASK) != 0
  }

  #[inline]
  pub fn set_new_target_is_base(&mut self, value: bool) {
    self.bit_field2 = (self.bit_field2 & !Self::NEW_TARGET_IS_BASE_MASK)
      | ((value as u8) << Self::NEW_TARGET_IS_BASE_SHIFT);
  }

  #[inline]
  pub const fn is_immutable_prototype(&self) -> bool {
    (self.bit_field2 & Self::IS_IMMUTABLE_PROTOTYPE_MASK) != 0
  }

  #[inline]
  pub fn set_is_immutable_prototype(&mut self, value: bool) {
    self.bit_field2 = (self.bit_field2 & !Self::IS_IMMUTABLE_PROTOTYPE_MASK)
      | ((value as u8) << Self::IS_IMMUTABLE_PROTOTYPE_SHIFT);
  }

  /// elements kind 原始值 (6 bit; 类型化 ElementsKind 枚举与谓词族归
  /// elements crate, 此处保留 raw 存取)。
  #[inline]
  pub const fn elements_kind(&self) -> u8 {
    (self.bit_field2 & Self::ELEMENTS_KIND_MASK) >> Self::ELEMENTS_KIND_SHIFT
  }

  /// 设置 elements kind 原始值 (须 < 64 = kElementsKindCount 上界)。
  #[inline]
  pub const fn set_elements_kind(&mut self, kind: u8) {
    debug_assert!(kind <= Self::ELEMENTS_KIND_MASK >> Self::ELEMENTS_KIND_SHIFT);
    self.bit_field2 =
      (self.bit_field2 & !Self::ELEMENTS_KIND_MASK) | (kind << Self::ELEMENTS_KIND_SHIFT);
  }

  // ── Bits3 访问器 (机械展开; bool 位) ────────────────────────────────────

  #[inline]
  pub const fn is_prototype_map(&self) -> bool {
    (self.bit_field3 & Self::IS_PROTOTYPE_MAP_MASK) != 0
  }

  #[inline]
  pub fn set_is_prototype_map(&mut self, value: bool) {
    self.bit_field3 = (self.bit_field3 & !Self::IS_PROTOTYPE_MAP_MASK)
      | ((value as u32) << Self::IS_PROTOTYPE_MAP_SHIFT);
  }

  #[inline]
  pub const fn owns_descriptors(&self) -> bool {
    (self.bit_field3 & Self::OWNS_DESCRIPTORS_MASK) != 0
  }

  #[inline]
  pub fn set_owns_descriptors(&mut self, value: bool) {
    self.bit_field3 = (self.bit_field3 & !Self::OWNS_DESCRIPTORS_MASK)
      | ((value as u32) << Self::OWNS_DESCRIPTORS_SHIFT);
  }

  #[inline]
  pub const fn is_in_retained_map_list(&self) -> bool {
    (self.bit_field3 & Self::IS_IN_RETAINED_MAP_LIST_MASK) != 0
  }

  #[inline]
  pub fn set_is_in_retained_map_list(&mut self, value: bool) {
    self.bit_field3 = (self.bit_field3 & !Self::IS_IN_RETAINED_MAP_LIST_MASK)
      | ((value as u32) << Self::IS_IN_RETAINED_MAP_LIST_SHIFT);
  }

  #[inline]
  pub const fn is_deprecated(&self) -> bool {
    (self.bit_field3 & Self::IS_DEPRECATED_MASK) != 0
  }

  #[inline]
  pub fn set_is_deprecated(&mut self, value: bool) {
    self.bit_field3 =
      (self.bit_field3 & !Self::IS_DEPRECATED_MASK) | ((value as u32) << Self::IS_DEPRECATED_SHIFT);
  }

  #[inline]
  pub const fn is_unstable(&self) -> bool {
    (self.bit_field3 & Self::IS_UNSTABLE_MASK) != 0
  }

  #[inline]
  pub fn set_is_unstable(&mut self, value: bool) {
    self.bit_field3 =
      (self.bit_field3 & !Self::IS_UNSTABLE_MASK) | ((value as u32) << Self::IS_UNSTABLE_SHIFT);
  }

  #[inline]
  pub const fn is_migration_target(&self) -> bool {
    (self.bit_field3 & Self::IS_MIGRATION_TARGET_MASK) != 0
  }

  #[inline]
  pub fn set_is_migration_target(&mut self, value: bool) {
    self.bit_field3 = (self.bit_field3 & !Self::IS_MIGRATION_TARGET_MASK)
      | ((value as u32) << Self::IS_MIGRATION_TARGET_SHIFT);
  }

  #[inline]
  pub const fn is_extensible(&self) -> bool {
    (self.bit_field3 & Self::IS_EXTENSIBLE_MASK) != 0
  }

  #[inline]
  pub fn set_is_extensible(&mut self, value: bool) {
    self.bit_field3 =
      (self.bit_field3 & !Self::IS_EXTENSIBLE_MASK) | ((value as u32) << Self::IS_EXTENSIBLE_SHIFT);
  }

  #[inline]
  pub const fn may_have_interesting_properties(&self) -> bool {
    (self.bit_field3 & Self::MAY_HAVE_INTERESTING_PROPERTIES_MASK) != 0
  }

  #[inline]
  pub fn set_may_have_interesting_properties(&mut self, value: bool) {
    self.bit_field3 = (self.bit_field3 & !Self::MAY_HAVE_INTERESTING_PROPERTIES_MASK)
      | ((value as u32) << Self::MAY_HAVE_INTERESTING_PROPERTIES_SHIFT);
  }

  /// enum cache 长度 (bit 0..9; kInvalidEnumCacheSentinel=1023 表示无效)。
  #[inline]
  pub const fn enum_length(&self) -> u32 {
    self.bit_field3 & Self::ENUM_LENGTH_MASK
  }

  /// 设置 enum cache 长度 (map-inl.h:366; 哨兵免检, 其余须 <= own descriptors)。
  #[inline]
  pub const fn set_enum_length(&mut self, length: u32) {
    if length != INVALID_ENUM_CACHE_SENTINEL {
      debug_assert!(length <= Self::ENUM_LENGTH_MASK);
    }
    self.bit_field3 =
      (self.bit_field3 & !Self::ENUM_LENGTH_MASK) | (length & Self::ENUM_LENGTH_MASK);
  }

  /// 自有描述符数 (bit 10..19; map-inl.h:345)。
  #[inline]
  pub const fn number_of_own_descriptors(&self) -> u32 {
    (self.bit_field3 & Self::NUMBER_OF_OWN_DESCRIPTORS_MASK)
      >> Self::NUMBER_OF_OWN_DESCRIPTORS_SHIFT
  }

  /// 设置自有描述符数 (map-inl.h:350; 上界 kMaxNumberOfDescriptors)。
  #[inline]
  pub const fn set_number_of_own_descriptors(&mut self, count: u32) {
    debug_assert!(count <= MAX_NUMBER_OF_DESCRIPTORS);
    self.bit_field3 = (self.bit_field3 & !Self::NUMBER_OF_OWN_DESCRIPTORS_MASK)
      | ((count << Self::NUMBER_OF_OWN_DESCRIPTORS_SHIFT) & Self::NUMBER_OF_OWN_DESCRIPTORS_MASK);
  }

  /// inobject slack tracking 构造计数器 (bit 29..31)。
  #[inline]
  pub const fn construction_counter(&self) -> u32 {
    (self.bit_field3 & Self::CONSTRUCTION_COUNTER_MASK) >> Self::CONSTRUCTION_COUNTER_SHIFT
  }

  /// 设置构造计数器 (<= 7 = ConstructionCounterBits::kMax)。
  #[inline]
  pub const fn set_construction_counter(&mut self, value: u32) {
    debug_assert!(value <= Self::CONSTRUCTION_COUNTER_MASK >> Self::CONSTRUCTION_COUNTER_SHIFT);
    self.bit_field3 = (self.bit_field3 & !Self::CONSTRUCTION_COUNTER_MASK)
      | ((value << Self::CONSTRUCTION_COUNTER_SHIFT) & Self::CONSTRUCTION_COUNTER_MASK);
  }

  /// 是否字典 mode map (map-inl.h:873)。
  #[inline]
  pub const fn is_dictionary_map(&self) -> bool {
    (self.bit_field3 & Self::IS_DICTIONARY_MAP_MASK) != 0
  }

  /// 设置字典 map 标志 (map-inl.h:866: is_dictionary_map 与 is_unstable
  /// 联动置位/清除 — 原版语义, 勿拆)。
  #[inline]
  pub fn set_is_dictionary_map(&mut self, value: bool) {
    self.bit_field3 = (self.bit_field3 & !Self::IS_DICTIONARY_MAP_MASK)
      | ((value as u32) << Self::IS_DICTIONARY_MAP_SHIFT);
    self.set_is_unstable(value);
  }
}

/// visitor_id 字节 → VisitorId (u8 全域可解码, 越界 → None)。
#[inline]
#[must_use]
pub const fn visitor_id_from_u8(raw: u8) -> Option<VisitorId> {
  if raw >= VisitorId::VisitorIdCount as u8 {
    return None;
  }
  // SAFETY: VisitorId 为 #[repr(u8)] 且判别值自 0 连续 (枚举体由机械展开
  // 生成, 无跳变), raw < VisitorIdCount → 转换落在合法判别值域内。
  Some(unsafe { transmute::<u8, VisitorId>(raw) })
}

// ── ExtendedMap (map.h:1264-1297) ────────────────────────────────────────
/// 带扩展字段的 Map 基类 (子类定义 @hasSameInstanceTypeAsParent 并补 padding)。
///
/// bit_field_ex 紧随 Map 主体 (逻辑承载: map_kind 3 bit + map_size_in_words
/// 5 bit); Rust 侧以独立结构体 + 原始位承载, 视图叠读归 v8-objects-maps。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ExtendedMap {
  /// Map 主体。
  pub map: Map,
  /// 扩展位域 (BitsEx)。
  pub bit_field_ex: u8,
  /// 预留 padding 至 kTaggedSize (子类字段区)。
  pub padding: [u8; 3],
}

impl ExtendedMap {
  /// ExtendedMap 最小尺寸 (map.h:1292-1296: RoundUp(sizeof, kTaggedSize))。
  pub const K_MINIMUM_SIZE: usize = K_SIZE + K_TAGGED_SIZE;
  /// 强扩展字段起始偏移 (map.h:1295)。
  pub const K_START_OF_STRONG_EXTENDED_FIELDS_OFFSET: usize = Self::K_MINIMUM_SIZE;

  /// bit_field_ex 原始值。
  #[inline]
  pub const fn bit_field_ex(&self) -> u8 {
    self.bit_field_ex
  }

  /// 设置 bit_field_ex 原始值。
  #[inline]
  pub const fn set_bit_field_ex(&mut self, value: u8) {
    self.bit_field_ex = value;
  }

  /// 扩展 map 类别 (BitsEx::MapKindBits; 3 bit 仅定义 0, 其余为腐化值)。
  #[inline]
  pub const fn map_kind(&self) -> ExtendedMapKind {
    let bits = self.bit_field_ex & MAP_KIND_MASK;
    debug_assert!(bits == 0, "map_kind 腐化值");
    ExtendedMapKind::JSInterceptorMap
  }

  /// 扩展 map 尺寸 (单位: word; BitsEx::MapSizeInWordsBits)。
  #[inline]
  pub const fn map_size_in_words(&self) -> u8 {
    (self.bit_field_ex & MAP_SIZE_IN_WORDS_MASK) >> MAP_SIZE_IN_WORDS_SHIFT
  }

  /// 扩展 map 尺寸 (字节; map-inl.h:1313)。
  #[inline]
  pub const fn map_size(&self) -> usize {
    (self.map_size_in_words() as usize) << K_TAGGED_SIZE_LOG2
  }

  /// 同时设置类别与尺寸 (map-inl.h:1317; 尺寸须 word 对齐且 <= 255 words)。
  #[inline]
  pub const fn set_map_kind_and_size(&mut self, kind: ExtendedMapKind, size_in_bytes: usize) {
    debug_assert!(size_in_bytes.is_multiple_of(K_TAGGED_SIZE));
    let size_in_words = size_in_bytes >> K_TAGGED_SIZE_LOG2;
    debug_assert!(size_in_words <= u8::MAX as usize);
    self.bit_field_ex =
      ((kind as u8) << MAP_KIND_SHIFT) | ((size_in_words as u8) << MAP_SIZE_IN_WORDS_SHIFT);
  }
}

/// 由扩展类别取尺寸 (map-inl.h:1277 ExtendedMapSizeForKind =
/// sizeof(JSInterceptorMap))。JSInterceptorMap 的 packed 布局
/// (ExtendedMap + flags/padding + 3 tagged 字段) 归 v8-objects-maps
/// 确认后承接 — 此处落 Err 登记。
#[inline]
pub fn extended_map_size_for_kind(kind: ExtendedMapKind) -> error::Result<usize> {
  match kind {
    ExtendedMapKind::JSInterceptorMap => Err(Error::Unsupported(
      "ExtendedMapSizeForKind: JSInterceptorMap 布局归 v8-objects-maps",
    )),
  }
}

// == map.cc 语义区段 (由 tool/src/objects/map.cc.js 维护) ==
// @cc-semantics-begin
// 由 tool/src/objects/map.cc.js 从 v8/src/objects/map.cc (GetVisitorId + ToString) 生成; 修订就地改, 复跑覆盖
// 转写/跳过区段登记见脚本头注释

use crate::error::Result; // Error 由 map.h 头部导入承接
use crate::{
  instance_type::{
    K_CONS_STRING_TAG, K_EXTERNAL_STRING_TAG, K_ONE_BYTE_STRING_TAG, K_SEQ_STRING_TAG,
    K_SLICED_STRING_TAG, K_STRING_ENCODING_MASK, K_STRING_REPRESENTATION_MASK, K_THIN_STRING_TAG,
    is_shortcut_candidate,
  },
  instance_type_checker as itc,
};

// ── Map::GetVisitorId (map.cc:69-559, 大表机械展开) ───────────────────────
// 逐 case 对齐原版; JS_PROMISE 分支依赖 JSObject::GetEmbedderFieldCount
// (JSObject 头部布局) 落 Err, 归 v8-objects-js-core 承接。
#[inline]
pub fn get_visitor_id(map: &Map) -> Result<VisitorId> {
  let ty = map.instance_type();

  // JS API 对象前置分支 (map.cc:102-104; JS_API_* 判别值均 >= FIRST_NONSTRING_TYPE)
  if itc::is_js_api_object(ty) {
    return Ok(VisitorId::VisitJSApiObject);
  }

  let instance_type = ty as u16;
  match ty {
    // string 表示法分派 (map.cc:74-100; string 族先于大表, 原版 if 前置)
    InstanceType::ConsOneByteString
    | InstanceType::ConsTwoByteString
    | InstanceType::ExternalInternalizedOneByteString
    | InstanceType::ExternalInternalizedTwoByteString
    | InstanceType::ExternalOneByteString
    | InstanceType::ExternalTwoByteString
    | InstanceType::InternalizedOneByteString
    | InstanceType::InternalizedTwoByteString
    | InstanceType::SeqOneByteString
    | InstanceType::SeqTwoByteString
    | InstanceType::SharedExternalOneByteString
    | InstanceType::SharedExternalTwoByteString
    | InstanceType::SharedSeqOneByteString
    | InstanceType::SharedSeqTwoByteString
    | InstanceType::SharedUncachedExternalOneByteString
    | InstanceType::SharedUncachedExternalTwoByteString
    | InstanceType::SlicedOneByteString
    | InstanceType::SlicedTwoByteString
    | InstanceType::ThinOneByteString
    | InstanceType::ThinTwoByteString
    | InstanceType::UncachedExternalInternalizedOneByteString
    | InstanceType::UncachedExternalInternalizedTwoByteString
    | InstanceType::UncachedExternalOneByteString
    | InstanceType::UncachedExternalTwoByteString => {
      let rep = (instance_type as u32) & K_STRING_REPRESENTATION_MASK;
      let one_byte = (instance_type as u32 & K_STRING_ENCODING_MASK) == K_ONE_BYTE_STRING_TAG;
      match rep {
        K_SEQ_STRING_TAG => Ok(if one_byte {
          VisitorId::VisitSeqOneByteString
        } else {
          VisitorId::VisitSeqTwoByteString
        }),
        K_CONS_STRING_TAG => Ok(if is_shortcut_candidate(instance_type) {
          VisitorId::VisitShortcutCandidate
        } else {
          VisitorId::VisitConsString
        }),
        K_SLICED_STRING_TAG => Ok(VisitorId::VisitSlicedString),
        K_EXTERNAL_STRING_TAG => Ok(VisitorId::VisitExternalString),
        K_THIN_STRING_TAG => Ok(VisitorId::VisitThinString),
        // 表示法 tag 仅定义 0/1/2/3/5 (4/6/7 未用); 原版 UNREACHABLE → 禁 panic 落 Err。
        _ => Err(Error::Unsupported("GetVisitorId: 非法 string 表示法 tag")),
      }
    }
    InstanceType::AccessCheckInfo => Ok(VisitorId::VisitStruct),
    InstanceType::AccessorInfo => Ok(VisitorId::VisitAccessorInfo),
    InstanceType::AccessorPair => Ok(VisitorId::VisitStruct),
    InstanceType::AliasedArgumentsEntry => Ok(VisitorId::VisitStruct),
    InstanceType::AllocationMemento => Ok(VisitorId::VisitStruct),
    InstanceType::AllocationSite => Ok(VisitorId::VisitAllocationSite),
    InstanceType::ArrayBoilerplateDescription => Ok(VisitorId::VisitStruct),
    InstanceType::ArrayList => Ok(VisitorId::VisitArrayList),
    InstanceType::AsyncGeneratorRequest => Ok(VisitorId::VisitStruct),
    InstanceType::AsyncResumeTask => Ok(VisitorId::VisitStruct),
    InstanceType::AtomRegExpData => Ok(VisitorId::VisitAtomRegExpData),
    InstanceType::AwaitContext => Ok(VisitorId::VisitContext),
    InstanceType::Bigint64TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::BigIntBase => Ok(VisitorId::VisitBigInt),
    InstanceType::Biguint64TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::BlockContext => Ok(VisitorId::VisitContext),
    InstanceType::BreakPoint => Ok(VisitorId::VisitStruct),
    InstanceType::BreakPointInfo => Ok(VisitorId::VisitStruct),
    InstanceType::ByteArray => Ok(VisitorId::VisitByteArray),
    InstanceType::BytecodeArray => Ok(VisitorId::VisitBytecodeArray),
    InstanceType::BytecodeWrapper => Ok(VisitorId::VisitBytecodeWrapper),
    InstanceType::CallableTask => Ok(VisitorId::VisitStruct),
    InstanceType::CallbackTask => Ok(VisitorId::VisitStruct),
    InstanceType::CallSiteInfo => Ok(VisitorId::VisitCallSiteInfo),
    InstanceType::CatchContext => Ok(VisitorId::VisitContext),
    InstanceType::Cell => Ok(VisitorId::VisitCell),
    InstanceType::ClassBoilerplate => Ok(VisitorId::VisitStruct),
    InstanceType::ClassPositions => Ok(VisitorId::VisitStruct),
    InstanceType::ClosureFeedbackCellArray => Ok(VisitorId::VisitClosureFeedbackCellArray),
    InstanceType::Code => Ok(VisitorId::VisitCode),
    InstanceType::CodeWrapper => Ok(VisitorId::VisitCodeWrapper),
    InstanceType::ContextCell => Ok(VisitorId::VisitContextCell),
    InstanceType::CoverageInfo => Ok(VisitorId::VisitCoverageInfo),
    InstanceType::CppGcmanagedBase => Ok(VisitorId::VisitCppGCManagedBase),
    InstanceType::CppHeapExternalObject => Ok(VisitorId::VisitCppHeapExternalObject),
    InstanceType::DebugEvaluateContext => Ok(VisitorId::VisitContext),
    InstanceType::DebugInfo => Ok(VisitorId::VisitDebugInfo),
    InstanceType::DebugScriptScopeInfo => Ok(VisitorId::VisitStruct),
    InstanceType::DescriptorArray => Ok(VisitorId::VisitDescriptorArray),
    InstanceType::DictionaryTemplateInfo => Ok(VisitorId::VisitStruct),
    InstanceType::DoubleStringCache => Ok(VisitorId::VisitDoubleStringCache),
    InstanceType::EmbedderDataArray => Ok(VisitorId::VisitEmbedderDataArray),
    InstanceType::EnumCache => Ok(VisitorId::VisitStruct),
    InstanceType::EphemeronHashTable => Ok(VisitorId::VisitEphemeronHashTable),
    InstanceType::ErrorStackData => Ok(VisitorId::VisitStruct),
    InstanceType::EvalContext => Ok(VisitorId::VisitContext),
    InstanceType::FeedbackCell => Ok(VisitorId::VisitFeedbackCell),
    InstanceType::FeedbackMetadata => Ok(VisitorId::VisitFeedbackMetadata),
    InstanceType::FeedbackVector => Ok(VisitorId::VisitFeedbackVector),
    InstanceType::Filler => Ok(VisitorId::VisitFiller),
    InstanceType::FixedArray => Ok(VisitorId::VisitFixedArray),
    InstanceType::FixedDoubleArray => Ok(VisitorId::VisitFixedDoubleArray),
    InstanceType::Float16TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::Float32TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::Float64TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::Foreign => Ok(VisitorId::VisitForeign),
    InstanceType::ForInEnumeratorHolder => Ok(VisitorId::VisitStruct),
    InstanceType::FreeSpace => Ok(VisitorId::VisitFreeSpace),
    InstanceType::FunctionContext => Ok(VisitorId::VisitContext),
    InstanceType::FunctionTemplateInfo => Ok(VisitorId::VisitFunctionTemplateInfo),
    InstanceType::FunctionTemplateRareData => Ok(VisitorId::VisitStruct),
    InstanceType::GlobalDictionary => Ok(VisitorId::VisitFixedArray),
    InstanceType::HashSeedWrapper => Ok(VisitorId::VisitHashSeedWrapper),
    InstanceType::HashTable => Ok(VisitorId::VisitFixedArray),
    InstanceType::HeapNumber => Ok(VisitorId::VisitHeapNumber),
    InstanceType::Hole => Ok(VisitorId::VisitHole),
    InstanceType::InstructionStream => Ok(VisitorId::VisitInstructionStream),
    InstanceType::Int16TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::Int32TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::Int8TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::InterceptorInfo => Ok(VisitorId::VisitInterceptorInfo),
    InstanceType::InterpreterData => Ok(VisitorId::VisitInterpreterData),
    InstanceType::IrRegExpData => Ok(VisitorId::VisitIrRegExpData),
    InstanceType::JSApiObject => Ok(VisitorId::VisitJSApiObject),
    InstanceType::JSArgumentsObject => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSArray => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSArrayBuffer => Ok(VisitorId::VisitJSArrayBuffer),
    InstanceType::JSArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::JSArrayIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSArrayIteratorPrototype => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSAsyncDisposableStack => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSAsyncFromSyncIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSAsyncFunctionObject => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSAsyncGeneratorObject => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSAtomicsCondition => Ok(VisitorId::VisitJSSynchronizationPrimitive),
    InstanceType::JSAtomicsMutex => Ok(VisitorId::VisitJSSynchronizationPrimitive),
    InstanceType::JSBoundFunction => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSClassConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::JSCollator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSContextExtensionObject => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSDataView => Ok(VisitorId::VisitJSDataViewOrRabGsabDataView),
    InstanceType::JSDate => Ok(VisitorId::VisitJSDate),
    InstanceType::JSDateTimeFormat => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSDeferredModuleNamespace => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSDetachedTypedArray => Ok(VisitorId::VisitJSTypedArray),
    InstanceType::JSDisplayNames => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSDisposableStackBase => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSDurationFormat => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSError => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSExternalObject => Ok(VisitorId::VisitJSExternalObject),
    InstanceType::JSFinalizationRegistry => Ok(VisitorId::VisitJSFinalizationRegistry),
    InstanceType::JSFunctionWithoutPrototype => Ok(VisitorId::VisitJSFunction),
    InstanceType::JSFunctionWithPrototype => Ok(VisitorId::VisitJSFunction),
    InstanceType::JSGeneratorObject => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSGlobalObject => Ok(VisitorId::VisitJSApiObject),
    InstanceType::JSGlobalProxy => Ok(VisitorId::VisitJSGlobalProxy),
    InstanceType::JSIteratorConcatHelper => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSIteratorDropHelper => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSIteratorFilterHelper => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSIteratorFlatMapHelper => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSIteratorMapHelper => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSIteratorPrototype => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSIteratorTakeHelper => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSIteratorZipHelper => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSIteratorZipKeyedHelper => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSListFormat => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSLocale => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSMap => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSMapIteratorPrototype => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSMapKeyIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSMapKeyValueIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSMapValueIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSMessageObject => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSModuleNamespace => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSNumberFormat => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSObject => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSObjectPrototype => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSPluralRules => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSPrimitiveWrapper => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSPromiseConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::JSPromisePrototype => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSProxy => Ok(VisitorId::VisitJSProxy),
    InstanceType::JSRabGsabDataView => Ok(VisitorId::VisitJSDataViewOrRabGsabDataView),
    InstanceType::JSRawJson => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSRegExp => Ok(VisitorId::VisitJSRegExp),
    InstanceType::JSRegExpConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::JSRegExpPrototype => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSRegExpStringIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSRelativeTimeFormat => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSegmenter => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSegmentIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSegments => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSet => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSetIteratorPrototype => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSetKeyValueIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSetPrototype => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSetValueIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSShadowRealm => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSharedArray => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSharedStruct => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSpecialApiObject => Ok(VisitorId::VisitJSApiObject),
    InstanceType::JSStringIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSStringIteratorPrototype => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSSyncDisposableStack => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSTypedArray => Ok(VisitorId::VisitJSTypedArray),
    InstanceType::JSTypedArrayPrototype => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSV8BreakIterator => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSValidIteratorWrapper => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::JSWeakMap => Ok(VisitorId::VisitJSWeakCollection),
    InstanceType::JSWeakRef => Ok(VisitorId::VisitJSWeakRef),
    InstanceType::JSWeakSet => Ok(VisitorId::VisitJSWeakCollection),
    InstanceType::JSWrappedFunction => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::LoadHandler => Ok(VisitorId::VisitDataHandler),
    InstanceType::Map => Ok(VisitorId::VisitMap),
    InstanceType::MegaDomHandler => Ok(VisitorId::VisitMegaDomHandler),
    InstanceType::ModuleContext => Ok(VisitorId::VisitContext),
    InstanceType::ModuleRequest => Ok(VisitorId::VisitStruct),
    InstanceType::NameDictionary => Ok(VisitorId::VisitFixedArray),
    InstanceType::NameToIndexHashTable => Ok(VisitorId::VisitFixedArray),
    InstanceType::NativeContext => Ok(VisitorId::VisitNativeContext),
    InstanceType::NumberDictionary => Ok(VisitorId::VisitFixedArray),
    InstanceType::ObjectBoilerplateDescription => Ok(VisitorId::VisitObjectBoilerplateDescription),
    InstanceType::ObjectTemplateInfo => Ok(VisitorId::VisitStruct),
    InstanceType::Oddball => Ok(VisitorId::VisitOddball),
    InstanceType::OnHeapBasicBlockProfilerData => Ok(VisitorId::VisitOnHeapBasicBlockProfilerData),
    InstanceType::OrderedHashMap => Ok(VisitorId::VisitFixedArray),
    InstanceType::OrderedHashSet => Ok(VisitorId::VisitFixedArray),
    InstanceType::OrderedNameDictionary => Ok(VisitorId::VisitFixedArray),
    InstanceType::PreparseData => Ok(VisitorId::VisitPreparseData),
    InstanceType::PromiseCapability => Ok(VisitorId::VisitStruct),
    InstanceType::PromiseFulfillReactionJobTask => Ok(VisitorId::VisitStruct),
    InstanceType::PromiseReaction => Ok(VisitorId::VisitStruct),
    InstanceType::PromiseRejectReactionJobTask => Ok(VisitorId::VisitStruct),
    InstanceType::PromiseResolveThenableJobTask => Ok(VisitorId::VisitStruct),
    InstanceType::PropertyArray => Ok(VisitorId::VisitPropertyArray),
    InstanceType::PropertyCell => Ok(VisitorId::VisitPropertyCell),
    InstanceType::PropertyDescriptorObject => Ok(VisitorId::VisitStruct),
    InstanceType::ProtectedFixedArray => Ok(VisitorId::VisitProtectedFixedArray),
    InstanceType::ProtectedWeakFixedArray => Ok(VisitorId::VisitProtectedWeakFixedArray),
    InstanceType::PrototypeInfo => Ok(VisitorId::VisitPrototypeInfo),
    InstanceType::PrototypeSharedClosureInfo => Ok(VisitorId::VisitPrototypeSharedClosureInfo),
    InstanceType::RegExpBoilerplateDescription => Ok(VisitorId::VisitRegExpBoilerplateDescription),
    InstanceType::RegExpData => Ok(VisitorId::VisitRegExpData),
    InstanceType::RegExpDataWrapper => Ok(VisitorId::VisitRegExpDataWrapper),
    InstanceType::RegExpMatchInfo => Ok(VisitorId::VisitRegExpMatchInfo),
    InstanceType::RegisteredSymbolTable => Ok(VisitorId::VisitFixedArray),
    InstanceType::ScopeInfo => Ok(VisitorId::VisitScopeInfo),
    InstanceType::Script => Ok(VisitorId::VisitStruct),
    InstanceType::ScriptContext => Ok(VisitorId::VisitContext),
    InstanceType::ScriptContextTable => Ok(VisitorId::VisitScriptContextTable),
    InstanceType::ScriptOrModule => Ok(VisitorId::VisitStruct),
    InstanceType::SharedFunctionInfo => Ok(VisitorId::VisitSharedFunctionInfo),
    InstanceType::SharedFunctionInfoWrapper => Ok(VisitorId::VisitSharedFunctionInfoWrapper),
    InstanceType::SimpleNameDictionary => Ok(VisitorId::VisitFixedArray),
    InstanceType::SimpleNumberDictionary => Ok(VisitorId::VisitFixedArray),
    InstanceType::SloppyArgumentsElements => Ok(VisitorId::VisitSloppyArgumentsElements),
    InstanceType::SmallOrderedHashMap => Ok(VisitorId::VisitSmallOrderedHashMap),
    InstanceType::SmallOrderedHashSet => Ok(VisitorId::VisitSmallOrderedHashSet),
    InstanceType::SmallOrderedNameDictionary => Ok(VisitorId::VisitSmallOrderedNameDictionary),
    InstanceType::SortState => Ok(VisitorId::VisitSortState),
    InstanceType::SourceTextModule => Ok(VisitorId::VisitSourceTextModule),
    InstanceType::SourceTextModuleInfoEntry => Ok(VisitorId::VisitStruct),
    InstanceType::StackFrameInfo => Ok(VisitorId::VisitStruct),
    InstanceType::StackTraceInfo => Ok(VisitorId::VisitStruct),
    InstanceType::StoreHandler => Ok(VisitorId::VisitDataHandler),
    InstanceType::SwissNameDictionary => Ok(VisitorId::VisitSwissNameDictionary),
    InstanceType::Symbol => Ok(VisitorId::VisitSymbol),
    InstanceType::SyntheticModule => Ok(VisitorId::VisitSyntheticModule),
    InstanceType::TemplateObjectDescription => Ok(VisitorId::VisitStruct),
    InstanceType::TransitionArray => Ok(VisitorId::VisitTransitionArray),
    InstanceType::TrustedByteArray => Ok(VisitorId::VisitTrustedByteArray),
    InstanceType::TrustedFixedArray => Ok(VisitorId::VisitTrustedFixedArray),
    InstanceType::TrustedForeign => Ok(VisitorId::VisitTrustedForeign),
    InstanceType::TrustedWeakFixedArray => Ok(VisitorId::VisitTrustedWeakFixedArray),
    InstanceType::Tuple2 => Ok(VisitorId::VisitStruct),
    InstanceType::TurbofanBitsetType => Ok(VisitorId::VisitTurbofanBitsetType),
    InstanceType::TurbofanHeapConstantType => Ok(VisitorId::VisitTurbofanHeapConstantType),
    InstanceType::TurbofanOtherNumberConstantType => {
      Ok(VisitorId::VisitTurbofanOtherNumberConstantType)
    }
    InstanceType::TurbofanRangeType => Ok(VisitorId::VisitTurbofanRangeType),
    InstanceType::TurbofanUnionType => Ok(VisitorId::VisitTurbofanUnionType),
    InstanceType::TurboshaftFloat64RangeType => Ok(VisitorId::VisitTurboshaftFloat64RangeType),
    InstanceType::TurboshaftFloat64SetType => Ok(VisitorId::VisitTurboshaftFloat64SetType),
    InstanceType::TurboshaftWord32RangeType => Ok(VisitorId::VisitTurboshaftWord32RangeType),
    InstanceType::TurboshaftWord32SetType => Ok(VisitorId::VisitTurboshaftWord32SetType),
    InstanceType::TurboshaftWord64RangeType => Ok(VisitorId::VisitTurboshaftWord64RangeType),
    InstanceType::TurboshaftWord64SetType => Ok(VisitorId::VisitTurboshaftWord64SetType),
    InstanceType::Uint16TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::Uint32TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::Uint8ClampedTypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::Uint8TypedArrayConstructor => Ok(VisitorId::VisitJSFunction),
    InstanceType::UncompiledDataWithoutPreparseData => {
      Ok(VisitorId::VisitUncompiledDataWithoutPreparseData)
    }
    InstanceType::UncompiledDataWithoutPreparseDataWithJob => {
      Ok(VisitorId::VisitUncompiledDataWithoutPreparseDataWithJob)
    }
    InstanceType::UncompiledDataWithPreparseData => {
      Ok(VisitorId::VisitUncompiledDataWithPreparseData)
    }
    InstanceType::UncompiledDataWithPreparseDataAndJob => {
      Ok(VisitorId::VisitUncompiledDataWithPreparseDataAndJob)
    }
    InstanceType::UninitializedHeapNumber => Ok(VisitorId::VisitUninitializedHeapNumber),
    InstanceType::WasmArray => Ok(VisitorId::VisitWasmArray),
    InstanceType::WasmCapiFunctionData => Ok(VisitorId::VisitWasmCapiFunctionData),
    InstanceType::WasmContinuationObject => Ok(VisitorId::VisitWasmContinuationObject),
    InstanceType::WasmCustomMap => Ok(VisitorId::VisitWasmCustomMap),
    InstanceType::WasmCustomMapWrapper => Ok(VisitorId::VisitJSObject),
    InstanceType::WasmDispatchTable => Ok(VisitorId::VisitWasmDispatchTable),
    InstanceType::WasmDispatchTableForImports => Ok(VisitorId::VisitWasmDispatchTableForImports),
    InstanceType::WasmExceptionPackage => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::WasmExceptionTag => Ok(VisitorId::VisitStruct),
    InstanceType::WasmExportedFunctionData => Ok(VisitorId::VisitWasmExportedFunctionData),
    InstanceType::WasmFastApiCallData => Ok(VisitorId::VisitWasmFastApiCallData),
    InstanceType::WasmFuncRef => Ok(VisitorId::VisitWasmFuncRef),
    InstanceType::WasmGlobalObject => Ok(VisitorId::VisitWasmGlobalObject),
    InstanceType::WasmImportData => Ok(VisitorId::VisitWasmImportData),
    InstanceType::WasmInstanceObject => Ok(VisitorId::VisitWasmInstanceObject),
    InstanceType::WasmInternalFunction => Ok(VisitorId::VisitWasmInternalFunction),
    InstanceType::WasmMemoryObject => Ok(VisitorId::VisitWasmMemoryObject),
    InstanceType::WasmModuleObject => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::WasmNull => Ok(VisitorId::VisitWasmNull),
    InstanceType::WasmResumeData => Ok(VisitorId::VisitWasmResumeData),
    InstanceType::WasmStackObject => Ok(VisitorId::VisitWasmStackObject),
    InstanceType::WasmStringViewIter => Ok(VisitorId::VisitWasmStringViewIter),
    InstanceType::WasmStruct => Ok(VisitorId::VisitWasmStruct),
    InstanceType::WasmSuspenderObject => Ok(VisitorId::VisitWasmSuspenderObject),
    InstanceType::WasmSuspendingObject => Ok(VisitorId::VisitWasmSuspendingObject),
    InstanceType::WasmTableObject => Ok(VisitorId::VisitWasmTableObject),
    InstanceType::WasmTagObject => Ok(VisitorId::VisitWasmTagObject),
    InstanceType::WasmTrustedInstanceData => Ok(VisitorId::VisitWasmTrustedInstanceData),
    InstanceType::WasmTypeInfo => Ok(VisitorId::VisitWasmTypeInfo),
    InstanceType::WasmValueObject => Ok(VisitorId::VisitJSObjectFast),
    InstanceType::WeakArrayList => Ok(VisitorId::VisitWeakArrayList),
    InstanceType::WeakCell => Ok(VisitorId::VisitWeakCell),
    InstanceType::WeakFixedArray => Ok(VisitorId::VisitWeakFixedArray),
    InstanceType::WeakHomomorphicFixedArray => Ok(VisitorId::VisitWeakHomomorphicFixedArray),
    InstanceType::WithContext => Ok(VisitorId::VisitContext),
    // JS_PROMISE (map.cc:297-301): COMPRESS_POINTERS_BOOL && embedder 字段数 > 0
    // → VisitJSObject, 否则 VisitJSObjectFast — 依赖 JSObject 头部布局。
    InstanceType::JSPromise => Err(Error::Unsupported(
      "GetVisitorId: JS_PROMISE embedder 字段判定归 v8-objects-js-core",
    )),
    // 抽象/占位实例类型: 运行时不可能成为 map 的 instance_type
    // (原版尾部 FATAL → 禁 panic 落 Err)。
    InstanceType::WasmFunctionData
    | InstanceType::BaseNameDictionary
    | InstanceType::Dictionary
    | InstanceType::ObjectHashTableBase
    | InstanceType::OrderedHashTable
    | InstanceType::SmallOrderedHashTableImpl
    | InstanceType::JSLastDummyApiObject
    | InstanceType::JSObjectWithEmbedderSlots
    | InstanceType::OrderedHashTableIterator => {
      Err(Error::Unsupported("GetVisitorId: 抽象/占位实例类型"))
    }
  }
}

// ── ToString(VisitorId) (map.cc:2742-2757) ───────────────────────────────
/// 访客 id 名 (哨兵变体返回哨兵名)。
#[inline]
#[must_use]
pub fn visitor_id_to_str(visitor_id: VisitorId) -> Option<&'static str> {
  match visitor_id {
    VisitorId::VisitBigInt => Some("BigInt"),
    VisitorId::VisitCoverageInfo => Some("CoverageInfo"),
    VisitorId::VisitFeedbackMetadata => Some("FeedbackMetadata"),
    VisitorId::VisitFiller => Some("Filler"),
    VisitorId::VisitHeapNumber => Some("HeapNumber"),
    VisitorId::VisitHashSeedWrapper => Some("HashSeedWrapper"),
    VisitorId::VisitUninitializedHeapNumber => Some("UninitializedHeapNumber"),
    VisitorId::VisitHole => Some("Hole"),
    VisitorId::VisitSeqOneByteString => Some("SeqOneByteString"),
    VisitorId::VisitSeqTwoByteString => Some("SeqTwoByteString"),
    VisitorId::VisitTurbofanBitsetType => Some("TurbofanBitsetType"),
    VisitorId::VisitTurbofanOtherNumberConstantType => Some("TurbofanOtherNumberConstantType"),
    VisitorId::VisitTurbofanRangeType => Some("TurbofanRangeType"),
    VisitorId::VisitTurboshaftFloat64RangeType => Some("TurboshaftFloat64RangeType"),
    VisitorId::VisitTurboshaftFloat64SetType => Some("TurboshaftFloat64SetType"),
    VisitorId::VisitTurboshaftWord32RangeType => Some("TurboshaftWord32RangeType"),
    VisitorId::VisitTurboshaftWord32SetType => Some("TurboshaftWord32SetType"),
    VisitorId::VisitTurboshaftWord64RangeType => Some("TurboshaftWord64RangeType"),
    VisitorId::VisitTurboshaftWord64SetType => Some("TurboshaftWord64SetType"),
    VisitorId::VisitWasmNull => Some("WasmNull"),
    VisitorId::VisitAccessorInfo => Some("AccessorInfo"),
    VisitorId::VisitAllocationSite => Some("AllocationSite"),
    VisitorId::VisitBytecodeWrapper => Some("BytecodeWrapper"),
    VisitorId::VisitCallSiteInfo => Some("CallSiteInfo"),
    VisitorId::VisitCell => Some("Cell"),
    VisitorId::VisitCodeWrapper => Some("CodeWrapper"),
    VisitorId::VisitConsString => Some("ConsString"),
    VisitorId::VisitContext => Some("Context"),
    VisitorId::VisitContextCell => Some("ContextCell"),
    VisitorId::VisitCppHeapExternalObject => Some("CppHeapExternalObject"),
    VisitorId::VisitCppGCManagedBase => Some("CppGCManagedBase"),
    VisitorId::VisitDataHandler => Some("DataHandler"),
    VisitorId::VisitDescriptorArray => Some("DescriptorArray"),
    VisitorId::VisitDoubleStringCache => Some("DoubleStringCache"),
    VisitorId::VisitEmbedderDataArray => Some("EmbedderDataArray"),
    VisitorId::VisitEphemeronHashTable => Some("EphemeronHashTable"),
    VisitorId::VisitExternalString => Some("ExternalString"),
    VisitorId::VisitFeedbackCell => Some("FeedbackCell"),
    VisitorId::VisitFeedbackVector => Some("FeedbackVector"),
    VisitorId::VisitForeign => Some("Foreign"),
    VisitorId::VisitFreeSpace => Some("FreeSpace"),
    VisitorId::VisitFunctionTemplateInfo => Some("FunctionTemplateInfo"),
    VisitorId::VisitInterceptorInfo => Some("InterceptorInfo"),
    VisitorId::VisitJSApiObject => Some("JSApiObject"),
    VisitorId::VisitJSArrayBuffer => Some("JSArrayBuffer"),
    VisitorId::VisitJSDataViewOrRabGsabDataView => Some("JSDataViewOrRabGsabDataView"),
    VisitorId::VisitJSDate => Some("JSDate"),
    VisitorId::VisitJSExternalObject => Some("JSExternalObject"),
    VisitorId::VisitJSFinalizationRegistry => Some("JSFinalizationRegistry"),
    VisitorId::VisitJSFunction => Some("JSFunction"),
    VisitorId::VisitJSGlobalProxy => Some("JSGlobalProxy"),
    VisitorId::VisitJSObject => Some("JSObject"),
    VisitorId::VisitJSObjectFast => Some("JSObjectFast"),
    VisitorId::VisitJSProxy => Some("JSProxy"),
    VisitorId::VisitJSRegExp => Some("JSRegExp"),
    VisitorId::VisitJSSynchronizationPrimitive => Some("JSSynchronizationPrimitive"),
    VisitorId::VisitJSTypedArray => Some("JSTypedArray"),
    VisitorId::VisitJSWeakCollection => Some("JSWeakCollection"),
    VisitorId::VisitJSWeakRef => Some("JSWeakRef"),
    VisitorId::VisitMap => Some("Map"),
    VisitorId::VisitMegaDomHandler => Some("MegaDomHandler"),
    VisitorId::VisitNativeContext => Some("NativeContext"),
    VisitorId::VisitOddball => Some("Oddball"),
    VisitorId::VisitOnHeapBasicBlockProfilerData => Some("OnHeapBasicBlockProfilerData"),
    VisitorId::VisitPreparseData => Some("PreparseData"),
    VisitorId::VisitPropertyArray => Some("PropertyArray"),
    VisitorId::VisitPropertyCell => Some("PropertyCell"),
    VisitorId::VisitPrototypeInfo => Some("PrototypeInfo"),
    VisitorId::VisitPrototypeSharedClosureInfo => Some("PrototypeSharedClosureInfo"),
    VisitorId::VisitRegExpBoilerplateDescription => Some("RegExpBoilerplateDescription"),
    VisitorId::VisitRegExpDataWrapper => Some("RegExpDataWrapper"),
    VisitorId::VisitScopeInfo => Some("ScopeInfo"),
    VisitorId::VisitSharedFunctionInfo => Some("SharedFunctionInfo"),
    VisitorId::VisitShortcutCandidate => Some("ShortcutCandidate"),
    VisitorId::VisitSlicedString => Some("SlicedString"),
    VisitorId::VisitSmallOrderedHashMap => Some("SmallOrderedHashMap"),
    VisitorId::VisitSmallOrderedHashSet => Some("SmallOrderedHashSet"),
    VisitorId::VisitSmallOrderedNameDictionary => Some("SmallOrderedNameDictionary"),
    VisitorId::VisitSortState => Some("SortState"),
    VisitorId::VisitSourceTextModule => Some("SourceTextModule"),
    VisitorId::VisitStruct => Some("Struct"),
    VisitorId::VisitSwissNameDictionary => Some("SwissNameDictionary"),
    VisitorId::VisitSymbol => Some("Symbol"),
    VisitorId::VisitTurbofanHeapConstantType => Some("TurbofanHeapConstantType"),
    VisitorId::VisitTurbofanUnionType => Some("TurbofanUnionType"),
    VisitorId::VisitSyntheticModule => Some("SyntheticModule"),
    VisitorId::VisitThinString => Some("ThinString"),
    VisitorId::VisitTransitionArray => Some("TransitionArray"),
    VisitorId::VisitWasmArray => Some("WasmArray"),
    VisitorId::VisitWasmCustomMap => Some("WasmCustomMap"),
    VisitorId::VisitWasmFuncRef => Some("WasmFuncRef"),
    VisitorId::VisitWasmGlobalObject => Some("WasmGlobalObject"),
    VisitorId::VisitWasmInstanceObject => Some("WasmInstanceObject"),
    VisitorId::VisitWasmMemoryObject => Some("WasmMemoryObject"),
    VisitorId::VisitWasmResumeData => Some("WasmResumeData"),
    VisitorId::VisitWasmStruct => Some("WasmStruct"),
    VisitorId::VisitWasmSuspendingObject => Some("WasmSuspendingObject"),
    VisitorId::VisitWasmContinuationObject => Some("WasmContinuationObject"),
    VisitorId::VisitWasmFastApiCallData => Some("WasmFastApiCallData"),
    VisitorId::VisitWasmStackObject => Some("WasmStackObject"),
    VisitorId::VisitWasmStringViewIter => Some("WasmStringViewIter"),
    VisitorId::VisitWasmTableObject => Some("WasmTableObject"),
    VisitorId::VisitWasmTagObject => Some("WasmTagObject"),
    VisitorId::VisitWasmTypeInfo => Some("WasmTypeInfo"),
    VisitorId::VisitWeakCell => Some("WeakCell"),
    VisitorId::VisitArrayList => Some("ArrayList"),
    VisitorId::VisitByteArray => Some("ByteArray"),
    VisitorId::VisitClosureFeedbackCellArray => Some("ClosureFeedbackCellArray"),
    VisitorId::VisitFixedArray => Some("FixedArray"),
    VisitorId::VisitFixedDoubleArray => Some("FixedDoubleArray"),
    VisitorId::VisitObjectBoilerplateDescription => Some("ObjectBoilerplateDescription"),
    VisitorId::VisitRegExpMatchInfo => Some("RegExpMatchInfo"),
    VisitorId::VisitScriptContextTable => Some("ScriptContextTable"),
    VisitorId::VisitSloppyArgumentsElements => Some("SloppyArgumentsElements"),
    VisitorId::VisitWeakArrayList => Some("WeakArrayList"),
    VisitorId::VisitWeakFixedArray => Some("WeakFixedArray"),
    VisitorId::VisitWeakHomomorphicFixedArray => Some("WeakHomomorphicFixedArray"),
    VisitorId::VisitBytecodeArray => Some("BytecodeArray"),
    VisitorId::VisitCode => Some("Code"),
    VisitorId::VisitDebugInfo => Some("DebugInfo"),
    VisitorId::VisitInstructionStream => Some("InstructionStream"),
    VisitorId::VisitInterpreterData => Some("InterpreterData"),
    VisitorId::VisitUncompiledDataWithPreparseData => Some("UncompiledDataWithPreparseData"),
    VisitorId::VisitUncompiledDataWithoutPreparseData => Some("UncompiledDataWithoutPreparseData"),
    VisitorId::VisitUncompiledDataWithPreparseDataAndJob => {
      Some("UncompiledDataWithPreparseDataAndJob")
    }
    VisitorId::VisitUncompiledDataWithoutPreparseDataWithJob => {
      Some("UncompiledDataWithoutPreparseDataWithJob")
    }
    VisitorId::VisitSharedFunctionInfoWrapper => Some("SharedFunctionInfoWrapper"),
    VisitorId::VisitProtectedFixedArray => Some("ProtectedFixedArray"),
    VisitorId::VisitProtectedWeakFixedArray => Some("ProtectedWeakFixedArray"),
    VisitorId::VisitTrustedByteArray => Some("TrustedByteArray"),
    VisitorId::VisitTrustedFixedArray => Some("TrustedFixedArray"),
    VisitorId::VisitTrustedForeign => Some("TrustedForeign"),
    VisitorId::VisitTrustedWeakFixedArray => Some("TrustedWeakFixedArray"),
    VisitorId::VisitAtomRegExpData => Some("AtomRegExpData"),
    VisitorId::VisitIrRegExpData => Some("IrRegExpData"),
    VisitorId::VisitRegExpData => Some("RegExpData"),
    VisitorId::VisitWasmImportData => Some("WasmImportData"),
    VisitorId::VisitWasmCapiFunctionData => Some("WasmCapiFunctionData"),
    VisitorId::VisitWasmDispatchTable => Some("WasmDispatchTable"),
    VisitorId::VisitWasmDispatchTableForImports => Some("WasmDispatchTableForImports"),
    VisitorId::VisitWasmExportedFunctionData => Some("WasmExportedFunctionData"),
    VisitorId::VisitWasmInternalFunction => Some("WasmInternalFunction"),
    VisitorId::VisitWasmTrustedInstanceData => Some("WasmTrustedInstanceData"),
    VisitorId::VisitWasmSuspenderObject => Some("WasmSuspenderObject"),
    VisitorId::DataOnlyVisitorIdCount => Some("DataOnlyVisitorIdCount"),
    VisitorId::VisitorIdCount => Some("VisitorIdCount"),
  }
}

// @cc-semantics-end
