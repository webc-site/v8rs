use aok::{OK, Void};
use log::info;
use v8_common::MessageTemplate;
use v8_execution_messages::{
  CallSiteFrame, Error, FrameSkipMode, MessageFormatter, MessageLocation, StackTraceCollection,
  StackTraceFormatter, TEMPLATES_WITH_MISMATCHED_ARGUMENTS, ToStringMessageSource,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_enums_and_constants() -> Void {
  // FrameSkipMode
  assert_eq!(FrameSkipMode::SkipFirst as u8, 0);
  assert_eq!(FrameSkipMode::SkipUntilSeen as u8, 1);
  assert_eq!(FrameSkipMode::SkipNone as u8, 2);

  // StackTraceCollection
  assert_eq!(StackTraceCollection::Enabled as u8, 0);
  assert_eq!(StackTraceCollection::Disabled as u8, 1);

  // ToStringMessageSource
  assert_eq!(ToStringMessageSource::PreferOriginalMessage as u8, 0);
  assert_eq!(ToStringMessageSource::CurrentMessageProperty as u8, 1);

  // kTemplatesWithMismatchedArguments 白名单数量与抽样
  assert_eq!(TEMPLATES_WITH_MISMATCHED_ARGUMENTS.len(), 43);
  assert!(MessageFormatter::allows_mismatched_arguments(
    MessageTemplate::ConstAssign
  ));
  assert!(MessageFormatter::allows_mismatched_arguments(
    MessageTemplate::MustBePositive
  ));
  assert!(!MessageFormatter::allows_mismatched_arguments(
    MessageTemplate::TargetNonFunction
  ));

  info!("基础枚举与白名单常量测试通过");
  OK
}

#[test]
fn test_message_location() -> Void {
  let loc1 = MessageLocation::with_positions(10, 25);
  assert_eq!(loc1.start_pos, 10);
  assert_eq!(loc1.end_pos, 25);
  assert_eq!(loc1.bytecode_offset, -1);
  assert_eq!(loc1.line_number, None);

  let loc2 = MessageLocation::with_bytecode_offset(42);
  assert_eq!(loc2.start_pos, -1);
  assert_eq!(loc2.bytecode_offset, 42);

  info!("MessageLocation 位置构造测试通过");
  OK
}

#[test]
fn test_call_site_frame_formatting() -> Void {
  // 普通函数调用帧
  let frame1 = CallSiteFrame {
    function_name: Some("renderUI".into()),
    script_name: Some("app.js".into()),
    line_number: Some(120),
    column_number: Some(15),
    is_eval: false,
    is_constructor: false,
    is_wasm: false,
  };
  assert_eq!(frame1.format(), "at renderUI (app.js:120:15)");

  // 构造函数调用帧
  let frame2 = CallSiteFrame {
    function_name: Some("MyClass".into()),
    script_name: Some("models.js".into()),
    line_number: Some(30),
    column_number: Some(5),
    is_eval: false,
    is_constructor: true,
    is_wasm: false,
  };
  assert_eq!(frame2.format(), "at new MyClass (models.js:30:5)");

  // 匿名顶层帧
  let frame3 = CallSiteFrame {
    function_name: None,
    script_name: Some("index.js".into()),
    line_number: Some(1),
    column_number: Some(1),
    is_eval: false,
    is_constructor: false,
    is_wasm: false,
  };
  assert_eq!(frame3.format(), "at index.js:1:1");

  // 匿名构造函数调用帧 (原版格式化为 at new <anonymous> (...))
  let frame_anon_ctor = CallSiteFrame {
    function_name: None,
    script_name: Some("app.js".into()),
    line_number: Some(10),
    column_number: Some(5),
    is_eval: false,
    is_constructor: true,
    is_wasm: false,
  };
  assert_eq!(frame_anon_ctor.format(), "at new <anonymous> (app.js:10:5)");

  // eval 帧
  let frame_eval = CallSiteFrame {
    function_name: None,
    script_name: Some("eval.js".into()),
    line_number: Some(1),
    column_number: Some(1),
    is_eval: true,
    is_constructor: false,
    is_wasm: false,
  };
  assert_eq!(frame_eval.format(), "at eval (eval.js:1:1)");

  // WASM 帧 (带函数名)
  let frame4 = CallSiteFrame {
    function_name: Some("computeHash".into()),
    script_name: Some("crypto.wasm".into()),
    line_number: Some(1),
    column_number: Some(256),
    is_eval: false,
    is_constructor: false,
    is_wasm: true,
  };
  assert_eq!(frame4.format(), "at computeHash (crypto.wasm:1:256)");

  // WASM 帧 (无函数名)
  let frame_wasm_anon = CallSiteFrame {
    function_name: None,
    script_name: Some("math.wasm".into()),
    line_number: Some(2),
    column_number: Some(10),
    is_eval: false,
    is_constructor: false,
    is_wasm: true,
  };
  assert_eq!(
    frame_wasm_anon.format(),
    "at wasm-function (math.wasm:2:10)"
  );

  // 综合异常栈跟踪格式化
  let title = "TypeError: Cannot read properties of undefined";
  let stack = StackTraceFormatter::format_stack_trace(title, &[frame1, frame2, frame3]);
  let expected = "TypeError: Cannot read properties of undefined\n    at renderUI (app.js:120:15)\n    at new MyClass (models.js:30:5)\n    at index.js:1:1";
  assert_eq!(stack, expected);

  info!("CallSiteFrame 与 StackTraceFormatter 格式化测试通过");
  OK
}

#[test]
fn test_message_formatter() -> Void {
  // 1. 标准占位符替换 (TargetNonFunction: "% was called on %, which is % and not a function")
  let formatted = MessageFormatter::try_format(
    MessageTemplate::TargetNonFunction,
    &["foo", "bar", "undefined"],
  )?;
  assert_eq!(
    formatted,
    "foo was called on bar, which is undefined and not a function"
  );

  // 2. 模板占位符转义 %% 测试
  let res_percent = MessageFormatter::try_format_template_str("% %%", &["100"], false)?;
  assert_eq!(res_percent, "100 %");

  let res_placeholder = MessageFormatter::try_format(MessageTemplate::PlaceholderOnly, &["100%"])?;
  assert_eq!(res_placeholder, "100%");

  // 3. 白名单模板参数不足: 回退为 "undefined" (MustBePositive 原型为 "must be positive: %" 或类似)
  let res_mismatch = MessageFormatter::try_format(MessageTemplate::MustBePositive, &[])?;
  assert!(
    res_mismatch.contains("undefined"),
    "白名单应安全插入 undefined"
  );

  // 4. 非白名单模板参数不足: 返回 MissingTemplateArgument 错误
  let err_missing = MessageFormatter::try_format(MessageTemplate::TargetNonFunction, &["foo"]);
  assert!(matches!(
    err_missing,
    Err(Error::MissingTemplateArgument { .. })
  ));

  // 5. 非白名单模板参数过多: 返回 TooManyTemplateArguments 错误
  let err_too_many = MessageFormatter::try_format(
    MessageTemplate::TargetNonFunction,
    &["1", "2", "3", "4", "5"],
  );
  assert!(matches!(
    err_too_many,
    Err(Error::TooManyTemplateArguments { .. })
  ));

  // 6. format 容错降级
  let fallback = MessageFormatter::format(MessageTemplate::TargetNonFunction, &["only_one"]);
  assert!(fallback.contains("only_one"));
  let fallback_too_many =
    MessageFormatter::format(MessageTemplate::TargetNonFunction, &["1", "2", "3", "4"]);
  assert!(fallback_too_many.contains("[args: 1, 2, 3, 4]"));

  info!("MessageFormatter 占位符、转义、容错与错误校验测试通过");
  OK
}
