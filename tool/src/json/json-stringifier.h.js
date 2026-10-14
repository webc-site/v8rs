#!/usr/bin/env bun
// tool/src/json/json-stringifier.h.js
// 机械提取 v8/src/json/json-stringifier.h: JsonStringify 入口签名骨架
// (.cc 侧词法表已由 tool/src/json/json-stringifier.cc.js 产出 escape_table.rs, 不重复)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/json/json-stringifier.h");

// 对数守卫: 恰好一个 JsonStringify 声明, 参数 4 个
const m = hSrc.match(
  /V8_WARN_UNUSED_RESULT\s+MaybeDirectHandle<Object>\s+JsonStringify\(([^)]*)\);/,
);
if (!m) throw new Error("未找到 JsonStringify 声明");
const params = m[1]
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
if (params.length !== 4) throw new Error(`JsonStringify 参数应 4, 实得 ${params.length}: ${params.join(", ")}`);
const expected = [
  "Isolate* isolate",
  "Handle<JSAny> object",
  "Handle<JSAny> replacer",
  "Handle<Object> gap",
];
if (params.join("|") !== expected.join("|")) throw new Error("JsonStringify 签名漂移: " + params.join(" | "));

let rust = banner("src/json/json-stringifier.h.js", "src/json/json-stringifier.h", "JSON 序列化入口签名");

rust += `
/// JSON 序列化总入口 (原版 JsonStringify, json-stringifier.h 唯一声明)。
///
/// 对应 C++ 签名:
///   MaybeDirectHandle<Object> JsonStringify(Isolate*, Handle<JSAny> object,
///                                           Handle<JSAny> replacer, Handle<Object> gap)
///
/// 语义实现见 crate 内手写 stringifier.rs; escape 表见 escape_table.rs。
pub mod stringifier_api_skel {
  /// 序列化失败哨兵 (原版 MaybeHandle 空)
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum StringifyResult {
    /// 成功产出 JSON 文本
    Ok,
    /// 值含 BigInt / 循环引用等, 原版抛 TypeError
    TypeError,
  }

  /// 入口骨架: object + replacer(array/function) + gap(缩进串)
  pub fn json_stringify(
    _isolate_key: usize,
    _object: usize,
    _replacer: usize,
    _gap: usize,
  ) -> StringifyResult {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-json", "stringifier_api_generated.rs", rust);
console.log(`written ${p}; 参数=${params.length}`);
