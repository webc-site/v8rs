// 由 tool/src/heap/memory-reducer.h.js 从 v8/src/heap/memory-reducer.{h,cc} 生成; 修订就地改, 复跑覆盖
// 内存回收器状态机枚举

/// 状态机状态 (对应 enum Id)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Id {
  /// kUninit
  Uninit = 0,
  /// kDone
  Done = 1,
  /// kWait
  Wait = 2,
  /// kRun
  Run = 3,
}

/// 触发事件 (对应 enum EventType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventType {
  /// kTimer
  Timer = 0,
  /// kMarkCompact
  MarkCompact = 1,
  /// kPossibleGarbage
  PossibleGarbage = 2,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(Id::Uninit as u8 == 0);
  assert!(Id::Run as u8 == 3);
  assert!(EventType::Timer as u8 == 0);
  assert!(EventType::PossibleGarbage as u8 == 2);
};
