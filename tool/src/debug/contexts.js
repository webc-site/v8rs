#!/usr/bin/env bun
// tool/src/debug/contexts.js — 覆盖 v8/src/objects/contexts.{cc,tq}
// 组→文件映射 (本脚本管辖): 席位表 2 文件 (.h/-inl.h 归既有 tool/src/objects/contexts.h.js)
// 产物: v8-debug/src/contexts.rs
// 机械提取: ContextCell 布局 + Context::State 五态 (kConst..kDetached, 交叉核对 contexts.h)
//   + ContextMode 枚举 + 9 个 Context 子类型 + .cc NativeContext 偏移 static_assert 锚

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/contexts.tq");
const cc = readV8("src/objects/contexts.cc");
const h = readV8("src/objects/contexts.h");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[contexts.js] ${m}`); };

// ── 1. tq ContextCell 布局 (5 字段, 1 条件 padding) ───────────────────────
const cellStart = tq.indexOf("extern class ContextCell extends HeapObject {");
const cell = tq.slice(cellStart, tq.indexOf("\n}", cellStart));
const cellFields = [...cell.matchAll(/^  (@if\(TAGGED_SIZE_8_BYTES\) )?(\w+): ([^;]+);$/gm)]
  .map((m) => [m[2], m[3].trim(), Boolean(m[1])]);
must(cellFields.length === 5, `ContextCell 字段数 ${cellFields.length} != 5`);
const expectCell = [
  ["tagged_value", "JSAny", false],
  ["dependent_code", "DependentCode", false],
  ["state", "int32", false],
  ["optional_padding", "uint32", true],
  ["double_value", "float64", false],
];
for (let i = 0; i < 5; i++) {
  must(cellFields[i][0] === expectCell[i][0] && cellFields[i][1] === expectCell[i][1] && cellFields[i][2] === expectCell[i][2],
    `ContextCell 字段 ${i} 应为 ${expectCell[i]}, 实得 ${cellFields[i]}`);
}

// ── 2. ContextCell::State 五态 (tq generates 常量 ↔ contexts.h enum State) ─
const stateEnum = h.match(/class ContextCell[\s\S]*?enum State : int32_t\s*\{([^}]*)\}/);
must(stateEnum, "contexts.h 缺 ContextCell::State 枚举");
const states = [...stateEnum[1].matchAll(/(k\w+)\s*=\s*(\d+)/g)].map((m) => [m[1], +m[2]]);
must(states.length === 5, `State 条数 ${states.length} != 5`);
states.forEach(([n, v], i) => must(n === ["kConst", "kSmi", "kInt32", "kFloat64", "kDetached"][i] && v === i, `State 第 ${i} 条漂移: ${n}=${v}`));
// tq 侧 generates 声明 5 条全部存在 (k 前缀与 h 枚举条目 1:1)
for (const [n] of states) {
  must(tq.includes(`generates 'ContextCell::${n}'`), `tq 缺 generates 'ContextCell::${n}'`);
}

// ── 3. ContextMode 枚举 (tq extern enum) ─────────────────────────────────
const mode = tq.match(/extern enum ContextMode extends uint32 \{([^}]*)\}/);
must(mode, "tq 缺 ContextMode");
const modeEntries = [...mode[1].matchAll(/(k\w+)/g)].map((m) => m[1]);
must(modeEntries.join(",") === "kNoContextCells,kHasContextCells", `ContextMode 条目漂移: ${modeEntries.join(",")}`);

// ── 4. Context 子类型 extern class (9 个, 全部 generates 'Context'; 允许跨行) ──
const subClasses = [...tq.matchAll(/^extern class (\w+) extends Context\s*\n?\s*generates 'Context';$/gm)].map((m) => m[1]);
must(subClasses.length === 9, `Context 子类型数 ${subClasses.length} != 9`);
must(subClasses.join(",") === "AwaitContext,BlockContext,CatchContext,DebugEvaluateContext,EvalContext,ModuleContext,ScriptContext,WithContext,FunctionContext",
  `Context 子类型序漂移: ${subClasses.join(",")}`);

// ── 5. Context/ScriptContextTable/NativeContext 基础锚 ──────────────────
must(tq.includes("extern class Context extends HeapObject {"), "tq 缺 Context 基类");
must(tq.includes("const length: Smi;") && tq.includes("elements[length]: Object;"), "tq Context 布局锚漂移");
must(tq.includes("extern class ScriptContextTable extends HeapObject {"), "tq 缺 ScriptContextTable");
must(tq.includes("extern class NativeContext extends Context;"), "tq 缺 NativeContext");
must(tq.includes("const kInitialContextSlotValue: Smi = 0;"), "tq 缺 kInitialContextSlotValue=0");

// ── 6. .cc static_assert 锚 (NativeContext 偏移链 6 处 + 槽位下限 2 处) ──
must(cc.includes("static_assert(Context::MIN_CONTEXT_SLOTS == 2);"), "cc 缺 MIN_CONTEXT_SLOTS 断言");
must(cc.includes("static_assert(Context::MIN_CONTEXT_EXTENDED_SLOTS == 3);"), "cc 缺 MIN_CONTEXT_EXTENDED_SLOTS 断言");
const saCount = (cc.match(/^static_assert\(/gm) || []).length;
must(saCount === 8, `cc static_assert 数 ${saCount} != 8`);
for (const key of ["kScopeInfoOffset", "kPreviousOffset", "kExtensionOffset", "kStartOfWeakFieldsOffset", "kMicrotaskQueueOffset", "kSize"]) {
  must(new RegExp(`NativeContext::${key} ==`).test(cc), `cc 缺 ${key} static_assert`);
}
// .cc ContextCell 状态机锚
must(cc.includes("cell->set_state(ContextCell::kInt32);"), "cc 缺 kInt32 迁移锚");
must(cc.includes("cell->set_state(ContextCell::kFloat64);"), "cc 缺 kFloat64 迁移锚");
must(cc.includes("DependentCode::kContextCellChangedGroup"), "cc 缺 kContextCellChangedGroup 去优化锚");

const pascal = (s) => s.replace(/^k/, "").replace(/(^|_)(\w)/g, (_, a, b) => b.toUpperCase());

let rust = banner("src/debug/contexts.js", "src/objects/contexts.{cc,tq}",
  "席位表差集: ContextCell 布局与五态 + ContextMode + Context 子类型 (.h 槽位归既有 contexts.h.js)");
rust += `//! Context 差集 (debug 域): ContextCell 上下文槽盒 (context cells proposal)。
//!
//! ContextCell 承载可优化槽位 (Smi/Int32/Float64 三态迁移 + 依赖代码去优化);
//! Context 槽位下标/偏移常量归既有 contexts.h.js, 本文件只登记 .tq/.cc 差集。

