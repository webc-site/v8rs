// 由 tool/src/debug/call-site-info.js 从 v8/src/objects/call-site-info.{-inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// 席位表差集: tq 位域/布局 + inl 访问器 + cc 签名骨架 (.h 位域数值归既有 call-site-info.h.js)
//! CallSiteInfo: Error.stack 栈帧对象 (debug 域)。
//!
//! tq 差集: CallSiteInfoFlags 位域 + Struct 布局 5 槽; .cc 方法面为堆无关转写,
//! 深度依赖 Isolate/Factory/Handle 的部分 (ConstructFromRawData 的 wrapper 解析、
//! ExpandDeferredFrames 的 baseline 码映射、EvalOrigin/MethodName/TypeName 的字符串
//! builder、GetScript/GetSharedFunctionInfo 的 SFI 桥接) 以参数注入并登记 `readme/zh.md` 推迟台账。

// ── CallSiteInfoFlags (tq uint31; 非 DRUMBRAKE 7 位 + DRUMBRAKE 位 7) ──
pub const K_IS_WASM_SHIFT: u32 = 0;
pub const K_IS_WASM_MASK: u32 = 0x1;
pub const K_IS_STRICT_SHIFT: u32 = 1;
pub const K_IS_STRICT_MASK: u32 = 0x2;
pub const K_IS_CONSTRUCTOR_SHIFT: u32 = 2;
pub const K_IS_CONSTRUCTOR_MASK: u32 = 0x4;
pub const K_IS_ASYNC_SHIFT: u32 = 3;
pub const K_IS_ASYNC_MASK: u32 = 0x8;
pub const K_IS_BUILTIN_SHIFT: u32 = 4;
pub const K_IS_BUILTIN_MASK: u32 = 0x10;
pub const K_IS_SOURCE_POSITION_COMPUTED_SHIFT: u32 = 5;
pub const K_IS_SOURCE_POSITION_COMPUTED_MASK: u32 = 0x20;
pub const K_IS_DEFERRED_BASELINE_FRAME_SHIFT: u32 = 6;
pub const K_IS_DEFERRED_BASELINE_FRAME_MASK: u32 = 0x40;
/// DRUMBRAKE 构建追加位 (默认关闭, bit 7)。
pub const K_IS_WASM_INTERPRETED_FRAME_SHIFT_DRUMBRAKE: u32 = 7;
pub const K_IS_WASM_INTERPRETED_FRAME_MASK_DRUMBRAKE: u32 = 0x80;

// ── 布局槽位 (tq extern class CallSiteInfo extends Struct, 源码序) ──
pub const CALL_SITE_INFO_FIELDS: [(&str, &str); 5] = [
  ("code_object", "TrustedPointer<Code|BytecodeArray>"),
  ("receiver_or_instance", "JSAny"),
  ("function", "JSFunction|Smi"),
  ("code_offset_or_source_position", "Smi"),
  ("flags", "SmiTagged<CallSiteInfoFlags>"),
];

// ── Message 侧哨兵 (v8/include/v8-message.h:195) ──
/// 无脚本行号信息 (原版 `Message::kNoLineNumberInfo`)。
pub const K_NO_LINE_NUMBER_INFO: i32 = 0;
/// 无脚本列号信息 (原版 `Message::kNoColumnInfo`)。
pub const K_NO_COLUMN_INFO: i32 = 0;
/// 无 script id 信息 (原版 `Message::kNoScriptIdInfo`)。
pub const K_NO_SCRIPT_ID_INFO: i32 = 0;
/// 无 wasm function index 信息 (原版 `Message::kNoWasmFunctionIndexInfo`)。
pub const K_NO_WASM_FUNCTION_INDEX_INFO: u32 = u32::MAX;
/// 无源码位置 (原版 `Message::kNoSourcePosition`)。
pub const K_NO_SOURCE_POSITION: i32 = -1;

