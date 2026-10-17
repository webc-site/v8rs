#!/usr/bin/env bun
// tool/src/wasm/compilation-hints-generation.cc.js
// 机械提取 v8/src/wasm/compilation-hints-generation.cc 的三节 (编译优先级/指令频度/调用目标)
// 输出布局、频度字节钳制与落盘文件名格式。
// 节名常量在 v8-wasm-core/src/compilation_hints_generation.rs.skel (compilation-hints-generation.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/compilation-hints-generation.cc");

// ---------- 1. 三节名 (未知名自定义节) ----------
if (!src.includes("buffer.write_u8(kUnknownSectionCode);")) throw new Error("未知名节码缺失");
for (const s of ["kCompilationPriorityString", "kInstructionFrequenciesString", "kCallTargetsString"]) {
  if (!src.includes(s)) throw new Error(`节名常量 ${s} 缺失`);
}
// 源注释: 暂不发 wasm-module 前导
if (!src.includes("// Do not emit the wasm-module prelude for now.")) throw new Error("前导抑制注释缺失");

// ---------- 2. tier-up 提示: 尺寸 2 (编译+优化优先级) vs 尺寸 1 (仅编译优先级) ----------
if (!src.includes("buffer.write_u8(2);    // Hint size.")) throw new Error("tier-up 提示尺寸 2 缺失");
if (!src.includes("buffer.write_u8(1);    // Hint size.")) throw new Error("普通提示尺寸 1 缺失");
// 函数级提示偏移恒 0
if (!src.includes("buffer.write_u8(0);  // Offset 0 for function-level hint.")) {
  throw new Error("函数级提示偏移缺失");
}

// ---------- 3. 频度字节: log2(相对调用计数)+32, 钳制 [1, 64] ----------
if (!/uint8_t\{1\},\s*std::min\(uint8_t\{64\}, static_cast<uint8_t>\(\s*std::log2\(relative_call_count\) \+ 32\)\)\)/.test(src)) {
  throw new Error("频度字节钳制缺失");
}
// 提示长度恒 1
if (!src.includes("buffer.write_u32v(1);  // Hint length.")) throw new Error("频度提示长度缺失");

// ---------- 4. 调用目标: 直接调用不发, 间接/引用调用才发 ----------
if (!src.includes("FunctionTypeFeedback::kCallIndirect &&")) throw new Error("间接调用过滤缺失");
if (!src.includes("FunctionTypeFeedback::kCallRef")) throw new Error("引用调用过滤缺失");
// 频度百分比: floor(count * 100 / total)
if (!src.includes("std::floor(static_cast<double>(slot.call_count(call)) * 100 /")) {
  throw new Error("频度百分比公式缺失");
}

// ---------- 5. 起始函数: 跳过导入函数 ----------
if (!/func_index = module->num_imported_functions;/.test(src)) throw new Error("导入跳过缺失");

// ---------- 6. tier-up 后禁止再生成 (liftoff 专属检查) ----------
if (!src.includes("You're holding it wrong! Don't call \"")) throw new Error("tier-up 检查缺失");
if (!src.includes("CHECK(v8_flags.fuzzing);")) throw new Error("fuzzing 崩溃豁免缺失");

// ---------- 7. 落盘文件名格式 ----------
if (!src.includes('"compilation-hints-wasm-%08x.wasm-no-header"')) {
  throw new Error("落盘文件名格式缺失");
}
const fnameBuf = src.match(/std::array<char, (\d+)> filename;/);
if (!fnameBuf || fnameBuf[1] !== "48") throw new Error("文件名缓冲 != 48");

console.log("compilation-hints: 节=3 频度钳=[1,64] 提示尺寸={1,2}");

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/compilation-hints-generation.cc.js",
  "src/wasm/compilation-hints-generation.cc",
  "编译提示三节输出布局 (节名常量在 compilation_hints_generation.rs.skel)",
)}
//! --wasm-generate-compilation-hints 输出格式。

/// 三节均以 kUnknownSectionCode 开, 含节名串/节数/函数条目 (u32v 计数)
/// v8: src/wasm/compilation-hints-generation.cc EmitCompilationHintsToBuffer
#[rustfmt::skip]
pub static HINT_SECTIONS: &[&str] = &[
    "compilation-priority: 函数号 u32v + 偏移 0 + 提示 (tier-up 尺寸 2, 否则 1), 优先级均 0",
    "instruction-frequencies: 函数号 + 槽位数 + 每槽 (偏移 u32v, 长度 1, 频度字节)",
    "call-targets: 函数号 + 提示数 + 每提示 (偏移 u32v, 尺寸, 目标号+百分比对)",
];

/// 不发 wasm magic/version 前导 (与 .wasm 模块区分)
/// v8: src/wasm/compilation-hints-generation.cc EmitCompilationHintsToBuffer
pub const NO_MODULE_PRELUDE: bool = true;

/// 频度字节公式: clamp(log2(槽总计数 / 调用总数) + 32, 1, 64)
/// v8: src/wasm/compilation-hints-generation.cc EmitCompilationHintsToBuffer
pub fn log_relative_call_count(total_count_at_slot: u64, num_invocations: u64) -> u8 {
    let relative = total_count_at_slot as f64 / num_invocations as f64;
    1u8.max(64u8.min((relative.log2() + 32.0) as u8))
}

/// 调用目标过滤: 仅 kCallIndirect / kCallRef 槽发出 (直接调用不发)
/// v8: src/wasm/compilation-hints-generation.cc EmitCompilationHintsToBuffer
#[rustfmt::skip]
pub static CALL_TARGET_FILTER: &[&str] = &["kCallIndirect", "kCallRef"];

/// 目标频度百分比: floor(该目标计数 * 100 / 槽总计数)
/// v8: src/wasm/compilation-hints-generation.cc EmitCompilationHintsToBuffer
pub const TARGET_PERCENTAGE_FORMULA: &str = "floor(count * 100 / total)";

/// 遍历起点: 首个非导入函数
/// v8: src/wasm/compilation-hints-generation.cc EmitCompilationHintsToBuffer
pub const START_AFTER_IMPORTED: &str = "func_index 从 num_imported_functions 起";

/// tier-up 后再生成直接报错 (仅 fuzzing 下豁免崩溃)
/// v8: src/wasm/compilation-hints-generation.cc EmitCompilationHintsToBuffer
pub const NO_TIER_UP_BEFORE_HINTS: &str = "You're holding it wrong!";

/// 落盘文件名: compilation-hints-wasm-<wire hash 08x>.wasm-no-header (缓冲 48 字节)
/// v8: src/wasm/compilation-hints-generation.cc WriteCompilationHintsToFile
pub const OUTPUT_FILENAME_FORMAT: &str = "compilation-hints-wasm-%08x.wasm-no-header";
pub const OUTPUT_FILENAME_BUFFER: usize = ${fnameBuf[1]};

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(HINT_SECTIONS.len(), 3);
    assert_eq!(CALL_TARGET_FILTER.len(), 2);
    assert_eq!(log_relative_call_count(0, 1), 1);
    assert_eq!(log_relative_call_count(1, 1), 33);
    assert_eq!(log_relative_call_count(1 << 40, 1), 64);
    assert_eq!(OUTPUT_FILENAME_BUFFER, 48);
};
`;

const p = writeSrc("v8-wasm-core", "compilation_hints_generation_cc.rs", rust);
console.log(`written ${p}`);
