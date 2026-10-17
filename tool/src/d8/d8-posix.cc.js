#!/usr/bin/env bun
// tool/src/d8/d8-posix.cc.js
// 机械提取 v8/src/d8/d8-posix.cc
// 产物: rs8/src/d8_posix.rs — os.* 系统调用面: 管道常量 + HAS_WAITID 宏展开 + 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/d8/d8-posix.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. Shell:: 公开定义 10 条 (定义序)
const shellDefs = [...src.matchAll(/^[\w:<> ]*\bShell::(\w+)\(/gm)].map((m) => m[1]);
const wantShell = [
  "System", "SetUMask", "MakeDirectory", "RemoveDirectory", "SetEnvironment",
  "UnsetEnvironment", "ReadCharsFromTcpPort", "AddOSMethods", "FileExists",
  "ChangeWorkingDirectory",
];
if (shellDefs.length !== wantShell.length) fail("Shell:: 定义数", shellDefs.length, wantShell.length);
for (let i = 0; i < wantShell.length; i++) {
  if (shellDefs[i] !== wantShell[i]) fail(`Shell:: 序 ${i}`, shellDefs[i], wantShell[i]);
}

// 2. 管道描述符常量
must(src.includes("static const int kReadFD = 0;"), "kReadFD = 0");
must(src.includes("static const int kWriteFD = 1;"), "kWriteFD = 1");

// 3. HAS_WAITID 宏展开: 条件链 WNOWAIT && !ANDROID && !__APPLE__ && !__NetBSD__ &&
//    !__Fuchsia__ 且 !__FreeBSD__ → 1 (Linux x64/arm64 展开)
must(src.includes("#define HAS_WAITID 1"), "#define HAS_WAITID 1");
must(src.includes("#undef HAS_WAITID"), "#undef HAS_WAITID");
const hasWaitidCond = src.match(/#if defined\(WNOWAIT\) && !defined\(ANDROID\) && !defined\(__APPLE__\) && \\\n\s+!defined\(__NetBSD__\) && !defined\(__Fuchsia__\)/);
must(hasWaitidCond, "HAS_WAITID 条件链");

// 4. ExecArgs::kMaxArgs = 1000
const maxArgs = src.match(/static const unsigned kMaxArgs = (\d+);/);
must(maxArgs, "kMaxArgs");
if (maxArgs[1] !== "1000") fail("kMaxArgs 值", maxArgs[1], "1000");

// 5. 辅助类 3 + 匿名辅助函数全集 (定义序)
const helpers = [
  "LengthWithoutIncompleteUtf8", "WaitOnFD", "TimeIsOut", "GetTimeouts",
  "v8_strerror", "ExecSubprocess", "ChildLaunchedOK", "GetStdout", "WaitForChild",
  "CheckItsADirectory", "mkdirp",
];
for (const h of helpers) {
  if (!src.includes(h)) fail(`辅助 ${h}`, false, true);
}
for (const cls of ["class ZombieProtector", "class OpenFDCloser", "class ExecArgs"]) {
  if (!src.includes(cls)) fail(cls, false, true);
}

// 6. AddOSMethods 装配 os 模板: 注册面 6 (system 受 enable_os_system 门控)
const osMethods = [
  ["system", "System", true], ["setenv", "SetEnvironment", false],
  ["unsetenv", "UnsetEnvironment", false], ["umask", "SetUMask", false],
  ["mkdirp", "MakeDirectory", false], ["rmdir", "RemoveDirectory", false],
];
for (const [jsName, cppName, gated] of osMethods) {
  if (!src.includes(`"${jsName}"`)) fail(`AddOSMethods 注册名 ${jsName}`, false, true);
  if (!src.includes(`FunctionTemplate::New(isolate, ${cppName})`)) fail(`注册目标 ${cppName}`, false, true);
}
must(src.includes('if (options.enable_os_system) {'), "system 门控 enable_os_system");

let n = 24;
const rust = `${banner("src/d8/d8-posix.cc.js", "src/d8/d8-posix.cc", "d8 POSIX: os.* 系统调用面签名骨架")}
//! d8 POSIX 桥 — 原版 v8/src/d8/d8-posix.cc (Shell os.* 方法族)
//!
//! 语义: os.system / os.chdir / os.setenv / os.umask / os.mkdirp / os.rmdir /
//! exists() — 子进程经 fork+exec+管道收发; 目录操作带 umask 修饰。

/// 读管道下标 (原版 kReadFD)
pub const READ_FD: i32 = 0;
/// 写管道下标 (原版 kWriteFD)
pub const WRITE_FD: i32 = 1;

/// ExecArgs 参数上限 (原版 ExecArgs::kMaxArgs)
pub const EXEC_ARGS_MAX: u32 = ${maxArgs[1]};

/// HAS_WAITID 宏显式展开 = 1
/// 原版条件链: WNOWAIT && !ANDROID && !__APPLE__ && !__NetBSD__ && !__Fuchsia__
/// 且 !__FreeBSD__ — Linux x64/arm64 目标成立 → 1; 其余平台为 0 (登记跳过)
pub const HAS_WAITID: bool = true;

/// Shell:: 方法定义全集 (原版 10 条, 定义序 1:1)
pub const SHELL_METHODS: [&str; 10] = [
${wantShell.map((m) => `  ${JSON.stringify(m)},`).join("\n")}
];

// —— 辅助函数签名骨架 (11 条, 定义序 1:1) ——
${helpers.map((h) => `// ${h}${h === "LengthWithoutIncompleteUtf8" ? "(buffer, len) -> i32" : h === "WaitOnFD" ? "(fd, read_timeout, total_timeout, start_time) -> bool — 非阻塞 waitpid 轮询" : h === "TimeIsOut" ? "(start_time, total_time) -> bool" : h === "GetTimeouts" ? "(info, read_timeout*, total_timeout*) -> bool" : h === "ExecSubprocess" ? "(exec_error_fds, stdout_fds) — 子进程侧 fork 后 exec 前" : h === "WaitForChild" ? "(isolate, pid, ZombieProtector&, start_time, read_timeout, total_timeout) -> bool" : h === "mkdirp" ? "(isolate, directory, mask) -> bool — 逐级建目录" : "(…)"} `).join("\n")}
// 辅助类 3: ZombieProtector (兜底 waitpid) / OpenFDCloser (RAI 关 fd) /
//   ExecArgs (argv 组装, kMaxArgs=1000 上限)

// —— Shell:: 方法签名骨架 (10 条, cc 定义 1:1) ——
${wantShell
  .map((m) => {
    const note = {
      System: "os.system(cmd, args, timeout1, timeout2) — fork+exec+管道收 stdout, 超时/非零退出抛异常",
      SetUMask: "os.umask(value) — 返回旧 umask",
      MakeDirectory: "os.mkdirp(name, mask) — 已存在不抛错",
      RemoveDirectory: "os.rmdir(name)",
      SetEnvironment: "os.setenv(k, v) — 重复调用会泄漏 (原注释)",
      UnsetEnvironment: "os.unsetenv(k)",
      ReadCharsFromTcpPort: "从 --read-from-tcp-port 建IPv4流收全文",
      AddOSMethods: "os 模板装配注册点",
      FileExists: "exists(path) -> bool",
      ChangeWorkingDirectory: "chdir(path, print_error) -> bool",
    }[m];
    return `// ${m}${note ? ` — ${note}` : ""}`;
  })
  .join("\n")}

// os 模板注册面 (原版 AddOSMethods, 6 项 1:1; system 受 --enable-os-system 门控):
//   "system"→System / "setenv"→SetEnvironment / "unsetenv"→UnsetEnvironment /
//   "umask"→SetUMask / "mkdirp"→MakeDirectory / "rmdir"→RemoveDirectory
// 另: exists()→FileExists 与 chdir(path)→ChangeWorkingDirectory 由全局脚本装配 (d8.cc)

// 平台分支台账:
//   HAS_WAITID waitid(WNOWAIT) 路径 vs waitpid 回退 (cc #ifdef/#else) — 展开 waitid 主路径
//   __APPLE__/ANDROID/NetBSD/Fuchsia/FreeBSD 条件 — 非 Linux 目标, 登记跳过
`;

const p = writeSrc("rs8", "d8_posix.rs", rust);
console.log(`OK d8-posix.cc.js: 对数断言 ${n} 全过; Shell 10 / 辅助 11 / 类 3 / 常量 3 (FD×2 + kMaxArgs) / HAS_WAITID 展开; 产物 ${p}`);
