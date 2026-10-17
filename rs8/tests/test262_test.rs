//! Test262 标准符合性核心集成测试
//!
//! 覆盖语言核心表达式 (expressions) 与控制流语句 (statements)，
//! 验证 ECMA-262 变量提升、隐式类型转换、原型链查找与 Test262 harness 断言框架。

use std::{env::temp_dir, fs, fs::remove_file, path::Path};

use aok::{OK, Void};
use rs8::{Error, JsValue, Shell, runtime::Interpreter};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 静态内嵌 Test262 官方核心 Harness 脚本 (避免 CI 容器或脱离 v8 目录时文件缺失)
const HARNESS_STA_JS: &str = include_str!("fixtures/harness/sta.js");
const HARNESS_ASSERT_JS: &str = include_str!("fixtures/harness/assert.js");

fn run_with_harness(test_source: &str) -> rs8::Result<JsValue> {
  let mut interpreter = Interpreter::new();
  interpreter.run(HARNESS_STA_JS)?;
  interpreter.run(HARNESS_ASSERT_JS)?;
  interpreter.run(test_source)
}

/// 1. 测试框架基础断言与错误类型
#[test]
fn test_harness_basic_assertions() -> Void {
  let test_code = r#"
    assert(true, "true should pass");
    assert.sameValue(1 + 1, 2, "1 + 1 === 2");
    assert.notSameValue(1, 2, "1 !== 2");
    assert.sameValue(Number.MIN_VALUE > 0, true, "MIN_VALUE is positive");
    assert.sameValue(isNegativeZero(-0), true, "-0 is negative zero");
    assert.sameValue(isNegativeZero(+0), false, "+0 is not negative zero");
  "#;
  run_with_harness(test_code)?;
  OK
}

/// 2. 算术与一元操作符及极值处理
#[test]
fn test_expressions_arithmetic_extremes() -> Void {
  let test_code = r#"
    assert.sameValue(-Number.MIN_VALUE + Number.MIN_VALUE, +0);
    assert.sameValue(1 / (-Number.MIN_VALUE + Number.MIN_VALUE), Number.POSITIVE_INFINITY);
    assert.sameValue(-Number.MAX_VALUE + Number.MAX_VALUE, +0);
    assert.sameValue(1 / (-Number.MAX_VALUE + Number.MAX_VALUE), Number.POSITIVE_INFINITY);
    assert.notSameValue(+0, -0);
    assert.sameValue(1 / +0, Infinity);
    assert.sameValue(1 / -0, -Infinity);
    assert.sameValue(0 / 0 !== 0 / 0, true); // NaN !== NaN
  "#;
  run_with_harness(test_code)?;
  OK
}

/// 3. 抽象相等与严格全等比较
#[test]
fn test_expressions_equality_coercion() -> Void {
  let test_code = r#"
    assert.sameValue(null == undefined, true);
    assert.sameValue(undefined == null, true);
    assert.sameValue(null === undefined, false);
    assert.sameValue("42" == 42, true);
    assert.sameValue(42 == "42", true);
    assert.sameValue("42" === 42, false);
    assert.sameValue(true == 1, true);
    assert.sameValue(false == 0, true);
    assert.sameValue(true == "1", true);
    assert.sameValue(false == "", true);
    assert.sameValue([1, 2] == "1,2", true);
  "#;
  run_with_harness(test_code)?;
  OK
}

/// 4. 关系比较与逻辑短路操作
#[test]
fn test_expressions_relational_and_logical() -> Void {
  let test_code = r#"
    assert.sameValue("apple" < "banana", true);
    assert.sameValue("10" < "9", true); // 字符串字典序比较
    assert.sameValue(10 < 9, false);

    var sideEffect = 0;
    var res1 = false && (sideEffect = 1);
    assert.sameValue(res1, false);
    assert.sameValue(sideEffect, 0); // 确认短路

    var res2 = true || (sideEffect = 2);
    assert.sameValue(res2, true);
    assert.sameValue(sideEffect, 0); // 确认短路
  "#;
  run_with_harness(test_code)?;
  OK
}

