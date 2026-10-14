// 由 tool/src/builtins/builtins-x64.js 生成; 来源 v8/src/builtins 下 1 文件
// 内建名表 + 核心调度元数据 (73 内建)
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};
use v8_execution_frames::StackFrameType;

/// x64 架构汇编内建名表 (73 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  #[strum(serialize = "AdaptShadowStackForDeopt")]
  AdaptShadowStackForDeopt,
  #[strum(serialize = "Adaptor")]
  Adaptor,
  #[strum(serialize = "BaselineOnStackReplacement")]
  BaselineOnStackReplacement,
  #[strum(serialize = "BaselineOutOfLinePrologue")]
  BaselineOutOfLinePrologue,
  #[strum(serialize = "BaselineOutOfLinePrologueDeopt")]
  BaselineOutOfLinePrologueDeopt,
  #[strum(serialize = "CEntry")]
  CEntry,
  #[strum(serialize = "Call")]
  Call,
  #[strum(serialize = "CallApiAccessorImpl")]
  CallApiAccessorImpl,
  #[strum(serialize = "CallApiCallbackImpl")]
  CallApiCallbackImpl,
  #[strum(serialize = "CallBoundFunctionImpl")]
  CallBoundFunctionImpl,
  #[strum(serialize = "CallFunction")]
  CallFunction,
  #[strum(serialize = "CallOrConstructForwardVarargs")]
  CallOrConstructForwardVarargs,
  #[strum(serialize = "CallOrConstructVarargs")]
  CallOrConstructVarargs,
  #[strum(serialize = "Construct")]
  Construct,
  #[strum(serialize = "ConstructBoundFunction")]
  ConstructBoundFunction,
  #[strum(serialize = "ConstructForwardAllArgsImpl")]
  ConstructForwardAllArgsImpl,
  #[strum(serialize = "ConstructFunction")]
  ConstructFunction,
  #[strum(serialize = "ConstructedNonConstructable")]
  ConstructedNonConstructable,
  #[strum(serialize = "ContinueToCodeStubBuiltin")]
  ContinueToCodeStubBuiltin,
  #[strum(serialize = "ContinueToCodeStubBuiltinWithResult")]
  ContinueToCodeStubBuiltinWithResult,
  #[strum(serialize = "ContinueToJavaScriptBuiltin")]
  ContinueToJavaScriptBuiltin,
  #[strum(serialize = "ContinueToJavaScriptBuiltinWithResult")]
  ContinueToJavaScriptBuiltinWithResult,
  #[strum(serialize = "DeoptimizationEntry_Eager")]
  DeoptimizationEntryEager,
  #[strum(serialize = "DeoptimizationEntry_Lazy")]
  DeoptimizationEntryLazy,
  #[strum(serialize = "DeoptimizationEntry_LazyAfterFastCall")]
  DeoptimizationEntryLazyAfterFastCall,
  #[strum(serialize = "DirectCEntry")]
  DirectCEntry,
  #[strum(serialize = "DoubleToI")]
  DoubleToI,
  #[strum(serialize = "DumpFrame")]
  DumpFrame,
  #[strum(serialize = "FunctionPrototypeApply")]
  FunctionPrototypeApply,
  #[strum(serialize = "FunctionPrototypeCall")]
  FunctionPrototypeCall,
  #[strum(serialize = "InterpreterEnterAtBytecode")]
  InterpreterEnterAtBytecode,
  #[strum(serialize = "InterpreterEnterAtNextBytecode")]
  InterpreterEnterAtNextBytecode,
  #[strum(serialize = "InterpreterEntryTrampoline")]
  InterpreterEntryTrampoline,
  #[strum(serialize = "InterpreterOnStackReplacement")]
  InterpreterOnStackReplacement,
  #[strum(serialize = "InterpreterOnStackReplacement_ToBaseline")]
  InterpreterOnStackReplacementToBaseline,
  #[strum(serialize = "InterpreterPushArgsThenCallImpl")]
  InterpreterPushArgsThenCallImpl,
  #[strum(serialize = "InterpreterPushArgsThenConstructImpl")]
  InterpreterPushArgsThenConstructImpl,
  #[strum(serialize = "InterpreterPushArgsThenFastConstructFunction")]
  InterpreterPushArgsThenFastConstructFunction,
  #[strum(serialize = "JSBuiltinsConstructStub")]
  JSBuiltinsConstructStub,
  #[strum(serialize = "JSConstructEntry")]
  JSConstructEntry,
  #[strum(serialize = "JSConstructEntryTrampoline")]
  JSConstructEntryTrampoline,
  #[strum(serialize = "JSConstructStubGeneric")]
  JSConstructStubGeneric,
  #[strum(serialize = "JSEntry")]
  JSEntry,
  #[strum(serialize = "JSEntryTrampoline")]
  JSEntryTrampoline,
  #[strum(serialize = "JSRunMicrotasksEntry")]
  JSRunMicrotasksEntry,
  #[strum(serialize = "JSToWasmStressSwitchStacksAsm")]
  JSToWasmStressSwitchStacksAsm,
  #[strum(serialize = "JSToWasmWrapperAsm")]
  JSToWasmWrapperAsm,
  #[strum(serialize = "MaglevFunctionEntryStackCheck")]
  MaglevFunctionEntryStackCheck,
  #[strum(serialize = "NotifyDeoptimized")]
  NotifyDeoptimized,
  #[strum(serialize = "ReflectApply")]
  ReflectApply,
  #[strum(serialize = "ReflectConstruct")]
  ReflectConstruct,
  #[strum(serialize = "RestartFrameTrampoline")]
  RestartFrameTrampoline,
  #[strum(serialize = "ResumeGeneratorTrampoline")]
  ResumeGeneratorTrampoline,
  #[strum(serialize = "RunMicrotasksTrampoline")]
  RunMicrotasksTrampoline,
  #[strum(serialize = "WasmCompileLazy")]
  WasmCompileLazy,
  #[strum(serialize = "WasmDebugBreak")]
  WasmDebugBreak,
  #[strum(serialize = "WasmFXResume")]
  WasmFXResume,
  #[strum(serialize = "WasmFXResumeThrow")]
  WasmFXResumeThrow,
  #[strum(serialize = "WasmFXResumeThrowRef")]
  WasmFXResumeThrowRef,
  #[strum(serialize = "WasmFXReturn")]
  WasmFXReturn,
  #[strum(serialize = "WasmFXSuspend")]
  WasmFXSuspend,
  #[strum(serialize = "WasmFXSwitch")]
  WasmFXSwitch,
  #[strum(serialize = "WasmHandleStackOverflow")]
  WasmHandleStackOverflow,
  #[strum(serialize = "WasmLiftoffFrameSetup")]
  WasmLiftoffFrameSetup,
  #[strum(serialize = "WasmOnStackReplace")]
  WasmOnStackReplace,
  #[strum(serialize = "WasmReject")]
  WasmReject,
  #[strum(serialize = "WasmResume")]
  WasmResume,
  #[strum(serialize = "WasmReturnFromSegment")]
  WasmReturnFromSegment,
  #[strum(serialize = "WasmReturnPromiseOnSuspendAsm")]
  WasmReturnPromiseOnSuspendAsm,
  #[strum(serialize = "WasmSuspend")]
  WasmSuspend,
  #[strum(serialize = "WasmToJsWrapperAsm")]
  WasmToJsWrapperAsm,
  #[strum(serialize = "WasmTrapHandlerLandingPad")]
  WasmTrapHandlerLandingPad,
  #[strum(serialize = "WasmTrapHandlerThrowTrap")]
  WasmTrapHandlerThrowTrap,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::AdaptShadowStackForDeopt => "AdaptShadowStackForDeopt",
      Self::Adaptor => "Adaptor",
      Self::BaselineOnStackReplacement => "BaselineOnStackReplacement",
      Self::BaselineOutOfLinePrologue => "BaselineOutOfLinePrologue",
      Self::BaselineOutOfLinePrologueDeopt => "BaselineOutOfLinePrologueDeopt",
      Self::CEntry => "CEntry",
      Self::Call => "Call",
      Self::CallApiAccessorImpl => "CallApiAccessorImpl",
      Self::CallApiCallbackImpl => "CallApiCallbackImpl",
      Self::CallBoundFunctionImpl => "CallBoundFunctionImpl",
      Self::CallFunction => "CallFunction",
      Self::CallOrConstructForwardVarargs => "CallOrConstructForwardVarargs",
      Self::CallOrConstructVarargs => "CallOrConstructVarargs",
      Self::Construct => "Construct",
      Self::ConstructBoundFunction => "ConstructBoundFunction",
      Self::ConstructForwardAllArgsImpl => "ConstructForwardAllArgsImpl",
      Self::ConstructFunction => "ConstructFunction",
      Self::ConstructedNonConstructable => "ConstructedNonConstructable",
      Self::ContinueToCodeStubBuiltin => "ContinueToCodeStubBuiltin",
      Self::ContinueToCodeStubBuiltinWithResult => "ContinueToCodeStubBuiltinWithResult",
      Self::ContinueToJavaScriptBuiltin => "ContinueToJavaScriptBuiltin",
      Self::ContinueToJavaScriptBuiltinWithResult => "ContinueToJavaScriptBuiltinWithResult",
      Self::DeoptimizationEntryEager => "DeoptimizationEntry_Eager",
      Self::DeoptimizationEntryLazy => "DeoptimizationEntry_Lazy",
      Self::DeoptimizationEntryLazyAfterFastCall => "DeoptimizationEntry_LazyAfterFastCall",
      Self::DirectCEntry => "DirectCEntry",
      Self::DoubleToI => "DoubleToI",
      Self::DumpFrame => "DumpFrame",
      Self::FunctionPrototypeApply => "FunctionPrototypeApply",
      Self::FunctionPrototypeCall => "FunctionPrototypeCall",
      Self::InterpreterEnterAtBytecode => "InterpreterEnterAtBytecode",
      Self::InterpreterEnterAtNextBytecode => "InterpreterEnterAtNextBytecode",
      Self::InterpreterEntryTrampoline => "InterpreterEntryTrampoline",
      Self::InterpreterOnStackReplacement => "InterpreterOnStackReplacement",
      Self::InterpreterOnStackReplacementToBaseline => "InterpreterOnStackReplacement_ToBaseline",
      Self::InterpreterPushArgsThenCallImpl => "InterpreterPushArgsThenCallImpl",
      Self::InterpreterPushArgsThenConstructImpl => "InterpreterPushArgsThenConstructImpl",
      Self::InterpreterPushArgsThenFastConstructFunction => {
        "InterpreterPushArgsThenFastConstructFunction"
      }
      Self::JSBuiltinsConstructStub => "JSBuiltinsConstructStub",
      Self::JSConstructEntry => "JSConstructEntry",
      Self::JSConstructEntryTrampoline => "JSConstructEntryTrampoline",
      Self::JSConstructStubGeneric => "JSConstructStubGeneric",
      Self::JSEntry => "JSEntry",
      Self::JSEntryTrampoline => "JSEntryTrampoline",
      Self::JSRunMicrotasksEntry => "JSRunMicrotasksEntry",
      Self::JSToWasmStressSwitchStacksAsm => "JSToWasmStressSwitchStacksAsm",
      Self::JSToWasmWrapperAsm => "JSToWasmWrapperAsm",
      Self::MaglevFunctionEntryStackCheck => "MaglevFunctionEntryStackCheck",
      Self::NotifyDeoptimized => "NotifyDeoptimized",
      Self::ReflectApply => "ReflectApply",
      Self::ReflectConstruct => "ReflectConstruct",
      Self::RestartFrameTrampoline => "RestartFrameTrampoline",
      Self::ResumeGeneratorTrampoline => "ResumeGeneratorTrampoline",
      Self::RunMicrotasksTrampoline => "RunMicrotasksTrampoline",
      Self::WasmCompileLazy => "WasmCompileLazy",
      Self::WasmDebugBreak => "WasmDebugBreak",
      Self::WasmFXResume => "WasmFXResume",
      Self::WasmFXResumeThrow => "WasmFXResumeThrow",
      Self::WasmFXResumeThrowRef => "WasmFXResumeThrowRef",
      Self::WasmFXReturn => "WasmFXReturn",
      Self::WasmFXSuspend => "WasmFXSuspend",
      Self::WasmFXSwitch => "WasmFXSwitch",
      Self::WasmHandleStackOverflow => "WasmHandleStackOverflow",
      Self::WasmLiftoffFrameSetup => "WasmLiftoffFrameSetup",
      Self::WasmOnStackReplace => "WasmOnStackReplace",
      Self::WasmReject => "WasmReject",
      Self::WasmResume => "WasmResume",
      Self::WasmReturnFromSegment => "WasmReturnFromSegment",
      Self::WasmReturnPromiseOnSuspendAsm => "WasmReturnPromiseOnSuspendAsm",
      Self::WasmSuspend => "WasmSuspend",
      Self::WasmToJsWrapperAsm => "WasmToJsWrapperAsm",
      Self::WasmTrapHandlerLandingPad => "WasmTrapHandlerLandingPad",
      Self::WasmTrapHandlerThrowTrap => "WasmTrapHandlerThrowTrap",
    }
  }

  /// 尝试关联至 v8_builtins_core 全局内建注册表
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取接口描述符名称
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 获取内建入口对应的预期栈帧类型
  ///
  /// 已按原版 v8/src/builtins/x64/builtins-x64.cc 各 Generate_* 的帧宏逐项精化
  /// (逐臂注释给出出处); 未逐臂精化的成员保留粗类启发式兜底, 真帧型面随
  /// 汇编体填波替换, 下游勿当权威语义引用。台账见 readme/zh.md。
  #[inline]
  pub const fn expected_frame_type(&self) -> Option<v8_execution_frames::StackFrameType> {
    match self {
      // Generate_JSEntry/JSRunMicrotasksEntry: Generate_JSEntryVariant(StackFrame::ENTRY) (cc:557-568)
      Self::JSEntry | Self::JSRunMicrotasksEntry => Some(StackFrameType::Entry),
      // Generate_JSConstructEntry: Generate_JSEntryVariant(StackFrame::CONSTRUCT_ENTRY) (cc:561-563; frames.h:121)
      Self::JSConstructEntry => Some(StackFrameType::ConstructEntry),
      // Generate_JSEntryTrampoline/JSConstructEntryTrampoline: helper 开 FrameScope INTERNAL (cc:620; 入口 cc:694-699)
      Self::JSEntryTrampoline | Self::JSConstructEntryTrampoline => Some(StackFrameType::Internal),
      // Generate_RunMicrotasksTrampoline: 无帧尾调 kRunMicrotasks, 不开帧 (cc:702-706)
      Self::RunMicrotasksTrampoline => None,
      // Generate_ResumeGeneratorTrampoline: 入 runtime 各支路 FrameScope INTERNAL (cc:894/908/919)
      Self::ResumeGeneratorTrampoline => Some(StackFrameType::Internal),
      // Generate_RestartFrameTrampoline: 当前帧按 INTERPRETED 处理并 LeaveFrame 重启 (cc:5741/5764)
      Self::RestartFrameTrampoline => Some(StackFrameType::Interpreted),
      // Generate_BaselineOutOfLinePrologue: EnterFrame(BASELINE) (cc:1896-1899; frames.h:124)
      Self::BaselineOutOfLinePrologue => Some(StackFrameType::Baseline),
      // Generate_BaselineOutOfLinePrologueDeopt: LeaveFrame(BASELINE) 后尾调解释器 (cc:2003)
      Self::BaselineOutOfLinePrologueDeopt => Some(StackFrameType::Baseline),
      // Generate_BaselineOnStackReplacement: 按 BaselineFrameConstants/OsrSourceTier::kBaseline 处理 (cc:3060-3067)
      Self::BaselineOnStackReplacement => Some(StackFrameType::Baseline),
      // Generate_MaglevFunctionEntryStackCheck: Maglev 序言后即刻调用, 所入帧为 MAGLEV
      // (cc:3073-3080; frames.h:125; 体内 INTERNAL FrameScope 仅是 runtime 调用暂帧)
      Self::MaglevFunctionEntryStackCheck => Some(StackFrameType::Maglev),
      // Generate_InterpreterEntryTrampoline: 手工推入建 INTERPRETED 帧 (cc:1075/1139-1144)
      Self::InterpreterEntryTrampoline => Some(StackFrameType::Interpreted),
      // Generate_InterpreterEnterAt(Next)Bytecode: 在当前 INTERPRETED 帧续跑 InterpreterFrameConstants (cc:1803-1815/1853)
      Self::InterpreterEnterAtBytecode | Self::InterpreterEnterAtNextBytecode => {
        Some(StackFrameType::Interpreted)
      }
      // Generate_InterpreterPushArgsThenFastConstructFunction: EnterFrame(FAST_CONSTRUCT) (cc:1598-1599; frames.h:135)
      Self::InterpreterPushArgsThenFastConstructFunction => Some(StackFrameType::FastConstruct),
      // Generate_ContinueTo*: 入口即在 CEntry 所建 BUILTIN_CONTINUATION 帧上按其常量弹出续跑
      // (helper cc:2009-2067, 四包装 cc:2069-2085; frames.h:129)
      Self::ContinueToCodeStubBuiltin
      | Self::ContinueToCodeStubBuiltinWithResult
      | Self::ContinueToJavaScriptBuiltin
      | Self::ContinueToJavaScriptBuiltinWithResult => Some(StackFrameType::BuiltinContinuation),
      // Generate_NotifyDeoptimized: FrameScope INTERNAL 后呼 runtime (cc:2087-2090)
      Self::NotifyDeoptimized => Some(StackFrameType::Internal),
      // 未逐臂精化成员 (PushArgsThen{Call,Construct}Impl/ConstructForwardAllArgsImpl 等): 启发式粗类兜底
      _ => {
        if self.is_interpreter_or_baseline() {
          Some(StackFrameType::Interpreted)
        } else {
          None
        }
      }
    }
  }

  /// 获取 x64 架构下内建默认使用的根寄存器 (r13)
  #[inline]
  pub const fn root_register() -> v8_codegen_x64::Register {
    v8_codegen_x64::R13
  }

  // 以下 6 谓词为启发式语义归组 (8+10+15+8+11+21=73 互斥完备), 非 V8 真分类面;
  // 仅供本 crate 名面自洽与测试分区, 不可被下游当权威语义; 成员归属已按原版
  // Generate_* 帧宏面订正 (ContinueTo* 四项按 CEntry 续执行桩归 runtime 辅助组);
  // 真分类面 (帧型/MOPs/寄存器分配) 随汇编体填波替换。台账见 readme/zh.md。

  /// 是否为 JS 入口或运行蹦床 (启发式归组; 帧型面已在 expected_frame_type 逐项精化)
  #[inline]
  pub const fn is_js_entry_or_trampoline(&self) -> bool {
    matches!(
      self,
      Self::JSConstructEntry
        | Self::JSConstructEntryTrampoline
        | Self::JSEntry
        | Self::JSEntryTrampoline
        | Self::JSRunMicrotasksEntry
        | Self::RestartFrameTrampoline
        | Self::ResumeGeneratorTrampoline
        | Self::RunMicrotasksTrampoline
    )
  }

  /// 是否为解释器或基线编译器辅助入口 (启发式归组; Baseline/Maglev/帧面已精化)
  #[inline]
  pub const fn is_interpreter_or_baseline(&self) -> bool {
    matches!(
      self,
      Self::BaselineOutOfLinePrologue
        | Self::BaselineOutOfLinePrologueDeopt
        | Self::ConstructForwardAllArgsImpl
        | Self::InterpreterEnterAtBytecode
        | Self::InterpreterEnterAtNextBytecode
        | Self::InterpreterEntryTrampoline
        | Self::InterpreterPushArgsThenCallImpl
        | Self::InterpreterPushArgsThenConstructImpl
        | Self::InterpreterPushArgsThenFastConstructFunction
        | Self::MaglevFunctionEntryStackCheck
    )
  }

  /// 是否为函数调用或对象构造入口 (启发式归组)
  #[inline]
  pub const fn is_call_or_construct(&self) -> bool {
    matches!(
      self,
      Self::Call
        | Self::CallBoundFunctionImpl
        | Self::CallFunction
        | Self::CallOrConstructForwardVarargs
        | Self::CallOrConstructVarargs
        | Self::Construct
        | Self::ConstructBoundFunction
        | Self::ConstructFunction
        | Self::ConstructedNonConstructable
        | Self::FunctionPrototypeApply
        | Self::FunctionPrototypeCall
        | Self::JSBuiltinsConstructStub
        | Self::JSConstructStubGeneric
        | Self::ReflectApply
        | Self::ReflectConstruct
    )
  }

  /// 是否为反优化或 OSR 栈替换入口 (8 项, 原版真 deopt/OSR 面:
  /// DeoptimizationEntry_* cc:5573-5583, *OnStackReplacement cc:3052-3068,
  /// AdaptShadowStackForDeopt cc:3168, NotifyDeoptimized cc:2087)
  #[inline]
  pub const fn is_deopt_or_osr(&self) -> bool {
    matches!(
      self,
      Self::AdaptShadowStackForDeopt
        | Self::BaselineOnStackReplacement
        | Self::DeoptimizationEntryEager
        | Self::DeoptimizationEntryLazy
        | Self::DeoptimizationEntryLazyAfterFastCall
        | Self::InterpreterOnStackReplacement
        | Self::InterpreterOnStackReplacementToBaseline
        | Self::NotifyDeoptimized
    )
  }

  /// 是否为运行时调用与辅助桩入口 (11 项; 原名 is_c_entry_or_api, 轮7 W7BXM 按
  /// 原版归类订正组名: CEntry/DirectCEntry 运行时调用主体, CallApiCallbackImpl/
  /// CallApiAccessorImpl C++ API 回调, Adaptor 参数适配桩, ContinueTo* 四项为
  /// CEntry 续执行桩 (Generate_ContinueToBuiltinHelper 按 BuiltinContinuationFrame
  /// 常量弹出续跑, cc:2009-2085, 自 deopt 组订正归入), DoubleToI 数值转换桩
  /// (cc:4753, 无帧宏), DumpFrame 帧 dump 辅助桩 (cc:5388, x64 独有))
  #[inline]
  pub const fn is_runtime_or_aux(&self) -> bool {
    matches!(
      self,
      Self::Adaptor
        | Self::CEntry
        | Self::CallApiAccessorImpl
        | Self::CallApiCallbackImpl
        | Self::ContinueToCodeStubBuiltin
        | Self::ContinueToCodeStubBuiltinWithResult
        | Self::ContinueToJavaScriptBuiltin
        | Self::ContinueToJavaScriptBuiltinWithResult
        | Self::DirectCEntry
        | Self::DoubleToI
        | Self::DumpFrame
    )
  }

  /// 是否为 WebAssembly 汇编运行入口 (启发式归组; 真帧型面 WASM_* 随汇编体填波精化)
  #[inline]
  pub const fn is_wasm(&self) -> bool {
    matches!(
      self,
      Self::JSToWasmStressSwitchStacksAsm
        | Self::JSToWasmWrapperAsm
        | Self::WasmCompileLazy
        | Self::WasmDebugBreak
        | Self::WasmFXResume
        | Self::WasmFXResumeThrow
        | Self::WasmFXResumeThrowRef
        | Self::WasmFXReturn
        | Self::WasmFXSuspend
        | Self::WasmFXSwitch
        | Self::WasmHandleStackOverflow
        | Self::WasmLiftoffFrameSetup
        | Self::WasmOnStackReplace
        | Self::WasmReject
        | Self::WasmResume
        | Self::WasmReturnFromSegment
        | Self::WasmReturnPromiseOnSuspendAsm
        | Self::WasmSuspend
        | Self::WasmToJsWrapperAsm
        | Self::WasmTrapHandlerLandingPad
        | Self::WasmTrapHandlerThrowTrap
    )
  }

  /// 是否为 x64 平台独有内建
  #[inline]
  pub const fn is_x64_specific(&self) -> bool {
    matches!(self, Self::AdaptShadowStackForDeopt | Self::DumpFrame)
  }
}

