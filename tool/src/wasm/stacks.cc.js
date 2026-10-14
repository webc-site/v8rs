#!/usr/bin/env bun
// tool/src/wasm/stacks.cc.js
// 机械提取 v8/src/wasm/stacks.cc 的栈内存分配/增长/收缩规则:
// guard page 数、初始尺寸公式、jslimit 余量、增长 clamp、栈池惰性收缩、zap 值、栈 id 序。
// 头侧状态机 (StackState) 已在 v8-wasm-core/src/stacks_consts.rs (stacks.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/stacks.cc");

// ---------- 1. guard page: 前后各一页, 分配 size + 2*page ----------
if (!src.includes("allocator->AllocatePages(\n      nullptr, size_ + 2 * page_size, allocator->AllocatePageSize(),\n      PageAllocator::kNoAccess);")) {
  // 允许换行差异, 再用宽松锚
  if (!/AllocatePages\(\s*nullptr, size_ \+ 2 \* page_size/.test(src)) {
    throw new Error("未找到双 guard page 分配");
  }
}
if (!/limit_ \+= page_size;/.test(src)) throw new Error("未找到前 guard page 跳过");

// ---------- 2. 初始尺寸: min(size_limit, jsStackSize + margin) * KB, 向上取整到页 ----------
if (!/std::min<size_t>\(\s*size_limit,\s*kJsStackSizeKB \+ StackMemory::JSGrowableStackLimitMarginKB\(\)\) \*\s*KB/.test(src)) {
  throw new Error("未找到初始尺寸公式");
}

// ---------- 3. jslimit 余量: 自有栈用可增长余量, libc 视图用全局余量 ----------
if (!src.includes("(owned_ ? StackMemory::JSGrowableStackLimitMarginKB() * KB")) {
  throw new Error("未找到 jslimit 余量分派");
}
if (!src.includes(": V8_STACK_LIMIT_MARGIN_KB * KB);")) {
  throw new Error("未找到 libc 视图余量");
}

// ---------- 4. 增长: new_size = clamp(2 * active.size, min_size, room_to_grow) ----------
if (!src.includes("std::clamp(2 * active_segment_->size_, min_size, room_to_grow)")) {
  throw new Error("未找到增长 clamp 公式");
}
if (!/room_to_grow = RoundDown\(size_limit - size_, page_size\);/.test(src)) {
  throw new Error("未找到可增长空间公式");
}

// ---------- 5. 过小 next segment 删除循环 ----------
if (!src.includes("// If the next segment is too small to fit the evicted frame, remove it.")) {
  throw new Error("未找到过小 next segment 删除语义");
}

// ---------- 6. 栈 id: 全局原子从 1 起; libc 视图固定 0 ----------
if (!/static std::atomic<int> next_id\(1\);\s*id_ = next_id\.fetch_add\(1\);/.test(src)) {
  throw new Error("未找到栈 id 原子计数");
}
if (!src.includes("id_ = 0;")) throw new Error("未找到 libc 视图 id=0");

// ---------- 7. DEBUG zap 值 ----------
const zap = src.match(/constexpr uint8_t kZapValue = (0x[0-9a-f]+);/);
if (!zap || zap[1] !== "0xab") throw new Error("未找到 zap 值 0xab");

// ---------- 8. 栈池: 惰性收缩到容量上限, 取栈时扣除 ----------
if (!/size_ >\s*static_cast<size_t>\(v8_flags\.wasm_stack_pool_capacity_mb\) \* MB/.test(src)) {
  throw new Error("未找到栈池容量收缩条件");
}
if (!src.includes("freelist_.pop_back();\n    size_ -= stack->allocated_size();")) {
  throw new Error("未找到取栈扣除容量");
}

// ---------- 9. Reset 语义锚 ----------
if (!src.includes("signature_id_ = CanonicalTypeIndex{kInvalidCanonicalIndex};")) {
  throw new Error("未找到 Reset 的签名 id 复位");
}

// ---------- 10. 追踪格式串 ----------
const traces = ["Delete stack #%d\\n", "Allocate stack #%d (limit: %p, base: %p, size: %zu)\\n", "Stack #%d reached the grow limit %zu bytes\\n", "Grow stack #%d by %zu bytes (limit: %p, base: %p)\\n", "Shrink stack #%d (limit: %p, base: %p)\\n"];
for (const t of traces) {
  if (!src.includes(`"${t}"`)) throw new Error(`追踪格式串缺失: ${t}`);
}

console.log(`stacks: guard=2 追踪格式=${traces.length} zap=0xab`);

// ---------- 生成 Rust ----------
const rust = `${banner(
  "src/wasm/stacks.cc.js",
  "src/wasm/stacks.cc",
  "栈内存分配/增长/收缩规则与栈池策略 (状态机在 stacks_consts.rs)",
)}
//! StackMemory 段分配与 StackPool 复用规则。

/// 每段前后各一页不可访问 guard page (共 2 页, 附加在段分配上)
/// v8: src/wasm/stacks.cc StackMemory::StackSegment::StackSegment
pub const GUARD_PAGES_PER_SEGMENT: usize = 2;

/// 初始栈尺寸公式: min(stack_size 上限, jsStackKB + marginKB) * KB, 再向上取整到页
/// v8: src/wasm/stacks.cc StackMemory::StackMemory()
pub fn initial_stack_size(stack_size_limit_kb: usize, js_stack_kb: usize, margin_kb: usize) -> usize {
    stack_size_limit_kb.min(js_stack_kb + margin_kb)
}

/// 增长段尺寸: clamp(2 * 当前段尺寸, 最小需求, 剩余可增长空间)
/// v8: src/wasm/stacks.cc StackMemory::Grow
pub fn grown_segment_size(current: usize, min_size: usize, room_to_grow: usize) -> usize {
    (2 * current).clamp(min_size, room_to_grow)
}

/// 剩余可增长空间: (上限 - 已用) 向下取整到页
/// v8: src/wasm/stacks.cc StackMemory::Grow
pub fn room_to_grow(size_limit: usize, used: usize, page_size: usize) -> usize {
    (size_limit - used) / page_size * page_size
}

/// jslimit 余量分派: 自有栈用可增长余量 (wasm_growable_stacks), libc 视图用全局余量
/// v8: src/wasm/stacks.cc StackMemory::jslimit
pub fn jslimit_margin_kb(owned: bool, js_growable_margin_kb: usize, global_margin_kb: usize) -> usize {
    if owned { js_growable_margin_kb } else { global_margin_kb }
}

/// DEBUG 模式取栈时填充的 zap 值
/// v8: src/wasm/stacks.cc StackPool::GetOrAllocate
pub const ZAP_VALUE: u8 = ${zap[1]};

/// 自有栈 id 从 1 起原子递增; libc 栈视图 id 恒 0
/// v8: src/wasm/stacks.cc StackMemory::StackMemory / StackMemory(limit, size)
pub const FIRST_OWNED_STACK_ID: i32 = 1;
pub const CENTRAL_STACK_VIEW_ID: i32 = 0;

/// 栈池收缩策略: 取栈时惰性释放, 直到总占用 <= wasm_stack_pool_capacity_mb * MB
/// v8: src/wasm/stacks.cc StackPool::GetOrAllocate
pub const POOL_SHRINK_STRATEGY: &str = "lazy: 释放到容量上限, 归还时只入队";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(GUARD_PAGES_PER_SEGMENT, 2);
    assert_eq!(initial_stack_size(984, 968, 16), 984);
    assert_eq!(initial_stack_size(500, 968, 16), 500);
    assert_eq!(grown_segment_size(100, 300, 1000), 300);
    assert_eq!(grown_segment_size(100, 50, 1000), 200);
    assert_eq!(grown_segment_size(600, 100, 1000), 1000);
    assert_eq!(room_to_grow(1024, 100, 128), 896);
    assert_eq!(ZAP_VALUE, 0xab);
    assert_eq!(FIRST_OWNED_STACK_ID, 1);
    assert_eq!(CENTRAL_STACK_VIEW_ID, 0);
};
`;

const p = writeSrc("v8-wasm-objects", "stacks_cc.rs", rust);
console.log(`written ${p}`);
