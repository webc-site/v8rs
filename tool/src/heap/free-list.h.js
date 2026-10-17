#!/usr/bin/env bun
// tool/src/heap/free-list.h.js — v8/src/heap/free-list.h + free-list-inl.h → v8-heap-core/src/free_list.rs (头台账)
// 规则:
//   ① 类声明 8 + 类型/常量/FreeMode 逐锚 (新版无 SpaceSpecificFreeList 模板族 — 票面旧叙事证伪, 见 skel 差异申报区)
//   ② FreeListCategory 成员 21 needle (含 inl 4 实体) + friend 4
//   ③ FreeList 基类成员 34 needle + friend 5 + FreeListCategoryIterator 内部类
//   ④ Many/Cached/FastPathBase 族: categories_min 24 逐值 + kFastPath 常量 5 + static_assert 2 + FRIEND_TEST 3
//   ⑤ 目标配置展开侧: DEBUG 区 (SumFreeLists/IsVeryLong/CheckCacheIntegrity) 剥除 + V8_EXPORT_PRIVATE 剥除 + 类体 8 留
//   ⑥ rs 落地锚 (HC-3: free_list.rs 实态对数, 只增不减) + 推迟台账区
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const V8 = join(root, "v8");
const REL = "src/heap/free-list.h";
const REL_INL = "src/heap/free-list-inl.h";
const src = readV8(REL);
const srcInl = readV8(REL_INL);
const STUB = join(tmpdir(), "v8rs-heap-stub");
mkdirSync(STUB, { recursive: true });
function preprocess(file) {
  const out = join(tmpdir(), "v8rs-heap-flh.ii");
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

// ── ① 类声明 8 + 类型/常量 ─────────────────────────────────────────────────
const CLASSES = [
  "class FreeListCategory {", "class FreeList {", "class V8_EXPORT_PRIVATE FreeListMany : public FreeList {",
  "class V8_EXPORT_PRIVATE FreeListManyCached : public FreeListMany {",
  "class V8_EXPORT_PRIVATE FreeListManyCachedFastPathBase\n    : public FreeListManyCached {",
  "class FreeListManyCachedFastPath : public FreeListManyCachedFastPathBase {",
  "class FreeListManyCachedFastPathForNewSpace\n    : public FreeListManyCachedFastPathBase {",
  "class V8_EXPORT_PRIVATE FreeListManyCachedOrigin\n    : public FreeListManyCachedFastPath {",
];
for (const c of CLASSES) has(src, new RegExp(esc(c), "g"), 1, `类声明 ${c.slice(0, 40)}`);
// 旧叙事证伪锚: SpaceSpecificFreeList / EnsureStarted / Push / Pop / Advance / PadAndSetLink /
// ComputeRequiredWaste / Mutex 在本仓 free-list.h/cc 均 0 存在 (见 skel 差异申报区)
for (const ghost of ["SpaceSpecificFreeList", "EnsureStarted", "PadAndSetLink", "ComputeRequiredWaste", "UncommittedLength", "CommittedLength"]) {
  has(src, new RegExp(esc(ghost), "g"), 0, `旧叙事缺席 ${ghost}`);
  has(readV8("src/heap/free-list.cc"), new RegExp(esc(ghost), "g"), 0, `旧叙事缺席 cc/${ghost}`);
}
has(src, /using FreeListCategoryType = int32_t;/, 1, "using FreeListCategoryType");
has(src, /static constexpr FreeListCategoryType kFirstCategory = 0;/, 1, "kFirstCategory");
has(src, /static constexpr FreeListCategoryType kInvalidCategory = -1;/, 1, "kInvalidCategory");
has(src, /enum FreeMode \{ kLinkCategory, kDoNotLinkCategory \};/, 1, "FreeMode 枚举");

// ── ② FreeListCategory 成员 21 + 字段 5 + friend 4 ─────────────────────────
const CAT_FNS = [
  ["void Initialize\\(FreeListCategoryType type\\)", 1], ["void Unlink\\(FreeList\\* owner\\);", 1],
  ["void Reset\\(FreeList\\* owner\\);", 1], ["void RepairFreeList\\(Heap\\* heap\\);", 1],
  ["void Relink\\(FreeList\\* owner\\);", 1], ["void Free\\(const Heap\\* heap, const WritableFreeSpace& writable_free_space,", 1],
  ["Tagged<FreeSpace> PickNodeFromList\\(size_t minimum_size,", 1],
  ["Tagged<FreeSpace> SearchForNodeInList\\(const Heap\\* heap, size_t minimum_size,", 1],
  ["inline bool is_linked\\(FreeList\\* owner\\) const;", 1], ["bool is_empty\\(\\) \\{ return top\\(\\)\\.is_null\\(\\); \\}", 1],
  ["uint32_t available\\(\\) const", 1], ["size_t SumFreeList\\(\\);", 1], ["int FreeListLength\\(\\);", 1],
  ["void IterateNodesForTesting\\(Callback callback\\)", 1], ["static constexpr int kVeryLongFreeList = 500;", 1],
  ["inline void UpdateCountersAfterAllocation\\(size_t allocation_size\\);", 1],
  ["Tagged<FreeSpace> top\\(\\) \\{ return top_; \\}", 1], ["void set_top\\(Tagged<FreeSpace> top\\)", 1],
  ["FreeListCategory\\* prev\\(\\)", 1], ["FreeListCategory\\* next\\(\\)", 1],
  ["void set_prev\\(FreeListCategory\\* prev\\)", 1], ["void set_next\\(FreeListCategory\\* next\\)", 1],
];
for (const [re, want] of CAT_FNS) has(src, new RegExp(re, "g"), want, `Category 锚 ${re.slice(0, 42)}`);
for (const f of ["FreeListCategoryType type_ = kInvalidCategory;", "uint32_t available_ = 0;", "Tagged<FreeSpace> top_;", "FreeListCategory\\* prev_ = nullptr;", "FreeListCategory\\* next_ = nullptr;"]) {
  has(src, new RegExp(f, "g"), 1, `Category 字段 ${f.slice(0, 40)}`);
}
const catRegion = src.slice(src.indexOf("class FreeListCategory {"), src.indexOf("// A free list maintains"));
has(catRegion, /friend class/g, 4, "Category friend 4");

// ── ③ FreeList 基类成员 34 + inl 实体 4 + friend 5 ─────────────────────────
const FL_FNS = [
  ["static std::unique_ptr<FreeList> CreateFreeList\\(\\);", 1],
  ["static std::unique_ptr<FreeList>\\s*CreateFreeListForNewSpace\\(\\);", 1],
  ["FreeList\\(int number_of_categories, size_t min_block_size\\);", 1], ["virtual ~FreeList\\(\\) = default;", 1],
  ["virtual size_t Free\\(const Isolate\\* isolate,\\s*const WritableFreeSpace& free_space, FreeMode mode\\);", 1],
  ["virtual V8_WARN_UNUSED_RESULT Tagged<FreeSpace> Allocate\\(\\s*const Heap\\* heap, size_t size_in_bytes, size_t\\* node_size,\\s*AllocationOrigin origin\\) = 0;", 1],
  ["V8_EXPORT_PRIVATE virtual NormalPage\\* GetPageForSize\\(\\s*size_t size_in_bytes\\) = 0;", 1],
  ["virtual void Reset\\(\\);", 1], ["virtual void ResetForNonBlackAllocatedPages\\(\\);", 1],
  ["size_t Available\\(\\) \\{", 1], ["void IncreaseAvailableBytes\\(size_t bytes\\) \\{ available_ \\+= bytes; \\}", 1],
  ["void DecreaseAvailableBytes\\(size_t bytes\\) \\{ available_ -= bytes; \\}", 1],
  ["size_t wasted_bytes\\(\\) const", 1], ["void increase_wasted_bytes\\(size_t bytes\\)", 1], ["void decrease_wasted_bytes\\(size_t bytes\\)", 1],
  ["inline bool IsEmpty\\(\\);", 1], ["void RepairLists\\(Heap\\* heap\\);", 1],
  ["V8_EXPORT_PRIVATE void EvictFreeListItems\\(NormalPage\\* page\\);", 1],
  ["int number_of_categories\\(\\) \\{ return number_of_categories_; \\}", 1], ["FreeListCategoryType last_category\\(\\) \\{ return last_category_; \\}", 1],
  ["size_t min_block_size\\(\\) const \\{ return min_block_size_; \\}", 1],
  ["void ForAllFreeListCategories\\(FreeListCategoryType type, Callback callback\\)", 1],
  ["void ForAllFreeListCategories\\(Callback callback\\)", 1],
  ["virtual bool AddCategory\\(FreeListCategory\\* category\\);", 1],
  ["virtual V8_EXPORT_PRIVATE void RemoveCategory\\(FreeListCategory\\* category\\);", 1],
  ["void PrintCategories\\(FreeListCategoryType type\\);", 1],
  ["class FreeListCategoryIterator final \\{", 1], ["bool HasNext\\(\\) const \\{ return current_ != nullptr; \\}", 1],
  ["V8_EXPORT_PRIVATE size_t SumFreeLists\\(\\);", 1], ["V8_EXPORT_PRIVATE bool IsVeryLong\\(\\);", 1],
  ["void VerifyAvailable\\(\\) \\{", 1], ["Tagged<FreeSpace> TryFindNodeIn\\(FreeListCategoryType type,", 1],
  ["Tagged<FreeSpace> SearchForNodeInList\\(const Heap\\* heap,\\s*FreeListCategoryType type,", 1],
  ["virtual FreeListCategoryType SelectFreeListCategoryType\\(\\s*size_t size_in_bytes\\) = 0;", 1],
  ["FreeListCategory\\* top\\(FreeListCategoryType type\\) const", 1], ["inline NormalPage\\* GetPageForCategoryType\\(FreeListCategoryType type\\);", 1],
];
for (const [re, want] of FL_FNS) has(src, new RegExp(re, "g"), want, `FreeList 锚 ${re.slice(0, 42)}`);
for (const f of ["const int number_of_categories_ = 0;", "const FreeListCategoryType last_category_ = 0;", "size_t min_block_size_ = 0;", "FreeListCategory\\*\\* categories_ = nullptr;", "size_t available_ = 0;", "std::atomic<size_t> wasted_bytes_ = 0;"]) {
  has(src, new RegExp(f, "g"), 1, `FreeList 字段 ${f.slice(0, 40)}`);
}
const flRegion = src.slice(src.indexOf("class FreeList {"), src.indexOf("// Use 24 Freelists"));
has(flRegion, /friend class/g, 5, "FreeList friend 5");
// inl 实体 4 (free-list-inl.h:17-42)
for (const [re, want] of [
  ["bool FreeListCategory::is_linked\\(FreeList\\* owner\\) const \\{", 1],
  ["void FreeListCategory::UpdateCountersAfterAllocation\\(size_t allocation_size\\) \\{", 1],
  ["NormalPage\\* FreeList::GetPageForCategoryType\\(FreeListCategoryType type\\) \\{", 1],
  ["bool FreeList::IsEmpty\\(\\) \\{", 1],
]) has(srcInl, new RegExp(re, "g"), want, `inl 实体 ${re.slice(0, 40)}`);

// ── ④ Many/Cached/FastPathBase 族 ──────────────────────────────────────────
has(src, /static constexpr size_t kMinBlockSize = 3 \* kTaggedSize;/, 1, "kMinBlockSize");
has(src, /static constexpr size_t kMaxBlockSize = kRegularPageSize;/, 1, "kMaxBlockSize");
has(src, /static constexpr size_t kPreciseCategoryMaxSize = 256;/, 1, "kPreciseCategoryMaxSize");
has(src, /static constexpr int kNumberOfCategories = 24;/, 1, "kNumberOfCategories");
const CATS_MIN = [24, 32, 48, 64, 80, 96, 112, 128, 144, 160, 176, 192, 208, 224, 240, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536];
has(src, /static constexpr unsigned int categories_min\[kNumberOfCategories\] = \{/, 1, "categories_min 声明");
const tbl = src.slice(src.indexOf("categories_min[kNumberOfCategories] = {"), src.indexOf("};", src.indexOf("categories_min[kNumberOfCategories] = {")));
for (const v of CATS_MIN) has(tbl, new RegExp(`(?:^|[\\s,{])${v}(?:[,\\s}]|$)`, "m"), 1, `categories_min 值 ${v}`);
has(src, /SelectFreeListCategoryType\(\s*size_t size_in_bytes\) override \{/, 1, "Many Select override");
has(src, /int next_nonempty_category\[kNumberOfCategories \+ 1\];/, 1, "缓存数组 25 槽");
has(src, /void UpdateCacheAfterAddition\(FreeListCategoryType cat\)/, 1, "UpdateCacheAfterAddition");
has(src, /void UpdateCacheAfterRemoval\(FreeListCategoryType cat\)/, 1, "UpdateCacheAfterRemoval");
has(src, /void CheckCacheIntegrity\(\)/, 1, "CheckCacheIntegrity");
has(src, /void ResetCache\(\)/, 1, "ResetCache");
has(src, /enum class SmallBlocksMode \{ kAllow, kProhibit \};/, 1, "SmallBlocksMode");
has(src, /static const FreeListCategoryType kFastPathFirstCategory = 18;/, 1, "kFastPathFirstCategory");
has(src, /static const size_t kFastPathStart = 2048;/, 1, "kFastPathStart");
has(src, /static const size_t kTinyObjectMaxSize = 128;/, 1, "kTinyObjectMaxSize");
has(src, /static const size_t kFastPathOffset = kFastPathStart - kTinyObjectMaxSize;/, 1, "kFastPathOffset");
has(src, /static const FreeListCategoryType kFastPathFallBackTiny = 15;/, 1, "kFastPathFallBackTiny");
has(src, /static_assert\(categories_min\[kFastPathFirstCategory\] == kFastPathStart\);/, 1, "静态断言 fast 18");
has(src, /static_assert\(categories_min\[kFastPathFallBackTiny\] ==\s*kTinyObjectMaxSize \* 2\);/, 1, "静态断言 fast 15");
has(src, /FreeListCategoryType SelectFastAllocationFreeListCategoryType\(/, 1, "SelectFast 定义");
has(src, /FRIEND_TEST\(/g, 3, "FRIEND_TEST 3");
// v8_flags 运行态读点 (ctor kProhibit 分支) — 注入参数承接锚
has(src, /v8_flags\.minor_ms && \(v8_flags\.minor_ms_min_lab_size_kb > 0\)/, 1, "minor_ms 条件");
has(src, /v8_flags\.minor_ms_min_lab_size_kb \* KB/, 1, "minor_ms_min_lab_size_kb \\* KB");

// ── ⑤ 展开侧 (目标配置: COMPRESS+SANDBOX+EXTERNAL_CODE, DEBUG 关) ──────────
const expanded = preprocess(REL);
for (const g of ["size_t FreeList::SumFreeLists", "bool FreeList::IsVeryLong", "void CheckCacheIntegrity"]) {
  has(expanded, new RegExp(esc(g), "g"), 0, `展开 DEBUG 剥除 ${g}`);
}
has(expanded, /V8_EXPORT_PRIVATE/g, 0, "展开 V8_EXPORT_PRIVATE 剥除");
has(expanded, /class FreeListCategory \{/, 1, "展开类 1");
has(expanded, /class FreeListManyCachedOrigin/, 1, "展开类 8 尾锚");
const expRegion = expanded.slice(expanded.indexOf("class FreeListCategory {"), expanded.indexOf("class FreeListManyCachedOrigin"));
has(expRegion, /class FreeListMany\b/, 1, "展开区域 Many");
has(expRegion, /class FreeListManyCached\b/, 1, "展开区域 ManyCached");
has(expRegion, /class FreeListManyCachedFastPathBase\b/, 1, "展开区域 FastPathBase");
has(expRegion, /FRIEND_TEST/g, 3, "展开区域 FRIEND_TEST 3 (gtest 桩头宏未定义, 照录)");
has(expRegion, /static_assert\(categories_min\[kFastPathFirstCategory\] == kFastPathStart\);/, 1, "展开静态断言 1");
// HC-23R 双侧对账补锚: h:467-468 断言展开侧照录拆行 (EXP 实测行首半截 `==` 后断行), 语义同 1
has(expRegion, /static_assert\(categories_min\[kFastPathFallBackTiny\] ==/, 1, "展开静态断言 2 (h:467-468 拆行照录)");
has(expanded, /using FreeListCategoryType = int32_t;/, 1, "展开 typedef");
has(expanded, /enum FreeMode \{ kLinkCategory, kDoNotLinkCategory \};/, 1, "展开 FreeMode");
has(expanded, /std::atomic<size_t> wasted_bytes_ = 0;/, 1, "展开 atomic wasted_bytes_");
has(expanded, /static constexpr int kVeryLongFreeList = 500;/, 1, "展开 kVeryLongFreeList");
has(expanded, /int next_nonempty_category\[kNumberOfCategories \+ 1\];/, 1, "展开缓存数组 25 槽 (标识符不折叠)");

// ── ⑥ rs 侧落地锚 (HC-3: v8-heap-core/src/free_list.rs 实态对数, 只增不减) ──
const rs = readFileSync(join(root, "v8-heap-core", "src", "free_list.rs"), "utf8");
const RS_FNS = [
  ["pub type FreeListCategoryType = i32", "typedef h:36"],
  ["pub const K_FIRST_CATEGORY: FreeListCategoryType = 0", "kFirstCategory h:38"],
  ["pub const K_INVALID_CATEGORY: FreeListCategoryType = -1", "kInvalidCategory h:39"],
  ["pub enum FreeMode", "FreeMode h:41"],
  ["LinkCategory = 0", "FreeMode::LinkCategory (PascalCase 承接 kLinkCategory)"],
  ["DoNotLinkCategory = 1", "FreeMode::DoNotLinkCategory (PascalCase 承接)"],
  ["pub enum FreeSpace {}", "FreeSpace opaque (objects/free-space.h 他票)"],
  ["pub enum NormalPage {}", "NormalPage opaque (heap/normal-page.h 他票)"],
  ["pub trait FreeListEnv", "耦合窄 trait (Isolate/Heap/NormalPage/FreeSpace 承接面)"],
  ["pub struct FreeListCategory\\b", "Category h:44"],
  ["pub fn initialize", "Initialize h:46"],
  ["pub fn unlink", "Unlink cc:22"],
  ["pub fn reset<O: FreeList>\\(&mut self, owner: &mut O\\)", "Reset cc:30 (签名现代化: owner 泛型化)"],
  ["pub fn relink", "Relink cc:126"],
  ["pub fn pick_node_from_list", "PickNodeFromList cc:36"],
  ["pub fn search_for_node_in_list", "Category SearchForNodeInList cc:52"],
  ["pub fn free<O: FreeList>", "Category Free cc:93 (签名现代化: owner 泛型化)"],
  ["pub fn is_linked", "is_linked inl:17"],
  ["pub fn is_empty\\(&self\\)", "is_empty h:79"],
  ["pub fn available\\(&self\\) -> u32", "available h:80"],
  ["pub fn sum_free_list\\(", "SumFreeList cc:553"],
  ["pub fn free_list_length", "FreeListLength cc:568"],
  ["pub fn iterate_nodes_for_testing", "IterateNodesForTesting h:85"],
  ["fn update_counters_after_allocation", "UpdateCountersAfterAllocation inl:22"],
  ["pub const K_VERY_LONG_FREE_LIST", "kVeryLongFreeList h:96"],
  ["next_nonempty_category: \\[FreeListCategoryType; K_NUMBER_OF_CATEGORIES \\+ 1\\]", "next_nonempty_category h:406 (25 槽数组内联入 Cached; 独立 NextNonemptyCache 结构体已并入)"],
  ["fn update_cache_after_addition", "UpdateCacheAfterAddition h:376 (更名 update_cache_after_addition)"],
  ["fn update_cache_after_removal", "UpdateCacheAfterRemoval h:384 (更名 update_cache_after_removal)"],
  ["fn check_cache_integrity", "CheckCacheIntegrity h:392 (更名 check_cache_integrity)"],
  ["fn reset_cache", "ResetCache h:409"],
  ["pub trait FreeList \\{", "FreeList h:135 (trait 化: 虚面 trait + 字段面 FreeListBase)"],
  ["pub struct FreeListBase \\{", "FreeListBase (原 FreeList 字段/非虚面 h:273-283 承接, 只增锚)"],
  ["pub fn create_free_list\\(\\)", "CreateFreeList cc:139"],
  ["pub fn create_free_list_for_new_space", "CreateFreeListForNewSpace cc:143"],
  ["pub fn new\\(number_of_categories: i32, min_block_size: usize\\)", "FreeList ctor cc:134"],
  ["fn try_find_node_in", "TryFindNodeIn cc:147 (trait 默认体)"],
  ["fn free<E: FreeListEnv>", "FreeList::Free cc:184 (trait 默认体 + 覆写/宏转接 4)", 5],
  ["fn add_category_impl", "AddCategory cc:500 (FreeListBase 共享体 + trait 默认转接)"],
  ["fn remove_category_impl", "RemoveCategory cc:519 (FreeListBase 共享体 + trait 默认转接)"],
  ["fn reset_impl", "FreeList::Reset cc:456 (reset_impl 走链 + trait 默认转接)"],
  ["fn reset_for_non_black_impl", "ResetForNonBlack cc:466 (reset_for_non_black_impl + trait 默认转接)"],
  ["fn evict_free_list_items", "EvictFreeListItems cc:485 (trait 默认体)"],
  ["pub fn print_categories", "PrintCategories cc:542"],
  ["pub fn for_all_free_list_categories<F: FnMut\\(&mut FreeListCategory\\)>", "ForAllFreeListCategories 全型 h:212 (FreeListBase, &mut self)"],
  ["pub fn for_all_free_list_categories_of\\b", "ForAllFreeListCategories 单型 h:202 (更名 _of 后缀)"],
  ["pub fn available\\(&self\\) -> usize", "Available h:171 (FreeListBase 非泛型化)"],
  ["pub fn verify_available", "VerifyAvailable h:247"],
  ["pub fn is_empty\\(&self\\)", "IsEmpty inl:36"],
  ["pub fn increase_available_bytes", "IncreaseAvailableBytes h:177"],
  ["pub fn decrease_available_bytes", "DecreaseAvailableBytes h:178"],
  ["pub fn wasted_bytes", "wasted_bytes h:180"],
  ["pub fn increase_wasted_bytes", "increase_wasted_bytes h:183"],
  ["pub fn decrease_wasted_bytes", "decrease_wasted_bytes h:186"],
  ["pub fn number_of_categories", "number_of_categories h:197"],
  ["pub fn last_category\\(&self\\)", "last_category h:198"],
  ["pub fn min_block_size\\(&self\\)", "min_block_size h:200"],
  ["pub fn top\\(&self, type_: FreeListCategoryType\\)", "top(type) h:267"],
  ["fn get_page_for_category_type", "GetPageForCategoryType inl:26 (trait 默认体)"],
  ["pub fn sum_free_lists", "SumFreeLists cc:594 (原版 #ifdef DEBUG, 常编承接)"],
  ["pub fn is_very_long", "IsVeryLong cc:579 (同上)"],
  ["pub struct FreeListMany \\{", "FreeListMany h:298"],
  ["K_NUMBER_OF_CATEGORIES: usize = 24", "kNumberOfCategories h:327"],
  ["pub const CATEGORIES_MIN: \\[i32; K_NUMBER_OF_CATEGORIES\\] = \\[", "categories_min h:328-330 (模块级 const i32, 类级差申报)"],
  ["K_MIN_BLOCK_SIZE: usize = 3 \\* K_TAGGED_SIZE", "kMinBlockSize h:310"],
  ["K_MAX_BLOCK_SIZE: usize = K_REGULAR_PAGE_SIZE", "kMaxBlockSize h:314"],
  ["K_PRECISE_CATEGORY_MAX_SIZE: usize = 256", "kPreciseCategoryMaxSize h:317"],
  ["fn select_free_list_category_type\\(&self, size_in_bytes: usize\\) -> FreeListCategoryType;", "SelectFreeListCategoryType h:333 (trait 纯虚声明; 实体见 FreeListMany impl)"],
  ["fn get_page_for_size<E: FreeListEnv>", "GetPageForSize cc:219 (纯虚声明 + Many 实体 + 转接 4)", 6],
  ["fn allocate<E: FreeListEnv>", "Allocate (纯虚声明 + Many/Cached/FastPathBase/Origin 实体 + 薄壳宏转接 2)", 6],
  ["pub struct FreeListManyCached \\{", "FreeListManyCached h:357"],
  ["pub struct FreeListManyCachedFastPathBase \\{", "FastPathBase h:438"],
  ["pub enum SmallBlocksMode", "SmallBlocksMode h:441"],
  ["K_FAST_PATH_FIRST_CATEGORY: FreeListCategoryType = 18", "kFastPathFirstCategory h:459"],
  ["K_FAST_PATH_START: usize = 2048", "kFastPathStart h:460"],
  ["K_TINY_OBJECT_MAX_SIZE: usize = 128", "kTinyObjectMaxSize h:461"],
  ["K_FAST_PATH_OFFSET: usize = K_FAST_PATH_START - K_TINY_OBJECT_MAX_SIZE", "kFastPathOffset h:462"],
  ["K_FAST_PATH_FALL_BACK_TINY: FreeListCategoryType = 15", "kFastPathFallBackTiny h:464"],
  ["pub fn select_fast_allocation_free_list_category_type", "SelectFast h:470"],
  ["pub struct FreeListManyCachedFastPath \\{", "FastPath h:493"],
  ["pub struct FreeListManyCachedFastPathForNewSpace \\{", "ForNewSpace h:499"],
  ["pub struct FreeListManyCachedOrigin \\{", "Origin h:514"],
  // HC-23R 承载形对账锚 (只增): 新版 memory_chunk 无 free_list 槽位 (HC-1 实测) 的映射落账面
  ["新版 memory_chunk 无 free_list 槽位", "模块申报: 类目承载=页元数据所有 (映射已落账)"],
  ["fn page_free_list_category", "FreeListEnv 承载取槽面 (原 page->free_list_category(type))"],
];
for (const [re, label, w = 1] of RS_FNS) has(rs, new RegExp(re, "g"), w, `rs 锚 ${label}`);
// categories_min 24 逐值 (rs 表区)
const rsTbl = rs.slice(rs.indexOf("pub const CATEGORIES_MIN"), rs.indexOf("];", rs.indexOf("pub const CATEGORIES_MIN")));
for (const v of CATS_MIN) has(rsTbl, new RegExp(`(?:^|[\\s,{])${v}(?:[,\\s}]|$)`, "m"), 1, `rs categories_min 值 ${v}`);
// 静态断言承接 (const 门; CATEGORIES_MIN 模块级 const, 类级作用域差申报;
//   实态更名符号常量下标 K_FAST_PATH_* — 字面 18/15 → 符号锚, 语义同 h:466-468;
//   HC-23R 修: 期望形补 `as i32` 窄化 — rs 侧 CATEGORIES_MIN 为 [i32; 24] (承接 int32_t 表),
//   K_FAST_PATH_START/K_TINY_OBJECT_MAX_SIZE 为 usize, Rust 同型比较必须显式 cast,
//   旧期望 `== K_FAST_PATH_START);` 在 rs 语法面不可表达 → 台账修正追实态, 断言数不减)
has(rs, /assert!\(CATEGORIES_MIN\[K_FAST_PATH_FIRST_CATEGORY as usize\] == K_FAST_PATH_START as i32\);/, 1, "h:466 static_assert 承接");
has(rs, /assert!\(CATEGORIES_MIN\[K_FAST_PATH_FALL_BACK_TINY as usize\] == K_TINY_OBJECT_MAX_SIZE as i32 \* 2\);/, 1, "h:467 static_assert 承接");
// 0 检: 旧叙事面禁落地
for (const ghost of ["SpaceSpecificFreeList", "EnsureStarted", "PadAndSetLink", "ComputeRequiredWaste", "parking_lot"]) {
  has(rs, new RegExp(esc(ghost), "g"), 0, `rs 旧叙事 0 残留 ${ghost}`);
}
// todo!() 禁面
has(rs, /todo!\(/g, 0, "rs todo!() 0 残留");
// 推迟台账区存在 (下限判断: 区标题 + RepairFreeList 双行号)
if (count(rs, /cc:112-124/g) < 1) fail("rs 推迟台账双行号 RepairFreeList", 0, ">=1");
n++;
if (count(rs, /推迟台账/g) < 1) fail("rs 推迟台账区", 0, ">=1");
n++;

const ln = (s) => src.split(String.fromCharCode(10)).findIndex((l) => l.includes(s)) + 1;
const skel = `${banner("src/heap/free-list.h.js", REL + " + " + REL_INL, "FreeList 族头台账: 类 8 + Category 成员 21 + FreeList 成员 34 + Many/Cached/FastPath 族常量表 + inl 实体 4 + 展开侧 DEBUG 剥除")}
// ══ free-list.h:1-525 + free-list-inl.h:1-47 → v8-heap-core/src/free_list.rs ══
// ── 票面差异申报 (实测为准, 旧叙事证伪) ──
// 本仓 v8/src/heap/free-list.h 为 2020 重构族: FreeListCategory + FreeList + FreeListMany 族 (8 类)。
// 票面 SpaceSpecificFreeList / EnsureStarted / Push / Pop / Advance / PadAndSetLink / ComputeRequiredWaste /
//   CommittedLength / UncommittedLength / Mutex 静态表 — h/cc 实测 0 存在 (上列 12 条 0-检锚);
//   该族属旧版 V8 free-list 与 cppgc 变体, 本仓 v8/src/heap/cppgc-internal/free-list.h 不存在 (ls 实测) → 不落地。
//   票面 Push/Pop 语义等价面 = FreeListCategory::Free 头插 (cc:93) / PickNodeFromList 弹出 (cc:36);
//   Advance/Padded 链尾语义无对应实体 → 金标准以 Free/Pick/Search 链尾边界覆盖 (见测试映射表)。
//   锁面: 原版无任何 Mutex (FreeList::wasted_bytes_ h:283 为 std::atomic) → Rust AtomicUsize relaxed 承接,
//   禁 parking_lot (无锁语义可承接, 票面锁叙事证伪)。
// 类族与行号:
//   FreeListCategory (h:44-126) / FreeList (h:135-290) / FreeListMany (h:298-350) / FreeListManyCached (h:357-417)
//   FreeListManyCachedFastPathBase (h:438-491) / FreeListManyCachedFastPath (h:493-497)
//   FreeListManyCachedFastPathForNewSpace (h:499-504) / FreeListManyCachedOrigin (h:514-520)
// FreeListCategory 成员 needle 全集 (${CAT_FNS.length} 条, 逐锚见脚本②):
${CAT_FNS.map(([re]) => `//   ${re}`).join("\n")}
// inl 实体 4: is_linked inl:17-20 / UpdateCountersAfterAllocation inl:22-24 / GetPageForCategoryType inl:26-34 / IsEmpty inl:36-42
// categories_min (h:328-330) 24 值: ${CATS_MIN.join(", ")}
// kFastPath 族 (h:459-464): kFastPathFirstCategory=18 / kFastPathStart=2048 / kTinyObjectMaxSize=128 / kFastPathOffset=1920 / kFastPathFallBackTiny=15; static_assert h:466-468
// v8_flags 运行态 (h:447-449 minor_ms 分支) — v8-flags 全局单例他票 → ctor 参数注入 (同 layout 票 InstructionStream 先例)
// 虚拟分派承接: C++ virtual Allocate/Free/AddCategory/RemoveCategory/SelectFreeListCategoryType 多态面 →
//   Rust trait FreeList (本仓禁 dyn) + 组合结构 (base 字段链 FreeListBase) + 具体类逐 override 落地 +
//   基类共享体 (*_impl) + 薄壳类宏转接 (forward_free_list_impl) 承接虚表回击 (申报差异)
// 展开实测: SumFreeLists/IsVeryLong (h:243-244 #ifdef DEBUG) 与 CheckCacheIntegrity (h:391-401) 目标配置剥除 →
//   Rust 常编 pub 承接 (debug_assert! 消费侧 release 零求值, 等价原版)
// ── rs 侧落地锚 (HC-3 实测台账) ──
// Landed @ v8-heap-core/src/free_list.rs: 8 类组合链 + Category 全成员 (RepairFreeList 推迟) + FreeList 基类
//   全表结构/链手术/计数器 + Many 24 类表与 Select + Cached 缓存 25 槽与双更新 + FastPathBase 双段快速路径 +
//   Origin 分派 + 工厂 2; 耦合面 FreeListEnv 窄 trait 10 方法 (Isolate/Heap/NormalPage/FreeSpace/SetNext-JIT 他票);
//   推迟台账双行号见模块尾 (RepairFreeList/CanAllocateOnChunk/AvailableInFreeList DCHECK/JIT 通道/Tag 面)。
`;
writeSrc("v8-heap-core", "free_list_h.skel", skel);
console.log(`OK free-list.h.js: 对数断言 ${n} 条全过; 类 8 + 旧叙事 0-检 12 + typedef/FreeMode 4 + Category 成员/字段/friend 28 + FreeList 成员/字段/friend 42 + inl 4 + Many 表 24 逐值 + Cached/FastPath 常量 17 + FRIEND_TEST 3 + v8_flags 2 + 展开侧 19 (DEBUG 剥除 3 / EXPORT_PRIVATE 0 / 区域类锚 4 / FRIEND_TEST 3 / 静态断言 2 双侧 / 常量折叠 25) + rs 落地锚 ${RS_FNS.length} + rs 表 24 + 断言 2 + 0-检 6; 产物 v8-heap-core/src/free_list_h.skel`);
