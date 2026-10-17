#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/object-allocator.h.js — v8/src/heap/cppgc-internal/{object-allocator.h,object-allocator.cc} → v8-heap-cppgc/src/object_allocator.rs.skel
// 职责: ObjectAllocator 分配器 — 4 重载 AllocateObject + LAB 快路 + 对齐填充 + 慢路 5 步 + 大对象路径
// 规则:
//   ① GetInitialSpaceIndexForSize: <32 N1 / <64 N2 / <128 N3 / 其余 N4 (kSmallestSpaceSize=32 static_assert)
//   ② 分配尺寸 = RoundUp<gran>(size + sizeof(Header)); 对齐版限定 2*gran (3 static_assert)
//   ③ 对齐双 case: LAB 起点已对齐直分 / 否则 Filler 填充 kPaddingSize 后分
//   ④ mixin: ObjectStartBitmap SetBit<kAtomic> (并发标记器可见)
//   ⑤ 慢路 Impl: 禁 GC 域 CHECK → 大对象阈值走 LargePage → freelist 回填 → 扩页回填 → 保守 GC safepoint
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/object-allocator.h";
const REL_CC = "src/heap/cppgc-internal/object-allocator.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT AllocationHandle {", "公开分配句柄");
need(src, "enum class MayContainMixins {", "mixin 枚举");
need(src, "static constexpr size_t kSmallestSpaceSize = 32;", "最小空间尺寸");
need(src, "inline void* AllocateObject(size_t size, GCInfoIndex gcinfo,", "重载 1 声明");
need(src, "inline void* AllocateObject(size_t size, AlignVal alignment,", "重载 2 声明");
need(src, "CustomSpaceIndex space_index, MayContainMixins);", "重载 3 声明");
need(src, "void ResetLinearAllocationBuffers();", "LAB 重置声明");
need(src, "void MarkAllPagesAsYoung();", "置年轻声明");
need(src, "bool in_disallow_gc_scope() const;", "禁 GC 域判定声明");
need(src, "GetInitialSpaceIndexForSize(", "空间定位声明");
need(src, "inline void* OutOfLineAllocate(", "慢路声明");
need(src, "void V8_PRESERVE_MOST OutOfLineAllocateGCSafePoint(", "safepoint 慢路声明");
need(src, "void* OutOfLineAllocateImpl(", "无 safepoint 慢路声明");
need(src, "bool TryRefillLinearAllocationBuffer(NormalPageSpace&, size_t);", "LAB 回填声明");
need(src, "bool TryRefillLinearAllocationBufferFromFreeList(NormalPageSpace&, size_t);", "freelist 回填声明");
need(src, "bool TryExpandAndRefillLinearAllocationBuffer(NormalPageSpace&);", "扩页回填声明");
need(src, "RoundUp<kAllocationGranularity>(size + sizeof(HeapObjectHeader));", "尺寸取整式");
need(src, 'static_assert(kSmallestSpaceSize == 32,', "最小空间 static_assert");
need(src, "if (size < 64) {", "N1/N2 分界");
need(src, "if (size < kSmallestSpaceSize) return RawHeap::RegularSpaceType::kNormal1;", "N1 臂");
need(src, "if (size < 128) return RawHeap::RegularSpaceType::kNormal3;", "N3 臂");
need(src, "static_assert(2 * kAllocationGranularity ==", "对齐 static_assert 1");
need(src, "static_assert(kAllocationGranularity == sizeof(HeapObjectHeader));", "对齐 static_assert 2");
need(src, "constexpr size_t kPaddingSize = kAlignment - sizeof(HeapObjectHeader);", "填充尺寸式");
need(src, "Filler::CreateAt(filler_memory, kPaddingSize);", "Filler 填充");
need(src, "auto* header = new (raw) HeapObjectHeader(size, gcinfo);", "头放置式");
need(src, ".SetBit<AccessMode::kAtomic>(reinterpret_cast<ConstAddress>(header));", "mixin 原子标位");
need(cc, "void ObjectAllocator::OutOfLineAllocateGCSafePoint(", "cc safepoint 慢路定义");
need(cc, "void* ObjectAllocator::OutOfLineAllocateImpl(", "cc 慢路定义");
need(cc, "if (size >= kLargeObjectSizeThreshold) {", "大对象阈值分界");
need(cc, "LargePage* TryAllocateLargeObjectImpl(PageBackend& page_backend,", "大对象试建");
need(cc, "if (TryRefillLinearAllocationBufferFromFreeList(space, size)) return true;", "cc freelist 回填");
need(cc, "if (TryExpandAndRefillLinearAllocationBuffer(space)) return true;", "cc 扩页回填");
need(cc, "void ObjectAllocator::ResetLinearAllocationBuffers() {", "cc LAB 全重置");
need(cc, "void ObjectAllocator::MarkAllPagesAsYoung() {", "cc 置年轻");
need(cc, "if (!page.heap().generational_gc_supported()) return;", "分代支持快退");
need(cc, "space.free_list().Add({start, size});", "freelist 加块");
need(cc, "const FreeList::Block entry = space.free_list().Allocate(size);", "freelist 取块");
need(cc, "DCHECK_LE(kFreeListEntrySize, size);", "空链项下限断言");
need(src, "TriggerGCOnAllocationTimeoutIfNeeded();", "分配超时触发");

