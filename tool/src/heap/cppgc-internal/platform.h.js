#!/usr/bin/env bun
// tool/src/heap/cppgc-internal/platform.h.js — v8/src/heap/cppgc-internal/{platform.h,platform.cc} → v8-heap-cppgc/src/platform.rs.skel
// 职责: FatalOutOfMemoryHandler 双段 OOM 报告 (自定义回调兜底 FATAL) + 进程级初始化/关停 + 全局单例取用
// 规则:
//   ① handler: custom_handler 存在则调回调且禁止返回, 否则 FATAL_WITH_LOC("Oilpan: Out of memory (%s)")
//   ② InitializeProcess: ASAN 64 位影子粒度对齐检查 → 分配器就绪 (lsan 覆盖) → GCInfoTable 初始化 → caged 初始化 → 全局指针落地; 重复初始化 CHECK 拒绝
//   ③ ShutdownProcess 置 null; IsInitialized 判全局分配器; 2 个全局单例 (OOM handler / page allocator)
import { banner, readV8, writeSrc } from "../../../lib.js";

const REL_H = "src/heap/cppgc-internal/platform.h";
const REL_CC = "src/heap/cppgc-internal/platform.cc";
const src = readV8(REL_H);
const cc = readV8(REL_CC);
let n = 0;
const need = (s, needle, label) => {
  if (!s.includes(needle)) throw new Error(`锚缺失 ${label}: ${needle}`);
  n++;
};

need(src, "class V8_EXPORT_PRIVATE FatalOutOfMemoryHandler final {", "handler 类");
need(src, "using Callback = void(const std::string&, SourceLocation, HeapBase*);", "回调签名");
need(src, "[[noreturn]] void operator()(", "调用运算符");
need(src, "void SetCustomHandler(Callback*);", "设置自定义");
need(src, "FatalOutOfMemoryHandler& GetGlobalOOMHandler();", "全局 OOM 单例声明");
need(src, "PageAllocator& GetGlobalPageAllocator();", "全局分配器单例声明");
need(cc, "if (custom_handler_) {", "自定义臂");
need(cc, 'FATAL("Custom out of memory handler should not have returned");', "回调禁返回");
need(cc, 'FATAL_WITH_LOC(loc, "Oilpan: Out of memory (%s)", reason.c_str());', "默认 FATAL");
need(cc, "CHECK_NOT_NULL(g_page_allocator);", "分配器非空检查");
need(cc, "CHECK(!internal::g_page_allocator);", "重复初始化拒绝");
need(cc, "internal::GlobalGCInfoTable::Initialize(allocator);", "GCInfo 表初始化");
need(cc, "internal::CagedHeap::InitializeIfNeeded(allocator, desired_heap_size);", "caged 初始化");
need(cc, "void ShutdownProcess() { internal::g_page_allocator = nullptr; }", "关停置空");
need(cc, "bool IsInitialized() { return internal::g_page_allocator != nullptr; }", "初始化判定");
need(cc, "static v8::base::LeakyObject<v8::base::LsanPageAllocator>", "lsan 覆盖层");
need(cc, "CHECK_EQ(0u, internal::kAllocationGranularity % poisoning_granularity);", "ASAN 粒度对齐");

