#!/usr/bin/env bun
// tool/src/snapshot/embedded/embedded-data-inl.h.js
// 机械提取 v8/src/snapshot/embedded/embedded-data-inl.h —
// EmbeddedData 内联访问面骨架 (InstructionStartOf/EndOf/SizeOf 族)。
// 注: embedded_data.rs (embedded-data.js 产物) 覆盖 .h; 本脚本落 _inl 旁路。
// 产物: v8-snapshot/src/embedded_data_inl_generated.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const INL_REL = "src/snapshot/embedded/embedded-data-inl.h";
const inl = readV8(INL_REL);

const sigs = [...inl.matchAll(/(?:Address|uint32_t)\s+EmbeddedData::(\w+)\(/gm)].map((m) => m[1]);
if (sigs.length !== 7) throw new Error(`内联方法面不符: ${sigs.length} != 7`);
if (!inl.includes("static_assert(Builtins::kBytecodeHandlersAreSortedLast);")) {
  throw new Error("缺 bytecode handler 排序断言锚");
}
if (!inl.includes("LayoutDescription(builtin)")) throw new Error("缺 LayoutDescription 锚");

let rust = banner("src/snapshot/embedded/embedded-data-inl.h.js", INL_REL, "EmbeddedData 内联访问骨架");
rust += `
// 布局描述 (LayoutDescription: instruction_offset/length + metadata_offset)
// 与段指针由 embedded_data.rs 覆盖; 本文件登记内联访问面。
// 方法注册表 (${sigs.length} 条):
${sigs.map((s) => `// - ${s}`).join("\n")}
// 关键断言锚: Builtins::kBytecodeHandlersAreSortedLast (字节码 handler 段排尾部,
// EndOfBytecodeHandlers 即 code_ + code_size_, 含尾部填充)。

pub struct EmbeddedDataInl;

// TODO(dev): 依 embedded_data.rs 补访问实现与 PadAndAlignCode。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-snapshot", "embedded_data_inl_generated.rs", rust);
console.log(`written ${p} (方法 ${sigs.length})`);
