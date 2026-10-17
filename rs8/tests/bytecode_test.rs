use aok::{OK, Void};
use log::info;
use rs8::{
  lexer::Lexer,
  parser::Parser,
  runtime::{
    bytecode_bridge::{
      js_value_to_interpreter_value, try_execute_with_bytecode, try_execute_with_bytecode_and_env,
    },
    interpreter::Interpreter,
  },
  value::JsValue,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_bytecode_while_loop() -> Void {
  info!("Testing bytecode while-loop execution");

  let code = r#"
    var x = 0;
    var i = 0;
    while (i < 10) {
      x = x + 5;
      i = i + 1;
    }
    x;
  "#;

  let mut lexer = Lexer::new(code);
  let tokens = lexer.tokenize_all()?;
  let mut parser = Parser::new(tokens, code);
  let program = parser.parse_program()?;

  let result = try_execute_with_bytecode(&program);
  assert!(
    result.is_some(),
    "Program should compile to Ignition bytecode"
  );
  let val = result.unwrap()?;
  assert_eq!(val, JsValue::Number(50.0));

  OK
}

#[test]
fn test_bytecode_for_loop() -> Void {
  info!("Testing bytecode for-loop execution");

  let code = r#"
    var sum = 0;
    for (var i = 1; i <= 10; i = i + 1) {
      sum = sum + i;
    }
    sum;
  "#;

  let mut lexer = Lexer::new(code);
  let tokens = lexer.tokenize_all()?;
  let mut parser = Parser::new(tokens, code);
  let program = parser.parse_program()?;

  let result = try_execute_with_bytecode(&program);
  assert!(
    result.is_some(),
    "Program should compile to Ignition bytecode"
  );
  let val = result.unwrap()?;
  assert_eq!(val, JsValue::Number(55.0));

  OK
}

#[test]
fn test_bytecode_conditional() -> Void {
  info!("Testing bytecode conditional operator");

  let code = r#"
    var a = 20;
    var b = 30;
    var max = (a > b) ? a : b;
    max;
  "#;

  let mut lexer = Lexer::new(code);
  let tokens = lexer.tokenize_all()?;
  let mut parser = Parser::new(tokens, code);
  let program = parser.parse_program()?;

  let result = try_execute_with_bytecode(&program);
  assert!(
    result.is_some(),
    "Program should compile to Ignition bytecode"
  );
  let val = result.unwrap()?;
  assert_eq!(val, JsValue::Number(30.0));

  OK
}

#[test]
fn test_run_bytecode_fast_loop() -> Void {
  info!("Testing run_bytecode with intensive computation loop");

  let code = r#"
    var acc = 0;
    for (var i = 0; i < 1000; i = i + 1) {
      acc = acc + (i % 7);
    }
    acc;
  "#;

  let val = rs8::run_bytecode(code)?;
  assert_eq!(val, JsValue::Number(2997.0));

  OK
}

#[test]
fn test_cyclic_object_bridge_safety() -> Void {
  info!("Testing cyclic object bridge safety");

  let cyclic_val = JsValue::new_object();
  let _ = cyclic_val.set_property("self", cyclic_val.clone());

  let interp_val = js_value_to_interpreter_value(&cyclic_val);
  assert!(interp_val.is_some());

  let back_js = JsValue::from(interp_val.unwrap());
  assert!(matches!(back_js, JsValue::Object(_)));

  OK
}

#[test]
fn test_bytecode_reference_equality() -> Void {
  info!("Testing bytecode reference equality for objects and arrays");

  let code = r#"
    var a = {};
    var b = a;
    var c = {};
    var is_same = (a === b);
    var is_diff = (a === c);
    var arr1 = [1, 2];
    var arr2 = arr1;
    var arr_same = (arr1 === arr2);
    (is_same && !is_diff && arr_same);
  "#;

  let val = rs8::run_bytecode(code)?;
  assert_eq!(val, JsValue::Boolean(true));

  OK
}

#[test]
fn test_bytecode_unicode_string_methods() -> Void {
  info!("Testing bytecode unicode string methods without panic");

  let code = r#"
    var s = "你好世界";
    var sub = s.substring(1, 3);
    var idx = s.indexOf("界");
    var len = s.length;
    sub + ":" + idx + ":" + len;
  "#;

  let val = rs8::run_bytecode(code)?;
  assert_eq!(val, JsValue::String("好世:3:4".into()));

  OK
}

#[test]
fn test_bytecode_array_native_methods() -> Void {
  info!("Testing bytecode array native methods");

  let code = r#"
    var arr = [10, 20, 30];
    arr.push(40);
    var joined = arr.join("-");
    var idx = arr.indexOf(20);
    var sliced = arr.slice(1, 3).join(",");
    joined + ":" + idx + ":" + sliced;
  "#;

  let val = rs8::run_bytecode(code)?;
  assert_eq!(val, JsValue::String("10-20-30-40:1:20,30".into()));

  OK
}

#[test]
fn test_earley_boyer_direct_bytecode() -> Void {
  info!("Testing earley-boyer direct bytecode");

  let base = include_str!("../../v8-benchmarks/benches/v8-v7/base.js");
  let script = include_str!("../../v8-benchmarks/benches/v8-v7/earley-boyer.js");
  let code = format!("{base}\n{script}");

  let mut lexer = Lexer::new(&code);
  let tokens = lexer.tokenize_all()?;
  let mut parser = Parser::new(tokens, &code);
  let program = parser.parse_program()?;

  let interp = Interpreter::new();
  let res = try_execute_with_bytecode_and_env(&program, Some(&interp.global));
  assert!(res.is_some(), "Earley boyer should compile to bytecode");
  OK
}

#[test]
fn test_raytrace_direct_bytecode() -> Void {
  info!("Testing raytrace direct bytecode");

  let base = include_str!("../../v8-benchmarks/benches/v8-v7/base.js");
  let script = include_str!("../../v8-benchmarks/benches/v8-v7/raytrace.js");
  let driver = r#"
    var __last_suite = (typeof BenchmarkSuite !== 'undefined' && BenchmarkSuite.suites) ? BenchmarkSuite.suites[BenchmarkSuite.suites.length - 1] : null;
    if (__last_suite) {
      for (var __i = 0; __i < __last_suite.benchmarks.length; __i++) {
        var __b = __last_suite.benchmarks[__i];
        if (__b.Setup) __b.Setup(); else if (__b.setup) __b.setup(); else if (__b.init) __b.init();
        __b.run();
        if (__b.TearDown) __b.TearDown(); else if (__b.tearDown) __b.tearDown();
      }
    }
    var __ref_out = (__last_suite && __last_suite.reference) ? __last_suite.reference : 0;
    JSON.stringify({ "reference": __ref_out });
  "#;
  let code = format!("{base}\n{script}\n{driver}");

  let mut lexer = Lexer::new(&code);
  let tokens = lexer.tokenize_all()?;
  let mut parser = Parser::new(tokens, &code);
  let program = parser.parse_program()?;

  let interp = Interpreter::new();
  let res = try_execute_with_bytecode_and_env(&program, Some(&interp.global));
  match res {
    Some(Ok(v)) => {
      info!("Raytrace SUCCESS: {:?}", v);
    }
    other => panic!("Raytrace FAILED: {:?}", other),
  }
  OK
}

#[test]
fn test_regexp_direct_bytecode() -> Void {
  info!("Testing regexp direct bytecode");

  let base = include_str!("../../v8-benchmarks/benches/v8-v7/base.js");
  let script = include_str!("../../v8-benchmarks/benches/v8-v7/regexp.js");
  let code = format!("{base}\n{script}");

  let mut lexer = Lexer::new(&code);
  let tokens = lexer.tokenize_all()?;
  let mut parser = Parser::new(tokens, &code);
  let program = parser.parse_program()?;

  let interp = Interpreter::new();
  let res = try_execute_with_bytecode_and_env(&program, Some(&interp.global));
  assert!(res.is_some(), "Regexp should compile to bytecode");
  OK
}

#[test]
fn test_splay_direct_bytecode() -> Void {
  info!("Testing splay direct bytecode");

  let base = include_str!("../../v8-benchmarks/benches/v8-v7/base.js");
  let script = include_str!("../../v8-benchmarks/benches/v8-v7/splay.js");
  let code = format!("{base}\n{script}");

  let mut lexer = Lexer::new(&code);
  let tokens = lexer.tokenize_all()?;
  let mut parser = Parser::new(tokens, &code);
  let program = parser.parse_program()?;

  let interp = Interpreter::new();
  let res = try_execute_with_bytecode_and_env(&program, Some(&interp.global));
  assert!(res.is_some(), "Splay should compile to bytecode");
  OK
}

#[test]
fn test_navier_stokes_direct_bytecode() -> Void {
  info!("Testing navier-stokes direct bytecode");

  let base = include_str!("../../v8-benchmarks/benches/v8-v7/base.js");
  let script = include_str!("../../v8-benchmarks/benches/v8-v7/navier-stokes.js");
  let code = format!("{base}\n{script}");

  let mut lexer = Lexer::new(&code);
  let tokens = lexer.tokenize_all()?;
  let mut parser = Parser::new(tokens, &code);
  let program = parser.parse_program()?;

  let interp = Interpreter::new();
  let res = try_execute_with_bytecode_and_env(&program, Some(&interp.global));
  assert!(res.is_some(), "NavierStokes should compile to bytecode");
  OK
}

#[test]
fn test_crypto_direct_bytecode() -> Void {
  info!("Testing crypto direct bytecode");

  let base = include_str!("../../v8-benchmarks/benches/v8-v7/base.js");
  let script = include_str!("../../v8-benchmarks/benches/v8-v7/crypto.js");
  let code = format!("{base}\n{script}");

  let mut lexer = Lexer::new(&code);
  let tokens = lexer.tokenize_all()?;
  let mut parser = Parser::new(tokens, &code);
  let program = parser.parse_program()?;

  let interp = Interpreter::new();
  let res = try_execute_with_bytecode_and_env(&program, Some(&interp.global));
  assert!(res.is_some(), "Crypto should compile to bytecode");
  OK
}
