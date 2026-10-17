use aok::{OK, Void};
use rs8::{
  AsyncContext, CodeCacheOptions, CodeType, Extension, Kind, MemoryAccessInformation, ModuleType,
  OUTPUT_ROUTES, PromiseHookType, RegValue, SHELL_OPTIONS, STRINGIFY_SOURCE, WorkerState,
  find_gpr_base, find_shell_option, get_output_route, is_cmp_mnemonic, width_from_suffix,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_shell_options_lookup() -> Void {
  // 查找已知 CLI 旗标对应的字段名
  assert_eq!(find_shell_option("can_block"), Some("can_block"));
  assert_eq!(find_shell_option("C"), Some("cwd"));
  assert_eq!(find_shell_option("shell"), Some("interactive_shell"));
  assert_eq!(find_shell_option("cpu-profiler"), Some("cpu_profiler"));

  // 未知旗标返回 None
  assert_eq!(find_shell_option("non_existent_flag_xyz"), None);

  // 验证旗标表中每一项均为非空有效标识符
  for &(field, cli) in &SHELL_OPTIONS {
    assert!(!field.is_empty());
    assert!(!cli.is_empty());
  }

  OK
}

#[test]
fn test_console_and_js_stringify() -> Void {
  // 测试控制台路由查询与默认流分配
  assert_eq!(get_output_route("Log"), Some(("stdout", "")));
  assert_eq!(
    get_output_route("Assert"),
    Some(("stdout", "console.assert"))
  );
  assert_eq!(get_output_route("Error"), Some(("stderr", "console.error")));
  assert_eq!(get_output_route("Warn"), Some(("stdout", "console.warn")));
  assert_eq!(get_output_route("NonExistent"), None);

  // 确保所有路由输出目标为标准 stdout 或 stderr
  for &(_, stream, _) in &OUTPUT_ROUTES {
    assert!(stream == "stdout" || stream == "stderr");
  }

  // JS Stringify 脚本完整性与防环机制检测
  assert!(STRINGIFY_SOURCE.contains("function Stringify("));
  assert!(STRINGIFY_SOURCE.contains("function StringifyProxy("));
  assert!(STRINGIFY_SOURCE.contains("stringifyDepthLimit"));

  OK
}

#[test]
fn test_async_hooks_context_lifecycle() -> Void {
  // 初始异步上下文状态判定
  let init_ctx = AsyncContext::initial();
  assert!(init_ctx.is_valid_execution());

  // 边界与异常状态测试
  let zero_ctx = AsyncContext {
    execution_async_id: 0.0,
    trigger_async_id: 0.0,
  };
  assert!(!zero_ctx.is_valid_execution());

  let nan_ctx = AsyncContext {
    execution_async_id: f64::NAN,
    trigger_async_id: 0.0,
  };
  assert!(!nan_ctx.is_valid_execution());

  let inf_ctx = AsyncContext {
    execution_async_id: f64::INFINITY,
    trigger_async_id: 0.0,
  };
  assert!(!inf_ctx.is_valid_execution());

  // PromiseHookType 枚举隔离性
  let p_init = PromiseHookType::Init;
  let p_before = PromiseHookType::Before;
  let p_after = PromiseHookType::After;
  let p_resolve = PromiseHookType::Resolve;
  assert_ne!(p_init, p_before);
  assert_ne!(p_after, p_resolve);

  OK
}

#[test]
fn test_memory_access_information_and_registers() -> Void {
  // 宽度解析逻辑 (b=1, w=2, l/d=4, q=8, 未知默认8)
  assert_eq!(width_from_suffix("b"), 1);
  assert_eq!(width_from_suffix("w"), 2);
  assert_eq!(width_from_suffix("l"), 4);
  assert_eq!(width_from_suffix("d"), 4);
  assert_eq!(width_from_suffix("q"), 8);
  assert_eq!(width_from_suffix("x"), 8);

  // GPR 寄存器别名查找映射到 64 位基名
  assert_eq!(find_gpr_base("rax"), Some("rax"));
  assert_eq!(find_gpr_base("eax"), Some("rax"));
  assert_eq!(find_gpr_base("ax"), Some("rax"));
  assert_eq!(find_gpr_base("al"), Some("rax"));
  assert_eq!(find_gpr_base("r8d"), Some("r8"));
  assert_eq!(find_gpr_base("r15b"), Some("r15"));
  assert_eq!(find_gpr_base("invalid_register"), None);

  // 比较类指令助记符判定
  assert!(is_cmp_mnemonic("cmp"));
  assert!(is_cmp_mnemonic("test"));
  assert!(is_cmp_mnemonic("comi"));
  assert!(!is_cmp_mnemonic("mov"));
  assert!(!is_cmp_mnemonic("add"));

  // 默认构造与读写类型一致性
  let r_info = MemoryAccessInformation::default_read(4);
  assert_eq!(r_info.kind, Kind::Read);
  assert_eq!(r_info.access_width, 4);
  assert_eq!(r_info.dest_width, 4);
  assert_eq!(r_info.extension, Extension::NoExtend);
  assert!(r_info.result_reg.is_none());

  let mut dummy_reg: RegValue = 0xbeef;
  let mut w_info = MemoryAccessInformation::default_write(8);
  w_info.result_reg = Some(&mut dummy_reg as *mut RegValue);
  assert_eq!(w_info.kind, Kind::Write);
  assert_eq!(w_info.access_width, 8);
  assert!(w_info.result_reg.is_some());

  // 状态机与选项枚举独立性
  assert_ne!(WorkerState::Ready, WorkerState::Running);
  assert_ne!(ModuleType::JavaScript, ModuleType::WebAssembly);
  assert_ne!(CodeType::FileName, CodeType::Function);
  assert_ne!(
    CodeCacheOptions::NoProduceCache,
    CodeCacheOptions::ProduceCache
  );

  OK
}
