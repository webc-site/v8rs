// 由 tool/src/execution/protectors.js 从 v8/src/execution/protectors.h, src/execution/protectors.cc, src/execution/protectors-inl.h 生成; 修订就地改, 复跑覆盖
// Protectors 22 有效位管理

/// 保护器有效标记 (对应原版 kProtectorValid)
pub const PROTECTOR_VALID: i32 = 1;
/// 保护器失效标记 (对应原版 kProtectorInvalid)
pub const PROTECTOR_INVALID: i32 = 0;

/// 22 个保护器枚举 (顺序与原版 DECLARED_PROTECTORS_ON_ISOLATE 严格 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Protector {
  /// 原版 ArrayBufferDetaching (RootIndex: ArrayBufferDetachingProtector)
  ArrayBufferDetaching = 0,
  /// 原版 ArrayBufferMutable (RootIndex: ArrayBufferMutableProtector)
  ArrayBufferMutable = 1,
  /// 原版 ArrayIteratorLookupChain (RootIndex: ArrayIteratorProtector)
  ArrayIteratorLookupChain = 2,
  /// 原版 ArraySpeciesLookupChain (RootIndex: ArraySpeciesProtector)
  ArraySpeciesLookupChain = 3,
  /// 原版 IsConcatSpreadableLookupChain (RootIndex: IsConcatSpreadableProtector)
  IsConcatSpreadableLookupChain = 4,
  /// 原版 NoDateTimeConfigurationChange (RootIndex: NoDateTimeConfigurationChangeProtector)
  NoDateTimeConfigurationChange = 5,
  /// 原版 NoElements (RootIndex: NoElementsProtector)
  NoElements = 6,
  /// 原版 MegaDOM (RootIndex: MegaDOMProtector)
  MegaDOM = 7,
  /// 原版 NoProfiling (RootIndex: NoProfilingProtector)
  NoProfiling = 8,
  /// 原版 NoUndetectableObjects (RootIndex: NoUndetectableObjectsProtector)
  NoUndetectableObjects = 9,
  /// 原版 MapIteratorLookupChain (RootIndex: MapIteratorProtector)
  MapIteratorLookupChain = 10,
  /// 原版 NumberStringNotRegexpLike (RootIndex: NumberStringNotRegexpLikeProtector)
  NumberStringNotRegexpLike = 11,
  /// 原版 RegExpSpeciesLookupChain (RootIndex: RegExpSpeciesProtector)
  RegExpSpeciesLookupChain = 12,
  /// 原版 PromiseHook (RootIndex: PromiseHookProtector)
  PromiseHook = 13,
  /// 原版 PromiseThenLookupChain (RootIndex: PromiseThenProtector)
  PromiseThenLookupChain = 14,
  /// 原版 PromiseResolveLookupChain (RootIndex: PromiseResolveProtector)
  PromiseResolveLookupChain = 15,
  /// 原版 PromiseSpeciesLookupChain (RootIndex: PromiseSpeciesProtector)
  PromiseSpeciesLookupChain = 16,
  /// 原版 SetIteratorLookupChain (RootIndex: SetIteratorProtector)
  SetIteratorLookupChain = 17,
  /// 原版 StringIteratorLookupChain (RootIndex: StringIteratorProtector)
  StringIteratorLookupChain = 18,
  /// 原版 StringLengthOverflowLookupChain (RootIndex: StringLengthProtector)
  StringLengthOverflowLookupChain = 19,
  /// 原版 StringWrapperToPrimitive (RootIndex: StringWrapperToPrimitiveProtector)
  StringWrapperToPrimitive = 20,
  /// 原版 TypedArraySpeciesLookupChain (RootIndex: TypedArraySpeciesProtector)
  TypedArraySpeciesLookupChain = 21,
}

impl Protector {
  /// 全部 22 项保护器列表
  pub const ALL: [Protector; 22] = [
    Protector::ArrayBufferDetaching,
    Protector::ArrayBufferMutable,
    Protector::ArrayIteratorLookupChain,
    Protector::ArraySpeciesLookupChain,
    Protector::IsConcatSpreadableLookupChain,
    Protector::NoDateTimeConfigurationChange,
    Protector::NoElements,
    Protector::MegaDOM,
    Protector::NoProfiling,
    Protector::NoUndetectableObjects,
    Protector::MapIteratorLookupChain,
    Protector::NumberStringNotRegexpLike,
    Protector::RegExpSpeciesLookupChain,
    Protector::PromiseHook,
    Protector::PromiseThenLookupChain,
    Protector::PromiseResolveLookupChain,
    Protector::PromiseSpeciesLookupChain,
    Protector::SetIteratorLookupChain,
    Protector::StringIteratorLookupChain,
    Protector::StringLengthOverflowLookupChain,
    Protector::StringWrapperToPrimitive,
    Protector::TypedArraySpeciesLookupChain,
  ];

