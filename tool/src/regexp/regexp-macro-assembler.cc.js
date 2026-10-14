#!/usr/bin/env bun
// tool/src/regexp/regexp-macro-assembler.cc.js — v8/src/regexp/regexp-macro-assembler.cc
//   → v8-regexp-compiler/src/macro_assembler_cc_generated.rs
// 机械提取: word_character_map_ 256 字节位图 (逐字节 0x00/0xFF 字面量解析) +
//   CalculateBoundsCheckOffset 规则 + RangeArrayLengthFor 开区间规则 +
//   IsCharacterInRangeArray 二分/奇偶下标规则 + 方法签名骨架清单;
// 对数断言: 字节 256 / 置位字节 63 (0-9 A-Z _ a-z) / 位计数 504。
import { banner, readV8, writeSrc } from "../../lib.js";

const cc = readV8("src/regexp/regexp-macro-assembler.cc");

// ── word_character_map_ 256 字节表 (cc:719-761) ──────────────────────────
const mapMatch = cc.match(/const uint8_t RegExpMacroAssembler::word_character_map_\[\] = \{([\s\S]*?)\};/);
if (!mapMatch) throw new Error("word_character_map_ 表未找到");
const bytes = [...mapMatch[1].matchAll(/0x(\w\w)u/g)].map((r) => parseInt(r[1], 16));
if (bytes.length !== 256) throw new Error(`位图应 256 字节, 实数 ${bytes.length}`);
const setBytes = bytes.filter((b) => b === 0xff).length;
// ASCII 词字符: '0'-'9' (10) + 'A'-'Z' (26) + '_' (1) + 'a'-'z' (26) = 63
if (setBytes !== 63) throw new Error(`置位字节应 63, 实数 ${setBytes}`);
for (let i = 0; i < 256; i++) {
  if (bytes[i] !== 0x00 && bytes[i] !== 0xff) throw new Error(`位图字节 ${i} 非纯位值: ${bytes[i]}`);
}
// 逐段位计数 = 8×置位字节
const totalBits = setBytes * 8;
if (totalBits !== 504) throw new Error(`位计数应 504, 实数 ${totalBits}`);
// 锚: '0'-'9' 段 (0x30-0x39) 前 8 字节 0x00 (0x28-0x2f) 后 10 字节 0xff
for (let i = 0x30; i <= 0x39; i++) {
  if (bytes[i] !== 0xff) throw new Error(`数字段 ${i} 应置位`);
}
if (bytes[0x5f] !== 0xff) throw new Error("下划线 0x5f 应置位");
if (bytes[0x60] !== 0x00) throw new Error("反引号 0x60 应清位");

// ── CalculateBoundsCheckOffset (cc:323-326) ──────────────────────────────
if (!cc.includes("return cp_offset >= 0 ? cp_offset + characters - 1 : cp_offset;")) {
  throw new Error("边界检查偏移公式漂移");
}
// kDefaultBoundsCheckOffset 哨兵使用面
if (!cc.includes("bounds_check_offset == kDefaultBoundsCheckOffset")) {
  throw new Error("kDefaultBoundsCheckOffset 哨兵面缺失");
}

// ── RangeArrayLengthFor 开区间规则 (cc:165-172) ──────────────────────────
if (!cc.includes("MaskEndOfRangeMarker(ranges->at(ranges_length - 1).to()) == kMaxUInt16")) {
  throw new Error("开区间判定漂移");
}
if (!cc.includes("? ranges_length * 2 - 1\n             : ranges_length * 2;") &&
    !cc.includes("ranges_length * 2 - 1")) {
  throw new Error("开区间长度公式漂移");
}
// 排布: from 闭 to 开 (to+1), 末段开区间省尾
if (!cc.includes("range_array->set(i * 2 + 1, to + 1);  // Exclusive.")) throw new Error("排布 to+1 规则漂移");
if (!cc.includes("// Avoid overflow by leaving the last range open-ended.")) throw new Error("末段开区间注释漂移");
// 掩码: 0x10ffff 末段标记 → 0xffff
if (!cc.includes("DCHECK_IMPLIES(c > kMaxUInt16, c == String::kMaxCodePoint);")) {
  throw new Error("末段标记 DCHECK 漂移");
}
if (!cc.includes("return c & 0xffff;")) throw new Error("末段标记掩码漂移");

// ── IsCharacterInRangeArray 二分/奇偶规则 (cc:232-275) ───────────────────
if (!cc.includes("static constexpr uint32_t kTrue = 1;")) throw new Error("kTrue=1 漂移");
if (!cc.includes("static constexpr uint32_t kFalse = 0;")) throw new Error("kFalse=0 漂移");
if (!cc.includes("return (ranges_len % 2) == 0 ? kFalse : kTrue;")) throw new Error("末段开区间命中规则漂移");
if (!cc.includes("// Ranges start at even indices and end at odd indices.")) throw new Error("奇偶下标注释漂移");
if (!cc.includes("return (current_range_start_index % 2) == 0 ? kTrue : kFalse;")) throw new Error("奇偶下标判定漂移");
if (!cc.includes("mid = lower + (upper - lower) / 2;")) throw new Error("二分中点公式漂移");

