// 由 tool/src/wasm/function-compiler.h.js 从 v8/src/wasm/function-compiler.h 生成; 修订就地改, 复跑覆盖
// Wasm 函数编译结果类别与校验标记

/// 校验标记 (原版 base::StrongAlias<ValidationFlagTag, bool>)
pub type Validation = bool;

/// 输入已通过校验 (原版 kAlreadyValidated)
pub const ALREADY_VALIDATED: Validation = false;

/// 输入需要校验 (原版 kMustValidate)
pub const MUST_VALIDATE: Validation = true;

/// 编译结果类别 (对应原版 WasmCompilationResult::Kind)
/// V8_ENABLE_DRUMBRAKE 关闭时为 3 项; 开启时 kInterpreter 亦有效
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum CompilationResultKind {
  /// kFunction — 普通 wasm 函数
  Function = 0,
  /// kWasmToJsWrapper — wasm→JS 包装器
  WasmToJsWrapper = 1,
  /// kStackEntryWrapper — 栈入口包装器
  StackEntryWrapper = 2,
  // kInterpreterEntry = 3, // 解释器入口, 仅 V8_ENABLE_DRUMBRAKE 有效
}

impl CompilationResultKind {
  /// DRUMBRAKE 开启时 kInterpreter 的取值 (原版枚举顺序位置)
  pub const INTERPRETER_ENTRY_REPR: i8 = 3;
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(MUST_VALIDATE);
  assert!(!ALREADY_VALIDATED);
  assert!(CompilationResultKind::Function as i8 == 0);
  assert!(CompilationResultKind::StackEntryWrapper as i8 == 2);
};
