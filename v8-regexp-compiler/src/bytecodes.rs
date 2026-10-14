//! 正则字节码布局与编解码基座 (v8/src/regexp/regexp-bytecodes-inl.h 的算法面 +
//! regexp-bytecodes.h 的 Bytecodes 纯函数面)。
//!
//! 表数据 (opcode 61 项 / 操作数类型 11 项 / flag 位 / 操作数名与类型 / 尺寸金标准)
//! 由 tool/src/regexp/regexp-bytecodes.h.js 从原版四表机械生成于 bytecodes_generated.rs;
//! 本文件承接 inl.h:107-152 的 4 字节块对齐排布算法 (编译期) 与 Get/Set 编解码工具。
//!
//! 端序登记: 原版 Get 为 host 端序 reinterpret_cast (inl.h:250-252), 本仓目标
//! x64/arm64 均为小端 → 统一 LE; 大端平台批 (s390 等) 不在 wave1。
//!
//! 同源性: v8-regexp-exec/src/regexp_interpreter.rs 的 Bytecode 枚举 /
//! BYTECODE_LAYOUTS / OperandType 宽度表与本表逐项交叉对数 (脚本 + 单测双通道)。

use crate::bytecodes_generated::{
  ALL_BYTECODES, BYTECODE_FLAGS, BYTECODE_NAMES, BYTECODE_OPERAND_NAMES, BYTECODE_OPERAND_TYPES,
  Bytecode, BytecodeFlag, BytecodeOperandType, GOLDEN_SIZES, K_BYTECODE_ALIGNMENT,
  K_BYTECODE_COUNT, K_MAX_OPERAND_COUNT, OPERAND_TYPE_ALIGNMENTS, OPERAND_TYPE_IS_BASIC,
  OPERAND_TYPE_LIMITS, OPERAND_TYPE_NAMES, OPERAND_TYPE_SIZES,
};

const _: () = assert!(K_BYTECODE_ALIGNMENT == 4);

/// 原版 detail::BytecodeOperandsTraits (inl.h:138-152) 的单条布局数据面:
/// 操作数帧内偏移表 (kOperandOffsets) + 指令总宽 (kSize)。类型/名字表在生成侧
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BytecodeLayout {
  /// kOperandOffsets (inl.h:147), 相对 pc 的字节偏移; 实测 ≤ 88, u8 充分
  pub offsets: [u8; K_MAX_OPERAND_COUNT],
  /// kSize (inl.h:149-151): RoundUp<4>(末操作数偏移 + 宽度), 零操作数 = 4
  pub total_size: u8,
}

/// 原版 base::RoundUp (inl.h:123/128 消费)
const fn round_up(value: usize, align: usize) -> usize {
  value.div_ceil(align) * align
}

/// 操作数类型宽度 (OperandTypeTraits::kSize, inl.h:30/45/63)
const fn operand_size_of(type_: BytecodeOperandType) -> usize {
  OPERAND_TYPE_SIZES[type_ as usize] as usize
}

/// 操作数类型对齐 (kAlignment, inl.h:35/54/65: basic=自身宽, BitTable=1)
const fn operand_align_of(type_: BytecodeOperandType) -> usize {
  OPERAND_TYPE_ALIGNMENTS[type_ as usize] as usize
}

