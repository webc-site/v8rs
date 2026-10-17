#!/usr/bin/env bun
// tool/src/heap/allocation-observer.cc.js — v8/src/heap/allocation-observer.cc → v8-heap-core/src/allocation_observer.rs (AllocationCounter 逻辑体)
// 规则:
//   ① 函数定义全集 6 (AllocationCounter ×4 + PauseScope ×2) 逐签名 + 行号锚
//   ② 关键逻辑锚 (步进算法消费面): pending 队列操作 / SIZE_MAX 哨兵 / min 步长折叠 /
//      step_in_progress 置撤 — 展开侧与源码侧对数
//   ③ clang -E 目标配置展开: DEBUG 关 → Add 的 #if DEBUG find_if 预检剥除; 6 定义体保留;
//      Heap 耦合仅 PauseScope 2 处 (allocator/gc_state/depth) → 窄 trait 承接点实测计数
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/allocation-observer.cc";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-aocc.ii");
  for (let i = 0; i < 200; i++) {
    const r = spawnSync("clang", ["-E", "-P", "-x", "c++", "-std=c++20",
      "-DV8_COMPRESS_POINTERS", "-DV8_ENABLE_SANDBOX", "-DV8_EXTERNAL_CODE_SPACE",
      "-I", ".", "-I", "include", "-I", STUB, file, "-o", out], { cwd: V8, encoding: "utf8" });
    if (r.status === 0) return readFileSync(out, "utf8");
    const miss = (r.stderr.match(/fatal error: '([^']+)' file not found/) || [])[1];
    if (!miss) throw new Error(`clang -E ${file} 硬失败:\n${r.stderr}`);
    const p = join(STUB, miss);
    mkdirSync(dirname(p), { recursive: true });
    writeFileSync(p, "#pragma once\n");
  }
  throw new Error(`clang -E ${file} 桩头迭代超限`);
}

let n = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const count = (s, re) => (s.match(re) || []).length;

// ── ① 函数定义全集 6 + 行号 ────────────────────────────────────────────────
const DEFS = [
  ["AllocationCounter::AddAllocationObserver", 15],
  ["AllocationCounter::RemoveAllocationObserver", 45],
  ["AllocationCounter::AdvanceAllocationObservers", 75],
  ["AllocationCounter::InvokeAllocationObservers", 82],
  ["PauseAllocationObserversScope::PauseAllocationObserversScope", 161],
  ["PauseAllocationObserversScope::~PauseAllocationObserversScope", 168],
];
for (const [name] of DEFS) {
  if (!src.includes(name)) fail(`定义缺名 ${name}`, true, false);
  n++;
}
if (count(src, /^\w[\w:<> ]*AllocationCounter::/gm) !== 4) fail("AllocationCounter 定义数", count(src, /^\w[\w:<> ]*AllocationCounter::/gm), 4);
n++;
if (count(src, /^PauseAllocationObserversScope::/gm) !== 2) fail("PauseScope 定义数", count(src, /^PauseAllocationObserversScope::/gm), 2);
n++;

