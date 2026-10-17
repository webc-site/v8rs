// 由 tool/src/builtins/builtins-typedarray.js 生成; 来源 v8/src/builtins 下 28 文件
// 内建名表 + 核心调度元数据 (19 内建)
// 来源: builtins-typed-array.cc, builtins-typed-array-gen.cc, builtins-typed-array-gen.h, typed-array.tq 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// TypedArray 内建名表 (19 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  TypedArrayBaseConstructor,
  TypedArrayConstructor,
  TypedArrayPrototypeBuffer,
  TypedArrayPrototypeByteLength,
  TypedArrayPrototypeByteOffset,
  TypedArrayPrototypeCopyWithin,
  TypedArrayPrototypeFill,
  TypedArrayPrototypeIncludes,
  TypedArrayPrototypeIndexOf,
  TypedArrayPrototypeLastIndexOf,
  TypedArrayPrototypeLength,
  TypedArrayPrototypeReverse,
  TypedArrayPrototypeToStringTag,
  Uint8ArrayFromBase64,
  Uint8ArrayFromHex,
  Uint8ArrayPrototypeSetFromBase64,
  Uint8ArrayPrototypeSetFromHex,
  Uint8ArrayPrototypeToBase64,
  Uint8ArrayPrototypeToHex,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::TypedArrayBaseConstructor => "TypedArrayBaseConstructor",
      Self::TypedArrayConstructor => "TypedArrayConstructor",
      Self::TypedArrayPrototypeBuffer => "TypedArrayPrototypeBuffer",
      Self::TypedArrayPrototypeByteLength => "TypedArrayPrototypeByteLength",
      Self::TypedArrayPrototypeByteOffset => "TypedArrayPrototypeByteOffset",
      Self::TypedArrayPrototypeCopyWithin => "TypedArrayPrototypeCopyWithin",
      Self::TypedArrayPrototypeFill => "TypedArrayPrototypeFill",
      Self::TypedArrayPrototypeIncludes => "TypedArrayPrototypeIncludes",
      Self::TypedArrayPrototypeIndexOf => "TypedArrayPrototypeIndexOf",
      Self::TypedArrayPrototypeLastIndexOf => "TypedArrayPrototypeLastIndexOf",
      Self::TypedArrayPrototypeLength => "TypedArrayPrototypeLength",
      Self::TypedArrayPrototypeReverse => "TypedArrayPrototypeReverse",
      Self::TypedArrayPrototypeToStringTag => "TypedArrayPrototypeToStringTag",
      Self::Uint8ArrayFromBase64 => "Uint8ArrayFromBase64",
      Self::Uint8ArrayFromHex => "Uint8ArrayFromHex",
      Self::Uint8ArrayPrototypeSetFromBase64 => "Uint8ArrayPrototypeSetFromBase64",
      Self::Uint8ArrayPrototypeSetFromHex => "Uint8ArrayPrototypeSetFromHex",
      Self::Uint8ArrayPrototypeToBase64 => "Uint8ArrayPrototypeToBase64",
      Self::Uint8ArrayPrototypeToHex => "Uint8ArrayPrototypeToHex",
    }
  }

  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取对应的接口描述符符号
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 是否为构造函数内建
  #[inline]
  pub const fn is_constructor(&self) -> bool {
    matches!(
      self,
      Self::TypedArrayBaseConstructor | Self::TypedArrayConstructor
    )
  }

  /// 是否为原型方法/访问器内建
  #[inline]
  pub const fn is_prototype_method(&self) -> bool {
    matches!(
      self,
      Self::TypedArrayPrototypeBuffer
        | Self::TypedArrayPrototypeByteLength
        | Self::TypedArrayPrototypeByteOffset
        | Self::TypedArrayPrototypeCopyWithin
        | Self::TypedArrayPrototypeFill
        | Self::TypedArrayPrototypeIncludes
        | Self::TypedArrayPrototypeIndexOf
        | Self::TypedArrayPrototypeLastIndexOf
        | Self::TypedArrayPrototypeLength
        | Self::TypedArrayPrototypeReverse
        | Self::TypedArrayPrototypeToStringTag
        | Self::Uint8ArrayPrototypeSetFromBase64
        | Self::Uint8ArrayPrototypeSetFromHex
        | Self::Uint8ArrayPrototypeToBase64
        | Self::Uint8ArrayPrototypeToHex
    )
  }

  /// 是否为搜索查询方法 (includes, indexOf, lastIndexOf)
  #[inline]
  pub const fn is_search_method(&self) -> bool {
    matches!(
      self,
      Self::TypedArrayPrototypeIncludes
        | Self::TypedArrayPrototypeIndexOf
        | Self::TypedArrayPrototypeLastIndexOf
    )
  }

  /// 是否为 Base64 或 Hex 编解码相关扩展
  #[inline]
  pub const fn is_base64_or_hex(&self) -> bool {
    matches!(
      self,
      Self::Uint8ArrayFromBase64
        | Self::Uint8ArrayFromHex
        | Self::Uint8ArrayPrototypeSetFromBase64
        | Self::Uint8ArrayPrototypeSetFromHex
        | Self::Uint8ArrayPrototypeToBase64
        | Self::Uint8ArrayPrototypeToHex
    )
  }
}

/// TypedArray 元素类型元数据 (基于 V8 ElementsKind 与 builtins-typed-array-gen.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum TypedArrayElementsKind {
  Uint8,
  Int8,
  Uint16,
  Int16,
  Uint32,
  Int32,
  Float16,
  Float32,
  Float64,
  Uint8Clamped,
  BigUint64,
  BigInt64,
  RabGsabUint8,
}

