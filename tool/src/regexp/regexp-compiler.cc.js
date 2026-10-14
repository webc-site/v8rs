#!/usr/bin/env bun
// tool/src/regexp/regexp-compiler.cc.js — v8/src/regexp/regexp-compiler.cc
//   → v8-regexp-compiler/src/compiler_consts_generated.rs
// 机械提取: 函数体内 static constexpr 常量面 (带行号与归属函数) +
//   ChoiceNode 掩码分派/跳跃表阈值 + QuickCheck 掩码 + 内联分支生成上限;
// 对数断言: 常量 19 处逐值锚定, BoyerMoore 表尺寸引用 (kTableSize 128) 交叉。
import { banner, readV8, writeSrc } from "../../lib.js";

const cc = readV8("src/regexp/regexp-compiler.cc");
const masmH = readV8("src/regexp/regexp-macro-assembler.h");

// ── 逐常量锚 (名称/值/值型/归属行/Rust 名; 同名不同值加后缀消歧) ────────
// [行号, 名称, 期望值, Rust 类型, 说明, Rust 名]
const CONSTS = [
  [460, "kFirstLimit", "32", "u32", "阶梯首限 (未命名类 IncrementalValueTracker)", "K_FIRST_LIMIT"],
  [1441, "kMinMembers", "3", "i32", "掩码折叠类最小成员数 (TextNode 类分支折叠)", "K_MIN_MEMBERS"],
  [1442, "kMaxMembers", "8", "i32", "掩码折叠类最大成员数", "K_MAX_MEMBERS"],
  [1497, "kMaxRangesForInlineBranchGeneration", "16", "i32", "内联分支生成区间上限 (超出走紧凑序列)", "K_MAX_RANGES_FOR_INLINE_BRANCH_GENERATION"],
  [1564, "kMaxAscii", "0x7f", "u16", "大小写无关字母快路 ASCII 上限 (GetCaseIndependentLetters)", "K_MAX_ASCII"],
  [2043, "kTwoByteMask", "0xFFFF", "u32", "QuickCheck 两字节预载掩码", "K_TWO_BYTE_MASK"],
  [2044, "kFourByteMask", "0xFFFFFFFF", "u32", "QuickCheck 四字节全掩码", "K_FOUR_BYTE_MASK"],
  [3004, "kBudget", "500", "i32", "FirstCharacterSetBuilder 遍历预算", "K_FIRST_CHAR_SET_BUDGET"],
  [4741, "kNumAlternatives", "2", "i32", "SkipUntilOneOfMasked 双分支前提", "K_NUM_ALTERNATIVES_2WAY"],
  [4765, "kSkipChars", "4", "i32", "掩码跳跃窗口宽度 (4 字符)", "K_SKIP_CHARS"],
  [4778, "kQCBudget", "1", "i32", "双分支 QuickCheck 预算", "K_QC_BUDGET"],
  [4782, "kQCWindowStart", "0", "i32", "双分支 QC 窗口起点", "K_QC_WINDOW_START"],
  [5154, "kNumAlternatives", "3", "i32", "SkipUntilOneOfMasked3 三路分派前提", "K_NUM_ALTERNATIVES_3WAY"],
  [5175, "kSkipChars", "4", "i32", "三路掩码跳跃窗口宽度 (同 2 路, 值一致)", null],
  [5198, "kCombinedBudget", "4", "i32", "三路合并 QuickCheck 预算 (递归 prefix→Choice)", "K_COMBINED_BUDGET"],
  [5199, "kAltBudget", "1", "i32", "三路每分支 QuickCheck 预算", "K_ALT_BUDGET"],
  [5371, "kMinAlternatives", "4", "i32", "掩码值分派最小分支数 (TryEmitMaskedValueDispatch)", "K_MIN_ALTERNATIVES"],
  [5372, "kMinGroups", "3", "i32", "掩码值分派最小分组数", "K_MIN_GROUPS"],
  [5523, "kMinGroupsForTableSwitch", "6", "i32", "跳跃表分派最小分组数", "K_MIN_GROUPS_FOR_TABLE_SWITCH"],
  [5524, "kMaxTableSwitchBits", "6", "i32", "跳跃表最大位宽 (至多 64 槽)", "K_MAX_TABLE_SWITCH_BITS"],
];

const found = [];
for (const [line, name, expectVal, , desc, rustName] of CONSTS) {
  const ln = cc.split("\n")[line - 1];
  if (ln === undefined) throw new Error(`regexp-compiler.cc:${line} 不存在`);
  const m = ln.match(new RegExp(`(static constexpr|static const) [\\w:]+ ${name} = ([^;]+);`));
  if (!m) throw new Error(`regexp-compiler.cc:${line} 常量 ${name} 形态漂移: ${ln.trim()}`);
  // 值求值: 十六进制/十进制/字符
  const got = m[2].trim();
  const evalNum = (s) => (s.startsWith("0x") || s.startsWith("0X") ? parseInt(s, 16) : Number(s));
  if (evalNum(got) !== evalNum(expectVal)) {
    throw new Error(`regexp-compiler.cc:${line} ${name} 值漂移: got ${got} want ${expectVal}`);
  }
  found.push({ line, name, value: expectVal, desc, rustName: rustName ?? null });
}
if (found.length !== CONSTS.length) throw new Error("常量对数不齐");

