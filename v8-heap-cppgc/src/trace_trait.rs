// 由 tool/src/heap/cppgc-internal/trace-trait.cc.js 从 v8/src/heap/cppgc-internal/trace-trait.cc 生成;
// 型面 (TraceCallback/TraceDescriptor) 声明在 include/cppgc/trace-trait.h; 修订就地改, 复跑覆盖。
// 推迟台账: TraceTraitFromInnerAddressImpl::GetTraceDescriptor (trace-trait.cc:15-27) 依赖
// BasePage::FromPayload + ObjectHeaderFromInnerAddress (heap_page) 页内回溯, 随 heap_page 批次挂载
// (见 readme/zh.md); TraceTrait<T>/TraceTraitImpl<T,bool> 与 TraceRootCallback/TraceDescriptorCallback
// (include/cppgc/trace-trait.h:21,61,75-122) 属公共 API 模板面, 待 embedding 层。

use core::ffi::c_void;

use crate::visitor::Visitor;

/// v8: include/cppgc/trace-trait.h:37 TraceCallback — `void (*)(Visitor* visitor, const void* object)`
/// Rust 统一 unsafe fn 指针 (回调侧解引用裸指针, 契约由注册方保证)。
pub type TraceCallback = unsafe fn(*mut Visitor, *const c_void);

/// v8: include/cppgc/trace-trait.h:43-53 TraceDescriptor — { base_object_payload, callback }
#[derive(Clone, Copy)]
pub struct TraceDescriptor {
  /// 原版 base_object_payload (:48) — 直接继承 GarbageCollected 的基类指针
  pub base_object_payload: *const c_void,
  /// 原版 callback (:52) — 该对象的 Trace 回调
  pub callback: TraceCallback,
}