// 统一提供 73 项零成本安全工厂函数
#[inline]
pub const fn adapt_shadow_stack_for_deopt() -> Builtin {
  Builtin::AdaptShadowStackForDeopt
}

#[inline]
pub const fn adaptor() -> Builtin {
  Builtin::Adaptor
}

#[inline]
pub const fn baseline_on_stack_replacement() -> Builtin {
  Builtin::BaselineOnStackReplacement
}

#[inline]
pub const fn baseline_out_of_line_prologue() -> Builtin {
  Builtin::BaselineOutOfLinePrologue
}

#[inline]
pub const fn baseline_out_of_line_prologue_deopt() -> Builtin {
  Builtin::BaselineOutOfLinePrologueDeopt
}

#[inline]
pub const fn c_entry() -> Builtin {
  Builtin::CEntry
}

#[inline]
pub const fn call() -> Builtin {
  Builtin::Call
}

#[inline]
pub const fn call_api_accessor_impl() -> Builtin {
  Builtin::CallApiAccessorImpl
}

#[inline]
pub const fn call_api_callback_impl() -> Builtin {
  Builtin::CallApiCallbackImpl
}

#[inline]
pub const fn call_bound_function_impl() -> Builtin {
  Builtin::CallBoundFunctionImpl
}

#[inline]
pub const fn call_function() -> Builtin {
  Builtin::CallFunction
}

