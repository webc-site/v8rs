//! 实验性正则引擎字节码 IR (v8/src/regexp/experimental/experimental-bytecode.h + .cc)。
//!
//! 背景 (原版 h:13-91 头注释, 中文摘要):
//! - 线性时间 (无回溯) 正则引擎的字节码层, 参照 Russ Cox regexp2 与 Google re2。
//! - 字节码描述一个非确定有限自动机 (NFA), 在"多线程"VM 上并发运行: 每个线程持有
//!   程序计数器 (PC, 指令下标)、若干输入串索引寄存器和单调递增的当前输入位置 (CP)。
//! - 指令编码定长: opcode 4 字节 + payload 4 字节 (原版 h:97-99, static_assert
//!   sizeof(Instruction)==8, h:322)。Rust 侧用 tagged Payload 枚举重建判别联合,
//!   位域语义 (LookaroundPayload) 与 C++ 逐 bit 一致。
//! - 线程优先级语义 (原版 h:61-91): 多解时输出必须与回溯实现 (Irregexp) 一致;
//!   FORK 派生线程 t0 优先级低于 t 且与其他线程的相对次序同 t。
//!
//! 本模块只覆盖 bytecode IR 与 Dump 文本层; 编译器 (experimental-compiler.*) 与
//! 解释器 (experimental-interpreter.*) 属后续席位。
//!
//! 推迟登记 (见模块尾 DOC 表):
//! - Instruction ↔ 8 字节裸编码互转 (VectorToByteArray/ToInstructionVector) —
//!   依据 v8/src/regexp/experimental/experimental.cc:108,155 + 本文件 (无裸编码层)

use std::fmt;

use v8_base::bit_field::BitField32;
use v8_regexp_syntax::{AssertionType, LookaroundType};

// ── Opcode 枚举 ─────────────────────────────────────────────────────────────

/// 指令操作码 (v8: experimental-bytecode.h:101-121 `Instruction::Opcode : int32_t`,
/// 序数逐值 1:1, 共 19 个)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Opcode {
  /// 接受: 停止本线程, 标记当前位置匹配成功 (h:102)
  Accept = 0,
  /// 零宽断言 (h:103)
  Assertion = 1,
  /// 清零寄存器 (复位为初值 -1) (h:104)
  ClearRegister = 2,
  /// 检查当前码点是否落在闭区间 [min, max] (h:105)
  ConsumeRange = 3,
  /// 检查当前字符能否被后续 n 个 CONSUME_RANGE 之一接受 (h:106)
  RangeCount = 4,
  /// 派生新线程, 其 PC 为 payload (h:107)
  Fork = 5,
  /// 跳转: PC 置为 payload (h:108)
  Jmp = 6,
  /// 将寄存器置为当前输入位置 CP (h:109)
  SetRegisterToCp = 7,
  /// 量词计时起点 (h:110)
  SetQuantifierToClock = 8,
  /// 量词捕获过滤 (h:111)
  FilterQuantifier = 9,
  /// 分组捕获过滤 (h:112)
  FilterGroup = 10,
  /// 环视捕获过滤 (h:113)
  FilterLookaround = 11,
  /// 子自动机捕获过滤 (h:114)
  FilterChild = 12,
  /// 循环开始标记 (h:115)
  BeginLoop = 13,
  /// 循环结束标记 (h:116)
  EndLoop = 14,
  /// 环视体开始 (h:117)
  StartLookaround = 15,
  /// 环视体结束 (h:118)
  EndLookaround = 16,
  /// 写环视结果表 (h:119)
  WriteLookaroundTable = 17,
  /// 读环视结果表 (h:120)
  ReadLookaroundTable = 18,
}

