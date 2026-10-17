#!/usr/bin/env bun
// tool/src/heap/memory-pool.h.js — v8/src/heap/memory-pool.h → v8-heap-core/src/memory_pool.rs (类型/声明面台账)
// 规则:
//   ① 源码声明全集逐 needle 唯一性断言 (PooledPage 21 + MemoryPool 外壳/Config 24 +
//      PoolImpl 17 + LargePagePoolImpl 9 + PooledVirtualMemory 5 + ReleaseStats 5 + 字段/断言族)
//   ② clang -E 目标配置展开侧 (CP+SANDBOX+ECS, DEBUG 关): DCHECK 记号 0 残留 / V8_EXPORT_PRIVATE 0 /
//      V8_NOEXCEPT→noexcept / static_assert(is_always_lock_free) 保留 / Config 默认值 5 项逐留
//   ③ rs 侧落地锚 (本席 HC-2a 实测台账, 只增不减): Reservation trait 5 面 + PooledPage/PooledEntry/
//      PoolImpl/LargePagePoolImpl/MemoryPool 类型与成员逐锚 + 推迟臂 0 检 (Create 族/悬空声明)
//   ④ 双行号台账: h 行号 ↔ rs 行号 (逐对查行, 幂等可 cmp 复验)
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/memory-pool.h";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-mp-h.ii");
  for (let i = 0; i < 200; i++) {
    const r = spawnSync("clang", ["-E", "-P", "-x", "c++", "-std=c++20",
      "-DV8_COMPRESS_POINTERS", "-DV8_ENABLE_SANDBOX", "-DV8_EXTERNAL_CODE_SPACE",
      "-I", ".", "-I", "include", "-I", STUB, file, "-o", out], { cwd: V8, encoding: "utf8" });
    if (r.status === 0) return readFileSync(out, "utf8");
    const miss = (r.stderr.match(/fatal error: '([^']+)' file not found/) || [])[1];
    if (!miss) throw new Error(`clang -E ${file} 硬失败:\n${r.stderr}`);
    const p = join(STUB, miss);
    mkdirSync(dirname(p), { recursive: true });
    writeFileSync(p, "#pragma once\n");
  }
  throw new Error(`clang -E ${file} 桩头迭代超限`);
}

let n = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const count = (s, re) => (s.match(re) || []).length;

