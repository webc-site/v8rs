// 由 tool/src/regexp/regexp-macro-assembler-tracer.cc.js 从 v8/src/regexp/regexp-macro-assembler-tracer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 宏汇编装饰器日志格式串表 (PrintF 格式 1:1); 装饰器实现面待 Developer 就地修订

/// 逐方法日志格式串 (cc PrintF 首参量, 相邻字面量已拼接; %08x/%d/%s 为插值位)
pub const TRACER_FORMATS: [(&str, &str); 53] = [
  ("Constructor", "RegExpMacroAssembler%s();\n"),
  ("AbortedCodeGeneration", " AbortedCodeGeneration\n"),
  ("Bind", "label[%08x]: (Bind)\n"),
  ("BindJumpTarget", "label[%08x]: (BindJumpTarget)\n"),
  ("AdvanceCurrentPosition", " AdvanceCurrentPosition(by=%d);\n"),
  ("CheckFixedLengthLoop", " CheckFixedLengthLoop(label[%08x]);\n\n"),
  ("PopCurrentPosition", " PopCurrentPosition();\n"),
  ("PushCurrentPosition", " PushCurrentPosition();\n"),
  ("Backtrack", " Backtrack();\n"),
  ("GoTo", " GoTo(label[%08x]);\n\n"),
  ("PushBacktrack", " PushBacktrack(label[%08x]);\n"),
  ("Succeed", " Succeed();%s\n"),
  ("Fail", " Fail();"),
  ("PopRegister", " PopRegister(register=%d);\n"),
  ("PushRegister", " PushRegister(register=%d, %s);\n"),
  ("AdvanceRegister", " AdvanceRegister(register=%d, by=%d);\n"),
  ("SetCurrentPositionFromEnd", " SetCurrentPositionFromEnd(by=%d);\n"),
  ("SetRegister", " SetRegister(register=%d, to=%d);\n"),
  ("WriteCurrentPositionToRegister", " WriteCurrentPositionToRegister(register=%d,cp_offset=%d);\n"),
  ("ClearRegisters", " ClearRegister(from=%d, to=%d);\n"),
  ("ReadCurrentPositionFromRegister", " ReadCurrentPositionFromRegister(register=%d);\n"),
  ("WriteStackPointerToRegister", " WriteStackPointerToRegister(register=%d);\n"),
  ("ReadStackPointerFromRegister", " ReadStackPointerFromRegister(register=%d);\n"),
  ("LoadCurrentCharacterImpl", " LoadCurrentCharacter(cp_offset=%d, label[%08x]%s (%d chars) (bounds check offset %d));\n"),
  ("CheckCharacterLT", " CheckCharacterLT(c=0x%04x%s, label[%08x]);\n"),
  ("CheckCharacterGT", " CheckCharacterGT(c=0x%04x%s, label[%08x]);\n"),
  ("CheckCharacter", " CheckCharacter(c=0x%04x%s, label[%08x]);\n"),
  ("CheckAtStart", " CheckAtStart(cp_offset=%d, label[%08x]);\n"),
  ("CheckNotAtStart", " CheckNotAtStart(cp_offset=%d, label[%08x]);\n"),
  ("CheckNotCharacter", " CheckNotCharacter(c=0x%04x%s, label[%08x]);\n"),
  ("CheckCharacterAfterAnd", " CheckCharacterAfterAnd(c=0x%04x%s, mask=0x%04x, label[%08x]);\n"),
  ("CheckNotCharacterAfterAnd", " CheckNotCharacterAfterAnd(c=0x%04x%s, mask=0x%04x, label[%08x]);\n"),
  ("CheckNotCharacterAfterMinusAnd", " CheckNotCharacterAfterMinusAnd(c=0x%04x, minus=%04x, mask=0x%04x, label[%08x]);\n"),
  ("CheckCharacterInRange", " CheckCharacterInRange(from=0x%04x%s, to=0x%04x%s, label[%08x]);\n"),
  ("CheckCharacterNotInRange", " CheckCharacterNotInRange(from=0x%04x%s, to=%04x%s, label[%08x]);\n"),
  ("CheckCharacterInRangeArray", " CheckCharacterInRangeArray(\n        label[%08x]);\n"),
  ("CheckCharacterNotInRangeArray", " CheckCharacterNotInRangeArray(\n        label[%08x]);\n"),
  ("CheckBitInTable", " CheckBitInTable(label[%08x] "),
  ("SkipUntilBitInTable", "SkipUntilBitInTable(cp_offset=%d, advance_by=%d, bounds_check_offset=%d, on_match=label[%08x], on_no_match=label[%08x]\n  "),
  ("SkipUntilCharAnd", "SkipUntilCharAnd(cp_offset=%d, advance_by=%d, character=0x%04x%s, mask=0x%04x, bounds_check_offset=%d, on_match=label[%08x], on_no_match=label[%08x]\n"),
  ("SkipUntilChar", "SkipUntilChar(cp_offset=%d, advance_by=%d, character=0x%04x%s, bounds_check_offset=%d, on_match=label[%08x], on_no_match=label[%08x]\n"),
  ("SkipUntilCharOrChar", "SkipUntilCharOrChar(cp_offset=%d, advance_by=%d, char1=0x%04x%s, char2=0x%04x%s, bounds_check_offset=%d, on_match=label[%08x], on_no_match=label[%08x]\n"),
  ("SkipUntilGtOrNotBitInTable", "SkipUntilGtOrNotBitInTable(cp_offset=%d, advance_by=%d, character=0x%04x%s, bounds_check_offset=%d, on_match=label[%08x], on_no_match=label[%08x]\n  "),
  ("SkipUntilOneOfMasked", "SkipUntilOneOfMasked(cp_offset=%d, advance_by=%d, both_chars=0x%04x%s, both_mask=0x%04x, max_offset=%d, chars1=0x%04x%s, mask1=0x%04x, chars2=0x%04x%s, mask2=0x%04x, on_match1=label[%08x], on_no_match2=label[%08x], on_failure=label[%08x]\n"),
  ("SkipUntilOneOfMasked3", "SkipUntilOneOfMasked3(bc0_cp_offset=%d, bc0_advance_by=%d, bc1_bounds_check_offset=%d, bc1_on_failure=label[%08x], bc1_cp_offset=%d, bc2_characters=0x%04x%s, bc2_mask=0x%04x, bc3_by=%d, bc4_bounds_check_offset=%d, bc4_cp_offset=%d, bc5_characters=0x%04x%s, bc5_mask=0x%04x, bc5_on_equal=label[%08x], bc6_characters=0x%04x%s, bc6_mask=0x%04x, bc6_on_equal=label[%08x], bc7_characters=0x%04x%s, bc7_mask=0x%04x, fallthrough_jump_target=label[%08x]\n  "),
  ("CheckNotBackReference", " CheckNotBackReference(register=%d, %s, label[%08x]);\n"),
  ("CheckNotBackReferenceIgnoreCase", " CheckNotBackReferenceIgnoreCase(register=%d, %s %s, label[%08x]);\n"),
  ("CheckPosition", " CheckPosition(cp_offset=%d, label[%08x]);\n"),
  ("CheckSpecialClassRanges", " CheckSpecialClassRanges(type='%c', label[%08x])\n"),
  ("IfRegisterLT", " IfRegisterLT(register=%d, number=%d, label[%08x]);\n"),
  ("IfRegisterEqPos", " IfRegisterEqPos(register=%d, label[%08x]);\n"),
  ("IfRegisterGE", " IfRegisterGE(register=%d, number=%d, label[%08x]);\n"),
  ("GetCode", " GetCode('%s', '%s');\n"),
];

