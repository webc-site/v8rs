// 由 tool/src/objects/objects-body-descriptors.h.js 从 v8/src/objects/objects-body-descriptors.h + objects-body-descriptors-inl.h 生成; 修订就地改, 复跑覆盖
// 描述符拓扑与 Visit 原语清单; 遍历语义留骨架

//! 对象体描述符 (原版 objects-body-descriptors.{h,-inl.h})
//!
//! 每个 V8 对象类型经 `T::BodyDescriptor` 声明 GC 遍历规则:
//! 强指针区间 / 可弱区间 / 仅数据 / 可变长后缀 / 父子叠加 / mixin 叠层.

// ===== 基础设施类拓扑 (h 侧 13 类 + 3 mixin) =====
/// 描述符基类族 (声明顺序: 名, 基类, 语义)
pub const DESCRIPTOR_CLASSES: [(&str, &str, &str); 13] = [
  ("BodyDescriptorBase", "-", "Visit 原语宿主 (仅静态方法)"),
  (
    "DataOnlyBodyDescriptor",
    "-",
    "无指针对象, IterateBody 为空",
  ),
  (
    "FixedRangeBodyDescriptor",
    "-",
    "强指针位于 [start,end) 定区间",
  ),
  (
    "FixedBodyDescriptor",
    "-",
    "定尺寸 + 定区间 (start==end 时退化为 DataOnly)",
  ),
  (
    "SuffixRangeBodyDescriptor",
    "-",
    "强指针位于 [start,object_size) 后缀",
  ),
  (
    "FlexibleBodyDescriptor",
    "-",
    "可变尺寸后缀, SizeOf 从 map 取",
  ),
  (
    "StructBodyDescriptor",
    "-",
    "Struct 通用: FlexibleBodyDescriptor<sizeof(Struct)>",
  ),
  ("SuffixRangeWeakBodyDescriptor", "-", "后缀区间可含弱引用"),
  ("FixedWeakBodyDescriptor", "-", "定尺寸 + 定区间可弱"),
  ("FlexibleWeakBodyDescriptor", "-", "可变尺寸后缀可弱"),
  (
    "SubclassBodyDescriptor",
    "-",
    "父描述符 ∪ 子描述符 (父定尺寸且区间不相交)",
  ),
  (
    "FixedExposedTrustedObjectBodyDescriptor",
    "-",
    "ExposedTrustedObject: 自间接指针 + Fixed 体",
  ),
  ("StackedBodyDescriptor", "-", "Base + 逐层 mixin 叠加"),
];

/// 指针字段 mixin (作用于任意 Base 的 BodyDescriptor 内嵌类)
pub const DESCRIPTOR_MIXINS: [&str; 3] = [
  "WithStrongTrustedPointer",
  "WithExternalPointer",
  "WithProtectedPointer",
];

// ===== Visit 原语 (BodyDescriptorBase, 15 种) =====
use crate::{
  error::Result,
  heap_object::HeapObject,
  tagged::Tagged,
  visitors::{MaybeObjectSlot, ObjectSlot, ObjectVisitor},
};

/// 原版 IterateBody
#[inline]
pub fn iterate_body<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  start_offset: usize,
  end_offset: usize,
  v: &mut V,
) -> Result<()> {
  iterate_pointers(obj, start_offset, end_offset, v)
}

/// 原版 IteratePointers
#[inline]
pub fn iterate_pointers<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  start_offset: usize,
  end_offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_pointers(
    obj,
    ObjectSlot::new(base + start_offset),
    ObjectSlot::new(base + end_offset),
  )
}

/// 原版 IteratePointer
#[inline]
pub fn iterate_pointer<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_pointer(obj, ObjectSlot::new(base + offset))
}

/// 原版 IterateCustomWeakPointers
#[inline]
pub fn iterate_custom_weak_pointers<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  start_offset: usize,
  end_offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_custom_weak_pointers(
    obj,
    ObjectSlot::new(base + start_offset),
    ObjectSlot::new(base + end_offset),
  )
}

