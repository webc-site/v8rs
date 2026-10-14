#!/usr/bin/env bun
// tool/src/compiler/js-operator.cc.js
// 从 v8/src/compiler/js-operator.h + js-operator.cc
// 算子参数类主面 (CallParameters/ConstructParameters) 已冻结在 v8-compiler-ops,
// 本脚本产前端差集: ForInMode 枚举、JSWasmCallParameters 参数类、cc 定义面表
// 生成 v8-compiler-frontend/src/js_operator.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-operator.h");
const cc = readV8("src/compiler/js-operator.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["enum class ForInMode : uint8_t {", "ForInMode 枚举"],
  ["kUseEnumCacheKeysAndIndices", "ForInMode 枚举项 1"],
  ["kUseEnumCacheKeys", "ForInMode 枚举项 2"],
  ["kGeneric", "ForInMode 枚举项 3"],
  ["class JSWasmCallParameters final {", "Wasm 调用参数类"],
  ["class ConstructParameters final {", "Construct 参数类 (上游已冻结)"],
  ["class CallParameters final {", "Call 参数类 (上游已冻结)"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-operator.h 锚点缺失: ${name}`);
}

// —— 2. ForInMode 枚举项提取 ——
const forInBlock = h.match(/enum class ForInMode : uint8_t \{([^}]+)\}/)[1];
const forInItems = [...forInBlock.matchAll(/k(\w+)/g)].map((m) => m[1]);
if (forInItems.length !== 3) throw new Error(`ForInMode 项数异常: ${forInItems.length}`);

// —— 3. cc 定义面 (operator 工厂计数, 对数基准) ——
const ops = [...new Set(
  [...cc.matchAll(/const Operator\* JSOperatorBuilder::(\w+)\(/g)].map((m) => m[1]),
)];

const body = `${banner("src/compiler/js-operator.cc.js", "src/compiler/js-operator.h + js-operator.cc", "JS 算子前端差集: ForInMode / JSWasmCallParameters (主面在 v8-compiler-ops)")}

/// for-in 遍历模式
/// v8: src/compiler/js-operator.h ForInMode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ForInMode {
  /// 枚举缓存 keys+indices
  /// v8: src/compiler/js-operator.h ForInMode::kUseEnumCacheKeysAndIndices
  UseEnumCacheKeysAndIndices = 0,
  /// 枚举缓存 keys
  /// v8: src/compiler/js-operator.h ForInMode::kUseEnumCacheKeys
  UseEnumCacheKeys = 1,
  /// 通用
  /// v8: src/compiler/js-operator.h ForInMode::kGeneric
  Generic = 2,
}

/// wasm 调用算子参数 — native module 指针 + slot
/// v8: src/compiler/js-operator.h JSWasmCallParameters
pub struct JSWasmCallParameters {
  /// 原版 wasm::NativeModule* (执行面, 推迟)
  pub native_module: *const (),
  pub slot: u64,
  pub signature: i32,
}

// —— 对数基准 ——
// ForInMode ${forInItems.length} 项 / cc 工厂定义面 ${ops.length} 项 (归 v8-compiler-ops JSOperatorBuilder)
`;

const p = writeSrc("v8-compiler-frontend", "js_operator.rs.skel", body);
console.log(`OK ${p} ops=${ops.length}`);
