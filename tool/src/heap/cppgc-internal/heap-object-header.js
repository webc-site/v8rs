#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap-object-header.js — v8/src/heap/cppgc-internal/heap-object-header.h + .cc → v8-heap-cppgc/src/heap_object_header.rs.skel
// 对象头位域: 14bit GCInfoIndex + 15bit 尺寸(×8) + markbit + 完整构造位; 大对象尺寸编码 0
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap-object-header.h";
const REL_CC = "src/heap/cppgc-internal/heap-object-header.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

// ── ① 尺寸常量 + 位域布局 ──────────────────────────────────────────────
need(src, "static constexpr size_t kSizeLog2 = 17;", "kSizeLog2 17");
need(src, "static constexpr size_t kMaxSize = (size_t{1} << kSizeLog2) - 1;", "kMaxSize");
need(src, "static constexpr uint16_t kLargeObjectSizeInHeader = 0;", "大对象头内尺寸 0");
need(src, "using GCInfoIndexField = v8::base::BitField16<GCInfoIndex, 0, 14>;", "GCInfoIndexField 14bit@0");
need(src, "using UnusedField1 = GCInfoIndexField::Next<bool, 1>;", "UnusedField1");
need(src, "using FullyConstructedField = UnusedField1::Next<bool, 1>;", "FullyConstructedField");
need(src, "using MarkBitField = v8::base::BitField16<bool, 0, 1>;", "MarkBitField 1bit@0");
need(src, "using SizeField =\n      MarkBitField::Next<size_t, 15>;", "SizeField 15bit@1");
need(src, "return SizeField::decode(encoded) * kAllocationGranularity;", "DecodeSize ×granularity");
need(src, "return SizeField::encode(size / kAllocationGranularity);", "EncodeSize ÷granularity");
need(src, "enum class EncodedHalf : uint8_t { kLow, kHigh };", "EncodedHalf");
need(src, "static_assert(kAllocationGranularity == sizeof(HeapObjectHeader),", "sizeof 断言");
need(src, "uint16_t encoded_high_;\n  uint16_t encoded_low_;", "双半字字段");
// ── ② 方法面 (逐组计数) ────────────────────────────────────────────────
need(src, "inline static HeapObjectHeader& FromObject(void* address);", "FromObject");
need(src, "inline HeapObjectHeader(size_t size, GCInfoIndex gc_info_index);", "构造");
need(src, "bool TryMarkAtomic();", "TryMarkAtomic");
need(src, "inline void MarkNonAtomic();", "MarkNonAtomic");
need(src, "bool IsFinalizable() const;", "IsFinalizable");
need(src, "void Finalize();", "Finalize");
need(src, "V8_EXPORT_PRIVATE HeapObjectName GetName() const;", "GetName");
need(src, "void MarkAsFullyConstructed();", "MarkAsFullyConstructed");
// AccessMode 模板锚计数: 声明 13 (11 方法 + Load/StoreEncoded 2) + 类外定义 13 = 26
const tmpl = (src.match(/template <AccessMode/g) || []).length;
if (tmpl !== 26) throw new Error(`AccessMode 模板方法计数失配: ${tmpl} want 26`);
n += tmpl;
// ── ③ .cc: 64 位小端布局字 ─────────────────────────────────────────────
need(src, "std::atomic_ref<uint64_t>(*reinterpret_cast<uint64_t*>(this))", "64 位整字构造");
need(src, "(static_cast<uint64_t>(EncodeSize(size)) << 48);", "小端 low 半字移 48");
need(src, "      (static_cast<uint64_t>(GCInfoIndexField::encode(gc_info_index)) << 32) |", "小端 high 半字移 32");
need(srcCc, "void HeapObjectHeader::MarkAsFullyConstructed() {", "cc MarkAsFullyConstructed");
need(srcCc, "MakeGarbageCollectedTraitInternal::MarkObjectAsFullyConstructed(", "完整构造委托 API");
need(srcCc, "static_assert(api_constants::kFullyConstructedBitMask ==", "API 位掩码断言");
need(srcCc, "void HeapObjectHeader::Finalize() {", "cc Finalize");
need(srcCc, "gc_info.finalize", "终结回调");

