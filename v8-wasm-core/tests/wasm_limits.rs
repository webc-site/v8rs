//! wasm-limits 常量对数 (与 v8/src/wasm/wasm-limits.h 逐个核对; 64 位平台分支)
use aok::{OK, Void};
use v8_wasm_core::wasm_limits::*;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 规格页上限
#[test]
fn spec_limits() -> Void {
  assert_eq!(K_SPEC_MAX_MEMORY32_PAGES, 65_536); // 4GB
  assert_eq!(K_SPEC_MAX_MEMORY64_PAGES, 262_144); // 16GB
  OK
}

/// V8 施加的限制 (百万族)
#[test]
fn v8_limits_millions() -> Void {
  assert_eq!(K_V8_MAX_WASM_TYPES, 1_000_000);
  assert_eq!(K_V8_MAX_WASM_DEFINED_FUNCTIONS, 1_000_000);
  assert_eq!(K_V8_MAX_WASM_IMPORTS, 1_000_000);
  assert_eq!(K_V8_MAX_WASM_EXPORTS, 1_000_000);
  assert_eq!(K_V8_MAX_WASM_GLOBALS, 1_000_000);
  assert_eq!(K_V8_MAX_WASM_TAGS, 1_000_000);
  assert_eq!(K_V8_MAX_WASM_STRING_LITERALS, 1_000_000);
  assert_eq!(K_V8_MAX_WASM_TOTAL_FUNCTIONS, 2_000_000); // defined + imports
  OK
}

/// 十万族与函数/表族
#[test]
fn v8_limits_others() -> Void {
  assert_eq!(K_V8_MAX_WASM_DATA_SEGMENTS, 100_000);
  assert_eq!(K_V8_MAX_WASM_TABLES, 100_000);
  assert_eq!(K_V8_MAX_WASM_MEMORIES, 100_000);
  assert_eq!(K_V8_MAX_WASM_MODULE_SIZE, 1024 * 1024 * 1024); // 1 GiB
  assert_eq!(K_V8_MAX_WASM_FUNCTION_SIZE, 7_654_321);
  assert_eq!(K_V8_MAX_WASM_FUNCTION_LOCALS, 50_000);
  assert_eq!(K_V8_MAX_WASM_FUNCTION_PARAMS, 1_000);
  assert_eq!(K_V8_MAX_WASM_FUNCTION_RETURNS, 1_000);
  assert_eq!(K_V8_MAX_WASM_FUNCTION_BR_TABLE_SIZE, 65_520);
  assert_eq!(K_V8_MAX_WASM_EFFECT_HANDLERS, 1_000);
  assert_eq!(K_V8_MAX_WASM_TABLE_SIZE, 10_000_000);
  assert_eq!(K_V8_MAX_WASM_TABLE_INIT_ENTRIES, 10_000_000);
  assert_eq!(K_V8_MAX_WASM_STRING_LENGTH, 100_000_000);
  assert_eq!(K_V8_MAX_WASM_STRUCT_FIELDS, 10_000);
  assert_eq!(K_V8_MAX_RTT_SUBTYPING_DEPTH, 63u32);
  assert_eq!(K_V8_MAX_WASM_ARRAY_NEW_FIXED_LENGTH, 10_000);
  assert_eq!(K_MAX_CONFIGURE_ALL_STATIC_PROPERTIES, 100_000);
  assert_eq!(K_MAX_CONFIGURE_ALL_METHODS, 100_000);
  OK
}

/// 内存页上限 (64 位平台: 完整规格上限)
#[test]
fn memory_page_limits() -> Void {
  // 64 位平台分支: kSystemPointerSize == 8
  assert_eq!(K_V8_MAX_WASM_MEMORY32_PAGES, 65_536); // 4 GiB
  assert_eq!(K_V8_MAX_WASM_MEMORY64_PAGES, 262_144); // 16 GiB
  // static_assert 族已在 src/wasm_limits.rs 编译期断言 (const assert)
  OK
}

/// 堆偏移与内存字节数上限
#[test]
fn heap_offset_limits() -> Void {
  // u32::MAX 基址 + u32::MAX 索引
  assert_eq!(K_WASM_MAX_HEAP_OFFSET, 0xFFFF_FFFFu64 + 0xFFFF_FFFFu64);
  // 262144 页 × 64KiB
  assert_eq!(K_MAX_MEMORY64_SIZE, 262_144 * 0x1_0000);
  OK
}
