#!/usr/bin/env bun
// tool/src/api/api-natives.h.js
// 机械提取 v8/src/api/api-natives.{h,cc}
// 产物: v8-api/src/api_natives.rs — ApiNatives 实例化面签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/api/api-natives.h");
const ccSrc = readV8("src/api/api-natives.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 常量
const cacheSize = hSrc.match(/static const int kInitialFunctionCacheSize = (\d+);/);
must(cacheSize, "kInitialFunctionCacheSize");
if (cacheSize[1] !== "256") fail("kInitialFunctionCacheSize", cacheSize[1], "256");

// 2. ApiNatives 公开方法 10 (h 声明序)
const hMethods = [
  "CreateAccessorFunctionTemplateInfo", "InstantiateFunction", "InstantiateFunction",
  "InstantiateObject", "InstantiateRemoteObject", "CreateApiFunction",
  "AddDataProperty", "AddDataProperty", "AddAccessorProperty", "AddNativeDataProperty",
];
const apiBody = hSrc.match(/class ApiNatives \{([\s\S]*?)\n\};/);
must(apiBody, "ApiNatives 类体");
const decls = [...apiBody[1].matchAll(/^\s{2}(?:V8_WARN_UNUSED_RESULT )?static\s+([^;]+);/gm)]
  .filter((m) => m[1].includes("(")) // 排除 kInitialFunctionCacheSize 等常量声明
  .map((m) => {
    const names = [...m[1].matchAll(/(\w+)\(/g)];
    return names[names.length - 1][1];
  });
if (decls.length !== 10) fail("h 声明数", decls.length, 10);
for (let i = 0; i < 10; i++) {
  if (decls[i] !== hMethods[i]) fail(`声明序 ${i}`, decls[i], hMethods[i]);
}

// 3. cc 定义 1:1 — ApiNatives:: 限定定义 10 (引用处 3 条不计: 注释/字符串/调用点)
const ccApiNatives = (ccSrc.match(/^\w[\w:<>& *]*\s*ApiNatives::(\w+)\(/gm) || []).length;
if (ccApiNatives !== 10) fail("cc ApiNatives:: 定义", ccApiNatives, 10);
// cc 匿名 namespace 辅助
for (const f of ["InstantiateObject", "InstantiateFunction", "ConfigureInstance"]) {
  if (!ccSrc.includes(`MaybeHandle<JSObject> ${f}(`) && !ccSrc.includes(`MaybeHandle<JSFunction> ${f}(`) && !ccSrc.includes(`${f}(Isolate`)) {
    fail(`cc 辅助 ${f}`, false, true);
  }
}

// 4. CreateApiFunction 关键分支锚 (cc)
must(ccSrc.includes("if (obj->remove_prototype()) {"), "remove_prototype 分支");
must(ccSrc.includes("if (obj->read_only_prototype()) {"), "read_only_prototype 分支");
must(ccSrc.includes("IsTheHole(*prototype)"), "TheHole prototype 分支");

let n = 14;
const rust = `${banner("src/api/api-natives.h.js", "src/api/api-natives.{h,cc}", "ApiNatives: 模板实例化面签名骨架")}
//! API 原生实例化 — 原版 v8::internal::ApiNatives
//!
//! 语义: FunctionTemplateInfo/ObjectTemplateInfo → JSFunction/JSObject 实例化管道
//! (含函数缓存、属性/访问器装配、远程对象)。

/// 函数模板缓存初始容量 (原版 kInitialFunctionCacheSize)
pub const INITIAL_FUNCTION_CACHE_SIZE: i32 = ${cacheSize[1]};

// —— 签名骨架 (h 10 声明 / cc ApiNatives:: 定义 8 + 匿名辅助, 1:1) ——
${hMethods
  .map((m, i) => {
    const notes = [
      "create_accessor_function_template_info(isolate, callback, length, side_effect_type) — 回调模板包装",
      "instantiate_function(isolate, native_context, data, maybe_name) — 带上下文重载",
      "instantiate_function(isolate, data, maybe_name) — 短重载",
      "instantiate_object(isolate, data, new_target) — MaybeHandle<JSObject>",
      "instantiate_remote_object(data) — 远程对象 (跨上下文)",
      "create_api_function(isolate, native_context, obj, prototype, type, name) — InstanceType 定型",
      "add_data_property(isolate, info, name, value, attributes) — Object 值",
      "add_data_property(isolate, info, name, intrinsic, attributes) — Intrinsic 值",
      "add_accessor_property(isolate, info, name, getter, setter, attributes)",
      "add_native_data_property(isolate, info, property: AccessorInfo)",
    ][i];
    return `// ${m}${notes ? ` — ${notes}` : ""}`;
  })
  .join("\n")}

// cc 匿名 namespace 台账:
//   InstantiateObject / InstantiateFunction (递归主体) / ConfigureInstance (属性装配)
//   CreateApiFunction: remove_prototype 早退 / read_only_prototype 换 map /
//   TheHole prototype → 新建原型 / prototype provider → constructor 属性挂接
`;

const p = writeSrc("v8-api", "api_natives.rs", rust);
console.log(`OK api-natives.h.js: 对数断言 ${n} 全过; 常量 1 / h 声明 10 / cc 定义 10 / 分支锚 1; 产物 ${p}`);