/// 原版 detail::CalculateAlignedOffsets (inl.h:107-136) + kSize (inl.h:149-151)
/// 的编译期 1:1: 逐操作数按自身对齐进位, 当前 4 字节块放不下则另起一块
const fn calculate_layout(types: &[BytecodeOperandType]) -> BytecodeLayout {
  // 原版数组按 kOperandCount 定长 (std::array<int, N>), Rust 以固定 u8 帧承接:
  // 操作数个数与偏移均须在限幅域内, 越界即编译失败而非静默截断
  assert!(
    types.len() <= K_MAX_OPERAND_COUNT,
    "操作数个数超出 kMaxOperandCount 限幅"
  );
  let mut offsets = [0u8; K_MAX_OPERAND_COUNT];
  // first_offset = sizeof(Bytecode) (inl.h:116-117), opcode 枚举为 u8
  let mut offset = 1usize;
  let mut i = 0;
  while i < types.len() {
    let size = operand_size_of(types[i]);
    let alignment = operand_align_of(types[i]);
    offset = round_up(offset, alignment); // inl.h:123
    if offset % K_BYTECODE_ALIGNMENT + size > K_BYTECODE_ALIGNMENT {
      // inl.h:125-129: 块内放不下 → 进位到下一 4 字节块
      offset = round_up(offset, K_BYTECODE_ALIGNMENT);
    }
    assert!(offset <= u8::MAX as usize, "操作数偏移超出 u8 承接域");
    offsets[i] = offset as u8;
    offset += size;
    i += 1;
  }
  let region_end = if types.is_empty() {
    1
  } else {
    offsets[i - 1] as usize + operand_size_of(types[i - 1])
  };
  let total = round_up(region_end, K_BYTECODE_ALIGNMENT);
  assert!(
    total <= u8::MAX as usize,
    "指令总宽超出 u8 承接域 (原版 kBytecodeSizes 同为 u8)"
  );
  BytecodeLayout {
    offsets,
    total_size: total as u8,
  }
}

// 排布算法合成对数 (round2 对抗复审): 非 GOLDEN 表内的自造操作数序列, 偏移为
// 按 inl.h:119-133 逐行人手算的值 (表内无 8 字节类型, 跨块场景由 BitTable 16B 超集覆盖)
const _: () = {
  // 零操作数: kSize = RoundUp<4>(sizeof(Bytecode)) = 4 (inl.h:149-151 true 支)
  assert!(calculate_layout(&[]).total_size == 4);
  // [Char, BitTable, Int16] (宽操作数已落块首不再进位):
  //   1→RoundUp₂=2 [off 2, →4]; BitTable align1 不动, 4%4+16>4 → RoundUp₄(4)=4
  //   [off 4, →20]; Int16 20%4+2≤4 [off 20, →22]; 总宽 RoundUp₄(22)=24
  let wide = calculate_layout(&[
    BytecodeOperandType::Char,
    BytecodeOperandType::BitTable,
    BytecodeOperandType::Int16,
  ]);
  assert!(wide.offsets[0] == 2 && wide.offsets[1] == 4 && wide.offsets[2] == 20);
  assert!(wide.total_size == 24);
  // [StackCheckFlag, BitTable, Uint32, Char] (块中 BitTable 进位 + 4B 对齐进位复合):
  //   1%4+1≤4 [off 1, →2]; BitTable 2%4+16>4 → RoundUp₄(2)=4 [off 4, →20];
  //   Uint32 RoundUp₄(20)=20, 20%4+4≤4 [off 20, →24]; Char 24%4+2≤4 [off 24, →26];
  //   总宽 RoundUp₄(26)=28
  let mixed = calculate_layout(&[
    BytecodeOperandType::StackCheckFlag,
    BytecodeOperandType::BitTable,
    BytecodeOperandType::Uint32,
    BytecodeOperandType::Char,
  ]);
  assert!(
    mixed.offsets[0] == 1
      && mixed.offsets[1] == 4
      && mixed.offsets[2] == 20
      && mixed.offsets[3] == 24
  );
  assert!(mixed.total_size == 28);
};

/// 61 项布局表 ≡ inl.h:312-316 kBytecodeSizes 与各 BytecodeOperands<bc> 特化
/// (inl.h:292-303) — 编译期由操作数类型序列重算, 并与脚本独立复刻的
/// GOLDEN_SIZES 逐项 assert (双实现交叉验证)
const LAYOUTS: [BytecodeLayout; K_BYTECODE_COUNT as usize] = {
  let mut out = [BytecodeLayout {
    offsets: [0; K_MAX_OPERAND_COUNT],
    total_size: 0,
  }; K_BYTECODE_COUNT as usize];
  let mut i = 0;
  while i < K_BYTECODE_COUNT as usize {
    out[i] = calculate_layout(BYTECODE_OPERAND_TYPES[i]);
    i += 1;
  }
  out
};

