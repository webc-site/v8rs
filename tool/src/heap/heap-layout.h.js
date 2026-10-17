#!/usr/bin/env bun
// tool/src/heap/heap-layout.h.js
// 机械提取 v8/src/heap/heap-layout.{h,cc,-inl.h}: HeapLayout/TrustedHeapLayout
// 全谓词台账 (13 判定函数) + InAnySharedSpace 只读排除规则 + IsSelfForwarded 语义。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/heap-layout.h");
const inl = readV8("src/heap/heap-layout-inl.h");
const cc = readV8("src/heap/heap-layout.cc");

// HeapLayout 10 判定 + TrustedHeapLayout 4 判定
const preds = [
  "static V8_INLINE bool InReadOnlySpace(Tagged<HeapObject> object);",
  "static V8_INLINE bool InYoungGeneration(Tagged<Object> object);",
  "static V8_INLINE bool InYoungGeneration(Tagged<HeapObject> object);",
  "static V8_INLINE bool InYoungGeneration(Tagged<MaybeObject> object);",
  "static V8_INLINE bool InWritableSharedSpace(Tagged<HeapObject> object);",
  "static V8_INLINE bool InAnySharedSpace(Tagged<HeapObject> object);",
  "static V8_INLINE bool InAnyLargeSpace(Tagged<HeapObject> object);",
  "V8_EXPORT_PRIVATE static bool IsSelfForwarded(Tagged<HeapObject> object);",
  "V8_EXPORT_PRIVATE static bool IsForwardedPointerTo(Tagged<HeapObject> src,",
  "static V8_INLINE bool InCodeSpace(Tagged<HeapObject> object);",
  "static V8_INLINE bool InTrustedSpace(Tagged<HeapObject> object);",
  "static V8_INLINE bool IsOwnedByAnyHeap(Tagged<HeapObject> object);",
  "static V8_INLINE bool InBlackAllocatedPage(Tagged<HeapObject> object);",
];
for (const a of preds) {
  if (!h.includes(a)) throw new Error(`heap-layout.h 谓词缺失: ${a}`);
}
// inl 实现体锚
for (const a of [
  "return MemoryChunk::FromHeapObject(object)->InReadOnlySpace();",
  "return HeapLayout::InReadOnlySpace(value) && CastTraits<T>::AllowFrom(value);",
  "return InYoungGenerationForStickyMarkbits(chunk, object);",
  "const bool in_young_generation = chunk->InYoungGeneration();",
  "return HeapLayout::InWritableSharedSpace(object);",
  "return MemoryChunk::FromHeapObject(object)->IsLargePage();",
  "->is_executable();",
  "->is_trusted();",
  "return MemoryChunk::FromHeapObject(object)->Metadata()->heap();",
  "->is_black_allocated();",
]) {
  if (!inl.includes(a)) throw new Error(`heap-layout-inl.h 实现锚缺失: ${a}`);
}
// InAnySharedSpace: 只读对象直接算共享堆 (return true), 其余看可写共享
if (!inl.includes("if (HeapLayout::InReadOnlySpace(object)) {\n    return true;")) {
  throw new Error("InAnySharedSpace 只读臂缺失");
}
// cc: sticky mark bits / self-forwarded
for (const a of [
  "bool HeapLayout::InYoungGenerationForStickyMarkbits(const MemoryChunk* chunk,",
  "CHECK(v8_flags.sticky_mark_bits.value());",
  "return !chunk->IsOnlyOldOrMajorMarkingOn() &&",
  "bool HeapLayout::IsSelfForwarded(Tagged<HeapObject> object) {",
  "return map_word == MapWord::FromForwardingAddress(object, object);",
  "bool HeapLayout::IsForwardedPointerTo(Tagged<HeapObject> src,",
  "src->map_word(kRelaxedLoad).ToForwardingAddress(src) == dest;",
]) {
  if (!cc.includes(a)) throw new Error(`heap-layout.cc 锚缺失: ${a}`);
}
console.log("HeapLayout 谓词 9 + TrustedHeapLayout 谓词 4 (重载计 13 声明)");

let rust = banner(
  "src/heap/heap-layout.h.js",
  "src/heap/heap-layout.{h,cc,-inl.h}",
  "堆布局判定: 只读/新生代/共享/大对象/代码/可信空间谓词台账",
);

