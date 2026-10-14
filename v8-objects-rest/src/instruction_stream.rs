// 由 tool/src/objects/instruction-stream.h.js 从 v8/src/objects/instruction-stream.{h,cc,-inl.h} 生成; 修订就地改, 复跑覆盖
// 基准: 非压缩/恒常池关/校验器关

//! 指令流对象 (原版 instruction-stream.{h,cc,-inl.h})
//!
//! InstructionStream 是信任对象但住在 code space (非 trusted space):
//! 头部只读 (写保护页), 体 = 指令段 + 对齐填充 + 元数据段.

use std::mem::size_of;

use v8_objects_arrays::primitive_array::{TrustedByteArray, TrustedByteArrayExt};
use v8_objects_core::{
  heap_object::HeapObject,
  smi::Smi,
  tagged::{Object, Tagged},
};

use crate::{
  code::{Code, K_CODE_ALIGNMENT_BASELINE},
  error::{Error, Result},
};

/// 指令流对象 (ExposedTrustedObject 头 + 只读头部; 指令体在柔性尾, 不入结构体)
///
/// 布局即 ISTREAM_FIELDS (kCodeOffset=8, kRelocationInfoOffset=16, kBodySizeOffset=24);
/// 头部住在写保护页 (不可原地改), 体经 `address() + K_HEADER_SIZE` 访问。
#[repr(C)]
pub struct InstructionStream {
  /// ExposedTrustedObject→TrustedObject→HeapObject 头 (map 槽, 偏移 0)
  pub header: HeapObject,
  /// [8] kCodeOffset 受保护槽 (Code 或 Smi::zero)
  pub code_slot: usize,
  /// [16] kRelocationInfoOffset 受保护槽 (TrustedByteArray)
  pub relocation_info_slot: usize,
  /// [24] kBodySizeOffset 指令体字节数
  pub body_size_field: u32,
}

const _: () = assert!(size_of::<InstructionStream>() == K_HEADER_SIZE);

/// InstructionStream 语义面 (原版 instruction-stream.{h,cc,-inl.h} 方法族, 挂载于 Tagged&lt;InstructionStream&gt;)
pub trait InstructionStreamExt: Copy {
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::instruction_start
  /// (address + kHeaderSize)
  fn instruction_start(&self) -> usize;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::body_size
  fn body_size(&self) -> u32;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::body_end
  fn body_end(&self) -> usize;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::constant_pool
  /// (基线 V8_EMBEDDED_CONSTANT_POOL_BOOL=false → kNullAddress)
  fn constant_pool(&self) -> usize;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::raw_code (Acquire_Load)
  fn raw_code(&self) -> Tagged<Object>;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::code
  fn code(&self) -> Result<Tagged<Code>>;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::TryGetCodeUnchecked
  fn try_get_code(&self) -> Option<Tagged<Code>>;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::IsFullyInitialized
  fn is_fully_initialized(&self) -> bool;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::relocation_info
  fn relocation_info(&self) -> Tagged<Object>;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::relocation_start
  /// (relocation_info()->begin(), TrustedByteArray 数据起点)
  fn relocation_start(&self) -> usize;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::relocation_end
  /// (relocation_info()->end())
  fn relocation_end(&self) -> usize;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::relocation_size
  /// (relocation_info()->ulength())
  fn relocation_size(&self) -> i32;
  /// v8: src/objects/instruction-stream-inl.h InstructionStream::Size (SizeFor(body_size))
  fn size(&self) -> i32;
}

impl InstructionStreamExt for Tagged<InstructionStream> {
  #[inline]
  fn instruction_start(&self) -> usize {
    // 原版: field_address(kHeaderSize) == address() + kHeaderSize
    self.address() + K_HEADER_SIZE
  }

  #[inline]
  fn body_size(&self) -> u32 {
    // SAFETY: 契约 — Tagged<InstructionStream> 指向有效对象
    unsafe { (*self.as_ptr()).body_size_field }
  }

  #[inline]
  fn body_end(&self) -> usize {
    // 原版: instruction_start() + body_size() (静态断言 kOnHeapBodyIsContiguous)
    self.instruction_start() + self.body_size() as usize
  }

