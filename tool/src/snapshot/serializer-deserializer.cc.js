#!/usr/bin/env bun
// tool/src/snapshot/serializer-deserializer.cc.js
// 机械提取 v8/src/snapshot/serializer-deserializer.cc — 缓存遍历/延迟判定骨架。
// 注: serializer_deserializer.rs (.h 侧) 已存在, 本脚本落 _cc_generated 旁路。
// 产物: v8-snapshot/src/serializer_deserializer_cc_generated.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const CC_REL = "src/snapshot/serializer-deserializer.cc";
const cc = readV8(CC_REL);

const sigs = [...cc.matchAll(/^(?:void|bool)\s+SerializerDeserializer::(\w+)\(/gm)].map((m) => m[1]);
if (sigs.length !== 3) throw new Error(`方法面不符: ${sigs.length} != 3`);
// CanBeDeferred 的类型豁免清单 (ByteArray/…不可延迟)
const deferReasons = [...cc.matchAll(/([\w ]+) cannot be deferred/g)].map((m) => m[1].trim());
if (deferReasons.length < 2) throw new Error(`不可延迟情形清单异常: ${deferReasons.length}`);

let rust = banner("src/snapshot/serializer-deserializer.cc.js", CC_REL, "序列化器-反序列化器 cc 面骨架");
rust += `
// 方法注册表 (${sigs.length} 条):
${sigs.map((s) => `// - ${s}`).join("\n")}
// 不可延迟情形登记 (${deferReasons.length} 条):
// 不可延迟情形: ${deferReasons.map((r) => `// - ${r}`).join("\n")}
// 语义面 (缓存遍历/豁免判定) 登记跳过。

pub struct SerializerDeserializerCc;

// TODO(dev): 依 serializer_deserializer.rs 补 Iterate*/CanBeDeferred。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-snapshot", "serializer_deserializer_cc_generated.rs", rust);
console.log(`written ${p} (方法 ${sigs.length})`);
