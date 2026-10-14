//! eh_frame 记录金标: CIE/FDE/hdr 字节流 + 迭代器回读 + LEB128 边界。

use v8_diagnostics::{
  AUGMENTATION_STRING, CIE_VERSION, DwarfOpcode, EH_FRAME_ARM64, EH_FRAME_HDR_SIZE, EH_FRAME_X64,
  EhFrameIterator, EhFrameWriter, INITIAL_STATE_OFFSET_IN_CIE, INT32_PLACEHOLDER, OMIT,
  decode_sleb128, decode_uleb128,
};

fn u32_at(buf: &[u8], off: usize) -> u32 {
  u32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

/// x64 CIE 金标: 记录 24 字节 + 4 尺寸头, 初态 def_cfa(rsp,8)+saved_ra, nop 补齐到 8 对齐
#[test]
fn x64_cie_golden() {
  let mut w = EhFrameWriter::new(EH_FRAME_X64);
  w.initialize();
  let b = w.buffer();
  assert_eq!(u32_at(b, 0), 24);
  assert_eq!(u32_at(b, 4), 0, "CIE identifier");
  assert_eq!(b[8], CIE_VERSION);
  assert_eq!(&b[9..13], &AUGMENTATION_STRING[..]);
  assert_eq!(b[13], 0x01, "code_align SLEB 1");
  assert_eq!(b[14], 0x78, "data_align SLEB -8");
  assert_eq!(b[15], 0x10, "return_address ULEB 16 (rip)");
  assert_eq!(b[16], 0x02, "augmentation data size");
  assert_eq!(b[17], OMIT);
  assert_eq!(b[18], 11 | 16, "FDE pointer encoding SDATA4|PC_REL");
  assert_eq!(INITIAL_STATE_OFFSET_IN_CIE, 19);
  assert_eq!(b[19], DwarfOpcode::DefCfa as u8);
  assert_eq!(b[20], 7, "cfa = rsp");
  assert_eq!(b[21], 8, "cfa offset 8");
  assert_eq!(b[22], (2 << 6) | 16, "GNU_compact_saved_register rip");
  assert_eq!(b[23], 1, "factored -8/-8 = 1");
  assert_eq!(&b[24..28], &[0, 0, 0, 0], "nop 补齐");
  assert_eq!(w.cie_size(), 28);
  // FDE 头
  assert_eq!(u32_at(b, 28), INT32_PLACEHOLDER, "尺寸待 finish 回填");
  assert_eq!(u32_at(b, 32), 32, "CIE 回指");
  assert_eq!(u32_at(b, 36), INT32_PLACEHOLDER, "过程地址占位");
  assert_eq!(u32_at(b, 40), INT32_PLACEHOLDER, "过程尺寸占位");
  assert_eq!(b[44], 0, "FDE augmentation length 0");
}

/// arm64 初态: def_cfa(fp,0) + lr same_value
#[test]
fn arm64_cie_golden() {
  let mut w = EhFrameWriter::new(EH_FRAME_ARM64);
  w.initialize();
  let b = w.buffer();
  assert_eq!(u32_at(b, 0), 24);
  assert_eq!(b[13], 0x04, "code_align SLEB 4");
  assert_eq!(b[14], 0x78, "data_align SLEB -8");
  assert_eq!(b[15], 30, "return_address lr");
  assert_eq!(b[19], DwarfOpcode::DefCfa as u8);
  assert_eq!(b[20], 29, "cfa = fp");
  assert_eq!(b[21], 0, "cfa offset 0");
  assert_eq!(b[22], DwarfOpcode::SameValue as u8);
  assert_eq!(b[23], 30, "same_value lr");
}

/// x64 完整流程金标: advance(8) + finish(16) → 72 字节 (CIE 28 + FDE 20 + 终止符 4 + hdr 20)
#[test]
fn x64_full_stream_golden() {
  let mut w = EhFrameWriter::new(EH_FRAME_X64);
  w.initialize();
  w.advance_location(8);
  w.finish(16);
  let b = w.buffer();
  assert_eq!(b.len(), 72);
  assert_eq!(b[45], (1 << 6) | 8, "advance_loc(8) 紧凑标签");
  assert_eq!(&b[46..48], &[0, 0], "FDE nop 补齐");
  // FDE 定稿回填
  assert_eq!(u32_at(b, 28), 16, "encoded FDE size = 48-28-4");
  assert_eq!(u32_at(b, 32), 32, "CIE 回指");
  assert_eq!(
    u32_at(b, 36),
    (-52i32) as u32,
    "过程地址 = -(16+36) PC 相对"
  );
  assert_eq!(u32_at(b, 40), 16, "code size");
  assert_eq!(&b[48..52], &[0, 0, 0, 0], ".eh_frame 终止符");
  // .eh_frame_hdr
  let hdr = &b[52..];
  assert_eq!(hdr.len(), EH_FRAME_HDR_SIZE);
  assert_eq!(hdr[0], 1, "hdr version");
  assert_eq!(hdr[1], 11 | 16, "ptr_enc");
  assert_eq!(hdr[2], 3, "fde_count_enc UDATA4");
  assert_eq!(hdr[3], 11 | 48, "table_enc SDATA4|DATA_REL");
  assert_eq!(
    u32_at(hdr, 4),
    (-56i32) as u32,
    "eh_frame 相对偏移 = -(52+1+3)"
  );
  assert_eq!(u32_at(hdr, 8), 1, "唯一例程");
  assert_eq!(
    u32_at(hdr, 12),
    (-68i32) as u32,
    "例程起始相对 hdr = -(16+52)"
  );
  assert_eq!(u32_at(hdr, 16), (-24i32) as u32, "FDE 起始相对 hdr = 28-52");
}

/// 迭代器回读: 从金标流逐字段重放 CIE, 再跳读到 FDE 指令
#[test]
fn iterator_roundtrip() {
  let mut w = EhFrameWriter::new(EH_FRAME_X64);
  w.initialize();
  w.advance_location(8);
  w.finish(16);
  let buf = w.buffer().to_vec();

  let mut it = EhFrameIterator::new(&buf);
  assert_eq!(it.next_u32(), Some(24), "CIE 尺寸");
  assert_eq!(it.next_u32(), Some(0), "identifier");
  assert_eq!(it.next_byte(), Some(CIE_VERSION));
  for expected in &AUGMENTATION_STRING {
    assert_eq!(it.next_byte(), Some(*expected));
  }
  assert_eq!(it.next_sleb128(), Some(1), "code_align");
  assert_eq!(it.next_sleb128(), Some(-8), "data_align");
  assert_eq!(it.next_uleb128(), Some(16), "ra code");
  assert_eq!(it.next_uleb128(), Some(2), "augmentation data size");
  assert_eq!(it.next_byte(), Some(OMIT));
  assert_eq!(it.next_byte(), Some(11 | 16));
  assert_eq!(it.current_offset(), INITIAL_STATE_OFFSET_IN_CIE);
  assert_eq!(it.next_byte(), Some(DwarfOpcode::DefCfa as u8));
  assert_eq!(it.next_uleb128(), Some(7));
  assert_eq!(it.next_uleb128(), Some(8));
  assert_eq!(
    it.next_byte(),
    Some((2 << 6) | 16),
    "saved_register 紧凑标签"
  );
  assert_eq!(it.next_uleb128(), Some(1));

  let mut it = EhFrameIterator::new(&buf);
  it.skip_to_fde_directives().unwrap();
  assert_eq!(it.current_offset(), 45, "FDE 指令起点");
  assert_eq!(it.next_byte(), Some((1 << 6) | 8), "重放 advance");

  let mut fresh = EhFrameIterator::new(&buf);
  assert!(!fresh.done());
  fresh.skip(buf.len());
  assert!(fresh.done());
}

/// skip_cie 终裁 1: 手工 zLR CIE (zR 编码形态 SDATA4|PC_REL) + FDE 字节序列,
/// 大块跳过语义 = 尺寸字段 + 4, 与 C++ SkipCie (eh-frame.h:220) 逐字节一致;
/// 内部 uleb/sleb 变长不参与解析, 落点必须恰为 FDE 起点。
#[test]
fn skip_cie_handmade_zlr() {
  let mut buf: Vec<u8> = Vec::new();
  // CIE: 尺寸占位后回填 24 (record = 4..28)
  buf.extend_from_slice(&[
    0, 0, 0, 0, // [0..4] 尺寸字段
    0, 0, 0, 0, // CIE identifier
    3, // version
    b'z', b'L', b'R', 0,    // augmentation "zLR"
    0x01, // code_align SLEB 1
    0x78, // data_align SLEB -8
    0x90,
    0x01, // ra ULEB 16 的双字节回绕形态 (0x90,0x01 = 16) → 陷阱: 变长 uleb
    0x02, // augmentation data size
    0xFF, // LSDA omit
    0x1B, // "R" 编码: SDATA4|PC_REL = 11|16
    0x0C, 0x07, 0x08, // def_cfa(rsp,8)
    0x90, 0x01, 0x7F, // offset_extended_sf 16, sleb -1 (负 data_align 形态)
  ]);
  // record 体长 = buf.len() - 4 = 22 → 补齐 nop 到 8 对齐 (RoundUp(22,8)=24)
  let record_len = buf.len() - 4;
  let padded = record_len.div_ceil(8) * 8;
  assert_eq!(record_len, 22, "手工 CIE 记录体");
  buf.resize(4 + padded, 0);
  let size = padded as u32;
  buf[0..4].copy_from_slice(&size.to_le_bytes());
  // FDE: 尺寸 16 + 回指 (size+4) + 地址 + 尺寸 + aug 0 + 3 字节补齐
  let cie_back = size + 4;
  buf.extend_from_slice(&[16, 0, 0, 0]);
  buf.extend_from_slice(&cie_back.to_le_bytes());
  buf.extend_from_slice(&[0xEF, 0xBE, 0xAD, 0xDE]);
  buf.extend_from_slice(&[16, 0, 0, 0]);
  buf.extend_from_slice(&[0]);
  buf.extend_from_slice(&[0, 0, 0]);

  let mut it = EhFrameIterator::new(&buf);
  it.skip_cie().unwrap();
  let fde_start = 4 + padded;
  assert_eq!(it.current_offset(), fde_start, "skip_cie 落点 = FDE 起点");
  assert_eq!(it.next_u32(), Some(16), "FDE 尺寸字段");
  assert_eq!(it.next_u32(), Some(cie_back), "CIE 回指 = 尺寸字段 + 4");
  assert_eq!(&buf[9..13], b"zLR\0", "augmentation 字符串");
  assert_eq!(buf[19], 0x1B, "zR 编码说明符 SDATA4|PC_REL");

  let mut it2 = EhFrameIterator::new(&buf);
  it2.skip_to_fde_directives().unwrap();
  assert_eq!(
    it2.current_offset(),
    fde_start + 17,
    "kDirectivesOffsetInFde=17"
  );
}

/// skip_cie 终裁 2: 尺寸字段在 uint32 域 +kInt32Size 回绕 (C++ `ReadUnalignedValue + kInt32Size`
/// 为 uint32 算术 mod 2^32, 仅 0xFFFFFFFC..0xFFFFFFFF 触发, 回绕步进 0..3; 短缓冲 → None)。
#[test]
fn skip_cie_size_field_wraparound() {
  // 0xFFFFFFFC + 4 = 2^32 → 回绕为 0, 游标不推进
  let buf = [0xFCu8, 0xFF, 0xFF, 0xFF, 1, 2, 3, 4];
  let mut it = EhFrameIterator::new(&buf);
  it.skip_cie().unwrap();
  assert_eq!(it.current_offset(), 0, "u32 回绕: 步进 0");
  assert_eq!(it.next_u32(), Some(0xFFFF_FFFC), "原地重读尺寸字段");

  // 0xFFFFFFFF + 4 = 0x1_00000003 → mod 2^32 = 3 字节 (非回绕的 usize 算术会推进 2^32+3 越界)
  let mut buf2 = vec![0xFFu8, 0xFF, 0xFF, 0xFF];
  buf2.resize(20, 0);
  let mut it2 = EhFrameIterator::new(&buf2);
  it2.skip_cie().unwrap();
  assert_eq!(it2.current_offset(), 3, "u32 回绕: 步进 3 而非 2^32+3");

  // 尺寸字段截断 (<4 字节): Option 面安全退出, 游标不动
  let short = [0x01u8, 0x02];
  let mut it3 = EhFrameIterator::new(&short);
  assert_eq!(it3.skip_cie(), None);
  assert_eq!(it3.current_offset(), 0);
}

/// advance_location 宽度退化: 紧凑 / 1 / 2 / 4 字节形式 (增量基于上一次 pc)
#[test]
fn advance_location_widths() {
  let mut w = EhFrameWriter::new(EH_FRAME_X64);
  w.initialize();
  w.advance_location(63); // factored 63 ≤ mask → 紧凑
  w.advance_location(127); // 增量 64 → AdvanceLoc1
  w.advance_location(427); // 增量 300 → AdvanceLoc2
  w.advance_location(70427); // 增量 70000 → AdvanceLoc4
  let b = w.buffer();
  let off = 45;
  assert_eq!(b[off], (1 << 6) | 63);
  assert_eq!(b[off + 1], DwarfOpcode::AdvanceLoc1 as u8);
  assert_eq!(b[off + 2], 64);
  assert_eq!(b[off + 3], DwarfOpcode::AdvanceLoc2 as u8);
  assert_eq!(
    u16::from_le_bytes(b[off + 4..off + 6].try_into().unwrap()),
    300
  );
  assert_eq!(b[off + 6], DwarfOpcode::AdvanceLoc4 as u8);
  assert_eq!(u32_at(b, off + 7), 70000);
}

/// 基址/恢复规则指令编码 (含 offset_extended_sf 负商分支)
#[test]
fn rule_directives() {
  let mut w = EhFrameWriter::new(EH_FRAME_X64);
  w.initialize();
  let start = w.offset();
  w.set_base_address_offset(16);
  w.set_base_address_register(6);
  w.record_register_saved_to_stack(3, 8); // 8/-8 = -1 < 0 → offset_extended_sf
  w.record_register_follows_initial_rule(6);
  w.record_register_follows_initial_rule(100); // > mask → restore_extended
  let b = w.buffer();
  assert_eq!(b[start], DwarfOpcode::DefCfaOffset as u8);
  assert_eq!(b[start + 1], 16);
  assert_eq!(b[start + 2], DwarfOpcode::DefCfaRegister as u8);
  assert_eq!(b[start + 3], 6);
  assert_eq!(b[start + 4], DwarfOpcode::OffsetExtendedSf as u8);
  assert_eq!(b[start + 5], 3);
  assert_eq!(b[start + 6], 0x7F, "SLEB -1 单字节");
  assert_eq!(b[start + 7], (3 << 6) | 6, "GNU_compact_restore");
  assert_eq!(b[start + 8], DwarfOpcode::RestoreExtended as u8);
  assert_eq!(b[start + 9], 100);
}

/// 空 eh_frame_hdr: 20 字节 (4 版本字节 + 16 零)
#[test]
fn empty_eh_frame_hdr() {
  let mut out = Vec::new();
  EhFrameWriter::write_empty_eh_frame(&mut out);
  assert_eq!(out.len(), EH_FRAME_HDR_SIZE);
  assert_eq!(&out[..4], &[1, 11 | 16, 3, 11 | 48]);
  assert!(out[4..].iter().all(|&b| b == 0));
}

/// LEB128 解码边界 (经典 ±624485 用例, 对齐上游 eh-frame-iterator-unittest)
#[test]
fn leb128_edges() {
  assert_eq!(decode_uleb128(&[0x00]), Some((0, 1)));
  assert_eq!(decode_uleb128(&[0x7F]), Some((127, 1)));
  assert_eq!(decode_uleb128(&[0xE5, 0x8E, 0x26]), Some((624485, 3)));
  assert_eq!(
    decode_uleb128(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]),
    Some((0xFFFF_FFFF, 5))
  );
  // round3 D3: 第 5 字节 bit4-6 置位 → u32 溢出拒绝分支 (eh_frame.rs:463-465) 补测
  assert_eq!(
    decode_uleb128(&[0xFF, 0xFF, 0xFF, 0xFF, 0x1F]),
    None,
    "第 5 字节 bit4-6 溢出"
  );
  assert_eq!(decode_uleb128(&[0xE5, 0x8E]), None, "截断");

  assert_eq!(decode_sleb128(&[0x00]), Some((0, 1)));
  assert_eq!(decode_sleb128(&[0x7F]), Some((-1, 1)));
  assert_eq!(
    decode_sleb128(&[0x78]),
    Some((-8, 1)),
    "标准 SLEB 单字节 -8"
  );
  assert_eq!(
    decode_sleb128(&[0xF8]),
    None,
    "0xF8 续接位=1 无后续字节 → 截断"
  );
  assert_eq!(
    decode_sleb128(&[0x3F]),
    Some((63, 1)),
    "0x3F 符号位=0 → +63 正边界"
  );
  assert_eq!(
    decode_sleb128(&[0x40]),
    Some((-64, 1)),
    "0x40 符号位=1 → -64 负边界"
  );
  assert_eq!(
    decode_sleb128(&[0x9B, 0xF1, 0x59]),
    Some((-624485, 3)),
    "上游 SLEB128DecodingNegative"
  );
  assert_eq!(decode_sleb128(&[0xE5, 0x8E, 0x26]), Some((624485, 3)));
  assert_eq!(decode_sleb128(&[0x9B, 0xF1]), None, "截断");
  assert_eq!(
    decode_uleb128(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x01]),
    None,
    "超长 >5 字节 uleb128 安全拒绝不 panic"
  );
  assert_eq!(
    decode_sleb128(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x01]),
    None,
    "超长 >5 字节 sleb128 安全拒绝不 panic"
  );
}

