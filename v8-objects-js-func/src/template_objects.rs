// 由 tool/src/objects/template-objects.h.js 从 v8/src/objects/template-objects.{h,-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// TemplateObjectDescription: tagged template 字面量描述 + 缓存语义

//! TemplateObjectDescription: 模板字符串字面量的 raw/cooked 双串表。
//! GetTemplateObject 依赖 (EphemeronHashTable/Factory/NativeContext) 未落地, 语义留 Developer。

/// 64 位布局: map(8) + raw_strings(8) + cooked_strings(8)。
pub const K_TEMPLATE_OBJECT_DESCRIPTION_SIZE: usize = 24;

/// tq 布局字段表 (1:1)。
pub const TEMPLATE_OBJECT_DESCRIPTION_FIELDS: [(&str, &str); 2] = [
  ("raw_strings", "FixedArray"),
  ("cooked_strings", "FixedArray"),
];

// —— -inl 合并指引 (3 定义) ——
/// v8: src/objects/template-objects-inl.h — 构造 map 取 roots.template_object_description_map。
pub const TEMPLATE_OBJECTS_INL_METHODS: &[&str] = &[
  "TemplateObjectDescription",
  "raw_strings",
  "cooked_strings",
];

// —— cc 语义: GetTemplateObject 流程 ——
/// v8: src/objects/template-objects.cc GetTemplateObject — 步骤序列:
pub const GET_TEMPLATE_OBJECT_STEPS: &[&str] = &[
  "1. function_literal_id = shared_info.function_literal_id (relaxed)",
  "2. hash = EphemeronHashTable.Hash(script)",
  "3. template_weakmap 存在时 Lookup(script) → ArrayList 线性搜索 CachedTemplateMatches",
  "4. 命中 → 直接返回缓存的 JSArray",
  "5. 未命中 → NewJSArrayForTemplateLiteralArray(cooked, raw, function_literal_id, slot_id)",
  "6. cached_templates 不存在则新建 (容量 1), ArrayList::Add 追加",
  "7. 与 weakmap 中旧列表不同 → EphemeronHashTable::Put 回写 + set_template_weakmap",
];

/// v8: src/objects/template-objects.cc CachedTemplateMatches — 双形态:
///   - 新形态: entry map == is_js_array_template_literal_object_map →
///     TemplateLiteralObject 判 function_literal_id 与 slot_id 双相等
///   - 旧形态: GetDataProperty 读 template_literal_function_literal_id_symbol /
///     template_literal_slot_id_symbol 两个 Smi 比对
pub const CACHED_TEMPLATE_MATCHES_RULES: &[&str] = &[
  "新形态判别: is_js_array_template_literal_object_map(entry.map)",
  "新形态匹配: function_literal_id 相等且 slot_id 相等",
  "旧形态匹配: symbol 属性读出的两个 Smi 分别比对",
];
