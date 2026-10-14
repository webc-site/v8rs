//! GDB-JIT 镜像构造行为面 (对应 gdb-jit.cc 的 Writer/ELF/MachO/JIT 注册纯逻辑)。
//!
//! DWARF 调试节生成 (.debug_*/CFI) 依赖 CodeDescription/RelocInfo, 推迟 (阻塞: codegen-core + L3);
//! 此处落: Writer 缓冲原语、ELF64/MachO64 镜像拼装流程、GDB JIT 注册 ABI 链表语义。
//! 小端序与 V8 ia32/x64 面一致。

use crate::gdb_jit_data::{
  ELF_SHT_STRTAB_INDEX, ELF_TYPE_REL, ELF_VERSION, ELF64_IDENT, ELFHEADER_SIZE,
  ELFSECTIONHEADER_SIZE, ElfSectionType, INITIAL_CAPACITY, JIT_DESCRIPTOR_VERSION, JitAction,
  LC_SEGMENT_64, MACHO_CPU_SUBTYPE_ALL, MACHO_CPU_TYPE_X86_64, MACHO_FILETYPE_OBJECT,
  MACHO_MAGIC_64, MACHO_NCMDS, MACHO_PROT_ALL, MACHOSECTIONHEADER_SIZE, MACHOSEGMENTCOMMAND_SIZE,
  SHF_ALLOC, SHF_EXEC,
};

/// 镜像写缓冲 (对应上游 Writer: 位置推进 + 就地回填)
#[derive(Debug)]
pub struct Writer {
  buf: Vec<u8>,
}

impl Writer {
  pub fn new() -> Self {
    Self {
      buf: Vec::with_capacity(INITIAL_CAPACITY),
    }
  }

  pub fn position(&self) -> usize {
    self.buf.len()
  }

  pub fn bytes(&self) -> &[u8] {
    &self.buf
  }

  pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
    &mut self.buf
  }

  pub fn into_bytes(self) -> Vec<u8> {
    self.buf
  }

  pub fn write_u8(&mut self, v: u8) {
    self.buf.push(v);
  }

  pub fn write_u16(&mut self, v: u16) {
    self.buf.extend_from_slice(&v.to_le_bytes());
  }

  pub fn write_u32(&mut self, v: u32) {
    self.buf.extend_from_slice(&v.to_le_bytes());
  }

  pub fn write_u64(&mut self, v: u64) {
    self.buf.extend_from_slice(&v.to_le_bytes());
  }

  pub fn write_bytes(&mut self, v: &[u8]) {
    self.buf.extend_from_slice(v);
  }

  /// 定长名字字段: 零填充, 超长截断 (对应 strncpy 语义)
  pub fn write_name(&mut self, name: &str, len: usize) {
    let raw = name.as_bytes();
    let take = raw.len().min(len);
    self.buf.extend_from_slice(&raw[..take]);
    self.buf.resize(self.buf.len() + len - take, 0);
  }

  /// 预留 size 字节并返回起始偏移 (对应 CreateSlotsHere: 头表先占位、后回填)
  pub fn reserve(&mut self, size: usize) -> usize {
    let at = self.position();
    self.buf.resize(at + size, 0);
    at
  }

  /// align 须为 2 的幂
  pub fn align(&mut self, align: usize) {
    debug_assert!(align.is_power_of_two());
    let delta = self.position() % align;
    if delta != 0 {
      let pad = align - delta;
      self.buf.resize(self.buf.len() + pad, 0);
    }
  }

  pub fn patch_u32(&mut self, offset: usize, value: u32) {
    self.buf[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
  }

  pub fn patch_u64(&mut self, offset: usize, value: u64) {
    self.buf[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
  }

  /// ULEB128 (对应 WriteULEB128)
  pub fn write_uleb128(&mut self, mut value: u64) {
    loop {
      let mut byte = (value & 0x7F) as u8;
      value >>= 7;
      if value != 0 {
        byte |= 0x80;
      }
      self.write_u8(byte);
      if value == 0 {
        break;
      }
    }
  }

  /// SLEB128 (对应 WriteSLEB128)
  pub fn write_sleb128(&mut self, mut value: i64) {
    loop {
      let mut byte = (value & 0x7F) as u8;
      let sign = byte & 0x40 != 0;
      value >>= 7;
      let done = (value == 0 && !sign) || (value == -1 && sign);
      if !done {
        byte |= 0x80;
      }
      self.write_u8(byte);
      if done {
        break;
      }
    }
  }

  /// NUL 结尾串 (对应 WriteString)
  pub fn write_str(&mut self, s: &str) {
    self.write_bytes(s.as_bytes());
    self.write_u8(0);
  }
}

impl Default for Writer {
  fn default() -> Self {
    Self::new()
  }
}

/// ELF 节: Null/Stra 由镜像构建器内置, 用户节支持预设头 (FullHeaderELFSection) 与实体 (Progbits)
#[derive(Debug)]
pub struct ElfSection {
  pub name: String,
  pub stype: ElfSectionType,
  pub align: u64,
  pub flags: u64,
  pub addr: u64,
  /// 预设头 (NOBITS .text): Some((offset, size)) 时不写实节、不回填
  pub preset: Option<(u64, u64)>,
  /// 实体节内容
  pub body: Vec<u8>,
}

impl ElfSection {
  pub fn progbits(name: &str, body: Vec<u8>) -> Self {
    Self {
      name: name.into(),
      stype: ElfSectionType::Progbits,
      align: 1,
      flags: 0,
      addr: 0,
      preset: None,
      body,
    }
  }

  /// .text: TYPE_NOBITS + ALLOC|EXEC, 代码实体不在镜像内 (上游 FullHeaderELFSection)
  pub fn text(addr: u64, size: u64) -> Self {
    Self {
      name: ".text".into(),
      stype: ElfSectionType::Nobits,
      // 上游传 kCodeAlignment (gdb-jit.cc:1837-1839)。bits=6→64 仅 x64/PPC64 支
      // (globals.h:1116-1117,1121); arm64 及其余落 #else bits=5→32 (:1127-1128,:1130)。
      // 现取 x64 支常量, 按后端 cfg 化归消费侧接入票 (round7 D1 口径修正)
      align: 64,
      flags: SHF_ALLOC | SHF_EXEC,
      addr,
      preset: Some((0, size)),
      body: Vec::new(),
    }
  }
}

/// ELF64 调试镜像 (对应上游 ELF 类: NULL 节 + .shstrtab + 用户节)
pub struct ElfImage {
  sections: Vec<ElfSection>,
}

impl ElfImage {
  pub fn new() -> Self {
    Self {
      sections: Vec::new(),
    }
  }

  /// 追加用户节, 返回节索引 (含内置 NULL/SHSTRTAB, 与上游 set_index 一致)
  pub fn add_section(&mut self, section: ElfSection) -> usize {
    self.sections.push(section);
    self.sections.len() + 1
  }

  /// 镜像总节数 = 用户节 + NULL + .shstrtab
  pub fn section_count(&self) -> usize {
    self.sections.len() + 2
  }

  /// 拼装 (对应 ELF::Write: 头 → 节头表 → 节实体), machine 为 EM_ 值
  pub fn write(&self, machine: u16) -> Vec<u8> {
    let total = self.section_count();
    let shdr_size = ELFSECTIONHEADER_SIZE;
    let mut w = Writer::new();

    // WriteHeader: ELF64 头 14 字段顺序直写 (对照 ELFHEADER_FIELDS), 凑满 ELFHEADER_SIZE
    w.write_bytes(&ELF64_IDENT);
    w.write_u16(ELF_TYPE_REL);
    w.write_u16(machine);
    w.write_u32(ELF_VERSION);
    w.write_u64(0); // entry
    w.write_u64(0); // pht_offset (无程序头表)
    w.write_u64(ELFHEADER_SIZE as u64); // sht_offset: 节头表紧随文件头
    w.write_u32(0); // flags
    w.write_u16(ELFHEADER_SIZE as u16); // header_size
    w.write_u16(0); // pht_entry_size
    w.write_u16(0); // pht_entry_num
    w.write_u16(shdr_size as u16); // sht_entry_size
    w.write_u16(total as u16); // sht_entry_num
    w.write_u16(ELF_SHT_STRTAB_INDEX); // sht_strtab_index
    debug_assert_eq!(w.position(), ELFHEADER_SIZE);

    // WriteSectionTable: 节头表占位, .shstrtab 字符串随表写出
    let table = w.reserve(shdr_size * total);
    let strtab_start = w.position();
    w.write_u8(0); // 首项空串
    let mut strtab_size = 1usize;
    let mut names: Vec<u32> = Vec::with_capacity(total);
    names.push(0); // NULL 节空名
    names.push(add_str(&mut w, ".shstrtab", &mut strtab_size) as u32);
    for s in &self.sections {
      names.push(add_str(&mut w, &s.name, &mut strtab_size) as u32);
    }

    // 节头: NULL / .shstrtab / 用户节 (offset/size 在 WriteSections 阶段回填)
    Shdr::NULL.write(&mut w, table, 0);
    Shdr::strtab(names[1]).write(&mut w, table, 1);
    for (i, s) in self.sections.iter().enumerate() {
      Shdr {
        stype: s.stype,
        name: names[i + 2],
        flags: s.flags,
        align: s.align,
        addr: s.addr,
      }
      .write(&mut w, table, i + 2);
    }

    // WriteSections: .shstrtab 回填, 实体节写实并回填
    let strtab_off_slot = table + shdr_size + 24; // name(0) type(4) flags(8) addr(16) offset(24)
    w.patch_u64(strtab_off_slot, strtab_start as u64);
    w.patch_u64(table + shdr_size + 32, strtab_size as u64);
    for (i, s) in self.sections.iter().enumerate() {
      let at = table + shdr_size * (i + 2);
      if let Some((offset, size)) = s.preset {
        w.patch_u64(at + 24, offset);
        w.patch_u64(at + 32, size);
        continue;
      }
      if !s.body.is_empty() {
        let start = w.position();
        w.write_bytes(&s.body);
        w.patch_u64(at + 24, start as u64);
        w.patch_u64(at + 32, s.body.len() as u64);
      }
    }
    w.into_bytes()
  }
}

impl Default for ElfImage {
  fn default() -> Self {
    Self::new()
  }
}

/// 向 .shstrtab 追加串并返回偏移 (对应 ELFStringTable::Add: 空串→0, 不去重)
fn add_str(w: &mut Writer, name: &str, strtab_size: &mut usize) -> usize {
  if name.is_empty() {
    return 0;
  }
  let offset = *strtab_size;
  w.write_str(name);
  *strtab_size += name.len() + 1;
  offset
}

/// 节头写出字段 (ELFSECTIONHEADER_FIELDS 的可写子集)
#[derive(Debug, Clone, Copy)]
struct Shdr {
  stype: ElfSectionType,
  name: u32,
  flags: u64,
  align: u64,
  addr: u64,
}

impl Shdr {
  const NULL: Self = Self {
    stype: ElfSectionType::Null,
    name: 0,
    flags: 0,
    align: 0,
    addr: 0,
  };

  fn strtab(name: u32) -> Self {
    Self {
      stype: ElfSectionType::Strtab,
      name,
      flags: 0,
      align: 1,
      addr: 0,
    }
  }

  /// 对照 ELFSECTIONHEADER_FIELDS: name(0) type(4) flags(8) address(16) alignment(48)
  fn write(self, w: &mut Writer, table: usize, index: usize) {
    let at = table + ELFSECTIONHEADER_SIZE * index;
    w.patch_u32(at, self.name);
    w.patch_u32(at + 4, self.stype as u32);
    w.patch_u64(at + 8, self.flags);
    w.patch_u64(at + 16, self.addr);
    w.patch_u64(at + 48, self.align);
  }
}

/// Mach-O 节 (对应 MachOSection: align 存 2 的幂指数)
#[derive(Debug)]
pub struct MachOSectionDef {
  pub name: String,
  pub segment: String,
  pub align_pow2: u32,
  pub flags: u32,
  pub addr: u64,
  pub size: u64,
  pub body: Vec<u8>,
}

/// __text 节 (对应 MachOTextSection)
impl MachOSectionDef {
  pub fn text(align: u32, addr: u64, size: u64) -> Self {
    use crate::gdb_jit_data::{S_ATTR_PURE_INSTRUCTIONS, S_ATTR_SOME_INSTRUCTIONS, S_REGULAR};
    Self {
      name: "__text".into(),
      segment: "__TEXT".into(),
      align_pow2: align.trailing_zeros(),
      flags: S_REGULAR | S_ATTR_SOME_INSTRUCTIONS | S_ATTR_PURE_INSTRUCTIONS,
      addr,
      size,
      body: Vec::new(),
    }
  }
}

/// MachO64 调试镜像 (对应上游 MachO 类: 头 + LC_SEGMENT_64 + 节)
pub struct MachOImage {
  sections: Vec<MachOSectionDef>,
}

impl MachOImage {
  pub fn new() -> Self {
    Self {
      sections: Vec::new(),
    }
  }

  pub fn add_section(&mut self, section: MachOSectionDef) -> usize {
    self.sections.push(section);
    self.sections.len() - 1
  }

  /// 拼装 (对应 MachO::Write: 头 → 段命令 → 节头 → 节实体)
  pub fn write(&self, code_start: u64, code_size: u64) -> Vec<u8> {
    let nsects = self.sections.len();
    let sect_hdr = MACHOSECTIONHEADER_SIZE;
    let cmd_size = MACHOSEGMENTCOMMAND_SIZE;
    let mut w = Writer::new();

    // MachOHeader
    w.write_u32(MACHO_MAGIC_64);
    w.write_u32(MACHO_CPU_TYPE_X86_64);
    w.write_u32(MACHO_CPU_SUBTYPE_ALL);
    w.write_u32(MACHO_FILETYPE_OBJECT);
    w.write_u32(MACHO_NCMDS);
    let sizeof_cmds_at = 20;
    w.write_u32(0); // sizeofcmds 回填
    w.write_u32(0); // flags
    w.write_u32(0); // reserved

    // MachOSegmentCommand (对照 MACHOSEGMENTCOMMAND_FIELDS)
    let load_command_start = w.position();
    w.write_u32(LC_SEGMENT_64);
    w.write_u32((cmd_size + sect_hdr * nsects) as u32); // cmdsize
    w.write_name("", 16); // segname 清零 (上游 memset)
    w.write_u64(code_start); // vmaddr
    w.write_u64(code_size); // vmsize
    let fileoff_at = w.position();
    w.write_u64(0); // fileoff 回填
    let filesize_at = w.position();
    w.write_u64(0); // filesize 回填
    w.write_u32(MACHO_PROT_ALL);
    w.write_u32(MACHO_PROT_ALL);
    w.write_u32(nsects as u32);
    w.write_u32(0); // flags

    // 节头表占位后写实体
    let sect_table = w.reserve(sect_hdr * nsects);
    let fileoff = w.position();
    w.patch_u64(fileoff_at, fileoff as u64);
    w.patch_u32(sizeof_cmds_at, (fileoff - load_command_start) as u32);

    for (i, s) in self.sections.iter().enumerate() {
      let at = sect_table + sect_hdr * i;
      // 节头字段按 MACHOSECTIONHEADER_FIELDS 布局回填
      let mut off = at;
      write_name_at(&mut w, &mut off, &s.name, 16);
      write_name_at(&mut w, &mut off, &s.segment, 16);
      let addr_at = off;
      w.patch_u64(off, s.addr);
      off += 8;
      w.patch_u64(off, s.size);
      off += 8;
      w.patch_u32(off, 0); // offset (写实节时在下方补)
      w.patch_u32(off + 4, s.align_pow2);
      w.patch_u32(off + 8, 0); // reloff
      w.patch_u32(off + 12, 0); // nreloc
      w.patch_u32(off + 16, s.flags);
      w.patch_u32(off + 20, 0); // reserved1
      w.patch_u32(off + 24, 0); // reserved2

      // WriteBody: 有实体才回填 offset/size (上游 WriteBodyInternal 语义);
      // Mach-O 下实体节强制 addr=0 (gdb-jit.cc:193-197 header->addr = 0)
      if !s.body.is_empty() {
        let start = w.position();
        w.write_bytes(&s.body);
        w.patch_u64(addr_at, 0); // addr 置零, 覆盖预写的 s.addr
        w.patch_u32(off, start as u32);
        // size 字段 (addr 后 8 字节) 对齐实体长度
        let size_at = at + 40;
        w.patch_u64(size_at, s.body.len() as u64);
      }
    }
    let filesize = w.position() - fileoff;
    w.patch_u64(filesize_at, filesize as u64);
    w.into_bytes()
  }
}

impl Default for MachOImage {
  fn default() -> Self {
    Self::new()
  }
}

fn write_name_at(w: &mut Writer, off: &mut usize, name: &str, len: usize) {
  let raw = name.as_bytes();
  let take = raw.len().min(len);
  w.bytes_mut()[*off..*off + take].copy_from_slice(&raw[..take]);
  w.bytes_mut()[*off + take..*off + len].fill(0);
  *off += len;
}

/// GDB JIT 注册表 (对应 __jit_debug_descriptor + Create/Register/Unregister/DestroyCodeEntry)
pub struct GdbJitRegistry {
  entries: Vec<Option<JitCodeEntry>>,
  descriptor: JitDescriptor,
}

/// 注册链节点: symfile 内联在节点尾部 (上游为 malloc 复合分配)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitCodeEntry {
  pub next: Option<usize>,
  pub prev: Option<usize>,
  /// 是否已挂链: 上游靠 GdbJitImageRegistration 单次注册守护, 此处显式化防重入
  pub linked: bool,
  pub symfile: Vec<u8>,
}

