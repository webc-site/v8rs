// 由 tool/src/wasm/compilation-hints-generation.cc.js 从 v8/src/wasm/compilation-hints-generation.cc 生成; 修订就地改, 复跑覆盖
// 编译提示三节输出布局 (节名常量在 compilation_hints_generation.rs.skel)

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
pub const OUTPUT_FILENAME_BUFFER: usize = 48;

// —— 提取不变式断言 ——
const _: () = {
  assert!(HINT_SECTIONS.len() == 3);
  assert!(CALL_TARGET_FILTER.len() == 2);
  assert!(OUTPUT_FILENAME_BUFFER == 48);
};
