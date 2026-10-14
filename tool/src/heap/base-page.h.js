#!/usr/bin/env bun
// tool/src/heap/base-page.h.js
// 机械提取 v8/src/heap/base-page.{h,cc,-inl.h}: BasePage 17 位 flags_ 位域表 +
// 谓词/记账方法台账 + UpdateHighWaterMark CAS 循环 + Contains/ContainsLimit 谓词。
// 规则: 位域链 (BitField<bool,0,1> + 16 级 Next) 顺序解析为常量表; flags 读写
// (decode/update) 机械转写为位运算规则代码。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/base-page.h");
const inl = readV8("src/heap/base-page-inl.h");
const cc = readV8("src/heap/base-page.cc");

// —— 位域链解析: 17 个 1-bit flag, 从 bit0 起 Next 递进 ——
const chain = ["IsPinnedForTestingField", "IsUnregisteredField", "IsPreeFreedField",
  "IsLargePageField", "IsExecutableField", "WillBePromotedField", "IsQuarantinedField",
  "IsEvacuationCandidateField", "EvacuationWasAbortedField", "NeverEvacuateField",
  "NeverAllocateOnChunk", "ForceEvacuationCandidateForTestingField", "IsTrustedField",
  "IsWritableSharedSpaceField", "IsSealedReadOnlySpaceField", "IsReadOnlyPageField",
  "IsBlackAllocated"];
if (!h.includes("using IsPinnedForTestingField = v8::base::BitField<bool, 0, 1, FlagsT>;")) {
  throw new Error("位域链首漂移");
}
const nexts = h.match(/using (\w+) =\s*(\w+)::Next<bool, 1>;/g) || [];
if (nexts.length !== chain.length - 1) throw new Error(`Next 链 ${nexts.length}, 期望 ${chain.length - 1}`);
// 链序核对
let prev = "IsPinnedForTestingField";
for (const n of nexts) {
  const m = n.match(/using (\w+) =\s*(\w+)::Next<bool, 1>;/);
  if (m[2] !== prev) throw new Error(`位域链序断裂: ${m[1]} 的前驱是 ${m[2]}, 期望 ${prev}`);
  prev = m[1];
}
if (prev !== "IsBlackAllocated") throw new Error("位域链尾漂移");

// —— 关键方法/字段锚 ——
for (const a of [
  "class BasePage {",
  "V8_INLINE static BasePage* FromAddress(Address a);",
  "V8_INLINE static BasePage* FromHeapObject(Tagged<HeapObject> o);",
  "V8_INLINE static void UpdateHighWaterMark(Address mark);",
  "bool IsLargePage() const {",
  "bool IsNormalPage() const { return IsMutablePage() && !IsLargePage(); }",
  "inline AllocationSpace owner_identity() const;",
  "bool Contains(Address addr) const {",
  "bool ContainsLimit(Address addr) const {",
  "void ResetAllocationStatistics() {",
  "using FlagsT = uint32_t;",
  "FlagsT flags_ = 0;",
  "std::atomic<intptr_t> high_water_mark_;",
]) {
  if (!h.includes(a)) throw new Error(`base-page.h 锚点缺失: ${a}`);
}
// -inl: CAS 循环 + FromAddress 实现
for (const a of [
  "BasePage* BasePage::FromAddress(Address a) {",
  "return MemoryChunk::FromAddress(a)->Metadata();",
  "void BasePage::UpdateHighWaterMark(Address mark) {",
  "while ((new_mark > old_mark) &&",
  "!chunk->high_water_mark_.compare_exchange_weak(",
  "bool BasePage::IsWritable() const {",
  "return !is_sealed_ro;",
]) {
  if (!inl.includes(a)) throw new Error(`base-page-inl.h 锚点缺失: ${a}`);
}
// cc: dtor/SynchronizedHeapLoad
if (!cc.includes("BasePage::~BasePage()")) throw new Error("cc dtor 缺失");
console.log(`BasePage: 位域 ${chain.length} 位 + 方法/字段锚 20`);

