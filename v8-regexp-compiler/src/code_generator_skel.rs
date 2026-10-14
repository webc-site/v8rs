// 由 tool/src/regexp/regexp-code-generator.h.js 从 v8/src/regexp/regexp-code-generator.{h,cc} 生成; 修订就地改, 复跑覆盖
// 字节码→机器码生成器结果类别与入口骨架

/// 生成器错误类别 (对应原版 CodeGenerator::Error)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CodeGenError {
  /// kNone
  None = 0,
  /// kUnsupportedBytecode
  UnsupportedBytecode = 1,
}

/// 字节码编译结果 (对应原版 CodeGenerator::Result; code 句柄语义待 Developer 接线)
pub struct CodeGenResult {
  pub error: CodeGenError,
}

impl CodeGenResult {
  /// 原版 Result::UnsupportedBytecode()
  pub fn unsupported_bytecode() -> Self {
    Self { error: CodeGenError::UnsupportedBytecode }
  }

  /// 原版 Succeeded()
  pub fn succeeded(&self) -> bool {
    self.error == CodeGenError::None
  }
}

/// 字节码编译器 (对应原版 CodeGenerator; visit 结构登记)
/// 原版流程: PreVisitBytecodes(为前跳分配 label) → VisitBytecodes → Visit<bc>
pub struct CodeGenerator;

impl CodeGenerator {
  /// 原版 Assemble(RegExpData*, Flags) -> Result
  pub fn assemble(&mut self) -> CodeGenResult {
    unimplemented!("登记骨架: 实现见 v8/src/regexp/regexp-code-generator.cc")
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CodeGenError::None as u8 == 0);
};
