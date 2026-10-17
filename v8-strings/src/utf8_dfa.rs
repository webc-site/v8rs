// 由 tool/src/strings/unicode.h.js 从 v8/third_party/utf8-decoder/utf8-decoder.h + generalized-utf8-decoder.h 生成; 修订就地改, 复跑覆盖
// DFA 两表逐数字提取 (Bjoern Hoehrmann DFA); 语义绑定, 禁库替换

//! UTF-8 / generalized-UTF-8 (WTF-8) DFA 解码器。
//! 状态转移表与原版逐字节一致; `Decode` 单步推进, buffer 累积码点。

/// 严格 UTF-8 DFA (拒绝代理区与超长编码)。
pub mod strict {
  /// DFA 状态 (枚举值 = states 表行基址)。
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  #[repr(u8)]
  pub enum State {
    Reject = 0,
    Accept = 12,
    TwoByte = 24,
    ThreeByte = 36,
    ThreeByteLowMid = 48,
    FourByte = 60,
    FourByteLow = 72,
    ThreeByteHigh = 84,
    FourByteMidHigh = 96,
  }

  /// 字节 → 转移类型。
  static TRANSITIONS: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    9, 9, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    10, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 11, 7, 7, 7, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9,
  ];

  /// (状态, 转移类型) → 新状态。
  static STATES: [u8; 108] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 24, 36, 48, 60, 72, 0, 84, 96, 0, 12, 12, 12,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 24, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 24, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 36, 36, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 36, 0, 0, 0, 0, 0, 0, 0, 0,
  ];

  /// 单步解码: 推进 state, 累积 buffer。
  #[inline]
  pub fn decode(byte: u8, state: &mut State, buffer: &mut u32) {
    let t = TRANSITIONS[usize::from(byte)];
    *state = State::from_table(STATES[usize::from(*state as u8) + usize::from(t)]);
    *buffer = (*buffer << 6) | u32::from(byte & (0x7F >> (t >> 1)));
  }

  impl State {
    /// states 表产出值 → 状态; 表值域恰为枚举基址, 兜底 Reject (不可达)。
    fn from_table(v: u8) -> State {
      match v {
        12 => State::Accept,
        24 => State::TwoByte,
        36 => State::ThreeByte,
        48 => State::ThreeByteLowMid,
        60 => State::FourByte,
        72 => State::FourByteLow,
        84 => State::ThreeByteHigh,
        96 => State::FourByteMidHigh,
        _ => State::Reject,
      }
    }
  }
}

/// 泛化 UTF-8 (WTF-8) DFA: 允许孤立代理, 拒绝代理对由外层捕获。
pub mod generalized {
  /// DFA 状态 (枚举值 = states 表行基址)。
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  #[repr(u8)]
  pub enum State {
    Reject = 0,
    Accept = 11,
    TwoByte = 22,
    ThreeByte = 33,
    FourByte = 44,
    FourByteLow = 55,
    ThreeByteHigh = 66,
    FourByteMidHigh = 77,
  }

  /// 字节 → 转移类型。
  static TRANSITIONS: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    8, 8, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    9, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 6, 6, 6, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8,
  ];

  /// (状态, 转移类型) → 新状态。
  static STATES: [u8; 88] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 22, 33, 44, 55, 0, 66, 77, 0, 11, 11, 11, 0, 0,
    0, 0, 0, 0, 0, 0, 22, 22, 22, 0, 0, 0, 0, 0, 0, 0, 0, 33, 33, 33, 0, 0, 0, 0, 0, 0, 0, 0, 33,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 33, 0, 0, 0, 0, 0, 0, 0,
  ];

  /// 单步解码: 推进 state, 累积 buffer。
  #[inline]
  pub fn decode(byte: u8, state: &mut State, buffer: &mut u32) {
    let t = TRANSITIONS[usize::from(byte)];
    *state = State::from_table(STATES[usize::from(*state as u8) + usize::from(t)]);
    *buffer = (*buffer << 6) | u32::from(byte & (0x7F >> (t >> 1)));
  }

  impl State {
    /// states 表产出值 → 状态; 表值域恰为枚举基址, 兜底 Reject (不可达)。
    fn from_table(v: u8) -> State {
      match v {
        11 => State::Accept,
        22 => State::TwoByte,
        33 => State::ThreeByte,
        44 => State::FourByte,
        55 => State::FourByteLow,
        66 => State::ThreeByteHigh,
        77 => State::FourByteMidHigh,
        _ => State::Reject,
      }
    }
  }
}