// ── 约定常量 ──
/// 标签打印约定: label[指针低 32 位, 8 位十六进制] (cc:33-35 LabelToInt)
pub const K_LABEL_FORMAT: &str = "label[%08x]";
/// 栈检查文案 (PushRegister 的 StackCheckFlag::kCheckStackLimit 分支)
pub const K_CHECK_STACK_LIMIT_TEXT: &str = "check stack limit";
/// 读方向二态 (CheckNotBackReference 系)
pub const K_READ_BACKWARD_TEXT: &str = "backward";
pub const K_READ_FORWARD_TEXT: &str = "forward";
/// unicode 二态 (CheckNotBackReferenceIgnoreCase)
pub const K_UNICODE_TEXT: &str = "unicode";
pub const K_NON_UNICODE_TEXT: &str = "non-unicode";
/// Succeed 全局匹配重启后缀
pub const K_RESTART_SUFFIX: &str = " [restart for global match]";
/// 可打印字符显示区间 (' '..'~', 括号包裹) (cc:170-175)
pub const K_PRINTABLE_MIN: u16 = 0x20;
pub const K_PRINTABLE_MAX: u16 = 0x7e;
/// 可打印包裹括号
pub const K_PRINTABLE_WRAP: [char; 2] = ['(', ')'];

// ── 位图/nibble 表布局 (PrintTables, cc:358-384) ──
/// 位图总尺寸 (kTableSize 断言 128)
pub const K_TABLE_SIZE: usize = 128;
/// nibble 表行数 (kRows = 16 = 128/8)
pub const K_TABLE_ROWS: usize = 16;
/// 每行位数 (i % 32 == 31 折行, 4 行 × 32)
pub const K_BITS_PER_ROW: usize = 32;
/// nibble 表头分隔线
pub const K_NIBBLE_HEADER_RULE: &str = "\n--+----------------";
/// nibble 表体分隔线
pub const K_NIBBLE_BODY_RULE: &str = "  +----------------\n";
/// 位图显示字符 (置位/清位)
pub const K_BITMAP_SET_CHAR: char = 'X';
pub const K_BITMAP_CLEAR_CHAR: char = '.';

const _: () = assert!(TRACER_FORMATS.len() == 53);
const _: () = assert!(K_TABLE_ROWS * 8 == K_TABLE_SIZE);
