#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/heap-statistics-collector.js — v8/src/heap/cppgc-internal/heap-statistics-collector.h + .cc → v8-heap-cppgc/src/heap_statistics_collector.rs.skel
// 详细堆统计遍历器: 空间命名规则 + 三级聚合 (page→space→heap) + 类型名规范化
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/heap-statistics-collector.h";
const REL_CC = "src/heap/cppgc-internal/heap-statistics-collector.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class HeapStatisticsCollector : private HeapVisitor<HeapStatisticsCollector> {", "CRTP 私有继承");
need(src, "HeapStatistics CollectDetailedStatistics(HeapBase*);", "CollectDetailedStatistics");
for (const m of ["bool VisitNormalPageSpace(NormalPageSpace&);", "bool VisitLargePageSpace(LargePageSpace&);",
  "bool VisitNormalPage(NormalPage&);", "bool VisitLargePage(LargePage&);", "bool VisitHeapObjectHeader(HeapObjectHeader&);"]) need(src, m, "Visit 钩子");
need(src, "std::unordered_map<const void*, size_t> type_name_to_index_map_;", "类型名索引映射");
// .cc 命名规则
need(srcCc, 'return "NormalPageSpace" + std::to_string(index);', "常规空间命名");
need(srcCc, 'return "CustomSpace" +', "定制空间命名");
need(srcCc, 'space_stats->name = std::move(name);', "空间名落位");
need(srcCc, 'space_stats->committed_size_bytes += (*page_stats)->committed_size_bytes;', "页→空间聚合");
need(srcCc, 'stats->committed_size_bytes += (*space_stats)->committed_size_bytes;', "空间→堆聚合");
need(srcCc, "object_statistics[type_index].allocated_bytes += object_size;", "类型累计字节");
need(srcCc, "object_statistics[type_index].object_count++;", "类型累计计数");
need(srcCc, "const auto it = type_map.insert({header->GetName().value, type_map.size()});", "类型名规范插入");
need(srcCc, "stats.detail_level = HeapStatistics::DetailLevel::kDetailed;", "详细级别");
need(srcCc, "ClassNameAsHeapObjectNameScope class_names_scope(*heap);", "类名作用域");
need(srcCc, "current_page_stats_->committed_size_bytes = kPageSize;", "常规页提交 = 页尺寸");
need(srcCc, "current_page_stats_->resident_size_bytes =\n      kPageSize - page.discarded_memory();", "常规页驻留 = 页尺寸 - discard");
need(srcCc, "const size_t allocated_size = LargePage::AllocationSize(object_size);", "大页分配尺寸");
need(srcCc, "size_t pooled_memory = heap->page_backend()->page_pool().PooledMemory();", "池化内存并入");
need(srcCc, "if (header.IsFree()) return true;", "自由项跳过");
// Visit 方法实体计数 5
const visits = (srcCc.match(/^bool HeapStatisticsCollector::Visit/gm) || []).length;
if (visits !== 5) throw new Error(`Visit 实体计数失配: ${visits} want 5`);
n += visits;