/// 原版 `Fields::kCount = 5` — raw_data FixedArray 每帧槽数 (ConstructFromRawData 步长)。
pub const K_RAW_DATA_FIELD_COUNT: usize = 5;
/// Fields::kReceiver 索引
pub const K_RAW_DATA_RECEIVER: usize = 0;
/// Fields::kFunction 索引
pub const K_RAW_DATA_FUNCTION: usize = 1;
/// Fields::kCode 索引
pub const K_RAW_DATA_CODE: usize = 2;
/// Fields::kOffset 索引
pub const K_RAW_DATA_OFFSET: usize = 3;
/// Fields::kFlags 索引
pub const K_RAW_DATA_FLAGS: usize = 4;

// ── CallSiteInfo 视图 ──
/// 原版 CallSiteInfo 五槽的堆无关投影。
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CallSiteInfoView {
  /// `code_object` (Code|BytecodeArray) 的 arena 索引; `None` 即 undefined
  pub code_object: Option<u32>,
  /// `receiver_or_instance` (JSAny) arena 索引
  pub receiver_or_instance: u32,
  /// `function` (JSFunction|Smi) — 若 `Some(Smi)` 表示 builtin index, `None` 表示 JSFunction 需外部解引用
  pub function_builtin_id: Option<i32>,
  /// `code_offset_or_source_position` (Smi)
  pub code_offset_or_source_position: i32,
  /// `flags` (SmiTagged<CallSiteInfoFlags>)
  pub flags: u32,
}

impl CallSiteInfoView {
  // ── 位域访问器 (inl.h BOOL_GETTER 1:1) ──
  /// v8: src/objects/call-site-info-inl.h `IsWasm`
  #[must_use]
  pub const fn is_wasm(&self) -> bool {
    self.flags & K_IS_WASM_MASK != 0
  }

  /// v8: src/objects/call-site-info-inl.h `IsStrict`
  #[must_use]
  pub const fn is_strict(&self) -> bool {
    self.flags & K_IS_STRICT_MASK != 0
  }

  /// v8: src/objects/call-site-info-inl.h `IsConstructor`
  #[must_use]
  pub const fn is_constructor(&self) -> bool {
    self.flags & K_IS_CONSTRUCTOR_MASK != 0
  }

  /// v8: src/objects/call-site-info-inl.h `IsAsync`
  #[must_use]
  pub const fn is_async(&self) -> bool {
    self.flags & K_IS_ASYNC_MASK != 0
  }

  /// v8: src/objects/call-site-info-inl.h `IsBuiltin`
  #[must_use]
  pub const fn is_builtin(&self) -> bool {
    self.flags & K_IS_BUILTIN_MASK != 0
  }

  /// v8: src/objects/call-site-info-inl.h `IsSourcePositionComputed`
  #[must_use]
  pub const fn is_source_position_computed(&self) -> bool {
    self.flags & K_IS_SOURCE_POSITION_COMPUTED_MASK != 0
  }

  /// v8: src/objects/call-site-info-inl.h `IsDeferredBaselineFrame`
  #[must_use]
  pub const fn is_deferred_baseline_frame(&self) -> bool {
    self.flags & K_IS_DEFERRED_BASELINE_FRAME_MASK != 0
  }

  /// v8: src/objects/call-site-info-inl.h `IsWasmInterpretedFrame` (DRUMBRAKE 才启用)
  #[must_use]
  pub const fn is_wasm_interpreted_frame_drumbrake(&self) -> bool {
    self.flags & K_IS_WASM_INTERPRETED_FRAME_MASK_DRUMBRAKE != 0
  }
}

// ── ScriptView 桥接 (供位置计算与属性读取) ──
/// 原版 `Script::PositionInfo` (line/column)。
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PositionInfo {
  pub line: i32,
  pub column: i32,
}

