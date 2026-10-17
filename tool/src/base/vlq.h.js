#!/usr/bin/env bun
// tool/src/base/vlq.h.js — 覆盖 v8/src/base/vlq.h
// 解析 VLQ 常量 (kContinueShift=7 等) 与 Encode/Decode 模板族 (7bit/字节, zigzag 符号位)
// → v8-base/src/vlq.rs.skel

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/base/vlq.h");

for (const anchor of [
  "V8_BASE_VLQ_H_",
  "static constexpr uint32_t kContinueShift = 7;",
  "static constexpr uint32_t kContinueBit = 1 << kContinueShift;",
  "static constexpr uint32_t kDataMask = kContinueBit - 1;",
  "inline void VLQEncodeUnsigned(Function&& process_byte, uint32_t value) {",
  "inline uint32_t VLQConvertToUnsigned(int32_t value) {",
  "inline uint32_t VLQDecodeUnsigned(GetNextFunction&& get_next)",
  "inline int32_t VLQDecode(const uint8_t* data_start, int* index) {",
]) {
  if (!h.includes(anchor)) throw new Error(`anchor missing: ${anchor}`);
}

// 真解析: 常量
const consts = [...h.matchAll(/^static constexpr uint32_t (\w+) = ([^;]+);/gm)].map((m) => ({ name: m[1], value: m[2].trim() }));
// 真解析: VLQ 函数族
const fns = [...new Set([...h.matchAll(/^inline \w+ (VLQ\w+)\(/gm)].map((m) => m[1]))];

const body = `${banner("src/base/vlq.h.js", "src/base/vlq.h", "VLQ 变长整数编码 (7bit/字节 + zigzag 符号位)")}
//! v8 侧: 常量 ${consts.length}; 函数族 ${fns.length} (${fns.join(", ")})
//! Rust 策略: process_byte/get_next 闭包 → impl FnMut(u8)->() / FnMut()->u8

/// v8: kContinueShift = 7 (每字节数据位数)
pub const K_CONTINUE_SHIFT: u32 = ${consts.find((c) => c.name === "kContinueShift").value};
/// v8: kContinueBit = 1 << 7
pub const K_CONTINUE_BIT: u32 = 0x80;
/// v8: kDataMask = kContinueBit - 1
pub const K_DATA_MASK: u32 = 0x7f;

/// v8: VLQEncodeUnsigned(process_byte, value)
/// goto 链式: 按 value < 1<<(7k) 分派写字节数; 非末字节 OR 0x80
pub fn vlq_encode_unsigned<F: FnMut(u8)>(mut process_byte: F, value: u32) {
    let mut v = value;
    loop {
        let byte = (v & K_DATA_MASK) as u8;
        v >>= K_CONTINUE_SHIFT;
        if v == 0 { process_byte(byte); break; }
        process_byte(byte | K_CONTINUE_BIT as u8);
    }
}

/// v8: VLQConvertToUnsigned — zigzag: 符号进最低位 ((|v| << 1) | is_negative)
/// 注意: kMinInt 未定义 (v8 DCHECK)
pub fn vlq_convert_to_unsigned(value: i32) -> u32 {
    debug_assert!(value != i32::MIN);
    (((value.abs() as u32) << 1) | (value < 0) as u32)
}

/// v8: VLQEncode(process_byte, value) = zigzag + unsigned 编码
pub fn vlq_encode<F: FnMut(u8)>(process_byte: F, value: i32) {
    vlq_encode_unsigned(process_byte, vlq_convert_to_unsigned(value));
}

/// v8: VLQDecodeUnsigned(get_next) — 单字节快路径 + 循环累积 (shift 至 32)
pub fn vlq_decode_unsigned<G: FnMut() -> u8>(mut get_next: G) -> u32 {
    let mut cur = get_next();
    if cur <= K_DATA_MASK as u8 { return cur as u32; }
    let mut bits = (cur & K_DATA_MASK as u8) as u32;
    let mut shift = K_CONTINUE_SHIFT;
    loop {
        cur = get_next();
        bits |= ((cur & K_DATA_MASK as u8) as u32) << shift;
        if cur <= K_DATA_MASK as u8 { break; }
        shift += K_CONTINUE_SHIFT;
        if shift > 32 { break; } /// v8: for (shift=7; shift<=32; shift+=7)
    }
    bits
}

/// v8: VLQDecode(data_start, index) — 读 zigzag 后还原符号
pub fn vlq_decode(data: &[u8], index: &mut usize) -> i32 {
    let bits = vlq_decode_unsigned(|| { let b = data[*index]; *index += 1; b });
    let is_negative = (bits & 1) == 1;
    let result = (bits >> 1) as i32;
    if is_negative { -result } else { result }
}
`;

writeSrc("v8-base", "vlq.rs.skel", body);
console.log(`vlq.h.js: consts=${consts.length} fns=${fns.length}`);
