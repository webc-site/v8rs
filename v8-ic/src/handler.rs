use strum::{Display, EnumIter, EnumString};

use crate::error::{Error, Result};

/// LoadHandler::Kind 16 项变体 (来源: handler-configuration.h:35)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum LoadHandlerKind {
  #[strum(serialize = "kElement")]
  Element = 0,
  #[strum(serialize = "kElementWithTransition")]
  ElementWithTransition = 1,
  #[strum(serialize = "kIndexedString")]
  IndexedString = 2,
  #[strum(serialize = "kNormal")]
  Normal = 3,
  #[strum(serialize = "kGlobal")]
  Global = 4,
  #[strum(serialize = "kField")]
  Field = 5,
  #[strum(serialize = "kConstantFromPrototype")]
  ConstantFromPrototype = 6,
  #[strum(serialize = "kAccessorFromPrototype")]
  AccessorFromPrototype = 7,
  #[strum(serialize = "kNativeDataProperty")]
  NativeDataProperty = 8,
  #[strum(serialize = "kApiGetter")]
  ApiGetter = 9,
  #[strum(serialize = "kInterceptor")]
  Interceptor = 10,
  #[strum(serialize = "kSlow")]
  Slow = 11,
  #[strum(serialize = "kProxy")]
  Proxy = 12,
  #[strum(serialize = "kNonExistent")]
  NonExistent = 13,
  #[strum(serialize = "kModuleExport")]
  ModuleExport = 14,
  #[strum(serialize = "kGeneric")]
  Generic = 15,
}

impl LoadHandlerKind {
  #[inline]
  pub const fn is_element(&self) -> bool {
    matches!(self, Self::Element | Self::ElementWithTransition)
  }

  #[inline]
  pub const fn is_field(&self) -> bool {
    matches!(self, Self::Field)
  }

  #[inline]
  pub const fn is_proxy(&self) -> bool {
    matches!(self, Self::Proxy)
  }
}

/// StoreHandler::Kind 12 项变体 (来源: handler-configuration.h:266)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum StoreHandlerKind {
  #[strum(serialize = "kField")]
  Field = 0,
  #[strum(serialize = "kConstField")]
  ConstField = 1,
  #[strum(serialize = "kAccessorFromPrototype")]
  AccessorFromPrototype = 2,
  #[strum(serialize = "kNativeDataProperty")]
  NativeDataProperty = 3,
  #[strum(serialize = "kSharedStructField")]
  SharedStructField = 4,
  #[strum(serialize = "kApiSetter")]
  ApiSetter = 5,
  #[strum(serialize = "kGlobalProxy")]
  GlobalProxy = 6,
  #[strum(serialize = "kNormal")]
  Normal = 7,
  #[strum(serialize = "kInterceptor")]
  Interceptor = 8,
  #[strum(serialize = "kSlow")]
  Slow = 9,
  #[strum(serialize = "kProxy")]
  Proxy = 10,
  #[strum(serialize = "kGeneric")]
  Generic = 11,
}

impl StoreHandlerKind {
  pub const KINDS_NUMBER: usize = 12;

  #[inline]
  pub const fn is_field(&self) -> bool {
    matches!(
      self,
      Self::Field | Self::ConstField | Self::SharedStructField
    )
  }

  #[inline]
  pub const fn is_proxy(&self) -> bool {
    matches!(self, Self::Proxy | Self::GlobalProxy)
  }
}

/// LoadHandler Smi 配置位段模型 (100% Safe Rust)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadHandlerConfig {
  pub kind: LoadHandlerKind,
  pub do_access_check_on_lookup_start_object: bool,
  pub lookup_on_lookup_start_object: bool,
  pub descriptor_index: u32,
}

impl LoadHandlerConfig {
  pub const KIND_BITS_MASK: u32 = 0x0F;
  pub const ACCESS_CHECK_BIT: u32 = 1 << 4;
  pub const LOOKUP_START_BIT: u32 = 1 << 5;
  pub const DESCRIPTOR_INDEX_SHIFT: u32 = 6;
  pub const DESCRIPTOR_INDEX_MASK: u32 = 0x0FFF; // 12 bits

  pub fn encode(&self) -> u32 {
    let mut raw = (self.kind as u32) & Self::KIND_BITS_MASK;
    if self.do_access_check_on_lookup_start_object {
      raw |= Self::ACCESS_CHECK_BIT;
    }
    if self.lookup_on_lookup_start_object {
      raw |= Self::LOOKUP_START_BIT;
    }
    raw |= (self.descriptor_index & Self::DESCRIPTOR_INDEX_MASK) << Self::DESCRIPTOR_INDEX_SHIFT;
    raw
  }

