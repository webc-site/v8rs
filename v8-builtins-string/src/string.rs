// 由 tool/src/builtins/builtins-string.js 生成; 来源 v8/src/builtins 下 21 文件
// 内建名表 + 核心调度元数据 (26 内建)
// 来源: builtins-string.cc, builtins-string-gen.cc, builtins-string-gen.h, builtins-string.tq, string-at.tq, string-endswith.tq 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// 字符串内建名表 (26 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  #[strum(serialize = "StringAdd_NoMapCheck")]
  StringAddNoMapCheck,
  StringCompare,
  StringEqual,
  StringFromCharCode,
  StringFromCodePoint,
  StringFromCodePointAt,
  StringGreaterThan,
  StringGreaterThanOrEqual,
  StringLessThan,
  StringLessThanOrEqual,
  StringPrototypeLastIndexOf,
  StringPrototypeLocaleCompare,
  StringPrototypeMatchAll,
  StringPrototypeNormalize,
  StringPrototypeReplace,
  StringPrototypeSplit,
  StringPrototypeToLocaleLowerCase,
  StringPrototypeToLocaleUpperCase,
  StringPrototypeToLowerCase,
  StringPrototypeToUpperCase,
  StringRaw,
  StringSubstring,
  SubString,
  WasmJSStringEqual,
  #[strum(serialize = "WasmStringAdd_NoMapCheck")]
  WasmStringAddNoMapCheck,
  WasmStringCompare,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::StringAddNoMapCheck => "StringAdd_NoMapCheck",
      Self::StringCompare => "StringCompare",
      Self::StringEqual => "StringEqual",
      Self::StringFromCharCode => "StringFromCharCode",
      Self::StringFromCodePoint => "StringFromCodePoint",
      Self::StringFromCodePointAt => "StringFromCodePointAt",
      Self::StringGreaterThan => "StringGreaterThan",
      Self::StringGreaterThanOrEqual => "StringGreaterThanOrEqual",
      Self::StringLessThan => "StringLessThan",
      Self::StringLessThanOrEqual => "StringLessThanOrEqual",
      Self::StringPrototypeLastIndexOf => "StringPrototypeLastIndexOf",
      Self::StringPrototypeLocaleCompare => "StringPrototypeLocaleCompare",
      Self::StringPrototypeMatchAll => "StringPrototypeMatchAll",
      Self::StringPrototypeNormalize => "StringPrototypeNormalize",
      Self::StringPrototypeReplace => "StringPrototypeReplace",
      Self::StringPrototypeSplit => "StringPrototypeSplit",
      Self::StringPrototypeToLocaleLowerCase => "StringPrototypeToLocaleLowerCase",
      Self::StringPrototypeToLocaleUpperCase => "StringPrototypeToLocaleUpperCase",
      Self::StringPrototypeToLowerCase => "StringPrototypeToLowerCase",
      Self::StringPrototypeToUpperCase => "StringPrototypeToUpperCase",
      Self::StringRaw => "StringRaw",
      Self::StringSubstring => "StringSubstring",
      Self::SubString => "SubString",
      Self::WasmJSStringEqual => "WasmJSStringEqual",
      Self::WasmStringAddNoMapCheck => "WasmStringAdd_NoMapCheck",
      Self::WasmStringCompare => "WasmStringCompare",
    }
  }

  /// 获取对应的接口描述符符号名称 (Builtin_<Name>_InterfaceDescriptor)
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 在 v8-builtins-core 全局表中检索内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 是否属于 String.prototype 上的原型方法
  #[inline]
  pub const fn is_prototype_method(&self) -> bool {
    matches!(
      self,
      Self::StringPrototypeLastIndexOf
        | Self::StringPrototypeLocaleCompare
        | Self::StringPrototypeMatchAll
        | Self::StringPrototypeNormalize
        | Self::StringPrototypeReplace
        | Self::StringPrototypeSplit
        | Self::StringPrototypeToLocaleLowerCase
        | Self::StringPrototypeToLocaleUpperCase
        | Self::StringPrototypeToLowerCase
        | Self::StringPrototypeToUpperCase
    )
  }

  /// 是否属于字符串比较运算族
  #[inline]
  pub const fn is_comparison(&self) -> bool {
    matches!(
      self,
      Self::StringCompare
        | Self::StringEqual
        | Self::StringGreaterThan
        | Self::StringGreaterThanOrEqual
        | Self::StringLessThan
        | Self::StringLessThanOrEqual
        | Self::WasmJSStringEqual
        | Self::WasmStringCompare
    )
  }

  /// 是否属于 WASM 字符串专用快速互操作内建
  #[inline]
  pub const fn is_wasm_string(&self) -> bool {
    matches!(
      self,
      Self::WasmJSStringEqual | Self::WasmStringAddNoMapCheck | Self::WasmStringCompare
    )
  }

  /// 是否为大小写转换方法族
  #[inline]
  pub const fn is_case_conversion(&self) -> bool {
    matches!(
      self,
      Self::StringPrototypeToLowerCase
        | Self::StringPrototypeToUpperCase
        | Self::StringPrototypeToLocaleLowerCase
        | Self::StringPrototypeToLocaleUpperCase
    )
  }
}

