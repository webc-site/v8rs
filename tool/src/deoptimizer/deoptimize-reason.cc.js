#!/usr/bin/env bun
// tool/src/deoptimizer/deoptimize-reason.cc.js
// 机械提取 v8/src/deoptimizer/ 主体族: deoptimize-reason.{h,cc} + deoptimizer.{h,cc}
// + frame-description.h — 反优化原因表/种类枚举引用/定义头清单

import { readV8, writeSrc, banner } from "../../lib.js";

const reasonH = readV8("src/deoptimizer/deoptimize-reason.h");
const reasonCc = readV8("src/deoptimizer/deoptimize-reason.cc");
const hSrc = readV8("src/deoptimizer/deoptimizer.h");
const ccSrc = readV8("src/deoptimizer/deoptimizer.cc");
const fdH = readV8("src/deoptimizer/frame-description.h");
const globalsH = readV8("src/common/globals.h");

// 1. 原因表 (eager: DEOPTIMIZE_REASON_LIST 含 builtin 子表; lazy 独立枚举)
function parseVList(src, listName) {
  // 宏体: 从 #define 起至首个未被续行符 \ 引出的换行
  const start = src.indexOf(`#define ${listName}(V)`);
  if (start < 0) throw new Error(`未找到 ${listName}`);
  let end = start;
  const ls = src.slice(start).split("\n");
  let body = "";
  for (const line of ls) {
    body += line.replace(/\\/g, " ") + "\n";
    if (!/\\\s*$/.test(line)) break;
  }
  void end;
  const clean = body.replace(/\/\/[^\n]*/g, " ");
  const items = [];
  const re = /(?:^|[\s(])V\((\w+),\s*"([^"]*)"/g;
  let mm;
  while ((mm = re.exec(clean)) !== null) items.push({ name: mm[1], msg: mm[2] });
  if (!items.length) throw new Error(`${listName} 空表`);
  return items;
}
const builtin = parseVList(reasonH, "DEOPTIMIZE_IN_BUILTIN_REASON_LIST");
const eagerRest = parseVList(reasonH, "DEOPTIMIZE_REASON_LIST");
// 枚举序: builtin 子表在前 (原版 DEOPTIMIZE_REASON_LIST 首行引入子表);
// eagerRest 首项为子表引入行误捕? 否 — 子表含 OutOfBounds 1 项, 主表 67 项, 合计 68
const eager = [...builtin, ...eagerRest];
const lazy = parseVList(reasonH, "LAZY_DEOPTIMIZE_REASON_LIST");
if (eager.length !== 68) throw new Error(`eager 原因应 68, 实得 ${eager.length}`);
if (lazy.length !== 17) throw new Error(`lazy 原因应 17, 实得 ${lazy.length}`);
if (eager[0].name !== "OutOfBounds" || eager[eager.length - 1].name !== "WrongValue") {
  throw new Error("eager 序漂移");
}

// 2. cc 侧三张字符串表 (operator<< / ToString×2)
for (const k of ["operator<<", "DeoptimizeReasonToString", "kDeoptimizeReasonStrings"]) {
  if (!reasonCc.includes(k)) throw new Error(`deoptimize-reason.cc 缺 ${k}`);
}

// 3. DeoptimizeKind 枚举在 src/common/globals.h (跨文件引用核对)
const kindM = globalsH.match(/enum class DeoptimizeKind : uint8_t \{([^}]+)\};/);
if (!kindM) throw new Error("globals.h 未找到 DeoptimizeKind");
const kinds = kindM[1].split(",").map((s) => s.trim()).filter((s) => s && !s.startsWith("//")).map((s) => s.split("=")[0].trim());
if (kinds.join() !== "kEager,kLazyAfterFastCall,kLazy") throw new Error(`DeoptimizeKind 漂移: ${kinds.join()}`);

