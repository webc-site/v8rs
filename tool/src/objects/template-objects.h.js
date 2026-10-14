#!/usr/bin/env bun
// tool/src/objects/template-objects.h.js — 覆盖 v8/src/objects/template-objects.{h,-inl.h,cc,tq} (全新族)
// → v8-objects-js-func/src/template_objects.rs
// 机械提取: TemplateObjectDescription 布局 + GetTemplateObject 语义流程 + 缓存匹配规则。
// 配置登记: 64 位非压缩 (kTaggedSize=8, Struct 头 = map(8))。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/template-objects.h");
const inl = readV8("src/objects/template-objects-inl.h");
const cc = readV8("src/objects/template-objects.cc");
const tq = readV8("src/objects/template-objects.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── 1. h/tq 布局 ───────────────────────────────────────────────────────
must(h.includes("class TemplateObjectDescription final : public Struct {"), "类声明锚缺失");
must(h.includes("const TaggedMember<FixedArray> raw_strings_;") &&
  h.includes("const TaggedMember<FixedArray> cooked_strings_;"), "字段锚缺失");
must(h.includes("static DirectHandle<JSArray> GetTemplateObject("), "GetTemplateObject 声明锚缺失");
const tqCls = tq.match(/extern class (\w+) extends (\w+) \{([^}]*)\n\}/);
must(tqCls[1] === "TemplateObjectDescription" && tqCls[2] === "Struct", "tq 类锚缺失");
const tqFields = [...tqCls[3].matchAll(/^\s*(\w+):\s*([^;]+);/gm)].map((m) => [m[1], m[2].trim()]);
must(JSON.stringify(tqFields) === JSON.stringify([["raw_strings", "FixedArray"], ["cooked_strings", "FixedArray"]]),
  `tq 字段失配: ${JSON.stringify(tqFields)}`);

// ── 2. -inl 方法清单 ───────────────────────────────────────────────────
const inlDefs = [...inl.matchAll(/^(?:\w[\w:<>,& ]* )?TemplateObjectDescription::(\w+)\(/gm)].map((m) => m[1]);
must(inlDefs.length === 3, `-inl 定义应 3 (构造/raw_strings/cooked_strings), 实得 ${JSON.stringify(inlDefs)}`);

// ── 3. cc 语义流程 (GetTemplateObject + CachedTemplateMatches) ────────
const ccDefs = [...new Set([...cc.matchAll(
  /^(?:[A-Za-z_][\w:<>,*&]*[ \t]+)?TemplateObjectDescription::(\w+)\(/gm,
)].map((m) => `TemplateObjectDescription::${m[1]}`))];
must(ccDefs.includes("TemplateObjectDescription::GetTemplateObject"),
  `cc 应含 GetTemplateObject, 实得 ${JSON.stringify(ccDefs)}`);
// 缓存匹配: 新形态 TemplateLiteralObject (map 判别 + 双 id) / 旧形态 symbol 属性
must(cc.includes("bool CachedTemplateMatches(Isolate* isolate,"), "CachedTemplateMatches 定义锚缺失");
must(cc.includes("is_js_array_template_literal_object_map(entry->map())"), "map 判别锚缺失");
must(cc.includes("template_object->function_literal_id() == function_literal_id &&"),
  "新形态双 id 匹配锚缺失");
must(cc.includes("template_literal_function_literal_id_symbol()"), "旧形态 literal id symbol 锚缺失");
must(cc.includes("template_literal_slot_id_symbol()"), "旧形态 slot id symbol 锚缺失");
// 主流程锚
must(cc.includes("shared_info->function_literal_id(kRelaxedLoad);"), "function_literal_id relaxed 锚缺失");
must(cc.includes("EphemeronHashTable::TodoShape::Hash(ReadOnlyRoots(isolate), script);"), "hash 计算锚缺失");
must(cc.includes("template_weakmap->Lookup(script, hash);"), "weakmap 查找锚缺失");
must(cc.includes("// Linear search over the cached template array list"), "线性搜索注释锚缺失");
must(cc.includes("NewJSArrayForTemplateLiteralArray("), "数组构造锚缺失");
must(cc.includes("cached_templates = ArrayList::Add(isolate, cached_templates, template_object);"), "缓存追加锚缺失");
must(cc.includes("template_weakmap = EphemeronHashTable::Put(isolate, template_weakmap,"),
  "weakmap 回写锚缺失");
must(cc.includes("native_context->set_template_weakmap(*template_weakmap);"), "native context 回写锚缺失");

// ── 4. 64 位布局 ───────────────────────────────────────────────────────
const K_SIZE = 8 + 2 * 8; // map + 2 tagged = 24
must(K_SIZE === 24, "布局推导失配");

// ── 5. 生成 ────────────────────────────────────────────────────────────
const body = `${banner("src/objects/template-objects.h.js", "src/objects/template-objects.{h,-inl.h,cc,tq}", "TemplateObjectDescription: tagged template 字面量描述 + 缓存语义")}
//! TemplateObjectDescription: 模板字符串字面量的 raw/cooked 双串表。
//! GetTemplateObject 依赖 (EphemeronHashTable/Factory/NativeContext) 未落地, 语义留 Developer。

/// 64 位布局: map(8) + raw_strings(8) + cooked_strings(8)。
pub const K_TEMPLATE_OBJECT_DESCRIPTION_SIZE: usize = ${K_SIZE};

/// tq 布局字段表 (1:1)。
pub const TEMPLATE_OBJECT_DESCRIPTION_FIELDS: [(&str, &str); 2] = [
${tqFields.map(([n, t]) => `  ("${n}", "${t}"),`).join("\n")}
];

// —— -inl 合并指引 (3 定义) ——
/// v8: src/objects/template-objects-inl.h — 构造 map 取 roots.template_object_description_map。
pub const TEMPLATE_OBJECTS_INL_METHODS: &[&str] = &[
${inlDefs.map((d) => `  "${d}",`).join("\n")}
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
`;

const p = writeSrc("v8-objects-js-func", "template_objects.rs", body);
console.log(
  `template-objects.h.js: ${p} — tq 字段 ${tqFields.length}、-inl ${inlDefs.length}、流程 ${7 + 3} 条、断言 ${checks} 项通过`,
);