// 编译期金标准对数: 布局算法结果 ≡ 脚本 JS 独立实现的尺寸基线 (61/61)
const _: () = {
  let mut i = 0;
  while i < K_BYTECODE_COUNT as usize {
    assert!(
      LAYOUTS[i].total_size == GOLDEN_SIZES[i],
      "布局尺寸与脚本基线漂移"
    );
    // 排布不变式 (inl.h:105-106 注释): 操作数按自身对齐 + 不跨 4 字节块
    let layout = &LAYOUTS[i];
    let types = BYTECODE_OPERAND_TYPES[i];
    let mut j = 0;
    while j < types.len() {
      let off = layout.offsets[j] as usize;
      let size = operand_size_of(types[j]);
      assert!(
        off.is_multiple_of(operand_align_of(types[j])),
        "操作数未按自身对齐"
      );
      if size <= K_BYTECODE_ALIGNMENT {
        // 不跨 4 字节块 (inl.h:125-129 进位规则的直接推论)
        assert!(
          off % K_BYTECODE_ALIGNMENT + size <= K_BYTECODE_ALIGNMENT,
          "操作数跨 4 字节块"
        );
      } else {
        // 唯一 >4 字节宽的类型是 BitTable (16 字节, 对齐 1): 进位后落块首、
        // 有意跨 4 个块 (原版单次进位不循环, 同款行为)
        assert!(off.is_multiple_of(K_BYTECODE_ALIGNMENT), "宽操作数未落块首");
      }
      j += 1;
    }
    i += 1;
  }
};

impl Bytecode {
  /// 原版 Bytecodes::kCount (h:355) — const 通道
  #[must_use]
  pub const fn count() -> u8 {
    K_BYTECODE_COUNT
  }

  /// 原版 Bytecodes::IsValid (h:371)
  #[must_use]
  pub const fn is_valid(byte: u8) -> bool {
    byte < K_BYTECODE_COUNT
  }

  /// 原版 Bytecodes::FromByte (h:359-362); DCHECK(IsValid) 以 Option 承接
  #[must_use]
  pub const fn from_byte(byte: u8) -> Option<Bytecode> {
    if Self::is_valid(byte) {
      // 生成侧 #[repr(u8)] 全集连续 0..61, 合法值必落在 ALL_BYTECODES 像上
      Some(ALL_BYTECODES[byte as usize])
    } else {
      None
    }
  }

  /// 原版 Bytecodes::ToByte (h:356-358)
  #[must_use]
  pub const fn to_byte(self) -> u8 {
    self as u8
  }

  /// 原版 Bytecodes::IsValidJumpTarget (h:372-374) — kBreak(0) 非合法跳转目标
  #[must_use]
  pub const fn is_valid_jump_target(byte: u8) -> bool {
    Self::is_valid(byte) && byte != Self::Break as u8
  }

  /// 原版 Bytecodes::Name (h:384-385 ≡ inl.h:346-354 kBytecodeNames)
  #[must_use]
  pub const fn name(self) -> &'static str {
    BYTECODE_NAMES[self as usize]
  }

  /// 原版 Bytecodes::Size (h:387-388 ≡ inl.h:312-316/357-365)
  #[must_use]
  pub const fn size(self) -> u8 {
    LAYOUTS[self as usize].total_size
  }

  /// 原版 Bytecodes::Flags (h:391-392 ≡ inl.h:318-322/373-381)
  #[must_use]
  pub const fn flags(self) -> u32 {
    BYTECODE_FLAGS[self as usize]
  }

  /// 原版 BytecodeFlags 的 HasFlag (base::Flags) — 分析面 (h:58-81)
  #[must_use]
  pub const fn has_flag(self, flag: BytecodeFlag) -> bool {
    self.flags() & flag as u32 != 0
  }

  /// 原版 BytecodeOperands<bc>::kCount (inl.h:177)
  #[must_use]
  pub const fn operand_count(self) -> usize {
    BYTECODE_OPERAND_TYPES[self as usize].len()
  }

  /// 原版 BytecodeOperands<bc>::Offset(op) (inl.h:183-185), 相对 pc 字节偏移
  #[must_use]
  pub const fn operand_offset(self, op: usize) -> usize {
    LAYOUTS[self as usize].offsets[op] as usize
  }

  /// 原版 BytecodeOperands<bc>::Size(op) (inl.h:180-182)
  #[must_use]
  pub const fn operand_size(self, op: usize) -> usize {
    operand_size_of(self.operand_type(op))
  }

  /// 原版 BytecodeOperands<bc>::Type(op) (inl.h:186-188)
  #[must_use]
  pub const fn operand_type(self, op: usize) -> BytecodeOperandType {
    BYTECODE_OPERAND_TYPES[self as usize][op]
  }

  /// 原版 BytecodeOperands<bc>::Name(op) (inl.h:190-192, SplitNames 面)
  #[must_use]
  pub const fn operand_name(self, op: usize) -> &'static str {
    BYTECODE_OPERAND_NAMES[self as usize][op]
  }
}

