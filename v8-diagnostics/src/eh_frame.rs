//! EH_FRAME 记录写出/读取行为面 (1:1 对应 v8/src/diagnostics/eh-frame.cc 平台无关逻辑)。
//!
//! 架构差异 (对齐因子/RA 伪寄存器码/CIE 初始态) 收敛为 `EhFrameArchParams` 入参,
//! 由 eh_frame_data.rs 基线生成; Label/RelocInfo/CodeDesc 回填面推迟 (阻塞: codegen-core)。

use crate::eh_frame_data::{
  AUGMENTATION_DATA_SIZE, AUGMENTATION_STRING, CIE_IDENTIFIER, CIE_VERSION, DATA_REL, DwarfOpcode,
  EH_FRAME_HDR_SIZE, EH_FRAME_HDR_VERSION, EH_FRAME_TERMINATOR_SIZE, EhFrameArchParams,
  FDE_ENCODING_SPECIFIERS_SIZE, FDE_PAD_ALIGNMENT, FDE_VERSION_SIZE, FOLLOW_INITIAL_RULE_MASK,
  FOLLOW_INITIAL_RULE_MASK_SIZE, FOLLOW_INITIAL_RULE_TAG, INITIAL_STATE_OFFSET_IN_CIE,
  INT32_PLACEHOLDER, INT32_SIZE, LOCATION_MASK, LOCATION_MASK_SIZE, LOCATION_TAG, OMIT, PC_REL,
  PROCEDURE_ADDRESS_OFFSET_IN_FDE, PROCEDURE_SIZE_OFFSET_IN_FDE, SAVED_REGISTER_MASK,
  SAVED_REGISTER_MASK_SIZE, SAVED_REGISTER_TAG, SDATA4, UDATA4,
};

/// 写出器内部状态 (对应上游 InternalState)
#[derive(Debug, PartialEq, Eq)]
enum State {
  Undefined,
  Initialized,
  Finalized,
}

/// 上游 RoundUp 语义: 向上取整到 align 的倍数
pub(crate) fn round_up(x: usize, align: usize) -> usize {
  x.div_ceil(align) * align
}

/// DWARF eh_frame 单例程记录写出器: CIE + FDE + .eh_frame_hdr, 小端字节序。
pub struct EhFrameWriter {
  arch: EhFrameArchParams,
  buf: Vec<u8>,
  cie_size: usize,
  last_pc_offset: i32,
  base_offset: i32,
  state: State,
}

impl EhFrameWriter {
  pub fn new(arch: EhFrameArchParams) -> Self {
    Self {
      arch,
      buf: Vec::with_capacity(128),
      cie_size: 0,
      last_pc_offset: 0,
      base_offset: 0,
      state: State::Undefined,
    }
  }

  /// 写出 CIE 与 FDE 头, 必须先于其他写调用
  pub fn initialize(&mut self) {
    debug_assert_eq!(self.state, State::Undefined);
    self.state = State::Initialized;
    self.write_cie();
    self.write_fde_header();
  }

  pub fn buffer(&self) -> &[u8] {
    &self.buf
  }

  pub fn offset(&self) -> usize {
    self.buf.len()
  }

  pub fn cie_size(&self) -> usize {
    self.cie_size
  }

  pub fn fde_offset(&self) -> usize {
    self.cie_size
  }

  pub fn last_pc_offset(&self) -> i32 {
    self.last_pc_offset
  }

  pub fn base_offset(&self) -> i32 {
    self.base_offset
  }

  pub fn procedure_address_offset(&self) -> usize {
    self.fde_offset() + PROCEDURE_ADDRESS_OFFSET_IN_FDE
  }

  pub fn procedure_size_offset(&self) -> usize {
    self.fde_offset() + PROCEDURE_SIZE_OFFSET_IN_FDE
  }

  fn write_byte(&mut self, v: u8) {
    self.buf.push(v);
  }

  fn write_opcode(&mut self, op: DwarfOpcode) {
    self.write_byte(op as u8);
  }

