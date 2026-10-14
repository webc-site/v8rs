#!/usr/bin/env bun
// tool/src/libsampler/sampler.js — 库级采样器组
// 覆盖: src/libsampler/sampler.h + src/libsampler/sampler.cc (1 族, 2 文件)
// 产出: v8-profiler/src/sampler.rs (规则代码: 帧上限常量 + USE_SIGNALS 展开 +
//       SIGPROF/sigaction 标志 + x64/arm64 寄存器提取索引 + RegisterState/原子互斥
//       类型别名 + Sampler/AtomicGuard/SamplerManager/SignalHandler 签名骨架;
//       无冲突故用 sampler.rs, 冲突时改 sampler_generated.rs)
// 复跑幂等: 纯文本扫描 + 源码序输出, 无时间戳。
import { banner, readV8, writeSrc } from "../../lib.js";

const GEN = "src/libsampler/sampler.js";
const H = "src/libsampler/sampler.h";
const CC = "src/libsampler/sampler.cc";
const FILES = [H, CC];
const hSrc = readV8(H);
const ccSrc = readV8(CC);
const txt = { [H]: hSrc, [CC]: ccSrc };
const die = (m) => { throw new Error(`[sampler.js] ${m}`); };
const need = (f, s) => { if (!txt[f].includes(s)) die(`${f} 缺针 ${s}`); };

// ① Sampler 类面针 (sampler.h)
need(H, "class V8_EXPORT_PRIVATE Sampler {");
need(H, "static const int kMaxFramesCountLog2 = 8;");
need(H, "static const unsigned kMaxFramesCount = (1u << kMaxFramesCountLog2) - 1;");
need(H, "virtual void SampleStack(const v8::RegisterState& regs) = 0;");
need(H, "bool ShouldRecordSample() {");
need(H, "unsigned js_sample_count() const { return js_sample_count_; }");
need(H, "void StartCountingSamples() {");
need(H, "#ifdef USE_SIGNALS");
need(H, "using AtomicMutex = std::atomic_bool;");
need(H, "class V8_EXPORT_PRIVATE V8_NODISCARD AtomicGuard {");
need(H, "explicit AtomicGuard(AtomicMutex* atomic, bool is_blocking = true);");
need(H, "class V8_EXPORT_PRIVATE SamplerManager {");
need(H, "using SamplerList = std::vector<Sampler*>;");
need(H, "void AddSampler(Sampler* sampler);");
need(H, "void RemoveSampler(Sampler* sampler);");
need(H, "void DoSample(const v8::RegisterState& state);");
need(H, "static SamplerManager* instance();");
need(H, "std::unordered_map<int, SamplerList> sampler_map_;");
need(H, "AtomicMutex samplers_access_counter_{false};");
// USE_SIGNALS 定义条件 (sampler.h:16-18)
const useSignalsCond = "V8_OS_POSIX && !V8_OS_CYGWIN && !V8_OS_FUCHSIA";
need(H, `#if ${useSignalsCond}`);

// ② 信号路径针 (sampler.cc)
need(CC, "void SignalHandler::FillRegisterState(void* context, RegisterState* state) {");
need(CC, "sa.sa_flags = SA_RESTART | SA_SIGINFO | SA_ONSTACK;");
need(CC, "sigaction(SIGPROF, &sa, &old_signal_handler_) == 0");
need(CC, "if (signal != SIGPROF) return;");
need(CC, "pthread_kill(platform_data()->vm_tself(), SIGPROF);");
need(CC, "int SignalHandler::client_count_ = 0;");
need(CC, "SamplerManager* SamplerManager::instance() {");
need(CC, "static base::LeakyObject<SamplerManager> instance;");
need(CC, "atomic->compare_exchange_strong(expected, true)");

// ③ 机械提取: 帧上限常量 (h:36-37)
const log2s = [...hSrc.matchAll(/kMaxFramesCountLog2\s*=\s*(\d+)\s*;/g)];
if (log2s.length !== 1) die(`kMaxFramesCountLog2 声明数=${log2s.length}, 期望 1`);
const maxFramesLog2 = parseInt(log2s[0][1], 10);
const maxFramesCount = (1 << maxFramesLog2) - 1; // 展开 (1u << 8) - 1
if (maxFramesLog2 !== 8) die(`kMaxFramesCountLog2=${maxFramesLog2}, 期望 8`);
if (maxFramesCount !== 255) die(`kMaxFramesCount 展开值=${maxFramesCount}, 期望 255`);

