// 由 tool/src/objects/tagged.h.js 从 v8/include/v8-internal.h 生成; 修订就地改, 复跑覆盖
// Tag 掩码与关键偏移量常量

//! V8 指针 Tagging 与堆对象内存布局常量

/// 堆对象 Tag 标记值 (最低位: 0b01)
pub const HEAP_OBJECT_TAG: usize = 1;

/// 弱引用堆对象 Tag 标记值 (最低位: 0b11)
pub const WEAK_HEAP_OBJECT_TAG: usize = 3;

/// 堆对象 Tag 位宽 (2 bits)
pub const HEAP_OBJECT_TAG_SIZE: usize = 2;

/// 堆对象 Tag 掩码 (0b11)
pub const HEAP_OBJECT_TAG_MASK: usize = 3;

/// 堆对象引用 Tag 掩码 (0b10)
pub const HEAP_OBJECT_REFERENCE_TAG_MASK: usize = 2;

/// 转发指针 Tag 标记值 (0b00)
pub const FORWARDING_TAG: usize = 0;

/// 转发指针 Tag 位宽 (2 bits)
pub const FORWARDING_TAG_SIZE: usize = 2;

/// 转发指针 Tag 掩码 (0b11)
pub const FORWARDING_TAG_MASK: usize = 3;

/// 小整数 (Smi) Tag 标记值 (最低位: 0b0)
pub const SMI_TAG: usize = 0;

/// 小整数 (Smi) Tag 位宽 (1 bit)
pub const SMI_TAG_SIZE: usize = 1;

/// 小整数 (Smi) Tag 掩码 (0b1)
pub const SMI_TAG_MASK: usize = 1;

/// 堆对象 Map 指针字段的首字节偏移量 (0)
pub const HEAP_OBJECT_MAP_OFFSET: usize = 0;

/// 31 位 Smi (小整数) 掩码与移位常量 (指针压缩 / 32 位系统)
pub mod smi31 {
  pub const SHIFT_SIZE: usize = 0;
  pub const VALUE_SIZE: usize = 31;
  pub const SHIFT_BITS: usize = super::SMI_TAG_SIZE + SHIFT_SIZE;
  pub const MIN_VALUE: i32 = -1073741824; // -2^30
  pub const MAX_VALUE: i32 = 1073741823; // 2^30 - 1
}

/// 32 位 Smi (小整数) 掩码与移位常量 (未压缩 64 位系统)
pub mod smi32 {
  pub const SHIFT_SIZE: usize = 31;
  pub const VALUE_SIZE: usize = 32;
  pub const SHIFT_BITS: usize = super::SMI_TAG_SIZE + SHIFT_SIZE;
  pub const MIN_VALUE: i32 = i32::MIN; // -2^31
  pub const MAX_VALUE: i32 = i32::MAX; // 2^31 - 1
}
