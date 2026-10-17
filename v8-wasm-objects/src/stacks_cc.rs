// 由 tool/src/wasm/stacks.cc.js 从 v8/src/wasm/stacks.cc 生成; 修订就地改, 复跑覆盖
// 栈内存分配/增长/收缩规则与栈池策略 (状态机在 stacks_consts.rs)

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
pub const ZAP_VALUE: u8 = 0xab;

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
