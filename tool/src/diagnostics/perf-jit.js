#!/usr/bin/env bun
// tool/src/perf-jit 侧: tool/src/diagnostics/perf-jit.js — 从 v8/src/diagnostics/perf-jit.{h,cc} 提取
// PerfJitHeader/Base/CodeLoad/DebugEntry/CodeDebugInfo/CodeUnwindingInfo 记录结构、
// PerfJitEvent 枚举、ELF machine 常量、文件名/缓冲区常量,
// 生成 v8-diagnostics/src/perf_jit_data.rs (纯记录定义面; 序列化流程见 perf_jit.rs)
// 复跑幂等: 纯文本扫描 + 源码序输出, 无时间戳。
import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/diagnostics/perf-jit.cc");
const h = readV8("src/diagnostics/perf-jit.h");
const internalH = readV8("include/v8-internal.h");

let n = 0;
const die = (m) => { throw new Error(`[perf-jit.js] ${m}`); };
const need = (src, s, where) => { if (!src.includes(s)) die(`${where} 缺针 ${s}`); n++; };

// 解析 struct 定义: 名称 / 基类 / 字段 (类型, 名) / 静态常量 (名, 原值文本)
function parseStruct(name) {
  const m = cc.match(new RegExp(`struct ${name}(?:\\s*:\\s*(\\w+))?\\s*\\{([\\s\\S]*?)\\n\\};`));
  if (!m) die(`struct ${name} 未找到`);
  const body = m[2].split("\n")
    .filter((l) => !l.trim().startsWith("//"))
    .filter((l) => !/enum\s+\w+\s*\{[\s\S]*?\}/.test(l))
    .join("\n");
  const fields = [];
  const consts = [];
  for (const line of body.split(";")) {
    const t = line.trim();
    if (!t) continue;
    let fm = t.match(/^static const (\w+) (\w+) = ([\w\s|]+)$/);
    if (fm) { consts.push([fm[2], fm[3]]); continue; }
    fm = t.match(/^(uint32_t|uint64_t|int)\s+(\w+)$/);
    if (fm) { fields.push([fm[1], fm[2].replace(/_$/, "")]); continue; }
    fm = t.match(/^enum PerfJitEvent \{([\s\S]*)$/);
    if (fm) continue; // 枚举单独处理
    if (/kLoad = |kMove = |kDebugInfo = |kClose = |kUnwindingInfo = /.test(t)) continue;
    die(`struct ${name} 无法解析行: ${JSON.stringify(t)}`);
  }
  return { base: m[1], fields, consts };
}

const TYPES = { uint32_t: "u32", uint64_t: "u64", int: "i32" };
const STRUCTS = ["PerfJitHeader", "PerfJitBase", "PerfJitCodeLoad", "PerfJitDebugEntry",
  "PerfJitCodeDebugInfo", "PerfJitCodeUnwindingInfo"];
const parsed = Object.fromEntries(STRUCTS.map((s) => [s, parseStruct(s)]));

// 事件枚举 (5 项)
const events = [...cc.matchAll(/(kLoad|kMove|kDebugInfo|kClose|kUnwindingInfo) = (\d+)/g)]
  .map((m) => [m[1], parseInt(m[2])]);
if (events.length !== 5) die(`PerfJitEvent 项数 ${events.length} != 5`);
n++;

// 头部填充/保留与序列化关键针
need(cc, 'header.reserved_ = 0xDEADBEEF;', "perf-jit.cc");
need(cc, 'header.flags_ = 0;', "perf-jit.cc");
need(cc, "code_load.size_ =\n      static_cast<uint32_t>(sizeof(code_load) + name_length + 1 + code_size);", "perf-jit.cc");
need(cc, 'constexpr char kUnknownScriptNameString[] = "<unknown>";', "perf-jit.cc");
need(cc, "char padding_bytes[8] = {0};", "perf-jit.cc");
need(cc, 'const char PerfJitLogger::kFilenameFormatString[] = "%s/jit-%d.dump";', "perf-jit.cc");
need(cc, "const int PerfJitLogger::kFilenameBufferPadding = 16;", "perf-jit.cc");
need(h, "static const int kLogBufferSize = 2 * MB;", "perf-jit.h");
need(internalH, "constexpr int MB = KB * 1024;", "v8-internal.h");
need(internalH, "constexpr int KB = 1024;", "v8-internal.h");

// ELF machine 常量 (9 个)
const machs = [...h.matchAll(/kElfMach(\w+) = (\d+);/g)].map((m) => [`ELF_MACH_${m[1].toUpperCase()}`, parseInt(m[2])]);
if (machs.length !== 9) die(`kElfMach 常量数 ${machs.length} != 9`);
n++;
// ELF 头尺寸 (32/64 位)
const elfHdr32 = parseInt(grabNeed(h, /kElfHeaderSize = (0x34);/, "kElfHeaderSize32"));
const elfHdr64 = parseInt(grabNeed(h, /kElfHeaderSize = (0x40);/, "kElfHeaderSize64"));
function grabNeed(src, re, what) {
  const m = src.match(re);
  if (!m) die(`未找到 ${what}`);
  n++;
  return m[1];
}

const SCREAM = (x) => x.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
const out = [];
out.push(banner("src/diagnostics/perf-jit.js", "src/diagnostics/perf-jit.h;src/diagnostics/perf-jit.cc"));
out.push(`//! perf-jit (jit-<pid>.dump) 记录纯定义面 (基线生成; 序列化流程见 perf_jit.rs)。`);
out.push(`//! Isolate/文件 IO 面推迟, 时间戳/PID/TID 由调用方注入。`);
out.push(``);
for (const s of STRUCTS) {
  const { base, fields, consts } = parsed[s];
  if (consts.length) {
    for (const [name, val] of consts) {
      const num = val.trim();
      out.push(`pub const ${SCREAM(s)}_${name.replace(/^k/, "").toUpperCase()}: u32 = ${num};`);
    }
  }
  out.push(`/// 上游 \`${s}${base ? ` : ${base}` : ""}\` (${fields.length} 字段, repr(C) 与 C++ POD 布局一致)`);
  out.push(`#[repr(C)]`);
  out.push(`#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]`);
  out.push(`pub struct ${s} {`);
  if (base) out.push(`  pub base: ${base},`);
  for (const [ty, name] of fields) out.push(`  pub ${name}: ${TYPES[ty]},`);
  out.push(`}`);
  out.push(``);
}
out.push(`// ── PerfJitEvent (${events.length} 项) ──`);
for (const [name, val] of events) out.push(`pub const EVENT_${name.replace(/^k/, "").toUpperCase()}: u32 = ${val};`);
out.push(``);
out.push(`// ── ELF machine 常量 (${machs.length} 项) ──`);
for (const [name, val] of machs) out.push(`pub const ${name}: u32 = ${val};`);
out.push(``);
out.push(`// ── ELF 头尺寸 / 文件与缓冲常量 ──`);
out.push(`pub const ELF_HEADER_SIZE_32: usize = ${elfHdr32};`);
out.push(`pub const ELF_HEADER_SIZE_64: usize = ${elfHdr64};`);
out.push(`pub const HEADER_RESERVED_FILL: u32 = 0xDEADBEEF;`);
out.push(`pub const FILENAME_FORMAT: &str = "%s/jit-%d.dump";`);
out.push(`pub const FILENAME_BUFFER_PADDING: usize = 16;`);
out.push(`pub const LOG_BUFFER_SIZE: usize = 2 * 1024 * 1024;`);
out.push(`pub const UNKNOWN_SCRIPT_NAME: &str = "<unknown>";`);
out.push(`pub const RECORD_PAD: usize = 8;`);
out.push(``);

writeSrc("v8-diagnostics", "perf_jit_data.rs", out.join("\n").trimEnd() + "\n");

// 记录布局派生 (64 位): 供日志/测试金标准
const sizeOf = (s) => {
  const { base, fields } = parsed[s];
  let off = 0;
  const align = (t) => (t === "u64" ? 8 : 4);
  const width = (t) => (t === "u64" ? 8 : 4);
  if (base) off = sizeOf(base);
  for (const [, name] of fields) {
    const t = TYPES[fields.find((f) => f[1] === name)[0]];
    off = Math.ceil(off / align(t)) * align(t) + width(t);
  }
  return off;
};
const sizes = Object.fromEntries(STRUCTS.map((s) => [s, sizeOf(s)]));
console.log(`OK perf-jit.js: 对数 ${n} (struct 6 + 事件 5 + elfmach 9 + 头尺寸 2 + 针 9) → v8-diagnostics/src/perf_jit_data.rs`);
console.log(`  记录尺寸: ${STRUCTS.map((s) => `${s}=${sizes[s]}`).join(" ")}`);
console.log(`  magic=0x4A695444 version=1 log_buffer=${2 * 1024 * 1024}`);