#[inline]
pub const fn call_or_construct_forward_varargs() -> Builtin {
  Builtin::CallOrConstructForwardVarargs
}

#[inline]
pub const fn call_or_construct_varargs() -> Builtin {
  Builtin::CallOrConstructVarargs
}

#[inline]
pub const fn construct() -> Builtin {
  Builtin::Construct
}

#[inline]
pub const fn construct_bound_function() -> Builtin {
  Builtin::ConstructBoundFunction
}

#[inline]
pub const fn construct_forward_all_args_impl() -> Builtin {
  Builtin::ConstructForwardAllArgsImpl
}

#[inline]
pub const fn construct_function() -> Builtin {
  Builtin::ConstructFunction
}

#[inline]
pub const fn constructed_non_constructable() -> Builtin {
  Builtin::ConstructedNonConstructable
}

#[inline]
pub const fn continue_to_code_stub_builtin() -> Builtin {
  Builtin::ContinueToCodeStubBuiltin
}

#[inline]
pub const fn continue_to_code_stub_builtin_with_result() -> Builtin {
  Builtin::ContinueToCodeStubBuiltinWithResult
}

#[inline]
pub const fn continue_to_java_script_builtin() -> Builtin {
  Builtin::ContinueToJavaScriptBuiltin
}

#[inline]
pub const fn continue_to_java_script_builtin_with_result() -> Builtin {
  Builtin::ContinueToJavaScriptBuiltinWithResult
}

