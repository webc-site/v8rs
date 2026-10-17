#!/usr/bin/env bun
// tool/src/compiler/backend/simd-shuffle.js
// v8/src/compiler/backend/simd-shuffle.{h,cc} → v8-compiler-backend/src/simd_shuffle.rs.skel
// SimdShuffle/SimdSwizzle: shuffle 模式匹配库 (TryMatch 族 + CanonicalShuffle 枚举)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/simd-shuffle.h");
const cc = readV8("src/compiler/backend/simd-shuffle.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 类 + CanonicalShuffle 枚举 (真解析)
must(h.includes("class V8_EXPORT_PRIVATE SimdShuffle {") || /class V8_EXPORT_PRIVATE SimdShuffle/.test(h), "h SimdShuffle");
const canon = h.match(/enum class CanonicalShuffle : uint8_t \{([\s\S]*?)\};/);
if (!canon) throw new Error("未找到 CanonicalShuffle");
const canonical = [...canon[1].matchAll(/\bk(\w+),/g)].map((m) => m[1]);
if (canonical.length !== 53) throw new Error(`CanonicalShuffle ${canonical.length} != 53`);
must(canonical[0] === "Unknown" && canonical[1] === "Identity", "kUnknown/kIdentity 前二");
must(canonical.filter((n) => n.startsWith("S")).length === 51, "S* 规范类 51");
must(canonical.includes("S8x16Even") && canonical.includes("S64x2Reverse"), "枚举成员抽查");

// h: 静态 TryMatch 族 (真解析计数)
const tryMatchH = [...h.matchAll(/static bool TryMatch(\w+)\(/g)].map((m) => m[1]);
if (tryMatchH.length !== 27) throw new Error(`h TryMatch ${tryMatchH.length} != 27`);
// h: Pack 族 4 + CanonicalizeShuffle
for (const p of ["PackShuffle4", "PackBlend8", "PackBlend4", "Pack4Lanes", "Pack16Lanes"])
  must(h.includes(`static ${p === "Pack16Lanes" ? "void" : /\d$/.test(p) || p === "PackBlend8" || p === "PackBlend4" || p === "PackShuffle4" ? "uint8_t" : "int32_t"} ${p}(`) ||
    new RegExp(`static (?:uint8_t|int32_t|void) ${p}\\(`).test(h), `h ${p}`);
must(h.includes("static void CanonicalizeShuffle("), "h CanonicalizeShuffle");
must(h.includes("static CanonicalShuffle TryMatchCanonical("), "h TryMatchCanonical");
// h: ShuffleEntry 模板特化 2 (128/256) + x64 架构表
must(h.includes("struct ShuffleEntry<kSimd128Size> {"), "h ShuffleEntry128");
must(h.includes("struct ShuffleEntry<kSimd256Size> {"), "h ShuffleEntry256");
must(h.includes("arch_shuffles128["), "h arch_shuffles128 表");
must(h.includes("arch_shuffles256[] = {"), "h arch_shuffles256 表");
must(h.includes("TryMatchArchShuffle"), "h TryMatchArchShuffle (x64)");
must(h.includes("#endif  // V8_TARGET_ARCH_X64"), "h x64 专属区界");
// h: SimdSwizzle 辅类
must(h.includes("class V8_EXPORT_PRIVATE SimdSwizzle {"), "h SimdSwizzle");
must(h.includes("static bool AllInRangeOrTopBitSet("), "h AllInRangeOrTopBitSet");

// cc: 哨兵常量 + expand 表 3 + 匿名 helper
must(cc.includes("static constexpr uint8_t kShuffleSentinel = 0xFF;"), "cc kShuffleSentinel=0xFF");
for (const e of ["expand(", "expandHalf(", "expandQuarter("])
  must(cc.includes(`constexpr SimdShuffle::ShuffleArray<kSimd128Size> ${e}`), `cc ${e} 表`);
must(cc.includes("bool MatchHelper(const uint8_t* input, uint8_t* output)"), "cc MatchHelper");
// cc: TryMatch 实现计数 (含匿名 7 辅助)
const tryMatchCc = count(cc, /\bTryMatch\w*\(/g);
if (tryMatchCc < 30) throw new Error(`cc TryMatch 面 ${tryMatchCc} < 30`);
// cc: CanonicalizeShuffle 实现
must(h.includes("static void CanonicalizeShuffle("), "h CanonicalizeShuffle 声明");
must(!cc.includes("SimdShuffle::CanonicalizeShuffle("), "cc 无 CanonicalizeShuffle (模板, 架构侧实例化)");
must(cc.includes("CanonicalShuffle SimdShuffle::TryMatchCanonical("), "cc TryMatchCanonical 实现");

// ── 产物 ────────────────────────────────────────────────────────────────────
const canonVariants = canonical.map((n) => `  /// k${n}\n  ${n.slice(1)},`).join("\n");
const tryList = tryMatchH.map((n) => `/// v8: SimdShuffle::TryMatch${n}\n// pub fn try_match_${n.slice(1).toLowerCase()}(...) -> bool`).join("\n");
const skel = `${banner("src/compiler/backend/simd-shuffle.js",
  "src/compiler/backend/simd-shuffle.h + simd-shuffle.cc",
  "SimdShuffle/SimdSwizzle: shuffle 模式匹配 (TryMatch 29 + Pack 5 + CanonicalShuffle 53)")}
//! 台账: CanonicalizeShuffle (输入等化 + 规范化); TryMatchCanonical → 枚举 50 值;
//!   TryMatch 族 27 + TryMatchCanonical (CanonicalShuffle 返回) + TryMatchArchShuffle
//!   (x64); Pack 族 5 (pshufd imm8/blend 掩码);
//!   x64 专属: arch_shuffles128/256 常表 + TryMatchArchShuffle (swizzle 掩码);
//!   kShuffleSentinel=0xFF; SimdSwizzle::AllInRangeOrTopBitSet。

/// v8: SimdShuffle::CanonicalShuffle — 规范化 shuffle 类 (实测 ${canonical.length} 值)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum CanonicalShuffle {
${canonVariants}
}

/// v8: kShuffleSentinel — 无效通道哨兵
pub const SHUFFLE_SENTINEL: u8 = 0xFF;

${tryList}
/// v8: SimdShuffle::PackShuffle4/PackBlend8/PackBlend4/Pack4Lanes/Pack16Lanes — 打包族
// pub fn pack_shuffle4 / pack_blend8 / pack_blend4 / pack4_lanes / pack16_lanes
/// v8: SimdSwizzle::AllInRangeOrTopBitSet — swizzle 直接量域检查
// pub fn all_in_range_or_top_bit_set(shuffle: [u8; 16]) -> bool

// —— 提取不变式断言 ——
const _: () = {
  assert!(SHUFFLE_SENTINEL == 0xFF);
};
pub const CANONICAL_SHUFFLE_COUNT: usize = ${canonical.length};
const _: () = {
  assert!(CANONICAL_SHUFFLE_COUNT == 53);
};
`;

writeSrc("v8-compiler-backend", "simd_shuffle.rs.skel", skel);
console.log(`simd-shuffle.js: 对数 ${checks} 条全过; CanonicalShuffle ${canonical.length} 值实测 (kUnknown+kIdentity+51 S*); ` +
  `TryMatch 27 + Pack 5; 哨兵 0xFF; SimdSwizzle 1; 产物 simd_shuffle.rs.skel`);
