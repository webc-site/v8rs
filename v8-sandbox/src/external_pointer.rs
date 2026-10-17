// 由 tool/src/sandbox/external-pointer.js 从 v8/src/sandbox/external-pointer{,-inl}.h + include/v8-internal.h(tag 段) + src/sandbox/tagged-payload.h(并入) + external-pointer-table{.h,-inl}.h(scheme/handle 位运算段) 生成; 修订就地改, 复跑覆盖
// 外部指针: tag 枚举/范围谓词 + Handle 移位语义 + payload 编解码位运算 1:1

//! 外部指针编码层。
//! 本版 V8 无独立 external-pointer-tag.h: tag 定义在 include/v8-internal.h, 由本模块承载。
//! tagged-payload.h 的 TaggedPayload 位运算以 ExternalPointerTable scheme 实例化并入
//! (泛型 scheme 层待 code-pointer/trusted-pointer 表族第二梯队落地再抽)。
//! Isolate 绑定 (InitExternalPointerField 等走 ExternalPointerTable 的接口) 登记第二梯队。
//! 生成脚本内置对数核对 14 条全部通过。

use crate::internal::{
  Address, K_EXTERNAL_POINTER_INDEX_SHIFT, K_EXTERNAL_POINTER_MARK_BIT,
  K_EXTERNAL_POINTER_PAYLOAD_MASK, K_EXTERNAL_POINTER_TAG_MASK, K_EXTERNAL_POINTER_TAG_SHIFT,
  K_NULL_ADDRESS,
};

// ---- ExternalPointerTag (v8-internal.h:692-771, 全 tag 逐值计算) ----
/// 7 位 tag (0..=0x7f); mark 位占 1 位, 两者合占 64 位字的高 15 位
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr)]
pub enum ExternalPointerTag {
  ExternalPointerNullTag = 0,
  WaiterQueueNodeTag = 1,
  ExternalStringResourceTag = 2,
  ExternalStringResourceDataTag = 3,
  NativeContextMicrotaskQueueTag = 4,
  FirstEmbedderDataTag = 5,
  LastEmbedderDataTag = 19,
  FirstExternalTypeTag = 20,
  FastApiExternalTypeTag = 59,
  FunctionTemplateInfoCallbackTag = 60,
  AccessorInfoGetterTag = 61,
  AccessorInfoSetterTag = 62,
  ApiNamedPropertyQueryCallbackTag = 63,
  ApiNamedPropertyGetterCallbackTag = 64,
  ApiNamedPropertySetterCallbackTag = 65,
  ApiNamedPropertyDescriptorCallbackTag = 66,
  ApiNamedPropertyDefinerCallbackTag = 67,
  ApiNamedPropertyDeleterCallbackTag = 68,
  ApiNamedPropertyEnumeratorCallbackTag = 69,
  ApiIndexedPropertyQueryCallbackTag = 70,
  ApiIndexedPropertyGetterCallbackTag = 71,
  ApiIndexedPropertySetterCallbackTag = 72,
  ApiIndexedPropertyDescriptorCallbackTag = 73,
  ApiIndexedPropertyDefinerCallbackTag = 74,
  ApiIndexedPropertyDeleterCallbackTag = 75,
  ApiIndexedPropertyEnumeratorCallbackTag = 76,
  ApiIndexedPropertyIndexOfCallbackTag = 77,
  ApiIndexedPropertyIterableToListCallbackTag = 78,
  WasmStackMemoryTag = 79,
  GenericForeignTag = 80,
  ApiAccessCheckCallbackTag = 81,
  ApiAbortScriptExecutionCallbackTag = 82,
  ApiTemporalHostSystemUTCEpochNanosecondsCallbackTag = 83,
  CFunctionTag = 84,
  SyntheticModuleTag = 85,
  MicrotaskCallbackTag = 86,
  MicrotaskCallbackDataTag = 87,
  MessageListenerTag = 88,
  WaiterQueueForeignTag = 89,
  WasmFutexManagedObjectWaitListTag = 90,
  IcuBreakIteratorTag = 91,
  IcuListFormatterTag = 92,
  IcuLocaleTag = 93,
  IcuSimpleDateFormatTag = 94,
  IcuDateIntervalFormatTag = 95,
  IcuRelativeDateTimeFormatterTag = 96,
  IcuLocalizedNumberFormatterTag = 97,
  IcuPluralRulesTag = 98,
  IcuCollatorTag = 99,
  ArrayBufferExtensionTag = 100,
  ExternalPointerZappedEntryTag = 125,
  ExternalPointerEvacuationEntryTag = 126,
  ExternalPointerFreeEntryTag = 127,
}