#[inline]
pub const fn deoptimization_entry_eager() -> Builtin {
  Builtin::DeoptimizationEntryEager
}

#[inline]
pub const fn deoptimization_entry_lazy() -> Builtin {
  Builtin::DeoptimizationEntryLazy
}

#[inline]
pub const fn deoptimization_entry_lazy_after_fast_call() -> Builtin {
  Builtin::DeoptimizationEntryLazyAfterFastCall
}

#[inline]
pub const fn direct_c_entry() -> Builtin {
  Builtin::DirectCEntry
}

#[inline]
pub const fn double_to_i() -> Builtin {
  Builtin::DoubleToI
}

#[inline]
pub const fn dump_frame() -> Builtin {
  Builtin::DumpFrame
}

#[inline]
pub const fn function_prototype_apply() -> Builtin {
  Builtin::FunctionPrototypeApply
}

#[inline]
pub const fn function_prototype_call() -> Builtin {
  Builtin::FunctionPrototypeCall
}

#[inline]
pub const fn interpreter_enter_at_bytecode() -> Builtin {
  Builtin::InterpreterEnterAtBytecode
}

#[inline]
pub const fn interpreter_enter_at_next_bytecode() -> Builtin {
  Builtin::InterpreterEnterAtNextBytecode
}

#[inline]
pub const fn interpreter_entry_trampoline() -> Builtin {
  Builtin::InterpreterEntryTrampoline
}

