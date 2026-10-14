// 由 tool/src/objects/bytecode-array.h.js 从 v8/src/objects/bytecode-array.{h,cc,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// 非压缩沙箱关布局: ExposedTrustedObject 头 8

//! 字节码数组族 (原版 bytecode-array.{h,cc,-inl.h,tq})
//!
//! BytecodeArray 活在 trusted space (沙箱外), 经 BytecodeWrapper (沙箱内
//! Struct) 与 tagged 世界互转. 布局: 头 64 字节 + 柔性 bytes 尾.
//!
//! 推迟登记 (见 readme/zh.md 台账):
//! - `source_position_table` 空表哨兵依赖 Isolate heap empty_trusted_byte_array (归 isolate/heap 席);
//! - `mark_verified` 依赖 BytecodeWrapper::set_bytecode + Publish (归 sandbox 席)。

use std::{
  mem::{offset_of, size_of},
  ops::Deref,
  ptr::{self, copy_nonoverlapping, write_bytes},
};

use v8_objects_arrays::{
  fixed_array::{TaggedArrayExt, TrustedFixedArray},
  primitive_array::{ByteArray, ByteArrayExt, TrustedByteArray, TrustedByteArrayExt},
};
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type_checker as itc,
  objects::instance_type_of,
  smi::Smi,
  tagged::{Object, Tagged},
};

use crate::error::Result;

// ===== 布局常量 (非压缩: kTaggedSize=8, 沙箱关) =====
/// 头大小 (map 8 + 5 个 tagged/protected 槽 40 + frame_size 4 + parameter 2 + max_arguments 2 + incoming 4 + padding 4)
pub const K_HEADER_SIZE: usize = 64;

/// bytes 柔性尾偏移 (原版 kBytesOffset == kHeaderSize == OFFSET_OF_DATA_START)
pub const K_BYTES_OFFSET: usize = 64;

/// 单个 BytecodeArray 内存上限 (原版 kMaxSize = 512 * MB)
pub const K_MAX_SIZE: usize = 536870912;

/// 最大字节码长度 (原版 kMaxLength = kMaxSize - kHeaderSize)
pub const K_MAX_LENGTH: i32 = 536870848;

/// 系统指针宽度 (非压缩基线 kSystemPointerSize = 8)
const K_SYSTEM_POINTER_SIZE: i32 = 8;

/// kSystemPointerSizeLog2
const K_SYSTEM_POINTER_SIZE_LOG2: u32 = 3;

// ===== 字段表 (tq 与 h 双源核对一致, 11 字段) =====
/// 字段清单 (声明顺序: 名, 类型, 偏移)
pub const BYTECODE_ARRAY_FIELDS: [(&str, &str, usize); 11] = [
  ("length", "Smi", 8),
  ("wrapper", "BytecodeWrapper", 16),
  (
    "source_position_table",
    "ProtectedPointer<TrustedByteArray>",
    24,
  ),
  ("handler_table", "ProtectedPointer<TrustedByteArray>", 32),
  ("constant_pool", "ProtectedPointer<TrustedFixedArray>", 40),
  ("frame_size", "int32", 48),
  ("parameter_size", "uint16", 52),
  ("max_arguments", "uint16", 54),
  ("incoming_new_target_or_generator_register", "int32", 56),
  ("optional_padding", "uint32", 60),
  ("bytes", "uint8", 64),
];

/// 按长度求尺寸 (原版 SizeFor: OBJECT_POINTER_ALIGN(kHeaderSize + length))
pub const fn size_for(length: usize) -> usize {
  (K_HEADER_SIZE + length + 7) & !7
}

/// BytecodeArray 对象 (ExposedTrustedObject 头 + 定长字段区; bytes 柔性尾不入结构体)
///
/// 偏移即 BYTECODE_ARRAY_FIELDS; 尾区经 `Tagged<BytecodeArray>::address() + K_BYTES_OFFSET` 访问。
#[repr(C)]
pub struct BytecodeArray {
  /// ExposedTrustedObject→TrustedObject→HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] const length (Smi, 字节码字节数)
  pub length: Tagged<Smi>,
  /// [16] BytecodeWrapper (沙箱内 tagged 回链)
  pub wrapper: Tagged<Object>,
  /// [24] source_position_table 受保护槽 (Smi::zero 或 TrustedByteArray)
  pub source_position_table: usize,
  /// [32] handler_table 受保护槽
  pub handler_table: usize,
  /// [40] constant_pool 受保护槽
  pub constant_pool: usize,
  /// [48] frame_size
  pub frame_size: i32,
  /// [52] parameter_size (含隐式接收者)
  pub parameter_size: u16,
  /// [54] max_arguments
  pub max_arguments: u16,
  /// [56] incoming_new_target_or_generator_register 操作数
  pub incoming_new_target_or_generator_register: i32,
  /// [60] optional_padding (TAGGED_SIZE_8_BYTES 下 4 字节填充)
  pub optional_padding: u32,
}

