# v8-base-platform 测试迁移断言规约 (7.3)

来源: `v8/test/unittests/base/platform/`（实际存在的 5 个文件: mutex / semaphore /
condition-variable / platform / time-unittest.cc）。本清单先于直译: 每条断言一行
（用例名 + 断言内容 + 期望值），Rust 测试按本清单 1:1 生成，断言强度守恒。

gtest→Rust 写法映射遵循 `.agents/glossary.md`。测试名保留原版 `TEST(Suite, Name)`
→ `fn suite_name()`（snake_case）。

标注约定:
- （历史 `[RED]` 项已全部转绿: LazyMutex 族、MulAssign/DivAssign、mach_timespec、
  ThreadTicks darwin 支持均已实现并验证。）
- `[SKIP]` = 不迁移，附理由（见文末跳过表）。

---

## 1. mutex-unittest.cc → tests/mutex_unittest.rs

### mutex_lock_guard_mutex (cc:20-24)
- L22 用 MutexGuard 锁 `mutex` 一次并出作用域释放；期望: 正常返回不阻塞
- L23 再次用 MutexGuard 锁同一 mutex；期望: 正常返回（若前次未释放将死锁→红）
- 强化: 每次出作用域后 `try_lock` 成功（==true），证明 guard drop 已解锁

### mutex_lock_guard_recursive_mutex (cc:27-34)
- L29 LockGuard<RecursiveMutex> 锁/放一次；期望: 正常返回
- L31-32 同一作用域内嵌套两把 RecursiveMutex 锁；期望: 可重入不死锁
- 强化: 出作用域后 `try_lock` 成功 == true

### mutex_lock_guard_lazy_mutex (cc:37-41)
- L38 LazyMutex = LAZY_MUTEX_INITIALIZER 静态初始化
- L39/L40 经 MutexGuard 两次锁放；期望: 正常返回
- 强化: 出作用域后 `try_lock` 成功 == true（LazyMutex 已回收 → v8-base::lazy_instance）

### mutex_lock_guard_lazy_recursive_mutex (cc:44-51)
- L45 LazyRecursiveMutex = LAZY_RECURSIVE_MUTEX_INITIALIZER
- L46 单次锁放; L48-49 嵌套双重锁；期望: 正常返回
- 强化: 出作用域后 `try_lock` 成功 == true

### mutex_multiple_mutexes (cc:54-72)
- Order1: mutex1.lock→mutex2.lock→mutex3.lock→unlock(1)→unlock(2)→unlock(3)；期望: 无死锁
- Order2: 同序加锁, 反序解锁；期望: 无死锁
- 强化: 两组结束后对每个 mutex `try_lock` == true（证明全部已释放）

### mutex_multiple_recursive_mutexes (cc:75-96)
- L81 EXPECT_TRUE recursive_mutex1.TryLock()（已 Lock，重入）→ true
- L82 EXPECT_TRUE recursive_mutex2.TryLock()（已 Lock，重入）→ true
- L89 EXPECT_TRUE recursive_mutex1.TryLock()（Lock 后立即重入）→ true
- L91 EXPECT_TRUE recursive_mutex2.TryLock()（Lock 后立即重入）→ true
- 加解锁序列 Order1/Order2 本身: 无死锁

---

## 2. semaphore-unittest.cc → tests/semaphore_unittest.rs

常量 (cc:16-19): kAlphabet="XKOAD"(5) / kBufferSize=987 / kDataSize=987*5*10=49350。

### semaphore_producer_consumer (cc:90-101)
- 消费者 L58 每次读 n: EXPECT_EQ kAlphabet[n%5] == buffer[n%987]（n=0..49350 共 49350 条）
  → Rust: 不匹配次数记录到原子计数，主线程 Join 后 assert_eq!(mismatches, 0)（避免 worker panic 致主线程挂死）
- 主线程 L97/L98 CHECK(Start()) 生产者/消费者 → is_ok
- 缓冲区初值全 0（cc:92 memset）: 前置 assert 全 0