// 调度入口 (安全常量构造)
pub const fn string_add_no_map_check() -> Builtin {
  Builtin::StringAddNoMapCheck
}

pub const fn string_compare() -> Builtin {
  Builtin::StringCompare
}

pub const fn string_equal() -> Builtin {
  Builtin::StringEqual
}

pub const fn string_from_char_code() -> Builtin {
  Builtin::StringFromCharCode
}

pub const fn string_from_code_point() -> Builtin {
  Builtin::StringFromCodePoint
}

pub const fn string_from_code_point_at() -> Builtin {
  Builtin::StringFromCodePointAt
}

pub const fn string_greater_than() -> Builtin {
  Builtin::StringGreaterThan
}

pub const fn string_greater_than_or_equal() -> Builtin {
  Builtin::StringGreaterThanOrEqual
}

pub const fn string_less_than() -> Builtin {
  Builtin::StringLessThan
}

pub const fn string_less_than_or_equal() -> Builtin {
  Builtin::StringLessThanOrEqual
}

pub const fn string_prototype_last_index_of() -> Builtin {
  Builtin::StringPrototypeLastIndexOf
}

pub const fn string_prototype_locale_compare() -> Builtin {
  Builtin::StringPrototypeLocaleCompare
}

pub const fn string_prototype_match_all() -> Builtin {
  Builtin::StringPrototypeMatchAll
}

pub const fn string_prototype_normalize() -> Builtin {
  Builtin::StringPrototypeNormalize
}

pub const fn string_prototype_replace() -> Builtin {
  Builtin::StringPrototypeReplace
}

pub const fn string_prototype_split() -> Builtin {
  Builtin::StringPrototypeSplit
}

pub const fn string_prototype_to_locale_lower_case() -> Builtin {
  Builtin::StringPrototypeToLocaleLowerCase
}

pub const fn string_prototype_to_locale_upper_case() -> Builtin {
  Builtin::StringPrototypeToLocaleUpperCase
}

pub const fn string_prototype_to_lower_case() -> Builtin {
  Builtin::StringPrototypeToLowerCase
}

pub const fn string_prototype_to_upper_case() -> Builtin {
  Builtin::StringPrototypeToUpperCase
}

pub const fn string_raw() -> Builtin {
  Builtin::StringRaw
}

pub const fn string_substring() -> Builtin {
  Builtin::StringSubstring
}

pub const fn sub_string() -> Builtin {
  Builtin::SubString
}

pub const fn wasm_js_string_equal() -> Builtin {
  Builtin::WasmJSStringEqual
}

pub const fn wasm_string_add_no_map_check() -> Builtin {
  Builtin::WasmStringAddNoMapCheck
}

pub const fn wasm_string_compare() -> Builtin {
  Builtin::WasmStringCompare
}
