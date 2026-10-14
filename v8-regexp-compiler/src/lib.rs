//! v8-regexp-compiler: v8/src/regexp 编译器侧纯逻辑面的 1:1 Rust 复刻 (wave1)
//!
//! 依赖白名单: v8-base / v8-common / v8-regexp-syntax (冻结)。
//! 禁止依赖 v8-codegen-core 与 v8-regexp-exec 在飞面 —— 与 exec 的字节码表
//! 同源性由转换脚本 tool/src/regexp/regexp-bytecodes.h.js 生成期交叉对数,
//! 并由 tests 运行时再次读 exec 源文件校验。
//!
//! 推迟面 (二梯队, 待 codegen-core 绿): RegExpAssembler / RegExpMacroAssembler
//! 驱动面全族、Trace::Flush/AdvanceCurrentPositionInTrace 的 emit/abort 通道、
//! Node IR 宿主 (Compiler visitor)、BoyerMoore 与 Parser 的接线。

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod boyer_moore;
mod bytecodes;
mod bytecodes_generated;
mod dump;
mod dynamic_bitset;
mod error;
mod flags;
mod frequency;
mod node_data;
mod quick_check;
mod range_split;

pub use boyer_moore::{
  BoyerMooreLookahead, BoyerMoorePositionInfo, ContainedInLattice, K_MAP_MASK, K_MAP_SIZE,
  K_RANGE_END_MARKER, add_range_lattice, bitset_first_set_bit, combine, word_boundaries,
};
pub use bytecodes::{
  Operand, opcode_at, operand_type_is_basic, operand_type_limits, operand_type_name,
  operand_type_size,
};
pub use bytecodes_generated::{
  ALL_BYTECODES, BYTECODE_NAMES, Bytecode, BytecodeFlag, BytecodeOperandType, K_BYTECODE_COUNT,
  K_MAX_OPERAND_COUNT, OPERAND_TYPE_LIMITS, OPERAND_TYPE_NAMES, OPERAND_TYPE_SIZES,
};
pub use dump::{
  STANDARD_CHARACTER_SET_NAMES, as_hex, disassemble, disassemble_single, encode,
  stack_check_flag_name, walk,
};
pub use dynamic_bitset::{DynamicBitSet, K_FIRST_LIMIT};
pub use error::{Error, Result};
pub use flags::{
  K_MAX_LOOKAHEAD_FOR_BOYER_MOORE, K_PATTERN_TOO_SHORT_FOR_BOYER_MOORE,
  needs_unicode_case_equivalents,
};
pub use frequency::{
  CharacterFrequency, FrequencyCollator, K_TABLE_MASK, K_TABLE_SIZE, K_TABLE_SIZE_BITS,
};
pub use node_data::{
  AtStartField, DeferredActionUndoType, EatsAtLeastInfo, FlushMode, HasAnyActionsField,
  K_MAX_CP_OFFSET, K_MIN_CP_OFFSET, K_NO_STORE, NODE_TYPE_NAMES, NodeInfo, NodeType, ParkedGrant,
  ParkedGrantField, PreloadState, RegisterFlushInfo, Trace, TriBool,
};
pub use quick_check::{
  K_MAX_ONE_BYTE_CHAR_CODE, K_MAX_POSITIONS, K_MAX_UTF16_CODE_UNIT, Position, QuickCheckDetails,
  char_mask, max_code_unit, smear_bits_right,
};
pub use range_split::{
  K_INITIAL_SIZE, UnicodeRangeSplitter, range_contains_latin1_equivalents,
  ranges_contain_latin1_equivalents,
};
