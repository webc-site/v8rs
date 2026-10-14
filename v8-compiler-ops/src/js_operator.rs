// 由 tool/src/compiler/js-operator.h.js 从 v8/src/compiler/js-operator.h + js-operator.cc 生成; 修订就地改, 复跑覆盖

//! JSOperatorBuilder 与 JS 算子族
//!
//! 对应原版 js-operator.h / js-operator.cc。
//! 包含 JSAdd, JSSubtract, JSCall, JSCreate, JSLoadProperty 等。

use core::{
  hash::{Hash, Hasher},
  result::Result,
};
use std::sync::LazyLock;

use v8_compiler_graph::{
  FeedbackSource, IrOpcode, Operator, Operator1, OperatorCounts, Properties,
};
use v8_zone::Zone;

use crate::Error;

/// 调用算子参数 (CallParameters)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CallParameters {
  pub arity: usize,
  pub frequency: f32,
  pub feedback: FeedbackSource,
}

impl CallParameters {
  pub const fn new(arity: usize, frequency: f32, feedback: FeedbackSource) -> Self {
    Self {
      arity,
      frequency,
      feedback,
    }
  }
}

impl Eq for CallParameters {}

impl Hash for CallParameters {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.arity.hash(state);
    self.frequency.to_bits().hash(state);
    self.feedback.hash(state);
  }
}

/// 构造调用算子参数 (ConstructParameters, js-operator.h:170-198)
/// JSConstruct / JSConstructWithArrayLike / JSConstructWithSpread 共用
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstructParameters {
  pub arity: usize,
  pub frequency: f32,
  pub feedback: FeedbackSource,
}

impl ConstructParameters {
  /// 原版 kExtraConstructInputCount (js-operator.h:175, = JSConstructNode::kExtraInputCount:
  /// target + new.target + receiver 三个隐式输入)
  pub const EXTRA_CONSTRUCT_INPUT_COUNT: usize = 3;

  /// 原版构造器 (js-operator.h:176-181):
  /// DCHECK_GE(arity, kExtraConstructInputCount) 与 DCHECK(is_int32(arity)) 以断言承接
  pub fn new(arity: usize, frequency: f32, feedback: FeedbackSource) -> Self {
    debug_assert!(arity >= Self::EXTRA_CONSTRUCT_INPUT_COUNT);
    debug_assert!(arity <= i32::MAX as usize);
    Self {
      arity,
      frequency,
      feedback,
    }
  }

  /// 原版 arity_without_implicit_args (js-operator.h:192)
  pub const fn arity_without_implicit_args(&self) -> i32 {
    (self.arity - Self::EXTRA_CONSTRUCT_INPUT_COUNT) as i32
  }
}

impl Eq for ConstructParameters {}

impl Hash for ConstructParameters {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.arity.hash(state);
    self.frequency.to_bits().hash(state);
    self.feedback.hash(state);
  }
}

/// 全局共享 JS 缓存算子 (镜像原版 JSOperatorGlobalCache)
pub struct JSOperatorGlobalCache {
  pub js_to_length: Operator,
  pub js_to_name: Operator,
  pub js_to_number: Operator,
  pub js_to_number_convert_big_int: Operator,
  pub js_to_big_int: Operator,
  pub js_to_big_int_convert_number: Operator,
  pub js_to_numeric: Operator,
  pub js_to_object: Operator,
  pub js_to_string: Operator,
  pub js_create: Operator,
  pub js_create_iter_result_object: Operator,
  pub js_create_string_iterator: Operator,
  pub js_create_key_value_array: Operator,
  pub js_create_promise: Operator,
  pub js_create_typed_array: Operator,
  pub js_create_object: Operator,
  pub js_create_string_wrapper: Operator,
  pub js_object_is_array: Operator,
  pub js_has_in_prototype_chain: Operator,
  pub js_ordinary_has_instance: Operator,
  pub js_for_in_enumerate: Operator,
  pub js_async_function_await: Operator,
  pub js_async_function_enter: Operator,
  pub js_async_function_reject: Operator,
  pub js_async_function_resolve: Operator,
  pub js_load_message: Operator,
  pub js_store_message: Operator,
  pub js_generator_restore_continuation: Operator,
  pub js_generator_restore_context_no_cell: Operator,
  pub js_generator_restore_input_or_debug_pos: Operator,
  pub js_debugger: Operator,
  pub js_fulfill_promise: Operator,
  pub js_perform_promise_then: Operator,
  pub js_promise_resolve: Operator,
  pub js_reject_promise: Operator,
  pub js_resolve_promise: Operator,
  pub js_get_super_constructor: Operator,
  pub js_find_non_default_constructor_or_construct: Operator,
  pub js_parse_int: Operator,
  pub js_reg_exp_test: Operator,
}

