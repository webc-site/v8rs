// 由 tool/src/wasm/signature-hashing.h.js 从 v8/src/wasm/signature-hashing.h 生成; 修订就地改, 复跑覆盖
// 签名哈希 (sandbox 加固) 位域常量 + 计数结构
// 深水推迟 (readme/zh.md 台账): Hash(sig)/IterateSignatureImpl 依赖调用约定槽位模拟
// (codegen Signature/LinkageLocation 完整面), 骨架不入挂载面。

/// 沙箱加固签名哈希: 参数/返回按 tagged-ness 分类计数后编码进 64 位。
/// (对应 class SignatureHasher; V8_ENABLE_SANDBOX 分支, x64/arm64 默认开启)
pub struct SignatureHasher;

impl SignatureHasher {
  /// 位域宽度 (原版 private static constexpr int)
  pub const UNTAGGED_IN_REG_BITS: i32 = 3;
  pub const TAGGED_IN_REG_BITS: i32 = 3;
  pub const UNTAGGED_ON_STACK_BITS: i32 = 11;
  pub const TAGGED_ON_STACK_BITS: i32 = 10;

  /// kTotalWidth = TaggedOnStack::kLastUsedBit + 1
  pub const TOTAL_WIDTH: i32 = 27;

  /// 单侧 (params 或 rets) 编码: 低 3 位 untagged_in_reg, 再 3 位 tagged_in_reg,
  /// 再 11 位 untagged_on_stack, 再 10 位 tagged_on_stack
  #[must_use]
  pub const fn encode_counts(counts: Counts) -> u64 {
    (counts.untagged_in_reg_ as u64)
      | ((counts.tagged_in_reg_ as u64) << 3)
      | ((counts.untagged_on_stack_ as u64) << 6)
      | ((counts.tagged_on_stack_ as u64) << 17)
  }

  /// 完整哈希: (rets << kTotalWidth) | params
  #[must_use]
  pub const fn get_hash(params: Counts, rets: Counts) -> u64 {
    (Self::encode_counts(rets) << 27) | Self::encode_counts(params)
  }
}

/// 寄存器/栈上 tagged-untagged 计数 (对应 struct SignatureHasher::Counts)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
  pub tagged_in_reg_: i32,
  pub untagged_in_reg_: i32,
  pub tagged_on_stack_: i32,
  pub untagged_on_stack_: i32,
}

/// 全零计数 (const 可用; Default::default 非 const)
pub const COUNTS_ZERO: Counts = Counts {
  tagged_in_reg_: 0,
  untagged_in_reg_: 0,
  tagged_on_stack_: 0,
  untagged_on_stack_: 0,
};

// —— 提取不变式断言 ——
const _: () = {
  assert!(SignatureHasher::TOTAL_WIDTH == 27);
  assert!(SignatureHasher::TOTAL_WIDTH * 2 <= 64);
  assert!(SignatureHasher::TOTAL_WIDTH * 2 < 64);
  assert!(
    Counts {
      tagged_in_reg_: 1,
      untagged_in_reg_: 2,
      tagged_on_stack_: 0,
      untagged_on_stack_: 0
    }
    .untagged_in_reg_
      == 2
  );
  // 编码冒烟: 全零计数哈希为零
  assert!(SignatureHasher::get_hash(COUNTS_ZERO, COUNTS_ZERO) == 0);
  assert!(
    SignatureHasher::encode_counts(Counts {
      tagged_in_reg_: 0,
      untagged_in_reg_: 1,
      tagged_on_stack_: 0,
      untagged_on_stack_: 0
    }) == 1
  );
};

// 非 sandbox 变体登记: 原版 #else 分支 Hash(sig) 恒返回 0。