impl Opcode {
  // C++ 枚举名 (UPPER_SNAKE) 常量别名, 保持原版可 grep
  pub const ACCEPT: Self = Self::Accept;
  pub const ASSERTION: Self = Self::Assertion;
  pub const CLEAR_REGISTER: Self = Self::ClearRegister;
  pub const CONSUME_RANGE: Self = Self::ConsumeRange;
  pub const RANGE_COUNT: Self = Self::RangeCount;
  pub const FORK: Self = Self::Fork;
  pub const JMP: Self = Self::Jmp;
  pub const SET_REGISTER_TO_CP: Self = Self::SetRegisterToCp;
  pub const SET_QUANTIFIER_TO_CLOCK: Self = Self::SetQuantifierToClock;
  pub const FILTER_QUANTIFIER: Self = Self::FilterQuantifier;
  pub const FILTER_GROUP: Self = Self::FilterGroup;
  pub const FILTER_LOOKAROUND: Self = Self::FilterLookaround;
  pub const FILTER_CHILD: Self = Self::FilterChild;
  pub const BEGIN_LOOP: Self = Self::BeginLoop;
  pub const END_LOOP: Self = Self::EndLoop;
  pub const START_LOOKAROUND: Self = Self::StartLookaround;
  pub const END_LOOKAROUND: Self = Self::EndLookaround;
  pub const WRITE_LOOKAROUND_TABLE: Self = Self::WriteLookaroundTable;
  pub const READ_LOOKAROUND_TABLE: Self = Self::ReadLookaroundTable;
}

/// opcode 全集 (原版 19 值, 序数即下标)
pub const ALL_OPCODES: [Opcode; 19] = [
  Opcode::Accept,
  Opcode::Assertion,
  Opcode::ClearRegister,
  Opcode::ConsumeRange,
  Opcode::RangeCount,
  Opcode::Fork,
  Opcode::Jmp,
  Opcode::SetRegisterToCp,
  Opcode::SetQuantifierToClock,
  Opcode::FilterQuantifier,
  Opcode::FilterGroup,
  Opcode::FilterLookaround,
  Opcode::FilterChild,
  Opcode::BeginLoop,
  Opcode::EndLoop,
  Opcode::StartLookaround,
  Opcode::EndLookaround,
  Opcode::WriteLookaroundTable,
  Opcode::ReadLookaroundTable,
];

// ── payload 结构 (原版 h:123-148, 299-320 union) ────────────────────────────

/// CONSUME_RANGE 载荷: UTF-16 码元闭区间 (v8: h:123-126 `struct Uc16Range`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uc16Range {
  /// 下界, 含 (base::uc16 min)
  pub min: u16,
  /// 上界, 含 (base::uc16 max)
  pub max: u16,
}

/// START_LOOKAROUND / READ_LOOKAROUND_TABLE 载荷位域
/// (v8: h:128-148 `class LookaroundPayload`)。
///
/// 位域布局与原版逐 bit 一致 (payload_ 为 uint32_t):
/// - bit 0     : IsPositive = base::BitField<bool, 0, 1>       (h:143)
/// - bit 1     : Type       = IsPositive::Next<Lookaround::Type, 1> → shift 1, size 1 (h:144)
/// - bits 2-31 : LookaroundIndex = Type::Next<uint32_t, 30>    → shift 2, size 30 (h:145)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LookaroundPayload {
  payload_: u32,
}

/// 原版位域别名 1:1 (bit-field.h:37 `template <class T, int shift, int size, class U = uint32_t>`;
/// h:75-76 `Next<T2, size2> = BitField<T2, kShift + kSize, size2, U>`)。
/// Lookaround::Type 为 2 值枚举 (regexp-ast.h:723 `enum Type { LOOKAHEAD, LOOKBEHIND }`),
/// 位宽 1, 经 u32 编解码通道 (v8_base BitFieldCodec)。
type IsPositive = BitField32<bool, 0, 1>;
type TypeField = BitField32<u32, 1, 1>;
type LookaroundIndex = BitField32<u32, 2, 30>;

