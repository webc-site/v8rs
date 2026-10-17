// 由 tool/src/sandbox/trap-fuzzer.h.js 从 v8/src/sandbox/trap-fuzzer.{h,cc} 生成; 修订就地改, 复跑覆盖
// TrapHandlerFuzzer 常量与骨架

/// 变异前最大访问次数 (原 kMaxAccessesBeforeMutation)
pub const MAX_ACCESSES_BEFORE_MUTATION: i32 = 10;

/// 采集缓冲大小 (原 kBufferSize)
pub const BUFFER_SIZE: i32 = 1024;

/// 原 class SandboxTrapFuzzer : AllStatic 方法面:
// - Enable
// - Disable
// - IsEnabled
// - RegisterMutationRequest
// - RemoveSandboxPkeyReadAccess
// - RestoreSandboxPkeyReadAccess
// - EnableSingleStepping
// - DisableSingleStepping
// - SafeRead
// - SafeWrite
// - ForwardSignal
// - SigvtalrmHandler
// - SigtrapHandler
// - SigsegvHandler
// - Print
// - PrintNumber
// - PrintHex
// - PrintDec
// - Flush

pub struct SandboxTrapFuzzer;

/// 原 class SandboxTrapFuzzer::SignalSafeLogger (信号安全日志子类)
pub struct SignalSafeLogger;

// TODO(dev): 依 fuzz 循环补采集/变异实现 (cc 语义面登记跳过)。
// 原静态成员: old_segv_action_/old_trap_action_/old_vtalrm_action_ (信号处理器保存槽)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_ACCESSES_BEFORE_MUTATION == 10);
  assert!(BUFFER_SIZE == 1024);
};
