// 由 tool/src/objects/hash-seed-wrapper-inl.h.js 从 v8/src/objects/hash-seed-wrapper-inl.h 生成; 修订就地改, 复跑覆盖
// 席位表差集: 非对齐载荷读写访问器 (.h 布局归既有 hash-seed-wrapper.h.js)
//! HashSeedWrapper -inl 差集: 哈希种子的非对齐读写。
//!
//! 布局 (UnalignedValueMember<HashSeed::Data> data_) 归既有
//! hash-seed-wrapper.h.js; 本文件登记访问语义: 读 = address() 解引用,
//! 写 = set_value (非对齐 memcpy 语义)。

use std::{
  mem::size_of,
  ptr::{read_unaligned, write_unaligned},
};

use v8_objects_core::heap_object::HeapObject;

/// 原版 HashSeed::Data::kSecretsCount = 3 (numbers/hash-seed.h:35)。
pub const K_SECRETS_COUNT: usize = 3;

/// 原版 HashSeed::Data (numbers/hash-seed.h:42-58; 目标配置
/// V8_ENABLE_SEEDED_ARRAY_INDEX_HASH 关, m1..m3_inv 字段不出现在布局中)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HashSeedData {
  /// --hash-seed 元种子 (0 = 启动期生成)。
  pub seed: u64,
  /// rapidhash secret (默认 V8_USE_DEFAULT_HASHER_SECRET 下即默认 secret)。
  pub secrets: [u64; K_SECRETS_COUNT],
}

/// 非对齐载荷槽 (原版 UnalignedValueMember<HashSeed::Data>: 字节存储,
/// 读写经 memcpy 语义, 不要求 8 字节对齐)。
#[repr(C, align(1))]
#[derive(Copy, Clone)]
pub struct UnalignedDataMember([u8; size_of::<HashSeedData>()]);

/// 堆对象: HeapObject 头 + 非对齐 Data 载荷 (hash-seed-wrapper.h 布局)。
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HashSeedWrapper {
  /// 继承自 HeapObject。
  pub header: HeapObject,
  /// 原版 data_ (UnalignedValueMember<HashSeed::Data>)。
  pub data: UnalignedDataMember,
}

impl HashSeedWrapper {
  /// 读访问: 原版 data() = *data_.address() (非对齐地址转引用)。
  ///
  /// v8: src/objects/hash-seed-wrapper-inl.h HashSeedWrapper::data
  #[must_use]
  pub fn data(&self) -> HashSeedData {
    // SAFETY: 字节数组→HashSeedData 经 read_unaligned, 与原版 UnalignedValueMember
    // 的 memcpy 语义等价; size_of 由 UnalignedDataMember 定义锁定同尺寸, 无越读。
    unsafe { read_unaligned(self.data.0.as_ptr() as *const HashSeedData) }
  }

  /// 写访问: 原版 set_data = data_.set_value(value) (非对齐写)。
  ///
  /// v8: src/objects/hash-seed-wrapper-inl.h HashSeedWrapper::set_data
  pub fn set_data(&mut self, value: &HashSeedData) {
    // SAFETY: 同 data(): write_unaligned 字节宽度与载荷一致, 对齐无关 (原版
    // UnalignedValueMember::set_value 即 memcpy 语义)。
    unsafe {
      write_unaligned(self.data.0.as_mut_ptr() as *mut HashSeedData, *value);
    }
  }
}

// ── 提取不变式断言 ──
const _: () = {
  // 载荷 = seed + secrets ×3, 24+8 = 32 字节 (Data 无填充: u64 全对齐)
  assert!(size_of::<HashSeedData>() == 8 * (1 + K_SECRETS_COUNT));
  assert!(size_of::<UnalignedDataMember>() == size_of::<HashSeedData>());
};