// 位域参数与原版 static_assert 组 1:1 (v8: h:143-145)
const _: () = assert!(IsPositive::K_SHIFT == 0 && IsPositive::K_SIZE == 1);
const _: () = assert!(TypeField::K_SHIFT == 1 && TypeField::K_SIZE == 1);
const _: () = assert!(LookaroundIndex::K_SHIFT == 2 && LookaroundIndex::K_SIZE == 30);
// 原版 payload union static_assert(sizeof(payload) == 4) (h:320);
// 位域基底 u32 恰 4 字节
const _: () = assert!(size_of::<LookaroundPayload>() == 4);

impl LookaroundPayload {
  /// v8: h:131-136 LookaroundPayload(lookaround_index, is_positive, type) —
  /// 嵌套 update 次序与原版逐字一致: Index::encode → IsPositive::update → Type::update
  #[must_use]
  pub fn new(lookaround_index: i32, is_positive: bool, type_: LookaroundType) -> Self {
    // C++ 形参 uint32_t; 负数经 as u32 后超 30 位, encode 的整数 CHECK 通道即中止
    // (v8-base bit_field.rs encode: !TRUSTED_TYPE → assert is_valid), 与原版 DCHECK 语义一致
    let index = lookaround_index as u32;
    let payload = IsPositive::update(LookaroundIndex::encode(index), is_positive);
    Self {
      payload_: TypeField::update(payload, type_ as u32),
    }
  }

  /// v8: h:138 index()
  #[must_use]
  pub fn index(&self) -> u32 {
    LookaroundIndex::decode(self.payload_)
  }

  /// v8: h:139 is_positive()
  #[must_use]
  pub fn is_positive(&self) -> bool {
    IsPositive::decode(self.payload_)
  }

  /// v8: h:140 type() — 1 位域仅 2 值, 非 0 即 LOOKBEHIND (与 C++ 隐式枚举还原一致)
  #[must_use]
  pub fn type_(&self) -> LookaroundType {
    if TypeField::decode(self.payload_) == 0 {
      LookaroundType::Lookahead
    } else {
      LookaroundType::Lookbehind
    }
  }
}

/// 指令载荷 (v8: h:299-319 union)。C++ 为无判别 union, 读取由 opcode 静态约定;
/// Rust 用 tagged 枚举重建 9 个 union 成员, 非法读组合编译期即排除 (审查环注记:
/// 与 union "按 opcode 直读" 语义等价, 因每个构造工厂只产出合法配对)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payload {
  /// 无载荷指令 (ACCEPT/BEGIN_LOOP/END_LOOP/END_LOOKAROUND); C++ 对应未初始化的
  /// union 字节, 从不被读取
  None,
  /// CONSUME_RANGE: union.consume_range (h:301)
  ConsumeRange(Uc16Range),
  /// RANGE_COUNT: union.num_ranges (h:303)
  NumRanges(i32),
  /// FORK/JMP/FILTER_CHILD: union.pc (h:304-306)
  Pc(i32),
  /// SET_REGISTER_TO_CP/CLEAR_REGISTER: union.register_index (h:307-308)
  RegisterIndex(i32),
  /// ASSERTION: union.assertion_type (h:309-310)
  Assertion(AssertionType),
  /// SET_QUANTIFIER_TO_CLOCK/FILTER_QUANTIFIER: union.quantifier_id (h:311-312)
  QuantifierId(i32),
  /// FILTER_GROUP: union.group_id (h:313-314)
  GroupId(i32),
  /// WRITE_LOOKAROUND_TABLE/FILTER_LOOKAROUND: union.lookaround_id (h:315-316)
  LookaroundId(i32),
  /// READ_LOOKAROUND_TABLE/START_LOOKAROUND: union.lookaround (h:317-318)
  Lookaround(LookaroundPayload),
}

/// 一条实验性字节码指令 (v8: h:100-321 `struct Instruction`, 原版定长 8 字节,
/// static_assert(sizeof(Instruction) == 8) h:322 — Rust tagged 枚举布局不承诺等长,
/// 裸编码序列化面推迟至解释器席位, 见模块头登记)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
  /// 操作码 (h:298 Opcode opcode)
  pub opcode: Opcode,
  /// 载荷 (h:299 union payload)
  pub payload: Payload,
}

