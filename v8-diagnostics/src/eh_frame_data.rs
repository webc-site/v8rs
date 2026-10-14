// 由 tool/src/diagnostics/eh-frame.js 从 v8/src/diagnostics/eh-frame.h;src/diagnostics/eh-frame.cc;src/diagnostics/x64/eh-frame-x64.cc;src/diagnostics/arm64/eh-frame-arm64.cc 生成; 修订就地改, 复跑覆盖

//! EH_FRAME 帧记录纯数据面 (基线生成; EH_FRAME 记录写出/读取行为见 eh_frame.rs)。
const _: () = assert!(size_of::<usize>() == 8);

// ── DwarfOpcodes (10 项) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DwarfOpcode {
  Nop = 0,
  AdvanceLoc1 = 2,
  AdvanceLoc2 = 3,
  AdvanceLoc4 = 4,
  RestoreExtended = 6,
  SameValue = 8,
  DefCfa = 12,
  DefCfaRegister = 13,
  DefCfaOffset = 14,
  OffsetExtendedSf = 17,
}

// ── DwarfEncodingSpecifiers (5 项) ──
pub const UDATA4: u8 = 3;
pub const SDATA4: u8 = 11;
pub const PC_REL: u8 = 16;
pub const DATA_REL: u8 = 48;
pub const OMIT: u8 = 255;

// ── 指令标签/掩码 ──
pub const LOCATION_TAG: i32 = 1;
pub const LOCATION_MASK: i32 = 63;
pub const LOCATION_MASK_SIZE: i32 = 6;
pub const SAVED_REGISTER_TAG: i32 = 2;
pub const SAVED_REGISTER_MASK: i32 = 63;
pub const SAVED_REGISTER_MASK_SIZE: i32 = 6;
pub const FOLLOW_INITIAL_RULE_TAG: i32 = 3;
pub const FOLLOW_INITIAL_RULE_MASK: i32 = 63;
pub const FOLLOW_INITIAL_RULE_MASK_SIZE: i32 = 6;

// ── 记录布局常量 (kInt32Size 编译期派生自 size_of::<i32>()) ──
pub const INT32_SIZE: usize = size_of::<i32>();
pub const PROCEDURE_ADDRESS_OFFSET_IN_FDE: usize = 2 * INT32_SIZE;
pub const PROCEDURE_SIZE_OFFSET_IN_FDE: usize = 3 * INT32_SIZE;
pub const EH_FRAME_TERMINATOR_SIZE: usize = 4;
pub const INITIAL_STATE_OFFSET_IN_CIE: usize = 19;
pub const FDE_VERSION_SIZE: usize = 1;
pub const FDE_ENCODING_SPECIFIERS_SIZE: usize = 3;
pub const EH_FRAME_HDR_VERSION: usize = 1;
pub const EH_FRAME_HDR_SIZE: usize = 20;

// ── FDE/CIE 头字段值 (eh-frame.cc WriteCie/WriteFdeHeader) ──
pub const INT32_PLACEHOLDER: u32 = 0xdeadc0de;
pub const CIE_IDENTIFIER: i32 = 0;
pub const CIE_VERSION: u8 = 3;
pub const AUGMENTATION_DATA_SIZE: u32 = 2;
pub const AUGMENTATION_STRING: [u8; 4] = [b'z', b'L', b'R', 0];
pub const FDE_PAD_ALIGNMENT: usize = size_of::<usize>();
// ── 架构参数 (代码对齐 × 数据对齐 × RA 伪寄存器 DWARF 码 × CIE 初始态) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EhFrameArchParams {
  pub code_align: i32,
  pub data_align: i32,
  pub ra_code: u32,
  pub cfa_reg_code: u32,
  pub cfa_offset: i32,
  pub saved_ra_offset: Option<i32>,
  pub same_value_reg: Option<u32>,
}

/// x64: def_cfa(rsp, 8); rip 保存于 -8
pub const EH_FRAME_X64: EhFrameArchParams = EhFrameArchParams {
  code_align: 1,
  data_align: -8,
  ra_code: 16,
  cfa_reg_code: 7,
  cfa_offset: size_of::<usize>() as i32,
  saved_ra_offset: Some(-(size_of::<usize>() as i32)),
  same_value_reg: None,
};

/// arm64: def_cfa(fp, 0); lr same_value
pub const EH_FRAME_ARM64: EhFrameArchParams = EhFrameArchParams {
  code_align: 4,
  data_align: -8,
  ra_code: 30,
  cfa_reg_code: 29,
  cfa_offset: 0,
  saved_ra_offset: None,
  same_value_reg: Some(30),
};
