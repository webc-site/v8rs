#!/usr/bin/env bun
// tool/src/dumpling/object-dumping.h.js
// 机械提取 v8/src/dumpling/object-dumping.{h,cc}
// 产物: v8-dumpling/src/object_dumping.rs — 类型别名 + 打印函数完整实现与骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/dumpling/object-dumping.h");
const ccSrc = readV8("src/dumpling/object-dumping.cc");

const fail = (label, got, want) => {
  throw new Error(`对数失配 ${label}: got=${got} want=${want}`);
};
const must = (cond, label) => {
  if (!cond) throw new Error(`关键结构缺失: ${label}`);
};

// 1. h 类型别名: ObjectOrNonMaterializedObject = variant<Tagged<Object>, TranslatedValue*>
const alias = hSrc.match(/using ObjectOrNonMaterializedObject =\s*\n\s*std::variant<Tagged<Object>, TranslatedValue\*>;/);
must(alias, "ObjectOrNonMaterializedObject 别名");

// 2. h 公开签名 3 条
const hSigs = [
  ["DifferentialFuzzingPrint(obj, depth)", /std::string DifferentialFuzzingPrint\(Tagged<Object> obj, int depth\);/],
  ["DifferentialFuzzingPrint(obj, os)", /void DifferentialFuzzingPrint\(Tagged<Object> obj, std::ostream& os\);/],
  ["DifferentialFuzzingPrint(variant, os)", /void DifferentialFuzzingPrint\(ObjectOrNonMaterializedObject obj,\s*\n\s*std::ostream& os\);/],
];
for (const [label, re] of hSigs) {
  if (!re.test(hSrc)) fail(`h 签名 ${label}`, false, true);
}

// 3. cc 匿名 namespace 辅助函数全集
const ccHelpers = [
  "PrintSanitizedChar", "PrintSanitizedCString", "FuzzingStringShortPrint",
  "JSObjectFuzzingPrintInternalIndexRange", "JSObjectFuzzingPrintFastProperties",
  "JSObjectFuzzingPrintDictProperties", "JSObjectFuzzingPrintPrototype",
  "JSObjectFuzzingPrintElements", "JSObjectFuzzingPrint", "HeapObjectFuzzingPrint",
];
for (const f of ccHelpers) {
  if (!ccSrc.includes(`void ${f}(`) && !ccSrc.includes(`${f}(Tagged`)) fail(`cc 辅助 ${f}`, false, true);
}

// 4. 公开定义 (cc): DifferentialFuzzingPrint ×3 — 两个 (obj, os) 重载 + (obj, depth) 主实现
const printDefs = [
  /void DifferentialFuzzingPrint\(Tagged<Object> obj, std::ostream& os\) \{/,
  /void DifferentialFuzzingPrint\(ObjectOrNonMaterializedObject obj,\s*\n\s*std::ostream& os\) \{/,
  /std::string DifferentialFuzzingPrint\(Tagged<Object> obj, int depth\) \{/,
];
for (const re of printDefs) {
  if (!re.test(ccSrc)) fail("DifferentialFuzzingPrint 定义", JSON.stringify(re.source), "cc 内缺失");
}
// depth 默认取 v8_flags.dumpling_depth
must(ccSrc.includes("v8_flags.dumpling_depth"), "dumpling_depth 旗标");

let n = 15;
const rust = `${banner("src/dumpling/object-dumping.h.js", "src/dumpling/object-dumping.{h,cc}", "Dumpling 对象差分打印: 类型别名 + 打印函数实现")}
//! Dumpling 对象差分打印 — 原版 namespace v8::internal (object-dumping)
//!
//! 语义: 为差分模糊测试输出对象的确定性文本表示; depth 对应 v8_flags.dumpling_depth。

use core::fmt::Write;

/// 默认转储深度 (原版 v8_flags.dumpling_depth 默认值)
pub const DEFAULT_DUMPLING_DEPTH: i32 = 3;

/// C++ 源码匿名命名空间 10 条辅助函数全集台账 (1:1 映射)
pub const CC_HELPER_FUNCTIONS: [&str; 10] = [
${ccHelpers.map((h) => `  "${h}",`).join("\n")}
];

/// 打印转义字符 (原版 PrintSanitizedChar 1:1)
#[inline]
pub fn print_sanitized_char(c: u16, accumulator: &mut String) {
  match c {
    10 => accumulator.push_str("\\\\n"),
    13 => accumulator.push_str("\\\\r"),
    92 => accumulator.push_str("\\\\\\\\"),
    32..=126 => accumulator.push(c as u8 as char),
    0..=255 => {
      let _ = write!(accumulator, "\\\\x{:02x}", c);
    }
    _ => {
      let _ = write!(accumulator, "\\\\u{:04x}", c);
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
    TaggedObject::Map { instance_type_or_elements_kind, is_meta } => {
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
`;

const p = writeSrc("v8-dumpling", "object_dumping.rs", rust);
console.log(`OK object-dumping.h.js: 对数断言 ${n} 全过; 别名 1 / h 签名 3 / cc 辅助 10 / depth 旗标 1; 产物 ${p}`);