  pub fn decode(raw: u32) -> Result<Self> {
    let kind_val = (raw & Self::KIND_BITS_MASK) as u8;
    let kind = match kind_val {
      0 => LoadHandlerKind::Element,
      1 => LoadHandlerKind::ElementWithTransition,
      2 => LoadHandlerKind::IndexedString,
      3 => LoadHandlerKind::Normal,
      4 => LoadHandlerKind::Global,
      5 => LoadHandlerKind::Field,
      6 => LoadHandlerKind::ConstantFromPrototype,
      7 => LoadHandlerKind::AccessorFromPrototype,
      8 => LoadHandlerKind::NativeDataProperty,
      9 => LoadHandlerKind::ApiGetter,
      10 => LoadHandlerKind::Interceptor,
      11 => LoadHandlerKind::Slow,
      12 => LoadHandlerKind::Proxy,
      13 => LoadHandlerKind::NonExistent,
      14 => LoadHandlerKind::ModuleExport,
      15 => LoadHandlerKind::Generic,
      _ => {
        return Err(Error::UnsupportedHandlerKind(format!(
          "Unknown LoadHandler kind {kind_val}"
        )));
      }
    };

    Ok(Self {
      kind,
      do_access_check_on_lookup_start_object: (raw & Self::ACCESS_CHECK_BIT) != 0,
      lookup_on_lookup_start_object: (raw & Self::LOOKUP_START_BIT) != 0,
      descriptor_index: (raw >> Self::DESCRIPTOR_INDEX_SHIFT) & Self::DESCRIPTOR_INDEX_MASK,
    })
  }
}

/// StoreHandler Smi 配置位段模型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreHandlerConfig {
  pub kind: StoreHandlerKind,
  pub is_inobject: bool,
  pub representation: u8,
  pub storage_offset_in_words: u16,
}

impl StoreHandlerConfig {
  pub const KIND_BITS_MASK: u32 = 0x0F;
  pub const INOBJECT_BIT: u32 = 1 << 4;
  pub const REPRESENTATION_SHIFT: u32 = 5;
  pub const REPRESENTATION_MASK: u32 = 0x07;
  pub const OFFSET_SHIFT: u32 = 8;
  pub const OFFSET_MASK: u32 = 0xFFFF;

  pub fn encode(&self) -> u32 {
    let mut raw = (self.kind as u32) & Self::KIND_BITS_MASK;
    if self.is_inobject {
      raw |= Self::INOBJECT_BIT;
    }
    raw |= ((self.representation as u32) & Self::REPRESENTATION_MASK) << Self::REPRESENTATION_SHIFT;
    raw |= ((self.storage_offset_in_words as u32) & Self::OFFSET_MASK) << Self::OFFSET_SHIFT;
    raw
  }

  pub fn decode(raw: u32) -> Result<Self> {
    let kind_val = (raw & Self::KIND_BITS_MASK) as u8;
    let kind = match kind_val {
      0 => StoreHandlerKind::Field,
      1 => StoreHandlerKind::ConstField,
      2 => StoreHandlerKind::AccessorFromPrototype,
      3 => StoreHandlerKind::NativeDataProperty,
      4 => StoreHandlerKind::SharedStructField,
      5 => StoreHandlerKind::ApiSetter,
      6 => StoreHandlerKind::GlobalProxy,
      7 => StoreHandlerKind::Normal,
      8 => StoreHandlerKind::Interceptor,
      9 => StoreHandlerKind::Slow,
      10 => StoreHandlerKind::Proxy,
      11 => StoreHandlerKind::Generic,
      _ => {
        return Err(Error::UnsupportedHandlerKind(format!(
          "Unknown StoreHandler kind {kind_val}"
        )));
      }
    };

    Ok(Self {
      kind,
      is_inobject: (raw & Self::INOBJECT_BIT) != 0,
      representation: ((raw >> Self::REPRESENTATION_SHIFT) & Self::REPRESENTATION_MASK) as u8,
      storage_offset_in_words: ((raw >> Self::OFFSET_SHIFT) & Self::OFFSET_MASK) as u16,
    })
  }
}

/// 来自 `handler-configuration.tq` 的声明清单
pub const HANDLER_TQ_DECLS: &[(&str, &str)] = &[
  ("extern class", "LoadHandler"),
  ("extern class", "StoreHandler"),
];
