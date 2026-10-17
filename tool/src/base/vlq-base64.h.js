#!/usr/bin/env bun
// tool/src/base/vlq-base64.h.js — 覆盖 v8/src/base/vlq-base64.{h,cc}
// 解析 base64 charToDigit 128 项查找表 (规则展开)、5bit VLQ 常量与解码循环
// → v8-base/src/vlq_base64.rs.skel

import { readV8, writeSrc, banner, lines } from "../../lib.js";

const h = readV8("src/base/vlq-base64.h");
const cc = readV8("src/base/vlq-base64.cc");

for (const anchor of [
  [h, "V8_BASE_VLQ_BASE64_H_"],
  [h, "int8_t charToDigitDecodeForTesting(uint8_t c);"],
  [h, "int32_t VLQBase64Decode(const char* start, size_t sz,"],
  [cc, "constexpr int8_t kCharToDigit[] = {"],
  [cc, "constexpr uint32_t kContinueShift = 5;"],
  [cc, "int32_t VLQBase64Decode(const char* start, size_t sz, size_t* pos) {"],
]) {
  if (!anchor[0].includes(anchor[1])) throw new Error(`anchor missing: ${anchor[1]}`);
}

// 真解析: kCharToDigit 表 → 128 项
const tableMatch = cc.match(/constexpr int8_t kCharToDigit\[\] = \{([\s\S]*?)\};/);
if (!tableMatch) throw new Error("kCharToDigit table not found");
const table = tableMatch[1].split(",").map((s) => s.trim()).filter((s) => s.length > 0).map((s) => (s.startsWith("0x") ? parseInt(s, 16) : parseInt(s, 10)));
if (table.length !== 128) throw new Error(`expect 128 table entries, got ${table.length}`);

// 真解析: cc 常量 (5bit 版)
const consts = [...cc.matchAll(/^constexpr uint32_t (\w+) = ([^;]+);/gm)].map((m) => ({ name: m[1], value: m[2].trim() }));

// 格式化为 Rust 数组 (8 项/行)
const rows = [];
for (let i = 0; i < table.length; i += 8) rows.push("    " + table.slice(i, i + 8).join(", ") + ",");

const body = `${banner("src/base/vlq-base64.h.js", "src/base/vlq-base64.h + .cc", "source map 用 VLQ-Base64 解码")}
//! v8 侧: 查找表 128 项; 常量 ${consts.length}; 导出 2 (charToDigitDecodeForTesting + VLQBase64Decode)

/// v8: src/base/vlq-base64.cc kCharToDigit[128]
/// ASCII → base64 值; -1 = 非法字符 ('+'=0x3e, '/'=0x3f, '0'-'9'=0x34-0x3d,
/// 'A'-'Z'=0x00-0x19, 'a'-'z'=0x1a-0x33)
pub static K_CHAR_TO_DIGIT: [i8; 128] = [
${rows.join("\n")}
];

/// v8: kContinueShift = 5 (base64 版每字节 5bit 数据)
pub const K_CONTINUE_SHIFT: u32 = ${consts.find((c) => c.name === "kContinueShift").value};
/// v8: kContinueMask = 1 << 5
pub const K_CONTINUE_MASK: u32 = 0x20;
/// v8: kDataMask = kContinueMask - 1
pub const K_DATA_MASK: u32 = 0x1f;

/// v8: charToDigitDecode(c) — c >= 128 时 -1
pub fn char_to_digit_decode(c: u8) -> i8 {
    if c < 128 { K_CHAR_TO_DIGIT[c as usize] } else { -1 }
}

/// v8: VLQBase64Decode(start, sz, pos)
/// 语义: do-while 读 digit; 越界/非法字符/末字节高 3 位非零 → i32::MIN (错误值);
/// zigzag 还原: res&1 → -(res>>1)
pub fn vlq_base64_decode(start: &[u8], pos: &mut usize) -> i32 {
    let mut res: u32 = 0;
    let mut shift: u32 = 0;
    loop {
        if *pos >= start.len() { return i32::MIN; }
        let digit = char_to_digit_decode(start[*pos]) as i32;
        let is_last_byte = shift + K_CONTINUE_SHIFT >= 32;
        if digit == -1 || (is_last_byte && (digit >> 2) != 0) { return i32::MIN; }
        res += ((digit & K_DATA_MASK as i32) as u32) << shift;
        shift += K_CONTINUE_SHIFT;
        *pos += 1;
        if (digit & K_CONTINUE_MASK as i32) == 0 { break; }
    }
    if (res & 1) == 1 { -((res >> 1) as i32) } else { (res >> 1) as i32 }
}
`;

writeSrc("v8-base", "vlq_base64.rs.skel", body);
console.log(`vlq-base64.h.js: table=${table.length} consts=${consts.length} exports=2`);
