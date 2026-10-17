// 指针编码往返测试: 对应 v8/test/unittests/sandbox/ 编解码语义
// (pointer-table-unittest 的 entry 编解码面 + sandbox-unittest 常量面)

use aok::{OK, Void};
use log::info;
use v8_sandbox::{
  ExternalPointerPayload, ExternalPointerTag, ExternalPointerTagRange,
  K_ANY_EXTERNAL_POINTER_TAG_RANGE, K_ANY_FOREIGN_EXTERNAL_POINTER_TAG_RANGE,
  K_ANY_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG_RANGE, K_ANY_MANAGED_EXTERNAL_POINTER_TAG_RANGE,
  K_ANY_MANAGED_RESOURCE_EXTERNAL_POINTER_TAG_RANGE, K_BOUNDED_SIZE_SHIFT,
  K_EXTERNAL_POINTER_INDEX_SHIFT, K_EXTERNAL_POINTER_MARK_BIT, K_EXTERNAL_POINTER_PAYLOAD_MASK,
  K_EXTERNAL_POINTER_TAG_SHIFT, K_MAX_EXTERNAL_POINTERS, K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX,
  K_NULL_EXTERNAL_POINTER_HANDLE, K_SANDBOX_SIZE, K_SANDBOXED_POINTER_SHIFT,
  V8_EMBEDDER_DATA_TAG_COUNT, V8_EXTERNAL_POINTER_TAG_COUNT, external_pointer_can_be_empty,
  handle_to_index, index_to_handle, is_managed_external_pointer_type,
  is_maybe_read_only_external_pointer_type, is_shared_external_pointer_type, is_valid_handle,
  read_bounded_size_field, read_sandboxed_pointer_field, write_bounded_size_field,
  write_sandboxed_pointer_field,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// tag 锚点对数 (v8-internal.h 显式值 + 派生关系)
#[test]
fn tag_anchor_values() -> Void {
  assert_eq!(ExternalPointerTag::ExternalPointerNullTag as u16, 0);
  assert_eq!(ExternalPointerTag::WaiterQueueNodeTag as u16, 1);
  assert_eq!(ExternalPointerTag::ExternalStringResourceDataTag as u16, 3);
  assert_eq!(ExternalPointerTag::ArrayBufferExtensionTag as u16, 100);
  assert_eq!(
    ExternalPointerTag::ExternalPointerZappedEntryTag as u16,
    0x7d
  );
  assert_eq!(
    ExternalPointerTag::ExternalPointerEvacuationEntryTag as u16,
    0x7e
  );
  assert_eq!(ExternalPointerTag::ExternalPointerFreeEntryTag as u16, 0x7f);
  assert_eq!(V8_EMBEDDER_DATA_TAG_COUNT, 15);
  assert_eq!(V8_EXTERNAL_POINTER_TAG_COUNT, 40);
  // 嵌入段 15 / 外部类型段 40 / 拦截器段 16
  assert_eq!(
    ExternalPointerTag::LastEmbedderDataTag as u16
      - ExternalPointerTag::FirstEmbedderDataTag as u16,
    14
  );
  assert_eq!(
    ExternalPointerTag::FastApiExternalTypeTag as u16
      - ExternalPointerTag::FirstExternalTypeTag as u16,
    39
  );
  assert_eq!(
    ExternalPointerTag::ApiIndexedPropertyIterableToListCallbackTag as u16
      - ExternalPointerTag::ApiNamedPropertyQueryCallbackTag as u16,
    15
  );
  OK
}

/// TagRange 区间语义 (v8-internal.h TagRange)
#[test]
fn tag_range_semantics() -> Void {
  let foreign = K_ANY_FOREIGN_EXTERNAL_POINTER_TAG_RANGE;
  assert!(foreign.contains(ExternalPointerTag::GenericForeignTag));
  assert!(foreign.contains(ExternalPointerTag::IcuCollatorTag));
  assert!(!foreign.contains(ExternalPointerTag::WasmStackMemoryTag));
  assert!(!foreign.contains(ExternalPointerTag::ExternalPointerFreeEntryTag));
  assert_eq!(foreign.size(), 20);

  // 全域
  assert_eq!(K_ANY_EXTERNAL_POINTER_TAG_RANGE.size(), 128);
  assert!(K_ANY_EXTERNAL_POINTER_TAG_RANGE.contains(ExternalPointerTag::ExternalPointerNullTag));

  // 空 range 与 u16 下溢防护: tag < first 时 contains 须为 false
  let empty = ExternalPointerTagRange::new(
    ExternalPointerTag::ExternalPointerNullTag,
    ExternalPointerTag::ExternalPointerNullTag,
  );
  assert!(empty.is_empty());
  assert_eq!(empty.size(), 0);
  let mid = ExternalPointerTagRange::new(
    ExternalPointerTag::AccessorInfoGetterTag,
    ExternalPointerTag::AccessorInfoSetterTag,
  );
  assert!(!mid.contains(ExternalPointerTag::FunctionTemplateInfoCallbackTag));
  assert!(mid.contains(ExternalPointerTag::AccessorInfoSetterTag));

  // 单 tag 隐式转 range
  assert!(
    ExternalPointerTagRange::of(ExternalPointerTag::CFunctionTag)
      .contains(ExternalPointerTag::CFunctionTag)
  );
  OK
}

/// Handle 移位语义 (external-pointer-table-inl.h HandleToIndex / IndexToHandle)
#[test]
fn handle_shift_roundtrip() -> Void {
  assert_eq!(K_EXTERNAL_POINTER_INDEX_SHIFT, 6);
  assert_eq!(handle_to_index(K_NULL_EXTERNAL_POINTER_HANDLE), 0);
  for &index in &[
    0u32,
    1,
    63,
    64,
    0x1234,
    u32::MAX >> K_EXTERNAL_POINTER_INDEX_SHIFT,
  ] {
    let handle = index_to_handle(index);
    assert!(is_valid_handle(handle));
    assert_eq!(handle_to_index(handle), index);
  }
  // 低 6 位非零 → 非法 handle
  assert!(!is_valid_handle(1));
  assert!(!is_valid_handle(0x41));
  assert!(is_valid_handle(0x40));
  // 移位保证: 32 位 handle 右移后必在 512MB/8 表内
  assert_eq!(
    1u64 << (32 - K_EXTERNAL_POINTER_INDEX_SHIFT),
    K_MAX_EXTERNAL_POINTERS as u64
  );
  OK
}

/// payload 编解码往返 (tagged-payload.h Tag/CheckTag/Untag/markbit/freelist)
#[test]
fn payload_codec_roundtrip() -> Void {
  let ptr = 0x0000_1234_5678_9abc_usize;
  let tag = ExternalPointerTag::ExternalStringResourceDataTag;
  let payload = ExternalPointerPayload::new(ptr, tag);
  assert_eq!(
    payload.encoded_word,
    (ptr as u64) | ((tag as u64) << K_EXTERNAL_POINTER_TAG_SHIFT)
  );
  assert!(payload.is_tagged_with(tag));
  // 同 tag 解码往返
  let range = ExternalPointerTagRange::of(tag);
  assert_eq!(payload.untag_allow_null(range), ptr);
  assert_eq!(payload.untag_disallow_null(range), Some(ptr));
  assert_eq!(payload.untag(range), ptr);
  // 错 tag: allow-null 返回 0 / disallow 返回 None
  let wrong = ExternalPointerTagRange::of(ExternalPointerTag::CFunctionTag);
  assert_eq!(payload.untag_allow_null(wrong), 0);
  assert_eq!(payload.untag_disallow_null(wrong), None);
  assert_eq!(payload.extract_tag(), Some(tag));

  // mark 位
  let mut marked = payload;
  assert!(!marked.has_mark_bit_set());
  marked.set_mark_bit();
  assert!(marked.has_mark_bit_set());
  assert!(marked.is_tagged_with(tag), "mark 位不影响 tag 判定");
  assert_eq!(marked.untag_disallow_null(range), Some(ptr));
  marked.clear_mark_bit();
  assert!(!marked.has_mark_bit_set());

  // freelist 链: 编码 = next_index | free_tag<<49
  let mut free = ExternalPointerPayload::new(0, ExternalPointerTag::ExternalPointerNullTag);
  let next_index = 0x1234_u32;
  free.encoded_word = (next_index as u64 & K_EXTERNAL_POINTER_PAYLOAD_MASK)
    | ((ExternalPointerTag::ExternalPointerFreeEntryTag as u64) << K_EXTERNAL_POINTER_TAG_SHIFT);
  assert!(free.contains_freelist_link());
  assert_eq!(free.extract_freelist_link(), Some(next_index));
  assert!(!free.contains_pointer());
  // 普通表项含指针
  assert!(payload.contains_pointer());
  assert!(!payload.contains_evacuation_entry());
  OK
}

/// 范围谓词 (v8-internal.h IsShared/IsMaybeReadOnly/IsManaged/CanBeEmpty)
#[test]
fn tag_range_predicates() -> Void {
  assert!(is_shared_external_pointer_type(
    ExternalPointerTagRange::of(ExternalPointerTag::WaiterQueueNodeTag)
  ));
  assert!(is_shared_external_pointer_type(
    ExternalPointerTagRange::of(ExternalPointerTag::ExternalStringResourceDataTag)
  ));
  assert!(!is_shared_external_pointer_type(
    ExternalPointerTagRange::of(ExternalPointerTag::NativeContextMicrotaskQueueTag)
  ));

  assert!(is_maybe_read_only_external_pointer_type(
    ExternalPointerTagRange::of(ExternalPointerTag::AccessorInfoGetterTag)
  ));
  assert!(!is_maybe_read_only_external_pointer_type(
    ExternalPointerTagRange::of(ExternalPointerTag::CFunctionTag)
  ));

  assert!(is_managed_external_pointer_type(
    ExternalPointerTagRange::of(ExternalPointerTag::IcuCollatorTag)
  ));
  assert!(!is_managed_external_pointer_type(
    ExternalPointerTagRange::of(ExternalPointerTag::GenericForeignTag)
  ));

  // 可空: embedder 段 / 拦截器段 / 含 ArrayBufferExtension
  assert!(external_pointer_can_be_empty(ExternalPointerTagRange::new(
    ExternalPointerTag::FirstEmbedderDataTag,
    ExternalPointerTag::LastEmbedderDataTag
  )));
  assert!(external_pointer_can_be_empty(
    K_ANY_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG_RANGE
  ));
  assert!(external_pointer_can_be_empty(ExternalPointerTagRange::of(
    ExternalPointerTag::ArrayBufferExtensionTag
  )));
  assert!(!external_pointer_can_be_empty(ExternalPointerTagRange::of(
    ExternalPointerTag::CFunctionTag
  )));
  // 域内常量断言: managed resource 与 managed 段首尾一致
  assert_eq!(
    K_ANY_MANAGED_RESOURCE_EXTERNAL_POINTER_TAG_RANGE.first as u16,
    K_ANY_MANAGED_EXTERNAL_POINTER_TAG_RANGE.first as u16
  );
  OK
}

/// bounded-size 编码: 值 << 29 存储; 读回必 ≤ 32GB-1 (攻击者篡改高位不影响有界性)
#[test]
fn bounded_size_roundtrip() -> Void {
  assert_eq!(K_BOUNDED_SIZE_SHIFT, 29);
  assert_eq!(
    K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX,
    32 * 1024 * 1024 * 1024 - 1
  );
  for &value in &[
    0usize,
    1,
    4096,
    0xdead_beef,
    K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX,
  ] {
    let mut field: usize = 0;
    // SAFETY: field 为本地 8 字节可写存储。
    unsafe { write_bounded_size_field(&mut field, value) };
    // SAFETY: 同上, 可读。
    assert_eq!(unsafe { read_bounded_size_field(&field) }, value);
  }
  // 模拟沙箱内攻击者置位高 29 位: 读回仍受 35 位掩码约束 (有界性不变量)
  // (!0 << 35) >> 29 = 2^35 - 2^6 = 0x7FFFFFFFC0; 原值 2^29 保留为 bit0
  let mut field: usize = 0;
  // SAFETY: 本地存储。
  unsafe { write_bounded_size_field(&mut field, 1) };
  field |= !0usize << (64 - K_BOUNDED_SIZE_SHIFT); // 高 29 位全 1
  // SAFETY: 本地存储。
  let corrupted = unsafe { read_bounded_size_field(&field) };
  assert_eq!(corrupted, 0x7_FFFF_FFC1);
  assert!(corrupted <= K_MAX_SAFE_BUFFER_SIZE_FOR_SANDBOX);
  OK
}

/// sandboxed-pointer 编码: 偏移 << 24 存储; 读 = cage_base + (raw >> 24);
/// 越界指针 (>= cage_base + 1TB) 写入被拒
#[test]
fn sandboxed_pointer_roundtrip() -> Void {
  assert_eq!(K_SANDBOXED_POINTER_SHIFT, 24);
  let cage_base: usize = 1 << 40; // 1TB 对齐 (kSandboxAlignment 语义)
  for &offset in &[0usize, 1, 0x1234_5678, K_SANDBOX_SIZE - 1] {
    let mut field: u64 = 0;
    let pointer = cage_base + offset;
    // SAFETY: 本地 8 字节存储。
    assert!(unsafe { write_sandboxed_pointer_field(&mut field, cage_base, pointer) }.is_ok());
    // SAFETY: 本地存储。
    let decoded = unsafe { read_sandboxed_pointer_field(&field, cage_base) };
    assert_eq!(decoded, pointer);
  }
  // 沙箱外指针: 低于 cage_base / 超出 1TB → SandboxViolation
  let mut field: u64 = 0;
  // SAFETY: 本地存储。
  assert!(unsafe { write_sandboxed_pointer_field(&mut field, cage_base, cage_base - 1) }.is_err());
  // SAFETY: 本地存储。
  assert!(
    unsafe { write_sandboxed_pointer_field(&mut field, cage_base, cage_base + K_SANDBOX_SIZE) }
      .is_err()
  );
  // 篡改后的编码值读回仍在 cage_base + [0, 1TB) 内 (安全不变量)
  let mut corrupted: u64 = 0;
  // SAFETY: 本地存储。
  unsafe { write_sandboxed_pointer_field(&mut corrupted, cage_base, cage_base + 1) }.ok();
  corrupted = u64::MAX;
  // SAFETY: 本地存储。
  let decoded = unsafe { read_sandboxed_pointer_field(&corrupted, cage_base) };
  assert!(decoded >= cage_base && decoded < cage_base + K_SANDBOX_SIZE);
  OK
}

/// mark 位常量与 payload 位布局对数 (v8-internal.h 非 LA57 分支)
#[test]
fn payload_bit_layout() -> Void {
  info!("payload 位布局对数");
  assert_eq!(K_EXTERNAL_POINTER_TAG_SHIFT, 49);
  assert_eq!(K_EXTERNAL_POINTER_MARK_BIT, 1u64 << 48);
  // tag 位 49..55, mark 位 48, payload 低 48 位 + 顶字节
  let word = ExternalPointerPayload::new(
    0x0000_ffff_ffff_ffff,
    ExternalPointerTag::ExternalPointerFreeEntryTag,
  )
  .encoded_word;
  assert_eq!(word & 0x00ff_0000_0000_0000, (0x7f_u64) << 49);
  OK
}
