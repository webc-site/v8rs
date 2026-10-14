// 由 tool/src/sandbox/testing.h.js 从 v8/src/sandbox/testing.{h,cc} 生成; 修订就地改, 复跑覆盖
// SandboxTesting 枚举与骨架

// 语义面登记跳过: 崩溃过滤器 (SIGSEGV/SIGTRAP 分诊)、内存破坏 API 安装、
// 安全内存区注册表、InstanceType/FieldOffset 映射构建 (cc ~1766 行)。

/// 沙箱测试模式 (原 enum class Mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Mode {
  /// 未激活
  Disabled = 0,
  /// 测试模式: 启用崩溃过滤 + 内存破坏 API; 无害崩溃以 0 退出
  ForTesting = 1,
  /// 模糊模式: 无害崩溃以非零退出, 供 fuzzer 识别提前终止
  ForFuzzing = 2,
}

/// 内存访问类型 (原 enum class MemoryAccessType, base::Flags 位集)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MemoryAccessType {
  Read = 1 << 0,
  Write = 1 << 1,
  Execute = 1 << 2,
}

/// 位集别名 (原 base::Flags<MemoryAccessType>)
pub type MemoryAccessTypes = u8;

/// 读写访问均安全 (原 kReadAndWriteAccessIsSafe)
pub const READ_AND_WRITE_ACCESS_IS_SAFE: MemoryAccessTypes = (MemoryAccessType::Read | MemoryAccessType::Write) as u8;
/// 仅读访问安全 (原 kOnlyReadAccessIsSafe)
pub const ONLY_READ_ACCESS_IS_SAFE: MemoryAccessTypes = MemoryAccessType::Read as u8;

/// 原 class SandboxTesting : AllStatic (全静态接口)
pub struct SandboxTesting;

impl SandboxTesting {
  // 原签名: static void Enable(Mode mode)
  // 原签名: static void Disable()
  // 原签名: static bool IsEnabled() { return mode_ != Mode::kDisabled; }
  // 原签名: static void InstallMemoryCorruptionApi(Isolate* isolate)
  // 原签名: static void RegisterSafeMemoryRegion(Address start)
  // 原签名: static void UnregisterSafeMemoryRegion(Address start)
  // 原签名: static InstanceTypeMap& GetInstanceTypeMap()
  // 原签名: static FieldOffsetMap& GetFieldOffsetMap()
  // 原签名: static std::optional<int> GetFieldOffset(instance_type, field_name)
  // 原签名: static Mode mode()
  // TODO(dev): 依语义面补实现。
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(READ_AND_WRITE_ACCESS_IS_SAFE == 0b011);
  assert!(ONLY_READ_ACCESS_IS_SAFE == 0b001);
  assert!(Mode::Disabled as u8 == 0);
};