  #[inline]
  fn constant_pool(&self) -> usize {
    // 基线 V8_EMBEDDED_CONSTANT_POOL_BOOL=false: 恒 kNullAddress
    0
  }

  #[inline]
  fn raw_code(&self) -> Tagged<Object> {
    // 原版: RawProtectedPointerField(kCodeOffset).Acquire_Load()
    // SAFETY: 契约同上; 全宽受保护槽即 usize
    unsafe { Tagged::<Object>::new((*self.as_ptr()).code_slot) }
  }

  #[inline]
  fn code(&self) -> Result<Tagged<Code>> {
    // 原版: TrustedCast<Code>(raw_code()) (DCHECK 非空)
    let raw = self.raw_code();
    if raw == Smi::zero().cast() {
      return Err(Error::Invariant(
        "InstructionStream::code: 关联 Code 未初始化 (Smi::zero)",
      ));
    }
    Ok(raw.cast())
  }

  #[inline]
  fn try_get_code(&self) -> Option<Tagged<Code>> {
    // 原版: raw_code == Smi::zero() → false; 否则 UncheckedCast
    let raw = self.raw_code();
    if raw == Smi::zero().cast() {
      None
    } else {
      Some(raw.cast())
    }
  }

  #[inline]
  fn is_fully_initialized(&self) -> bool {
    // 原版: raw_code(kAcquireLoad) != Smi::zero()
    self.raw_code() != Smi::zero().cast()
  }

  #[inline]
  fn relocation_info(&self) -> Tagged<Object> {
    // 原版: ReadProtectedPointerField<TrustedByteArray>(kRelocationInfoOffset)
    // SAFETY: 契约同上; 全宽受保护槽直读
    unsafe { Tagged::<Object>::new((*self.as_ptr()).relocation_info_slot) }
  }

  #[inline]
  fn relocation_start(&self) -> usize {
    // 原版: relocation_info()->begin() — 数据区首地址 (仅指针运算)
    let info: Tagged<TrustedByteArray> = self.relocation_info().cast();
    info.address() + TrustedByteArray::DATA_OFFSET
  }

  #[inline]
  fn relocation_end(&self) -> usize {
    // 原版: relocation_info()->end() = begin + ulength (字节元素)
    let info: Tagged<TrustedByteArray> = self.relocation_info().cast();
    let len = info.length();
    self.relocation_start() + len as usize * TrustedByteArray::ELEMENT_SIZE
  }

  #[inline]
  fn relocation_size(&self) -> i32 {
    // 原版: relocation_info()->ulength().value() (SafeHeapObjectSize, uint32 宽)
    let info: Tagged<TrustedByteArray> = self.relocation_info().cast();
    info.length() as i32
  }

  #[inline]
  fn size(&self) -> i32 {
    // 原版: SizeFor(body_size()); 基线对齐传 K_CODE_ALIGNMENT (由尺寸公式内部处理)
    size_for(self.body_size() as usize, K_CODE_ALIGNMENT_BASELINE) as i32
  }
}

// ===== 布局 (ISTREAM_FIELDS, 7 项) =====
/// 字段偏移表 (基准配置求值)
pub const ISTREAM_FIELDS: [(&str, usize, usize, &str); 7] = [
  ("kCodeOffset", 8, 8, "kProtectedPointerSize"),
  ("kRelocationInfoOffset", 16, 8, "kProtectedPointerSize"),
  ("kDataStart", 24, 0, "0"),
  ("kBodySizeOffset", 24, 4, "kUInt32Size"),
  (
    "kConstantPoolOffsetOffset",
    28,
    0,
    "V8_EMBEDDED_CONSTANT_POOL_BOOL ? kIntSize : 0",
  ),
  (
    "kUnalignedSize",
    28,
    4,
    "OBJECT_POINTER_PADDING(kUnalignedSize",
  ),
  ("kHeaderSize", 32, 0, "0"),
];

/// 头大小 (基准配置 kHeaderSize)
pub const K_HEADER_SIZE: usize = 32;

