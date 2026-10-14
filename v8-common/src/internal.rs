//! v8-internal.h / src/base/build_config.h 底层常量移植层
//!
//! v8-common 零引擎依赖, globals.h 依赖的 include/v8-internal.h 值在此落位。
//! 目标配置: 64 位 (x64+arm64), 指针压缩开 → kApiTaggedSize = 4, Smi 31 位。

/// 原版 v8-internal.h:41 KB
pub const KB: usize = 1024;
/// 原版 v8-internal.h:42 MB
pub const MB: usize = KB * 1024;
/// 原版 v8-internal.h:43 GB
pub const GB: usize = MB * 1024;

/// 原版 v8-internal.h:39 kNullAddress
pub const K_NULL_ADDRESS: usize = 0;

/// 原版 v8-internal.h Address (uintptr_t) — 术语表映射 usize
pub type Address = usize;

// --- 堆对象 tag (v8-internal.h:58-62) ---
pub const K_HEAP_OBJECT_TAG: usize = 1;
pub const K_WEAK_HEAP_OBJECT_TAG: usize = 3;
pub const K_HEAP_OBJECT_TAG_SIZE: usize = 2;
pub const K_HEAP_OBJECT_TAG_MASK: usize = (1 << K_HEAP_OBJECT_TAG_SIZE) - 1;
pub const K_HEAP_OBJECT_REFERENCE_TAG_MASK: usize = 1 << (K_HEAP_OBJECT_TAG_SIZE - 1);

// --- Smi tag (v8-internal.h:72-74) ---
pub const K_SMI_TAG: usize = 0;
pub const K_SMI_TAG_SIZE: usize = 1;
pub const K_SMI_TAG_MASK: usize = (1 << K_SMI_TAG_SIZE) - 1;

// --- Smi tagging (v8-internal.h:86, 64 位 + 压缩指针 → SmiTagging<kApiInt32Size>) ---
/// 原版 SmiTagging<4>::kSmiShiftSize
pub const K_SMI_SHIFT_SIZE: usize = 0;
/// 原版 SmiTagging<4>::kSmiValueSize
pub const K_SMI_VALUE_SIZE: usize = 31;

/// 原版 SmiValuesAre31Bits() — 目标配置恒真
pub const SMI_VALUES_ARE_31_BITS: bool = K_SMI_VALUE_SIZE == 31;
/// 原版 SmiValuesAre32Bits() — 目标配置恒假
pub const SMI_VALUES_ARE_32_BITS: bool = K_SMI_VALUE_SIZE == 32;

// --- API 尺寸 (v8-internal.h:51-54, 173-175) ---
pub const K_API_SYSTEM_POINTER_SIZE: usize = 8;
pub const K_API_DOUBLE_SIZE: usize = 8;
pub const K_API_INT32_SIZE: usize = 4;
pub const K_API_INT64_SIZE: usize = 8;
/// 原版 V8_COMPRESS_POINTERS 分支: kApiTaggedSize = kApiInt32Size
pub const K_API_TAGGED_SIZE: usize = K_API_INT32_SIZE;

// --- 指针压缩 cage (v8-internal.h:167-168) ---
pub const K_PTR_COMPR_CAGE_RESERVATION_SIZE: usize = 1usize << 32;
pub const K_PTR_COMPR_CAGE_BASE_ALIGNMENT: usize = 1usize << 32;

// --- 页大小 (src/base/build_config.h:80, 非 PPC64/非 HUGEPAGE 分支) ---
pub const K_PAGE_SIZE_BITS: usize = 18;
pub const K_REGULAR_PAGE_SIZE: usize = 1 << K_PAGE_SIZE_BITS;

// --- Internals 静态量 (v8-internal.h) ---
/// 原版 v8-internal.h:1157 kDisallowGarbageCollectionAlign = alignof(uint32_t)
pub const K_DISALLOW_GARBAGE_COLLECTION_ALIGN: usize = 4;
/// 原版 v8-internal.h:1158 kDisallowGarbageCollectionSize = sizeof(uint32_t)
pub const K_DISALLOW_GARBAGE_COLLECTION_SIZE: usize = 4;
/// 原版 v8-internal.h:1227 kDontThrow
pub const K_DONT_THROW: i32 = 0;
/// 原版 v8-internal.h:1228 kThrowOnError
pub const K_THROW_ON_ERROR: i32 = 1;
/// 原版 v8-internal.h:1233 kExternalAllocationSoftLimit
pub const K_EXTERNAL_ALLOCATION_SOFT_LIMIT: usize = 64 * 1024 * 1024;
/// 原版 v8-internal.h:1598 kGarbageCollectionReasonMaxValue
pub const K_GARBAGE_COLLECTION_REASON_MAX_VALUE: i32 = 30;

// --- 句柄存储尺寸 (v8-internal.h, sandbox/compress 开启分支) ---
/// ExternalPointer_t = ExternalPointerHandle = uint32_t (sandbox 开)
pub const K_EXTERNAL_POINTER_HANDLE_SIZE: usize = 4;
/// CppHeapPointer_t = CppHeapPointerHandle = uint32_t (compress 开)
pub const K_CPP_HEAP_POINTER_HANDLE_SIZE: usize = 4;
/// IndirectPointerHandle = uint32_t (sandbox 开)
pub const K_INDIRECT_POINTER_HANDLE_SIZE: usize = 4;

// --- wasm 代码指针表 (v8-internal.h:957) ---
pub const K_WASM_CODE_POINTER_TABLE_RESERVATION_SIZE: usize = 128 * MB;

/// 原版 globals.h:28 V8_INFINITY
pub const V8_INFINITY: f64 = f64::INFINITY;