// ── ContextCell::State (原版 enum State : int32_t; tq generates 5 常量) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum ContextCellState {
`;
states.forEach(([n, v]) => { rust += `  /// ${n}\n  ${pascal(n)} = ${v},\n`; });
rust += `}

impl ContextCellState {
  /// 状态总数
  pub const COUNT: usize = ${states.length};
}

// ── ContextMode (tq extern enum ContextMode extends uint32) ──
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ContextMode {
  /// kNoContextCells
  NoContextCells = 0,
  /// kHasContextCells
  HasContextCells = 1,
}

// ── ContextCell 布局 (tq extern class, 源码序; padding 仅 8B tagged 构建) ──
pub const CONTEXT_CELL_FIELDS: [(&str, &str, bool); ${cellFields.length}] = [
`;
for (const f of cellFields) rust += `  ("${f[0]}", "${f[1]}", ${f[2]}),\n`;
rust += `];

// ── Context 子类型 (tq generates 'Context', 布局同 Context) ──
pub const CONTEXT_SUBCLASSES: [&str; ${subClasses.length}] = [
`;
for (const c of subClasses) rust += `  "${c}",\n`;
rust += `];

/// 新建槽位初始值 (原版 kInitialContextSlotValue)。
pub const K_INITIAL_CONTEXT_SLOT_VALUE: i32 = 0;

// ── .cc NativeContext 偏移 static_assert 面 (8 处, 语义锚) ──
//   MIN_CONTEXT_SLOTS == 2; MIN_CONTEXT_EXTENDED_SLOTS == 3;
//   kScopeInfoOffset/kPreviousOffset/kExtensionOffset == OffsetOfElementAt(对应下标);
//   kStartOfWeakFieldsOffset == OffsetOfElementAt(FIRST_WEAK_SLOT);
//   kMicrotaskQueueOffset == Context::SizeFor(NATIVE_CONTEXT_SLOTS);
//   kSize == SizeFor(NATIVE_CONTEXT_SLOTS) + kMicrotaskQueueSlotSize;
//   (数值依赖 NATIVE_CONTEXT_FIELDS 展开, 由 Developer 依宏展开落值)

// ── .cc ContextCell 状态机签名骨架 ──
/// 原版 TransitionContextCellToUntagged (contexts.cc): HeapNumber → Int32/Float64 迁移。
pub fn transition_context_cell_to_untagged() {
  todo!("就地修订: contexts.cc TransitionContextCellToUntagged")
}

/// 原版 NotifyContextCellStateWillChange (contexts.cc): 通知依赖代码去优化。
pub fn notify_context_cell_state_will_change() {
  todo!("就地修订: contexts.cc NotifyContextCellStateWillChange")
}

/// 编译期字符串相等 (str 的 const PartialEq 尚未稳定, 手写逐字节比较)
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

// ── 提取不变式断言 ──
const _: () = {
  assert!(ContextCellState::COUNT == 5);
  assert!(ContextCellState::Const as i32 == 0);
  assert!(ContextCellState::Detached as i32 == 4);
  assert!(CONTEXT_CELL_FIELDS.len() == 5);
  assert!(str_eq(CONTEXT_CELL_FIELDS[3].0, "optional_padding"));
  assert!(str_eq(CONTEXT_CELL_FIELDS[4].1, "float64"));
  assert!(CONTEXT_SUBCLASSES.len() == 9);
  assert!(K_INITIAL_CONTEXT_SLOT_VALUE == 0);
};
`;

const p = writeSrc("v8-debug", "contexts.rs", rust);
console.log(`contexts.js: ${p} — State ${states.length} 态 + ContextMode 2 + 布局 ${cellFields.length} 字段 + 子类型 ${subClasses.length} + static_assert ${saCount}, 断言 ${checks} 项全过`);