// ---- 范围标记别名 (原版枚举 kFirst*/kLast* 成员, 值与对应变体相同) ----
pub const K_FIRST_EXTERNAL_POINTER_TAG: ExternalPointerTag =
  ExternalPointerTag::ExternalPointerNullTag;
pub const K_FIRST_SHARED_EXTERNAL_POINTER_TAG: ExternalPointerTag =
  ExternalPointerTag::WaiterQueueNodeTag;
pub const K_LAST_SHARED_EXTERNAL_POINTER_TAG: ExternalPointerTag =
  ExternalPointerTag::ExternalStringResourceDataTag;
pub const K_LAST_EXTERNAL_TYPE_TAG: ExternalPointerTag = ExternalPointerTag::FastApiExternalTypeTag;
pub const K_FIRST_MAYBE_READ_ONLY_EXTERNAL_POINTER_TAG: ExternalPointerTag =
  ExternalPointerTag::FunctionTemplateInfoCallbackTag;
pub const K_FIRST_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG: ExternalPointerTag =
  ExternalPointerTag::ApiNamedPropertyQueryCallbackTag;
pub const K_LAST_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG: ExternalPointerTag =
  ExternalPointerTag::ApiIndexedPropertyIterableToListCallbackTag;
pub const K_LAST_MAYBE_READ_ONLY_EXTERNAL_POINTER_TAG: ExternalPointerTag =
  ExternalPointerTag::ApiIndexedPropertyIterableToListCallbackTag;
pub const K_LAST_MANAGED_RESOURCE_TAG: ExternalPointerTag =
  ExternalPointerTag::ArrayBufferExtensionTag;
pub const K_LAST_EXTERNAL_POINTER_TAG: ExternalPointerTag =
  ExternalPointerTag::ExternalPointerFreeEntryTag;

// ---- ExternalPointerTagRange (v8-internal.h:535-595 TagRange) ----
/// 闭区间 [first, last]; u32 提升防 u16 减法下溢 (原版注释语义)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalPointerTagRange {
  pub first: ExternalPointerTag,
  pub last: ExternalPointerTag,
}

impl ExternalPointerTagRange {
  /// 原版 TagRange(first, last)
  pub const fn new(first: ExternalPointerTag, last: ExternalPointerTag) -> Self {
    Self { first, last }
  }
  /// 原版 TagRange(tag): 单 tag 隐式转范围
  pub const fn of(tag: ExternalPointerTag) -> Self {
    Self {
      first: tag,
      last: tag,
    }
  }
  /// 仅含 null tag 视为空 (matches! 为 const 语境下 enum 判等的唯一途径)
  pub const fn is_empty(&self) -> bool {
    matches!(self.first, ExternalPointerTag::ExternalPointerNullTag)
      && matches!(self.last, ExternalPointerTag::ExternalPointerNullTag)
  }
  pub const fn size(&self) -> u32 {
    if self.is_empty() {
      0
    } else {
      (self.last as u32) - (self.first as u32) + 1
    }
  }
  pub const fn contains(&self, tag: ExternalPointerTag) -> bool {
    (tag as u32).wrapping_sub(self.first as u32)
      <= (self.last as u32).wrapping_sub(self.first as u32)
  }
  pub const fn contains_range(&self, r: Self) -> bool {
    (r.first as u32) >= (self.first as u32) && (r.last as u32) <= (self.last as u32)
  }
}

