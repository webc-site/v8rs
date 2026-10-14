#!/usr/bin/env bun
// tool/src/regexp/regexp-stack.h.js — v8/src/regexp/regexp-stack.h → v8-regexp-exec/src/regexp_stack.rs
// 规则: 常量表实测值对数 + ThreadLocal 字段序/offset 结构 + StackScope/Stack/ThreadLocal 方法全集
//   + 锁结构核查 (原版 0 处 Mutex/条件变量 — 线程安全靠 per-isolate 所有权 + archive/restore)
// 产物: v8-regexp-exec/src/regexp_stack.rs.skel (签名骨架基线, 审计专用; 落地文件以就地修订为准)
//   + 落地对数核对 (锚点逐项须在 regexp_stack.rs 找到, 缺项 throw 列票单)
// 配置展开: V8_TARGET_ARCH_PPC64/S390X 分支排除 (默认 x64/arm64 → kSlotSize=kInt32Size);
//   V8_ENABLE_SANDBOX_HARDWARE_SUPPORT 排除 (非沙箱硬件默认)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const h = readV8("src/regexp/regexp-stack.h");

// ── 对数断言 (原版锚点 + 实测值; 任一不符 throw) ────────────────────────────
let assertCount = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配: ${label} got=${JSON.stringify(got)} want=${JSON.stringify(want)}`);
};
const count = (src, re) => (src.match(re) || []).length;
const grab = (label, re) => {
  const m = h.match(re);
  if (!m) throw new Error(`原版锚点缺失: ${label}`);
  return m;
};

// 1) 类常量实测值
const kSlotBranches = [
  ["kSlotSize = kSystemPointerSize", /kSlotSize = kSystemPointerSize/g, 1],
  ["kSlotSize = kInt32Size", /kSlotSize = kInt32Size/g, 1],
  ["kStackLimitSlackSlotCount = 32", /kStackLimitSlackSlotCount = 32;/g, 1],
  ["kStackLimitSlackSize 公式", /kStackLimitSlackSlotCount \* kSlotSize/g, 1],
  ["kMaximumStackSize = 64 * MB", /kMaximumStackSize = 64 \* MB;/g, 1],
  ["kStaticStackSize = 1 * KB", /kStaticStackSize = 1 \* KB;/g, 1],
  ["kMinimumDynamicStackSize = 2 * KB", /kMinimumDynamicStackSize = 2 \* KB;/g, 1],
  ["kMemoryTop = (Address)(uintptr_t)-1", /static_cast<Address>\(static_cast<uintptr_t>\(-1\)\)/g, 1],
  ["kThreadLocalSize = sizeof(ThreadLocal)", /kThreadLocalSize = sizeof\(ThreadLocal\)/g, 1],
] ;
for (const [label, re, want] of kSlotBranches) {
  if (count(h, re) !== want) fail(label, count(h, re), want);
  assertCount++;
}
// 常量声明总数 (static constexpr / static const 混合表)
const CONST_TOTAL = 12;
if (count(h, /static (constexpr|const) [\w:]+ k\w+/g) !== CONST_TOTAL)
  fail("Stack/StackScope 常量声明总数", count(h, /static (constexpr|const) [\w:]+ k\w+/g), CONST_TOTAL);
assertCount++;

// 2) static_assert ×3 (kStaticStack 双界 + 最小动态栈)
if (count(h, /static_assert\(/g) !== 3) fail("static_assert 总数", count(h, /static_assert\(/g), 3);
assertCount++;

// 3) ThreadLocal 字段序 (h:131-160: memory_/memory_top_/memory_size_/stack_pointer_/limit_/owns_memory_)
const FIELD_ORDER = [
  "uint8_t* memory_ = nullptr;",
  "uint8_t* memory_top_ = nullptr;",
  "size_t memory_size_ = 0;",
  "uint8_t* stack_pointer_ = nullptr;",
  "Address limit_ = kNullAddress;",
  "bool owns_memory_ = false;",
];
const fieldLines = grab("ThreadLocal 字段区", /struct ThreadLocal \{([\s\S]*?)\n  \};/)[1];
for (const f of FIELD_ORDER) {
  if (!fieldLines.includes(f)) fail(`ThreadLocal 字段 ${f}`, true, false);
  assertCount++;
}
// 生成码访存 offset ×3 (memory_top_/stack_pointer_/limit_; 声明可跨行)
if (count(h, /offsetof\(ThreadLocal, \w+\)/g) !== 3)
  fail("offsetof 常量总数", count(h, /offsetof\(ThreadLocal, \w+\)/g), 3);
assertCount++;

// 4) 方法全集 (StackScope 3 + Stack 公有 12 + Stack 私有 6 + ThreadLocal 6; deleted copy ×2 族不计)
const METHOD_TABLE = {
  // StackScope: ctor / dtor / stack()
  "StackScope(Isolate": /explicit StackScope\(Isolate\* isolate\);/,
  "~StackScope": /~StackScope\(\);/,
  "stack()": /Stack\* stack\(\) const/,
  // Stack 公有
  New: /static Stack\* New\(\);/,
  Delete: /static void Delete\(Stack\* instance\);/,
  begin: /Address begin\(\) const/,
  end: /Address end\(\) const/,
  memory_top: /Address memory_top\(\) const/,
  stack_pointer: /Address stack_pointer\(\) const/,
  memory_size: /size_t memory_size\(\) const/,
  EnsureCapacity: /Address EnsureCapacity\(size_t size\);/,
  ArchiveSpacePerThread: /static constexpr int ArchiveSpacePerThread\(\)/,
  ArchiveStack: /char\* ArchiveStack\(char\* to\);/,
  RestoreStack: /char\* RestoreStack\(char\* from\);/,
  FreeThreadResources: /void FreeThreadResources\(\)/,
  // Stack 私有
  StackCtor: /Stack\(\);/,
  StackDtor: /~Stack\(\);/,
  thread_local_address: /Address thread_local_address\(\)/,
  sp_top_delta: /ptrdiff_t sp_top_delta\(\) const/,
  ResetIfEmpty: /void ResetIfEmpty\(\)/,
  IsValid: /bool IsValid\(\) const/,
  // ThreadLocal
  ThreadLocalCtor: /explicit ThreadLocal\(Stack\* regexp_stack\)/,
  ResetToStaticStack: /void ResetToStaticStack\(Stack\* regexp_stack\);/,
  ResetToStaticStackIfEmpty: /void ResetToStaticStackIfEmpty\(Stack\* regexp_stack\)/,
  FreeAndInvalidate: /void FreeAndInvalidate\(\);/,
  NewDynamicStack: /static uint8_t\* NewDynamicStack\(size_t size\);/,
  DeleteDynamicStack: /void DeleteDynamicStack\(\);/,
};
for (const [name, re] of Object.entries(METHOD_TABLE)) {
  if (!re.test(h)) fail(`方法锚 ${name}`, true, false);
  assertCount++;
}
if (Object.keys(METHOD_TABLE).length !== 27) fail("方法全集大小", Object.keys(METHOD_TABLE).length, 27);
assertCount++;

// 5) friend 面 (ExternalReference + StackScope + 9 宏汇编 = 11)
if (count(h, /friend class /g) !== 11) fail("friend 声明总数", count(h, /friend class /g), 11);
assertCount++;

// 6) Mutex/条件变量结构核查: 原版 0 处 — 线程安全 = per-isolate 所有权 + 显式归档/恢复
if (count(h, /Mutex|LockGuard|condition_variable|std::atomic/g) !== 0)
  fail("h 锁结构应为 0 (线程亲和模型)", count(h, /Mutex|LockGuard|condition_variable|std::atomic/g), 0);
assertCount++;
// V8_NODISCARD StackScope + 禁拷贝族 (Stack/StackScope 各 2)
if (count(h, /= delete;/g) !== 4) fail("deleted copy/assign 总数", count(h, /= delete;/g), 4);
assertCount++;

// ── 产物: 签名骨架 ─────────────────────────────────────────────────────────
const skel = `${banner(
  "src/regexp/regexp-stack.h.js",
  "src/regexp/regexp-stack.h",
  "签名骨架基线 (审计专用): 常量/字段序/方法全集逐项 1:1; 落地以 regexp_stack.rs 就地修订为准",
)}
//! regexp-stack.h → regexp_stack.rs 处置表 (${Object.keys(METHOD_TABLE).length} 方法 / ${CONST_TOTAL} 常量):
//! - kSlotSize 非 PPC64/S390X 分支 = K_INT32_SIZE; 沙箱硬件分支排除
//! - ThreadLocal: repr(C) 六字段序 1:1; offset 常量 ×3 + K_THREAD_LOCAL_SIZE
//! - Stack::New/Delete → Stack::new() (Box 堆定址静态区) + Drop
//! - StackScope(Isolate*) → StackScope::new(&Stack) 参数注入 (Isolate L3 前占位)
//! - friend 访存面 → pub thread_local_address() + K_*_OFFSET (代码生成侧消费)
//! - 锁: 0 处 (线程亲和 !Send/!Sync + archive/restore)

