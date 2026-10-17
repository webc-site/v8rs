// 由 tool/src/codegen/source-position.h.js 从 v8/src/codegen/source-position.{h,cc} 生成; 修订就地改, 复跑覆盖
// SourcePosition 位域64布局 + 结构骨架 + InliningPosition/SourcePositionInfo

// 位域64布局 (原版 base::BitField64<...>):
// [0]      is_external (1 位)
// 外部分支: [1..21] external_line (20 位) | [21..31] external_file_id (10 位)
// JS 分支:  [1..31] script_offset (30 位, 存 offset+1)
// 公共:    [31..47] inlining_id (16 位, 存 id+1); 最高位保留给符号转换
pub const IS_EXTERNAL_SHIFT: u32 = 0;
pub const IS_EXTERNAL_BITS: u32 = 1;
pub const EXTERNAL_LINE_SHIFT: u32 = 1;
pub const EXTERNAL_LINE_BITS: u32 = 20;
pub const EXTERNAL_FILE_ID_SHIFT: u32 = 21;
pub const EXTERNAL_FILE_ID_BITS: u32 = 10;
pub const SCRIPT_OFFSET_SHIFT: u32 = 1;
pub const SCRIPT_OFFSET_BITS: u32 = 30;
pub const INLINING_ID_SHIFT: u32 = 31;
pub const INLINING_ID_BITS: u32 = 16;

pub const IS_EXTERNAL_MASK: u64 = 0x1u64 << IS_EXTERNAL_SHIFT;
pub const EXTERNAL_LINE_MASK: u64 = 0xfffffu64 << EXTERNAL_LINE_SHIFT;
pub const EXTERNAL_FILE_ID_MASK: u64 = 0x3ffu64 << EXTERNAL_FILE_ID_SHIFT;
pub const SCRIPT_OFFSET_MASK: u64 = 0x3fffffffu64 << SCRIPT_OFFSET_SHIFT;
pub const INLINING_ID_MASK: u64 = 0xffffu64 << INLINING_ID_SHIFT;

/// 原版 kNotInlined
pub const K_NOT_INLINED: i32 = -1;
/// 原版 static_assert(kNoSourcePosition == -1)
pub const K_NO_SOURCE_POSITION: i32 = -1;

/// 原版 SourcePosition: 64 位打包源位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourcePosition {
  /// 原版 value_ (最高位保留)
  value_: u64,
}

impl Default for SourcePosition {
  fn default() -> Self {
    Self::unknown()
  }
}

impl SourcePosition {
  #[inline]
  const fn from_value(value_: u64) -> Self {
    Self { value_ }
  }

  /// 原版 Unknown(): kNoSourcePosition + kNotInlined
  #[inline]
  #[must_use]
  pub const fn unknown() -> Self {
    Self::from_value(0)
  }

  /// 原版 External(line, file_id)
  #[must_use]
  pub const fn external(line: i32, file_id: i32) -> Self {
    // 机械对应: SetIsExternal(true) + SetExternalLine + SetExternalFileId + SetInliningId(kNotInlined)
    let v = IS_EXTERNAL_MASK;
    let v = v | (((line as u64) << EXTERNAL_LINE_SHIFT) & EXTERNAL_LINE_MASK);
    let v = v | (((file_id as u64) << EXTERNAL_FILE_ID_SHIFT) & EXTERNAL_FILE_ID_MASK);
    // inlining_id + 1 = 0
    Self::from_value(v)
  }

  /// 原版 SourcePosition(script_offset = kNoSourcePosition, inlining_id = kNotInlined)
  #[must_use]
  pub const fn from_script_offset(script_offset: i32, inlining_id: i32) -> Self {
    // SetIsExternal(false) + SetScriptOffset(offset+1) + SetInliningId(id+1)
    let v = (((script_offset as u64 + 1) << SCRIPT_OFFSET_SHIFT) & SCRIPT_OFFSET_MASK)
      | (((inlining_id as u64 + 1) << INLINING_ID_SHIFT) & INLINING_ID_MASK);
    Self::from_value(v)
  }

  /// 原版 IsKnown: raw != Unknown().raw
  #[inline]
  #[must_use]
  pub const fn is_known(&self) -> bool {
    self.value_ != 0
  }

  /// 原版 IsExternal
  #[inline]
  #[must_use]
  pub const fn is_external(&self) -> bool {
    (self.value_ & IS_EXTERNAL_MASK) != 0
  }

  /// 原版 IsJavaScript
  #[inline]
  #[must_use]
  pub const fn is_javascript(&self) -> bool {
    !self.is_external()
  }

  /// 原版 isInlined
  #[inline]
  #[must_use]
  pub const fn is_inlined(&self) -> bool {
    if self.is_external() {
      return false;
    }
    self.inlining_id() != K_NOT_INLINED
  }

  /// 原版 ExternalLine
  #[inline]
  #[must_use]
  pub const fn external_line(&self) -> i32 {
    ((self.value_ & EXTERNAL_LINE_MASK) >> EXTERNAL_LINE_SHIFT) as i32
  }

  /// 原版 ExternalFileId
  #[inline]
  #[must_use]
  pub const fn external_file_id(&self) -> i32 {
    ((self.value_ & EXTERNAL_FILE_ID_MASK) >> EXTERNAL_FILE_ID_SHIFT) as i32
  }

  /// 原版 ScriptOffset (decode - 1)
  #[inline]
  #[must_use]
  pub const fn script_offset(&self) -> i32 {
    ((self.value_ & SCRIPT_OFFSET_MASK) >> SCRIPT_OFFSET_SHIFT) as i32 - 1
  }

  /// 原版 InliningId (decode - 1)
  #[inline]
  #[must_use]
  pub const fn inlining_id(&self) -> i32 {
    ((self.value_ & INLINING_ID_MASK) >> INLINING_ID_SHIFT) as i32 - 1
  }