// ④ 机械提取: FillRegisterState 平台分支清单 (展开 x64+arm64, 其余登记跳过)
const fillIdx = ccSrc.indexOf("void SignalHandler::FillRegisterState(void* context, RegisterState* state) {");
if (fillIdx < 0) die("FillRegisterState 未定位");
const fillEndIdx = ccSrc.indexOf("#endif  // USE_SIGNALS", fillIdx);
if (fillEndIdx < 0) die("FillRegisterState 结束 #endif 未定位");
const fillBody = ccSrc.slice(fillIdx, fillEndIdx);

// 架构分支 (V8_HOST_ARCH_*): 期望 10 种, 展开 X64/ARM64
const archSet = [...new Set([...fillBody.matchAll(/V8_HOST_ARCH_(\w+)/g)].map((m) => m[1]))].sort();
if (archSet.length !== 10) die(`架构分支数=${archSet.length} (${archSet}), 期望 10`);
for (const a of ["X64", "ARM64"]) if (!archSet.includes(a)) die(`架构分支缺 ${a}`);
const skippedArches = archSet.filter((a) => a !== "X64" && a !== "ARM64");

// 系统分支 (V8_OS_*): 期望 10 种, 展开 LINUX/DARWIN
const osSet = [...new Set([...fillBody.matchAll(/V8_OS_(\w+)/g)].map((m) => m[1]))].sort();
if (osSet.length !== 10) die(`系统分支数=${osSet.length} (${osSet}), 期望 10`);
for (const o of ["LINUX", "DARWIN"]) if (!osSet.includes(o)) die(`系统分支缺 ${o}`);
const skippedOses = osSet.filter((o) => o !== "LINUX" && o !== "DARWIN");

// ⑤ 机械提取: linux-x64 gregs[] 索引 (取自 cc:179 android-x64 枚举, glibc ucontext 同值)
const regEnum = ccSrc.match(/enum \{ REG_RBP = (\d+), REG_RSP = (\d+), REG_RIP = (\d+) \};/);
if (!regEnum) die("REG_RBP/RSP/RIP 枚举未找到");
const REG_RBP = parseInt(regEnum[1], 10);
const REG_RSP = parseInt(regEnum[2], 10);
const REG_RIP = parseInt(regEnum[3], 10);
if (REG_RBP !== 10 || REG_RSP !== 15 || REG_RIP !== 16) {
  die(`gregs 索引失配: RBP=${REG_RBP} RSP=${REG_RSP} RIP=${REG_RIP}, 期望 10/15/16`);
}

// ⑥ 机械提取: linux-arm64 regs[] 索引 (cc:439-444)
const arm64Fp = ccSrc.match(/state->fp = reinterpret_cast<void\*>\(mcontext\.regs\[(\d+)\]\);/);
const arm64Lr = ccSrc.match(/state->lr = reinterpret_cast<void\*>\(mcontext\.regs\[(\d+)\]\);/);
if (!arm64Fp || !arm64Lr) die("arm64 regs[29]/regs[30] 未找到");
const ARM64_FP = parseInt(arm64Fp[1], 10);
const ARM64_LR = parseInt(arm64Lr[1], 10);
if (ARM64_FP !== 29 || ARM64_LR !== 30) die(`arm64 FP/LR=${ARM64_FP}/${ARM64_LR}, 期望 29/30`);

// ⑦ 对数: SIGPROF 引用 (代码 4: Install/Restore/HandleProfilerSignal/pthread_kill
//    + 注释 2: cc:366 Aix 说明, cc:585 构造器说明, 期望 6)
const sigprofCount = (ccSrc.match(/SIGPROF/g) ?? []).length;
if (sigprofCount !== 6) die(`SIGPROF 引用数=${sigprofCount}, 期望 6`);

// ⑧ darwin / win 提取面针 (accessor 直取, 无索引)
for (const s of ["__ss.__rip", "__ss.__rsp", "__ss.__rbp",
  "arm_thread_state64_get_pc", "arm_thread_state64_get_sp",
  "arm_thread_state64_get_fp", "arm_thread_state64_get_lr"]) need(CC, s);
// win 走 Sampler::DoSample 的 CONTEXT 路径 (非 USE_SIGNALS)
for (const s of ["context.Rip", "context.Rsp", "context.Rbp",
  "context.Pc", "context.Sp", "context.Fp"]) need(CC, s);

