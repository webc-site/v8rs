#!/usr/bin/env bun
// tool/src/codegen/signature.h.js
// 机械提取 v8/src/codegen/signature.h: Signature 布局常量 + 签名骨架
// → v8-codegen-core/src/signature.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/codegen/signature.h");

function must(re, what) {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${what}`);
  return m;
}

// 布局常量对数 (kSizetSize = 8, 64 位目标)
const retOff = must(/static constexpr size_t kReturnCountOffset = (\d+);/, "kReturnCountOffset");
const paramOffM = must(
  /static constexpr size_t kParameterCountOffset =\s*kReturnCountOffset \+ (\w+);/,
  "kParameterCountOffset",
);
const repsOffM = must(
  /static constexpr size_t kRepsOffset = kParameterCountOffset \+ (\w+);/,
  "kRepsOffset",
);
if (paramOffM[1] !== "kSizetSize" || repsOffM[1] !== "kSizetSize")
  throw new Error("布局常量依赖异常");
if (retOff[1] !== "0") throw new Error("kReturnCountOffset 异常");
const K_SIZET_SIZE = 8;

// 方法清点 (对数)
for (const want of ["return_count()", "parameter_count()", "GetParam", "GetReturn", "contains",
  "parameters()", "returns()", "all()"]) {
  if (!src.includes(want)) throw new Error(`缺 Signature 成员 ${want}`);
}
if (!src.includes("class SignatureBuilder")) throw new Error("缺 SignatureBuilder");
if (!src.includes("FixedSizeSignature")) throw new Error("缺 FixedSizeSignature");
if (!src.includes("using MachineSignature = Signature<MachineType>;"))
  throw new Error("缺 MachineSignature 别名");

let rust = banner(
  "src/codegen/signature.h.js",
  "src/codegen/signature.h",
  "Signature 布局常量 + 泛型骨架 (返回+参数连续布局)",
);

rust += `
/// 原版 Signature 布局常量: kReturnCountOffset (64 位目标 kSizetSize = 8)
pub const K_RETURN_COUNT_OFFSET: usize = ${retOff[1]};
/// 原版 kParameterCountOffset = kReturnCountOffset + kSizetSize
pub const K_PARAMETER_COUNT_OFFSET: usize = K_RETURN_COUNT_OFFSET + ${K_SIZET_SIZE};
/// 原版 kRepsOffset = kParameterCountOffset + kSizetSize
pub const K_REPS_OFFSET: usize = K_PARAMETER_COUNT_OFFSET + ${K_SIZET_SIZE};

/// 原版 Signature<T>: 描述函数/调用的输入输出。
/// 布局: [returns..., params...] 连续存放于 reps。
/// 原版为 Zone 分配的尾随数组结构; Rust 侧以 owned Vec 承载 (基线形态)。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Signature<T> {
  /// 原版 return_count_
  return_count_: usize,
  /// 原版 parameter_count_
  parameter_count_: usize,
  /// 原版 reps_ (returns ++ params)
  reps_: Vec<T>,
}

impl<T> Signature<T> {
  /// 原版构造 (constexpr): (return_count, parameter_count, reps)
  #[inline]
  #[must_use]
  pub const fn from_parts(return_count: usize, parameter_count: usize, reps: Vec<T>) -> Self {
    Self { return_count_: return_count, parameter_count_: parameter_count, reps_: reps }
  }

  /// 原版 return_count()
  #[inline]
  #[must_use]
  pub const fn return_count(&self) -> usize {
    self.return_count_
  }

  /// 原版 parameter_count()
  #[inline]
  #[must_use]
  pub const fn parameter_count(&self) -> usize {
    self.parameter_count_
  }

  /// 原版 GetParam(index): reps[return_count + index] (沙箱校验: 越界 panic)
  #[inline]
  #[must_use]
  pub fn get_param(&self, index: usize) -> &T {
    assert!(index < self.parameter_count_, "SBXCHECK: param 越界");
    &self.reps_[self.return_count_ + index]
  }

