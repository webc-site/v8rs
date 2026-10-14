//! gdb_jit 面金标: Writer 原语 / ELF64 与 MachO64 镜像拼装 / GDB JIT 注册链状态机。

use v8_diagnostics::{
  ELF64_IDENT, ELFHEADER_SIZE, ELFSECTIONHEADER_SIZE, ElfImage, ElfSection, ElfSectionType,
  GdbJitRegistry, JitAction, JitDescriptor, LC_SEGMENT_64, MACHOSECTIONHEADER_SIZE, MachOImage,
  MachOSectionDef, SHF_ALLOC, SHF_EXEC, Writer,
};

fn u16_at(b: &[u8], o: usize) -> u16 {
  u16::from_le_bytes(b[o..o + 2].try_into().unwrap())
}
fn u32_at(b: &[u8], o: usize) -> u32 {
  u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn u64_at(b: &[u8], o: usize) -> u64 {
  u64::from_le_bytes(b[o..o + 8].try_into().unwrap())
}

/// Writer 小端直写与名字/预留/对齐原语
#[test]
fn writer_primitives() {
  let mut w = Writer::new();
  w.write_u8(0xAB);
  w.write_u16(0x1234);
  w.write_u32(0xDEADBEEF);
  w.write_u64(0x0102030405060708);
  assert_eq!(
    w.bytes(),
    &[
      0xAB, 0x34, 0x12, 0xEF, 0xBE, 0xAD, 0xDE, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01
    ]
  );

  let mut w = Writer::new();
  w.write_name("abcdefghij", 4); // 超长截断
  w.write_name("ab", 5); // 短名零填充
  assert_eq!(w.bytes(), b"abcdab\0\0\0");

  let mut w = Writer::new();
  let at = w.reserve(16);
  assert_eq!(at, 0);
  assert_eq!(w.position(), 16);
  w.align(8); // 已对齐不动
  assert_eq!(w.position(), 16);
  w.write_u8(1);
  w.align(8); // 17 → 24
  assert_eq!(w.position(), 24);
  assert_eq!(&w.bytes()[17..24], &[0; 7]);

  // ULEB/SLEB 边界 (64 位)
  let mut w = Writer::new();
  w.write_uleb128(8192);
  assert_eq!(w.bytes(), &[0x80, 0x40]);
  let mut w = Writer::new();
  w.write_sleb128(-8);
  assert_eq!(w.bytes(), &[0x78]);
  let mut w = Writer::new();
  w.write_sleb128(64);
  assert_eq!(w.bytes(), &[0xC0, 0x00]);
  let mut w = Writer::new();
  w.write_sleb128(-65);
  assert_eq!(w.bytes(), &[0xBF, 0x7F]);
}

/// ELF64 镜像金标: 头 14 字段 + 节头表 4 项 + strtab + 实体回填
#[test]
fn elf_image_golden() {
  let mut img = ElfImage::new();
  let dbg_idx = img.add_section(ElfSection::progbits(".debug_info", vec![1, 2, 3]));
  img.add_section(ElfSection::text(0x1000, 0x90));
  assert_eq!(dbg_idx, 2, "首用户节索引 = NULL + .shstrtab 之后");
  assert_eq!(img.section_count(), 4);

  let b = img.write(62);
  // 头部 (ELFHEADER_FIELDS: ident 0/type 16/machine 18/version 20/shoff 40/shentsize 58/shnum 60/shstrndx 62)
  assert_eq!(&b[..16], &ELF64_IDENT);
  assert_eq!(u16_at(&b, 16), 1, "ET_REL");
  assert_eq!(u16_at(&b, 18), 62, "EM_X86_64");
  assert_eq!(u32_at(&b, 20), 1, "EV_CURRENT");
  assert_eq!(u64_at(&b, 40), ELFHEADER_SIZE as u64, "节头表紧随文件头");
  assert_eq!(u16_at(&b, 58), ELFSECTIONHEADER_SIZE as u16);
  assert_eq!(u16_at(&b, 60), 4);
  assert_eq!(u16_at(&b, 62), 1);

  // .shstrtab 内容: NUL + ".shstrtab" + ".debug_info" + ".text"
  let table = ELFHEADER_SIZE;
  let strtab = table + ELFSECTIONHEADER_SIZE * 4;
  assert_eq!(b[strtab], 0);
  assert_eq!(&b[strtab + 1..strtab + 11], b".shstrtab\0");
  assert_eq!(&b[strtab + 11..strtab + 23], b".debug_info\0");
  assert_eq!(&b[strtab + 23..strtab + 29], b".text\0");
  let strtab_size = 29usize;

  // 节头 1: .shstrtab
  let s1 = table + ELFSECTIONHEADER_SIZE;
  assert_eq!(u32_at(&b, s1), 1, "name=偏移");
  assert_eq!(u32_at(&b, s1 + 4), ElfSectionType::Strtab as u32);
  assert_eq!(u64_at(&b, s1 + 24), strtab as u64, "offset");
  assert_eq!(u64_at(&b, s1 + 32), strtab_size as u64, "size");
  assert_eq!(u64_at(&b, s1 + 48), 1, "align");

  // 节头 2: .debug_info 实体回填
  let s2 = s1 + ELFSECTIONHEADER_SIZE;
  assert_eq!(u32_at(&b, s2), 11);
  assert_eq!(u32_at(&b, s2 + 4), ElfSectionType::Progbits as u32);
  let body = strtab + strtab_size;
  assert_eq!(u64_at(&b, s2 + 24), body as u64);
  assert_eq!(u64_at(&b, s2 + 32), 3);
  assert_eq!(&b[body..body + 3], &[1, 2, 3]);

  // 节头 3: .text 预设头 (NOBITS, 不写实节)
  let s3 = s2 + ELFSECTIONHEADER_SIZE;
  assert_eq!(u32_at(&b, s3 + 4), ElfSectionType::Nobits as u32);
  assert_eq!(u64_at(&b, s3 + 8), SHF_ALLOC | SHF_EXEC);
  assert_eq!(u64_at(&b, s3 + 16), 0x1000, "addr");
  assert_eq!(u64_at(&b, s3 + 32), 0x90, "size 预设");
  assert_eq!(
    u64_at(&b, s3 + 48),
    64,
    "align=kCodeAlignment (gdb-jit.cc:1837-1839; globals.h:1117 bits=6)"
  );
  assert_eq!(b.len(), body + 3, "NOBITS 不追加实体");
}

/// MachO64 镜像金标: 头 + LC_SEGMENT_64 + 节头 + sizeofcmds/fileoff 回填
#[test]
fn macho_image_golden() {
  let mut img = MachOImage::new();
  img.add_section(MachOSectionDef::text(64, 0x1000, 0x90));
  let b = img.write(0x1000, 0x90);

  assert_eq!(u32_at(&b, 0), 0xFEEDFACF, "magic");
  assert_eq!(u32_at(&b, 16), 1, "ncmds");
  let cmd = 32;
  let sect = cmd + 72;
  let fileoff = sect + MACHOSECTIONHEADER_SIZE;
  assert_eq!(u32_at(&b, 20), (fileoff - cmd) as u32, "sizeofcmds");
  assert_eq!(u32_at(&b, cmd), LC_SEGMENT_64);
  assert_eq!(u32_at(&b, cmd + 4), (fileoff - cmd) as u32, "cmdsize");
  assert_eq!(u64_at(&b, cmd + 24), 0x1000, "vmaddr");
  assert_eq!(u64_at(&b, cmd + 32), 0x90, "vmsize");
  assert_eq!(u64_at(&b, cmd + 40), fileoff as u64, "fileoff");
  assert_eq!(u64_at(&b, cmd + 48), 0, "filesize 空实体");
  assert_eq!(u32_at(&b, cmd + 64), 1, "nsects");

  // 节头 (MACHOSECTIONHEADER_FIELDS: sectname0 segname16 addr32 size40 offset48 align52 flags64)
  assert_eq!(&b[sect..sect + 7], b"__text\0");
  assert_eq!(&b[sect + 16..sect + 23], b"__TEXT\0");
  assert_eq!(u64_at(&b, sect + 32), 0x1000);
  assert_eq!(u64_at(&b, sect + 40), 0x90, "无实体保留预设 size");
  assert_eq!(u32_at(&b, sect + 48), 0, "offset 无实体为 0");
  assert_eq!(
    u32_at(&b, sect + 52),
    6,
    "align 幂指数 (64=2^6, 上游 code_alignment=kCodeAlignment, gdb-jit.cc:1823-1827)"
  );
  assert_eq!(u32_at(&b, sect + 64), 0x8000_0400, "PURE|SOME_INSTRUCTIONS");
  assert_eq!(b.len(), fileoff);
}

/// GDB JIT 注册链: 头插 / 摘链 / 描述符动作 (对应 Register/UnregisterCodeEntry)
#[test]
fn registry_chain() {
  let mut r = GdbJitRegistry::new();
  assert_eq!(
    r.descriptor(),
    &JitDescriptor {
      version: 1,
      action: JitAction::Noaction,
      relevant_entry: None,
      first_entry: None
    }
  );

  let e0 = r.create_entry(b"a".to_vec());
  let e1 = r.create_entry(b"bb".to_vec());
  r.register_entry(e0);
  r.register_entry(e1);
  assert_eq!(r.descriptor().first_entry, Some(e1));
  assert_eq!(r.descriptor().action, JitAction::RegisterFn);
  assert_eq!(r.descriptor().relevant_entry, Some(e1));
  let e = r.entry(e1).unwrap();
  assert_eq!((e.next, e.prev), (Some(e0), None));
  assert_eq!(r.entry(e0).unwrap().prev, Some(e1));
  assert_eq!(&r.entry(e0).unwrap().symfile, b"a");

  r.unregister_entry(e1);
  assert_eq!(r.descriptor().first_entry, Some(e0), "摘链后头指向 e0");
  assert_eq!(r.descriptor().action, JitAction::UnregisterFn);
  assert_eq!(r.descriptor().relevant_entry, Some(e1));
  assert_eq!(r.entry(e0).unwrap().prev, None);
  let e = r.entry(e1).unwrap();
  assert_eq!((e.next, e.prev), (None, None), "摘链即清本节点指针");

  r.unregister_entry(e0);
  r.destroy_entry(e1);
  r.destroy_entry(e0);
  assert!(r.entry(e0).is_none());
}

/// 在链节点上 destroy 触发 debug_assert (状态机非法态防护)
#[test]
#[should_panic]
fn destroy_linked_entry_rejected() {
  let mut r = GdbJitRegistry::new();
  let e0 = r.create_entry(b"a".to_vec());
  r.register_entry(e0);
  r.destroy_entry(e0);
}

/// 越界与已释放索引安全防守: 不 panic
#[test]
fn registry_invalid_indices_safe() {
  let mut r = GdbJitRegistry::new();
  // 越界索引调用
  r.register_entry(999);
  r.unregister_entry(999);
  r.destroy_entry(999);
  assert!(r.entry(999).is_none());

  // 已销毁索引二次调用
  let e0 = r.create_entry(b"xyz".to_vec());
  r.register_entry(e0);
  r.unregister_entry(e0);
  r.destroy_entry(e0);
  r.register_entry(e0);
  r.unregister_entry(e0);
  r.destroy_entry(e0);
  assert!(r.entry(e0).is_none());
}

/// round2 补 3 条关键迁移 (上游不可达的 UB 路径在本模型下须零破坏):
/// 二次注册不成环 / 二次摘除不丢头 / 摘未注册节点不动链
#[test]
fn registry_reentrant_migrations() {
  let mut r = GdbJitRegistry::new();
  let e0 = r.create_entry(b"a".to_vec());
  let e1 = r.create_entry(b"b".to_vec());
  r.register_entry(e0);
  r.register_entry(e1);

  // 迁移 1: 重入 register — 若放行则 e1.next 指向自身成环 (GDB 遍历死循环), 须幂等忽略
  r.register_entry(e1);
  assert_eq!(r.entry(e1).unwrap().next, Some(e0), "不得自环");
  assert_eq!(r.entry(e0).unwrap().prev, Some(e1), "前驱未被二次头插污染");
  assert_eq!(r.descriptor().first_entry, Some(e1));

  // 迁移 2: 重入 unregister — 摘链后节点 next/prev 已清, 若放行会把 first_entry 误置 None 丢链
  r.unregister_entry(e1);
  r.unregister_entry(e1);
  assert_eq!(r.descriptor().first_entry, Some(e0), "二次摘除不得丢头");
  assert!(r.entry(e0).unwrap().prev.is_none());
  assert_eq!(r.descriptor().action, JitAction::UnregisterFn);

  // 迁移 3: 摘除从未注册的节点 — 上游 prev_=null 分支会把 first_entry_ 覆写为 null
  let e2 = r.create_entry(b"c".to_vec());
  r.register_entry(e2);
  let e3 = r.create_entry(b"d".to_vec());
  r.unregister_entry(e3);
  assert_eq!(r.descriptor().first_entry, Some(e2), "未注册摘除为 no-op");
  assert!(r.entry(e2).unwrap().linked, "在链节点状态不变");
}

// round4 D2 回归: Mach-O 实体节强制 addr=0 (gdb-jit.cc:193-197 DebugSectionBase::WriteBody)
#[test]
fn macho_body_section_forces_addr_zero() {
  let mut img = MachOImage::new();
  let mut s = MachOSectionDef::text(64, 0x1000, 0);
  s.body = vec![0x90u8; 8];
  img.add_section(s);
  let b = img.write(0x1000, 8);
  let cmd = 32;
  let sect = cmd + 72;
  assert_eq!(u64_at(&b, sect + 32), 0, "有实体时 addr 必零");
  assert_eq!(u64_at(&b, sect + 40), 8, "size 对齐实体长");
  assert_eq!(
    u32_at(&b, sect + 48),
    (sect + MACHOSECTIONHEADER_SIZE) as u32,
    "offset 指向实体"
  );
}