// ── 生成 Rust ──
const out = [];
out.push(banner(GEN, FILES.join(", "), "库级采样器; USE_SIGNALS 在 x64+arm64 的 linux/darwin 目标恒开, 信号路径为唯一采样路径"));
out.push(`// 对数: 帧上限针 2 + 类面针 19 + FillRegisterState 架构分支 ${archSet.length}/系统分支 ${osSet.length} +`);
out.push(`//       gregs 索引 10/15/16 + arm64 regs 29/30 + SIGPROF 引用 6 (代码 4 + 注释 2) 全过`);

out.push(``);
out.push(`use core::sync::atomic::AtomicBool;`);

// 帧上限 (h:36-37)
out.push(``);
out.push(`/// 采样帧上限 log2 (原版 Sampler::kMaxFramesCountLog2)`);
out.push(`pub const MAX_FRAMES_COUNT_LOG2: u32 = ${maxFramesLog2};`);
out.push(`/// 采样帧上限 (原版 Sampler::kMaxFramesCount = (1u << ${maxFramesLog2}) - 1 展开)`);
out.push(`pub const MAX_FRAMES_COUNT: usize = ${maxFramesCount};`);

// USE_SIGNALS 展开
out.push(``);
out.push(`/// USE_SIGNALS 展开: 原 #if ${useSignalsCond}`);
out.push(`/// x64+arm64 目标仅 linux/darwin(均 POSIX, 非 Cygwin/Fuchsia) → 恒真`);
out.push(`pub const USE_SIGNALS: bool = true;`);

// 平台分支登记 (跳过注释)
out.push(``);
out.push(`// 平台分支登记 (规则 9: 按 x64+arm64 展开, 其余跳过):`);
out.push(`// - 架构跳过 (FillRegisterState 内): ${skippedArches.join(" / ")}`);
out.push(`// - 系统跳过 (FillRegisterState 内): ${skippedOses.join(" / ")}`);
out.push(`// - FUCHSIA zx_* 路径 / WIN DuplicateHandle+CONTEXT 路径整体跳过 (非 POSIX 信号路径)`);
out.push(`// - ANDROID 兼容 ucontext 补丁块 (cc:81-182) 跳过; SIGPROF=27 (linux/macos 系统头同值)`);

// 信号常量
out.push(``);
out.push(`/// SIGPROF 信号编号 (linux 与 darwin 同为 27; 原版引用 6 处 = 代码 4 + 注释 2)`);
out.push(`pub const SIGPROF: i32 = 27;`);
out.push(``);
out.push(`/// sigaction.sa_flags 展开 (原版 sa.sa_flags = SA_RESTART | SA_SIGINFO | SA_ONSTACK)`);
out.push(`#[cfg(target_os = "linux")]`);
out.push(`pub const SIGACTION_FLAGS: i32 = 0x1000_0000 | 0x0000_0004 | 0x0800_0000; // RESTART|SIGINFO|ONSTACK`);
out.push(`#[cfg(target_os = "macos")]`);
out.push(`pub const SIGACTION_FLAGS: i32 = 0x0000_0002 | 0x0000_0040 | 0x0000_0001; // RESTART|SIGINFO|ONSTACK`);

// 寄存器提取索引表
out.push(``);
out.push(`/// linux-x64 寄存器提取 gregs[] 索引 (原版 mcontext.gregs[REG_R*];`);
out.push(`/// 取自 cc:179 枚举, glibc ucontext 同值; 原版引用 cc:421-423)`);
out.push(`pub mod linux_x64 {`);
out.push(`  /// RBP → pc 用 RIP`);
out.push(`  pub const RBP: usize = ${REG_RBP};`);
out.push(`  pub const RSP: usize = ${REG_RSP};`);
out.push(`  pub const RIP: usize = ${REG_RIP};`);
out.push(`}`);
out.push(``);
out.push(`/// linux-arm64 寄存器提取索引 (原版 cc:439-444: pc=mcontext.pc, sp=mcontext.sp,`);
out.push(`/// FP 是 x29 别名, LR 是 x30 别名)`);
out.push(`pub mod linux_arm64 {`);
out.push(`  pub const FP_REG: usize = ${ARM64_FP};`);
out.push(`  pub const LR_REG: usize = ${ARM64_LR};`);
out.push(`}`);
out.push(``);
out.push(`// darwin/win 寄存器提取面 (accessor/字段直取, 无索引表):`);
out.push(`//   darwin-x64: __ss.__rip / __ss.__rsp / __ss.__rbp (cc:503-505)`);
out.push(`//   darwin-arm64: arm_thread_state64_get_{pc,sp,fp,lr} (cc:511-518)`);
out.push(`//   win-x64: CONTEXT.Rip / Rsp / Rbp (DoSample, cc:637-639)`);
out.push(`//   win-arm64: CONTEXT.Pc / Sp / Fp (DoSample, cc:641-643)`);

