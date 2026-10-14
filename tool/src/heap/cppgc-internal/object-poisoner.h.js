#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/object-poisoner.h.js — v8/src/heap/cppgc-internal/object-poisoner.h → v8-heap-cppgc/src/object_poisoner.rs.skel
// 职责: UnmarkedObjectsPoisoner — ASAN 专属访问者, 对未标活对象 payload 打毒
// 规则:
//   ① 整类 V8_USE_ADDRESS_SANITIZER 守护 → Rust cfg(sanitize = "address") 门
//   ② HeapVisitor CRTP 单方法 VisitHeapObjectHeader: IsFree||IsMarked 早退, 否则 ASAN_POISON_MEMORY_REGION(ObjectStart, ObjectView<>.Size())
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/object-poisoner.h";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need("#ifdef V8_USE_ADDRESS_SANITIZER", "ASAN 守护开");
need("#endif  // V8_USE_ADDRESS_SANITIZER", "ASAN 守护关");
need("class UnmarkedObjectsPoisoner : public HeapVisitor<UnmarkedObjectsPoisoner> {", "CRTP 基类");
need("bool VisitHeapObjectHeader(HeapObjectHeader& header) {", "唯一方法");
need("if (header.IsFree() || header.IsMarked()) return true;", "早退条件");
need("ASAN_POISON_MEMORY_REGION(header.ObjectStart(),", "打毒调用");
need("ObjectView<>(header).Size());", "尺寸取 ObjectView");

const body = `${banner("src/heap/cppgc-internal/object-poisoner.h.js", REL, "UnmarkedObjectsPoisoner — ASAN 未标对象 payload 打毒访问者")}
// ══ UnmarkedObjectsPoisoner (原版 object-poisoner.h:20-31, 整类 V8_USE_ADDRESS_SANITIZER 门) ══
// CRTP: class UnmarkedObjectsPoisoner : public HeapVisitor<UnmarkedObjectsPoisoner>
// 遍历语义由 HeapVisitor (a 半区 heap-visitor.h 族) 提供, 本类只落 Visit 钩子。

/// v8: 原版整类 #ifdef V8_USE_ADDRESS_SANITIZER — Rust 侧 cfg 门对应
#[cfg(sanitize = "address")]
pub struct UnmarkedObjectsPoisoner;

#[cfg(sanitize = "address")]
impl UnmarkedObjectsPoisoner {
    /// 原版 object-poisoner.h:24-30 — 游标钩子: free 或已标 → 跳过 (true); 否则 payload 打毒
    /// v8: ASAN_POISON_MEMORY_REGION(start, ObjectView<>::Size()) — 非大对象默认 AccessMode::kNonAtomic
    pub fn visit_heap_object_header(header: &mut HeapObjectHeader) -> bool {
        if header.is_free() || header.is_marked() {
            return true;
        }
        let view = ObjectView::new(header);
        // v8: 原版 object-poisoner.h:27-28 ASAN 打毒调用 — Rust 侧待接 sanitizer ABI
        unsafe { asan_poison_memory_region(header.object_start(), view.size()) };
        true
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "object_poisoner.rs.skel", body);
console.log(`OK object-poisoner.h.js: 锚 ${n} 条 (守护2+CRTP1+方法1+早退1+打毒2); ASAN cfg 门 1; 产物 ${out}`);