impl Instruction {
  /// v8: h:150-155 ConsumeRange(min, max)
  #[must_use]
  pub fn consume_range(min: u16, max: u16) -> Self {
    Self {
      opcode: Opcode::ConsumeRange,
      payload: Payload::ConsumeRange(Uc16Range { min, max }),
    }
  }

  /// v8: h:157 ConsumeAnyChar() = ConsumeRange(0x0000, 0xFFFF)
  #[must_use]
  pub fn consume_any_char() -> Self {
    Self::consume_range(0x0000, 0xFFFF)
  }

  /// v8: h:159-163 Fail() — 空区间 CONSUME_RANGE (0xFFFF <= c <= 0x0000), 恒失败
  #[must_use]
  pub fn fail() -> Self {
    Self::consume_range(0xFFFF, 0x0000)
  }

  /// v8: h:165-170 RangeCount(num_ranges)
  #[must_use]
  pub fn range_count(num_ranges: i32) -> Self {
    Self {
      opcode: Opcode::RangeCount,
      payload: Payload::NumRanges(num_ranges),
    }
  }

  /// v8: h:172-177 Fork(alt_index)
  #[must_use]
  pub fn fork(alt_index: i32) -> Self {
    Self {
      opcode: Opcode::Fork,
      payload: Payload::Pc(alt_index),
    }
  }

  /// v8: h:179-184 Jmp(alt_index)
  #[must_use]
  pub fn jmp(alt_index: i32) -> Self {
    Self {
      opcode: Opcode::Jmp,
      payload: Payload::Pc(alt_index),
    }
  }

  /// v8: h:186-190 Accept()
  #[must_use]
  pub fn accept() -> Self {
    Self {
      opcode: Opcode::Accept,
      payload: Payload::None,
    }
  }

  /// v8: h:192-197 SetRegisterToCp(register_index)
  #[must_use]
  pub fn set_register_to_cp(register_index: i32) -> Self {
    Self {
      opcode: Opcode::SetRegisterToCp,
      payload: Payload::RegisterIndex(register_index),
    }
  }

  /// v8: h:199-204 Assertion(Assertion::Type t)
  #[must_use]
  pub fn assertion(t: AssertionType) -> Self {
    Self {
      opcode: Opcode::Assertion,
      payload: Payload::Assertion(t),
    }
  }

  /// v8: h:206-211 ClearRegister(register_index)
  #[must_use]
  pub fn clear_register(register_index: i32) -> Self {
    Self {
      opcode: Opcode::ClearRegister,
      payload: Payload::RegisterIndex(register_index),
    }
  }

  /// v8: h:213-218 SetQuantifierToClock(quantifier_id)
  #[must_use]
  pub fn set_quantifier_to_clock(quantifier_id: i32) -> Self {
    Self {
      opcode: Opcode::SetQuantifierToClock,
      payload: Payload::QuantifierId(quantifier_id),
    }
  }

  /// v8: h:220-225 FilterQuantifier(quantifier_id)
  #[must_use]
  pub fn filter_quantifier(quantifier_id: i32) -> Self {
    Self {
      opcode: Opcode::FilterQuantifier,
      payload: Payload::QuantifierId(quantifier_id),
    }
  }

  /// v8: h:227-232 FilterGroup(group_id)
  #[must_use]
  pub fn filter_group(group_id: i32) -> Self {
    Self {
      opcode: Opcode::FilterGroup,
      payload: Payload::GroupId(group_id),
    }
  }

  /// v8: h:234-239 FilterLookaround(lookaround_id)
  #[must_use]
  pub fn filter_lookaround(lookaround_id: i32) -> Self {
    Self {
      opcode: Opcode::FilterLookaround,
      payload: Payload::LookaroundId(lookaround_id),
    }
  }

