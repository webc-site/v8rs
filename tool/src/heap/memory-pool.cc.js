#!/usr/bin/env bun
// tool/src/heap/memory-pool.cc.js — v8/src/heap/memory-pool.cc → v8-heap-core/src/memory_pool.rs (实体函数区台账)
// 规则:
//   ① 定义全集 43: PooledPage 5 (ctor/Create×2/dtor/ToResult) + PoolImpl 11 + LargePagePoolImpl 6
//      + MemoryPool 级 21; 另嵌套 ReleasePooledChunksTask 定义类 (cc:445-489) 逐面锚
//   ② 行为族锚: DCHECK 28 / 独立 CHECK 1 / MutexGuard 14 / erase_if 4 / kTimeSlack 2 /
//      total_size_ 恒等式 5 / pool_emptied 8 / epoch()<= 2 / 投递面 / trace 文案
//   ③ clang -E 目标配置展开侧 (CP+SANDBOX+ECS, DEBUG/TSAN 关): DCHECK→((void)0) 0 残留、
//      CHECK 保留 brk 形态条件实体、EXPORT/NOEXCEPT 0; 定义计数 PoolImpl 11 / Large 6
//   ④ rs 双行号台账 (cc 行号 ↔ rs 行号, 逐对查行) + rs 对数锚 (含 0 检推迟臂, 只增不减)
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/memory-pool.cc";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-mppoolcc.ii");
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
const eq = (label, s, re, want) => {
  const got = count(s, re);
  const ok = typeof want === "string" && want.startsWith(">=") ? got >= +want.slice(2) : got === want;
  if (!ok) fail(label, got, want);
  n++;
};

// ── ① 定义全集 (签名 ↔ 原版起始行; 展开侧计数另验) ─────────────────────────
const DEFS = [
  // PooledPage (cc:20-95)
  ["PooledPage::PooledPage(void* uninitialized_metadata, VirtualMemory reservation,", 20],
  ["PooledPage PooledPage::Create(NormalPage* metadata, Epoch epoch) {", 27],
  ["PooledPage PooledPage::Create(LargePage* metadata, Epoch epoch) {", 56],
  ["PooledPage::~PooledPage() {", 76],
  ["PooledPage::Result PooledPage::ToResult() {", 87],
  // MemoryPool::PoolImpl<PoolEntry> (cc:97-234) 11
  ["void MemoryPool::PoolImpl<PoolEntry>::TearDown() {", 98],
  ["void MemoryPool::PoolImpl<PoolEntry>::PutLocal(Isolate* isolate,", 106],
  ["std::optional<PoolEntry> MemoryPool::PoolImpl<PoolEntry>::Get(", 113],
  ["bool MemoryPool::PoolImpl<PoolEntry>::MoveLocalToShared(Isolate* isolate) {", 147],
  ["void MemoryPool::PoolImpl<PoolEntry>::ReleaseShared() {", 161],
  ["void MemoryPool::PoolImpl<PoolEntry>::ReleaseLocal() {", 167],
  ["void MemoryPool::PoolImpl<PoolEntry>::ReleaseLocal(Isolate* isolate) {", 173],
  ["size_t MemoryPool::PoolImpl<PoolEntry>::Size() const {", 179],
  ["size_t MemoryPool::PoolImpl<PoolEntry>::LocalSize(Isolate* isolate) const {", 190],
  ["size_t MemoryPool::PoolImpl<PoolEntry>::SharedSize() const {", 197],
  ["MemoryPool::PoolReleaseStats MemoryPool::PoolImpl<PoolEntry>::ReleaseUpTo(", 203],
  // MemoryPool::LargePagePoolImpl (cc:236-330) 6
  ["bool MemoryPool::LargePagePoolImpl::Add(std::vector<LargePage*>& pages,", 236],
  ["std::optional<PooledPage::Result> MemoryPool::LargePagePoolImpl::Remove(", 260],
  ["void MemoryPool::LargePagePoolImpl::ReleaseAll() {", 290],
  ["MemoryPool::PoolReleaseStats MemoryPool::LargePagePoolImpl::ReleaseUpTo(", 296],
  ["size_t MemoryPool::LargePagePoolImpl::ComputeTotalSize() const {", 320],
  ["void MemoryPool::LargePagePoolImpl::TearDown() { CHECK(pages_.empty()); }", 330],
  // MemoryPool 级 (cc:332-536) 21
  ["PooledPage MemoryPool::CreatePooledPage(NormalPage* metadata) {", 332],
  ["MemoryPool::MemoryPool(Config config)", 337],
  ["MemoryPool::~MemoryPool() = default;", 344],
  ["void MemoryPool::ReleaseOnTearDown(Isolate* isolate) {", 346],
  ["void MemoryPool::ReleaseImmediately(Isolate* isolate) {", 357],
  ["void MemoryPool::ReleaseAllImmediately() {", 363],
  ["void MemoryPool::TearDown() {", 371],
  ["MemoryPool::ReleaseStats MemoryPool::ReleaseUpTo(Epoch release_epoch) {", 379],
  ["size_t MemoryPool::GetCount(Isolate* isolate) const {", 393],
  ["size_t MemoryPool::GetSharedCount() const { return page_pool_.SharedSize(); }", 397],
  ["size_t MemoryPool::GetTotalCount() const { return page_pool_.Size(); }", 399],
  ["void MemoryPool::Add(Isolate* isolate, MutablePage* page) {", 401],
  ["std::optional<PooledPage::Result> MemoryPool::Remove(Isolate* isolate) {", 410],
  ["void MemoryPool::AddLarge(Isolate* isolate, std::vector<LargePage*>& pages) {", 419],
  ["std::optional<PooledPage::Result> MemoryPool::RemoveLarge(Isolate* isolate,", 423],
  ["void MemoryPool::AddZoneReservation(Isolate* isolate,", 428],
  ["std::optional<VirtualMemory> MemoryPool::RemoveZoneReservation(", 437],
  ["void MemoryPool::PostDelayedReleaseTask(Isolate* isolate,", 491],
  ["void MemoryPool::PostDelayedReleaseTaskIfNeeded(Isolate* isolate) {", 514],
  ["void MemoryPool::CancelAndWaitForTaskToFinishForTesting() {", 531],
  ["void MemoryPool::ReenableTaskForTesting() { task_disabled_.store(false); }", 536],
];
for (const [sig] of DEFS) {
  eq(`定义 ${sig.slice(0, 44)}`, src,
    new RegExp(sig.replace(/[().*+?^${}|\[\]\\]/g, (c) => "\\" + c), "g"), 1);
}
eq("PoolImpl 定义前缀总数 11", src, /MemoryPool::PoolImpl<PoolEntry>::/g, 11);
eq("template <typename PoolEntry> 定义处 11", src, /template <typename PoolEntry>/g, 11);
eq("LargePagePoolImpl 定义前缀总数 6", src, /MemoryPool::LargePagePoolImpl::/g, 6);
eq("PooledPage::Create 出现 5 (定义 2 + 调用 251/405/333)", src, /PooledPage::Create/g, 5);

