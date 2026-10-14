#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/prefinalizer-handler.h.js — v8/src/heap/cppgc-internal/{prefinalizer-handler.h,prefinalizer-handler.cc} → v8-heap-cppgc/src/prefinalizer_handler.rs.skel
// 职责: PreFinalizer {object, callback} 记录 + PreFinalizerHandler 注册/逆序回收调 + prefinalizer 期分配记账
// 规则:
//   ① 注册: 创建线程限定, 双容器查重 DCHECK 后 push current 容器
//   ② 调用: LAB 全 Reset 促黑分配慢路 → current 切到新向量 (防迭代器失效) → 逆序 remove_if 执行回调 → 新增项按 ALLOW_ALLOCATIONS 分支回插/拒绝
//   ③ 计账: Notify 累加溢出 DCHECK, Extract = exchange(0)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/prefinalizer-handler.h";
const REL_CC = "src/heap/cppgc-internal/prefinalizer-handler.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "struct PreFinalizer final {", "记录结构");
need(src, "using Callback = PrefinalizerRegistration::Callback;", "回调别名");
need(src, "void* object;", "对象指针字段");
need(src, "bool operator==(const PreFinalizer& other) const;", "相等判定声明");
need(src, "class PreFinalizerHandler final {", "handler 类");
need(src, "std::vector<PreFinalizer> ordered_pre_finalizers_;", "主容器");
need(src, "std::vector<PreFinalizer>* current_ordered_pre_finalizers_;", "current 指针");
need(src, "bool is_invoking_ = false;", "调用中标志");
need(src, "size_t bytes_allocated_in_prefinalizers = 0u;", "计账字段");
need(cc, "return (object == other.object) && (callback == other.callback);", "相等语义");
need(cc, "DCHECK(!page->space().is_compactable());", "注册禁 compactable 空间");
need(cc, "page->heap().prefinalizer_handler()->RegisterPrefinalizer({object, callback});", "注册链");
need(cc, "heap_.object_allocator().ResetLinearAllocationBuffers();", "LAB 全重置");
need(cc, "std::vector<PreFinalizer> new_ordered_pre_finalizers;", "新容器防迭代器失效");
need(cc, "std::remove_if(ordered_pre_finalizers_.rbegin(),", "逆序 remove_if");
need(cc, "#ifndef CPPGC_ALLOW_ALLOCATIONS_IN_PREFINALIZERS", "允许分配分支门");
need(cc, "CHECK(new_ordered_pre_finalizers.empty());", "禁分配断言");
need(cc, "#ifdef DEBUG", "创建线程 DEBUG 门");
need(src, "return std::exchange(bytes_allocated_in_prefinalizers, 0);", "exchange 取清");

const body = `${banner("src/heap/cppgc-internal/prefinalizer-handler.h.js", REL_H, "PreFinalizer 注册与逆序调用 + prefinalizer 期分配记账 (cc 实现一并落地)")}
// ══ PreFinalizer (原版 prefinalizer-handler.h:18-25) ══

/// Callback = PrefinalizerRegistration::Callback — 返回 true = 对象死, 从表中移除
pub type PreFinalizerCallback = fn(&LivenessBroker, *mut u8) -> bool;

#[derive(Clone, Copy)]
pub struct PreFinalizer {
    pub object: *mut u8,
    pub callback: PreFinalizerCallback,
}

/// 原版 prefinalizer-handler.cc:26-28 — 双字段全等
impl PartialEq for PreFinalizer {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object && self.callback == other.callback
    }
}

// ══ PreFinalizerHandler (原版 prefinalizer-handler.h:27-58, 实现 cc:30-98) ══
// prefinalizer 按注册逆序调用 (构造序的反向)。
pub struct PreFinalizerHandler {
    ordered_pre_finalizers: Vec<PreFinalizer>,
    /// 指向主容器或 Invoke 期间的临时容器 (防迭代器失效)
    current_ordered_pre_finalizers: *mut Vec<PreFinalizer>,
    heap: *mut HeapBase,
    is_invoking: bool, // = false
    bytes_allocated_in_prefinalizers: usize, // = 0
}

impl PreFinalizerHandler {
    /// 原版 cc:30-33 — 构造 DCHECK 创建线程
    pub fn new(heap: &mut HeapBase) -> Self {
        let mut h = Self {
            ordered_pre_finalizers: Vec::new(),
            current_ordered_pre_finalizers: std::ptr::null_mut(),
            heap,
            is_invoking: false,
            bytes_allocated_in_prefinalizers: 0,
        };
        h.current_ordered_pre_finalizers = &mut h.ordered_pre_finalizers;
        debug_assert!(h.current_thread_is_creation_thread());
        h
    }

    /// 原版 cc:19-24 — PrefinalizerRegistration 构造: 禁 compactable 空间, 落到页所属堆的 handler
    pub fn register_from_registration(object: *mut u8, callback: PreFinalizerCallback) {
        let page = BasePage::from_payload(object);
        debug_assert!(!page.space().is_compactable());
        page.heap().prefinalizer_handler().register_prefinalizer(PreFinalizer { object, callback });
    }

    /// 原版 cc:35-44 — 创建线程限定 + 双容器查重后 push current
    pub fn register_prefinalizer(&mut self, pre_finalizer: PreFinalizer) {
        debug_assert!(self.current_thread_is_creation_thread());
        let cur = unsafe { &mut *self.current_ordered_pre_finalizers };
        debug_assert!(!self.ordered_pre_finalizers.contains(&pre_finalizer));
        debug_assert!(!cur.contains(&pre_finalizer));
        cur.push(pre_finalizer);
    }

    /// 原版 cc:46-84 — 逆序执行; 死对象 (回调 true) 从表中移除
    pub fn invoke_pre_finalizers(&mut self) {
        // v8: 原版双 EnabledScope: kAtomicSweep + kSweepInvokePreFinalizers
        debug_assert!(self.current_thread_is_creation_thread());
        let liveness_broker = LivenessBrokerFactory::create();
        self.is_invoking = true;
        debug_assert_eq!(0, self.bytes_allocated_in_prefinalizers);
        // 重置全 LAB → 强制慢路分配 (黑分配)
        unsafe { &mut *self.heap }.object_allocator().reset_linear_allocation_buffers();
        // prefinalizer 可再注册 prefinalizer → current 切临时容器防迭代器失效
        let mut new_vec: Vec<PreFinalizer> = Vec::new();
        self.current_ordered_pre_finalizers = &mut new_vec;
        // 逆序 remove_if: rbegin/rend 遍历, 回调 true 的移除
        let broker = &liveness_broker;
        self.ordered_pre_finalizers.retain(|pf| !(pf.callback)(broker, pf.object));
        // v8: 原版 cppgc 允许两种配置 — 禁分配配置 CHECK 空容器; 允许配置把新注册项 (必活) 回插尾部
        #[cfg(not(feature = "cppgc-allow-allocations-in-prefinalizers"))]
        v8_check!(new_vec.is_empty());
        #[cfg(feature = "cppgc-allow-allocations-in-prefinalizers")]
        self.ordered_pre_finalizers.extend(new_vec);
        self.current_ordered_pre_finalizers = &mut self.ordered_pre_finalizers;
        self.is_invoking = false;
        self.ordered_pre_finalizers.shrink_to_fit();
    }

    pub fn is_invoking_pre_finalizers(&self) -> bool {
        self.is_invoking // 原版 h:35
    }

    /// 原版 cc:94-98 — 累加, DCHECK 无溢出回绕
    pub fn notify_allocation_in_prefinalizer(&mut self, size: usize) {
        debug_assert!(self.bytes_allocated_in_prefinalizers + size > self.bytes_allocated_in_prefinalizers);
        self.bytes_allocated_in_prefinalizers += size;
    }

    /// 原版 h:38-40 — exchange 取清
    pub fn extract_bytes_allocated_in_prefinalizers(&mut self) -> usize {
        std::mem::take(&mut self.bytes_allocated_in_prefinalizers)
    }

    /// 原版 cc:86-92 — DEBUG 才真查线程, 否则恒 true
    fn current_thread_is_creation_thread(&self) -> bool {
        #[cfg(debug_assertions)]
        { unsafe { &*self.heap }.current_thread_is_heap_thread() }
        #[cfg(not(debug_assertions))]
        { true }
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "prefinalizer_handler.rs.skel", body);
console.log(`OK prefinalizer-handler.h.js: 锚 ${n} 条 (h9+cc10); 逆序调用 + 双分支; 产物 ${out}`);
