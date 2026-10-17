#!/usr/bin/env bun
// v8/src/numbers/conversions.{cc,h} → v8-numbers/src/conversions/tables.rs
// 提取 kRadix100 两张 200 项字符表 + conversions.h 的 FP64/FP16/FP32/格式常量族
// 数值均为脚本机械提取/表达式直出, 禁手抄; 复跑覆盖, 修订就地改
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-numbers";

// ---------------------------------------------------------------------------
// 1. kRadix100Table / kRadix100HeadTable (conversions.cc 47-91)
// ---------------------------------------------------------------------------

const cc = readV8("src/numbers/conversions.cc");

// 提取 char 数组初始化器内的字符字面量序列 → { rust 字面量, 字符码 } 对
function extractCharTable(name) {
  const re = new RegExp(
    `static constexpr char ${name}\\[200\\] = \\{([\\s\\S]*?)\\};`,
  );
  const m = cc.match(re);
  if (!m) throw new Error(`${name} 未找到`);
  const lits = m[1].match(/'(?:\\.|[^'])'/g) ?? [];
  if (lits.length !== 200) {
    throw new Error(`${name} 条目数 ${lits.length} != 200`);
  }
  return lits.map((l) => {
    if (l === "'\\0'") return { rust: "0u8", code: 0 };
    const ch = l[1]; // 表内仅数字字符, 无转义歧义
    return { rust: `b'${ch}'`, code: ch.charCodeAt(0) };
  });
}

function rustTable(name, rustName, items, note) {
  // 生成 byte-str 字面量: 数字字符直书, \0 → \x00 转义
  const chars = items.map((x) => (x.code === 0 ? "\\x00" : String.fromCharCode(x.code)));
  const rows = [];
  for (let i = 0; i < chars.length; i += 40) {
    rows.push(chars.slice(i, i + 40).join(""));
  }
  const lit = rows.join("\\\n");
  return `/// ${name} — ${note}\npub const ${rustName}: [u8; 200] = *b"${lit}";\n`;
}

// 表内规则自检: RADIX_100 每组两字符 == 两位十进制;
// HEAD 表 n=0 为哨兵 (\0,\0) (ConvertHeadDigits 永不消费), 1<=n<10 为 (数字,0) 1 位,
// n>=10 为 (十位, 个位) 2 位
function assertTables(t100, tHead) {
  for (let n = 0; n < 100; n++) {
    const d0 = String(Math.floor(n / 10)).charCodeAt(0);
    const d1 = String(n % 10).charCodeAt(0);
    if (t100[2 * n].code !== d0 || t100[2 * n + 1].code !== d1) {
      throw new Error(`kRadix100Table[${n}] 内容不符`);
    }
    const headFirst = n === 0 ? 0 : n >= 10 ? d0 : d1;
    const headSecond = n >= 10 ? d1 : 0;
    if (tHead[2 * n].code !== headFirst || tHead[2 * n + 1].code !== headSecond) {
      throw new Error(`kRadix100HeadTable[${n}] 内容不符`);
    }
  }
}

const t100 = extractCharTable("kRadix100Table");
const tHead = extractCharTable("kRadix100HeadTable");
assertTables(t100, tHead);

// ---------------------------------------------------------------------------
// 2. conversions.h 20-87 的常量族 (FP64 位段 / FP16 阈值 / FP32 / 格式容量)
// ---------------------------------------------------------------------------

const h = readV8("src/numbers/conversions.h");

// constexpr <type> <name> = <expr>; 提取 (type, name, expr)
const consts = [];
const cre = /constexpr (int|uint16_t|uint32_t|uint64_t) (\w+) =([\s\S]*?);/g;
let m;
while ((m = cre.exec(h))) {
  consts.push({ type: m[1], name: m[2], expr: m[3].trim() });
}
if (consts.length !== 25) {
  throw new Error(`conversions.h 常量数 ${consts.length} != 25`);
}