const doc = {
  IsPinnedForTestingField: "测试用钉住位: 页不可移动 (is_pinned_for_testing)",
  IsUnregisteredField: "注销记账已完成, 内存尚未真正释放 (is_unregistered)",
  IsPreeFreedField: "逻辑上已释放, 实际释放待执行 (is_pre_freed)",
  IsLargePageField: "大对象页 (is_large)",
  IsExecutableField: "可执行页 (is_executable)",
  WillBePromotedField: "GC 末停顿晋升老生代 (will_be_promoted)",
  IsQuarantinedField: "隔离页: 含栈可达对象, 新空间不再分配 (is_quarantined)",
  IsEvacuationCandidateField: "压实候选页: 写屏障记录指向它的槽 (is_evacuation_candidate)",
  EvacuationWasAbortedField: "压空中止, 仍算候选页 (evacuation_was_aborted)",
  NeverEvacuateField: "永不压空 (never_evacuate)",
  NeverAllocateOnChunk: "禁止在此页分配 (never_allocate_on_chunk)",
  ForceEvacuationCandidateForTestingField: "测试: 强制为压实候选 (manual_evacuation_candidates_selection)",
  IsTrustedField: "可信空间页: sandbox 外 (is_trusted)",
  IsWritableSharedSpaceField: "可写共享空间页 (is_writable_shared)",
  IsSealedReadOnlySpaceField: "封存只读空间页 (IsSealedReadOnlySpace)",
  IsReadOnlyPageField: "只读空间页 (IsReadOnlyPage)",
  IsBlackAllocated: "黑分配页 (is_black_allocated)",
};
const snake = (s) => s.replace(/Field$/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

let rust = banner(
  "src/heap/base-page.h.js",
  "src/heap/base-page.{h,cc,-inl.h}",
  "BasePage: 任意堆页元数据基类, 17 位 flags_ 位域与记账",
);

rust += `
/// BasePage flags_ 位域 (对应 FlagsT = uint32_t 的 17 个 1-bit BitField)。
/// 布局: 首位 BitField<bool, 0, 1>, 其余逐位 Next 递进 (bit0..bit16)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BasePageFlags(pub u32);

impl BasePageFlags {
`;
chain.forEach((name, i) => {
  const rn = snake(name);
  rust += `  /// bit${i}: ${doc[name]}
  pub const ${rn}: u32 = 1 << ${i};
`;
});
rust += `
  /// 对应 decode: (flags_ & bit) != 0
  pub fn decode(self, bit: u32) -> bool {
    (self.0 & bit) != 0
  }

  /// 对应 update: 置/清位后返回新 flags_
  pub fn update(self, bit: u32, value: bool) -> Self {
    Self(if value { self.0 | bit } else { self.0 & !bit })
  }

  /// 对应 IsNormalPage = IsMutablePage && !IsLargePage
  pub fn is_normal_page(self) -> bool {
    !self.decode(Self::IS_READ_ONLY_PAGE) && !self.decode(Self::IS_LARGE_PAGE)
  }

  /// 对应 CanAllocateOnChunk = !is_evacuation_candidate && !never_allocate_on_chunk
  pub fn can_allocate_on_chunk(self) -> bool {
    !self.decode(Self::IS_EVACUATION_CANDIDATE) && !self.decode(Self::NEVER_ALLOCATE_ON_CHUNK)
  }
}

/// 对应 UpdateHighWaterMark: mark-1 定位页, 高水位 CAS 单调抬升
/// v8: src/heap/base-page-inl.h BasePage::UpdateHighWaterMark
pub fn update_high_water_mark(high_water_mark: &core::sync::atomic::AtomicIsize, mark: u64, chunk_address: u64) {
  if mark == 0 {
    return;
  }
  // mark - 1: 页满时 top 已越过本页首地址
  let new_mark = (mark - 1 + 1 - chunk_address) as isize; // 对应 mark - chunk->ChunkAddress()
  let mut old_mark = high_water_mark.load(core::sync::atomic::Ordering::Relaxed);
  while new_mark > old_mark
    && high_water_mark
      .compare_exchange_weak(old_mark, new_mark, core::sync::atomic::Ordering::AcqRel,
        core::sync::atomic::Ordering::Relaxed)
      .is_err()
  {
    old_mark = high_water_mark.load(core::sync::atomic::Ordering::Relaxed);
  }
}

/// 对应 Contains: [area_start, area_end)
/// v8: src/heap/base-page.h BasePage::Contains
#[must_use]
pub fn contains(area_start: u64, area_end: u64, addr: u64) -> bool {
  addr >= area_start && addr < area_end
}

/// 对应 ContainsLimit: [area_start, area_end] (端点可作分配上限)
/// v8: src/heap/base-page.h BasePage::ContainsLimit
#[must_use]
pub fn contains_limit(area_start: u64, area_end: u64, addr: u64) -> bool {
  addr >= area_start && addr <= area_end
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(BasePageFlags::IS_PINNED_FOR_TESTING, 1 << 0);
  assert_eq!(BasePageFlags::IS_BLACK_ALLOCATED, 1 << 16);
  let f = BasePageFlags(0);
  assert!(!f.decode(BasePageFlags::IS_LARGE_PAGE));
  assert!(f.update(BasePageFlags::IS_LARGE_PAGE, true).decode(BasePageFlags::IS_LARGE_PAGE));
  assert!(!f.update(BasePageFlags::IS_LARGE_PAGE, true).is_normal_page());
  assert!(f.update(BasePageFlags::IS_EVACUATION_CANDIDATE, true).can_allocate_on_chunk() == false);
  assert!(f.update(BasePageFlags::NEVER_ALLOCATE_ON_CHUNK, true).can_allocate_on_chunk() == false);
  assert!(contains(10, 20, 10) && contains(10, 20, 19) && !contains(10, 20, 20));
  assert!(contains_limit(10, 20, 20) && !contains_limit(10, 20, 21));
};
`;

const p = writeSrc("v8-heap-core", "base_page.rs.skel", rust);
console.log(`written ${p}`);
