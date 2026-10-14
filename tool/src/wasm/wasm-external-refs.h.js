#!/usr/bin/env bun
// tool/src/wasm/wasm-external-refs.h.js
// 机械提取 v8/src/wasm/wasm-external-refs.{h,cc} 的外部引用函数清单与浮点语义

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-external-refs.h");
const ccSrc = readV8("src/wasm/wasm-external-refs.cc");

// 1. h: 真解析全部 wrapper 声明 (含导出与非导出)
const wrappers = [...hSrc.matchAll(/(\w[\w\d]*_wrapper)\s*\(/g)].map((m) => m[1]);
const uniqWrappers = [...new Set(wrappers)];
if (uniqWrappers.length < 70) throw new Error(`wrapper 数 ${uniqWrappers.length} < 70`);

// 2. h: Address 约定 (data 指针即参数块)
if (!hSrc.includes("using Address = uintptr_t;")) throw new Error("未找到 Address 别名");

// 3. 按 SIMD lane 形状分组
const groups = {
  scalarF32: uniqWrappers.filter((w) => /^f32_/.test(w)),
  scalarF64: uniqWrappers.filter((w) => /^f64_(trunc|floor|ceil|nearest)/.test(w)),
  converts: uniqWrappers.filter((w) => /^(int64|uint64|float32|float64|float16)_to/.test(w) || /^float16_to/.test(w)),
  intDiv: uniqWrappers.filter((w) => /^(int64|uint64)_(div|mod)/.test(w)),
  wordRollook: uniqWrappers.filter((w) => /^word(32|64)_(rol|ror)/.test(w)),
  f64x2: uniqWrappers.filter((w) => /^f64x2_/.test(w)),
  f32x4: uniqWrappers.filter((w) => /^f32x4_/.test(w)),
  f16x8: uniqWrappers.filter((w) => /^f16x8_/.test(w) || /^(i16x8_(s|u)convert|f32x4_promote|f16x8_demote)/.test(w)),
};
const grouped = Object.values(groups).reduce((a, b) => a + b.length, 0);
if (grouped < 60) throw new Error(`分组覆盖 ${grouped} < 60`);

// 4. h: 内存三操作 (int32 返回值语义)
for (const m of ["memory_init_wrapper", "memory_copy_wrapper", "memory_fill_wrapper"]) {
  if (!hSrc.includes(m)) throw new Error(`内存操作缺失: ${m}`);
  if (!hSrc.includes("int32_t")) throw new Error("返回值非 int32");
}
if (!hSrc.includes("// The return type is {int32_t} instead of {bool} to enforce the compiler to")) {
  throw new Error("未找到 int32 返回注释");
}

// 5. h: 栈切换族
const stackFns = ["start_stack", "suspend_stack", "resume_jspi_stack", "resume_wasmfx_stack", "return_jspi_stack", "return_wasmfx_stack", "retire_stack"];
for (const s of stackFns) {
  if (!hSrc.includes(`${s}(`)) throw new Error(`栈切换缺失: ${s}`);
}

// 6. cc: 浮点语义锚 (nearbyint 不是 rint; AIX workaround)
if (!ccSrc.includes("nearbyintf(input);")) throw new Error("未找到 nearbyintf");
if (!ccSrc.includes("nearbyint(input);")) throw new Error("未找到 nearbyint");
if (!ccSrc.includes("FpOpWorkaround")) throw new Error("未找到 AIX workaround");

// 7. cc: 中心栈切换
if (!ccSrc.includes("switch_to_the_central_stack")) throw new Error("未找到中心栈切换实现");

console.log(
  `wasm-external-refs: wrapper=${uniqWrappers.length} 分组覆盖=${grouped} 栈族=${stackFns.length}`,
);

// 8. 生成 Rust
let rust = banner(
  "src/wasm/wasm-external-refs.h.js",
  "src/wasm/wasm-external-refs.{h,cc}",
  "wasm 外部引用 (软浮点/SIMD/内存/栈切换) 清单",
);

rust += `
//! 生成代码经 CSignature 调用的运行时辅助函数族。
/// v8: src/wasm/wasm-external-refs.h wasm 命名空间

/// 全部 *_wrapper 函数 (软实现入口, data 参数指向操作数块)
/// v8: src/wasm/wasm-external-refs.h 各 _wrapper 声明
pub const WRAPPER_FUNCTIONS: &[&str] = &[
${uniqWrappers.map((w) => `    "${w}",`).join("\n")}
];

/// 标量取整族语义: trunc/floor/ceil 直调 libc; nearest 用 nearbyint (非 rint, 不受舍入模式影响由调用方保证)
/// v8: src/wasm/wasm-external-refs.cc f32_nearest_int_wrapper
pub const NEAREST_INT_USES_NEARBYINT: bool = true;

/// 内存三操作返回 int32 而非 bool: 强制编译器零扩展返回寄存器
/// v8: src/wasm/wasm-external-refs.h 注释
pub const MEMORY_OPS_RETURN_INT32: bool = true;
pub const MEMORY_OPERATIONS: &[&str] = &["memory_init_wrapper", "memory_copy_wrapper", "memory_fill_wrapper"];

/// array_copy 前置: 区间已判界且 length > 0
/// v8: src/wasm/wasm-external-refs.h array_copy_wrapper 注释
pub const ARRAY_COPY_ASSUMES_INBOUNDS: bool = true;

/// JSPI / WasmFX 栈切换入口 (sp/fp/pc 三元组穿梭)
/// v8: src/wasm/wasm-external-refs.h start_stack 等
pub const STACK_SWITCH_FUNCTIONS: &[&str] = &[
${stackFns.map((s) => `    "${s}",`).join("\n")}
    "suspend_wasmfx_stack",
    "switch_wasmfx_stack",
    "cont_bind",
    "wasmfx_set_wasm_code",
];

/// 中心栈切换 (JS 互操作双向)
/// v8: src/wasm/wasm-external-refs.h switch_to/from_the_central_stack
pub const CENTRAL_STACK_SWITCH: &[&str] = &[
    "switch_to_the_central_stack",
    "switch_from_the_central_stack",
    "switch_to_the_central_stack_for_js",
    "switch_from_the_central_stack_for_js",
    "grow_stack",
    "shrink_stack",
];

// —— 提取不变式断言 ——
const _: () = {
    assert!(WRAPPER_FUNCTIONS.len() >= 70);
    assert_eq!(MEMORY_OPERATIONS.len(), 3);
    assert_eq!(STACK_SWITCH_FUNCTIONS.len(), ${stackFns.length} + 4);
};
`;

const p = writeSrc("v8-wasm-core", "wasm_external_refs.rs.skel", rust);
console.log(`written ${p}`);
