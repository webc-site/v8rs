// 由 tool/src/codegen/arm64/constant-pool-arm64.h.js 从 v8/src/codegen/arm64/constant-pool-arm64.{h,cc} 生成; 修订就地改, 复跑覆盖
// arm64 常量池: Key 结构/枚举/距离常量/签名骨架

// ===== 池枚举 (原版 enum class, 5 组) =====

use std::{cmp::Ordering, collections::BTreeMap, mem::take};

/// 原版 enum class Jump: 池上是否需要跳越分支
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Jump {
  Omitted = 0,
  Required = 1,
}

/// 原版 enum class Emission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Emission {
  IfNeeded = 0,
  Forced = 1,
}

/// 原版 enum class Alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Alignment {
  Omitted = 0,
  Required = 1,
}

/// 原版 enum class RelocInfoStatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RelocInfoStatus {
  MustRecord = 0,
  MustOmitForDuplicate = 1,
}

/// 原版 enum class PoolEmissionCheck
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PoolEmissionCheck {
  Skip = 0,
}

// ===== 距离常量 (ConstantPool 静态成员, 定义于 assembler-arm64.cc; 机械求值) =====

/// 汇编器 kGap (assembler-arm64.h Assembler::kGap)
pub const K_GAP: i32 = 64;
/// 32 位条目池最大距离 (1 * MB)
pub const K_MAX_DIST_TO_POOL32: usize = 1048576;
/// 64 位条目池最大距离 (1 * MB)
pub const K_MAX_DIST_TO_POOL64: usize = 1048576;
/// 池发射检查间隔 (128 * kInstrSize = 128 * 4)
pub const K_CHECK_INTERVAL: usize = 512;
/// 近似池距离 (64 * KB)
pub const K_APPROX_DIST_TO_POOL32: usize = 65536;
pub const K_APPROX_DIST_TO_POOL64: usize = 65536;
/// 池发射机会距离 (64 * KB)
pub const K_OPPORTUNITY_DIST_TO_POOL32: usize = 65536;
pub const K_OPPORTUNITY_DIST_TO_POOL64: usize = 65536;
/// 近似最大条目数
pub const K_APPROX_MAX_ENTRY_COUNT: usize = 512;

// ===== ConstantPoolKey =====

/// 原版 class ConstantPoolKey: is_value32_ | union{value64_/value32_} | rmode_
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantPoolKey {
  is_value32_: bool,
  value_: u64,
  /// RelocInfo::Mode 编码 (见 v8-codegen-core::reloc_info)
  rmode_: u32,
}

/// RelocInfo::NO_INFO = 0
pub const RELOC_NO_INFO: u32 = 0;

impl ConstantPoolKey {
  /// 原版构造 1: uint64 值
  #[inline]
  #[must_use]
  pub const fn from_value64(value: u64, rmode: u32) -> Self {
    Self {
      is_value32_: false,
      value_: value,
      rmode_: rmode,
    }
  }

  /// 原版构造 2: uint32 值
  #[inline]
  #[must_use]
  pub const fn from_value32(value: u32, rmode: u32) -> Self {
    Self {
      is_value32_: true,
      value_: value as u64,
      rmode_: rmode,
    }
  }

  /// 原版 value64(): CHECK 非 32 位
  #[inline]
  #[must_use]
  pub const fn value64(&self) -> u64 {
    debug_assert!(!self.is_value32_);
    self.value_
  }

  /// 原版 value32(): CHECK 为 32 位
  #[inline]
  #[must_use]
  pub const fn value32(&self) -> u32 {
    debug_assert!(self.is_value32_);
    self.value_ as u32
  }

  /// 原版 is_value32()
  #[inline]
  #[must_use]
  pub const fn is_value32(&self) -> bool {
    self.is_value32_
  }

  /// 原版 rmode()
  #[inline]
  #[must_use]
  pub const fn rmode(&self) -> u32 {
    self.rmode_
  }