/// 5. 控制流循环与分支跳转
#[test]
fn test_statements_control_flow() -> Void {
  let test_code = r#"
    // for 循环与空 step
    var accessed = false;
    var obj = { value: false };
    for (var i = 0; obj; ) {
      accessed = true;
      break;
    }
    assert.sameValue(accessed, true);

    // do..while
    var count = 0;
    do {
      count++;
    } while (count < 5);
    assert.sameValue(count, 5);

    // for..in
    var keys = [];
    var sample = { a: 10, b: 20, c: 30 };
    for (var k in sample) {
      keys.push(k);
    }
    assert.sameValue(keys.length, 3);

    // switch 穿透与 break
    var fallthrough = [];
    var sw = 2;
    switch (sw) {
      case 1:
        fallthrough.push(1);
        break;
      case 2:
        fallthrough.push(2);
      case 3:
        fallthrough.push(3);
        break;
      default:
        fallthrough.push(4);
    }
    assert.sameValue(fallthrough.length, 2);
    assert.sameValue(fallthrough[0], 2);
    assert.sameValue(fallthrough[1], 3);
  "#;
  run_with_harness(test_code)?;
  OK
}

/// 6. 词法作用域与变量提升
#[test]
fn test_hoisting_and_closures() -> Void {
  let test_code = r#"
    // 函数提升优先于执行
    assert.sameValue(typeof hoistedFunc, "function");
    assert.sameValue(hoistedFunc(), 42);
    function hoistedFunc() { return 42; }

    // var 变量提升为 undefined
    assert.sameValue(hoistedVar, undefined);
    var hoistedVar = "hello";
    assert.sameValue(hoistedVar, "hello");

    // 块级代码中 var 声明在函数级生效
    {
      var blockVar = 100;
    }
    assert.sameValue(blockVar, 100);

    // 闭包变量捕获
    function makeCounter() {
      var n = 0;
      return function() {
        n = n + 1;
        return n;
      };
    }
    var counter = makeCounter();
    assert.sameValue(counter(), 1);
    assert.sameValue(counter(), 2);
  "#;
  run_with_harness(test_code)?;
  OK
}

/// 7. 内建构造器与原型链反射
#[test]
fn test_builtins_prototypes_and_reflection() -> Void {
  let test_code = r#"
    // Object.prototype.toString.call
    assert.sameValue(Object.prototype.toString.call(123), "[object Number]");
    assert.sameValue(Object.prototype.toString.call("abc"), "[object String]");
    assert.sameValue(Object.prototype.toString.call(true), "[object Boolean]");
    assert.sameValue(Object.prototype.toString.call([]), "[object Array]");
    assert.sameValue(Object.prototype.toString.call({}), "[object Object]");

    // Array.prototype.map.call
    var mapped = Array.prototype.map.call([1, 2, 3], function(x) { return x * 10; });
    assert.sameValue(mapped.join(", "), "10, 20, 30");

    // JSON.stringify
    assert.sameValue(JSON.stringify({ a: 1, b: "text" }), '{"a":1,"b":"text"}');
    assert.sameValue(JSON.stringify([1, "x", null]), '[1,"x",null]');

    // instanceof 原型链查找
    function Person(name) { this.name = name; }
    var p = new Person("Alice");
    assert.sameValue(p instanceof Person, true);
    assert.sameValue(p instanceof Object, true);
    assert.sameValue(p instanceof Array, false);
  "#;
  run_with_harness(test_code)?;
  OK
}

/// 8. 异常捕获与断言校验
#[test]
fn test_exceptions_and_assert_throws() -> Void {
  let test_code = r#"
    var threw = false;
    try {
      nonExistentVariable();
    } catch (e) {
      threw = true;
    }
    assert.sameValue(threw, true);

    assert.throws(Test262Error, function() {
      throw new Test262Error("explicit error");
    });
  "#;
  run_with_harness(test_code)?;
  OK
}

