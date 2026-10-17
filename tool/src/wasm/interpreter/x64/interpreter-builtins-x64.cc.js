#!/usr/bin/env bun
// tool/src/wasm/interpreter/x64/interpreter-builtins-x64.cc.js
// 机械提取 v8/src/wasm/interpreter/x64/interpreter-builtins-x64.cc 的入口寄存器与 spill 槽链

import { readV8, writeSrc, banner } from "../../../../lib.js";

const src = readV8("src/wasm/interpreter/x64/interpreter-builtins-x64.cc");

// 1. 4 个手写 builtin (与 arm64 对称)
for (const b of [
  "void Builtins::Generate_WasmInterpreterEntry(MacroAssembler* masm)",
  "void Builtins::Generate_JSToWasmInterpreterWrapperAsm(MacroAssembler* masm)",
  "void Builtins::Generate_WasmInterpreterCWasmEntry(MacroAssembler* masm)",
  "void Builtins::Generate_GenericWasmToJSInterpreterWrapper(",
]) {
  if (!src.includes(b)) throw new Error(`builtin 缺失: ${b}`);
}

// 2. WasmInterpreterEntry 入口寄存器约定 (x64: 参数在通用寄存器)
const entryRegs = [
  ["wasm_instance", "kWasmImplicitArgRegister"],
  ["function_index", "r12"],
  ["array_start", "r15"],
  ["result_start", "rax"],
  ["ref_params_array", "rbx"],
];
for (const [name, reg] of entryRegs) {
  if (!src.includes(`Register ${name} = ${reg};`)) {
    throw new Error(`入口寄存器缺失: ${name}=${reg}`);
  }
}

// 3. tagged 值先压栈 (GC 扫描连续)
if (!src.includes("// Push tagged values first so they are contiguous for GC scanning.")) {
  throw new Error("未找到 GC 连续注释");
}
const pushOrder = [...src.matchAll(/__ pushq\((\w+)\);/g)].map((m) => m[1]).slice(0, 5);
if (pushOrder.join(",") !== "wasm_instance,ref_params_array,function_index,array_start,result_start") {
  throw new Error(`压栈序异常: ${pushOrder.join(",")}`);
}
if (!src.includes("__ CallRuntime(Runtime::kWasmRunInterpreter, 5);")) {
  throw new Error("未找到 kWasmRunInterpreter 5 参调用");
}

// 4. WasmToJS wrapper 的 spill 槽链 (每槽 1 指针宽)
if (!src.includes("constexpr int kPackedArrayOffset =")) throw new Error("未找到 kPackedArrayOffset");
const chain = [
  "kExpectedArityOffset = kPackedArrayOffset - kSystemPointerSize",
  "kReturnCountOffset = kExpectedArityOffset - kSystemPointerSize",
  "kParamCountOffset = kReturnCountOffset - kSystemPointerSize",
  "kSignatureOffset = kParamCountOffset - kSystemPointerSize",
  "kParamIndexOffset = kSignatureOffset - kSystemPointerSize",
];
for (const c of chain) {
  if (!src.includes(c)) throw new Error(`spill 槽链缺失: ${c}`);
}
if (!src.includes("constexpr int kNumSpillSlots =\n      (kMarkerOffset - kCurrentResultAddressOffset) / kSystemPointerSize;")) {
  throw new Error("未找到 kNumSpillSlots 公式");
}

// 5. 槽复用 (参数迭代与结果迭代共用)
const reuseCount = (src.match(/\/\/ Reuse this slot when iterating over return values\./g) || []).length;
if (reuseCount !== 2) throw new Error(`槽复用注释 ${reuseCount} != 2`);

