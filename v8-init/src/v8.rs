// 由 tool/src/init/v8.js 从 v8/src/init/v8.{h,cc} 生成; 修订就地改, 复跑覆盖
// V8 全局初始化门面与启动状态机

/// V8 进程级启动/关闭状态机 (原版 v8.cc 匿名 namespace 内 enum class V8StartupState)
/// 状态严格按序推进: 每次 AdvanceStartupState 只允许 +1 步, 乱序 FATAL。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum V8StartupState {
  /// kIdle
  Idle = 0,
  /// kPlatformInitializing
  PlatformInitializing = 1,
  /// kPlatformInitialized
  PlatformInitialized = 2,
  /// kV8Initializing
  V8Initializing = 3,
  /// kV8Initialized
  V8Initialized = 4,
  /// kV8Disposing
  V8Disposing = 5,
  /// kV8Disposed
  V8Disposed = 6,
  /// kPlatformDisposing
  PlatformDisposing = 7,
  /// kPlatformDisposed
  PlatformDisposed = 8,
}

impl V8StartupState {
  /// 状态总数 (原版共 9 态)
  pub const COUNT: usize = 9;

  /// 按序单步推进到下一个状态
  pub const fn next(self) -> Option<Self> {
    match self {
      Self::Idle => Some(Self::PlatformInitializing),
      Self::PlatformInitializing => Some(Self::PlatformInitialized),
      Self::PlatformInitialized => Some(Self::V8Initializing),
      Self::V8Initializing => Some(Self::V8Initialized),
      Self::V8Initialized => Some(Self::V8Disposing),
      Self::V8Disposing => Some(Self::V8Disposed),
      Self::V8Disposed => Some(Self::PlatformDisposing),
      Self::PlatformDisposing => Some(Self::PlatformDisposed),
      Self::PlatformDisposed => None,
    }
  }

  /// 验证状态推进是否合法 (必须严格 +1 步)
  pub const fn can_transition_to(self, target: Self) -> bool {
    match self.next() {
      Some(expected) => expected as u8 == target as u8,
      None => false,
    }
  }
}

/// V8 全局门面 (原版 class V8 : AllStatic, 全静态方法)
/// 签名骨架: 语义实现由 Developer 就地修订
pub struct V8;

impl V8 {
  // 原版签名: static void Initialize();
  // 原版签名: static void Dispose();
  // 原版签名: [[noreturn]] V8_EXPORT_PRIVATE static void FatalProcessOutOfMemory( Isolate* isolate, const char* location, const OOMDetails& details = kNoOOMDetails);
  // 原版签名: static void InitializePlatform(v8::Platform* platform);
  // 原版签名: V8_EXPORT_PRIVATE static void InitializePlatformForTesting( v8::Platform* platform);
  // 原版签名: static void DisposePlatform();
  // 原版签名: V8_EXPORT_PRIVATE static void GetCurrentPlatform();
  // 原版签名: V8_EXPORT_PRIVATE static void SetPlatformForTesting(v8::Platform* platform);
  // 原版签名: static void SetSnapshotBlob(StartupData* snapshot_blob);
  // (骨架占位: 具体方法由 Developer 按上列原版签名补齐)

  // OOM 详情常量 (原版 v8.cc 定义, 类型为 v8::OOMDetails, 由 Developer 落型):
  // V8::kNoOOMDetails 初始化式 { false, nullptr }
  // V8::kHeapOOM 初始化式 { true, nullptr }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(V8StartupState::COUNT == 9);
  assert!(V8StartupState::Idle as u8 == 0);
  assert!(V8StartupState::PlatformDisposed as u8 == 8);
};