/// 原版 `Script` 侧 CallSiteInfo 用到的字段/方法视图。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ScriptView {
  /// `id()` (Smi)
  pub id: i32,
  /// `type()` — 原版 Script::Type 枚举; 2 = kNative。
  pub is_native_type: bool,
  /// `has_eval_origin()`
  pub has_eval_origin: bool,
  /// `name()` (String|Undefined): `None` 即 undefined
  pub name: Option<String>,
  /// `source_url()` (String|Undefined)
  pub source_url: Option<String>,
  /// `source_mapping_url()` (String|Undefined)
  pub source_mapping_url: Option<String>,
  /// `HasValidSource()` — 有则 source 可读
  pub has_valid_source: bool,
  /// `source()` (String|Undefined) — 若 has_valid_source
  pub source: Option<String>,
  /// `HasSourceURLComment()`
  pub has_source_url_comment: bool,
  /// `line_offset()` (Smi) — 若 HasSourceURLComment
  pub line_offset: i32,
  /// `column_offset()` (Smi)
  pub column_offset: i32,
}

impl ScriptView {
  /// v8: src/objects/script.cc `Script::GetNameOrSourceURL`
  #[must_use]
  pub fn get_name_or_source_url(&self) -> Option<String> {
    if let Some(url) = self.source_url.as_ref().filter(|u| !u.is_empty()) {
      return Some(url.clone());
    }
    self.name.clone()
  }

  /// v8: src/objects/script.cc `Script::GetLineNumber(script, position)` —
  /// 简化模型: 通过 `\n` 计数; 原版会走 PositionInfo binary-search。
  #[must_use]
  pub fn get_line_number(&self, position: i32) -> i32 {
    if let Some(src) = &self.source {
      let pos = position.max(0).min(src.len() as i32) as usize;
      let bytes = &src.as_bytes()[..pos];
      bytes.iter().filter(|&&b| b == b'\n').count() as i32
    } else {
      -1
    }
  }

  /// v8: src/objects/script.cc `Script::GetPositionInfo` —
  /// 简化模型: 直接按 `\n` 分割计算列。
  #[must_use]
  pub fn get_position_info(&self, position: i32) -> PositionInfo {
    if let Some(src) = &self.source {
      let pos = position.max(0).min(src.len() as i32) as usize;
      let bytes = &src.as_bytes()[..pos];
      let line = bytes.iter().filter(|&&b| b == b'\n').count() as i32;
      let last_nl = bytes
        .iter()
        .rposition(|&b| b == b'\n')
        .map(|i| i + 1)
        .unwrap_or(0);
      let column = (pos - last_nl) as i32;
      PositionInfo { line, column }
    } else {
      PositionInfo {
        line: -1,
        column: -1,
      }
    }
  }

  /// v8: src/objects/script.cc `Script::GetColumnNumber(script, position)` —
  /// = GetPositionInfo.column。
  #[must_use]
  pub fn get_column_number(&self, position: i32) -> i32 {
    self.get_position_info(position).column
  }
}

// ── .cc 方法面 (源码序, 33 个) ──

/// v8: src/objects/call-site-info.cc:26 `CallSiteInfo::ConstructFromRawData`
/// — 从 raw_data FixedArray 的第 index 帧解出 5 槽, 组装 CallSiteInfoView。
/// 原版 Isolate/Factory 分配已堆无关化; code_obj wrapper 解析由 `resolve_code` 注入。
#[must_use]
pub fn construct_from_raw_data(
  raw_data: &[i64],
  index: usize,
  resolve_code: impl FnOnce(i64) -> Option<u32>,
) -> CallSiteInfoView {
  let Some(frame) = raw_data.as_chunks::<K_RAW_DATA_FIELD_COUNT>().0.get(index) else {
    return CallSiteInfoView::default();
  };
  let receiver = frame[K_RAW_DATA_RECEIVER] as u32;
  let function_slot = frame[K_RAW_DATA_FUNCTION];
  let code_obj_slot = frame[K_RAW_DATA_CODE];
  let offset = frame[K_RAW_DATA_OFFSET] as i32;
  let flags = frame[K_RAW_DATA_FLAGS] as u32;
  CallSiteInfoView {
    code_object: resolve_code(code_obj_slot),
    receiver_or_instance: receiver,
    function_builtin_id: if flags & K_IS_BUILTIN_MASK != 0 {
      Some(function_slot as i32)
    } else {
      None
    },
    code_offset_or_source_position: offset,
    flags,
  }
}

