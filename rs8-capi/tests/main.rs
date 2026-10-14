use std::{
  ffi::{CStr, CString},
  ptr::{null, null_mut},
};

use aok::{OK, Void};
use rs8_capi::{
  V8Global, V8Local, V8StartupData, v8_boolean_new, v8_context_dispose, v8_context_enter,
  v8_context_exit, v8_context_new, v8_dispose, v8_false, v8_get_version, v8_global_is_empty,
  v8_global_reset, v8_initialize, v8_is_initialized, v8_isolate_dispose, v8_isolate_enter,
  v8_isolate_exit, v8_isolate_get_current, v8_isolate_new, v8_local_is_empty, v8_null,
  v8_platform_dispose, v8_platform_new_default, v8_platform_pump_message_loop,
  v8_set_flags_from_string, v8_startup_data_is_empty, v8_true, v8_undefined, v8_value_is_boolean,
  v8_value_is_false, v8_value_is_null, v8_value_is_true, v8_value_is_undefined,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_capi_handles() -> Void {
  let empty_local = V8Local::empty();
  assert!(empty_local.is_empty());
  assert!(v8_local_is_empty(empty_local));

  let mut global = V8Global::empty();
  // SAFETY: 传入有效引用的指针
  assert!(unsafe { v8_global_is_empty(&global) });

  global.ptr = 0x1234 as *mut _;
  // SAFETY: 传入有效引用的指针
  assert!(!unsafe { v8_global_is_empty(&global) });

  // SAFETY: 传入有效可变引用的指针
  unsafe { v8_global_reset(&mut global) };
  // SAFETY: 传入有效引用的指针
  assert!(unsafe { v8_global_is_empty(&global) });

  // 验证 Local 句柄类型支持转换检测
  assert!(rs8_capi::supports_local_conversion("Object"));
  assert!(rs8_capi::supports_local_conversion("String"));
  assert!(!rs8_capi::supports_local_conversion("NonExistentType"));

  OK
}

#[test]
fn test_capi_lifecycle_and_flags() -> Void {
  assert!(v8_initialize());
  assert!(v8_is_initialized());

  // 版本号查询
  let ver_ptr = v8_get_version();
  assert!(!ver_ptr.is_null());
  // SAFETY: ver_ptr 指向有效静态 C 字符串
  let ver_str = unsafe { CStr::from_ptr(ver_ptr) }.to_str()?;
  assert!(ver_str.contains("v8rs"));

  // 命令行参数设置
  let flags = CString::new("--harmony --max-opt=2")?;
  // SAFETY: flags 为合法的 CString
  assert!(unsafe { v8_set_flags_from_string(flags.as_ptr()) });
  // SAFETY: 允许测试空指针防御
  assert!(!unsafe { v8_set_flags_from_string(null()) });

  assert!(v8_dispose());
  assert!(!v8_is_initialized());

  OK
}

#[test]
fn test_capi_isolate_and_context() -> Void {
  // SAFETY: 允许传递 null 作为 IsolateCreateParams
  let isolate = unsafe { v8_isolate_new(null()) };
  assert!(!isolate.is_null());

  v8_isolate_enter(isolate);
  assert_eq!(v8_isolate_get_current(), isolate);

  let ctx = v8_context_new(isolate);
  assert!(!ctx.is_empty());

  v8_context_enter(ctx);
  v8_context_exit(ctx);
  // SAFETY: ctx 由 v8_context_new 分配
  unsafe { v8_context_dispose(ctx) };

  v8_isolate_exit(isolate);
  assert_eq!(v8_isolate_get_current(), null_mut());

  // SAFETY: isolate 由 v8_isolate_new 分配
  unsafe { v8_isolate_dispose(isolate) };

  OK
}

#[test]
fn test_capi_values_and_literals() -> Void {
  let undef = v8_undefined(null_mut());
  assert!(v8_value_is_undefined(undef));
  assert!(!v8_value_is_null(undef));

  let null_val = v8_null(null_mut());
  assert!(v8_value_is_null(null_val));
  assert!(!v8_value_is_undefined(null_val));

  let t = v8_true(null_mut());
  assert!(v8_value_is_true(t));
  assert!(v8_value_is_boolean(t));
  assert!(!v8_value_is_false(t));

  let f = v8_false(null_mut());
  assert!(v8_value_is_false(f));
  assert!(v8_value_is_boolean(f));
  assert!(!v8_value_is_true(f));

  let b_true = v8_boolean_new(null_mut(), true);
  assert!(v8_value_is_true(b_true));

  let b_false = v8_boolean_new(null_mut(), false);
  assert!(v8_value_is_false(b_false));

  OK
}

#[test]
fn test_capi_platform_and_snapshot() -> Void {
  let platform = v8_platform_new_default(4, true);
  assert!(!platform.is_null());

  assert!(v8_platform_pump_message_loop(platform, null_mut()));
  // SAFETY: platform 由 v8_platform_new_default 分配
  unsafe { v8_platform_dispose(platform) };

  // 空快照与有效快照判定
  // SAFETY: 允许测试空指针
  assert!(unsafe { v8_startup_data_is_empty(null()) });

  let empty_data = V8StartupData {
    data: null(),
    raw_size: 0,
  };
  // SAFETY: 传入有效引用的指针
  assert!(unsafe { v8_startup_data_is_empty(&empty_data) });

  let buf = [1u8, 2, 3];
  let valid_data = V8StartupData {
    data: buf.as_ptr(),
    raw_size: 3,
  };
  // SAFETY: 传入有效引用的指针
  assert!(!unsafe { v8_startup_data_is_empty(&valid_data) });

  OK
}

#[test]
fn test_capi_script_compile_and_run() -> Void {
  use rs8_capi::{
    v8_context_dispose, v8_context_new, v8_isolate_dispose, v8_isolate_new, v8_script_compile,
    v8_script_dispose, v8_script_run, v8_string_new_from_utf8, v8_value_dispose,
    v8_value_number_value,
  };

  // 1. 创建 Isolate 与 Context
  // SAFETY: 传入合法空指针默认配置
  let isolate = unsafe { v8_isolate_new(null()) };
  assert!(!isolate.is_null());

  // SAFETY: 传入有效 isolate
  let context = v8_context_new(isolate);
  assert!(!context.is_empty());

  // 2. 构造 JavaScript 源码
  let js_src = CString::new("var a = 100; var b = 25; (a * 2 + b) / 5;")?;
  // SAFETY: 传入有效 C 字符串指针
  let source = unsafe { v8_string_new_from_utf8(isolate, js_src.as_ptr(), -1) };
  assert!(!source.is_empty());

  // 3. 编译脚本
  // SAFETY: 传入有效 context 与 source
  let script = unsafe { v8_script_compile(context, source) };
  assert!(!script.is_null());

  // 4. 执行脚本并获取结果
  // SAFETY: 传入有效 script 与 context
  let result = unsafe { v8_script_run(script, context) };
  assert!(!result.is_empty());

  // 5. 校验计算结果: (100 * 2 + 25) / 5 = 225 / 5 = 45.0
  // SAFETY: 传入有效 result 与 context
  let num_val = unsafe { v8_value_number_value(result, context) };
  assert!((num_val - 45.0).abs() < 1e-6);

  // 6. 资源释放
  unsafe {
    v8_value_dispose(result);
    v8_value_dispose(source);
    v8_script_dispose(script);
    v8_context_dispose(context);
    v8_isolate_dispose(isolate);
  }

  OK
}

#[test]
fn test_capi_shared_context_state() -> Void {
  use rs8_capi::{
    v8_context_dispose, v8_context_new, v8_isolate_dispose, v8_isolate_new, v8_script_compile,
    v8_script_dispose, v8_script_run, v8_string_new_from_utf8, v8_value_dispose,
    v8_value_number_value,
  };

  let isolate = unsafe { v8_isolate_new(null()) };
  let context = v8_context_new(isolate);

  // 脚本 1: 定义全局变量
  let src1 = CString::new("var counter = 100;")?;
  let s1 = unsafe { v8_string_new_from_utf8(isolate, src1.as_ptr(), -1) };
  let script1 = unsafe { v8_script_compile(context, s1) };
  let _ = unsafe { v8_script_run(script1, context) };

  // 脚本 2: 读取并累加同一 Context 下的变量
  let src2 = CString::new("counter = counter + 50; counter * 2;")?;
  let s2 = unsafe { v8_string_new_from_utf8(isolate, src2.as_ptr(), -1) };
  let script2 = unsafe { v8_script_compile(context, s2) };
  let res2 = unsafe { v8_script_run(script2, context) };

  let val = unsafe { v8_value_number_value(res2, context) };
  assert!((val - 300.0).abs() < 1e-6);

  unsafe {
    v8_value_dispose(res2);
    v8_value_dispose(s2);
    v8_script_dispose(script2);
    v8_value_dispose(s1);
    v8_script_dispose(script1);
    v8_context_dispose(context);
    v8_isolate_dispose(isolate);
  }

  OK
}

#[test]
fn test_capi_panic_isolation() -> Void {
  use rs8_capi::{
    V8Local, v8_context_dispose, v8_context_new, v8_isolate_dispose, v8_isolate_new,
    v8_script_compile, v8_script_run, v8_string_new_from_utf8, v8_value_dispose,
    v8_value_number_value,
  };

  // 1. 空指针安全
  assert!(!unsafe { v8_isolate_new(null()) }.is_null());
  let isolate = unsafe { v8_isolate_new(null()) };
  assert!(v8_context_new(null_mut()).is_empty());

  let ctx = v8_context_new(isolate);
  assert!(!ctx.is_empty());

  // 2. 空源码 / 空上下文编译隔离
  let empty_local = V8Local::empty();
  assert!(unsafe { v8_script_compile(empty_local, empty_local) }.is_null());
  assert!(unsafe { v8_script_run(null_mut(), ctx) }.is_empty());

  // 3. 语法错误隔离
  let bad_code = CString::new("function bad( { ;;;")?;
  let bad_src = unsafe { v8_string_new_from_utf8(isolate, bad_code.as_ptr(), -1) };
  let script = unsafe { v8_script_compile(ctx, bad_src) };
  assert!(script.is_null());

  // 4. 空值取数值隔离
  let nan_val = unsafe { v8_value_number_value(empty_local, ctx) };
  assert!(nan_val.is_nan());

  // 5. 释放隔离
  unsafe {
    v8_value_dispose(bad_src);
    v8_value_dispose(empty_local);
    v8_context_dispose(ctx);
    v8_isolate_dispose(isolate);
  }

  OK
}