#[inline]
pub const fn interpreter_on_stack_replacement() -> Builtin {
  Builtin::InterpreterOnStackReplacement
}

#[inline]
pub const fn interpreter_on_stack_replacement_to_baseline() -> Builtin {
  Builtin::InterpreterOnStackReplacementToBaseline
}

#[inline]
pub const fn interpreter_push_args_then_call_impl() -> Builtin {
  Builtin::InterpreterPushArgsThenCallImpl
}

#[inline]
pub const fn interpreter_push_args_then_construct_impl() -> Builtin {
  Builtin::InterpreterPushArgsThenConstructImpl
}

#[inline]
pub const fn interpreter_push_args_then_fast_construct_function() -> Builtin {
  Builtin::InterpreterPushArgsThenFastConstructFunction
}

#[inline]
pub const fn js_builtins_construct_stub() -> Builtin {
  Builtin::JSBuiltinsConstructStub
}

#[inline]
pub const fn js_construct_entry() -> Builtin {
  Builtin::JSConstructEntry
}

#[inline]
pub const fn js_construct_entry_trampoline() -> Builtin {
  Builtin::JSConstructEntryTrampoline
}

#[inline]
pub const fn js_construct_stub_generic() -> Builtin {
  Builtin::JSConstructStubGeneric
}

