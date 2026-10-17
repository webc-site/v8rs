// 由 tool/src/sandbox/sandboxable-thread.h.js 从 v8/src/sandbox/sandboxable-thread.{h,cc} 生成; 修订就地改, 复跑覆盖
// SandboxableThread 骨架

/// 原 class SandboxableThread : public v8::base::Thread
/// 运行 JS/WASM 的线程必须沙箱化: 线程启动前做硬件沙箱初始化。
/// - 原签名: explicit SandboxableThread(const Options&)
/// - 原签名: void Dispatch() override  (per-thread 硬件沙箱初始化后转 Run())
/// cc 侧 Dispatch 调用的平台初始化入口: `SandboxHardwareSupport::EnableForCurrentThread`
pub struct SandboxableThread;

// TODO(dev): 以线程 trait 补 Dispatch→初始化→Run 流程。

// —— 提取不变式断言 ——
const _: () = {};