// 布局逐项断言 (与 BYTECODE_ARRAY_FIELDS 偏移全等)
const _: () = assert!(offset_of!(BytecodeArray, header) == 0);
const _: () = assert!(offset_of!(BytecodeArray, length) == 8);
const _: () = assert!(offset_of!(BytecodeArray, wrapper) == 16);
const _: () = assert!(offset_of!(BytecodeArray, source_position_table) == 24);
const _: () = assert!(offset_of!(BytecodeArray, handler_table) == 32);
const _: () = assert!(offset_of!(BytecodeArray, constant_pool) == 40);
const _: () = assert!(offset_of!(BytecodeArray, frame_size) == 48);
const _: () = assert!(offset_of!(BytecodeArray, parameter_size) == 52);
const _: () = assert!(offset_of!(BytecodeArray, max_arguments) == 54);
const _: () = assert!(offset_of!(BytecodeArray, incoming_new_target_or_generator_register) == 56);
const _: () = assert!(offset_of!(BytecodeArray, optional_padding) == 60);
const _: () = assert!(size_of::<BytecodeArray>() == K_HEADER_SIZE);

// ===== 方法面 =====
/// BytecodeArray 语义面 (原版 bytecode-array.{h,cc,-inl.h} 方法族, 挂载于 Tagged&lt;BytecodeArray&gt;)
pub trait BytecodeArrayExt: Copy {
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::length (length_.value())
  fn length(&self) -> i32;
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::get (bytes()[index])
  fn get(&self, index: usize) -> u8;
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::set
  fn set(&self, index: usize, value: u8);
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::GetFirstBytecodeAddress
  fn get_first_bytecode_address(&self) -> usize;
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::register_count
  fn register_count(&self) -> i32;
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::parameter_count
  fn parameter_count(&self) -> u16;
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::parameter_count_without_receiver
  fn parameter_count_without_receiver(&self) -> u16;
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::max_frame_size
  fn max_frame_size(&self) -> i32;
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::SourcePositionTable
  /// (空表/空槽分支, 依赖 Isolate heap empty_trusted_byte_array → 台账)
  fn source_position_table(&self) -> Result<Tagged<Object>>;
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::SetSourcePositionsFailedToCollect
  fn set_source_positions_failed_to_collect(&self);
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::SizeIncludingMetadata
  /// (三表尺寸累加, 依赖 byte-array 尺寸面 → 台账)
  fn size_including_metadata(&self) -> Result<i32>;
  /// v8: 拷贝路径 (原版 CopyBytecodesTo: 等长断言 + memcpy)
  fn copy_bytecodes_to(&self, to: Tagged<BytecodeArray>);
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::MarkVerified
  /// (Publish 沙箱支 + wrapper 注册 → 台账)
  fn mark_verified(&self) -> Result<()>;
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::clear_padding
  fn clear_padding(&self);
  /// v8: src/objects/bytecode-array-inl.h BytecodeArray::BytecodeArraySize
  fn bytecode_array_size(&self) -> i32;
}

impl BytecodeArrayExt for Tagged<BytecodeArray> {
  #[inline]
  fn length(&self) -> i32 {
    // SAFETY: 契约 — Tagged<BytecodeArray> 指向有效对象 (同 FeedbackVector 先例)
    unsafe { Smi::value((*self.as_ptr()).length) }
  }

  #[inline]
  fn get(&self, index: usize) -> u8 {
    let len = self.length();
    debug_assert!(
      index < len as usize,
      "原版 DCHECK(index >= 0 && index < length())"
    );
    // 原版: bytes()[index] — bytes 柔性尾基址 + index
    let addr = self.address() + K_BYTES_OFFSET + index;
    // SAFETY: addr 位于 [bytes 起点, bytes 起点+length) 内 (上方 debug_assert 保证)
    unsafe { *(addr as *const u8) }
  }