/// 原版 IterateCustomWeakPointer
#[inline]
pub fn iterate_custom_weak_pointer<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_custom_weak_pointer(obj, ObjectSlot::new(base + offset))
}

/// 原版 IterateEphemeron
#[inline]
pub fn iterate_ephemeron<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  index: i32,
  key_offset: usize,
  value_offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_ephemeron(
    obj,
    index,
    ObjectSlot::new(base + key_offset),
    ObjectSlot::new(base + value_offset),
  )
}

/// 原版 IterateMaybeWeakPointers
#[inline]
pub fn iterate_maybe_weak_pointers<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  start_offset: usize,
  end_offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_maybe_pointers(
    obj,
    MaybeObjectSlot::new(base + start_offset),
    MaybeObjectSlot::new(base + end_offset),
  )
}

/// 原版 IterateMaybeWeakPointer
#[inline]
pub fn iterate_maybe_weak_pointer<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_maybe_pointer(obj, MaybeObjectSlot::new(base + offset))
}

/// 原版 IterateTrustedPointer
#[inline]
pub fn iterate_trusted_pointer<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_pointer(obj, ObjectSlot::new(base + offset))
}

/// 原版 IterateCodePointer
#[inline]
pub fn iterate_code_pointer<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_pointer(obj, ObjectSlot::new(base + offset))
}

/// 原版 IterateSelfIndirectPointer
#[inline]
pub fn iterate_self_indirect_pointer<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_pointer(obj, ObjectSlot::new(base + offset))
}

/// 原版 IterateProtectedPointer
#[inline]
pub fn iterate_protected_pointer<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_pointer(obj, ObjectSlot::new(base + offset))
}

/// 原版 IterateJSDispatchEntry
#[inline]
pub fn iterate_jsdispatch_entry<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  offset: usize,
  v: &mut V,
) -> Result<()> {
  let base = obj.address();
  v.visit_pointer(obj, ObjectSlot::new(base + offset))
}

/// 原版 IterateJSObjectBodyImpl
#[inline]
pub fn iterate_jsobject_body_impl<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  start_offset: usize,
  end_offset: usize,
  v: &mut V,
) -> Result<()> {
  iterate_pointers(obj, start_offset, end_offset, v)
}

/// 原版 IterateJSObjectBodyWithoutEmbedderFieldsImpl
#[inline]
pub fn iterate_jsobject_body_without_embedder_fields_impl<V: ObjectVisitor>(
  obj: Tagged<HeapObject>,
  start_offset: usize,
  end_offset: usize,
  v: &mut V,
) -> Result<()> {
  iterate_pointers(obj, start_offset, end_offset, v)
}

