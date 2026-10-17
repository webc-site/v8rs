# v8-base-platform 开发日志

> 2026-10-12 收敛: 自 readme/{zh,en}.md 台账迁入 (仅中文)。看板 (todo.md) 只留状态。

## readme 台账 (2026-10-12 自 readme/{zh,en}.md 收敛)

### 对抗审查 round1（2026-10-11，缺陷就地修复 + 登记补全）

修复 13 处（原版引用 → Rust 引用）：
| # | 缺陷 | 原版 | Rust | 修复 |
|---|---|---|---|---|
| 1 | QOS 常量错：USER_INITIATED 误用 0x21（实为 USER_INTERACTIVE；SDK 实测 0x19） | platform-posix.cc:1283-1287 | src/thread.rs apply_priority | 0x19 |
| 2 | GetUserTime 语义错：用墙钟而非 getrusage 用户 CPU 时间 | platform-posix.cc:952-960 | src/os.rs get_user_time | getrusage(RUSAGE_SELF).ru_utime |
| 3 | reservation::create 把整块 set_permissions(max_permission)（应保持 PROT_NONE；max_permission 只决定 MAP_JIT）且忽略 alignment | platform-posix.cc:710-729 | src/reservation.rs create | 走 OS::allocate + NoAccessWillJitLater 回退 |
| 4 | reservation::allocate 用 MAP_FIXED 重映射（原版仅 SetPermissions；kNoAccess 早退 true） | platform-posix.cc:1150-1161 | src/reservation.rs allocate | set_permissions 语义 |
| 5 | reservation::free 吞错误恒 true | platform-posix.cc:1163-1166 | src/reservation.rs free | 透传 decommit 结果 |
| 6 | MAP_NORESERVE 漏 NoAccessWillJitLater | platform-posix.cc:142-150 | src/os.rs mmap_flags_for_access | matches! 两分支 |
| 7 | recommit_pages 额外 mprotect（原版只 madvise / 直接 true） | platform-posix.cc:621-630 | src/os.rs recommit_pages | 移除 set_permissions |
| 8 | remap_pages 额外 mprotect + darwin 缺 target 校验 + cur_protection 未按请求初始化（交集后变 NONE） | platform-darwin.cc:190-214 | src/os.rs remap_pages | req_prot 初始化 + 地址断言 |
| 9 | darwin mmap fd 未用 VM_MAKE_TAG(255)（vmmap 标识丢失） | platform-posix.cc:170-177 | src/os.rs mmap_fd_for_access | 0xFF000000 |
| 10 | 空文件 open/create 返回 None（原版返回有效 (file,null,0)） | platform-posix.cc:869/894 | src/memory_mapped_file.rs | Empty 变体 |
| 11 | allocate 未对 hint 做 AlignedAddress 对齐、缺 size 页对齐 DCHECK | platform-posix.cc:469-471 | src/os.rs allocate | 对齐 + debug_assert |
| 12 | from_timespec 多余 i32::MAX 哨兵（2038 值误判 max） | time.cc:425-437 | src/time.rs from_timespec | 仅 i64::MAX |
| 13 | stack_size 未钳 PTHREAD_STACK_MIN；peak-mem 失败应 -1；ExitProcess 缺 fflush | platform-posix.cc:1232-1233/971/939-945 | src/thread.rs / os.rs | 全部对齐 |

补 API：Mutex/RecursiveMutex::assert_held（mutex.h:58-62/153-157）、
TimeDelta::from_seconds_d/from_milliseconds_d（time.h:97-103）、
LazyConditionVariable + LAZY_CONDITION_VARIABLE_INITIALIZER（condition-variable.h:73-88，
回收 v8-base::lazy_instance）、FileMode 导出。

### 对抗审查 round2（2026-10-11，缺陷就地修复 + 漏登记补全）

