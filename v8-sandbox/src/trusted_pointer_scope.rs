// 由 tool/src/sandbox/trusted-pointer-scope.h.js 从 v8/src/sandbox/trusted-pointer-scope.{h,cc} 生成; 修订就地改, 复跑覆盖
// TrustedPointerPublishingScope 骨架

/// 发布状态机 (原 enum class State : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
  /// kInProgress
  InProgress = 0,
  /// kSuccess
  Success = 1,
  /// kFailure
  Failure = 2,
}

/// 指针追踪存储形态 (原 enum class Storage : uint8_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Storage {
  /// kEmpty
  Empty = 0,
  /// kSingleton
  Singleton = 1,
  /// kVector
  Vector = 2,
}

/// 原 class TrustedPointerPublishingScope (沙箱开启分支)
/// - MarkSuccess/MarkFailure 置状态
/// - TrackPointer(TrustedPointerTableEntry*) 登记待发布指针
/// 沙箱关闭分支: MarkSuccess/MarkFailure 为空实现。
pub struct TrustedPointerPublishingScope {
  pub state: State,
  pub storage: Storage,
}

impl TrustedPointerPublishingScope {
  // 原签名: void MarkSuccess() { state_ = State::kSuccess; }
  // 原签名: void MarkFailure() { state_ = State::kFailure; }
  // 原签名: void TrackPointer(TrustedPointerTableEntry* entry);
  // TODO(dev): 补 TrackPointer 存储 (单例/向量) 与析构回滚。
}

/// 原 class DisableTrustedPointerPublishingScope (开关面, 双分支各一份)
pub struct DisableTrustedPointerPublishingScope;

// —— 提取不变式断言 ——
const _: () = {
  assert!(State::InProgress as u8 == 0);
  assert!(Storage::Empty as u8 == 0);
};
