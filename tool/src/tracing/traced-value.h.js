#!/usr/bin/env bun
// v8/src/tracing/{traced-value.h, traced-value.cc} → v8-tracing/src/traced_value.rs
//
// 提取规则代码:
//   - 方法签名清单 (traced-value.h:31-64, 全 19 个公共方法)
//   - EscapeAndAppendString 转义表 (traced-value.cc:31-69): 6 个显式转义 + \u%04X 范围规则
//   - DEBUG nesting_stack 语义 (kStackTypeDict=false / kStackTypeArray=true)
// 复跑幂等; 语义体 (WriteComma/WriteName/双精度格式化) 就地修订。
import { readV8, writeSrc, banner } from "../../lib.js";

const CRATE = "v8-tracing";
const GEN = "src/tracing/traced-value.h.js";
const H = readV8("src/tracing/traced-value.h");
const CC = readV8("src/tracing/traced-value.cc");

// ---- 转义表: switch case 机械提取 (含 '\\"': "\\\"" 这类嵌套引号) ----
const escapes = [...CC.matchAll(/case '(\\?.)':\s*\*result \+= "((?:[^"\\]|\\.)*)";/g)]
  .map((m) => [m[1], m[2]]);
// case '\b' → \b 之类字面量还原: JS regex 已给 \\b 形态, 直接映射
if (escapes.length !== 7) throw new Error(`显式转义 ${escapes.length} != 7`);

// ---- 方法签名清单 ----
const methods = [...H.matchAll(/^  (?:void|static std::unique_ptr<TracedValue>|~TracedValue\(\))\s+(\w+)/gm)]
  .map((m) => m[1]);
const setN = methods.filter((m) => m.startsWith("Set")).length;
const appendN = methods.filter((m) => m.startsWith("Append")).length;

// rust 转义字面量: c 转义序列 → rust 字节转义
const rustEsc = {
  "\\b": "\\x08", "\\f": "\\x0c", "\\n": "\\n", "\\r": "\\r", "\\t": "\\t",
  '\\"': '"', "\\\\": "\\\\",
};

let out = `${banner(GEN, "src/tracing/traced-value.h;src/tracing/traced-value.cc", "转义表+签名骨架, 语义体就地修订")}
//! TracedValue: 追踪事件的结构化 JSON 参数构造器 (traced-value.h/.cc, 89+231 行)。
//!
//! 原版继承 \`ConvertableToTraceFormat\` (include/v8-platform.h:389) —
//! \`AppendAsTraceFormat\` 输出完整 JSON 对象 (首尾 \`{}\` 由本方法补)。
//! \`EscapeAndAppendString\` (traced-value.cc:31-69) 字节级转义:
//!   显式 6 转义 + c < 0x20 || c == 0x7F → \`\\\\uXXXX\` (大写十六进制, SNPrintF "%04X")。
//! 双精度走 \`internal::DoubleToStringView\` (src/numbers/conversions.cc:1147,
//! ECMA-262 9.8.1 最短表示) — 本模块以 v8-base dtoa(SHORTEST) 等价落地。
//! DEBUG 构建 \`nesting_stack_\` 校验 {Begin,End}{Dictionary,Array} 配对
//! (traced-value.cc:19-29): Rust 以 debug_assert 等价承接。

use crate::platform::ConvertableToTraceFormat;

/// \`EscapeAndAppendString\` (traced-value.cc:31): 带引号转义追加。
pub(crate) fn escape_and_append_string(value: &str, result: &mut String) {
  result.push('"');
  for &c in value.as_bytes() {
    match c {
${escapes.map(([k, v]) => `      b'${rustEsc[k] ?? k}' => result.push_str("${v}"),`).join("\n")}
      _ if c < 0x20 || c == 0x7f => {
        // SNPrintF("\\\\u%04X") — 大写十六进制
        result.push_str(&format!("\\\\u{c:04X}"));
      }
      _ => result.push(c as char),
    }
  }
  result.push('"');
}

// TODO(修订): TracedValue 结构体 + ${methods.length} 方法 (Set×${setN} / Append×${appendN} / Begin/End)
//   data_: String, first_item_: bool, [debug] nesting_stack_: Vec<bool>
//   SetDouble/AppendDouble → double_to_string (ECMA-262 最短表示)
`;

writeSrc(CRATE, "traced_value.rs", out);
console.log(`== traced-value.h.js == 转义=${escapes.length} 方法=${methods.length} (Set×${setN}, Append×${appendN})`);
console.log(`方法: ${methods.join(", ")}`);
