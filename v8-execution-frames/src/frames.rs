// 由 tool/src/execution/frames.js 从 v8/src/execution/frames.h, src/execution/frames.cc, src/execution/frames-inl.h, src/execution/frame-constants.h 生成; 修订就地改, 复跑覆盖
// 栈帧类型表 34 项 (23 标准 + 11 wasm) 与栈帧遍历抽象

/// 栈帧类型 (对应原版 StackFrame::Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum StackFrameType {
  /// 无帧类型标记
  NoFrameType = 0,
  /// 原版 ENTRY (EntryFrame)
  Entry = 1,
  /// 原版 CONSTRUCT_ENTRY (ConstructEntryFrame)
  ConstructEntry = 2,
  /// 原版 EXIT (ExitFrame)
  Exit = 3,
  /// 原版 INTERPRETED (InterpretedFrame)
  Interpreted = 4,
  /// 原版 BASELINE (BaselineFrame)
  Baseline = 5,
  /// 原版 MAGLEV (MaglevFrame)
  Maglev = 6,
  /// 原版 TURBOFAN_JS (TurbofanJSFrame)
  TurbofanJs = 7,
  /// 原版 STUB (StubFrame)
  Stub = 8,
  /// 原版 TURBOFAN_STUB_WITH_CONTEXT (TurbofanStubWithContextFrame)
  TurbofanStubWithContext = 9,
  /// 原版 BUILTIN_CONTINUATION (BuiltinContinuationFrame)
  BuiltinContinuation = 10,
  /// 原版 JAVASCRIPT_BUILTIN_CONTINUATION (JavaScriptBuiltinContinuationFrame)
  JavascriptBuiltinContinuation = 11,
  /// 原版 JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH (JavaScriptBuiltinContinuationWithCatchFrame)
  JavascriptBuiltinContinuationWithCatch = 12,
  /// 原版 INTERNAL (InternalFrame)
  Internal = 13,
  /// 原版 CONSTRUCT (ConstructFrame)
  Construct = 14,
  /// 原版 FAST_CONSTRUCT (FastConstructFrame)
  FastConstruct = 15,
  /// 原版 BUILTIN (BuiltinFrame)
  Builtin = 16,
  /// 原版 BUILTIN_EXIT (BuiltinExitFrame)
  BuiltinExit = 17,
  /// 原版 API_CALLBACK_EXIT (ApiCallbackExitFrame)
  ApiCallbackExit = 18,
  /// 原版 API_CONSTRUCT_EXIT (ApiConstructExitFrame)
  ApiConstructExit = 19,
  /// 原版 API_NAMED_ACCESSOR_EXIT (ApiNamedAccessorExitFrame)
  ApiNamedAccessorExit = 20,
  /// 原版 API_INDEXED_ACCESSOR_EXIT (ApiIndexedAccessorExitFrame)
  ApiIndexedAccessorExit = 21,
  /// 原版 NATIVE (NativeFrame)
  Native = 22,
  /// 原版 IRREGEXP (IrregexpFrame)
  Irregexp = 23,
  /// 原版 WASM (WasmFrame)
  Wasm = 24,
  /// 原版 WASM_TO_JS (WasmToJsFrame)
  WasmToJs = 25,
  /// 原版 JS_TO_WASM (JsToWasmFrame)
  JsToWasm = 26,
  /// 原版 WASM_JSPI (WasmJspiFrame)
  WasmJspi = 27,
  /// 原版 WASM_INTERPRETER_ENTRY (WasmInterpreterEntryFrame)
  WasmInterpreterEntry = 28,
  /// 原版 WASM_DEBUG_BREAK (WasmDebugBreakFrame)
  WasmDebugBreak = 29,
  /// 原版 C_WASM_ENTRY (CWasmEntryFrame)
  CWasmEntry = 30,
  /// 原版 WASM_STACK_ENTRY (WasmStackEntryFrame)
  WasmStackEntry = 31,
  /// 原版 WASM_STACK_EXIT (WasmStackExitFrame)
  WasmStackExit = 32,
  /// 原版 WASM_EXIT (WasmExitFrame)
  WasmExit = 33,
  /// 原版 WASM_LIFTOFF_SETUP (WasmLiftoffSetupFrame)
  WasmLiftoffSetup = 34,
  /// 帧类型总数哨兵 (对应原版 NUMBER_OF_TYPES)
  NumberOfTypes = 35,
  /// 手工构造帧标记 (对应原版 MANUAL)
  Manual = 36,
}

impl StackFrameType {
  pub const COUNT: usize = 34;