// ---- 预定义范围 (v8-internal.h:792-841, X-macro GET_FIRST/GET_LAST 展开) ----
pub const K_ANY_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange = ExternalPointerTagRange::new(
  ExternalPointerTag::ExternalPointerNullTag,
  ExternalPointerTag::ExternalPointerFreeEntryTag,
);
pub const K_ANY_FOREIGN_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange =
  ExternalPointerTagRange::new(
    ExternalPointerTag::GenericForeignTag,
    ExternalPointerTag::IcuCollatorTag,
  );
pub const K_ANY_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange =
  ExternalPointerTagRange::new(
    ExternalPointerTag::ApiNamedPropertyQueryCallbackTag,
    ExternalPointerTag::ApiIndexedPropertyIterableToListCallbackTag,
  );
pub const K_ANY_MANAGED_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange =
  ExternalPointerTagRange::new(
    ExternalPointerTag::WasmFutexManagedObjectWaitListTag,
    ExternalPointerTag::IcuCollatorTag,
  );
pub const K_ANY_MAYBE_READ_ONLY_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange =
  ExternalPointerTagRange::new(
    ExternalPointerTag::FunctionTemplateInfoCallbackTag,
    ExternalPointerTag::ApiIndexedPropertyIterableToListCallbackTag,
  );
pub const K_ANY_MANAGED_RESOURCE_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange =
  ExternalPointerTagRange::new(
    ExternalPointerTag::WasmFutexManagedObjectWaitListTag,
    ExternalPointerTag::ArrayBufferExtensionTag,
  );
pub const K_ANY_SHARED_MANAGED_EXTERNAL_POINTER_TAG_RANGE: ExternalPointerTagRange =
  ExternalPointerTagRange::new(
    ExternalPointerTag::WasmFutexManagedObjectWaitListTag,
    ExternalPointerTag::WasmFutexManagedObjectWaitListTag,
  );

/// 原版 IsSharedExternalPointerType: shared 段 (WaiterQueueNode..ExternalStringResourceData)
/// 或 shared managed 段 → 走共享 Isolate 的外部指针表
pub const fn is_shared_external_pointer_type(tag_range: ExternalPointerTagRange) -> bool {
  const SHARED: ExternalPointerTagRange = ExternalPointerTagRange::new(
    ExternalPointerTag::WaiterQueueNodeTag,
    ExternalPointerTag::ExternalStringResourceDataTag,
  );
  SHARED.contains_range(tag_range)
    || K_ANY_SHARED_MANAGED_EXTERNAL_POINTER_TAG_RANGE.contains_range(tag_range)
}

/// 原版 IsMaybeReadOnlyExternalPointerType
pub const fn is_maybe_read_only_external_pointer_type(tag_range: ExternalPointerTagRange) -> bool {
  K_ANY_MAYBE_READ_ONLY_EXTERNAL_POINTER_TAG_RANGE.contains_range(tag_range)
}

/// 原版 IsManagedExternalPointerType: 生命周期绑定表项 (ManagedResource 族)
pub const fn is_managed_external_pointer_type(tag_range: ExternalPointerTagRange) -> bool {
  K_ANY_MANAGED_RESOURCE_EXTERNAL_POINTER_TAG_RANGE.contains_range(tag_range)
}

/// 原版 ExternalPointerCanBeEmpty: 范围含 ArrayBufferExtension / 与 embedder 段相交 /
/// 属 InterceptorInfo 段时, 允许 null handle (需同时放行 null tag)
pub const fn external_pointer_can_be_empty(tag_range: ExternalPointerTagRange) -> bool {
  tag_range.contains(ExternalPointerTag::ArrayBufferExtensionTag)
    || ((tag_range.first as u32) <= (ExternalPointerTag::LastEmbedderDataTag as u32)
      && (ExternalPointerTag::FirstEmbedderDataTag as u32) <= (tag_range.last as u32))
    || K_ANY_INTERCEPTOR_INFO_EXTERNAL_POINTER_TAG_RANGE.contains_range(tag_range)
}

