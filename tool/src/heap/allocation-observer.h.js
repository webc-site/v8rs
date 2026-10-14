#!/usr/bin/env bun
// tool/src/heap/allocation-observer.h.js — v8/src/heap/allocation-observer.h → v8-heap-core/src/allocation_observer.rs
// 规则:
//   ① 层级全集: AllocationObserver (5 成员) / AllocationCounter (9 方法 + 内嵌结构 1 + 字段 7) /
//      PauseAllocationObserversScope (3) — 逐签名锚
//   ② clang -E 目标配置展开侧: DEBUG 关 → HasAllocationObservers 剥除; 方法名逐项在案;
//      kNotUsingFixedStepSize = -1 锚
//   ③ Heap 硬耦合 (PauseScope) → 窄 trait PauseEnv 承接 (推迟台账双引用在 skel)
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/allocation-observer.h";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-ao.ii");
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

// ── ① 三阶级联成员锚 (原版行号随表) ───────────────────────────────────────
const OBS = [
  ["kNotUsingFixedStepSize=-1", /static constexpr intptr_t kNotUsingFixedStepSize = -1;/, 1, 20],
  ["ctor(step_size)", /explicit AllocationObserver\(intptr_t step_size\)/, 1, 21],
  ["virtual dtor", /virtual ~AllocationObserver\(\) = default;/, 1, 22],
  ["禁拷贝对", /AllocationObserver\(const AllocationObserver&\) = delete;/, 1, 23],
  ["纯虚 Step", /virtual void Step\(int bytes_allocated, Address soon_object, size_t size\) = 0;/, 1, 39],
  ["虚 GetNextStepSize", /virtual intptr_t GetNextStepSize\(\) \{/, 1, 42],
  ["字段 step_size_", /const intptr_t step_size_;/, 1, 48],
];
const COUNTER = [
  ["AllocationCounter ctor", /AllocationCounter\(\) = default;/, 1, 56],
  ["AddAllocationObserver", /void AddAllocationObserver\(AllocationObserver\* observer\);/, 1, 59],
  ["RemoveAllocationObserver", /void RemoveAllocationObserver\(AllocationObserver\* observer\);/, 1, 62],
  ["AdvanceAllocationObservers", /void AdvanceAllocationObservers\(size_t allocated\);/, 1, 65],
  ["InvokeAllocationObservers", /void InvokeAllocationObservers\(Address soon_object,/, 1, 69],
  ["IsStepInProgress", /bool IsStepInProgress\(\) const \{ return step_in_progress_; \}/, 1, 73],
  ["NextBytes", /size_t NextBytes\(\) const \{/, 1, 75],
  ["HasAllocationObservers(DEBUG)", /bool HasAllocationObservers\(\) const \{/, 1, 81],
  ["内嵌 AllocationObserverCounter", /struct AllocationObserverCounter final \{/, 1, 88],
];
const SCOPE = [
  ["PauseScope ctor(Heap*)", /explicit PauseAllocationObserversScope\(Heap\* heap\);/, 1, 111],
  ["PauseScope dtor", /~PauseAllocationObserversScope\(\);/, 1, 112],
  ["PauseScope 禁拷贝", /PauseAllocationObserversScope\(const PauseAllocationObserversScope&\) = delete;/, 1, 113],
];
// 类声明 3 + 字段 7 (counter) + counter 内嵌字段 3
const MISC = [
  ["class AllocationObserver \{", /^class AllocationObserver \{/m, 1],
  ["class AllocationCounter final", /^class AllocationCounter final \{/m, 1],
  ["V8_NODISCARD PauseScope", /class V8_EXPORT_PRIVATE V8_NODISCARD PauseAllocationObserversScope \{/, 1],
  ["observers_ 字段", /std::vector<AllocationObserverCounter> observers_;/, 1],
  ["pending_added_ 字段", /std::vector<AllocationObserverCounter> pending_added_;/, 1],
  ["pending_removed_ 字段", /std::unordered_set<AllocationObserver\*> pending_removed_;/, 1],
  ["current_counter_=0", /size_t current_counter_ = 0;/, 1],
  ["next_counter_=0", /size_t next_counter_ = 0;/, 1],
  ["step_in_progress_=false", /bool step_in_progress_ = false;/, 1],
  ["AOC 字段 observer_", /AllocationObserver\* observer_;/, 1],
  ["AOC 字段 prev_counter_", /size_t prev_counter_;/, 1],
  ["AOC 字段 next_counter_", /size_t next_counter_;/, 1],
];
for (const table of [OBS, COUNTER, SCOPE, MISC]) {
  for (const [label, re, want] of table) {
    if (count(src, re) !== want) fail(`锚 ${label}`, count(src, re), want);
    n++;
  }
}

// ── ② 展开侧 (DEBUG 关) ────────────────────────────────────────────────────
const expanded = preprocess(REL);
for (const cls of ["class AllocationObserver {", "class AllocationCounter final {", "class [[nodiscard]] PauseAllocationObserversScope {"]) {
  const got = expanded.split(cls).length - 1;
  if (got !== 1) fail(`展开类锚 ${cls}`, got, 1);
  n++;
}
if (!/static constexpr intptr_t kNotUsingFixedStepSize = -1;/.test(expanded)) fail("展开 kNotUsingFixedStepSize", false, true);
n++;
if (count(expanded, /HasAllocationObservers/g) !== 0) fail("展开后 DEBUG 方法残留", count(expanded, /HasAllocationObservers/g), 0);
n++;
if (count(expanded, /#if DEBUG|size_t AllocatedOnPage/g) !== 0) fail("展开后 DEBUG 守护残留", 0, 0);
n++;
for (const m of ["AddAllocationObserver", "RemoveAllocationObserver", "AdvanceAllocationObservers", "InvokeAllocationObservers", "IsStepInProgress", "NextBytes"]) {
  if (!expanded.includes(m)) fail(`展开缺方法 ${m}`, true, false);
  n++;
}
// 纯虚 Step 展开保留 (= 0)
if (count(expanded, /virtual void Step\(int bytes_allocated, Address soon_object, size_t size\) = 0;/) !== 1) fail("展开纯虚 Step", 0, 1);
n++;

// ── ③ rs 侧落地锚 (HC-1R: v8-heap-core/src/allocation_observer.rs 实态对数, 只增不减) ──
const rs = readFileSync(join(root, "v8-heap-core", "src", "allocation_observer.rs"), "utf8");
const rsCount = (re) => (rs.match(new RegExp(re.source, "g")) || []).length;
for (const [label, re, want] of [
  // Observer 基类面 (h:18-51) → trait + 哨兵常量
  ["kNotUsingFixedStepSize=-1 (h:20)", /pub const K_NOT_USING_FIXED_STEP_SIZE: isize = -1;/, 1],
  ["trait AllocationObserver (h:18)", /pub trait AllocationObserver \{/, 1],
  ["纯虚 Step → fn step 签名 (h:39)", /fn step\(&mut self, bytes_allocated: i32, soon_object: Address, size: usize\);/, 1],
  ["step_size_ 读取面 (h:48)", /fn step_size\(&self\) -> isize;/, 1],
  ["GetNextStepSize 默认实现 (h:42)", /fn next_step_size\(&self\) -> isize \{/, 1],
  ["h:43 DCHECK_NE 承接 debug_assert_ne", /debug_assert_ne!\(/, 3],
  // 禁 dyn → 手动虚表
  ["虚表槽 Step 类型", /type StepFn = unsafe fn\(this: \*mut \(\), bytes_allocated: i32, soon_object: Address, size: usize\);/, 1],
  ["虚表槽 NextStepSize 类型", /type NextStepSizeFn = unsafe fn\(this: \*mut \(\)\) -> isize;/, 1],
  ["ObserverRef (h:39/h:42 多态消费点)", /pub struct ObserverRef \{/, 1],
  ["ObserverRef::new<T> 单态化", /pub fn new<T: AllocationObserver>\(observer: &mut T\) -> Self/, 1],
  ["invoke_step (cc:100 调用面)", /unsafe fn invoke_step\(&mut self/, 1],
  ["invoke_next_step_size (cc:104/121 调用面)", /unsafe fn invoke_next_step_size\(&self\) -> isize/, 1],
  ["指针身份键 (原版指针相等比较)", /fn key\(&self\) -> usize/, 1],
  ["禁 dyn (本仓规范)", /\bdyn /, 0],
  // AllocationCounter (h:54-107)
  ["struct AllocationCounter (h:54)", /pub struct AllocationCounter \{/, 1],
  ["内嵌 AOC (h:88)", /struct AllocationObserverCounter \{/, 1],
  ["AOC 字段 observer_ (h:95)", /observer_: ObserverRef,/, 1],
  ["AOC 字段 prev_counter_ (h:96)", /prev_counter_: usize,/, 1],
  ["AOC 字段 next_counter_ (h:97)", /next_counter_: usize,/, 2],
  ["字段 observers_ (h:100)", /observers_: Vec<AllocationObserverCounter>,/, 1],
  ["字段 pending_added_ (h:101)", /pending_added_: Vec<AllocationObserverCounter>,/, 1],
  ["字段 pending_removed_ (h:102, 键=指针地址)", /pending_removed_: HashSet<usize>,/, 1],
  ["字段 current_counter_ (h:104)", /current_counter_: usize,/, 1],
  ["字段 step_in_progress_ (h:106)", /step_in_progress_: bool,/, 1],
  ["ctor = default (h:56)", /impl Default for AllocationCounter/, 1],
  ["AddAllocationObserver (h:59)", /pub fn add_allocation_observer\(&mut self, observer: ObserverRef\)/, 1],
  ["RemoveAllocationObserver (h:62)", /pub fn remove_allocation_observer\(&mut self, observer: ObserverRef\)/, 1],
  ["AdvanceAllocationObservers (h:65)", /pub fn advance_allocation_observers\(&mut self, allocated: usize\)/, 1],
  ["InvokeAllocationObservers (h:67-69)", /pub fn invoke_allocation_observers\(/, 1],
  ["IsStepInProgress (h:73)", /pub fn is_step_in_progress\(&self\) -> bool/, 1],
  ["NextBytes (h:75)", /pub fn next_bytes\(&self\) -> usize/, 1],
  ["NextBytes SIZE_MAX 面 (h:77)", /return usize::MAX;/, 1],
  ["HasAllocationObservers (h:81, #if DEBUG)", /pub fn has_allocation_observers\(&self\) -> bool/, 1],
  ["DEBUG 门 cfg(debug_assertions) (cc:16-22 重复注册预检 + h:81 方法)", /#\[cfg\(debug_assertions\)\]/, 2],
  ["DCHECK 面全以 debug_assert* 承接 (无 panic 路径)", /debug_assert/, 18],
  ["unsafe 虚表 SAFETY 契约标注", /SAFETY:/, 10],
  ["禁假绿/禁 panic 面: todo!/panic!/unimplemented!/unreachable! 零", /todo!\(\)|panic!\(\)|unimplemented!\(\)|unreachable!\(\)/, 0],
  // PauseAllocationObserversScope (h:109-119)
  ["V8_NODISCARD → #[must_use] (h:109)", /#\[must_use = "原版 V8_NODISCARD h:109/, 1],
  ["PauseScope 结构 (h:109)", /pub struct PauseAllocationObserversScope<'a, P: PauseEnv> \{/, 1],
  ["窄环境 trait PauseEnv (Heap 耦合面)", /pub trait PauseEnv \{/, 1],
  ["PauseEnv::gc_state_is_not_in_gc (cc:163 DCHECK)", /fn gc_state_is_not_in_gc\(&self\) -> bool;/, 1],
  ["PauseEnv::pause_allocation_observers (cc:164)", /fn pause_allocation_observers\(&mut self\);/, 1],
  ["PauseEnv::increment_pause_depth (cc:165)", /fn increment_pause_depth\(&mut self\);/, 1],
  ["PauseEnv::decrement_pause_depth (cc:169)", /fn decrement_pause_depth\(&mut self\);/, 1],
  ["PauseEnv::resume_allocation_observers (cc:170)", /fn resume_allocation_observers\(&mut self\);/, 1],
  ["ctor (h:111)", /pub fn new\(env: &'a mut P\) -> Self/, 1],
  ["dtor → Drop (h:112)", /impl<P: PauseEnv> Drop for PauseAllocationObserversScope/, 1],
  ["heap_ 字段 → 借用窄环境 (h:118)", /env: &'a mut P,/, 1],
]) {
  if (rsCount(re) !== want) fail(`rs 锚 ${label}`, rsCount(re), want);
  n++;
}
// 双行号 doc 面: 头侧声明行 h:NN 逐项在案 (实测各 1)
for (const ln of [20, 39, 42, 48, 56, 73, 75, 88, 100, 101, 102, 104, 105, 106, 109, 111, 112, 118]) {
  const got = rsCount(new RegExp(`h:${ln}\\b`));
  if (got < 1) fail(`rs 双行号 doc h:${ln}`, got, ">=1");
  n++;
}

const RS_ARM = `
// ── rs 侧落地锚 (HC-1R 实测台账) ──
// Landed @ v8-heap-core/src/allocation_observer.rs (46 结构/方法/字段锚 + 18 双行号 doc 锚):
//   Observer 7 面 → pub trait AllocationObserver (step/step_size/next_step_size 默认实现) +
//     ObserverRef 手动虚表 (type StepFn/NextStepSizeFn, new<T> 单态化, invoke_step/invoke_next_step_size,
//     key 指针身份; 全区 0 处 dyn — 本仓禁 dyn);
//   Counter 9 面 + 字段 7 → 同名 struct + Vec AOC + HashSet pending + SeqCst-free 纯 usize 计数;
//   PauseScope 3 面 → PauseEnv 窄 trait (5 调用面) + #[must_use] (V8_NODISCARD) + Drop RAII (dtor);
//   Heap 耦合实现 (gc_state/allocator/pause_depth) 推迟至 Heap 票 (模块头双行号: cc:161-171 ↔ PauseEnv 区)。
`;

const skel = `${banner("src/heap/allocation-observer.h.js", REL, "分配观察者层级台账: Observer 7 + Counter 9+7 字段 + PauseScope 3; DEBUG-only 1 剥除")
}
// ══ AllocationObserver (原版 h:18-51) → v8-heap-core/src/allocation_observer.rs ══
${OBS.map(([label, , , ln]) => `// ${label.padEnd(28)} h:${ln}`).join("\n")}
// 处置: 落地 — Rust trait AllocationObserver (step 纯虚 / next_step_size 默认实现);
// C++ 多态容器消费 (AllocationCounter 持有异构观察者) → ObserverRef 手动 fn 指针 vtable
// (等价 C++ 虚表, 遵本仓 禁 dyn 规范; SAFETY 逐块标注)
// ══ AllocationCounter (原版 h:54-107) → 同名结构落地, 逻辑全在 .cc (见 allocation-observer.cc.js) ══
${COUNTER.map(([label, , , ln]) => `// ${label.padEnd(32)} h:${ln}`).join("\n")}
// 字段 7: observers_/pending_added_/pending_removed_/current_counter_/next_counter_/step_in_progress_ (h:100-106)
// HasAllocationObservers h:81 — DEBUG-only: 落地为 #[cfg(debug_assertions)] 方法 (非假绿, 原版即调试断言面)
// ══ PauseAllocationObserversScope (原版 h:109-119) → 窄 trait PauseEnv 承接 ══
${SCOPE.map(([label, , , ln]) => `// ${label.padEnd(30)} h:${ln}`).join("\n")}
// Heap::allocator()/gc_state()/pause_allocation_observers_depth_ 属 Heap 票面 →
// Rust 侧以 PauseEnv trait 承接 (原版 h:111 声明 / cc:161-171 实现 双引用见模块), Drop 语义等价 RAII
`;
writeSrc("v8-heap-core", "allocation_observer_h.skel", skel + RS_ARM);
console.log(`OK allocation-observer.h.js: 对数断言 ${n} 条全过 (原版/展开侧 44 + rs 落地锚 66); Observer 7 + Counter 9 + PauseScope 3 + 类声明/字段 12 锚 + 展开侧 (类锚 3 / 常量 / DEBUG 剥除 0 / 方法 6 / 纯虚 Step); rs 侧 trait/虚表/Counter 字段/PauseScope 结构锚 + 双行号 doc 18; 产物 v8-heap-core/src/allocation_observer_h.skel`);