### semaphore_wait_and_signal (cc:104-121)
- 工作线程 (x2, 各 100 轮) L78 ASSERT_FALSE semaphore.WaitFor(1µs) → false（共 200 条，原子记录违规）
- 主线程 L109/L110 CHECK(t1.Start())/CHECK(t2.Start()) → is_ok
- 主线程 L118 semaphore.Wait() 最终取得 1 个令牌 → 正常返回
- 主线程 L120 EXPECT_FALSE semaphore.WaitFor(1µs) → false

### semaphore_wait_for (cc:124-139)
- L128 ASSERT_FALSE WaitFor(0µs)（未 signal）→ false
- L129 ASSERT_FALSE WaitFor(100µs) → false
- L130 ASSERT_FALSE WaitFor(1000µs) → false
- L134 ASSERT_TRUE WaitFor(0µs)（signal 后）→ true
- L136 ASSERT_TRUE WaitFor(100µs) → true
- L138 ASSERT_TRUE WaitFor(1000µs) → true

---

## 3. condition-variable-unittest.cc → tests/condition_variable_unittest.rs

### condition_variable_wait_for_after_nofity_on_same_thread (cc:14-27, 保留原版拼写)
- 每轮 n=0..9: L22 EXPECT_FALSE cv.WaitFor(&mutex, n µs)（NotifyOne 后）→ false（10 条）
- 每轮 n=0..9: L25 EXPECT_FALSE cv.WaitFor(&mutex, n µs)（NotifyAll 后）→ false（10 条）

### condition_variable_multiple_threads_with_separate_condition_variables (cc:59-105)
kThreadCount=128, 每线程独立 Mutex/CV/running_/finished_。
- 启动阶段 (每线程): EXPECT_FALSE running_（==false）; EXPECT_FALSE finished_（==false）; CHECK(Start())→is_ok; 等 running_ 置 true（while+cv.Wait，事件同步）
- 检查阶段1 (逆序 x128): EXPECT_TRUE running_; EXPECT_FALSE finished_
- 检查阶段2 (正序 x128): EXPECT_TRUE running_; EXPECT_FALSE finished_; 置 running_=false; NotifyOne
- 退出等待 (逆序 x128): while !finished_ cv.Wait; EXPECT_FALSE running_; EXPECT_TRUE finished_
- Join (x128): Join 完成后再持锁 EXPECT_FALSE running_; EXPECT_TRUE finished_

### condition_variable_multiple_threads_with_shared_separate_condition_variables (cc:139-214)
kThreadCount=128, 共享一个 CV + Mutex（保留原版名 SharedSeparate…）。
- 启动 (持大锁 x128): EXPECT_FALSE running_; EXPECT_FALSE finished_; CHECK(Start())→is_ok
- 等待全部启动: while !running_n cv.Wait（事件同步, 无墙钟依赖）
- 检查1 (x128): EXPECT_TRUE running_; EXPECT_FALSE finished_
- 检查2 (逆序 x128): EXPECT_TRUE running_; EXPECT_FALSE finished_; 置 running_=false; 最后 NotifyAll
- 等待退出 (x128): while !finished_n cv.Wait
- 检查3 (逆序 x128): EXPECT_FALSE running_; EXPECT_TRUE finished_
- Join x128

### condition_variable_loop_increment (cc:267-296)
thread_count=1..16（不含16, 原版 `< kMaxThreadCount`）, limit=thread_count*10。
- 构造 (每线程): EXPECT_LT(rem, thread_count); EXPECT_EQ(0, limit % thread_count)
- Run 内 (每增量): EXPECT_EQ(*counter, count)（加锁后值未变）
- Run 内 (last_count!=-1 时): EXPECT_EQ(last_count + thread_count - 1, count)（模轮转不变量）
- CHECK(threads[n]->Start())→is_ok（逆序启动）
- Join 后: EXPECT_EQ(counter, limit)（每 thread_count 一条）
- 注: Run 内不变量违规记录到原子标志（防 panic 死锁挂起），Join 后主线程 assert 标志==无违规，强度不变

