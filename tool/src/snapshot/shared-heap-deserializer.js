#!/usr/bin/env bun
// tool/src/snapshot/shared-heap-deserializer.js
// 机械提取 v8/src/snapshot/shared-heap-deserializer.{h,cc} 的方法面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/shared-heap-deserializer.h");
const ccSrc = readV8("src/snapshot/shared-heap-deserializer.cc");

// 1. 方法面 (h)
const clsM = hSrc.match(/class SharedHeapDeserializer final : public Deserializer<Isolate> \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 SharedHeapDeserializer 类体");
const methods = [...clsM[1].matchAll(/^\s{2}(?:void|bool)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 2) {
  throw new Error(`方法数不符: 期望 2, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. cc 实现清点
const ccFns = [...ccSrc.matchAll(/^void SharedHeapDeserializer::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length !== methods.length) {
  throw new Error(`cc 实现数 (${ccFns.length}) 与声明 (${methods.length}) 不齐`);
}
console.log(`方法 ${methods.length}`);

let rust = banner("src/snapshot/shared-heap-deserializer.js", "src/snapshot/shared-heap-deserializer.{h,cc}", "共享堆反序列化器");
rust += `
/// SharedHeapDeserializer (原版): 初始化共享 isolate 中未随 startup 快照
/// 装载的对象。方法面 (原版 h, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// 契约: 按运行时旗标决定是否把共享堆对象装入 Isolate;
/// cc 实现: ${ccFns.join(", ")} (字符串表单独装载)。
`;

const p = writeSrc("v8-snapshot", "shared_heap_deserializer.rs", rust);
console.log(`written ${p}`);