  fn write_u16(&mut self, v: u16) {
    self.buf.extend_from_slice(&v.to_le_bytes());
  }

  fn write_u32(&mut self, v: u32) {
    self.buf.extend_from_slice(&v.to_le_bytes());
  }

  fn patch_u32(&mut self, offset: usize, value: u32) {
    debug_assert!(offset + INT32_SIZE < self.buf.len());
    if let Some(target) = self.buf.get_mut(offset..offset + INT32_SIZE) {
      let mut old_bytes = [0u8; INT32_SIZE];
      old_bytes.copy_from_slice(target);
      let old = u32::from_le_bytes(old_bytes);
      debug_assert_eq!(old, INT32_PLACEHOLDER);
      target.copy_from_slice(&value.to_le_bytes());
    }
  }

  pub fn write_uleb128(&mut self, mut value: u32) {
    loop {
      let mut chunk = (value & 0x7F) as u8;
      value >>= 7;
      if value != 0 {
        chunk |= 0x80;
      }
      self.write_byte(chunk);
      if value == 0 {
        break;
      }
    }
  }

  pub fn write_sleb128(&mut self, mut value: i32) {
    loop {
      let chunk = (value & 0x7F) as u8;
      value >>= 7;
      let done = (value == 0 && (chunk & 0x40) == 0) || (value == -1 && (chunk & 0x40) != 0);
      self.write_byte(if done { chunk } else { chunk | 0x80 });
      if done {
        break;
      }
    }
  }

  fn write_cie(&mut self) {
    // CIE 尺寸占位, 记录布局定稿后回填
    let size_offset = self.offset();
    self.write_u32(INT32_PLACEHOLDER);

    let record_start = self.offset();
    self.write_u32(CIE_IDENTIFIER as u32);
    self.write_byte(CIE_VERSION);

    // 增强数据描述: LSDA 与 FDE 编码
    self.buf.extend_from_slice(&AUGMENTATION_STRING);

    self.write_sleb128(self.arch.code_align);
    self.write_sleb128(self.arch.data_align);
    self.write_uleb128(self.arch.ra_code);

    // 增强数据: 无 LSDA + FDE 指针编码
    self.write_uleb128(AUGMENTATION_DATA_SIZE);
    self.write_byte(OMIT);
    self.write_byte(SDATA4 | PC_REL);

    debug_assert_eq!(self.offset() - size_offset, INITIAL_STATE_OFFSET_IN_CIE);
    self.write_initial_state();

    self.write_padding_to_aligned_size(self.offset() - record_start);
    let encoded_cie_size = (self.offset() - record_start) as u32;
    self.cie_size = self.offset() - size_offset;
    self.patch_u32(size_offset, encoded_cie_size);
  }

  /// 架构初始态: x64 def_cfa(rsp)+保存 RA / arm64 def_cfa(fp)+lr same_value
  fn write_initial_state(&mut self) {
    self.set_base_address_register_and_offset(self.arch.cfa_reg_code, self.arch.cfa_offset);
    if let Some(offset) = self.arch.saved_ra_offset {
      self.record_register_saved_to_stack(self.arch.ra_code, offset);
    }
    if let Some(code) = self.arch.same_value_reg {
      self.record_register_not_modified(code);
    }
  }

  fn write_fde_header(&mut self) {
    debug_assert_ne!(self.cie_size, 0);
    debug_assert_eq!(self.offset(), self.fde_offset());
    self.write_u32(INT32_PLACEHOLDER);
    // 回指 CIE
    self.write_u32((self.cie_size + INT32_SIZE) as u32);
    debug_assert_eq!(self.offset(), self.procedure_address_offset());
    self.write_u32(INT32_PLACEHOLDER);
    debug_assert_eq!(self.offset(), self.procedure_size_offset());
    self.write_u32(INT32_PLACEHOLDER);
    // 无增强数据
    self.write_byte(0);
  }

