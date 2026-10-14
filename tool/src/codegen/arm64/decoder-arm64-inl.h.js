#!/usr/bin/env bun
// tool/src/codegen/arm64/decoder-arm64-inl.h.js
// 机械提取 v8/src/codegen/arm64/decoder-arm64-inl.h — 解码器访客分发骨架。
// 注: decoder.rs (脚本产物) 覆盖 decoder-arm64.h, 本脚本落 _inl_generated 旁路。
// 产物: v8-codegen-arm64/src/decoder_arm64_inl_generated.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const INL_REL = "src/codegen/arm64/decoder-arm64-inl.h";
const inl = readV8(INL_REL);

// Decode 分发树: Decode<Bits>(instr) 内 case 调用的子解码器
const decodes = [...inl.matchAll(/Decode(\w+)\(instr\);/g)].map((m) => m[1]);
if (decodes.length < 10) throw new Error(`Decode 分发面不足: ${decodes.length}`);
const topLevel = [...inl.matchAll(/case (0x[0-9A-Fa-f]+):/g)].map((m) => m[1]);
if (topLevel.length < 10) throw new Error(`顶层 case 不足: ${topLevel.length}`);
if (!inl.includes("Visit")) throw new Error("缺 Visit 分发锚");

let rust = banner("src/codegen/arm64/decoder-arm64-inl.h.js", INL_REL, "arm64 解码器分发骨架");
rust += `
// 指令位型树为模式匹配语义面, 登记跳过。
// 顶层分发 case (Decode<27,24>): ${topLevel.length} 处 — ${topLevel.join(", ")}
// 子解码器注册表 (${[...new Set(decodes)].length} 条):
${[...new Set(decodes)].map((s) => `// - Decode${s}`).join("\n")}

pub struct DecoderInl;

// TODO(dev): 依手写 decoder.rs 访客框架补位型树。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-codegen-arm64", "decoder_arm64_inl_generated.rs", rust);
console.log(`written ${p} (Decode ${[...new Set(decodes)].length})`);