/// v8: src/objects/call-site-info.cc:56 `CallSiteInfo::ExpandDeferredFrames`
/// — 遍历 raw_data 帧, 若带 kIsDeferredBaselineFrame 位, 将 pc_offset 解算为 bytecode_offset,
/// 并清 deferred flag 位。本层以 `resolve_baseline` 闭包注入 Code→BytecodeArray 映射。
pub fn expand_deferred_frames<F>(raw_data: &mut [i64], resolve_baseline: F)
where
  F: Fn(Option<u32>, u32) -> Option<(Option<u32>, i32)>,
{
  for frame in raw_data.as_chunks_mut::<K_RAW_DATA_FIELD_COUNT>().0 {
    let flags = frame[K_RAW_DATA_FLAGS] as u32;
    if flags & K_IS_DEFERRED_BASELINE_FRAME_MASK == 0 {
      continue;
    }
    let code_slot = frame[K_RAW_DATA_CODE] as u32;
    let pc_offset = frame[K_RAW_DATA_OFFSET] as u32;
    if let Some((new_code, bytecode_offset)) = resolve_baseline(Some(code_slot), pc_offset) {
      frame[K_RAW_DATA_CODE] = new_code.unwrap_or(0) as i64;
      frame[K_RAW_DATA_OFFSET] = bytecode_offset as i64;
      frame[K_RAW_DATA_FLAGS] = (flags
        & !(K_IS_DEFERRED_BASELINE_FRAME_MASK | K_IS_WASM_INTERPRETED_FRAME_MASK_DRUMBRAKE))
        as i64;
    }
  }
}

/// v8: src/objects/call-site-info.cc:90 `CallSiteInfo::IsPromiseAll`
/// — IsAsync && function == native_context.promise_all;
/// 本层以 `promise_all_index` 参数注入外部 function 索引。
#[must_use]
pub fn is_promise_all(
  info: &CallSiteInfoView,
  function_index: u32,
  promise_all_index: u32,
) -> bool {
  info.is_async() && function_index == promise_all_index
}

/// v8: src/objects/call-site-info.cc:96 `CallSiteInfo::IsPromiseAllSettled`
#[must_use]
pub fn is_promise_all_settled(
  info: &CallSiteInfoView,
  function_index: u32,
  promise_all_settled_index: u32,
) -> bool {
  info.is_async() && function_index == promise_all_settled_index
}

/// v8: src/objects/call-site-info.cc:102 `CallSiteInfo::IsPromiseAny`
#[must_use]
pub fn is_promise_any(
  info: &CallSiteInfoView,
  function_index: u32,
  promise_any_index: u32,
) -> bool {
  info.is_async() && function_index == promise_any_index
}

/// v8: src/objects/call-site-info.cc:108 `CallSiteInfo::IsNative`
/// — IsBuiltin (WebAssembly 编译期) 或 script.type == kNative。
#[must_use]
pub fn is_native(info: &CallSiteInfoView, script: Option<&ScriptView>) -> bool {
  if info.is_builtin() {
    return true;
  }
  script.is_some_and(|s| s.is_native_type)
}

/// v8: src/objects/call-site-info.cc:118 `CallSiteInfo::IsEval`
#[must_use]
pub fn is_eval(info: &CallSiteInfoView, script: Option<&ScriptView>) -> bool {
  if info.is_wasm() || info.is_builtin() {
    return false;
  }
  script.is_some_and(|s| s.has_eval_origin)
}

/// v8: src/objects/call-site-info.cc:125 `CallSiteInfo::IsUserJavaScript`
/// — IsWasm/IsBuiltin 短路 false; 否则委托 SFI 判定, 本层由 `sfi_is_user_js` 注入。
#[must_use]
pub fn is_user_java_script(info: &CallSiteInfoView, sfi_is_user_js: impl FnOnce() -> bool) -> bool {
  if info.is_wasm() || info.is_builtin() {
    return false;
  }
  sfi_is_user_js()
}