const body = `${banner("src/heap/cppgc-internal/heap-object-header.js", `${REL} + ${REL_CC}`, "对象头: 双 u16 半字位域 + 大小/标记/构造位")}
#![allow(dead_code)]

pub mod constants {
    /// v8: src/heap/cppgc-internal/heap-object-header.h HeapObjectHeader::kSizeLog2
    pub const K_SIZE_LOG2: usize = 17;
    /// v8: HeapObjectHeader::kMaxSize
    pub const K_MAX_SIZE: usize = (1usize << K_SIZE_LOG2) - 1;
    /// v8: HeapObjectHeader::kLargeObjectSizeInHeader — 大对象尺寸不存头内
    pub const K_LARGE_OBJECT_SIZE_IN_HEADER: u16 = 0;
    /// v8: GCInfoIndexField — BitField16<GCInfoIndex, 0, 14>
    pub const K_GC_INFO_INDEX_BITS: u32 = 14;
    /// v8: SizeField — MarkBitField::Next<size_t, 15> (低半字 bit1..15)
    pub const K_SIZE_FIELD_BITS: u32 = 15;
}

/// v8: src/heap/cppgc-internal/heap-object-header.h HeapObjectHeader
/// 布局 (64 位): [next_unfinalized_: u32 | padding][encoded_high_: u16][encoded_low_: u16]
/// - encoded_high_: GCInfoIndex(14) | unused(1) | fully_constructed(1, 0=构造中)
/// - encoded_low_:  mark_bit(1) | size(15, 编码值×granularity)
pub struct HeapObjectHeader {
    /// v8: next_unfinalized_ (CPPGC_CAGED_HEAP) / padding_
    next_unfinalized: u32,
    /// v8: encoded_high_
    encoded_high: u16,
    /// v8: encoded_low_
    encoded_low: u16,
}

impl HeapObjectHeader {
    /// v8: src/heap/cppgc-internal/heap-object-header.h HeapObjectHeader::FromObject
    pub fn from_object(object: *const core::ffi::c_void) -> &'static Self {
        unsafe { &*((object as *const u8).sub(core::mem::size_of::<Self>()) as *const Self) }
    }

    /// v8: src/heap/cppgc-internal/heap-object-header.h HeapObjectHeader::HeapObjectHeader
    /// 64 位小端: 整字 = index<<32 | EncodeSize(size)<<48; 构造位隐 0 (即构造中)。
    pub fn new(size: usize, gc_info_index: GcInfoIndex) -> Self {
        debug_assert!(gc_info_index < K_MAX_GC_INFO_INDEX);
        debug_assert_eq!(0, size & (core::mem::size_of::<Self>() - 1));
        debug_assert!(constants::K_MAX_SIZE >= size);
        Self {
            next_unfinalized: 0,
            encoded_high: (gc_info_index as u16) & ((1 << constants::K_GC_INFO_INDEX_BITS) - 1),
            encoded_low: Self::encode_size(size),
        }
    }

    /// v8: EncodeSize — size / kAllocationGranularity
    fn encode_size(size: usize) -> u16 {
        ((size / K_ALLOCATION_GRANULARITY) as u16) & ((1 << constants::K_SIZE_FIELD_BITS) - 1)
    }
    /// v8: DecodeSize — decoded * kAllocationGranularity
    fn decode_size(encoded: u16) -> usize {
        ((encoded & ((1 << constants::K_SIZE_FIELD_BITS) - 1)) as usize) * K_ALLOCATION_GRANULARITY
    }

    /// v8: GetGCInfoIndex<mode> — acquire 读高半字
    pub fn get_gc_info_index(&self) -> GcInfoIndex {
        self.encoded_high & ((1 << constants::K_GC_INFO_INDEX_BITS) - 1)
    }
    /// v8: AllocatedSize<mode> — relaxed 读低半字
    pub fn allocated_size(&self) -> usize { Self::decode_size(self.encoded_low) }
    /// v8: SetAllocatedSize — 保位写低半字 (不清 mark bit)
    pub fn set_allocated_size(&mut self, size: usize) {
        self.encoded_low &= !(((1 << constants::K_SIZE_FIELD_BITS) - 1) << 1);
        self.encoded_low |= Self::encode_size(size);
    }
    /// v8: ObjectSize — AllocatedSize - sizeof(HeapObjectHeader)
    pub fn object_size(&self) -> usize { self.allocated_size() - core::mem::size_of::<Self>() }
    /// v8: ObjectStart
    pub fn object_start(&self) -> *mut u8 { unsafe { (self as *const Self as *mut u8).add(core::mem::size_of::<Self>()) } }
    /// v8: IsLargeObject — 头内尺寸 == 0
    pub fn is_large_object(&self) -> bool { self.allocated_size() == constants::K_LARGE_OBJECT_SIZE_IN_HEADER as usize }
    /// v8: IsInConstruction — acquire, fully_constructed 位为 0
    pub fn is_in_construction(&self) -> bool { self.encoded_high & 0x4000 == 0 }
    /// v8: MarkAsFullyConstructed (cc) — acquire/release CAS 置位
    pub fn mark_as_fully_constructed(&self) { todo!("atomic_ref<u16> CAS") }
    /// v8: IsMarked<mode> — relaxed 低半字 bit0
    pub fn is_marked(&self) -> bool { self.encoded_low & 0x1 != 0 }
    /// v8: Unmark<mode>
    pub fn unmark(&mut self) { self.encoded_low &= !0x1; }
    /// v8: TryMarkAtomic — relaxed CAS 置 mark bit, 已标返回 false
    pub fn try_mark_atomic(&self) -> bool { todo!("compare_exchange_strong") }
    /// v8: MarkNonAtomic
    pub fn mark_non_atomic(&mut self) { self.encoded_low |= 0x1; }
    /// v8: IsYoung — !IsMarked (非年轻代构建下 sticky bits)
    pub fn is_young(&self) -> bool { !self.is_marked() }
    /// v8: IsFree — index == kFreeListGCInfoIndex
    pub fn is_free(&self) -> bool { self.get_gc_info_index() == K_FREE_LIST_GC_INFO_INDEX }
    /// v8: IsFinalizable — GCInfo.finalize 非空
    pub fn is_finalizable(&self) -> bool { GlobalGcInfoTable::gc_info_from_index(self.get_gc_info_index()).finalize.is_some() }
    /// v8: Finalize (cc) — 调 GCInfo.finalize(this)
    pub fn finalize(&mut self) { todo!("终结回调 + next_unfinalized 链维护") }
    /// v8: GetName / GetName(option) (cc)
    pub fn get_name(&self) -> HeapObjectName { todo!("NameTrait 查询") }
    /// v8: TraceImpl<mode> — GCInfo.trace(visitor, ObjectStart())
    pub fn trace_impl(&self, visitor: &mut Visitor) { todo!("trace 回调") }
}
`;

writeSrc("v8-heap-cppgc", "heap_object_header.rs.skel", body);
console.log(`heap-object-header.js: 对数 ${n} 项 (布局 13 + 方法 9 + 模板计数 23 + cc 5) → v8-heap-cppgc/src/heap_object_header.rs.skel`);
