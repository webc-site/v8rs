// 由 tool/src/wasm/wasm-tracing.cc.js 从 v8/src/wasm/wasm-tracing.cc 生成; 修订就地改, 复跑覆盖
// 内存/全局追踪打印格式 (记录布局在 wasm_tracing.rs.skel)

//! --trace-wasm 输出格式。

/// 内存追踪头格式 (tier 左对齐 8, 函数号 5, 帧位 0x 左对齐 4, 内存号, 操作码名左对齐 12, 方向, 偏移 016x)
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceString
pub const MEMORY_TRACE_HEADER_FORMAT: &str = "%-8s func %5d:0x%-4x mem %d %-12s %s %016x val: ";

/// 存/取方向标签 (4 字符定宽)
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceString
pub const DIRECTION_STORE: &str = "to  ";
pub const DIRECTION_LOAD: &str = "from";

/// 数值打印格式: 标签右对齐 4 + 十进制 / 十六进制
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceValue
pub const VALUE_FORMAT_U8: &str = "%4s:%u / 0x%x";
pub const VALUE_FORMAT_GENERIC: &str = "%4s:%v / 0x%x";

/// SIMD128 打印格式: 4 个 u32 十进制 + 4 个 %08x
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceString (kSimd128)
pub const VALUE_FORMAT_S128: &str = "s128:%d %d %d %d / %08x %08x %08x %08x";

/// MachineRepresentation -> 打印标签 (7 项)
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceString
#[rustfmt::skip]
pub static REP_TAGS: &[(&str, &str)] = &[
    ("kWord8", "i8"),
    ("kWord16", "i16"),
    ("kWord32", "i32"),
    ("kWord64", "i64"),
    ("kFloat32", "f32"),
    ("kFloat64", "f64"),
    ("kSimd128", "s128"),
];

/// 浮点位表示: 打印用浮点解释, 十六进制位用等宽整数解释
/// v8: src/wasm/wasm-tracing.cc PrintMemoryTraceValue<float, uint32_t> 等
pub const FLOAT_BIT_REPS: &[(&str, &str)] = &[("f32", "uint32_t"), ("f64", "uint64_t")];

/// 全局追踪头格式 (无内存号与方向, 含全局号与类型名)
/// v8: src/wasm/wasm-tracing.cc PrintGlobalTraceString
pub const GLOBAL_TRACE_HEADER_FORMAT: &str = "%-8s func %5d:0x%-4x %s %d val: %s:";

/// 全局值分派: 数值类型走 WasmValue.to_string, 引用类型打 "0x%x"
/// v8: src/wasm/wasm-tracing.cc PrintGlobalTraceString
pub const GLOBAL_REF_VALUE_FORMAT: &str = "0x%x";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(REP_TAGS.len(), 7);
    assert_eq!(DIRECTION_STORE.len(), 4);
    assert_eq!(DIRECTION_LOAD.len(), 4);
    assert_eq!(FLOAT_BIT_REPS.len(), 2);
};