/// v8: src/objects/call-site-info.cc:133 `CallSiteInfo::IsMethodCall`
/// — !IsWasm && !IsBuiltin && !IsToplevel && !IsConstructor。
/// `is_toplevel_result` 由调用方通过 [`is_toplevel`] 计算。
#[must_use]
pub fn is_method_call(info: &CallSiteInfoView, is_toplevel_result: bool) -> bool {
  if info.is_wasm() || info.is_builtin() {
    return false;
  }
  !is_toplevel_result && !info.is_constructor()
}

/// v8: src/objects/call-site-info.cc:141 `CallSiteInfo::IsToplevel`
/// — receiver 是 JSGlobalProxy/undefined/null; 本层以 `receiver_kind` 参数
/// (0=GlobalProxy, 1=Undefined, 2=Null, 3=其他) 编码。
#[must_use]
pub fn is_toplevel(receiver_kind: u8) -> bool {
  matches!(receiver_kind, 0..=2)
}

/// v8: src/objects/call-site-info.cc:147 `CallSiteInfo::GetLineNumber`
/// — IsWasm → 1; 否则按 GetSourcePosition 计算, 减 line_offset 若 HasSourceURLComment。
#[must_use]
pub fn get_line_number(info: &CallSiteInfoView, script: Option<&ScriptView>, position: i32) -> i32 {
  if info.is_wasm() {
    return 1;
  }
  match script {
    Some(s) => {
      let line = s.get_line_number(position) + 1;
      if s.has_source_url_comment {
        line - s.line_offset
      } else {
        line
      }
    }
    None => K_NO_LINE_NUMBER_INFO,
  }
}

/// v8: src/objects/call-site-info.cc:165 `CallSiteInfo::GetColumnNumber`
#[must_use]
pub fn get_column_number(
  info: &CallSiteInfoView,
  script: Option<&ScriptView>,
  position: i32,
) -> i32 {
  if info.is_wasm() {
    return position + 1;
  }
  match script {
    Some(s) => {
      let pi = s.get_position_info(position);
      let col = pi.column + 1;
      if s.has_source_url_comment && pi.line == s.line_offset {
        col - s.column_offset
      } else {
        col
      }
    }
    None => K_NO_COLUMN_INFO,
  }
}

/// v8: src/objects/call-site-info.cc:186 `CallSiteInfo::GetEnclosingLineNumber`
/// — IsWasm → 1; 否则按 SFI.function_token_position() 求行。
#[must_use]
pub fn get_enclosing_line_number(
  info: &CallSiteInfoView,
  script: Option<&ScriptView>,
  function_token_position: i32,
) -> i32 {
  if info.is_wasm() {
    return 1;
  }
  match script {
    Some(s) => s.get_line_number(function_token_position) + 1,
    None => K_NO_LINE_NUMBER_INFO,
  }
}

/// v8: src/objects/call-site-info.cc:200 `CallSiteInfo::GetEnclosingColumnNumber`
/// — IsWasm → GetWasmFunctionOffset (推迟); 否则按 function_token_position 求列。
#[must_use]
pub fn get_enclosing_column_number(
  info: &CallSiteInfoView,
  script: Option<&ScriptView>,
  function_token_position: i32,
) -> i32 {
  if info.is_wasm() {
    // Wasm function offset 需 wasm::Module 内部结构; 见推迟台账。
    return K_NO_COLUMN_INFO;
  }
  match script {
    Some(s) => s.get_column_number(function_token_position) + 1,
    None => K_NO_COLUMN_INFO,
  }
}

/// v8: src/objects/call-site-info.cc:218 `CallSiteInfo::GetScriptId`
#[must_use]
pub fn get_script_id(info: &CallSiteInfoView, script: Option<&ScriptView>) -> i32 {
  if info.is_wasm() || info.is_builtin() {
    return K_NO_SCRIPT_ID_INFO;
  }
  script.map_or(K_NO_SCRIPT_ID_INFO, |s| s.id)
}

