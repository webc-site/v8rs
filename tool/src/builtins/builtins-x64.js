#!/usr/bin/env bun
// tool/src/builtins/builtins-x64.js — v8-builtins-x64 基线生成 (汇编内建)
// 来源: x64/builtins-x64.cc (JSEntry/InterpreterEntryTrampoline/CEntry/Call/Construct)
// → v8-builtins-x64/src/entry.rs + lib.rs
import { anchors, ccBuiltins, fmt, fnName, readV8, writeSrc } from "./builtins-lib.js";

const CRATE = "v8-builtins-x64";
const GENERATOR = "builtins-x64.js";
const CC_FILES = ["x64/builtins-x64.cc"];
const ANCHORS = [
  ["void Builtins::Generate_JSEntry(MacroAssembler* masm)", "x64/builtins-x64.cc"],
  ["void Builtins::Generate_Adaptor(MacroAssembler* masm,", "x64/builtins-x64.cc"],
];

// 产物头
const head = (note) =>
  `// 由 tool/src/builtins/${GENERATOR} 生成; 来源 v8/src/builtins 下 ${CC_FILES.length} 文件\n// ${note}\n// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义\n`;

// 6 大语义分类集合
const JS_ENTRY_OR_TRAMPOLINE = new Set([
  "JSEntry", "JSConstructEntry", "JSRunMicrotasksEntry",
  "JSEntryTrampoline", "JSConstructEntryTrampoline", "RunMicrotasksTrampoline",
  "ResumeGeneratorTrampoline", "RestartFrameTrampoline"
]);

const INTERPRETER_OR_BASELINE = new Set([
  "InterpreterEntryTrampoline", "InterpreterPushArgsThenCallImpl",
  "InterpreterPushArgsThenConstructImpl", "ConstructForwardAllArgsImpl",
  "InterpreterPushArgsThenFastConstructFunction", "InterpreterEnterAtNextBytecode",
  "InterpreterEnterAtBytecode", "BaselineOutOfLinePrologue",
  "BaselineOutOfLinePrologueDeopt", "MaglevFunctionEntryStackCheck"
]);

const CALL_OR_CONSTRUCT = new Set([
  "CallOrConstructVarargs", "CallOrConstructForwardVarargs", "CallFunction",
  "CallBoundFunctionImpl", "Call", "ConstructFunction", "ConstructBoundFunction",
  "Construct", "JSConstructStubGeneric", "JSBuiltinsConstructStub",
  "ConstructedNonConstructable", "FunctionPrototypeApply", "FunctionPrototypeCall",
  "ReflectApply", "ReflectConstruct"
]);

const DEOPT_OR_OSR = new Set([
  "AdaptShadowStackForDeopt", "BaselineOnStackReplacement",
  "DeoptimizationEntry_Eager", "DeoptimizationEntry_Lazy",
  "DeoptimizationEntry_LazyAfterFastCall", "InterpreterOnStackReplacement",
  "InterpreterOnStackReplacement_ToBaseline", "NotifyDeoptimized"
]);

const RUNTIME_OR_AUX = new Set([
  "Adaptor", "CEntry", "CallApiAccessorImpl", "CallApiCallbackImpl",
  "ContinueToCodeStubBuiltin", "ContinueToCodeStubBuiltinWithResult",
  "ContinueToJavaScriptBuiltin", "ContinueToJavaScriptBuiltinWithResult",
  "DirectCEntry", "DoubleToI", "DumpFrame"
]);

const WASM = new Set([
  "WasmLiftoffFrameSetup", "WasmCompileLazy", "WasmDebugBreak",
  "WasmTrapHandlerThrowTrap", "JSToWasmWrapperAsm", "WasmReturnPromiseOnSuspendAsm",
  "JSToWasmStressSwitchStacksAsm", "WasmToJsWrapperAsm", "WasmTrapHandlerLandingPad",
  "WasmSuspend", "WasmResume", "WasmReject", "WasmFXResume", "WasmFXResumeThrow",
  "WasmFXResumeThrowRef", "WasmFXSuspend", "WasmFXSwitch", "WasmFXReturn",
  "WasmOnStackReplace", "WasmHandleStackOverflow", "WasmReturnFromSegment"
]);

const X64_SPECIFIC = new Set(["AdaptShadowStackForDeopt", "DumpFrame"]);

function getVariantName(name) {
  return name.replace(/_/g, "");
}

