# v8-base-platform

## Original-test coverage mapping (v8/test/unittests/base/platform → tests/)

The per-assertion spec manifest lives in `tests/SPEC.md` (one line per assertion, with original cc line numbers). Gate command:
`CARGO_TARGET_DIR=target/base-plat-test cargo nextest run -p v8-base-platform --all-features`
= 55 tests run: 55 passed (37 mapped cases + 1 wiring smoke + review-regression cases: 7 in `tests/review_round1.rs`, 6 in `tests/review_round2.rs`, 5 in `tests/review_round3.rs`).

Status legend: green = all assertions pass (no weakened fake greens).

Note: allocator-family tests (region-allocator-unittest.cc / virtual-address-space-unittest.cc, at the `unittests/base/` root) are out of this crate's scope; they belong to the v8-base crate.

### mutex-unittest.cc → tests/mutex_unittest.rs (6 cases)

| Original case | Original location | Rust test fn | Status |
|---|---|---|---|
| TEST(Mutex, LockGuardMutex) | cc:20-24 | mutex_lock_guard_mutex | green |
| TEST(Mutex, LockGuardRecursiveMutex) | cc:27-34 | mutex_lock_guard_recursive_mutex | green |
| TEST(Mutex, LockGuardLazyMutex) | cc:37-41 | mutex_lock_guard_lazy_mutex | green (reclaimed → v8-base::lazy_instance) |
| TEST(Mutex, LockGuardLazyRecursiveMutex) | cc:44-51 | mutex_lock_guard_lazy_recursive_mutex | green (same) |
| TEST(Mutex, MultipleMutexes) | cc:54-72 | mutex_multiple_mutexes | green |
| TEST(Mutex, MultipleRecursiveMutexes) | cc:75-96 | mutex_multiple_recursive_mutexes | green |

### semaphore-unittest.cc → tests/semaphore_unittest.rs (3 cases)

| Original case | Original location | Rust test fn | Status |
|---|---|---|---|
| TEST(Semaphore, ProducerConsumer) (incl. Worker cc:36-88 ASSERT_FALSE WaitFor(1µs) cc:78) | cc:90-101 | semaphore_producer_consumer | green |
| TEST(Semaphore, WaitAndSignal) | cc:104-121 | semaphore_wait_and_signal | green |
| TEST(Semaphore, WaitFor) | cc:124-139 | semaphore_wait_for | green |

### condition-variable-unittest.cc → tests/condition_variable_unittest.rs (4 cases)

| Original case | Original location | Rust test fn | Status |
|---|---|---|---|
| TEST(ConditionVariable, WaitForAfterNofityOnSameThread) (original typo preserved) | cc:14-27 | condition_variable_wait_for_after_nofity_on_same_thread | green |
| TEST(ConditionVariable, MultipleThreadsWithSeparateConditionVariables) (128 threads) | cc:59-105 | condition_variable_multiple_threads_with_separate_condition_variables | green |
| TEST(ConditionVariable, MultipleThreadsWithSharedSeparateConditionVariables) (128 threads, shared CV) | cc:139-214 | condition_variable_multiple_threads_with_shared_separate_condition_variables | green |
| TEST(ConditionVariable, LoopIncrement) (1..16 rotating threads) | cc:267-296 | condition_variable_loop_increment | green |

### platform-unittest.cc → tests/platform_unittest.rs (6 cases)

| Original case | Original location | Rust test fn | Status |
|---|---|---|---|
| TEST(OS, GetCurrentProcessId) (POSIX branch) | cc:49-58 | os_get_current_process_id | green |
| TEST(OS, RemapPages) (gated by is_remap_page_supported) | cc:60-78 | os_remap_pages | green |
| TEST_F(ThreadLocalStorageTest, DoTest) (32 keys; Run on main thread + worker) | cc:275-330 | thread_local_storage_test_do_test | green |
| TEST(StackTest, GetStackStart) | cc:332 | stack_test_get_stack_start | green |
| TEST(StackTest, GetCurrentStackPosition) | cc:334-336 | stack_test_get_current_stack_position | green |
| TEST(StackTest, StackVariableInBounds) | cc:339-347 | stack_test_stack_variable_in_bounds | green |
| (Rust-side supplement, no upstream case) | platform-posix.cc:276-278/355-395/785-787 | os_random_mmap_and_fake_mmap | green |

### time-unittest.cc → tests/time_unittest.rs (17 cases)

