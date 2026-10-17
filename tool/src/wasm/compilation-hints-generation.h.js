#!/usr/bin/env bun
// tool/src/wasm/compilation-hints-generation.h.js
// 机械提取 v8/src/wasm/compilation-hints-generation.{h,cc} 的三个 hint section 布局与编码公式

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/compilation-hints-generation.h");
const ccSrc = readV8("src/wasm/compilation-hints-generation.cc");
const constSrc = readV8("src/wasm/wasm-constants.h");

// 1. h: 两个入口签名
if (!hSrc.includes("void EmitCompilationHintsToBuffer(")) {
  throw new Error("未找到 EmitCompilationHintsToBuffer");
}
if (!hSrc.includes("void WriteCompilationHintsToFile(")) {
  throw new Error("未找到 WriteCompilationHintsToFile");
}

// 2. cc: 三个 section 名常量 (定义在 wasm-constants.h)
const sectionNames = {};
for (const [key, name] of [
  ["kCompilationPriorityString", "COMPILATION_PRIORITY"],
  ["kInstructionFrequenciesString", "INSTRUCTION_FREQUENCIES"],
  ["kCallTargetsString", "CALL_TARGETS"],
]) {
  const m = constSrc.match(new RegExp(`constexpr char ${key}\\[\\] =\\s*"([^"]+)";`));
  if (!m) throw new Error(`wasm-constants.h 未找到 ${key}`);
  sectionNames[name] = m[1];
  if (!ccSrc.includes(key)) throw new Error(`cc 未引用 ${key}`);
}
if (sectionNames.COMPILATION_PRIORITY !== "metadata.code.compilation_priority") {
  throw new Error("compilation_priority 名不符");
}

// 3. cc: tier-up 提示编码 (hint size 1 vs 2)
if (!ccSrc.includes("buffer.write_u8(2);    // Hint size.")) {
  throw new Error("未找到 tierup hint size=2");
}
if (!ccSrc.includes("buffer.write_u8(1);    // Hint size.")) {
  throw new Error("未找到普通 hint size=1");
}
if (!ccSrc.includes("buffer.write_u8(0);  // Offset 0 for function-level hint.")) {
  throw new Error("未找到函数级 hint offset=0");
}

// 4. cc: log 相对调用计数公式钳位 [1, 64] + 32 偏移
const logMatch = ccSrc.match(
  /uint8_t log_relative_call_count = std::max\(\s*uint8_t\{1\},\s*std::min\(uint8_t\{64\}, static_cast<uint8_t>\(\s*std::log2\(relative_call_count\) \+ 32\)\)\);/,
);
if (!logMatch) throw new Error("未找到 log 相对调用计数公式");

// 5. cc: call target 百分比公式
if (!ccSrc.includes("std::floor(static_cast<double>(slot.call_count(call)) * 100 /")) {
  throw new Error("未找到 call target 百分比公式");
}

// 6. cc: 文件名模板与缓冲尺寸
const fileMatch = ccSrc.match(/std::array<char, (\d+)> filename;/);
if (!fileMatch) throw new Error("未找到 filename 缓冲");
const filenameBufSize = parseInt(fileMatch[1], 10);
const tmplMatch = ccSrc.match(/"([^"]*compilation-hints[^"]*)"/);
if (!tmplMatch) throw new Error("未找到文件名模板");
const fileTemplate = tmplMatch[1];

// 7. cc: section 头 kUnknownSectionCode
const unknownCount = (ccSrc.match(/buffer\.write_u8\(kUnknownSectionCode\);/g) || []).length;
if (unknownCount !== 3) throw new Error(`kUnknownSectionCode 出现 ${unknownCount} 次 != 3`);

// 8. 槽位 hint 长度
if (!ccSrc.includes("buffer.write_u32v(1);  // Hint length.")) {
  throw new Error("未找到 instruction-frequency hint length=1");
}

