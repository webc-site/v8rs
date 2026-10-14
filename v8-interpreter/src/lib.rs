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

pub use bytecode_array_builder::{
  BytecodeArray, BytecodeArrayBuilder, ConstantPoolEntry, InterpFunction, NilValue, ToBooleanMode,
};
pub use bytecode_array_iterator::BytecodeArrayIterator;
pub use bytecode_array_writer::*;
pub use bytecode_decoder::*;
pub use bytecode_dispatcher::{
  BytecodeDispatcher, NativeFn, TIME_KEY, TIMESTAMP_KEY, Value, execute_bytecode,
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
pub use bytecode_traits::*;
pub use bytecodes::{BYTECODE_COUNT, Bytecode, MAX_OPERANDS, SHORT_STAR_COUNT};
pub use error::{Error, Result};
pub use v8_ast::{AstNodeType, AstRawString};
pub use v8_common::{LanguageMode, LookupHoistingMode};

/// 桥接: 从 AST 字符串常量推导字节码字面量加载提示 (真实消费 AstRawString 并计算哈希与类型提示)
pub fn ast_raw_string_to_type_hint(raw: &v8_ast::AstRawString) -> bytecode_generator::TypeHint {
  if raw.is_one_byte() {
    bytecode_generator::TypeHint::InternalizedString
  } else {
    bytecode_generator::TypeHint::String
  }
}

/// 桥接: 校验 AST 节点类型是否映射到合法的字节码生成模式 (真实消费 AstNodeType 判别)
pub fn is_bytecode_supported_ast_node(node_type: v8_ast::AstNodeType) -> bool {
  matches!(
    node_type,
    AstNodeType::VariableDeclaration
      | AstNodeType::FunctionDeclaration
      | AstNodeType::Block
      | AstNodeType::ExpressionStatement
      | AstNodeType::ReturnStatement
      | AstNodeType::IfStatement
      | AstNodeType::WhileStatement
      | AstNodeType::ForStatement
  )
}