impl TypedArrayElementsKind {
  /// 单个元素占用的字节数
  #[inline]
  pub const fn element_size(&self) -> usize {
    match self {
      Self::Uint8 | Self::Int8 | Self::Uint8Clamped | Self::RabGsabUint8 => 1,
      Self::Uint16 | Self::Int16 | Self::Float16 => 2,
      Self::Uint32 | Self::Int32 | Self::Float32 => 4,
      Self::Float64 | Self::BigUint64 | Self::BigInt64 => 8,
    }
  }

  /// 是否为 Uint8 家族 (Uint8, Uint8Clamped, RabGsabUint8)
  #[inline]
  pub const fn is_uint8(&self) -> bool {
    matches!(self, Self::Uint8 | Self::Uint8Clamped | Self::RabGsabUint8)
  }

  /// 是否为 BigInt 家族 (BigInt64, BigUint64)
  #[inline]
  pub const fn is_bigint(&self) -> bool {
    matches!(self, Self::BigInt64 | Self::BigUint64)
  }

  /// 是否为浮点数家族 (Float16, Float32, Float64)
  #[inline]
  pub const fn is_float(&self) -> bool {
    matches!(self, Self::Float16 | Self::Float32 | Self::Float64)
  }
}

// 统一提供 19 项内建构造入口工厂函数
pub const fn typed_array_base_constructor() -> Builtin {
  Builtin::TypedArrayBaseConstructor
}

pub const fn typed_array_constructor() -> Builtin {
  Builtin::TypedArrayConstructor
}

pub const fn typed_array_prototype_buffer() -> Builtin {
  Builtin::TypedArrayPrototypeBuffer
}

pub const fn typed_array_prototype_byte_length() -> Builtin {
  Builtin::TypedArrayPrototypeByteLength
}

pub const fn typed_array_prototype_byte_offset() -> Builtin {
  Builtin::TypedArrayPrototypeByteOffset
}

pub const fn typed_array_prototype_copy_within() -> Builtin {
  Builtin::TypedArrayPrototypeCopyWithin
}

pub const fn typed_array_prototype_fill() -> Builtin {
  Builtin::TypedArrayPrototypeFill
}

pub const fn typed_array_prototype_includes() -> Builtin {
  Builtin::TypedArrayPrototypeIncludes
}

pub const fn typed_array_prototype_index_of() -> Builtin {
  Builtin::TypedArrayPrototypeIndexOf
}

pub const fn typed_array_prototype_last_index_of() -> Builtin {
  Builtin::TypedArrayPrototypeLastIndexOf
}

pub const fn typed_array_prototype_length() -> Builtin {
  Builtin::TypedArrayPrototypeLength
}

pub const fn typed_array_prototype_reverse() -> Builtin {
  Builtin::TypedArrayPrototypeReverse
}

pub const fn typed_array_prototype_to_string_tag() -> Builtin {
  Builtin::TypedArrayPrototypeToStringTag
}

pub const fn uint8_array_from_base64() -> Builtin {
  Builtin::Uint8ArrayFromBase64
}

pub const fn uint8_array_from_hex() -> Builtin {
  Builtin::Uint8ArrayFromHex
}

pub const fn uint8_array_prototype_set_from_base64() -> Builtin {
  Builtin::Uint8ArrayPrototypeSetFromBase64
}

pub const fn uint8_array_prototype_set_from_hex() -> Builtin {
  Builtin::Uint8ArrayPrototypeSetFromHex
}

pub const fn uint8_array_prototype_to_base64() -> Builtin {
  Builtin::Uint8ArrayPrototypeToBase64
}

pub const fn uint8_array_prototype_to_hex() -> Builtin {
  Builtin::Uint8ArrayPrototypeToHex
}

/// 来自 `typed-array*.tq` 的声明清单 (共 61 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("type", "Uint8Elements"),
  ("type", "Int8Elements"),
  ("type", "Uint16Elements"),
  ("type", "Int16Elements"),
  ("type", "Uint32Elements"),
  ("type", "Int32Elements"),
  ("type", "Float16Elements"),
  ("type", "Float32Elements"),
  ("type", "Float64Elements"),
  ("type", "Uint8ClampedElements"),
  ("type", "BigUint64Elements"),
  ("type", "BigInt64Elements"),
  ("type", "RabGsabUint8Elements"),
  ("struct", "TypedArrayElementsInfo"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "GetTypedArrayBuffer"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "LoadFixedTypedArrayElementAsTagged"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "LoadJSTypedArrayLengthAndValidate"),
  ("type", "LoadNumericFn"),
  ("type", "StoreNumericFn"),
  ("type", "StoreJSAnyFn"),
  ("struct", "TypedArrayAccessor"),
  ("macro", "GetTypedArrayAccessor"),
  ("macro", "GetTypedArrayAccessor"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "IsJSArrayBufferViewValid"),
  ("extern macro", "IsJSArrayBufferViewDetachedOrOutOfBounds"),
  (
    "extern macro",
    "IsJSArrayBufferViewDetachedOrOutOfBoundsBoolean",
  ),
  ("macro", "EnsureValid"),
  ("struct", "ValidJSTypedArrayAndLength"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("macro", "EnsureValidAndReadLength"),
  ("struct", "AttachedJSTypedArrayWitness"),
  ("macro", "NewAttachedJSTypedArrayWitness"),
  ("macro", "KindForArrayType"),
  ("builtin", "LoadTypedElement"),
  ("builtin", "StoreTypedElementNumeric"),
  ("builtin", "StoreTypedElementJSAny"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("macro", "AttachView"),
  ("macro", "CanCopyElementsFastNumber"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("extern macro", "TypedArrayBuiltinsAssembler"),
  ("macro", "FastCopy"),
  ("macro", "SlowCopy"),
];