  /// pc 位置推进: 紧凑 advance_loc 标签优先, 退化到 1/2/4 字节形式
  pub fn advance_location(&mut self, pc_offset: i32) {
    debug_assert_eq!(self.state, State::Initialized);
    debug_assert!(pc_offset >= self.last_pc_offset);
    debug_assert_eq!((pc_offset - self.last_pc_offset) % self.arch.code_align, 0);
    let factored = ((pc_offset - self.last_pc_offset) / self.arch.code_align) as u32;

    if factored <= LOCATION_MASK as u32 {
      self.write_byte(
        ((LOCATION_TAG << LOCATION_MASK_SIZE) as u32 | (factored & LOCATION_MASK as u32)) as u8,
      );
    } else if factored <= u8::MAX as u32 {
      self.write_opcode(DwarfOpcode::AdvanceLoc1);
      self.write_byte(factored as u8);
    } else if factored <= u16::MAX as u32 {
      self.write_opcode(DwarfOpcode::AdvanceLoc2);
      self.write_u16(factored as u16);
    } else {
      self.write_opcode(DwarfOpcode::AdvanceLoc4);
      self.write_u32(factored);
    }
    self.last_pc_offset = pc_offset;
  }

  pub fn set_base_address_offset(&mut self, offset: i32) {
    debug_assert_eq!(self.state, State::Initialized);
    debug_assert!(offset >= 0);
    self.write_opcode(DwarfOpcode::DefCfaOffset);
    self.write_uleb128(offset as u32);
    self.base_offset = offset;
  }

  pub fn increase_base_address_offset(&mut self, delta: i32) {
    let offset = self.base_offset + delta;
    self.set_base_address_offset(offset);
  }

  pub fn set_base_address_register(&mut self, dwarf_code: u32) {
    debug_assert_eq!(self.state, State::Initialized);
    self.write_opcode(DwarfOpcode::DefCfaRegister);
    self.write_uleb128(dwarf_code);
  }

  pub fn set_base_address_register_and_offset(&mut self, dwarf_code: u32, offset: i32) {
    debug_assert_eq!(self.state, State::Initialized);
    debug_assert!(offset >= 0);
    self.write_opcode(DwarfOpcode::DefCfa);
    self.write_uleb128(dwarf_code);
    self.write_uleb128(offset as u32);
    self.base_offset = offset;
  }

  /// 寄存器保存于 base+offset; 负偏移退化到 offset_extended_sf
  pub fn record_register_saved_to_stack(&mut self, dwarf_code: u32, offset: i32) {
    debug_assert_eq!(self.state, State::Initialized);
    debug_assert_eq!(offset % self.arch.data_align, 0);
    let factored = offset / self.arch.data_align;
    if factored >= 0 {
      debug_assert!(dwarf_code <= SAVED_REGISTER_MASK as u32);
      self.write_byte(
        ((SAVED_REGISTER_TAG << SAVED_REGISTER_MASK_SIZE) as u32
          | (dwarf_code & SAVED_REGISTER_MASK as u32)) as u8,
      );
      self.write_uleb128(factored as u32);
    } else {
      self.write_opcode(DwarfOpcode::OffsetExtendedSf);
      self.write_uleb128(dwarf_code);
      self.write_sleb128(factored);
    }
  }

  pub fn record_register_not_modified(&mut self, dwarf_code: u32) {
    debug_assert_eq!(self.state, State::Initialized);
    self.write_opcode(DwarfOpcode::SameValue);
    self.write_uleb128(dwarf_code);
  }

  pub fn record_register_follows_initial_rule(&mut self, dwarf_code: u32) {
    debug_assert_eq!(self.state, State::Initialized);
    if dwarf_code <= FOLLOW_INITIAL_RULE_MASK as u32 {
      self.write_byte(
        ((FOLLOW_INITIAL_RULE_TAG << FOLLOW_INITIAL_RULE_MASK_SIZE) as u32
          | (dwarf_code & FOLLOW_INITIAL_RULE_MASK as u32)) as u8,
      );
    } else {
      self.write_opcode(DwarfOpcode::RestoreExtended);
      self.write_uleb128(dwarf_code);
    }
  }

