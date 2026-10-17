// 由 tool/src/regexp/regexp-macro-assembler-arch.h.js 从 v8/src/regexp/regexp-macro-assembler-arch.h 生成; 修订就地改, 复跑覆盖
// 目标架构宏汇编分发 (x64+arm64 展开; 其余架构登记)

/// 非对齐读取支持 (原版 kUnalignedReadSupported)
/// V8_TARGET_ARCH_X64 (源分支 include src/regexp/x64/regexp-macro-assembler-x64.h)
pub mod x64 {
  pub const UNALIGNED_READ_SUPPORTED: bool = true;
}

/// V8_TARGET_ARCH_ARM64 (源分支 include src/regexp/arm64/regexp-macro-assembler-arm64.h)
pub mod arm64 {
  pub const UNALIGNED_READ_SUPPORTED: bool = true;
}

/// 跳过的架构登记 (硬规则 9): IA32 / ARM / PPC64 / MIPS64 / LOONG64 / S390X — 均登记于源文件, 按需再展开。
/// 语义: 其余分支取值见源; riscv32/64: RISCV_HAS_NO_UNALIGNED 定义时 false, 否则 true。

// —— 提取不变式断言 ——
const _: () = {
  assert!(x64::UNALIGNED_READ_SUPPORTED);
  assert!(arm64::UNALIGNED_READ_SUPPORTED);
};