/// 截断缓冲安全测试: 尺寸字段超出物理缓冲时返回 None 而非 panic
#[test]
fn truncated_buffer_safety() {
  let truncated_cie = [100u8, 0, 0, 0, 1, 2, 3];
  let mut it = EhFrameIterator::new(&truncated_cie);
  assert_eq!(it.skip_cie(), None);
  assert_eq!(it.skip_to_fde_directives(), None);

  let valid_cie_truncated_fde = [
    0, 0, 0, 0, // CIE size 0 (step 4)
  ];
  let mut it2 = EhFrameIterator::new(&valid_cie_truncated_fde);
  assert_eq!(it2.skip_to_fde_directives(), None);
}

/// Writer 编码与解码器互逆
#[test]
fn leb_encode_decode_inverse() {
  let mut w = EhFrameWriter::new(EH_FRAME_X64);
  for v in [0u32, 1, 127, 128, 624485, u32::MAX] {
    let start = w.offset();
    w.write_uleb128(v);
    let (d, n) = decode_uleb128(&w.buffer()[start..]).unwrap();
    assert_eq!((d, n), (v, w.offset() - start), "uleb {v}");
  }
  for v in [0i32, 1, -1, 63, 64, -64, -65, -624485] {
    let start = w.offset();
    w.write_sleb128(v);
    let (d, n) = decode_sleb128(&w.buffer()[start..]).unwrap();
    assert_eq!((d, n), (v, w.offset() - start), "sleb {v}");
  }
}
