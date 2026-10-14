//! v8/src/base/free_deleter.h 的 1:1 转写
//! 原版: 对 malloc 指针调用 base::Free 的函数对象 (配 std::unique_ptr 用)
//! Rust 侧: Deleter trait 承接 "所有权终结时的释放动作", FreeDeleter 为其实现;
//! vector/small_vector 一族以 Deleter 参数化释放后端, 不写死具体分配器

/// 释放后端参数化 (原版模板形参 Deleter/Allocator 的 trait 表达)
pub trait Deleter {
  /// void operator()(void* ptr) const
  /// # Safety
  /// ptr 必须为 null 或由配对分配器 (malloc 系) 返回的指针
  unsafe fn delete(ptr: *mut u8);
}

/// 原版 base::FreeDeleter: 对参数调用 free()
///
/// 可用于存放 malloc 分配指针的所有权包装 (对应
/// `std::unique_ptr<int, base::FreeDeleter>`)
#[derive(Debug, Clone, Copy, Default)]
pub struct FreeDeleter;

impl FreeDeleter {
  /// inline void operator()(void* ptr) const { base::Free(ptr); }
  ///
  /// # Safety
  /// ptr 必须为 null 或由 malloc/calloc/realloc 返回且未被释放的指针,
  /// 与原版 base::Free(free()) 的前提一致
  #[inline]
  pub unsafe fn call(&self, ptr: *mut u8) {
    // SAFETY: 前提由调用方维持, free(nullptr) 为标准库定义的 no-op
    unsafe { libc::free(ptr.cast()) };
  }
}

impl Deleter for FreeDeleter {
  #[inline]
  unsafe fn delete(ptr: *mut u8) {
    // SAFETY: 与 FreeDeleter::call 同一契约
    unsafe { libc::free(ptr.cast()) };
  }
}