修复 6 处（原版引用 → Rust 引用）：
| # | 缺陷 | 原版 | Rust | 修复 |
|---|---|---|---|---|
| 1 | start_synchronously 漏钳 PTHREAD_STACK_MIN（round1 #13 只修了 start 一条路径；原版构造一次两路共享） | platform-posix.cc:1232-1233 | src/thread.rs start_synchronously | 抽 effective_stack_size 两路共用 |
| 2 | darwin stack_size==0 未按原版默认 1MB（std/pthread 默认 512kB） | platform-posix.cc:1325-1329 | src/thread.rs effective_stack_size | darwin 分支默认 1MB |
| 3 | panic 路径：wait_for(TimeDelta::max()) 时 `Instant::now() + dur` 溢出 panic（原版为远未来 timespec 无限等待） | semaphore.cc:82-106 / condition-variable.cc:33-44 | src/semaphore.rs / condition_variable.rs wait_for | checked_add，None → 无限期等待 |
| 4 | MPK ComputeRegisterMaskForPermissionSwitch 签名 (from,to,key) 且返回「变化位掩码」，均偏离原版 (key,permission)→`permission<<(key*2)` | memory-protection-key.h:122-123 / cc:142-153 | src/memory_protection_key.rs | 恢复原版签名与公式 |
| 5 | MPK HasMemoryProtectionKeyAPIs 做实际 pkey_alloc 探测（昂贵且混同 TestKeyAllocation；原版是廉价符号检查、明确不测 CPU/内核支持） | memory-protection-key.cc:46-52 | src/memory_protection_key.rs | cfg(linux x86_64) 恒 true（libc 静态绑定等价符号存在） |
| 6 | darwin RecommitPages 非 EAGAIN 失败返回 Err（原版 while-EAGAIN 后一律 return true） | platform-posix.cc:621-630 | src/os.rs recommit_pages | 对齐恒 Ok |

附带清理：删除死代码 `Mutex::raw` / `RecursiveMutex::raw`（零调用、无原版对应，
原版用 friend 非公开 API）；`OS::time_current_millis` 改走 `Time::to_js_time()`
（原版 cc:984-986 即 `Time::Now().ToJsTime()`，含 null/max 哨兵分支）。

round2 补登记的可接受偏差（登记正文现存于 readme/zh.md「已核实可接受偏差」节）：
- `Semaphore::new(负数)` 钳 0（原版 POSIX `DCHECK_GE(count, 0)`，release 下
  sem_init 静默失败；dispatch 分支不检查）——防御式钳 0 更安全
- `MemoryMappedFile::open(ReadOnly)` 走 memmap2 `Mmap::map`（MAP_SHARED）；
  原版 ReadOnly 为 `MAP_PRIVATE`（platform-posix.cc:874-877）——只读访问不写
  映射，两者可观察行为一致

### 对抗审查 round3（2026-10-11，全新独立视角，缺陷就地修复）

修复 5 处（原版引用 → Rust 引用）：
| # | 缺陷 | 原版 | Rust | 修复 |
|---|---|---|---|---|
| 1 | linux RemapPages 成功后缺 mprotect（round1 #8 误把 darwin「不 mprotect」推广到 linux） | platform-linux.cc:244-245 | src/os.rs remap_pages linux 分支 | mremap 成功后 mprotect(prot)，失败 MProtectFailed |
| 2 | MPK `set_permissions_for_key(addr,size,access,pkey)` 名实不符：名字对应原版 `SetPermissionsForKey`（PKRU 写），实现却是 `SetPermissionsAndKey`（pkey_mprotect），WRPKRU 语义缺失误导消费方（v8-sandbox 被迫自带 wrpkru 副本） | memory-protection-key.h:110 / cc:118-126 | src/memory_protection_key.rs | 删误名函数；新增 `set_key_permissions(key, permission)`（rdpkru 读改写 + wrpkru，不加 nomem 防重排） |
| 3 | `MemoryMappedFile::create` 短 initial 静默补零成功（原版 fwrite 整块 size、短写即 nullptr）；头注释却声称对齐 | platform-posix.cc:888-892 | src/memory_mapped_file.rs create | `initial.len() < size → None` |
| 4 | `Thread::join` 失败误映射 `ThreadStartFailed` | platform.h:607（pthread_join） | src/thread.rs join / error.rs | 新增 `Error::JoinFailed` |
| 5 | `AllocateAtLeast` 失败路径 count=n（原版 {nullptr, 0}；调用方拿 n 长度误用越界） | memory.h:144-163 | src/allocation.rs AllocateAtLeast | count=usable（失败即 0） |

