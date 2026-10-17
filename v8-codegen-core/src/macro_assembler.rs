// 由 tool/src/codegen/macro-assembler.h.js 从 v8/src/codegen/macro-assembler.h;src/codegen/macro-assembler-base.{h,cc};src/codegen/macro-assembler-inl.h 生成; 修订就地改, 复跑覆盖
// 宏汇编枚举/位标志/常量 + 平台派发登记 + 签名骨架

// macro-assembler.h 平台派发: x64 → macro-assembler-x64.h (v8-codegen-x64),
// arm64 → macro-assembler-arm64.h (v8-codegen-arm64);
// ia32/arm/ppc/mips64/loong64/s390/riscv 登记跳过。

/// 原版 enum class InvokeType: 函数调用发起方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InvokeType {
  kCall = 0,
  kJump = 1,
}

/// 原版 enum AllocationFlags: AllocateInNewSpace 位标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AllocationFlags {
  NO_ALLOCATION_FLAGS = 0,
  /// 结果寄存器已含 new space allocation top
  RESULT_CONTAINS_TOP = 1 << 0,
  /// 请求大小以字而非字节计
  SIZE_IN_WORDS = 1 << 1,
  /// 按 kDoubleSize 对齐
  DOUBLE_ALIGNMENT = 1 << 2,
  /// 直接分配到老生代
  PRETENURE = 1 << 3,
}

/// 原版 enum class JumpMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JumpMode {
  /// 直接跳转到给定地址
  kJump = 0,
  /// 压入地址作为返回地址后 ret
  kPushAndReturn = 1,
}

/// 原版 enum class SmiCheck
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SmiCheck {
  kOmit = 0,
  kInline = 1,
}

/// 原版 enum class ReadOnlyCheck
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ReadOnlyCheck {
  kOmit = 0,
  kInline = 1,
}

/// 原版 enum class ComparisonMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ComparisonMode {
  /// 压缩指针开启且根为 tagged 时可用 32 位比较
  kDefault = 0,
  /// 比较值可能不在主压缩笼内
  kFullPointer = 1,
}

/// 原版 enum class SetIsolateDataSlots
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SetIsolateDataSlots {
  kNo = 0,
  kYes = 1,
}

/// 原版 enum class ArgumentAdaptionMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArgumentAdaptionMode {
  kAdapt = 0,
  kDontAdapt = 1,
}

/// 原版 enum class RecordWriteCallMode (macro-assembler-base.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RecordWriteCallMode {
  kDefault = 0,
  kWasm = 1,
}

/// C 调用最大参数数 (原版 kMaxCParameters; 非模拟器构建 = 256, 模拟器 = 20)
pub const K_MAX_C_PARAMETERS: i32 = 256;
/// 栈扩页触页哨兵 (原版 kStackPageSize, V8_TARGET_OS_WIN/MACOS 分支) = 4KB
pub const K_STACK_PAGE_SIZE: i32 = 4096;

/// 原版 MacroAssemblerBase: 平台宏汇编公共基类 (状态面骨架)
pub struct MacroAssemblerBase {
  /// 原版 isolate_
  pub isolate_: *mut (),
  /// 原版 code_object_
  pub code_object_: u32,
  /// 原版 root_array_available_
  pub root_array_available_: bool, // = true
  /// 原版 hard_abort_
  pub hard_abort_: bool, // = false
  /// 原版 builtin_jump_table_entries_ 等其余成员待 Developer 接线
}

impl Default for MacroAssemblerBase {
  fn default() -> Self {
    Self {
      isolate_: std::ptr::null_mut(),
      code_object_: 0,
      root_array_available_: true,
      hard_abort_: false,
    }
  }
}

impl MacroAssemblerBase {
  /// 原版 CanBeImmediate(RootIndex) (macro-assembler-base.h:109):
  /// `V8_STATIC_ROOTS_BOOL && RootsTable::IsReadOnly(index)`
  #[must_use]
  pub const fn can_be_immediate(_index: u32) -> bool {
    false
  }

  /// 原版 CommentForOffHeapTrampoline(prefix, builtin) (macro-assembler-base.h:139-145):
  /// `if (!v8_flags.code_comments) return "";` 基线 flags.code_comments = false
  /// (台账注记: RecordComment / --code-comments 基线关闭), 恒返回空串;
  /// flag 接线后需补 "Inlined  Trampoline for {prefix} to {Builtins::name(builtin)}"。
  #[must_use]
  pub fn comment_for_off_heap_trampoline(_prefix: &str, _builtin: u32) -> String {
    String::new()
  }

  /// 原版 MacroAssembler::Abort(reason) — 各架构 macro-assembler-{x64,arm64}.cc
  /// 记录停机原因并置位 hard_abort_
  pub fn abort(&mut self, _reason: crate::bailout::AbortReason) {
    self.hard_abort_ = true;
  }

  /// 原版 MacroAssembler::Assert(cond, reason) — 架构断言指令发射
  pub fn assert_(&mut self, cond: bool, reason: crate::bailout::AbortReason) {
    if !cond {
      self.abort(reason);
    }
  }

  /// 原版 MacroAssembler::Check(cond, reason) — 架构检查跳转发射
  pub fn check(&mut self, cond: bool, reason: crate::bailout::AbortReason) {
    if !cond {
      self.abort(reason);
    }
  }
}

/// 原版 class FrameScope: 帧构造/析构 RAII (骨架)
pub struct FrameScope;

/// 原版 class HardAbortScope: RAII 开关 hard_abort_ (骨架)
pub struct HardAbortScope;

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_MAX_C_PARAMETERS == 256);
  assert!(K_STACK_PAGE_SIZE == 4096);
  assert!(InvokeType::kCall as u8 == 0);
  assert!(JumpMode::kPushAndReturn as u8 == 1);
  assert!(AllocationFlags::NO_ALLOCATION_FLAGS as u8 == 0);
  assert!(AllocationFlags::PRETENURE as u8 == 0b1000);
  assert!(ComparisonMode::kFullPointer as u8 == 1);
  assert!(RecordWriteCallMode::kWasm as u8 == 1);
};
