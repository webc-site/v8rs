// 由 tool/src/logging/runtime-call-stats.h.js 从 v8/src/logging/runtime-call-stats.{h,cc} + runtime-call-stats-scope.h 生成; 修订就地改, 复跑覆盖
// 运行时调用统计: 枚举结构与作用域协议

/// RCS 线程类别 (原版 enum ThreadType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ThreadType {
  MainIsolateThread = 0,
  WorkerThread = 1,
}

/// 计数器模式 (原版 enum CounterMode): 精确单计数器 vs 主/工作线程分立
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CounterMode {
  Exact = 0,
  ThreadSpecific = 1,
}

/// RuntimeCallCounterId 枚举段序 (原版展开次序; 完整名表见 rcs_names.rs)
pub mod rcs_enum_structure {
  /// 五段 X-macro 来源 (对数: 7)
  pub const SECTIONS: [&str; 7] = ["FOR_EACH_GC_COUNTER", "FOR_EACH_MANUAL_COUNTER", "FOR_EACH_INTRINSIC", "BUILTIN_LIST_C", "FOR_EACH_API_COUNTER", "FOR_EACH_HANDLER_COUNTER", "FOR_EACH_THREAD_SPECIFIC_COUNTER"];
  /// 尾部哨兵 (原版 kNumberOfCounters)
  pub const NUMBER_OF_COUNTERS_SENTINEL: &str = "kNumberOfCounters";
  /// 开关: V8_RUNTIME_CALL_STATS (x64/arm64 常规构建关闭)
  pub const ENABLED: bool = false;
}

/// RCS 作用域协议骨架 (语义见手写 runtime_call_stats.rs)
pub mod rcs_scope_skel {
  /// 进入计数 (原版 RuntimeCallScope: 挂到当前线程栈顶, 暂停前任)
  pub fn enter(_counter_id: usize) {
    unimplemented!("骨架")
  }
  /// 离开计数 (原版 ~RuntimeCallScope: 累加用时, 恢复前任)
  pub fn leave() {
    unimplemented!("骨架")
  }
  /// 计时器栈深度 (原版 current_timer 栈)
  pub fn current_timer() -> usize {
    unimplemented!("骨架")
  }
}

/// RuntimeCallCounter 记录单元骨架 (原版 name_/count_/time_)
#[derive(Debug, Clone, Copy, Default)]
pub struct RuntimeCallCounterSkel {
  pub count: i64,
  /// 微秒累计 (原版 time_ 以微秒存)
  pub time_us: i64,
}
