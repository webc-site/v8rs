// 由 tool/src/objects/managed.h.js 从 v8/src/objects/managed.{h,inl.h,cc,tq} 生成; 修订就地改, 复跑覆盖
// 托管资源骨架 (CppGCManaged / ManagedPtrDestructor / Managed / TrustedManaged)

use core::ffi::c_void;

/// CppGCManagedWrapper 字段面 (cppgc 分配, 不落 V8 堆)
pub const CPP_GC_MANAGED_WRAPPER_FIELDS: [&str; 7] = [
  "type_id_",
  "estimated_size_",
  "shared_ptr_ptr_",
  "destructor_",
  "isolate_",
  "isolate_alive_token_",
  "external_memory_accounter_",
];

/// ManagedPtrDestructor 字段面 (isolate 级析构双链表节点)
pub const MANAGED_PTR_DESTRUCTOR_FIELDS: [&str; 8] = [
  "estimated_size_",
  "prev_",
  "next_",
  "shared_ptr_ptr_",
  "destructor_",
  "shared_",
  "global_handle_location_",
  "external_memory_accounter_",
];

/// 外部内存计量回调面 (原版 v8::ExternalMemoryAccounter::Increase/Decrease;
/// isolate 承载与计量落桩归上层, 此处窄 trait 承接差额收支语义)
pub trait ExternalMemoryAccounter {
  /// 原版 Increase(v8_isolate, delta)
  fn increase(&mut self, delta: usize);
  /// 原版 Decrease(v8_isolate, delta)
  fn decrease(&mut self, delta: usize);
}

/// 类层级表 (类, 基类, 注记)
pub const HIERARCHY: [(&str, &str, &str); 6] = [
  (
    "CppGCManagedWrapper",
    "cppgc::GarbageCollected",
    "cppgc 侧包装 (非 V8 堆对象)",
  ),
  (
    "CppGCManagedBase",
    "HeapObject",
    "V8_OBJECT; tq @cppObjectLayoutDefinition",
  ),
  (
    "CppGCManaged",
    "CppGCManagedBase",
    "模板; V8_IT_REUSE_PARENT",
  ),
  (
    "ManagedPtrDestructor",
    "ExternalPointerTable::ManagedResource | Malloced",
    "隔离析构双链表节点",
  ),
  ("Managed", "Foreign", "已弃用, 迁 CppGCManaged"),
  ("TrustedManaged", "TrustedForeign", "可信空间等价物, 免 tag"),
];
/// CppGCManaged::Ptr 公开操作 (Managed::Ptr 同构)
pub const PTR_OPS: [&str; 10] = [
  "Ptr",
  "operator=",
  "operator->",
  "operator*",
  "raw",
  "get",
  "as_shared_ptr",
  "Reset",
  "operator bool",
  "operator==",
];

/// cpp 堆指针 tag 锚 (原版 kCppGCManagedTag)
pub const CPP_GC_MANAGED_TAG: &str = "kCppGCManagedTag";

/// 类型注册宏对 (TypeId / ExternalPointerTag 特化入口)
pub const ASSIGN_MACROS: [&str; 2] = [
  "ASSIGN_MANAGED_TYPE_ID_FOR_MANAGED",
  "ASSIGN_EXTERNAL_POINTER_TAG_FOR_MANAGED",
];

/// tq 布局 (CppGCManagedBase extends HeapObject)
pub const TQ_LAYOUT: [(&str, &str); 1] = [("cpp_gc_wrapper", "CppHeapPointer")];

/// 空计量实现 (无 embedder MemoryEstimator 时的占位; 亦是差值语义测试的默认替身)
#[derive(Debug, Default, Clone, Copy)]
pub struct NoMemoryAccounting;

impl ExternalMemoryAccounter for NoMemoryAccounting {
  fn increase(&mut self, _delta: usize) {}
  fn decrease(&mut self, _delta: usize) {}
}

/// CppGCManagedBase 布局标记 (tq @cppObjectLayoutDefinition; 单字段 cpp_gc_wrapper_)
pub struct CppGCManagedBase {
  /// 原版 CppHeapPointerMember cpp_gc_wrapper_ (沙箱开: uint32 表句柄)。
  pub cpp_gc_wrapper: u32,
}

impl CppGCManagedBase {
  /// 原版 kHeaderSize = kSize = sizeof(CppGCManagedBase) (cpp_gc_wrapper_ 单字段)
  ///
  /// v8: src/objects/managed.h CppGCManagedBase::kHeaderSize —
  /// = sizeof(HeapObject 头) + sizeof(CppHeapPointerMember) (沙箱/压缩二态由
  /// 金标准 offset 表供给两参, 见 todo.md 五.布局金标准)。
  #[inline]
  #[must_use]
  pub const fn header_size(heap_object_size: usize, cpp_heap_pointer_size: usize) -> usize {
    heap_object_size + cpp_heap_pointer_size
  }