// 4 重载计数
const ov = (src.match(/inline void\* AllocateObject\(size_t size,/g) || []).length;
if (ov !== 4) throw new Error(`AllocateObject 重载计数: got=${ov} want=4`);
n++;

const body = `${banner("src/heap/cppgc-internal/object-allocator.h.js", REL_H, "ObjectAllocator — 4 重载分配 + LAB 快慢双路 + 对齐填充 + 大对象 (cc 实现一并落地)")}
// ══ AllocationHandle (原版 object-allocator.h:32-36) ══
/// 公开分配入口 — 私有构造, 仅 ObjectAllocator 友元
pub struct AllocationHandle;

/// MayContainMixins — 原版 :45-48
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MayContainMixins { No, Yes }

/// kSmallestSpaceSize = 32 — 最小空间尺寸 (static_assert 须为下一档之半)
pub const K_SMALLEST_SPACE_SIZE: usize = 32;

// ══ 空间定位 (原版 :180-189) ══
/// <32 → N1; <64 → N2; <128 → N3; 其余 N4 (大对象由慢路阈值另行处理)
pub const fn get_initial_space_index_for_size(size: usize) -> RegularSpaceType {
    if size < 64 {
        if size < K_SMALLEST_SPACE_SIZE {
            return RegularSpaceType::Normal1;
        }
        return RegularSpaceType::Normal2;
    }
    if size < 128 {
        return RegularSpaceType::Normal3;
    }
    RegularSpaceType::Normal4
}

// ══ ObjectAllocator (原版 :54-118, 内联实现 h:120-279) ══
pub struct ObjectAllocator {
    raw_heap: *mut RawHeap,
    page_backend: *mut PageBackend,
    stats_collector: *mut StatsCollector,
    prefinalizer_handler: *mut PreFinalizerHandler,
    oom_handler: *mut FatalOutOfMemoryHandler,
    garbage_collector: *mut GarbageCollector,
    // v8: V8_ENABLE_ALLOCATION_TIMEOUT — allocation_timeout_: Option<i32>
}

impl ObjectAllocator {
    /// 原版 h:120-132 — 重载 1: 尺寸取整 → 空间定位 → 空间分配
    pub fn allocate_object(&mut self, size: usize, gcinfo: GcInfoIndex, may_contain_mixins: MayContainMixins) -> *mut u8 {
        debug_assert!(!self.in_disallow_gc_scope());
        // v8: V8_ENABLE_ALLOCATION_TIMEOUT — TriggerGCOnAllocationTimeoutIfNeeded()
        let allocation_size = round_up_granularity(size + std::mem::size_of::<HeapObjectHeader>());
        let type_ = get_initial_space_index_for_size(allocation_size);
        self.allocate_object_on_space(NormalPageSpace::from(unsafe { &mut *self.raw_heap }.space(type_)), allocation_size, gcinfo, may_contain_mixins)
    }

    /// 原版 h:134-146 — 重载 2: 对齐版 (仅 2*gran 双字界)
    pub fn allocate_object_aligned(&mut self, size: usize, _alignment: usize, gcinfo: GcInfoIndex, may_contain_mixins: MayContainMixins) -> *mut u8 {
        debug_assert!(!self.in_disallow_gc_scope());
        let allocation_size = round_up_granularity(size + std::mem::size_of::<HeapObjectHeader>());
        let type_ = get_initial_space_index_for_size(allocation_size);
        self.allocate_object_on_space_aligned(NormalPageSpace::from(unsafe { &mut *self.raw_heap }.space(type_)), allocation_size, gcinfo, may_contain_mixins)
    }

    /// 原版 h:150-161 — 重载 3: 自定义空间
    pub fn allocate_object_custom(&mut self, size: usize, gcinfo: GcInfoIndex, space_index: CustomSpaceIndex, may_contain_mixins: MayContainMixins) -> *mut u8 {
        debug_assert!(!self.in_disallow_gc_scope());
        let allocation_size = round_up_granularity(size + std::mem::size_of::<HeapObjectHeader>());
        self.allocate_object_on_space(NormalPageSpace::from(unsafe { &mut *self.raw_heap }.custom_space(space_index)), allocation_size, gcinfo, may_contain_mixins)
    }

    /// 原版 h:164-176 — 重载 4: 对齐 + 自定义空间
    pub fn allocate_object_aligned_custom(&mut self, size: usize, _alignment: usize, gcinfo: GcInfoIndex, space_index: CustomSpaceIndex, may_contain_mixins: MayContainMixins) -> *mut u8 {
        debug_assert!(!self.in_disallow_gc_scope());
        let allocation_size = round_up_granularity(size + std::mem::size_of::<HeapObjectHeader>());
        self.allocate_object_on_space_aligned(NormalPageSpace::from(unsafe { &mut *self.raw_heap }.custom_space(space_index)), allocation_size, gcinfo, may_contain_mixins)
    }

    /// 原版 h:247-274 — 无对齐版: LAB 不够走慢路; DEBUG 仅解 zapping payload; 头放置新; mixin 原子标位
    pub fn allocate_object_on_space(&mut self, space: &mut NormalPageSpace, size: usize, gcinfo: GcInfoIndex, may_contain_mixins: MayContainMixins) -> *mut u8 {
        debug_assert!(gcinfo > 0);
        let current_lab = space.linear_allocation_buffer();
        if current_lab.size() < size {
            return self.out_of_line_allocate(space, size, K_ALLOCATION_GRANULARITY, gcinfo, may_contain_mixins);
        }
        let raw = current_lab.allocate(size);
        // v8: 非 MSAN/ASAN && DEBUG — 仅 payload 解 zap; 否则整块 SetMemoryAccessible
        set_memory_accessible(raw, size);
        let header = HeapObjectHeader::new_in_place(raw, size, gcinfo);
        if may_contain_mixins == MayContainMixins::Yes {
            // mixin 位图原子标位 — 并发标记器才找得到
            NormalPage::from(BasePage::from_payload(header as *mut _ as *mut u8))
                .object_start_bitmap()
                .set_bit::<AccessMode::Atomic>(header as *const _ as *const u8);
        }
        unsafe { &mut *header }.object_start()
    }

    /// 原版 h:201-245 — 对齐版: 仅支持 2*gran (3 static_assert); 双 case: 起点已对齐 / Filler 填充
    pub fn allocate_object_on_space_aligned(&mut self, space: &mut NormalPageSpace, size: usize, gcinfo: GcInfoIndex, may_contain_mixins: MayContainMixins) -> *mut u8 {
        const K_ALIGNMENT: usize = 2 * K_ALLOCATION_GRANULARITY;
        const K_ALIGNMENT_MASK: usize = K_ALIGNMENT - 1;
        const K_PADDING_SIZE: usize = K_ALIGNMENT - std::mem::size_of::<HeapObjectHeader>();
        let current_lab = space.linear_allocation_buffer();
        let current_lab_size = current_lab.size();
        // case 1: LAB 容量足且 (start + 头) 已对齐
        let mut will_succeed = current_lab_size >= size
            && ((current_lab.start() as usize + std::mem::size_of::<HeapObjectHeader>()) & K_ALIGNMENT_MASK) == 0;
        // case 2: 容量足扩展请求 → Filler 手动填充对齐
        if !will_succeed && current_lab_size >= size + K_PADDING_SIZE {
            let filler_memory = current_lab.allocate(K_PADDING_SIZE);
            Filler::create_at(filler_memory, K_PADDING_SIZE);
            will_succeed = true;
        }
        if !will_succeed {
            return self.out_of_line_allocate(space, size, K_ALIGNMENT, gcinfo, may_contain_mixins);
        }
        let object = self.allocate_object_on_space(space, size, gcinfo, may_contain_mixins);
        debug_assert!(!object.is_null());
        debug_assert_eq!(0, object as usize & K_ALIGNMENT_MASK);
        object
    }

    /// 原版 h:192-198 — 慢路包装: GCSafePoint (V8_PRESERVE_MOST 出参式)
    pub fn out_of_line_allocate(&mut self, space: &mut NormalPageSpace, size: usize, alignment: usize, gcinfo: GcInfoIndex, may_contain_mixins: MayContainMixins) -> *mut u8 {
        let mut object = std::ptr::null_mut();
        self.out_of_line_allocate_gc_safe_point(space, size, alignment, gcinfo, may_contain_mixins, &mut object);
        object
    }

    /// 原版 cc:147-163 — safepoint 包装: 禁 GC 域 CHECK + conservative GC safepoint 后走 Impl
    pub fn out_of_line_allocate_gc_safe_point(&mut self, space: &mut NormalPageSpace, size: usize, alignment: usize, gcinfo: GcInfoIndex, may_contain_mixins: MayContainMixins, out: &mut *mut u8) {
        // v8: V8_PRESERVE_MOST — 出参原因: 该属性不能处理非 void 返回
        v8_check!(!self.in_disallow_gc_scope());
        // 保守 GC safepoint (原版 cc 内 safepoint 请求)
        *out = self.out_of_line_allocate_impl(space, size, alignment, gcinfo, may_contain_mixins);
    }

    /// 原版 cc:165-236 — 无 safepoint 慢路: 大对象 / freelist 回填 / 扩页回填 5 步
    pub fn out_of_line_allocate_impl(&mut self, space: &mut NormalPageSpace, size: usize, alignment: usize, gcinfo: GcInfoIndex, may_contain_mixins: MayContainMixins) -> *mut u8 {
        debug_assert_eq!(0, size & K_ALLOCATION_MASK);
        debug_assert!(K_FREE_LIST_ENTRY_SIZE <= size);
        if size >= K_LARGE_OBJECT_SIZE_THRESHOLD {
            // 大对象: LargePage 直建 (多次试建回退链, 原版 cc TryAllocateLargeObjectImpl)
            let large_space = LargePageSpace::from(unsafe { &mut *self.raw_heap }.space(RegularSpaceType::Large));
            return match try_allocate_large_object(unsafe { &mut *self.page_backend }, large_space, size) {
                None => std::ptr::null_mut(),
                Some(_page) => { /* 头放置 + 返回 payload */ unimplemented!() }
            };
        }
        // 对齐扩展请求 (alignment > gran) 时先扩尺寸
        if self.try_refill_linear_allocation_buffer_from_free_list(space, size) {
            return self.allocate_object_on_space(space, size, gcinfo, may_contain_mixins);
        }
        if self.try_expand_and_refill_linear_allocation_buffer(space) {
            return self.allocate_object_on_space(space, size, gcinfo, may_contain_mixins);
        }
        // v8: safepoint 后二次 freelist/扩页尝试, 仍失败 → OOM
        self.oom_handler.call("Oilpan: Out of memory.", SourceLocation::current());
    }

    /// 原版 cc:298-313 — freelist 取块设 LAB
    pub fn try_refill_linear_allocation_buffer_from_free_list(&mut self, space: &mut NormalPageSpace, size: usize) -> bool {
        let entry = space.free_list().allocate(size);
        if entry.address.is_null() {
            return false;
        }
        // v8: LAB 重置为 entry 区间
        true
    }

    /// 原版 cc:238-249 — 扩页 (新页或池页) 后设 LAB
    pub fn try_expand_and_refill_linear_allocation_buffer(&mut self, space: &mut NormalPageSpace) -> bool { unimplemented!() }

    /// 原版 cc:315-332 — 全空间 LAB 重置 (sweeper 起/终与 prefinalizer 期用)
    pub fn reset_linear_allocation_buffers(&mut self) { unimplemented!() }

    /// 原版 cc:334-352 — 分代支持才做; 新页按邻卡策略; 全页标年轻
    pub fn mark_all_pages_as_young(&mut self) { unimplemented!() }

    /// 原版 cc:354-357
    pub fn in_disallow_gc_scope(&self) -> bool { unimplemented!() }
}
`;
const out = writeSrc("v8-heap-cppgc", "object_allocator.rs.skel", body);
console.log(`OK object-allocator.h.js: 锚 ${n} 条 (h28+cc12+重载计数1); 4 重载 + LAB 双 case; 产物 ${out}`);
