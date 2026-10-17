// rs8-capi/src/context.rs
//! C ABI 上下文 (Context) 控制

use std::panic::{AssertUnwindSafe, catch_unwind};

use rs8::Interpreter;

use crate::handles::{V8Context, V8Isolate, V8Local};

#[unsafe(no_mangle)]
pub extern "C" fn v8_context_new(isolate: *mut V8Isolate) -> V8Local {
  catch_unwind(AssertUnwindSafe(|| {
    if isolate.is_null() {
      return V8Local::empty();
    }
    let boxed = Box::new(V8Context {
      isolate,
      interpreter: Interpreter::new(),
    });
    V8Local::from_ptr(Box::into_raw(boxed).cast())
  }))
  .unwrap_or(V8Local::empty())
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_context_enter(context: V8Local) {
  let _ = context;
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_context_exit(context: V8Local) {
  let _ = context;
}

/// 销毁上下文实例
///
/// # Safety
/// 调用者须确保 `context` 为空句柄或包含由 `v8_context_new` 创建且未被释放的上下文。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_context_dispose(context: V8Local) {
  let _ = catch_unwind(AssertUnwindSafe(|| {
    if !context.is_empty() {
      // SAFETY: context 由 v8_context_new 分配
      unsafe {
        drop(Box::from_raw(context.ptr as *mut V8Context));
      }
    }
  }));
}
