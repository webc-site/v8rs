#!/usr/bin/env bun
// tool/src/wasm/signature-hashing.h.js
// 机械提取 v8/src/wasm/signature-hashing.h: 签名哈希位域常量与计数结构。
// V8_ENABLE_SANDBOX 按 x64/arm64 默认开启展开; 非 sandbox 变体登记注释。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/signature-hashing.h");

const bits = {};
for (const [, k, v] of hSrc.matchAll(/static constexpr int k(\w+Bits) = (\d+);/g)) {
  bits[k] = Number(v);
}
// kUntaggedInRegBits / kTaggedInRegBits / kUntaggedOnStackBits / kTaggedOnStackBits
for (const k of ["UntaggedInRegBits", "TaggedInRegBits", "UntaggedOnStackBits", "TaggedOnStackBits"]) {
  if (!(k in bits)) throw new Error(`位宽常量缺失: k${k}`);
}
const totalWidth = bits.UntaggedInRegBits + bits.TaggedInRegBits + bits.UntaggedOnStackBits + bits.TaggedOnStackBits;
if (!hSrc.includes("kTotalWidth * 2 <= 64")) throw new Error("static_assert kTotalWidth*2<=64 缺失");
if (!hSrc.includes("kTotalWidth * 2 < 64")) throw new Error("static_assert kTotalWidth*2<64 缺失");
if (!hSrc.includes("class SignatureHasher")) throw new Error("未找到 SignatureHasher");
if (!hSrc.includes("IterateSignatureImpl")) throw new Error("未找到 IterateSignatureImpl");

console.log(`位宽: ${JSON.stringify(bits)}, kTotalWidth=${totalWidth}`);

let rust = banner(
  "src/wasm/signature-hashing.h.js",
  "src/wasm/signature-hashing.h",
  "签名哈希 (sandbox 加固) 位域常量 + 计数结构 + 骨架",
);

rust += `
/// 沙箱加固签名哈希: 参数/返回按 tagged-ness 分类计数后编码进 64 位。
/// (对应 class SignatureHasher; V8_ENABLE_SANDBOX 分支, x64/arm64 默认开启)
pub struct SignatureHasher;

impl SignatureHasher {
  /// 位域宽度 (原版 private static constexpr int)
  pub const UNTAGGED_IN_REG_BITS: i32 = ${bits.UntaggedInRegBits};
  pub const TAGGED_IN_REG_BITS: i32 = ${bits.TaggedInRegBits};
  pub const UNTAGGED_ON_STACK_BITS: i32 = ${bits.UntaggedOnStackBits};
  pub const TAGGED_ON_STACK_BITS: i32 = ${bits.TaggedOnStackBits};

  /// kTotalWidth = TaggedOnStack::kLastUsedBit + 1
  pub const TOTAL_WIDTH: i32 = ${totalWidth};

  /// 单侧 (params 或 rets) 编码: 低 3 位 untagged_in_reg, 再 3 位 tagged_in_reg,
  /// 再 11 位 untagged_on_stack, 再 10 位 tagged_on_stack
  #[must_use]
  pub const fn encode_counts(counts: Counts) -> u64 {
    (counts.untagged_in_reg_ as u64)
      | ((counts.tagged_in_reg_ as u64) << ${bits.UntaggedInRegBits})
      | ((counts.untagged_on_stack_ as u64) << ${bits.UntaggedInRegBits + bits.TaggedInRegBits})
      | ((counts.tagged_on_stack_ as u64) << ${bits.UntaggedInRegBits + bits.TaggedInRegBits + bits.UntaggedOnStackBits})
  }

  /// 完整哈希: (rets << kTotalWidth) | params
  #[must_use]
  pub const fn get_hash(params: Counts, rets: Counts) -> u64 {
    (Self::encode_counts(rets) << ${totalWidth}) | Self::encode_counts(params)
  }

  /// 对应 SignatureHasher::Hash(sig) (骨架: 位置模拟待 Developer 修订)
  pub fn hash(_sig: &crate::wasm_opcodes::SigDef) -> u64 {
    unimplemented!("骨架: 参照原版 Hash→IterateSignatureImpl 模拟修订")
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

/// 签名位置遍历 (对应 IterateSignatureImpl, 骨架: tagged/untagged 分组次序见原版注释)
pub fn iterate_signature_impl(
  _sig: &crate::wasm_opcodes::SigDef,
  _extra_callable_param: bool,
  _locations: &mut dyn FnMut(usize, crate::wasm_linkage::LinkageLocation),
) {
  unimplemented!("骨架: 参照原版 IterateSignatureImpl 修订 (instance@0 → untagged → EndSlotArea → tagged → callable)")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SignatureHasher::TOTAL_WIDTH == ${totalWidth});
  assert!(SignatureHasher::TOTAL_WIDTH * 2 <= 64);
  assert!(SignatureHasher::TOTAL_WIDTH * 2 < 64);
  assert!(Counts { tagged_in_reg_: 1, untagged_in_reg_: 2, tagged_on_stack_: 0, untagged_on_stack_: 0 }.untagged_in_reg_ == 2);
  // 编码冒烟: 全零计数哈希为零
  assert!(SignatureHasher::get_hash(Counts::default(), Counts::default()) == 0);
  assert!(SignatureHasher::encode_counts(Counts { tagged_in_reg_: 0, untagged_in_reg_: 1, tagged_on_stack_: 0, untagged_on_stack_: 0 }) == 1);
};

// 非 sandbox 变体登记: 原版 #else 分支 Hash(sig) 恒返回 0。
`;
;

const p = writeSrc("v8-wasm-core", "signature_hashing.rs", rust);
console.log(`written ${p}`);
