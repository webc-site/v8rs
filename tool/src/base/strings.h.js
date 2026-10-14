#!/usr/bin/env bun
// tool/src/base/strings.h.js — 覆盖 v8/src/base/strings.{h,cc}
// 解析 uc16/uc32 别名、kUC16Size、VSNPrintF/SNPrintF/StrNCpy 转发与 HexValue/HexCharOfValue 内联
// → v8-base/src/strings.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/strings.h");
const cc = readV8("src/base/strings.cc");

for (const anchor of [
  [h, "V8_BASE_STRINGS_H_"],
  [h, "using uc16 = uint16_t;"],
  [h, "constexpr int kUC16Size = sizeof(uc16);"],
  [h, "int PRINTF_FORMAT(2, 0)"],
  [h, "int PRINTF_FORMAT(2, 3)"],
  [h, "V8_BASE_EXPORT void StrNCpy(base::Vector<char> dest, const char* src, size_t n);"],
  [h, "inline int HexValue(uc32 c) {"],
  [h, "inline char HexCharOfValue(int value) {"],
  [cc, "return OS::VSNPrintF(str.begin(), str.length(), format, args);"],
  [cc, "base::OS::StrNCpy(dest.begin(), dest.length(), src, n);"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: 导出/内联函数与别名
const exports = [...h.matchAll(/^V8_BASE_EXPORT \w+[\s\S]{0,80}?(\w+)\(/g)].map((m) => m[1]);
const inlines = [...h.matchAll(/^inline \w+ (\w+)\(/gm)].map((m) => m[1]);
const aliases = [...h.matchAll(/^using (\w+) = (\w+);/gm)].map((m) => ({ from: m[1], to: m[2] }));

const body = `${banner("src/base/strings.h.js", "src/base/strings.h + .cc", "C 字符串格式化/拷贝转发与十六进制转换")}
//! v8 侧: 导出 ${exports.length} + 内联 ${inlines.length}; 类型别名 ${aliases.length}
//! Rust 策略: SNPrintF → core::fmt 写 &mut [u8]; va_list 无对应, 用格式 trait

${aliases.map((a) => `/// v8: using ${a.from} = ${a.to}
pub type ${a.from} = ${a.to === "uint16_t" ? "u16" : "u32"};`).join("\n")}
/// v8: kUC16Size = sizeof(uc16)
pub const K_UC16_SIZE: usize = 2;

/// v8: VSNPrintF(str, format, args) — 转发 OS::VSNPrintF
/// Rust: 语义面用 core::fmt::Write + Ada 风格截断返回, 就地修订
pub fn vsnprint_f(buf: &mut [u8], format: &str, args: core::fmt::Arguments) -> i32 { todo!() }

/// v8: SNPrintF(str, format, ...) — 保证 null 结尾; 截断返回 -1
pub fn snprint_f(buf: &mut [u8], args: core::fmt::Arguments) -> i32 { todo!() }

/// v8: StrNCpy(dest, src, n) — 转发 OS::StrNCpy
pub fn str_ncpy(dest: &mut [u8], src: &[u8], n: usize) { todo!() }

/// v8: HexValue(c) — '0'-'9'/'a'-'f'/'A'-'F' → 0..15, 非法 < 0
/// v8 判定式: c -= '0'; <=9 直返; (c|0x20)-('a'-'0') ∈ [0,5] 返回 +10
pub fn hex_value(c: u32) -> i32 {
    let c = (c as i32).wrapping_sub('0' as i32);
    if (0..=9).contains(&c) { return c; }
    let c = (c | 0x20) - ('a' as i32 - '0' as i32);
    if (0..=5).contains(&c) { return c + 10; }
    -1
}

/// v8: HexCharOfValue(value) — 0..15 → '0'-'9'/'A'-'F'
pub fn hex_char_of_value(value: i32) -> u8 {
    debug_assert!((0..=16).contains(&value));
    if value < 10 { (value + '0' as i32) as u8 } else { (value - 10 + 'A' as i32) as u8 }
}
`;

writeSrc("v8-base", "strings.rs.skel", body);
console.log(`strings.h.js: exports=${exports.length} inlines=${inlines.length} aliases=${aliases.length}`);