// ---- ExternalPointerHandle (v8-internal.h:395-414) ----
/// 不透明 32 位句柄 = 表索引左移 6 位 (移位保证索引必在表预留内)
pub type ExternalPointerHandle = u32;
pub const K_NULL_EXTERNAL_POINTER_HANDLE: ExternalPointerHandle = 0;

/// 原版 ExternalPointerTable::HandleToIndex (external-pointer-table-inl.h:355)
#[inline]
pub const fn handle_to_index(handle: ExternalPointerHandle) -> u32 {
  handle >> K_EXTERNAL_POINTER_INDEX_SHIFT
}

/// 原版 ExternalPointerTable::IndexToHandle (external-pointer-table-inl.h:375)
#[inline]
pub const fn index_to_handle(index: u32) -> ExternalPointerHandle {
  index << K_EXTERNAL_POINTER_INDEX_SHIFT
}

/// 原版 IsValidHandle (external-pointer-table-inl.h:349): 移位还原后须等于原值
#[inline]
pub const fn is_valid_handle(handle: ExternalPointerHandle) -> bool {
  handle == handle >> K_EXTERNAL_POINTER_INDEX_SHIFT << K_EXTERNAL_POINTER_INDEX_SHIFT
}

// ---- ExternalPointerPayload (tagged-payload.h + ExternalPointerTablePayloadTaggingScheme) ----
/// 表项 payload 编码: pointer | (tag << 49); mark 位 1<<48;
/// free 表项: payload 低 48 位 = freelist 下一索引
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalPointerPayload {
  pub encoded_word: u64,
}

impl ExternalPointerPayload {
  pub const FREE_ENTRY_TAG: ExternalPointerTag = ExternalPointerTag::ExternalPointerFreeEntryTag;
  pub const EVACUATION_ENTRY_TAG: ExternalPointerTag =
    ExternalPointerTag::ExternalPointerEvacuationEntryTag;

  /// 原版 TaggedPayload::Tag (tagged-payload.h:38-41); kPayloadShift == 0
  #[inline]
  pub fn tag(pointer: Address, tag: ExternalPointerTag) -> u64 {
    (pointer as u64) | ((tag as u64) << K_EXTERNAL_POINTER_TAG_SHIFT)
  }

  /// 原版 TaggedPayload 构造
  #[inline]
  pub fn new(pointer: Address, tag: ExternalPointerTag) -> Self {
    Self {
      encoded_word: Self::tag(pointer, tag),
    }
  }

  /// 原版 CheckTag (tagged-payload.h:43-48)
  #[inline]
  pub fn check_tag(content: u64, tag_range: ExternalPointerTagRange) -> bool {
    let tag = ((content & K_EXTERNAL_POINTER_TAG_MASK) >> K_EXTERNAL_POINTER_TAG_SHIFT) as u16;
    match ExternalPointerTag::from_repr(tag) {
      Some(t) => tag_range.contains(t),
      // 超出 7 位 tag 域的编码视为不匹配
      None => false,
    }
  }

  /// 原版 UntagAllowNullHandle: tag 不匹配返回 0
  #[inline]
  pub fn untag_allow_null(&self, tag_range: ExternalPointerTagRange) -> Address {
    if Self::check_tag(self.encoded_word, tag_range) {
      (self.encoded_word & K_EXTERNAL_POINTER_PAYLOAD_MASK) as Address
    } else {
      K_NULL_ADDRESS
    }
  }

  /// 原版 UntagDisallowNullHandle (SBXCHECK → None, 不 panic)
  #[inline]
  pub fn untag_disallow_null(&self, tag_range: ExternalPointerTagRange) -> Option<Address> {
    if Self::check_tag(self.encoded_word, tag_range) {
      Some((self.encoded_word & K_EXTERNAL_POINTER_PAYLOAD_MASK) as Address)
    } else {
      None
    }
  }

