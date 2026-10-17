// 由 tool/src/init/icu_util.js 从 v8/src/init/icu_util.{h,cc} 生成; 修订就地改, 复跑覆盖
// ICU 数据表初始化

/// ICU 快照数据装载模式 (原版 cc 内宏机械展开)
pub const ICU_UTIL_DATA_FILE: i32 = 0;
pub const ICU_UTIL_DATA_STATIC: i32 = 1;

/// ICU_UTIL_DATA_IMPL 构建期选择: 目标配置 (V8_INTL_SUPPORT + 文件模式) 展开为
/// ICU_UTIL_DATA_FILE(0); 静态链接模式取 ICU_UTIL_DATA_STATIC(1),
/// 本 crate 按 file 模式展开, 其他配置跳过并在此登记。
pub const ICU_UTIL_DATA_IMPL: i32 = 0;

// 公开 API 签名骨架 (原版 icu_util.h):
// - `bool InitializeICU(const char* icu_data_file)` — V8_WARN_UNUSED_RESULT
// - `bool InitializeICUDefaultLocation(const char* exec_path, const char* icu_data_file)` — V8_WARN_UNUSED_RESULT

// 平台分支登记 (原版 cc 内条件编译, 语义实现由 Developer 落):
// #if defined(_WIN32)
// #if defined(V8_INTL_SUPPORT)
// #if defined(V8_INTL_SUPPORT) && (ICU_UTIL_DATA_IMPL == ICU_UTIL_DATA_FILE)
// #if V8_OS_POSIX && !V8_OS_ZOS
// #if !defined(V8_INTL_SUPPORT)
// #if ICU_UTIL_DATA_IMPL == ICU_UTIL_DATA_FILE
// #if defined(V8_TARGET_LITTLE_ENDIAN)
// #if defined(V8_TARGET_BIG_ENDIAN)
// #if !defined(V8_INTL_SUPPORT)
// #if ICU_UTIL_DATA_IMPL == ICU_UTIL_DATA_STATIC
// #if ICU_UTIL_DATA_IMPL == ICU_UTIL_DATA_FILE
// #if V8_OS_POSIX && !V8_OS_ZOS

// 全局驻留 (POSIX 分支): g_icu_data_file (MemoryMappedFile) / 其他平台:
// g_icu_data_ptr — 由 Developer 按目标平台二选一。
