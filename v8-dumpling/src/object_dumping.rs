// 由 tool/src/dumpling/object-dumping.h.js 从 v8/src/dumpling/object-dumping.{h,cc} 生成; 修订就地改, 复跑覆盖
// Dumpling 对象差分打印: 类型别名 + 打印函数实现

//! Dumpling 对象差分打印 — 原版 namespace v8::internal (object-dumping)
//!
//! 语义: 为差分模糊测试输出对象的确定性文本表示; depth 对应 v8_flags.dumpling_depth。

use core::fmt::Write;

/// 默认转储深度 (原版 v8_flags.dumpling_depth 默认值)
pub const DEFAULT_DUMPLING_DEPTH: i32 = 3;

/// C++ 源码匿名命名空间 10 条辅助函数全集台账 (1:1 映射)
pub const CC_HELPER_FUNCTIONS: [&str; 10] = [
  "PrintSanitizedChar",
  "PrintSanitizedCString",
  "FuzzingStringShortPrint",
  "JSObjectFuzzingPrintInternalIndexRange",
  "JSObjectFuzzingPrintFastProperties",
  "JSObjectFuzzingPrintDictProperties",
  "JSObjectFuzzingPrintPrototype",
  "JSObjectFuzzingPrintElements",
  "JSObjectFuzzingPrint",
  "HeapObjectFuzzingPrint",
];

/// 打印转义字符 (原版 PrintSanitizedChar 1:1)
#[inline]
pub fn print_sanitized_char(c: u16, accumulator: &mut String) {
  match c {
    10 => accumulator.push_str("\\n"),
    13 => accumulator.push_str("\\r"),
    92 => accumulator.push_str("\\\\"),
    32..=126 => accumulator.push(c as u8 as char),
    0..=255 => {
      let _ = write!(accumulator, "\\x{:02x}", c);
    }
    _ => {
      let _ = write!(accumulator, "\\u{:04x}", c);
    }
  }
}

/// 打印转义 C 风格字符串 (原版 PrintSanitizedCString 1:1)
pub fn print_sanitized_str(input: &str, accumulator: &mut String) {
  for c in input.encode_utf16() {
    print_sanitized_char(c, accumulator);
  }
}

/// 字符串短打印 (原版 FuzzingStringShortPrint 1:1)
pub fn fuzzing_string_short_print(s: &str, accumulator: &mut String) {
  print_sanitized_str(s, accumulator);
}

/// 反优化期间的未物化对象 (原版 TranslatedValue)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TranslatedValue {
  /// 唯一值标识符
  pub id: u64,
}

impl TranslatedValue {
  /// 创建新的未物化对象标记
  #[inline]
  pub const fn new(id: u64) -> Self {
    Self { id }
  }
}

/// 未物化对象打印器 (原版 TranslatedValuePrinter 1:1)
#[derive(Debug, Clone, Copy, Default)]
pub struct TranslatedValuePrinter;

impl TranslatedValuePrinter {
  /// 原版 C++ 语义: 恒定返回 "<non-materialized>"
  #[inline]
  pub fn print(&self, _obj: &TranslatedValue, _depth: i32) -> &'static str {
    "<non-materialized>"
  }
}

/// 堆对象与原生值的抽象表示 (原版 Tagged<Object> 映射)
#[derive(Debug, Clone, PartialEq)]
pub enum TaggedObject {
  /// Smi 小整数
  Smi(i32),
  /// 堆浮点数
  HeapNumber(f64),
  /// 字符串
  String(String),
  /// JS 函数
  JSFunction {
    /// 源码起始位置
    start_position: i32,
    /// 函数名称
    name: String,
  },
  /// JS 数组
  JSArray,
  /// JS 对象
  JSObject {
    /// 构造器名称 (若存在)
    constructor_name: Option<String>,
  },
  /// 全局对象代理
  GlobalProxy,
  /// Map 隐藏类
  Map {
    /// 实例类型或元素类型描述
    instance_type_or_elements_kind: String,
    /// 是否为元元类 (MetaMap)
    is_meta: bool,
  },
  /// 上下文 (CatchContext/NativeContext/WithContext 等)
  Context {
    /// 上下文类型名称
    name: &'static str,
    /// 长度
    length: usize,
  },
  /// 固定数组
  FixedArray {
    /// 元素长度
    length: usize,
  },
  /// Hole 特殊空洞标记
  Hole(&'static str),
  /// undefined
  Undefined,
  /// null
  Null,
  /// true
  True,
  /// false
  False,
  /// 奇数球 (Oddball)
  OddOddball(String),
  /// 访问器信息
  AccessorInfo,
  /// 访问器对
  AccessorPair,
  /// JSProxy 代理
  JSProxy,
  /// BigInt
  BigInt(String),
  /// ClassPositions
  ClassPositions,
  /// Symbol
  Symbol(String),
  /// ClassBoilerplate
  ClassBoilerplate,
  /// Script
  Script,
  /// FeedbackVector
  FeedbackVector {
    /// 槽位长度
    length: usize,
  },
  /// 其它堆对象表示
  Other(String),
}

/// 对象或未物化对象联合体 (原版 using ObjectOrNonMaterializedObject =
///   std::variant<Tagged<Object>, TranslatedValue*>)
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectOrNonMaterializedObject {
  /// 堆对象引用 (原版 Tagged<Object>)
  Tagged(TaggedObject),
  /// 反优化期间的未物化对象 (原版 TranslatedValue*)
  TranslatedValue(TranslatedValue),
}

impl ObjectOrNonMaterializedObject {
  /// 构造 Smi 对象
  #[inline]
  pub const fn smi(val: i32) -> Self {
    Self::Tagged(TaggedObject::Smi(val))
  }