  /// v8: src/objects/managed-inl.h CppGCManagedBase::GetWrapper — ReadCppHeapPointerField(kCppGCManagedTag)
  #[must_use]
  pub fn get_wrapper(&self) -> &'static CppGCManagedWrapper<NoMemoryAccounting> {
    todo!(
      "CppGCManagedBase::GetWrapper: 依赖 core 字段访问器 + v8-sandbox cpp 堆指针表; 解锁 v8-sandbox + v8-objects-core"
    )
  }

  /// v8: src/objects/managed-inl.h CppGCManagedBase::estimated_size — GetWrapper()->estimated_size()
  #[must_use]
  pub fn estimated_size(&self) -> usize {
    todo!("CppGCManagedBase::estimated_size: 依赖 GetWrapper; 解锁 v8-sandbox + v8-objects-core")
  }
}

/// cppgc 侧包装 (持有 shared_ptr 与外部内存计量; managed.h CppGCManagedWrapper 字段面)
pub struct CppGCManagedWrapper<A: ExternalMemoryAccounter> {
  /// 原版 ManagedTypeId type_id_。
  pub type_id: u32,
  /// 原版 size_t estimated_size_。
  pub estimated_size: usize,
  /// 原版 void* shared_ptr_ptr_ (std::shared_ptr 存储地址)。
  pub shared_ptr_ptr: usize,
  /// 原版 void (*destructor_)(void*)。
  pub destructor: Option<fn(*mut c_void)>,
  /// 原版 v8::Isolate* isolate_ + isolate_alive_token_ (析构期存活判定;
  /// Isolate 形态未就位, 以存活谓词承接, 接线归推迟台账)。
  pub isolate_alive: bool,
  /// 原版 ExternalMemoryAccounter external_memory_accounter_。
  pub external_memory_accounter: A,
}

impl<A: ExternalMemoryAccounter> CppGCManagedWrapper<A> {
  /// 构造 (原版 ctor: estimated_size > 0 且 isolate 存活即 Increase)。
  ///
  /// v8: src/objects/managed.h CppGCManagedWrapper::CppGCManagedWrapper
  #[must_use]
  pub fn new(
    type_id: u32,
    estimated_size: usize,
    shared_ptr_ptr: usize,
    destructor: Option<fn(*mut c_void)>,
    isolate_alive: bool,
    external_memory_accounter: A,
  ) -> Self {
    let mut accounter = external_memory_accounter;
    if estimated_size > 0 && isolate_alive {
      accounter.increase(estimated_size);
    }
    Self {
      type_id,
      estimated_size,
      shared_ptr_ptr,
      destructor,
      isolate_alive,
      external_memory_accounter: accounter,
    }
  }

  /// v8: managed.cc CppGCManagedWrapper::UpdateEstimatedSize — 差额增减 ExternalMemoryAccounter
  ///
  /// 原版 (managed.cc:64-77): 等值早返; 增大记差额 Increase, 减小记差额
  /// Decrease; 最后回写 estimated_size_。isolate 形参在 Rust 侧由上层
  /// accounter 实现承接 (reinterpret_cast<v8::Isolate*> 仅透传给计量 API)。
  pub fn update_estimated_size(&mut self, new_estimated_size: usize) {
    if self.estimated_size == new_estimated_size {
      return;
    }
    if self.estimated_size < new_estimated_size {
      self
        .external_memory_accounter
        .increase(new_estimated_size - self.estimated_size);
    } else {
      self
        .external_memory_accounter
        .decrease(self.estimated_size - new_estimated_size);
    }
    self.estimated_size = new_estimated_size;
  }

  /// 原版 type_id() / estimated_size() / shared_ptr_ptr() getter (inline 直读)。
  ///
  /// v8: src/objects/managed.h CppGCManagedWrapper::{type_id,estimated_size,shared_ptr_ptr}
  #[inline]
  #[must_use]
  pub fn type_id(&self) -> u32 {
    self.type_id
  }

  /// v8: src/objects/managed.h CppGCManagedWrapper::estimated_size
  #[inline]
  #[must_use]
  pub fn estimated_size(&self) -> usize {
    self.estimated_size
  }

  /// v8: src/objects/managed.h CppGCManagedWrapper::shared_ptr_ptr
  #[inline]
  #[must_use]
  pub fn shared_ptr_ptr(&self) -> usize {
    self.shared_ptr_ptr
  }
}