/// 原版 Bytecodes::Size(BytecodeOperandType) (h:389 ≡ inl.h:324-328/368-370)
#[must_use]
pub const fn operand_type_size(type_: BytecodeOperandType) -> u8 {
  OPERAND_TYPE_SIZES[type_ as usize]
}

/// 原版 OperandTypeTraits::kMinValue/kMaxValue (inl.h:33-34/51-52) —
/// BitTable 无 C 型限幅, (0,0) 占位不参与校验
#[must_use]
pub const fn operand_type_limits(type_: BytecodeOperandType) -> (i64, i64) {
  OPERAND_TYPE_LIMITS[type_ as usize]
}

/// 原版 OperandTypeTraits::kIsBasic (inl.h:32/53/64) — false 仅 BitTable
#[must_use]
pub const fn operand_type_is_basic(type_: BytecodeOperandType) -> bool {
  OPERAND_TYPE_IS_BASIC[type_ as usize]
}

/// 原版操作数类型名 (#Name 形态, 生成台账可读性面)
#[must_use]
pub const fn operand_type_name(type_: BytecodeOperandType) -> &'static str {
  OPERAND_TYPE_NAMES[type_ as usize]
}

// ── 字节编码工具: 操作数定宽 LE 读写 (Get<op>/Set<op> 面, inl.h:244-285) ────

/// 取定宽小端字节串; 越界返回 None (原版 DCHECK 对齐 + 裸读, Rust 以边界承接)
fn read_le<const N: usize>(code: &[u8], at: usize) -> Option<[u8; N]> {
  let end = at.checked_add(N)?;
  code.get(at..end).map(|s| s.try_into().unwrap_or([0; N]))
}

fn write_le<const N: usize>(code: &mut [u8], at: usize, bytes: [u8; N]) -> bool {
  let end = match at.checked_add(N) {
    Some(e) => e,
    None => return false,
  };
  match code.get_mut(at..end) {
    Some(s) => {
      s.copy_from_slice(&bytes);
      true
    }
    None => false,
  }
}

/// 操作数解码值 (BitTable 为 16 字节切片借用, ≡ inl.h:265-273 指针分支)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand<'a> {
  Int16(i16),
  Int32(i32),
  Uint32(u32),
  Char(u16),
  JumpTarget(u32),
  Offset(i16),
  BoundsCheckOffset(i32),
  Register(u16),
  /// 原版 enum : uint8_t 底层位模式 (语义映射在调用方)
  StackCheckFlag(u8),
  /// 原版 enum : char 底层位模式 (转义字母表)
  StandardCharacterSet(u8),
  BitTable(&'a [u8]),
}