// pub const K_SLOT_SIZE / K_STACK_LIMIT_SLACK_SLOT_COUNT / K_STACK_LIMIT_SLACK_SIZE
// pub const K_MAXIMUM_STACK_SIZE / K_STATIC_STACK_SIZE / K_MINIMUM_DYNAMIC_STACK_SIZE
// const K_MEMORY_TOP; static_assert ×3 → const _: () = assert!(..)
// struct ThreadLocal (repr(C), Clone+Copy) + INIT + 6 方法
// pub struct Stack { static_stack_: Box<UnsafeCell<[u8; K]>>, thread_local_: UnsafeCell<ThreadLocal> }
//   new/begin/end/memory_top/stack_pointer/memory_size/ensure_capacity/
//   archive_space_per_thread/archive_stack/restore_stack/free_thread_resources/thread_local_address
//   私有: sp_top_delta/reset_if_empty/is_valid/static_stack_ptr/tl
// pub struct StackScope<'a> { regexp_stack_: &Stack, old_sp_top_delta_: isize } + Drop
`;

writeSrc("v8-regexp-exec", "regexp_stack.rs.skel", skel);

// ── 落地对数核对 (regexp_stack.rs 就地修订版逐项命中; 缺项 throw 列票单) ────
const landed = readFileSync(join(root, "v8-regexp-exec", "src", "regexp_stack.rs"), "utf8");
const LAND_TABLE = [
  [/pub const K_SLOT_SIZE: usize = K_INT32_SIZE;/g, 1, "K_SLOT_SIZE 默认分支"],
  [/pub const K_STACK_LIMIT_SLACK_SLOT_COUNT: usize = 32;/g, 1, "slack 槽数"],
  [/pub const K_STACK_LIMIT_SLACK_SIZE: usize = K_STACK_LIMIT_SLACK_SLOT_COUNT \* K_SLOT_SIZE;/g, 1, "slack 公式"],
  [/pub const K_MAXIMUM_STACK_SIZE: usize = 64 \* MB;/g, 1, "上限 64MB"],
  [/const K_STATIC_STACK_SIZE: usize = KB;/g, 1, "静态区 1KB"],
  [/const K_MINIMUM_DYNAMIC_STACK_SIZE: usize = 2 \* KB;/g, 1, "最小动态栈 2KB"],
  [/const K_MEMORY_TOP: Address = usize::MAX;/g, 1, "kMemoryTop"],
  [/const _: \(\) = assert!\(/g, 3, "static_assert ×3"],
  [/#\[repr\(C\)\]/g, 1, "ThreadLocal repr(C)"],
  [/offset_of!\(ThreadLocal, (memory_top_|stack_pointer_|limit_)\)/g, 3, "offset 常量 ×3"],
  [/pub const K_THREAD_LOCAL_SIZE: usize = size_of::<ThreadLocal>\(\);/g, 1, "K_THREAD_LOCAL_SIZE"],
  [/memory_: \*mut u8,/g, 1, "字段 memory_"],
  [/memory_top_: \*mut u8,/g, 1, "字段 memory_top_"],
  [/memory_size_: usize,/g, 1, "字段 memory_size_"],
  [/stack_pointer_: \*mut u8,/g, 1, "字段 stack_pointer_"],
  [/limit_: Address,/g, 1, "字段 limit_"],
  [/owns_memory_: bool,/g, 1, "字段 owns_memory_"],
  [/fn reset_to_static_stack\(&mut self/g, 1, "ResetToStaticStack"],
  [/fn reset_to_static_stack_if_empty\(&mut self/g, 1, "ResetToStaticStackIfEmpty"],
  [/fn free_and_invalidate\(&mut self/g, 1, "FreeAndInvalidate"],
  [/fn new_dynamic_stack\(size: usize\) -> \*mut u8/g, 1, "NewDynamicStack"],
  [/fn delete_dynamic_stack\(&mut self/g, 1, "DeleteDynamicStack"],
  [/pub fn begin\(&self\) -> Address/g, 1, "begin"],
  [/pub fn end\(&self\) -> Address/g, 1, "end"],
  [/pub fn memory_top\(&self\) -> Address/g, 1, "memory_top"],
  [/pub fn stack_pointer\(&self\) -> Address/g, 1, "stack_pointer"],
  [/pub fn memory_size\(&self\) -> usize/g, 1, "memory_size"],
  [/pub fn ensure_capacity\(&self, size: usize\) -> Address/g, 1, "EnsureCapacity"],
  [/pub const fn archive_space_per_thread\(\) -> i32/g, 1, "ArchiveSpacePerThread"],
  [/pub unsafe fn archive_stack\(&self, to: \*mut u8\) -> \*mut u8/g, 1, "ArchiveStack"],
  [/pub unsafe fn restore_stack\(&self, from: \*mut u8\) -> \*mut u8/g, 1, "RestoreStack"],
  [/pub fn free_thread_resources\(&self\)/g, 1, "FreeThreadResources"],
  [/pub fn thread_local_address\(&self\) -> Address/g, 1, "thread_local_address (friend 面)"],
  [/fn sp_top_delta\(&self\) -> isize/g, 1, "sp_top_delta"],
  [/fn reset_if_empty\(&self\)/g, 1, "ResetIfEmpty"],
  [/fn is_valid\(&self\) -> bool/g, 1, "IsValid"],
  [/pub struct StackScope<'a>/g, 1, "StackScope"],
  [/pub fn stack\(&self\) -> &'a Stack/g, 1, "StackScope::stack()"],
  [/impl Drop for Stack \{/g, 1, "~Stack → Drop"],
  [/impl Drop for StackScope/g, 1, "~StackScope → Drop"],
  [/check_op_eq\(&self\.old_sp_top_delta_, &now, "sp_top_delta"\)/g, 1, "CHECK_EQ 冻结件承接"],
];
const missing = [];
for (const [re, want, label] of LAND_TABLE) {
  const got = count(landed, re);
  if (got !== want) missing.push(`${label}: got=${got} want=${want}`);
  assertCount++;
}
if (missing.length) throw new Error(`regexp_stack.rs 落地对数缺项 (票单):\n  ${missing.join("\n  ")}`);

console.log(
  `regexp-stack.h.js: 对数断言 ${assertCount} 条全过; 原版 ${CONST_TOTAL} 常量 (kSlotSize=kInt32Size 默认分支, ` +
    `slack 32 槽=128B, 上限 64MB, 静态 1KB, 动态最小 2KB) / 27 方法全集 / ThreadLocal 六字段序 + offsetof ×3 / ` +
    `friend ×11 / 锁结构 0 处 (archive/restore 模型); 落地 36 锚逐项命中, 缺项 0; 产物 v8-regexp-exec/src/regexp_stack.rs.skel`,
);