  #[inline]
  fn set(&self, index: usize, value: u8) {
    let len = self.length();
    debug_assert!(
      index < len as usize,
      "原版 DCHECK(index >= 0 && index < length())"
    );
    let addr = self.address() + K_BYTES_OFFSET + index;
    // SAFETY: 同 get 写路径
    unsafe { *(addr as *mut u8) = value }
  }

  #[inline]
  fn get_first_bytecode_address(&self) -> usize {
    // 原版: reinterpret_cast<Address>(bytes()) == address() + kBytesOffset
    self.address() + K_BYTES_OFFSET
  }

  #[inline]
  fn register_count(&self) -> i32 {
    // 原版: frame_size() / kSystemPointerSize
    // SAFETY: 契约同上
    let frame_size = unsafe { (*self.as_ptr()).frame_size };
    frame_size / K_SYSTEM_POINTER_SIZE
  }

  #[inline]
  fn parameter_count(&self) -> u16 {
    // SAFETY: 契约同上
    unsafe { (*self.as_ptr()).parameter_size }
  }

  #[inline]
  fn parameter_count_without_receiver(&self) -> u16 {
    // 原版: parameter_count() - 1 (含隐式接收者)
    self.parameter_count() - 1
  }

  #[inline]
  fn max_frame_size(&self) -> i32 {
    // 原版: frame_size() + (max_arguments() << kSystemPointerSizeLog2)
    // SAFETY: 契约同上
    let (frame_size, max_arguments) =
      unsafe { ((*self.as_ptr()).frame_size, (*self.as_ptr()).max_arguments) };
    frame_size + ((max_arguments as i32) << K_SYSTEM_POINTER_SIZE_LOG2)
  }

  #[inline]
  fn source_position_table(&self) -> Result<Tagged<Object>> {
    // 原版: raw_source_position_table(kAcquireLoad); == Smi::zero() → heap()->empty_trusted_byte_array()
    // TODO(台账): empty_trusted_byte_array 哨兵归 isolate/heap 席, byte-array Cast 归 byte-array 席
    todo!("推迟登记(readme/zh.md): BytecodeArray::SourcePositionTable 空表哨兵归 isolate/heap 席")
  }

  #[inline]
  fn set_source_positions_failed_to_collect(&self) {
    // 原版: TaggedField<Object,...>::Release_Store(source_position_table_, Smi::zero())
    // 受保护槽存 Smi::zero 原始位 (基线非压缩 = 全宽槽直写; 屏障桩 SKIP 本就无)
    // SAFETY: 契约同上
    unsafe {
      (*self.as_mut_ptr()).source_position_table = Smi::zero().ptr();
    }
  }

  #[inline]
  fn size_including_metadata(&self) -> Result<i32> {
    // 原版 (bytecode-array-inl.h:217): BytecodeArraySize() + 三支表尺寸:
    //   constant_pool TryCast TrustedFixedArray → array->Size()
    //   handler_table TryCast TrustedByteArray  → bytes->AllocatedSize()
    //   source_position_table IsByteArray        → Cast<ByteArray>::AllocatedSize()
    let mut size = self.bytecode_array_size();

    // SAFETY: 契约同上 (三受保护槽均为对象内全宽 tagged 值)
    let (pool_raw, handler_raw, table_raw) = unsafe {
      (
        (*self.as_ptr()).constant_pool,
        (*self.as_ptr()).handler_table,
        (*self.as_ptr()).source_position_table,
      )
    };

    // constant_pool: Smi::zero (未设) 无 instance type → Some(ty) 天然过滤
    let pool: Tagged<Object> = Tagged::new(pool_raw);
    if let Some(ty) = instance_type_of(pool)
      && itc::is_trusted_fixed_array(ty)
    {
      let array: Tagged<TrustedFixedArray> = pool.cast();
      // Size() = size_for(length) (TaggedArrayBase 尺寸公式)
      let len = unsafe { array.ulength() };
      size += <Tagged<TrustedFixedArray> as TaggedArrayExt>::size_for(len) as i32;
    }

    // handler_table
    let handler: Tagged<Object> = Tagged::new(handler_raw);
    if let Some(ty) = instance_type_of(handler)
      && itc::is_trusted_byte_array(ty)
    {
      let bytes: Tagged<TrustedByteArray> = handler.cast();
      // 原版 AllocatedSize (按 acquire 长度换算分配字节)
      size += unsafe { bytes.allocated_size() } as i32;
    }

    // source_position_table (原版此处判 ByteArray 支, 逐法保持)
    let table: Tagged<Object> = Tagged::new(table_raw);
    if let Some(ty) = instance_type_of(table)
      && itc::is_byte_array(ty)
    {
      let bytes: Tagged<ByteArray> = table.cast();
      size += unsafe { bytes.allocated_size() } as i32;
    }

    Ok(size)
  }