/// GDB 观察的描述符
#[derive(Debug, PartialEq, Eq)]
pub struct JitDescriptor {
  pub version: u32,
  pub action: JitAction,
  pub relevant_entry: Option<usize>,
  pub first_entry: Option<usize>,
}

impl GdbJitRegistry {
  pub fn new() -> Self {
    Self {
      entries: Vec::new(),
      descriptor: JitDescriptor {
        version: JIT_DESCRIPTOR_VERSION,
        action: JitAction::Noaction,
        relevant_entry: None,
        first_entry: None,
      },
    }
  }

  pub fn descriptor(&self) -> &JitDescriptor {
    &self.descriptor
  }

  pub fn entry(&self, index: usize) -> Option<&JitCodeEntry> {
    self.entries.get(index).and_then(Option::as_ref)
  }

  /// CreateCodeEntry: 拷贝 symfile, 前后指针清空
  pub fn create_entry(&mut self, symfile: Vec<u8>) -> usize {
    self.entries.push(Some(JitCodeEntry {
      next: None,
      prev: None,
      linked: false,
      symfile,
    }));
    self.entries.len() - 1
  }

  /// RegisterCodeEntry: 头插 + relevant 指向本项 + REGISTER 动作
  /// 重入守护: 上游对同一 entry 仅注册一次 (GdbJitImageRegistration 守护);
  /// 此处重复注册若放行会令 next 指向自身成环 (GDB 遍历死循环), 故直接忽略
  pub fn register_entry(&mut self, index: usize) {
    let first = self.descriptor.first_entry;
    if let Some(Some(entry)) = self.entries.get_mut(index) {
      if entry.linked {
        return;
      }
      entry.linked = true;
      entry.next = first;
      entry.prev = None;
    } else {
      return;
    }
    if let Some(next) = first
      && let Some(Some(next_entry)) = self.entries.get_mut(next)
    {
      next_entry.prev = Some(index);
    }
    self.descriptor.first_entry = Some(index);
    self.descriptor.relevant_entry = Some(index);
    self.descriptor.action = JitAction::RegisterFn;
  }

