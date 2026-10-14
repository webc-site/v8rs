// 由 tool/src/baseline/baseline.h.js 从 v8/src/baseline/baseline.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 入口三函数 (原版自由函数, 机械登记; 语义属 macro-assembler 族)
pub const ENTRY_SIGNATURES: [&str; 3] = [
  "bool CanCompileWithBaseline(Isolate* isolate, Tagged<SharedFunctionInfo> shared);",
  "MaybeDirectHandle<Code> GenerateBaselineCode(Isolate* isolate, Handle<SharedFunctionInfo> shared);",
  "void EmitReturnBaseline(MacroAssembler* masm);",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(ENTRY_SIGNATURES.len() == 3);
};

use crate::{BaselineCompiler, CompiledBaselineCode, Error};

/// 检查函数是否可以通过 Baseline 编译 (需要包含有效字节码且非仅解释执行约束)
#[inline]
#[must_use]
pub const fn can_compile_with_baseline(has_bytecode: bool, is_interpreted: bool) -> bool {
  has_bytecode && is_interpreted
}

/// 对给定的字节码生成 Baseline 机器码
#[inline]
pub fn generate_baseline_code(bytecode: &[u8]) -> Result<CompiledBaselineCode, Error> {
  let mut compiler = BaselineCompiler::new(bytecode);
  compiler.generate_code()
}