  #[inline]
  fn copy_bytecodes_to(&self, to: Tagged<BytecodeArray>) {
    // 原版: DCHECK_EQ(this->length(), that->length()) + CopyBytes(first, first, length)
    let len = self.length();
    debug_assert_eq!(len, to.length(), "原版 DCHECK_EQ(length, dest->length)");
    let n = len as usize;
    if n == 0 {
      return;
    }
    let src = self.get_first_bytecode_address() as *const u8;
    let dst = to.get_first_bytecode_address() as *mut u8;
    // SAFETY: 两区各 n 字节有效且等长 (上方断言); 不重叠 (两独立对象尾区)
    unsafe {
      copy_nonoverlapping(src, dst, n);
    }
  }

  #[inline]
  fn mark_verified(&self) -> Result<()> {
    // 原版: Publish(isolate) (沙箱支) + wrapper()->set_bytecode(this)
    // TODO(台账): Publish 与 BytecodeWrapper::set_bytecode 归 sandbox/trusted-object 席
    todo!("推迟登记(readme/zh.md): BytecodeArray::MarkVerified Publish/wrapper 归 sandbox 席")
  }

  #[inline]
  fn clear_padding(&self) {
    // 原版: memset(address()+data_size, 0, SizeFor(length)-data_size), data_size = kHeaderSize+length
    let len = self.length() as usize;
    let data_size = K_HEADER_SIZE + len;
    let pad = size_for(len) - data_size;
    if pad == 0 {
      return;
    }
    let dst = (self.address() + data_size) as *mut u8;
    // SAFETY: [address()+data_size, address()+SizeFor(length)) 属本对象分配尾区
    unsafe {
      write_bytes(dst, 0, pad);
    }
  }

  #[inline]
  fn bytecode_array_size(&self) -> i32 {
    // 原版: SizeFor(this->length())
    size_for(self.length() as usize) as i32
  }
}

use v8_objects_core::map::Map;

/// 拥有独立内存块的 BytecodeArray 缓冲区 (先例: FeedbackVectorBuf / CodeBuf)
pub struct BytecodeArrayBuf {
  _storage: Vec<u64>,
  tagged: Tagged<BytecodeArray>,
}

impl BytecodeArrayBuf {
  /// 分配并初始化指定字节码字节数的 BytecodeArray
  pub fn new(map: Tagged<Map>, length: usize) -> Self {
    let total_bytes = size_for(length);
    let words = total_bytes.div_ceil(8);
    let mut storage = vec![0u64; words];
    let base = storage.as_mut_ptr() as usize;
    let header = BytecodeArray {
      header: HeapObject::new(map),
      length: Smi::from_int(length as i32),
      wrapper: Tagged::NULL,
      source_position_table: Smi::zero().ptr(),
      handler_table: Smi::zero().ptr(),
      constant_pool: Smi::zero().ptr(),
      frame_size: 0,
      parameter_size: 0,
      max_arguments: 0,
      incoming_new_target_or_generator_register: 0,
      optional_padding: 0,
    };
    // SAFETY: base 指向 storage 首地址, u64 对齐且容量满足 size_for(length)
    unsafe {
      ptr::write(base as *mut BytecodeArray, header);
    }
    Self {
      _storage: storage,
      // SAFETY: base 为有效 BytecodeArray 内存
      tagged: unsafe { Tagged::from_raw_address(base) },
    }
  }

  /// 获取指向 Tagged<BytecodeArray> 的句柄
  #[inline]
  #[must_use]
  pub const fn tagged(&self) -> Tagged<BytecodeArray> {
    self.tagged
  }
}

impl Deref for BytecodeArrayBuf {
  type Target = Tagged<BytecodeArray>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.tagged
  }
}
