#!/usr/bin/env bun
// tool/src/heap/free-list.cc.js — v8/src/heap/free-list.cc → v8-heap-core/src/free_list.rs (实体函数区)
// 规则:
//   ① 函数定义全集 31 (+DEBUG 2 剥除) 逐名行号锚 + constexpr categories_min 静态定义 1
//   ② 关键逻辑体 needle: 链手术/缓存更新/三段快速路径/分派 逐锚 (行号=实测)
//   ③ 展开侧: IsVeryLong/SumFreeLists (#ifdef DEBUG cc:578-600) 剥除; 其余 31 定义留
//   ④ rs 落地锚 (HC-3: free_list.rs 实态对数, 只增不减) + 推迟台账双行号
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/free-list.cc";
const src = readV8(REL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-flcc.ii");
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
const esc = (t) => t.replace(/[.*+?^${}()|[\]\\]/g, (c) => "\\" + c);
const has = (s, re, want, label) => {
  const got = count(s, re);
  if (got !== want) fail(label, got, want);
  n++;
};

// ── ① 函数定义全集 33 (留 31 + DEBUG 2) 逐名+行号 ──────────────────────────
const DEFS = [
  ["void FreeListCategory::Unlink(FreeList* owner)", 22],
  ["void FreeListCategory::Reset(FreeList* owner)", 30],
  ["Tagged<FreeSpace> FreeListCategory::PickNodeFromList(size_t minimum_size,", 36],
  ["Tagged<FreeSpace> FreeListCategory::SearchForNodeInList(const Heap* heap,", 52],
  ["void FreeListCategory::Free(const Heap* heap,", 93],
  ["void FreeListCategory::RepairFreeList(Heap* heap)", 112],
  ["void FreeListCategory::Relink(FreeList* owner)", 126],
  ["FreeList::FreeList(int number_of_categories, size_t min_block_size)", 134],
  ["std::unique_ptr<FreeList> FreeList::CreateFreeList()", 139],
  ["std::unique_ptr<FreeList> FreeList::CreateFreeListForNewSpace()", 143],
  ["Tagged<FreeSpace> FreeList::TryFindNodeIn(FreeListCategoryType type,", 147],
  ["Tagged<FreeSpace> FreeList::SearchForNodeInList(const Heap* heap,", 163],
  ["size_t FreeList::Free(const Isolate* isolate,", 184],
  ["FreeListMany::FreeListMany() : FreeList(kNumberOfCategories, kMinBlockSize)", 211],
  ["FreeListMany::~FreeListMany()", 217],
  ["NormalPage* FreeListMany::GetPageForSize(size_t size_in_bytes)", 219],
  ["Tagged<FreeSpace> FreeListMany::Allocate(const Heap* heap, size_t size_in_bytes,", 233],
  ["FreeListManyCached::FreeListManyCached()", 260],
  ["void FreeListManyCached::Reset()", 262],
  ["void FreeListManyCached::ResetForNonBlackAllocatedPages()", 267],
  ["bool FreeListManyCached::AddCategory(FreeListCategory* category)", 272],
  ["void FreeListManyCached::RemoveCategory(FreeListCategory* category)", 287],
  ["size_t FreeListManyCached::Free(const Isolate* isolate,", 301],
  ["Tagged<FreeSpace> FreeListManyCached::Allocate(const Heap* heap,", 334],
  ["Tagged<FreeSpace> FreeListManyCachedFastPathBase::Allocate(", 375],
  ["Tagged<FreeSpace> FreeListManyCachedOrigin::Allocate(const Heap* heap,", 441],
  ["void FreeList::Reset()", 456],
  ["void FreeList::ResetForNonBlackAllocatedPages()", 466],
  ["void FreeList::EvictFreeListItems(NormalPage* page)", 485],
  ["void FreeList::RepairLists(Heap* heap)", 495],
  ["bool FreeList::AddCategory(FreeListCategory* category)", 500],
  ["void FreeList::RemoveCategory(FreeListCategory* category)", 519],
  ["void FreeList::PrintCategories(FreeListCategoryType type)", 542],
  ["size_t FreeListCategory::SumFreeList()", 553],
  ["int FreeListCategory::FreeListLength()", 568],
  ["bool FreeList::IsVeryLong()", 579],
  ["size_t FreeList::SumFreeLists()", 594],
];
for (const [sig] of DEFS) has(src, new RegExp(esc(sig), "g"), 1, `定义 ${sig.slice(0, 46)}`);
has(src, /constexpr unsigned int FreeListMany::categories_min\[kNumberOfCategories\];/, 1, "categories_min 静态定义 cc:209");
has(src, /#ifdef DEBUG/g, 6, "DEBUG 守护 6 区 (cc:280/296/324/360/430/578)");

// ── ② 关键逻辑体 needle (行号=实测, rs 侧一一对应) ─────────────────────────
for (const [re, want, label] of [
  [/owner->DecreaseAvailableBytes\(available_\);/, 1, "Unlink 计数器 cc:24"],
  [/set_top\(node->next\(\)\);/, 1, "Pick 弹顶 cc:46"],
  [/\*node_size = 0;/, 1, "Pick 不足回零 cc:43"],
  [/prev_non_evac_node/g, 12, "Search 中段脱链 cc:55-88 (变量 12 见点; g 标志修复: 非 g 时 match 只返首个)"],
  [/is_executable\(\)/, 1, "JIT 通道判据 cc:70 (推迟面)"],
  [/WritableJitPage jit_page/, 1, "JIT FreeRange cc:71 (推迟面)"],
  [/WritableFreeSpace::ForNonExecutableMemory/, 1, "非 JIT 通道 cc:79"],
  [/free_space->SetNext\(heap, writable_free_space, top\(\)\);/, 1, "头插 SetNext cc:99"],
  [/owner->AddCategory\(this\);/g, 2, "AddCategory 触达 cc:107/cc:128"],
  [/make_unique<FreeListManyCachedOrigin>\(\)/, 1, "工厂 1 cc:140"],
  [/make_unique<FreeListManyCachedFastPathForNewSpace>\(\)/, 1, "工厂 2 cc:144"],
  [/if \(category->is_empty\(\)\) \{\s*RemoveCategory\(category\);/, 1, "TryFind 空类摘除 cc:157-159"],
  [/page->free_list_category\(type\)->Free\(isolate->heap\(\), free_space, mode, this\);/g, 2, "页类回查双处 cc:200/cc:318"],
  [/if \(size_in_bytes < min_block_size_\) \{/g, 2, "min_block 废弃双处 cc:192/cc:310"],
  [/page->add_wasted_memory\(size_in_bytes\);/g, 2, "wasted 记页双处 cc:193/cc:311"],
  [/page->add_wasted_memory\(sum\);/, 1, "Evict 废弃 cc:492"],
  [/cat <= last_category_; cat\+\+/, 1, "GetPageForSize 上溯环 cc:223"],
  [/for \(int i = type; i < last_category_ && node.is_null\(\); i\+\+\)/, 1, "Many Allocate 环 cc:239"],
  [/Searching each element of the last category/g, 3, "末类全寻注释 cc:245/cc:350/cc:410"],
  [/next_nonempty_category\[type \+ 1\]/g, 4, "缓存跳空 4 环 (cc:344/387/399/419)"],
  [/UpdateCacheAfterAddition\(category->type_\);/, 1, "Cached AddCategory cc:277"],
  [/UpdateCacheAfterAddition\(type\);/, 1, "Cached Free cc:322"],
  [/UpdateCacheAfterRemoval\(type\);/g, 3, "Cached Removal 触达 cc:293/cc:357/cc:426"],
  [/if \(origin == AllocationOrigin::kGC\)/, 1, "Origin 分派 cc:445"],
  [/FreeListManyCached::Allocate\(heap, size_in_bytes, node_size, origin\);/, 1, "Origin GC 支 cc:446"],
  [/FreeListManyCachedFastPath::Allocate\(heap, size_in_bytes, node_size,/, 1, "Origin 非 GC 支 cc:448"],
  [/category->Unlink\(this\);/, 1, "ResetForNonBlack 黑链脱 cc:473"],
  [/chunk->IsBlackAllocated\(\)/, 1, "chunk 谓词消费 cc:471 (memory_chunk 已落面)"],
  [/kVeryLongFreeList\) return true;/, 1, "IsVeryLong 截断 cc:585"],
  [/PrintF\(/g, 3, "PrintCategories PrintF 3 cc:544-550"],
  [/DCHECK\(v8_flags.black_allocated_pages\)/, 1, "black flag DCHECK cc:467"],
]) {
  has(src, re, want, `逻辑 ${label}`);
}
has(src, /USE\(origin\);/g, 2, "USE(origin) 2 处 cc:338/cc:378 (g 标志修复)");

// ── ③ 展开侧 (目标配置 DEBUG 关) ───────────────────────────────────────────
const expanded = preprocess(REL);
// 展开输出含全量头文本; 泛名锚 (flag/谓词/JIT) 限定 free-list.cc 自身区 (首定义起),
//   防 include 链噪声 (实测本版 v8: flag-definitions.h/paged-spaces.h/memory-chunk 头带同名符号)
const ownStart = expanded.indexOf("void FreeListCategory::Unlink(FreeList* owner)");
const own = ownStart >= 0 ? expanded.slice(ownStart) : expanded;
has(expanded, /bool FreeList::IsVeryLong\(\)/g, 0, "展开剥除 IsVeryLong cc:579");
has(expanded, /size_t FreeList::SumFreeLists\(\)/g, 0, "展开剥除 SumFreeLists cc:594");
for (const [sig] of DEFS) {
  if (sig.includes("IsVeryLong") || sig.includes("SumFreeLists")) continue;
  has(expanded, new RegExp(esc(sig), "g"), 1, `展开定义留 ${sig.slice(0, 40)}`);
}
has(own, /black_allocated_pages/g, 0, "展开自身区 flag 读点剥 (DCHECK cc:467 目标配置全剥; rs 侧 flags 参数注入承接)");
has(own, /IsBlackAllocated\(\)/g, 1, "展开黑链谓词调用留 (自身区)");
has(own, /WritableJitPage/g, 1, "展开 JIT 通道文本留 (自身区; 声明他面, 语义由 env 承接)");

// ── ④ rs 侧落地锚 (HC-3: v8-heap-core/src/free_list.rs 实态对数, 只增不减) ──
const rs = readFileSync(join(root, "v8-heap-core", "src", "free_list.rs"), "utf8");
const RS_FNS = [
  ["pub fn unlink", "Unlink cc:22"],
  ["pub fn pick_node_from_list", "PickNodeFromList cc:36"],
  ["pub fn search_for_node_in_list\\(\\s*&mut self,\\s*env", "Category Search cc:52 (env 形参跨行, \\s* 适配)"],
  ["pub fn free<O: FreeList>", "Category Free cc:93 (签名现代化: owner 泛型化)"],
  ["pub fn relink", "Relink cc:126"],
  ["pub fn create_free_list\\(", "CreateFreeList cc:139 (加 \\) 防前缀吞并 _for_new_space)"],
  ["pub fn create_free_list_for_new_space", "CreateFreeListForNewSpace cc:143"],
  ["fn try_find_node_in", "TryFindNodeIn cc:147 (trait 默认体)"],
  ["fn free<E: FreeListEnv>", "FreeList::Free cc:184 (trait 默认体 + 覆写/宏转接 4)", 5],
  ["unsafe \\{ &mut \\*category \\}", "FreeListCategory* 解引用承接位 (原版裸指针链手术, 5 见点)", 5],
  ["cc:209 类外 constexpr 重复定义", "categories_min 台账注"],
  ["fn get_page_for_size<E: FreeListEnv>", "GetPageForSize cc:219 (纯虚声明 + Many 实体 + 转接 4)", 6],
  ["page_add_wasted_memory\\(page, sum\\); // cc:492", "Evict sum cc:486-492"],
  ["fn reset_impl", "Reset 走链承接 (reset_impl 内联走链 + trait 默认转接)"],
  ["fn add_category_impl", "AddCategory cc:500 (FreeListBase 共享体 + trait 默认转接)"],
  ["fn remove_category_impl", "RemoveCategory cc:519 (FreeListBase 共享体 + trait 默认转接)"],
  ["pub fn print_categories", "PrintCategories cc:542"],
  ["pub fn sum_free_list\\(", "SumFreeList cc:553"],
  ["pub fn free_list_length", "FreeListLength cc:568"],
  ["pub fn sum_free_lists", "SumFreeLists cc:594 (DEBUG 常编承接)"],
  ["pub fn is_very_long", "IsVeryLong cc:579 (同上)"],
]
for (const [re, label, w = 1] of RS_FNS) has(rs, new RegExp(re, "g"), w, `rs 锚 ${label}`);
has(rs, /debug_assert!\(!node.is_null\(\)|debug_assert\(!node.is_null\(\)\)/, 1, "cc:39 DCHECK 弹顶承接");
has(rs, /AtomicUsize/g, 3, "wasted_bytes_ atomic h:283 (use 导入 + 字段声明 + new 三见点; g 标志修复)");
has(rs, /Ordering::Relaxed/g, 5, "relaxed 五处 (load/fetch_add/fetch_sub + Reset 双归零 store; g 标志修复)");
// 推迟台账双行号 (cc 侧)
for (const [needle, label] of [
  ["cc:112-124", "RepairFreeList 推迟"],
  ["cc:495-498", "RepairLists 推迟"],
  ["cc:68-74", "JIT 通道推迟"],
  ["cc:40-41", "CanAllocateOnChunk 推迟"],
  ["cc:201-202", "AvailableInFreeList DCHECK 推迟"],
]) {
  if (count(rs, new RegExp(esc(needle), "g")) < 1) fail(`rs 推迟双行号 ${label}`, 0, ">=1");
  n++;
}
has(rs, /todo!\(/g, 0, "rs todo!() 0 残留");

const ln = (s) => src.split(String.fromCharCode(10)).findIndex((l) => l.includes(s)) + 1;
const skel = `${banner("src/heap/free-list.cc.js", REL, "FreeList 实体台账: 37 定义 (目标配置留 35) + 逻辑体 33 needle + USE/DEBUG 区锚 + 展开侧核对")}
// ══ free-list.cc:1-603 → v8-heap-core/src/free_list.rs ══
// ── 函数定义处置 (原版行号实测) ──
${DEFS.map(([sig, l]) => `// cc:${String(l).padEnd(4)} ${sig}`).join("\n")}
// cc:209 constexpr categories_min 再声明 (C++ 链接件) → Rust pub const 即定义处
// cc:578-600 #ifdef DEBUG (IsVeryLong/SumFreeLists) 展开侧剥除 → Rust 常编 pub, debug_assert! 消费侧零求值
// ── 耦合处置 (窄 trait FreeListEnv 承接, 双行号进模块推迟台账) ──
// Isolate*/Heap* 形参 (cc:93/152/163/184/233/301/334/375/441) → env 吸收 (同 chunk 票 MetadataTableEnv 先例)
// NormalPage 反查/计数 (cc:188-193/200/250/306-311/318/365/427/487-492) → env 页面包 7 法
// Tagged<FreeSpace> 读写 (cc:42/46/57/60/90/99/563/572) → env 节点面包 3 法 (objects/free-space.h 他票)
// SetNext JIT 双通道 (cc:68-76 Metadata()->is_executable 判据) → env.set_node_next 实现面; Metadata 族
//   属 memory_chunk 推迟台账 1 (BasePage/IsolateGroup 波次) → 本席登记推迟, 禁假绿
// RepairFreeList/RepairLists (cc:112-124/495-498 ReadOnlyRoots/ObjectSlot store_map) → 推迟 (Heap roots 他票)
// DCHECK(v8_flags.black_allocated_pages) (cc:467) → v8-flags 运行态单例他票, release 剥除注记
// ── 虚拟分派承接 (本仓禁 dyn) ──
// virtual Allocate/Free/AddCategory/RemoveCategory/Select 的虚表回击 (如 cc:107 AddCategory 触达 ManyCached
//   override cc:272; cc:157/175 RemoveCategory 触达 cc:287) → Rust trait FreeList 默认体 + FreeListBase
//   共享体 (*_impl) + 薄壳宏转接 (forward_free_list_impl), 逐触达点与 C++ 分派效果等价 (申报差异)
// ── 工厂/组合链 ──
// CreateFreeList→FreeListManyCachedOrigin (cc:140) / CreateFreeListForNewSpace→…ForNewSpace (cc:144);
//   unique_ptr<FreeList> 多态返回面 → Rust 具体类型返回 (消费面 PagedSpace 他票, 申报差异)
// ForNewSpace kProhibit ctor (h:443-451 读 v8_flags.minor_ms/minor_ms_min_lab_size_kb) → ctor 参数注入
// ── rs 侧落地锚 (HC-3 实测台账) ──
// Landed @ v8-heap-core/src/free_list.rs: 35 留体全落 (RepairFreeList/RepairLists 2 推迟台账);
//   PrintCategories PrintF 流输出面 (cc:544-550) 由链格式化承接 (base::OS 面他票);
//   测试面 tests/free_list_unittest.rs (spaces-unittest.cc:485/540 两可迁用例 + 金标准边界族)。
`;
writeSrc("v8-heap-core", "free_list_cc.skel", skel);
console.log(`OK free-list.cc.js: 对数断言 ${n} 条全过; 定义 37 逐名 + cc:209 表定义 + DEBUG 区 2 + 逻辑体 33 + USE 2 + 展开侧 (剥除 2 / 留 35 / DCHECK 文本 2 / JIT 文本 2) + rs 落地锚 ${RS_FNS.length} + 承接锚 3 + 推迟双行号 5 + todo 0-检; 产物 v8-heap-core/src/free_list_cc.skel`);
