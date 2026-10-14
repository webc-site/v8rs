#!/usr/bin/env bun
// tool/src/wasm/interpreter/wasm-interpreter-objects.h.js
// 机械提取 v8/src/wasm/interpreter/wasm-interpreter-objects.{h,cc} 的入口契约

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/wasm/interpreter/wasm-interpreter-objects.h");
const ccSrc = readV8("src/wasm/interpreter/wasm-interpreter-objects.cc");

// 1. h: 栈条目结构
if (!hSrc.includes("struct WasmInterpreterStackEntry {")) {
  throw new Error("未找到 WasmInterpreterStackEntry");
}
if (!hSrc.includes("int function_index;\n  int byte_offset;")) {
  throw new Error("未找到栈条目字段");
}

// 2. h: 静态类 4 个方法
for (const m of [
  "static bool RunInterpreter(",
  "static std::vector<WasmInterpreterStackEntry> GetInterpretedStack(",
  "static int GetFunctionIndex(",
]) {
  if (!hSrc.includes(m)) throw new Error(`方法缺失: ${m}`);
}
if ((hSrc.match(/static bool RunInterpreter\(/g) || []).length !== 2) {
  throw new Error("RunInterpreter 重载数 != 2");
}

// 3. h: handle 工厂对
for (const f of [
  "GetInterpreterHandle(Isolate* isolate,",
  "GetOrCreateInterpreterHandle(",
]) {
  if (!hSrc.includes(f)) throw new Error(`工厂缺失: ${f}`);
}

// 4. cc: 两个 RunInterpreter 的关键差异
if (!ccSrc.includes("const std::vector<wasm::WasmValue>& argument_values,")) {
  throw new Error("未找到参数向量重载");
}
if (!ccSrc.includes("uint8_t* interpreter_sp) {")) {
  throw new Error("未找到 interpreter_sp 重载");
}
// 值参版本走 GetOrCreate, 指针版本走 Get (复用现成 handle)
if (!ccSrc.includes("wasm::GetOrCreateInterpreterHandle(isolate, trusted_data);")) {
  throw new Error("值参版本未用 GetOrCreate");
}
if (!ccSrc.includes("wasm::GetInterpreterHandle(isolate, trusted_data);")) {
  throw new Error("指针版本未用 Get");
}

// 5. cc: InstanceScope 发布语义
const scopeCount = (ccSrc.match(/InstanceScope instance_scope\(/g) || []).length;
if (scopeCount !== 2) throw new Error(`InstanceScope ${scopeCount} 次 != 2`);

// 6. cc: 栈序约定
if (!hSrc.includes("The list is ordered bottom-to-top, i.e. caller before callee.")) {
  throw new Error("未找到栈序注释");
}

// 7. cc: CHECK 前置
if ((ccSrc.match(/CHECK\(trusted_data->has_interpreter_handle\(\)\);/g) || []).length !== 2) {
  throw new Error("has_interpreter_handle CHECK != 2");
}

console.log(`wasm-interpreter-objects: 方法=4 Run重载=2 InstanceScope=${scopeCount}`);

// 8. 生成 Rust
let rust = banner(
  "src/wasm/interpreter/wasm-interpreter-objects.h.js",
  "src/wasm/interpreter/wasm-interpreter-objects.{h,cc}",
  "WasmInterpreterObject 入口与解释栈条目",
);

rust += `
//! 解释器对象入口: 状态经 TrustedManaged<InterpreterHandle> 挂在实例受信数据上。
/// v8: src/wasm/interpreter/wasm-interpreter-objects.h WasmInterpreterObject

/// 解释栈条目 (自底向上: caller 在前)
/// v8: src/wasm/interpreter/wasm-interpreter-objects.h struct WasmInterpreterStackEntry
#[derive(Debug, Clone, Copy)]
pub struct WasmInterpreterStackEntry {
    /// v8: int function_index
    pub function_index: i32,
    /// v8: int byte_offset
    pub byte_offset: i32,
}

/// RunInterpreter 两种入口的参数形态
/// v8: src/wasm/interpreter/wasm-interpreter-objects.cc RunInterpreter 两个重载
#[derive(Debug)]
pub enum RunInterpreterArgs<'a> {
    /// v8: const std::vector<wasm::WasmValue>& argument_values (GetOrCreate 新 handle)
    ValueArgs { argument_values: &'a [WasmValue] },
    /// v8: uint8_t* interpreter_sp (Get 复用现成 handle, 恢复执行)
    ResumeSp { interpreter_sp: *mut u8 },
}

/// 入口前置: 每次执行都要把当前 trusted data 发布进 runtime 的实例作用域
/// v8: src/wasm/interpreter/wasm-interpreter-objects.cc WasmInterpreterRuntime::InstanceScope
pub const PUBLISHES_INSTANCE_SCOPE: bool = true;

/// GetInterpretedStack / GetFunctionIndex 的前置检查
/// v8: src/wasm/interpreter/wasm-interpreter-objects.cc CHECK(has_interpreter_handle())
pub const REQUIRES_INTERPRETER_HANDLE: bool = true;

/// 假设: 一个实例同时只在一个线程运行
/// v8: src/wasm/interpreter/wasm-interpreter-objects.cc "Assume an instance can run in only one thread."
pub const SINGLE_THREAD_PER_INSTANCE: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert!(PUBLISHES_INSTANCE_SCOPE);
    assert!(SINGLE_THREAD_PER_INSTANCE);
};
`;

const p = writeSrc("v8-wasm-interp", "wasm_interpreter_objects.rs.skel", rust);
console.log(`written ${p}`);
