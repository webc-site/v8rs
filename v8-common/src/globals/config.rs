//! globals.h 配置布尔展开
//!
//! 目标配置: x64 + arm64, V8_COMPRESS_POINTERS / V8_COMPRESS_POINTERS_IN_SHARED_CAGE
//! / V8_ENABLE_SANDBOX / V8_EXTERNAL_CODE_SPACE 开; 其余实验特性关。
//! 每条登记原版 globals.h 行号与展开依据。平台相关条目用 cfg! 运行时常量。

use crate::internal::{K_API_SYSTEM_POINTER_SIZE, K_API_TAGGED_SIZE};

/// 原版 globals.h:48-79 USE_SIMULATOR — 目标架构即宿主架构, 不模拟
pub const USE_SIMULATOR: bool = false;

/// 原版 globals.h:83-87 V8_EMBEDDED_CONSTANT_POOL — 仅 PPC64
pub const V8_EMBEDDED_CONSTANT_POOL: bool = false;

/// 原版 globals.h:89-93 DEBUG_BOOL — DEBUG 宏等价物: debug 构建为真
pub const DEBUG_BOOL: bool = cfg!(debug_assertions);

/// 原版 globals.h:95-99 V8_MAP_PACKING — 实验特性, 关
pub const V8_MAP_PACKING: bool = false;

/// 原版 globals.h:101-105 COMPRESS_POINTERS_BOOL — 目标配置开
pub const COMPRESS_POINTERS: bool = true;

/// 原版 globals.h:107-111 V8_ENABLE_GEARBOX — V8_BUILTINS_GEARBOX 未定义
pub const V8_ENABLE_GEARBOX: bool = false;

/// 原版 globals.h:113-117 — COMPRESS_POINTERS && x64; arm64 为 false
pub const DECOMPRESS_POINTER_BY_ADDRESSING_MODE: bool = cfg!(target_arch = "x86_64");

/// 原版 globals.h:119-123 — sandbox 要求 shared cage
pub const COMPRESS_POINTERS_IN_SHARED_CAGE: bool = true;

/// 原版 globals.h:125-129 — 实验特性, 关
pub const CONTIGUOUS_COMPRESSED_READ_ONLY_SPACE: bool = false;

/// 原版 globals.h:131-135 — shared cage 的补集
pub const COMPRESS_POINTERS_IN_MULTIPLE_CAGES: bool =
  COMPRESS_POINTERS && !COMPRESS_POINTERS_IN_SHARED_CAGE;

/// 原版 globals.h:137-141 — V8_DISABLE_WRITE_BARRIERS 未定义
pub const V8_CAN_CREATE_SHARED_HEAP: bool = true;

/// 原版 globals.h:143-147 — 默认关
pub const V8_LOWER_LIMITS_MODE: bool = false;

/// 原版 globals.h:149-153 — 仅构建期
pub const V8_STATIC_ROOTS_GENERATION: bool = false;

/// 原版 globals.h:155-159 — COMPRESS_POINTERS 开
pub const V8_STATIC_DISPATCH_HANDLES: bool = COMPRESS_POINTERS;

/// 原版 globals.h:161-168 — 目标配置开
pub const V8_ENABLE_SANDBOX: bool = true;

/// 原版 globals.h:161-168 — 随 sandbox
pub const V8_JS_LINKAGE_INCLUDES_DISPATCH_HANDLE: bool = V8_ENABLE_SANDBOX;

/// 原版 globals.h:170-174 — 默认关
pub const ENABLE_CONTROL_FLOW_INTEGRITY: bool = false;

/// 原版 globals.h:244-249 — EXTERNAL_CODE_SPACE 开
/// SHORT_BUILTIN_CALLS 为 GN 构建参数 (V8_SHORT_BUILTIN_CALLS), 非 cargo feature;
/// 目标配置未开, 落常量。外部代码空间开 → 表达式恒真。
pub const SHORT_BUILTIN_CALLS: bool = false;
pub const V8_ENABLE_NEAR_CODE_RANGE: bool =
  (SHORT_BUILTIN_CALLS && !COMPRESS_POINTERS) || V8_EXTERNAL_CODE_SPACE;

/// 原版 globals.h:256-260 — 上游默认 false (BUILD.gn:425), 非目标配置决策项,
/// 若后续 v8-flags/v8-objects-collections 语义需要再按真实 gn args 升
pub const V8_ENABLE_SWISS_NAME_DICTIONARY: bool = false;

