// 由 tool/src/regexp/regexp-result-vector.h.js 从 v8/src/regexp/regexp-result-vector.{h,cc} 生成; 修订就地改, 复跑覆盖
// 匹配结果向量骨架 (i32 寄存器对; 语义面待 Developer 修订)

use core::ptr::null_mut;

use crate::regexp_utils::IsolateMarker;

/// 结果向量作用域 (对应原版 ResultVectorScope)
/// 析构时归还静态向量或释放动态向量
pub struct ResultVectorScope<'a> {
  isolate: &'a IsolateMarker,
  is_dynamic: bool,
  value: Option<&'a mut [i32]>,
}

impl<'a> ResultVectorScope<'a> {
  /// 原版 ResultVectorScope(Isolate*) (cc:11) — 仅持 isolate, 静态向量待 Initialize 接管
  pub fn new(isolate: &'a IsolateMarker) -> Self {
    Self {
      isolate,
      is_dynamic: false,
      value: None,
    }
  }

  /// 原版 ResultVectorScope(Isolate*, int size) (cc:13-16) — 构造即 Initialize
  pub fn with_size(isolate: &'a IsolateMarker, size: i32) -> Self {
    let mut scope = Self::new(isolate);
    scope.initialize(size);
    scope
  }

  /// 原版 Initialize(int size) (cc:30-46) — 静态向量/动态分配二择;
  /// IsolateMarker 无静态向量存储面 (语义推迟 L3), 占位恒返回空切片
  pub fn initialize(&mut self, _size: i32) -> &mut [i32] {
    &mut []
  }

  /// 原版 value() (h:23-26)
  pub fn value(&self) -> Option<&[i32]> {
    self.value.as_deref()
  }
}

impl Drop for ResultVectorScope<'_> {
  /// 原版析构 (cc:18-28): 动态向量经 ResultVector::Free 归还,
  /// 静态向量 set_regexp_static_result_offsets_vector 交还 isolate;
  /// IsolateMarker 无存储面, 静态交还分支推迟为空操作 (语义面待 L3)
  fn drop(&mut self) {
    if self.is_dynamic
      && let Some(v) = self.value.as_deref_mut()
    {
      ResultVector::free(self.isolate, v.as_mut_ptr());
    }
  }
}

/// 静态分配/释放 (对应原版 ResultVector, AllStatic)
pub struct ResultVector;

impl ResultVector {
  /// 原版 Allocate(Isolate*, uint32_t size) (cc:51-68) — sandbox/堆分配 +
  /// isolate 动态向量注册表登记; 注册表面推迟 L3, 占位恒返回 null
  pub fn allocate(_isolate: &IsolateMarker, _size: u32) -> *mut i32 {
    null_mut()
  }

  /// 原版 Free(Isolate*, int32_t*) (cc:74-85) — 注册表摘除 + 释放; 推迟为空操作
  pub fn free(_isolate: &IsolateMarker, _vector: *mut i32) {}
}