/// 元数据段对齐 (原版 kMetadataAlignment = kIntSize)
pub const K_METADATA_ALIGNMENT: usize = 4;

/// 头邻接性: 堆上体连续 true / 离堆体连续 false (Code::metadata_start 依赖此对)
pub const K_ON_HEAP_BODY_IS_CONTIGUOUS: bool = true;
pub const K_OFF_HEAP_BODY_IS_CONTIGUOUS: bool = false;

/// 尾部填充 (原版 TrailingPaddingSizeFor: RoundUp<kCodeAlignment>(kHeaderSize+body_size) - kHeaderSize - body_size)
pub const fn trailing_padding_size_for(body_size: usize, k_code_alignment: usize) -> usize {
  k_header_size_padded(K_HEADER_SIZE + body_size, k_code_alignment) - K_HEADER_SIZE - body_size
}

/// 按体大小求对象尺寸 (原版 SizeFor: kHeaderSize + body_size + TrailingPaddingSizeFor)
pub const fn size_for(body_size: usize, k_code_alignment: usize) -> usize {
  K_HEADER_SIZE + body_size + trailing_padding_size_for(body_size, k_code_alignment)
}

const fn k_header_size_padded(v: usize, k_code_alignment: usize) -> usize {
  v.div_ceil(k_code_alignment) * k_code_alignment
}

/// 条目地址对齐补头 (原版 kCodeAlignmentMinusCodeHeader = kCodeAlignment - kHeaderSize;
/// MemoryChunk 首部一次性垫该值使入口对 kCodeAlignment 对齐)
pub const fn k_code_alignment_minus_code_header(k_code_alignment: usize) -> usize {
  k_code_alignment - K_HEADER_SIZE
}

// ===== 方法面 (指令流只读头部 + 关联 Code; 语义面见 InstructionStreamExt) =====

// 静态地址还原面 (原版 FromTargetAddress / FromEntryAddress: 均为静态入口反查对象)
/// 从代码目标地址还原对象 (原版 FromTargetAddress: address - kHeaderSize + kHeapObjectTag)
pub fn from_target_address(address: usize) -> usize {
  address - K_HEADER_SIZE + 1 /* kHeapObjectTag */
}

/// 从存放地址的位置还原对象 (原版 FromEntryAddress: 解引用一次再走 FromTargetAddress)
pub fn from_entry_address(location_of_address: usize) -> usize {
  // 原版: code_entry = base::Memory<Address>(location_of_address);
  //       FromAddress(code_entry - kHeaderSize)
  // SAFETY: location_of_address 为指向一个 usize 入口槽的有效地址 (调用方契约)
  let code_entry = unsafe { *(location_of_address as *const usize) };
  from_target_address(code_entry)
}

/// 整体重定位 (原版 cc Relocate: WritableJitAllocation 授权下逐 RelocInfo 平移 delta)
/// TODO(台账): WritableRelocIterator/JitAllocation 归 codegen-core + heap 席
pub fn relocate(/* this, jit_allocation, delta */) {
  todo!("推迟登记(readme/zh.md): InstructionStream::Relocate 归 codegen-core+heap 席")
}

/// 从 CodeDesc 装配 (原版 Initialize: 头部 + 指令拷贝 + 元数据布局)
/// TODO(台账): WritableJitAllocation/CodeDesc 归 codegen-core 席
pub fn initialize(/* self, map, body_size, constant_pool_offset, reloc_info */) {
  todo!("推迟登记(readme/zh.md): InstructionStream::Initialize 归 codegen-core 席")
}

/// 完成装配 (原版 Finalize: 反向填 Code 槽 + 重定位)
/// TODO(台账): SetTaggedField 屏障桩 + RelocateFromDesc 归 heap+codegen-core 席
pub fn finalize(/* this, code, reloc_info, desc, heap */) {
  todo!("推迟登记(readme/zh.md): InstructionStream::Finalize 归 heap+codegen-core 席")
}

/// 写屏障承诺 (原版 WriteBarrierPromise: DEBUG 下登记延迟写屏障地址集合)
pub struct WriteBarrierPromise {
  _private: (),
}