  /// 原版 GetReturn(index = 0)
  #[inline]
  #[must_use]
  pub fn get_return(&self, index: usize) -> &T {
    assert!(index < self.return_count_, "SBXCHECK: return 越界");
    &self.reps_[index]
  }

  /// 原版 parameters(): 参数切片
  #[inline]
  #[must_use]
  pub fn parameters(&self) -> &[T] {
    &self.reps_[self.return_count_..]
  }

  /// 原版 returns(): 返回值切片
  #[inline]
  #[must_use]
  pub fn returns(&self) -> &[T] {
    &self.reps_[..self.return_count_]
  }

  /// 原版 all(): returns ++ params 全切片
  #[inline]
  #[must_use]
  pub fn all(&self) -> &[T] {
    &self.reps_[..]
  }
}

impl<T: PartialEq> Signature<T> {
  /// 原版 contains(element)
  #[inline]
  #[must_use]
  pub fn contains(&self, element: &T) -> bool {
    self.all().contains(element)
  }
  // 原版 operator== / operator!= 由上方 derive(PartialEq, Eq) 直接提供
}

/// 原版 using MachineSignature = Signature<MachineType>;
/// MachineType 待 Developer 接线 (见 machine-type 覆盖面)。
pub type MachineSignature = Signature<u8>;

/// 原版 hash_value(Signature): 参数计数 + 全表示哈希
/// (哈希算法与 base::Hasher 不同, 仅语义对应; 待 Developer 复核)
#[must_use]
pub fn hash_value<T: std::hash::Hash>(sig: &Signature<T>) -> u64 {
  use std::hash::{Hash, Hasher};
  let mut h = std::collections::hash_map::DefaultHasher::new();
  sig.parameter_count().hash(&mut h);
  Hash::hash_slice(sig.all(), &mut h);
  h.finish()
}

/// 原版 SignatureBuilder: 增量构建 Signature (签名骨架)
pub struct SignatureBuilder<'a, T> {
  return_count_: usize,
  parameter_count_: usize,
  rcursor_: usize,
  pcursor_: usize,
  sig_: &'a mut Signature<T>,
}

impl<'a, T> SignatureBuilder<'a, T> {
  /// 原版构造: (storage, return_count, parameter_count)
  #[must_use]
  pub fn new(storage: &'a mut Signature<T>, return_count: usize, parameter_count: usize) -> Self {
    Self {
      return_count_: return_count,
      parameter_count_: parameter_count,
      rcursor_: 0,
      pcursor_: 0,
      sig_: storage,
    }
  }

  /// 原版 AddReturn
  pub fn add_return(&mut self, _val: T) {
    todo!("待 Developer 修订: 对齐原版 SignatureBuilder::AddReturn")
  }

  /// 原版 AddParam
  pub fn add_param(&mut self, _val: T) {
    todo!("待 Developer 修订: 对齐原版 SignatureBuilder::AddParam")
  }

  /// 原版 Get: 校验游标后交出 Signature
  #[must_use]
  pub fn get(&mut self) -> &Signature<T> {
    todo!("待 Developer 修订: 对齐原版 SignatureBuilder::Get")
  }
}

/// 原版 FixedSizeSignature<T, kNumReturns, kNumParams>: 栈上定长签名 (签名骨架, 泛型常量待 Developer 定型)
pub struct FixedSizeSignature<T, const NUM_RETURNS: usize = 0, const NUM_PARAMS: usize = 0> {
  base: Signature<T>,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_RETURN_COUNT_OFFSET == 0);
  assert!(K_PARAMETER_COUNT_OFFSET == 8);
  assert!(K_REPS_OFFSET == 16);
};
`;

const p = writeSrc("v8-codegen-core", "signature.rs", rust);
console.log(
  `signature: 布局常量 3 / Signature 方法 8 / Builder 3 / 别名 2 → ${p}`,
);
