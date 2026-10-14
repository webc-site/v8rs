// 由 tool/src/builtins/builtins-regexp.js 生成; 来源 v8/src/builtins 下 1 文件
// 模块索引
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

#![forbid(unsafe_code)]

mod error;
mod regexp;

pub use error::{Error, Result};
pub use regexp::{
  Builtin as RegexpBuiltin, TQ_DECLS as REGEXP_TQ_DECLS, reg_exp_capture1_getter,
  reg_exp_capture2_getter, reg_exp_capture3_getter, reg_exp_capture4_getter,
  reg_exp_capture5_getter, reg_exp_capture6_getter, reg_exp_capture7_getter,
  reg_exp_capture8_getter, reg_exp_capture9_getter, reg_exp_constructor, reg_exp_escape,
  reg_exp_exec_atom, reg_exp_experimental_trampoline, reg_exp_input_getter, reg_exp_input_setter,
  reg_exp_interpreter_trampoline, reg_exp_last_match_getter, reg_exp_last_paren_getter,
  reg_exp_left_context_getter, reg_exp_match_fast, reg_exp_prototype_compile,
  reg_exp_prototype_exec, reg_exp_prototype_exec_slow, reg_exp_prototype_match,
  reg_exp_prototype_match_all, reg_exp_prototype_replace, reg_exp_prototype_search,
  reg_exp_prototype_source_getter, reg_exp_prototype_split, reg_exp_prototype_test,
  reg_exp_prototype_test_fast, reg_exp_prototype_to_string, reg_exp_replace,
  reg_exp_right_context_getter, reg_exp_search_fast, reg_exp_split,
  reg_exp_string_iterator_prototype_next,
};
