// 由 tool/src/objects/elements.h.js 从 v8/src/objects/elements.h + elements-inl.h 生成; 修订就地改, 复跑覆盖
// ElementsAccessor 接面清单与自由函数骨架; 对数 clang -E
//! elements: ElementsAccessor 接面 (多态元素访问器) 的方法清单骨架。
//! 类型系统依赖 Isolate/JSObject/KeyAccumulator 等未落地子系统 — 本模块只登记接面,
//! 语义实现挂 elements_impl (elements.cc 骨架) 与后续 Developer 修订。

/// ElementsAccessor 方法类别。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodKind {
  /// 纯虚 (子类必须实现)
  Pure,
  /// 虚 (仅析构)
  Virtual,
  /// 内联转发 (定义于 elements-inl.h)
  Inline,
  /// 静态 (ForKind / InitializeOncePerProcess / TearDown / Concat)
  Static,
}

/// v8: src/objects/elements.h class ElementsAccessor 方法全集 (clang -E 对数: 47 项)。
pub const ELEMENTS_ACCESSOR_METHODS: [(&str, MethodKind); 47] = [
  ("Validate", MethodKind::Pure),
  ("HasElement", MethodKind::Pure),
  ("HasElement", MethodKind::Inline),
  ("HasEntry", MethodKind::Pure),
  ("Get", MethodKind::Pure),
  ("GetAtomic", MethodKind::Pure),
  ("HasAccessors", MethodKind::Pure),
  ("NumberOfElements", MethodKind::Pure),
  ("SetLength", MethodKind::Pure),
  ("CollectElementIndices", MethodKind::Pure),
  ("CollectElementIndices", MethodKind::Inline),
  ("CollectValuesOrEntries", MethodKind::Pure),
  ("PrependElementIndices", MethodKind::Pure),
  ("PrependElementIndices", MethodKind::Inline),
  ("AddElementsToKeyAccumulator", MethodKind::Pure),
  ("TransitionElementsKind", MethodKind::Pure),
  ("GrowCapacityAndConvert", MethodKind::Pure),
  ("GrowCapacity", MethodKind::Pure),
  ("InitializeOncePerProcess", MethodKind::Static),
  ("TearDown", MethodKind::Static),
  ("Set", MethodKind::Pure),
  ("SetAtomic", MethodKind::Pure),
  ("SwapAtomic", MethodKind::Pure),
  ("CompareAndSwapAtomic", MethodKind::Pure),
  ("Add", MethodKind::Pure),
  ("Concat", MethodKind::Static),
  ("Push", MethodKind::Pure),
  ("Unshift", MethodKind::Pure),
  ("Pop", MethodKind::Pure),
  ("Shift", MethodKind::Pure),
  ("Normalize", MethodKind::Pure),
  ("GetCapacity", MethodKind::Pure),
  ("Fill", MethodKind::Pure),
  ("IncludesValue", MethodKind::Pure),
  ("IndexOfValue", MethodKind::Pure),
  ("LastIndexOfValue", MethodKind::Pure),
  ("Reverse", MethodKind::Pure),
  ("CopyElements", MethodKind::Pure),
  ("CreateListFromArrayLike", MethodKind::Pure),
  ("CopyTypedArrayElementsSlice", MethodKind::Pure),
  ("GetEntryForIndex", MethodKind::Pure),
  ("GetDetails", MethodKind::Pure),
  ("Reconfigure", MethodKind::Pure),
  ("Delete", MethodKind::Pure),
  ("ForKind", MethodKind::Static),
  ("~ElementsAccessor", MethodKind::Virtual),
  ("ElementsAccessor", MethodKind::Inline),
];

/// 纯虚方法名单 (38; 每个访问器子类的实现义务清单)。
pub const PURE_VIRTUAL_METHODS: [&str; 38] = [
  "Validate",
  "HasElement",
  "HasEntry",
  "Get",
  "GetAtomic",
  "HasAccessors",
  "NumberOfElements",
  "SetLength",
  "CollectElementIndices",
  "CollectValuesOrEntries",
  "PrependElementIndices",
  "AddElementsToKeyAccumulator",
  "TransitionElementsKind",
  "GrowCapacityAndConvert",
  "GrowCapacity",
  "Set",
  "SetAtomic",
  "SwapAtomic",
  "CompareAndSwapAtomic",
  "Add",
  "Push",
  "Unshift",
  "Pop",
  "Shift",
  "Normalize",
  "GetCapacity",
  "Fill",
  "IncludesValue",
  "IndexOfValue",
  "LastIndexOfValue",
  "Reverse",
  "CopyElements",
  "CreateListFromArrayLike",
  "CopyTypedArrayElementsSlice",
  "GetEntryForIndex",
  "GetDetails",
  "Reconfigure",
  "Delete",
];

/// v8: src/objects/elements.h elements-inl.h 转发体语义
/// (CollectElementIndices / PrependElementIndices / HasElement 以 holder->elements() 为后备仓)。
pub const INL_FORWARDERS: [&str; 3] = [
  "CollectElementIndices",
  "PrependElementIndices",
  "HasElement",
];

/// v8: src/objects/elements.h ArrayConstructInitializeElements
pub fn array_construct_initialize_elements() {
  todo!("readme 台账 ELEM-ISO: 需 Isolate + Factory + JSArray/JSObject / JavaScriptArguments");
}

/// v8: src/objects/elements.h CopyFastNumberJSArrayElementsToTypedArray
pub fn copy_fast_number_js_array_elements_to_typed_array() {
  todo!("readme 台账 ELEM-CSA: 需 Context, JSArray, JSTypedArray 堆对象定义");
}

/// v8: src/objects/elements.h CopyTypedArrayElementsToTypedArray
pub fn copy_typed_array_elements_to_typed_array() {
  todo!("readme 台账 ELEM-CSA: 需 JSTypedArray 堆对象定义");
}

/// v8: src/objects/elements.h CopyTypedArrayElementsSlice
pub fn copy_typed_array_elements_slice() {
  todo!("readme 台账 ELEM-CSA: 需 JSTypedArray 堆对象定义");
}

// —— 提取不变式断言 (clang -E 对数; 各 kind 分项计数在生成侧锁定后烧入) ——
const _: () = {
  assert!(ELEMENTS_ACCESSOR_METHODS.len() == 47);
  assert!(PURE_VIRTUAL_METHODS.len() == 38);
  assert!(INL_FORWARDERS.len() == 3);
  assert!(ELEMENTS_ACCESSOR_METHODS.len() == 38 + 1 + 4 + 4);
};