// C++ 表达式 → Rust 表达式 (uint64_t{N} → Nu64; 类型映射)
// PATCH: C++ 无符号回绕/整型提升在 Rust const 下需等价显式改写 (数值不变)
const EXPR_PATCHES = [
  // uint64_t{15} - kFP64ExponentBias 依赖无符号回绕 (原版注释: 溢出位被截断)
  [/15u64 - FP64_EXPONENT_BIAS/g, "15u64.wrapping_sub(FP64_EXPONENT_BIAS)"],
  // u64 + i32 混型: 移位宽度差转 u64
  [
    /FP16_MIN_EXPONENT \+ \(FP64_MANTISSA_BITS - FP16_MANTISSA_BITS\)/g,
    "FP16_MIN_EXPONENT + ((FP64_MANTISSA_BITS - FP16_MANTISSA_BITS) as u64)",
  ],
];
const TY = { int: "i32", uint16_t: "u16", uint32_t: "u32", uint64_t: "u64" };
// Rust 消费面为数组维/len 比较 (usize 域) 的缓冲容量常量: 原版 int 升级 usize
const TY_OVERRIDES = { kDoubleToStringMinBufferSize: "usize", kDoubleToRadixMaxChars: "usize" };
function toRustExpr(expr) {
  let out = expr
    .replace(/uint64_t\{(\d+)\}/g, "$1u64")
    .replace(/\buint64_t\(/g, "(")
    .replace(/\s+/g, " ");
  for (const [re, to] of EXPR_PATCHES) out = out.replace(re, to);
  return out;
}

// 常量名 → 中文注释 (脚本内置映射, 与原版注释语义一致)
const NOTES = {
  kFP64ExponentBits: "FP64 指数位宽",
  kFP64MantissaBits: "FP64 尾数位宽",
  kFP64ExponentBias: "FP64 指数偏置",
  kFP64SignMask: "FP64 符号位掩码",
  kFP64Infinity: "FP64 无穷位型",
  kFP16InfinityAndNaNInfimum: "FP64 中 FP16 无穷/NaN 的下界位型",
  kFP16MinExponent: "FP64 中 FP16 最小指数 (偏置后)",
  kFP16DenormalThreshold: "FP64 中 FP16 非规格化阈值位型",
  kFP16MantissaBits: "FP16 尾数位宽",
  kFP16qNaN: "FP16 quiet NaN 位型",
  kFP16Infinity: "FP16 无穷位型",
  kFP64To16RoundingAddend: "double→FP16 舍入加数 (低 41 位尾数非零时置位第 11 位)",
  kFP64To16RebiasExponentAndRound: "double→FP16 指数重偏置 + 就近舍入",
  kFP64To16DenormalMagic: "double→FP16 非规格化对齐魔数 (依赖舍入到偶)",
  kFP32WithoutSignMask: "FP32 去符号掩码",
  kFP32MinFP16ZeroRepresentable: "FP32 中 FP16 可表示零的最小位型",
  kFP32MaxFP16Representable: "FP32 中 FP16 最大可表示位型",
  kFP32SubnormalThresholdOfFP16: "FP32 中 FP16 非规格化阈值位型",
  kMaxFractionDigits: "toFixed/toPrecision/toExponential 的位数上限",
  kDoubleToStringMinBufferSize: "DoubleToStringView 结果缓冲最小容量 (原版 int; Rust 消费面为数组维/len 比较, usize 域)",
  kDoubleToFixedMaxDigitsBeforePoint: "toFixed 定点表示的整数部分最大位数",
  kDoubleToFixedMaxChars: "toFixed 结果缓冲容量",
  kDoubleToPrecisionMaxChars: "toPrecision 结果缓冲容量",
  kDoubleToExponentialMaxChars: "toExponential 结果缓冲容量",
  kDoubleToRadixMaxChars: "任意进制转换缓冲容量 (原版 int; 同 kDoubleToStringMinBufferSize, usize 域)",
};

// camelCase → SCREAMING_SNAKE_CASE (kFP64SignMask → K_FP64_SIGN_MASK)
// "NaN" 视作不可拆词 (占位保护, 否则切成 NA_N)
function screaming(name) {
  return name
    .replace(/^k/, "")
    .replace(/NaN/g, "_NaN_")
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
    .replace(/_+/g, "_")
    .toUpperCase()
    .replace(/NA_N/g, "NAN")
    .replace(/^_+|_+$/g, "");
}

// 表达式内的 C++ 常量标识符一并换名
function renameExpr(expr) {
  let out = expr;
  for (const { name } of consts) {
    out = out.replaceAll(name, `@@${screaming(name)}@@`);
  }
  return out.replaceAll("@@", "");
}

const constLines = consts
  .map(({ type, name, expr }) => {
    const note = NOTES[name];
    if (!note) throw new Error(`常量 ${name} 缺注释映射`);
    return `${note ? `/// ${note}\n` : ""}pub const ${screaming(name)}: ${TY_OVERRIDES[name] ?? TY[type]} = ${toRustExpr(renameExpr(expr))};`;
  })
  .join("\n\n");

// ---------------------------------------------------------------------------
// 生成
// ---------------------------------------------------------------------------

const body = `${banner("src/numbers/conversions.cc.js", "src/numbers/conversions.cc + conversions.h (表与常量族)")}

//! conversions 的纯数据层: 数字字符查表 + FP64/FP16/FP32 位型常量 + 格式缓冲容量
//! 语义函数见同目录其余模块; 本文件全部由脚本提取, 值不变式见尾部断言

${rustTable("kRadix100Table", "RADIX_100_TABLE", t100, "两位数字 n → 字符对 ('3','5' ⇔ 35), 尾数快速写出的查表")}

${rustTable("kRadix100HeadTable", "RADIX_100_HEAD_TABLE", tHead, "两位数字 n → 头部字符 (1 位数第二字节为 0 哨兵), ConvertHeadDigits 用")}

${constLines}

// —— 提取不变式: 两表各 200 项, RADIX_100 组内两字符即两位十进制 ——

/// 头表第 n 组写入位数: n>=10 为 2, 否则 1 (ConvertHeadDigits 语义)
#[inline]
pub const fn radix_100_head_digit_count(n: u8) -> u8 {
  1 + (n >= 10) as u8
}

const _: () = {
  assert!(RADIX_100_TABLE[0] == b'0' && RADIX_100_TABLE[1] == b'0');
  assert!(RADIX_100_TABLE[198] == b'9' && RADIX_100_TABLE[199] == b'9');
  assert!(RADIX_100_HEAD_TABLE[0] == 0 && RADIX_100_HEAD_TABLE[1] == 0);
  assert!(RADIX_100_HEAD_TABLE[20] == b'1' && RADIX_100_HEAD_TABLE[21] == b'0');
  assert!(FP64_SIGN_MASK == 1u64 << 63);
  assert!(FP64_INFINITY == 2047u64 << 52);
  assert!(DOUBLE_TO_FIXED_MAX_CHARS == 123);
  assert!(DOUBLE_TO_RADIX_MAX_CHARS == 2200);
};
`;

const p = writeSrc(CRATE, "conversions/tables.rs", body);
console.log(`written ${p} (tables 2x200, consts ${consts.length})`);