#[inline]
pub const fn js_entry() -> Builtin {
  Builtin::JSEntry
}

#[inline]
pub const fn js_entry_trampoline() -> Builtin {
  Builtin::JSEntryTrampoline
}

#[inline]
pub const fn js_run_microtasks_entry() -> Builtin {
  Builtin::JSRunMicrotasksEntry
}

#[inline]
pub const fn js_to_wasm_stress_switch_stacks_asm() -> Builtin {
  Builtin::JSToWasmStressSwitchStacksAsm
}

#[inline]
pub const fn js_to_wasm_wrapper_asm() -> Builtin {
  Builtin::JSToWasmWrapperAsm
}

#[inline]
pub const fn maglev_function_entry_stack_check() -> Builtin {
  Builtin::MaglevFunctionEntryStackCheck
}

#[inline]
pub const fn notify_deoptimized() -> Builtin {
  Builtin::NotifyDeoptimized
}

#[inline]
pub const fn reflect_apply() -> Builtin {
  Builtin::ReflectApply
}

#[inline]
pub const fn reflect_construct() -> Builtin {
  Builtin::ReflectConstruct
}

#[inline]
pub const fn restart_frame_trampoline() -> Builtin {
  Builtin::RestartFrameTrampoline
}

#[inline]
pub const fn resume_generator_trampoline() -> Builtin {
  Builtin::ResumeGeneratorTrampoline
}