// ── ① 源码锚 (h 行号 ↔ needle; want 恒 1 除非注明) ─────────────────────────
const H_ANCHORS = [
  // PooledPage (h:28-67)
  ["h:28 class PooledPage final {", /class PooledPage final \{/, 1],
  ["h:31 using Epoch = size_t", /using Epoch = size_t;/, 1],
  ["h:33 struct Result {", /struct Result \{/, 1],
  ["h:34 void\* uninitialized_metadata;", /void\* uninitialized_metadata;/, 1],
  ["h:35 base::AddressRegion uninitialized_chunk;", /base::AddressRegion uninitialized_chunk;/, 1],
  ["h:38 ~PooledPage();", /~PooledPage\(\);/, 1],
  ["h:40 PooledPage\(PooledPage&&\) V8_NOEXCEPT = default;", /PooledPage\(PooledPage&&\) V8_NOEXCEPT = default;/, 1],
  ["h:41 operator=\(PooledPage&&\) V8_NOEXCEPT = default;", /PooledPage& operator=\(PooledPage&&\) V8_NOEXCEPT = default;/, 1],
  ["h:42 copy ctor = delete", /PooledPage\(const PooledPage&\) = delete;/, 1],
  ["h:43 copy assign = delete", /PooledPage& operator=\(const PooledPage&\) = delete;/, 1],
  ["h:45 size\(\) const", /size_t size\(\) const \{ return reservation_\.size\(\); \}/, 1],
  ["h:46 epoch\(\) const", /size_t epoch\(\) const \{ return epoch_; \}/, 1],
  ["h:50 Result ToResult\(\);", /Result ToResult\(\);/, 1],
  ["h:53 friend class MemoryPool;", /friend class MemoryPool;/, 1],
  ["h:57 Create\(NormalPage\*\)", /static PooledPage Create\(NormalPage\* metadata, Epoch epoch\);/, 1],
  ["h:58 Create\(LargePage\*\)", /static PooledPage Create\(LargePage\* metadata, Epoch epoch\);/, 1],
  ["h:60 PooledPage ctor 声明", /PooledPage\(void\* uninitialized_metadata, VirtualMemory reservation,/, 1],
  ["h:63 字段 uninitialized_metadata_;", /void\* uninitialized_metadata_;/, 1],
  ["h:65 字段 reservation_", /VirtualMemory reservation_;/, 1],
  // MemoryPool 外壳 + Config (h:74-151)
  ["h:74 class MemoryPool final {", /class MemoryPool final \{/, 1],
  ["h:75 using Epoch = PooledPage::Epoch", /using Epoch = PooledPage::Epoch;/, 1],
  ["h:81 struct Config final {", /struct Config final \{/, 1],
  ["h:82 single_threaded", /bool single_threaded = false;/, 1],
  ["h:83 share_memory_on_teardown", /bool share_memory_on_teardown = true;/, 1],
  ["h:84 trace_gc_nvp", /bool trace_gc_nvp = false;/, 1],
  ["h:85 max_large_page_pool_size", /size_t max_large_page_pool_size = 32;/, 1],
  ["h:86 timeout_in_sec", /size_t timeout_in_sec = 8;/, 1],
  ["h:89 explicit MemoryPool\(Config\);", /explicit MemoryPool\(Config\);/, 1],
  ["h:90 ~MemoryPool\(\);", /~MemoryPool\(\);/, 1],
  ["h:92-93 MemoryPool copy delete ×2", /MemoryPool\(const MemoryPool&\) = delete;|MemoryPool& operator=\(const MemoryPool&\) = delete;/g, 2],
  ["h:96 CreatePooledPage", /PooledPage CreatePooledPage\(NormalPage\* metadata\);/, 1],
  ["h:99 Add", /void Add\(Isolate\* isolate, MutablePage\* chunk\);/, 1],
  ["h:104 Remove", /std::optional<PooledPage::Result> Remove\(Isolate\* isolate\);/, 1],
  ["h:106 AddLarge", /void AddLarge\(Isolate\* isolate, std::vector<LargePage\*>& pages\);/, 1],
  ["h:109 RemoveLarge", /std::optional<PooledPage::Result> RemoveLarge\(Isolate\* isolate,/, 1],
  ["h:115 AddZoneReservation", /void AddZoneReservation\(Isolate\* isolate, VirtualMemory zone_reservation\);/, 1],
  ["h:120 RemoveZoneReservation", /std::optional<VirtualMemory> RemoveZoneReservation\(Isolate\* isolate\);/, 1],
  ["h:126 ReleaseOnTearDown", /void ReleaseOnTearDown\(Isolate\* isolate\);/, 1],
  ["h:129 ReleaseImmediately", /V8_EXPORT_PRIVATE void ReleaseImmediately\(Isolate\* isolate\);/, 1],
  ["h:132 ReleaseAllImmediately", /void ReleaseAllImmediately\(\);/, 1],
  ["h:138 GarbageCollectionPrologue (悬空声明, 台账 7)", /void GarbageCollectionPrologue\(Isolate\* isolate, GarbageCollector collector\);/, 1],
  ["h:141 GetCount", /size_t GetCount\(Isolate\* isolate\) const;/, 1],
  ["h:144 GetSharedCount", /size_t GetSharedCount\(\) const;/, 1],
  ["h:147 GetTotalCount", /size_t GetTotalCount\(\) const;/, 1],
  ["h:150 CancelAndWaitForTaskToFinishForTesting", /V8_EXPORT_PRIVATE void CancelAndWaitForTaskToFinishForTesting\(\);/, 1],
  ["h:151 ReenableTaskForTesting", /V8_EXPORT_PRIVATE void ReenableTaskForTesting\(\);/, 1],
  // 私有嵌套类 (h:154-233)
  ["h:154 class ReleasePooledChunksTask;", /class ReleasePooledChunksTask;/, 1],
  ["h:156 struct PoolReleaseStats {", /struct PoolReleaseStats \{/, 1],
  ["h:157 removed_entries", /size_t removed_entries = 0;/, 1],
  ["h:161 template <typename PoolEntry>", /template <typename PoolEntry>/, 1],
  ["h:162 class PoolImpl final {", /class PoolImpl final \{/, 1],
  ["h:164 explicit PoolImpl", /explicit PoolImpl\(MemoryPool::Config config\) : config_\(config\) \{\}/, 1],
  ["h:167 ~PoolImpl DCHECK local", /DCHECK\(local_pools_\.empty\(\)\);/, 1],
  ["h:168 ~PoolImpl DCHECK shared", /DCHECK\(shared_pool_\.empty\(\)\);/, 1],
  ["h:171 PutLocal", /void PutLocal\(Isolate\* isolate, PoolEntry entry\);/, 1],
  ["h:172 Get", /std::optional<PoolEntry> Get\(Isolate\* isolate\);/, 1],
  ["h:173 MoveLocalToShared", /bool MoveLocalToShared\(Isolate\* isolate\);/, 1],
  ["h:174 ReleaseShared", /void ReleaseShared\(\);/, 1],
  ["h:175 ReleaseLocal 无参", /void ReleaseLocal\(\);/, 1],
  ["h:176 ReleaseLocal isolate 参", /void ReleaseLocal\(Isolate\* isolate\);/, 1],
  ["h:177 PoolImpl ReleaseUpTo", /PoolReleaseStats ReleaseUpTo\(Epoch release_epoch\);/, 1],
  ["h:179 void TearDown\(\);", /void TearDown\(\);/, 1],
  ["h:181 Size", /size_t Size\(\) const;/, 1],
  ["h:182 LocalSize", /size_t LocalSize\(Isolate\* isolate\) const;/, 1],
  ["h:183 SharedSize", /size_t SharedSize\(\) const;/, 1],
  ["h:186 PoolImpl/Large config_ 字段 ×2", /const MemoryPool::Config config_;/g, 2],
  ["h:187 local_pools_ 字段", /absl::flat_hash_map<Isolate\*, std::vector<PoolEntry>> local_pools_;/, 1],
  ["h:188 shared_pool_ 字段", /std::vector<PoolEntry> shared_pool_;/, 1],
  ["h:189 mutable base::Mutex mutex_;", /mutable base::Mutex mutex_;/, 1],
  ["h:192 class LargePagePoolImpl final {", /class LargePagePoolImpl final \{/, 1],
  ["h:194 explicit LargePagePoolImpl", /explicit LargePagePoolImpl\(MemoryPool::Config config\) : config_\(config\) \{\}/, 1],
  ["h:196 ~LargePagePoolImpl DCHECK pages", /~LargePagePoolImpl\(\) \{ DCHECK\(pages_\.empty\(\)\); \}/, 1],
  ["h:198 Large Add", /bool Add\(std::vector<LargePage\*>& pages, Epoch epoch\);/, 1],
  ["h:199 Large Remove", /std::optional<PooledPage::Result> Remove\(Isolate\* isolate,/, 1],
  ["h:201 ReleaseAll", /void ReleaseAll\(\);/, 1],
  ["h:205 ComputeTotalSize", /size_t ComputeTotalSize\(\) const;/, 1],
  ["h:209 base::Mutex mutex_ (含 mutable 前缀行, 共 2)", /base::Mutex mutex_;/g, 2],
  ["h:210 pages_ 字段", /std::vector<PooledPage> pages_;/, 1],
  ["h:211 total_size_", /size_t total_size_ = 0;/, 1],
  ["h:214 class PooledVirtualMemory final {", /class PooledVirtualMemory final \{/, 1],
  ["h:216 PooledVirtualMemory ctor", /PooledVirtualMemory\(VirtualMemory memory, Epoch epoch\)/, 1],
  ["h:219 virtual_memory\(\)", /VirtualMemory& virtual_memory\(\) \{ return memory_; \}/, 1],
  ["h:224 memory_ 字段", /VirtualMemory memory_;/, 1],
  ["h:228 struct ReleaseStats final {", /struct ReleaseStats final \{/, 1],
  ["h:229 pages_removed", /size_t pages_removed = 0;/, 1],
  ["h:230 large_pages_removed", /size_t large_pages_removed = 0;/, 1],
  ["h:231 zone_reservations_removed", /size_t zone_reservations_removed = 0;/, 1],
  // MemoryPool 字段/静态面 (h:235-254)
  ["h:235 PostDelayedReleaseTask", /void PostDelayedReleaseTask\(Isolate\* isolate, base::TimeDelta delay\);/, 1],
  ["h:236 PostDelayedReleaseTaskIfNeeded", /void PostDelayedReleaseTaskIfNeeded\(Isolate\* isolate\);/, 1],
  ["h:238 const Config config_;", /const Config config_;/, 1],
  ["h:239 task_disabled_", /std::atomic<bool> task_disabled_ = false;/, 1],
  ["h:241 page_pool_", /PoolImpl<PooledPage> page_pool_;/, 1],
  ["h:242 zone_pool_", /PoolImpl<PooledVirtualMemory> zone_pool_;/, 1],
  ["h:244 large_pool_", /LargePagePoolImpl large_pool_;/, 1],
  ["h:248 MemoryPool ReleaseUpTo 声明", /ReleaseStats ReleaseUpTo\(Epoch release_epoch\);/, 1],
  ["h:250 current_epoch_", /std::atomic<Epoch> current_epoch_\{0\};/, 1],
  ["h:251 posted_time_", /std::atomic<base::TimeTicks> posted_time_;/, 1],
  ["h:252 cancellable_task_manager_", /std::unique_ptr<CancelableTaskManager> cancellable_task_manager_;/, 1],
  ["h:254 static_assert is_always_lock_free", /static_assert\(std::atomic<base::TimeTicks>::is_always_lock_free\);/, 1],
  ["h:15-20 前向声明 6", /^class [A-Za-z]+;$/gm, 6],
  ["h Epoch epoch_ (PooledPage+PooledVirtualMemory)", /Epoch epoch_;/g, 2],
  ["h pool_emptied (PoolReleaseStats+ReleaseStats)", /bool pool_emptied = false;/g, 2],
  ["h TearDown 声明 3 (MemoryPool+PoolImpl+Large)", /void TearDown\(\);/g, 3],
  ["h 宏记号 DCHECK (源码侧 3)", /DCHECK/g, 3],
];
for (const [label, re, want] of H_ANCHORS) {
  const got = count(src, re);
  if (got !== want) fail(`h 锚 ${label}`, got, want);
  n++;
}

// ── ② clang -E 目标配置展开侧 ──────────────────────────────────────────────
const expanded = preprocess(REL);
// 全文记号: DCHECK/CHECK/V8_EXPORT_PRIVATE/V8_NOEXCEPT 展开后 0 残留 (DEBUG 关)
for (const [label, re, want] of [
  ["展开 DCHECK 残留", /DCHECK/g, 0],
  ["展开 CHECK 残留", /CHECK/g, 0],
  ["展开 V8_EXPORT_PRIVATE 残留", /V8_EXPORT_PRIVATE/g, 0],
  ["展开 V8_NOEXCEPT 残留", /V8_NOEXCEPT/g, 0],
  ["展开 noexcept (V8_NOEXCEPT 代入)", /PooledPage\(PooledPage&&\) noexcept = default;/, 1],
  ["展开 PooledPage 类锚", /class PooledPage final \{/, 1],
  ["展开 MemoryPool 类锚", /class MemoryPool final \{/, 1],
  ["展开 Config 锚", /struct Config final \{/, 1],
  ["展开 Config 默认值 5 项", /bool single_threaded = false;|bool share_memory_on_teardown = true;|bool trace_gc_nvp = false;|size_t max_large_page_pool_size = 32;|size_t timeout_in_sec = 8;/g, 5],
  ["展开 Create 重载 ×2", /static PooledPage Create/g, 2],
  ["展开 friend class MemoryPool", /friend class MemoryPool;/, 1],
  ["展开 LargePagePoolImpl 锚", /class LargePagePoolImpl final/, 1],
  ["展开 PooledVirtualMemory 锚", /class PooledVirtualMemory final/, 1],
  ["展开 ReleaseStats 锚", /struct ReleaseStats final/, 1],
  ["展开 flat_hash_map 字段 (absl 桩保留原名)", /absl::flat_hash_map<Isolate\*, std::vector<PoolEntry>> local_pools_;/, 1],
  ["展开 mutable mutex_", /mutable base::Mutex mutex_;/, 1],
  ["展开 static_assert is_always_lock_free (保留)", /std::atomic<base::TimeTicks>::is_always_lock_free/, 1],
  ["展开 GarbageCollectionPrologue (声明仍在, 无实体=台账 7)", /void GarbageCollectionPrologue\(Isolate\* isolate, GarbageCollector collector\);/, 1],
  ["展开 ~PoolImpl 体内 DCHECK 剥除→(void)0 ×2", /~PoolImpl\(\) \{\s*\(\(void\) 0\);\s*\(\(void\) 0\);\s*\}/, 1],
  ["展开 ~LargePagePoolImpl 空化 (DCHECK→(void)0)", /~LargePagePoolImpl\(\) \{ \(\(void\) 0\); \}/, 1],
]) {
  const got = count(expanded, re);
  if (got !== want) fail(`展开锚 ${label}`, got, want);
  n++;
}

// ── ③④ rs 侧落地锚 + 双行号台账 (h 行号 ↔ rs 行号; 实测 v8-heap-core/src/memory_pool.rs) ──
const rs = readFileSync(join(root, "v8-heap-core", "src", "memory_pool.rs"), "utf8");
const rsLines = rs.split(String.fromCharCode(10));
const rsLn = (needle) => {
  const hits = rsLines.filter((l) => l.includes(needle));
  if (hits.length !== 1) fail(`rs 查行 ${needle}`, hits.length, 1);
  n++;
  return rsLines.findIndex((l) => l.includes(needle)) + 1;
};
// [h 行号, 面名, rs 唯一 needle] — 双行号台账逐对
const LEDGER = [
  [31, "Epoch 别名", "pub type Epoch = usize;"],
  [16, "Isolate 前向声明承接", "pub enum Isolate {}"],
  [60, "PooledPage ctor/承接位", "pub fn new(uninitialized_metadata: *mut c_void"],
  [63, "字段 uninitialized_metadata", "pub uninitialized_metadata: *mut c_void,"],
  [65, "字段 reservation (VirtualMemory→trait)", "pub reservation: R,"],
  [66, "字段 epoch", "/// h:66 `Epoch epoch_`。"],
  [45, "PooledPage::size", "/// 原版 h:45 `size_t size() const { return reservation_.size(); }`。"],
  [46, "PooledPage::epoch", "/// 原版 h:46 `size_t epoch() const { return epoch_; }`。"],
  [50, "PooledPage::ToResult", "pub fn to_result(&mut self) -> PooledPageResult"],
  [34, "Result.uninitialized_metadata", "pub uninitialized_metadata: Address,"],
  [35, "Result.uninitialized_chunk", "pub uninitialized_chunk: AddressRegion,"],
  [81, "Config 结构", "pub struct Config {"],
  [82, "Config.single_threaded", "pub single_threaded: bool,"],
  [83, "Config.share_memory_on_teardown", "pub share_memory_on_teardown: bool,"],
  [84, "Config.trace_gc_nvp", "pub trace_gc_nvp: bool,"],
  [85, "Config.max_large_page_pool_size", "pub max_large_page_pool_size: usize,"],
  [86, "Config.timeout_in_sec", "pub timeout_in_sec: usize,"],
  [156, "PoolReleaseStats", "pub struct PoolReleaseStats {"],
  [228, "ReleaseStats", "pub struct ReleaseStats {"],
  [162, "PoolImpl 结构", "pub struct PoolImpl<E> {"],
  [187, "local_pools 承接 absl map", "pub local_pools: HashMap<*mut Isolate, Vec<E>>,"],
  [188, "shared_pool", "pub shared_pool: Vec<E>,"],
  [192, "LargePagePoolImpl 结构", "pub struct LargePagePoolImpl<R: Reservation> {"],
  [210, "pages_", "pub pages: Vec<PooledPage<R>>,"],
  [211, "total_size_", "total_size: usize,"],
  [214, "PooledVirtualMemory 结构", "pub struct PooledVirtualMemory<R> {"],
  [219, "virtual_memory()", "pub fn virtual_memory(&mut self) -> &mut R"],
  [221, "PooledVirtualMemory::size", "/// 原版 h:221 `size_t size() const { return memory_.size(); }`。"],
  [154, "ReleasePooledChunksTask 类型", "pub struct ReleasePooledChunksTask {"],
  [74, "MemoryPool 结构", "pub struct MemoryPool<R: Reservation> {"],
  [239, "task_disabled", "task_disabled: AtomicBool,"],
  [241, "page_pool", "page_pool: PoolImpl<PooledPage<R>>,"],
  [242, "zone_pool", "zone_pool: PoolImpl<PooledVirtualMemory<R>>,"],
  [244, "large_pool", "large_pool: LargePagePoolImpl<R>,"],
  [250, "current_epoch", "current_epoch: AtomicUsize,"],
  [251, "posted_time (AtomicI64 承载 µs)", "posted_time: AtomicI64,"],
  [252, "cancellable_task_manager", "cancellable_task_manager: CancelableTaskManager,"],
  [254, "static_assert→const 断言", 'assert!(cfg!(target_has_atomic = "64")'],
];
const rsLedger = LEDGER.map(([h, face, needle]) => `//   h:${String(h).padStart(3)} ↔ rs:${String(rsLn(needle)).padStart(4)}  ${face}`);

// rs 侧对数锚 (trait/方法存在性 + 推迟臂 0 检, 只增不减)
for (const [label, re, want] of [
  ["Reservation trait", /pub trait Reservation \{/, 1],
  ["Reservation 五面 (is_reserved/size/region/free/reset)", /fn is_reserved\(&self\) -> bool;|fn size\(&self\) -> usize;|fn region\(&self\) -> AddressRegion;|fn free\(&mut self\);|fn reset\(&mut self\);/g, 5],
  ["PooledEntry trait (h:161 模板形参约束)", /pub trait PooledEntry \{/, 1],
  ["ReleaseTaskHost trait (h:235-236 投递/打点宿主面)", /pub trait ReleaseTaskHost \{/, 1],
  ["PooledPage Drop (h:38 ~PooledPage)", /impl<R: Reservation> Drop for PooledPage<R> \{/, 1],
  ["PoolImpl Drop (h:166-169 DCHECK 双空)", /impl<E> Drop for PoolImpl<E> \{/, 1],
  ["LargePagePoolImpl Drop (h:196)", /impl<R: Reservation> Drop for LargePagePoolImpl<R> \{/, 1],
  ["create_pooled_page 等价承接面 (h:96, 台账 2)", /pub fn create_pooled_page\(&mut self, reservation: R\) -> PooledPage<R>/, 1],
  ["Add (h:99, 台账 3: 参为 Create 产物)", /pub fn add<H: ReleaseTaskHost \+ \?Sized>\(/, 1],
  ["Remove (h:104)", /pub fn remove\(&mut self, isolate: \*mut Isolate\) -> Option<PooledPageResult>/, 1],
  ["AddLarge (h:106, 台账 3)", /pub fn add_large<H: ReleaseTaskHost \+ \?Sized>\(/, 1],
  ["RemoveLarge (h:109)", /pub fn remove_large\(/, 1],
  ["AddZoneReservation (h:115)", /pub fn add_zone_reservation<H: ReleaseTaskHost \+ \?Sized>\(/, 1],
  ["RemoveZoneReservation (h:120)", /pub fn remove_zone_reservation\(&mut self, isolate: \*mut Isolate\) -> Option<R>/, 1],
  ["ReleaseOnTearDown (h:126)", /pub fn release_on_teardown\(&mut self, isolate: \*mut Isolate\)/, 1],
  ["ReleaseImmediately (h:129)", /pub fn release_immediately\(&mut self, isolate: \*mut Isolate\)/, 1],
  ["ReleaseAllImmediately (h:132)", /pub fn release_all_immediately\(&mut self\)/, 1],
  ["TearDown (h:135)", /pub fn tear_down\(&mut self\)/, 1],
  ["GetCount (h:141)", /pub fn get_count\(&self, isolate: \*mut Isolate\) -> usize/, 1],
  ["GetSharedCount (h:144)", /pub fn get_shared_count\(&self\) -> usize/, 1],
  ["GetTotalCount (h:147)", /pub fn get_total_count\(&self\) -> usize/, 1],
  ["CancelAndWaitForTaskToFinishForTesting (h:150)", /pub fn cancel_and_wait_for_task_to_finish_for_testing\(&mut self\)/, 1],
  ["ReenableTaskForTesting (h:151)", /pub fn reenable_task_for_testing\(&mut self\)/, 1],
  ["ReleaseUpTo 三池汇总 (h:248)", /pub fn release_up_to\(&mut self, release_epoch: Epoch\) -> ReleaseStats/, 1],
  ["PostDelayedReleaseTask (h:235)", /pub fn post_delayed_release_task<H: ReleaseTaskHost/, 1],
  ["PostDelayedReleaseTaskIfNeeded (h:236)", /pub fn post_delayed_release_task_if_needed<H: ReleaseTaskHost/, 1],
  ["推迟臂: Create(NormalPage*/LargePage*) 不落地 (台账 1)", /pub fn create_normal|pub fn create_large/, 0],
  ["推迟臂: GarbageCollectionPrologue 不落地 (台账 7)", /pub fn garbage_collection_prologue/, 0],
  ["推迟台账区存在", /推迟台账/, ">=1"],
]) {
  const got = count(rs, re);
  const ok = want === ">=1" ? got >= 1 : got === want;
  if (!ok) fail(`rs 锚 ${label}`, got, want);
  n++;
}

const ln = (s) => src.split(String.fromCharCode(10)).findIndex((l) => l.includes(s)) + 1;
const skel = `${banner("src/heap/memory-pool.h.js", REL, "MemoryPool 头台账: PooledPage 21 锚 + MemoryPool 外壳 26 + PoolImpl 17 + Large 8 + PVM 4 + 字段/断言族; 展开侧 DCHECK/EXPORT/NOEXCEPT 0 残留")
}
// ══ MemoryPool/PooledPage (原版 memory-pool.h:1-259) → v8-heap-core/src/memory_pool.rs ══
// 前向声明 (h:15-20 ×6): CancelableTaskManager 直用 v8-tasks 落地面; Isolate 落 opaque enum
//   (台账 10); LargePage/BasePage/MutablePage/NormalPage 由 Create 族台账 1 覆盖。
// PooledPage (h:28-67): size/epoch/ToResult 全落; Create ×2 (h:57-58) 与 placement 析构面推迟 (台账 1);
//   VirtualMemory reservation_ → Reservation 窄 trait 承接 (台账 6, 5 面全消费)。
// MemoryPool (h:74-255): Config 5 默认值逐项; PoolImpl<E> 模板 → 泛型 + PooledEntry 约束;
//   absl::flat_hash_map → HashMap (台账 9); LargePagePoolImpl best-fit 单一堆;
//   PooledVirtualMemory zone 条目; ReleaseStats/PoolReleaseStats 账目 struct。
// h:254 static_assert(atomic<TimeTicks>::is_always_lock_free) → Rust 无 atomic<TimeTicks> 形态,
//   posted_time_ 以 AtomicI64 承载 µs 位值 + cfg!(target_has_atomic="64") const 断言代入 (台账 11)。
// h:138 GarbageCollectionPrologue — 悬空声明 (v8/src+v8/test 全树 grep 无定义无调用) → 不落地 (台账 7)。
// ── 双行号台账 (h 行号 ↔ rs 行号, 实测查行) ──
${rsLedger.join("\n")}
// ── 原版关键面行号自证 (查行实测) ──
// PooledPage 类体 h:${ln("class PooledPage final {")} / MemoryPool 类体 h:${ln("class MemoryPool final {")}
// Config h:${ln("struct Config final {")} / PoolImpl h:${ln("class PoolImpl final {")}
// LargePagePoolImpl h:${ln("class LargePagePoolImpl final")} / PooledVirtualMemory h:${ln("class PooledVirtualMemory final")}
`;
writeSrc("v8-heap-core", "memory_pool_h.skel", skel);
console.log(`OK memory-pool.h.js: 对数断言 ${n} 条全过; h 源码锚 ${H_ANCHORS.length} + 展开侧锚 20 (DCHECK/CHECK/EXPORT/NOEXCEPT 0 残留 + 空化 dtor 2) + rs 双行号台账 ${LEDGER.length} 查行 + rs 对数锚 29 (含推迟臂 0 检 2); 产物 v8-heap-core/src/memory_pool_h.skel`);