const body = `${banner("src/heap/cppgc-internal/platform.h.js", REL_H, "FatalOutOfMemoryHandler + 进程级初始化/关停/全局单例 (cc 侧实现一并落地)")}
// ══ FatalOutOfMemoryHandler (原版 platform.h:18-38, 实现 platform.cc:51-62) ══
// 双段报告: 自定义回调优先, 回调不得返回; 否则 FATAL_WITH_LOC。

/// 原版 platform.h:20 — Callback = void(const std::string&, SourceLocation, HeapBase*)
pub type FatalOutOfMemoryCallback =
    fn(&str, SourceLocation, *mut HeapBase);

pub struct FatalOutOfMemoryHandler {
    heap: *mut HeapBase,                    // 原版 :36 = nullptr
    custom_handler: Option<FatalOutOfMemoryCallback>, // 原版 :37
}

impl FatalOutOfMemoryHandler {
    /// 原版 platform.cc:51-58 — [[noreturn]]; 自定义臂回调后仍 FATAL (回调不得返回)
    pub fn call(&self, reason: &str, loc: SourceLocation) -> ! {
        if let Some(cb) = self.custom_handler {
            cb(reason, loc, self.heap);
            v8_fatal("Custom out of memory handler should not have returned");
        }
        // 原版 platform.cc:57
        v8_fatal_with_loc(loc, "Oilpan: Out of memory ({})", reason)
    }

    /// 原版 platform.cc:60-62
    pub fn set_custom_handler(&mut self, callback: FatalOutOfMemoryCallback) {
        self.custom_handler = Some(callback);
    }
}

// ══ 全局单例 (原版 platform.h:41-44, 实现 platform.cc:64-72) ══

/// GetGlobalOOMHandler — 无堆绑定全局 OOM 句柄 (static 局部单例)
pub fn get_global_oom_handler() -> &'static mut FatalOutOfMemoryHandler {
    static OOM: FatalOutOfMemoryHandler = /* Leaky 静态 */ unsafe { std::mem::zeroed() };
    &mut OOM
}

/// GetGlobalPageAllocator — CHECK 非空后返回全局分配器
pub fn get_global_page_allocator() -> &'static mut PageAllocator {
    let p = G_PAGE_ALLOCATOR.load(std::sync::atomic::Ordering::Acquire);
    v8_check_not_null(p);
    unsafe { &mut *p.expect("page allocator 已初始化") }
}

// ══ 进程初始化 (原版 platform.cc:81-106) ══

pub fn is_initialized() -> bool {
    !G_PAGE_ALLOCATOR.load(std::sync::atomic::Ordering::Acquire).is_null()
}

/// 原版 platform.cc:83-104 — 全序: ASAN 粒度检查 → 分配器 (lsan 覆盖) → GCInfo 表 → caged 堆 → 全局指针
pub fn initialize_process(page_allocator: Option<*mut PageAllocator>, desired_heap_size: usize) {
    // v8: 原版 ASAN+64bit 臂 — 分配粒度须整除影子粒度
    #[cfg(all(sanitize = "address", target_pointer_width = "64"))]
    {
        let poisoning_granularity = 1 << asan_shadow_scale();
        assert_eq!(crate::K_ALLOCATION_GRANULARITY % poisoning_granularity, 0);
    }
    let allocator = create_allocator_if_needed(page_allocator);
    v8_check(G_PAGE_ALLOCATOR.load(std::sync::atomic::Ordering::Relaxed).is_null()); // 重复初始化拒绝
    GlobalGcInfoTable::initialize(allocator);
    // v8: CPPGC_CAGED_HEAP 目标配置开 — caged 堆按需初始化
    CagedHeap::initialize_if_needed(allocator, desired_heap_size);
    G_PAGE_ALLOCATOR.store(allocator as *mut _, std::sync::atomic::Ordering::Release);
}

/// 原版 platform.cc:106 — 关停: 全局分配器置空
pub fn shutdown_process() {
    G_PAGE_ALLOCATOR.store(std::ptr::null_mut(), std::sync::atomic::Ordering::Release);
}

// ══ 平台杂项 (原版 platform.cc:76-79) ══
/// Platform::GetTracingController — Leaky 静态单例
pub fn get_tracing_controller() -> &'static mut TracingController { unimplemented!() }
`;
const out = writeSrc("v8-heap-cppgc", "platform.rs.skel", body);
console.log(`OK platform.h.js: 锚 ${n} 条 (声明6+实现11); 双段 OOM + 3 进程函数; 产物 ${out}`);
