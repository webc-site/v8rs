# v8-base-platform

## 原版测试覆盖映射（v8/test/unittests/base/platform → tests/）

断言级规约清单见 `tests/SPEC.md`（每断言一行，含原版行号）。门禁：
`CARGO_TARGET_DIR=target/base-plat-test cargo nextest run -p v8-base-platform --all-features`
= 55 tests run: 55 passed（37 映射用例 + 接线冒烟 1 例 + 审查回归
7 例（tests/review_round1.rs）+ 6 例（tests/review_round2.rs）+ 5 例
（tests/review_round3.rs））。

状态说明：绿 = 全断言通过（无弱化假绿）。

注：分配器族测试（region-allocator-unittest.cc / virtual-address-space-unittest.cc，
位于 `unittests/base/` 根）不在本 crate 范围，归属 v8-base crate。

### mutex-unittest.cc → tests/mutex_unittest.rs（6 用例）

| 原版用例 | 原版位置 | Rust 测试函数 | 状态 |
|---|---|---|---|
| TEST(Mutex, LockGuardMutex) | cc:20-24 | mutex_lock_guard_mutex | 绿 |
| TEST(Mutex, LockGuardRecursiveMutex) | cc:27-34 | mutex_lock_guard_recursive_mutex | 绿 |
| TEST(Mutex, LockGuardLazyMutex) | cc:37-41 | mutex_lock_guard_lazy_mutex | 绿（已回收 → v8-base::lazy_instance） |
| TEST(Mutex, LockGuardLazyRecursiveMutex) | cc:44-51 | mutex_lock_guard_lazy_recursive_mutex | 绿（同上） |
| TEST(Mutex, MultipleMutexes) | cc:54-72 | mutex_multiple_mutexes | 绿 |
| TEST(Mutex, MultipleRecursiveMutexes) | cc:75-96 | mutex_multiple_recursive_mutexes | 绿 |

### semaphore-unittest.cc → tests/semaphore_unittest.rs（3 用例）

| 原版用例 | 原版位置 | Rust 测试函数 | 状态 |
|---|---|---|---|
| TEST(Semaphore, ProducerConsumer)（含 Worker cc:36-88 的 ASSERT_FALSE WaitFor(1µs) cc:78） | cc:90-101 | semaphore_producer_consumer | 绿 |
| TEST(Semaphore, WaitAndSignal) | cc:104-121 | semaphore_wait_and_signal | 绿 |
| TEST(Semaphore, WaitFor) | cc:124-139 | semaphore_wait_for | 绿 |

### condition-variable-unittest.cc → tests/condition_variable_unittest.rs（4 用例）

| 原版用例 | 原版位置 | Rust 测试函数 | 状态 |
|---|---|---|---|
| TEST(ConditionVariable, WaitForAfterNofityOnSameThread)（保留原版拼写） | cc:14-27 | condition_variable_wait_for_after_nofity_on_same_thread | 绿 |
| TEST(ConditionVariable, MultipleThreadsWithSeparateConditionVariables)（128 线程） | cc:59-105 | condition_variable_multiple_threads_with_separate_condition_variables | 绿 |
| TEST(ConditionVariable, MultipleThreadsWithSharedSeparateConditionVariables)（128 线程共享 CV） | cc:139-214 | condition_variable_multiple_threads_with_shared_separate_condition_variables | 绿 |
| TEST(ConditionVariable, LoopIncrement)（1..16 线程轮转） | cc:267-296 | condition_variable_loop_increment | 绿 |

### platform-unittest.cc → tests/platform_unittest.rs（6 用例）

| 原版用例 | 原版位置 | Rust 测试函数 | 状态 |
|---|---|---|---|
| TEST(OS, GetCurrentProcessId)（POSIX 分支） | cc:49-58 | os_get_current_process_id | 绿 |
| TEST(OS, RemapPages)（is_remap_page_supported 门控） | cc:60-78 | os_remap_pages | 绿 |
| TEST_F(ThreadLocalStorageTest, DoTest)（32 key，主线程 Run + 子线程 Run） | cc:275-330 | thread_local_storage_test_do_test | 绿 |
| TEST(StackTest, GetStackStart) | cc:332 | stack_test_get_stack_start | 绿 |
| TEST(StackTest, GetCurrentStackPosition) | cc:334-336 | stack_test_get_current_stack_position | 绿 |
| TEST(StackTest, StackVariableInBounds) | cc:339-347 | stack_test_stack_variable_in_bounds | 绿 |
| （Rust 侧强化补充，原版无对应） | platform-posix.cc:276-278/355-395/785-787 | os_random_mmap_and_fake_mmap | 绿 |

### time-unittest.cc → tests/time_unittest.rs（17 用例）

