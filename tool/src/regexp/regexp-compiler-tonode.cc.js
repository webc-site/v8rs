#!/usr/bin/env bun
// tool/src/regexp/regexp-compiler-tonode.cc.js — v8/src/regexp/regexp-compiler-tonode.cc
//   → v8-regexp-compiler/src/tonode_consts_generated.rs
// 机械提取: UnicodeRangeSplitter 五段表 (kStarts/kEnds, 代理区常量求值) +
//   10 条 static_assert 承接 + UC16Range 打包三函数 + 展开常量;
// 对数断言: 段 5 / 断言 10 / 常量 5 逐值; 与既有 range_split.rs (修订文件,
// 只读) 交叉对段数, 不写它。
import { banner, readV8, writeSrc, root } from "../../lib.js";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const cc = readV8("src/regexp/regexp-compiler-tonode.cc");

// ── 代理区/非 BMP 常量 (tonode.cc 顶部引用, 定义在 compiler.h/全局) ──────
const SYM = {
  kLeadSurrogateStart: 0xd800,
  kLeadSurrogateEnd: 0xdbff,
  kTrailSurrogateStart: 0xdc00,
  kTrailSurrogateEnd: 0xdfff,
  kNonBmpStart: 0x10000,
  kNonBmpEnd: 0x10ffff,
  // AddRange 内的本地别名 (cc:224-227)
  kBmp1Start: 0,
  kBmp1End: 0xd7ff,
  kBmp2Start: 0xe000,
  kBmp2End: 0xffff,
};

// ── 五段表 (cc:224-254) ───────────────────────────────────────────────────
function evalUc32(tok) {
  const t = tok.trim();
  if (/^0[xX][0-9a-fA-F]+$/.test(t)) return parseInt(t, 16);
  if (/^\d+$/.test(t)) return Number(t);
  if (t in SYM) return SYM[t];
  const m = t.match(/^(\w+) - 1$/);
  if (m && m[1] in SYM) return SYM[m[1]] - 1;
  const p = t.match(/^(\w+) \+ 1$/);
  if (p && p[1] in SYM) return SYM[p[1]] + 1;
  throw new Error(`tonode 常量未登记: ${t}`);
}

const addRangeFn = cc.match(/void UnicodeRangeSplitter::AddRange\(CharacterRange range\) \{([\s\S]*?)\n\}/);
if (!addRangeFn) throw new Error("AddRange 未找到");

const startsArr = addRangeFn[1].match(/static constexpr base::uc32 kStarts\[\] = \{([\s\S]*?)\};/);
const endsArr = addRangeFn[1].match(/static constexpr base::uc32 kEnds\[\] = \{([\s\S]*?)\};/);
const targetsArr = addRangeFn[1].match(/CharacterRangeVector\* const kTargets\[\] = \{([\s\S]*?)\};/);
if (!startsArr || !endsArr || !targetsArr) throw new Error("五段表形态漂移");

const splitTop = (s) => s.split(",").map((x) => x.trim()).filter(Boolean);
const starts = splitTop(startsArr[1]).map(evalUc32);
const ends = splitTop(endsArr[1]).map(evalUc32);
const targets = splitTop(targetsArr[1]).map((t) => t.replace(/^&/, ""));
if (starts.length !== 5 || ends.length !== 5 || targets.length !== 5) {
  throw new Error(`五段表应各 5, 实得 ${starts.length}/${ends.length}/${targets.length}`);
}
// 人工锚: 五段值
const WANT = [
  [0, 0xd7ff, "bmp_"],
  [0xd800, 0xdbff, "lead_surrogates_"],
  [0xdc00, 0xdfff, "trail_surrogates_"],
  [0xe000, 0xffff, "bmp_"],
  [0x10000, 0x10ffff, "non_bmp_"],
];
for (let i = 0; i < 5; i++) {
  if (starts[i] !== WANT[i][0] || ends[i] !== WANT[i][1] || targets[i] !== WANT[i][2]) {
    throw new Error(`段 ${i} 漂移: [${starts[i]},${ends[i]}]→${targets[i]}`);
  }
}

