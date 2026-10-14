#!/usr/bin/env bun
// tool/src/wasm/interpreter/arm64/interpreter-builtins-arm64.cc.js
// 机械提取 v8/src/wasm/interpreter/arm64/interpreter-builtins-arm64.cc 的入口寄存器与 handler builtin 族

import { readV8, writeSrc, banner } from "../../../../lib.js";

const src = readV8("src/wasm/interpreter/arm64/interpreter-builtins-arm64.cc");

// 1. 4 个手写 builtin
for (const b of [
  "void Builtins::Generate_WasmInterpreterEntry(MacroAssembler* masm)",
  "void Builtins::Generate_JSToWasmInterpreterWrapperAsm(MacroAssembler* masm)",
  "void Builtins::Generate_WasmInterpreterCWasmEntry(MacroAssembler* masm)",
  "void Builtins::Generate_GenericWasmToJSInterpreterWrapper(",
]) {
  if (!src.includes(b)) throw new Error(`builtin 缺失: ${b}`);
}

// 2. WasmInterpreterEntry 入口寄存器约定
const regs = [
  ["array_start", "x12"],
  ["result_start", "x13"],
  ["ref_params_array", "x14"],
  ["function_index", "x15"],
];
for (const [name, reg] of regs) {
  if (!src.includes(`Register ${name} = ${reg};`) && !src.includes(`DEFINE_PINNED(${name}, ${reg}`)) {
    throw new Error(`入口寄存器缺失: ${name}=${reg}`);
  }
}
if (!src.includes("//  x7 (kWasmImplicitArgRegister): wasm_instance")) {
  throw new Error("未找到 wasm_instance 注释");
}

// 3. 入口帧布局 (args[0..4] 自 fp-0x10 到 fp-0x30)
for (const anchor of [
  "fp-0x30  result_start      (args[4])",
  "fp-0x28  array_start       (args[3])",
  "fp-0x20  function_index    (args[2])",
  "fp-0x18  ref_params_array  (GC-scanned, args[1])",
  "fp-0x10  wasm_instance     (GC-scanned, args[0])",
  "fp-0x08  marker(StackFrame::WASM_INTERPRETER_ENTRY)",
]) {
  if (!src.includes(anchor)) throw new Error(`帧布局注释缺失: ${anchor}`);
}
if (!src.includes("__ CallRuntime(Runtime::kWasmRunInterpreter, 5);")) {
  throw new Error("未找到 kWasmRunInterpreter 5 参调用");
}

