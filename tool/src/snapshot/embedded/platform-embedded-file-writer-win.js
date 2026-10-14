#!/usr/bin/env bun
// tool/src/snapshot/embedded/platform-embedded-file-writer-win.js
// 机械提取 v8/src/snapshot/embedded/platform-embedded-file-writer-win.{h,cc}:
// COFF 风格指令表 + unwind 数据发射

import { readV8, writeSrc, banner } from "../../../lib.js";

const hSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-win.h");
const ccSrc = readV8("src/snapshot/embedded/platform-embedded-file-writer-win.cc");

// 1. SYMBOL_PREFIX 双分支 (Win64 → "_", 其余 "")
// 32 位架构符号加下划线; 目标 x64+arm64 → "" (机械展开 #if !x64 && !arm64)
const prefixM = ccSrc.match(/#if !defined\(V8_TARGET_ARCH_X64\) && !defined\(V8_TARGET_ARCH_ARM64\)\s*\n#define SYMBOL_PREFIX "([^"]*)"\s*\n#else\s*\n#define SYMBOL_PREFIX "([^"]*)"/);
if (!prefixM) throw new Error("未找到 SYMBOL_PREFIX 双分支");
const prefixUnderscore = prefixM[1];
const prefixTarget = prefixM[2];

// 2. DirectiveAsString 三分支 (V8_COMPILER_IS_MSVC × target arch / 非 MSVC):
//    MSVC 非 Arm64 (BYTE/DWORD/QWORD), MSVC Arm64 (DCB/DCDU/DCQU), 非 MSVC (.byte/...)
const dirFn = ccSrc.match(
  /PlatformEmbeddedFileWriterWin::DirectiveAsString\(\s*DataDirective directive\) \{([\s\S]*?)\n#endif\n\}/,
);
if (!dirFn) throw new Error("未找到 DirectiveAsString");
const msvcX86 = [...dirFn[1].matchAll(/case k(\w+):\s*return "(BYTE|DWORD|QWORD)";/g)].map(
  (m) => ({ name: m[1], directive: m[2] }),
);
const msvcArm = [...dirFn[1].matchAll(/case k(\w+):\s*return "(DCB|DCDU|DCQU)";/g)].map(
  (m) => ({ name: m[1], directive: m[2] }),
);
const gasTable = [...dirFn[1].matchAll(/case k(\w+):\s*return "(\.\w+)";/g)].map(
  (m) => ({ name: m[1], directive: m[2] }),
);
if (msvcX86.length !== 3 || msvcArm.length !== 3 || gasTable.length !== 4) {
  throw new Error(`指令表条数不符: msvcX86 ${msvcX86.length}/3, msvcArm ${msvcArm.length}/3, gas ${gasTable.length}/4`);
}

// 3. unwind 发射链
const unwindM = ccSrc.includes("CanEmitUnwindInfoForBuiltins()");
const winArm64M = ccSrc.includes("#if defined(V8_OS_WIN_ARM64)");
if (!unwindM || !winArm64M) throw new Error("unwind 分支登记异常");

// 4. 实现方法面
const methods = [...new Set([...ccSrc.matchAll(/PlatformEmbeddedFileWriterWin::(\w+)\(/g)].map((m) => m[1]))];
if (methods.length !== 26) {
  throw new Error(`Win 实现方法数不符: 期望 26, 实得 ${methods.length}`);
}
console.log(`双前缀 + msvcX86 ${msvcX86.length}/msvcArm ${msvcArm.length}/gas ${gasTable.length} + 方法 ${methods.length}`);

let rust = banner(
  "src/snapshot/embedded/platform-embedded-file-writer-win.js",
  "src/snapshot/embedded/platform-embedded-file-writer-win.{h,cc}",
  "COFF 风格平台写入器 (Windows)",
);
rust += `
/// PlatformEmbeddedFileWriterWin (COFF 风格)。
/// 符号前缀 (原版 SYMBOL_PREFIX 宏, 双分支机械展开):
/// - 32 位架构 (非 x64/arm64) → "${prefixUnderscore}" (符号加下划线)
/// - 目标 x64 + arm64 → "${prefixTarget}"
/// 汇编指令表 (原版 DirectiveAsString, V8_COMPILER_IS_MSVC 三分支):
/// - MSVC 非 Arm64: ${msvcX86.map((d) => `k${d.name} → \`${d.directive}\``).join(", ")} (kOcta UNREACHABLE)
/// - MSVC Arm64: ${msvcArm.map((d) => `k${d.name} → \`${d.directive}\``).join(", ")} (kOcta UNREACHABLE)
/// - 非 MSVC: ${gasTable.map((d) => `k${d.name} → \`${d.directive}\``).join(", ")}
/// unwind 登记: MaybeEmitUnwindData 在 V8_OS_WIN64 且 CanEmitUnwindInfoForBuiltins()
/// 时发射 x64 unwind 数据; V8_OS_WIN_ARM64 下仅目标 arch 为 Arm64 时处理
/// (交叉构建宿主 unwind 忽略, 仅构建期使用)。
/// 实现方法面 (共 ${methods.length} 项): ${methods.join(", ")}
`;

const p = writeSrc("v8-snapshot", "platform_embedded_file_writer_win.rs", rust);
console.log(`written ${p}`);