  fn write_padding_to_aligned_size(&mut self, unpadded: usize) {
    debug_assert_eq!(self.state, State::Initialized);
    let padding = round_up(unpadded, FDE_PAD_ALIGNMENT) - unpadded;
    debug_assert!(padding <= 8);
    self
      .buf
      .resize(self.buf.len() + padding, DwarfOpcode::Nop as u8);
  }

  /// 收尾: 补齐 FDE、回填尺寸/地址、终止符、.eh_frame_hdr
  pub fn finish(&mut self, code_size: i32) {
    debug_assert_eq!(self.state, State::Initialized);
    debug_assert!(self.offset() >= self.cie_size);
    debug_assert!(self.offset() >= self.fde_offset() + INT32_SIZE);
    self.write_padding_to_aligned_size(self.offset() - self.fde_offset() - INT32_SIZE);

    let encoded_fde_size = (self.offset() - self.fde_offset() - INT32_SIZE) as u32;
    self.patch_u32(self.fde_offset(), encoded_fde_size);

    let addr = self.procedure_address_offset();
    self.patch_u32(
      addr,
      (-(round_up(code_size as usize, 8) as i32 + addr as i32)) as u32,
    );
    let size_off = self.procedure_size_offset();
    self.patch_u32(size_off, code_size as u32);

    // 终止 .eh_frame
    self
      .buf
      .resize(self.buf.len() + EH_FRAME_TERMINATOR_SIZE, 0);

    self.write_eh_frame_hdr(code_size);
    self.state = State::Finalized;
  }

  fn write_eh_frame_hdr(&mut self, code_size: i32) {
    debug_assert_eq!(self.state, State::Initialized);
    let eh_frame_size = self.offset();

    self.write_byte(EH_FRAME_HDR_VERSION as u8);
    self.write_byte(SDATA4 | PC_REL);
    self.write_byte(UDATA4);
    self.write_byte(SDATA4 | DATA_REL);

    // 指向 .eh_frame (相对本处)
    self.write_u32(
      (-(eh_frame_size as i32 + FDE_VERSION_SIZE as i32 + FDE_ENCODING_SPECIFIERS_SIZE as i32))
        as u32,
    );
    // LUT 表项数: 唯一例程 1 条
    self.write_u32(1);
    // 例程起始指针 (相对 .eh_frame_hdr)
    self.write_u32((-(round_up(code_size as usize, 8) as i32 + eh_frame_size as i32)) as u32);
    // 关联 FDE 起始指针 (相对 .eh_frame_hdr)
    self.write_u32((-(eh_frame_size as i32 - self.cie_size as i32)) as u32);

    debug_assert_eq!(self.offset() - eh_frame_size, EH_FRAME_HDR_SIZE);
  }

  /// 空 .eh_frame_hdr: 触发 Linux perf libunwind 的 fp 栈回溯 hack
  pub fn write_empty_eh_frame(out: &mut Vec<u8>) {
    out.push(EH_FRAME_HDR_VERSION as u8);
    out.push(SDATA4 | PC_REL);
    out.push(UDATA4);
    out.push(SDATA4 | DATA_REL);
    out.resize(out.len() + EH_FRAME_HDR_SIZE - 4, 0);
  }
}

/// eh_frame 缓冲只读迭代器 (对应上游 EhFrameIterator)
pub struct EhFrameIterator<'a> {
  buf: &'a [u8],
  next: usize,
}

impl<'a> EhFrameIterator<'a> {
  pub fn new(buf: &'a [u8]) -> Self {
    Self { buf, next: 0 }
  }

  pub fn done(&self) -> bool {
    self.next == self.buf.len()
  }

  pub fn current_offset(&self) -> usize {
    self.next
  }

  pub fn buffer_size(&self) -> usize {
    self.buf.len()
  }