// 4. deoptimizer.h 方法面
const deoptFns = [...hSrc.matchAll(/\n  (?:static )?(?:void|Address|int|bool|DirectHandle|Tagged)[\w:<>,*& ]*?(\w+)\(/g)].map((m) => m[1]);
if (deoptFns.length < 10) throw new Error(`Deoptimizer 方法应 ≥10, 实得 ${deoptFns.length}`);

// 5. cc 定义头对数 (>2k 行, 规则 10 只取定义头)
function collectDefs(src, prefix) {
  const out = [];
  const ls = src.split("\n");
  for (let i = 0; i < ls.length; i++) {
    const l = ls[i];
    if (!new RegExp(`^[\\w:<>,*&\\s]*\\b${prefix}::~?\\w+\\(`).test(l) || /^\s*\/\//.test(l)) continue;
    let sig = l;
    let j = i;
    while (!/\{\}?\s*$|;\s*$/.test(sig) && j + 1 < ls.length) sig += " " + ls[++j].trim();
    sig = sig.replace(/\{\s*$/, "").replace(/\{\}\s*$/, "").trim();
    if (sig.endsWith(";")) continue;
    out.push(sig);
  }
  return out;
}
const ccDefs = collectDefs(ccSrc, "Deoptimizer");
if (ccDefs.length < 20) throw new Error(`deoptimizer.cc 定义头应 ≥20, 实得 ${ccDefs.length}`);

// 6. frame-description 关键结构
for (const k of ["FrameDescription", "RegisterValues"]) {
  if (!fdH.includes(k)) throw new Error(`frame-description.h 缺 ${k}`);
}

function screaming(n) {
  return n.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
}

let rust = banner(
  "src/deoptimizer/deoptimize-reason.cc.js",
  "src/deoptimizer/deoptimize-reason.{h,cc} + deoptimizer.{h,cc} + frame-description.h",
  "反优化原因表与主体骨架",
);

rust += `
/// 反优化原因 (原版 enum class DeoptimizeReason, DEOPTIMIZE_REASON_LIST 1:1 展开, ${eager.length} 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeoptimizeReason {
${eager.map((r, i) => `  /// "${r.msg}"\n  ${r.name} = ${i},`).join("\n")}
}

impl DeoptimizeReason {
  /// 总数 (对数守卫)
  pub const COUNT: usize = ${eager.length};

  /// 消息串 (原版 DeoptimizeReasonToString → kDeoptimizeReasonStrings)
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
${eager.map((r) => `      Self::${r.name} => "${r.msg}",`).join("\n")}
    }
  }

  /// 枚举名 (原版 operator<< 输出 #Name)
  #[must_use]
  pub const fn name(self) -> &'static str {
    match self {
${eager.map((r) => `      Self::${r.name} => "${r.name}",`).join("\n")}
    }
  }
}

/// 惰性反优化原因 (原版 enum class LazyDeoptimizeReason, ${lazy.length} 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LazyDeoptimizeReason {
${lazy.map((r, i) => `  /// "${r.msg}"\n  ${r.name} = ${i},`).join("\n")}
}

impl LazyDeoptimizeReason {
  pub const COUNT: usize = ${lazy.length};

  /// 消息串 (原版重载 DeoptimizeReasonToString)
  #[must_use]
  pub const fn message(self) -> &'static str {
    match self {
${lazy.map((r) => `      Self::${r.name} => "${r.msg}",`).join("\n")}
    }
  }
}

/// 反优化种类 (原版 globals.h enum class DeoptimizeKind : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeoptimizeKind {
  Eager = 0,
  LazyAfterFastCall = 1,
  Lazy = 2,
}

/// 反优化种类字符串表 (定义于 src/common/globals.h, 此处跨文件登记, ${kinds.length} 项)
pub const DEOPTIMIZE_KINDS: [&str; ${kinds.length}] = [${kinds.map((k) => `"${k}"`).join(", ")}];

/// Deoptimizer 方法面 (原版 deoptimizer.h, ${deoptFns.length} 项, 顺序登记)
pub const DEOPTIMIZER_METHODS: [&str; ${deoptFns.length}] = [${deoptFns.map((f) => `"${f}"`).join(", ")}];

/// FrameDescription / TranslatedFrame 结构锚点 (原版 frame-description.h)
pub const FRAME_DESCRIPTION_ANCHORS: [&str; 2] = ["FrameDescription", "RegisterValues"];

// deoptimizer.cc 定义头清单 (对数基准, ${ccDefs.length} 项):
${ccDefs.map((s) => "// " + s).join("\n")}
`;

const p = writeSrc("v8-deoptimizer", "deoptimizer_generated.rs", rust);
console.log(`written ${p}; eager=${eager.length}, lazy=${lazy.length}, kind=${kinds.length}, 方法=${deoptFns.length}, cc定义=${ccDefs.length}`);