// modBody: 内建名表 + 方法 + 零成本工厂函数
function modBody(builtins) {
  const o = [];
  o.push(head(`内建名表 + 核心调度元数据 (${builtins.length} 内建)`));
  o.push('use strum::{Display, EnumIter, EnumString};');
  o.push('use v8_builtins_core::{descriptor_name, find_builtin};');
  o.push('use v8_execution_frames::StackFrameType;\n');
  o.push('/// x64 架构汇编内建名表 (73 项)');
  o.push('#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]');
  o.push('pub enum Builtin {');
  for (const b of builtins) {
    const v = getVariantName(b.name);
    o.push(`  #[strum(serialize = "${b.name}")]`);
    o.push(`  ${v},`);
  }
  o.push('}\n');

  o.push('impl Builtin {');
  o.push('  /// 获取内建的 V8 原版规范名称');
  o.push('  #[inline]');
  o.push('  pub const fn name(&self) -> &\'static str {');
  o.push('    match self {');
  for (const b of builtins) {
    o.push(`      Self::${getVariantName(b.name)} => "${b.name}",`);
  }
  o.push('    }');
  o.push('  }\n');

  o.push('  /// 尝试关联至 v8_builtins_core 全局内建注册表');
  o.push('  #[inline]');
  o.push('  pub fn global_id(&self) -> Option<usize> {');
  o.push('    find_builtin(self.name())');
  o.push('  }\n');
  o.push('  /// 获取接口描述符名称');
  o.push('  #[inline]');
  o.push('  pub fn descriptor_name(&self) -> String {');
  o.push('    descriptor_name(self.name())');
  o.push('  }\n');
  o.push('  /// 获取内建入口对应的预期栈帧类型');
  o.push('  ///');
  o.push('  /// 已按原版 v8/src/builtins/x64/builtins-x64.cc 各 Generate_* 的帧宏逐项精化');
  o.push('  /// (逐臂注释给出出处); 未逐臂精化的成员保留粗类启发式兜底, 真帧型面随');
  o.push('  /// 汇编体填波替换, 下游勿当权威语义引用。台账见 readme/zh.md。');
  o.push('  #[inline]');
  o.push('  pub const fn expected_frame_type(&self) -> Option<v8_execution_frames::StackFrameType> {');
  o.push('    match self {');
  o.push('      // Generate_JSEntry/JSRunMicrotasksEntry: Generate_JSEntryVariant(StackFrame::ENTRY) (cc:557-568)');
  o.push('      Self::JSEntry | Self::JSRunMicrotasksEntry => Some(StackFrameType::Entry),');
  o.push('      // Generate_JSConstructEntry: Generate_JSEntryVariant(StackFrame::CONSTRUCT_ENTRY) (cc:561-563; frames.h:121)');
  o.push('      Self::JSConstructEntry => Some(StackFrameType::ConstructEntry),');
  o.push('      // Generate_JSEntryTrampoline/JSConstructEntryTrampoline: helper 开 FrameScope INTERNAL (cc:620; 入口 cc:694-699)');
  o.push('      Self::JSEntryTrampoline | Self::JSConstructEntryTrampoline => Some(StackFrameType::Internal),');
  o.push('      // Generate_RunMicrotasksTrampoline: 无帧尾调 kRunMicrotasks, 不开帧 (cc:702-706)');
  o.push('      Self::RunMicrotasksTrampoline => None,');
  o.push('      // Generate_ResumeGeneratorTrampoline: 入 runtime 各支路 FrameScope INTERNAL (cc:894/908/919)');
  o.push('      Self::ResumeGeneratorTrampoline => Some(StackFrameType::Internal),');
  o.push('      // Generate_RestartFrameTrampoline: 当前帧按 INTERPRETED 处理并 LeaveFrame 重启 (cc:5741/5764)');
  o.push('      Self::RestartFrameTrampoline => Some(StackFrameType::Interpreted),');
  o.push('      // Generate_BaselineOutOfLinePrologue: EnterFrame(BASELINE) (cc:1896-1899; frames.h:124)');
  o.push('      Self::BaselineOutOfLinePrologue => Some(StackFrameType::Baseline),');
  o.push('      // Generate_BaselineOutOfLinePrologueDeopt: LeaveFrame(BASELINE) 后尾调解释器 (cc:2003)');
  o.push('      Self::BaselineOutOfLinePrologueDeopt => Some(StackFrameType::Baseline),');
  o.push('      // Generate_BaselineOnStackReplacement: 按 BaselineFrameConstants/OsrSourceTier::kBaseline 处理 (cc:3060-3067)');
  o.push('      Self::BaselineOnStackReplacement => Some(StackFrameType::Baseline),');
  o.push('      // Generate_MaglevFunctionEntryStackCheck: Maglev 序言后即刻调用, 所入帧为 MAGLEV');
  o.push('      // (cc:3073-3080; frames.h:125; 体内 INTERNAL FrameScope 仅是 runtime 调用暂帧)');
  o.push('      Self::MaglevFunctionEntryStackCheck => Some(StackFrameType::Maglev),');
  o.push('      // Generate_InterpreterEntryTrampoline: 手工推入建 INTERPRETED 帧 (cc:1075/1139-1144)');
  o.push('      Self::InterpreterEntryTrampoline => Some(StackFrameType::Interpreted),');
  o.push('      // Generate_InterpreterEnterAt(Next)Bytecode: 在当前 INTERPRETED 帧续跑 InterpreterFrameConstants (cc:1803-1815/1853)');
  o.push('      Self::InterpreterEnterAtBytecode | Self::InterpreterEnterAtNextBytecode => {');
  o.push('        Some(StackFrameType::Interpreted)');
  o.push('      }');
  o.push('      // Generate_InterpreterPushArgsThenFastConstructFunction: EnterFrame(FAST_CONSTRUCT) (cc:1598-1599; frames.h:135)');
  o.push('      Self::InterpreterPushArgsThenFastConstructFunction => Some(StackFrameType::FastConstruct),');
  o.push('      // Generate_ContinueTo*: 入口即在 CEntry 所建 BUILTIN_CONTINUATION 帧上按其常量弹出续跑');
  o.push('      // (helper cc:2009-2067, 四包装 cc:2069-2085; frames.h:129)');
  o.push('      Self::ContinueToCodeStubBuiltin');
  o.push('      | Self::ContinueToCodeStubBuiltinWithResult');
  o.push('      | Self::ContinueToJavaScriptBuiltin');
  o.push('      | Self::ContinueToJavaScriptBuiltinWithResult => Some(StackFrameType::BuiltinContinuation),');
  o.push('      // Generate_NotifyDeoptimized: FrameScope INTERNAL 后呼 runtime (cc:2087-2090)');
  o.push('      Self::NotifyDeoptimized => Some(StackFrameType::Internal),');
  o.push('      // 未逐臂精化成员 (PushArgsThen{Call,Construct}Impl/ConstructForwardAllArgsImpl 等): 启发式粗类兜底');
  o.push('      _ => {');
  o.push('        if self.is_interpreter_or_baseline() {');
  o.push('          Some(StackFrameType::Interpreted)');
  o.push('        } else {');
  o.push('          None');
  o.push('        }');
  o.push('      }');
  o.push('    }');
  o.push('  }\n');
  o.push('  /// 获取 x64 架构下内建默认使用的根寄存器 (r13)');
  o.push('  #[inline]');
  o.push('  pub const fn root_register() -> v8_codegen_x64::Register {');
  o.push('    v8_codegen_x64::R13');
  o.push('  }\n');

  // 语义谓词
  o.push('  // 以下 6 谓词为启发式语义归组 (8+10+15+8+11+21=73 互斥完备), 非 V8 真分类面;');
  o.push('  // 仅供本 crate 名面自洽与测试分区, 不可被下游当权威语义; 成员归属已按原版');
  o.push('  // Generate_* 帧宏面订正 (ContinueTo* 四项按 CEntry 续执行桩归 runtime 辅助组);');
  o.push('  // 真分类面 (帧型/MOPs/寄存器分配) 随汇编体填波替换。台账见 readme/zh.md。\n');

  o.push('  /// 是否为 JS 入口或运行蹦床 (启发式归组; 帧型面已在 expected_frame_type 逐项精化)');
  o.push('  #[inline]');
  o.push('  pub const fn is_js_entry_or_trampoline(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const jsEntryVariants = builtins.filter(b => JS_ENTRY_OR_TRAMPOLINE.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${jsEntryVariants.join(" | ")}`);
  o.push('    )');
  o.push('  }\n');

  o.push('  /// 是否为解释器或基线编译器辅助入口 (启发式归组; Baseline/Maglev/帧面已精化)');
  o.push('  #[inline]');
  o.push('  pub const fn is_interpreter_or_baseline(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const interpVariants = builtins.filter(b => INTERPRETER_OR_BASELINE.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${interpVariants.join(" | ")}`);
  o.push('    )');
  o.push('  }\n');

  o.push('  /// 是否为函数调用或对象构造入口 (启发式归组)');
  o.push('  #[inline]');
  o.push('  pub const fn is_call_or_construct(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const callVariants = builtins.filter(b => CALL_OR_CONSTRUCT.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${callVariants.join(" | ")}`);
  o.push('    )');
  o.push('  }\n');

  o.push('  /// 是否为反优化或 OSR 栈替换入口 (8 项, 原版真 deopt/OSR 面:');
  o.push('  /// DeoptimizationEntry_* cc:5573-5583, *OnStackReplacement cc:3052-3068,');
  o.push('  /// AdaptShadowStackForDeopt cc:3168, NotifyDeoptimized cc:2087)');
  o.push('  #[inline]');
  o.push('  pub const fn is_deopt_or_osr(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const deoptVariants = builtins.filter(b => DEOPT_OR_OSR.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${deoptVariants.join(" | ")}`);
  o.push('    )');
  o.push('  }\n');

  o.push('  /// 是否为运行时调用与辅助桩入口 (11 项; 原名 is_c_entry_or_api, 轮7 W7BXM 按');
  o.push('  /// 原版归类订正组名: CEntry/DirectCEntry 运行时调用主体, CallApiCallbackImpl/');
  o.push('  /// CallApiAccessorImpl C++ API 回调, Adaptor 参数适配桩, ContinueTo* 四项为');
  o.push('  /// CEntry 续执行桩 (Generate_ContinueToBuiltinHelper 按 BuiltinContinuationFrame');
  o.push('  /// 常量弹出续跑, cc:2009-2085, 自 deopt 组订正归入), DoubleToI 数值转换桩');
  o.push('  /// (cc:4753, 无帧宏), DumpFrame 帧 dump 辅助桩 (cc:5388, x64 独有))');
  o.push('  #[inline]');
  o.push('  pub const fn is_runtime_or_aux(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const runtimeAuxVariants = builtins.filter(b => RUNTIME_OR_AUX.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${runtimeAuxVariants.join(" | ")}`);
  o.push('    )');
  o.push('  }\n');

  o.push('  /// 是否为 WebAssembly 汇编运行入口 (启发式归组; 真帧型面 WASM_* 随汇编体填波精化)');
  o.push('  #[inline]');
  o.push('  pub const fn is_wasm(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const wasmVariants = builtins.filter(b => WASM.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${wasmVariants.join(" | ")}`);
  o.push('    )');
  o.push('  }\n');

  o.push('  /// 是否为 x64 平台独有内建');
  o.push('  #[inline]');
  o.push('  pub const fn is_x64_specific(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const x64SpecVariants = builtins.filter(b => X64_SPECIFIC.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${x64SpecVariants.join(" | ")}`);
  o.push('    )');
  o.push('  }');
  o.push('}\n');

  o.push(`// 统一提供 ${builtins.length} 项零成本安全工厂函数`);
  for (const b of builtins) {
    const v = getVariantName(b.name);
    o.push(`#[inline]`);
    o.push(`pub const fn ${fnName(b.name)}() -> Builtin {`);
    o.push(`  Builtin::${v}`);
    o.push(`}\n`);
  }

  return fmt(o.join("\n"));
}

// libBody: 顶层统一扁平导出
function libBody(builtins) {
  const o = [head("模块索引")];
  o.push("#![forbid(unsafe_code)]\n");
  o.push("mod builtins_x64;");
  o.push("mod entry;\n");
  o.push("pub use builtins_x64::{");
  o.push("  ASM_GENERATE_ENTRIES, ArgumentsElementType, CC_BUILTINS, CLASSES, CcBuiltin, DebugBreakKind,");
  o.push("  OsrSourceTier,");
  o.push("};");
  o.push("pub use entry::{");
  o.push("  Builtin as X64Builtin,");
  for (const b of builtins) {
    o.push(`  ${fnName(b.name)},`);
  }
  o.push("};\n");
  o.push("// 死面导出与冗余依赖已清理: Error/Result 全 src 零构造零消费 (轮7 W7BXM 整改 nit4)\n");
  return fmt(o.join("\n"));
}

// 主流程
for (const [sub, file] of ANCHORS) anchors(readV8(`src/builtins/${file}`), [sub], file);
const seen = new Map();
for (const f of CC_FILES) for (const b of ccBuiltins(readV8(`src/builtins/${f}`))) seen.set(b.name, b);
const builtins = [...seen.values()].sort((a, b) => (a.name < b.name ? -1 : 1));
writeSrc(CRATE, "entry.rs", modBody(builtins));
writeSrc(CRATE, "lib.rs", libBody(builtins));
console.log(`[${GENERATOR}] entry: 内建 ${builtins.length}, 完成`);