// ── 签名骨架清单 ────────────────────────────────────────────────────────
const defs = [...cc.matchAll(/^(?:void|bool|int|uint32_t|Address|Handle<\w+>|const uint8_t)\s+([\w:]+)::([\w~]+)\(/gm)];
const classNames = new Set(defs.map((d) => d[1]));
if (!classNames.has("RegExpMacroAssembler") || !classNames.has("NativeRegExpMacroAssembler")) {
  throw new Error("两类宿主缺失");
}
// 栈守卫/执行面存在性
for (const sig of [
  "int NativeRegExpMacroAssembler::CheckStackGuardState(",
  "int NativeRegExpMacroAssembler::Execute(",
  "Address NativeRegExpMacroAssembler::GrowStack(Isolate* isolate) {",
]) {
  if (!cc.includes(sig)) throw new Error(`签名缺失: ${sig}`);
}
// CaseInsensitive 比较回调双版本
if (!cc.includes("int RegExpMacroAssembler::CaseInsensitiveCompareNonUnicode(")) throw new Error("非 unicode 比较回调缺失");
if (!cc.includes("int RegExpMacroAssembler::CaseInsensitiveCompareUnicode(")) throw new Error("unicode 比较回调缺失");

console.log(
  `regexp-macro-assembler.cc: 位图 256 字节 (置位 ${setBytes}/位 ${totalBits}), ` +
    `方法定义 ${defs.length}, 宿主类 ${classNames.size} ✓`,
);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const rowStrings = [];
for (let row = 0; row < 16; row++) {
  rowStrings.push(`    ${bytes.slice(row * 16, row * 16 + 16).map((b) => `0x${b.toString(16).padStart(2, "0")}`).join(", ")},`);
}
const body = `${banner(
  "src/regexp/regexp-macro-assembler.cc.js",
  "src/regexp/regexp-macro-assembler.cc",
  "宏汇编公共面基线: 词字符位图 + 边界检查偏移 + 区间数组编码; 平台汇编面待 Developer 就地修订",
)}
/// 词字符位图 (原版 word_character_map_, cc:719-761; 256 字节, Latin-1 全集)
/// 置位即词字符: '0'-'9' / 'A'-'Z' / '_' / 'a'-'z' 共 63 字节 504 位
pub const WORD_CHARACTER_MAP: [u8; 256] = [
${rowStrings.join("\n")}
];

pub const K_WORD_CHARACTER_MAP_SIZE: usize = 256;
/// 置位字节数
pub const K_WORD_CHARACTER_SET_BYTES: usize = ${setBytes};
/// 置位位数
pub const K_WORD_CHARACTER_SET_BITS: usize = ${totalBits};

// ── 边界检查偏移 (CalculateBoundsCheckOffset, cc:323-326) ──
/// cp_offset >= 0: cp_offset + characters - 1; 负: 原值
#[must_use]
pub const fn calculate_bounds_check_offset(cp_offset: i32, characters: i32) -> i32 {
  if cp_offset >= 0 {
    cp_offset + characters - 1
  } else {
    cp_offset
  }
}

// ── 区间数组编码 (MakeRangeArray, cc:191-213) ──
/// 编码: [from0, to0+1, from1, to1+1, ...], to 为开区间端;
/// 末段 to==0xffff 时省尾元素 (长度 2n-1, 防溢出)
#[must_use]
pub const fn range_array_length(ranges_len: usize, last_to_masked: u16) -> usize {
  if last_to_masked == u16::MAX {
    ranges_len * 2 - 1
  } else {
    ranges_len * 2
  }
}
/// 末段标记掩码: 0x10ffff → 0xffff (MaskEndOfRangeMarker, cc:158-163)
#[must_use]
pub const fn mask_end_of_range_marker(c: u32) -> u32 {
  c & 0xffff
}

/// 范围内判定回值 (生成代码调用约定: u32 而非 bool, cc:236-237)
pub const K_IN_RANGE_TRUE: u32 = 1;
pub const K_IN_RANGE_FALSE: u32 = 0;

const _: () = assert!(WORD_CHARACTER_MAP.len() == 256);
const _: () = assert!(K_WORD_CHARACTER_SET_BYTES == 63 && K_WORD_CHARACTER_SET_BITS == 504);
const _: () = assert!(WORD_CHARACTER_MAP[0x30] == 0xff && WORD_CHARACTER_MAP[0x39] == 0xff);
const _: () = assert!(WORD_CHARACTER_MAP[0x40] == 0x00 && WORD_CHARACTER_MAP[0x5f] == 0xff);
const _: () = assert!(WORD_CHARACTER_MAP[0x7a] == 0xff && WORD_CHARACTER_MAP[0x7b] == 0x00);
const _: () = assert!(calculate_bounds_check_offset(0, 1) == 0 && calculate_bounds_check_offset(-3, 1) == -3);
const _: () = assert!(range_array_length(3, u16::MAX) == 5 && range_array_length(3, 0xfffe) == 6);
const _: () = assert!(mask_end_of_range_marker(0x10ffff) == 0xffff);
`;

writeSrc("v8-regexp-compiler", "macro_assembler_cc_generated.rs", body);
console.log("written: v8-regexp-compiler/src/macro_assembler_cc_generated.rs");
