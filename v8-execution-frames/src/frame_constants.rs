// 由 tool/src/execution/frame-constants.js 从 v8/src/execution/frame-constants.h, src/execution/pointer-authentication.h, src/execution/pointer-authentication-dummy.h, src/execution/arm64/pointer-auth-arm64.cc, src/execution/arm64/pointer-authentication-arm64.h, src/execution/x64/frame-constants-x64.h, src/execution/x64/frame-constants-x64.cc, src/execution/arm64/frame-constants-arm64.h, src/execution/arm64/frame-constants-arm64.cc, src/execution/arm/frame-constants-arm.h, src/execution/arm/frame-constants-arm.cc, src/execution/ia32/frame-constants-ia32.h, src/execution/ia32/frame-constants-ia32.cc, src/execution/mips64/frame-constants-mips64.h, src/execution/mips64/frame-constants-mips64.cc, src/execution/ppc/frame-constants-ppc.h, src/execution/ppc/frame-constants-ppc.cc, src/execution/s390/frame-constants-s390.h, src/execution/s390/frame-constants-s390.cc, src/execution/riscv/frame-constants-riscv.h, src/execution/riscv/frame-constants-riscv.cc, src/execution/loong64/frame-constants-loong64.h, src/execution/loong64/frame-constants-loong64.cc 生成; 修订就地改, 复跑覆盖
// 通用与平台无关帧常量定义 (64 位架构)

/// 系统指针位宽 (64 位架构恒等于 8 字节)
pub const SYSTEM_POINTER_SIZE: i32 = 8;
pub const SYSTEM_POINTER_SIZE_LOG2: i32 = 3;

/// 通用帧常量 (对应原版 CommonFrameConstants)
pub mod common_frame_constants {
  use super::SYSTEM_POINTER_SIZE;

  /// 调用者帧指针相对于当前帧 FP 的偏移
  pub const CALLER_FP_OFFSET: i32 = 0;
  /// 调用者返回地址 (PC) 相对于当前帧 FP 的偏移
  pub const CALLER_PC_OFFSET: i32 = SYSTEM_POINTER_SIZE;
  /// 调用者栈顶 (SP) 相对于当前帧 FP 的偏移
  pub const CALLER_SP_OFFSET: i32 = 2 * SYSTEM_POINTER_SIZE;

  /// FP 上方固定帧大小 (FP + LR / Return Address，共 16 字节)
  pub const FIXED_FRAME_SIZE_ABOVE_FP: i32 = 2 * SYSTEM_POINTER_SIZE;
  pub const FIXED_SLOT_COUNT_ABOVE_FP: i32 = 2;

  /// 常量池槽大小 (x64 / arm64 无常量池，恒为 0)
  pub const CP_SLOT_SIZE: i32 = 0;

  /// 上下文或帧类型标记槽相对于当前 FP 的偏移 (-8)
  pub const CONTEXT_OR_FRAME_TYPE_OFFSET: i32 = -(CP_SLOT_SIZE + SYSTEM_POINTER_SIZE);
}

/// 标准 JS 帧常量 (对应原版 StandardFrameConstants)
pub mod standard_frame_constants {
  use super::SYSTEM_POINTER_SIZE;

  /// FP 下方固定帧大小
  pub const FIXED_FRAME_SIZE_FROM_FP: i32 = 3 * SYSTEM_POINTER_SIZE;
  pub const FIXED_FRAME_SIZE: i32 =
    super::common_frame_constants::FIXED_FRAME_SIZE_ABOVE_FP + FIXED_FRAME_SIZE_FROM_FP;

  /// 函数闭包槽偏移 (-16)
  pub const FUNCTION_OFFSET: i32 = -2 * SYSTEM_POINTER_SIZE;
  /// 上下文槽偏移 (-8)
  pub const CONTEXT_OFFSET: i32 = -SYSTEM_POINTER_SIZE;
  /// 实参个数槽偏移 (-24)
  pub const ARG_C_OFFSET: i32 = -3 * SYSTEM_POINTER_SIZE;
  /// 表达式栈基址偏移 (-32)
  pub const EXPRESSIONS_OFFSET: i32 = -4 * SYSTEM_POINTER_SIZE;
  /// 首个压入栈的表达式槽偏移 (-32)
  pub const FIRST_PUSHED_FRAME_VALUE_OFFSET: i32 = EXPRESSIONS_OFFSET;
}

/// 类型化帧常量 (对应原版 TypedFrameConstants)
pub mod typed_frame_constants {
  use super::SYSTEM_POINTER_SIZE;

  /// 帧类型标记偏移 (-8)
  pub const FRAME_TYPE_OFFSET: i32 = super::common_frame_constants::CONTEXT_OR_FRAME_TYPE_OFFSET;

  /// 第一个被压入栈的局部变量偏移 (-16)
  pub const FIRST_PUSHED_FRAME_VALUE_OFFSET: i32 = -2 * SYSTEM_POINTER_SIZE;

  /// 计算第 index 个压入值的 FP 相对偏移
  #[inline]
  pub const fn pushed_value_offset(index: i32) -> i32 {
    FIRST_PUSHED_FRAME_VALUE_OFFSET - index * SYSTEM_POINTER_SIZE
  }
}

// ── 编译期静态断言 ──
const _: () = {
  assert!(common_frame_constants::CALLER_FP_OFFSET == 0);
  assert!(common_frame_constants::CALLER_PC_OFFSET == 8);
  assert!(common_frame_constants::CALLER_SP_OFFSET == 16);
  assert!(common_frame_constants::FIXED_FRAME_SIZE_ABOVE_FP == 16);
  assert!(common_frame_constants::CONTEXT_OR_FRAME_TYPE_OFFSET == -8);
  assert!(standard_frame_constants::FIXED_FRAME_SIZE == 40);
  assert!(standard_frame_constants::FUNCTION_OFFSET == -16);
  assert!(standard_frame_constants::ARG_C_OFFSET == -24);
  assert!(standard_frame_constants::EXPRESSIONS_OFFSET == -32);
  assert!(standard_frame_constants::FIRST_PUSHED_FRAME_VALUE_OFFSET == -32);
  assert!(typed_frame_constants::pushed_value_offset(0) == -16);
  assert!(typed_frame_constants::pushed_value_offset(1) == -24);
};