---

## 4. platform-unittest.cc → tests/platform_unittest.rs

kMaxPageSize=16384（非 win/ppc64 分支, cc:42）; kArray 为该字符串补零至 16384 对齐页。

### os_get_current_process_id (cc:49-58, V8_OS_POSIX 分支)
- EXPECT_EQ (int)getpid() == OS::GetCurrentProcessId()

### os_remap_pages (cc:60-78) — if OS::IsRemapPageSupported()（darwin=true）
- L63 ASSERT_TRUE AllocatePageSize() <= kMaxPageSize
- L70 ASSERT_TRUE remapped_data 非空（OS::Allocate(null,size,AllocatePageSize,kReadWrite)）
- L72 EXPECT_TRUE OS::RemapPages(kArray, size, remapped_data, kReadExecute)
- L74 EXPECT_EQ 0, memcmp(remapped_data, data, size)
- L76 OS::Free(remapped_data, size)

### [SKIP] os_get_shared_library_addresses (cc:81-107) — `#ifdef V8_TARGET_OS_LINUX`，darwin 原版不编译
### [SKIP] os_signal_safe_maps_parser_for_current_process (cc:109-138) — 同上 Linux-only
### [SKIP] os_signal_safe_maps_parser_for_custom_maps_file (cc:142-239) — Linux-only + 依赖 makedev/proc maps 解析
### [SKIP] os_signal_safe_maps_parser_custom_name (cc:241-270) — Linux-only + VirtualAddressSpace 子系统未导出

### os_random_mmap_and_fake_mmap (Rust 侧强化补充, 原版 cc 无对应用例)
- SetRandomMmapSeed(42) → GetRandomMmapAddr 序列可复现（同种子两次序列相等, 2 条 EQ）
- hint 地址页对齐 == true; 落在 46 位掩码内 == true（platform-posix.cc:388-395 规则, x64/mac-arm64）
- Initialize(Some)/GetGCFakeMMapFile 往返 EQ; Initialize(None) 后 == None（cc:276-278/785-787）

### thread_local_storage_test_do_test (cc:326-330 + fixture Run cc:288-314)
- 子线程内: 32× L290 CHECK(!HasThreadLocal(keys[i])) → 全 false
- 32× L293 SetThreadLocal(keys[i], i+1)
- 32× L296 CHECK(HasThreadLocal) → 全 true
- 32× L299 CHECK_EQ GetValue(i)==GetThreadLocal; 32× L300 CHECK_EQ ==GetExistingThreadLocal
- 反序重写: 32× SetThreadLocal(keys[i], GetValue(31-i))
- 32× L306 CHECK(HasThreadLocal); 32× L309-310 CHECK_EQ(GetValue(31-i), Get/GetExisting)
- CHECK(Start())→is_ok; Join() 完成; 主线程 join 结果 ok（worker panic→Err→红）
- 析构: DeleteThreadLocalKey×32

### stack_test_get_stack_start (cc:332)
- EXPECT_NE(nullptr, Stack::GetStackStart()) → slot.value() != 0

### stack_test_get_current_stack_position (cc:334-336)
- EXPECT_NE(nullptr, Stack::GetCurrentStackPosition()) → != 0

### stack_test_stack_variable_in_bounds (cc:339-347, darwin 有效)
- ASSERT_GE GetStackStart() >= GetCurrentStackPosition()
- EXPECT_GE GetStackStart() >= GetRealStackAddressForSlot(&dummy)
- EXPECT_LE GetCurrentStackPosition() <= GetRealStackAddressForSlot(&dummy)