// ── ② 行为族锚 (源码侧) ────────────────────────────────────────────────────
eq("DCHECK 全家 (含 _NOT_NULL/_EQ)", src, /DCHECK/g, 28);
eq("独立 CHECK(pages_.empty()) 1 (非 DCHECK, 目标配置保留)", src, /CHECK\(pages_\.empty\(\)\);/, 1);
eq("base::MutexGuard 14", src, /base::MutexGuard/g, 14);
eq("std::erase_if 4 (218/222/244/302)", src, /std::erase_if/g, 4);
eq("kTimeSlack 2 (声明 517 + 用 522)", src, /kTimeSlack/g, 2);
eq("total_size_ 恒等式 DCHECK_EQ 5 (240/256/264/286/314)", src, /DCHECK_EQ\(total_size_, ComputeTotalSize\(\)\);/g, 5);
eq("total_size_ -= 2 (284/304)", src, /total_size_ -= /g, 2);
eq("pool_emptied 8 (PoolImpl 3 + Large 3 + 汇总/自续 2)", src, /pool_emptied/g, 8);
eq("epoch() <= release_epoch 2 (209/303)", src, /entry\.epoch\(\) <= release_epoch/g, 2);
eq("std::memory_order_relaxed 7", src, /std::memory_order_relaxed/g, 7);
eq("current_epoch_.load 4 (334/406/420/433)", src, /current_epoch_\.load\(std::memory_order_relaxed\)/g, 4);
eq("fetch_add 1 (499)", src, /current_epoch_\.fetch_add\(1, std::memory_order_relaxed\)/, 1);
eq("local_pools_.erase 族 4 (100/138/155/175)", src, /local_pools_\.erase\(/g, 4);
eq("shared_pool_.clear 2 (102/163)", src, /shared_pool_\.clear\(\);/g, 2);
eq("make_move_iterator 2 (153/154)", src, /std::make_move_iterator/g, 2);
eq("non_empty_local_pools 3 (220/224/227)", src, /non_empty_local_pools/g, 3);
eq("Add 预算 MB (242)", src, /config_\.max_large_page_pool_size \* MB/, 1);
eq("Add 预算判 (246)", src, /total_size_ \+ page->size\(\) > max_total_size/, 1);
eq("best-fit continue (271)", src, /page_size < chunk_size/, 1);
eq("best-fit 严格小 (274, 同值取先者)", src, /page_size < selected->size\(\)/, 1);
eq("dtor 面 IsReserved/Free/free 各 1", src, /reservation_\.IsReserved\(\)|reservation_\.Free\(\)|free\(uninitialized_metadata_\);/g, 3);
eq("DiscardSealedMemoryScope 窗口 (81)", src, /DiscardSealedMemoryScope discard_scope\("Deleting MemoryChunk reservation"\);/, 1);
eq("ToResult 面 region/Reset/nullptr 各 1", src, /reservation_\.region\(\)|reservation_\.Reset\(\);|uninitialized_metadata_ = nullptr/g, 3);
eq("task 定义类锚 (445)", src, /class MemoryPool::ReleasePooledChunksTask final : public CancelableTask \{/, 1);
eq("RunInternal (464)", src, /void RunInternal\(\) override \{/, 1);
eq("IsolateGroup 打点选点 (469)", src, /IsolateGroup::current\(\)->FindAnotherIsolateLocked/, 1);
eq("trace 文案头 (472)", src, /"Memory pool: Removed pages: %zu, removed large pages: %zu, "/, 1);
eq("trace 文案尾 (473)", src, /"removed zone reservations: %zu\\n",/, 1);
eq("make_unique 任务 (500)", src, /std::make_unique<ReleasePooledChunksTask>/, 1);
eq("单线程投递 (503)", src, /isolate->task_runner\(\)->PostDelayedTask/, 1);
eq("平台工作线程投递 (508)", src, /V8::GetCurrentPlatform\(\)->PostDelayedTaskOnWorkerThread/, 1);
eq("CTM 挂点选择 2 (451/502)", src, /single_threaded && isolate/g, 2);
eq("PostDelayedReleaseTaskIfNeeded 定义+调用 4", src, /PostDelayedReleaseTaskIfNeeded/g, 4);
eq("CancelAndWait 2 (373/532)", src, /cancellable_task_manager_->CancelAndWait\(\);/g, 2);
eq("ReleaseUpTo 汇总解构 3 (380/382/384)", src, /const auto \[pages_removed, normal_pools_empty\]|const auto \[large_pages_removed, large_pool_empty\]|const auto \[zone_reservations_removed, zone_pool_empty\]/g, 3);
eq("designated-init 返回 (386)", src, /return \{\.pages_removed = pages_removed,/, 1);
eq("posted_time_ store Now (511)", src, /posted_time_\.store\(base::TimeTicks::Now\(\), std::memory_order_relaxed\);/, 1);
eq("posted_time_ load + delta (525)", src, /posted_time_\.load\(std::memory_order_relaxed\) \+ delta/, 1);
eq("task_disabled_ 族 4 (494/519/533/536)", src, /task_disabled_/g, 4);
eq("MemoryPool dtor = default (344)", src, /MemoryPool::~MemoryPool\(\) = default;/, 1);

// ── ③ clang -E 展开侧 (目标配置: DEBUG/TSAN 关) ────────────────────────────
const expanded = preprocess(REL);
eq("展开 DCHECK 0 残留", expanded, /DCHECK/g, 0);
eq("展开 V8_EXPORT_PRIVATE 0 残留", expanded, /V8_EXPORT_PRIVATE/g, 0);
eq("展开 V8_NOEXCEPT 0 残留", expanded, /V8_NOEXCEPT/g, 0);
eq("展开 CHECK 条件实体保留 (brk 形态, cc:330)", expanded,
  /__builtin_expect\(!!\(!\(pages_\.empty\(\)\)\), 0\)/, 1);
eq("展开 PoolImpl 定义 11", expanded, /MemoryPool::PoolImpl<PoolEntry>::/g, 11);
eq("展开 LargePagePoolImpl 定义 6", expanded, /MemoryPool::LargePagePoolImpl::/g, 6);
eq("展开 MutexGuard (含头链)", expanded, /base::MutexGuard/g, 38);
eq("展开 erase_if (含 <algorithm> 实体)", expanded, /std::erase_if/g, 6);
eq("展开 kTimeSlack 2", expanded, /kTimeSlack/g, 2);
eq("展开 MB 预算", expanded, /config_\.max_large_page_pool_size \* MB/, 1);
eq("展开 task 定义类", expanded, /class MemoryPool::ReleasePooledChunksTask final : public CancelableTask \{/, 1);
eq("展开 RunInternal", expanded, /void RunInternal\(\) override \{/, 1);
eq("展开 trace 文案", expanded, /"Memory pool: Removed pages: %zu, removed large pages: %zu, "/, 1);
eq("展开 placement dtor 面 (50/71) 保留 2", expanded, /metadata->~NormalPage\(\);|metadata->~LargePage\(\);/g, 2);
eq("展开 DiscardSealedMemoryScope", expanded, /DiscardSealedMemoryScope discard_scope/, 1);
eq("展开 TearDown cc:101 DCHECK 剥除→(void)0", expanded, /local_pools_\.erase\(nullptr\);\s*\(\(void\) 0\);/, 1);
eq("展开 MoveLocalToShared cc:151 DCHECK 剥除", expanded, /auto it = local_pools_\.find\(isolate\);\s*if \(it != local_pools_\.end\(\)\) \{\s*\(\(void\) 0\);/, 1);
eq("展开 Add cc:240 恒等式 DCHECK 剥除", expanded, /bool added_to_pool = false;\s*base::MutexGuard guard\(&mutex_\);\s*\(\(void\) 0\);/, 1);
eq("展开 Large ReleaseUpTo cc:314 恒等式剥除", expanded, /\(\(void\) 0\);\s*\}\s*return \{entries_to_free\.size\(\), pool_emptied\};/, 1);
eq("展开 best-fit 循环保留 (cc:274)", expanded, /page_size < selected->size\(\)/, 1);
eq("展开 total_size_ 恒等式 DCHECK 0 残留", expanded, /DCHECK_EQ\(total_size_, ComputeTotalSize\(\)\);/g, 0);
eq("展开 MemoryPool 关键定义逐 1 (采样 8)", expanded,
  /MemoryPool::CreatePooledPage|MemoryPool::ReleaseOnTearDown|MemoryPool::ReleaseAllImmediately|MemoryPool::Add\(Isolate\* isolate, MutablePage\* page\)|MemoryPool::Remove\(Isolate\* isolate\)|MemoryPool::PostDelayedReleaseTask\(Isolate\* isolate|MemoryPool::CancelAndWaitForTaskToFinishForTesting\(\)|MemoryPool::MemoryPool\(Config config\)/g, 8);
eq("展开 dtor = default (非 DEBUG 保留)", expanded, /MemoryPool::~MemoryPool\(\) = default;/, 1);
eq("展开 ReenableTaskForTesting 单行实体", expanded, /void MemoryPool::ReenableTaskForTesting\(\) \{ task_disabled_\.store\(false\); \}/, 1);

// ── ④ rs 双行号台账 + 对数锚 (实测 v8-heap-core/src/memory_pool.rs, 只增不减) ──
const rs = readFileSync(join(root, "v8-heap-core", "src", "memory_pool.rs"), "utf8");
const rsLines = rs.split(String.fromCharCode(10));
const rsLn = (needle) => {
  const hits = rsLines.filter((l) => l.includes(needle));
  if (hits.length !== 1) fail(`rs 查行 ${needle}`, hits.length, 1);
  n++;
  return rsLines.findIndex((l) => l.includes(needle)) + 1;
};
// [cc 行号, 面名, rs 唯一 needle] — 实体区双行号台账
const LEDGER = [
  [20, "PooledPage ctor 承接位", "h:60-61 私有 ctor 声明 + cc:20-24"],
  [27, "Create(NormalPage*) 推迟 (台账 1)", "1. PooledPage::Create ×2"],
  [76, "PooledPage dtor → Drop", "/// 原版 cc:76-85"],
  [81, "DiscardSealedMemoryScope 消费", "← cc:81"],
  [84, "free(uninitialized_metadata_) 消费", "← cc:84"],
  [87, "ToResult → to_result", "/// 原版 h:50 声明 + cc:87-95"],
  [98, "PoolImpl::TearDown", "cc:97-103"],
  [106, "PoolImpl::PutLocal", "cc:105-110"],
  [113, "PoolImpl::Get (本地→共享→窃取)", "cc:112-144"],
  [137, "Get 空键即删 (retain 等价)", "桶空即整键移除 (cc:137-139)"],
  [147, "PoolImpl::MoveLocalToShared", "cc:146-158"],
  [161, "PoolImpl::ReleaseShared", "cc:160-164"],
  [167, "PoolImpl::ReleaseLocal 无参", "cc:166-170"],
  [173, "PoolImpl::ReleaseLocal(isolate)", "cc:172-176"],
  [179, "PoolImpl::Size", "cc:178-187"],
  [190, "PoolImpl::LocalSize", "cc:189-194"],
  [197, "PoolImpl::SharedSize", "cc:196-200"],
  [203, "PoolImpl::ReleaseUpTo (锁外释)", "cc:202-234"],
  [209, "epoch <= release_epoch 判据", "(cc:209) 者"],
  [236, "Large::Add (erase_if 预算)", "cc:236-258"],
  [242, "max_large_page_pool_size * MB", "单位 MB, cc:242"],
  [260, "Large::Remove best-fit", "cc:260-288"],
  [290, "Large::ReleaseAll", "cc:290-294"],
  [296, "Large::ReleaseUpTo", "cc:296-318"],
  [320, "Large::ComputeTotalSize", "cc:320-328"],
  [330, "Large::TearDown CHECK 承接", "/// 原版 cc:330"],
  [332, "CreatePooledPage 承接面 (台账 2)", "h:96 声明 + cc:332-335"],
  [337, "MemoryPool ctor (CTM make_unique)", "cc:337-342"],
  [344, "MemoryPool dtor = default 说明", "// 原版 cc:344"],
  [346, "ReleaseOnTearDown", "h:126 声明 + cc:346-355"],
  [357, "ReleaseImmediately", "h:129 声明 + cc:357-361"],
  [363, "ReleaseAllImmediately", "h:132 声明 + cc:363-369"],
  [371, "MemoryPool::TearDown (先取消)", "h:135 声明 + cc:371-377"],
  [379, "MemoryPool::ReleaseUpTo 三池汇总", "cc:379-391"],
  [393, "GetCount", "cc:393-395"],
  [397, "GetSharedCount", "cc:397"],
  [399, "GetTotalCount", "cc:399"],
  [401, "Add (台账 1 产物入参)", "h:99 声明 + cc:401-408"],
  [410, "Remove", "h:104 声明 + cc:410-417"],
  [419, "AddLarge (台账 1)", "h:106 声明 + cc:419-422"],
  [423, "RemoveLarge", "h:109-110 声明 + cc:423-426"],
  [428, "AddZoneReservation", "h:115 声明 + cc:428-435"],
  [437, "RemoveZoneReservation", "h:120 声明 + cc:437-443"],
  [445, "ReleasePooledChunksTask 类型", "h:154 前向声明 + cc:445-489"],
  [464, "RunInternal → run", "cc:464-483"],
  [472, "trace 文案 → ReleaseStats::message", "原版 cc:472-475 trace_gc_nvp"],
  [491, "PostDelayedReleaseTask", "h:235 声明 + cc:491-512"],
  [514, "PostDelayedReleaseTaskIfNeeded", "h:236 声明 + cc:514-529"],
  [517, "kTimeSlack 常量", "/// cc:517"],
  [531, "CancelAndWaitForTaskToFinishForTesting", "cc:531-534"],
  [536, "ReenableTaskForTesting", "cc:536"],
];
const rsLedger = LEDGER.map(([cc, face, needle]) => `//   cc:${String(cc).padStart(3)} ↔ rs:${String(rsLn(needle)).padStart(4)}  ${face}`);

// rs 侧对数锚 (实体行为等价 + 0 检推迟臂)
eq("锁外释放等价 (entries_to_free 出作用域)", rs, /entries_to_free 出作用域即释放/g, 2);
eq("空键即删 retain 等价 (cc:219-227)", rs, /self\.local_pools\.retain\(/, 1);
eq("Get 非空 DCHECK 承接 ×2 (cc:134/151)", rs, /debug_assert!\(!entries\.is_empty\(\)/g, 2);
eq("best-fit min_by_key (同值取先者)", rs, /min_by_key/, 1);
eq("CHECK 承接走 check_with_msg (v8 fatal, 非 panic)", rs, /check_with_msg\(/, 1);
eq("fetch_add 取旧值 release_epoch (cc:499)", rs, /current_epoch\.fetch_add\(/, 1);
eq("kTimeSlack 承接 (定义+消费)", rs, /K_TIME_SLACK_SECONDS/g, 2);
eq("trace 文案 %zu×3 → {} 格式化", rs, /Removed pages/, 1);
eq("自投面由宿主承接 (台账 4): run 泛型", rs, /pub fn run<R/, 1);
eq("单线程挂点判断承接 (cc:451)", rs, /config\.single_threaded && !self\.isolate\.is_null\(\)/, 1);
eq("0 检: 无 dyn (本仓禁 dyn)", rs, /\bdyn\b/, 0);
eq("0 检: 无 todo!/unimplemented!", rs, /\btodo!|\bunimplemented!/, 0);
// 0 检只查代码行: 剥除 // 起首注释行 (doc 注释合法转述原版 task_runner 文案, 不算落地)
const rsCode = rs.split("\n").filter((l) => !l.trim().startsWith("//")).join("\n");
eq("0 检: 真实 task_runner/Platform 投递不落地 (台账 4)", rsCode, /fn post_delayed_task_on_worker_thread|\.task_runner\(\)|GetCurrentPlatform/, 0);
eq("推迟台账区存在 (cc 侧双行号)", rs, /推迟台账/, ">=1");

const skel = `${banner("src/heap/memory-pool.cc.js", REL, "MemoryPool 实体台账: 定义 43 (PooledPage 5 + PoolImpl 11 + Large 6 + MemoryPool 21) + 嵌套 task 类; 展开侧 DCHECK 0 残留/CHECK brk 保留/恒等式剥除 4 区")
}
// ══ memory-pool.cc 处置台账 (原版行号实测; 展开侧=目标配置 CP+SANDBOX+ECS, DEBUG/TSAN 关) ══
${DEFS.map(([sig, ln]) => `// cc:${String(ln).padStart(4)} ${sig}`).join("\n")}
// 展开实测: DCHECK ×28 → ((void)0) (TearDown/MoveLocalToShared/Add/Large-ReleaseUpTo 四区逐验);
//   独立 CHECK(pages_.empty()) cc:330 目标配置保留 → __builtin_expect+brk 条件实体 (台账内逐形);
//   placement dtor (cc:49-50/70-71) 与 DiscardSealedMemoryScope (cc:81) 文本保留 (Create 族推迟 台账 1);
//   MemoryPool::~MemoryPool() = default (cc:344) 非 DEBUG 保留 → Rust 字段递归落释无自定义 Drop。
// 行为族: MutexGuard 14 → mutex.lock_guard() 作用域; erase_if 4 → 索引循环/retain 等价 (空键即删,
//   预算内入池移除入参、超预算原地留下); total_size_ 恒等式 5 → debug_assert 反证;
//   epoch() <= release_epoch 2 → 锁外 entries_to_free 出作用域释放 (cc:232/316 注释语义);
//   best-fit 严格小于 (cc:274) → min_by_key 同值取先者; Remove 形参 isolate 原版未消费 (cc:261)。
// 心跳任务: 真实投递 (cc:502-510) 与 IsolateGroup 打点选点 (cc:469-476) 属他票 → ReleaseTaskHost
//   窄 trait (台账 4/5); 文案面 cc:472-475 已落 ReleaseStats::message; CTM 直用 v8-tasks 落地面。
// ── 双行号台账 (cc 行号 ↔ rs 行号, 实测查行 ${LEDGER.length} 对) ──
${rsLedger.join("\n")}
`;
writeSrc("v8-heap-core", "memory_pool_cc.skel", skel);
console.log(`OK memory-pool.cc.js: 对数断言 ${n} 条全过; 定义 ${DEFS.length} 逐名 + 前缀/出现锚 4 + 行为族锚 40 + 展开侧 26 (DCHECK 0 残留 / CHECK brk 保留 / 剥除区 4 / 定义 11+6+8) + rs 双行号台账 ${LEDGER.length} 查行 + rs 对数锚 14 (含 0 检 3); 产物 v8-heap-core/src/memory_pool_cc.skel`);
