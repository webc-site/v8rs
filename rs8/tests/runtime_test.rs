use aok::{OK, Void};
use rs8::{JsValue, run_js};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_js_arithmetic_and_bitwise() -> Void {
  let res = run_js(
    r#"
    var a = 10;
    var b = 20;
    var c = ((a * 3 + b) ^ 0x55) & 0xFF;
    c;
  "#,
  )?;
  assert_eq!(res, JsValue::Number(((10 * 3 + 20) ^ 0x55) as f64));
  OK
}

#[test]
fn test_js_closure_and_lexical_scope() -> Void {
  let res = run_js(
    r#"
    function makeCounter(init) {
      var count = init;
      return function() {
        count = count + 1;
        return count;
      };
    }
    var counter = makeCounter(100);
    counter();
    counter();
    var finalVal = counter();
    finalVal;
  "#,
  )?;
  assert_eq!(res, JsValue::Number(103.0));
  OK
}

#[test]
fn test_js_fibonacci_recursion() -> Void {
  let res = run_js(
    r#"
    function fib(n) {
      if (n <= 1) return n;
      return fib(n - 1) + fib(n - 2);
    }
    fib(10);
  "#,
  )?;
  assert_eq!(res, JsValue::Number(55.0));
  OK
}

#[test]
fn test_js_array_and_object_fast_index() -> Void {
  let res = run_js(
    r#"
    var arr = [10, 20, 30];
    arr.push(40);
    var obj = { x: 1, y: 2 };
    obj.x = 100;
    var sum = arr[0] + arr[1] + arr[2] + arr[3] + obj.x + obj.y;
    sum;
  "#,
  )?;
  assert_eq!(res, JsValue::Number(100.0 + 102.0));
  OK
}

#[test]
fn test_js_runtime_error_propagation() -> Void {
  let res = run_js("undefinedVariable + 1;");
  assert!(res.is_err(), "未定义变量必须如实返回运行时错误");
  OK
}

#[test]
fn test_js_array_builtin_and_edge_indices() -> Void {
  let res = run_js(
    r#"
    var arr = [1, 2, 3];
    arr.push(4, 5);
    var idx = arr.indexOf(3);
    var joined = arr.join("-");
    var str = [undefined, null, 1].toString();
    var nonInt = arr[1.5];
    var nanVal = arr[NaN];
    joined + "|" + str + "|" + idx + "|" + nonInt + "|" + nanVal;
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String("1-2-3-4-5|,,1|2|undefined|undefined".into())
  );
  OK
}

#[test]
fn test_js_hoisting_functions_and_scope() -> Void {
  let res = run_js(
    r#"
    function outer() {
      var a = 1;
      function getA() { return a; }
      function setA(v) { a = v; }
      setA(42);
      return getA();
    }
    outer();
  "#,
  )?;
  assert_eq!(res, JsValue::Number(42.0));
  OK
}

#[test]
fn test_js_array_mutating_builtins_and_side_effects() -> Void {
  let res = run_js(
    r#"
    var arr = [10, 20, 30, 40, 50];
    var pushRet = arr.push(60);
    var effect = 0;
    var popVal = arr.pop(effect = 1);
    var shiftVal = arr.shift();
    var sliced = arr.slice(1, 3);
    var spliced = arr.splice(1, 2, 99, 100);
    pushRet + "|" + effect + "|" + popVal + "|" + shiftVal + "|" + sliced.join(",") + "|" + spliced.join(",") + "|" + arr.join(",");
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String("6|1|60|10|30,40|30,40|20,99,100,50".into())
  );
  OK
}
