// 由 tool/src/d8/d8-windows.cc.js 从 v8/src/d8/d8-windows.cc 生成; 修订就地改, 复跑覆盖
// d8 Windows 平台: Shell 方法面签名骨架

//! d8 Windows 桥 — 原版 v8/src/d8/d8-windows.cc (Shell 方法 Windows 变体)
//!
//! 语义: UTF-16 命令行参数转 UTF-8 (进程退出前持有); os 方法面为空;
//! exists/chdir 经 W 系 API。

/// wchar_t 宽度约束 (原版 static_assert sizeof(wchar_t) == 2)
pub const WCHAR_T_SIZE: usize = 2;

/// UTF-8 文件名缓存说明 (原版 static vector<char*> utf8_filenames —
/// 分配的转码文件名持有至进程退出, 由 FreeUnicodeFilenameArgs 统一释放)
pub const OWNS_UTF8_FILENAMES: bool = true;

// —— Shell:: 签名骨架 (6 条, cc 定义 1:1) ——
// PreProcessUnicodeFilenameArg — argv[i] ← CommandLineToArgvW 第 i 项转码结果
// AddOSMethods — 空体 — Windows 不装配 os 模板
// ReadCharsFromTcpPort — 空实现返回 None (TODO: Windows 也可实现)
// FileExists — GetFileAttributesW != INVALID_FILE_ATTRIBUTES
// ChangeWorkingDirectory — SetCurrentDirectoryW; 失败按 print_error 打印
// FreeUnicodeFilenameArgs — 释放转码缓存并清空

// WIN32 API 面: CommandLineToArgvW / GetCommandLineW / LocalFree /
//   WideCharToMultiByte(CP_UTF8) / GetFileAttributesW / SetCurrentDirectoryW
// Rust 侧经 windows-sys FFI 承接 (仅 Windows 目标编译, cfg(windows))
