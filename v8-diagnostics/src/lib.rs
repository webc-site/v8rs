//! v8-diagnostics: V8 src/diagnostics 平台无关面 (wave1 零可落子集)。
//!
//! 数据面由 tool/src/diagnostics/*.js 从上游 C++ 快照生成 (*_data.rs), 行为面手工 1:1。

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod basic_block_profiler;
mod compilation_statistics;
mod eh_frame;
mod eh_frame_data;
mod error;
mod gdb_jit;
mod gdb_jit_data;
mod objects_debug_generated;
mod objects_print;
mod objects_print_data;
mod perf_jit;
mod perf_jit_data;

pub use basic_block_profiler::{
  BLOCK_HINT_MARKER, BLOCK_MARKER, BUILTIN_HASH_MARKER, BasicBlockProfiler, BasicBlockProfilerData,
  BlockCallees, BuiltinCallMap, BuiltinCallees, BuiltinsCallGraph,
};
pub use compilation_statistics::{
  AsPrintableStatistics, BasicStats, CompilationStatistics, OrderedStats, PhaseKindMap, PhaseMap,
  PhaseStats, TotalStats, printable_statistics,
};
pub use eh_frame::{EhFrameIterator, EhFrameWriter, decode_sleb128, decode_uleb128};
pub use eh_frame_data::{
  AUGMENTATION_DATA_SIZE, AUGMENTATION_STRING, CIE_IDENTIFIER, CIE_VERSION, DATA_REL, DwarfOpcode,
  EH_FRAME_ARM64, EH_FRAME_HDR_SIZE, EH_FRAME_HDR_VERSION, EH_FRAME_TERMINATOR_SIZE, EH_FRAME_X64,
  EhFrameArchParams, FDE_ENCODING_SPECIFIERS_SIZE, FDE_PAD_ALIGNMENT, FDE_VERSION_SIZE,
  FOLLOW_INITIAL_RULE_MASK, FOLLOW_INITIAL_RULE_MASK_SIZE, FOLLOW_INITIAL_RULE_TAG,
  INITIAL_STATE_OFFSET_IN_CIE, INT32_PLACEHOLDER, INT32_SIZE, LOCATION_MASK, LOCATION_MASK_SIZE,
  LOCATION_TAG, OMIT, PC_REL, PROCEDURE_ADDRESS_OFFSET_IN_FDE, PROCEDURE_SIZE_OFFSET_IN_FDE,
  SAVED_REGISTER_MASK, SAVED_REGISTER_MASK_SIZE, SAVED_REGISTER_TAG, SDATA4, UDATA4,
};
pub use error::{Error, Result};
pub use gdb_jit::{
  ElfImage, ElfSection, GdbJitRegistry, JitCodeEntry, JitDescriptor, MachOImage, MachOSectionDef,
  Writer,
};
pub use gdb_jit_data::{
  ELF_INDEX_ABSOLUTE, ELF_MACHINE_X64, ELF_SHT_STRTAB_INDEX, ELF_TYPE_REL, ELF_VERSION,
  ELF64_IDENT, ELFHEADER_FIELDS, ELFHEADER_SIZE, ELFSECTIONHEADER_FIELDS, ELFSECTIONHEADER_SIZE,
  ElfSectionType, INITIAL_CAPACITY, JIT_DESCRIPTOR_VERSION, JITCODEENTRY_FIELDS, JITCODEENTRY_SIZE,
  JITDESCRIPTOR_FIELDS, JITDESCRIPTOR_SIZE, JitAction, LC_SEGMENT_32, LC_SEGMENT_64,
  MACHO_CPU_SUBTYPE_ALL, MACHO_CPU_TYPE_X86_64, MACHO_FILETYPE_OBJECT, MACHO_MAGIC_64, MACHO_NCMDS,
  MACHO_PROT_ALL, MACHOHEADER_FIELDS, MACHOHEADER_SIZE, MACHOSECTIONHEADER_FIELDS,
  MACHOSECTIONHEADER_SIZE, MACHOSEGMENTCOMMAND_FIELDS, MACHOSEGMENTCOMMAND_SIZE, S_ATTR_COALESCED,
  S_ATTR_DEBUG, S_ATTR_PURE_INSTRUCTIONS, S_ATTR_SOME_INSTRUCTIONS, S_REGULAR, SHF_ALLOC, SHF_EXEC,
  SHF_WRITE,
};
pub use objects_debug_generated::{VERIFY_REGISTRY, VERIFY_REGISTRY_LEN};
pub use objects_print::{
  double_to_integer, is_minus_zero, iterator_helper_state_to_str, iterator_zip_helper_mode_to_str,
  print_double,
};
pub use objects_print_data::{
  ITER_HELPER_STATE_NAMES, ITER_ZIP_HELPER_MODE_NAMES, JsIteratorHelperState,
  JsIteratorZipHelperMode, MAX_SAFE_INTEGER_BITS, MAX_SAFE_INTEGER_F64, MIN_SAFE_INTEGER_F64,
  MINUS_ZERO_STR, NAN_SUFFIX,
};
pub use perf_jit::{
  CodeLoadArgs, SourceLine, code_load_bytes, debug_address_base, debug_info_bytes, header_bytes,
  unwinding_bytes,
};
pub use perf_jit_data::{
  ELF_HEADER_SIZE_32, ELF_HEADER_SIZE_64, ELF_MACH_ARM, ELF_MACH_ARM64, ELF_MACH_IA32,
  ELF_MACH_LOONG64, ELF_MACH_MIPS64, ELF_MACH_PPC64, ELF_MACH_RISCV, ELF_MACH_S390X, ELF_MACH_X64,
  EVENT_CLOSE, EVENT_DEBUGINFO, EVENT_LOAD, EVENT_MOVE, EVENT_UNWINDINGINFO,
  FILENAME_BUFFER_PADDING, FILENAME_FORMAT, HEADER_RESERVED_FILL, LOG_BUFFER_SIZE,
  PERF_JIT_HEADER_MAGIC, PERF_JIT_HEADER_VERSION, PerfJitBase, PerfJitCodeDebugInfo,
  PerfJitCodeLoad, PerfJitCodeUnwindingInfo, PerfJitDebugEntry, PerfJitHeader, RECORD_PAD,
  UNKNOWN_SCRIPT_NAME,
};
