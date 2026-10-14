use aok::{OK, Void};
use rs8::{JsValue, check_api_static_root, create_d8_isolate, init_d8_platform, run_js};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 1. 作用域遮蔽与变量就地更新
#[test]
fn test_variable_shadowing_and_mutation() -> Void {
  let res = run_js(
    r#"
    var x = 10;
    function outer() {
      var x = 20;
      function inner() {
        x = x + 5;
        return x;
      }
      return inner() + x;
    }
    outer() + x;
  "#,
  )?;
  assert_eq!(res, JsValue::Number(25.0 + 25.0 + 10.0));
  OK
}

/// 2. 控制流: 嵌套循环与 break / continue 边界
#[test]
fn test_loop_break_continue_semantics() -> Void {
  let res = run_js(
    r#"
    var sum = 0;
    for (var i = 0; i < 10; i++) {
      if (i % 2 === 0) continue;
      if (i > 7) break;
      for (var j = 0; j < 3; j++) {
        sum = sum + i * 10 + j;
      }
    }
    sum;
  "#,
  )?;
  // i=1 (j=0,1,2 => 10,11,12 => 33)
  // i=3 (j=0,1,2 => 30,31,32 => 93)
  // i=5 (j=0,1,2 => 50,51,52 => 153)
  // i=7 (j=0,1,2 => 70,71,72 => 213)
  // sum = 33 + 93 + 153 + 213 = 492
  assert_eq!(res, JsValue::Number(492.0));
  OK
}

/// 3. 高阶函数与仿数组闭包流水线
#[test]
fn test_higher_order_functions_and_pipeline() -> Void {
  let res = run_js(
    r#"
    function map(arr, fn) {
      var res = [];
      for (var i = 0; i < arr.length; i++) {
        res.push(fn(arr[i], i));
      }
      return res;
    }

    function reduce(arr, fn, initial) {
      var acc = initial;
      for (var i = 0; i < arr.length; i++) {
        acc = fn(acc, arr[i]);
      }
      return acc;
    }

    var numbers = [1, 2, 3, 4, 5];
    var squared = map(numbers, function(x) { return x * x; });
    var total = reduce(squared, function(acc, x) { return acc + x; }, 0);
    total;
  "#,
  )?;
  assert_eq!(res, JsValue::Number(1.0 + 4.0 + 9.0 + 16.0 + 25.0));
  OK
}

/// 4. 进制与位操作复合逻辑
#[test]
fn test_bitwise_radix_edge_cases() -> Void {
  let res = run_js(
    r#"
    var hex = 0xAA;
    var oct = 0o12;
    var bin = 0b1010;
    var combined = (hex ^ oct) | bin;
    combined;
  "#,
  )?;
  let expected = ((0xAA ^ 0o12) | 0b1010) as f64;
  assert_eq!(res, JsValue::Number(expected));
  OK
}

/// 5. 对象动态属性覆写与嵌套查找
#[test]
fn test_dynamic_object_properties() -> Void {
  let res = run_js(
    r#"
    var person = {
      name: "alice",
      age: 28,
      address: {
        city: "Hangzhou",
        code: 310000
      }
    };
    person.age = person.age + 1;
    person.address.code = person.address.code + 1;
    person.age + person.address.code;
  "#,
  )?;
  assert_eq!(res, JsValue::Number(29.0 + 310001.0));
  OK
}

/// 6. D8 平台与 API 静态根节点打通
#[test]
fn test_d8_platform_and_roots() -> Void {
  let platform = init_d8_platform();
  assert!(platform.monotonically_increasing_time() >= 0.0);
  assert!(check_api_static_root("UndefinedValue"));
  let isolate = create_d8_isolate();
  assert!(!isolate.has_pending_exception());
  OK
}

/// 7. ECMAScript 原型链继承与 new 构造器实例化
#[test]
fn test_prototype_chain_and_new() -> Void {
  let res = run_js(
    r#"
    function Point(x, y) {
      this.x = x;
      this.y = y;
    }
    Point.prototype.distSquared = function() {
      return this.x * this.x + this.y * this.y;
    };
    var pt = new Point(3, 4);
    pt.distSquared();
  "#,
  )?;
  assert_eq!(res, JsValue::Number(25.0));
  OK
}

/// 8. 反射调用 (call/apply)、arguments 隐式列表与 Date 时间计算
#[test]
fn test_call_apply_arguments_and_date() -> Void {
  let res = run_js(
    r#"
    function sum() {
      var s = 0;
      for (var i = 0; i < arguments.length; i++) {
        s = s + arguments[i];
      }
      return s;
    }

    function delegate(a, b, c) {
      return sum.apply(null, arguments);
    }

    function Person(name) {
      this.name = name;
    }
    function Employee(name, salary) {
      Person.call(this, name);
      this.salary = salary;
    }

    var emp = new Employee("Alice", 1000);
    var d1 = new Date();
    var d2 = new Date();
    var delta = d2 - d1;

    delegate(10, 20, 30) + emp.salary + (delta >= 0 ? 1 : 0);
  "#,
  )?;
  assert_eq!(res, JsValue::Number(60.0 + 1000.0 + 1.0));
  OK
}

/// 9. 异常处理 (try / catch / finally) 与 throw 语义
#[test]
fn test_try_catch_finally_exception_handling() -> Void {
  let res = run_js(
    r#"
    var val = 0;
    try {
      val = val + 1;
      throw "boom";
      val = val + 100;
    } catch (e) {
      val = val + 10;
    } finally {
      val = val + 5;
    }
    val;
  "#,
  )?;
  assert_eq!(res, JsValue::Number(16.0));
  OK
}

/// 10. typeof 运算符与含下划线分隔符的数值字面量解析
#[test]
fn test_typeof_and_numeric_separators() -> Void {
  let res = run_js(
    r#"
    var hex = 0x12_34;
    var bin = 0b10_10;
    var oct = 0o7_7;
    var dec = 1_000_000;
    var sum = hex + bin + oct + dec;
    var t_num = typeof sum;
    var t_str = typeof "hello";
    var t_bool = typeof true;
    var t_undef = typeof undefined;
    var t_obj = typeof null;
    var t_fn = typeof function() {};
    t_num === "number" && t_str === "string" && t_bool === "boolean" && t_undef === "undefined" && t_obj === "object" && t_fn === "function" && sum === (0x1234 + 0b1010 + 0o77 + 1000000);
  "#,
  )?;
  assert_eq!(res, JsValue::Boolean(true));
  OK
}

/// 11. 内置数组与字符串切片与高阶方法
#[test]
fn test_builtin_string_array_methods() -> Void {
  let res = run_js(
    r#"
    var arr = [10, 20, 30, 40];
    var sliced = arr.slice(1, 3);
    var str = "JavaScript";
    var sub = str.slice(0, 4);
    var joined = sliced.join("-");
    joined + ":" + sub;
  "#,
  )?;
  assert_eq!(res, JsValue::String("20-30:Java".to_string()));
  OK
}
