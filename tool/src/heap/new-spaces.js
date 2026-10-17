#!/usr/bin/env bun
// tool/src/heap/new-spaces.js
// 机械提取 v8/src/heap/new-spaces{.cc,-inl.h} 族 (族 .h 归他席, 读取仅作上下文):
// SemiSpaceId 枚举 + SemiSpace/SemiSpaceNewSpace/PagedNewSpace 骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/new-spaces.h"); // 上下文 (本席表外)
const ccSrc = readV8("src/heap/new-spaces.cc");
const inlSrc = readV8("src/heap/new-spaces-inl.h");

const anchors = [
  [hSrc, "enum SemiSpaceId { kFromSpace = 0, kToSpace = 1 };"],
  [hSrc, "class SemiSpace final : public Space {"],
  [hSrc, "static void Swap(SemiSpace* from, SemiSpace* to);"],
  [hSrc, "bool AdvancePage(size_t target_capacity);"],
  [hSrc, "size_t current_capacity() const { return current_capacity_; }"],
  [hSrc, "size_t current_capacity_safe() const {"],
  [hSrc, "class SemiSpaceObjectIterator : public ObjectIterator {"],
  [hSrc, "class NewSpace : NON_EXPORTED_BASE(public SpaceWithLinearArea) {"],
  [hSrc, "class V8_EXPORT_PRIVATE SemiSpaceNewSpace final : public NewSpace {"],
  [hSrc, "class V8_EXPORT_PRIVATE PagedSpaceForNewSpace final : public PagedSpaceBase {"],
  [hSrc, "class V8_EXPORT_PRIVATE PagedNewSpace final : public NewSpace {"],
  [ccSrc, "bool SemiSpace::AllocateFreshPage() {"],
  [ccSrc, "void SemiSpace::Swap("],
  [inlSrc, "SemiSpaceNewSpace::"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// SemiSpaceId 真提取
const sm = hSrc.match(/enum SemiSpaceId \{([^}]*)\};/);
if (!sm) throw new Error("SemiSpaceId 未找到");
const entries = sm[1].split(",").map((s) => s.trim()).filter(Boolean).map((e) => {
  const m = e.match(/(\w+) = (\d+)/);
  if (!m) throw new Error(`SemiSpaceId 项无显式值: ${e}`);
  return { name: m[1], value: parseInt(m[2], 10) };
});
if (entries.length !== 2 || entries[0].value !== 0 || entries[1].value !== 1) throw new Error("SemiSpaceId 漂移");
console.log(`new-spaces: SemiSpaceId ${entries.length} (显式值), 头文件类 6, Swap 静态 1`);

let rust = banner(
  "src/heap/new-spaces.js",
  "src/heap/new-spaces.cc, src/heap/new-spaces-inl.h (族 .h: src/heap/new-spaces.h)",
  "新生代空间: 半空间/分页新生代",
);

rust += `/// v8: src/heap/new-spaces.h SemiSpaceId
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SemiSpaceId {
`;
entries.forEach((e) => (rust += `  /// ${e.name} = ${e.value}\n  ${e.name.replace(/^k/, "").toUpperCase()} = ${e.value},\n`));
rust += `}

/// v8: src/heap/new-spaces.h SemiSpace
pub struct SemiSpace {
  /// v8: current_capacity_ = 0 — 已提交容量 (Relaxed_Load 原子读变体)
  pub current_capacity: usize,
  /// v8: committed_physical_memory_ = 0
  pub committed_physical_memory: usize,
  /// v8: id_
  pub id: SemiSpaceId,
  /// v8: current_page_ = nullptr
  pub current_page: *mut NormalPage,
  /// v8: quarantined_pages_count_ = 0 — 隔离页挂链表头
  pub quarantined_pages_count: usize,
}

impl SemiSpace {
  /// v8: SemiSpace::Swap(from, to) — 交换两半空间内容 (static)
  pub fn swap(from: &mut SemiSpace, to: &mut SemiSpace) { todo!() }
  /// v8: AdvancePage(target_capacity) — 推进当前页
  pub fn advance_page(&mut self, target_capacity: usize) -> bool { todo!() }
  /// v8: space_start / page_low / page_high — 首页与当前页边界
  /// v8: ContainsSlow / Uncommit / IsCommitted / CommittedPhysicalMemory
  /// v8: RemovePage / MovePageToTheEnd / InitializePage / GetObjectIterator
  /// v8: AllocateFreshPage / RewindPages / FixPagesFlags / ShrinkCapacityTo / Reset
  /// v8: MoveQuarantinedPage / AddRangeToActiveSystemPages
  /// v8: Size/SizeOfObjects/Available — UNREACHABLE (半空间不独立计量)
  /// v8: AssertValidRange (#ifdef DEBUG) / VerifyPages (#ifdef VERIFY_HEAP)
  pub fn current_capacity_safe(&self) -> usize { self.current_capacity }
}

/// v8: src/heap/new-spaces.h SemiSpaceObjectIterator — 遍历活跃半空间对象
/// v8: src/heap/new-spaces.h NewSpace (SpaceWithLinearArea 基类)
/// v8: src/heap/new-spaces.h SemiSpaceNewSpace — to/from 两半 + Flip/Swap语义
/// v8: src/heap/new-spaces.h PagedSpaceForNewSpace — MinorMS 用分页新生代
/// v8: src/heap/new-spaces.h PagedNewSpace
/// v8: src/heap/new-spaces.cc 各实现 + src/heap/new-spaces-inl.h 内联面

// —— 提取不变式断言 ——
const _: () = {
  assert!(SemiSpaceId::FROM_SPACE == 0);
  assert!(SemiSpaceId::TO_SPACE == 1);
};
`;

const p = writeSrc("v8-heap-core", "new_spaces.rs.skel", rust);
console.log(`written ${p}`);