rust += `
/// 页级标志输入 (对应 MemoryChunk / BasePage 元数据的判定输入)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChunkLayoutFacts {
  pub in_read_only_space: bool,
  pub in_young_generation: bool,
  pub in_writable_shared_space: bool,
  pub is_large_page: bool,
  pub is_executable: bool,
  pub is_trusted: bool,
  pub owned_by_heap: bool,
  pub is_black_allocated: bool,
}

/// 对应 class HeapLayout final : AllStatic (不依赖实例类型, 只用堆设施)。
/// v8: src/heap/heap-layout.h HeapLayout
pub struct HeapLayout;

impl HeapLayout {
  /// 对应 InReadOnlySpace
  /// v8: src/heap/heap-layout-inl.h HeapLayout::InReadOnlySpace
  #[must_use]
  pub fn in_read_only_space(f: &ChunkLayoutFacts) -> bool {
    f.in_read_only_space
  }

  /// 对应 InYoungGeneration (chunk 视角; sticky mark bits 臂见 cc)
  /// v8: src/heap/heap-layout-inl.h HeapLayout::InYoungGeneration
  #[must_use]
  pub fn in_young_generation(f: &ChunkLayoutFacts) -> bool {
    f.in_young_generation
  }

  /// 对应 InYoungGenerationForStickyMarkbits:
  /// sticky 下新判据 = 非 only-old marking 且对象标记位未置
  /// v8: src/heap/heap-layout.cc HeapLayout::InYoungGenerationForStickyMarkbits
  #[must_use]
  pub fn in_young_generation_sticky(is_only_old_or_major_marking_on: bool, mark_bit_set: bool) -> bool {
    !is_only_old_or_major_marking_on && !mark_bit_set
  }

  /// 对应 InWritableSharedSpace
  #[must_use]
  pub fn in_writable_shared_space(f: &ChunkLayoutFacts) -> bool {
    f.in_writable_shared_space
  }

  /// 对应 InAnySharedSpace: 只读空间的对象本身在共享只读堆 → true;
  /// 其余看是否在可写共享空间
  /// v8: src/heap/heap-layout-inl.h HeapLayout::InAnySharedSpace
  #[must_use]
  pub fn in_any_shared_space(f: &ChunkLayoutFacts) -> bool {
    if f.in_read_only_space {
      return true;
    }
    f.in_writable_shared_space
  }

  /// 对应 InAnyLargeSpace: 页的 IsLargePage
  #[must_use]
  pub fn in_any_large_space(f: &ChunkLayoutFacts) -> bool {
    f.is_large_page
  }

  /// 对应 IsSelfForwarded: map word == 指向自身的转发地址
  /// (Scavenger 钉住对象/活大对象的表现)
  /// v8: src/heap/heap-layout.cc HeapLayout::IsSelfForwarded
  #[must_use]
  pub fn is_self_forwarded(map_word: u64, object: u64, from_forwarding_address: impl Fn(u64, u64) -> u64) -> bool {
    map_word == from_forwarding_address(object, object)
  }

  /// 对应 IsForwardedPointerTo: src 的转发地址 == dest
  /// v8: src/heap/heap-layout.cc HeapLayout::IsForwardedPointerTo
  #[must_use]
  pub fn is_forwarded_pointer_to(forwarded: u64, dest: u64) -> bool {
    forwarded == dest
  }
}

/// 对应 class TrustedHeapLayout final : AllStatic (sandbox 下只用可信元数据)。
/// v8: src/heap/heap-layout.h TrustedHeapLayout
pub struct TrustedHeapLayout;

impl TrustedHeapLayout {
  /// 对应 InCodeSpace: 元数据 is_executable (各代码空间统一覆盖)
  #[must_use]
  pub fn in_code_space(f: &ChunkLayoutFacts) -> bool {
    f.is_executable
  }

  /// 对应 InTrustedSpace
  #[must_use]
  pub fn in_trusted_space(f: &ChunkLayoutFacts) -> bool {
    f.is_trusted
  }

  /// 对应 IsOwnedByAnyHeap: metadata->heap() 非空 (序列化期与 !InReadOnlySpace 有差)
  #[must_use]
  pub fn is_owned_by_any_heap(f: &ChunkLayoutFacts) -> bool {
    f.owned_by_heap
  }

  /// 对应 InBlackAllocatedPage
  #[must_use]
  pub fn in_black_allocated_page(f: &ChunkLayoutFacts) -> bool {
    f.is_black_allocated
  }
}

// —— 提取不变式断言 ——
const _: () = {
  let f = ChunkLayoutFacts { in_read_only_space: true, ..Default::default() };
  assert!(HeapLayout::in_read_only_space(&f));
  assert!(HeapLayout::in_any_shared_space(&f)); // 只读对象在共享只读堆
  let f = ChunkLayoutFacts { in_writable_shared_space: true, ..Default::default() };
  assert!(HeapLayout::in_any_shared_space(&f));
  assert!(HeapLayout::in_young_generation_sticky(false, false));
  assert!(!HeapLayout::in_young_generation_sticky(true, false));
  assert!(!HeapLayout::in_young_generation_sticky(false, true));
  // 自转发: map_word 等于"object 转发到自身"编码
  let fwd = |o: u64, d: u64| o ^ d ^ 0x1234;
  assert!(HeapLayout::is_self_forwarded(fwd(7, 7), 7, fwd));
  assert!(!HeapLayout::is_self_forwarded(fwd(7, 8), 7, fwd));
  assert!(HeapLayout::is_forwarded_pointer_to(9, 9));
};
`;

const p = writeSrc("v8-heap-core", "heap_layout.rs.skel", rust);
console.log(`written ${p}`);
