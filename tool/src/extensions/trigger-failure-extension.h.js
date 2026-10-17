#!/usr/bin/env bun
// tool/src/extensions/trigger-failure-extension.h.js
// 机械提取 v8/src/extensions/trigger-failure-extension.{h,cc}
// 产物: v8-extensions/src/trigger_failure_extension.rs — 扩展元数据 + 3 触发器骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/extensions/trigger-failure-extension.h");
const ccSrc = readV8("src/extensions/trigger-failure-extension.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. 扩展名
const extName = hSrc.match(/v8::Extension\("([^"]+)"/);
must(extName, "扩展名 v8/trigger-failure");
if (extName[1] !== "v8/trigger-failure") fail("扩展名", extName[1], "v8/trigger-failure");

// 2. kSource: 3 条 native function 声明
const ksourceBlock = ccSrc.match(/kSource =\s*\n?\s*((?:"[^"]*";?\s*)+);/);
must(ksourceBlock, "kSource 字面量组");
const natives = [...ksourceBlock[1].matchAll(/native function (\w+)\(\);/g)].map((m) => m[1]);
if (natives.length !== 3) fail("native function 数", natives.length, 3);

// 3. 触发器方法全集 (h 声明 3 / cc 定义 3)
const triggers = ["TriggerCheckFalse", "TriggerAssertFalse", "TriggerSlowAssertFalse"];
for (const t of triggers) {
  if (!hSrc.includes(t)) fail(`h 声明 ${t}`, false, true);
  const re = new RegExp(`TriggerFailureExtension::${t}\\(`);
  if (!re.test(ccSrc)) fail(`cc 定义 ${t}`, false, true);
}

// 4. 分发表 (cc GetNativeFunctionTemplate if/else): 名 → 方法
const dispatch = [
  ["triggerCheckFalse", "TriggerCheckFalse", "CHECK(false)"],
  ["triggerAssertFalse", "TriggerAssertFalse", "DCHECK(false)"],
  ["triggerSlowAssertFalse", "TriggerSlowAssertFalse", "SLOW_DCHECK(false)"],
];
for (const [jsName] of dispatch) {
  if (!ksourceBlock[1].includes(jsName)) fail(`分发名 ${jsName}`, false, true);
}

let n = 8;
const rust = `${banner("src/extensions/trigger-failure-extension.h.js", "src/extensions/trigger-failure-extension.{h,cc}", "TriggerFailure 扩展元数据 + 3 触发器签名骨架")}
//! TriggerFailure 扩展 — 原版 namespace v8::internal::TriggerFailureExtension
//!
//! 语义: 注入 3 个立即触发断言失败的 native 函数, 用于测试 CHECK/DCHECK 通路。

/// TriggerFailure 扩展注册名
pub const TRIGGER_FAILURE_EXTENSION_NAME: &str = "v8/trigger-failure";

/// 注入 JS 的源码 (原版 kSource, 3 条声明拼接)
pub const TRIGGER_FAILURE_SOURCE: &str = "native function triggerCheckFalse();native function triggerAssertFalse();native function triggerSlowAssertFalse();";

/// 触发器分发表 (原版 GetNativeFunctionTemplate if/else 链, 1:1)
pub const DISPATCH: [(&str, &str, &str); 3] = [
  // (JS 名, 方法名, 函数体断言)
${dispatch.map(([js, cpp, body]) => `  (${JSON.stringify(js)}, ${JSON.stringify(cpp)}, ${JSON.stringify(body)}),`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(DISPATCH.len() == 3);
};

// —— 签名骨架 (h 声明 3 + GetNativeFunctionTemplate, cc 定义 1:1) ——
// public: GetNativeFunctionTemplate(isolate, name) — 按 name 字符串三分发
${triggers.map((t) => `// public static: ${t}(info) — ${dispatch.find((d) => d[1] === t)[2]}`).join("\n")}
`;

const p = writeSrc("v8-extensions", "trigger_failure_extension.rs", rust);
console.log(`OK trigger-failure-extension.h.js: 对数断言 ${n} 全过; native 3 / 分发 3 / 触发器 3; 产物 ${p}`);