const body = `${banner("src/heap/cppgc-internal/heap-statistics-collector.js", `${REL} + ${REL_CC}`, "先序遍历产出详细堆统计")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/heap-statistics-collector.cc GetNormalPageSpaceName
fn get_normal_page_space_name(index: usize) -> String {
    debug_assert_ne!(RawHeap::K_NUMBER_OF_REGULAR_SPACES - 1, index);
    if index < RawHeap::K_NUMBER_OF_REGULAR_SPACES {
        format!("NormalPageSpace{index}")
    } else {
        format!("CustomSpace{}", index - RawHeap::K_NUMBER_OF_REGULAR_SPACES)
    }
}

/// v8: src/heap/cppgc-internal/heap-statistics-collector.cc RecordObjectType
/// type_map 插入 (name.value, size); 命中旧值则复用索引; entry{allocated_bytes, object_count} 累计。
fn record_object_type(
    type_map: &mut HashMap<*const core::ffi::c_void, usize>,
    object_statistics: &mut Vec<ObjectStatsEntry>,
    header: &HeapObjectHeader,
    object_size: usize,
) {
    let next = type_map.len();
    let type_index = *type_map.entry(header.get_name().value).or_insert(next);
    if object_statistics.len() <= type_index {
        object_statistics.resize(type_index + 1, ObjectStatsEntry::default());
    }
    object_statistics[type_index].allocated_bytes += object_size;
    object_statistics[type_index].object_count += 1;
}

/// v8: src/heap/cppgc-internal/heap-statistics-collector.h HeapStatisticsCollector
pub struct HeapStatisticsCollector {
    /// v8: current_stats_
    current_stats: *mut HeapStatistics,
    /// v8: current_space_stats_
    current_space_stats: Option<usize>,
    /// v8: current_page_stats_
    current_page_stats: Option<usize>,
    /// v8: type_name_to_index_map_
    type_name_to_index_map: HashMap<*const core::ffi::c_void, usize>,
}

impl HeapStatisticsCollector {
    /// v8: src/heap/cppgc-internal/heap-statistics-collector.cc CollectDetailedStatistics
    /// detail=kDetailed + ClassNameAsHeapObjectNameScope + Traverse + FinalizeSpace +
    /// type_names 回填 + 池化内存并入 committed/resident。
    pub fn collect_detailed_statistics(&mut self, heap: &mut HeapBase) -> HeapStatistics {
        let mut stats = HeapStatistics { detail_level: DetailLevel::Detailed, ..Default::default() };
        self.current_stats = &mut stats;
        let _scope = ClassNameAsHeapObjectNameScope::new(heap);
        traverse_heap(self, heap.raw_heap_mut());
        todo!("FinalizeSpace + type_names 回填 + pooled memory 并入")
    }

    /// v8: src/heap/cppgc-internal/heap-statistics-collector.cc VisitNormalPageSpace
    /// FinalizeSpace → InitializeSpace(GetNormalPageSpaceName(index)) → free_list().CollectStatistics
    pub fn visit_normal_page_space(&mut self, space: &mut NormalPageSpace) -> bool {
        debug_assert_eq!(0, space.linear_allocation_buffer().size());
        todo!("空间级统计开卷")
    }

    /// v8: VisitLargePageSpace — FinalizeSpace → InitializeSpace("LargePageSpace")
    pub fn visit_large_page_space(&mut self, _space: &mut LargePageSpace) -> bool { todo!("大页空间开卷") }

    /// v8: VisitNormalPage — committed = kPageSize; resident = kPageSize - discarded
    pub fn visit_normal_page(&mut self, page: &mut NormalPage) -> bool {
        let _ = page;
        todo!("页级统计")
    }

    /// v8: VisitLargePage — committed = resident = LargePage::AllocationSize(PayloadSize)
    pub fn visit_large_page(&mut self, page: &mut LargePage) -> bool {
        let _ = page;
        todo!("大页页级统计")
    }

    /// v8: VisitHeapObjectHeader — IsFree 跳过; 尺寸 = 大页 PayloadSize 或 AllocatedSize; RecordObjectType
    pub fn visit_heap_object_header(&mut self, header: &mut HeapObjectHeader) -> bool {
        if header.is_free() { return true; }
        todo!("对象级累计")
    }
}

/// v8: src/heap/cppgc-internal/heap-statistics-collector.cc FinalizePage / FinalizeSpace
/// page: committed/resident/used → space; space: 同三项 → heap stats。
fn finalize_page_and_space() { todo!("三级聚合 (页→空间→堆)") }
`;

writeSrc("v8-heap-cppgc", "heap_statistics_collector.rs.skel", body);
console.log(`heap-statistics-collector.js: 对数 ${n} 项 (.h 7 + .cc 14 + 实体计数 5) → v8-heap-cppgc/src/heap_statistics_collector.rs.skel`);
