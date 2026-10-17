#!/usr/bin/env bun
// v8/src/bigint → v8-bigint 常量表基线生成
//
// 只提取"枚举/常量"类缺口 (手抄 128 项表必错), 语义体由修订阶段就地补。
// 生成:
//   v8-bigint/src/from_string.rs  ← bigint-inl.h 的 kCharValue / kCharBits
//   v8-bigint/src/to_string.rs    ← tostring.cc 的 kMaxBitsPerChar
// 复跑幂等 (同输入同输出); 修订后就地改, 复跑会覆盖修订 — 复跑仅限重生成基线。
import { readV8, writeSrc, banner } from "../../lib.js";

// ---------------------------------------------------------------------------
// 提取工具
// ---------------------------------------------------------------------------

// 抠出 C++ `static constexpr T NAME[] = { ... };` 的初始化项, 逐项转 u8/u32
function extractArray(src, name) {
  const re = new RegExp(`constexpr[^;]*\\b${name}\\[\\]\\s*=\\s*\\{([^}]*)\\}`, "s");
  const m = re.exec(src);
  if (!m) throw new Error(`未找到常量表 ${name}`);
  return m[1]
    .split("\n")
    .map((line) => line.replace(/\/\/.*$/, ""))
    .join("\n")
    .split(",")
    .map((s) => s.trim())
    .filter((s) => s.length > 0)
    .map((s) => Number(s));
}

// u8 表 → Rust 数组文本 (每行 8 项, 对齐原版注释节奏)
function rustU8Table(name, vals, perLine) {
  const rows = [];
  for (let i = 0; i < vals.length; i += perLine) {
    const cells = vals.slice(i, i + perLine).map((v) => String(v).padStart(3));
    rows.push(`  ${cells.join(", ")},`);
  }
  return `pub(crate) const ${name}: [u8; ${vals.length}] = [\n${rows.join("\n")}\n];`;
}

// ---------------------------------------------------------------------------
// from_string.rs ← bigint-inl.h: kCharValue (128 项) / kCharBits (9 项)
// ---------------------------------------------------------------------------

const inl = readV8("src/bigint/bigint-inl.h");
const charValue = extractArray(inl, "kCharValue");
const charBits = extractArray(inl, "kCharBits");
if (charValue.length !== 128) throw new Error(`kCharValue 期望 128 项, 实得 ${charValue.length}`);
if (charBits.length !== 9) throw new Error(`kCharBits 期望 9 项, 实得 ${charBits.length}`);

const fromStringRs = `${banner("src/bigint/gen_consts.js", "src/bigint/bigint-inl.h", "常量表基线; FromStringAccumulator / FromString* 为语义体修订")}
//! 字符串 → BigInt 解析: 累加器 + 各进制算法 (对应原版 fromstring.cc + bigint-inl.h 的解析部分)

// 填体注: digit_mul/div_ceil/Digit/K_DIGIT_BITS 在 crate::bigint; Processor 在
// crate::bigint_internal。填体时按需导入。

// 前 127 个 ASCII 字符的数值, 255 表示"非法"
${rustU8Table("K_CHAR_VALUE", charValue, 8)}

// {2,4,8,16,32} → {1,2,3,4,5} 的紧凑映射 (下标 = radix >> 2)
${rustU8Table("K_CHAR_BITS", charBits, 9)}

// TODO(修订): FromStringAccumulator / Parse / ParsePowerTwo / AddPart
// TODO(修订): Processor::from_string_classic / from_string_large / from_string_base_power_of_two / from_string
`;

writeSrc("v8-bigint", "from_string.rs", fromStringRs);

// ---------------------------------------------------------------------------
// to_string.rs ← tostring.cc: kMaxBitsPerChar (37 项)
// ---------------------------------------------------------------------------

const toStr = readV8("src/bigint/tostring.cc");
const maxBitsPerChar = extractArray(toStr, "kMaxBitsPerChar");
if (maxBitsPerChar.length !== 37) throw new Error(`kMaxBitsPerChar 期望 37 项, 实得 ${maxBitsPerChar.length}`);

const toStringRs = `${banner("src/bigint/gen_consts.js", "src/bigint/tostring.cc", "常量表基线; ToStringFormatter / RecursionLevel 为语义体修订")}
//! BigInt → 字符串: 经典算法 + 快速分治算法 (对应原版 tostring.cc)

// 填体注: count_trailing_zeros 在 crate::util (原版 util.h:22 CountTrailingZeros,
// mul_fft.rs 同源消费); div_ceil/Digit 等在 crate::bigint。填体时按需导入。

// 每字符最大位数表 (值放大 32 倍以提升估算精度)
${rustU8Table("K_MAX_BITS_PER_CHAR", maxBitsPerChar, 9)}

// TODO(修订): ToStringFormatter (Classic / BasePowerOfTwo / Fast) / RecursionLevel
// TODO(修订): Processor::to_string / to_string_impl / ToStringResultLength
`;

writeSrc("v8-bigint", "to_string.rs", toStringRs);

console.log("v8-bigint/src/{from_string,to_string}.rs 常量基线已生成");