impl<'a> Operand<'a> {
  /// 按类型重组为 Operand (编码/解码统一入口)
  #[must_use]
  pub fn of(type_: BytecodeOperandType, code: &'a [u8], at: usize) -> Option<Operand<'a>> {
    match type_ {
      BytecodeOperandType::Int16 => {
        read_le::<2>(code, at).map(|b| Self::Int16(i16::from_le_bytes(b)))
      }
      BytecodeOperandType::Int32 => {
        read_le::<4>(code, at).map(|b| Self::Int32(i32::from_le_bytes(b)))
      }
      BytecodeOperandType::Uint32 => {
        read_le::<4>(code, at).map(|b| Self::Uint32(u32::from_le_bytes(b)))
      }
      BytecodeOperandType::Char => {
        read_le::<2>(code, at).map(|b| Self::Char(u16::from_le_bytes(b)))
      }
      BytecodeOperandType::JumpTarget => {
        read_le::<4>(code, at).map(|b| Self::JumpTarget(u32::from_le_bytes(b)))
      }
      BytecodeOperandType::Offset => {
        read_le::<2>(code, at).map(|b| Self::Offset(i16::from_le_bytes(b)))
      }
      BytecodeOperandType::BoundsCheckOffset => {
        read_le::<4>(code, at).map(|b| Self::BoundsCheckOffset(i32::from_le_bytes(b)))
      }
      BytecodeOperandType::Register => {
        read_le::<2>(code, at).map(|b| Self::Register(u16::from_le_bytes(b)))
      }
      BytecodeOperandType::StackCheckFlag => {
        read_le::<1>(code, at).map(|b| Self::StackCheckFlag(b[0]))
      }
      BytecodeOperandType::StandardCharacterSet => {
        read_le::<1>(code, at).map(|b| Self::StandardCharacterSet(b[0]))
      }
      BytecodeOperandType::BitTable => at
        .checked_add(16)
        .and_then(|end| code.get(at..end))
        .map(Self::BitTable),
    }
  }

  /// 写回帧内位置; 宽度与 [Operand] 变体一致 (1:1 Set 面, 供二梯队 RegExpAssembler 消费)
  #[must_use]
  pub fn write(&self, code: &mut [u8], at: usize) -> bool {
    match self {
      Self::Int16(v) => write_le(code, at, (*v).to_le_bytes()),
      Self::Int32(v) | Self::BoundsCheckOffset(v) => write_le(code, at, (*v).to_le_bytes()),
      Self::Uint32(v) | Self::JumpTarget(v) => write_le(code, at, (*v).to_le_bytes()),
      Self::Char(v) | Self::Register(v) => write_le(code, at, (*v).to_le_bytes()),
      Self::Offset(v) => write_le(code, at, (*v).to_le_bytes()),
      Self::StackCheckFlag(v) | Self::StandardCharacterSet(v) => {
        write_le(code, at, (*v).to_le_bytes())
      }
      Self::BitTable(table) => {
        let end = match at.checked_add(16) {
          Some(e) => e,
          None => return false,
        };
        match code.get_mut(at..end) {
          Some(s) if table.len() == 16 => {
            s.copy_from_slice(table);
            true
          }
          _ => false,
        }
      }
    }
  }
}

impl Bytecode {
  /// 读第 op 个操作数 (原版 handler 侧 Get<op>(pc, no_gc) 的边界安全形态)
  #[must_use]
  pub fn read_operand<'a>(&self, code: &'a [u8], pc: usize, op: usize) -> Option<Operand<'a>> {
    debug_assert_eq!(
      pc % K_BYTECODE_ALIGNMENT,
      0,
      "pc 须 4 字节对齐 (h:367 DCHECK)"
    );
    let type_ = BYTECODE_OPERAND_TYPES[*self as usize].get(op).copied()?;
    let at = pc + self.operand_offset(op);
    Operand::of(type_, code, at)
  }

  /// 写第 op 个操作数 (BytecodeOperandSetter 面)
  #[must_use]
  pub fn write_operand(&self, code: &mut [u8], pc: usize, op: usize, value: &Operand<'_>) -> bool {
    debug_assert_eq!(pc % K_BYTECODE_ALIGNMENT, 0);
    if BYTECODE_OPERAND_TYPES[*self as usize].get(op).is_none() {
      return false;
    }
    value.write(code, pc + self.operand_offset(op))
  }
}

/// 原版 Bytecodes::FromPtr (h:365-371): pc 处取 opcode; 越界/非法 → None
#[must_use]
pub fn opcode_at(code: &[u8], pc: usize) -> Option<Bytecode> {
  Bytecode::from_byte(*code.get(pc)?)
}
