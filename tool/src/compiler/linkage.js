#!/usr/bin/env bun
// v8/src/compiler/linkage.cc + linkage.h
// → v8-compiler-passes/src/linkage.rs.skel
//
// 规则面: CallDescriptor::Kind 枚举 (WASM 条件 5)、Flag 位标志 (10 位 +
// kHasEffectHandler 别名 + kFlagsBitsEncodedInInstructionCode=9)、
// OSR 常量 (-1/-1)、Linkage JS 参数下标公式 4 项, 逐字提取。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/linkage.h");
const cc = readV8("src/compiler/linkage.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE CallDescriptor final/, "CallDescriptor"],
  [h, /static constexpr int kFlagsBitsEncodedInInstructionCode = 9;/, "编码位数"],
  [h, /kHasEffectHandler = kNoAllocate,/, "EffectHandler 别名"],
  [h, /class V8_EXPORT_PRIVATE Linkage : public NON_EXPORTED_BASE\(ZoneObject\) \{/, "Linkage"],
  [h, /static const int kOsrContextSpillSlotIndex = -1;/, "OSR 常量"],
  [h, /static constexpr int GetJSCallNewTargetParamIndex\(int parameter_count\) \{/, "JS 参数公式"],
  [h, /return parameter_count \+ 0;/, "NewTarget 公式体"],
  [cc, /CallDescriptor\* Linkage::ComputeIncoming\(/, "cc ComputeIncoming"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}

// Kind 变体
const kindBlock = h.match(/enum Kind \{([\s\S]*?)\n  \};/)[1];
const kinds = [...kindBlock.matchAll(/^\s*k(\w+),?\s*(?=\/\/|$)/gm)].map((m) => m[1]);
if (kinds.join() !== "CallCodeObject,CallJSFunction,CallAddress,CallWasmCapiFunction,CallWasmFunction,CallWasmFunctionIndirect,CallWasmImportWrapper,ResumeWasmContinuation,CallBuiltinPointer") {
  throw new Error(`Kind 漂移: ${kinds}`);
}
// Flag 位标志 (值 = 1u << n)
const flagBlock = h.match(/enum Flag \{([\s\S]*?)\n  \};/)[1];
const flags = [...flagBlock.matchAll(/^\s*k(\w+) = (1u << (\d+)|0u),?\s*$/gm)].map((m) => [m[1], m[3] ? 1 << Number(m[3]) : 0]);
if (flags.length !== 11) throw new Error(`Flag ${flags.length} != 11 (kNoFlags..kNoFunctionDescriptor)`);
const flagMap = Object.fromEntries(flags);
if (flagMap.NeedsFrameState !== 1 || flagMap.HasExceptionHandler !== 2 || flagMap.NoAllocate !== 16 || flagMap.NoFunctionDescriptor !== 512) {
  throw new Error(`Flag 位值漂移: ${JSON.stringify(flagMap)}`);
}
// OSR 常量
if (!/kOsrAccumulatorRegisterIndex = -1;/.test(h)) throw new Error("kOsrAccumulatorRegisterIndex 缺失");
// JS 参数公式 (NewTarget = parameter_count; ArgCount = parameter_count+1;
// DispatchHandle = parameter_count+2; Context = parameter_count+3 — cc 实现对齐)
if (!cc.includes("parameter_count + 0") && !cc.includes("parameter_count")) throw new Error("cc 参数公式缺失");

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
const wasmKinds = kinds.filter((k) => /Wasm|ResumeWasm/.test(k));

const body = `${banner("src/compiler/linkage.js", "src/compiler/linkage.h + src/compiler/linkage.cc", "CallDescriptor + Linkage — 调用约定描述 (Kind ${kinds.length}/Flag ${flags.length} 位)")}
//! 调用描述符: 目标类别 + 位置签名 + callee-save 集 + 标志位。
//! 低 9 位 Flag 编码进 InstructionCode (kFlagsBitsEncodedInInstructionCode)。

#![allow(dead_code, non_camel_case_types)]

/// v8: src/compiler/linkage.h CallDescriptor::Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
  /// kCallCodeObject
  CallCodeObject,
  /// kCallJSFunction
  CallJSFunction,
  /// kCallAddress
  CallAddress,
${wasmKinds.map((k) => `  /// k${k} (V8_ENABLE_WEBASSEMBLY)\n  ${k},`).join("\n")}
  /// kCallBuiltinPointer
  CallBuiltinPointer,
}

/// v8: src/compiler/linkage.h kFlagsBitsEncodedInInstructionCode
pub const K_FLAGS_BITS_ENCODED_IN_INSTRUCTION_CODE: i32 = 9;

// v8: src/compiler/linkage.h CallDescriptor::Flag (位标志)
${flags.map(([name, val]) => `pub const K_${snake(name)}: u32 = ${val};`).join("\n")}
/// v8: src/compiler/linkage.h kHasEffectHandler — 与 kNoAllocate (=${flagMap.NoAllocate}) 复用同位, 指令选择期语义不同
pub const K_HAS_EFFECT_HANDLER: u32 = K_${snake("NoAllocate")};

/// v8: src/compiler/linkage.h CallDescriptor
pub struct CallDescriptor {
  /// 原 kind_
  pub kind: Kind,
  /// 原 tag_ (CodeEntrypointTag)
  pub tag: u64,
  /// 原 target_type_ / target_loc_
  pub target_type: crate::machine_type::MachineType,
  pub target_loc: crate::linkage_location::LinkageLocation,
  /// 原 location_sig_
  pub location_sig: *mut crate::linkage_location::LocationSignature,
  /// 原 stack_param_count (param_slot_count_)
  pub param_slot_count: usize,
  /// 原 properties_
  pub properties: crate::operator::Properties,
  /// 原 callee_saved_registers_ / callee_saved_fp_registers_
  pub callee_saved_registers: crate::reg_list::RegList,
  pub callee_saved_fp_registers: crate::reg_list::DoubleRegList,
  /// 原 flags_
  pub flags: u32,
  /// 原 debug_name_
  pub debug_name: &'static str,
  /// 原 allocatable_registers_
  pub allocatable_registers: crate::reg_list::RegList,
  /// 原 return_slot_count_
  pub return_slot_count: usize,
  /// 原 signature_hash_
  pub signature_hash: u64,
  /// 原 expected_parameter_count_ (kDontAdaptArgumentsSentinel)
  pub expected_parameter_count: u16,
}

/// v8: src/compiler/linkage.h Linkage — 常量与 JS 参数下标公式
pub mod linkage {
  /// v8: src/compiler/linkage.h Linkage::kOsrContextSpillSlotIndex
  pub const K_OSR_CONTEXT_SPILL_SLOT_INDEX: i32 = -1;
  /// v8: src/compiler/linkage.h Linkage::kOsrAccumulatorRegisterIndex
  pub const K_OSR_ACCUMULATOR_REGISTER_INDEX: i32 = -1;
  /// v8: src/compiler/linkage.h Linkage::kJSCallClosureParamIndex
  pub const K_JS_CALL_CLOSURE_PARAM_INDEX: i32 = 0;

  /// v8: src/compiler/linkage.h Linkage::GetJSCallNewTargetParamIndex
  /// 原文: return parameter_count + 0;
  pub const fn get_js_call_new_target_param_index(parameter_count: i32) -> i32 {
    parameter_count + 0
  }
  /// v8: src/compiler/linkage.h Linkage::GetJSCallArgCountParamIndex
  /// 原文: GetJSCallNewTargetParamIndex(pc) + 1
  pub const fn get_js_call_arg_count_param_index(parameter_count: i32) -> i32 {
    get_js_call_new_target_param_index(parameter_count) + 1
  }
  /// v8: src/compiler/linkage.h Linkage::GetJSCallDispatchHandleParamIndex
  /// 原文: GetJSCallArgCountParamIndex(pc) + 1 (仅 V8_JS_LINKAGE_INCLUDES_DISPATCH_HANDLE)
  pub const fn get_js_call_dispatch_handle_param_index(parameter_count: i32) -> i32 {
    get_js_call_arg_count_param_index(parameter_count) + 1
  }
  /// v8: src/compiler/linkage.h Linkage::GetJSCallContextParamIndex
  /// 原文: 开 dispatch handle 时 = GetJSCallDispatchHandleParamIndex(pc)+1 (pc+3);
  /// 关时 = GetJSCallArgCountParamIndex(pc)+1 (pc+2)
  pub const fn get_js_call_context_param_index(parameter_count: i32) -> i32 {
    if cfg!(feature = "js_linkage_includes_dispatch_handle") {
      get_js_call_dispatch_handle_param_index(parameter_count) + 1
    } else {
      get_js_call_arg_count_param_index(parameter_count) + 1
    }
  }
}
`;

const p = writeSrc("v8-compiler-passes", "linkage.rs.skel", body);
console.log(`written ${p}\nKind=${kinds.length} (WASM条件=${wasmKinds.length}) Flag=${flags.length}位 OSR常量=2 JS公式=4`);
