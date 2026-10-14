#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/process-heap.h.js — v8/src/heap/cppgc-internal/{process-heap.h,process-heap.cc} → v8-heap-cppgc/src/process_heap.rs.skel
// 职责: ProcessGlobalLock 进程级互斥 (Perfetto 跟踪双 Reason) + HeapRegistry 堆注册表 (指针定位所属堆)
// 规则:
//   ① ProcessGlobalLock::Reason 枚举 kForGC/kForCrossThreadHandleCreation; Lock/Unlock 模板; PERFETTO 门内 DEBUG 子门只盖跨线程臂
//   ② HeapRegistry: Register (查重 push) / Unregister (DCHECK page_backend 活) / TryFromManagedPointer (线性扫 page_backend.Lookup)
//   ③ Subscription RAII: 构造注册析构反注册; 2 把 LazyMutex (process / registry)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/process-heap.h";
const REL_CC = "src/heap/cppgc-internal/process-heap.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class ProcessGlobalLock final {", "进程锁类");
need(src, "enum class Reason {", "Reason 枚举");
need(src, "kForGC,", "GC reason");
need(src, "kForCrossThreadHandleCreation,", "跨线程句柄 reason");
need(src, "static v8::base::LazyMutex process_mutex_;", "进程互斥成员");
need(src, "V8_INLINE static void AssertHeld()", "持锁断言");
need(src, "class V8_EXPORT_PRIVATE HeapRegistry final {", "注册表类");
need(src, "using Storage = std::vector<HeapBase*>;", "存储别名");
need(src, "class Subscription final {", "RAII 订阅");
need(src, "static HeapBase* TryFromManagedPointer(const void* needle);", "指针定位声明");
need(src, "static const Storage& GetRegisteredHeapsForTesting();", "测试只读视图");
need(cc, "v8::base::LazyMutex ProcessGlobalLock::process_mutex_ = LAZY_MUTEX_INITIALIZER;", "进程锁 ODR");
need(cc, "v8::base::LazyMutex g_heap_registry_mutex = LAZY_MUTEX_INITIALIZER;", "注册表互斥 ODR");
need(cc, "DCHECK_EQ(storage.end(), std::find(storage.begin(), storage.end(), &heap));", "注册查重");
need(cc, "DCHECK_NOT_NULL(heap.page_backend());", "反注册时页后端须活");
need(cc, "const auto pos = std::find(storage.begin(), storage.end(), &heap);", "反注册定位");
need(cc, "heap->page_backend()->Lookup(reinterpret_cast<ConstAddress>(needle));", "指针定位查找链");
need(cc, "if (address) return heap;", "命中即返回");
need(src, "HeapRegistry::RegisterHeap(heap_);", "RAII 注册");
need(src, "HeapRegistry::UnregisterHeap(heap_);", "RAII 反注册");

const body = `${banner("src/heap/cppgc-internal/process-heap.h.js", REL_H, "ProcessGlobalLock 进程互斥 + HeapRegistry 堆注册表 (cc 实现一并落地)")}
// ══ ProcessGlobalLock (原版 process-heap.h:18-70, ODR cc:19) ══

/// 原版 process-heap.h:20-23
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    ForGc,
    ForCrossThreadHandleCreation,
}

/// v8: 原版 Lock/Unlock 均模板 <Reason reason>; PERFETTO 门内 kForCrossThreadHandleCreation 臂再套 DEBUG 门
pub struct ProcessGlobalLock;

impl ProcessGlobalLock {
    pub fn lock<REASON: ReasonTag>() {
        PROCESS_MUTEX.get().lock();
        // v8: V8_USE_PERFETTO — GC 臂 TRACE_EVENT_BEGIN("AcquiredForGC"); 跨线程臂仅 DEBUG 追踪
    }
    pub fn unlock<REASON: ReasonTag>() {
        // v8: V8_USE_PERFETTO — 对应 TRACE_EVENT_END
        PROCESS_MUTEX.get().unlock();
    }
    /// 原版 process-heap.h:66
    pub fn assert_held() {
        PROCESS_MUTEX.get().assert_held();
    }
}

// ══ HeapRegistry (原版 process-heap.h:72-93, 实现 cc:34-71) ══
// 全进程堆表; 指针 → 所属堆定位靠 PageBackend::Lookup 线性扫。

pub struct HeapRegistry;
type Storage = Vec<*mut HeapBase>;

impl HeapRegistry {
    /// 原版 cc:34-40 — mutex + 查重 push
    pub fn register_heap(heap: &mut HeapBase) {
        let _g = HEAP_REGISTRY_MUTEX.get().guard();
        let storage = heap_registry_storage();
        debug_assert!(!storage.contains(&(heap as *mut _)));
        storage.push(heap);
    }

    /// 原版 cc:43-54 — 反注册时 DCHECK page_backend 仍活 (TryFromManagedPointer 依赖)
    pub fn unregister_heap(heap: &mut HeapBase) {
        let _g = HEAP_REGISTRY_MUTEX.get().guard();
        debug_assert!(!heap.page_backend().is_null());
        let storage = heap_registry_storage();
        let pos = storage.iter().position(|h| *h == heap as *mut _);
        debug_assert!(pos.is_some());
        storage.remove(pos.unwrap());
    }

    /// 原版 cc:57-66 — 线性扫各堆 page_backend().Lookup(needle), 命中即返
    pub fn try_from_managed_pointer(needle: *const u8) -> *mut HeapBase {
        let _g = HEAP_REGISTRY_MUTEX.get().guard();
        for heap in heap_registry_storage() {
            let address = unsafe { &**heap }.page_backend().lookup(needle);
            if !address.is_null() {
                return *heap;
            }
        }
        std::ptr::null_mut()
    }

    /// 原版 cc:69-71 — 不取锁, 仅测试用
    pub fn get_registered_heaps_for_testing() -> &'static Storage {
        heap_registry_storage()
    }
}

// ══ Subscription RAII (原版 process-heap.h:76-83, 内联实现 :95-101) ══

/// 构造注册 / 析构反注册
pub struct Subscription {
    heap: *mut HeapBase,
}

impl Subscription {
    pub fn new(heap: &mut HeapBase) -> Self {
        HeapRegistry::register_heap(heap);
        Self { heap }
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        HeapRegistry::unregister_heap(unsafe { &mut *self.heap });
    }
}
`;
const out = writeSrc("v8-heap-cppgc", "process_heap.rs.skel", body);
console.log(`OK process-heap.h.js: 锚 ${n} 条 (h10+cc10); 2 类 + RAII; 产物 ${out}`);
