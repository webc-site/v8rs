#!/usr/bin/env bun
// tool/src/snapshot/context-serializer.js
// 机械提取 v8/src/snapshot/context-serializer.{h,cc} 的方法面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/context-serializer.h");
const ccSrc = readV8("src/snapshot/context-serializer.cc");

// 1. ContextSerializer 方法面 (h, public + private)
const clsM = hSrc.match(/class V8_EXPORT_PRIVATE ContextSerializer : public Serializer \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 ContextSerializer 类体");
const methods = [...clsM[1].matchAll(/^\s{2,4}(?:bool|void)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 7) {
  throw new Error(`ContextSerializer 方法数不符: 期望 7, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. cc 规模
const ccLines = ccSrc.split("\n").length;
const ccFns = [...ccSrc.matchAll(/^(?:void|bool) ContextSerializer::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length < 4) throw new Error(`cc 实现过少: ${ccFns.length}`);
console.log(`方法 ${methods.length} + cc ${ccLines} 行 (${ccFns.length} 实现)`);

let rust = banner("src/snapshot/context-serializer.js", "src/snapshot/context-serializer.{h,cc}", "上下文序列化器");
rust += `
/// ContextSerializer (原版, 继承 RootsSerializer): 序列化上下文依赖对象图
/// (不含代码对象)。方法面 (原版 h, 共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// 契约: embedder 字段与 API wrapper 字段单独序列化
/// (SerializeObjectWithEmbedderFields / SerializeApiWrapperFields);
/// 启动对象缓存准入由 ShouldBeInTheStartupObjectCache 判定。
/// cc 实现 (机械清点, ${ccLines} 行): ${ccFns.join(", ")}
`;

const p = writeSrc("v8-snapshot", "context_serializer.rs", rust);
console.log(`written ${p}`);
