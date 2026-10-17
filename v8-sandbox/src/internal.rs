// 由 tool/src/sandbox/internal.js 从 v8/include/v8-internal.h (sandbox/external-pointer 常量段) 生成; 修订就地改, 复跑覆盖
// 目标分支: 64 位桌面 (非 Android/iOS/RISCV/LOONG, 非 LA57); 原版 static_assert 已在脚本内对数核对

//! 沙箱常量层; 对应 v8/include/v8-internal.h 的 V8_ENABLE_SANDBOX 段。
//! 生成脚本内置对数核对 11 条全部通过。

/// 原版 v8-internal.h:41-43
pub const KB: usize = 1024;
pub const MB: usize = KB * 1024;
pub const GB: usize = MB * 1024;

/// 原版 Address (uintptr_t); 术语表映射 usize
pub type Address = usize;

/// 原版 kNullAddress (v8/src/common/globals.h): 0 地址哨兵
pub const K_NULL_ADDRESS: Address = 0;

// ---- 指针压缩 cage (v8-internal.h:167-168) ----
pub const K_PTR_COMPR_CAGE_RESERVATION_SIZE: usize = 4294967296; // 4 GB
pub const K_PTR_COMPR_CAGE_BASE_ALIGNMENT: usize = 4294967296; // 4 GB

// ---- 沙箱尺寸/对齐 (v8-internal.h:218-258) ----
/// 1 TB; Android/ChromeOS-ARM64/iOS/RISCV/LOONG 分支 (37/34) 见原版, 目标取默认
pub const K_SANDBOX_SIZE_LOG2: u32 = 40;
pub const K_SANDBOX_SIZE: usize = 1099511627776;
pub const K_SANDBOX_ALIGNMENT: usize = 4294967296; // = kPtrComprCageBaseAlignment
/// 沙箱内指针编码右移量 = 64 - kSandboxSizeLog2 = 24
pub const K_SANDBOXED_POINTER_SHIFT: u32 = 24;
/// 部分预留沙箱最小预留 = 8 GB
pub const K_SANDBOX_MINIMUM_RESERVATION_SIZE: usize = 8589934592;
/// 沙箱内最大安全缓冲 = 32 GB - 1
pub const K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX: usize = 34359738367;

// ---- 有界大小编码 (v8-internal.h:282-293) ----
pub const K_BOUNDED_SIZE_SHIFT: u32 = 29;
/// (1 << (64 - 29)) - 1 = 2^35 - 1
pub const K_BOUNDED_SIZE_MASK: u64 = 34359738367;

// ---- 保护区域 (v8-internal.h:295-329) ----
/// 前后 guard = 32GB + (32GB-1 + 1) = 64 GB
pub const K_SANDBOX_GUARD_REGION_SIZE: usize = 68719476736;
/// 追加尾部 guard = 288GB - 64GB = 224 GB (Android/ChromeOS-ARM64/LOONG 为 0)
pub const K_ADDITIONAL_TRAILING_GUARD_REGION_SIZE: usize = 240518168576;
pub const K_REQUIRES_TYPED_ARRAY_ACCESS_MASKS: bool = false;

// ---- 外部指针表 (v8-internal.h:333-368, 默认分支) ----
/// Android 256MB/7 与 iOS 128MB/8 分支见原版, 目标取默认
pub const K_EXTERNAL_POINTER_TABLE_RESERVATION_SIZE: usize = 536870912;
pub const K_EXTERNAL_POINTER_INDEX_SHIFT: u32 = 6;
pub const K_EXTERNAL_POINTER_TABLE_ENTRY_SIZE: usize = 8;
pub const K_EXTERNAL_POINTER_TABLE_ENTRY_SIZE_LOG2: u32 = 3;
/// = 2**(32-6), 保证移位后索引必在表内
pub const K_MAX_EXTERNAL_POINTERS: usize = 67108864;

// ---- 外部指针 tag 位布局 (v8-internal.h:377-393, 非 LA57 分支) ----
/// LA57 分支 (x64 LA57): mark=1<<56, shift=57, tag=0xfe00000000000000, payload=0x00ffffffffffffff
pub const K_EXTERNAL_POINTER_MARK_BIT: u64 = 281474976710656; // 1<<48
pub const K_EXTERNAL_POINTER_TAG_SHIFT: u64 = 49; // 49
pub const K_EXTERNAL_POINTER_TAG_MASK: u64 = 71494644084506624; // 0x00fe000000000000
pub const K_EXTERNAL_POINTER_TAG_AND_MARKBIT_MASK: u64 = 71776119061217280; // 0x00ff000000000000
pub const K_EXTERNAL_POINTER_PAYLOAD_MASK: u64 = 18374967954648334335; // 0xff00ffffffffffff
/// tagMask >> tagShift = 0x7f (7 位 tag)
pub const K_EXTERNAL_POINTER_SHIFTED_TAG_MASK: u64 = 127;

// ---- tag 数量宏 (v8-internal.h:469-474) ----
pub const V8_EMBEDDER_DATA_TAG_COUNT: u32 = 15;
pub const V8_EXTERNAL_POINTER_TAG_COUNT: u32 = 40;