  /// 全部具体帧类型清单 (排除 NoFrameType, NumberOfTypes, Manual)
  pub const ALL: [StackFrameType; 34] = [
    StackFrameType::Entry,
    StackFrameType::ConstructEntry,
    StackFrameType::Exit,
    StackFrameType::Interpreted,
    StackFrameType::Baseline,
    StackFrameType::Maglev,
    StackFrameType::TurbofanJs,
    StackFrameType::Stub,
    StackFrameType::TurbofanStubWithContext,
    StackFrameType::BuiltinContinuation,
    StackFrameType::JavascriptBuiltinContinuation,
    StackFrameType::JavascriptBuiltinContinuationWithCatch,
    StackFrameType::Internal,
    StackFrameType::Construct,
    StackFrameType::FastConstruct,
    StackFrameType::Builtin,
    StackFrameType::BuiltinExit,
    StackFrameType::ApiCallbackExit,
    StackFrameType::ApiConstructExit,
    StackFrameType::ApiNamedAccessorExit,
    StackFrameType::ApiIndexedAccessorExit,
    StackFrameType::Native,
    StackFrameType::Irregexp,
    StackFrameType::Wasm,
    StackFrameType::WasmToJs,
    StackFrameType::JsToWasm,
    StackFrameType::WasmJspi,
    StackFrameType::WasmInterpreterEntry,
    StackFrameType::WasmDebugBreak,
    StackFrameType::CWasmEntry,
    StackFrameType::WasmStackEntry,
    StackFrameType::WasmStackExit,
    StackFrameType::WasmExit,
    StackFrameType::WasmLiftoffSetup,
  ];

  /// 是否为严格 JavaScriptFrame (可安全下转型为 JavaScriptFrame, 对应原版 IsJavaScript)
  #[inline]
  pub const fn is_javascript(self) -> bool {
    matches!(
      self,
      StackFrameType::Interpreted
        | StackFrameType::Baseline
        | StackFrameType::Maglev
        | StackFrameType::TurbofanJs
    )
  }

  /// 同 `is_javascript` (对应原版 is_javascript())
  #[inline]
  pub const fn is_js(self) -> bool {
    self.is_javascript()
  }

  /// 是否属于广义 JS 相关帧 (包含 BuiltinContinuation 族)
  #[inline]
  pub const fn is_js_family(self) -> bool {
    self.is_javascript()
      || matches!(
        self,
        StackFrameType::JavascriptBuiltinContinuation
          | StackFrameType::JavascriptBuiltinContinuationWithCatch
      )
  }

  /// 是否为严格 WebAssembly 基础帧 (对应原版 is_wasm())
  #[inline]
  pub const fn is_wasm(self) -> bool {
    matches!(
      self,
      StackFrameType::Wasm | StackFrameType::WasmInterpreterEntry
    )
  }

  /// 是否属于广义 WebAssembly 帧家族 (包含 11 种全部 WASM 帧)
  #[inline]
  pub const fn is_wasm_family(self) -> bool {
    matches!(self, |StackFrameType::Wasm| StackFrameType::WasmToJs
      | StackFrameType::JsToWasm
      | StackFrameType::WasmJspi
      | StackFrameType::WasmInterpreterEntry
      | StackFrameType::WasmDebugBreak
      | StackFrameType::CWasmEntry
      | StackFrameType::WasmStackEntry
      | StackFrameType::WasmStackExit
      | StackFrameType::WasmExit
      | StackFrameType::WasmLiftoffSetup)
  }

  /// 是否为严格 ExitFrame (进入 C++ 运行时, 对应原版 is_exit())
  #[inline]
  pub const fn is_exit(self) -> bool {
    matches!(self, StackFrameType::Exit)
  }

  /// 是否属于广义退出帧家族 (包含内建与 API 回调退出帧)
  #[inline]
  pub const fn is_exit_family(self) -> bool {
    matches!(
      self,
      StackFrameType::Exit
        | StackFrameType::BuiltinExit
        | StackFrameType::ApiCallbackExit
        | StackFrameType::ApiConstructExit
        | StackFrameType::ApiNamedAccessorExit
        | StackFrameType::ApiIndexedAccessorExit
        | StackFrameType::WasmExit
    )
  }

