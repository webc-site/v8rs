//! JSOperatorBuilder 单元测试

use aok::{OK, Void};
use v8_compiler_graph::{FeedbackSource, IrOpcode, OperatorProperties, Properties};
use v8_compiler_ops::JSOperatorBuilder;
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_js_cached_operators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let js = JSOperatorBuilder::new(&zone);

  // JSCreate
  let create = js.js_create();
  assert_eq!(create.opcode(), IrOpcode::JSCreate as u16);
  assert_eq!(create.value_input_count(), 2);
  assert_eq!(create.effect_input_count(), 1);
  assert_eq!(create.control_input_count(), 1);
  assert_eq!(create.value_output_count(), 1);
  assert_eq!(create.effect_output_count(), 1);
  assert_eq!(create.control_output_count(), 2); // IfSuccess + IfException

  // JS 算子通过 OperatorProperties 恒有 1 个 Context 输入
  assert!(OperatorProperties::has_context_input(create));
  assert_eq!(OperatorProperties::get_context_input_count(create), 1);

  // JSToNumber & JSToString & JSToObject
  let to_num = js.js_to_number();
  assert_eq!(to_num.opcode(), IrOpcode::JSToNumber as u16);
  assert_eq!(to_num.value_input_count(), 1);

  let to_str = js.js_to_string();
  assert_eq!(to_str.opcode(), IrOpcode::JSToString as u16);

  let to_obj = js.js_to_object();
  assert_eq!(to_obj.opcode(), IrOpcode::JSToObject as u16);
  assert!(to_obj.properties().contains(Properties::FOLDABLE));

  // Eliminatable 算子 (kEliminatable = kNoDeopt | kNoWrite | kNoThrow): control_in = 0, control_out = 0
  let iter_result = js.js_create_iter_result_object();
  assert_eq!(
    iter_result.opcode(),
    IrOpcode::JSCreateIterResultObject as u16
  );
  assert_eq!(iter_result.value_input_count(), 2);
  assert_eq!(iter_result.control_input_count(), 0);
  assert_eq!(iter_result.control_output_count(), 0);

  let promise = js.js_create_promise();
  assert_eq!(promise.opcode(), IrOpcode::JSCreatePromise as u16);
  assert_eq!(promise.control_input_count(), 0);
  assert_eq!(promise.control_output_count(), 0);

  // NoThrow 算子 (含 kNoThrow): control_out = 0
  let async_reject = js.js_async_function_reject();
  assert_eq!(
    async_reject.opcode(),
    IrOpcode::JSAsyncFunctionReject as u16
  );
  assert_eq!(async_reject.control_output_count(), 0);

  OK
}

#[test]
fn test_js_dynamic_operators() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let js = JSOperatorBuilder::new(&zone);

  // JSAdd
  let js_add = js.js_add(FeedbackSource::invalid())?;
  assert_eq!(js_add.opcode(), IrOpcode::JSAdd as u16);
  assert_eq!(js_add.value_input_count(), 3);
  assert_eq!(js_add.effect_input_count(), 1);
  assert_eq!(js_add.control_input_count(), 1);
  assert_eq!(js_add.value_output_count(), 1);
  assert_eq!(js_add.effect_output_count(), 1);
  assert_eq!(js_add.control_output_count(), 2);

  // JSSubtract & JSMultiply & JSDivide
  let js_sub = js.js_subtract(FeedbackSource::invalid())?;
  assert_eq!(js_sub.opcode(), IrOpcode::JSSubtract as u16);

  let js_mul = js.js_multiply(FeedbackSource::invalid())?;
  assert_eq!(js_mul.opcode(), IrOpcode::JSMultiply as u16);

  let js_div = js.js_divide(FeedbackSource::invalid())?;
  assert_eq!(js_div.opcode(), IrOpcode::JSDivide as u16);

  // JSCall (arity = 4: target + receiver + 2 args)
  let js_call = js.js_call(4, 1.0, FeedbackSource::invalid())?;
  assert_eq!(js_call.opcode(), IrOpcode::JSCall as u16);
  assert_eq!(js_call.value_input_count(), 4);
  assert_eq!(js_call.control_output_count(), 2);

  // JSLoadProperty (3 value inputs: object, key, feedback_vector)
  let js_load = js.js_load_property(FeedbackSource::invalid())?;
  assert_eq!(js_load.opcode(), IrOpcode::JSLoadProperty as u16);
  assert_eq!(js_load.value_input_count(), 3);
  assert_eq!(js_load.control_output_count(), 2);

  // JSStoreProperty (4 value inputs: object, key, value, feedback_vector)
  let js_store = js.js_store_property(FeedbackSource::invalid())?;
  assert_eq!(js_store.opcode(), IrOpcode::JSSetKeyedProperty as u16);
  assert_eq!(js_store.value_input_count(), 4);
  assert_eq!(js_store.value_output_count(), 0);

  // JSConstruct (arity = 3: target + new_target + feedback_vector)
  let js_construct = js.js_construct(3, 1.0, FeedbackSource::invalid())?;
  assert_eq!(js_construct.opcode(), IrOpcode::JSConstruct as u16);
  assert_eq!(js_construct.value_input_count(), 3);
  assert_eq!(js_construct.control_output_count(), 2);

  OK
}