#[inline]
fn unwrap_op<T>(res: Result<T, v8_compiler_graph::Error>) -> T {
  match res {
    Ok(v) => v,
    Err(_) => unreachable!(),
  }
}

impl JSOperatorGlobalCache {
  fn new() -> Self {
    Self {
      js_to_length: unwrap_op(Operator::new(
        IrOpcode::JSToLength as u16,
        Properties::NONE,
        "JSToLength",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_to_name: unwrap_op(Operator::new(
        IrOpcode::JSToName as u16,
        Properties::NONE,
        "JSToName",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_to_number: unwrap_op(Operator::new(
        IrOpcode::JSToNumber as u16,
        Properties::NONE,
        "JSToNumber",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_to_number_convert_big_int: unwrap_op(Operator::new(
        IrOpcode::JSToNumberConvertBigInt as u16,
        Properties::NONE,
        "JSToNumberConvertBigInt",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_to_big_int: unwrap_op(Operator::new(
        IrOpcode::JSToBigInt as u16,
        Properties::NONE,
        "JSToBigInt",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_to_big_int_convert_number: unwrap_op(Operator::new(
        IrOpcode::JSToBigIntConvertNumber as u16,
        Properties::NONE,
        "JSToBigIntConvertNumber",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_to_numeric: unwrap_op(Operator::new(
        IrOpcode::JSToNumeric as u16,
        Properties::NONE,
        "JSToNumeric",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_to_object: unwrap_op(Operator::new(
        IrOpcode::JSToObject as u16,
        Properties::FOLDABLE,
        "JSToObject",
        OperatorCounts::new(
          1,
          (Properties::FOLDABLE).zero_if_pure(),
          (Properties::FOLDABLE).zero_if_eliminatable(),
          1,
          (Properties::FOLDABLE).zero_if_pure(),
          (Properties::FOLDABLE).zero_if_no_throw(),
        ),
      )),
      js_to_string: unwrap_op(Operator::new(
        IrOpcode::JSToString as u16,
        Properties::NONE,
        "JSToString",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_create: unwrap_op(Operator::new(
        IrOpcode::JSCreate as u16,
        Properties::NONE,
        "JSCreate",
        OperatorCounts::new(
          2,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_create_iter_result_object: unwrap_op(Operator::new(
        IrOpcode::JSCreateIterResultObject as u16,
        Properties::ELIMINATABLE,
        "JSCreateIterResultObject",
        OperatorCounts::new(
          2,
          (Properties::ELIMINATABLE).zero_if_pure(),
          (Properties::ELIMINATABLE).zero_if_eliminatable(),
          1,
          (Properties::ELIMINATABLE).zero_if_pure(),
          (Properties::ELIMINATABLE).zero_if_no_throw(),
        ),
      )),
      js_create_string_iterator: unwrap_op(Operator::new(
        IrOpcode::JSCreateStringIterator as u16,
        Properties::ELIMINATABLE,
        "JSCreateStringIterator",
        OperatorCounts::new(
          1,
          (Properties::ELIMINATABLE).zero_if_pure(),
          (Properties::ELIMINATABLE).zero_if_eliminatable(),
          1,
          (Properties::ELIMINATABLE).zero_if_pure(),
          (Properties::ELIMINATABLE).zero_if_no_throw(),
        ),
      )),
      js_create_key_value_array: unwrap_op(Operator::new(
        IrOpcode::JSCreateKeyValueArray as u16,
        Properties::ELIMINATABLE,
        "JSCreateKeyValueArray",
        OperatorCounts::new(
          2,
          (Properties::ELIMINATABLE).zero_if_pure(),
          (Properties::ELIMINATABLE).zero_if_eliminatable(),
          1,
          (Properties::ELIMINATABLE).zero_if_pure(),
          (Properties::ELIMINATABLE).zero_if_no_throw(),
        ),
      )),
      js_create_promise: unwrap_op(Operator::new(
        IrOpcode::JSCreatePromise as u16,
        Properties::ELIMINATABLE,
        "JSCreatePromise",
        OperatorCounts::new(
          0,
          (Properties::ELIMINATABLE).zero_if_pure(),
          (Properties::ELIMINATABLE).zero_if_eliminatable(),
          1,
          (Properties::ELIMINATABLE).zero_if_pure(),
          (Properties::ELIMINATABLE).zero_if_no_throw(),
        ),
      )),
      js_create_typed_array: unwrap_op(Operator::new(
        IrOpcode::JSCreateTypedArray as u16,
        Properties::NONE,
        "JSCreateTypedArray",
        OperatorCounts::new(
          5,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_create_object: unwrap_op(Operator::new(
        IrOpcode::JSCreateObject as u16,
        Properties::NONE,
        "JSCreateObject",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_create_string_wrapper: unwrap_op(Operator::new(
        IrOpcode::JSCreateStringWrapper as u16,
        Properties::ELIMINATABLE,
        "JSCreateStringWrapper",
        OperatorCounts::new(
          1,
          (Properties::ELIMINATABLE).zero_if_pure(),
          (Properties::ELIMINATABLE).zero_if_eliminatable(),
          1,
          (Properties::ELIMINATABLE).zero_if_pure(),
          (Properties::ELIMINATABLE).zero_if_no_throw(),
        ),
      )),
      js_object_is_array: unwrap_op(Operator::new(
        IrOpcode::JSObjectIsArray as u16,
        Properties::NONE,
        "JSObjectIsArray",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_has_in_prototype_chain: unwrap_op(Operator::new(
        IrOpcode::JSHasInPrototypeChain as u16,
        Properties::NONE,
        "JSHasInPrototypeChain",
        OperatorCounts::new(
          2,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_ordinary_has_instance: unwrap_op(Operator::new(
        IrOpcode::JSOrdinaryHasInstance as u16,
        Properties::NONE,
        "JSOrdinaryHasInstance",
        OperatorCounts::new(
          2,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_for_in_enumerate: unwrap_op(Operator::new(
        IrOpcode::JSForInEnumerate as u16,
        Properties::NONE,
        "JSForInEnumerate",
        OperatorCounts::new(
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_async_function_await: unwrap_op(Operator::new(
        IrOpcode::JSAsyncFunctionAwait as u16,
        Properties::NO_DEOPT,
        "JSAsyncFunctionAwait",
        OperatorCounts::new(
          2,
          (Properties::NO_DEOPT).zero_if_pure(),
          (Properties::NO_DEOPT).zero_if_eliminatable(),
          1,
          (Properties::NO_DEOPT).zero_if_pure(),
          (Properties::NO_DEOPT).zero_if_no_throw(),
        ),
      )),
      js_async_function_enter: unwrap_op(Operator::new(
        IrOpcode::JSAsyncFunctionEnter as u16,
        Properties::NONE,
        "JSAsyncFunctionEnter",
        OperatorCounts::new(
          2,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_async_function_reject: unwrap_op(Operator::new(
        IrOpcode::JSAsyncFunctionReject as u16,
        Properties::NO_DEOPT | Properties::NO_THROW,
        "JSAsyncFunctionReject",
        OperatorCounts::new(
          2,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_eliminatable(),
          1,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_async_function_resolve: unwrap_op(Operator::new(
        IrOpcode::JSAsyncFunctionResolve as u16,
        Properties::NO_DEOPT | Properties::NO_THROW,
        "JSAsyncFunctionResolve",
        OperatorCounts::new(
          2,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_eliminatable(),
          1,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_load_message: unwrap_op(Operator::new(
        IrOpcode::JSLoadMessage as u16,
        Properties::NO_THROW | Properties::NO_WRITE,
        "JSLoadMessage",
        OperatorCounts::new(
          0,
          (Properties::NO_THROW | Properties::NO_WRITE).zero_if_pure(),
          (Properties::NO_THROW | Properties::NO_WRITE).zero_if_eliminatable(),
          1,
          (Properties::NO_THROW | Properties::NO_WRITE).zero_if_pure(),
          (Properties::NO_THROW | Properties::NO_WRITE).zero_if_no_throw(),
        ),
      )),
      js_store_message: unwrap_op(Operator::new(
        IrOpcode::JSStoreMessage as u16,
        Properties::NO_READ | Properties::NO_THROW,
        "JSStoreMessage",
        OperatorCounts::new(
          1,
          (Properties::NO_READ | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_READ | Properties::NO_THROW).zero_if_eliminatable(),
          0,
          (Properties::NO_READ | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_READ | Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_generator_restore_continuation: unwrap_op(Operator::new(
        IrOpcode::JSGeneratorRestoreContinuation as u16,
        Properties::NO_THROW,
        "JSGeneratorRestoreContinuation",
        OperatorCounts::new(
          1,
          (Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_THROW).zero_if_eliminatable(),
          1,
          (Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_generator_restore_context_no_cell: unwrap_op(Operator::new(
        IrOpcode::JSGeneratorRestoreContextNoCell as u16,
        Properties::NO_THROW,
        "JSGeneratorRestoreContextNoCell",
        OperatorCounts::new(
          1,
          (Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_THROW).zero_if_eliminatable(),
          1,
          (Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_generator_restore_input_or_debug_pos: unwrap_op(Operator::new(
        IrOpcode::JSGeneratorRestoreInputOrDebugPos as u16,
        Properties::NO_THROW,
        "JSGeneratorRestoreInputOrDebugPos",
        OperatorCounts::new(
          1,
          (Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_THROW).zero_if_eliminatable(),
          1,
          (Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_debugger: unwrap_op(Operator::new(
        IrOpcode::JSDebugger as u16,
        Properties::NONE,
        "JSDebugger",
        OperatorCounts::new(
          0,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          0,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_fulfill_promise: unwrap_op(Operator::new(
        IrOpcode::JSFulfillPromise as u16,
        Properties::NO_DEOPT | Properties::NO_THROW,
        "JSFulfillPromise",
        OperatorCounts::new(
          2,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_eliminatable(),
          1,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_perform_promise_then: unwrap_op(Operator::new(
        IrOpcode::JSPerformPromiseThen as u16,
        Properties::NO_DEOPT | Properties::NO_THROW,
        "JSPerformPromiseThen",
        OperatorCounts::new(
          4,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_eliminatable(),
          1,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_promise_resolve: unwrap_op(Operator::new(
        IrOpcode::JSPromiseResolve as u16,
        Properties::NONE,
        "JSPromiseResolve",
        OperatorCounts::new(
          2,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_reject_promise: unwrap_op(Operator::new(
        IrOpcode::JSRejectPromise as u16,
        Properties::NO_DEOPT | Properties::NO_THROW,
        "JSRejectPromise",
        OperatorCounts::new(
          3,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_eliminatable(),
          1,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_resolve_promise: unwrap_op(Operator::new(
        IrOpcode::JSResolvePromise as u16,
        Properties::NO_DEOPT | Properties::NO_THROW,
        "JSResolvePromise",
        OperatorCounts::new(
          2,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_eliminatable(),
          1,
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_DEOPT | Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_get_super_constructor: unwrap_op(Operator::new(
        IrOpcode::JSGetSuperConstructor as u16,
        Properties::NO_WRITE | Properties::NO_THROW,
        "JSGetSuperConstructor",
        OperatorCounts::new(
          1,
          (Properties::NO_WRITE | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_WRITE | Properties::NO_THROW).zero_if_eliminatable(),
          1,
          (Properties::NO_WRITE | Properties::NO_THROW).zero_if_pure(),
          (Properties::NO_WRITE | Properties::NO_THROW).zero_if_no_throw(),
        ),
      )),
      js_find_non_default_constructor_or_construct: unwrap_op(Operator::new(
        IrOpcode::JSFindNonDefaultConstructorOrConstruct as u16,
        Properties::NONE,
        "JSFindNonDefaultConstructorOrConstruct",
        OperatorCounts::new(
          2,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          2,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_parse_int: unwrap_op(Operator::new(
        IrOpcode::JSParseInt as u16,
        Properties::NONE,
        "JSParseInt",
        OperatorCounts::new(
          2,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
      js_reg_exp_test: unwrap_op(Operator::new(
        IrOpcode::JSRegExpTest as u16,
        Properties::NONE,
        "JSRegExpTest",
        OperatorCounts::new(
          2,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_eliminatable(),
          1,
          (Properties::NONE).zero_if_pure(),
          (Properties::NONE).zero_if_no_throw(),
        ),
      )),
    }
  }
}

static JS_CACHE: LazyLock<JSOperatorGlobalCache> = LazyLock::new(JSOperatorGlobalCache::new);

/// JSOperatorBuilder
pub struct JSOperatorBuilder<'z, 'a> {
  zone: &'z Zone<'a>,
}

impl<'z, 'a> JSOperatorBuilder<'z, 'a> {
  /// 创建构建器
  pub fn new(zone: &'z Zone<'a>) -> Self {
    Self { zone }
  }

  /// 底层 Zone 引用
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  // —— 静态缓存 JS 算子 ——

  /// JSToLength
  #[inline]
  pub fn js_to_length(&self) -> &'static Operator {
    &JS_CACHE.js_to_length
  }

  /// JSToName
  #[inline]
  pub fn js_to_name(&self) -> &'static Operator {
    &JS_CACHE.js_to_name
  }

  /// JSToNumber
  #[inline]
  pub fn js_to_number(&self) -> &'static Operator {
    &JS_CACHE.js_to_number
  }

  /// JSToNumberConvertBigInt
  #[inline]
  pub fn js_to_number_convert_big_int(&self) -> &'static Operator {
    &JS_CACHE.js_to_number_convert_big_int
  }

  /// JSToBigInt
  #[inline]
  pub fn js_to_big_int(&self) -> &'static Operator {
    &JS_CACHE.js_to_big_int
  }

  /// JSToBigIntConvertNumber
  #[inline]
  pub fn js_to_big_int_convert_number(&self) -> &'static Operator {
    &JS_CACHE.js_to_big_int_convert_number
  }

  /// JSToNumeric
  #[inline]
  pub fn js_to_numeric(&self) -> &'static Operator {
    &JS_CACHE.js_to_numeric
  }

  /// JSToObject
  #[inline]
  pub fn js_to_object(&self) -> &'static Operator {
    &JS_CACHE.js_to_object
  }

  /// JSToString
  #[inline]
  pub fn js_to_string(&self) -> &'static Operator {
    &JS_CACHE.js_to_string
  }

  /// JSCreate
  #[inline]
  pub fn js_create(&self) -> &'static Operator {
    &JS_CACHE.js_create
  }

  /// JSCreateIterResultObject
  #[inline]
  pub fn js_create_iter_result_object(&self) -> &'static Operator {
    &JS_CACHE.js_create_iter_result_object
  }

  /// JSCreateStringIterator
  #[inline]
  pub fn js_create_string_iterator(&self) -> &'static Operator {
    &JS_CACHE.js_create_string_iterator
  }

  /// JSCreateKeyValueArray
  #[inline]
  pub fn js_create_key_value_array(&self) -> &'static Operator {
    &JS_CACHE.js_create_key_value_array
  }

  /// JSCreatePromise
  #[inline]
  pub fn js_create_promise(&self) -> &'static Operator {
    &JS_CACHE.js_create_promise
  }

  /// JSCreateTypedArray
  #[inline]
  pub fn js_create_typed_array(&self) -> &'static Operator {
    &JS_CACHE.js_create_typed_array
  }

  /// JSCreateObject
  #[inline]
  pub fn js_create_object(&self) -> &'static Operator {
    &JS_CACHE.js_create_object
  }

  /// JSCreateStringWrapper
  #[inline]
  pub fn js_create_string_wrapper(&self) -> &'static Operator {
    &JS_CACHE.js_create_string_wrapper
  }

  /// JSObjectIsArray
  #[inline]
  pub fn js_object_is_array(&self) -> &'static Operator {
    &JS_CACHE.js_object_is_array
  }

  /// JSHasInPrototypeChain
  #[inline]
  pub fn js_has_in_prototype_chain(&self) -> &'static Operator {
    &JS_CACHE.js_has_in_prototype_chain
  }

  /// JSOrdinaryHasInstance
  #[inline]
  pub fn js_ordinary_has_instance(&self) -> &'static Operator {
    &JS_CACHE.js_ordinary_has_instance
  }

  /// JSForInEnumerate
  #[inline]
  pub fn js_for_in_enumerate(&self) -> &'static Operator {
    &JS_CACHE.js_for_in_enumerate
  }

  /// JSAsyncFunctionAwait
  #[inline]
  pub fn js_async_function_await(&self) -> &'static Operator {
    &JS_CACHE.js_async_function_await
  }

  /// JSAsyncFunctionEnter
  #[inline]
  pub fn js_async_function_enter(&self) -> &'static Operator {
    &JS_CACHE.js_async_function_enter
  }

  /// JSAsyncFunctionReject
  #[inline]
  pub fn js_async_function_reject(&self) -> &'static Operator {
    &JS_CACHE.js_async_function_reject
  }

  /// JSAsyncFunctionResolve
  #[inline]
  pub fn js_async_function_resolve(&self) -> &'static Operator {
    &JS_CACHE.js_async_function_resolve
  }

  /// JSLoadMessage
  #[inline]
  pub fn js_load_message(&self) -> &'static Operator {
    &JS_CACHE.js_load_message
  }

  /// JSStoreMessage
  #[inline]
  pub fn js_store_message(&self) -> &'static Operator {
    &JS_CACHE.js_store_message
  }

  /// JSGeneratorRestoreContinuation
  #[inline]
  pub fn js_generator_restore_continuation(&self) -> &'static Operator {
    &JS_CACHE.js_generator_restore_continuation
  }

  /// JSGeneratorRestoreContextNoCell
  #[inline]
  pub fn js_generator_restore_context_no_cell(&self) -> &'static Operator {
    &JS_CACHE.js_generator_restore_context_no_cell
  }

  /// JSGeneratorRestoreInputOrDebugPos
  #[inline]
  pub fn js_generator_restore_input_or_debug_pos(&self) -> &'static Operator {
    &JS_CACHE.js_generator_restore_input_or_debug_pos
  }

  /// JSDebugger
  #[inline]
  pub fn js_debugger(&self) -> &'static Operator {
    &JS_CACHE.js_debugger
  }

  /// JSFulfillPromise
  #[inline]
  pub fn js_fulfill_promise(&self) -> &'static Operator {
    &JS_CACHE.js_fulfill_promise
  }

  /// JSPerformPromiseThen
  #[inline]
  pub fn js_perform_promise_then(&self) -> &'static Operator {
    &JS_CACHE.js_perform_promise_then
  }

  /// JSPromiseResolve
  #[inline]
  pub fn js_promise_resolve(&self) -> &'static Operator {
    &JS_CACHE.js_promise_resolve
  }

  /// JSRejectPromise
  #[inline]
  pub fn js_reject_promise(&self) -> &'static Operator {
    &JS_CACHE.js_reject_promise
  }

  /// JSResolvePromise
  #[inline]
  pub fn js_resolve_promise(&self) -> &'static Operator {
    &JS_CACHE.js_resolve_promise
  }

  /// JSGetSuperConstructor
  #[inline]
  pub fn js_get_super_constructor(&self) -> &'static Operator {
    &JS_CACHE.js_get_super_constructor
  }

  /// JSFindNonDefaultConstructorOrConstruct
  #[inline]
  pub fn js_find_non_default_constructor_or_construct(&self) -> &'static Operator {
    &JS_CACHE.js_find_non_default_constructor_or_construct
  }

  /// JSParseInt
  #[inline]
  pub fn js_parse_int(&self) -> &'static Operator {
    &JS_CACHE.js_parse_int
  }

  /// JSRegExpTest
  #[inline]
  pub fn js_reg_exp_test(&self) -> &'static Operator {
    &JS_CACHE.js_reg_exp_test
  }

  /// JSAdd 算子
  pub fn js_add(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSAdd as u16,
      Properties::NONE,
      "JSAdd",
      OperatorCounts::new(3, 1, 1, 1, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSSubtract 算子
  pub fn js_subtract(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSSubtract as u16,
      Properties::NONE,
      "JSSubtract",
      OperatorCounts::new(3, 1, 1, 1, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSMultiply 算子
  pub fn js_multiply(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSMultiply as u16,
      Properties::NONE,
      "JSMultiply",
      OperatorCounts::new(3, 1, 1, 1, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSDivide 算子
  pub fn js_divide(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSDivide as u16,
      Properties::NONE,
      "JSDivide",
      OperatorCounts::new(3, 1, 1, 1, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSCall 算子
  pub fn js_call(
    &self,
    arity: usize,
    frequency: f32,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    let params = CallParameters::new(arity, frequency, feedback);
    let op = Operator1::new(
      IrOpcode::JSCall as u16,
      Properties::NONE,
      "JSCall",
      OperatorCounts::new(arity as u32, 1, 1, 1, 1, 2),
      params,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSConstruct 算子 (js-operator.cc:1134): value_in = arity, control_out = 2
  pub fn js_construct(
    &self,
    arity: usize,
    frequency: f32,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    let params = ConstructParameters::new(arity, frequency, feedback);
    let op = Operator1::new(
      IrOpcode::JSConstruct as u16,
      Properties::NONE,
      "JSConstruct",
      OperatorCounts::new(arity as u32, 1, 1, 1, 1, 2),
      params,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSLoadProperty 算子
  pub fn js_load_property(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSLoadProperty as u16,
      Properties::NONE,
      "JSLoadProperty",
      OperatorCounts::new(3, 1, 1, 1, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// JSStoreProperty 算子
  pub fn js_store_property(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::JSSetKeyedProperty as u16,
      Properties::NONE,
      "JSSetKeyedProperty",
      OperatorCounts::new(4, 1, 1, 0, 1, 2),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }
}