/// v8: src/objects/call-site-info.cc:225 `CallSiteInfo::GetScriptName`
#[must_use]
pub fn get_script_name(info: &CallSiteInfoView, script: Option<&ScriptView>) -> Option<String> {
  if info.is_wasm() || info.is_builtin() {
    return None;
  }
  script.and_then(|s| s.name.clone())
}

/// v8: src/objects/call-site-info.cc:232 `CallSiteInfo::GetScriptNameOrSourceURL`
#[must_use]
pub fn get_script_name_or_source_url(
  info: &CallSiteInfoView,
  script: Option<&ScriptView>,
) -> Option<String> {
  if info.is_wasm() || info.is_builtin() {
    return None;
  }
  script.and_then(|s| s.get_name_or_source_url())
}

/// v8: src/objects/call-site-info.cc:239 `CallSiteInfo::GetScriptSource`
#[must_use]
pub fn get_script_source(info: &CallSiteInfoView, script: Option<&ScriptView>) -> Option<String> {
  if info.is_wasm() || info.is_builtin() {
    return None;
  }
  script
    .filter(|s| s.has_valid_source)
    .and_then(|s| s.source.clone())
}

/// v8: src/objects/call-site-info.cc:248 `CallSiteInfo::GetScriptSourceMappingURL`
#[must_use]
pub fn get_script_source_mapping_url(
  info: &CallSiteInfoView,
  script: Option<&ScriptView>,
) -> Option<String> {
  if info.is_wasm() || info.is_builtin() {
    return None;
  }
  script.and_then(|s| s.source_mapping_url.clone())
}

/// v8: src/objects/call-site-info.cc:256 `CallSiteInfo::GetScriptHash`
/// — 需 Script::GetScriptHash (走 hash 计算 + cache); 本层以 `compute_hash` 注入。
#[must_use]
pub fn get_script_hash(
  _info: &CallSiteInfoView,
  script: Option<&ScriptView>,
  compute_hash: impl FnOnce(&ScriptView) -> Option<String>,
) -> Option<String> {
  script.filter(|s| s.has_valid_source).and_then(compute_hash)
}

/// v8: src/objects/call-site-info.cc:326 `CallSiteInfo::GetEvalOrigin`
/// — 依赖 IncrementalStringBuilder + 递归 FormatEvalOrigin; 本层以 `format_eval` 注入。
#[must_use]
pub fn get_eval_origin(
  info: &CallSiteInfoView,
  script: Option<&ScriptView>,
  format_eval: impl FnOnce(&ScriptView) -> String,
) -> Option<String> {
  if info.is_wasm() || info.is_builtin() {
    return None;
  }
  script.filter(|s| s.has_eval_origin).map(format_eval)
}

/// v8: src/objects/call-site-info.cc:338 `CallSiteInfo::GetFunctionName`
/// — IsWasm→module name; IsBuiltin→Builtins::NameForStackTrace; 否则 JSFunction::GetDebugName;
/// 空且 IsEval → "eval"; 否则 null。本层以 `wasm_name`/`builtin_name`/`debug_name` 注入。
#[must_use]
pub fn get_function_name(
  info: &CallSiteInfoView,
  wasm_name: impl FnOnce(u32) -> Option<String>,
  builtin_name: impl FnOnce(i32) -> Option<String>,
  debug_name: impl FnOnce() -> Option<String>,
) -> Option<String> {
  if info.is_wasm() {
    return wasm_name(K_NO_WASM_FUNCTION_INDEX_INFO);
  }
  if info.is_builtin() {
    let builtin_id = info.function_builtin_id.unwrap_or(-1);
    return builtin_name(builtin_id);
  }
  let name = debug_name();
  if name.as_ref().is_some_and(|n| !n.is_empty()) {
    return name;
  }
  // IsEval 分支: 返回 "eval" 字面量。
  // (原版要求 IsEval(info) 检查, 本层无 script 参数, 由上层过滤)
  None
}