// ===== 逐类型描述符清单 (-inl 侧, 119 个) =====
/// Type::BodyDescriptor → 基描述符 (对数基准, GC 遍历表生成输入)
pub const TYPE_BODY_DESCRIPTORS: [(&str, &str); 119] = [
  (
    "JSAPIObjectWithEmbedderSlots",
    "public JSAPIObjectWithEmbedderSlotsOrJSSpecialObjectBodyDescriptor",
  ),
  (
    "JSGlobalProxy",
    "public JSAPIObjectWithEmbedderSlotsOrJSSpecialObjectBodyDescriptor",
  ),
  ("CppHeapExternalObject", "public BodyDescriptorBase"),
  ("HeapNumber", "public DataOnlyBodyDescriptor"),
  ("UninitializedHeapNumber", "public DataOnlyBodyDescriptor"),
  ("HashSeedWrapper", "public DataOnlyBodyDescriptor"),
  ("TurboshaftWord32SetType", "public DataOnlyBodyDescriptor"),
  ("TurboshaftWord64SetType", "public DataOnlyBodyDescriptor"),
  ("TurboshaftFloat64SetType", "public DataOnlyBodyDescriptor"),
  ("TurbofanBitsetType", "public DataOnlyBodyDescriptor"),
  ("TurbofanRangeType", "public DataOnlyBodyDescriptor"),
  (
    "TurbofanOtherNumberConstantType",
    "public DataOnlyBodyDescriptor",
  ),
  (
    "TurbofanUnionType",
    "public FixedBodyDescriptor<offsetof(TurbofanUnionType, type1_),",
  ),
  (
    "TurbofanHeapConstantType",
    "public FixedBodyDescriptor<offsetof(TurbofanHeapConstantType, constant_),",
  ),
  (
    "OnHeapBasicBlockProfilerData",
    "public FixedBodyDescriptor<offsetof(OnHeapBasicBlockProfilerData,",
  ),
  (
    "SortState",
    "public FixedBodyDescriptor<offsetof(SortState, receiver_),",
  ),
  (
    "WasmFastApiCallData",
    "public FixedWeakBodyDescriptor<offsetof(WasmFastApiCallData, signature_),",
  ),
  (
    "WasmStringViewIter",
    "public FixedBodyDescriptor<offsetof(WasmStringViewIter, string_),",
  ),
  ("FreeSpace", "public DataOnlyBodyDescriptor"),
  ("JSObject", "public BodyDescriptorBase"),
  ("JSObject", "public BodyDescriptorBase"),
  (
    "JSMessageObject",
    "public FixedBodyDescriptor<offsetof(JSMessageObject, properties_or_hash_),",
  ),
  ("JSDate", "public BodyDescriptorBase"),
  ("JSRegExp", "public BodyDescriptorBase"),
  ("RegExpData", "public BodyDescriptorBase"),
  ("AtomRegExpData", "public BodyDescriptorBase"),
  ("IrRegExpData", "public BodyDescriptorBase"),
  ("RegExpDataWrapper", "public BodyDescriptorBase"),
  (
    "ScopeInfo",
    "public SuffixRangeBodyDescriptor<offsetof(ScopeInfo, parameter_count_)>",
  ),
  ("FeedbackVector", "public BodyDescriptorBase"),
  ("DescriptorArray", "public BodyDescriptorBase"),
  ("WeakCell", "public BodyDescriptorBase"),
  ("JSWeakRef", "public BodyDescriptorBase"),
  ("JSProxy", "public BodyDescriptorBase"),
  ("JSFinalizationRegistry", "public BodyDescriptorBase"),
  ("AllocationSite", "public BodyDescriptorBase"),
  ("JSFunction", "public BodyDescriptorBase"),
  (
    "JSArrayBuffer",
    "public JSAPIObjectWithEmbedderSlots::BodyDescriptor",
  ),
  (
    "JSArrayBufferView",
    "public JSAPIObjectWithEmbedderSlots::BodyDescriptor",
  ),
  ("JSTypedArray", "public JSArrayBufferView::BodyDescriptor"),
  (
    "JSDataViewOrRabGsabDataView",
    "public JSArrayBufferView::BodyDescriptor",
  ),
  ("JSExternalObject", "public BodyDescriptorBase"),
  ("Foreign", "public BodyDescriptorBase"),
  ("CppGCManagedBase", "public BodyDescriptorBase"),
  ("SmallOrderedHashTableImpl", "public BodyDescriptorBase"),
  ("SwissNameDictionary", "public BodyDescriptorBase"),
  ("ByteArray", "public DataOnlyBodyDescriptor"),
  ("TrustedByteArray", "public DataOnlyBodyDescriptor"),
  ("BytecodeArray", "public BodyDescriptorBase"),
  ("BytecodeWrapper", "public BodyDescriptorBase"),
  ("BigInt", "public DataOnlyBodyDescriptor"),
  ("FixedDoubleArray", "public DataOnlyBodyDescriptor"),
  ("FeedbackMetadata", "public DataOnlyBodyDescriptor"),
  ("PreparseData", "public BodyDescriptorBase"),
  ("InterpreterData", "public BodyDescriptorBase"),
  (
    "UncompiledDataWithoutPreparseData",
    "public BodyDescriptorBase",
  ),
  (
    "UncompiledDataWithPreparseData",
    "public BodyDescriptorBase",
  ),
  (
    "UncompiledDataWithoutPreparseDataWithJob",
    "public BodyDescriptorBase",
  ),
  (
    "UncompiledDataWithPreparseDataAndJob",
    "public BodyDescriptorBase",
  ),
  ("SharedFunctionInfo", "public BodyDescriptorBase"),
  ("DebugInfo", "public BodyDescriptorBase"),
  ("PrototypeInfo", "public BodyDescriptorBase"),
  ("JSWeakCollection", "public BodyDescriptorBase"),
  ("JSSynchronizationPrimitive", "public BodyDescriptorBase"),
  ("Hole", "public DataOnlyBodyDescriptor"),
  ("WasmTypeInfo", "public BodyDescriptorBase"),
  ("WasmFuncRef", "public BodyDescriptorBase"),
  ("WasmResumeData", "public BodyDescriptorBase"),
  ("WasmStackObject", "public BodyDescriptorBase"),
  ("WasmSuspendingObject", "public BodyDescriptorBase"),
  ("WasmModuleObject", "public BodyDescriptorBase"),
  ("WasmInstanceObject", "public BodyDescriptorBase"),
  ("WasmTagObject", "public BodyDescriptorBase"),
  ("WasmImportData", "public BodyDescriptorBase"),
  ("WasmInternalFunction", "public BodyDescriptorBase"),
  ("WasmFunctionData", "public BodyDescriptorBase"),
  ("WasmExportedFunctionData", "public BodyDescriptorBase"),
  ("WasmCapiFunctionData", "public BodyDescriptorBase"),
  ("WasmSuspenderObject", "public BodyDescriptorBase"),
  ("WasmTrustedInstanceData", "public BodyDescriptorBase"),
  ("WasmTableObject", "public BodyDescriptorBase"),
  ("WasmGlobalObject", "public BodyDescriptorBase"),
  ("WasmDispatchTable", "public BodyDescriptorBase"),
  ("WasmDispatchTableForImports", "public BodyDescriptorBase"),
  ("WasmArray", "public BodyDescriptorBase"),
  ("WasmStruct", "public BodyDescriptorBase"),
  ("WasmNull", "public DataOnlyBodyDescriptor"),
  ("WasmMemoryObject", "public BodyDescriptorBase"),
  ("ExternalString", "public BodyDescriptorBase"),
  ("CoverageInfo", "public DataOnlyBodyDescriptor"),
  ("InstructionStream", "public BodyDescriptorBase"),
  ("Map", "public BodyDescriptorBase"),
  ("WasmCustomMap", "public BodyDescriptorBase"),
  ("DataHandler", "public BodyDescriptorBase"),
  (
    "Context",
    "public SuffixRangeBodyDescriptor<OFFSET_OF_DATA_START(Context)>",
  ),
  ("NativeContext", "public BodyDescriptorBase"),
  ("Code", "public BodyDescriptorBase"),
  ("CodeWrapper", "public BodyDescriptorBase"),
  ("EmbedderDataArray", "public BodyDescriptorBase"),
  ("EphemeronHashTable", "public BodyDescriptorBase"),
  ("AccessorInfo", "public BodyDescriptorBase"),
  ("InterceptorInfo", "public BodyDescriptorBase"),
  ("FunctionTemplateInfo", "public BodyDescriptorBase"),
  (
    "FixedArray",
    "public SuffixRangeBodyDescriptor<OFFSET_OF_DATA_START(FixedArray)>",
  ),
  (
    "TrustedFixedArray",
    "public SuffixRangeBodyDescriptor<OFFSET_OF_DATA_START(",
  ),
  ("ProtectedFixedArray", "public BodyDescriptorBase"),
  (
    "SloppyArgumentsElements",
    "public SuffixRangeBodyDescriptor<offsetof(SloppyArgumentsElements,",
  ),
  (
    "RegExpMatchInfo",
    "public SuffixRangeBodyDescriptor<offsetof(RegExpMatchInfo,",
  ),
  (
    "ArrayList",
    "public SuffixRangeBodyDescriptor<ArrayList::kHeaderSize>",
  ),
  (
    "WeakArrayList",
    "public SuffixRangeWeakBodyDescriptor<WeakArrayList::kHeaderSize>",
  ),
  (
    "ObjectBoilerplateDescription",
    "public SuffixRangeBodyDescriptor<offsetof(ObjectBoilerplateDescription,",
  ),
  ("FeedbackCell", "public BodyDescriptorBase"),
  (
    "ClosureFeedbackCellArray",
    "public SuffixRangeBodyDescriptor<OFFSET_OF_DATA_START(",
  ),
  (
    "ScriptContextTable",
    "public SuffixRangeBodyDescriptor<offsetof(ScriptContextTable,",
  ),
  (
    "WeakFixedArray",
    "public SuffixRangeWeakBodyDescriptor<OFFSET_OF_DATA_START(",
  ),
  (
    "WeakHomomorphicFixedArray",
    "public SuffixRangeWeakBodyDescriptor<OFFSET_OF_DATA_START(",
  ),
  (
    "TrustedWeakFixedArray",
    "public SuffixRangeWeakBodyDescriptor<OFFSET_OF_DATA_START(",
  ),
  ("ProtectedWeakFixedArray", "public BodyDescriptorBase"),
  ("DoubleStringCache", "public BodyDescriptorBase"),
];