  /// 原版 AllowsDeduplication: 可共享重定位模式或可共享 code_target/embedded_object
  #[must_use]
  pub fn allows_deduplication(&self) -> bool {
    use v8_codegen_core::RelocMode;
    debug_assert!(
      self.rmode_ != RelocMode::ConstPool as u32
        && self.rmode_ != RelocMode::VeneerPool as u32
        && !(self.rmode_ >= RelocMode::DeoptScriptOffset as u32
          && self.rmode_ <= RelocMode::DeoptNodeId as u32)
    );
    let val_nonzero = if self.is_value32_ {
      self.value32() != 0
    } else {
      self.value64() != 0
    };
    let is_sharable_code_target = self.rmode_ == RelocMode::CodeTarget as u32 && val_nonzero;
    let is_sharable_embedded_object = self.rmode_ == RelocMode::CompressedEmbeddedObject as u32
      || self.rmode_ == RelocMode::FullEmbeddedObject as u32;
    let is_shareable_reloc_mode =
      self.rmode_ == RelocMode::NoInfo as u32 || self.rmode_ >= RelocMode::WasmCall as u32;
    is_shareable_reloc_mode || is_sharable_code_target || is_sharable_embedded_object
  }
}

/// 原版 operator<: 64 位条目排序在前 (机械直译)
impl Ord for ConstantPoolKey {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .is_value32_
      .cmp(&other.is_value32_)
      .then_with(|| self.rmode_.cmp(&other.rmode_))
      .then_with(|| {
        if self.is_value32_ {
          self.value32().cmp(&other.value32())
        } else {
          self.value64().cmp(&other.value64())
        }
      })
  }
}

impl PartialOrd for ConstantPoolKey {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

// ===== ConstantPool (实现) =====

/// 原版 class ConstantPool: 池生成器 (条目 32/64 分池, 64 位在前)
pub struct ConstantPool {
  /// 原版 entries_ (映射 key 到多个引用 pc_offset)
  entries_: BTreeMap<ConstantPoolKey, Vec<i32>>,
  /// 原版 entry32_count_
  entry32_count_: usize,
  /// 原版 entry64_count_
  entry64_count_: usize,
  first_use_32_: i32,
  first_use_64_: i32,
  next_check_: i32,
  old_next_check_: i32,
  blocked_nesting_: i32,
}

impl ConstantPool {
  /// 原版构造
  #[must_use]
  pub fn new(_assm: *mut ()) -> Self {
    Self {
      entries_: BTreeMap::new(),
      entry32_count_: 0,
      entry64_count_: 0,
      first_use_32_: -1,
      first_use_64_: -1,
      next_check_: 0,
      old_next_check_: 0,
      blocked_nesting_: 0,
    }
  }

  /// 原版 Entry32Count
  #[inline]
  #[must_use]
  pub const fn entry32_count(&self) -> usize {
    self.entry32_count_
  }

  /// 原版 Entry64Count
  #[inline]
  #[must_use]
  pub const fn entry64_count(&self) -> usize {
    self.entry64_count_
  }

  /// 原版 IsEmpty
  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.entries_.is_empty()
  }

  /// 常量池是否被暂时阻塞
  #[inline]
  #[must_use]
  pub const fn is_blocked(&self) -> bool {
    self.blocked_nesting_ > 0
  }

  pub fn start_block(&mut self) {
    if self.blocked_nesting_ == 0 {
      self.old_next_check_ = self.next_check_;
      self.next_check_ = i32::MAX;
    }
    self.blocked_nesting_ += 1;
  }

  pub fn end_block(&mut self) {
    self.blocked_nesting_ = self.blocked_nesting_.saturating_sub(1);
    if self.blocked_nesting_ == 0 {
      self.next_check_ = self.next_check_.min(self.old_next_check_);
    }
  }

  pub fn set_next_check_in(&mut self, pc_offset: i32, instructions: usize) {
    self.next_check_ = pc_offset.saturating_add((instructions * 4) as i32);
  }