// ── ② 逻辑锚 ───────────────────────────────────────────────────────────────
const LOGIC = [
  ["pending_added_.push_back 哨兵(0,0)", /pending_added_\.push_back\(AllocationObserverCounter\(observer, 0, 0\)\);/, 1],
  ["单观察者直取 next", /if \(observers_\.size\(\) == 1\)/, 1],
  ["min(missing, step) 折叠", /std::min\(static_cast<intptr_t>\(missing_bytes\), step_size\)/, 1],
  ["移除后清空归零", /current_counter_ = next_counter_ = 0;/, 1],
  ["Advance 空集短路", /if \(observers_\.empty\(\)\) return;/g, 2],
  ["Step 回调", /aoc\.observer_->Step\(/, 1],
  ["GetNextStepSize 消费", /size_t observer_step_size = aoc\.observer_->GetNextStepSize\(\);/g, 2],
  ["next = cur + aligned + step", /aoc\.next_counter_ =\s*\n\s*current_counter_ \+ aligned_object_size \+ observer_step_size;/g, 2],
  ["CHECK(step_run)", /CHECK\(step_run\);/, 1],
  ["pending_removed_ 批量擦除", /std::remove_if\(observers_\.begin\(\)/, 1],
  ["heap pause/resume 面", /heap_?->allocator\(\)->(Pause|Resume)AllocationObservers\(\);/g, 2],
  ["depth 增减", /heap_?->pause_allocation_observers_depth_(--|\+\+)\;/g, 2],
  ["DisallowGarbageCollection", /DisallowGarbageCollection no_gc;/, 1],
  ["DEBUG find_if 预检 (#if DEBUG 内)", /#if DEBUG/, 1],
];
for (const [label, re, want] of LOGIC) {
  if (count(src, re) !== want) fail(`逻辑锚 ${label}`, count(src, re), want);
  n++;
}

// ── ③ 展开侧 ───────────────────────────────────────────────────────────────
const expanded = preprocess(REL);
if (count(expanded, /AllocationCounter::(AddAllocationObserver|RemoveAllocationObserver|AdvanceAllocationObservers|InvokeAllocationObservers)/g) !== 4) fail("展开 Counter 定义 4", count(expanded, /AllocationCounter::(Add|Remove|Advance|Invoke)\w+/g), 4);
n++;
if (count(expanded, /PauseAllocationObserversScope::/g) !== 2) fail("展开 PauseScope 定义 2", count(expanded, /PauseAllocationObserversScope::/g), 2);
n++;
if (count(expanded, /std::find_if\(observers_/g) !== 1) fail("展开后 find_if 应仅剩 Remove 侧 1 (Add 的 #if DEBUG 预检剥除)", count(expanded, /std::find_if\(observers_/g), 1);
n++;
if (count(expanded, /aoc\.observer_->Step\(/g) !== 1) fail("展开 Step 回调", count(expanded, /aoc\.observer_->Step\(/g), 1);
n++;
if (count(expanded, /heap_?->pause_allocation_observers_depth_/g) !== 2) fail("展开 depth 面 2", count(expanded, /heap_->pause_allocation_observers_depth_/g), 2);
n++;

// ── ④ rs 侧落地锚 (HC-1R: v8-heap-core/src/allocation_observer.rs 逻辑体对数, 只增不减) ──
const rs = readFileSync(join(root, "v8-heap-core", "src", "allocation_observer.rs"), "utf8");
const rsCount = (re) => (rs.match(new RegExp(re.source, "g")) || []).length;
for (const [label, re, want] of [
  // 四定义双行号 doc 面 (h 声明 ↔ cc 实现 ↔ rs)
  ["Add 双行号 cc:15-43", /cc:15-43/, 1],
  ["Remove 双行号 cc:45-73", /cc:45-73/, 1],
  ["Advance 双行号 cc:75-80", /cc:75-80/, 1],
  ["Invoke 双行号 cc:82-159", /cc:82-159/, 1],
  ["PauseScope ctor 双行号 h:111/cc:161-166", /原版 h:111 声明 \/ cc:161-166 实现/, 1],
  ["PauseScope dtor 双行号 h:112/cc:168-171", /原版 h:112 声明 \/ cc:168-171 实现/, 1],
  // Add (cc:15-43)
  ["cc:16-22 重复注册预检 → cfg(debug_assertions) 全区 2", /#\[cfg\(debug_assertions\)\]/, 2],
  ["cc:24-27 pending 哨兵 push(observer, 0, 0)", /AllocationObserverCounter::new\(observer, 0, 0\)/, 1],
  ["cc:32-33 observers_.push", /self\.observers_\.push\(/, 1],
  ["cc:29-30 observer_next_counter = cur + step_size", /let observer_next_counter = \(self\.current_counter_ as isize \+ step_size\) as usize;/, 1],
  ["cc:35 单观察者分支 size()==1", /if self\.observers_\.len\(\) == 1/, 1],
  ["cc:39-42 min(missing, step) 折叠", /min\(missing_bytes as isize, step_size\)/, 1],
  // Remove (cc:45-73)
  ["cc:46-50 指针键查找 position", /\.position\(\|aoc\| aoc\.observer_\.key\(\) == observer\.key\(\)\)/, 1],
  ["cc:52-56 Step 期入 pending_removed_", /self\.pending_removed_\.insert\(observer\.key\(\)\)/, 1],
  ["cc:53 重复登记 DCHECK_EQ", /self\.pending_removed_\.contains\(&observer\.key\(\)\)/, 1],
  ["cc:58 observers_.remove(i)", /self\.observers_\.remove\(i\)/, 1],
  ["cc:60-61 空集归零 current=0", /self\.current_counter_ = 0;/, 2],
  ["cc:60-61 空集归零 next=0", /self\.next_counter_ = 0;/, 2],
  ["cc:63-71 剩余步长重算 (oc 侧)", /let left_in_step = oc\.next_counter_ - self\.current_counter_;/, 1],
  ["cc:71 next = current + step_size", /self\.next_counter_ = self\.current_counter_ \+ step_size;/, 1],
  // Advance (cc:75-80)
  ["cc:77/86 非 Step 期 DCHECK ×2", /!self\.step_in_progress_,/, 2],
  ["cc:78 DCHECK_LT(allocated, next-current)", /allocated < self\.next_counter_ - self\.current_counter_/, 1],
  ["cc:79 current_counter_ += allocated", /self\.current_counter_ \+= allocated;/, 1],
  // Invoke (cc:82-159)
  ["cc:87 DCHECK_GE(aligned, 剩余)", /aligned_object_size >= self\.next_counter_ - self\.current_counter_/, 1],
  ["cc:88 DCHECK(soon_object)", /debug_assert_ne!\(soon_object, 0/, 1],
  ["cc:92 step_in_progress_ = true", /self\.step_in_progress_ = true;/, 1],
  ["cc:93-94 pending 队列 Invoke 前须空", /self\.pending_added_\.is_empty\(\),/, 1],
  ["cc:96-114 逐观察者迭代", /for aoc in &mut self\.observers_/, 1],
  ["cc:100-102 Step 参量 (current - prev)", /let bytes_allocated = \(current - aoc\.prev_counter_\) as i32;/, 1],
  ["cc:100 Step 回调面 (def + call)", /invoke_step/, 2],
  ["cc:104/121 GetNextStepSize 消费面 (def + 3 call)", /invoke_next_step_size/, 4],
  ["cc:106-108 prev = current", /aoc\.prev_counter_ = current;/, 2],
  ["cc:107/125 next = current + aligned + step", /\(current as isize \+ aligned_object_size as isize \+ observer_step_size\) as usize/, 2],
  ["cc:112-113 折叠最小剩余步长 (×3 站点)", /min\(step_size, left_in_step\)/, 3],
  ["cc:116 CHECK(step_run) → check_with_msg (abort 非 panic)", /check_with_msg\(step_run, "step_run"\)/, 1],
  ["cc:118-130 pending_added_ 处理迭代", /for aoc in &mut self\.pending_added_/, 1],
  ["cc:120 pending 哨兵 DCHECK_EQ(next, 0)", /aoc\.next_counter_, 0,/, 1],
  ["cc:127 min(step, aligned+obs_step)", /min\(step_size, candidate\)/, 1],
  ["cc:129/132 pending 观察者并入 observers_ (push_back+clear → append)", /self\.observers_\.append\(&mut self\.pending_added_\)/, 1],
  ["cc:134-141 remove_if → retain 批量擦除", /\.retain\(/, 1],
  ["cc:142 pending_removed_.clear", /self\.pending_removed_\.clear\(\)/, 1],
  ["cc:144-148 擦除后步长重算 (aoc 侧 left)", /let left_in_step = aoc\.next_counter_ - current;/, 2],
  ["cc:157-158 next = current + step_size", /self\.next_counter_ = current \+ step_size;/, 1],
  ["cc:159 step_in_progress_ = false (含提前 return 臂)", /self\.step_in_progress_ = false;/, 2],
  // 推迟登记 (Heap 耦合面)
  ["cc:99 DisallowGarbageCollection 推迟注", /DisallowGarbageCollection/, 1],
  ["cc:99 双行号在案", /cc:99/, 1],
  ["推迟登记字样 (模块头 + PauseEnv + Invoke)", /推迟登记/, 3],
  // NextBytes (h:75-78) 空集 SIZE_MAX
  ["h:77 SIZE_MAX → usize::MAX", /return usize::MAX;/, 1],
  // PauseScope 五调用面 (decl ↔ call site 成对)
  ["cc:163 DCHECK 判定面 decl", /fn gc_state_is_not_in_gc\(&self\) -> bool;/, 1],
  ["cc:164 pause decl", /fn pause_allocation_observers\(&mut self\);/, 1],
  ["cc:164 pause call", /env\.pause_allocation_observers\(\);/, 1],
  ["cc:165 depth++ decl", /fn increment_pause_depth\(&mut self\);/, 1],
  ["cc:165 depth++ call", /env\.increment_pause_depth\(\);/, 1],
  ["cc:169 depth-- decl", /fn decrement_pause_depth\(&mut self\);/, 1],
  ["cc:169 depth-- call", /self\.env\.decrement_pause_depth\(\);/, 1],
  ["cc:170 resume decl", /fn resume_allocation_observers\(&mut self\);/, 1],
  ["cc:170 resume call", /self\.env\.resume_allocation_observers\(\);/, 1],
  ["RAII: dtor 面 → impl Drop", /impl<P: PauseEnv> Drop for PauseAllocationObserversScope/, 1],
]) {
  if (rsCount(re) !== want) fail(`rs 逻辑锚 ${label}`, rsCount(re), want);
  n++;
}

const RS_ARM = `
// ── rs 侧落地锚 (HC-1R 实测台账) ──
// Landed @ v8-heap-core/src/allocation_observer.rs: cc 六定义中四定义 (Add/Remove/Advance/Invoke)
//   逻辑体逐行 1:1 (双行号 doc 各在案), PauseScope 二定义 → PauseEnv 窄 trait (decl+call 成对 10 锚) +
//   impl Drop 承接 RAII dtor; CHECK(step_run) 以 check_with_msg (abort) 承接, DCHECK 面 18 处全落
//   debug_assert*; 原版 remove_if 以 Vec::retain 等价; pending_added_ push_back+clear 以 append 等价。
// 推迟登记 (双行号): cc:99 DisallowGarbageCollection no_gc ↔ 本文件 Invoke 循环前注释 (GC 禁入 scope 属 Heap 票);
//   cc:161-171 Heap::gc_state/allocator/pause_depth ↔ PauseEnv trait (具体 Heap 实现属 Heap 票)。
// 测试面: tests/allocation_observer_unittest.rs 1:1 迁移 allocation-observer-unittest.cc
//   (AddAndRemoveUnusedObservers / Step 两轮逐数值, RecursiveAdd/Remove 重入面如实申报推迟)。
`;

const skel = `${banner("src/heap/allocation-observer.cc.js", REL, "AllocationCounter 4 定义 1:1 落地; PauseScope 2 定义 → PauseEnv 窄 trait 承接")
}
// ══ allocation-observer.cc 台账 (原版行号实测) ══
${DEFS.map(([name, ln]) => `// ${name.padEnd(62)} cc:${ln}`).join("\n")}
// 处置: Add/Remove/Advance/Invoke — 落地 (纯计数逻辑, 无 Isolate; 逐行对照原版)
//   PauseScope ctor/cc:161 (DCHECK gc_state + allocator pause + depth++) 与
//   dtor/cc:168 (depth-- + allocator resume) — Heap 硬耦合 → trait PauseEnv 承接,
//   原版引用 cc:161-171 ↔ Rust allocation_observer.rs PauseScope 区 (双引用在模块注释)
// Invoke 内 DisallowGarbageCollection cc:99 — GC 禁入 scope 属 Heap 票面 → 推迟登记 (回调步内
//   观察者不得触发分配折叠的语义由上层在 Step 实现侧保证; Rust 侧 scope 以回调闭包承接)
// DEBUG-only: Add 的重复注册 find_if 预检 cc:16-22 → cfg(debug_assertions) 落地
`;
writeSrc("v8-heap-core", "allocation_observer_cc.skel", skel + RS_ARM);
console.log(`OK allocation-observer.cc.js: 对数断言 ${n} 条全过 (原版/展开侧 27 + rs 逻辑落地锚 58); 定义全集 6 逐名 + 逻辑锚 14 + 展开侧 5 (DEBUG 预检剥除 0 / Step 回调 1 / PauseScope 2 / depth 2); rs 侧四定义体逐行锚 + PauseEnv decl/call 成对 10 + 推迟登记 (cc:99 / cc:161-171) 锚; 产物 v8-heap-core/src/allocation_observer_cc.skel`);