round3 补登记的可接受偏差（登记正文现存于 readme/zh.md「已核实可接受偏差」节）：
- linux RemapPages 的 kernel ≥5.13 门控（KernelSupportsRemapDontUnmap）与
  /proc/self/maps file-backed fallback（platform-linux.cc:175-194/248-292）未迁；
  老内核上 mremap EINVAL → Err（原版 fallback false→false 近似）；成功后 memcmp
  sanity CHECK（禁 panic）不迁
- `OS::get_user_time` 失败返回 (0,0)（原版 return -1 且不写参数；
  platform-posix.cc:952-960）；调用方在 V8 中均忽略返回值
- darwin x86_64 的 PTHREAD_STACK_MIN 实为 8192（SDK limits.h:113-118），
  Rust 统一钳 16384 偏高（安全方向）；arm64/linux 精确
- PTHREAD_STACK_MIN 修正说明已写入 src/thread.rs effective_stack_size 注释

### 终验复核（2026-10-11，测试迁移终验子代理）

- 门禁复跑（配额 `CARGO_TARGET_DIR=target/platform1`）：`cargo check` 0 错 /
  `cargo clippy --all-targets -- -D warnings` 0 警告 / `cargo nextest run --all-features`
  = 38 tests run: 38 passed（37 映射用例 + `tests/main.rs` 接线冒烟 1 例）。
- 盘点结论：`v8/test/unittests/base/platform/` 本 checkout 恰 5 文件（mutex /
  semaphore / condition-variable / platform / time-unittest.cc），上表全迁移，无缺漏；
  本 checkout 不存在 page-allocator-unittest.cc。分配器族测试
  （region-allocator-unittest.cc / virtual-address-space-unittest.cc，位于
  `unittests/base/` 根）归属 v8-base crate——其 page_allocator / region_allocator /
  virtual_address_space 等模块已落地，测试迁移随 v8-base 轮次进行。
- 推迟项逐条复核（对照原版 `v8/src/base/platform` 源码）：9 项跳过理由全部成立
  （4 项 V8_TARGET_OS_LINUX 门控 cc:80-271、SetDataReadOnly ASSERT_DEATH x2 cc:367-368、
  StackAlignment TestWithIsolate cc:386-410、DISABLED_Overflows 原版禁用、
  Filetime / TimerPerformance V8_OS_WIN cc:295/536）；4 个回收点 3 已收 1 待收——
  MemoryProtectionKey → trap-handler 维持不收（v8-trap-handler crate 尚未在
  workspace 建立，`src/memory_protection_key.rs` 头注释同步核实）。

### 已修复的历史红项（原 tests/SPEC.md 缺陷登记）

| 用例 | 修复 |
|---|---|
| mutex_lock_guard_lazy_mutex / mutex_lock_guard_lazy_recursive_mutex | LazyMutex 族导出并回收 v8-base::lazy_instance |
| time_delta_numeric_operators_compound | TimeDelta 实现 MulAssign/DivAssign<i64>（src/time.rs） |
| time_delta_mach_timespec | from_mach_timespec / to_mach_timespec（src/time.rs，V8_OS_DARWIN） |
| thread_ticks_maybe_thread_now | darwin ThreadTicks（THREAD_BASIC_INFO，src/monotonic.rs）+ wait_until_initialized |

### en.md 独有信息核对

readme/en.md 的 round1/round2 头部摘要与 "Final acceptance recheck" 节均为
zh.md 对应小节的英文转述，无独有信息，不另补录。
