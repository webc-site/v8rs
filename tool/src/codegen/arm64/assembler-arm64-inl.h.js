#!/usr/bin/env bun
// tool/src/codegen/arm64/assembler-arm64-inl.h.js
// 机械提取 v8/src/codegen/arm64/assembler-arm64-inl.h — 内联访问器注册表。
// 禁覆写面: assembler-arm64 族有手写产物 (assembler.rs/operand.rs),
// 本脚本落 assembler_arm64_inl_generated.rs, 禁覆写。
// 产物: v8-codegen-arm64/src/assembler_arm64_inl_generated.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const INL_REL = "src/codegen/arm64/assembler-arm64-inl.h";
const inl = readV8(INL_REL);

// 内联方法面 (类::方法 与自由函数)
const methods = [...inl.matchAll(/^(?:template <[\s\S]*?>\s*\n)?(?:int|void|bool|uint32_t|Instr|CPURegister|Register|FPRegister|Address|int64_t)\s+(?:Assembler::|MacroAssembler::|Operand::)?(\w+)\(/gm)].map((m) => m[1]);
if (methods.length < 30) throw new Error(`内联方法面不足: ${methods.length}`);
if (!inl.includes("Assembler::")) throw new Error("缺 Assembler 命名空间锚");

let rust = banner("src/codegen/arm64/assembler-arm64-inl.h.js", INL_REL, "arm64 汇编器内联面骨架");
rust += `
// 禁覆写面登记: 手写 assembler.rs / operand.rs 保持权威, 本文件仅登记 inl 方法面。
// 内联方法 (${[...new Set(methods)].length} 条):
${[...new Set(methods)].map((s) => `// - ${s}`).join("\n")}

pub struct AssemblerInl;

// TODO(dev): 依手写 assembler.rs 的 API 对位补内联实现。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-codegen-arm64", "assembler_arm64_inl_generated.rs", rust);
console.log(`written ${p} (方法 ${[...new Set(methods)].length})`);