/// v8: src/objects/call-site-info.cc:380 `CallSiteInfo::GetFunctionDebugName`
/// — 与 GetFunctionName 类似, 但无 builtin 特化。空且 IsEval → "eval"。
#[must_use]
pub fn get_function_debug_name(
  info: &CallSiteInfoView,
  wasm_debug_name: impl FnOnce(u32) -> Option<String>,
  debug_name: impl FnOnce() -> Option<String>,
) -> Option<String> {
  if info.is_wasm() {
    return wasm_debug_name(K_NO_WASM_FUNCTION_INDEX_INFO);
  }
  if info.is_builtin() {
    return debug_name();
  }
  debug_name()
}

/// v8: src/objects/call-site-info.cc:501 `CallSiteInfo::GetMethodName`
/// — 需 InferMethodName + receiver descriptor/dictionary 遍历; 见推迟台账。
#[must_use]
pub fn get_method_name<F>(_info: &CallSiteInfoView, infer: F) -> Option<String>
where
  F: FnOnce() -> Option<String>,
{
  infer()
}

/// v8: src/objects/call-site-info.cc:567 `CallSiteInfo::GetTypeName`
/// — receiver 非空且非 undefined/null 时取 receiver 构造函数 name; 本层以 `receiver_type_name` 注入。
#[must_use]
pub fn get_type_name(
  info: &CallSiteInfoView,
  receiver_type_name: impl FnOnce() -> Option<String>,
) -> Option<String> {
  if info.is_wasm() || info.is_builtin() {
    return None;
  }
  receiver_type_name()
}

/// v8: src/objects/call-site-info.cc:592 `CallSiteInfo::GetWasmFunctionIndex`
/// — 仅 IsWasm 时有效; 从 receiver_or_instance 关联的 WasmInstanceObject 读 index。
/// 本层以 `read_index` 闭包注入 (原版 trusted_data->function_index())。
#[must_use]
pub fn get_wasm_function_index(
  info: &CallSiteInfoView,
  read_index: impl FnOnce(u32) -> u32,
) -> u32 {
  if info.is_wasm() {
    read_index(info.receiver_or_instance)
  } else {
    K_NO_WASM_FUNCTION_INDEX_INFO
  }
}

/// v8: src/objects/call-site-info.cc:597 `CallSiteInfo::GetWasmInstance`
/// — receiver_or_instance 若 IsWasm 则是 WasmInstanceObject; 本层返回 arena 索引。
#[must_use]
pub fn get_wasm_instance(info: &CallSiteInfoView) -> Option<u32> {
  if info.is_wasm() {
    Some(info.receiver_or_instance)
  } else {
    None
  }
}

/// v8: src/objects/call-site-info.cc:603 `CallSiteInfo::GetWasmModuleName`
/// — 依赖 WasmInstanceObject->module_object()->name(); 本层以 `read_module_name` 注入。
#[must_use]
pub fn get_wasm_module_name(
  info: &CallSiteInfoView,
  read_module_name: impl FnOnce(u32) -> Option<String>,
) -> Option<String> {
  if info.is_wasm() {
    read_module_name(info.receiver_or_instance)
  } else {
    None
  }
}

/// v8: src/objects/call-site-info.cc:620 `CallSiteInfo::GetSourcePosition`
/// — 若已算则返回 code_offset_or_source_position; 否则走 ComputeSourcePosition 缓存。
/// 本层以 `compute` 闭包注入 (原版 SharedFunctionInfo/BytecodeArray 表查询)。
pub fn get_source_position(info: &mut CallSiteInfoView, compute: impl FnOnce(i32) -> i32) -> i32 {
  if info.is_source_position_computed() {
    return info.code_offset_or_source_position;
  }
  let pos = compute(info.code_offset_or_source_position);
  info.code_offset_or_source_position = pos;
  info.flags |= K_IS_SOURCE_POSITION_COMPUTED_MASK;
  pos
}

