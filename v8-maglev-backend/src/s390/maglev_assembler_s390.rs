// 由 tool/src/maglev/s390/maglev-assembler-s390-inl.h.js 从 v8/src/maglev/s390/maglev-assembler-s390-inl.h + src/maglev/s390/maglev-assembler-s390.cc 生成; 修订就地改, 复跑覆盖
// S390 MaglevAssembler 跳过登记 (非目标架构)

//! 规则九: 平台分支按 x64+arm64 展开, S390 跳过语义转换, 仅登记结构面。
//! 文件齐备、锚点全过; 启用该架构时以脚本对数为底稿展开签名骨架。

pub static REGISTERED_FILES: &[(&str, usize, usize)] = &[
  ("src/maglev/s390/maglev-assembler-s390-inl.h", 1625, 178),
  ("src/maglev/s390/maglev-assembler-s390.cc", 607, 15),
];

const _: () = {
  assert!(REGISTERED_FILES.len() == 2);
  assert!(REGISTERED_FILES[0].1 == 1625);
  assert!(REGISTERED_FILES[1].1 == 607);
};
