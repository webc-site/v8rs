// 由 tool/src/wasm/names-provider.cc.js 从 v8/src/wasm/names-provider.cc 生成; 修订就地改, 复跑覆盖
// 标识符合法字符表与名称合成格式 (状态位在 names_provider_consts.rs)

//! 名称合成: name section 优先, 缺失时从导入/导出表合成。

/// 标识符合法字符表 (索引 = ASCII 码 - 32; 非法字符替换为 '_')
/// v8: src/wasm/names-provider.cc kIdentifierChar
/// 参考: https://webassembly.github.io/spec/core/text/values.html#text-id
#[rustfmt::skip]
pub static IDENTIFIER_CHAR: [char; 96] = [
    '_', '!', '_', '#', '$', '%', '&', '\'', '_', '_', '*', '+', '_', '-', '.', '/',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', '_', '<', '=', '>', '?',
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_', '\\', '_', '^', '_',
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_', '|', '_', '~', '_',
];

/// UTF16 码元清洗: 控制字符或非 ASCII 替换为 '_', 其余查表
/// v8: src/wasm/names-provider.cc SanitizeUnicodeName
pub fn sanitize_char(c: u16) -> char {
    if c < 32 || c >= 127 {
        '_'
    } else {
        IDENTIFIER_CHAR[(c - 32) as usize]
    }
}

/// 导入名合成格式: "\$<module>.<field>"
/// v8: src/wasm/names-provider.cc ComputeImportName
pub const IMPORT_NAME_FORMAT: &str = "$<module>.<field>";

/// 默认函数名格式: "\$func<index>"
/// v8: src/wasm/names-provider.cc PrintFunctionName
pub const DEFAULT_FUNCTION_NAME_FORMAT: &str = "$func<index>";

/// DevTools 行为: 函数名带 '$' 前缀; kWasmInternal 行为: 不带前缀
/// v8: src/wasm/names-provider.cc PrintFunctionName
pub const DEVTOOLS_DOLLAR_PREFIX: bool = true;
pub const WASM_INTERNAL_DOLLAR_PREFIX: bool = false;

/// 非 function 导入的合成类别 (name section 优先, 4 类)
/// v8: src/wasm/names-provider.cc ComputeNamesFromImportsExports
#[rustfmt::skip]
pub static IMPORT_NAME_KINDS: &[(&str, &str)] = &[
    ("kExternalTable", "table"),
    ("kExternalMemory", "memory"),
    ("kExternalGlobal", "global"),
    ("kExternalTag", "tag"),
];

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(IDENTIFIER_CHAR.len(), 96);
    assert_eq!(sanitize_char('a' as u16), 'a');
    assert_eq!(sanitize_char('$' as u16), '$');
    assert_eq!(sanitize_char('_' as u16), '_');
    assert_eq!(sanitize_char(31), '_');
    assert_eq!(sanitize_char(127), '_');
    // '"' (0x22) 与 ',' (0x2c) 是 spec 非法字符, 查表后替换
    assert_eq!(IDENTIFIER_CHAR[0x22 - 32], '_');
    assert_eq!(IDENTIFIER_CHAR[0x2c - 32], '_');
    assert_eq!(IMPORT_NAME_KINDS.len(), 4);
};