  /// 原版 SetIsExternal
  #[inline]
  pub const fn set_is_external(&mut self, external: bool) {
    self.value_ = if external {
      self.value_ | IS_EXTERNAL_MASK
    } else {
      self.value_ & !IS_EXTERNAL_MASK
    };
  }

  /// 原版 SetExternalLine
  #[inline]
  pub const fn set_external_line(&mut self, line: i32) {
    self.value_ = (self.value_ & !EXTERNAL_LINE_MASK)
      | (((line as u64) << EXTERNAL_LINE_SHIFT) & EXTERNAL_LINE_MASK);
  }

  /// 原版 SetExternalFileId
  #[inline]
  pub const fn set_external_file_id(&mut self, file_id: i32) {
    self.value_ = (self.value_ & !EXTERNAL_FILE_ID_MASK)
      | (((file_id as u64) << EXTERNAL_FILE_ID_SHIFT) & EXTERNAL_FILE_ID_MASK);
  }

  /// 原版 SetScriptOffset (存 offset + 1)
  #[inline]
  pub const fn set_script_offset(&mut self, script_offset: i32) {
    self.value_ = (self.value_ & !SCRIPT_OFFSET_MASK)
      | (((script_offset as u64 + 1) << SCRIPT_OFFSET_SHIFT) & SCRIPT_OFFSET_MASK);
  }

  /// 原版 SetInliningId (存 id + 1)
  #[inline]
  pub const fn set_inlining_id(&mut self, inlining_id: i32) {
    self.value_ = (self.value_ & !INLINING_ID_MASK)
      | (((inlining_id as u64 + 1) << INLINING_ID_SHIFT) & INLINING_ID_MASK);
  }

  /// 原版 MaxInliningId: InliningIdField::kMax
  #[inline]
  #[must_use]
  pub const fn max_inlining_id() -> i32 {
    ((1i64 << INLINING_ID_BITS) - 1) as i32
  }

  /// 原版 raw()
  #[inline]
  #[must_use]
  pub const fn raw(&self) -> i64 {
    self.value_ as i64
  }

  /// 原版 FromRaw(raw)
  #[inline]
  #[must_use]
  pub const fn from_raw(raw: i64) -> Self {
    Self::from_value(raw as u64)
  }

  // —— 签名骨架 (依赖 Isolate/Code/OptimizedCompilationInfo, 待 Developer) ——

  /// 原版 InliningStack(isolate, code)
  #[must_use]
  pub fn inlining_stack(&self, isolate: *mut (), code: *mut ()) -> Vec<SourcePositionInfo> {
    if !self.is_known() {
      return Vec::new();
    }
    vec![self.first_info(isolate, code)]
  }

  /// 原版 FirstInfo(isolate, code)
  #[must_use]
  pub fn first_info(&self, _isolate: *mut (), _code: *mut ()) -> SourcePositionInfo {
    SourcePositionInfo {
      position: *self,
      shared: 0,
      script: 0,
      line: if self.is_external() {
        self.external_line()
      } else {
        -1
      },
      column: -1,
    }
  }
}

/// 原版 InliningPosition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InliningPosition {
  /// 被内联调用点位置
  pub position: SourcePosition,
  /// DeoptimizationData::literals() 中的内联函数 id
  pub inlined_function_id: i32,
}

impl Default for InliningPosition {
  fn default() -> Self {
    Self {
      position: SourcePosition::unknown(),
      inlined_function_id: 0,
    }
  }
}

/// 原版 WasmInliningPosition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WasmInliningPosition {
  /// 未规范化 (模块内) 被内联函数索引
  pub inlinee_func_index: i32,
  /// 是否尾调用
  pub was_tail_call: bool,
  /// 调用方源位置
  pub caller_pos: SourcePosition,
}

/// 原版 SourcePositionInfo (骨架: handles 待接线)
#[derive(Debug, Clone)]
pub struct SourcePositionInfo {
  pub position: SourcePosition,
  /// 原版 shared: IndirectHandle<SharedFunctionInfo> (占位)
  pub shared: u32,
  /// 原版 script: IndirectHandle<Script> (占位)
  pub script: u32,
  pub line: i32,
  pub column: i32,
}

impl Default for SourcePositionInfo {
  fn default() -> Self {
    Self {
      position: SourcePosition::unknown(),
      shared: 0,
      script: 0,
      line: -1,
      column: -1,
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(IS_EXTERNAL_SHIFT == 0 && IS_EXTERNAL_BITS == 1);
  assert!(EXTERNAL_LINE_SHIFT == 1 && EXTERNAL_LINE_BITS == 20);
  assert!(EXTERNAL_FILE_ID_SHIFT == 21 && EXTERNAL_FILE_ID_BITS == 10);
  assert!(SCRIPT_OFFSET_SHIFT == 1 && SCRIPT_OFFSET_BITS == 30);
  assert!(INLINING_ID_SHIFT == 31 && INLINING_ID_BITS == 16);
  assert!(K_NOT_INLINED == -1 && K_NO_SOURCE_POSITION == -1);
  let u = SourcePosition::unknown();
  assert!(!u.is_known() && !u.is_external() && !u.is_inlined());
  let e = SourcePosition::external(5, 7);
  assert!(e.is_external() && e.external_line() == 5 && e.external_file_id() == 7);
  assert!(!e.is_inlined() && e.inlining_id() == -1);
  let j = SourcePosition::from_script_offset(42, 3);
  assert!(j.is_javascript() && j.script_offset() == 42 && j.inlining_id() == 3);
  assert!(j.is_inlined());
  let raw = SourcePosition::from_raw(j.raw());
  assert!(raw == j);
};
