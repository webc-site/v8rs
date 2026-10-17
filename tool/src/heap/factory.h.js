#!/usr/bin/env bun
// tool/src/heap/factory.h.js
// 机械提取 v8/src/heap/factory.{h,cc,-inl.h}:
// Factory 类台账: DECLARE_ERROR X-macro 13 错误构造器展开 + New* 方法名清单
// (正则提取计数) + ROOT_ACCESSOR (MUTABLE_ROOT_LIST) 展开 + InternalizeString 族锚。
// factory 类本体 5000+ 行语义实现留 Developer; 此处产规则清单。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/factory.h");
const cc = readV8("src/heap/factory.cc");
const inl = readV8("src/heap/factory-inl.h");

// —— DECLARE_ERROR 13 项展开 ——
const errBlock = h.match(/#define DECLARE_ERROR\(NAME\)[\s\S]*?#undef DECLARE_ERROR/);
if (!errBlock) throw new Error("DECLARE_ERROR 区缺失");
const errors = [...errBlock[0].matchAll(/DECLARE_ERROR\((\w+)\)/g)].map((m) => m[1]).filter((n) => n !== "NAME");
if (errors.length !== 12) throw new Error(`DECLARE_ERROR ${errors.length}, 期望 12`);
if (errors[0] !== "Error" || errors[11] !== "WasmExceptionError") {
  throw new Error(`错误构造器首尾漂移: ${errors[0]}/${errors[12]}`);
}
// —— New* 方法名清单 ——
const news = [...new Set([...h.matchAll(/(?:Direct)?Handle<[\w:<>, ]+>\s+(New\w+)\(/g)].map((m) => m[1]))];
if (news.length < 180) throw new Error(`Factory New* ${news.length}, 期望 >= 180`);
for (const must of ["NewJSArray", "NewSharedFunctionInfoForBuiltin", "NewStringFromUtf8", "NewStringFromStaticChars"]) {
  if (!news.includes(must)) throw new Error(`New* 清单缺 ${must}`);
}
// —— 关键锚 ——
for (const a of [
  "class V8_EXPORT_PRIVATE Factory : public FactoryBase<Factory> {",
  "using FactoryBase<Factory>::New;",
  "using FactoryBase::InternalizeString;",
  "#define ROOT_ACCESSOR(Type, name, CamelName) inline Handle<Type> name();",
  "MUTABLE_ROOT_LIST(ROOT_ACCESSOR)",
  "Handle<String> SizeToString(size_t value, bool check_cache = true);",
  "inline DirectHandle<String> Uint32ToString(uint32_t value,",
  "friend class FactoryBase<Factory>;",
  "// Customization points for FactoryBase",
]) {
  if (!h.includes(a)) throw new Error(`factory.h 锚缺失: ${a}`);
}
// cc 定义计数对数: DEFINE_ERROR X-macro 展开 (WasmExceptionError 定义在 wasm 侧, 不在此 cc)
const ccErrors = [...cc.matchAll(/DEFINE_ERROR\((\w+), (\w+)\)/g)].map((m) => m[1]).filter((n) => n !== "NAME");
if (ccErrors.length !== 11) throw new Error(`cc DEFINE_ERROR ${ccErrors.length}, 期望 11`);
if (ccErrors[0] !== "Error" || ccErrors[10] !== "WasmRuntimeError") throw new Error("cc DEFINE_ERROR 首尾漂移");
const missingInCc = errors.filter((e) => !ccErrors.includes(e));
if (missingInCc.join() !== "WasmExceptionError") throw new Error(`h/cc 差集异常: ${missingInCc}`);
if (!cc.includes("Handle<String> Factory::SizeToString(")) throw new Error("cc SizeToString 缺失");
// inl
for (const a of [
  "Handle<String> Factory::InternalizeString(Handle<T> string) {",
  "Handle<Name> Factory::InternalizeName(Handle<T> name) {",
  "Handle<String> Factory::NewStringFromStaticChars(const char (&str)[N],",
  "HandleType<String> Factory::NewSubString(HandleType<T> str, uint32_t begin,",
  "Handle<JSArray> Factory::NewJSArrayWithElements(",
]) {
  if (!inl.includes(a)) throw new Error(`factory-inl.h 锚缺失: ${a}`);
}
console.log(`Factory: 错误构造器 ${errors.length} + New* ${news.length} + ROOT_ACCESSOR 展开面`);

let rust = banner(
  "src/heap/factory.h.js",
  "src/heap/factory.{h,cc,-inl.h}",
  "主工厂: 错误构造器 X-macro 展开 + New* 方法清单",
);

rust += `
/// 错误对象构造器清单 (对应 DECLARE_ERROR X-macro 12 展开)。
/// v8: src/heap/factory.h DECLARE_ERROR
pub const FACTORY_ERROR_CONSTRUCTORS: &[&str] = &[
${errors.map((e) => `  "${e}",`).join("\n")}
];

/// Factory::New<Error> 族签名骨架 (对应 New##NAME(MessageTemplate, args...)):
/// 模板消息 + 参数向量 → 新 JSObject (错误实例)。
/// v8: src/heap/factory.cc Factory::NewError
pub fn new_error_skeleton(template_index: u32, args: &[u64]) -> Result<u64, ()> {
  let _ = (template_index, args);
  unimplemented!("骨架: NewError → error_function/JSObject 构造待对象族就位");
}

/// Factory New* 方法名清单 (正则提取去重, 与 h 声明面 1:1 计数)。
pub const FACTORY_NEW_METHODS: &[&str] = &[
${news.map((n) => `  "${n}",`).join("\n")}
];

/// 对应 SizeToString/Uint32ToString: 数值 → 缓存优先字符串。
/// v8: src/heap/factory.cc Factory::SizeToString
pub fn size_to_string_skeleton(_value: usize, check_cache: bool) -> u64 {
  let _ = check_cache;
  unimplemented!("骨架: number_string_cache 查填待 roots/字符串族就位");
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(FACTORY_ERROR_CONSTRUCTORS.len(), 12);
  assert_eq!(FACTORY_ERROR_CONSTRUCTORS[0], "Error");
  assert_eq!(FACTORY_ERROR_CONSTRUCTORS[11], "WasmExceptionError");
  assert!(FACTORY_ERROR_CONSTRUCTORS.contains(&"TypeError"));
  assert!(FACTORY_NEW_METHODS.contains(&"NewJSArray"));
  assert!(FACTORY_NEW_METHODS.len() >= 180);
};
`;

const p = writeSrc("v8-heap-core", "factory.rs.skel", rust);
console.log(`written ${p}`);
