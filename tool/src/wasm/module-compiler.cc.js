#!/usr/bin/env bun
// tool/src/wasm/module-compiler.cc.js
// 机械提取 v8/src/wasm/module-compiler.cc 的差集:
// 编译事件名表 (GetCompilationEventName), kMainTaskId, 按类分组的方法定义台账
// (CompilationStateImpl / CompilationState / AsyncCompileJob / AsyncStreamingProcessor 等)。
// 头侧在 v8-wasm-core/src/module_compiler_consts.rs (module-compiler.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/module-compiler.cc");

// ---------- 锚断言 ----------
const anchors = [
  ["include 自头", src.includes('#include "src/wasm/module-compiler.h"')],
  ["事件名函数", src.includes('const char* GetCompilationEventName(const WasmCompilationUnit& unit,')],
  ["主任务 id", src.includes("constexpr uint8_t kMainTaskId = 0;")],
  ["ExecuteCompilationUnits", src.includes("CompilationExecutionResult ExecuteCompilationUnits(")],
  ["Impl 转发", src.includes("const CompilationStateImpl* Impl(const CompilationState* compilation_state) {")],
];
for (const [n, ok] of anchors) {
  if (!ok) throw new Error(`锚点缺失: ${n}`);
}

// ---------- 编译事件名表 ----------
const evtRegion = src.slice(
  src.indexOf("const char* GetCompilationEventName"),
  src.indexOf("constexpr uint8_t kMainTaskId"),
);
const evtNames = [...evtRegion.matchAll(/return "(wasm\.\w+)";/g)].map((m) => m[1]);
if (evtNames.length !== 4) throw new Error(`编译事件名 ${evtNames.length} 条, 期望 4`);
for (const need of ["wasm.BaselineCompilation", "wasm.TopTierCompilation", "wasm.WasmToJSWrapperCompilation", "wasm.OtherCompilation"]) {
  if (!evtNames.includes(need)) throw new Error(`事件名缺失: ${need}`);
}

// ---------- 方法定义台账 (按类分组; 类内联方法不计, 只收文件级 out-of-line 定义) ----------
// 注: CompilationStateImpl / CompilationState 类本体在 .cc 内定义, 多数方法类内联,
// 此台账只覆盖文件级 `Ret Class::method(` 形态。
const methods = [];
let prevStatic = false;
for (const [i, line] of src.split("\n").entries()) {
  if (/^\/\/ static$/.test(line)) { prevStatic = true; continue; }
  const m = line.match(/^([A-Za-z_][\w:<>,&*]*?)[ *&]+([A-Z]\w*(?:<[^>]*>)?)::~?(\w+)\(/);
  const ctor = line.match(/^([A-Z]\w*)::~?(~?\w+)\(/);
  if (m) methods.push({ cls: m[2], name: m[3], line: i + 1, isStatic: prevStatic });
  else if (ctor && ctor[2].replace("~", "") === ctor[1]) {
    methods.push({ cls: ctor[1], name: ctor[2], line: i + 1, isStatic: prevStatic });
  }
  prevStatic = false;
}
if (methods.length < 60) throw new Error(`方法台账 ${methods.length} 条, 期望 >= 60`);
// 显式模板实例化也入账
const instantiations = [...src.matchAll(/^template void (\w+<\w+>)::(\w+)\(\);/gm)].map(
  (m) => ({ cls: m[1], name: m[2] }),
);
// 分组
const groups = new Map();
for (const m of methods) {
  if (!groups.has(m.cls)) groups.set(m.cls, []);
  groups.get(m.cls).push(m);
}
for (const need of ["AsyncCompileJob", "AsyncStreamingProcessor", "CompilationStateImpl"]) {
  if (!groups.has(need)) throw new Error(`台账缺类: ${need}`);
}

console.log(
  `module-compiler.cc: 事件名=${evtNames.length} 方法=${methods.length} (类=${groups.size}) 显式实例化=${instantiations.length}`,
);

// ---------- 生成 Rust ----------
let rust = `${banner(
  "src/wasm/module-compiler.cc.js",
  "src/wasm/module-compiler.cc",
  "编译事件名表 + kMainTaskId + 按类方法台账",
)}
//! 编译编排层的方法台账与追踪事件名。

/// 编译事件名表 (${evtNames.length} 条)
/// v8: src/wasm/module-compiler.cc GetCompilationEventName
#[rustfmt::skip]
pub static COMPILATION_EVENT_NAMES: &[(&str, &str)] = &[
  ("kLiftoff", "${evtNames[0]}"),
  ("kTurbofan", "${evtNames[1]}"),
  ("func_index < num_imported_functions", "${evtNames[2]}"),
  ("其它", "${evtNames[3]}"),
];

/// 主任务 id (前台同步编译单元使用)
/// v8: src/wasm/module-compiler.cc kMainTaskId
pub const MAIN_TASK_ID: u8 = 0;

/// 后台编译入口: ExecuteCompilationUnits (JobDelegate 驱动, jitless 下 CHECK 失败)
/// v8: src/wasm/module-compiler.cc ExecuteCompilationUnits
pub const BACKGROUND_COMPILE_REQUIRES_JIT: bool = true;

/// CompilationState → CompilationStateImpl 转发 (Impl 辅助)
/// v8: src/wasm/module-compiler.cc Impl
pub const STATE_FORWARDS_TO_IMPL: bool = true;

/// 方法定义台账 (按类分组, ${methods.length} 条 + 显式实例化 ${instantiations.length} 条)
/// v8: src/wasm/module-compiler.cc
#[rustfmt::skip]
pub static METHODS_BY_CLASS: &[(&str, &[(&str, u32)])] = &[
`;
for (const [cls, list] of groups) {
  rust += `  ("${cls}", &[\n`;
  for (const m of list) {
    rust += `    ("${m.name}", ${m.line}),\n`;
  }
  rust += `  ]),\n`;
}
for (const inst of instantiations) {
  rust += `  ("${inst.cls} (显式实例化)", &[("${inst.name}", 0)]),\n`;
}
rust += `];

// —— 提取不变式断言 ——
const _: () = {
    assert!(COMPILATION_EVENT_NAMES.len() == 4);
    assert!(COMPILATION_EVENT_NAMES[0].1 == "wasm.BaselineCompilation");
    assert!(MAIN_TASK_ID == 0);
    assert!(METHODS_BY_CLASS.iter().map(|(_, ms)| ms.len()).sum::<usize>() >= ${methods.length});
    assert!(!METHODS_BY_CLASS.is_empty());
};
`;

const p = writeSrc("v8-wasm-core", "module_compiler_cc.rs.skel", rust);
console.log(`written ${p}`);
