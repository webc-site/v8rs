#!/usr/bin/env bun
// tool/src/compiler/turboshaft/wasm-in-js-inlining-reducer-inl.h.js
// 机械提取 v8/src/compiler/turboshaft/wasm-in-js-inlining-reducer-inl.h
// → v8-turboshaft/src/wasm_in_js_inlining_reducer.rs.skel
//
// 提取面: REDUCE 面; WasmInJsInliningInterface (decoder 回调面);
// kUsesPoppedArgs; kMaxArguments 哨兵。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/wasm-in-js-inlining-reducer-inl.h");

// 1. REDUCE 面
const reduces = [...h.matchAll(/\bREDUCE\((\w+)\)\(/g)].map((m) => m[1]);
if (reduces.length !== 2 || !reduces.includes("ProcessWasmArgument") || !reduces.includes("Call")) {
  throw new Error(`REDUCE 面 ${reduces.join(",")} != ProcessWasmArgument/Call`);
}

// 2. interface 哨兵
for (const [re, name] of [
  [/class WasmInJSInliningReducer : public Next/, "reducer 类"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(WasmInJSInlining\)/, "boilerplate"],
  [/class WasmInJsInliningInterface \{/, "decoder 接口"],
  [/using FullDecoder =\s*\n\s*wasm::WasmFullDecoder<ValidationTag, WasmInJsInliningInterface>;/, "FullDecoder 别名"],
  [/using ValidationTag = wasm::Decoder::NoValidationTag;/, "免校验 tag"],
  [/static constexpr bool kUsesPoppedArgs = false;/, "kUsesPoppedArgs=false"],
  [/constexpr uint16_t kMaxArguments = 0;/, "kMaxArguments=0"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 3. decoder 回调面计数
const decoderCallbacks = [...new Set([...h.matchAll(/^\s{2}(?:V<\w+>|std::optional<\w+>|wasm::\w+|void|bool|std::monostate|WasmCodeOffsets|detail::\w+|int)\s+(\w+)\(/gm)].map((m) => m[1]))];

let rust = banner(
  "src/compiler/turboshaft/wasm-in-js-inlining-reducer-inl.h.js",
  "src/compiler/turboshaft/wasm-in-js-inlining-reducer-inl.h",
  "wasm-in-JS 内联 reducer: 字节码级内联执行",
);

rust += `
/// 原版 WasmInJSInliningReducer — 拦 Call 到 wasm 函数时,
/// 借 WasmFullDecoder 在 JS 图内直接执行 wasm 字节码。
/// v8: src/compiler/turboshaft/wasm-in-js-inlining-reducer-inl.h
pub struct WasmInJSInliningReducer;

impl WasmInJSInliningReducer {
  /// v8: REDUCE(ProcessWasmArgument)(value, type) — JS 值到 wasm 表示
  pub fn reduce_process_wasm_argument(&mut self, value: V<Object>, ty: ValueType) -> V<Object> { todo!() }
  /// v8: REDUCE(Call)(callee, arguments, label, feedback, descriptor, frame_state)
  /// — 命中可内联 wasm 目标时走 decoder; 否则透传
  pub fn reduce_call(&mut self) -> V<Any> { todo!() }
}

/// 原版 WasmInJsInliningInterface — wasm 字节码 decoder 回调面
/// (${decoderCallbacks.length} 个方法名):
${decoderCallbacks.map((c) => `/// - ${c}`).join("\n")}
/// 约束: ValidationTag = NoValidationTag (已在前端校验);
/// kUsesPoppedArgs = false; 暂不支持控制流 (Control = ControlBase)。
/// v8: wasm-in-js-inlining-reducer-inl.h WasmInJsInliningInterface
pub struct WasmInJsInliningInterface<'a> {
  pub asm: &'a mut Assembler,
  pub arguments: Vec<OpIndex>,
  pub trusted_instance_data: V<WasmTrustedInstanceData>,
  pub frame_state: V<EagerFrameState>,
  pub inlining_id: i32,
}
`;

const p = writeSrc("v8-turboshaft", "wasm_in_js_inlining_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`REDUCE=${reduces.length} 回调面=${decoderCallbacks.length}`);