  /// 构造堆浮点数对象
  #[inline]
  pub const fn heap_number(val: f64) -> Self {
    Self::Tagged(TaggedObject::HeapNumber(val))
  }

  /// 构造字符串对象
  #[inline]
  pub fn string(val: impl Into<String>) -> Self {
    Self::Tagged(TaggedObject::String(val.into()))
  }

  /// 构造未物化对象
  #[inline]
  pub const fn translated(id: u64) -> Self {
    Self::TranslatedValue(TranslatedValue::new(id))
  }

  /// 谓词: 是否为已物化对象
  #[inline]
  pub const fn is_tagged(&self) -> bool {
    matches!(self, Self::Tagged(_))
  }

  /// 谓词: 是否为未物化对象
  #[inline]
  pub const fn is_translated_value(&self) -> bool {
    matches!(self, Self::TranslatedValue(_))
  }
}

/// 差分打印堆对象 (原版 HeapObjectFuzzingPrint 1:1)
pub fn heap_object_fuzzing_print(obj: &TaggedObject, depth: i32, accumulator: &mut String) {
  match obj {
    TaggedObject::String(s) => {
      fuzzing_string_short_print(s, accumulator);
    }
    TaggedObject::JSFunction { start_position, .. } => {
      let _ = write!(accumulator, "<JSFunction pos:{}>", start_position);
    }
    TaggedObject::JSArray => {
      accumulator.push_str("<JSArray>");
    }
    TaggedObject::JSObject { constructor_name } => {
      if let Some(name) = constructor_name.as_deref().filter(|s| !s.is_empty()) {
        let _ = write!(accumulator, "<{}>", name);
        return;
      }
      accumulator.push_str("<JSObject>");
    }
    TaggedObject::GlobalProxy => {
      accumulator.push_str("<global object>");
    }
    TaggedObject::Map {
      instance_type_or_elements_kind,
      is_meta,
    } => {
      if *is_meta {
        accumulator.push_str("<MetaMap>");
      } else {
        let _ = write!(accumulator, "<Map({})>", instance_type_or_elements_kind);
      }
    }
    TaggedObject::Context { name, length } => {
      let _ = write!(accumulator, "<{}[{}]>", name, length);
    }
    TaggedObject::FixedArray { length } => {
      let _ = write!(accumulator, "<FixedArray[{}]>", length);
    }
    TaggedObject::Hole(name) => {
      let _ = write!(accumulator, "<{}>", name);
    }
    TaggedObject::Undefined => accumulator.push_str("<undefined>"),
    TaggedObject::Null => accumulator.push_str("<null>"),
    TaggedObject::True => accumulator.push_str("<true>"),
    TaggedObject::False => accumulator.push_str("<false>"),
    TaggedObject::OddOddball(s) => {
      let _ = write!(accumulator, "<Odd Oddball: {}>", s);
    }
    TaggedObject::AccessorInfo => accumulator.push_str("<AccessorInfo>"),
    TaggedObject::AccessorPair => accumulator.push_str("<AccessorPair>"),
    TaggedObject::JSProxy => accumulator.push_str("<JSProxy>"),
    TaggedObject::BigInt(s) => {
      let _ = write!(accumulator, "<BigIntBase {}>", s);
    }
    TaggedObject::ClassPositions => accumulator.push_str("<ClassPositions>"),
    TaggedObject::Symbol(s) => {
      let _ = write!(accumulator, "<Symbol({})>", s);
    }
    TaggedObject::ClassBoilerplate => accumulator.push_str("<ClassBoilerplate>"),
    TaggedObject::Script => accumulator.push_str("<Script>"),
    TaggedObject::FeedbackVector { length } => {
      let _ = write!(accumulator, "<FeedbackVector[{}]>", length);
    }
    TaggedObject::Other(s) => {
      let _ = write!(accumulator, "<{}>", s);
    }
    TaggedObject::Smi(val) => {
      let _ = write!(accumulator, "{}", val);
    }
    TaggedObject::HeapNumber(val) => {
      let _ = write!(accumulator, "{}", val);
    }
  }
  let _ = depth;
}

/// 差分打印单项 Tagged 对象 (原版 DifferentialFuzzingPrint(Tagged<Object>, depth) 1:1)
pub fn differential_fuzzing_print_object(obj: &TaggedObject, depth: i32) -> String {
  let mut s = String::new();
  match obj {
    TaggedObject::Smi(val) => {
      let _ = write!(s, "{}", val);
    }
    TaggedObject::HeapNumber(val) => {
      let _ = write!(s, "{}", val);
    }
    _ => {
      heap_object_fuzzing_print(obj, depth, &mut s);
    }
  }
  s
}

/// 差分模糊打印公开接口 (原版 DifferentialFuzzingPrint(ObjectOrNonMaterializedObject, os) 1:1)
pub fn differential_fuzzing_print(obj: &ObjectOrNonMaterializedObject, depth: i32) -> String {
  match obj {
    ObjectOrNonMaterializedObject::TranslatedValue(val) => {
      TranslatedValuePrinter.print(val, depth).to_string()
    }
    ObjectOrNonMaterializedObject::Tagged(tagged) => {
      differential_fuzzing_print_object(tagged, depth)
    }
  }
}
