//! v8/src/wasm/wasm-limits.h 的 1:1 转写
//! 登记差异:
//! - 三元 (kSystemPointerSize == 4 ? .. : ..) 统一取 64 位平台分支。
//! - max_mem32_pages()/max_mem64_pages()/max_table_size()/max_module_size()
//!   及 max_mem32_bytes()/max_mem64_bytes() 定义于 wasm-engine.cc, 依赖
//!   v8_flags → 推迟至 flags/engine 梯队 (见 crate README)。
// 由 tool/src/wasm/wasm-limits.h.js 从 v8/src/wasm/wasm-limits.h 生成; 修订就地改, 复跑覆盖
// 纯常量表; 平台三元统一取 64 位分支 (kSystemPointerSize==8)
use crate::wasm_constants::WASM_PAGE_SIZE;

/// 规格内存页上限 (声明上限; 运行时仅可增长至此)
/// https://webassembly.github.io/spec/js-api/index.html#limits
pub const K_SPEC_MAX_MEMORY32_PAGES: usize = 65_536; // 4GB
pub const K_SPEC_MAX_MEMORY64_PAGES: usize = 262_144; // 16GB

// --- V8 施加的限制 (与其他引擎协商一致) ---
pub const K_V8_MAX_WASM_TYPES: usize = 1_000_000;
pub const K_V8_MAX_WASM_DEFINED_FUNCTIONS: usize = 1_000_000;
pub const K_V8_MAX_WASM_IMPORTS: usize = 1_000_000;
pub const K_V8_MAX_WASM_EXPORTS: usize = 1_000_000;
pub const K_V8_MAX_WASM_GLOBALS: usize = 1_000_000;
pub const K_V8_MAX_WASM_TAGS: usize = 1_000_000;
pub const K_V8_MAX_WASM_DATA_SEGMENTS: usize = 100_000;
pub const K_V8_MAX_WASM_MEMORY32_PAGES: usize = 65_536;
pub const K_V8_MAX_WASM_MEMORY64_PAGES: usize = 262_144;
pub const K_V8_MAX_WASM_MODULE_SIZE: usize = 1_073_741_824;
pub const K_V8_MAX_WASM_FUNCTION_SIZE: usize = 7_654_321;
pub const K_V8_MAX_WASM_FUNCTION_LOCALS: usize = 50_000;
pub const K_V8_MAX_WASM_FUNCTION_PARAMS: usize = 1_000;
pub const K_V8_MAX_WASM_FUNCTION_RETURNS: usize = 1_000;
pub const K_V8_MAX_WASM_FUNCTION_BR_TABLE_SIZE: usize = 65_520;
pub const K_V8_MAX_WASM_EFFECT_HANDLERS: usize = 1_000;
pub const K_V8_MAX_WASM_TABLE_SIZE: usize = 10_000_000;
pub const K_V8_MAX_WASM_TABLE_INIT_ENTRIES: usize = 10_000_000;
pub const K_V8_MAX_WASM_TABLES: usize = 100_000;
pub const K_V8_MAX_WASM_MEMORIES: usize = 100_000;
pub const K_V8_MAX_WASM_STRUCT_FIELDS: usize = 10_000;
pub const K_V8_MAX_RTT_SUBTYPING_DEPTH: u32 = 63;
pub const K_V8_MAX_WASM_ARRAY_NEW_FIXED_LENGTH: usize = 10_000;
pub const K_MAX_CONFIGURE_ALL_STATIC_PROPERTIES: usize = 100_000;
pub const K_MAX_CONFIGURE_ALL_METHODS: usize = 100_000;
pub const K_V8_MAX_WASM_STRING_LITERALS: usize = 1_000_000;
pub const K_V8_MAX_WASM_STRING_LENGTH: usize = 100_000_000;
pub const K_WASM_MAX_HEAP_OFFSET: u64 = 8_589_934_590;
pub const K_V8_MAX_WASM_TOTAL_FUNCTIONS: usize = 2_000_000;
pub const K_MAX_MEMORY64_SIZE: usize = 17_179_869_184;

// static_assert (wasm-limits.h:81-89, 108): 64 位平台支持完整规格内存上限
const _: () = assert!(
  K_V8_MAX_WASM_TABLE_SIZE <= 4_294_967_295,
  "v8 should not exceed WebAssembly's non-web embedding limits"
);
const _: () = assert!(
  K_V8_MAX_WASM_TABLE_INIT_ENTRIES <= K_V8_MAX_WASM_TABLE_SIZE,
  "JS-API should not exceed v8's limit"
);
const _: () = assert!(K_SPEC_MAX_MEMORY32_PAGES == 65_536 && K_SPEC_MAX_MEMORY64_PAGES == 262_144);
const _: () = assert!(K_V8_MAX_WASM_MEMORY64_PAGES <= u32::MAX as usize);
const _: () = assert!(K_MAX_MEMORY64_SIZE == K_V8_MAX_WASM_MEMORY64_PAGES * WASM_PAGE_SIZE);
