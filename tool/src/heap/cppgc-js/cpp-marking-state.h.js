#!/usr/bin/env bun
// tool/src/heap/cppgc-js/cpp-marking-state.h.js — v8/src/heap/cppgc-js/cpp-marking-state.h → v8-heap-cppgc/src/cpp_marking_state.rs.skel
// 职责: CppMarkingState — 主线程借用 / 并发独占双构造的标记状态包装
// 规则:
//   ① 构造 1: 借用主线程 MarkingStateBase (owned=nullptr); 构造 2: 接管 unique_ptr 并借用之
//   ② Publish/MarkAndPush/IsLocalEmpty 三转发; MarkAndPush 经 HeapObjectHeader::FromObject
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL = "src/heap/cppgc-js/cpp-marking-state.h";
const src = readV8(REL);
let n = 0;
const need = (needle, label) => {
  if (!src.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need("class CppMarkingState final {", "类声明");
need("cppgc::internal::MarkingStateBase& main_thread_marking_state)", "借用构造参数");
need(": owned_marking_state_(nullptr),", "借用构造 owned 空");
need("explicit CppMarkingState(std::unique_ptr<cppgc::internal::MarkingStateBase>", "独占构造");
need("marking_state_(*owned_marking_state_) {}", "独占构造借自 owned");
need("void Publish() { marking_state_.Publish(); }", "Publish 内联");
need("V8_INLINE void MarkAndPush(void* instance) {", "MarkAndPush 内联");
need("cppgc::internal::HeapObjectHeader::FromObject(instance));", "FromObject 链");
need("return marking_state_.marking_worklist().IsLocalEmpty();", "IsLocalEmpty 链");
need("std::unique_ptr<cppgc::internal::MarkingStateBase> owned_marking_state_;", "owned 成员");
need("cppgc::internal::MarkingStateBase& marking_state_;", "借用成员");

const body = `${banner("src/heap/cppgc-js/cpp-marking-state.h.js", REL, "CppMarkingState — 主线程借用/并发独占双源标记状态包装")}
// ══ CppMarkingState (原版 cpp-marking-state.h:21-49) ══
// 双构造一成员对: 借用 (主线程, owned 空) / 独占 (并发, owned 持有后借用)。

pub struct CppMarkingState {
    /// 并发标记态独占所有权; 主线程构造时为 None
    owned_marking_state: Option<Box<cppgc_internal::MarkingStateBase>>,
    /// 恒有值 — 借用 owned 或外部主线程态
    marking_state: *mut cppgc_internal::MarkingStateBase,
}

impl CppMarkingState {
    /// 原版 :23-26 — 借用主线程态
    pub fn new_borrowed(main_thread_marking_state: &mut cppgc_internal::MarkingStateBase) -> Self {
        Self { owned_marking_state: None, marking_state: main_thread_marking_state }
    }

    /// 原版 :28-31 — 接管并发态, 借自 owned
    pub fn new_owned(concurrent_marking_state: Box<cppgc_internal::MarkingStateBase>) -> Self {
        Self { marking_state: concurrent_marking_state.as_ref() as *const _ as *mut _, owned_marking_state: Some(concurrent_marking_state) }
    }

    /// 原版 :35 — 发布本地工作表
    pub fn publish(&mut self) {
        unsafe { &mut *self.marking_state }.publish();
    }

    /// 原版 :37-40 — 对象指针 → 头 → MarkAndPush
    pub fn mark_and_push(&mut self, instance: *mut u8) {
        unsafe { &mut *self.marking_state }
            .mark_and_push(cppgc_internal::HeapObjectHeader::from_object(instance));
    }

    /// 原版 :42-44 — 本地工作表空判定
    pub fn is_local_empty(&self) -> bool {
        unsafe { &*self.marking_state }.marking_worklist().is_local_empty()
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "cpp_marking_state.rs.skel", body);
console.log(`OK cpp-marking-state.h.js: 锚 ${n} 条 (11); 双构造 + 3 转发; 产物 ${out}`);
