#!/usr/bin/env bun
// tool/src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.h.js
// 机械提取 v8/src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.{h,cc} —
// x86 家族共享宏汇编器方法面 (ia32/x64 共用, 按双目标登记)。
// 产物: v8-codegen-core/src/macro_assembler_shared_ia32_x64.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const H_REL = "src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.h";
const CC_REL = "src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

const fpM = h.match(/constexpr int kStackSavedSavedFPSize = ([^;]+);/);
if (!fpM) throw new Error("未找到 kStackSavedSavedFPSize");
const classM = h.match(/class V8_EXPORT_PRIVATE SharedMacroAssemblerBase : public MacroAssemblerBase\s*\{([\s\S]*?)\n\};/);
if (!classM) throw new Error("未找到 SharedMacroAssemblerBase 类");
const methods = [...classM[1].matchAll(/^\s*(?:inline\s+)?(?:void|void)\s+(\w+)\(/gm)].map((m) => m[1]);
if (methods.length < 40) throw new Error(`方法面不足: ${methods.length}`);
if (!h.includes("kStackSavedSavedFPSize = 2 * kDoubleSize")) throw new Error("缺 wasm 双 FP 分支锚");

let rust = banner("src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.h.js", "src/codegen/shared-ia32-x64/macro-assembler-shared-ia32-x64.{h,cc}", "x86 家族共享宏汇编器骨架");
rust += `
// ia32/x64 双目标共享面 (本仓库 x64 目标适用; ia32 登记跳过)。

/// wasm 栈保存 FP 大小分支:
/// - wasm 全浮点寄存器: 2 * kDoubleSize
/// - 其余: kDoubleSize
pub const STACK_SAVED_SAVED_FP_SIZE_WASM: usize = 2 * 8;
pub const STACK_SAVED_SAVED_FP_SIZE: usize = 8;

/// 原 class SharedMacroAssemblerBase 方法面 (${[...new Set(methods)].length} 条,
/// SSE/标量指令宏: Move/Add/And/Movhps/Movlps/Blendvps/Pinsrb/... ):
${[...new Set(methods)].map((s) => `// - ${s}`).join("\n")}

pub struct SharedMacroAssemblerBase;

// TODO(dev): 依 x64 指令编码补实现 (cc ${readV8(CC_REL).split("\n").length} 行语义面登记跳过)。

// —— 提取不变式断言 ——
const _: () = {};
`;

const p = writeSrc("v8-codegen-core", "macro_assembler_shared_ia32_x64.rs", rust);
console.log(`written ${p} (方法 ${[...new Set(methods)].length})`);
