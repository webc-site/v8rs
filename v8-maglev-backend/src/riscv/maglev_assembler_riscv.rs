// 由 tool/src/maglev/riscv/maglev-assembler-riscv-inl.h.js 从 v8/src/maglev/riscv/maglev-assembler-riscv-inl.h + src/maglev/riscv/maglev-assembler-riscv.cc 生成; 修订就地改, 复跑覆盖
// RISCV64 MaglevAssembler 跳过登记 (非目标架构)

//! 规则九: 平台分支按 x64+arm64 展开, RISCV64 跳过语义转换, 仅登记结构面。
//! 文件齐备、锚点全过; 启用该架构时以脚本对数为底稿展开签名骨架。

pub static REGISTERED_FILES: &[(&str, usize, usize)] = &[
  ("src/maglev/riscv/maglev-assembler-riscv-inl.h", 2245, 194),
  ("src/maglev/riscv/maglev-assembler-riscv.cc", 692, 16),
];

const _: () = {
  assert!(REGISTERED_FILES.len() == 2);
  assert!(REGISTERED_FILES[0].1 == 2245);
  assert!(REGISTERED_FILES[1].1 == 692);
};
