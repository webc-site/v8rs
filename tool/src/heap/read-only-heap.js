#!/usr/bin/env bun
// tool/src/heap/read-only-heap.js
// 机械提取 v8/src/heap/read-only-heap{.cc,-inl.h} 族 (族 .h 归他席, 读取仅作上下文):
// ReadOnlyHeap 生命周期方法 + EarlyGetReadOnlyRoots

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/heap/read-only-heap.cc");
const inlSrc = readV8("src/heap/read-only-heap-inl.h");
const hSrc = readV8("src/heap/read-only-heap.h"); // 上下文 (本席表外)

const anchors = [
  [ccSrc, "void ReadOnlyHeap::SetUp(Isolate* isolate,"],
  [ccSrc, "ReadOnlyHeap::~ReadOnlyHeap() {"],
  [ccSrc, "group->InitializeReadOnlyArtifacts();"],
  [ccSrc, "isolate->SetUpFromReadOnlyArtifacts(artifacts);"],
  [inlSrc, "EarlyReadOnlyRoots ReadOnlyHeap::EarlyGetReadOnlyRoots("],
  [inlSrc, "IsolateGroup::current()->shared_read_only_heap();"],
  [inlSrc, "shared_ro_heap->roots_init_complete()"],
  [hSrc, "class ReadOnlyHeap final {"],
  [hSrc, "static constexpr size_t kEntriesCount ="],
  [hSrc, "RootIndex::kReadOnlyRootsCount"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

const ccFns = ["ReadOnlyHeap::SetUp", "ReadOnlyHeap::~ReadOnlyHeap", "ReadOnlyHeap::DeserializeIntoIsolate", "ReadOnlyHeap::DecommitGuardRegions", "ReadOnlyHeap::OnCreateRootsComplete", "ReadOnlyHeap::OnCreateHeapObjectsComplete", "ReadOnlyHeap::CreateInitialHeapForBootstrapping", "ReadOnlyHeap::InitializeIsolateRoots", "ReadOnlyHeap::InitializeFromIsolateRoots", "ReadOnlyHeap::InitFromIsolate", "ReadOnlyHeap::PopulateReadOnlySpaceStatistics", "ReadOnlyHeap::Contains"];
let have = 0;
for (const f of ccFns) if (ccSrc.includes(f)) have++;
if (have !== ccFns.length) throw new Error(`cc 方法命中 ${have}/${ccFns.length}`);
console.log(`read-only-heap: cc 方法命中 ${have}/${ccFns.length}, kEntriesCount = RootIndex::kReadOnlyRootsCount, inl 1 (EarlyGetReadOnlyRoots)`);

let rust = banner(
  "src/heap/read-only-heap.js",
  "src/heap/read-only-heap.cc, src/heap/read-only-heap-inl.h (族 .h: src/heap/read-only-heap.h)",
  "只读堆: artifacts 共享与早取根",
);

rust += `/// v8: src/heap/read-only-heap.h ReadOnlyHeap
pub struct ReadOnlyHeap {
  /// v8: read_only_space_ / read_only_artifacts_ / read_only_roots_ (cached)
  pub fields_todo: (),
}

impl ReadOnlyHeap {
  /// v8: kEntriesCount = RootIndex::kReadOnlyRootsCount (缓存根表长)
  pub const K_ENTRIES_COUNT: usize = ROOT_INDEX_K_READ_ONLY_ROOTS_COUNT;

  /// v8: ReadOnlyHeap::SetUp — 首个 isolate 创建 artifacts, 后续共享
  pub fn set_up(isolate: &mut Isolate, can_share_heap: bool) { todo!() }
  /// v8: ReadOnlyHeap::TearDown — 引用计数, 末持有者释放
  pub fn tear_down(&mut self, isolate: &mut Isolate) { todo!() }
  /// v8: DeserializeIntoIsolate — 从快照反序列化
  /// v8: DecommitGuardRegions — 释放 guard 页
  /// v8: OnCreateHeapObjectsComplete / OnCreateRootsComplete — 快照切换点
  /// v8: CreateInitialHeapForBootstrapping / InitFromIsolate
  /// v8: InitializeIsolateRoots / InitializeFromIsolateRoots
  /// v8: PopulateReadOnlySpaceStatistics / Contains

  /// v8: src/heap/read-only-heap-inl.h EarlyGetReadOnlyRoots(object)
  /// 共享堆 roots 就绪 → 直接取 shared_ro_heap->read_only_roots_;
  /// 否则回退 Isolate::Current()。
  pub fn early_get_read_only_roots(object: TaggedHeapObject) -> EarlyReadOnlyRoots { todo!() }
}
`;

const p = writeSrc("v8-heap-core", "read_only_heap.rs.skel", rust);
console.log(`written ${p}`);