  /// v8: h:241-246 FilterChild(pc)
  #[must_use]
  pub fn filter_child(pc: i32) -> Self {
    Self {
      opcode: Opcode::FilterChild,
      payload: Payload::Pc(pc),
    }
  }

  /// v8: h:248-252 BeginLoop()
  #[must_use]
  pub fn begin_loop() -> Self {
    Self {
      opcode: Opcode::BeginLoop,
      payload: Payload::None,
    }
  }

  /// v8: h:254-258 EndLoop()
  #[must_use]
  pub fn end_loop() -> Self {
    Self {
      opcode: Opcode::EndLoop,
      payload: Payload::None,
    }
  }

  /// v8: h:260-267 StartLookaround(lookaround_index, is_positive, type)
  #[must_use]
  pub fn start_lookaround(lookaround_index: i32, is_positive: bool, type_: LookaroundType) -> Self {
    Self {
      opcode: Opcode::StartLookaround,
      payload: Payload::Lookaround(LookaroundPayload::new(lookaround_index, is_positive, type_)),
    }
  }

  /// v8: h:269-273 EndLookaround()
  #[must_use]
  pub fn end_lookaround() -> Self {
    Self {
      opcode: Opcode::EndLookaround,
      payload: Payload::None,
    }
  }

  /// v8: h:275-280 WriteLookTable(index)
  #[must_use]
  pub fn write_look_table(index: i32) -> Self {
    Self {
      opcode: Opcode::WriteLookaroundTable,
      payload: Payload::LookaroundId(index),
    }
  }

  /// v8: h:282-288 ReadLookTable(index, is_positive, type)
  #[must_use]
  pub fn read_look_table(index: i32, is_positive: bool, type_: LookaroundType) -> Self {
    Self {
      opcode: Opcode::ReadLookaroundTable,
      payload: Payload::Lookaround(LookaroundPayload::new(index, is_positive, type_)),
    }
  }

  /// v8: h:290-296 IsFilter(instruction) — 是否 FILTER_GROUP / FILTER_QUANTIFIER /
  /// FILTER_CHILD 三者之一
  #[must_use]
  pub fn is_filter(&self) -> bool {
    matches!(
      self.opcode,
      Opcode::FilterGroup | Opcode::FilterQuantifier | Opcode::FilterChild
    )
  }
}

// ── Dump 文本层 (v8: experimental-bytecode.cc 三个 operator<<) ───────────────

/// std::ostream 语义的最小复刻 (金标准逐字符 1:1 的关键):
/// - basefield: 十进制 (默认) / 十六进制; **std::hex 一经设置不复原** —
///   PrintAsciiOrHex 的非 ASCII 分支会永久翻转流状态, 影响同一流上的后续输出
///   (cc:16-23, cc:136-147 的向量打印在同一 os 上连续消费, 行号/后续数值随流进 hex)
/// - fill 粘性 (std::setfill), width 一次性 (std::setw 只作用下一个插入项, cc:143)
/// - hex 为小写、无前导零; i32 负值按符号扩展至 64 位位模式输出 (libstdc++/libc++
///   num_put %llX 行为; 引擎实际 pc/寄存器索引均非负, 此分支仅保真兜底)
struct Ostream {
  buf: String,
  hex: bool,
  fill: char,
  width: usize,
}

impl Ostream {
  fn new() -> Self {
    Self {
      buf: String::new(),
      hex: false,
      fill: ' ',
      width: 0,
    }
  }

  fn text(&mut self, s: &str) {
    self.buf.push_str(s);
  }

  /// 整数插入 (os << int/uint32_t/size_t), 应用并消耗当前 width。
  /// 调用方保证: 实际引擎产出的数值均非负且 < 2^31; 负 i32 按 v as u64 符号扩展
  /// 取位模式 (libstdc++/libc++ num_put %llX 行为), 仅保真兜底
  fn number(&mut self, v: i64) {
    let s = if self.hex {
      format!("{:x}", v as u64)
    } else {
      format!("{v}")
    };
    let pad = self.width.saturating_sub(s.chars().count());
    if pad > 0 {
      for _ in 0..pad {
        self.buf.push(self.fill);
      }
    }
    self.buf.push_str(&s);
    self.width = 0;
  }