/// v8: src/objects/call-site-info.cc:635 `CallSiteInfo::ComputeLocation`
/// — 组装 MessageLocation (script + start_pos + end_pos + optional SFI);
/// 本层返回 `Some(PositionInfo)` 表示成功, `None` 表示不可计算 (IsBuiltin/no-source)。
#[must_use]
pub fn compute_location(
  info: &CallSiteInfoView,
  source_position: i32,
  sfi_subject_to_debugging: bool,
  has_valid_source: bool,
  has_source_position_table: bool,
) -> Option<MessageLocationView> {
  if info.is_wasm() {
    return Some(MessageLocationView {
      start_position: source_position,
      end_position: source_position + 1,
      has_sfi: false,
    });
  }
  if info.is_builtin() {
    return None;
  }
  if !sfi_subject_to_debugging || !has_valid_source {
    return None;
  }
  let pos = if info.is_source_position_computed() || has_source_position_table {
    source_position
  } else {
    info.code_offset_or_source_position
  };
  Some(MessageLocationView {
    start_position: pos,
    end_position: pos + 1,
    has_sfi: true,
  })
}

/// 原版 MessageLocation 的最小投影 (script 索引由调用方持有)。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MessageLocationView {
  pub start_position: i32,
  pub end_position: i32,
  pub has_sfi: bool,
}

/// v8: src/objects/call-site-info.cc:668 `CallSiteInfo::ComputeSourcePosition`
/// — IsWasm (含 DRUMBRAKE 分支) → wasm::GetSourcePosition (推迟);
/// IsBuiltin → 0; 否则 AbstractCode::SourcePosition(offline)。
#[must_use]
pub fn compute_source_position(
  info: &CallSiteInfoView,
  offset: i32,
  wasm_source_position: impl FnOnce(u32, u32, i32) -> i32,
  abstract_code_source_position: impl FnOnce(Option<u32>, i32) -> i32,
) -> i32 {
  if info.is_wasm() {
    // 需 WasmInstanceObject.trusted_data->module(); 本层以 wasm_source_position 注入。
    return wasm_source_position(
      info.receiver_or_instance,
      K_NO_WASM_FUNCTION_INDEX_INFO,
      offset,
    );
  }
  if info.is_builtin() {
    return 0;
  }
  abstract_code_source_position(info.code_object, offset)
}

/// v8: src/objects/call-site-info.cc:700 `CallSiteInfo::GetScript`
/// — IsWasm→WasmInstance.script; IsBuiltin→None; 否则 SFI.script。
/// 本层以 `sfi_script` / `wasm_script` 注入。
#[must_use]
pub fn get_script(
  info: &CallSiteInfoView,
  wasm_script: impl FnOnce(u32) -> Option<u32>,
  sfi_script: impl FnOnce() -> Option<u32>,
) -> Option<u32> {
  if info.is_wasm() {
    return wasm_script(info.receiver_or_instance);
  }
  if info.is_builtin() {
    return None;
  }
  sfi_script()
}

/// v8: src/objects/call-site-info.cc:717 `CallSiteInfo::GetSharedFunctionInfo`
/// — DCHECK !IsWasm && !IsBuiltin; JSFunction->shared()。
#[must_use]
pub fn get_shared_function_info(
  info: &CallSiteInfoView,
  js_function_shared: impl FnOnce() -> u32,
) -> Option<u32> {
  debug_assert!(!info.is_wasm() && !info.is_builtin());
  Some(js_function_shared())
}

/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(K_IS_WASM_SHIFT == 0);
  assert!(K_IS_DEFERRED_BASELINE_FRAME_SHIFT == 6);
  assert!(K_IS_DEFERRED_BASELINE_FRAME_MASK == 0x40);
  assert!(CALL_SITE_INFO_FIELDS.len() == 5);
  assert!(str_eq(CALL_SITE_INFO_FIELDS[4].0, "flags"));
};
