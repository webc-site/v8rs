#!/usr/bin/env bun
// tool/src/heap/cppgc-js/cross-heap-remembered-set.h.js — v8/src/heap/cppgc-js/{cross-heap-remembered-set.h,cross-heap-remembered-set.cc} → v8-heap-cppgc/src/cross_heap_remembered_set.rs.skel
// 职责: CrossHeapRememberedSet — V8→Oilpan 跨堆引用记忆 (全局句柄向量承载宿主对象)
// 规则:
//   ① RememberReferenceIfNeeded: 内地址页回溯, 无页快退 (on-cage 过滤 TODO), 目标非 young 快退, 否则建全局句柄
//   ② Reset: 逐句柄 Destroy 后 clear+shrink; ③ Visit: 模板逐回调; ④ IsEmpty: 向量空
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-js/cross-heap-remembered-set.h";
const REL_CC = "src/heap/cppgc-js/cross-heap-remembered-set.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE CrossHeapRememberedSet final {", "类声明");
need(src, "void RememberReferenceIfNeeded(Isolate& isolate,", "记忆声明");
need(src, "Tagged<CppHeapPointerWrapperObjectT> host_obj,", "宿主参数");
need(src, "void Reset(Isolate& isolate);", "重置声明");
need(src, "template <typename F>\n  void Visit(Isolate&, F);", "访问声明");
need(src, "bool IsEmpty() const { return remembered_v8_to_cppgc_references_.empty(); }", "空判定内联");
need(src, "std::vector<IndirectHandle<CppHeapPointerWrapperObjectT>>", "全局句柄向量成员");
need(cc, "cppgc::internal::BasePage::FromInnerAddress(&heap_base_, cppgc_object);", "cc 页回溯");
need(cc, "// TODO(v8:13475): Better filter with on-cage check.", "cage 过滤 TODO");
need(cc, "if (!page) return;", "无页快退");
need(cc, "if (!value_hoh.IsYoung()) return;", "非年轻快退");
need(cc, "isolate.global_handles()->Create(host_obj));", "建全局句柄");
need(cc, "isolate.global_handles()->Destroy(h.location());", "毁全局句柄");
need(cc, "remembered_v8_to_cppgc_references_.shrink_to_fit();", "收缩");

const body = `${banner("src/heap/cppgc-js/cross-heap-remembered-set.h.js", REL_H, "V8→Oilpan 跨堆引用记忆集 (全局句柄承载宿主) (cc 实现一并落地)")}
// ══ CrossHeapRememberedSet (原版 cross-heap-remembered-set.h:22-53, 实现 cc:13-34) ══
// v8::internal 命名空间 — 记 V8 侧对象 (有指向 cppgc 堆出引用者), 句柄为全局。

pub struct CrossHeapRememberedSet {
    heap_base: *mut cppgc_internal::HeapBase,
    /// 记忆的 V8 宿主对象 — 全局句柄
    remembered_v8_to_cppgc_references: Vec<IndirectHandle<CppHeapPointerWrapperObjectT>>,
}

impl CrossHeapRememberedSet {
    pub fn new(heap_base: &mut cppgc_internal::HeapBase) -> Self {
        Self { heap_base, remembered_v8_to_cppgc_references: Vec::new() }
    }

    /// 原版 cc:13-26 — 三段: 页回溯 (无页退) → 目标 young 判定 (非 young 退) → 建句柄记忆
    pub fn remember_reference_if_needed(
        &mut self,
        isolate: &mut Isolate,
        host_obj: Tagged<CppHeapPointerWrapperObjectT>,
        cppgc_object: *mut u8,
    ) {
        debug_assert!(!cppgc_object.is_null());
        // cage 内指针必指向有效未释放 cppgc 对象
        let page = cppgc_internal::BasePage::from_inner_address(unsafe { &mut *self.heap_base }, cppgc_object);
        // v8: TODO(v8:13475) — on-cage 检查做更好过滤
        if page.is_null() {
            return;
        }
        let value_hoh = unsafe { &*page }.object_header_from_inner_address(cppgc_object);
        if !value_hoh.is_young() {
            return;
        }
        self.remembered_v8_to_cppgc_references.push(isolate.global_handles().create(host_obj));
    }

    /// 原版 cc:28-34 — 逐句柄 Destroy, clear + shrink
    pub fn reset(&mut self, isolate: &mut Isolate) {
        for h in &self.remembered_v8_to_cppgc_references {
            isolate.global_handles().destroy(h.location());
        }
        self.remembered_v8_to_cppgc_references.clear();
        self.remembered_v8_to_cppgc_references.shrink_to_fit();
    }

    /// 原版 h:48-53 — 逐对象回调
    pub fn visit<F: FnMut(Tagged<CppHeapPointerWrapperObjectT>)>(&self, _isolate: &Isolate, mut f: F) {
        for obj in &self.remembered_v8_to_cppgc_references {
            f(*obj);
        }
    }

    /// 原版 h:38
    pub fn is_empty(&self) -> bool {
        self.remembered_v8_to_cppgc_references.is_empty()
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "cross_heap_remembered_set.rs.skel", body);
console.log(`OK cross-heap-remembered-set.h.js: 锚 ${n} 条 (h7+cc8); 3 快退段 + 句柄生命周期; 产物 ${out}`);
