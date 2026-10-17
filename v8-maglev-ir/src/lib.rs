#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub use error::{Error, Result};

mod opcodes;
pub use opcodes::{
  FIRST_BRANCH_CONTROL_NODE_OPCODE, FIRST_CONDITIONAL_CONTROL_NODE_OPCODE,
  FIRST_CONSTANT_NODE_OPCODE, FIRST_CONTROL_NODE_OPCODE, FIRST_CONVERSION_NODE_OPCODE,
  FIRST_GAP_MOVE_NODE_OPCODE, FIRST_NODE_OPCODE, FIRST_OPCODE, FIRST_TERMINAL_CONTROL_NODE_OPCODE,
  FIRST_UNCONDITIONAL_CONTROL_NODE_OPCODE, FIRST_VALUE_NODE_OPCODE, HoleIndex,
  LAST_BRANCH_CONTROL_NODE_OPCODE, LAST_CONDITIONAL_CONTROL_NODE_OPCODE, LAST_CONSTANT_NODE_OPCODE,
  LAST_CONTROL_NODE_OPCODE, LAST_CONVERSION_NODE_OPCODE, LAST_GAP_MOVE_NODE_OPCODE,
  LAST_NODE_OPCODE, LAST_OPCODE, LAST_TERMINAL_CONTROL_NODE_OPCODE,
  LAST_UNCONDITIONAL_CONTROL_NODE_OPCODE, LAST_VALUE_NODE_OPCODE, OPCODE_COUNT, OPCODE_NAMES,
  OPCODES, Opcode, can_be_hole_value, can_be_store_to_non_escaped_object,
  can_trigger_truncation_pass, has_range_field, is_branch_control_node, is_commutative_node,
  is_conditional_control_node, is_constant_node, is_control_node, is_conversion_node,
  is_elements_array_write, is_gap_move_node, is_int32_bitwise_binary_operation_node, is_node,
  is_simple_field_store, is_terminal_control_node, is_truncating_to_int32, is_typed_array_store,
  is_unconditional_control_node, is_value_node, is_zero_cost_node,
  preserves_tagged_keyed_properties,
};

mod op_properties;
pub use op_properties::{
  ATTACHED_DEOPT_INFO_MASK, ATTACHED_DEOPT_INFO_SHIFT, ATTACHED_DEOPT_INFO_WIDTH,
  AttachedDeoptInfo, CAN_ALLOCATE_MASK, CAN_ALLOCATE_SHIFT, CAN_ALLOCATE_WIDTH, CAN_READ_MASK,
  CAN_READ_SHIFT, CAN_READ_WIDTH, CAN_THROW_MASK, CAN_THROW_SHIFT, CAN_THROW_WIDTH, CAN_WRITE_MASK,
  CAN_WRITE_SHIFT, CAN_WRITE_WIDTH, IS_CALL_MASK, IS_CALL_SHIFT, IS_CALL_WIDTH, IS_CONVERSION_MASK,
  IS_CONVERSION_SHIFT, IS_CONVERSION_WIDTH, K_PURE_MASK, K_PURE_VALUE, MAX_INPUTS,
  NEEDS_REGISTER_SNAPSHOT_MASK, NEEDS_REGISTER_SNAPSHOT_SHIFT, NEEDS_REGISTER_SNAPSHOT_WIDTH,
  NODE_INPUT_COUNT_MASK, NODE_INPUT_COUNT_SHIFT, NODE_INPUT_COUNT_WIDTH,
  NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_MASK, NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_SHIFT,
  NODE_NUM_DOUBLE_TEMPORARIES_NEEDED_WIDTH, NODE_NUM_TEMPORARIES_NEEDED_MASK,
  NODE_NUM_TEMPORARIES_NEEDED_SHIFT, NODE_NUM_TEMPORARIES_NEEDED_WIDTH, NODE_OP_PROPERTIES_MASK,
  NODE_OP_PROPERTIES_SHIFT, NODE_OP_PROPERTIES_WIDTH, NODE_OPCODE_MASK, NODE_OPCODE_SHIFT,
  NODE_OPCODE_WIDTH, NODE_RESERVED_FIELDS_MASK, NODE_RESERVED_FIELDS_SHIFT,
  NODE_RESERVED_FIELDS_WIDTH, NOT_IDEMPOTENT_MASK, NOT_IDEMPOTENT_SHIFT, NOT_IDEMPOTENT_WIDTH,
  OpProperties, PROPERTIES_SIZE, PROPERTIES_USED_SIZE, REPR_FLOAT64, REPR_HOLEY_FLOAT64,
  REPR_INT_PTR, REPR_INT32, REPR_NONE, REPR_RAW_PTR, REPR_TAGGED, REPR_UINT32,
  VALUE_REPRESENTATION_MASK, VALUE_REPRESENTATION_SHIFT, VALUE_REPRESENTATION_WIDTH,
  decode_input_count, decode_num_double_temporaries, decode_num_temporaries, decode_opcode,
  decode_properties, encode_node_bitfield, update_properties,
};

mod node_type;
pub use node_type::{
  COMBINED_NODE_TYPE_NAMES, LEAF_NODE_TYPE_NAMES, NEVER_STANDALONE, NODE_TYPE_NAMES,
  NUMBER_OF_LEAF_NODE_TYPES, NodeType, NodeTypeInt, NodeTypeIsVariant, empty_node_type,
  intersect_type, is_empty_node_type, node_type_can_be, node_type_is, node_type_is_any_heap_object,
  node_type_is_big_int, node_type_is_boolean, node_type_is_callable, node_type_is_context,
  node_type_is_for_printing, node_type_is_heap_number, node_type_is_internalized_string,
  node_type_is_js_array, node_type_is_js_data_view, node_type_is_js_function,
  node_type_is_js_generator_object, node_type_is_js_primitive, node_type_is_js_receiver,
  node_type_is_js_receiver_or_null, node_type_is_js_receiver_or_null_or_undefined,
  node_type_is_name, node_type_is_never_standalone, node_type_is_none,
  node_type_is_not_undetectable, node_type_is_null, node_type_is_null_or_undefined,
  node_type_is_number, node_type_is_number_or_boolean, node_type_is_number_or_oddball,
  node_type_is_number_or_undefined, node_type_is_numeric, node_type_is_oddball,
  node_type_is_other_callable, node_type_is_other_heap_object, node_type_is_other_js_receiver,
  node_type_is_other_string, node_type_is_smi, node_type_is_string, node_type_is_string_or_oddball,
  node_type_is_string_or_string_wrapper, node_type_is_string_wrapper, node_type_is_symbol,
  node_type_is_undefined, node_type_is_unknown, node_type_may_be_null_or_undefined, remove_type,
  union_type,
};

mod basic_block;
pub use basic_block::{
  BASIC_BLOCK_TYPE_BITS, BLOCK_FLAG_COUNT, BLOCK_FLAG_DEFERRED, BLOCK_FLAG_IS_DEAD,
  BLOCK_FLAG_IS_START_BLOCK_OF_SWITCH_CASE, BLOCK_FLAG_NAMES, BasicBlockType,
};