  pub fn record_key(&mut self, key: ConstantPoolKey, offset: i32) -> RelocInfoStatus {
    let write_reloc = self.get_reloc_info_status_for(&key);
    if write_reloc == RelocInfoStatus::MustRecord {
      if key.is_value32() {
        if self.entry32_count_ == 0 {
          self.first_use_32_ = offset;
        }
        self.entry32_count_ += 1;
      } else {
        if self.entry64_count_ == 0 {
          self.first_use_64_ = offset;
        }
        self.entry64_count_ += 1;
      }
    }
    self.entries_.entry(key).or_default().push(offset);
    if self.entry32_count_ + self.entry64_count_ > K_APPROX_MAX_ENTRY_COUNT {
      self.set_next_check_in(offset, 1);
    }
    write_reloc
  }

  pub fn get_reloc_info_status_for(&self, key: &ConstantPoolKey) -> RelocInfoStatus {
    if key.allows_deduplication() && self.entries_.contains_key(key) {
      RelocInfoStatus::MustOmitForDuplicate
    } else {
      RelocInfoStatus::MustRecord
    }
  }

  /// 原版 RecordEntry(u32 data, rmode)
  pub fn record_entry32(&mut self, data: u32, rmode: u32, offset: i32) -> RelocInfoStatus {
    let key = ConstantPoolKey::from_value32(data, rmode);
    self.record_key(key, offset)
  }

  /// 原版 RecordEntry(u64 data, rmode)
  pub fn record_entry64(&mut self, data: u64, rmode: u32, offset: i32) -> RelocInfoStatus {
    let key = ConstantPoolKey::from_value64(data, rmode);
    self.record_key(key, offset)
  }

  pub const fn prologue_size(require_jump: Jump) -> i32 {
    let jump_size = match require_jump {
      Jump::Required => 4,
      Jump::Omitted => 0,
    };
    jump_size + 8
  }

  pub fn is_alignment_required_if_emitted_at(
    &self,
    require_jump: Jump,
    pc_offset: i32,
  ) -> Alignment {
    let size_up_to_marker = Self::prologue_size(require_jump);
    if self.entry64_count_ != 0 && ((pc_offset + size_up_to_marker) % 8 != 0) {
      Alignment::Required
    } else {
      Alignment::Omitted
    }
  }

  /// 原版 ComputeSize(require_jump, require_alignment)
  pub fn compute_size(&self, require_jump: Jump, require_alignment: Alignment) -> i32 {
    let size_up_to_marker = Self::prologue_size(require_jump);
    let alignment = match require_alignment {
      Alignment::Required => 4,
      Alignment::Omitted => 0,
    };
    let size_after_marker = (self.entry32_count_ * 4) + alignment + (self.entry64_count_ * 8);
    size_up_to_marker + (size_after_marker as i32)
  }

  /// 原版 IsInImmRangeIfEmittedAt(pc_offset)
  #[must_use]
  pub fn is_in_imm_range_if_emitted_at(&self, pc_offset: i32) -> bool {
    let req_align = self.is_alignment_required_if_emitted_at(Jump::Required, pc_offset);
    let pool_end_32 =
      (pc_offset as usize) + (self.compute_size(Jump::Required, req_align) as usize);
    let pool_end_64 = pool_end_32.saturating_sub(self.entry32_count_ * 4);
    let entries_in_range_32 = self.entry32_count_ == 0
      || (self.first_use_32_ >= 0
        && pool_end_32 < (self.first_use_32_ as usize + K_MAX_DIST_TO_POOL32));
    let entries_in_range_64 = self.entry64_count_ == 0
      || (self.first_use_64_ >= 0
        && pool_end_64 < (self.first_use_64_ as usize + K_MAX_DIST_TO_POOL64));
    entries_in_range_32 && entries_in_range_64
  }