#[inline]
pub const fn run_microtasks_trampoline() -> Builtin {
  Builtin::RunMicrotasksTrampoline
}

#[inline]
pub const fn wasm_compile_lazy() -> Builtin {
  Builtin::WasmCompileLazy
}

#[inline]
pub const fn wasm_debug_break() -> Builtin {
  Builtin::WasmDebugBreak
}

#[inline]
pub const fn wasm_fx_resume() -> Builtin {
  Builtin::WasmFXResume
}

#[inline]
pub const fn wasm_fx_resume_throw() -> Builtin {
  Builtin::WasmFXResumeThrow
}

#[inline]
pub const fn wasm_fx_resume_throw_ref() -> Builtin {
  Builtin::WasmFXResumeThrowRef
}

#[inline]
pub const fn wasm_fx_return() -> Builtin {
  Builtin::WasmFXReturn
}

#[inline]
pub const fn wasm_fx_suspend() -> Builtin {
  Builtin::WasmFXSuspend
}

#[inline]
pub const fn wasm_fx_switch() -> Builtin {
  Builtin::WasmFXSwitch
}

#[inline]
pub const fn wasm_handle_stack_overflow() -> Builtin {
  Builtin::WasmHandleStackOverflow
}

#[inline]
pub const fn wasm_liftoff_frame_setup() -> Builtin {
  Builtin::WasmLiftoffFrameSetup
}

#[inline]
pub const fn wasm_on_stack_replace() -> Builtin {
  Builtin::WasmOnStackReplace
}

#[inline]
pub const fn wasm_reject() -> Builtin {
  Builtin::WasmReject
}

#[inline]
pub const fn wasm_resume() -> Builtin {
  Builtin::WasmResume
}

#[inline]
pub const fn wasm_return_from_segment() -> Builtin {
  Builtin::WasmReturnFromSegment
}

#[inline]
pub const fn wasm_return_promise_on_suspend_asm() -> Builtin {
  Builtin::WasmReturnPromiseOnSuspendAsm
}

#[inline]
pub const fn wasm_suspend() -> Builtin {
  Builtin::WasmSuspend
}

#[inline]
pub const fn wasm_to_js_wrapper_asm() -> Builtin {
  Builtin::WasmToJsWrapperAsm
}

#[inline]
pub const fn wasm_trap_handler_landing_pad() -> Builtin {
  Builtin::WasmTrapHandlerLandingPad
}

#[inline]
pub const fn wasm_trap_handler_throw_trap() -> Builtin {
  Builtin::WasmTrapHandlerThrowTrap
}