// ── 交叉: kTableSize/kTableMask/kTableSizeBits 来自 macro-assembler.h ────
for (const [sym, want] of [
  ["kTableSizeBits = 7", 7],
  ["kTableSize = 1 << kTableSizeBits", 128],
  ["kTableMask = kTableSize - 1", 127],
]) {
  if (!masmH.includes(sym)) throw new Error(`macro-assembler.h 缺 ${sym}`);
}
// cc 内引用 kTableSize 的 BoyerMoore 位图面 (1126/1202/3850/4915)
const tableSizeRefs = [...cc.matchAll(/RegExpMacroAssembler::kTableSize/g)];
if (tableSizeRefs.length < 3) throw new Error(`kTableSize 引用应 ≥3, 实数 ${tableSizeRefs.length}`);
// kBitsPerByte 面 (4916)
if (!cc.includes("static constexpr int kBitsPerByte = 8;")) throw new Error("kBitsPerByte 漂移");
// 掩码分派成员位图 uint64 (BoyerMoorePositionInfo Bitset)
if (!cc.includes("constexpr BoyerMoorePositionInfo::Bitset mask(~uint64_t{0});")) {
  throw new Error("BoyerMoore 全 1 位图漂移");
}

// ── 函数签名骨架面: 顶层方法定义计数 ────────────────────────────────────
const methodDefs = [...cc.matchAll(/^[A-Za-z][\w:<>,\s\*&]*?[\w>]::([\w~]+)\(/gm)];
const classCount = new Set([...cc.matchAll(/^class (\w+)[^;{]*\{/gm)].map((r) => r[1]));
console.log(
  `regexp-compiler.cc: 常量 ${found.length} 逐值锚定, kTableSize 引用 ${tableSizeRefs.length}, ` +
    `方法定义 ${methodDefs.length}, 类 ${classCount.size} ✓`,
);

// ── 生成 Rust ──────────────────────────────────────────────────────────────
const rustConst = found
  .filter((f) => f.rustName)
  .map((f) => {
    const ty = CONSTS.find((c) => c[5] === f.rustName)[3];
    const isHex = f.value.startsWith("0x");
    return `/// ${f.desc} (cc:${f.line})
pub const ${f.rustName}: ${ty} = ${isHex ? f.value : Number(f.value)};`;
  })
  .join("\n\n");

const body = `${banner(
  "src/regexp/regexp-compiler.cc.js",
  "src/regexp/regexp-compiler.cc",
  "编译器常量基线 (函数内 static constexpr 逐值锚定); 语义实现面待 Developer 就地修订",
)}
${rustConst}

// ── 交叉引用面 (regexp-macro-assembler.h) ──
/// 位图表位宽 (kTableSizeBits = 7)
pub const K_TABLE_SIZE_BITS: u32 = 7;
/// 位图表尺寸 (kTableSize = 128)
pub const K_TABLE_SIZE: u32 = 128;
/// 位图表掩码 (kTableMask = 127)
pub const K_TABLE_MASK: u32 = 127;
/// 每字节数 (cc:4916)
pub const K_BITS_PER_BYTE: i32 = 8;
/// BoyerMoore 全 1 位图 (cc:3767, uint64)
pub const K_BOYER_MOORE_FULL_MASK: u64 = u64::MAX;

// ── 掩码折叠阈值组 (cc:1441-1457 整组语义) ──
/// 折叠条件: 成员数 ∈ [K_MIN_MEMBERS, K_MAX_MEMBERS] 且 diff 位计数 ≤ 4
pub const K_MAX_FOLD_DIFF_BITS: u32 = 4;

const _: () = assert!(K_FIRST_LIMIT == 32);
const _: () = assert!(K_MIN_MEMBERS == 3 && K_MAX_MEMBERS == 8);
const _: () = assert!(K_MAX_RANGES_FOR_INLINE_BRANCH_GENERATION == 16);
const _: () = assert!(K_TWO_BYTE_MASK == 0xFFFF && K_FOUR_BYTE_MASK == 0xFFFFFFFF);
const _: () = assert!(K_FIRST_CHAR_SET_BUDGET == 500);
const _: () = assert!(K_NUM_ALTERNATIVES_2WAY == 2 && K_NUM_ALTERNATIVES_3WAY == 3);
const _: () = assert!(K_SKIP_CHARS == 4 && K_QC_BUDGET == 1 && K_QC_WINDOW_START == 0);
const _: () = assert!(K_COMBINED_BUDGET == 4 && K_ALT_BUDGET == 1);
const _: () = assert!(K_MIN_ALTERNATIVES == 4 && K_MIN_GROUPS == 3);
const _: () = assert!(K_MIN_GROUPS_FOR_TABLE_SWITCH == 6 && K_MAX_TABLE_SWITCH_BITS == 6);
const _: () = assert!(K_TABLE_SIZE == 128 && K_TABLE_MASK == 127 && K_TABLE_SIZE_BITS == 7);
`;

writeSrc("v8-regexp-compiler", "compiler_consts_generated.rs", body);
console.log("written: v8-regexp-compiler/src/compiler_consts_generated.rs");
