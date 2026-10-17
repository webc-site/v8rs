#!/usr/bin/env bun
// tool/src/regexp/experimental/experimental-interpreter.h.js
//   — v8/src/regexp/experimental/experimental-interpreter.h → v8-regexp-exec/src/experimental/interpreter.rs
// 规则: h 面为 AllStatic 入口类, 全集恰 1 类 × 1 静态方法 FindMatches (9 参) —
//   入口面 (Isolate/Tagged/String/CallOrigin/Zone 依赖) 整体推迟 EXP-4 (h/cc 入口面
//   禁碰票面约束); 本票以参数注入的 NfaInterpreter 核 (interpreter.rs) 承接其声明语义,
//   脚本对数核对承接面锚 + h/cc 形参名差异登记 (capture_count vs register_count_per_match)
// 产物: v8-regexp-exec/src/experimental/interpreter.rs.skel (映射骨架, 审计专用; 落地以就地修订为准)
//   + 落地对数核对 (interpreter.rs/tests 逐项命中, 缺项 throw 列票单)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../../lib.js";

const h = readV8("src/regexp/experimental/experimental-interpreter.h");

let assertCount = 0;
const must = (cond, msg) => {
  assertCount++;
  if (!cond) throw new Error(`对数失配: ${msg}`);
};
const count = (src, re) => (src.match(re) || []).length;

// ── 原版侧解析 (h:1-39 全集机械提取) ────────────────────────────────────────

// 1) 全集类: 恰 1 个 AllStatic 入口类 (前置声明 3 个 class X; 不计)
must(count(h, /^class \w+ final/gm) === 1, `h 定义类应 1, 实得 ${count(h, /^class \w+ final/gm)}`);
must(h.includes("class ExperimentalRegExpInterpreter final : public AllStatic"), "h:20 AllStatic 入口类");
// 前置声明 3 (h:14-16)
for (const fwd of ["class String;", "class TrustedByteArray;", "class Zone;"]) {
  must(h.includes(fwd), `h:14-16 前置声明 ${fwd}`);
}

