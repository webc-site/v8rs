// 由 tool/src/tracing/code-data-source.h.js 从 v8/src/tracing/perfetto-logger.{h,cc} 生成; 修订就地改, 复跑覆盖
// PerfettoLogger 事件面签名骨架

/// PerfettoLogger — LogEventListener 到 perfetto 包的转写层 (签名骨架)
pub trait PerfettoEventListenerSkel {
  /// 代码创建事件, 5 个重载 (AbstractCode ×4 + WasmCode ×1)
  fn code_create_event(&mut self, _tag: u8, _code: usize, _name: usize);
  /// 回调登记
  fn callback_event(&mut self, _name: usize, _entry_point: usize);
  fn getter_callback_event(&mut self, _name: usize, _entry_point: usize);
  fn setter_callback_event(&mut self, _name: usize, _entry_point: usize);
  /// 正则代码创建
  fn reg_exp_code_create_event(&mut self, _code: usize, _name: usize);
  /// 代码移动
  fn code_move_event(&mut self, _from: usize, _to: usize);
  fn bytecode_move_event(&mut self, _from: usize, _to: usize);
  fn shared_function_info_move_event(&mut self, _from: usize, _to: usize);
  fn native_context_move_event(&mut self, _from: usize, _to: usize);
  /// GC 触发的整体移动
  fn code_moving_gc_event(&mut self);
  /// 关优化
  fn code_disable_opt_event(&mut self, _code: usize, _sfi: usize);
  /// 反优化 (kind + pc + 原栈参数)
  fn code_deopt_event(&mut self, _code: usize, _kind: u8, _pc: usize, _fp_to_sp: usize);
  /// 依赖变更
  fn code_dependency_change_event(&mut self, _code: usize);
  /// 弱代码清理
  fn weak_code_clear_event(&mut self);
}

/// 非覆写辅助 (原版 perfetto-logger.h 顶部两个公有方法)
pub mod perfetto_logger_skel {
  /// 登记既有代码补发 (原版 LogExistingCode)
  pub fn log_existing_code() {
    unimplemented!("骨架")
  }
  /// 触发存量代码日志 (原版 TriggerExistingCodeLogging)
  pub fn trigger_existing_code_logging() {
    unimplemented!("骨架")
  }

  /// 事件方法总数对数守卫 (含重载)
  pub const EVENT_METHOD_COUNT: usize = 25;
}