  /// 原版 Untag: 可空范围走 allow-null 路径
  #[inline]
  pub fn untag(&self, tag_range: ExternalPointerTagRange) -> Address {
    if external_pointer_can_be_empty(tag_range) {
      self.untag_allow_null(tag_range)
    } else {
      self
        .untag_disallow_null(tag_range)
        .unwrap_or(K_NULL_ADDRESS)
    }
  }

  pub fn is_tagged_with_tag_in(&self, tag_range: ExternalPointerTagRange) -> bool {
    Self::check_tag(self.encoded_word, tag_range)
  }
  pub fn is_tagged_with(&self, tag: ExternalPointerTag) -> bool {
    Self::check_tag(self.encoded_word, ExternalPointerTagRange::of(tag))
  }

  /// 原版 SetMarkBit / ClearMarkBit / HasMarkBitSet
  #[inline]
  pub fn set_mark_bit(&mut self) {
    self.encoded_word |= K_EXTERNAL_POINTER_MARK_BIT;
  }
  #[inline]
  pub fn clear_mark_bit(&mut self) {
    self.encoded_word &= !K_EXTERNAL_POINTER_MARK_BIT;
  }
  #[inline]
  pub const fn has_mark_bit_set(&self) -> bool {
    self.encoded_word & K_EXTERNAL_POINTER_MARK_BIT != 0
  }

  /// 原版 ExtractFreelistLink (tagged-payload.h:92-99): free 表项低 48 位为下一索引
  #[inline]
  pub fn extract_freelist_link(&self) -> Option<u32> {
    if self.is_tagged_with(Self::FREE_ENTRY_TAG) {
      Some((self.encoded_word & K_EXTERNAL_POINTER_PAYLOAD_MASK) as u32)
    } else {
      None
    }
  }

  /// 原版 SetTag (tagged-payload.h:101-104)
  #[inline]
  pub fn set_tag(&mut self, tag: ExternalPointerTag) {
    self.encoded_word = (self.encoded_word & !K_EXTERNAL_POINTER_TAG_MASK)
      | ((tag as u64) << K_EXTERNAL_POINTER_TAG_SHIFT);
  }

  /// 原版 ExtractTag (tagged-payload.h:106-108)
  #[inline]
  pub fn extract_tag(&self) -> Option<ExternalPointerTag> {
    ExternalPointerTag::from_repr(
      ((self.encoded_word & K_EXTERNAL_POINTER_TAG_MASK) >> K_EXTERNAL_POINTER_TAG_SHIFT) as u16,
    )
  }

  pub fn contains_freelist_link(&self) -> bool {
    self.is_tagged_with(Self::FREE_ENTRY_TAG)
  }
  pub fn contains_evacuation_entry(&self) -> bool {
    self.is_tagged_with(Self::EVACUATION_ENTRY_TAG)
  }
  pub fn contains_pointer(&self) -> bool {
    !self.contains_freelist_link() && !self.contains_evacuation_entry()
  }
}

// ---- ExternalPointerMember 存储语义 (external-pointer.h:31-90) ----
/// 对象内外部指针槽位 (沙箱开 = ExternalPointerHandle 存储)。
/// Init/load/store/exchange 走 IsolateForSandbox + ExternalPointerTable (第二梯队登记)。
#[derive(Debug, Clone, Copy)]
pub struct ExternalPointerMember {
  pub storage: ExternalPointerHandle,
}

impl ExternalPointerMember {
  pub const fn null() -> Self {
    Self {
      storage: K_NULL_EXTERNAL_POINTER_HANDLE,
    }
  }

  /// 原版 load_encoded (external-pointer-inl.h:103-106)
  #[inline]
  pub const fn load_encoded(&self) -> ExternalPointerHandle {
    self.storage
  }

  /// 原版 store_encoded (external-pointer-inl.h:108-112)
  #[inline]
  pub fn store_encoded(&mut self, value: ExternalPointerHandle) {
    self.storage = value;
  }

  /// 槽位地址
  pub fn storage_address(&mut self) -> *mut ExternalPointerHandle {
    &mut self.storage
  }
}
