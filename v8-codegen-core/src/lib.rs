#![cfg_attr(docsrs, feature(doc_cfg))]

use v8_base_platform as _;
use v8_objects_core as _;

mod aligned_slot;
mod assembler_base;
mod code_comments;
mod code_desc;
mod code_factory;
mod code_reference;
mod cpu_features;
mod error;
mod external_reference;
mod flush_instruction_cache;
mod handler_table;
mod interface_descriptors;
mod jump_table_info;
mod label;
mod machine_type;
mod register_config;
mod reloc_info;
mod safepoint_table;
mod signature;
mod source_position_table;
mod tick_counter;

pub use aligned_slot::{AlignedSlotAllocator, INVALID_SLOT, SLOT_SIZE};
pub use assembler_base::{
  AssemblerBase, AssemblerBuffer, AssemblerOptions, BufferGrowthStrategy, BuiltinCallJumpMode,
  K_DEFAULT_BUFFER_SIZE, K_MAXIMAL_BUFFER_SIZE, K_MINIMAL_BUFFER_SIZE, K_ONE_MB,
  compute_new_buffer_size,
};
pub use code_comments::*;
pub use code_desc::*;
pub use code_factory::*;
pub use code_reference::*;
pub use cpu_features::{CpuFeatureArm64, CpuFeatureX64, CpuFeatures, K_MAX_PROBED_CPU_FEATURES};
pub use error::{Error, Result};
pub use external_reference::{ExternalReference, ExternalReferenceType};
pub use flush_instruction_cache::flush_instruction_cache;
pub use handler_table::*;
pub use interface_descriptors::{
  CallInterfaceDescriptor, CallInterfaceDescriptorData, CallInterfaceDescriptorFlags,
  K_JS_BUILTIN_REGISTER_PARAMS, K_MAX_BUILTIN_REGISTER_PARAMS, K_MAX_TFS_BUILTIN_REGISTER_PARAMS,
  RegistersInit, StackArgumentOrder,
};
pub use jump_table_info::*;
pub use label::{Distance, Label};
pub use machine_type::*;
pub use register_config::*;
pub use reloc_info::{
  K_ALL_MODES_MASK, K_CODE_TARGET_TAG, K_DEFAULT_TAG, K_EMBEDDED_OBJECT_TAG, K_LONG_TAG_BITS,
  K_MAX_SMALL_PC_DELTA, K_SMALL_PC_DELTA_BITS, K_SMALL_PC_DELTA_MASK, K_TAG_BITS, K_TAG_MASK,
  K_WASM_STUB_CALL_TAG, RelocInfo, RelocInfoWriter, RelocIterator, RelocMode, all_real_modes_mask,
  apply_mask, code_target_mask, embedded_object_mode_mask, js_dispatch_handle_mode_mask, mode_mask,
  post_codegen_relocation_mask,
};
pub use safepoint_table::{
  BitVector, GrowableBitVector, K_CONFIG_OFFSET, K_HEADER_SIZE, K_IMPLICIT_START_DEOPT_INDEX,
  K_IMPLICIT_START_PC, K_IMPLICIT_START_TRAMPOLINE, K_MIN_SKIP_BITS, K_NO_DEOPT_INDEX,
  K_NO_TRAMPOLINE_PC, K_STACK_SLOTS_OFFSET, SafepointEntry, SafepointTable, SafepointTableBuilder,
  compare_and_create_xor_patch, decode_safepoint_entry, encode_safepoint_entry, read_uleb32,
  write_uleb32,
};
pub use signature::*;
pub use source_position_table::{
  K_FUNCTION_ENTRY_BYTECODE_OFFSET, PositionTableEntry, RecordingMode, SourcePositionTableBuilder,
  SourcePositionTableIterator,
};
pub use tick_counter::TickCounter;
