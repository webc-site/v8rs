// 由 tool/src/objects/pod-array.h.js 从 v8/src/objects/pod-array.{h,-inl.h} 生成; 修订就地改, 复跑覆盖
// PodArray 族: memcpy 语义基座 + 字节长换算

//! PodArray: 任意可 memcpy 的 C++ 类型 (POD) 装箱到 ByteArray / TrustedByteArray。
//! Rust 侧以泛型承载: PodArrayBase<T, Super> → 参数化宿主字节数组。

/// 元素数量换算 (原版 PodArrayBase::length = Super 字节长 / sizeof(T))。
#[inline]
#[must_use]
pub const fn pod_length(super_byte_length: usize, elem_size: usize) -> usize {
  super_byte_length / elem_size
}

/// 字节长换算 + u32 溢出检查 (原版 New: CheckedNumeric<Length> *= sizeof(T),
/// AssignIfValid 失败即 CHECK 崩溃 → Rust 侧返回 None 由调用方决定)。
#[inline]
#[must_use]
pub const fn pod_byte_length(length: u32, elem_size: usize) -> Option<u32> {
  let bytes = (length as u64) * (elem_size as u64);
  if bytes > u32::MAX as u64 {
    None
  } else {
    Some(bytes as u32)
  }
}

/// PodArrayBase 方法清单 (memcpy 族 + length 换算; 语义就地修订进宿主实现)。
/// v8: src/objects/pod-array.h — 下标均 DCHECK 越界后走 values() 偏移:
pub const POD_ARRAY_BASE_METHODS: &[(&str, &str)] = &[
  (
    "copy_out",
    "MemCopy(result, &values[index * sizeof(T)], length * sizeof(T))",
  ),
  (
    "copy_in",
    "MemCopy(&values[index * sizeof(T)], buffer, length * sizeof(T))",
  ),
  (
    "matches(0)",
    "memcmp(begin, buffer, length * sizeof(T)) == 0",
  ),
  (
    "matches(offset)",
    "memcmp(begin + sizeof(T) * offset, buffer, length * sizeof(T)) == 0",
  ),
  ("get", "copy_out(index, &result, 1) 单元素读"),
  ("set", "copy_in(index, &value, 1) 单元素写"),
  ("length", "Super::length().value() / sizeof(T) 元素数换算"),
];

/// New 重载清单 (4 形态; 分配类型登记)。
pub const POD_ARRAY_NEW_SIGNATURES: &[(&str, &str, &str)] = &[
  (
    "PodArray<T>",
    "Isolate",
    "AllocationType::kYoung (默认) → factory NewByteArray",
  ),
  (
    "PodArray<T>",
    "LocalIsolate",
    "AllocationType::kOld (默认) → factory NewByteArray",
  ),
  (
    "TrustedPodArray<T>",
    "Isolate",
    "factory NewTrustedByteArray",
  ),
  (
    "TrustedPodArray<T>",
    "LocalIsolate",
    "factory NewTrustedByteArray",
  ),
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(pod_length(100, 4) == 25);
  assert!(pod_length(7, 4) == 1);
  assert!(matches!(pod_byte_length(10, 8), Some(80)));
  assert!(pod_byte_length(u32::MAX, 8).is_none());
  assert!(matches!(pod_byte_length(0, 8), Some(0)));
  assert!(POD_ARRAY_BASE_METHODS.len() == 7);
  assert!(POD_ARRAY_NEW_SIGNATURES.len() == 4);
};
