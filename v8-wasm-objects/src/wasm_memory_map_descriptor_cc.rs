// 由 tool/src/wasm/wasm-memory-map-descriptor.cc.js 从 v8/src/wasm/wasm-memory-map-descriptor.cc 生成; 修订就地改, 复跑覆盖
// 内存映射描述符平台门控与映射检查清单

//! WasmMemoryMapDescriptor: 文件描述符映射进 wasm 线性内存。

/// 平台门控: 仅 Linux 且非 Android 实现, 其余平台 Map/Unmap/NewFromAnonymous 直接失败
/// v8: src/wasm/wasm-memory-map-descriptor.cc V8_OS_LINUX && !V8_OS_ANDROID
pub const PLATFORM_GATE: &str = "V8_OS_LINUX && !V8_OS_ANDROID";

/// 匿名 fd 的 memfd 名与标志
/// v8: src/wasm/wasm-memory-map-descriptor.cc NewFromAnonymous
pub const MEMFD_NAME: &str = "wasm_memory_map_descriptor";
pub const MEMFD_FLAGS: &str = "MFD_CLOEXEC";
/// V8 拥有的匿名 fd 在析构时关闭 (-1 为无效哨兵)
/// v8: src/wasm/wasm-memory-map-descriptor.cc ~WasmMemoryMapDescriptor
pub const INVALID_FD: i32 = -1;

/// 双次映射错误消息
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
pub const DOUBLE_MAP_ERROR: &str = "WasmMemoryMapDescriptor::Map called more than once";

/// Map 拒绝条件 (返回 0, 共 6 条)
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
#[rustfmt::skip]
pub static MAP_REJECTS: &[&str] = &[
    "已映射过 (双次调用)",
    "共享内存 (并发未处理)",
    "memory64 (待测试)",
    "fstat 取文件大小失败",
    "size + offset 溢出",
    "size + offset 越界 backing_store.byte_length",
];

/// 文件大小向上取整到页分配器页宽后才映射
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
pub const SIZE_ROUNDING: &str = "RoundUp(st_size, AllocatePageSize)";

/// mmap 标志: Map 用 PROT_READ|PROT_WRITE + MAP_FIXED|MAP_SHARED;
/// Unmap 用 MAP_FIXED|MAP_SHARED|MAP_ANONYMOUS 覆盖回退
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map / Unmap
pub const MMAP_PROT: &str = "PROT_READ | PROT_WRITE";
pub const MMAP_FLAGS_MAP: &str = "MAP_FIXED | MAP_SHARED";
pub const MMAP_FLAGS_UNMAP: &str = "MAP_FIXED | MAP_SHARED | MAP_ANONYMOUS";

/// Unmap 复查清单 (Map 时已通过的条件, 共 5 条)
/// v8: src/wasm/wasm-memory-map-descriptor.cc Unmap
#[rustfmt::skip]
pub static UNMAP_RECHECKS: &[&str] = &[
    "非 memory64",
    "非共享 backing store",
    "size 页对齐",
    "size + offset 无溢出",
    "size + offset 界内",
];

/// 映射后以弱引用持有 memory 对象, 实例可被回收
/// v8: src/wasm/wasm-memory-map-descriptor.cc Map
pub const HOLD_WEAK_MEMORY: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert_eq!(MAP_REJECTS.len(), 6);
    assert_eq!(UNMAP_RECHECKS.len(), 5);
    assert_eq!(INVALID_FD, -1);
};