// ── 10 条 static_assert 承接 (cc:230-239 + 255-256) ──────────────────────
const saCount = [...addRangeFn[1].matchAll(/static_assert\(/g)].length;
if (saCount < 10) throw new Error(`static_assert 应 ≥10, 实数 ${saCount}`);
for (const a of [
  "kBmp1Start == 0",
  "kBmp1Start < kBmp1End",
  "kBmp1End + 1 == kLeadSurrogateStart",
  "kLeadSurrogateStart < kLeadSurrogateEnd",
  "kLeadSurrogateEnd + 1 == kTrailSurrogateStart",
  "kTrailSurrogateStart < kTrailSurrogateEnd",
  "kTrailSurrogateEnd + 1 == kBmp2Start",
  "kBmp2Start < kBmp2End",
  "kBmp2End + 1 == kNonBmpStart",
  "kNonBmpStart < kNonBmpEnd",
]) {
  if (!addRangeFn[1].includes(a)) throw new Error(`static_assert 缺: ${a}`);
}

// ── UC16Range 打包三函数 (cc:297-306) ────────────────────────────────────
for (const sig of [
  "constexpr UC16Range ToUC16Range(base::uc16 from, base::uc16 to) {",
  "constexpr base::uc16 ExtractFrom(UC16Range r) {",
  "constexpr base::uc16 ExtractTo(UC16Range r) {",
]) {
  if (!cc.includes(sig)) throw new Error(`UC16Range 函数缺: ${sig}`);
}
if (!cc.includes("return (static_cast<uint32_t>(from) << 16) | to;")) throw new Error("打包公式漂移");
if (!cc.includes("return static_cast<base::uc16>(r >> 16);")) throw new Error("解包 from 公式漂移");
if (!cc.includes("return static_cast<base::uc16>(r);")) throw new Error("解包 to 公式漂移");

// ── 其余常量逐值锚 ────────────────────────────────────────────────────────
const CONSTS = [
  [625, "kMaxRangesToInline", 32, "内联区间上限 (ClassRanges 编译, 注释 Arbitrary)"],
  [2198, "kMaxExpansionFactor", 6, "类展开放大因子上限 (ClassRanges→choice 数)"],
  [2262, "kMaxUnrolledMinMatches", 3, "min 展开上限 (Unroll (foo)+ / {3,})"],
  [2263, "kMaxUnrolledMaxMatches", 3, "max 展开上限 (Unroll (foo)? / {x,3})"],
];
for (const [line, name, want] of CONSTS) {
  const ln = cc.split("\n")[line - 1];
  const m = ln.match(new RegExp(`(static constexpr|static const) [\\w:]+ ${name} = (\\d+);`));
  if (!m || Number(m[2]) !== want) throw new Error(`cc:${line} ${name} 漂移: ${ln.trim()}`);
}
// kCrlf 数组 (cc:1292)
{
  const ln = cc.split("\n")[1292 - 1];
  if (!/static constexpr base::uc16 kCrlf\[\] = \{'\\r', '\\n'\};/.test(ln)) {
    throw new Error(`cc:1292 kCrlf 漂移: ${ln.trim()}`);
  }
}

// ── 交叉: 既有 range_split.rs (修订文件, 只读) STARTS/ENDS/TARGETS 全等 ──
const rangeSplit = readFileSync(join(root, "v8-regexp-compiler", "src", "range_split.rs"), "utf8");
const rsStarts = rangeSplit.match(/const STARTS: \[u32; 5\] = \[([\s\S]*?)\];/);
const rsEnds = rangeSplit.match(/const ENDS: \[u32; 5\] = \[([\s\S]*?)\];/);
const rsTargets = rangeSplit.match(/const TARGETS: \[u8; 5\] = \[(.*?)\];/);
if (!rsStarts || !rsEnds || !rsTargets) throw new Error("range_split.rs 段表标识漂移 (STARTS/ENDS/TARGETS)");
// range_split.rs 用符号名, 求值后比对
const RS_SYM = {
  BMP1_START: 0,
  BMP1_END: 0xd7ff,
  BMP2_START: 0xe000,
  BMP2_END: 0xffff,
  K_LEAD_SURROGATE_START: 0xd800,
  K_LEAD_SURROGATE_END: 0xdbff,
  K_TRAIL_SURROGATE_START: 0xdc00,
  K_TRAIL_SURROGATE_END: 0xdfff,
  K_NON_BMP_START: 0x10000,
  K_MAX_CODE_POINT: 0x10ffff,
};
const parseArr = (s) =>
  s.split(",").map((x) => {
    const t = x.trim();
    if (t in RS_SYM) return RS_SYM[t];
    if (/^0x[0-9a-fA-F]+$/.test(t)) return parseInt(t, 16);
    return Number(t);
  });
const rsS = parseArr(rsStarts[1]);
const rsE = parseArr(rsEnds[1]);
for (let i = 0; i < 5; i++) {
  if (rsS[i] !== starts[i] || rsE[i] !== ends[i]) {
    throw new Error(`range_split.rs 段 ${i} 与 C++ 不符: rs [${rsS[i]},${rsE[i]}] vs C++ [${starts[i]},${ends[i]}]`);
  }
}
const TGT_IDX = { bmp_: 0, lead_surrogates_: 1, trail_surrogates_: 2, non_bmp_: 3 };
const rsT = parseArr(rsTargets[1]);
for (let i = 0; i < 5; i++) {
  if (rsT[i] !== TGT_IDX[targets[i]]) throw new Error(`range_split.rs TARGETS[${i}] 与 C++ 归属不符`);
}

console.log(`regexp-compiler-tonode.cc: 五段表 5 (${starts.map((s, i) => `0x${s.toString(16)}-0x${ends[i].toString(16)}`).join(" ")}), static_assert ${saCount}, 常量 ${CONSTS.length} ✓`);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const body = `${banner(
  "src/regexp/regexp-compiler-tonode.cc.js",
  "src/regexp/regexp-compiler-tonode.cc",
  "Tree→Node 转换常量基线: Unicode 五段分裂表 + UC16Range 打包 + 展开阈值; 语义在 range_split.rs, 就地修订",
)}
/// Unicode 区间五段分裂表 (AddRange, cc:224-265): BMP 双段 + 代理区两段 + 非 BMP 段
pub const K_SEGMENT_STARTS: [u32; 5] = [
${starts.map((s) => `  0x${s.toString(16)},`).join("\n")}
];
pub const K_SEGMENT_ENDS: [u32; 5] = [
${ends.map((e) => `  0x${e.toString(16)},`).join("\n")}
];
/// 段归属 (bmp_ / lead_surrogates_ / trail_surrogates_ / non_bmp_; 第 4 段归 bmp_)
pub const K_SEGMENT_TARGETS: [SegmentTarget; 5] = [
${targets.map((t) => `  SegmentTarget::${t === "bmp_" ? "Bmp" : t === "lead_surrogates_" ? "LeadSurrogates" : t === "trail_surrogates_" ? "TrailSurrogates" : "NonBmp"},`).join("\n")}
];

/// 分裂段归属 (对应原版 CharacterRangeVector 成员)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentTarget {
  /// BMP 可单码点表示段 (两段共用)
  Bmp,
  /// 孤立前导代理
  LeadSurrogates,
  /// 孤立尾随代理
  TrailSurrogates,
  /// 非 BMP (代理对)
  NonBmp,
}

/// 原版 10 条 static_assert 的承接 (段边界无缝衔接性)
pub const K_SEGMENT_COUNT: usize = 5;

// ── UC16Range 打包 (cc:297-306: {from,to} 压入单 u32) ──
#[must_use]
pub const fn to_uc16_range(from: u16, to: u16) -> u32 {
  (u32::from(from) << 16) | u32::from(to)
}
#[must_use]
pub const fn extract_from(r: u32) -> u16 {
  (r >> 16) as u16
}
#[must_use]
pub const fn extract_to(r: u32) -> u16 {
  r as u16
}

// ── 展开阈值 ──
/// 类编译内联区间上限 (cc:625)
pub const K_MAX_RANGES_TO_INLINE: usize = 32;
/// 类展开放大因子上限 (cc:2198)
pub const K_MAX_EXPANSION_FACTOR: i32 = 6;
/// min 展开上限 (cc:2262)
pub const K_MAX_UNROLLED_MIN_MATCHES: i32 = 3;
/// max 展开上限 (cc:2263)
pub const K_MAX_UNROLLED_MAX_MATCHES: i32 = 3;
/// CRLF 行终止符对 (cc:1292)
pub const K_CRLF: [u16; 2] = [0x0d, 0x0a];

const _: () = assert!(K_SEGMENT_COUNT == 5);
const _: () = assert!(K_SEGMENT_STARTS[0] == 0 && K_SEGMENT_ENDS[0] == 0xd7ff);
const _: () = assert!(K_SEGMENT_STARTS[1] == 0xd800 && K_SEGMENT_ENDS[1] == 0xdbff);
const _: () = assert!(K_SEGMENT_STARTS[2] == 0xdc00 && K_SEGMENT_ENDS[2] == 0xdfff);
const _: () = assert!(K_SEGMENT_STARTS[3] == 0xe000 && K_SEGMENT_ENDS[3] == 0xffff);
const _: () = assert!(K_SEGMENT_STARTS[4] == 0x10000 && K_SEGMENT_ENDS[4] == 0x10ffff);
const _: () = assert!(K_SEGMENT_STARTS[1] == K_SEGMENT_ENDS[0] + 1);
const _: () = assert!(K_SEGMENT_STARTS[2] == K_SEGMENT_ENDS[1] + 1);
const _: () = assert!(K_SEGMENT_STARTS[3] == K_SEGMENT_ENDS[2] + 1);
const _: () = assert!(K_SEGMENT_STARTS[4] == K_SEGMENT_ENDS[3] + 1);
const _: () = assert!(to_uc16_range(0x1234, 0x5678) == 0x1234_5678);
const _: () = assert!(extract_from(0x1234_5678) == 0x1234 && extract_to(0x1234_5678) == 0x5678);
`;

writeSrc("v8-regexp-compiler", "tonode_consts_generated.rs", body);
console.log("written: v8-regexp-compiler/src/tonode_consts_generated.rs");
