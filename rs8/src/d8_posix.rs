// 由 tool/src/d8/d8-posix.cc.js 从 v8/src/d8/d8-posix.cc 生成; 修订就地改, 复跑覆盖
// d8 POSIX: os.* 系统调用面签名骨架

//! d8 POSIX 桥 — 原版 v8/src/d8/d8-posix.cc (Shell os.* 方法族)
//!
//! 语义: os.system / os.chdir / os.setenv / os.umask / os.mkdirp / os.rmdir /
//! exists() — 子进程经 fork+exec+管道收发; 目录操作带 umask 修饰。

/// 读管道下标 (原版 kReadFD)
pub const READ_FD: i32 = 0;
/// 写管道下标 (原版 kWriteFD)
pub const WRITE_FD: i32 = 1;

/// ExecArgs 参数上限 (原版 ExecArgs::kMaxArgs)
pub const EXEC_ARGS_MAX: u32 = 1000;

/// HAS_WAITID 宏显式展开 = 1
/// 原版条件链: WNOWAIT && !ANDROID && !__APPLE__ && !__NetBSD__ && !__Fuchsia__
/// 且 !__FreeBSD__ — Linux x64/arm64 目标成立 → 1; 其余平台为 0 (登记跳过)
pub const HAS_WAITID: bool = true;

/// Shell:: 方法定义全集 (原版 10 条, 定义序 1:1)
pub const SHELL_METHODS: [&str; 10] = [
  "System",
  "SetUMask",
  "MakeDirectory",
  "RemoveDirectory",
  "SetEnvironment",
  "UnsetEnvironment",
  "ReadCharsFromTcpPort",
  "AddOSMethods",
  "FileExists",
  "ChangeWorkingDirectory",
];

// —— 辅助函数签名骨架 (11 条, 定义序 1:1) ——
// LengthWithoutIncompleteUtf8(buffer, len) -> i32
// WaitOnFD(fd, read_timeout, total_timeout, start_time) -> bool — 非阻塞 waitpid 轮询
// TimeIsOut(start_time, total_time) -> bool
// GetTimeouts(info, read_timeout*, total_timeout*) -> bool
// v8_strerror(…)
// ExecSubprocess(exec_error_fds, stdout_fds) — 子进程侧 fork 后 exec 前
// ChildLaunchedOK(…)
// GetStdout(…)
// WaitForChild(isolate, pid, ZombieProtector&, start_time, read_timeout, total_timeout) -> bool
// CheckItsADirectory(…)
// mkdirp(isolate, directory, mask) -> bool — 逐级建目录
// 辅助类 3: ZombieProtector (兜底 waitpid) / OpenFDCloser (RAI 关 fd) /
//   ExecArgs (argv 组装, kMaxArgs=1000 上限)

// —— Shell:: 方法签名骨架 (10 条, cc 定义 1:1) ——
// System — os.system(cmd, args, timeout1, timeout2) — fork+exec+管道收 stdout, 超时/非零退出抛异常
// SetUMask — os.umask(value) — 返回旧 umask
// MakeDirectory — os.mkdirp(name, mask) — 已存在不抛错
// RemoveDirectory — os.rmdir(name)
// SetEnvironment — os.setenv(k, v) — 重复调用会泄漏 (原注释)
// UnsetEnvironment — os.unsetenv(k)
// ReadCharsFromTcpPort — 从 --read-from-tcp-port 建IPv4流收全文
// AddOSMethods — os 模板装配注册点
// FileExists — exists(path) -> bool
// ChangeWorkingDirectory — chdir(path, print_error) -> bool

// os 模板注册面 (原版 AddOSMethods, 6 项 1:1; system 受 --enable-os-system 门控):
//   "system"→System / "setenv"→SetEnvironment / "unsetenv"→UnsetEnvironment /
//   "umask"→SetUMask / "mkdirp"→MakeDirectory / "rmdir"→RemoveDirectory
// 另: exists()→FileExists 与 chdir(path)→ChangeWorkingDirectory 由全局脚本装配 (d8.cc)

// 平台分支台账:
//   HAS_WAITID waitid(WNOWAIT) 路径 vs waitpid 回退 (cc #ifdef/#else) — 展开 waitid 主路径
//   __APPLE__/ANDROID/NetBSD/Fuchsia/FreeBSD 条件 — 非 Linux 目标, 登记跳过
