#!/usr/bin/env bun
// tool/src/base/string-format.h.js — 覆盖 v8/src/base/string-format.h
// 解析 JoinedStringViews/FixedSizeString/FormattedStringPart 特化、kMaxLen 公式与 JSONEscaped 转义表
// → v8-base/src/string_format.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/string-format.h");

for (const anchor of [
  "V8_BASE_STRING_FORMAT_H_",
  "struct JoinedStringViews {",
  "concept FixedSizeString =",
  "struct FormattedStringPart<I> {",
  "PrintFormattedStringToArray",
  "class FormattedString {",
  "class JSONEscaped {",
  'if (c == \'"\') return os << "\\\\\\"";',
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 整型 Part 的 kMaxLen 公式元素
const intMaxLen = h.match(/kMaxLen = \(kIs64Bit \? (\d+) : (\d+)\) \+ \(kIsSigned \? (\d+) : (\d+)\);/);
if (!intMaxLen) throw new Error("int kMaxLen formula not found");
// 真解析: 静态限制
const limit128 = (h.match(/kMaxLen < (\d+)/g) || []).map((s) => s.match(/(\d+)/)[1]);
// 真解析: JSON 转义对
const escapes = [...h.matchAll(/if \(c == '(\\?.)'\) return os << "(.*?)";/g)].map((m) => ({ c: m[1], esc: m[2] }));

const body = `${banner("src/base/string-format.h.js", "src/base/string-format.h", "零分配格式化字符串 (FormattedString) 与 JSON 转义")}
//! v8 侧: impl 结构 ${h.includes("struct FormattedStringPart<I>") ? "2 特化" : "?"}; 转义对 ${escapes.length}; 长度上限 ${[...new Set(limit128)].join("/")}
//! Rust 策略: const 泛型数组 + format! 宏; kFormat 静态拼接 → 编译期 &str

/// v8: impl::FormattedStringPart<I> — 整型特化
/// kMaxLen = (64bit ? ${intMaxLen[1]} : ${intMaxLen[2]}) + (signed ? ${intMaxLen[3]} : ${intMaxLen[4]})
/// kFormatPart: %u32/%d32/%u64/%d64 (PRI 宏)
/// Rust: i32/u32/i64/u64 的 itoa 上界即该公式; 格式串由 format! 自动推导

/// v8: impl::FormattedStringPart<S> — 定长字符串特化
/// kMaxLen = N-1; kFormatPart = "%s"; 静态断言 1 <= N <= 128

/// v8: impl::JoinedStringViews — 编译期 string_view 拼接进静态数组
/// Rust: concat! / const fn 拼接等价

/// v8: class FormattedString<Ts...>
/// operator<< 右值追加 (trivial Part 元组); PrintToArray → SNPrintF 进栈上数组
/// 语义: 无动态内存, OOM/致命错误路径可用
pub struct FormattedString<const N: usize> {
    /// v8: parts_ (std::tuple<Part<Ts>...>, trivial)
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> FormattedString<N> {
    /// v8: PrintToArray() — CHECK(characters < kMaxLen), 静态上限 ${[...new Set(limit128)].join(" & ")}
    pub fn print_to_array(&self) -> [u8; N] { self.buf }
}

/// v8: class JSONEscaped — operator<< 逐字符管道
/// 转义对 (v8 原文 ${escapes.length} 条):
${escapes.map((e) => `///   '${e.c}' → "${e.esc}"`).join("\n")}
pub fn json_escape(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            ${escapes.map((e) => `'${e.c}' => out.push_str("${e.esc}"),`).join("\n            ")}
            _ => out.push(c),
        }
    }
}
`;

writeSrc("v8-base", "string_format.rs.skel", body);
console.log(`string-format.h.js: escapes=${escapes.length} limits=${[...new Set(limit128)].join("/")} intMaxLen=${intMaxLen[1]}/${intMaxLen[2]}+${intMaxLen[3]}/${intMaxLen[4]}`);
