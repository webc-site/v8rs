// 硬件支持测试: 对应 sandbox-hardware-support-unittest.cc 的平台无关面
// (PKU 仅 linux x86_64; macOS 侧验证 inactive 退化语义)

use aok::{OK, Void};
use v8_sandbox::{
  AllowSandboxAccess, CodeSandboxingMode, DisallowSandboxAccess, EnterSandboxScope,
  ExitSandboxScope, SandboxHardwareSupport, enter_sandbox, exit_sandbox,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn inactive_platform_semantics() -> Void {
  // 非 linux x86_64 (如 macOS): 未激活
  if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
    return OK; // 有 PKU 平台的面在第二梯队硬件集成时覆盖
  }
  assert!(!SandboxHardwareSupport::try_activate_before_thread_creation());
  assert!(!SandboxHardwareSupport::is_active());
  assert!(!SandboxHardwareSupport::is_strict());
  assert_eq!(SandboxHardwareSupport::sandbox_pkey(), -1);
  assert_eq!(
    SandboxHardwareSupport::sandboxed_mode_pkey_mask(),
    0,
    "未激活掩码为 0"
  );
  // Enter/Exit 为 no-op 成功
  enter_sandbox().expect("enter");
  exit_sandbox().expect("exit");
  // 模式判定: 未激活恒 Unsandboxed / 谓词恒 true
  assert_eq!(
    SandboxHardwareSupport::current_sandboxing_mode(),
    CodeSandboxingMode::Unsandboxed
  );
  assert!(SandboxHardwareSupport::current_sandboxing_mode_is(
    CodeSandboxingMode::Sandboxed
  ));
  // 作用域构造/析构 no-op (LIFO 保障)
  {
    let _disallow = DisallowSandboxAccess::new("test");
    let _allow = AllowSandboxAccess::new("test");
    let _enter = EnterSandboxScope::new().expect("enter scope");
    let _exit = ExitSandboxScope::new().expect("exit scope");
  }
  OK
}

/// 内核信号投递查询跨平台特性 (全平台独立覆盖, 消除 x86_64 盲区)
#[test]
fn kernel_supports_signal_delivery() -> Void {
  #[cfg(target_os = "linux")]
  assert!(SandboxHardwareSupport::kernel_supports_signal_delivery_in_sandbox().is_ok());

  #[cfg(not(target_os = "linux"))]
  assert!(SandboxHardwareSupport::kernel_supports_signal_delivery_in_sandbox().is_err());

  OK
}