// 2) 恰 1 个静态方法声明; 参数恰 9 个
must(count(h, /static V8_WARN_UNUSED_RESULT int FindMatches\(/g) === 1, "h:28 FindMatches 唯一静态方法");
must(count(h, /^class \w+;/gm) === 3, "h:14-16 前置声明恰 3");
must(count(h, /static/g) === 1, "h static 恰 1 处 (无其他成员)");
const decl = h.match(/static V8_WARN_UNUSED_RESULT int FindMatches\(([\s\S]*?)\);/)[1];
for (const p of ["Isolate* isolate", "RegExp::CallOrigin call_origin", "Tagged<TrustedByteArray> bytecode",
  "int capture_count", "Tagged<String> input", "int start_index", "int32_t* output_registers",
  "int output_register_count", "Zone* zone"]) {
  must(decl.includes(p), `h:28-32 FindMatches 形参 ${p}`);
}
// h/cc 形参名差异登记: cc:1585 同一位叫 register_count_per_match (h 侧注释语义
// "boundaries of matching subranges" — 实际寄存器数即 2×capture 计数, h 侧命名滞后)
must(!decl.includes("register_count_per_match"), "h 侧应叫 capture_count (与 cc 差异面登记)");

// 3) 依赖 include 面 (入口→字节码 + RegExp::CallOrigin)
must(h.includes('#include "src/regexp/experimental/experimental-bytecode.h"'), "h:8 bytecode 依赖");
must(h.includes('#include "src/regexp/regexp.h"'), "h:9 regexp.h 依赖 (CallOrigin/错误码面)");

// ── 产物: 映射骨架 ─────────────────────────────────────────────────────────
const skel = `${banner(
  "src/regexp/experimental/experimental-interpreter.h.js",
  "src/regexp/experimental/experimental-interpreter.h",
  "映射骨架基线 (审计专用): h 面全集 1 类 1 方法; 落地以 interpreter.rs 就地修订为准",
)}
//! ExperimentalRegExpInterpreter (h:20-33, AllStatic) → 入口面整体推迟 EXP-4
//!   (Isolate/Tagged<TrustedByteArray>/Tagged<String>/CallOrigin/Zone 依赖面,
//!    本席禁触 experimental.h/cc 入口 Rust 落点)
//! 承接核 (EXP-3a 已落, interpreter.rs): NfaInterpreter — h:22-27 语义逐句对应:
//!   广度优先 NFA 无回溯 (cc:252-346 架构注释) / max_match_num×start_index 对应
//!   input_index 注入 / output_registers 对应 best_match_thread 寄存器面
//! 参数注入映射 (h:28-32 → Rust NfaInterpreter::new + find_matches 面):
//!   Tagged<TrustedByteArray> bytecode → &[Instruction] (bytecode.rs IR, 裸解码 EXP-4)
//!   Tagged<String> input → &[u16] (one-byte 入口方展宽, 单点适配)
//!   int start_index → input_index: i32
//!   Isolate/CallOrigin/Zone → 本票不建模 (EXP-4 入口 + HandleInterrupts 面)
//!   int capture_count(h)/register_count_per_match(cc:1585) → register_count_per_match:
//!     usize (h/cc 命名差异登记, 取 cc 实况语义)
`;
writeSrc("v8-regexp-exec", "experimental/interpreter.rs.skel", skel);

// ── 落地对数核对 (interpreter.rs 就地修订版逐项命中; 缺项 throw 列票单) ──────
const landed = readFileSync(join(root, "v8-regexp-exec", "src", "experimental", "interpreter.rs"), "utf8");
const missing = [];
const land = (re, want, label) => {
  const got = count(landed, re);
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
};
// 承接核 + 数据结构族全集
land(/pub struct NfaInterpreter<'a> \{/g, 1, "Rust NfaInterpreter 核");
land(/pub fn new\(\s*bytecode: &'a \[Instruction\],\s*input: &'a \[u16\],\s*input_index: i32,\s*register_count_per_match: usize,\s*capture_group_opt: bool,\s*\) -> Result<Self> \{/g, 1, "Rust new 参数注入签名 (h:28-32 映射面)");
land(/pub struct InterpreterThread \{/g, 1, "Rust InterpreterThread (cc:544 族)");
land(/pub struct LastInputIndex \{/g, 1, "Rust LastInputIndex (cc:1492 族)");
land(/pub struct LookaroundInfo \{/g, 1, "Rust LookaroundInfo (cc:1532 族)");
land(/struct FilterGroups<'a> \{/g, 1, "Rust FilterGroups (cc:98 族)");
land(/pub fn filter_groups\(/g, 1, "Rust filter_groups 公开入口 (h 面 FindMatches 的过滤子语义)");
land(/pub fn satisfies_assertion\(/g, 1, "Rust satisfies_assertion (h:22 无回溯语义的零宽面)");
// 常量全集 4 (含 cc:733 函数内 static constexpr 对数)
land(/pub const K_UNDEFINED_REGISTER_VALUE: i32 = -1;/g, 1, "K_UNDEFINED_REGISTER_VALUE");
land(/pub const K_UNDEFINED_MATCH_INDEX_VALUE: i32 = -1;/g, 1, "K_UNDEFINED_MATCH_INDEX_VALUE");
land(/pub const K_UNDEFINED_CLOCK_VALUE: u64 = u64::MAX;/g, 1, "K_UNDEFINED_CLOCK_VALUE (-1 回绕实测核对)");
land(/pub const K_TICKS_BETWEEN_INTERRUPT_HANDLING: i32 = 64;/g, 1, "K_TICKS_BETWEEN_INTERRUPT_HANDLING = 64");
// 入口推迟登记面 (skel 与 Rust 模块头台账双引用)
land(/FindMatches 入口 cc:1583-1603/g, 1, "EXP-4 推迟登记 FindMatches 行锚");
land(/h:20-33 ExperimentalRegExpInterpreter 声明面/g, 1, "EXP-4 推迟登记 h 声明面行锚");
// 骨架主循环节 (h 面执行语义的核心承接, EXP-3a 仅接线)
land(/pub fn run_active_thread\(&mut self, mut thread: InterpreterThread\) -> Result<\(\)> \{/g, 1, "Rust run_active_thread 签名骨架 (实态按值收线程, 贴合原版 cc:886 `InterpreterThread t` 按值形参)");
land(/InterpreterError::NotImplemented \{/g, 1, "NotImplemented 显式失败通道 (禁 todo! 承接)");
// 禁 todo! / 假绿 (代码区, 剥注释后)
const codeOnly = landed.replace(/^\s*\/\/.*$/gm, "");
if (count(codeOnly, /\b(todo|unimplemented)!\s*\(/g) !== 0) missing.push("interpreter.rs 代码含 todo!/unimplemented! (违规)");
assertCount++;
if (missing.length) throw new Error(`interpreter.rs 落地对数缺项 (票单):\n  ${missing.join("\n  ")}`);

console.log(
  `experimental-interpreter.h.js: 对数断言 ${assertCount} 条全过; 原版 h 面全集 = 1 AllStatic 类 × 1 静态 ` +
    "FindMatches (9 参逐参命中, h/cc 形参名差异 capture_count↔register_count_per_match 登记)" +
    ` / 前置声明 3 / include 2; 落地 15 锚逐项命中 (核+数据结构族 5+公开函数 3+常量 4+推迟登记 2, ` +
    "todo!/unimplemented! 0 处), 缺项 0; 产物 v8-regexp-exec/src/experimental/interpreter.rs.skel",
);
