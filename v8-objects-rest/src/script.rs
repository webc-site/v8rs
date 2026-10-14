// 由 tool/src/objects/script.h.js 从 v8/src/objects/script.{h,tq} 生成; 修订就地改, 复跑覆盖
// Script 类型/编译状态枚举与位置信息骨架
//! Script: 脚本元数据 (源/位置/编译状态)。

/// 临时脚本 ID (不加入 script list)。
pub const TEMPORARY_SCRIPT_ID: i32 = -2;

/// 对应原版 Script::ScriptType, 共 5 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ScriptType {
  /// kNative = 0
  Native = 0 = 0,
  /// kExtension = 1
  Extension = 1 = 1,
  /// kNormal = 2
  Normal = 2 = 2,
  /// kWasm = 3
  Wasm = 3 = 3,
  /// kInspector = 4
  Inspector = 4 = 4,
}

impl ScriptType {
  /// 变体总数
  pub const COUNT: usize = 5;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::Native = 0),
      1 => Some(Self::Extension = 1),
      2 => Some(Self::Normal = 2),
      3 => Some(Self::Wasm = 3),
      4 => Some(Self::Inspector = 4),
      _ => None,
    }
  }
}

/// 对应原版 Script::CompilationState, 共 2 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CompilationState {
  /// kInitial = 0
  Initial = 0 = 0,
  /// kCompiled = 1
  Compiled = 1 = 1,
}

impl CompilationState {
  /// 变体总数
  pub const COUNT: usize = 2;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::Initial = 0),
      1 => Some(Self::Compiled = 1),
      _ => None,
    }
  }
}

/// 对应原版 Script::CompilationKind, 共 5 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CompilationKind {
  /// kHost = 0
  Host = 0 = 0,
  /// kDirectEval = 1
  Directeval = 1 = 1,
  /// kIndirectEval = 2
  Indirecteval = 2 = 2,
  /// kFunctionConstructor = 3
  Functionconstructor = 3 = 3,
  /// kWrapped = 4
  Wrapped = 4 = 4,
}

impl CompilationKind {
  /// 变体总数
  pub const COUNT: usize = 5;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::Host = 0),
      1 => Some(Self::Directeval = 1),
      2 => Some(Self::Indirecteval = 2),
      3 => Some(Self::Functionconstructor = 3),
      4 => Some(Self::Wrapped = 4),
      _ => None,
    }
  }
}

/// 对应原版 Script::OffsetFlag, 共 2 变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OffsetFlag {
  /// kNoOffset
  Nooffset = 0,
  /// kWithOffset
  Withoffset = 1,
}

impl OffsetFlag {
  /// 变体总数
  pub const COUNT: usize = 2;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::Nooffset),
      1 => Some(Self::Withoffset),
      _ => None,
    }
  }
}

/// 源位置信息 (原版 Script::PositionInfo 骨架; 语义实现由 Developer 补)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionInfo {
  /// 零基行号。
  pub line: i32,
  /// 零基列号。
  pub column: i32,
  /// 行首字符位置。
  pub line_start: i32,
  /// 行尾换行位置。
  pub line_end: i32,
}

impl Default for PositionInfo {
  fn default() -> Self {
    Self { line: -1, column: -1, line_start: -1, line_end: -1 }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ScriptType::COUNT == 5);
  assert!(ScriptType::Wasm as u8 == 3);
  assert!(ScriptType::Inspector as u8 == 4);
  assert!(CompilationState::COUNT == 2);
  assert!(CompilationKind::COUNT == 5);
  assert!(OffsetFlag::COUNT == 2);
  assert!(TEMPORARY_SCRIPT_ID == -2);
};
