#!/usr/bin/env bun
// tool/src/snapshot/context-deserializer.js
// 机械提取 v8/src/snapshot/context-deserializer.{h,cc} 的方法面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/context-deserializer.h");
const ccSrc = readV8("src/snapshot/context-deserializer.cc");

// 1. 方法面 (h)
const clsM = hSrc.match(/class V8_EXPORT_PRIVATE ContextDeserializer final\s*: public Deserializer<Isolate> \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 ContextDeserializer 类体");
const methods = [...clsM[1].matchAll(/^\s{2,4}(?:static\s+)?(?:MaybeDirectHandle<\w+>|void)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 4) {
  throw new Error(`ContextDeserializer 方法数不符: 期望 4, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. cc 实现清点
const ccFns = [...ccSrc.matchAll(/^(?:MaybeDirectHandle<\w+>|void) ContextDeserializer::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length !== methods.length) {
  throw new Error(`cc 实现数 (${ccFns.length}) 与声明 (${methods.length}) 不齐`);
}
const ccLines = ccSrc.split("\n").length;
console.log(`方法 ${methods.length} + cc ${ccLines} 行`);

let rust = banner("src/snapshot/context-deserializer.js", "src/snapshot/context-deserializer.{h,cc}", "上下文反序列化器");
rust += `
/// ContextDeserializer (原版): 反序列化以给定对象为根的上下文依赖对象图,
/// 不应反序列化任何代码对象。方法面 (原版 h, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// 契约: embedder 字段 (DeserializeEmbedderFields) 与 API wrapper 字段
/// (DeserializeApiWrapperFields) 在对象图装载后单独填回。
/// cc 实现: ${ccFns.join(", ")} (共 ${ccLines} 行)。
`;

const p = writeSrc("v8-snapshot", "context_deserializer.rs", rust);
console.log(`written ${p}`);
