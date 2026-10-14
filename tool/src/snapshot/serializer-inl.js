#!/usr/bin/env bun
// tool/src/snapshot/serializer-inl.js
// 机械提取 v8/src/snapshot/serializer-inl.h 的内联函数面

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/snapshot/serializer-inl.h");

// 提取内联函数签名
const fns = [...src.matchAll(/^(?:inline\s+)?(?:bool|void|int|uint32_t)\s+Serializer::(\w+)\(/gm)].map((m) => m[1]);
if (fns.length !== 1) {
  throw new Error(`serializer-inl.h 函数数不符: 期望 1, 实得 ${fns.length}: ${fns.join(",")}`);
}
if (!src.includes("IsNotMappedSymbol")) throw new Error("未找到 IsNotMappedSymbol");
// 分支登记: V8_EXTERNAL_CODE_SPACE_BOOL
if (!src.includes("V8_EXTERNAL_CODE_SPACE_BOOL")) throw new Error("未找到 V8_EXTERNAL_CODE_SPACE_BOOL 分支");
console.log(`serializer-inl 函数 ${fns.length} 条`);

let rust = banner("src/snapshot/serializer-inl.js", "src/snapshot/serializer-inl.h", "序列化器内联函数面");
rust += `
/// 内联函数面 (原版 serializer-inl.h, 共 ${fns.length} 项):
/// - \`IsNotMappedSymbol\`: 判定对象是否非 not_mapped_symbol 哨兵。
///   分支登记: V8_EXTERNAL_CODE_SPACE_BOOL 为真时按完整指针比较
///   (压缩态 InstructionStream 可能与哨兵压缩值同形), 否则按压缩指针比较。
`;

const p = writeSrc("v8-snapshot", "serializer_inl.rs", rust);
console.log(`written ${p}`);
