#!/usr/bin/env bun
// tool/src/objects/contexts.tq.js — 覆盖 v8/src/objects/contexts.{tq,cc 内 X-macro 源头 contexts.h}
// → v8-debug/src/context_slot.rs
// 机械提取: Context::Field 全量枚举 (NATIVE_CONTEXT_FIELDS X-macro 展开, 大头)
//   + ContextCell::State + ContextLookupFlags + Context/ScriptContextTable/ContextCell 布局
//   + contexts.tq Torque 布局类字段序与子类清单。
// 对数 oracle: clang -E (probe.cc 展开 NATIVE_CONTEXT_FIELDS; 缺失第三方头以空 stub 绕过预处理,
//   stub 集固定 7 个, 超限即 throw)。双配置: 默认 (V8_TEMPORAL_SUPPORT 关) 与开启 (+11 条登记不产常量)。

import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const hSrc = readV8("src/objects/contexts.h");
const tqSrc = readV8("src/objects/contexts.tq");

let checks = 0;
function must(cond, msg) {
  checks++;
  if (!cond) throw new Error(`[contexts.tq.js] 对数失败: ${msg}`);
}

// ── 1. 自解析 NATIVE_CONTEXT_FIELDS 宏体 (括号深度扫描, 不用贪婪正则) ──────
function parseXMacro(src) {
  const start = src.indexOf("#define NATIVE_CONTEXT_FIELDS(V)");
  must(start !== -1, "宏定义起点未找到");
  const end = src.indexOf("NATIVE_CONTEXT_FIELDS_TEMPORAL(V)", start);
  must(end !== -1, "宏体终点 (TEMPORAL 收尾) 未找到");
  let body = src.slice(start, end);
  // 去 // 注释 (宏体内无块注释), 合并续行符 (V(...) 三元组可跨行)
  body = body.replace(/\/\/[^\n]*/g, "").replace(/\\\n/g, " ");
  const items = [];
  for (let i = 0; i < body.length - 1; i++) {
    if (body[i] !== "V" || body[i + 1] !== "(") continue;
    let depth = 0;
    let j = i + 1;
    for (; j < body.length; j++) {
      if (body[j] === "(") depth++;
      else if (body[j] === ")") {
        depth--;
        if (depth === 0) break;
      }
    }
    must(depth === 0, `V(...) 括号不平衡 @${i}`);
    const inner = body.slice(i + 2, j);
    // 顶层逗号切 3 段: INDEX, TYPE(可含 <,> 嵌套逗号), NAME
    const parts = [];
    let d = 0;
    let cur = "";
    for (const ch of inner) {
      if (ch === "<" || ch === "(") d++;
      else if (ch === ">" || ch === ")") d--;
      if (ch === "," && d === 0) {
        parts.push(cur.trim());
        cur = "";
      } else cur += ch;
    }
    parts.push(cur.trim());
    must(parts.length === 3, `V(...) 段数 ${parts.length} != 3: ${inner.slice(0, 60)}`);
    items.push({ index: parts[0], type: parts[1], name: parts[2] });
    i = j;
  }
  return items;
}
const selfParsed = parseXMacro(hSrc);