  /// 对应原版大写标识符
  #[inline]
  pub const fn name(self) -> &'static str {
    match self {
      StackFrameType::NoFrameType => "NO_FRAME_TYPE",
      StackFrameType::Entry => "ENTRY",
      StackFrameType::ConstructEntry => "CONSTRUCT_ENTRY",
      StackFrameType::Exit => "EXIT",
      StackFrameType::Interpreted => "INTERPRETED",
      StackFrameType::Baseline => "BASELINE",
      StackFrameType::Maglev => "MAGLEV",
      StackFrameType::TurbofanJs => "TURBOFAN_JS",
      StackFrameType::Stub => "STUB",
      StackFrameType::TurbofanStubWithContext => "TURBOFAN_STUB_WITH_CONTEXT",
      StackFrameType::BuiltinContinuation => "BUILTIN_CONTINUATION",
      StackFrameType::JavascriptBuiltinContinuation => "JAVASCRIPT_BUILTIN_CONTINUATION",
      StackFrameType::JavascriptBuiltinContinuationWithCatch => {
        "JAVASCRIPT_BUILTIN_CONTINUATION_WITH_CATCH"
      }
      StackFrameType::Internal => "INTERNAL",
      StackFrameType::Construct => "CONSTRUCT",
      StackFrameType::FastConstruct => "FAST_CONSTRUCT",
      StackFrameType::Builtin => "BUILTIN",
      StackFrameType::BuiltinExit => "BUILTIN_EXIT",
      StackFrameType::ApiCallbackExit => "API_CALLBACK_EXIT",
      StackFrameType::ApiConstructExit => "API_CONSTRUCT_EXIT",
      StackFrameType::ApiNamedAccessorExit => "API_NAMED_ACCESSOR_EXIT",
      StackFrameType::ApiIndexedAccessorExit => "API_INDEXED_ACCESSOR_EXIT",
      StackFrameType::Native => "NATIVE",
      StackFrameType::Irregexp => "IRREGEXP",
      StackFrameType::Wasm => "WASM",
      StackFrameType::WasmToJs => "WASM_TO_JS",
      StackFrameType::JsToWasm => "JS_TO_WASM",
      StackFrameType::WasmJspi => "WASM_JSPI",
      StackFrameType::WasmInterpreterEntry => "WASM_INTERPRETER_ENTRY",
      StackFrameType::WasmDebugBreak => "WASM_DEBUG_BREAK",
      StackFrameType::CWasmEntry => "C_WASM_ENTRY",
      StackFrameType::WasmStackEntry => "WASM_STACK_ENTRY",
      StackFrameType::WasmStackExit => "WASM_STACK_EXIT",
      StackFrameType::WasmExit => "WASM_EXIT",
      StackFrameType::WasmLiftoffSetup => "WASM_LIFTOFF_SETUP",
      StackFrameType::NumberOfTypes => "NUMBER_OF_TYPES",
      StackFrameType::Manual => "MANUAL",
    }
  }
}

/// 内建调用续延模式 (对应原版 BuiltinContinuationMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BuiltinContinuationMode {
  /// 普通机器码桩续延
  Stub = 0,
  /// JavaScript 续延
  JavaScript = 1,
  /// 带异常捕获的 JavaScript 续延
  JavaScriptWithCatch = 2,
  /// 异常处理分发续延
  JavaScriptHandleException = 3,
}

/// 栈帧信息精度模式 (对应原版 FrameInfoKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FrameInfoKind {
  /// 精确模式
  Precise = 0,
  /// 保守模式
  Conservative = 1,
}

/// JS 入口标记 (对应原版 JsFrameMarker, Smi 编码以防 GC 栈扫描误判为堆指针)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JsFrameMarker {
  /// 内部 JS 入口帧 ((0 << kSmiTagSize) | kSmiTag = 0)
  InnerJsEntryFrame = 0,
  /// 最外层 JS 入口帧 ((1 << kSmiTagSize) | kSmiTag = 2)
  OutermostJsEntryFrame = 2,
}

/// 栈帧指针与遍历抽象
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackFrame {
  /// 帧基址指针 (Frame Pointer)
  fp: usize,
  /// 栈顶指针 (Stack Pointer)
  sp: usize,
  /// 栈帧类型
  frame_type: StackFrameType,
}

impl StackFrame {
  /// 构造栈帧描述
  #[inline]
  pub const fn new(fp: usize, sp: usize, frame_type: StackFrameType) -> Self {
    Self { fp, sp, frame_type }
  }

  /// 获取 FP
  #[inline]
  pub const fn fp(&self) -> usize {
    self.fp
  }

  /// 获取 SP
  #[inline]
  pub const fn sp(&self) -> usize {
    self.sp
  }

  /// 获取帧类型
  #[inline]
  pub const fn frame_type(&self) -> StackFrameType {
    self.frame_type
  }

  /// 计算 caller 的标准 FP 偏移位置 (以指针步长 8 为基准)
  #[inline]
  pub const fn caller_fp_address(&self) -> usize {
    self.fp
  }

  /// 计算 caller 的标准返回地址 (PC) 槽偏移位置
  #[inline]
  pub const fn caller_pc_address(&self) -> usize {
    self.fp + 8
  }

  /// 计算 caller 的标准栈顶 (SP) 偏移位置
  #[inline]
  pub const fn caller_sp_address(&self) -> usize {
    self.fp + 16
  }
}

// ── 编译期静态断言 ──
const _: () = {
  assert!(StackFrameType::COUNT == 34);
  assert!(StackFrameType::NoFrameType as u8 == 0);
  assert!(BuiltinContinuationMode::JavaScriptWithCatch as u8 == 2);
  assert!(JsFrameMarker::InnerJsEntryFrame as u8 == 0);
  assert!(JsFrameMarker::OutermostJsEntryFrame as u8 == 2);
  assert!(StackFrameType::Interpreted.is_javascript());
  assert!(!StackFrameType::JavascriptBuiltinContinuation.is_javascript());
  assert!(StackFrameType::JavascriptBuiltinContinuation.is_js_family());
  assert!(StackFrameType::Exit.is_exit());
  assert!(!StackFrameType::BuiltinExit.is_exit());
  assert!(StackFrameType::BuiltinExit.is_exit_family());
};
