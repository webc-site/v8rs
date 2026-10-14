#!/usr/bin/env bun
// tool/src/objects/property-descriptor.cc.js — 覆盖 v8/src/objects/property-descriptor.cc
// → v8-objects-rest/src/property_descriptor_cc.rs.skel
// 机械提取: ToPropertyDescriptor (ES6 6.2.4.5 步骤 1-23) / FromPropertyDescriptor (ToObject)
//   / CompletePropertyDescriptor 补全规则 / ToPropertyDescriptorObject 九位标志编码。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/property-descriptor.cc");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[property-descriptor.cc.js] ${m}`); };

// ── 1. 匿名命名空间 helper (3 个) ──────────────────────────────────────────
must(cc.includes("bool GetPropertyIfPresent("), "缺 GetPropertyIfPresent");
must(cc.includes("bool ToPropertyDescriptorFastPath("), "缺 ToPropertyDescriptorFastPath");
must(cc.includes("void CreateDataProperty("), "缺 CreateDataProperty");
// 快路径 6 项前置护栏 (map 形态检查)
for (const a of [
  "raw_map->instance_type() != JS_OBJECT_TYPE",
  "raw_map->is_access_check_needed()",
  "raw_map->prototype() != *isolate->initial_object_prototype()",
  "isolate->bootstrapper()->IsActive()",
  "raw_map->is_dictionary_map()",
]) must(cc.includes(a), `快路径护栏缺: ${a}`);
// 快路径六键匹配序 (roots 串)
const keyOrder = [...cc.matchAll(/key == roots\.(\w+)_string\(\)/g)].map((m) => m[1]);
must(keyOrder.join(",") === "enumerable,configurable,value,writable,get,set", `六键序漂移: ${keyOrder.join(",")}`);
// accessor 槽 bail-out (get/set 需 IsCallable)
const callableBail = [...cc.matchAll(/if \(!IsCallable\(\*value\)\) return false;/g)].length;
must(callableBail === 2, `快路径 IsCallable 护栏 ${callableBail} != 2 (get/set)`);

// ── 2. 慢路径步骤锚 (ES6 6.2.4.5, 步骤 23 收尾) ────────────────────────────
must(cc.includes("// 23. Return desc."), "缺步骤 23 收尾");
const stepComments = [...cc.matchAll(/^  \/\/ \d+[^.]*\./gm)].map((m) => m[0]);
must(stepComments.length === 26, `步骤注释数 ${stepComments.length} != 26`);
must(cc.includes("MessageTemplate::kPropertyDescObject"), "缺 kPropertyDescObject 抛错");
must(cc.includes("MessageTemplate::kObjectGetterCallable"), "缺 kObjectGetterCallable 抛错");
must(cc.includes("MessageTemplate::kObjectSetterCallable"), "缺 kObjectSetterCallable 抛错");
must(cc.includes("MessageTemplate::kValueAndAccessor"), "缺 kValueAndAccessor 抛错");
// getter/setter 可调用性: IsCallable(x) 为假且非 undefined 才抛
must(cc.includes("!IsCallable(*getter) && !IsUndefined(*getter)"), "缺 getter 双判");
must(cc.includes("!IsCallable(*setter) && !IsUndefined(*setter)"), "缺 setter 双判");

// ── 3. ToObject 快路径 (两条专用 map) ──────────────────────────────────────
must(cc.includes("isolate->accessor_property_descriptor_map()"), "缺 accessor 专用 map");
must(cc.includes("isolate->data_property_descriptor_map()"), "缺 data 专用 map");
for (const a of [
  "JSAccessorPropertyDescriptor::kGetOffset", "JSAccessorPropertyDescriptor::kSetOffset",
  "JSAccessorPropertyDescriptor::kEnumerableOffset", "JSAccessorPropertyDescriptor::kConfigurableOffset",
  "JSDataPropertyDescriptor::kValueOffset", "JSDataPropertyDescriptor::kWritableOffset",
  "JSDataPropertyDescriptor::kEnumerableOffset", "JSDataPropertyDescriptor::kConfigurableOffset",
]) must(cc.includes(a), `ToObject 快路径缺槽位: ${a}`);

// ── 4. ToPropertyDescriptorObject 九位编码 (定序) ───────────────────────────
const bits = [...cc.matchAll(/PropertyDescriptorObject::(\w+Bit)::encode/g)].map((m) => m[1]);
must(bits.join(",") === "IsEnumerableBit,HasEnumerableBit,IsConfigurableBit,HasConfigurableBit,IsWritableBit,HasWritableBit,HasValueBit,HasGetBit,HasSetBit",
  `九位编码序漂移: ${bits.join(",")}`);