  /// 跳过一个 CIE 记录 (尺寸字段 + 记录体), 停在 FDE 起点。
  /// C++ 锚 eh-frame.h:220 SkipCie: ReadUnalignedValue 不推进指针, 单步 `next_ += u32 + kInt32Size`;
  /// 加法在 uint32 域内回绕 (如尺寸 0xFFFFFFFC 步进 0), 此处以 wrapping_add 镜像;
  /// DCHECK_EQ(next_, start_) 由 debug_assert 承接, 仅允许在缓冲起点调用。
  pub fn skip_cie(&mut self) -> Option<()> {
    debug_assert_eq!(self.next, 0);
    let slice = self.buf.get(self.next..self.next + INT32_SIZE)?;
    let mut arr = [0u8; INT32_SIZE];
    arr.copy_from_slice(slice);
    let size = u32::from_le_bytes(arr);
    let step = size.wrapping_add(INT32_SIZE as u32) as usize;
    if self.next.checked_add(step)? > self.buf.len() {
      return None;
    }
    self.skip(step);
    Some(())
  }

  /// 停在 FDE 指令区起点: CIE 之后跳过 FDE 的 4 个 u32 与编码字节
  pub fn skip_to_fde_directives(&mut self) -> Option<()> {
    self.skip_cie()?;
    let directives_offset = 4 * INT32_SIZE + 1;
    if self.next.checked_add(directives_offset)? > self.buf.len() {
      return None;
    }
    self.skip(directives_offset);
    Some(())
  }

  pub fn skip(&mut self, how_many: usize) {
    self.next += how_many;
    debug_assert!(self.next <= self.buf.len());
  }

  pub fn next_byte(&mut self) -> Option<u8> {
    let v = self.buf.get(self.next).copied()?;
    self.next += 1;
    Some(v)
  }

  pub fn next_u16(&mut self) -> Option<u16> {
    let s = self.take(2)?;
    let mut arr = [0u8; 2];
    arr.copy_from_slice(s);
    Some(u16::from_le_bytes(arr))
  }

  pub fn next_u32(&mut self) -> Option<u32> {
    let s = self.take(4)?;
    let mut arr = [0u8; 4];
    arr.copy_from_slice(s);
    Some(u32::from_le_bytes(arr))
  }

  pub fn next_uleb128(&mut self) -> Option<u32> {
    let (v, size) = decode_uleb128(&self.buf[self.next..])?;
    self.skip(size);
    Some(v)
  }

  pub fn next_sleb128(&mut self) -> Option<i32> {
    let (v, size) = decode_sleb128(&self.buf[self.next..])?;
    self.skip(size);
    Some(v)
  }

  fn take(&mut self, n: usize) -> Option<&'a [u8]> {
    let s = self.buf.get(self.next..self.next + n)?;
    self.next += n;
    Some(s)
  }
}

/// ULEB128 解码, 返回 (值, 消耗字节数); 截断或溢出返回 None
pub fn decode_uleb128(encoded: &[u8]) -> Option<(u32, usize)> {
  let (mut result, mut shift) = (0u32, 0u32);
  for (i, &chunk) in encoded.iter().enumerate() {
    if shift >= 35 {
      return None;
    }
    let low7 = (chunk & 0x7F) as u32;
    if shift >= 28 && (low7 >> (32 - shift)) != 0 {
      return None;
    }
    if shift < 32 {
      result |= low7 << shift;
    }
    shift += 7;
    if chunk < 128 {
      return Some((result, i + 1));
    }
  }
  None
}

/// SLEB128 解码, 返回 (值, 消耗字节数); 截断或溢出返回 None
pub fn decode_sleb128(encoded: &[u8]) -> Option<(i32, usize)> {
  let (mut result, mut shift) = (0u32, 0u32);
  for (i, &chunk) in encoded.iter().enumerate() {
    if shift >= 35 {
      return None;
    }
    let low7 = (chunk & 0x7F) as u32;
    if shift < 32 {
      result |= low7 << shift;
    }
    shift += 7;
    if chunk < 128 {
      // 末字节符号位为 1 时向高位扩展
      if chunk & 0x40 != 0 && shift < 32 {
        result |= !0u32 << shift;
      }
      return Some((result as i32, i + 1));
    }
  }
  None
}
