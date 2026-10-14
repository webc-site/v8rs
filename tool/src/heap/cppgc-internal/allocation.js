#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/allocation.js — v8/src/heap/cppgc-internal/allocation.cc → v8-heap-cppgc/src/allocation.rs.skel
// MakeGarbageCollectedTraitInternal::Allocate 八重载: (对齐 x 定制空间 x mixin) 组合全委托 ObjectAllocator
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/allocation.cc";
const src = readV8(REL);

let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`allocation.cc 缺锚 ${label}: ${needle}`);
  n++;
};

need("#define CPPGC_FORCE_ALWAYS_INLINE __attribute__((always_inline))", "强制内联宏");
need("static_assert(api_constants::kLargeObjectSizeThreshold ==", "大对象阈值一致性断言");
need("static_assert(api_constants::kMaxSupportedAlignment >=", "对齐上限断言");
need("ObjectAllocator::MayContainMixins::kNo", "非 mixin 分派");
need("ObjectAllocator::MayContainMixins::kYes", "mixin 分派");
// 八重载签名: 参数组合 (size, [align], index, [space_index], [mixin])
const SIGS = [
  "cppgc::AllocationHandle& handle, size_t size, GCInfoIndex index) {",
  "cppgc::AllocationHandle& handle, size_t size, AlignVal alignment,\n    GCInfoIndex index) {",
  "cppgc::AllocationHandle& handle, size_t size, GCInfoIndex index,\n    CustomSpaceIndex space_index) {",
  "cppgc::AllocationHandle& handle, size_t size, AlignVal alignment,\n    GCInfoIndex index, CustomSpaceIndex space_index) {",
  "cppgc::AllocationHandle& handle, size_t size, GCInfoIndex index, MixinTag) {",
  "cppgc::AllocationHandle& handle, size_t size, AlignVal alignment,\n    GCInfoIndex index, MixinTag) {",
  "cppgc::AllocationHandle& handle, size_t size, GCInfoIndex index,\n    CustomSpaceIndex space_index, MixinTag) {",
  "cppgc::AllocationHandle& handle, size_t size, AlignVal alignment,\n    GCInfoIndex index, CustomSpaceIndex space_index, MixinTag) {",
];
for (const s of SIGS) need(s, "Allocate 重载签名");
// 计数核对: kNo 4 处 / kYes 4 处
const noCount = (src.match(/MayContainMixins::kNo\)/g) || []).length;
const yesCount = (src.match(/MayContainMixins::kYes\)/g) || []).length;
if (noCount !== 4 || yesCount !== 4) throw new Error(`mixin 分派计数失配: no=${noCount} yes=${yesCount} want 4/4`);
n += noCount + yesCount;

const body = `${banner("src/heap/cppgc-internal/allocation.js", REL, "分配入口八重载 → ObjectAllocator::AllocateObject")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/allocation.cc MakeGarbageCollectedTraitInternal
pub mod make_garbage_collected_trait_internal {
    /// v8: Allocate(handle, size, index) — 基础
    pub fn allocate(handle: &mut AllocationHandle, size: usize, index: GcInfoIndex) -> *mut u8 {
        object_allocator(handle).allocate_object(size, index, None, MayContainMixins::No)
    }
    /// v8: Allocate(handle, size, alignment, index) — 带对齐
    pub fn allocate_aligned(handle: &mut AllocationHandle, size: usize, alignment: AlignVal, index: GcInfoIndex) -> *mut u8 {
        object_allocator(handle).allocate_object_aligned(size, alignment, index, None, MayContainMixins::No)
    }
    /// v8: Allocate(handle, size, index, space_index) — 定制空间
    pub fn allocate_in_space(handle: &mut AllocationHandle, size: usize, index: GcInfoIndex, space_index: CustomSpaceIndex) -> *mut u8 {
        object_allocator(handle).allocate_object(size, index, Some(space_index), MayContainMixins::No)
    }
    /// v8: Allocate(handle, size, alignment, index, space_index) — 对齐 + 定制空间
    pub fn allocate_aligned_in_space(handle: &mut AllocationHandle, size: usize, alignment: AlignVal, index: GcInfoIndex, space_index: CustomSpaceIndex) -> *mut u8 {
        object_allocator(handle).allocate_object_aligned(size, alignment, index, Some(space_index), MayContainMixins::No)
    }
    /// v8: Allocate(handle, size, index, MixinTag) — mixin
    pub fn allocate_mixin(handle: &mut AllocationHandle, size: usize, index: GcInfoIndex) -> *mut u8 {
        object_allocator(handle).allocate_object(size, index, None, MayContainMixins::Yes)
    }
    /// v8: Allocate(handle, size, alignment, index, MixinTag) — 对齐 + mixin
    pub fn allocate_aligned_mixin(handle: &mut AllocationHandle, size: usize, alignment: AlignVal, index: GcInfoIndex) -> *mut u8 {
        object_allocator(handle).allocate_object_aligned(size, alignment, index, None, MayContainMixins::Yes)
    }
    /// v8: Allocate(handle, size, index, space_index, MixinTag) — 定制空间 + mixin
    pub fn allocate_in_space_mixin(handle: &mut AllocationHandle, size: usize, index: GcInfoIndex, space_index: CustomSpaceIndex) -> *mut u8 {
        object_allocator(handle).allocate_object(size, index, Some(space_index), MayContainMixins::Yes)
    }
    /// v8: Allocate(handle, size, alignment, index, space_index, MixinTag) — 全参
    pub fn allocate_aligned_in_space_mixin(handle: &mut AllocationHandle, size: usize, alignment: AlignVal, index: GcInfoIndex, space_index: CustomSpaceIndex) -> *mut u8 {
        object_allocator(handle).allocate_object_aligned(size, alignment, index, Some(space_index), MayContainMixins::Yes)
    }
}
`;

writeSrc("v8-heap-cppgc", "allocation.rs.skel", body);
console.log(`allocation.js: 对数 ${n} 项 (宏 1 + 断言 2 + 签名 8 + 分派计数 8) → v8-heap-cppgc/src/allocation.rs.skel`);
