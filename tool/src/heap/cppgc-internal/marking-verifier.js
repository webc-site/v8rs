#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/marking-verifier.js — v8/src/heap/cppgc-internal/marking-verifier.h + .cc → v8-heap-cppgc/src/marking_verifier.rs.skel
// 标记验证器: 未标即 FATAL + 构造中对象双集合 (heap/stack) + 差异报告四路
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-internal/marking-verifier.h";
const REL_CC = "src/heap/cppgc-internal/marking-verifier.cc";
const src = readV8(REL);
const srcCc = readV8(REL_CC);

let n = 0;
const need = (s, needle, label, file) => {
  if (!s.includes(needle)) throw new Error(`${file} 缺锚 ${label}: ${needle}`);
  n++;
};

need(src, "class VerificationState {", "VerificationState 声明");
need(src, "void VerifyMarked(const void*) const;", "VerifyMarked 声明");
need(src, "void SetCurrentParent(const HeapObjectHeader* header) { parent_ = header; }", "SetCurrentParent");
need(src, "bool IsParentOnStack() const { return !parent_; }", "IsParentOnStack");
need(src, "class V8_EXPORT_PRIVATE MarkingVerifierBase", "MarkingVerifierBase 声明");
need(src, "public ConservativeTracingVisitor,\n      public heap::base::StackVisitor {", "双继承");
need(src, "void Run(StackState, std::optional<size_t>);", "Run 声明");
for (const m of ["void VisitInConstructionConservatively(HeapObjectHeader&,", "void VisitPointer(const void*) final;",
  "bool VisitNormalPage(NormalPage&);", "bool VisitLargePage(LargePage&);", "bool VisitHeapObjectHeader(HeapObjectHeader&);"]) need(src, m, "Base 钩子");
for (const m of ["void ReportDifferences(size_t) const;", "void ReportNormalPage(const NormalPage&, size_t) const;",
  "void ReportLargePage(const LargePage&, size_t) const;", "void ReportHeapObjectHeader(const HeapObjectHeader&) const;"]) need(src, m, "Report 四路");
need(src, "absl::flat_hash_set<const HeapObjectHeader*> in_construction_objects_heap_;", "构造中堆集合");
need(src, "absl::flat_hash_set<const HeapObjectHeader*> in_construction_objects_stack_;", "构造中栈集合");
need(src, "size_t verifier_found_marked_bytes_ = 0;", "验证侧标记字节");
need(src, "bool verifier_found_marked_bytes_are_exact_ = true;", "精确标志");
need(src, "class V8_EXPORT_PRIVATE MarkingVerifier final : public MarkingVerifierBase {", "MarkingVerifier 声明");
// .cc
need(srcCc, 'FATAL(\n        "MarkingVerifier: Encountered unmarked object.\\n"', "未标记致命");
need(srcCc, 'parent_ ? parent_->ObjectStart() : nullptr,', "父对象或栈");
need(srcCc, "HeapObjectNameForUnnamedObject::kUseClassNameIfSupported)", "报告命名选项");
need(srcCc, "      collection_type_(collection_type) {}", "收集类型初值");
need(srcCc, "collection_type_ == CollectionType::kMinor) {", "次要收集分支");

const body = `${banner("src/heap/cppgc-internal/marking-verifier.js", `${REL} + ${REL_CC}`, "标记闭包验证: 遍历全堆核对标记位")}
#![allow(dead_code)]

/// v8: src/heap/cppgc-internal/marking-verifier.h VerificationState
pub struct VerificationState {
    /// v8: parent_ — null 表示父在栈上
    parent: *const HeapObjectHeader,
}

impl VerificationState {
    /// v8: src/heap/cppgc-internal/marking-verifier.cc VerificationState::VerifyMarked
    /// 未标记 → FATAL (父名/址 + 子名/址); 命名选项 kUseClassNameIfSupported。
    pub fn verify_marked(&self, base_object_payload: *const core::ffi::c_void) {
        let child = HeapObjectHeader::from_object(base_object_payload);
        if !child.is_marked() {
            panic!("MarkingVerifier: Encountered unmarked object. parent={:?} child={:?}", self.parent, child);
        }
    }
    /// v8: SetCurrentParent
    pub fn set_current_parent(&mut self, header: *const HeapObjectHeader) { self.parent = header; }
    /// v8: IsParentOnStack — 无父即栈上
    pub fn is_parent_on_stack(&self) -> bool { self.parent.is_null() }
}

/// v8: src/heap/cppgc-internal/marking-verifier.h MarkingVerifierBase
pub struct MarkingVerifierBase {
    /// v8: verification_state_
    verification_state: *mut VerificationState,
    /// v8: visitor_
    visitor: Box<dyn Visitor>,
    /// v8: in_construction_objects_heap_
    in_construction_objects_heap: HashSet<*const HeapObjectHeader>,
    /// v8: in_construction_objects_stack_
    in_construction_objects_stack: HashSet<*const HeapObjectHeader>,
    /// v8: verifier_found_marked_bytes_
    verifier_found_marked_bytes: usize,
    /// v8: verifier_found_marked_bytes_are_exact_
    verifier_found_marked_bytes_are_exact: bool,
    /// v8: collection_type_
    collection_type: CollectionType,
    /// v8: verifier_found_marked_bytes_in_pages_
    verifier_found_marked_bytes_in_pages: usize,
}

impl MarkingVerifierBase {
    /// v8: src/heap/cppgc-internal/marking-verifier.cc MarkingVerifierBase::Run
    /// 双集合切换 (栈扫描期 vs 堆遍历期) + Traverse + 差异报告。
    pub fn run(&mut self, _stack_state: StackState, _marked_bytes: Option<usize>) { todo!("双阶段验证主流程") }

    /// v8: VisitInConstructionConservatively / VisitPointer — 保守访问记入栈侧集合
    pub fn visit_in_construction_conservatively(&mut self, _header: &mut HeapObjectHeader, _cb: TraceConservativelyCallback) { todo!("栈侧构造中登记") }
    pub fn visit_pointer(&mut self, _address: *const core::ffi::c_void) { todo!("保守指针委托") }

    /// v8: VisitNormalPage / VisitLargePage — 页级标记字节累计
    pub fn visit_normal_page(&mut self, _page: &mut NormalPage) -> bool { todo!("页标记字节累计") }
    pub fn visit_large_page(&mut self, _page: &mut LargePage) -> bool { todo!("大页标记字节累计") }

    /// v8: VisitHeapObjectHeader — VerifyMarked + 构造中对象核对
    pub fn visit_heap_object_header(&mut self, _header: &mut HeapObjectHeader) -> bool { todo!("逐头核对") }

    /// v8: ReportDifferences / ReportNormalPage / ReportLargePage / ReportHeapObjectHeader
    fn report_differences(&self, _found: usize) { todo!("验证 vs 实际差异打印") }
}

/// v8: src/heap/cppgc-internal/marking-verifier.h MarkingVerifier
pub struct MarkingVerifier {
    /// v8: MarkingVerifierBase 基类子对象
    pub base: MarkingVerifierBase,
    /// v8: state_
    state: VerificationState,
}
`;

writeSrc("v8-heap-cppgc", "marking_verifier.rs.skel", body);
console.log(`marking-verifier.js: 对数 ${n} 项 (.h 17 + .cc 4) → v8-heap-cppgc/src/marking_verifier.rs.skel`);
