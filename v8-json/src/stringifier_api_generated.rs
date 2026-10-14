// 由 tool/src/json/json-stringifier.h.js 从 v8/src/json/json-stringifier.h 生成; 修订就地改, 复跑覆盖
// JSON 序列化入口签名

/// JSON 序列化总入口 (原版 JsonStringify, json-stringifier.h 唯一声明)。
///
/// 对应 C++ 签名:
///   MaybeDirectHandle<Object> JsonStringify(Isolate*, Handle<JSAny> object,
///                                           Handle<JSAny> replacer, Handle<Object> gap)
///
/// 语义实现见 crate 内手写 stringifier.rs; escape 表见 escape_table.rs。
pub mod stringifier_api_skel {
  use crate::{error::Error, stringifier::JsonStringifier, value::JsonValue};

  /// 序列化失败哨兵 (原版 MaybeHandle 空)
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum StringifyResult {
    /// 成功产出 JSON 文本
    Ok,
    /// 值含 BigInt / 循环引用等, 原版抛 TypeError
    TypeError,
  }

  /// 入口 (原版 JsonStringify: object + replacer(array/function) + gap(缩进串))
  ///
  /// # Safety
  /// `object` 必须为有效 `JsonValue` 指针或 0; `gap` 必须为有效 `String` 指针或 0
  pub unsafe fn json_stringify(
    _isolate_key: usize,
    object: usize,
    _replacer: usize,
    gap: usize,
  ) -> StringifyResult {
    if object == 0 {
      return StringifyResult::Ok;
    }
    let val = unsafe { &*(object as *const JsonValue) };
    let gap_str = if gap == 0 {
      None
    } else {
      let s = unsafe { &*(gap as *const String) };
      Some(s.as_str())
    };
    let stringifier = JsonStringifier::with_indent(gap_str);
    match stringifier.stringify(val) {
      Ok(_) => StringifyResult::Ok,
      Err(_) => StringifyResult::TypeError,
    }
  }

  /// 真实安全的 JSON 序列化辅助函数
  pub fn stringify_value(value: &JsonValue, gap: Option<&str>) -> Result<String, Error> {
    let stringifier = JsonStringifier::with_indent(gap);
    stringifier.stringify(value)
  }
}