  /// v8: cc:16-23 PrintAsciiOrHex — c < 128 且 std::isprint (C locale: 0x20..=0x7E)
  /// 输出字符本身; 否则输出 "0x" + 小写十六进制, 且 **std::hex 粘性保留**
  fn print_ascii_or_hex(&mut self, c: u16) {
    if c < 128 && (0x20..=0x7E).contains(&c) {
      // 可打印 ASCII 直写 (C locale isprint 即 0x20..=0x7E)
      self.buf.push(char::from(c as u8));
    } else {
      self.text("0x");
      self.hex = true;
      self.number(i64::from(c));
    }
  }

  /// v8: cc:151-154 LookaroundPayload 打印 (index 随流进制)
  fn lookaround_payload(&mut self, payload: &LookaroundPayload) {
    self.number(i64::from(payload.index()));
    self.text(" (");
    self.text(if payload.type_() == LookaroundType::Lookahead {
      "ahead"
    } else {
      "behind"
    });
    self.text(", ");
    self.text(if payload.is_positive() {
      "positive"
    } else {
      "negative"
    });
    self.text(")");
  }

  /// v8: cc:41-67 ASSERTION 名字表; END_OF_BUFFER 在 cc:50-53 为 UNREACHABLE():
  /// \Z 需要 /u, 实验性引擎拒绝 /u, 该断言永不到达字节码层 (glossary: 100% 不可达
  /// 才用 unreachable!)
  fn assertion_name(&mut self, t: AssertionType) {
    let name = match t {
      AssertionType::StartOfInput => "START_OF_INPUT",
      AssertionType::EndOfInput => "END_OF_INPUT",
      AssertionType::EndOfBuffer => {
        unreachable!("v8 cc:50-53 UNREACHABLE(): END_OF_BUFFER 依赖 /u, 实验性引擎不产出")
      }
      AssertionType::StartOfLine => "START_OF_LINE",
      AssertionType::EndOfLine => "END_OF_LINE",
      AssertionType::Boundary => "BOUNDARY",
      AssertionType::NonBoundary => "NON_BOUNDARY",
    };
    self.text(name);
  }

