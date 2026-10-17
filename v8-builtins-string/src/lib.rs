// 由 tool/src/builtins/builtins-string.js 生成; 来源 v8/src/builtins 下 2 文件
// 模块索引
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

#![forbid(unsafe_code)]

mod error;
mod string;
mod string_tsa;

pub use error::{Error, Result};
pub use string::{
  Builtin as StringBuiltin, string_add_no_map_check, string_compare, string_equal,
  string_from_char_code, string_from_code_point, string_from_code_point_at, string_greater_than,
  string_greater_than_or_equal, string_less_than, string_less_than_or_equal,
  string_prototype_last_index_of, string_prototype_locale_compare, string_prototype_match_all,
  string_prototype_normalize, string_prototype_replace, string_prototype_split,
  string_prototype_to_locale_lower_case, string_prototype_to_locale_upper_case,
  string_prototype_to_lower_case, string_prototype_to_upper_case, string_raw, string_substring,
  sub_string, wasm_js_string_equal, wasm_string_add_no_map_check, wasm_string_compare,
};
pub use string_tsa::Builtin as StringTsaBuiltin;
