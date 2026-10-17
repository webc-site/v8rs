// 由 tool/src/common.js 从 v8/src/common/thread-local-storage.h 生成;
// 修订就地改, 复跑覆盖
//
// 修订 (Developer): TLS 模型宏展开为文档性常量 — 本 crate 为静态链接目标
// (非 COMPONENT_BUILD), V8_TLS_LIBRARY_MODE = 0。Rust thread_local! 由
// 调用方直接使用, 宏不落代码实体。

/// 原版 V8_TLS_LIBRARY_MODE — COMPONENT_BUILD/V8_TLS_USED_IN_LIBRARY 时为 1,
/// 其余 0; 静态链接目标 → 0
pub const V8_TLS_LIBRARY_MODE: u8 = 0;

/// 原版 V8_TLS_MODEL 展开表 (thread-local-storage.h:31-41, 分支序 1:1):
/// 1. 先判 library 模式 (V8_TLS_LIBRARY_MODE)  → "local-dynamic" (含 Windows
///    COMPONENT_BUILD)
/// 2. 非 library + Windows → "initial-exec"
/// 3. 非 library + Android  → "local-dynamic"
/// 4. 非 library + 其它 (Linux/macOS) → "local-exec"
///
/// V8_TLS_DECLARE_GETTER / V8_TLS_DEFINE_GETTER → Rust 直接 thread_local! +
/// 访问函数, library 模式差异由调用方 (L3+) 承担, 本 crate 不展开
/// (登记: 宏族无运行时实体, 以常量 + 注释承载)。
pub const V8_TLS_MODEL: &str = if V8_TLS_LIBRARY_MODE == 1 {
  "local-dynamic"
} else if cfg!(target_os = "windows") {
  "initial-exec"
} else if cfg!(target_os = "android") {
  "local-dynamic"
} else {
  "local-exec"
};
