// rs8-capi/src/value.rs
//! C ABI JavaScript 值类型判断与基本值创建

use crate::handles::{V8Isolate, V8Local};

// 预定义内部编码 Tagged 伪值
const TAG_UNDEFINED: usize = 0x01;
const TAG_NULL: usize = 0x03;
const TAG_TRUE: usize = 0x05;
const TAG_FALSE: usize = 0x07;

#[unsafe(no_mangle)]
pub extern "C" fn v8_undefined(isolate: *mut V8Isolate) -> V8Local {
  let _ = isolate;
  V8Local::from_ptr(TAG_UNDEFINED as *mut _)
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_null(isolate: *mut V8Isolate) -> V8Local {
  let _ = isolate;
  V8Local::from_ptr(TAG_NULL as *mut _)
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_true(isolate: *mut V8Isolate) -> V8Local {
  let _ = isolate;
  V8Local::from_ptr(TAG_TRUE as *mut _)
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_false(isolate: *mut V8Isolate) -> V8Local {
  let _ = isolate;
  V8Local::from_ptr(TAG_FALSE as *mut _)
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_boolean_new(isolate: *mut V8Isolate, value: bool) -> V8Local {
  if value {
    v8_true(isolate)
  } else {
    v8_false(isolate)
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_value_is_undefined(val: V8Local) -> bool {
  val.ptr as usize == TAG_UNDEFINED
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_value_is_null(val: V8Local) -> bool {
  val.ptr as usize == TAG_NULL
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_value_is_true(val: V8Local) -> bool {
  val.ptr as usize == TAG_TRUE
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_value_is_false(val: V8Local) -> bool {
  val.ptr as usize == TAG_FALSE
}

#[unsafe(no_mangle)]
pub extern "C" fn v8_value_is_boolean(val: V8Local) -> bool {
  let addr = val.ptr as usize;
  addr == TAG_TRUE || addr == TAG_FALSE
}