/// 基描述符分布对数 (DataOnly 18 / 其余 101; -inl static_assert 35 处)
pub const DESCRIPTOR_BASE_STATS: (usize, usize, usize) = (18, 101, 35);

/// 编译期完美散列描述符表 (O(1) 静态查找)
pub static BODY_DESCRIPTORS: phf::Map<&'static str, &'static str> = phf::phf_map! {
  "JSAPIObjectWithEmbedderSlots" => "public JSAPIObjectWithEmbedderSlotsOrJSSpecialObjectBodyDescriptor",
  "JSGlobalProxy" => "public JSAPIObjectWithEmbedderSlotsOrJSSpecialObjectBodyDescriptor",
  "CppHeapExternalObject" => "public BodyDescriptorBase",
  "HeapNumber" => "public DataOnlyBodyDescriptor",
  "UninitializedHeapNumber" => "public DataOnlyBodyDescriptor",
  "HashSeedWrapper" => "public DataOnlyBodyDescriptor",
  "TurboshaftWord32SetType" => "public DataOnlyBodyDescriptor",
  "TurboshaftWord64SetType" => "public DataOnlyBodyDescriptor",
  "TurboshaftFloat64SetType" => "public DataOnlyBodyDescriptor",
  "TurbofanBitsetType" => "public DataOnlyBodyDescriptor",
  "TurbofanRangeType" => "public DataOnlyBodyDescriptor",
  "TurbofanOtherNumberConstantType" => "public DataOnlyBodyDescriptor",
  "TurbofanUnionType" => "public FixedBodyDescriptor<offsetof(TurbofanUnionType, type1_),",
  "TurbofanHeapConstantType" => "public FixedBodyDescriptor<offsetof(TurbofanHeapConstantType, constant_),",
  "OnHeapBasicBlockProfilerData" => "public FixedBodyDescriptor<offsetof(OnHeapBasicBlockProfilerData,",
  "SortState" => "public FixedBodyDescriptor<offsetof(SortState, receiver_),",
  "WasmFastApiCallData" => "public FixedWeakBodyDescriptor<offsetof(WasmFastApiCallData, signature_),",
  "WasmStringViewIter" => "public FixedBodyDescriptor<offsetof(WasmStringViewIter, string_),",
  "FreeSpace" => "public DataOnlyBodyDescriptor",
  "JSObject" => "public BodyDescriptorBase",
  "JSMessageObject" => "public FixedBodyDescriptor<offsetof(JSMessageObject, properties_or_hash_),",
  "JSDate" => "public BodyDescriptorBase",
  "JSRegExp" => "public BodyDescriptorBase",
  "RegExpData" => "public BodyDescriptorBase",
  "AtomRegExpData" => "public BodyDescriptorBase",
  "IrRegExpData" => "public BodyDescriptorBase",
  "RegExpDataWrapper" => "public BodyDescriptorBase",
  "ScopeInfo" => "public SuffixRangeBodyDescriptor<offsetof(ScopeInfo, parameter_count_)>",
  "FeedbackVector" => "public BodyDescriptorBase",
  "DescriptorArray" => "public BodyDescriptorBase",
  "WeakCell" => "public BodyDescriptorBase",
  "JSWeakRef" => "public BodyDescriptorBase",
  "JSProxy" => "public BodyDescriptorBase",
  "JSFinalizationRegistry" => "public BodyDescriptorBase",
  "AllocationSite" => "public BodyDescriptorBase",
  "JSFunction" => "public BodyDescriptorBase",
  "JSArrayBuffer" => "public JSAPIObjectWithEmbedderSlots::BodyDescriptor",
  "JSArrayBufferView" => "public JSAPIObjectWithEmbedderSlots::BodyDescriptor",
  "JSTypedArray" => "public JSArrayBufferView::BodyDescriptor",
  "JSDataViewOrRabGsabDataView" => "public JSArrayBufferView::BodyDescriptor",
  "JSExternalObject" => "public BodyDescriptorBase",
  "Foreign" => "public BodyDescriptorBase",
  "CppGCManagedBase" => "public BodyDescriptorBase",
  "SmallOrderedHashTableImpl" => "public BodyDescriptorBase",
  "SwissNameDictionary" => "public BodyDescriptorBase",
  "ByteArray" => "public DataOnlyBodyDescriptor",
  "TrustedByteArray" => "public DataOnlyBodyDescriptor",
  "BytecodeArray" => "public BodyDescriptorBase",
  "BytecodeWrapper" => "public BodyDescriptorBase",
  "BigInt" => "public DataOnlyBodyDescriptor",
  "FixedDoubleArray" => "public DataOnlyBodyDescriptor",
  "FeedbackMetadata" => "public DataOnlyBodyDescriptor",
  "PreparseData" => "public BodyDescriptorBase",
  "InterpreterData" => "public BodyDescriptorBase",
  "UncompiledDataWithoutPreparseData" => "public BodyDescriptorBase",
  "UncompiledDataWithPreparseData" => "public BodyDescriptorBase",
  "UncompiledDataWithoutPreparseDataWithJob" => "public BodyDescriptorBase",
  "UncompiledDataWithPreparseDataAndJob" => "public BodyDescriptorBase",
  "SharedFunctionInfo" => "public BodyDescriptorBase",
  "DebugInfo" => "public BodyDescriptorBase",
  "PrototypeInfo" => "public BodyDescriptorBase",
  "JSWeakCollection" => "public BodyDescriptorBase",
  "JSSynchronizationPrimitive" => "public BodyDescriptorBase",
  "Hole" => "public DataOnlyBodyDescriptor",
  "WasmTypeInfo" => "public BodyDescriptorBase",
  "WasmFuncRef" => "public BodyDescriptorBase",
  "WasmResumeData" => "public BodyDescriptorBase",
  "WasmStackObject" => "public BodyDescriptorBase",
  "WasmSuspendingObject" => "public BodyDescriptorBase",
  "WasmModuleObject" => "public BodyDescriptorBase",
  "WasmInstanceObject" => "public BodyDescriptorBase",
  "WasmTagObject" => "public BodyDescriptorBase",
  "WasmImportData" => "public BodyDescriptorBase",
  "WasmInternalFunction" => "public BodyDescriptorBase",
  "WasmFunctionData" => "public BodyDescriptorBase",
  "WasmExportedFunctionData" => "public BodyDescriptorBase",
  "WasmCapiFunctionData" => "public BodyDescriptorBase",
  "WasmSuspenderObject" => "public BodyDescriptorBase",
  "WasmTrustedInstanceData" => "public BodyDescriptorBase",
  "WasmTableObject" => "public BodyDescriptorBase",
  "WasmGlobalObject" => "public BodyDescriptorBase",
  "WasmDispatchTable" => "public BodyDescriptorBase",
  "WasmDispatchTableForImports" => "public BodyDescriptorBase",
  "WasmArray" => "public BodyDescriptorBase",
  "WasmStruct" => "public BodyDescriptorBase",
  "WasmNull" => "public DataOnlyBodyDescriptor",
  "WasmMemoryObject" => "public BodyDescriptorBase",
  "ExternalString" => "public BodyDescriptorBase",
  "CoverageInfo" => "public DataOnlyBodyDescriptor",
  "InstructionStream" => "public BodyDescriptorBase",
  "Map" => "public BodyDescriptorBase",
  "WasmCustomMap" => "public BodyDescriptorBase",
  "DataHandler" => "public BodyDescriptorBase",
  "Context" => "public SuffixRangeBodyDescriptor<OFFSET_OF_DATA_START(Context)>",
  "NativeContext" => "public BodyDescriptorBase",
  "Code" => "public BodyDescriptorBase",
  "CodeWrapper" => "public BodyDescriptorBase",
  "EmbedderDataArray" => "public BodyDescriptorBase",
  "EphemeronHashTable" => "public BodyDescriptorBase",
  "AccessorInfo" => "public BodyDescriptorBase",
  "InterceptorInfo" => "public BodyDescriptorBase",
  "FunctionTemplateInfo" => "public BodyDescriptorBase",
  "FixedArray" => "public SuffixRangeBodyDescriptor<OFFSET_OF_DATA_START(FixedArray)>",
  "TrustedFixedArray" => "public SuffixRangeBodyDescriptor<OFFSET_OF_DATA_START(",
  "ProtectedFixedArray" => "public BodyDescriptorBase",
  "SloppyArgumentsElements" => "public SuffixRangeBodyDescriptor<offsetof(SloppyArgumentsElements,",
  "RegExpMatchInfo" => "public SuffixRangeBodyDescriptor<offsetof(RegExpMatchInfo,",
  "ArrayList" => "public SuffixRangeBodyDescriptor<ArrayList::kHeaderSize>",
  "WeakArrayList" => "public SuffixRangeWeakBodyDescriptor<WeakArrayList::kHeaderSize>",
  "ObjectBoilerplateDescription" => "public SuffixRangeBodyDescriptor<offsetof(ObjectBoilerplateDescription,",
  "FeedbackCell" => "public BodyDescriptorBase",
  "ClosureFeedbackCellArray" => "public SuffixRangeBodyDescriptor<OFFSET_OF_DATA_START(",
  "ScriptContextTable" => "public SuffixRangeBodyDescriptor<offsetof(ScriptContextTable,",
  "WeakFixedArray" => "public SuffixRangeWeakBodyDescriptor<OFFSET_OF_DATA_START(",
  "WeakHomomorphicFixedArray" => "public SuffixRangeWeakBodyDescriptor<OFFSET_OF_DATA_START(",
  "TrustedWeakFixedArray" => "public SuffixRangeWeakBodyDescriptor<OFFSET_OF_DATA_START(",
  "ProtectedWeakFixedArray" => "public BodyDescriptorBase",
  "DoubleStringCache" => "public BodyDescriptorBase",
};

/// 描述符查询 (原版经 C++ 模板特化解析; 采用 O(1) 编译期完美散列精确查表)
#[inline]
pub fn body_descriptor_of(owner: &str) -> Option<&'static str> {
  BODY_DESCRIPTORS.get(owner).copied()
}
