#!/usr/bin/env bun
// tool/src/regexp/gen-regexp-special-case.cc.js — v8/src/regexp/gen-regexp-special-case.cc
//   → v8-regexp-compiler/src/gen_regexp_special_case.cc.rs.skel
// 机械提取: 3 常量 (kSurrogateStart/kSurrogateEnd/kNonBmpStart) + Canonicalize 五步算法锚
//   + PrintSet 输出模板 4 段 + PrintSpecial 4 集合/2 出表 + WriteHeader/main 协议;
// 对数: closeOver/Canonicalize 调用计数; 同源交叉: special-case.h.js 产物
//   v8-regexp-compiler/src/special_case.rs 的 IgnoreSet/SupplementaryEquivalentsSet 两表必在场
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const cc = readV8("src/regexp/gen-regexp-special-case.cc");

let n = 0;
const must = (c, m) => { n++; if (!c) throw new Error(m); };
const cnt = (s, re) => (s.match(re) || []).length;

// ── 常量 (BMP/代理区边界) ─────────────────────────────────────────────────
const consts = [...cc.matchAll(/static const base::uc32 (k\w+) = (0x[0-9a-f]+);/g)].map((m) => [m[1], parseInt(m[2], 16)]);
if (consts.length !== 3) throw new Error(`常量应 3, 实得 ${consts.length}`);
const CV = Object.fromEntries(consts);
if (CV.kSurrogateStart !== 0xd800 || CV.kSurrogateEnd !== 0xdfff || CV.kNonBmpStart !== 0x10000) {
  throw new Error(`常量值漂移: ${JSON.stringify(CV)}`);
}

// ── Canonicalize (ECMA-2020 21.2.2.8.2 step 3) 五步锚 ────────────────────
for (const [a, label] of [
  ["CHECK_LE(ch, 0xffff);", "a. 断言 BMP 码元"],
  ["icu::UnicodeString& u = s.toUpper(icu::Locale::getRoot());", "c/d. root Locale toUpper"],
  ["if (u.length() != 1) {\n    return ch;\n  }", "e. 多码元返回原值"],
  ["if (ch >= 128 && cu < 128) {\n    return ch;\n  }", "g. ch>=128 且 cu<128 返回原值"],
  ["UChar32 cu = u.char32At(0);", "f. 取单码元"],
]) must(cc.includes(a), `Canonicalize ${label} 缺失`);

// ── RemoveSupplementary (Unicode 18 补充面等价类剥离) ──────────────────────
must(cc.includes("constexpr UChar32 kFirstSupplementary = static_cast<UChar32>(kNonBmpStart);"), "kFirstSupplementary 锚缺失");
must(cc.includes("set.remove(kFirstSupplementary, UCHAR_MAX_VALUE);"), "remove [kNonBmpStart, UCHAR_MAX] 缺失");

// ── PrintSet 输出模板 (4 段: Build/Data/访问器/LazyInstance) ───────────────
for (const [a, label] of [
  ['out << "icu::UnicodeSet Build" << name << "() {\\n"', "Build<name>() 函数段"],
  ['out << "struct " << name << "Data {\\n"', "<name>Data 结构段"],
  ['out << "//static\\n"\n      << "const icu::UnicodeSet& CaseFolding::" << name << "() {\\n"', "CaseFolding::<name>() 访问器段"],
  ["LAZY_INSTANCE_INITIALIZER;", "LazyInstance 段"],
  ["set.freeze();", "freeze 收尾"],
]) must(cc.includes(a), `PrintSet ${label} 缺失`);

