/// 外部引用类型 (对应原版 v8::internal::ExternalReference)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternalReferenceType {
  BuiltinCall,
  FastCCall,
}

/// 外部引用地址与符号定义
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExternalReference {
  raw: usize,
  type_: ExternalReferenceType,
  name: Option<&'static str>,
}

impl ExternalReference {
  #[inline]
  pub const fn null() -> Self {
    Self {
      raw: 0,
      type_: ExternalReferenceType::BuiltinCall,
      name: None,
    }
  }

  #[inline]
  pub const fn from_raw_address(raw: usize) -> Self {
    Self {
      raw,
      type_: ExternalReferenceType::BuiltinCall,
      name: None,
    }
  }

  #[inline]
  pub const fn from_symbol(name: &'static str, raw: usize) -> Self {
    Self {
      raw,
      type_: ExternalReferenceType::BuiltinCall,
      name: Some(name),
    }
  }

  #[inline]
  pub const fn with_type(mut self, type_: ExternalReferenceType) -> Self {
    self.type_ = type_;
    self
  }

  #[inline]
  pub const fn is_null(&self) -> bool {
    self.raw == 0
  }

  #[inline]
  pub const fn raw(&self) -> usize {
    self.raw
  }

  #[inline]
  pub const fn address(&self) -> usize {
    self.raw
  }

  #[inline]
  pub const fn type_(&self) -> ExternalReferenceType {
    self.type_
  }

  #[inline]
  pub const fn name(&self) -> Option<&'static str> {
    self.name
  }
}
