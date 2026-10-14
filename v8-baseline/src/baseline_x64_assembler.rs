// 由 tool/src/baseline/x64/baseline-assembler-x64-inl.h.js 从 v8/src/baseline/x64/baseline-assembler-x64-inl.h 生成; 修订就地改, 复跑覆盖

/// x64 scratch 寄存器组 (原版 detail::kScratchRegisters; 避开 r10 — 宏汇编器保留)
/// 名单按 v8-codegen-x64 寄存器枚举对齐, 修订阶段接入 Register 类型
pub const SCRATCH_REGISTERS: [&str; 5] = ["r8", "r9", "r11", "r12", "r15"];

/// scratch 寄存器数 (原版 kNumScratchRegisters)
pub const NUM_SCRATCH_REGISTERS: usize = 5;

/// x64 ScratchRegisterScope 特化成员声明 (机械 1:1)
pub const X64_SCRATCH_REGISTER_SCOPE_DECLARATIONS: &[&str] = &[
  "[public/method] explicit ScratchRegisterScope(BaselineAssembler* assembler) : assembler_(assembler), prev_scope_(assembler->scratch_register_scope_), registers_used_(prev_scope_ == nullptr ? 0 : prev_scope_->registers_used_)",
  "[public/method] ~ScratchRegisterScope()",
  "[public/method] Register AcquireScratch()",
  "[private/field] BaselineAssembler* assembler_",
  "[private/field] ScratchRegisterScope* prev_scope_",
  "[private/field] int registers_used_",
];
// —— 提取不变式断言 ——
const _: () = {
  assert!(X64_SCRATCH_REGISTER_SCOPE_DECLARATIONS.len() == 6);
  assert!(SCRATCH_REGISTERS.len() == 5);
  assert!(NUM_SCRATCH_REGISTERS == 5);
};