  /// 对应位掩码
  #[inline]
  pub const fn mask(self) -> u32 {
    1 << (self as u8)
  }

  /// 保护器名称
  #[inline]
  pub const fn name(self) -> &'static str {
    match self {
      Protector::ArrayBufferDetaching => "ArrayBufferDetaching",
      Protector::ArrayBufferMutable => "ArrayBufferMutable",
      Protector::ArrayIteratorLookupChain => "ArrayIteratorLookupChain",
      Protector::ArraySpeciesLookupChain => "ArraySpeciesLookupChain",
      Protector::IsConcatSpreadableLookupChain => "IsConcatSpreadableLookupChain",
      Protector::NoDateTimeConfigurationChange => "NoDateTimeConfigurationChange",
      Protector::NoElements => "NoElements",
      Protector::MegaDOM => "MegaDOM",
      Protector::NoProfiling => "NoProfiling",
      Protector::NoUndetectableObjects => "NoUndetectableObjects",
      Protector::MapIteratorLookupChain => "MapIteratorLookupChain",
      Protector::NumberStringNotRegexpLike => "NumberStringNotRegexpLike",
      Protector::RegExpSpeciesLookupChain => "RegExpSpeciesLookupChain",
      Protector::PromiseHook => "PromiseHook",
      Protector::PromiseThenLookupChain => "PromiseThenLookupChain",
      Protector::PromiseResolveLookupChain => "PromiseResolveLookupChain",
      Protector::PromiseSpeciesLookupChain => "PromiseSpeciesLookupChain",
      Protector::SetIteratorLookupChain => "SetIteratorLookupChain",
      Protector::StringIteratorLookupChain => "StringIteratorLookupChain",
      Protector::StringLengthOverflowLookupChain => "StringLengthOverflowLookupChain",
      Protector::StringWrapperToPrimitive => "StringWrapperToPrimitive",
      Protector::TypedArraySpeciesLookupChain => "TypedArraySpeciesLookupChain",
    }
  }
}

/// 保护器有效位管理状态机 (22 项位图压缩)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Protectors {
  bits: u32,
}

impl Default for Protectors {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl Protectors {
  /// 全有效位掩码 (低 22 位全 1)
  pub const ALL_VALID_MASK: u32 = (1 << 22) - 1;

  /// 初始化保护器：所有保护器默认完整有效
  #[inline]
  pub const fn new() -> Self {
    Self {
      bits: Self::ALL_VALID_MASK,
    }
  }

  /// 检查特定保护器是否依然完整有效 (对应原版 Is##name##Intact)
  #[inline]
  pub const fn is_intact(&self, protector: Protector) -> bool {
    (self.bits & protector.mask()) != 0
  }

  /// 将特定保护器置为失效 (对应原版 Invalidate##name)
  #[inline]
  pub fn invalidate(&mut self, protector: Protector) {
    self.bits &= !protector.mask();
  }

  /// 是否全部 22 个保护器均完整
  #[inline]
  pub const fn is_all_intact(&self) -> bool {
    self.bits == Self::ALL_VALID_MASK
  }

  /// 获取当前位图原始值
  #[inline]
  pub const fn raw_bits(&self) -> u32 {
    self.bits
  }

  /// 原版 IsArrayBufferDetachingIntact
  #[inline]
  pub const fn is_array_buffer_detaching_intact(&self) -> bool {
    self.is_intact(Protector::ArrayBufferDetaching)
  }

  /// 原版 InvalidateArrayBufferDetaching
  #[inline]
  pub fn invalidate_array_buffer_detaching(&mut self) {
    self.invalidate(Protector::ArrayBufferDetaching);
  }

  /// 原版 IsArrayBufferMutableIntact
  #[inline]
  pub const fn is_array_buffer_mutable_intact(&self) -> bool {
    self.is_intact(Protector::ArrayBufferMutable)
  }

