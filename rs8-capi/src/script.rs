// rs8-capi/src/script.rs
//! C ABI JavaScript 脚本编译与执行 (Script / String / Number API)

use std::{
  ffi::CStr,
  os::raw::c_char,
  panic::{AssertUnwindSafe, catch_unwind},
  ptr::null_mut,
  slice,
};

use rs8::{
  JsValue,
  runtime::{ast::Stmt, lexer::Lexer, parser::Parser},
};

use crate::handles::{V8Context, V8Isolate, V8Local};

/// 内部包装的 JavaScript 脚本实例
pub struct V8Script {
  pub program: Vec<Stmt>,
  pub source: String,
}

/// 内部包装的动态堆值
pub struct V8HeapValue {
  pub value: JsValue,
}

/// 从 UTF-8 C 字符串创建 V8 字符串句柄
///
/// # Safety
/// 调用者须确保 `data` 指向有效的 UTF-8 字节串。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_string_new_from_utf8(
  isolate: *mut V8Isolate,
  data: *const c_char,
  length: i32,
) -> V8Local {
  catch_unwind(AssertUnwindSafe(|| {
    if isolate.is_null() || data.is_null() {
      return V8Local::empty();
    }
    let s = if length >= 0 {
      let slice = unsafe { slice::from_raw_parts(data as *const u8, length as usize) };
      String::from_utf8_lossy(slice).into_owned()
    } else {
      unsafe { CStr::from_ptr(data) }
        .to_string_lossy()
        .into_owned()
    };
    let boxed = Box::new(V8HeapValue {
      value: JsValue::String(s.into()),
    });
    V8Local::from_ptr(Box::into_raw(boxed).cast())
  }))
  .unwrap_or(V8Local::empty())
}

/// 创建 IEEE-754 浮点数值句柄
#[unsafe(no_mangle)]
pub extern "C" fn v8_number_new(isolate: *mut V8Isolate, value: f64) -> V8Local {
  catch_unwind(|| {
    if isolate.is_null() {
      return V8Local::empty();
    }
    let boxed = Box::new(V8HeapValue {
      value: JsValue::Number(value),
    });
    V8Local::from_ptr(Box::into_raw(boxed).cast())
  })
  .unwrap_or(V8Local::empty())
}

/// 编译 JavaScript 源代码生成 AST
///
/// # Safety
/// 调用者须确保 `source` 为有效的 V8Local 字符串句柄。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_script_compile(context: V8Local, source: V8Local) -> *mut V8Script {
  catch_unwind(AssertUnwindSafe(|| {
    if context.is_empty() || source.is_empty() {
      return null_mut();
    }
    let heap_val = source.ptr as *mut V8HeapValue;
    let code = unsafe {
      match &(*heap_val).value {
        JsValue::String(s) => s.to_string(),
        _ => return null_mut(),
      }
    };
    let mut lexer = Lexer::new(&code);
    let Ok(tokens) = lexer.tokenize_all() else {
      return null_mut();
    };
    let mut parser = Parser::new(tokens, &code);
    let Ok(program) = parser.parse_program() else {
      return null_mut();
    };
    Box::into_raw(Box::new(V8Script {
      program,
      source: code,
    }))
  }))
  .unwrap_or(null_mut())
}

/// 在指定上下文执行已编译的脚本并返回结果
///
/// # Safety
/// 调用者须确保 `script` 指向由 `v8_script_compile` 产生的有效实例且 `context` 有效。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_script_run(script: *mut V8Script, context: V8Local) -> V8Local {
  catch_unwind(AssertUnwindSafe(|| {
    if script.is_null() || context.is_empty() {
      return V8Local::empty();
    }
    let ctx = unsafe { &mut *(context.ptr as *mut V8Context) };
    let script_ref = unsafe { &*script };
    match ctx.interpreter.execute(&script_ref.program) {
      Ok(val) => {
        let boxed = Box::new(V8HeapValue { value: val });
        V8Local::from_ptr(Box::into_raw(boxed).cast())
      }
      Err(_) => V8Local::empty(),
    }
  }))
  .unwrap_or(V8Local::empty())
}

/// 释放脚本实例
///
/// # Safety
/// 调用者须确保 `script` 由 `v8_script_compile` 分配且未被重复释放。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_script_dispose(script: *mut V8Script) {
  let _ = catch_unwind(AssertUnwindSafe(|| {
    if !script.is_null() {
      unsafe {
        drop(Box::from_raw(script));
      }
    }
  }));
}

/// 从数值句柄中提取 f64
///
/// # Safety
/// 调用者须确保 `val` 为有效的数值句柄。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_value_number_value(val: V8Local, context: V8Local) -> f64 {
  catch_unwind(AssertUnwindSafe(|| {
    if val.is_empty() || context.is_empty() {
      return f64::NAN;
    }
    let heap_val = val.ptr as *mut V8HeapValue;
    unsafe {
      match (*heap_val).value {
        JsValue::Number(n) => n,
        ref other => other.to_number(),
      }
    }
  }))
  .unwrap_or(f64::NAN)
}

/// 释放堆值句柄
///
/// # Safety
/// 调用者须确保 `val` 是通过 `v8_string_new_from_utf8` 或 `v8_number_new` 或 `v8_script_run` 创建的句柄。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn v8_value_dispose(val: V8Local) {
  let _ = catch_unwind(AssertUnwindSafe(|| {
    let addr = val.ptr as usize;
    // 过滤预定义的静态立即数 (undefined, null, true, false)
    if addr > 0x10 && !val.ptr.is_null() {
      unsafe {
        drop(Box::from_raw(val.ptr as *mut V8HeapValue));
      }
    }
  }));
}
