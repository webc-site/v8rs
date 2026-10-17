//! 原版 v8/src/base/compiler-specific.h — 编译器注解在 Rust 中由属性/编译器本身承担。
//!
//! 登记对应关系:
//! - `ALIGNAS(n)` → `#[repr(align(n))]`
//! - `V8_NOEXCEPT` → Rust 移动语义默认不抛出
//! - `V8_DEBUGGING_EXPORT` → GDB 可调函数用 `#[no_mangle] pub extern "C"`
//! - `PRINTF_FORMAT` → Rust 类型化格式化天然安全
//! - `MSVC_*` / `STATIC_CONST_MEMBER_DEFINITION` / `NON_EXPORTED_BASE` → 无对应需求