// 类型别名 + RegisterState 骨架
out.push(``);
out.push(`/// 原版 using AtomicMutex = std::atomic_bool (h:100)`);
out.push(`pub type AtomicMutex = AtomicBool;`);
out.push(``);
out.push(`/// 采样寄存器状态骨架 (正主 v8::RegisterState 定义在 include/v8-unwinder.h;`);
out.push(`/// 此处仅采样面: pc/sp/fp 必有, lr 仅 arm64/win32 路径写回)`);
out.push(`#[derive(Debug, Clone, Copy, Default)]`);
out.push(`pub struct RegisterState {`);
out.push(`  pub pc: usize,`);
out.push(`  pub sp: usize,`);
out.push(`  pub fp: usize,`);
out.push(`  pub lr: Option<usize>,`);
out.push(`}`);
out.push(``);
out.push(`/// 采样回调面 (原版 virtual void SampleStack(const v8::RegisterState&) = 0; 客户端覆写)`);
out.push(`pub trait SampleStackHook {`);
out.push(`  fn sample_stack(&mut self, regs: &RegisterState);`);
out.push(`}`);

// 类面骨架 (修订就地补语义)
out.push(``);
out.push(`// ── 类面骨架 (修订就地补语义) ──`);
out.push(`//   Sampler (isolate_; active_/record_sample_: AtomicMutex 即 atomic_bool;`);
out.push(`//     Start/Stop/IsActive(relaxed load)/ShouldRecordSample(exchange(false))/`);
out.push(`//     DoSample/SetActive/SetShouldRecordSample;`);
out.push(`//     计数面: is_counting_samples_/js_sample_count_/external_sample_count_ + StartCountingSamples)`);
out.push(`//   AtomicGuard (AtomicMutex* + is_success_; 构造 compare_exchange_strong 抢锁,`);
out.push(`//     is_blocking=false 只试一次; 析构持锁才放)`);
out.push(`//   SamplerManager (sampler_map_: 按 vm_tid i32 分桶的 SamplerList;`);
out.push(`//     AddSampler/RemoveSampler 受 AtomicGuard; DoSample 非阻塞抢锁失败即返回;`);
out.push(`//     instance() = base::LeakyObject 全局泄漏单例)`);
out.push(`//   SignalHandler (client_count_ 0→1 Install / →0 Restore; sigaction(SIGPROF) 装/卸;`);
out.push(`//     HandleProfilerSignal → FillRegisterState → SamplerManager::instance()->DoSample)`);
out.push(`//   Sampler::DoSample 信号路径 (cc:614-619): RecursiveMutexGuard + Installed 检查 +`);
out.push(`//     SetShouldRecordSample + pthread_kill(vm_tself, SIGPROF)`);

// 提取不变式断言
out.push(``);
out.push(`// ── 提取不变式断言 ──`);
out.push(`const _: () = {`);
out.push(`  assert!(MAX_FRAMES_COUNT_LOG2 == ${maxFramesLog2});`);
out.push(`  assert!(MAX_FRAMES_COUNT == ${maxFramesCount});`);
out.push(`  assert!(USE_SIGNALS);`);
out.push(`  assert!(SIGPROF == 27);`);
out.push(`  assert!(linux_x64::RBP == ${REG_RBP} && linux_x64::RSP == ${REG_RSP} && linux_x64::RIP == ${REG_RIP});`);
out.push(`  assert!(linux_arm64::FP_REG == ${ARM64_FP} && linux_arm64::LR_REG == ${ARM64_LR});`);
out.push(`};`);

const rust = out.join("\n") + "\n";
const p = writeSrc("v8-profiler", "sampler.rs", rust);
console.log(`帧上限 log2=${maxFramesLog2} 上限=${maxFramesCount}; SIGPROF 引用=${sigprofCount}; 架构分支 ${archSet.length} (展开 X64+ARM64, 跳过 ${skippedArches.length}); 系统分支 ${osSet.length} (展开 LINUX+DARWIN, 跳过 ${skippedOses.length})`);
console.log(`written ${p}`);