console.log(
  `compilation-hints-generation: sections=${Object.keys(sectionNames).length} ` +
    `unknownSection=${unknownCount} filenameBuf=${filenameBufSize}`,
);

// 9. 生成 Rust
let rust = banner(
  "src/wasm/compilation-hints-generation.h.js",
  "src/wasm/compilation-hints-generation.{h,cc}",
  "compilation hints 三 section 输出格式常量",
);

rust += `
//! 三个自定义 section 的输出格式 (供 wasm-module-builder ZoneBuffer 写出)。
/// v8: src/wasm/compilation-hints-generation.cc EmitCompilationHintsToBuffer

/// section 名 (原版 wasm-constants.h kXxxString)
pub mod section_name {
    /// v8: src/wasm/wasm-constants.h kCompilationPriorityString
    pub const COMPILATION_PRIORITY: &str = "${sectionNames.COMPILATION_PRIORITY}";
    /// v8: src/wasm/wasm-constants.h kInstructionFrequenciesString
    pub const INSTRUCTION_FREQUENCIES: &str = "${sectionNames.INSTRUCTION_FREQUENCIES}";
    /// v8: src/wasm/wasm-constants.h kCallTargetsString
    pub const CALL_TARGETS: &str = "${sectionNames.CALL_TARGETS}";
}

/// 函数级 hint 的 offset 字段固定 0
/// v8: src/wasm/compilation-hints-generation.cc "Offset 0 for function-level hint"
pub const FUNCTION_LEVEL_HINT_OFFSET: u8 = 0;

/// 未标记 tier-up: hint size = 1 (仅 compilation priority)
/// v8: src/wasm/compilation-hints-generation.cc buffer.write_u8(1)
pub const HINT_SIZE_PLAIN: u8 = 1;

/// 标记 tier-up: hint size = 2 (compilation + optimization priority)
/// v8: src/wasm/compilation-hints-generation.cc buffer.write_u8(2)
pub const HINT_SIZE_TIERUP: u8 = 2;

/// instruction-frequency 槽位 hint 长度恒 1
/// v8: src/wasm/compilation-hints-generation.cc "Hint length"
pub const FREQ_HINT_LENGTH: u32 = 1;

/// log 相对调用计数钳位域
/// v8: src/wasm/compilation-hints-generation.cc log_relative_call_count
pub mod log_call_count {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 64;
    /// 编码: clamp(min, max, log2(relative) + 32)
    pub fn encode(relative_call_count: f64) -> u8 {
        let raw = (relative_call_count.log2() + 32.0) as u8;
        raw.clamp(MIN, MAX)
    }
}

/// call-target 百分比: floor(count * 100 / total)
/// v8: src/wasm/compilation-hints-generation.cc call_targets 分支
pub fn call_target_percentage(call_count: u32, total_at_slot: u32) -> u32 {
    ((call_count as f64 * 100.0) / total_at_slot as f64).floor() as u32
}

/// 输出文件名模板: compilation-hints-wasm-<hash:08x>.wasm-no-header
/// v8: src/wasm/compilation-hints-generation.cc WriteCompilationHintsToFile
pub const HINTS_FILENAME_TEMPLATE: &str = "${fileTemplate}";

/// 文件名缓冲尺寸 (std::array<char, N>)
pub const HINTS_FILENAME_BUF_SIZE: usize = ${filenameBufSize};

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(HINT_SIZE_PLAIN, 1);
    assert_eq!(HINT_SIZE_TIERUP, 2);
    assert_eq!(log_call_count::MIN, 1);
    assert_eq!(log_call_count::MAX, 64);
    assert_eq!(HINTS_FILENAME_BUF_SIZE, 48);
    assert!(HINTS_FILENAME_TEMPLATE.starts_with("compilation-hints-wasm-"));
};
`;

const p = writeSrc("v8-wasm-core", "compilation_hints_generation.rs.skel", rust);
console.log(`written ${p}`);