  /// 原版 InvalidateArrayBufferMutable
  #[inline]
  pub fn invalidate_array_buffer_mutable(&mut self) {
    self.invalidate(Protector::ArrayBufferMutable);
  }

  /// 原版 IsArrayIteratorLookupChainIntact
  #[inline]
  pub const fn is_array_iterator_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::ArrayIteratorLookupChain)
  }

  /// 原版 InvalidateArrayIteratorLookupChain
  #[inline]
  pub fn invalidate_array_iterator_lookup_chain(&mut self) {
    self.invalidate(Protector::ArrayIteratorLookupChain);
  }

  /// 原版 IsArraySpeciesLookupChainIntact
  #[inline]
  pub const fn is_array_species_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::ArraySpeciesLookupChain)
  }

  /// 原版 InvalidateArraySpeciesLookupChain
  #[inline]
  pub fn invalidate_array_species_lookup_chain(&mut self) {
    self.invalidate(Protector::ArraySpeciesLookupChain);
  }

  /// 原版 IsIsConcatSpreadableLookupChainIntact
  #[inline]
  pub const fn is_is_concat_spreadable_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::IsConcatSpreadableLookupChain)
  }

  /// 原版 InvalidateIsConcatSpreadableLookupChain
  #[inline]
  pub fn invalidate_is_concat_spreadable_lookup_chain(&mut self) {
    self.invalidate(Protector::IsConcatSpreadableLookupChain);
  }

  /// 原版 IsNoDateTimeConfigurationChangeIntact
  #[inline]
  pub const fn is_no_date_time_configuration_change_intact(&self) -> bool {
    self.is_intact(Protector::NoDateTimeConfigurationChange)
  }

  /// 原版 InvalidateNoDateTimeConfigurationChange
  #[inline]
  pub fn invalidate_no_date_time_configuration_change(&mut self) {
    self.invalidate(Protector::NoDateTimeConfigurationChange);
  }

  /// 原版 IsNoElementsIntact
  #[inline]
  pub const fn is_no_elements_intact(&self) -> bool {
    self.is_intact(Protector::NoElements)
  }

  /// 原版 InvalidateNoElements
  #[inline]
  pub fn invalidate_no_elements(&mut self) {
    self.invalidate(Protector::NoElements);
  }

  /// 原版 IsMegaDOMIntact
  #[inline]
  pub const fn is_mega_dom_intact(&self) -> bool {
    self.is_intact(Protector::MegaDOM)
  }

  /// 原版 InvalidateMegaDOM
  #[inline]
  pub fn invalidate_mega_dom(&mut self) {
    self.invalidate(Protector::MegaDOM);
  }

  /// 原版 IsNoProfilingIntact
  #[inline]
  pub const fn is_no_profiling_intact(&self) -> bool {
    self.is_intact(Protector::NoProfiling)
  }

  /// 原版 InvalidateNoProfiling
  #[inline]
  pub fn invalidate_no_profiling(&mut self) {
    self.invalidate(Protector::NoProfiling);
  }

  /// 原版 IsNoUndetectableObjectsIntact
  #[inline]
  pub const fn is_no_undetectable_objects_intact(&self) -> bool {
    self.is_intact(Protector::NoUndetectableObjects)
  }

  /// 原版 InvalidateNoUndetectableObjects
  #[inline]
  pub fn invalidate_no_undetectable_objects(&mut self) {
    self.invalidate(Protector::NoUndetectableObjects);
  }

  /// 原版 IsMapIteratorLookupChainIntact
  #[inline]
  pub const fn is_map_iterator_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::MapIteratorLookupChain)
  }

  /// 原版 InvalidateMapIteratorLookupChain
  #[inline]
  pub fn invalidate_map_iterator_lookup_chain(&mut self) {
    self.invalidate(Protector::MapIteratorLookupChain);
  }

  /// 原版 IsNumberStringNotRegexpLikeIntact
  #[inline]
  pub const fn is_number_string_not_regexp_like_intact(&self) -> bool {
    self.is_intact(Protector::NumberStringNotRegexpLike)
  }

  /// 原版 InvalidateNumberStringNotRegexpLike
  #[inline]
  pub fn invalidate_number_string_not_regexp_like(&mut self) {
    self.invalidate(Protector::NumberStringNotRegexpLike);
  }

  /// 原版 IsRegExpSpeciesLookupChainIntact
  #[inline]
  pub const fn is_reg_exp_species_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::RegExpSpeciesLookupChain)
  }

  /// 原版 InvalidateRegExpSpeciesLookupChain
  #[inline]
  pub fn invalidate_reg_exp_species_lookup_chain(&mut self) {
    self.invalidate(Protector::RegExpSpeciesLookupChain);
  }

  /// 原版 IsPromiseHookIntact
  #[inline]
  pub const fn is_promise_hook_intact(&self) -> bool {
    self.is_intact(Protector::PromiseHook)
  }

  /// 原版 InvalidatePromiseHook
  #[inline]
  pub fn invalidate_promise_hook(&mut self) {
    self.invalidate(Protector::PromiseHook);
  }

  /// 原版 IsPromiseThenLookupChainIntact
  #[inline]
  pub const fn is_promise_then_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::PromiseThenLookupChain)
  }

  /// 原版 InvalidatePromiseThenLookupChain
  #[inline]
  pub fn invalidate_promise_then_lookup_chain(&mut self) {
    self.invalidate(Protector::PromiseThenLookupChain);
  }

  /// 原版 IsPromiseResolveLookupChainIntact
  #[inline]
  pub const fn is_promise_resolve_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::PromiseResolveLookupChain)
  }

  /// 原版 InvalidatePromiseResolveLookupChain
  #[inline]
  pub fn invalidate_promise_resolve_lookup_chain(&mut self) {
    self.invalidate(Protector::PromiseResolveLookupChain);
  }

  /// 原版 IsPromiseSpeciesLookupChainIntact
  #[inline]
  pub const fn is_promise_species_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::PromiseSpeciesLookupChain)
  }

  /// 原版 InvalidatePromiseSpeciesLookupChain
  #[inline]
  pub fn invalidate_promise_species_lookup_chain(&mut self) {
    self.invalidate(Protector::PromiseSpeciesLookupChain);
  }

  /// 原版 IsSetIteratorLookupChainIntact
  #[inline]
  pub const fn is_set_iterator_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::SetIteratorLookupChain)
  }

  /// 原版 InvalidateSetIteratorLookupChain
  #[inline]
  pub fn invalidate_set_iterator_lookup_chain(&mut self) {
    self.invalidate(Protector::SetIteratorLookupChain);
  }

  /// 原版 IsStringIteratorLookupChainIntact
  #[inline]
  pub const fn is_string_iterator_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::StringIteratorLookupChain)
  }

  /// 原版 InvalidateStringIteratorLookupChain
  #[inline]
  pub fn invalidate_string_iterator_lookup_chain(&mut self) {
    self.invalidate(Protector::StringIteratorLookupChain);
  }

  /// 原版 IsStringLengthOverflowLookupChainIntact
  #[inline]
  pub const fn is_string_length_overflow_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::StringLengthOverflowLookupChain)
  }

  /// 原版 InvalidateStringLengthOverflowLookupChain
  #[inline]
  pub fn invalidate_string_length_overflow_lookup_chain(&mut self) {
    self.invalidate(Protector::StringLengthOverflowLookupChain);
  }

  /// 原版 IsStringWrapperToPrimitiveIntact
  #[inline]
  pub const fn is_string_wrapper_to_primitive_intact(&self) -> bool {
    self.is_intact(Protector::StringWrapperToPrimitive)
  }

  /// 原版 InvalidateStringWrapperToPrimitive
  #[inline]
  pub fn invalidate_string_wrapper_to_primitive(&mut self) {
    self.invalidate(Protector::StringWrapperToPrimitive);
  }

  /// 原版 IsTypedArraySpeciesLookupChainIntact
  #[inline]
  pub const fn is_typed_array_species_lookup_chain_intact(&self) -> bool {
    self.is_intact(Protector::TypedArraySpeciesLookupChain)
  }

  /// 原版 InvalidateTypedArraySpeciesLookupChain
  #[inline]
  pub fn invalidate_typed_array_species_lookup_chain(&mut self) {
    self.invalidate(Protector::TypedArraySpeciesLookupChain);
  }
}

// ── 编译期静态断言 ──
const _: () = {
  assert!(Protector::ALL.len() == 22);
  assert!(Protectors::ALL_VALID_MASK == 0x003F_FFFF);
};