### [SKIP] set_data_read_only_test_set_data_read_only (cc:352-369)
- 含 ASSERT_DEATH_IF_SUPPORTED x2（写只读页必须 SIGSEGV）→ glossary: EXPECT_DEATH 跳过并登记理由
### [SKIP] platform_test_stack_alignment (cc:387-410)
- 依赖 v8::Isolate/JS 执行（v8::TestWithIsolate），非 L0 crate 范围

---

## 5. time-unittest.cc → tests/time_unittest.rs

### time_delta_zero_min_max (cc:27-42)
- kZero.is_zero() == true（constexpr→运行时断言, static_assert 等价）
- kMax.is_max() == true; kMax == TimeDelta::Max(); kMax > FromDays(100*365); kMax > kZero
- kMin.is_min() == true; kMin == TimeDelta::Min(); kMin < FromDays(-100*365); kMin < kZero

### time_delta_max_conversions (cc:44-54)
- kMax.in_days()==INT32_MAX; in_hours()==INT32_MAX; in_minutes()==INT32_MAX
- in_seconds_f()==+inf; in_seconds()==INT64_MAX; in_milliseconds_f()==+inf
- in_milliseconds()==INT64_MAX; in_milliseconds_rounded_up()==INT64_MAX

### time_delta_numeric_operators (cc:95-124)
（Rust 侧 Mul/Div 仅实现 i64；原版 i32/i64/字面量三组以 `i as i64` 等价覆盖, 登记偏差）
- FromMs(1000)*2 == FromMs(2000)（i32 组）; /2 == FromMs(500)
- 同上 i64 组; 同上字面量 2 组 → 共 6 条 EQ

### time_delta_numeric_operators_compound (cc:101-104,111-114,121-122)
- FromMs(1000) *= 2 == FromMs(2000)；/= 2 == FromMs(500)（3 组共 6 条）
- MulAssign/DivAssign<i64> 已实现（src/time.rs）

### time_delta_disabled_overflows (cc:127-182) [SKIP] — 原版 `DISABLED_` 前缀, gtest 默认不运行

### time_delta_from_and_in (cc:184-200)
- FromDays(2)==FromHours(48); FromHours(3)==FromMinutes(180); FromMinutes(2)==FromSeconds(120);
  FromSeconds(2)==FromMilliseconds(2000); FromMilliseconds(2)==FromMicroseconds(2000)
- FromDays(13).in_days()==13; FromHours(13).in_hours()==13; FromMinutes(13).in_minutes()==13
- FromSeconds(13).in_seconds()==13; in_seconds_f()≈13.0 (EXPECT_DOUBLE_EQ→4ulp)
- FromMilliseconds(13).in_milliseconds()==13; in_milliseconds_f()≈13.0 (4ulp)
- FromMicroseconds(13).in_microseconds()==13

### time_delta_mach_timespec (cc:202-211, V8_OS_DARWIN=本机能编译生效)
- 往返: FromMachTimespec(null.ToMachTimespec())==null
- 往返: 42ms 与 42d 同上
- from_mach_timespec/to_mach_timespec 已实现（src/time.rs）

### time_max (cc:213-219)
- max.is_max(); max == Time::Max(); max > Time::now(); max > Time::new()

### time_max_conversions (cc:221-253, OS_POSIX 分支)
- Time::max().to_internal_value() == INT64_MAX
- from_timeval(time_t::MAX, µs_per_sec-1).is_max() == true
- 回转 to_timeval(): (INT64_MAX, kMicrosecondsPerSecond-1)

### time_js_time (cc:255-258)
- Time::from_js_time(700000.3).to_js_time() ≈ 700000.3（EXPECT_DOUBLE_EQ 4ulp）

### time_timespec (cc:262-275, V8_OS_POSIX 生效)
- null.is_null(); 往返 FromTimespec(ToTimespec()): null / now / now_from_system_time / unix_epoch / max 各 1 条
- max.is_max() 前置断言

### time_timeval (cc:278-291, V8_OS_POSIX 生效)
- 同上 Timeval 版本 5 条 + is_null/is_max