  /// v8: cc:27-118 operator<<(ostream&, const Instruction&) —
  /// switch 21 个可达 case (19 opcode, ASSERTION 子表 6 可达) 逐字符 1:1
  fn instruction(&mut self, inst: &Instruction) {
    match (inst.opcode, inst.payload) {
      (Opcode::ConsumeRange, Payload::ConsumeRange(r)) => {
        self.text("CONSUME_RANGE [");
        self.print_ascii_or_hex(r.min);
        self.text(", ");
        self.print_ascii_or_hex(r.max);
        self.text("]");
      }
      (Opcode::RangeCount, Payload::NumRanges(n)) => {
        self.text("RANGE_COUNT ");
        self.number(i64::from(n));
      }
      (Opcode::Assertion, Payload::Assertion(t)) => {
        self.text("ASSERTION ");
        self.assertion_name(t);
      }
      (Opcode::Fork, Payload::Pc(pc)) => {
        self.text("FORK ");
        self.number(i64::from(pc));
      }
      (Opcode::Jmp, Payload::Pc(pc)) => {
        self.text("JMP ");
        self.number(i64::from(pc));
      }
      (Opcode::Accept, _) => self.text("ACCEPT"),
      (Opcode::SetRegisterToCp, Payload::RegisterIndex(r)) => {
        self.text("SET_REGISTER_TO_CP ");
        self.number(i64::from(r));
      }
      (Opcode::ClearRegister, Payload::RegisterIndex(r)) => {
        self.text("CLEAR_REGISTER ");
        self.number(i64::from(r));
      }
      (Opcode::SetQuantifierToClock, Payload::QuantifierId(q)) => {
        self.text("SET_QUANTIFIER_TO_CLOCK ");
        self.number(i64::from(q));
      }
      (Opcode::FilterQuantifier, Payload::QuantifierId(q)) => {
        self.text("FILTER_QUANTIFIER ");
        self.number(i64::from(q));
      }
      (Opcode::FilterGroup, Payload::GroupId(g)) => {
        self.text("FILTER_GROUP ");
        self.number(i64::from(g));
      }
      (Opcode::FilterLookaround, Payload::LookaroundId(l)) => {
        self.text("FILTER_LOOKAROUND ");
        self.number(i64::from(l));
      }
      (Opcode::FilterChild, Payload::Pc(pc)) => {
        self.text("FILTER_CHILD ");
        self.number(i64::from(pc));
      }
      (Opcode::BeginLoop, _) => self.text("BEGIN_LOOP"),
      (Opcode::EndLoop, _) => self.text("END_LOOP"),
      (Opcode::StartLookaround, Payload::Lookaround(p)) => {
        self.text("START_LOOKAROUND ");
        self.lookaround_payload(&p);
      }
      (Opcode::EndLookaround, _) => self.text("END_LOOKAROUND"),
      (Opcode::WriteLookaroundTable, Payload::LookaroundId(l)) => {
        self.text("WRITE_LOOKAROUND_TABLE ");
        self.number(i64::from(l));
      }
      (Opcode::ReadLookaroundTable, Payload::Lookaround(p)) => {
        self.text("READ_LOOKAROUND_TABLE ");
        self.lookaround_payload(&p);
      }
      // 非法 opcode/payload 配对: 工厂函数唯一构造合法配对 (见 Payload 注释),
      // tagged 枚举使此处不可达; C++ 对应 union 按 opcode 约定直读
      _ => unreachable!("v8 h:299-319: payload 与 opcode 由构造工厂静态配对"),
    }
  }
}

impl fmt::Display for LookaroundPayload {
  /// v8: cc:149-155 operator<<(ostream&, const LookaroundPayload&) —
  /// `<index> (ahead|behind, positive|negative)`; Display 入口等价全新流 (十进制)
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut os = Ostream::new();
    os.lookaround_payload(self);
    f.write_str(&os.buf)
  }
}

impl fmt::Debug for LookaroundPayload {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(self, f)
  }
}

impl fmt::Display for Instruction {
  /// v8: cc:27-118 operator<<(ostream&, const Instruction&) 的全新流入口 (十进制起步)
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut os = Ostream::new();
    os.instruction(self);
    f.write_str(&os.buf)
  }
}

/// v8: cc:120-132 DigitsRequiredBelow(n) — 显示 [0, n) 内非负数所需的最大十进制位数
/// (int 域; n<0 时原版 DCHECK_GE 失败, Rust usize 天然非负)
fn digits_required_below(n: usize) -> usize {
  let mut result = 1;
  let mut i: u64 = 10;
  while i < n as u64 {
    result += 1;
    i = i.saturating_mul(10);
  }
  result
}

/// v8: cc:136-147 operator<<(ostream&, base::Vector<const Instruction>) —
/// 每行 `setfill('0') setw(digits) <i>: <inst> endl`。
///
/// 金标准要点: 整个向量共享一个流 — PrintAsciiOrHex 触发 std::hex 后**不复原**,
/// 后续行号与各 payload 数值一律按 hex 输出 (原版真实行为, 逐字符保真)。
#[must_use]
pub fn dump(insts: &[Instruction]) -> String {
  let mut os = Ostream::new();
  let inst_num = insts.len();
  let line_digit_num = digits_required_below(inst_num);
  for (i, inst) in insts.iter().enumerate() {
    // os << std::setfill('0') << std::setw(line_digit_num) << i << ": " << inst << endl
    os.fill = '0';
    os.width = line_digit_num;
    os.number(i as i64);
    os.text(": ");
    os.instruction(inst);
    os.text("\n");
  }
  os.buf
}