// 4. X-macro handler 族 (_s/_l 双变体)
const vEntries = (src.match(/^  V\(\w+,/gm) || []).length;
if (vEntries !== 96) throw new Error(`V( 条目 ${vEntries} != 96`);
for (const m of ["#define FOREACH_INT_LOADSTORE_BUILTIN", "#define FOREACH_FLOAT_LOADMEM_BUILTIN"]) {
  if (!src.includes(m)) throw new Error(`宏缺失: ${m}`);
}
if (!src.includes("void Builtins::Generate_##builtin_name##_s(MacroAssembler* masm)")) {
  throw new Error("未找到 _s 变体生成");
}
if (!src.includes("void Builtins::Generate_##builtin_name##_l(MacroAssembler* masm)")) {
  throw new Error("未找到 _l 变体生成");
}

// 5. 生成器命名模式 (r2s/s2s/r2r/s2r 前缀)
const patterns = [...new Set([...src.matchAll(/^  V\((\w+?)_I?(?:32|64)/gm)].map((m) => m[1]))];
if (!patterns.includes("r2s") || !patterns.includes("s2s")) {
  throw new Error(`前缀族异常: ${patterns.join(",")}`);
}

// 6. DEFINE_PINNED 寄存器锁定宏
if (!src.includes("#define DEFINE_PINNED(Name, Reg)")) throw new Error("未找到 DEFINE_PINNED");

// 7. sandbox 双路径 (DecompressProtected)
if (!src.includes("__ DecompressProtected(")) throw new Error("未找到 DecompressProtected");

console.log(
  `interpreter-builtins-arm64: 手写=4 入口寄存器=${regs.length} V条目=${vEntries} 前缀=${patterns.length}`,
);

// 8. 生成 Rust
let rust = banner(
  "src/wasm/interpreter/arm64/interpreter-builtins-arm64.cc.js",
  "src/wasm/interpreter/arm64/interpreter-builtins-arm64.cc",
  "arm64 解释器 builtin: 入口寄存器约定与 load/store handler 族",
);

rust += `
//! Drumbrake 解释器的 arm64 汇编 builtin。
/// v8: src/wasm/interpreter/arm64/interpreter-builtins-arm64.cc Builtins::Generate_*

/// WasmInterpreterEntry 输入寄存器
/// v8: src/wasm/interpreter/arm64/interpreter-builtins-arm64.cc Generate_WasmInterpreterEntry
pub mod interpreter_entry_regs {
    /// v8: x7 (kWasmImplicitArgRegister) — wasm_instance
    pub const WASM_INSTANCE: &str = "x7";
    /// v8: x12 — array_start
    pub const ARRAY_START: &str = "x12";
    /// v8: x13 — result_start
    pub const RESULT_START: &str = "x13";
    /// v8: x14 — ref_params_array
    pub const REF_PARAMS_ARRAY: &str = "x14";
    /// v8: w15 — function_index
    pub const FUNCTION_INDEX: &str = "w15";
}

/// 入口帧槽 (args[0..4], GC 扫描 args[0]/args[1])
/// v8: Generate_WasmInterpreterEntry 帧布局注释
pub const ENTRY_ARGS_GC_SCANNED: usize = 2;
pub const ENTRY_RUNTIME_ARGC: usize = 5;
pub const ENTRY_FRAME_MARKER: &str = "StackFrame::WASM_INTERPRETER_ENTRY";

/// 手写 builtin (4 个)
/// v8: Builtins::Generate_WasmInterpreterEntry / JSToWasmInterpreterWrapperAsm / WasmInterpreterCWasmEntry / GenericWasmToJSInterpreterWrapper
pub const HANDWRITTEN_BUILTINS: &[&str] = &[
    "WasmInterpreterEntry",
    "JSToWasmInterpreterWrapperAsm",
    "WasmInterpreterCWasmEntry",
    "GenericWasmToJSInterpreterWrapper",
];

/// X-macro handler 条目数 (INT_LOADSTORE + FLOAT_LOADMEM)
/// v8: FOREACH_INT_LOADSTORE_BUILTIN / FOREACH_FLOAT_LOADMEM_BUILTIN
pub const HANDLER_V_ENTRIES: usize = ${vEntries};

/// 每条目生成 _s/_l 双变体 (small/large 内存偏移寻径)
/// v8: GENERATE_INT_LOADSTORE_BUILTIN / GENERATE_FLOAT_LOADMEM_BUILTIN
pub const VARIANTS_PER_ENTRY: usize = 2;

/// 生成 handler 总数 = 条目 × 变体
/// v8: FOREACH_*(GENERATE_*) 展开
pub const TOTAL_HANDLER_BUILTINS: usize = HANDLER_V_ENTRIES * VARIANTS_PER_ENTRY;

/// 操作数流向前缀: reg→stack / stack→stack / reg→reg / stack→reg
/// v8: V(r2s_.../s2s_.../r2r_.../s2r_...) 命名
pub const OPERAND_FLOW_PREFIXES: &[&str] = &["r2r", "r2s", "s2r", "s2s"];

/// 沙箱构建下受信字段走 DecompressProtected, 否则 LoadTaggedField
/// v8: LoadWasmInstanceFromFunctionData
pub const SANDBOX_DUAL_PATH: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(HANDWRITTEN_BUILTINS.len(), 4);
    assert_eq!(HANDLER_V_ENTRIES, 96);
    assert_eq!(TOTAL_HANDLER_BUILTINS, 192);
    assert_eq!(ENTRY_RUNTIME_ARGC, 5);
};
`;

const p = writeSrc("v8-wasm-interp", "interpreter_builtins_arm64.rs.skel", rust);
console.log(`written ${p}`);
