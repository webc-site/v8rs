// 由 tool/src/regexp/regexp-compiler.cc.js 从 v8/src/regexp/regexp-compiler.cc 生成; 修订就地改, 复跑覆盖
// 编译器常量基线 (函数内 static constexpr 逐值锚定); 语义实现面待 Developer 就地修订

/// 阶梯首限 (未命名类 IncrementalValueTracker) (cc:460)
pub const K_FIRST_LIMIT: u32 = 32;

/// 掩码折叠类最小成员数 (TextNode 类分支折叠) (cc:1441)
pub const K_MIN_MEMBERS: i32 = 3;

/// 掩码折叠类最大成员数 (cc:1442)
pub const K_MAX_MEMBERS: i32 = 8;

/// 内联分支生成区间上限 (超出走紧凑序列) (cc:1497)
pub const K_MAX_RANGES_FOR_INLINE_BRANCH_GENERATION: i32 = 16;

/// 大小写无关字母快路 ASCII 上限 (GetCaseIndependentLetters) (cc:1564)
pub const K_MAX_ASCII: u16 = 0x7f;

/// QuickCheck 两字节预载掩码 (cc:2043)
pub const K_TWO_BYTE_MASK: u32 = 0xFFFF;

/// QuickCheck 四字节全掩码 (cc:2044)
pub const K_FOUR_BYTE_MASK: u32 = 0xFFFFFFFF;

/// FirstCharacterSetBuilder 遍历预算 (cc:3004)
pub const K_FIRST_CHAR_SET_BUDGET: i32 = 500;

/// SkipUntilOneOfMasked 双分支前提 (cc:4741)
pub const K_NUM_ALTERNATIVES_2WAY: i32 = 2;

/// 掩码跳跃窗口宽度 (4 字符) (cc:4765)
pub const K_SKIP_CHARS: i32 = 4;

/// 双分支 QuickCheck 预算 (cc:4778)
pub const K_QC_BUDGET: i32 = 1;

/// 双分支 QC 窗口起点 (cc:4782)
pub const K_QC_WINDOW_START: i32 = 0;

/// SkipUntilOneOfMasked3 三路分派前提 (cc:5154)
pub const K_NUM_ALTERNATIVES_3WAY: i32 = 3;

/// 三路合并 QuickCheck 预算 (递归 prefix→Choice) (cc:5198)
pub const K_COMBINED_BUDGET: i32 = 4;

/// 三路每分支 QuickCheck 预算 (cc:5199)
pub const K_ALT_BUDGET: i32 = 1;

/// 掩码值分派最小分支数 (TryEmitMaskedValueDispatch) (cc:5371)
pub const K_MIN_ALTERNATIVES: i32 = 4;

/// 掩码值分派最小分组数 (cc:5372)
pub const K_MIN_GROUPS: i32 = 3;

/// 跳跃表分派最小分组数 (cc:5523)
pub const K_MIN_GROUPS_FOR_TABLE_SWITCH: i32 = 6;

/// 跳跃表最大位宽 (至多 64 槽) (cc:5524)
pub const K_MAX_TABLE_SWITCH_BITS: i32 = 6;

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
