#![cfg_attr(docsrs, feature(doc_cfg))]

//! v8/src/wasm 基础层: 常量/类型系统/特性标志/LEB128/解码器。
//! 映射登记与推迟项见 crate README (readme/zh.md 台账)。

pub mod basic_block_calculator_cc;
pub mod branch_hint_map;
pub mod canonical_types;
pub mod compilation_environment_consts;
pub mod compilation_environment_inl;
pub mod compilation_hints_generation_cc;
pub mod constant_expression;
pub mod constant_expression_interface_cc;
pub mod decoder;
pub mod effect_handler;
pub mod error;
pub mod function_body_decoder;
pub mod function_body_decoder_tables;
pub mod function_compiler_consts;
pub mod inlining_tree_consts;
pub mod jump_table_assembler_consts;
pub mod leb_helper;
pub mod local_decl_encoder;
pub mod module_compiler_consts;
pub mod module_decoder;
pub mod names_provider_consts;
pub mod pgo;
pub mod random_module_generation_consts;
pub mod signature_hashing;
pub mod stacks_consts;
pub mod streaming_decoder_consts;
pub mod string_builder;
pub(crate) mod table_eq;
pub mod value_type;
pub mod value_type_cc;
pub mod wasm_arguments;
pub mod wasm_builtin_list;
pub mod wasm_code_coverage;
pub mod wasm_code_manager_consts;
pub mod wasm_code_pointer_table;
pub mod wasm_constants;
pub mod wasm_debug_consts;
pub mod wasm_disassembler_consts;
pub mod wasm_features;
pub mod wasm_features_cc;
pub mod wasm_init_expr;
pub mod wasm_limits;
pub mod wasm_linkage;
pub mod wasm_module_builder_cc;
pub mod wasm_module_consts;
pub mod wasm_opcodes;
pub mod wasm_serialization_consts;
pub mod wasm_subtyping;
pub mod wasm_tier;
pub mod wasm_value;
pub mod well_known_imports;
