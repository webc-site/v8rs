#!/usr/bin/env bun
// tool/src/heap/cppgc-js/unified-heap-marking-verifier.h.js — v8/src/heap/cppgc-js/{unified-heap-marking-verifier.h,unified-heap-marking-verifier.cc} → v8-heap-cppgc/src/unified_heap_marking_verifier.rs.skel
// 职责: 统一堆标记验证 — UnifiedHeapVerificationState (TracedReference 在用校验 FATAL) + 4 钩子验证访问者
// 规则:
//   ① VerifyMarkedTracedReference: 空槽早退 → IsValidInUseNode 不满足 FATAL (带 parent 名/栈 提示)
//   ② UnifiedHeapVerificationVisitor: Visit/VisitWeak/VisitWeakContainer/Visit(TracedRef) 4 final 钩子全走 VerifyMarked
//   ③ Verifier 构造: MarkingVerifierBase(heap, type, state, visitor 独占)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-js/unified-heap-marking-verifier.h";
const REL_CC = "src/heap/cppgc-js/unified-heap-marking-verifier.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class UnifiedHeapVerificationState : public cppgc::internal::VerificationState {", "验证状态类");
need(src, "void VerifyMarkedTracedReference(const TracedReferenceBase& ref) const;", "引用验证声明");
need(src, "class V8_EXPORT_PRIVATE UnifiedHeapMarkingVerifier final", "验证器类");
need(src, ": public cppgc::internal::MarkingVerifierBase {", "验证器基类");
need(src, "UnifiedHeapVerificationState state_;", "状态成员");
need(cc, "class UnifiedHeapVerificationVisitor final : public JSVisitor {", "验证访问者");
need(cc, "cppgc::internal::VisitorFactory::CreateKey()),", "工厂钥匙构造");
need(cc, "state_.VerifyMarked(desc.base_object_payload);", "VerifyMarked 调用");
need(cc, "// Weak objects should have been cleared at this point.", "弱清空注释");
need(cc, "if (!object) return;", "空容器退");
need(cc, "state_.VerifyMarked(weak_desc.base_object_payload);", "弱容器验证");
need(cc, "void Visit(const TracedReferenceBase& ref) final {", "引用钩子");
need(cc, "if (!TracedHandles::IsValidInUseNode(traced_handle_location)) {", "在用节点判定");
need(cc, '"MarkingVerifier: Encountered unmarked TracedReference.\\n"', "FATAL 文案");
need(cc, "kUseClassNameIfSupported)", "未命名对象类名策略");
need(cc, ': "Stack",', "栈兜底名");
need(cc, "std::make_unique<UnifiedHeapVerificationVisitor>(state_)) {}", "visitor 独占注入");

// VerifyMarked 调用计数: Visit 1 + VisitWeak 1 + VisitWeakContainer 1 = 3
const vm = (cc.match(/state_\.VerifyMarked\(/g) || []).length;
if (vm !== 3) throw new Error(`VerifyMarked 调用计数: got=${vm} want=3`);
n++;

const body = `${banner("src/heap/cppgc-js/unified-heap-marking-verifier.h.js", REL_H, "统一堆标记验证 — TracedReference 在用校验 + 4 钩子验证访问者 (cc 实现一并落地)")}
// ══ UnifiedHeapVerificationState (原版 unified-heap-marking-verifier.h:14-17, 实现 cc:60-89) ══

pub struct UnifiedHeapVerificationState {
    /// VerificationState 基: parent_ 指针等
    pub parent: *const HeapObjectHeader,
}

impl UnifiedHeapVerificationState {
    /// 原版 cc:60-89 — 空槽早退; 节点须仍标记在用, 否则 FATAL (含父对象名或 Stack 提示)
    pub fn verify_marked_traced_reference(&self, ref: &TracedReferenceBase) {
        // 分离模式下遇非空 TracedReferenceBase 空指针解引崩溃 (原版注释)
        let traced_handle_location = BasicTracedReferenceExtractor::get_object_slot_for_marking(ref);
        // ephemeron 追踪可达此处, 无早退口 — 槽可空
        if traced_handle_location.is_null() {
            return;
        }
        // 验证跑在未标节点释放后 — 该 TracedReference 节点应仍标记在用
        if !TracedHandles::is_valid_in_use_node(traced_handle_location) {
            let parent_name: &str = if !self.parent.is_null() {
                unsafe { &*self.parent }.get_name(HeapObjectNameForUnnamedObject::UseClassNameIfSupported).value
            } else {
                "Stack"
            };
            v8_fatal!(
                "MarkingVerifier: Encountered unmarked TracedReference.\\n#\\n# Hint:\\n#   {} ({:?})\\n#     \\\\-> TracedReference ({:?})",
                parent_name, self.parent, ref
            );
        }
    }
}

// ══ UnifiedHeapVerificationVisitor (原版 cc:20-56, 匿名 ns) ══
/// 4 final 钩子, 全走 VerifyMarked (强/弱/弱容器/引用)

pub struct UnifiedHeapVerificationVisitor {
    /// JSVisitor 基 (VisitorFactory 钥匙构造)
    pub state: UnifiedHeapVerificationState,
}

impl UnifiedHeapVerificationVisitor {
    /// 原版 cc:26-28 — 强引用: 载体须已标
    pub fn visit(&self, _object: *const u8, desc: TraceDescriptor) {
        self.state.verify_marked(desc.base_object_payload);
    }
    /// 原版 cc:30-36 — 弱引用此时应已清空 → 经弱引用可达的对象必活
    pub fn visit_weak(&self, _object: *const u8, desc: TraceDescriptor, _cb: WeakCallback, _p: *const u8) {
        self.state.verify_marked(desc.base_object_payload);
    }
    /// 原版 cc:38-48 — 弱容器: 内容由页迭代强处理, 此处只验容器自身已标; 空容器退
    pub fn visit_weak_container(&self, object: *const u8, _desc: TraceDescriptor, weak_desc: TraceDescriptor, _cb: WeakCallback, _p: *const u8) {
        if object.is_null() {
            return;
        }
        self.state.verify_marked(weak_desc.base_object_payload);
    }
    /// 原版 cc:50-52 — TracedReference 专项
    pub fn visit_traced_reference(&self, ref: &TracedReferenceBase) {
        self.state.verify_marked_traced_reference(ref);
    }
}

// ══ UnifiedHeapMarkingVerifier (原版 .h:19-28, 构造 cc:91-96) ══

pub struct UnifiedHeapMarkingVerifier {
    /// MarkingVerifierBase(heap_base, collection_type, state_, visitor)
    pub base: cppgc_internal::MarkingVerifierBase,
    pub state: UnifiedHeapVerificationState,
}

impl UnifiedHeapMarkingVerifier {
    /// 原版 cc:91-96 — 注入共享 state 与独占 visitor
    pub fn new(heap_base: &mut cppgc_internal::HeapBase, collection_type: CollectionType) -> Self {
        let state = UnifiedHeapVerificationState { parent: std::ptr::null() };
        let visitor = Box::new(UnifiedHeapVerificationVisitor { state: /* 共享 */ unsafe { std::ptr::read(&state) } });
        Self { base: cppgc_internal::MarkingVerifierBase::new(heap_base, collection_type, &state, visitor), state }
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "unified_heap_marking_verifier.rs.skel", body);
console.log(`OK unified-heap-marking-verifier.h.js: 锚 ${n} 条 (h4+cc11+调用计数1); 4 钩子; 产物 ${out}`);