// ── 2. clang -E oracle ────────────────────────────────────────────────────
const PROBE = `#include "src/objects/contexts.h"
#define PROBE_NAME(index, type, name) #index,
static const char* kProbeNames[] = {NATIVE_CONTEXT_FIELDS(PROBE_NAME)};
`;
function clangOracle(defines) {
  const dir = mkdtempSync(join(tmpdir(), "r3o7-ctx-"));
  try {
    writeFileSync(join(dir, "probe.cc"), PROBE);
    const stubDir = join(dir, "stub");
    const dFlags = defines.flatMap((d) => ["-D", d]);
    const args = [
      "-E", "-std=c++20", ...dFlags,
      "-I", stubDir, "-I", join(root, "v8"), "-I", join(root, "v8", "include"),
      join(dir, "probe.cc"),
    ];
    for (let round = 0; round <= 32; round++) {
      const r = spawnSync("clang", args, { encoding: "utf8", maxBuffer: 1 << 26 });
      if (r.status === 0) return r.stdout;
      const m = String(r.stderr ?? "").match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E 失败 (status=${r.status}):\n${r.stderr}`);
      if (round === 32) throw new Error(`stub 轮次超限, 仍缺 ${m[1]}`);
      const p = join(stubDir, m[1]);
      mkdirSync(dirname(p), { recursive: true });
      writeFileSync(p, "// preprocess-only stub\n");
    }
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
function extractNames(pp) {
  const line = pp.split("\n").find((l) => l.includes("kProbeNames[]"));
  must(!!line, "预处理产物缺 kProbeNames");
  return [...line.matchAll(/"([A-Z0-9_]+)"/g)].map((m) => m[1]);
}
const oracleDefault = extractNames(clangOracle([]));
const oracleTemporal = extractNames(clangOracle(["V8_TEMPORAL_SUPPORT"]));

// ── 3. 双侧对数 ───────────────────────────────────────────────────────────
must(oracleDefault.length === 294, `默认配置字段数 ${oracleDefault.length} != 294`);
must(oracleTemporal.length === 304, `TEMPORAL 配置字段数 ${oracleTemporal.length} != 304`);
must(
  JSON.stringify(selfParsed.map((x) => x.index)) === JSON.stringify(oracleDefault),
  "自解析名单与 clang -E 展开不一致",
);
must(oracleDefault[0] === "GLOBAL_PROXY_INDEX", "首项应为 GLOBAL_PROXY_INDEX");
must(oracleDefault.at(-1) === "SHARED_SPACE_JS_OBJECT_HAS_INSTANCE_INDEX", "末项失配");
const temporalExtra = oracleTemporal.filter((n) => !oracleDefault.includes(n));
must(temporalExtra.length === 10 && temporalExtra.every((n) => n.includes("TEMPORAL")),
  `TEMPORAL 追加 10 条失配: ${temporalExtra.join(",")}`);

// 枚举值 (C++ 顺序枚举规则): SCOPE_INFO=0 PREVIOUS=1 EXTENSION=2, native 从 3 起
const N = oracleDefault.length; // 294
const FIRST_NATIVE = 3;
const NEXT_CONTEXT_LINK = FIRST_NATIVE + N; // 297
const NATIVE_CONTEXT_SLOTS = NEXT_CONTEXT_LINK + 1; // 298
const val = new Map(oracleDefault.map((n, i) => [n, FIRST_NATIVE + i]));
const named = (n) => {
  const v = val.get(n);
  must(v !== undefined, `名单缺 ${n}`);
  return v;
};
// .h 显式别名/常量锚
for (const anchor of [
  "MIN_CONTEXT_SLOTS = EXTENSION_INDEX",
  "MIN_CONTEXT_EXTENDED_SLOTS = EXTENSION_INDEX + 1",
  "FIRST_WEAK_SLOT = NEXT_CONTEXT_LINK",
  "FIRST_JS_ARRAY_MAP_SLOT = JS_ARRAY_PACKED_SMI_ELEMENTS_MAP_INDEX",
  "THROWN_OBJECT_INDEX = MIN_CONTEXT_SLOTS",
  "WRAPPED_CONTEXT_INDEX = MIN_CONTEXT_EXTENDED_SLOTS",
]) must(hSrc.includes(anchor), `contexts.h 缺锚 ${anchor}`);
const FIRST_FUNCTION_MAP = named("SLOPPY_FUNCTION_MAP_INDEX");
const LAST_FUNCTION_MAP = named("CLASS_FUNCTION_MAP_INDEX");
const FIRST_JS_ARRAY_MAP_SLOT = named("JS_ARRAY_PACKED_SMI_ELEMENTS_MAP_INDEX");
const FIRST_TYPED_ARRAY_FUN = named("UINT8_ARRAY_FUN_INDEX");
const FIRST_RAB_GSAB_MAP = named("RAB_GSAB_UINT8_ARRAY_MAP_INDEX");

// ── 4. ContextCell::State / ContextLookupFlags (contexts.h 显式值) ────────
const stateRe = hSrc.match(/enum State : int32_t \{([\s\S]*?)\};/);
must(!!stateRe, "ContextCell::State 未找到");
const states = [...stateRe[1].matchAll(/k(\w+) = (\d+)/g)].map((m) => [m[1], Number(m[2])]);
must(states.length === 5, `State 条数 ${states.length} != 5`);
states.forEach(([, v], i) => must(v === i, `State 值 ${v} != ${i}`));
// contexts.tq generates 声明 5 条对数
for (const s of states) must(tqSrc.includes(`'ContextCell::k${s[0]}'`), `.tq 缺 generates ContextCell::k${s[0]}`);

must(hSrc.includes("FOLLOW_CONTEXT_CHAIN = 1 << 0"), "FOLLOW_CONTEXT_CHAIN 锚失配");
must(hSrc.includes("FOLLOW_PROTOTYPE_CHAIN = 1 << 1"), "FOLLOW_PROTOTYPE_CHAIN 锚失配");
must(hSrc.includes("DONT_FOLLOW_CHAINS = 0"), "DONT_FOLLOW_CHAINS 锚失配");
must(hSrc.includes("FOLLOW_CHAINS = FOLLOW_CONTEXT_CHAIN | FOLLOW_PROTOTYPE_CHAIN"), "FOLLOW_CHAINS 锚失配");

// ── 5. contexts.tq 布局类字段序 + 子类清单 + ContextSlot 子序列对数 ────────
function tqClassFields(name) {
  const re = new RegExp(`extern class ${name} extends [\\w\\s{]*\\{([\\s\\S]*?)\\n\\}`, "m");
  const m = tqSrc.match(re);
  must(!!m, `.tq 缺 extern class ${name}`);
  return [...m[1].matchAll(/^\s*(?:@if\([^)]*\)\s*)?(?:const\s+)?(\w+)(?:\[[^\]]*\])?:/gm)].map((x) => x[1]);
}
const ctxFields = tqClassFields("Context");
const sctFields = tqClassFields("ScriptContextTable");
const ccFields = tqClassFields("ContextCell");
must(JSON.stringify(ctxFields) === JSON.stringify(["length", "elements"]), `Context .tq 字段序失配: ${ctxFields}`);
must(
  JSON.stringify(sctFields) === JSON.stringify(["capacity", "length", "names_to_context_index", "objects"]),
  `ScriptContextTable .tq 字段序失配: ${sctFields}`,
);
must(
  JSON.stringify(ccFields) === JSON.stringify(["tagged_value", "dependent_code", "state", "optional_padding", "double_value"]),
  `ContextCell .tq 字段序失配: ${ccFields}`,
);
const subclasses = [...tqSrc.matchAll(/^extern class (\w+) extends Context\s+generates 'Context';/gm)].map((m) => m[1]);
must(subclasses.length === 9, `Context 子类数 ${subclasses.length} != 9`);
// .tq ContextSlot 为 Torque 侧引用子集 (顺序自成一派, 值来自 constexpr 'Context::Field');
// 对数规则: 每条必须存在于 .h Field 全集
const tqSlotBlock = tqSrc.match(/extern enum ContextSlot extends intptr[\s\S]*?\n\}/)?.[0] ?? "";
must(!!tqSlotBlock, ".tq 缺 extern enum ContextSlot");
const tqSlots = [...tqSlotBlock.matchAll(/^\s{2}([A-Z0-9_]+):/gm)].map((m) => m[1]);
must(tqSlots.length > 60, `.tq ContextSlot 条数异常: ${tqSlots.length}`);
const fieldUniverse = new Set(["SCOPE_INFO_INDEX", "PREVIOUS_INDEX", ...oracleDefault]);
for (const s of tqSlots) must(fieldUniverse.has(s), `.tq ContextSlot 条目 ${s} 不在 .h Field 全集`);

// ── 6. 生成 Rust ──────────────────────────────────────────────────────────
const pascal = (s) => s.toLowerCase().replace(/(^|_)(\w)/g, (_, __, c) => c.toUpperCase());

let out = banner(
  "src/objects/contexts.tq.js",
  "src/objects/contexts.{tq,h}",
  `Context::Field X-macro 全量展开 (${N} native 字段, V8_TEMPORAL_SUPPORT 关闭分支; 开启时追加 10 条 TEMPORAL_* 登记于文档)`,
);
out += `//! Context 槽位体系: \`Context::Field\` 枚举 + NativeContext 字段名表 + 查找标志
//! + Context/ScriptContextTable/ContextCell 布局公式, 压缩指针 K_TAGGED_SIZE=4 默认基准, 双态。
//!
//! 差集登记: ContextCellState/ContextMode/ContextCell 字段表/Context 子类清单归
//! tool/src/debug/contexts.js → v8-debug/src/contexts.rs; 本文件独占 X-macro 展开,
//! 对方 static_assert 锚所需 FIRST_WEAK_SLOT 与 size_for(NATIVE_CONTEXT_SLOTS) 数值由此供给。
//!
//! TEMPORAL 追加字段默认关闭共 10 条: ${temporalExtra.join(", ")}。

use v8_common::{K_HEAP_OBJECT_TAG, K_TAGGED_SIZE, TAGGED_SIZE_8_BYTES};

/// Context 通用槽位 (原版 Context::Field 前三项 + 尾部合计项, 全量镜像)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ContextSlot {
`;
out += `  /// SCOPE_INFO_INDEX
  ScopeInfoIndex = 0,
  /// PREVIOUS_INDEX
  PreviousIndex = 1,
  /// EXTENSION_INDEX (仅 ScopeInfo::HasContextExtensionSlot 为真时存在)
  ExtensionIndex = 2,
`;
oracleDefault.forEach((n, i) => {
  const it = selfParsed[i];
  out += `  /// ${n} (${it.type}) — accessor ${it.name}
  ${pascal(n)} = ${FIRST_NATIVE + i},
`;
});
out += `  /// NEXT_CONTEXT_LINK (弱引用区起点)
  NextContextLink = ${NEXT_CONTEXT_LINK},
  /// NATIVE_CONTEXT_SLOTS (总槽数, 不含 microtask_queue 尾部)
  NativeContextSlots = ${NATIVE_CONTEXT_SLOTS},
}

impl ContextSlot {
  /// native 字段数 (不含 3 个通用槽与尾部 2 项)
  pub const NATIVE_FIELD_COUNT: usize = ${N};
  /// 枚举变体总数
  pub const COUNT: usize = ${NATIVE_CONTEXT_SLOTS + 1};

  /// 数值反查 (原版隐式整型转换)
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u16) -> Option<Self> {
    match v {
`;
out += `      0 => Some(Self::ScopeInfoIndex),
      1 => Some(Self::PreviousIndex),
      2 => Some(Self::ExtensionIndex),
`;
oracleDefault.forEach((n, i) => {
  out += `      ${FIRST_NATIVE + i} => Some(Self::${pascal(n)}),\n`;
});
out += `      ${NEXT_CONTEXT_LINK} => Some(Self::NextContextLink),
      ${NATIVE_CONTEXT_SLOTS} => Some(Self::NativeContextSlots),
      _ => None,
    }
  }
}

/// native 字段名表 (原版顺序, 索引 i ↔ ContextSlot 值 ${FIRST_NATIVE}+i)。
pub const NATIVE_CONTEXT_NAMES: [&str; ${N}] = [
${oracleDefault.map((n) => `  "${n}",`).join("\n")}
];

/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
#[inline]
#[must_use]
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

/// native 槽位名反查 (值域 ${FIRST_NATIVE}..${NEXT_CONTEXT_LINK})。
#[inline]
#[must_use]
pub fn native_field_name(index: u16) -> Option<&'static str> {
  NATIVE_CONTEXT_NAMES.get(usize::from(index).wrapping_sub(${FIRST_NATIVE})).copied()
}

// —— 显式别名与区间常量 (原版 enum 内赋值项) ——
/// 最少上下文槽数 (= EXTENSION_INDEX)
pub const MIN_CONTEXT_SLOTS: u16 = 2;
/// 含扩展槽最少槽数
pub const MIN_CONTEXT_EXTENDED_SLOTS: u16 = 3;
/// catch 上下文抛出值槽
pub const THROWN_OBJECT_INDEX: u16 = 2;
/// debug evaluate 上下文包装槽
pub const WRAPPED_CONTEXT_INDEX: u16 = 3;
/// 弱引用区首槽 (= NEXT_CONTEXT_LINK)
pub const FIRST_WEAK_SLOT: u16 = ${NEXT_CONTEXT_LINK};
/// JSArray 六种 ElementsKind 地图区首槽
pub const FIRST_JS_ARRAY_MAP_SLOT: u16 = ${FIRST_JS_ARRAY_MAP_SLOT};
/// Builtin::kFastNewClosure 函数地图区首槽
pub const FIRST_FUNCTION_MAP_INDEX: u16 = ${FIRST_FUNCTION_MAP};
/// 函数地图区末槽
pub const LAST_FUNCTION_MAP_INDEX: u16 = ${LAST_FUNCTION_MAP};
/// TypedArray 构造器区首槽 (顺序敏感)
pub const FIRST_FIXED_TYPED_ARRAY_FUN_INDEX: u16 = ${FIRST_TYPED_ARRAY_FUN};
/// RAB/GSAB TypedArray 地图区首槽
pub const FIRST_RAB_GSAB_TYPED_ARRAY_MAP_INDEX: u16 = ${FIRST_RAB_GSAB_MAP};

/// 变量查找标志 (原版 enum ContextLookupFlags, 位标志)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ContextLookupFlags(pub u8);

impl ContextLookupFlags {
  /// DONT_FOLLOW_CHAINS: 不追链
  pub const DONT_FOLLOW_CHAINS: Self = Self(0);
  /// FOLLOW_CONTEXT_CHAIN: 追上下文链
  pub const FOLLOW_CONTEXT_CHAIN: Self = Self(1);
  /// FOLLOW_PROTOTYPE_CHAIN: 追原型链
  pub const FOLLOW_PROTOTYPE_CHAIN: Self = Self(2);
  /// FOLLOW_CHAINS: 双链皆追
  pub const FOLLOW_CHAINS: Self = Self(3);

  /// 含判定
  #[inline]
  #[must_use]
  pub const fn contains(self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }

  /// 合并
  #[inline]
  #[must_use]
  pub const fn union(self, other: Self) -> Self {
    Self(self.0 | other.0)
  }
}

// —— Context 布局 (原版 kElementsOffset..kExtendedHeaderSize 公式链) ——
/// elements 首槽偏移 (= map + length 头, 原版 OFFSET_OF_DATA_START(Context))
pub const K_ELEMENTS_OFFSET: usize = 2 * K_TAGGED_SIZE;
/// kHeaderSize (同 kElementsOffset)
pub const K_HEADER_SIZE: usize = K_ELEMENTS_OFFSET;
/// scope_info 槽偏移
pub const K_SCOPE_INFO_OFFSET: usize = K_ELEMENTS_OFFSET;
/// previous 槽偏移
pub const K_PREVIOUS_OFFSET: usize = K_SCOPE_INFO_OFFSET + K_TAGGED_SIZE;
/// 过渡头尺寸 (MIN_CONTEXT_SLOTS 尚未移除前的头大小)
pub const K_TODO_HEADER_SIZE: usize = K_PREVIOUS_OFFSET + K_TAGGED_SIZE;
/// extension 槽偏移 (存在时为头后首槽)
pub const K_EXTENSION_OFFSET: usize = K_TODO_HEADER_SIZE;
/// extension 槽大小
pub const K_EXTENSION_SIZE: usize =
  (MIN_CONTEXT_EXTENDED_SLOTS as usize - MIN_CONTEXT_SLOTS as usize) * K_TAGGED_SIZE;
/// 扩展头尺寸
pub const K_EXTENDED_HEADER_SIZE: usize = K_TODO_HEADER_SIZE + K_EXTENSION_SIZE;

/// 对象总大小 (原版 Context::SizeFor)
#[inline]
#[must_use]
pub const fn size_for(length: usize) -> usize {
  K_ELEMENTS_OFFSET + length * K_TAGGED_SIZE
}

/// 槽 i 相对对象起址偏移 (原版 Context::OffsetOfElementAt)
#[inline]
#[must_use]
pub const fn offset_of_element_at(index: usize) -> usize {
  size_for(index)
}

/// 槽 i 相对 tagged 指针偏移 (原版 Context::SlotOffset)
#[inline]
#[must_use]
pub const fn slot_offset(index: usize) -> usize {
  offset_of_element_at(index) - K_HEAP_OBJECT_TAG
}

// —— ScriptContextTable 布局 (原版显式公式) ——
/// capacity 偏移 (= sizeof(HeapObject))
pub const K_SCT_CAPACITY_OFFSET: usize = K_TAGGED_SIZE;
/// length 偏移 (capacity 为 uint32)
pub const K_SCT_LENGTH_OFFSET: usize = K_SCT_CAPACITY_OFFSET + 4;
/// 头大小 (names_to_context_index 之后, objects 变长区起点)
pub const K_SCT_HEADER_SIZE: usize =
  K_SCT_LENGTH_OFFSET + 4 + if TAGGED_SIZE_8_BYTES { K_TAGGED_SIZE } else { 4 };

// —— ContextCell 布局数值偏移 (字段表归 contexts.rs, 此处独占偏移公式; 成员序 tagged_value/dependent_code/state/[padding]/double_value) ——
/// state 偏移 (map + 2 个 tagged 槽之后)
pub const K_CC_STATE_OFFSET: usize = 3 * K_TAGGED_SIZE;
/// double_value 偏移 (仅 TAGGED_SIZE_8_BYTES 有 optional_padding)
pub const K_CC_DOUBLE_VALUE_OFFSET: usize = if TAGGED_SIZE_8_BYTES {
  4 * K_TAGGED_SIZE
} else {
  3 * K_TAGGED_SIZE + 4
};
/// ContextCell 总大小
pub const K_CC_SIZE: usize = K_CC_DOUBLE_VALUE_OFFSET + 8;

// —— contexts.tq 布局类登记 (ContextCell 字段表与 Context 子类清单归 contexts.rs, 此处登记变长尾数组字段序) ——
/// extern class Context 字段序 (elements 为变长尾数组)
pub const CONTEXT_TQ_FIELDS: &[&str] = &["length", "elements"];
/// extern class ScriptContextTable 字段序 (objects 为变长尾数组)
pub const SCT_TQ_FIELDS: &[&str] = &["capacity", "length", "names_to_context_index", "objects"];

// —— 提取不变式断言 ——
const _: () = {
  assert!(ContextSlot::NATIVE_FIELD_COUNT == ${N});
  assert!(ContextSlot::COUNT == ${NATIVE_CONTEXT_SLOTS + 1});
  assert!(ContextSlot::ExtensionIndex as u16 == 2);
  assert!(ContextSlot::NextContextLink as u16 == ${NEXT_CONTEXT_LINK});
  assert!(ContextSlot::NativeContextSlots as u16 == ${NATIVE_CONTEXT_SLOTS});
  assert!(FIRST_WEAK_SLOT == ContextSlot::NextContextLink as u16);
  assert!(FIRST_JS_ARRAY_MAP_SLOT == ContextSlot::${pascal("JS_ARRAY_PACKED_SMI_ELEMENTS_MAP_INDEX")} as u16);
  assert!(FIRST_FUNCTION_MAP_INDEX == ${FIRST_FUNCTION_MAP});
  assert!(LAST_FUNCTION_MAP_INDEX > FIRST_FUNCTION_MAP_INDEX);
  assert!(NATIVE_CONTEXT_NAMES.len() == ${N});
  assert!(str_eq(NATIVE_CONTEXT_NAMES[0], "GLOBAL_PROXY_INDEX"));
  assert!(str_eq(NATIVE_CONTEXT_NAMES[${N - 1}], "SHARED_SPACE_JS_OBJECT_HAS_INSTANCE_INDEX"));
  assert!(ContextLookupFlags::FOLLOW_CHAINS.contains(ContextLookupFlags::FOLLOW_CONTEXT_CHAIN));
  assert!(MIN_CONTEXT_SLOTS == 2 && MIN_CONTEXT_EXTENDED_SLOTS == 3);
  assert!(size_for(${NATIVE_CONTEXT_SLOTS}) == K_ELEMENTS_OFFSET + ${NATIVE_CONTEXT_SLOTS} * K_TAGGED_SIZE);
  if TAGGED_SIZE_8_BYTES {
    assert!(K_EXTENDED_HEADER_SIZE == 40);
    assert!(K_SCT_HEADER_SIZE == 24);
    assert!(K_CC_SIZE == 40);
  } else {
    assert!(K_EXTENDED_HEADER_SIZE == 20);
    assert!(K_SCT_HEADER_SIZE == 16);
    assert!(K_CC_SIZE == 24);
  }
};
`;

const p = writeSrc("v8-debug", "context_slot.rs", out);
console.log(`context_slot.rs: native 字段 ${N} (clang -E 双配置 ${oracleDefault.length}/${oracleTemporal.length} 对数), 检 ${checks} 项, written ${p}`);
