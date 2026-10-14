[English](#en) | [中文](#zh)

---

<a name="en"></a>

#

---

<a name="zh"></a>

#

- [系统杂项族 (owner: base-sys2)](#系统杂项族-owner-base-sys2)
  - [logging (base/logging.cc/h) 与 log crate 的映射](#logging-baseloggingcch-与-log-crate-的映射)
  - [其它模块映射要点](#其它模块映射要点)
  - [依赖缺口登记](#依赖缺口登记)
- [A2 数值族 (owner: base-num)](#a2-数值族-owner-base-num)
  - [已核对要点](#已核对要点)
  - [依赖缺口登记](#依赖缺口登记)

## 系统杂项族 (owner: base-sys2)

对应原版: `v8/src/base/{file-utils,sys-info,string-format,timezone-cache,logging,debug/,strings,types,cpu,abort-mode,immediate-crash,build_config,compiler-specific,sanitizer/,vlq,vlq-base64,strings}`.

### logging (base/logging.cc/h) 与 log crate 的映射

红线: 日志统一 `log` crate, 禁自造日志门面. v8-base 目前 Cargo.toml 未列 `log` (禁 cargo add), 因此 base 层 fatal/dcheck 输出**直接落 stderr** (等价原版 `OS::PrintError`), 上层 `v8-logging` 通过本模块暴露的三个钩子接入 log crate:

| 原版符号 | Rust 位置 | 与 log crate 的桥接路径 |
|---|---|---|
| `SetPrintStackTrace(void (*)())` | `logging::set_print_stack_trace` | v8-logging 注册 `log::warn!` 版栈打印 |
| `SetDcheckFunction(...)` | `logging::set_dcheck_function` | 桥接到 `log::warn!(target: "dcheck", ...)` |
| `SetFatalFunction(...)` | `logging::set_fatal_function` | 桥接到 `log::error!(target: "fatal", ...)` |
| `V8_Dcheck(msg)` | `logging::v8_dcheck` | 走 dcheck_function, 默认 stderr + abort |
| `V8_Fatal(fmt, ...)` | `logging::v8_fatal` (noreturn) | 走 fatal_function, 末尾 `std::process::abort` |
| `FatalOOM(type, msg)` | `logging::fatal_oom` | 由 v8-logging 用 `log::error!` 打印后再 abort |
| `FatalNoSecurityImpact(fmt, ...)` | `logging::fatal_no_security_impact` | 同上, 允许 ExitProcess(-1) 分支 |
| 匿名 `PrettyPrintChar` | `logging::pretty_print_char` | 导出供测试 1:1 转写 `logging-unittest.cc::CheckChars` |

对应**任务提示中的 `V8BaseLog`/`Flush`/`SetLogMetaData`/`OutputDebugString`** — 这四项源自 `v8/src/logging/log.cc/h` (非 base 层), 归 `v8-logging` crate; base 层无对应实现, 由上表钩子承接, `Flush`/`OutputDebugString` 由 v8-logging 依 `log::logger().flush()` / Windows OutputDebugStringA 落地.

### 其它模块映射要点

- **strings (base/strings.h/cc)** — 不重复实现格式化引擎, 用 `core::fmt::Arguments` + `format_args!` 承接 `VSNPrintF/SNPrintF`; 截断/缓冲区上限/返回值语义与 `platform-posix.cc:1102-1116` 逐条对齐. `StrNCpy`/`HexValue`/`HexCharOfValue` 保留原版逐字节行为. 原版 `base/strings` 不含 `ReadLine/SplitAt/SStringViewToCString`, 该组符号或属 v8-base-platform 的 `OS::` 侧, 或属上层 `v8-strings`, 本模块不实现.
- **file_utils (base/file-utils.cc/h)** — 原版仅 `RelativePath`; `ReadFile`/`WriteFile`/`FileExists`/`GetExecutablePath`/`FileSystemInfo`/`Time` 依任务补齐, 使用 `std::fs` + libc (`_NSGetExecutablePath`, `time`); 错误一律 `Result<T, FileUtilsError>`, 禁 panic/unwrap.
- **sys_info (base/sys-info.cc/h)** — macOS 走 `libc::sysctl(CTL_HW, HW_MEMSIZE)`, Linux 走 `sysconf(_SC_PHYS_PAGES)*_SC_PAGESIZE`; `NumberOfProcessors`/`AmountOfVirtualMemory` 双平台 sysconf+getrlimit; `AddressSpaceEnd` POSIX 恒 `usize::MAX`. 只覆盖 macOS + Linux; Windows/OpenBSD/QNX/AIX/ZOS/Starboard 后置.
- **string_format (base/string-format.h)** — `FormattedString` 的编译期静态拼接由 Rust `format_args!` 天然承接; 零分配栈写入提供 [`StackWriter<N>`] + [`formatted_to_array<N>`], 语义等价原版 `PrintFormattedStringToArray`. `JSONEscaped` 逐字符 1:1 转写.
- **timezone_cache (base/timezone-cache.h)** — 抽象 `TimezoneCache` trait + `TimeZoneDetection` 枚举严格对应; `LocalTimezoneCache` 用 libc `localtime_r`/`gmtime_r` + `tzset`/`tzname`/`timezone`  extern 声明实现, **禁引 chrono/jiff 等第三方** (引擎 Date 语义绑定 v8-date).
- **debug (base/debug/stack_trace.h/cc + _posix.cc)** — 落地 `StackTrace` 结构 + `capture` + `addresses` + `to_string` + `print`, 使用 libc `backtrace`/`backtrace_symbols`; **推迟**: `EnableInProcessStackDumping` / `DisableSignalStackDump` (依赖 sigaction/sigaltstack, 归 platform 层), Windows/Android/Fuchsia/ZOS 分支.
- **vlq / vlq_base64** — 编解码边界 (u32 溢出/截断输入/INT_MIN 哨兵) 逐条与 `vlq.h:20-111`、`vlq-base64.cc:37-56` 对齐, source-map 语义基石, 保留 `char_to_digit_decode_for_testing`.

### 依赖缺口登记

- libc crate 未直接导出 `backtrace_symbols_free` — debug.rs 采用 `libc::free` 手动逐元素 + 数组释放.
- libc crate 未直接导出 `tzset`/`tzname`/`timezone` unix 静态 — timezone_cache.rs 用 `unsafe extern "C"` 块补齐 (POSIX 保证).
- base 层若需实际接入 `log` crate, 待 crate 拥有者 `cargo add log` 后由 v8-logging 桥接, 见上表.

## A2 数值族 (owner: base-num)

对应原版: `v8/src/base/numerics/*`, `v8/src/base/numbers/*`, `division-by-constant.{cc,h}`, `ieee754.{cc,h}`, `float16.h`, `fpu.{cc,h}`. 覆盖 safe/checked/clamped/wrapping 运算族、安全转换、角度/字节转换、ranges; Bignum/DiyFp/Double/cached-powers/dtoa/fast-dtoa/fixed-dtoa/bignum-dtoa/strtod 的 ECMAScript 双精度↔十进制族; 常数除法魔数; ieee754 位级访问与自含超越函数; float16 RNE 转换; fpu FTZ/DAZ.

红线: 数值语义逐字节等价, 禁把静默截断当正确. 与 `v8-numbers` 的分工待 M1 定案 (todo 将 dtoa 主方案置于 L1 `v8-numbers` zmij 内核; 本 crate 仅 1:1 转写 base/numbers 原版, 勿删勿挪).

### 已核对要点

- **ieee754** — `Double↔Bits/SignBit/Exponent/Mantissa/NaN` 位级访问逐位等价; `acosh/asinh/atanh/cosh/sinh` 为 .cc 自含算法 1:1 转写 (含 `get/set_high_word`、wrapping 取负、INF/NaN/±0 分支). **推迟**: `acos/asin/atan/atan2/cos/sin/tan/exp/log/log1p/log2/log10/expm1/cbrt/tanh/pow` 原版委托 third_party/llvm-libc `shared::` 内核, 该第三方源码未随仓库分发, 当前以平台 libm 承接, 待 shared math 内核 1:1 转写后逐条核对 unittests 金标准.
- **division_by_constant** — `signed/unsigned_division_by_constant` 全 wrapping 位级等价, 除零入参以 0 兜底避免 panic; golden 魔数 (Hacker's Delight 表 10-1) 见 `tests/division_by_constant.rs`.
- **bignum → strings 耦合** — `hex_char_value` 复用 `crate::strings::hex_char_of_value` (base-sys2 owner 的 `pub mod strings`), 语义一致但存在跨族耦合, 记此登记.

### 依赖缺口登记

- Gay `dtoa` 金标准 (`gay-shortest-*`/`gay-fixed-*`) 属 L1 `v8-numbers` (zmij) 验证范围, 本轮 base/numbers 不引入.
- 集成测试 (`tests/division_by_constant.rs`/`ieee754.rs`/`float16.rs`/`diy_fp.rs`/`numerics.rs`) 已按原版转写, 但 v8-base lib 当前因其它族 (atomic_utils/once/bits/enum_set/small_map/unique_array/contextual/lazy_instance 等并发代理文件) 未编译通过而无法运行 `cargo nextest`, 待全 crate 绿灯后执行.

