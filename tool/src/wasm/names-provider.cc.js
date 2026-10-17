#!/usr/bin/env bun
// tool/src/wasm/names-provider.cc.js
// 机械提取 v8/src/wasm/names-provider.cc 的标识符合法字符表与名称合成格式:
// kIdentifierChar 96 项表、非法 UTF16 码元替换规则、导入/导出名 "$mod.field" 格式、
// 默认函数名 "$func<index>"、name section 优先序。
// 头侧状态位已在 v8-wasm-core/src/names_provider_consts.rs (names-provider.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/names-provider.cc");

// ---------- 1. kIdentifierChar 表 (c - 32 索引, 非法字符替换为 '_') ----------
const tableMatch = src.match(
  /static constexpr char kIdentifierChar\[\] = \{([\s\S]*?)\};/,
);
if (!tableMatch) throw new Error("未找到 kIdentifierChar 表");
const body = tableMatch[1].replace(/\/\/[^\n]*/g, "");
// C++ 字符转义正规化: '\'' -> "'", '\\' -> "\"
const chars = [...body.matchAll(/'(\\?.)'/g)].map((m) =>
  m[1].length === 2 && m[1][0] === "\\" ? m[1][1] : m[1],
);
if (chars.length !== 96) throw new Error(`kIdentifierChar ${chars.length} != 96`);

// 关键替换位锚点: 表覆盖 ASCII 32..127 (96 项), 非法字符替换为 '_'
const idx = (i) => String.fromCharCode(32 + i);
const replaced = chars.map((c, i) => (c === "_" && idx(i) !== "_" ? i : -1)).filter((i) => i >= 0);
// 32..127 的可打印区: 表覆盖 32..127 (96 项)
if (replaced.length === 0) throw new Error("kIdentifierChar 无替换位");
// 已知 spec 允许字符必须原样保留: a-z A-Z 0-9 _ ! # $ % & ' * + - . / : < = > ? @ \ ^ ` | ~
for (const good of "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_!#$%&'*+-./:<=>?@\\^`|~") {
  if (chars[good.charCodeAt(0) - 32] !== good) {
    throw new Error(`合法标识符字符 '${good}' 被替换`);
  }
}
// 已知非法字符必须替换为 '_'
for (const bad of ['"', ",", ";", "[", "]", "{", "}", "(", ")", " "]) {
  if (chars[bad.charCodeAt(0) - 32] !== "_") {
    throw new Error(`非法标识符字符 '${bad}' 未替换`);
  }
}

// ---------- 2. SanitizeUnicodeName: 控制字符与高位替换 ----------
if (!src.includes("if (c < 32 || c >= 127) {")) {
  throw new Error("未找到 UTF16 码元替换条件");
}
if (!src.includes("out << kIdentifierChar[c - 32];")) {
  throw new Error("未找到查表索引 c-32");
}

// ---------- 3. 导入名合成 "$<module>.<field>" ----------
const importFmt = src.match(
  /StringBuilder buffer;\s*buffer << '(\$)';\s*SanitizeUnicodeName\(buffer, mod_start, mod_length\);\s*buffer << '(\.)';\s*SanitizeUnicodeName\(buffer, field_start, field_length\);/,
);
if (!importFmt) throw new Error("未找到导入名 $mod.field 合成格式");

// ---------- 4. 默认函数名 "$func<index>" ----------
if (!/out << "(\$func)" << function_index;/.test(src)) {
  throw new Error("未找到默认函数名 $func<index>");
}

// ---------- 5. DevTools 行为加 '$' 前缀, kWasmInternal 不加 ----------
if (!src.includes("// For kWasmInternal behavior, function names don't get a `$` prefix.")) {
  throw new Error("未找到 kWasmInternal 无前缀语义注释");
}

// ---------- 6. 非 function 导入按类别合成 (table/memory/global/tag), name section 优先 ----------
const kinds = [...src.matchAll(/case (kExternal\w+):\s*\n\s*if \(name_section_names_->(\w+)_names_\.Has\(import\.index\)\) continue;/g)].map((m) => [m[1], m[2]]);
if (kinds.length !== 4) throw new Error(`导入类别 ${kinds.length} != 4`);
const kindOrder = kinds.map((k) => k[1]).join(",");
if (kindOrder !== "table,memory,global,tag") throw new Error(`导入类别序异常: ${kindOrder}`);

// ---------- 7. 函数导入/导出跳过条件 (name section 已有则不合成) ----------
if (!src.includes("if (module_->lazily_generated_names.Has(import.index)) continue;")) {
  throw new Error("未找到函数名 name section 优先短路");
}

// ---------- 8. 大小估算基准 ----------
if (!src.includes('PrintF("NamesProvider: %zu\\n", result);')) {
  throw new Error("未找到 NamesProvider 大小打印锚");
}

console.log(`names-provider: 字符表=${chars.length} 替换位=${replaced.length} 导入类别=${kinds.length}`);

// ---------- 生成 Rust ----------
const rustChar = (c) => {
  if (c === "\\") return "'\\\\'";
  if (c === "'") return "'\\''";
  return `'${c}'`;
};
let table = "";
for (let i = 0; i < 96; i += 16) {
  const row = chars.slice(i, i + 16).map(rustChar).join(", ");
  table += `    ${row},\n`;
}

const rust = `${banner(
  "src/wasm/names-provider.cc.js",
  "src/wasm/names-provider.cc",
  "标识符合法字符表与名称合成格式 (状态位在 names_provider_consts.rs)",
)}
//! 名称合成: name section 优先, 缺失时从导入/导出表合成。

/// 标识符合法字符表 (索引 = ASCII 码 - 32; 非法字符替换为 '_')
/// v8: src/wasm/names-provider.cc kIdentifierChar
/// 参考: https://webassembly.github.io/spec/core/text/values.html#text-id
#[rustfmt::skip]
pub static IDENTIFIER_CHAR: [char; 96] = [
${table}];

/// UTF16 码元清洗: 控制字符或非 ASCII 替换为 '_', 其余查表
/// v8: src/wasm/names-provider.cc SanitizeUnicodeName
pub fn sanitize_char(c: u16) -> char {
    if c < 32 || c >= 127 {
        '_'
    } else {
        IDENTIFIER_CHAR[(c - 32) as usize]
    }
}

/// 导入名合成格式: "\\$<module>.<field>"
/// v8: src/wasm/names-provider.cc ComputeImportName
pub const IMPORT_NAME_FORMAT: &str = "$<module>.<field>";

/// 默认函数名格式: "\\$func<index>"
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
`;

const p = writeSrc("v8-wasm-objects", "names_provider_cc.rs", rust);
console.log(`written ${p}`);
