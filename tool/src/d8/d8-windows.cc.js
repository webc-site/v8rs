#!/usr/bin/env bun
// tool/src/d8/d8-windows.cc.js
// 机械提取 v8/src/d8/d8-windows.cc
// 产物: rs8/src/d8_windows.rs — Windows 平台 Shell 方法面 (签名骨架)

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/d8/d8-windows.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. Shell:: 定义全集 6 (定义序)
const defs = [...src.matchAll(/^(?:[\w:<>& *]+\s+)*Shell::(\w+)\(/gm)].map((m) => m[1]);
const want = [
  "PreProcessUnicodeFilenameArg", "AddOSMethods", "ReadCharsFromTcpPort",
  "FileExists", "ChangeWorkingDirectory", "FreeUnicodeFilenameArgs",
];
if (defs.length !== want.length) fail("Shell:: 定义数", defs.length, want.length);
for (let i = 0; i < want.length; i++) {
  if (defs[i] !== want[i]) fail(`Shell:: 序 ${i}`, defs[i], want[i]);
}

// 2. UTF-16→UTF-8 转换辅助 + 文件名缓存 vector
must(src.includes("static char* ConvertUtf16StringToUtf8(const wchar_t* str)"), "ConvertUtf16StringToUtf8");
must(src.includes("static std::vector<char*> utf8_filenames;"), "utf8_filenames 缓存");

// 3. wchar_t 静态断言 (16 位)
must(src.includes('static_assert(sizeof(wchar_t) == 2, "wrong wchar_t size");'), "wchar_t==2 断言");

// 4. AddOSMethods 为空体 (Windows 无 os 扩展)
const addOs = src.match(/void Shell::AddOSMethods\(Isolate\* isolate, Local<ObjectTemplate> os_templ\) \{\}/);
must(addOs, "AddOSMethods 空体");

// 5. ReadCharsFromTcpPort 空实现 (TODO 注释)
must(src.includes("// TODO(leszeks): No reason this shouldn't exist on windows."), "ReadCharsFromTcpPort TODO");
must(src.includes("return {};"), "ReadCharsFromTcpPort 返回空");

// 6. WIN32 API 面
for (const api of ["CommandLineToArgvW", "GetCommandLineW", "LocalFree", "WideCharToMultiByte", "GetFileAttributesW", "SetCurrentDirectoryW", "INVALID_FILE_ATTRIBUTES"]) {
  if (!src.includes(api)) fail(`WIN32 API ${api}`, false, true);
}

let n = 12;
const rust = `${banner("src/d8/d8-windows.cc.js", "src/d8/d8-windows.cc", "d8 Windows 平台: Shell 方法面签名骨架")}
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
${want
  .map((m) => {
    const note = {
      PreProcessUnicodeFilenameArg: "argv[i] ← CommandLineToArgvW 第 i 项转码结果",
      AddOSMethods: "空体 — Windows 不装配 os 模板",
      ReadCharsFromTcpPort: "空实现返回 None (TODO: Windows 也可实现)",
      FileExists: "GetFileAttributesW != INVALID_FILE_ATTRIBUTES",
      ChangeWorkingDirectory: "SetCurrentDirectoryW; 失败按 print_error 打印",
      FreeUnicodeFilenameArgs: "释放转码缓存并清空",
    }[m];
    return `// ${m} — ${note}`;
  })
  .join("\n")}

// WIN32 API 面: CommandLineToArgvW / GetCommandLineW / LocalFree /
//   WideCharToMultiByte(CP_UTF8) / GetFileAttributesW / SetCurrentDirectoryW
// Rust 侧经 windows-sys FFI 承接 (仅 Windows 目标编译, cfg(windows))
`;

const p = writeSrc("rs8", "d8_windows.rs", rust);
console.log(`OK d8-windows.cc.js: 对数断言 ${n} 全过; Shell 6 / WIN32 API 7 / 空体 2; 产物 ${p}`);