/// 原版 globals.h:263-267 — 实验特性, 关
pub const V8_DICT_PROPERTY_CONST_TRACKING: bool = false;

/// 原版 globals.h:269-273 — sandbox 要求
pub const V8_EXTERNAL_CODE_SPACE: bool = true;

/// 原版 globals.h:276-281 — ENABLE_DISASSEMBLER 未开
pub const V8_JUMP_TABLE_INFO: bool = false;

/// 原版 globals.h:311-316 — 宿主 macOS arm64 且 (CP && !ECS) 不成立
pub const V8_HEAP_USE_PTHREAD_JIT_WRITE_PROTECT: bool =
  cfg!(all(target_os = "macos", target_arch = "aarch64"))
    && !(COMPRESS_POINTERS && !V8_EXTERNAL_CODE_SPACE);

/// 原版 globals.h:319-324 — iOS 17.4+ 宿主, 非目标
pub const V8_HEAP_USE_BECORE_JIT_WRITE_PROTECT: bool = false;

/// 原版 globals.h:327-332 — 宿主 Linux x64 且 (CP && !ECS) 不成立
pub const V8_HEAP_USE_PKU_JIT_WRITE_PROTECT: bool =
  cfg!(all(target_os = "linux", target_arch = "x86_64"))
    && !(COMPRESS_POINTERS && !V8_EXTERNAL_CODE_SPACE);

/// 原版 V8_HAS_PKU_JIT_WRITE_PROTECT (base/build_config.h:30-37) —
/// V8_HAS_PKU_SUPPORT = 宿主 Linux && 宿主 x64, 该宏直接取其值。
/// 注意: 原版按宿主架构展开 (V8_HOST_ARCH_X64), Rust cfg! 按目标三元组;
/// v8rs 用法宿主=目标恒成立 (交叉编译场景差异登记 readme)。
/// 区别于 V8_HEAP_USE_PKU_JIT_WRITE_PROTECT (globals.h:327-332, 含 CP 组合
/// 条件): 本常量仅平台判定, 是 THREAD_ISOLATION_ALIGN_SZ (code-memory-access.h:43-66)
/// 与 RwxMemoryWriteScope::memory_protection_key (cc:21-34) 的门控条件。
pub const V8_HAS_PKU_JIT_WRITE_PROTECT: bool =
  cfg!(all(target_os = "linux", target_arch = "x86_64"));

/// 原版 globals.h:336-340 — 指针压缩开 → tagged 4 字节
pub const TAGGED_SIZE_8_BYTES: bool = K_API_TAGGED_SIZE == 8 && !COMPRESS_POINTERS;

/// 原版 globals.h:342-352 V8_OS_WIN64 — 按目标平台展开
pub const V8_OS_WIN_X64: bool = cfg!(all(target_os = "windows", target_arch = "x86_64"));
pub const V8_OS_WIN_ARM64: bool = cfg!(all(target_os = "windows", target_arch = "aarch64"));
pub const V8_OS_WIN64: bool = V8_OS_WIN_X64 || V8_OS_WIN_ARM64;

/// 原版 globals.h:358-362 — x64/arm64 均在支持列表
pub const V8_ENABLE_FP_PARAMS_IN_C_LINKAGE: bool =
  cfg!(any(target_arch = "x86_64", target_arch = "aarch64"));

/// 原版 globals.h:364-368 — 实验特性, 关
pub const V8_UNDEFINED_DOUBLE: bool = false;

/// 原版 globals.h:370-380 — 实验特性, 关
pub const V8_EXPERIMENTAL_TSA_BUILTINS: bool = false;
pub const V8_EXPERIMENTAL_TQ_TO_TSA: bool = false;

/// 原版 globals.h:382-386 — 默认关
pub const V8_X64_16BYTE_STACK_ALIGNMENT: bool = false;

/// 原版 globals.h:679-684 — 默认关
pub const V8_COMPRESS_POINTERS_8GB: bool = false;

/// 原版 globals.h:807-819 — 仅 PPC64/AIX
pub const USES_FUNCTION_DESCRIPTORS: bool = false;

/// 编译期校验: kApiTaggedSize 映射正确 (原版 globals.h:670 static_assert)
const _: () = assert!(K_API_TAGGED_SIZE == 4 && K_API_SYSTEM_POINTER_SIZE == 8);