// 6. X-macro handler 族 (与 arm64 同构)
const vEntries = (src.match(/^  V\(\w+,/gm) || []).length;
if (vEntries !== 96) throw new Error(`V( 条目 ${vEntries} != 96`);
for (const m of ["#define FOREACH_INT_LOADSTORE_BUILTIN", "#define FOREACH_FLOAT_LOADMEM_BUILTIN"]) {
  if (!src.includes(m)) throw new Error(`宏缺失: ${m}`);
}

// 7. sandbox 双路径
if (!src.includes("__ DecompressProtected(")) throw new Error("未找到 DecompressProtected");

console.log(
  `interpreter-builtins-x64: 手写=4 入口寄存器=${entryRegs.length} spill链=${chain.length} V条目=${vEntries}`,
);

// 8. 生成 Rust
let rust = banner(
  "src/wasm/interpreter/x64/interpreter-builtins-x64.cc.js",
  "src/wasm/interpreter/x64/interpreter-builtins-x64.cc",
  "x64 解释器 builtin: 入口寄存器约定与 WasmToJS spill 槽链",
);

rust += `
//! Drumbrake 解释器的 x64 汇编 builtin。
/// v8: src/wasm/interpreter/x64/interpreter-builtins-x64.cc Builtins::Generate_*

/// WasmInterpreterEntry 输入寄存器
/// v8: src/wasm/interpreter/x64/interpreter-builtins-x64.cc Generate_WasmInterpreterEntry
pub mod interpreter_entry_regs {
    /// v8: kWasmImplicitArgRegister — wasm_instance
    pub const WASM_INSTANCE: &str = "kWasmImplicitArgRegister";
    /// v8: r12 — function_index
    pub const FUNCTION_INDEX: &str = "r12";
    /// v8: r15 — array_start
    pub const ARRAY_START: &str = "r15";
    /// v8: rax — result_start
    pub const RESULT_START: &str = "rax";
    /// v8: rbx — ref_params_array
    pub const REF_PARAMS_ARRAY: &str = "rbx";
}

/// 压栈序: tagged (instance, ref_params) 先压保 GC 连续, 再压非 tagged
/// v8: Generate_WasmInterpreterEntry pushq 序列
pub const PUSH_ORDER: &[&str] = &[
    "wasm_instance", "ref_params_array", "function_index", "array_start", "result_start",
];

/// v8: __ CallRuntime(Runtime::kWasmRunInterpreter, 5)
pub const ENTRY_RUNTIME_ARGC: usize = 5;

/// 手写 builtin (4 个)
/// v8: Builtins::Generate_*
pub const HANDWRITTEN_BUILTINS: &[&str] = &[
    "WasmInterpreterEntry",
    "JSToWasmInterpreterWrapperAsm",
    "WasmInterpreterCWasmEntry",
    "GenericWasmToJSInterpreterWrapper",
];

/// GenericWasmToJSInterpreterWrapper 的 spill 槽链 (高→低地址)
/// v8: Generate_GenericWasmToJSInterpreterWrapper constexpr 偏移链
pub const WASM_TO_JS_SPILL_SLOTS: &[&str] = &[
    "packed_array",
    "expected_arity",
    "return_count",
    "param_count",
    "signature",
    "param_index", // 迭代返回值时复用
    "value_types_array_start",
    "current_param", // 迭代返回值地址时复用
];

/// 复用槽对数 (param_index/result_index, current_param/current_result_address)
/// v8: "Reuse this slot when iterating over return values."
pub const REUSED_SLOT_PAIRS: usize = ${reuseCount};

/// spill 槽总数 = (marker - 最低槽) / 指针宽
/// v8: kNumSpillSlots 公式
pub fn num_spill_slots(marker_offset: i32, lowest_slot_offset: i32, pointer_size: i32) -> i32 {
    (marker_offset - lowest_slot_offset) / pointer_size
}

/// X-macro handler 条目数 (INT_LOADSTORE + FLOAT_LOADMEM, 与 arm64 同构)
/// v8: FOREACH_INT_LOADSTORE_BUILTIN / FOREACH_FLOAT_LOADMEM_BUILTIN
pub const HANDLER_V_ENTRIES: usize = ${vEntries};
pub const VARIANTS_PER_ENTRY: usize = 2;
pub const TOTAL_HANDLER_BUILTINS: usize = HANDLER_V_ENTRIES * VARIANTS_PER_ENTRY;

/// 沙箱构建下受信字段走 DecomompressProtected, 否则 LoadTaggedField
/// v8: LoadWasmInstanceFromFunctionData
pub const SANDBOX_DUAL_PATH: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(HANDWRITTEN_BUILTINS.len(), 4);
    assert_eq!(PUSH_ORDER.len(), 5);
    assert_eq!(WASM_TO_JS_SPILL_SLOTS.len(), 8);
    assert_eq!(HANDLER_V_ENTRIES, 96);
    assert_eq!(TOTAL_HANDLER_BUILTINS, 192);
    assert_eq!(num_spill_slots(0x30, 0x10, 8), 4);
};
`;

const p = writeSrc("v8-wasm-interp", "interpreter_builtins_x64.rs.skel", rust);
console.log(`written ${p}`);