| 原版用例 | 原版位置 | Rust 测试函数 | 状态 |
|---|---|---|---|
| TEST(TimeDelta, ZeroMinMax) | cc:27-42 | time_delta_zero_min_max | 绿 |
| TEST(TimeDelta, MaxConversions) | cc:44-54 | time_delta_max_conversions | 绿 |
| TEST(TimeDelta, NumericOperators)（乘除标量部分；i64 重载） | cc:95-124 | time_delta_numeric_operators | 绿 |
| （同上复合赋值 *= / /=） | cc:101-104,111-114,121-122 | time_delta_numeric_operators_compound | 绿（MulAssign/DivAssign 已实现） |
| TEST(TimeDelta, FromAndIn) | cc:184-200 | time_delta_from_and_in | 绿 |
| TEST(TimeDelta, MachTimespec)（V8_OS_DARWIN，本机生效） | cc:202-211 | time_delta_mach_timespec | 绿（from/to_mach_timespec 已实现） |
| TEST(Time, Max) | cc:213-219 | time_max | 绿 |
| TEST(Time, MaxConversions)（POSIX 分支） | cc:221-253 | time_max_conversions | 绿 |
| TEST(Time, JsTime) | cc:255-258 | time_js_time | 绿 |
| TEST(Time, Timespec)（POSIX） | cc:262-275 | time_timespec | 绿 |
| TEST(Time, Timeval)（POSIX） | cc:278-291 | time_timeval | 绿 |
| TEST(Time, NowResolution)（1s 重试预算） | cc:345-349 | time_now_resolution | 绿 |
| TEST(TimeTicks, NowResolution) | cc:352-360 | time_ticks_now_resolution | 绿 |
| TEST(TimeTicks, IsMonotonic) | cc:362-372 | time_ticks_is_monotonic | 绿 |
| TEST(ElapsedTimer, StartStop) | cc:384-423 | elapsed_timer_start_stop | 绿 |
| TEST(ElapsedTimer, StartStopArgs) | cc:425-482 | elapsed_timer_start_stop_args | 绿 |
| TEST(ThreadTicks, MAYBE_ThreadNow) | cc:485-533 | thread_ticks_maybe_thread_now | 绿（darwin THREAD_BASIC_INFO 已实现；wait_until_initialized 非 Windows 空实现） |

### 跳过项及理由（原版存在、本机构建不迁移）

| 原版用例 | 原版位置 | 跳过理由 |
|---|---|---|
| OS.GetSharedLibraryAddresses | cc:81-107 | `#ifdef V8_TARGET_OS_LINUX`，darwin 原版不编译 |
| OSSignalSafeMapsParserForCurrentProcess | cc:109-138 | Linux-only |
| OSSignalSafeMapsParserForCustomMapsFile | cc:142-239 | Linux-only + makedev/proc maps 解析 |
| OSSignalSafeMapsParserCustomName | cc:241-270 | Linux-only + VirtualAddressSpace 子系统未导出 |
| SetDataReadOnlyTest.SetDataReadOnly | cc:352-369 | 含 ASSERT_DEATH_IF_SUPPORTED x2（写只读页必 SIGSEGV）；death/真实信号类按规约跳过 |
| PlatformTest.StackAlignment | cc:387-410 | 依赖 v8::Isolate/JS 执行（TestWithIsolate），非 L0 crate 范围 |
| TimeDelta.DISABLED_Overflows | time cc:127-182 | 原版 `DISABLED_` 前缀，gtest 默认不运行 |
| Time.Filetime | time cc:296-309 | `#if V8_OS_WIN`，本机构建不编译 |
| TimeTicks.TimerPerformance | time cc:537-578 | `#if V8_OS_WIN`；且其断言已被原版注释掉，仅剩 printf |

### 待回收点处置（v8-base 依赖盘点）

| 回收点 | 处置 | 说明 |
|---|---|---|
| LazyMutex / LazyRecursiveMutex → `v8-base::lazy_instance` | 已收 | `src/mutex.rs` 以 `LazyInstance<Mutex>` 承接（const new / Default / pointer），与原版 platform→lazy-instance include 方向一致 |
| OS::Initialize / SetRandomMmapSeed / GetRandomMmapAddr (+ GetGCFakeMMapFile) → `v8-base::page_allocator` 进程级 RNG | 已收 | `src/os.rs` 四方法委托 `v8_base::page_allocator::{os_set_random_mmap_seed, os_get_random_mmap_addr}`；gc_fake_mmap 为进程级静态；强化用例 os_random_mmap_and_fake_mmap 覆盖 |
| MemoryProtectionKey signal-handler / 栈保护 → v8-base trap-handler | 不收 | v8-base 尚无 trap-handler/signal-handling 子系统转写，无从接续；待该模块出现后回收（`src/memory_protection_key.rs` 头注释同步） |
| AddressSpaceReservation::SetName (prctl PR_SET_VMA_ANON_NAME) | 已完成 | `set_name` 已委托 `OS::set_memory_region_name`，Linux ≥5.17 生效、其他平台 false（原版同款） |

## 推迟 API 登记（原版存在、本 crate 未实现）

