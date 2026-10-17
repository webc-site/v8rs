#!/usr/bin/env bun
// tool/src/heap/base/unsafe-json-emitter.js — unsafe_json_emitter.rs 初版生成
// 组→文件映射 (本脚本管辖):
//   v8/src/heap/base/unsafe-json-emitter.h + unsafe-json-emitter.cc
//   → v8-heap-base/src/unsafe_json_emitter.rs
// 规则:
//   - emit_value 重载集 (bool / const char* / 浮点/整数) 机械提取
//   - p 的逗号/first_ 语义骨架
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/heap/base/unsafe-json-emitter.h");
const cc = readV8("src/heap/base/unsafe-json-emitter.cc");

// ---- 锚点断言 ----
for (const fn of ["object_start", "object_end", "p", "ToString", "emit_property_name", "emit_value"]) {
  if (!new RegExp(`\\b${fn}\\s*\\(`).test(h + cc)) throw new Error(`unsafe-json-emitter: ${fn} 缺失`);
}
// 重载集证据
if (!/void emit_value\(bool b\)/.test(h)) throw new Error("emit_value(bool) 缺失");
if (!/void emit_value\(const char\* value\)/.test(h)) throw new Error("emit_value(const char*) 缺失");
if (!/is_floating_point_v<T> \|\| std::is_integral_v<T>/.test(h)) throw new Error("emit_value 算术约束不符");
// 首属性逗号语义
if (!/if \(!first_\) out_ << ",";/.test(h)) throw new Error("p 逗号语义不符");
// object_start 重置 first_
if (!cc.includes("first_ = true;")) throw new Error("object_start 未重置 first_");

const body = `${banner("src/heap/base/unsafe-json-emitter.js", "src/heap/base/unsafe-json-emitter.h + unsafe-json-emitter.cc", "信任输入的 JSON 拼接器")}
//! UnsafeJsonEmitter: 只用于可信输入的轻量 JSON 对象拼接。
//! 重载集 (机械提取): bool → true/false; 字符串 → 引号包裹; 算术 → 原样输出。
//! 偏差: C++ ostream 浮点默认 6 位有效数字; Rust {} 为最短往返表示。

use std::fmt::Write as _;

/// 原 UnsafeJsonEmitter (unsafe-json-emitter.h:15)。
pub struct UnsafeJsonEmitter {
  out: String,
  first: bool,
}

/// 对应 emit_value 重载集: 各类型的输出协议。
pub trait JsonValue {
  fn emit(&self, out: &mut String);
}

impl JsonValue for bool {
  /// 原 emit_value(bool): true/false。
  fn emit(&self, out: &mut String) { todo!("就地修订") }
}

impl JsonValue for &str {
  /// 原 emit_value(const char*): "\\""...\\""。
  fn emit(&self, out: &mut String) { todo!("就地修订") }
}

impl JsonValue for f64 { fn emit(&self, out: &mut String) { todo!("就地修订") } }
impl JsonValue for f32 { fn emit(&self, out: &mut String) { todo!("就地修订") } }

impl UnsafeJsonEmitter {
  pub fn new() -> Self { todo!("就地修订") }
  /// 原 object_start (cc:9-13): '{' + first_ = true。
  pub fn object_start(&mut self) -> &mut Self { todo!("就地修订") }
  /// 原 object_end (cc:15-18)。
  pub fn object_end(&mut self) -> &mut Self { todo!("就地修订") }
  /// 原 p (h:23-29): 非首属性前置逗号; "\\""name\\":value"。
  pub fn p(&mut self, name: &str, value: impl JsonValue) -> &mut Self { todo!("就地修订") }
  /// 原 ToString (cc:20)。
  pub fn to_string(&self) -> String { todo!("就地修订") }
  /// 原 emit_property_name (cc:22-24)。
  fn emit_property_name(&mut self, name: &str) { todo!("就地修订") }
}
`;

const p = writeSrc("v8-heap-base", "unsafe_json_emitter.rs", body);
console.log(`unsafe-json-emitter.js: ${p} (方法 6 项 + 重载集 3 类 + 逗号/first_ 语义核对通过)`);
