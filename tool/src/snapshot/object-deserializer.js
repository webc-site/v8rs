#!/usr/bin/env bun
// tool/src/snapshot/object-deserializer.js
// 机械提取 v8/src/snapshot/object-deserializer.{h,cc} 的双线程变体方法面

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/snapshot/object-deserializer.h");
const ccSrc = readV8("src/snapshot/object-deserializer.cc");

// 1. ObjectDeserializer 方法面
const clsM = hSrc.match(/class ObjectDeserializer final : public Deserializer<Isolate> \{([\s\S]*?)\n\};/);
if (!clsM) throw new Error("未找到 ObjectDeserializer 类体");
const methods = [...clsM[1].matchAll(/^\s{2,4}(?:static\s+)?(?:MaybeDirectHandle<\w+>|void)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length !== 4) {
  throw new Error(`ObjectDeserializer 方法数不符: 期望 4, 实得 ${methods.length}: ${methods.join(",")}`);
}

// 2. OffThreadObjectDeserializer 方法面
const offM = hSrc.match(/class OffThreadObjectDeserializer final : public Deserializer<LocalIsolate> \{([\s\S]*?)\n\};/);
if (!offM) throw new Error("未找到 OffThreadObjectDeserializer 类体");
const offMethods = [...offM[1].matchAll(/^\s{2,4}(?:static\s+)?(?:MaybeDirectHandle<\w+>|void)\s+(\w+)\(/gm)].map((m) => m[1]);
if (offMethods.length !== 3) {
  throw new Error(`OffThreadObjectDeserializer 方法数不符: 期望 3, 实得 ${offMethods.length}`);
}

// 3. cc 实现清点
const ccFns = [...ccSrc.matchAll(/^(?:MaybeDirectHandle<\w+>|void) \w+Deserializer::(\w+)\(/gm)].map((m) => m[1]);
if (ccFns.length !== 5) throw new Error(`cc 实现不符: 期望 5, 实得 ${ccFns.length}`);
console.log(`Object ${methods.length} + OffThread ${offMethods.length} + cc ${ccFns.length} 实现`);

let rust = banner("src/snapshot/object-deserializer.js", "src/snapshot/object-deserializer.{h,cc}", "对象图反序列化器");
rust += `
/// ObjectDeserializer (原版, Deserializer<Isolate>): 反序列化以对象为根的图,
/// 失败优雅降级。方法面 (共 ${methods.length} 项):
`;
for (const m of methods) rust += `/// - \`${m}\`\n`;
rust += `
/// OffThreadObjectDeserializer (原版, Deserializer<LocalIsolate>): 后台线程变体,
/// 反序列化的脚本句柄外移给调用方。方法面 (共 ${offMethods.length} 项):
`;
for (const m of offMethods) rust += `/// - \`${m}\`\n`;
rust += `
/// 契约: LinkAllocationSites 把新分配点回链至旧 allocation site;
/// CommitPostProcessedObjects 提交脚本/SFI 等后处理产物。
/// cc 实现 (机械清点): ${ccFns.join(", ")}
`;

const p = writeSrc("v8-snapshot", "object_deserializer.rs", rust);
console.log(`written ${p}`);
