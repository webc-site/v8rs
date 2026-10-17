#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/unmarker.h.js — v8/src/heap/cppgc-internal/unmarker.h → v8-heap-cppgc/src/unmarker.rs.skel
// 职责: SequentialUnmarker — 全堆顺序清标访问者, 页级 ResetMarkedBytes + 对象级 Unmark
// 规则:
//   ① private 继承 HeapVisitor CRTP + 构造即 Traverse(heap)
//   ② VisitNormalPage/VisitLargePage 返回 false (终止页内遍历) 且 ResetMarkedBytes
//   ③ VisitHeapObjectHeader: 已标才 Unmark, 恒 true
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/unmarker.h";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need("class SequentialUnmarker final : private HeapVisitor<SequentialUnmarker> {", "CRTP 私有继承");
need("explicit SequentialUnmarker(RawHeap& heap) { Traverse(heap); }", "构造即遍历");
need("bool VisitNormalPage(NormalPage& page) {", "普通页钩子");
need("page.ResetMarkedBytes();", "普通页清字节账");
need("bool VisitLargePage(LargePage& page) {", "大页钩子");
need("if (header.IsMarked()) {", "已标判定");
need("header.Unmark();", "清标");
if ((src.match(/return false;/g) || []).length !== 2)
  throw new Error(`页级 return false 计数: got=${(src.match(/return false;/g) || []).length} want=2`);
n++;

const body = `${banner("src/heap/cppgc-internal/unmarker.h.js", REL, "SequentialUnmarker 全堆顺序清标访问者")}
// ══ SequentialUnmarker (原版 unmarker.h:14-37) ══
// CRTP: private HeapVisitor<SequentialUnmarker>, 构造即 Traverse(RawHeap&) — 一步副作用型访问者。

pub struct SequentialUnmarker;

impl SequentialUnmarker {
    /// 原版 unmarker.h:18 — 构造即遍历全堆
    pub fn new(heap: &mut RawHeap) -> Self {
        heap.traverse_with(Self);
        Self
    }

    /// 原版 unmarker.h:20-23 — 普通页: 清页标字节账, false = 不深入页内 (页级即达)
    pub fn visit_normal_page(page: &mut NormalPage) -> bool {
        page.reset_marked_bytes();
        false
    }

    /// 原版 unmarker.h:25-28 — 大页: 同普通页
    pub fn visit_large_page(page: &mut LargePage) -> bool {
        page.reset_marked_bytes();
        false
    }

    /// 原版 unmarker.h:31-36 — 对象头: 已标才清, 恒续遍历
    fn visit_heap_object_header(header: &mut HeapObjectHeader) -> bool {
        if header.is_marked() {
            header.unmark();
        }
        true
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "unmarker.rs.skel", body);
console.log(`OK unmarker.h.js: 锚 ${n} 条 (CRTP1+构造1+页钩子2+清账1+判定1+清标1+false 计数1); 产物 ${out}`);