  /// 原版 ShouldEmitNow(require_jump, margin)
  #[must_use]
  pub fn should_emit_now(&self, require_jump: Jump, margin: usize, pc_offset: i32) -> bool {
    if self.is_empty() {
      return false;
    }
    if self.entry32_count_ + self.entry64_count_ > K_APPROX_MAX_ENTRY_COUNT {
      return true;
    }
    let worst_case_size = self.compute_size(Jump::Required, Alignment::Required);
    let pool_end_32 = (pc_offset as usize) + margin + (worst_case_size as usize);
    let pool_end_64 = pool_end_32.saturating_sub(self.entry32_count_ * 4);
    if self.entry64_count_ != 0 && self.first_use_64_ >= 0 {
      let dist64 = pool_end_64.saturating_sub(self.first_use_64_ as usize);
      let next_check_too_late = dist64 + 2 * K_CHECK_INTERVAL >= K_MAX_DIST_TO_POOL64;
      let opportune = require_jump == Jump::Omitted && (dist64 >= K_OPPORTUNITY_DIST_TO_POOL64);
      let approx_exceeded = dist64 >= K_APPROX_DIST_TO_POOL64;
      if next_check_too_late || opportune || approx_exceeded {
        return true;
      }
    }
    if self.entry32_count_ != 0 && self.first_use_32_ >= 0 {
      let dist32 = pool_end_32.saturating_sub(self.first_use_32_ as usize);
      let next_check_too_late = dist32 + 2 * K_CHECK_INTERVAL >= K_MAX_DIST_TO_POOL32;
      let opportune = require_jump == Jump::Omitted && (dist32 >= K_OPPORTUNITY_DIST_TO_POOL32);
      let approx_exceeded = dist32 >= K_APPROX_DIST_TO_POOL32;
      if next_check_too_late || opportune || approx_exceeded {
        return true;
      }
    }
    false
  }

  /// 原版 Check(force_emission, require_jump, margin)
  pub fn check(
    &mut self,
    force: Emission,
    require_jump: Jump,
    margin: usize,
    pc_offset: i32,
  ) -> bool {
    if self.is_blocked() {
      debug_assert_eq!(force, Emission::IfNeeded);
      return false;
    }
    !self.is_empty()
      && (force == Emission::Forced || self.should_emit_now(require_jump, margin, pc_offset))
  }

  /// 原版 MaybeCheck()
  pub fn maybe_check(&mut self, pc_offset: i32) -> bool {
    if pc_offset >= self.next_check_ {
      self.check(Emission::IfNeeded, Jump::Required, 0, pc_offset)
    } else {
      false
    }
  }

  /// 原版 EmitAndClear(require)
  pub fn emit_and_clear(&mut self, _require: Jump) -> Vec<(ConstantPoolKey, Vec<i32>)> {
    debug_assert!(!self.is_blocked());
    let cap = self.entry32_count_ + self.entry64_count_;
    let mut result = Vec::with_capacity(cap);
    let entries = take(&mut self.entries_);
    for (key, offsets) in entries {
      if key.allows_deduplication() {
        result.push((key, offsets));
      } else {
        for offset in offsets {
          result.push((key, vec![offset]));
        }
      }
    }
    self.clear();
    result
  }

  /// 原版 Clear()
  pub fn clear(&mut self) {
    self.entries_.clear();
    self.first_use_32_ = -1;
    self.first_use_64_ = -1;
    self.entry32_count_ = 0;
    self.entry64_count_ = 0;
    self.next_check_ = 0;
    self.old_next_check_ = 0;
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_GAP == 64);
  assert!(K_MAX_DIST_TO_POOL32 == 1 << 20);
  assert!(K_CHECK_INTERVAL == 128 * 4);
  assert!(K_APPROX_DIST_TO_POOL32 == 64 << 10);
  assert!(K_APPROX_MAX_ENTRY_COUNT == 512);
  assert!(Jump::Required as u8 == 1);
  assert!(Emission::Forced as u8 == 1);
  assert!(Alignment::Required as u8 == 1);
  assert!(RelocInfoStatus::MustOmitForDuplicate as u8 == 1);
  assert!(PoolEmissionCheck::Skip as u8 == 0);
};
