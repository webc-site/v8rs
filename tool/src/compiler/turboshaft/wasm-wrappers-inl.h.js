#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-wrappers-inl.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-wrappers-inl.h
// → v8-turboshaft/src/wasm_wrappers_inl.rs.skel
//
// 席位表要求 -inl.h 同名族独立脚本; wasm-wrappers.h.js 已覆盖 .{h,cc} 的
// 类型/枚举面, 本脚本只提 -inl.h 的 WasmWrapperTSGraphBuilder 方法面:
// 13 个模板方法签名 (逐参登记) + 关键分支语义锚。
// 方法体依赖 Isolate/builtin 调用/heap 执行面, 属深水项, 只出签名骨架。
import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/compiler/turboshaft/wasm-wrappers-inl.h");

// ---------------------------------------------------------------------------
// 1. 方法面提取: (void|auto) WasmWrapperTSGraphBuilder<Assembler>::Name(参数)
// ---------------------------------------------------------------------------
const methodRe =
  /(void|auto)\s+WasmWrapperTSGraphBuilder<Assembler>::(\w+)\(\s*([^;{}]*?)\)/g;
const methods = [];
for (const m of src.matchAll(methodRe)) {
  const params = m[3]
    .replace(/\/\/[^\n]*/g, "")
    .split(",")
    .map((p) => p.trim())
    .filter((p) => p.length && p !== "void");
  methods.push({ ret: m[1], name: m[2], params });
}
// 对数: 原版 13 个模板方法
const EXPECT = [
  "AbortIfNot",
  "BuildChangeInt32ToNumber",
  "BuildToJSFunctionRef",
  "CheckAndConvertSharedString",
  "ToJS",
  "BuildCallWasmFromWrapper",
  "ConvertWasmResultsToJS",
  "BuildJSToWasmWrapper",
  "BuildWasmToJSWrapper",
  "BuildWasmStackEntryWrapper",
  "BuildCapiCallWrapper",
  "BuildCWasmEntryWrapper",
  "BuildJSFastApiCallWrapper",
];
const names = methods.map((x) => x.name);
for (const e of EXPECT) {
  if (!names.includes(e)) throw new Error(`缺方法 ${e}`);
}
const extra = names.filter((n) => !EXPECT.includes(n));
if (extra.length) throw new Error(`未知方法: ${extra.join(",")}`);
// BuildJSToWasmWrapper 有 重载 (无参入口 + 带参构建), 共 14 定义/13 唯一名
const uniq = new Set(names);
if (uniq.size !== EXPECT.length || names.length !== EXPECT.length + 1) {
  throw new Error(`方法面漂移: ${names.length} 定义 / ${uniq.size} 唯一名`);
}

// ---------------------------------------------------------------------------
// 2. 语义哨兵 (关键分支锚, 复跑防漂移)
// ---------------------------------------------------------------------------
for (const [re, note] of [
  [/if \(is_inlining_into_js_\)\s*\{\s*return __ ConvertInt32ToNumber/, "Int32→Number 的 JS 内联分支"],
  [/if constexpr \(SmiValuesAre32Bits\(\)\)\s*\{\s*return __ TagSmi/, "Smi32 直接 TagSmi"],
  [/Int32AddCheckOverflow\(value, value\)/, "Smi31 溢出试算"],
  [/WasmInt32ToHeapNumberDescriptor/, "HeapNumber 回退 builtin"],
  [/kWasmInternalFunctionIndirectPointerTag/, "ToJSFunctionRef 可信指针加载"],
  [/RootIndex::kUndefinedValue/, "external_ 未初始化判定"],
  [/AbortReason/, "AbortIfNot 调试断言面"],
]) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${note}`);
}

// 各方法原文行区间 (供 Reviewer 对照); 重载按出现次序逐一登记
const lineCursor = new Map();
function lineOf(name) {
  const arr = src.split("\n");
  const start = lineCursor.get(name) || 0;
  let found = arr.length;
  for (let i = start; i < arr.length; i++) {
    if (
      arr[i].includes(`::${name}(`) ||
      (arr[i].includes("WasmWrapperTSGraphBuilder<Assembler>") &&
        arr[i + 1]?.trim().startsWith(`${name}(`))
    ) {
      found = i;
      break;
    }
  }
  lineCursor.set(name, found + 1);
  return found + 1;
}

// ---------------------------------------------------------------------------
// 3. 生成骨架
// ---------------------------------------------------------------------------
let rust = banner(
  "src/compiler/turboshaft/wasm-wrappers-inl.h.js",
  "src/compiler/turboshaft/wasm-wrappers-inl.h",
  "Wasm wrapper TS 图构建器方法面 — 深水面登记, 签名 1:1",
);

rust += `
/// 原版 WasmWrapperTSGraphBuilder<Assembler> 的 -inl.h 方法面。
/// 类型与成员字段面见 wasm_wrappers.rs (wasm-wrappers.h.js 产出);
/// 本模块只承载 13 个模板方法的签名骨架。
/// v8: src/compiler/turboshaft/wasm-wrappers-inl.h WasmWrapperTSGraphBuilder
pub struct WasmWrapperTsGraphBuilder {
  /// v8: is_inlining_into_js_ — 内联进 JS 管线时走高层转换
  pub is_inlining_into_js: bool,
}

impl WasmWrapperTsGraphBuilder {
`;

const seen = new Map();
for (const m of methods) {
  const ln = lineOf(m.name);
  rust += `  /// v8: src/compiler/turboshaft/wasm-wrappers-inl.h L${ln} ${m.name}(${m.params.join(", ")})\n`;
  let snake = m.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();
  const n = seen.get(snake) || 0;
  seen.set(snake, n + 1);
  if (n === 1) snake += "_entry"; else if (n > 1) snake += `_${n}`;
  rust += `  pub fn ${snake}(&mut self) { todo!() }\n\n`;
}

rust += `}

// —— 深水面登记 (推迟台账) ——
// 13 个方法体全部依赖 Isolate 执行面: CallBuiltin<Descriptor>、
// WasmCallRuntime、LoadRoot、TrustedPointer 加载、Simulator 边界。
// 解锁前置: v8-execution-isolate / v8-roots / v8-objects-maps /
// v8-compiler-broker (builtin 描述符) 与 assembler.rs 的 __ 门面。
`;

const p = writeSrc("v8-turboshaft", "wasm_wrappers_inl.rs.skel", rust);
console.log(p);
