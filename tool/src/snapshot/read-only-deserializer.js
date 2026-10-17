#!/usr/bin/env bun
// tool/src/snapshot/read-only-deserializer.js
// 机械提取 v8/src/snapshot/read-only-deserializer.{h,cc} 的方法面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/read-only-deserializer.h");
const ccSrc = readV8("src/snapshot/read-only-deserializer.cc");

// 1. 方法面 (h)
const clsM = hSrc.match(/class ReadOnlyDeserializer final : public Deserializer<Isolate> \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 ReadOnlyDeserializer 类体");
const methods = [...clsM[1].matchAll(/^\s{2}(?:void|bool)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 2) {
  throw new Error(`方法数不符: 期望 2, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. cc 实现清点
const ccFns = [...ccSrc.matchAll(/^void ReadOnlyDeserializer::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length !== methods.length) {
  throw new Error(`cc 实现数 (${ccFns.length}) 与头声明 (${methods.length}) 不齐`);
}
const ccLines = ccSrc.split("\n").length;
// V8_STATIC_ROOTS 分支登记
const staticRoots = (ccSrc.match(/#ifdef V8_STATIC_ROOTS|#if .*V8_STATIC_ROOTS/g) || []).length;
console.log(`方法 ${methods.length} + cc ${ccLines} 行 + STATIC_ROOTS 分支 ${staticRoots}`);

let rust = banner("src/snapshot/read-only-deserializer.js", "src/snapshot/read-only-deserializer.{h,cc}", "只读空间反序列化器");
rust += `
/// ReadOnlyDeserializer (原版, 继承 Deserializer<Isolate>): 反序列化 RO blob,
/// 建立只读根表。方法面 (原版 h, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// cc 实现 (${ccLines} 行, 机械清点): ${ccFns.join(", ")}
/// 分支登记: V8_STATIC_ROOTS x${staticRoots} — 静态根模式下 RO 页直接按
/// 预编译常量表落位, 跳过重定位; 否则走 EncodedTagged 重定位路径。
`;

const p = writeSrc("v8-snapshot", "read_only_deserializer.rs", rust);
console.log(`written ${p}`);
