#!/usr/bin/env bun
// tool/src/objects/transitions.cc.js — 覆盖 v8/src/objects/transitions.cc (单文件族; .h 侧归 transitions.h.js)
// → v8-objects-maps/src/transitions_cc_generated.rs
// 机械提取: 全部 36 个定义 (TransitionsAccessor 25 + TransitionArray 10 + operator<< 1)
//   的 TransitionArray/TransitionsAccessor 签名骨架 (函数体留 todo! + /// v8: 行号锚)
// 对数 oracle: clang -E 预处理 v8/src/objects/transitions.cc (定义清单 1:1)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { readV8, writeSrc, banner, root } from "../../lib.js";

const cc = readV8("src/objects/transitions.cc");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── clang -E 对数基准 (原版侧 = 编译器预处理产物; 缺失头自动空 stub) ──────
const ppShim = join(tmpdir(), "v8rs-r3o4-pp-shim");
function clangE(rel) {
  const args = [
    "-E", "-P", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include",
    "-I", ppShim, "-DV8_INTL_SUPPORT=1", "-DV8_ENABLE_WEBASSEMBLY=1", "-",
  ];
  for (let round = 0; round < 48; round++) {
    try {
      return execFileSync("clang", args, {
        input: `#include "${rel}"\n`,
        encoding: "utf8",
        cwd: root,
        maxBuffer: 512 * 1024 * 1024,
      });
    } catch (e) {
      const err = String(e.stderr ?? "");
      const m = err.match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E ${rel} 失败: ${err.slice(0, 300)}`);
      const stub = join(ppShim, m[1]);
      mkdirSync(dirname(stub), { recursive: true });
      writeFileSync(stub, "#pragma once\n");
    }
  }
  throw new Error(`clang -E ${rel}: stub 轮数超限`);
}

// ── 产物 fmt 归一 (与 cargo +nightly fmt 同配置; 幂等) ────────────────────
function fmtNormalize(p) {
  const rustfmt = execFileSync("rustup", ["which", "--toolchain", "nightly", "rustfmt"],
    { encoding: "utf8", cwd: root }).trim();
  execFileSync(rustfmt, ["--edition", "2024", "--config-path", join(root, "rustfmt.toml"), p],
    { cwd: root });
}


// ── 定义清单扫描 (列首 "RET Class::method(" / "RET operator<<(") ──────────
function scanDefs(rawText) {
  const out = [];
  const lines = rawText.replace(/__attribute__\(\(.*?\)\)/g, "")
    .split("\n").map((l) => l.replace(/^\s+/, ""));
  for (let i = 0; i < lines.length; i++) {
    // 组: 1=RET 前缀, 2=类名, 3=方法名, 4=operator<<
    const m = lines[i].match(/^([A-Za-z_][\w:<>,&* ]*? )?(?:(TransitionsAccessor|TransitionArray)::(\w+)|(operator<<))\s*\(/);
    if (!m) continue;
    const klass = m[2] ?? (m[4] ? "operator" : "");
    const name = m[3] ?? m[4] ?? "";
    if (!klass) continue;
    // 括号配对扫完整签名 (跨行), 再找 "{"
    const start = lines[i].indexOf("(");
    let depth = 0, j = i, k = start, seenClose = -1;
    for (; j < lines.length; j++) {
      const line = j === i ? lines[j] : lines[j];
      for (; k < line.length; k++) {
        const ch = line[k];
        if (ch === "(") depth++;
        else if (ch === ")") {
          depth--;
          if (depth === 0) { seenClose = j; break; }
        }
      }
      if (seenClose >= 0) break;
      k = 0;
    }
    if (seenClose < 0) throw new Error(`签名未闭合 @${name}`);
    // 判别定义 vs 声明/调用: 闭括号后第一个 ";" 或 "{" 谁先出现
    let ej = seenClose;
    let verdict = "";
    outer: for (let y = seenClose; y < Math.min(seenClose + 16, lines.length); y++) {
      const from = y === seenClose ? k + 1 : 0;
      for (let x = from; x < lines[y].length; x++) {
        const ch = lines[y][x];
        if (ch === "{" || ch === ";") { verdict = ch; ej = y; break outer; }
      }
    }
    if (verdict !== "{") continue;
    const sigText = lines.slice(i, seenClose + 1).join(" ").replace(/\s+/g, " ");
    const pOpen = sigText.indexOf("(");
    let d = 0, pe = -1;
    for (let x = pOpen; x < sigText.length; x++) {
      if (sigText[x] === "(") d++;
      else if (sigText[x] === ")") { d--; if (d === 0) { pe = x; break; } }
    }
    const paramsText = sigText.slice(pOpen + 1, pe);
    const params = [];
    if (paramsText.trim()) {
      let pd = 0, cur = "";
      for (const ch of paramsText) {
        if (ch === "<" || ch === "(") pd++;
        else if (ch === ">" || ch === ")") pd--;
        if (ch === "," && pd === 0) { params.push(cur); cur = ""; }
        else cur += ch;
      }
      params.push(cur);
    }
    const parsed = params.map((p) => {
      p = p.replace(/=\s*[^,]*$/, "").trim();
      const words = p.split(/\s+/);
      const pname = (words[words.length - 1] ?? "").replace(/[^A-Za-z0-9_]/g, "");
      const ty = words.slice(0, -1).join(" ") || "usize";
      return { type: ty, name: pname || "arg" };
    });
    const ret = (m[1] ?? "").trim();
    const afterClose = sigText.slice(pe + 1).replace(/\{[\s\S]*$/, "").trim();
    const sig = sigText.slice(0, pe + 1) + (afterClose ? ` ${afterClose}` : "");
    out.push({ line: i + 1, klass, name, ret, params: parsed, sig });
    i = ej;
  }
  return out;
}

const rawDefs = scanDefs(cc);
const byClass = { TransitionsAccessor: 0, TransitionArray: 0, operator: 0 };
for (const d of rawDefs) byClass[d.klass]++;
must(byClass.TransitionsAccessor === 25, `原文 TransitionsAccessor ${byClass.TransitionsAccessor} != 25`);
must(byClass.TransitionArray === 10, `原文 TransitionArray ${byClass.TransitionArray} != 10`);
must(byClass.operator === 1, `原文 operator<< ${byClass.operator} != 1`);

const pp = clangE("src/objects/transitions.cc");
const tailZone = pp.slice(pp.lastIndexOf("namespace v8::internal {"));
const ppDefs = scanDefs(tailZone);
// DEBUG-only 定义: 原文 #ifdef DEBUG 区内定义在 pp (无 -DDEBUG) 中被剔除;
// 逐个验证其确实处于原文 #ifdef DEBUG 区 (transitions.cc:687-717)
const DEBUG_ONLY = new Set(["TransitionsAccessor::CheckNewTransitionsAreConsistent"]);
{
  const rawLines = cc.split("\n");
  const di = rawDefs.findIndex((d) => DEBUG_ONLY.has(`${d.klass}::${d.name}`));
  must(di >= 0, "原文缺 DEBUG-only 定义 CheckNewTransitionsAreConsistent");
  const before = rawLines.slice(Math.max(0, rawDefs[di].line - 8), rawDefs[di].line - 1).join("\n");
  must(before.includes("#ifdef DEBUG"), "DEBUG-only 定义不在 #ifdef DEBUG 区");
}
// 对数: pp 序列 = 原文序列剔除 DEBUG-only (顺序严格 1:1)
const rawKept = rawDefs.filter((d) => !DEBUG_ONLY.has(`${d.klass}::${d.name}`));
must(ppDefs.length === rawKept.length, `pp 定义数 ${ppDefs.length} != 原文(剔 DEBUG-only) ${rawKept.length}`);
rawKept.forEach((d, i) => {
  must(ppDefs[i].name === d.name && ppDefs[i].klass === d.klass,
    `定义序失配 @${i}: ${d.klass}::${d.name} vs pp ${ppDefs[i].klass}::${ppDefs[i].name}`);
});
for (const [k, n] of [["InsertHelper", "TransitionsAccessor"], ["SearchTransition", "TransitionsAccessor"],
  ["Sort", "TransitionArray"], ["PutPrototypeTransition", "TransitionsAccessor"],
  ["EnsureHasFullTransitionArray", "TransitionsAccessor"], ["TraverseTransitionTreeInternal", "TransitionsAccessor"]])
  must(rawDefs.some((d) => d.klass === n && d.name === k), `缺 ${n}::${k}`);

// ── Rust 骨架生成 ─────────────────────────────────────────────────────────
const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").replace(/(^|_)([A-Z]+)/g, (_, p, c) => p + c.toLowerCase()).replace(/__/g, "_").toLowerCase();
function rustTy(cpp) {
  cpp = cpp.trim();
  if (cpp === "bool") return "bool";
  if (/^(int|uint32_t|int32_t|uint64_t|size_t)$/.test(cpp)) return "u32";
  if (/^(double|float)$/.test(cpp)) return "f64";
  if (cpp === "const char*") return "&str";
  return "usize";
}
function rustRet(cpp) {
  cpp = cpp.trim();
  if (cpp === "void") return "";
  if (cpp === "bool") return "bool";
  if (cpp === "double") return "f64";
  if (/^std::optional</.test(cpp)) return "Option<usize>";
  if (/^std::ostream&$/.test(cpp)) return "&'static str"; // operator<< (Kind) 名字化
  return "usize";
}
// 本波可脱离 Isolate 的纯逻辑语义面就地填体 (Developer 回填); 其余保持 todo! + 旁注。
// key = rustName, value = Rust 函数体行 (含缩进)。
// 静态面 (无 self) 且原版不触 Isolate/factory/ReadOnlyRoots/锁 的, 按 usize↔Tagged 地址契约
// 委托 crate::transitions 既有 1:1 语义面; 分支/常量与 transitions.cc 逐行对齐。
const BODIES = {
  // v8: transitions.cc:893 — operator<<(ostream&, SideStepTransition::Kind): 枚举判别→固定串。
  side_step_transition_kind_to_str: [
    "  match _sidestep {",
    "    0 => \"Clone-object-IC-map\",",
    "    1 => \"Object.assign-map\",",
    "    2 => \"Object.assign-validity-cell\",",
    "    _ => \"\",",
    "  }",
  ],
  // v8: transitions.cc:17-26 — GetSimpleTransition: Acquire 读槽 → kWeakRef 取弱目标, default 空 Tagged(0)。
  transitions_accessor_get_simple_transition: [
    "  // SAFETY: _map 为 crate 堆模拟缓冲内有效 Map 对象地址 (usize↔Tagged 地址契约, 与 tests 布局一致)",
    "  let map = unsafe { Tagged::<Map>::from_raw_address(_map) };",
    "  // 原版 :19-20 raw_transitions(kAcquireLoad) + GetEncoding; :21-24 kWeakRef ⇒ 弱目标, default ⇒ 空",
    "  let accessor = TransitionsAccessor::new_with_concurrent(map, false);",
    "  match accessor.encoding() {",
    "    Encoding::WeakRef => accessor.get_target_from_weak_ref().map_or(0, |t| t.ptr()),",
    "    _ => 0,",
    "  }",
  ],
  // v8: transitions.cc:333-342 — CanHaveMoreTransitions: 字典 map ⇒ false; FullArray ⇒ 条目数 < 1536; 其余 ⇒ true。
  transitions_accessor_can_have_more_transitions: [
    "  // SAFETY: _map 为 crate 堆模拟缓冲内有效 Map 对象地址 (usize↔Tagged 地址契约)",
    "  let map = unsafe { Tagged::<Map>::from_raw_address(_map) };",
    "  // 原版 :335 map->is_dictionary_map() ⇒ false",
    "  if unsafe { (*map.as_ptr()).is_dictionary_map() } {",
    "    return false;",
    "  }",
    "  // 原版 :336-337 raw_transitions(kAcquireLoad) + GetEncoding",
    "  let accessor = TransitionsAccessor::new_with_concurrent(map, false);",
    "  match accessor.encoding() {",
    "    // 原版 :338-339 number_of_transitions() < kMaxNumberOfTransitions (编码为 Full 时 Some 恒成立)",
    "    Encoding::FullTransitionArray => {",
    "      accessor",
    "        .transitions()",
    "        .is_some_and(|t| t.number_of_transitions() < K_MAX_NUMBER_OF_TRANSITIONS)",
    "    }",
    "    // 原版 :341 其余编码 ⇒ true",
    "    _ => true,",
    "  }",
  ],
  // v8: transitions.cc:517-522 — SetNumberOfPrototypeTransitions: 槽 0 写 Smi::FromUInt (DCHECK ulength!=0)。
  transition_array_set_number_of_prototype_transitions: [
    "  // SAFETY: _proto_transitions 为 crate 堆模拟缓冲内有效 WeakFixedArray 对象地址 (地址契约)",
    "  let array = unsafe { Tagged::<WeakFixedArray>::from_raw_address(_proto_transitions) };",
    "  // 原版 :519 DCHECK_NE(proto_transitions->ulength().value(), 0)",
    "  debug_assert_ne!(unsafe { array.length() }, 0);",
    "  // 原版 :520-521 set(kProtoTransitionNumberOfEntriesOffset, Smi::FromUInt(value))",
    "  unsafe {",
    "    array.set(",
    "      K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET as u32,",
    "      Smi::from_uint(_value).cast(),",
    "      WriteBarrierMode::SkipWriteBarrier,",
    "    );",
    "  }",
  ],
  // v8: transitions.cc:554-562 — SetMigrationTarget: 仅 Uninitialized 时 Release 写槽 (DCHECK is_deprecated)。
  transitions_accessor_set_migration_target: [
    "  // SAFETY: _map/_migration_target 为 crate 堆模拟缓冲内有效 Map 对象地址 (地址契约)",
    "  unsafe {",
    "    let map = Tagged::<Map>::from_raw_address(_map);",
    "    // 原版 :559 GetEncoding(isolate, map) != kUninitialized ⇒ 直接返回 (仅空过渡 map 缓存迁移目标)",
    "    if TransitionsAccessor::new_with_concurrent(map, false).encoding() != Encoding::Uninitialized {",
    "      return;",
    "    }",
    "    // 原版 :560 DCHECK(map->is_deprecated())",
    "    debug_assert!((*map.as_ptr()).is_deprecated());",
    "    // 原版 :561 map->set_raw_transitions(migration_target, kReleaseStore)",
    "    let target = Tagged::<Map>::from_raw_address(_migration_target);",
    "    let slot_addr = (map.address() + mem::offset_of!(Map, transitions_or_prototype_info)) as *mut usize;",
    "    AtomicUsize::from_ptr(slot_addr).store(target.cast::<MaybeObject>().ptr(), Ordering::Release);",
    "  }",
  ],
  // v8: transitions.cc:572-585 — ReplaceTransitions: Release 写过渡槽 (DEBUG 一致性校验区随 #ifdef DEBUG 剔除)。
  transitions_accessor_replace_transitions: [
    "  // SAFETY: _map 为有效 Map 对象地址, _new_transitions 为 Tagged<UnionOf<TransitionArray, MaybeWeak<Map>>>",
    "  // 原编码位; 槽偏移对齐 Map::transitions_or_prototype_info, 与 crate::transitions 写侧同构, Release 发布",
    "  unsafe {",
    "    let map = Tagged::<Map>::from_raw_address(_map);",
    "    // 原版 :575-582 #if DEBUG CheckNewTransitionsAreConsistent 校验区不落地 (debug 面)",
    "    // 原版 :583 map->set_raw_transitions(new_transitions, kReleaseStore)",
    "    let slot_addr = (map.address() + mem::offset_of!(Map, transitions_or_prototype_info)) as *mut usize;",
    "    AtomicUsize::from_ptr(slot_addr).store(_new_transitions, Ordering::Release);",
    "  }",
  ],
  // v8: transitions.cc:588-592 — ReplaceTransitions(DirectHandle 重载): 转发裸 Union 重载 (原版 :591)。
  transitions_accessor_replace_transitions_2: [
    "  // 原版 :591 ReplaceTransitions(isolate, map, *new_transitions) — DirectHandle 解包转发",
    "  transitions_accessor_replace_transitions(_isolate, _map, _new_transitions);",
  ],
};
// 未填体面统一旁注 (readme/zh.md TR-ISO): 需 Isolate + DirectHandle + 堆分配/写屏障。
const B_TODO = "  todo!(\"HC/波次登记: TR-ISO — 依赖 v8-runtime/v8-heap-core/v8-ic\")";
function bodyLines(rustName) {
  return BODIES[rustName] ?? [B_TODO];
}

function fnBlock(d, rustName, retOverride) {
  const params = d.params.map((p) => `_${p.name}: ${rustTy(p.type)}`);
  const ret = retOverride ?? rustRet(d.ret);
  const retStr = ret ? ` -> ${ret}` : "";
  const doc = `/// v8: transitions.cc:${d.line} — ${d.sig.replace(/\s+/g, " ").trim()}`;
  const body = bodyLines(rustName);
  const one = `pub fn ${rustName}(${params.join(", ")})${retStr} {`;
  if (one.length <= 96) {
    return [doc, one, ...body, "}"];
  }
  return [doc, `pub fn ${rustName}(`,
    ...params.map((p) => `    ${p},`),
    `)${retStr} {`, ...body, "}"];
}

const used = new Map();
let rust = banner("src/objects/transitions.cc.js", "src/objects/transitions.cc",
  "TransitionsAccessor/TransitionArray 签名骨架 (clang -E 对数 25+10+1; 纯逻辑面就地填, 其余推迟登记 readme/zh.md 台账 TR-ISO)");
rust += `//! transitions.cc 实现清单骨架: TransitionsAccessor 25 + TransitionArray 10 + operator<< 1。
//! 参数类型为粗粒度占位 (句柄→usize/索引→u32/bool/&str), 为 handlified clang -E 对数存档。
//! 布局常量与非 isolate 语义面已在 \`crate::transitions\` 用 Tagged/Name/Map 类型 1:1 实现
//! (search_transition/insert/sort/search/for_each/traverse/target_details/is_matching_map)。
//! operator<< (SideStepTransition::Kind) 为纯枚举→串映射, 本波就地填体。
//! 本波另就地填体 6 个无 Isolate 依赖的静态面 (get_simple_transition/can_have_more_transitions/
//! set_number_of_prototype_transitions/set_migration_target/replace_transitions×2), 按
//! usize↔Tagged 地址契约委托 crate::transitions, 分支/常量与原版逐行对齐。
//! 其余 todo! 推迟 (readme/zh.md TR-ISO): 需 Isolate + DirectHandle + 堆分配 (WeakFixedArray 原型过渡)
//! + 写屏障的 handlified 分支, 解锁 crate v8-runtime / v8-heap-core / v8-ic。

use std::{
  mem,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_arrays::fixed_array::{TaggedArrayExt, WeakFixedArray};
use v8_objects_core::{
  map::Map,
  objects::WriteBarrierMode,
  smi::Smi,
  tagged::{MaybeObject, Tagged},
};

use crate::transitions::{
  Encoding, K_MAX_NUMBER_OF_TRANSITIONS, K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET,
  TransitionArrayExt, TransitionsAccessor,
};

`;

const blocks = [];
for (const d of rawDefs) {
  let rustName, retOverride;
  if (d.klass === "operator") {
    rustName = "side_step_transition_kind_to_str"; // operator<<(ostream&, SideStepTransition::Kind)
    retOverride = "&'static str";
  } else {
    const prefix = d.klass === "TransitionsAccessor" ? "transitions_accessor_" : "transition_array_";
    const base = `${prefix}${snake(d.name)}`;
    const n = (used.get(base) ?? 0) + 1;
    used.set(base, n);
    rustName = n === 1 ? base : `${base}_${n}`;
  }
  blocks.push(...fnBlock(d, rustName, retOverride), "");
}
rust += blocks.join("\n");

const p = writeSrc("v8-objects-maps", "transitions_cc_generated.rs", rust);
fmtNormalize(p);
console.log(`transitions_cc_generated.rs: TransitionsAccessor ${byClass.TransitionsAccessor} + TransitionArray ${byClass.TransitionArray} + operator<< 1 (pp 定义清单 1:1), 断言 ${checks} 项通过 → ${p}`);
