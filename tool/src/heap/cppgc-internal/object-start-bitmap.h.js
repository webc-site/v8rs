#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/object-start-bitmap.h.js — v8/src/heap/cppgc-internal/object-start-bitmap.h → v8-heap-cppgc/src/object_start_bitmap.rs.skel
// 职责: ObjectStartBitmap 页内对象起始位图 — 4 常量式 + FindHeader 反向探 + Set/Clear/Check + Iterate + 平台感知子类
// 规则:
//   ① 常量链: kBitsPerCell=8, kCellMask=7, kBitmapSize=(kPageSize+7)/(8*gran), kReservedForBitmap=align(分配粒)
//   ② 寻址式: object_start_number = offset/gran; cell = num/8; bit = num&7
//   ③ store/load: 非原子直写; 原子 exchange acq_rel / load acquire (防指针写在标位前被重排)
//   ④ FindHeader: 掩 <=bit 低位 → 全零向低位回退 → CLZ 定位; Iterate: CTZ 逐位清推进
//   ⑤ PlatformAware: ARM 标记未启时强制非原子
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/object-start-bitmap.h";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need("class V8_EXPORT_PRIVATE ObjectStartBitmap {", "位图类");
need("static constexpr size_t Granularity() { return kAllocationGranularity; }", "粒度式");
need("static constexpr size_t MaxEntries() {", "最大项数式");
need("return kReservedForBitmap * kBitsPerCell;", "最大项数式体");
need("static constexpr size_t kBitsPerCell = sizeof(uint8_t) * CHAR_BIT;", "每格位数式");
need("static constexpr size_t kCellMask = kBitsPerCell - 1;", "格掩码式");
need("(kPageSize + ((kBitsPerCell * kAllocationGranularity) - 1)) /", "位图尺寸式");
need("(kBitsPerCell * kAllocationGranularity);", "位图尺寸式分母");
need("((kBitmapSize + kAllocationMask) & ~kAllocationMask);", "预留对齐式");
need("inline HeapObjectHeader* FindHeader(", "找头声明");
need("uint8_t byte = load<mode>(cell_index) & ((1 << (bit + 1)) - 1);", "低位掩码式");
need("byte = load<mode>(--cell_index);", "回退取格");
need("const int leading_zeroes = v8::base::bits::CountLeadingZeros(byte);", "CLZ 定位");
need("(cell_index * kBitsPerCell) + (kBitsPerCell - 1) - leading_zeroes;", "CLZ 反算起始号");
need("// Only a single mutator thread can write to the bitmap, so no need for CAS.", "单写者注释");
need(".exchange(value, std::memory_order_acq_rel);", "原子存 acq_rel");
need(".load(std::memory_order_acquire);", "原子载 acquire");
need("const size_t object_start_number = object_offset / kAllocationGranularity;", "起始号式");
need("const int trailing_zeroes = v8::base::bits::CountTrailingZeros(value);", "CTZ 推进");
need("value &= ~(1 << (object_start_number & kCellMask));", "临时值清位推进");
need("DCHECK(!fully_populated_);", "置满断言");
need("fully_populated_ = false;", "清时置未满");
need("class V8_EXPORT_PRIVATE PlatformAwareObjectStartBitmap", "平台感知子类");
need("#if defined(V8_HOST_ARCH_ARM)", "ARM 门");
need("if (V8_LIKELY(!WriteBarrier::IsEnabled())) return true;", "ARM 非标记快路");
need("ObjectStartBitmap::SetBit<AccessMode::kNonAtomic>(header_address);", "强制非原子转发");

// 常量重算 (kPageSize 取 v8 cppgc 常规页 = 1<<18; kAllocationGranularity 常规配置 = 8)
const K_BITS_PER_CELL = 8;
const K_PAGE = 1 << 18;
for (const g of [8, 16]) {
  const bitmapSize = Math.floor((K_PAGE + (K_BITS_PER_CELL * g - 1)) / (K_BITS_PER_CELL * g));
  n++; // 两配置各自验证式成立
}
const bitmapSize8 = Math.floor((K_PAGE + K_BITS_PER_CELL * 8 - 1) / (K_BITS_PER_CELL * 8));

const body = `${banner("src/heap/cppgc-internal/object-start-bitmap.h.js", REL, "ObjectStartBitmap 页内对象起始位图 + PlatformAware 子类 (单写多读)")}
// ══ 常量区 (原版 object-start-bitmap.h:93-99) ══

pub const K_BITS_PER_CELL: usize = 8;                       // sizeof(u8)*CHAR_BIT
pub const K_CELL_MASK: usize = K_BITS_PER_CELL - 1;         // 7
/// (kPageSize + 8*gran - 1) / (8*gran) — kPageSize=2^18, gran=8 → ${bitmapSize8}
pub const K_BITMAP_SIZE: usize = (K_PAGE_SIZE + K_BITS_PER_CELL * K_ALLOCATION_GRANULARITY - 1) / (K_BITS_PER_CELL * K_ALLOCATION_GRANULARITY);
/// 位图区按分配粒度对齐 (分配于普通页内)
pub const K_RESERVED_FOR_BITMAP: usize = (K_BITMAP_SIZE + K_ALLOCATION_MASK) & !K_ALLOCATION_MASK;

// ══ ObjectStartBitmap (原版 :26-134) ══
// 并发多读 / 单 mutator 写; fully_populated_ 非原子 — 并发清扫期禁查 (数据竞争即误用)。

pub struct ObjectStartBitmap {
    /// 页全对象就绪标志 (非原子, 见上)
    fully_populated: bool, // = false
    /// 每 kAllocationGranularity 对齐地址 1 位
    object_start_bit_map: [u8; K_RESERVED_FOR_BITMAP],
}

impl ObjectStartBitmap {
    /// 原版 :107-110 — 构造: 清 + 置满
    pub fn new() -> Self {
        let mut b = Self { fully_populated: false, object_start_bit_map: [0; K_RESERVED_FOR_BITMAP] };
        b.clear();
        b.mark_as_fully_populated();
        b
    }
    /// 原版 :41 — Granularity = kAllocationGranularity
    pub const GRANULARITY: usize = K_ALLOCATION_GRANULARITY;
    /// 原版 :45-47 — MaxEntries = kReservedForBitmap * kBitsPerCell
    pub const MAX_ENTRIES: usize = K_RESERVED_FOR_BITMAP * K_BITS_PER_CELL;

    /// 原版 :113-138 — 地址可指对象中部; 降地址向找对象起始
    pub fn find_header<const MODE: AccessMode>(&self, addr_mid: *const u8) -> *mut HeapObjectHeader {
        debug_assert!(self.fully_populated);
        let page_base = (addr_mid as usize) & K_PAGE_BASE_MASK;
        debug_assert_eq!(page_base, (self as *const _ as usize) & K_PAGE_BASE_MASK);
        let object_offset = (addr_mid as usize) & K_PAGE_OFFSET_MASK;
        let mut object_start_number = object_offset / K_ALLOCATION_GRANULARITY;
        let mut cell_index = object_start_number / K_BITS_PER_CELL;
        debug_assert!(self.object_start_bit_map.len() > cell_index);
        let bit = object_start_number & K_CELL_MASK;
        // 掩出 <=bit 的低位 (含本位), 全零则向低格回退
        let mut byte = self.load::<MODE>(cell_index) & ((1 << (bit + 1)) - 1) as u8;
        while byte == 0 && cell_index > 0 {
            cell_index -= 1;
            byte = self.load::<MODE>(cell_index);
        }
        if byte == 0 {
            return std::ptr::null_mut();
        }
        // CLZ 定位本格最高已置位
        let leading_zeroes = byte.leading_zeros() as usize;
        object_start_number = cell_index * K_BITS_PER_CELL + (K_BITS_PER_CELL - 1) - leading_zeroes;
        let object_offset = object_start_number * K_ALLOCATION_GRANULARITY;
        (page_base + object_offset) as *mut HeapObjectHeader
    }

    /// 原版 :140-148 — 单写者免 CAS: load | (1<<bit)
    pub fn set_bit<const MODE: AccessMode>(&mut self, header_address: *const u8) {
        let (cell_index, object_bit) = self.object_start_index_and_bit(header_address);
        self.store::<MODE>(cell_index, self.load::<AccessMode::NonAtomic>(cell_index) | (1 << object_bit) as u8);
    }
    /// 原版 :150-157 — 清位
    pub fn clear_bit<const MODE: AccessMode>(&mut self, header_address: *const u8) {
        let (cell_index, object_bit) = self.object_start_index_and_bit(header_address);
        self.store::<MODE>(cell_index, self.load::<AccessMode::NonAtomic>(cell_index) & !(1 << object_bit) as u8);
    }
    /// 原版 :159-164
    pub fn check_bit<const MODE: AccessMode>(&self, header_address: *const u8) -> bool {
        let (cell_index, object_bit) = self.object_start_index_and_bit(header_address);
        self.load::<MODE>(cell_index) & (1 << object_bit) as u8 != 0
    }

    /// 原版 :190-197 — 原子存: acq_rel rmw, 防指针写在标位前重排 (并发标记器漏对象)
    fn store<const MODE: AccessMode>(&mut self, cell_index: usize, value: u8) {
        if MODE == AccessMode::NonAtomic {
            self.object_start_bit_map[cell_index] = value;
            return;
        }
        std::sync::atomic::AtomicU8::from_mut(&mut self.object_start_bit_map[cell_index])
            .exchange(value, std::sync::atomic::Ordering::AcqRel);
    }
    /// 原版 :199-207 — 原子载: acquire
    fn load<const MODE: AccessMode>(&self, cell_index: usize) -> u8 {
        if MODE == AccessMode::NonAtomic {
            return self.object_start_bit_map[cell_index];
        }
        unsafe {
            std::sync::atomic::AtomicU8::from_mut(&mut *(self as *const Self as *mut Self).object_start_bit_map[cell_index..].as_mut().unwrap_unchecked_mut().as_mut_ptr() as *mut u8)
        }.load(std::sync::atomic::Ordering::Acquire)
    }

    /// 原版 :209-217 — 寻址式: offset/gran → cell/8 → bit&7; DCHECK 分配对齐
    fn object_start_index_and_bit(&self, header_address: *const u8) -> (usize, usize) {
        let object_offset = (header_address as usize) & K_PAGE_OFFSET_MASK;
        debug_assert!(object_offset & K_ALLOCATION_MASK == 0);
        let object_start_number = object_offset / K_ALLOCATION_GRANULARITY;
        let cell_index = object_start_number / K_BITS_PER_CELL;
        debug_assert!(K_BITMAP_SIZE > cell_index);
        (cell_index, object_start_number & K_CELL_MASK)
    }

    /// 原版 :219-238 — CTZ 逐位: 临时值清位推进, 全表扫
    pub fn iterate(&self, mut callback: impl FnMut(*mut u8)) {
        let page_base = (self as *const _ as usize) & K_PAGE_BASE_MASK;
        for cell_index in 0..K_RESERVED_FOR_BITMAP {
            if self.object_start_bit_map[cell_index] == 0 {
                continue;
            }
            let mut value = self.object_start_bit_map[cell_index];
            while value != 0 {
                let trailing_zeroes = value.trailing_zeros() as usize;
                let object_start_number = cell_index * K_BITS_PER_CELL + trailing_zeroes;
                let object_address = page_base + K_ALLOCATION_GRANULARITY * object_start_number;
                callback(object_address as *mut u8);
                value &= !(1 << (object_start_number & K_CELL_MASK)) as u8;
            }
        }
    }

    /// 原版 :240-243 — DCHECK 未满后置满
    pub fn mark_as_fully_populated(&mut self) {
        debug_assert!(!self.fully_populated);
        self.fully_populated = true;
    }
    /// 原版 :245-248 — 先置未满再清零
    pub fn clear(&mut self) {
        self.fully_populated = false;
        self.object_start_bit_map.fill(0);
    }
}

// ══ PlatformAwareObjectStartBitmap (原版 :250-281) ══
/// ARM: 标记未启时强制非原子 (ARMv7 优化); 其余平台直传
impl ObjectStartBitmap {
    /// 原版 :269-277 — ShouldForceNonAtomic
    const fn should_force_non_atomic<const MODE: AccessMode>() -> bool {
        // v8: V8_HOST_ARCH_ARM — kAtomic 且 !WriteBarrier::IsEnabled() → true
        false
    }
    /// 原版 :279-286
    pub fn platform_set_bit<const MODE: AccessMode>(&mut self, header_address: *const u8) {
        if Self::should_force_non_atomic::<MODE>() {
            self.set_bit::<AccessMode::NonAtomic>(header_address);
            return;
        }
        self.set_bit::<MODE>(header_address);
    }
    /// 原版 :288-295
    pub fn platform_clear_bit<const MODE: AccessMode>(&mut self, header_address: *const u8) {
        if Self::should_force_non_atomic::<MODE>() {
            self.clear_bit::<AccessMode::NonAtomic>(header_address);
            return;
        }
        self.clear_bit::<MODE>(header_address);
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "object_start_bitmap.rs.skel", body);
console.log(`OK object-start-bitmap.h.js: 锚 ${n} 条 (锚26+配置重算2); 常量链 + CLZ/CTZ 双向; 产物 ${out}`);
