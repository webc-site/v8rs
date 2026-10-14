[English](#en) | [中文](#zh)

---

<a name="en"></a>

# v8-trap-handler

Wasm OOB trap handler: out-of-bounds wasm access raises SIGSEGV/SIGBUS ->
the signal handler intercepts it and rewrites the ucontext IP to a landing pad.

The posix subset is complete; Windows and simulator paths are deferred.

- [Source mapping](#source-mapping)
- [Deferred](#deferred)
- [Semantic deviations](#semantic-deviations)

## Source mapping

| v8/src/trap-handler/ | this crate | conversion script |
|---|---|---|
| trap-handler.h | src/lib.rs | tool/src/trap-handler/trap-handler.h.js |
| trap-handler-internal.h | src/internal.rs | tool/src/trap-handler/trap-handler-internal.h.js |
| handler-shared.cc | src/shared.rs | tool/src/trap-handler/handler-shared.cc.js |
| handler-inside.cc | src/inside.rs | tool/src/trap-handler/handler-inside.cc.js |
| handler-inside-posix.h + .cc | src/inside_posix.rs | tool/src/trap-handler/handler-inside-posix.js |
| handler-outside.cc | src/outside.rs | tool/src/trap-handler/handler-outside.cc.js |
| handler-outside-posix.cc | src/outside_posix.rs | tool/src/trap-handler/handler-outside-posix.cc.js |
| (error hub, no upstream file) | src/error.rs | hand-written |

Note: handler-outside.h listed in the task does not exist in this v8 checkout;
the 8th file of the posix path is handler-inside.cc (platform-independent
IsFaultAddressCovered/IsAccessedMemoryCovered).

## Deferred

- handler-inside-win.* / handler-outside-win.cc (Windows)
- handler-outside-simulator.cc / trap-handler-simulator.h (simulator)
- full native-unittest with JIT code injection (real-fault integration
  tests exist: tests/signal.rs covers landing-pad recovery and fallback
  restore+replay)
- sanitizer install-detection (handler-outside-posix.cc:65-83)
- linux-side synthetic-siginfo ucontext rewrite unit test (libc siginfo
  union fields are not writable; linux covered by real-fault path)

## Semantic deviations

- thread_in_wasm flag: removed upstream; nesting semantics carried by
  TrapHandlerGuard (TLS), translated 1:1
- malloc-failure paths: allocation failure aborts in Rust, no false return
- EnableTrapHandler twice: IMMEDIATE_CRASH -> Err(Error::EnableTwice)
- is_fault_address_covered / is_accessed_memory_covered exposed as pub
  queries (self-guarded to satisfy the lock contract)
- g_old_handler: plain global upstream (racy but deadlock-free) ->
  Locked::lock_outside spinlock; remove_trap_handler runs inside the
  signal handler and takes this lock, so the lock window blocks
  kOobSignal via pthread_sigmask (outside_posix::with_old_handler)
- ReleaseHandlerData negative index: upstream release build is negative
  subscript UB -> debug crash (matches TH_DCHECK) + release early return
- gRecoveredTrapCount hit counting: upstream load+store (two-step, racy)
  -> fetch_add (lock-free, async-signal-safe, no lost updates)
- kEnableSlowChecks: ValidateCodeObjects free-list consistency check
  landed as debug_assert (hooked in register/release); per-offset >= 0
  check dropped (u32 is always true)


---

<a name="zh"></a>

# v8-trap-handler

wasm OOB 陷阱处理器: wasm 越界访问触发 SIGSEGV/SIGBUS → 信号处理器拦截 →
改写 ucontext IP 跳转 landing pad。

posix 子集已落地; Windows 与模拟器路径登记推迟。

- [原版映射](#原版映射)
- [推迟登记](#推迟登记)
- [语义偏离说明](#语义偏离说明)

## 原版映射

| v8/src/trap-handler/ | 本 crate | 转换脚本 |
|---|---|---|
| trap-handler.h | src/lib.rs | tool/src/trap-handler/trap-handler.h.js |
| trap-handler-internal.h | src/internal.rs | tool/src/trap-handler/trap-handler-internal.h.js |
| handler-shared.cc | src/shared.rs | tool/src/trap-handler/handler-shared.cc.js |
| handler-inside.cc | src/inside.rs | tool/src/trap-handler/handler-inside.cc.js |
| handler-inside-posix.h + .cc | src/inside_posix.rs | tool/src/trap-handler/handler-inside-posix.js |
| handler-outside.cc | src/outside.rs | tool/src/trap-handler/handler-outside.cc.js |
| handler-outside-posix.cc | src/outside_posix.rs | tool/src/trap-handler/handler-outside-posix.cc.js |
| (错误集中, 无对应原版文件) | src/error.rs | 手工维护 |

注: 任务清单中的 handler-outside.h 在本版 v8 源码树不存在; posix 路径的第 8
个文件为 handler-inside.cc (IsFaultAddressCovered/IsAccessedMemoryCovered 平台无关实现)。

## 推迟登记

- handler-inside-win.* / handler-outside-win.cc (Windows)
- handler-outside-simulator.cc / trap-handler-simulator.h (模拟器, 架构 crate 就位后回接)
- JIT 代码注入的完整 native-unittest (本 crate 已含真机故障集成测试
  tests/signal.rs: 接管跳 landing pad + fallback 恢复重演)
- sanitizer 拦截安装检测 (handler-outside-posix.cc:65-83)
- linux 侧伪造 siginfo 的 ucontext 改写单元测试 (libc 的 siginfo union
  字段不可写; linux 由 tests/signal.rs 真机路径覆盖)

## 语义偏离说明

- thread_in_wasm 标志: 本版 v8 已移除, 防嵌套语义由 TrapHandlerGuard (TLS) 承担, 1:1 转写
- malloc 失败路径: Rust 分配失败语言级终止 (与原版 abort 等价), 无 false 返回路径
- EnableTrapHandler 重复调用: 原版 IMMEDIATE_CRASH → Err(Error::EnableTwice) (错误集中)
- is_fault_address_covered / is_accessed_memory_covered: 原版仅信号上下文内部使用,
  Rust 侧 pub 暴露 (自持 guard 满足锁契约) 供注册方与测试查询
- g_old_handler: 原版无锁普通全局 (数据竞争但无死锁) → Locked::lock_outside
  (自旋锁); remove_trap_handler 会在信号处理器内被调且取此锁, 故持锁窗口
  以 pthread_sigmask 屏蔽 kOobSignal 消除重入死锁 (见 outside_posix::with_old_handler)
- ReleaseHandlerData 负索引: 原版 release 下为负下标 UB → debug 崩溃 (对齐
  TH_DCHECK) + release 直接返回 (禁 panic)
- gRecoveredTrapCount 命中计数: 原版 load+store 两步非原子 → fetch_add
  (同为 lock-free 原子, async-signal-safe 且并发不丢计数)
- kEnableSlowChecks: ValidateCodeObjects 的自由链表一致性校验落 debug_assert
  (register/release 两挂点); 指令偏移逐条 >= 0 检查不迁移 (u32 类型恒真)