| 原版 API | 原版位置 | 推迟理由 |
|---|---|---|
| OS::GetSharedLibraryAddresses | platform-darwin.cc:83-104 | 依赖 `_dyld_image_count`/`getsectiondata` FFI 族；V8 内部仅 profiler/信号解析使用，待消费方出现再实现（测试侧 Linux-only 已另登记） |
| SharedMemoryHandle 族（Create/Destroy/AllocateShared/FreeShared/reservation::AllocateShared） | platform-darwin.cc:150-187 / platform-posix.cc:513-543 | mach port / memfd 共享内存子系统整体推迟 |
| OS::CreateSharedMemoryHandleForTesting | platform-darwin.cc:150-160 | 同上 |
| TimezoneCache 族（CreateTimezoneCache / PosixTimezoneCache / DaylightSavingsOffset / LocalTimezone / LocalTimeOffset） | platform-posix-time.{h,cc} | 日期时区语义属 v8-date 上下文，待 jiff 承接（依赖策略禁止提前引入） |
| OS::stdio 族（FOpen/Remove/OpenTemporaryFile/Print/FPrint/SNPrintF/StrNCpy/LogFileOpenMode） | platform-posix.cc:1003-1125 | C va_list printf 面在 Rust 无意义；调用方用 std::fmt/fs |
| MemoryProtectionKey::RegisterExternallyAllocatedKey / SetDefaultPermissionsForAllKeysInSignalHandler / SetKeyForCurrentThreadsStack | memory-protection-key.h:89-160 | signal-handler/栈保护随 trap-handler 一并回收（见上表）；SetPermissionsForKey（WRPKRU 写 PKRU）已由 `set_key_permissions` 实现 |
| ThreadEntry 的 FPU::SetFlushDenormals(false) | platform-posix.cc:1268-1270 | Rust 线程不复制创建线程 MXCSR 的场景未确认，避免误设全局 FPU 状态 |
| OS::SealPages 真实现（mseal） | platform-posix.cc:693-704 | V8_ENABLE_MEMORY_SEALING 默认关，原版即 return false |
| OS::Abort 的 AbortMode 分支 | platform-posix.cc:795-809 | fuzz 模式挂在引擎层，L0 直接 libc::abort |
| SetJitWriteProtected（pthread_jit_write_protect_np） | platform-darwin.cc:232-243 | W^X 切换由 JIT 上层（代码区管理）驱动，非 L0 |
| OS::EnsureAlternativeSignalStackIsAvailableForCurrentThread / IsHardwareEnforcedShadowStacksEnabled / GetFirstFreeMemoryRangeWithin / AdjustSchedulingParams(darwin x64) | platform-posix.cc:281-319 / platform-darwin.cc:112-136 | 依赖 signal 子系统 / 仅 x64 macos tcsm sysctl |
| LazySemaphore<N> / LAZY_SEMAPHORE_INITIALIZER | semaphore.h:91-104 | LazyDynamicInstance（堆分配 + 定制计数），待 v8-base::lazy_instance 支持自定义 Create trait 后回收 |

## 已核实可接受偏差（不修，登记说明）

- TimeTicks::IsHighResolution 恒 true（原版 linux 运行时探测 100 次采样；现代内核
  CLOCK_MONOTONIC 恒 ns 级，探测必真；darwin 原版本就恒 true）
- darwin 单调钟换算 `ticks*numer/denom/1000` 与原版 `ticks*numer/(1000*denom)`
  除法顺序不同，差异 ≤1µs 舍入
- Malloc(0)/AlignedAlloc(0) 返回 null（原版 malloc(0) 返回实现定义非空指针；
  Rust 特判更安全）
- Thread drop 未 join 时按 detach（std JoinHandle 语义；原版析构 pthread_detach 等价）
- Thread::name() 保留全名（原版 strncpy 截 15 字节；OS 层命名行为一致）
- linux musl 主线程 ObtainCurrentThreadStackReservedLimit 的 nullptr 特判未迁
 （原版 platform-posix.cc:1507-1509；本 crate 目标 glibc）
- ScopedTimer 构造必填 location（原版允许 nullptr 禁用；Rust 不暴露空指针面）
- `Semaphore::new(负数)` 钳 0（原版 POSIX `DCHECK_GE(count, 0)`，release 下
  sem_init 静默失败；dispatch 分支不检查）——防御式钳 0 更安全
- `MemoryMappedFile::open(ReadOnly)` 走 memmap2 `Mmap::map`（MAP_SHARED）；
  原版 ReadOnly 为 `MAP_PRIVATE`（platform-posix.cc:874-877）——只读访问不写
  映射，两者可观察行为一致
- linux RemapPages 的 kernel ≥5.13 门控（KernelSupportsRemapDontUnmap）与
  /proc/self/maps file-backed fallback（platform-linux.cc:175-194/248-292）未迁；
  老内核上 mremap EINVAL → Err（原版 fallback false→false 近似）；成功后 memcmp
  sanity CHECK（禁 panic）不迁
- `OS::get_user_time` 失败返回 (0,0)（原版 return -1 且不写参数；
  platform-posix.cc:952-960）；调用方在 V8 中均忽略返回值
- darwin x86_64 的 PTHREAD_STACK_MIN 实为 8192（SDK limits.h:113-118），
  Rust 统一钳 16384 偏高（安全方向）；arm64/linux 精确
  （PTHREAD_STACK_MIN 修正说明见 src/thread.rs effective_stack_size 注释）