### [SKIP] time_filetime (cc:296-309) — `#if V8_OS_WIN`，本机构建不编译

### time_now_resolution (cc:345-349 + ResolutionTest cc:315-340)
- 循环不变量: 时钟推进后 delta.in_microseconds() != 0（每轮 1 条）
- 终态: delta <= 16ms（重试预算=原版 1s; 7.4 余量: 保留 1s 结构, 若抖动大重试自然多轮）

### time_ticks_now_resolution (cc:352-360)
- IsHighResolution()==true→目标粒度 1µs, 否则 16ms（按运行值选）
- 同 ResolutionTest 两组断言

### time_ticks_is_monotonic (cc:362-372)
- 100ms 窗口自旋: 每次 ticks >= previous（GE 断言）
- (ticks - previous).in_microseconds() >= 0

### elapsed_timer_start_stop (cc:384-423)
- 前置 !is_started(); start 后 is_started()（原版 DCHECK→Rust debug_assert+显式断言）
- Sleep(100ms): elapsed >= 100ms; < 300ms（wait+noise, 原版余量）
- pause→is_paused; sleep 100; resume→is_started; elapsed>=100ms <300ms; pause→is_paused
- sleep100→resume→!is_paused&&is_started; elapsed>=100ms <300ms
- stop→!is_started

### elapsed_timer_start_stop_args (cc:425-482)
- 两 timer start_at(同一 now): is_started ×2
- delta1=elapsed_at(now); sleep; delta2=elapsed_at(now) → GE(delta1, delta2)
- sleep; NE(delta1, timer2.elapsed())
- now2=now(); NE(elapsed_at(now), elapsed_at(now2)); NE(delta1, elapsed_at(now2));
  NE(delta2, timer2.elapsed_at(now2)); GE(t1.elapsed_at(now2), t2.elapsed_at(now2))
- pause_at(now): is_paused ×2; resume_at(now) t1: !t1.is_paused && t2.is_paused
- resume_at(now) t2: 都 !is_paused && is_started
- delta1>=delta2 (elapsed_at(now)); stop 后 !is_started ×2

### thread_ticks_maybe_thread_now (cc:485-533, MAYBE_ThreadNow=ThreadNow 非 android)
- 前置 if IsSupported: WaitUntilInitialized(); begin_thread > ThreadTicks()(null)
- 10ms 预算内 ticks 推进; EXPECT_GT(now, begin)
- Sleep(10ms) 后 wall delta>=10ms(≤2 次尝试); delta_thread>0; difference>=9ms
- darwin ThreadTicks 已实现（src/monotonic.rs: THREAD_BASIC_INFO）; wait_until_initialized
  为非 Windows 空实现（原版同款）
- 原版 if IsSupported 门控：平台不支持时安全跳过主体，对标原版行为

### [SKIP] time_ticks_timer_performance (cc:537-578) — `#if V8_OS_WIN`；且其“断言”已被原版注释掉, 仅剩 printf

---

## 确定性说明 (7.4)

- 无随机数据, 未引入 fastrand; 无 HashMap 迭代序依赖。
- 线程同步一律用 Mutex/CV/Semaphore 事件同步; 等待“线程启动”用原版 while+wait 模式。
- 超时断言沿用原版余量参数（noise=200ms、100ms 单调窗口、1s 分辨率重试预算）;
  分辨率类断言为重试循环, 非一次性墙钟比较。
- producer/consumer 与 worker 线程内的 EXPECT 用原子计数聚合, 主线程统一 assert, 语义
  等价且避免 panic 死锁导致 nextest 挂起。

## 缺陷登记（当前红色用例）

无。历史红色 5 项（LazyMutex 族 / MulAssign/DivAssign / mach_timespec /
ThreadTicks darwin）均已实现转绿；随机源入口（SetRandomMmapSeed / GetRandomMmapAddr /
Initialize / GetGCFakeMMapFile）已回收 v8-base::page_allocator 并补充强化用例。