/// 9. 官方测试用例执行验证
#[test]
fn test_real_test262_official_cases() -> Void {
  let base_dirs = [
    "v8/test/test262/test",
    "../v8/test/test262/test",
    "../../v8/test/test262/test",
  ];
  let mut test_dir = None;
  for b in base_dirs {
    if Path::new(b).exists() {
      test_dir = Some(b);
      break;
    }
  }
  let Some(dir) = test_dir else {
    // 软链接或数据若不可达则跳过外部文件测试
    return OK;
  };

  let cases = [
    format!("{dir}/language/expressions/addition/S11.6.1_A4_T7.js"),
    format!("{dir}/language/expressions/addition/S11.6.1_A3.1_T1.1.js"),
    format!("{dir}/language/expressions/addition/S11.6.1_A4_T1.js"),
    format!("{dir}/language/statements/while/S12.6.2_A1.js"),
    format!("{dir}/language/statements/for/12.6.3_2-3-a-ii-1.js"),
    format!("{dir}/language/statements/if/cptn-else-false-nrml.js"),
  ];

  for case_path in cases {
    if let Ok(src) = fs::read_to_string(&case_path) {
      // 失败必须带上用例名: 官方用例只留断言文本时无法定位是哪个文件红了
      run_with_harness(&src).map_err(|e| Error::RuntimeError(format!("{case_path}: {e}")))?;
    }
  }

  OK
}

/// 10. 执行外壳与隔离区生命周期验证
#[test]
fn test_d8_shell_execution_and_lifecycle() -> Void {
  let mut shell = Shell::new();
  assert!(!shell.options().can_block);
  shell.options_mut().can_block = true;
  assert!(shell.options().can_block);

  let isolate = shell.create_isolate();
  let val = shell.execute_string(&isolate, "1 + 2 * 3", None, false, false)?;
  assert_eq!(val, JsValue::Number(7.0));

  assert_eq!(Shell::version(), env!("CARGO_PKG_VERSION"));
  assert!(Shell::performance_now() > 0.0);
  Shell::collect_garbage(&isolate);

  // 验证真实文件读写与脚本执行链
  let test_file = temp_dir().join("test_d8_script.js");
  let test_path = test_file.to_string_lossy().to_string();
  let script_content = b"let a = 10; let b = 20; a * b;";

  Shell::write_file(&test_path, script_content)?;
  assert!(Shell::file_exists(&test_path));

  let read_back = Shell::read_file(&test_path)?;
  assert_eq!(read_back, "let a = 10; let b = 20; a * b;");

  let buffer = Shell::read_buffer(&test_path)?;
  assert_eq!(buffer, script_content);

  let file_result = shell.execute_file(&isolate, &test_path)?;
  assert_eq!(file_result, JsValue::Number(200.0));

  let _ = remove_file(&test_file);
  assert!(!Shell::file_exists(&test_path));

  OK
}

/// 11. ECMAScript ToPrimitive 规范隐式转换专项
#[test]
fn test_ecmascript_toprimitive_and_coercion() -> Void {
  let test_code = r#"
    // 1. 数组隐式转换为字符串拼接
    assert.sameValue([1, 2] + "3", "1,23");
    assert.sameValue([1, 2] + [3, 4], "1,23,4");
    assert.sameValue("hello " + [1, 2], "hello 1,2");
    assert.sameValue([] + [], "");

    // 2. 复合赋值加法中的字符串强制类型提升
    var acc = 1;
    acc += "2";
    assert.sameValue(acc, "12");

    var s = "count: ";
    s += [1, 2, 3];
    assert.sameValue(s, "count: 1,2,3");

    // 3. 关系比较中的 ToPrimitive 提示转换
    assert.sameValue([10] < "9", true); // 字符串字典序 "10" < "9" 为 true
    assert.sameValue([10] < 9, false);  // 数值比较 10 < 9 为 false
    assert.sameValue(false == [], true);
    assert.sameValue(true == [1], true);
  "#;
  run_with_harness(test_code)?;
  OK
}
