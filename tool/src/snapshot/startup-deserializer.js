#!/usr/bin/env bun
// tool/src/snapshot/startup-deserializer.js
// 机械提取 v8/src/snapshot/startup-deserializer.{h,cc} 的方法面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/startup-deserializer.h");
const ccSrc = readV8("src/snapshot/startup-deserializer.cc");

// 1. 方法面 (h)
const clsM = hSrc.match(/class StartupDeserializer final : public Deserializer<Isolate> \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 StartupDeserializer 类体");
const methods = [...clsM[1].matchAll(/^\s{2}(?:void|bool)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 4) {
  throw new Error(`方法数不符: 期望 4, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. cc 实现清点
const ccFns = [...ccSrc.matchAll(/^void StartupDeserializer::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length < 4) throw new Error(`cc 实现过少: ${ccFns.length}`);
const ccLines = ccSrc.split("\n").length;
console.log(`方法 ${methods.length} + cc ${ccLines} 行 (${ccFns.length} 实现)`);

let rust = banner("src/snapshot/startup-deserializer.js", "src/snapshot/startup-deserializer.{h,cc}", "启动反序列化器");
rust += `
/// StartupDeserializer (原版): 将上下文无关快照数据装入空堆。
/// 方法面 (原版 h, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// cc 实现 (机械清点, ${ccLines} 行): ${ccFns.join(", ")}
/// 契约: 反序列化完成后 FlushICache; 外部引用表逐项
/// DeserializeAndCheckExternalReferenceTable 核对。
`;

const p = writeSrc("v8-snapshot", "startup_deserializer.rs", rust);
console.log(`written ${p}`);
