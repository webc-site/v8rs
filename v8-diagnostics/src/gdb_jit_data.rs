// 由 tool/src/diagnostics/gdb-jit.js 从 v8/src/diagnostics/gdb-jit.cc 生成; 修订就地改, 复跑覆盖

//! GDB-JIT 镜像构造纯数据面 (基线生成; Writer/镜像拼装/注册流程见 gdb_jit.rs)。
const _: () = assert!(size_of::<usize>() == 8);

// ── Writer 缓冲参数 ──
pub const INITIAL_CAPACITY: usize = 1024;

// ── ELF ident / 头字段值 ──
pub const ELF64_IDENT: [u8; 16] = [0x7F, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const ELF_TYPE_REL: u16 = 1;
pub const ELF_MACHINE_X64: u16 = 62;
pub const ELF_VERSION: u32 = 1;
pub const ELF_SHT_STRTAB_INDEX: u16 = 1;

// ── Mach-O 头常量 (x64 分支) ──
pub const MACHO_MAGIC_64: u32 = 4277009103;
pub const MACHO_CPU_TYPE_X86_64: u32 = 16777223;
pub const MACHO_CPU_SUBTYPE_ALL: u32 = 3;
pub const MACHO_FILETYPE_OBJECT: u32 = 1;
pub const MACHO_PROT_ALL: u32 = 7;
pub const MACHO_NCMDS: u32 = 1;
pub const LC_SEGMENT_32: u32 = 1;
pub const LC_SEGMENT_64: u32 = 25;

// ── ELFSection::Type (17 项) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ElfSectionType {
  Null = 0,
  Progbits = 1,
  Symtab = 2,
  Strtab = 3,
  Rela = 4,
  Hash = 5,
  Dynamic = 6,
  Note = 7,
  Nobits = 8,
  Rel = 9,
  Shlib = 10,
  Dynsym = 11,
  Loproc = 1879048192,
  X8664Unwind = 1879048193,
  Hiproc = 2147483647,
  Louser = 2147483648,
  Hiuser = 4294967295,
}

// ── ELFSection::Flags / SpecialIndexes ──
pub const SHF_WRITE: u64 = 1;
pub const SHF_ALLOC: u64 = 2;
pub const SHF_EXEC: u64 = 4;
pub const ELF_INDEX_ABSOLUTE: u16 = 0xFFF1;

// ── MachOSection::Type (5 项) ──
pub const S_REGULAR: u32 = 0;
pub const S_ATTR_COALESCED: u32 = 11;
pub const S_ATTR_SOME_INSTRUCTIONS: u32 = 1024;
pub const S_ATTR_DEBUG: u32 = 33554432;
pub const S_ATTR_PURE_INSTRUCTIONS: u32 = 2147483648;

// ── GDB JIT 注册 ABI (extern "C") ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum JitAction {
  Noaction = 0,
  RegisterFn = 1,
  UnregisterFn = 2,
}
pub const JIT_DESCRIPTOR_VERSION: u32 = 1;

// ── 记录布局表 (64 位): [字段, 偏移, 大小] (SIZE = sizeof, 含尾部填充) ──
pub const ELFHEADER_FIELDS: &[(&str, usize, usize)] = &[
  ("ident", 0, 16),
  ("type", 16, 2),
  ("machine", 18, 2),
  ("version", 20, 4),
  ("entry", 24, 8),
  ("pht_offset", 32, 8),
  ("sht_offset", 40, 8),
  ("flags", 48, 4),
  ("header_size", 52, 2),
  ("pht_entry_size", 54, 2),
  ("pht_entry_num", 56, 2),
  ("sht_entry_size", 58, 2),
  ("sht_entry_num", 60, 2),
  ("sht_strtab_index", 62, 2),
];
pub const ELFHEADER_SIZE: usize = 64;
pub const ELFSECTIONHEADER_FIELDS: &[(&str, usize, usize)] = &[
  ("name", 0, 4),
  ("type", 4, 4),
  ("flags", 8, 8),
  ("address", 16, 8),
  ("offset", 24, 8),
  ("size", 32, 8),
  ("link", 40, 4),
  ("info", 44, 4),
  ("alignment", 48, 8),
  ("entry_size", 56, 8),
];
pub const ELFSECTIONHEADER_SIZE: usize = 64;
pub const MACHOHEADER_FIELDS: &[(&str, usize, usize)] = &[
  ("magic", 0, 4),
  ("cputype", 4, 4),
  ("cpusubtype", 8, 4),
  ("filetype", 12, 4),
  ("ncmds", 16, 4),
  ("sizeofcmds", 20, 4),
  ("flags", 24, 4),
  ("reserved", 28, 4),
];
pub const MACHOHEADER_SIZE: usize = 32;
pub const MACHOSEGMENTCOMMAND_FIELDS: &[(&str, usize, usize)] = &[
  ("cmd", 0, 4),
  ("cmdsize", 4, 4),
  ("segname", 8, 16),
  ("vmaddr", 24, 8),
  ("vmsize", 32, 8),
  ("fileoff", 40, 8),
  ("filesize", 48, 8),
  ("maxprot", 56, 4),
  ("initprot", 60, 4),
  ("nsects", 64, 4),
  ("flags", 68, 4),
];
pub const MACHOSEGMENTCOMMAND_SIZE: usize = 72;
pub const MACHOSECTIONHEADER_FIELDS: &[(&str, usize, usize)] = &[
  ("sectname", 0, 16),
  ("segname", 16, 16),
  ("addr", 32, 8),
  ("size", 40, 8),
  ("offset", 48, 4),
  ("align", 52, 4),
  ("reloff", 56, 4),
  ("nreloc", 60, 4),
  ("flags", 64, 4),
  ("reserved1", 68, 4),
  ("reserved2", 72, 4),
];
pub const MACHOSECTIONHEADER_SIZE: usize = 80;
pub const JITCODEENTRY_FIELDS: &[(&str, usize, usize)] = &[
  ("next_", 0, 8),
  ("prev_", 8, 8),
  ("symfile_addr_", 16, 8),
  ("symfile_size_", 24, 8),
];
pub const JITCODEENTRY_SIZE: usize = 32;
pub const JITDESCRIPTOR_FIELDS: &[(&str, usize, usize)] = &[
  ("version_", 0, 4),
  ("action_flag_", 4, 4),
  ("relevant_entry_", 8, 8),
  ("first_entry_", 16, 8),
];
pub const JITDESCRIPTOR_SIZE: usize = 24;
