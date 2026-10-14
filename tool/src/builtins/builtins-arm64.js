#!/usr/bin/env bun
// tool/src/builtins/builtins-arm64.js — v8-builtins-arm64 基线生成 (汇编内建)
// 来源: arm64/builtins-arm64.cc (JSEntry/InterpreterEntryTrampoline/CEntry/Call/Construct)
// → v8-builtins-arm64/src/entry.rs + lib.rs
import { anchors, ccBuiltins, fmt, fnName, readV8, writeSrc } from "./builtins-lib.js";

const CRATE = "v8-builtins-arm64";
const GENERATOR = "builtins-arm64.js";
const CC_FILES = ["arm64/builtins-arm64.cc"];
const ANCHORS = [["void Builtins::Generate_Adaptor(MacroAssembler* masm,", "arm64/builtins-arm64.cc"]];

// 产物头
const head = (note) =>
  `// 由 tool/src/builtins/${GENERATOR} 生成; 来源 v8/src/builtins 下 ${CC_FILES.length} 文件\n// ${note}\n// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义\n`;

// 6 大语义分类集合 (arm64 71 项)
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
  "ContinueToCodeStubBuiltin", "ContinueToCodeStubBuiltinWithResult",
  "ContinueToJavaScriptBuiltin", "ContinueToJavaScriptBuiltinWithResult",
  "NotifyDeoptimized", "InterpreterOnStackReplacement",
  "InterpreterOnStackReplacement_ToBaseline", "BaselineOnStackReplacement",
  "DeoptimizationEntry_Eager", "DeoptimizationEntry_Lazy",
  "DeoptimizationEntry_LazyAfterFastCall"
]);

const C_ENTRY_OR_API = new Set([
  "CEntry", "DirectCEntry", "Adaptor", "CallApiCallbackImpl",
  "CallApiAccessorImpl", "DoubleToI"
]);

const WASM = new Set([
  "WasmLiftoffFrameSetup", "WasmCompileLazy", "WasmDebugBreak",
  "WasmTrapHandlerThrowTrap", "JSToWasmWrapperAsm", "WasmReturnPromiseOnSuspendAsm",
  "JSToWasmStressSwitchStacksAsm", "WasmToJsWrapperAsm", "WasmTrapHandlerLandingPad",
  "WasmSuspend", "WasmResume", "WasmReject", "WasmFXResume", "WasmFXResumeThrow",
  "WasmFXResumeThrowRef", "WasmFXSuspend", "WasmFXSwitch", "WasmFXReturn",
  "WasmOnStackReplace", "WasmHandleStackOverflow", "WasmReturnFromSegment"
]);

function getVariantName(name) {
  return name.replace(/_/g, "");
}

// modBody: 内建名表 + 核心调度元数据 (71 内建)
function modBody(builtins) {
  const o = [];
  o.push(head(`内建名表 + 核心调度元数据 (${builtins.length} 内建)`));
  o.push('use strum::{Display, EnumIter, EnumString};');
  o.push('use v8_builtins_core::{descriptor_name, find_builtin};');
  o.push('use v8_execution_frames::StackFrameType;\n');
  o.push(`/// arm64 架构汇编内建名表 (${builtins.length} 项)`);
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
  o.push('  /// 注意: 启发式归组, 非 V8 真帧型, 原版无 1:1 对应 (如 Maglev 栈检查归');
  o.push('  /// Interpreted、Resume/Restart 蹦床归 Entry 均不契合真实帧型); 真帧型面');
  o.push('  /// 随汇编体填波替换。');
  o.push('  #[inline]');
  o.push('  pub const fn expected_frame_type(&self) -> Option<StackFrameType> {');
  o.push('    if self.is_js_entry_or_trampoline() {');
  o.push('      Some(StackFrameType::Entry)');
  o.push('    } else if self.is_interpreter_or_baseline() {');
  o.push('      Some(StackFrameType::Interpreted)');
  o.push('    } else {');
  o.push('      None');
  o.push('    }');
  o.push('  }\n');

  o.push('  /// 获取 arm64 架构下内建默认使用的根寄存器 (x26)');
  o.push('  #[inline]');
  o.push('  pub const fn root_register() -> v8_codegen_arm64::Register {');
  o.push('    v8_codegen_arm64::K_ROOT_REGISTER');
  o.push('  }\n');

  // 语义谓词
  o.push('  // 以下 6 谓词为启发式语义归组 (8+10+15+11+6+21=71 互斥完备), 非 V8 真帧型/');
  o.push('  // 真分类面; 仅供本 crate 名面自洽与测试分区, 不可被下游当权威语义;');
  o.push('  // 真分类面 (帧型/MOPs/寄存器分配) 随汇编体填波替换。台账见 readme/zh.md。\n');
  o.push('  /// 是否为 JS 入口或运行蹦床 (启发式归组)');
  o.push('  #[inline]');
  o.push('  pub const fn is_js_entry_or_trampoline(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const jsEntryVariants = builtins.filter(b => JS_ENTRY_OR_TRAMPOLINE.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${jsEntryVariants.join(" | ")}`);
  o.push('    )');
  o.push('  }\n');

  o.push('  /// 是否为解释器或基线编译器辅助入口 (启发式归组)');
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

  o.push('  /// 是否为反优化或 OSR 栈替换入口 (启发式归组)');
  o.push('  #[inline]');
  o.push('  pub const fn is_deopt_or_osr(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const deoptVariants = builtins.filter(b => DEOPT_OR_OSR.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${deoptVariants.join(" | ")}`);
  o.push('    )');
  o.push('  }\n');

  o.push('  /// 是否为 CEntry 运行时调用或 API 回调辅助入口 (启发式归组)');
  o.push('  #[inline]');
  o.push('  pub const fn is_c_entry_or_api(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const cEntryVariants = builtins.filter(b => C_ENTRY_OR_API.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${cEntryVariants.join(" | ")}`);
  o.push('    )');
  o.push('  }\n');

  o.push('  /// 是否为 WebAssembly 汇编运行入口 (启发式归组)');
  o.push('  #[inline]');
  o.push('  pub const fn is_wasm(&self) -> bool {');
  o.push('    matches!(');
  o.push('      self,');
  const wasmVariants = builtins.filter(b => WASM.has(b.name)).map(b => `Self::${getVariantName(b.name)}`);
  o.push(`      ${wasmVariants.join(" | ")}`);
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
  o.push("mod builtins_arm64;");
  o.push("mod entry;\n");
  o.push("pub use builtins_arm64::{");
  o.push("  ASM_GENERATE_ENTRIES, CC_BUILTINS, CLASSES, CcBuiltin, DebugBreakKind, OsrSourceTier,");
  o.push("};");
  o.push("pub use entry::{");
  o.push("  Builtin as Arm64Builtin,");
  for (const b of builtins) {
    o.push(`  ${fnName(b.name)},`);
  }
  o.push("};\n");
  o.push("// 死面导出与冗余依赖已清理: Error/Result 全 src 零构造零消费 (轮7 W7BAM 整改 nit2)\n");
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