| Original case | Original location | Rust test fn | Status |
|---|---|---|---|
| TEST(TimeDelta, ZeroMinMax) | cc:27-42 | time_delta_zero_min_max | green |
| TEST(TimeDelta, MaxConversions) | cc:44-54 | time_delta_max_conversions | green |
| TEST(TimeDelta, NumericOperators) (scalar mul/div part; i64 overloads) | cc:95-124 | time_delta_numeric_operators | green |
| (same test, compound `*=` / `/=`) | cc:101-104,111-114,121-122 | time_delta_numeric_operators_compound | green (MulAssign/DivAssign implemented) |
| TEST(TimeDelta, FromAndIn) | cc:184-200 | time_delta_from_and_in | green |
| TEST(TimeDelta, MachTimespec) (V8_OS_DARWIN, active on this host) | cc:202-211 | time_delta_mach_timespec | green (from/to_mach_timespec implemented) |
| TEST(Time, Max) | cc:213-219 | time_max | green |
| TEST(Time, MaxConversions) (POSIX branch) | cc:221-253 | time_max_conversions | green |
| TEST(Time, JsTime) | cc:255-258 | time_js_time | green |
| TEST(Time, Timespec) (POSIX) | cc:262-275 | time_timespec | green |
| TEST(Time, Timeval) (POSIX) | cc:278-291 | time_timeval | green |
| TEST(Time, NowResolution) (1s retry budget) | cc:345-349 | time_now_resolution | green |
| TEST(TimeTicks, NowResolution) | cc:352-360 | time_ticks_now_resolution | green |
| TEST(TimeTicks, IsMonotonic) | cc:362-372 | time_ticks_is_monotonic | green |
| TEST(ElapsedTimer, StartStop) | cc:384-423 | elapsed_timer_start_stop | green |
| TEST(ElapsedTimer, StartStopArgs) | cc:425-482 | elapsed_timer_start_stop_args | green |
| TEST(ThreadTicks, MAYBE_ThreadNow) | cc:485-533 | thread_ticks_maybe_thread_now | green (darwin THREAD_BASIC_INFO implemented; wait_until_initialized is a non-Windows no-op) |

### Skipped items and reasons (present upstream, not migrated on this host)

| Original case | Original location | Skip reason |
|---|---|---|
| OS.GetSharedLibraryAddresses | cc:81-107 | `#ifdef V8_TARGET_OS_LINUX`; not compiled in the original on darwin |
| OSSignalSafeMapsParserForCurrentProcess | cc:109-138 | Linux-only |
| OSSignalSafeMapsParserForCustomMapsFile | cc:142-239 | Linux-only + makedev/proc maps parsing |
| OSSignalSafeMapsParserCustomName | cc:241-270 | Linux-only + VirtualAddressSpace subsystem not exported |
| SetDataReadOnlyTest.SetDataReadOnly | cc:352-369 | Contains ASSERT_DEATH_IF_SUPPORTED x2 (writing a read-only page must SIGSEGV); death/real-signal cases skipped per spec |
| PlatformTest.StackAlignment | cc:387-410 | Requires v8::Isolate/JS execution (TestWithIsolate), out of L0 crate scope |
| TimeDelta.DISABLED_Overflows | time cc:127-182 | Original `DISABLED_` prefix; gtest never runs it |
| Time.Filetime | time cc:296-309 | `#if V8_OS_WIN`; not compiled on this host |
| TimeTicks.TimerPerformance | time cc:537-578 | `#if V8_OS_WIN`; its assertions are already commented out upstream (printf only) |

### Pending reclaim points (v8-base dependency audit)

| Reclaim point | Disposition | Notes |
|---|---|---|
| LazyMutex / LazyRecursiveMutex → `v8-base::lazy_instance` | Reclaimed | `src/mutex.rs` wraps `LazyInstance<Mutex>` (const new / Default / pointer), matching the upstream platform→lazy-instance include direction |
| OS::Initialize / SetRandomMmapSeed / GetRandomMmapAddr (+ GetGCFakeMMapFile) → `v8-base::page_allocator` process RNG | Reclaimed | `src/os.rs` delegates to `v8_base::page_allocator::{os_set_random_mmap_seed, os_get_random_mmap_addr}`; gc_fake_mmap is a process static; covered by the supplement test os_random_mmap_and_fake_mmap |
| MemoryProtectionKey signal handler / stack protection → v8-base trap-handler | Not reclaimed | v8-base has no trap-handler/signal-handling subsystem transposed yet; revisit once that module exists (header note in `src/memory_protection_key.rs` updated) |
| AddressSpaceReservation::SetName (prctl PR_SET_VMA_ANON_NAME) | Done | `set_name` already delegates to `OS::set_memory_region_name`, Linux ≥5.17 only, false elsewhere (upstream semantics) |