// ── PrintSpecial 算法锚 + 计数 ────────────────────────────────────────────
const counts = [
  [/closeOver\(USET_SIMPLE_CASE_INSENSITIVE\)/g, 3, "closeOver ×3 (主循环 + 验证循环 + 生成头注释文本)"],
  [/Canonicalize\(/g, 5, "Canonicalize 调用 5 (定义 1 + 主循环 2 + 验证 2)"],
  [/RemoveSupplementary\(/g, 3, "RemoveSupplementary 3 (定义 1 + 调用 2)"],
  [/class_has_matching_canonical_char/g, 3, "匹配同典字符判定 ×3"],
  [/class_has_non_matching_canonical_char/g, 3, "异典字符判定 ×3"],
  [/special_add\.add\(i\)/g, 1, "special_add.add ×1"],
  [/ignore\.add\(i\)/g, 1, "ignore.add ×1"],
  [/PrintSet\(out, "/g, 2, "PrintSet 调用 2 (IgnoreSet/SupplementaryEquivalentsSet)"],
  [/CHECK_EQ\(canonical, Canonicalize\(c2\)\);/g, 1, "验证循环 CHECK_EQ"],
  [/"\[\\\\p\{Lu\}\]"/g, 1, "[\\p{Lu}] 种子集"],
  [/getRangeCount\(\)/g, 4, "getRangeCount ×4 (主循环 current + PrintSet 遍历 + 验证 special_add/current)"],
];
for (const [re, want, label] of counts) {
  must(cnt(cc, re) === want, `${label}: 实得 ${cnt(cc, re)} want ${want}`);
}
// main 协议
must(cc.includes("if (argc != 2) {") && cc.includes("WriteHeader(argv[1]);"), "main(argc!=2 → WriteHeader) 协议缺失");
// 生成的头两表 (顺序: IgnoreSet 先)
must(cc.indexOf('PrintSet(out, "IgnoreSet"') < cc.indexOf('PrintSet(out, "SupplementaryEquivalentsSet"'), "两表输出顺序: IgnoreSet 先");

// ── 同源交叉: 生成的 special-case 落地面两集合类型在场 (骨架态, 数据待本生成器接线) ──
const landed = readFileSync(join(root, "v8-regexp-compiler", "src", "special_case.rs"), "utf8");
for (const t of ["pub struct IgnoreSet;", "pub struct SupplementaryEquivalentsSet;"]) {
  must(landed.includes(t), `special_case.rs 缺生成器产物面 ${t} (同源交叉断链)`);
}
must(landed.includes("gen-regexp-special-case.cc"), "special_case.rs 未登记生成器来源锚");
console.log(`gen-regexp-special-case: 常量 3, 算法锚 15, 计数 ${counts.length} 组, 断言 ${n} 条全过; special_case.rs 两集合面交叉在场 ✓`);

// ── 生成 Rust 骨架 ────────────────────────────────────────────────────────
const body = `${banner(
  "src/regexp/gen-regexp-special-case.cc.js",
  "src/regexp/gen-regexp-special-case.cc",
  "special-case.h 生成器算法台账 (Canonicalize/PrintSpecial/PrintSet); 数据面落地在 special_case.rs",
)}
//! 生成器 = 独立 main (icu 依赖), Rust 侧不复刻运行, 本 skel 登记算法不变式:
//! - BMP 主循环跳过代理区 [kSurrogateStart, kSurrogateEnd], 种子 [\\p{Lu}] 仅验证用
//! - Canonicalize = ECMA-2020 21.2.2.8.2 step 3: root Locale toUpper → 单码元?
//!   → (ch>=128 && cu<128)? ch : cu; BMP 断言 CHECK_LE(ch, 0xffff)
//! - 每码点 closeOver(USET_SIMPLE_CASE_INSENSITIVE) 后剥补充面 (Unicode 18 起
//!   BMP 大小写折叠可含补充码点, 如 U+1DF95 → U+00DF; 非 unicode ignoreCase
//!   按 UTF-16 码元比对, 补充码点永不匹配 BMP 字符 → RemoveSupplementary)
//! - 类内全部同典 → ignore (加备选时可直接跳过); 类内存在异典 → special_add
//! - 验证循环: special_add 每成员的 closeOver - ignore 余集应全同典 (CHECK_EQ);
//!   该不变式是 CaseFolding::CloseOver 可从结果只删 IgnoreSet 的前提

pub const SURROGATE_START: u32 = ${CV.kSurrogateStart};
pub const SURROGATE_END: u32 = ${CV.kSurrogateEnd};
pub const NON_BMP_START: u32 = ${CV.kNonBmpStart};
/// v8: cc RemoveSupplementary kFirstSupplementary
pub const FIRST_SUPPLEMENTARY: u32 = ${CV.kNonBmpStart};

// /// v8: cc Canonicalize — 五步 1:1 (icu toUpper → Rust 侧由 special_case.rs 静态表承接)
// /// v8: cc PrintSpecial — 4 集合: current/special_add/ignore/supplementary_equivalents
// /// v8: cc PrintSet — 4 段模板 (Build/Data/CaseFolding 访问器/LazyInstance);
//   Rust 落地省略 LazyInstance, IGNORE_SET / SUPPLEMENTARY_EQUIVALENTS_SET 为
//   const 表 (同源: tool/src/regexp/special-case.h.js 交叉)
`;
writeSrc("v8-regexp-compiler", "gen_regexp_special_case.cc.rs.skel", body);
console.log("written: v8-regexp-compiler/src/gen_regexp_special_case.cc.rs.skel");
