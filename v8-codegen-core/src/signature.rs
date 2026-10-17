// 由 tool/src/codegen/signature.h.js 从 v8/src/codegen/signature.h 生成; 修订就地改, 复跑覆盖
// Signature 布局常量 + 泛型骨架 (返回+参数连续布局)

use std::{
  collections::hash_map::DefaultHasher,
  hash::Hash,
  ops::{Deref, DerefMut},
};

/// 原版 Signature 布局常量: kReturnCountOffset (64 位目标 kSizetSize = 8)
pub const K_RETURN_COUNT_OFFSET: usize = 0;
/// 原版 kParameterCountOffset = kReturnCountOffset + kSizetSize
pub const K_PARAMETER_COUNT_OFFSET: usize = K_RETURN_COUNT_OFFSET + 8;
/// 原版 kRepsOffset = kParameterCountOffset + kSizetSize
pub const K_REPS_OFFSET: usize = K_PARAMETER_COUNT_OFFSET + 8;

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
    Self {
      return_count_: return_count,
      parameter_count_: parameter_count,
      reps_: reps,
    }
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
pub fn hash_value<T: Hash>(sig: &Signature<T>) -> u64 {
  use std::hash::{Hash, Hasher};
  let mut h = DefaultHasher::new();
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
  /// 原版构造: (storage, return_count, parameter_count) — storage 的 reps 作全新 backing array
  #[must_use]
  pub fn new(storage: &'a mut Signature<T>, return_count: usize, parameter_count: usize) -> Self {
    // 原版在 storage 上就地分配 (return_count + parameter_count) 的全新 rep buffer。
    storage.reps_.clear();
    Self {
      return_count_: return_count,
      parameter_count_: parameter_count,
      rcursor_: 0,
      pcursor_: 0,
      sig_: storage,
    }
  }

  /// 原版 signature.h SignatureBuilder::AddReturn: DCHECK_LT(rcursor_, return_count_); buffer_[rcursor_++] = val
  pub fn add_return(&mut self, val: T) {
    debug_assert!(self.rcursor_ < self.return_count_);
    self.sig_.reps_.push(val);
    self.rcursor_ += 1;
  }

  /// 原版 signature.h SignatureBuilder::AddParam: DCHECK_LT(pcursor_, parameter_count_);
  /// buffer_[return_count_ + pcursor_++] = val (reps 布局 returns++params, 顺序 push 即落该位)
  pub fn add_param(&mut self, val: T) {
    debug_assert!(self.pcursor_ < self.parameter_count_);
    self.sig_.reps_.push(val);
    self.pcursor_ += 1;
  }

  /// 原版 signature.h SignatureBuilder::Get: 校验游标满后交出 Signature
  #[must_use]
  pub fn get(&mut self) -> &Signature<T> {
    debug_assert!(self.rcursor_ == self.return_count_);
    debug_assert!(self.pcursor_ == self.parameter_count_);
    self.sig_.return_count_ = self.return_count_;
    self.sig_.parameter_count_ = self.parameter_count_;
    self.sig_
  }
}

/// 原版 FixedSizeSignature<T, kNumReturns, kNumParams>: 栈上定长签名 (基线形态, 泛型常量承载)
pub struct FixedSizeSignature<T, const NUM_RETURNS: usize = 0, const NUM_PARAMS: usize = 0> {
  /// 原版继承自 Signature<T> 的基座
  pub base: Signature<T>,
}

impl<T: Default + Clone, const NUM_RETURNS: usize, const NUM_PARAMS: usize> Default
  for FixedSizeSignature<T, NUM_RETURNS, NUM_PARAMS>
{
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Clone, const NUM_RETURNS: usize, const NUM_PARAMS: usize>
  FixedSizeSignature<T, NUM_RETURNS, NUM_PARAMS>
{
  #[must_use]
  pub fn with_default(default_val: T) -> Self {
    Self {
      base: Signature::from_parts(
        NUM_RETURNS,
        NUM_PARAMS,
        vec![default_val; NUM_RETURNS + NUM_PARAMS],
      ),
    }
  }
}

impl<T: Default + Clone, const NUM_RETURNS: usize, const NUM_PARAMS: usize>
  FixedSizeSignature<T, NUM_RETURNS, NUM_PARAMS>
{
  #[must_use]
  pub fn new() -> Self {
    Self::with_default(T::default())
  }
}

impl<T, const NUM_RETURNS: usize, const NUM_PARAMS: usize> Deref
  for FixedSizeSignature<T, NUM_RETURNS, NUM_PARAMS>
{
  type Target = Signature<T>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.base
  }
}

impl<T, const NUM_RETURNS: usize, const NUM_PARAMS: usize> DerefMut
  for FixedSizeSignature<T, NUM_RETURNS, NUM_PARAMS>
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.base
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_RETURN_COUNT_OFFSET == 0);
  assert!(K_PARAMETER_COUNT_OFFSET == 8);
  assert!(K_REPS_OFFSET == 16);
};
