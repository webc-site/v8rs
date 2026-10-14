// 由 tool/src/wasm/wasm-debug.cc.js 从 v8/src/wasm/wasm-debug.cc 生成; 修订就地改, 复跑覆盖
// 断点 flooding 与调试代码缓存规则 (骨架在 wasm_debug_consts.rs)

//! 断点/步进的调试代码管理规则。

/// flooding 哨兵: 偏移 0 是无效偏移, 用于表示"泛洪"模式
/// v8: src/wasm/wasm-debug.cc FloodWithBreakpoints
pub const K_FLOODING_BREAKPOINTS: &[i32] = &[0];

/// 调试代码 LRU 缓存上限 (最近使用在前; 步进每暂停清一次、下一步重装)
/// v8: src/wasm/wasm-debug.cc kMaxCachedDebuggingCode
pub const K_MAX_CACHED_DEBUGGING_CODE: usize = 3;

/// 缓存条目字段 (LRU 淘汰)
/// v8: src/wasm/wasm-debug.cc CachedDebuggingCode
#[rustfmt::skip]
pub static CACHED_DEBUGGING_CODE_FIELDS: &[&str] = &[
    "func_index",
    "breakpoint_offsets",
    "dead_breakpoints",
    "code",
];

/// flooding 仅支持 liftoff 代码; jitless (解释器) 无断点
/// v8: src/wasm/wasm-debug.cc FloodWithBreakpoints
pub const FLOODING_REQUIRES_LIFTOFF: bool = true;
pub const JITLESS_NO_BREAKPOINTS: bool = true;

/// 步进以帧 id 登记 (stepping_frame), 断点重编译后更新返回地址
/// v8: src/wasm/wasm-debug.cc FloodWithBreakpoints / ReturnLocation
pub const STEPPING_FRAME_SEMANTICS: &str = "帧 id 登记 + 返回地址更新";

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(K_FLOODING_BREAKPOINTS, &[0]);
    assert_eq!(K_MAX_CACHED_DEBUGGING_CODE, 3);
    assert_eq!(CACHED_DEBUGGING_CODE_FIELDS.len(), 4);
};