/// isolate 级析构双链表节点 (Managed<T> 路径; managed.h ManagedPtrDestructor 字段面)
pub struct ManagedPtrDestructor<A: ExternalMemoryAccounter> {
  /// 原版 size_t estimated_size_。
  pub estimated_size: usize,
  /// 原版 ManagedPtrDestructor* prev_ (isolate 侵入双链, 挂接面
  /// Isolate::Register/UnregisterManagedPtrDestructor 归推迟台账)。
  pub prev: usize,
  /// 原版 ManagedPtrDestructor* next_。
  pub next: usize,
  /// 原版 void* shared_ptr_ptr_。
  pub shared_ptr_ptr: usize,
  /// 原版 void (*destructor_)(void*)。
  pub destructor: Option<fn(*mut c_void)>,
  /// 原版 bool shared_ (true = 计费落 shared-space isolate)。
  pub shared: bool,
  /// 原版 GlobalHandles::Location global_handle_location_ (句柄形态未就位,
  /// 以槽标识透传; 接线归推迟台账)。
  pub global_handle_location: usize,
  /// 原版 ExternalMemoryAccounter external_memory_accounter_。
  pub external_memory_accounter: A,
}

impl<A: ExternalMemoryAccounter> ManagedPtrDestructor<A> {
  /// v8: managed.cc ManagedPtrDestructor::UpdateEstimatedSize — 差额增减后回写
  ///
  /// 原版 (managed.cc:34-47): 与 CppGCManagedWrapper::UpdateEstimatedSize 同构;
  /// isolate 形参承接口径同上。
  pub fn update_estimated_size(&mut self, new_estimated_size: usize) {
    if self.estimated_size == new_estimated_size {
      return;
    }
    if self.estimated_size < new_estimated_size {
      self
        .external_memory_accounter
        .increase(new_estimated_size - self.estimated_size);
    } else {
      self
        .external_memory_accounter
        .decrease(self.estimated_size - new_estimated_size);
    }
    self.estimated_size = new_estimated_size;
  }
}

/// GC 终结器第一遍: 销毁 global handle 并挂第二遍回调
///
/// v8: src/objects/managed.cc ManagedObjectFinalizer — GlobalHandles::Destroy + SetSecondPassCallback
pub fn managed_object_finalizer() {
  todo!(
    "ManagedObjectFinalizer: 依赖 GlobalHandles::Destroy + SetSecondPassCallback + Handle; 解锁 v8-handles + v8-heap-cppgc"
  )
}

/// GC 终结器第二遍: 注销节点 → 析构 shared_ptr → 扣减外部内存 → 删节点
///
/// v8: src/objects/managed.cc ManagedObjectFinalizerSecondPass
pub fn managed_object_finalizer_second_pass() {
  todo!(
    "ManagedObjectFinalizerSecondPass: 依赖 Isolate::UnregisterManagedPtrDestructor + shared_ptr delete + ExternalMemoryAccounter; 解锁 v8-execution-isolate + v8-handles + v8-heap-cppgc"
  )
}

/// 语义实现面登记 (Developer 接线):
/// - detail::Destructor<T>(void*) — inl: delete shared_ptr
/// - Managed<T>::From / TrustedManaged<T>::From — New{,Trusted}Foreign + RegisterManagedPtrDestructor
/// - CppGCManaged<T>::Create — MakeGarbageCollected + NewCppGCManagedBase + 写屏障
/// - CppGCManaged<T>::ptr/raw/get/GetSharedPtr/SetManagedObject — CHECK_EQ type_id 锚
/// - Ptr::as_shared_ptr/Reset — 移动语义面
pub const SIGNATURES: [&str; 10] = [
  "destructor_t",
  "managed_from",
  "trusted_managed_from",
  "cpp_gc_managed_create",
  "cpp_gc_managed_ptr",
  "cpp_gc_managed_raw",
  "cpp_gc_managed_get",
  "cpp_gc_managed_get_shared_ptr",
  "set_managed_object",
  "ptr_as_shared_ptr",
];

// —— 提取不变式断言 —— (const 上下文无 str 比较, 用字节循环 const fn)
const fn str_eq(a: &str, b: &str) -> bool {
  let (a, b) = (a.as_bytes(), b.as_bytes());
  if a.len() != b.len() {
    return false;
  }
  let mut i = 0;
  while i < a.len() {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}
const _: () = {
  assert!(str_eq(CPP_GC_MANAGED_WRAPPER_FIELDS[0], "type_id_"));
  assert!(CPP_GC_MANAGED_WRAPPER_FIELDS.len() == 7);
  assert!(str_eq(MANAGED_PTR_DESTRUCTOR_FIELDS[0], "estimated_size_"));
  assert!(MANAGED_PTR_DESTRUCTOR_FIELDS.len() == 8);
  assert!(HIERARCHY.len() == 6);
  assert!(PTR_OPS.len() == 10);
  assert!(str_eq(TQ_LAYOUT[0].0, "cpp_gc_wrapper") && str_eq(TQ_LAYOUT[0].1, "CppHeapPointer"));
  assert!(SIGNATURES.len() == 10);
};
