#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod block_coverage_builder;
pub mod bytecode_array_builder;
pub mod bytecode_array_iterator;
pub mod bytecode_array_random_iterator;
pub mod bytecode_array_writer;
pub mod bytecode_decoder;
pub mod bytecode_dispatcher;
pub mod bytecode_flags_and_tokens;
pub mod bytecode_generator;
pub mod bytecode_jump_table;
pub mod bytecode_label;
pub mod bytecode_node;
pub mod bytecode_operands;
pub mod bytecode_register;
pub mod bytecode_register_allocator;
pub mod bytecode_register_optimizer;
pub mod bytecode_source_info;
pub mod bytecode_traits;
pub mod bytecodes;
pub mod constant_array_builder;
pub mod control_flow_builders;
pub mod error;
pub mod handler_table_builder;
pub mod interpreter_assembler;
pub mod interpreter_core;
pub mod interpreter_generator;
pub mod interpreter_generator_tsa;
pub mod interpreter_intrinsics;
pub mod interpreter_intrinsics_generator;
pub mod prototype_assignment_sequence_builder;
pub mod regexp;

pub use bytecode_array_builder::{
  BytecodeArray, BytecodeArrayBuilder, ConstantPoolEntry, InterpFunction, NilValue, ToBooleanMode,
};
pub use bytecode_array_iterator::BytecodeArrayIterator;
pub use bytecode_array_writer::*;
pub use bytecode_decoder::*;
pub use bytecode_dispatcher::{
  ACCESSOR_FLAG_KEY, AT_KEY, BytecodeDispatcher, CHAR_AT_KEY, CHAR_CODE_AT_KEY, CODE_POINT_AT_KEY,
  CONCAT_KEY, CONSTRUCTOR_KEY, COPY_WITHIN_KEY, Closure, ENDS_WITH_KEY, EVERY_KEY, EXEC_KEY,
  FILL_KEY, FILTER_KEY, FIND_INDEX_KEY, FIND_KEY, FLAGS_KEY, FLAT_KEY, FLAT_MAP_KEY, FOR_EACH_KEY,
  FROM_CODE_POINT_KEY, GETTER_PREFIX, GLOBAL_KEY, GlobalsTable, HAS_OWN_PROPERTY_KEY, INCLUDES_KEY,
  INDEX_KEY, INDEX_OF_KEY, INPUT_KEY, JOIN_KEY, LAST_INDEX_KEY, LAST_INDEX_OF_KEY, LENGTH_KEY,
  MAP_KEY, MATCH_KEY, MESSAGE_KEY, NAME_KEY, NativeFn, PAD_END_KEY, PAD_START_KEY, POP_KEY,
  PROPERTY_IS_ENUMERABLE_KEY, PROTO_KEY, PROTOTYPE_KEY, PUSH_KEY, REDUCE_KEY, REDUCE_RIGHT_KEY,
  REPEAT_KEY, REPLACE_ALL_KEY, REPLACE_KEY, REVERSE_KEY, SETTER_PREFIX, SHIFT_KEY, SLICE_KEY,
  SOME_KEY, SORT_KEY, SOURCE_KEY, SPLICE_KEY, SPLIT_KEY, STARTS_WITH_KEY, SUBSTR_KEY,
  SUBSTRING_KEY, SYM_TO_PRIMITIVE, SYMBOL_PREFIX, SliceKind, TEST_KEY, TIME_KEY, TIMESTAMP_KEY,
  TO_EXPONENTIAL_KEY, TO_FIXED_KEY, TO_LOWER_CASE_KEY, TO_PRECISION_KEY, TO_STRING_KEY,
  TO_UPPER_CASE_KEY, TRIM_END_KEY, TRIM_KEY, TRIM_LEFT_KEY, TRIM_RIGHT_KEY, TRIM_START_KEY,
  TrimKind, UNSHIFT_KEY, VALUE_OF_KEY, Value, canonical_index, char_count, char_to_byte_index,
  enumerable_name, enumerable_names, execute_bytecode, fits_smi, format_getter_key,
  format_setter_key, format_symbol_key, is_internal_key, is_regexp_readonly_key, same_value_x64,
  slice_window, str_at, str_code_point_at, str_ends_with, str_includes, str_last_index_of, str_pad,
  str_repeat, str_replace_all, str_starts_with, str_trim, to_index_i64, uri_decode, uri_encode,
};
pub use bytecode_flags_and_tokens::*;
pub use bytecode_generator::{
  AccumulatorPreservingMode, BytecodeGenerator, TestFallthrough, TypeHint,
};
pub use bytecode_jump_table::{
  BYTECODE_JUMP_TABLE_DECLARATIONS, INVALID_INDEX, INVALID_OFFSET, constant_pool_entry_for,
};
pub use bytecode_label::*;
pub use bytecode_node::BytecodeNode;
pub use bytecode_operands::*;
pub use bytecode_register::{Register, RegisterList};
pub use bytecode_source_info::BytecodeSourceInfo;
pub use bytecodes::{BYTECODE_COUNT, Bytecode, MAX_OPERANDS, SHORT_STAR_COUNT};
pub use error::{Error, Result};
pub use regexp::{RegExpCore, regex_split_parts};
pub use v8_common::{LanguageMode, LookupHoistingMode};