// ── 5. CompletePropertyDescriptor 补全规则 ──────────────────────────────────
must(cc.includes("desc->set_writable(false);"), "缺 writable 补 false");
must(cc.includes("desc->set_enumerable(false);"), "缺 enumerable 补 false");
must(cc.includes("desc->set_configurable(false);"), "缺 configurable 补 false");
must(cc.includes("isolate->factory()->undefined_value()" ), "缺 undefined 补值");

// ── 6. 顶层方法分派 (4 个; ToPropertyDescriptorObject 签名跨行, 逐名断言) ──
const methods = ["ToObject", "ToPropertyDescriptor", "CompletePropertyDescriptor", "ToPropertyDescriptorObject"];
for (const m of methods) must(new RegExp(`PropertyDescriptor::${m}\\(`).test(cc), `缺方法 PropertyDescriptor::${m}`);
must(cc.includes("DirectHandle<JSObject> PropertyDescriptor::ToObject(Isolate* isolate) {"), "ToObject 签名漂移");

let rust = banner("src/objects/property-descriptor.cc.js", "src/objects/property-descriptor.cc",
  "PropertyDescriptor 运行时: ES6 6.2.4.4-6.2.4.6 对象互转 + 九位标志编码");
rust += `//! PropertyDescriptor 运行时面: JS 对象 ↔ PropertyDescriptor 互转 (ES6 6.2.4.4-6)。

// ── ToPropertyDescriptorObject 九位标志编码 (.cc 唯一公式) ──
/// 位序: IsEnumerable | HasEnumerable | IsConfigurable | HasConfigurable
///     | IsWritable | HasWritable | HasValue | HasGet | HasSet (低位起)。
pub fn to_property_descriptor_object_flags(
  enumerable: bool, has_enumerable: bool, configurable: bool, has_configurable: bool,
  writable: bool, has_writable: bool, has_value: bool, has_get: bool, has_set: bool,
) -> i32 {
  (enumerable as i32)
    | ((has_enumerable as i32) << 1)
    | ((configurable as i32) << 2)
    | ((has_configurable as i32) << 3)
    | ((writable as i32) << 4)
    | ((has_writable as i32) << 5)
    | ((has_value as i32) << 6)
    | ((has_get as i32) << 7)
    | ((has_set as i32) << 8)
}

/// 原版 PropertyDescriptor::ToPropertyDescriptor (ES6 6.2.4.5, 步骤 1-23) —
/// 快路径 (JS_OBJECT_TYPE + 纯 data 字段 + 无原形链干扰) 失败即整体重走慢路径;
/// 六键读取序 enumerable→configurable→value→writable→get→set;
/// get/set 须 IsCallable 或 undefined; get/set 与 value/writable 互斥抛 kValueAndAccessor。
pub fn to_property_descriptor(isolate: &Isolate, obj: Handle<JSAny>, desc: &mut PropertyDescriptor) -> bool {
  todo!("就地修订: property-descriptor.cc")
}

/// 原版 PropertyDescriptor::ToObject (ES6 6.2.4.4 FromPropertyDescriptor) —
/// 正则 accessor 用 accessor_property_descriptor_map (kGet/kSet/kEnumerable/kConfigurable);
/// 正则 data 用 data_property_descriptor_map (kValue/kWritable/kEnumerable/kConfigurable);
/// 其余退普通 JSObject 逐键 CreateDataProperty。
pub fn to_object(desc: &PropertyDescriptor, isolate: &Isolate) -> DirectHandle<JSObject> {
  todo!("就地修订: property-descriptor.cc")
}

/// 原版 PropertyDescriptor::CompletePropertyDescriptor (ES6 6.2.4.6) —
/// data 侧补 value=undefined/writable=false; accessor 侧补 get/set=undefined;
/// enumerable/configurable 一律补 false。
pub fn complete_property_descriptor(desc: &mut PropertyDescriptor, isolate: &Isolate) {
  todo!("就地修订: property-descriptor.cc")
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(FLAG_BITS == 9);
  assert!(to_property_descriptor_object_flags(true, true, false, false, false, false, false, false, false) == 0b11);
  assert!(to_property_descriptor_object_flags(false, false, false, false, false, false, false, false, true) == 1 << 8);
};
pub const FLAG_BITS: usize = ${bits.length};
`;

const p = writeSrc("v8-objects-rest", "property_descriptor_cc.rs.skel", rust);
console.log(`property-descriptor.cc.js: ${p} — 方法 ${methods.length} + helper 3 + 六键序 ${keyOrder.length} + 九位编码, 步骤注释 ${stepComments.length}, 断言 ${checks} 项全过`);