  /// UnregisterCodeEntry: 摘链 + relevant 指向本项 + UNREGISTER 动作
  /// 上游保留 entry 自身 next_/prev_ 供 GDB 轮询; 本所有权模型下摘链即清空, 使 destroy 前置可达。
  /// 幂等守护: 二次摘除/摘未注册节点在上游是改写陈旧指针的 UB 路径 (头节点二次摘除会把
  /// first_entry_ 指向失效 next_), 此处以 linked 标志令其对合法链零破坏
  pub fn unregister_entry(&mut self, index: usize) {
    let (next, prev) = match self.entries.get_mut(index) {
      Some(Some(entry)) if entry.linked => {
        entry.linked = false;
        (entry.next, entry.prev)
      }
      _ => return,
    };
    match prev {
      Some(p) => {
        if let Some(Some(prev_entry)) = self.entries.get_mut(p) {
          prev_entry.next = next;
        }
      }
      None => self.descriptor.first_entry = next,
    }
    if let Some(nx) = next
      && let Some(Some(next_entry)) = self.entries.get_mut(nx)
    {
      next_entry.prev = prev;
    }
    if let Some(Some(entry)) = self.entries.get_mut(index) {
      entry.next = None;
      entry.prev = None;
    }
    self.descriptor.relevant_entry = Some(index);
    self.descriptor.action = JitAction::UnregisterFn;
  }

  /// DestroyCodeEntry: 释放节点 (上游 free; 此处仅允许销毁已摘链节点)
  pub fn destroy_entry(&mut self, index: usize) {
    if let Some(Some(entry)) = self.entries.get(index) {
      debug_assert!(
        entry.next.is_none() && entry.prev.is_none() && self.descriptor.first_entry != Some(index)
      );
      self.entries[index] = None;
    }
  }
}

impl Default for GdbJitRegistry {
  fn default() -> Self {
    Self::new()
  }
}
