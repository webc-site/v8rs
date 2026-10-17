// 由 tool/src/wasm/wasm-disassembler.cc.js 从 v8/src/wasm/wasm-disassembler.cc 生成; 修订就地改, 复跑覆盖
// 默认对齐表与 WAT 打印格式 (骨架在 wasm_disassembler_consts.rs)

//! WAT 文本打印规则。

/// 十六进制字符表 (小写用于字节转义, 大写用于立即数)
/// v8: src/wasm/wasm-disassembler.cc kHexChars / kUpperHexChars
pub const HEX_CHARS_LOWER: &str = "0123456789abcdef";
pub const HEX_CHARS_UPPER: &str = "0123456789ABCDEF";

/// 默认对齐表: (log2 对齐, 组内操作码数, 说明)
/// 对齐值即 .wasm 二进制里的 alignment 字段 (4 => 1<<4 == 16 字节);
/// 原子操作的对齐由 MachineType 表示宽派生 (ATOMIC_OP_LIST/ATOMIC_STORE_OP_LIST)
/// v8: src/wasm/wasm-disassembler.cc GetDefaultAlignment
#[rustfmt::skip]
pub static DEFAULT_ALIGNMENT_GROUPS: &[(u32, usize, &str)] = &[
    (4, 2, "S128 整体访问 (kS128LoadMem/kS128StoreMem), 16 字节"),
    (3, 14, "64 位标量与 S128 半宽/64 位 lane 访问, 8 字节"),
    (2, 11, "32 位标量与 S128 32 位 lane 访问, 4 字节"),
    (1, 11, "16 位标量、F16 与 S128 16 位 lane 访问, 2 字节"),
    (0, 9, "8 位标量与 S128 8 位 lane 访问, 1 字节"),
];

/// 原始字符串转义: 控制字符/非 ASCII/引号/反斜杠 -> "\xx" (小写十六进制)
/// v8: src/wasm/wasm-disassembler.cc PrintStringRaw
pub const STRING_ESCAPE_RULE: &str = "b < 32 || b >= 127 || b == '\"' || b == '\\' -> \\xx";

/// 签名单行格式: 带名参数逐个 " (param <名> <类型>)"; 匿名合并 " (param <类型>...)";
/// 每个返回值单独 " (result <类型>)"
/// v8: src/wasm/wasm-disassembler.cc PrintSignatureOneLine
#[rustfmt::skip]
pub static SIGNATURE_FORMATS: &[&str] = &[
    " (param <name> <type>)  — 参数带名",
    " (param <type> ...)     — 参数匿名合并",
    " (result <type>)        — 每返回值一组",
];

/// 语义组指令格式: " (<name>" 打开折叠组
/// v8: src/wasm/wasm-disassembler.cc FunctionBodyDisassembler
pub const OPCODE_GROUP_FORMAT: &str = " (<opcode-name>";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS.len(), 5);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[0].0, 4);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[0].1, 2);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[1].1, 14);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[2].1, 11);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[3].1, 11);
    assert_eq!(DEFAULT_ALIGNMENT_GROUPS[4].1, 9);
    assert_eq!(HEX_CHARS_LOWER.len(), 16);
    assert_eq!(HEX_CHARS_UPPER.as_bytes()[10], b'A');
    assert_eq!(SIGNATURE_FORMATS.len(), 3);
};
