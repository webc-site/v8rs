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
  assert_eq!(res, JsValue::String("20-30:Java".into()));
  OK
}

/// 12. 闭包按调用各自捕获环境, 实例身份互不相同
#[test]
fn test_closure_instances_capture_own_environment() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    function mk(x) { return function() { return x + 1; }; }
    var a = mk(1);
    var b = mk(2);
    a() + "/" + b() + "/" + (a === b) + "/" + (a === a);
  "#,
  )?;
  assert_eq!(res, JsValue::String("2/3/false/true".into()));
  OK
}

/// 13. 语句完成值: if 分支的末条语句值即语句值, 空分支为 undefined
#[test]
fn test_if_statement_completion_value() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    var a = eval("1; if (false) { } else { }");
    var b = eval("2; if (false) { } else { 3; }");
    var c = eval("4; if (false) { 5; } else { }");
    var d = eval("6; if (true) { 7; } else { 8; }");
    (a === undefined) + "/" + b + "/" + (c === undefined) + "/" + d;
  "#,
  )?;
  assert_eq!(res, JsValue::String("true/3/true/7".into()));
  OK
}

/// 14. return 穿透 try-finally: 收尾语句必须执行, finally 内的 return 覆盖先前返回值
#[test]
fn test_return_passes_through_finally() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    var log = "";
    function innerFirst() { try { try { return 1; } finally { log += "i"; } } finally { log += "o"; } }
    function finallyWins() { try { try { return 1; } finally { return 2; } } finally { log += "w"; } }
    function finallyThrows() { try { return 1; } finally { throw 9; } }
    var caught = 0;
    try { finallyThrows(); } catch (e) { caught = e; }
    innerFirst() + "/" + finallyWins() + "/" + caught + "/" + log;
  "#,
  )?;
  assert_eq!(res, JsValue::String("1/2/9/iow".into()));
  OK
}

/// 15. 数组的属性面: 具名键不再被静默丢弃, 与元素槽各自独立 (对标原版 JSArray = JSObject + 元素)
#[test]
fn test_array_named_property_slot() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    var a = [1, 2, 3];
    a.foo = 42;
    a[1] = 9;
    a.length = 2;
    var seen = "";
    for (var k in a) { seen += k + ":" + a[k] + " "; }
    var b = [7];
    b.tag = "bee";
    b.length = 0;
    ("foo" in a) + "/" + a.foo + "/" + a.length + "/" + a[0] + "/" + a[1] + "/"
      + b.length + "/" + b.tag + "/" + seen;
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String("true/42/2/1/9/0/bee/0:1 1:9 foo:42 ".into())
  );
  OK
}

/// 16. break/continue 穿出 try-finally: 每层收尾副本都要执行一次, 且不得串进相邻副本
#[test]
fn test_break_continue_crosses_finally() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    var out = [];
    function p(s) { out.push(s); }
    function contThree() {
      var i = 0;
      while (i < 3) {
        i++;
        try { try { try { p("C" + i); continue; } finally { p("A" + i); } } finally { p("B" + i); } } finally { p("D" + i); }
        p("NEVER" + i);
      }
      p("done" + i);
    }
    function breakTwo() {
      for (var j = 0; j < 2; j++) {
        try { try { p("k" + j); break; } finally { p("f1" + j); } } finally { p("f2" + j); }
        p("NOPE");
      }
      p("b-end");
    }
    function breakInCatch() {
      for (var k = 0; k < 3; k++) {
        try { try { throw 1; } catch (e) { p("c" + k); break; } finally { p("cf" + k); } } finally { p("cg" + k); }
        p("tail" + k);
      }
      p("cb-end");
    }
    // 同一条 for 上, finally 体的 break 与 try 体的 return 是两个不同出口
    function returnAndBreakSameFinally() {
      for (var m = 0; m < 5; m++) {
        try { if (m == 2) { p("ret"); return m * 10; } } finally { if (m == 1) { p("finBrk"); break; } p("fin" + m); }
      }
      p("afterLoop");
      return -1;
    }
    contThree(); breakTwo(); breakInCatch();
    p("mix=" + returnAndBreakSameFinally());
    out.join("|");
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "C1|A1|B1|D1|C2|A2|B2|D2|C3|A3|B3|D3|done3|k0|f10|f20|b-end|c0|cf0|cg0|cb-end|fin0|finBrk|afterLoop|mix=-1".into()
    )
  );
  OK
}

/// 17. 宽松相等: 两侧同为对象时退化为身份比较, 不许各自 ToPrimitive 成同一串 "[object Object]"
#[test]
fn test_loose_equality_object_identity() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    var o1 = {}, o2 = {};
    var a1 = [1], a2 = [1];
    var boxed = new String("ab");
    function eq(o) { return o == "ab"; }
    (o1 == o2) + "/" + (o1 == o1) + "/" + (a1 == a2) + "/" + (a1 == a1) + "/"
      + (o1 == null) + "/" + (o1 == undefined) + "/" + (boxed == "ab") + "/"
      + (boxed == new String("ab")) + "/" + (eq(o1) ? "y" : "n") + "/" + ([] == "0");
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String("false/true/false/true/false/false/true/false/n/false".into())
  );
  OK
}

/// 18. `Object.keys` 对数组要吐索引键 (宿主内建原先只认 Object, 数组一律空数组)
#[test]
fn test_object_keys_of_array_lists_indices() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    var a = [10, 20, 30];
    var empty = [];
    Object.keys(a).join(",") + "|" + a.map(function (v, i) { return i + ":" + v; }).join(" ")
      + "|" + Object.keys(empty).length;
  "#,
  )?;
  assert_eq!(res, JsValue::String("0,1,2|0:10 1:20 2:30|0".into()));
  OK
}

/// 19. Number → String 全族必须走 v8-numbers 的 conversions.cc 移植
///
/// 引擎侧手写 `format!("{:.f}")` 三处偏: 平局是 round-half-even 而 V8 是 half-away-from-zero、
/// toPrecision 被写成"小数位"而非"有效数字"、ToString 不切科学计数法
#[test]
fn test_number_to_string_family_matches_v8() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    var out = [];
    function t(f) { try { out.push(f()); } catch (e) { out.push(e.name); } }
    t(function () { return (2.5).toFixed(0) + "," + (-2.5).toFixed(0); });
    t(function () { return (1.005).toFixed(2) + "," + (0.145).toFixed(2); });
    t(function () { return (0).toFixed(2) + "," + (-0.0001).toFixed(2); });
    t(function () { return (2.5).toPrecision(1) + "," + (100).toPrecision(5); });
    t(function () { return (1234).toPrecision(2) + "," + (1e20).toPrecision(3); });
    t(function () {
      return (0.000001).toPrecision(2) + "," + (0.0000001).toPrecision(2) + ","
        + (99999999999999999999).toPrecision(2);
    });
    t(function () { return (1e21).toString() + "," + (1e-7).toString() + "," + (-0).toString(); });
    t(function () { return [1e21, 1e-7, 2.5, -0].join(","); });
    t(function () { return (3.5).toString(2) + "," + (255).toString(16) + "," + (-255).toString(16); });
    t(function () { return (5).toString(0); });
    t(function () { return (5).toString(null); });
    t(function () { return (1).toPrecision(0); });
    t(function () { return (1).toFixed(101); });
    t(function () { return (1).toFixed(Infinity); });
    out.join("|");
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "3,-3|1.00,0.14|0.00,-0.00|3,100.00|1.2e+3,1.00e+20|0.0000010,1.0e-7,1.0e+20|1e+21,1e-7,0|1e+21,1e-7,2.5,0|11.1,ff,-ff|RangeError|RangeError|RangeError|RangeError|RangeError".into()
    )
  );
  OK
}

/// 20. 具名属性读的写入可见性
///
/// 字节码侧曾用"按调用点缓存属性值"的 IC, 于是 `o["x"]=5` / `delete o.x`
/// 从另一条字节码路径改了对象后, `o.x` 仍读到缓存的旧值。缓存删掉后本用例成为回归门禁
/// (读到的必须是当前值, 不是首次读的值)
#[test]
fn test_named_property_read_sees_cross_path_writes() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    var o = {x: 1};
    function read() { return o.x; }
    var seq = [read()];
    o["x"] = 5;
    seq.push(read());
    seq.push(o.x);
    delete o.x;
    seq.push("del:" + read());
    o.y = 7;
    seq.push(String(o.y));
    var f = 0;
    function bump() { f = f + 1; return f; }
    seq.push(bump() + "," + bump());
    seq.join("|");
  "#,
  )?;
  assert_eq!(res, JsValue::String("1|5|5|del:undefined|7|1,2".into()));
  OK
}

/// 21. 数字 → 串只剩一条路: 大数/小数经 JSON.stringify 与数字对象键都必须走格式层
///
/// JSON 序列化曾手写 `f64::to_string` + `as i64`: 1e21 印成 22 位长串、超出 2^53 的整值被
/// 截成 i64::MAX; 数字对象键同源, 键写成 "1000000000000000000000" 就读不到 `o[1e21]`
#[test]
fn test_number_key_and_json_use_format_layer() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    var big = 1e21, tiny = 1e-7;
    var o = {1e21: "hit"};
    var p = {1.5: "half"};
    JSON.stringify([big, tiny, 2.5, -0]) + "|" + o[big] + "|" + Object.keys(p).join(",");
  "#,
  )?;
  assert_eq!(res, JsValue::String("[1e+21,1e-7,2.5,0]|hit|1.5".into()));
  OK
}

/// 22. 数组越界下标不得溢出 panic: 超出可表达范围的数字键退回具名属性槽
///
/// 下标会参与 `resize(idx + 1)`, 而 f64 → usize 的饱和转换把 1e21 折成 `usize::MAX`, 加一即
/// 溢出 panic (曾实测崩在 SetKeyedProperty)。V8 对非数组下标 (`> 2^32-2`) 本就当字符串键,
/// 这里按同一序: 越界键写读对称, 且不影响 `length`
#[test]
fn test_out_of_range_array_index_is_named_property() -> Void {
  let res = rs8::run_strict_bytecode(
    r#"
    var a = [];
    a[1e21] = "huge";
    a["18446744073709551615"] = "max";
    a[3] = 7;
    a[1e21] + "|" + a["18446744073709551615"] + "|" + a.length + "|" + a[3] + "|" + a[-1];
  "#,
  )?;
  assert_eq!(res, JsValue::String("huge|max|4|7|undefined".into()));
  OK
}

/// 双链路差分断言: 同一段源码在 AST 与字节码引擎下必须给出同一个完成值
///
/// 两条链路各自实现时最容易各抄一份语义, 结果一侧对一侧错 yet 单跑一侧的测试全绿
fn both_chains(src: &str) -> Result<JsValue, rs8::Error> {
  let ast = rs8::run_ast(src);
  let bc = rs8::run_strict_bytecode(src);
  match (ast, bc) {
    (Ok(a), Ok(b)) => {
      assert_eq!(a, b, "双链路结果分裂");
      Ok(a)
    }
    (Err(e), _) | (_, Err(e)) => Err(e),
  }
}

/// 23. 对象字面量 accessor: get/set 触发、this 绑定、伪键不外泄、简写与尾逗号
///
/// 此前 `get p(){}` 在两条链路都是硬语法错误。断言覆盖: getter 读、setter 写、`+=` 复合赋值
/// 走读→算→写回、只有 getter 时赋值被忽略、只有 setter 时读为 undefined、`get`/`set` 作普通
/// 数据键不受影响、for-in 与 Object.keys 不泄漏 `@@get:` 伪键
#[test]
fn test_object_literal_accessors() -> Void {
  let res = both_chains(
    r#"
    var o = { _v: 1, get p() { return this._v * 2; }, set p(x) { this._v = x; } };
    var seq = [];
    seq.push("read=" + o.p);
    o.p = 5;
    seq.push("afterSet=" + o.p);
    seq.push("keys=" + Object.keys(o).sort().join(","));
    seq.push("in=" + ("p" in o) + "|" + ("q" in o));
    var sp = { get: 1, set: 2 };
    seq.push("word=" + sp.get + "," + sp.set);
    var g = 0;
    var m = { get inc() { return ++g; } };
    seq.push("side=" + m.inc + "," + m.inc);
    var ro = { get only() { return 1; } };
    ro.only = 99;
    seq.push("getterOnly=" + ro.only);
    var so = { set only(v) { so._v = v; } };
    so.only = 3;
    seq.push("setterOnly=" + so.only + "|" + so._v);
    var t = { _n: 1, get n() { return this._n + 1; }, set n(v) { this._n = v * 2; } };
    t.n = 5;
    seq.push("thisSet=" + t.n + "|" + t._n);
    var a = 1;
    var sh = { a, f() { return this.a + 1; }, };
    seq.push("shorthand=" + sh.a + "," + sh.f());
    var out = [];
    for (var k in t) { out.push(k); }
    seq.push("forIn=" + out.sort().join(","));
    var nest = { inner: { get v() { return "IV"; } } };
    seq.push("nest=" + nest.inner.v);
    seq.push("compound=" + (t.n += 1) + "|" + t._n);
    seq.join("|");
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "read=2|afterSet=10|keys=_v,p|in=true|false|word=1,2|side=1,2|getterOnly=1|\
       setterOnly=undefined|3|thisSet=11|10|shorthand=1,2|forIn=_n,n|nest=IV|compound=12|24"
        .into()
    )
  );
  OK
}

/// 24. 数组内建方法表补齐 sort/includes/concat, 语义对齐 node
///
/// AST 链路此前只有 9 个方法且与 `in` 判定各列一份名单, `Object.keys(o).sort()` 在回退路径上
/// 直接 TypeError。sort 的比较器是用户函数, 只能由持有执行态的一侧就地回调 (无比较器按 ToString
/// 码点序、undefined 恒排末尾、稳定序、比较器抛错向外传播)
#[test]
fn test_array_builtin_sort_includes_concat() -> Void {
  let res = both_chains(
    r#"
    var seq = [];
    seq.push("default=" + [3, 1, 22, 4].sort().join(","));
    seq.push("cmp=" + [3, 1, 22, 4].sort(function (a, b) { return a - b; }).join(","));
    seq.push("desc=" + [3, 1, 22, 4].sort(function (a, b) { return b - a; }).join(","));
    seq.push("undefCmp=" + [2, 1, 3].sort(undefined).join(","));
    var nullMsg = "none";
    try { [2, 1, 3].sort(null); } catch (e) { nullMsg = "throw"; }
    seq.push("nullCmp=" + nullMsg);
    var inarr = [2, 1];
    seq.push("inPlace=" + (inarr.sort() === inarr) + "|" + inarr.join(","));
    seq.push("str=" + ["b", "a", "C"].sort().join(","));
    seq.push("undefLast=" + [undefined, 2, undefined, 1].sort().join("|"));
    seq.push("stable=" + [10, 1, 2].sort(function () { return 0; }).join(","));
    seq.push("keysSorted=" + Object.keys({ b: 1, a: 2 }).sort().join(","));
    seq.push("includes=" + [1, 2, 3].includes(2) + "|" + [1, 2, 3].includes(9) + "|" + [NaN].includes(NaN));
    var ref = {};
    seq.push("includesRef=" + [ref].includes(ref));
    seq.push("concat=" + [1, 2].concat(3, [4, 5]).join(","));
    var orig = [1];
    orig.concat(2);
    seq.push("concatNoMut=" + orig.join(","));
    seq.push("in=" + ("sort" in [1]) + "|" + ("includes" in [1]) + "|" + ("concat" in [1]) + "|" + ("zz" in [1]));
    var thrown = "none";
    try { [3, 1, 2].sort(function (a, b) { if (a > b) { throw new Error("boom"); } return a - b; }); } catch (e) { thrown = e.message; }
    seq.push("cmpThrow=" + thrown);
    seq.join("|");
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "default=1,22,3,4|cmp=1,3,4,22|desc=22,4,3,1|undefCmp=1,2,3|nullCmp=throw|\
       inPlace=true|1,2|str=C,a,b|undefLast=1|2|||stable=10,1,2|keysSorted=a,b|\
       includes=true|false|true|includesRef=true|concat=1,2,3,4,5|concatNoMut=1|\
       in=true|true|true|false|cmpThrow=boom"
        .into()
    )
  );
  OK
}

/// 25. 计算键 accessor 走字节码链, 不再让整脚本回退
///
/// 此前桥接层没有 "计算键 + `@@get:` 前缀" 的属性形态, 一个 `get [k](){}` 就让 `total=1` 的
/// 整程序回退粒度把整个脚本交回 AST 求值器。现用 `"@@get:" + k` 这条 Add 表达式按计算键落属性,
/// 纯字节码模式必须自己跑通 (`--print-bytecode-coverage` 100%), 且两链同值
///
/// `json=` 一项钉的是**已知偏差**: node 触发 getter 得 `{"ck":7}`, 两链一致地给 `{}` (JSON 走宿主
/// native, 拿不到执行态, 见 todo 12.12)。改动让 JSON 能触发 getter 时, 这条断言要一起改
#[test]
fn test_computed_key_accessor_lowers_to_bytecode() -> Void {
  let src = r##"
    var k = "ck";
    var v = 0;
    var c = { get [k]() { return 7; }, set [k](x) { v = x; } };
    c.ck = 8;
    var n = 5;
    var o = { get [n]() { return "num"; } };
    var out = [];
    out.push(c.ck + "|set:" + v + "|" + Object.keys(c).sort().join(","));
    out.push("num=" + o[5] + "|" + o["5"]);
    out.push("json=" + JSON.stringify(c));
    out.join("#");
  "##;
  let res = both_chains(src)?;
  assert_eq!(
    res,
    JsValue::String("7|set:8|ck#num=num|num#json={}".into())
  );
  OK
}

/// 26. Date 实例必须带 getTime/valueOf
///
/// 实例只有 `__timestamp__` 数值槽时 `d.getTime()` 恒为 undefined, Octane 的
/// `BenchmarkSuite.GetTime` 静默退化成一轮 —— Score 照样打印, 但基准数字全失真。这类"不报错
/// 的缺失内建"必须由门禁钉住
#[test]
fn test_date_instance_methods() -> Void {
  let res = both_chains(
    r#"
    var d = new Date();
    var t = d.getTime();
    var later = new Date().getTime();
    (typeof t) + "|" + (t > 1e12) + "|" + (d.valueOf() === t) + "|" + (later >= t)
      + "|" + (typeof Date.now()) + "|" + ((later - t) >= 0);
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String("number|true|true|true|number|true".into())
  );
  OK
}

/// 27. 内部伪键与槽位键不得泄进 JSON; delete 计算键与读写键归一同源
///
/// 门禁本轮三处修复: (a) accessor 对象的 `@@accessor` 哨兵与 `@@get:`/`@@set:` 伪键曾原样
/// 进 `JSON.stringify` 输出; (b) AST 链路 `delete o[k]` 的数字键走 `as i64`、符号键走 Display,
/// 与写入点 `element_key` 不同源, 非整数键删不掉; (c) `delete o.q` 落在 accessor 上曾一侧
/// true 一侧 false 且删完 getter 照常工作 —— 两链现均连带回收伪键, 删后读 undefined, 对齐 node
///
/// (c) 必须用 get/set 成对的 `o3` 断言: 只有单侧 accessor 时, "删一条" 与 "删两条" 的短路实现
/// 结果相同, 残留的 setter 要等到删后再赋值才暴露 (`pairRead:7|v=1` 而非 `pairRead:undefined|v=7`)
#[test]
fn test_internal_keys_never_leak_and_delete_key_consistency() -> Void {
  let res = both_chains(
    r#"
    var seq = [];
    var o = { _v: 1, get p() { return this._v * 2; }, set p(x) { this._v = x; } };
    o.p = 4;
    seq.push("json=" + JSON.stringify(o));
    var n = {};
    n[1.5] = "x";
    n[2] = "y";
    delete n[1.5];
    delete n[2];
    seq.push("delNumKey=" + Object.keys(n).length + "|" + String(n[1.5]));
    var a = [];
    a.push(1);
    seq.push("arrJson=" + JSON.stringify(a));
    var o2 = { get q() { return 7; } };
    seq.push("del=" + delete o2.q + "|read:" + o2.q);
    var o3 = { _v: 1, get q() { return "G"; }, set q(x) { o3._v = x; } };
    seq.push("delPair=" + delete o3.q);
    o3.q = 7;
    seq.push("pairRead:" + String(o3.q) + "|v=" + o3._v + "|keys=" + Object.keys(o3).sort().join(","));
    var t = { get p() { return 1; }, p: 2 };
    seq.push("dupKeys=" + Object.keys(t).sort().join(","));
    seq.push("numProto=" + (1).toFixed(2) + "," + (2.5).toString(2) + "," + (1e21).toString());
    seq.join("|");
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "json={\"_v\":4}|delNumKey=0|undefined|arrJson=[1]|del=true|read:undefined|\
       delPair=true|pairRead:7|v=1|keys=_v,q|dupKeys=p|numProto=1.00,10.1,1e+21"
        .into()
    )
  );
  OK
}

/// 28. 数组迭代类内建 + 内建接收者的原型回落
///
/// 覆盖本轮四处修复: (a) AST 链路 `[1,2].map` 一类方法此前全量 undefined (数组实例不接
/// `Array.prototype`), 回退一个脚本就换一个 TypeError 悬崖; (b) 预置脚本的循环条件写 `i <
/// this.length`, 回调里往同一个数组 push 会把 `map` 撑成死循环 (规范是 GetLength 一次);
/// (c) `reduce(fn, undefined)` 与 `reduce(fn)` 是两回事, 原先按 `acc === undefined` 判定,
/// 显式传 undefined 也被当成缺初值; (d) 字节码链 `"x" in "abc"` 静默给 false, 规范是 TypeError
///
/// 末段 `hole=`/`filled0=`/`in7=` 钉住一处**有意偏离 node** 的取舍: 数组的规范下标不回溯原型 (`a[7]` 给
/// undefined, node 给 `proto7`), 字符串键照旧回溯 (`tag` 给 `protoTag`), 且元素槽的 own 值永远压过
/// 原型 (`[9][0]` 是 9 而不是 `p0`)。数组实例以稠密 Vec 存储, 若整数下标也走原型链, 每次 `a[i]`
/// 都要先判"是不是空洞", 热路径上的元素读就得起价; 而 `Array.prototype[n]` 赋值是极冷门写法。
/// `in7=` 是同一规则的另一半: 读侧筛除而 `in` 侧回溯原型会让引擎**自相矛盾** —— `(7 in a)` 真而
/// `a[7]` undefined, `if (i in a) use a[i]` 直接踩空, 故两臂必须同口径。
/// 此处记录的是决策而非疏漏, 若要改为严格实现, 改的是这几条断言而不是修 bug
#[test]
fn test_array_iterator_methods_and_proto_fallback() -> Void {
  let res = both_chains(
    r##"
    var a = [1, 2, 3];
    var out = [];
    out.push(a.map(function (x, i) { return x + ":" + i; }).join("|"));
    out.push(a.filter(function (x) { return x % 2 === 1; }).join(","));
    var sum = 0;
    a.forEach(function (x) { sum += x; });
    out.push("sum" + sum);
    out.push("r" + a.reduce(function (p, c) { return p + c; }));
    out.push("ri" + a.reduce(function (p, c) { return p + c; }, 10));
    out.push("ru" + a.reduce(function (p, c) { return String(p) + c; }, undefined));
    out.push("e" + a.every(function (x) { return x > 2; }) + "s" + a.some(function (x) { return x > 2; }));
    var mut = [1, 2];
    out.push("mut" + mut.map(function (x) { mut.push(0); return x; }).length);
    out.push("noCb" + (function () { try { a.map(); return "no"; } catch (e) { return "yes"; } })());
    out.push("emptyR" + (function () { try { [].reduce(function (p, c) { return c; }); return "no"; } catch (e) { return "yes"; } })());
    out.push("inMap" + ("map" in a) + "typeof" + (typeof a["map"]));
    String.prototype.byProto = function () { return "bp" + this; };
    out.push("strProto" + "x".byProto());
    out.push("protoIn" + (function () { try { return "x" in "abc" ? "yes" : "no"; } catch (e) { return "throw"; } })());
    Array.prototype[7] = "proto7";
    Array.prototype[0] = "p0";
    Array.prototype.tag = "protoTag";
    out.push("hole=" + a[7] + "|own0=" + a[0] + "|filled0=" + [9][0] + "|tag=" + a["tag"]);
    out.push("in7=" + ("7" in a) + "|in0=" + (0 in a) + "|in10=" + (10 in a) + "|inTag=" + ("tag" in a));
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "1:0|2:1|3:2#1,3#sum6#r6#ri16#ruundefined123#efalsestrue#mut2#noCbyes#emptyRyes#\
       inMaptruetypeoffunction#strProtobpx#protoInthrow#hole=undefined|own0=1|filled0=9|tag=protoTag#\
       in7=false|in0=true|in10=false|inTag=true"
        .into()
    )
  );
  OK
}

/// 29. 数组迭代每步的取元素时机: 长度一次 + 元素现取 + 回调前定值
///
/// 本轮抓出两处各自独立成立的退化, 只有"回调里改同一个数组"的形状才暴露:
/// (a) 字节码链曾整表快照 (`read_elems(arr).clone()`), 回调里 `pop` 后仍交出旧元素 ——
///     `b.map` 给 `1,2,3` 而 node 给 `1,2,<hole>`; 现改为"长度取一次 + 每步脱锁现取一个元素"
/// (b) AST 链预置脚本曾写 `fn(this[i], ...)` 再 `res.push(this[i])`, 回调里改写同下标会反映到
///     结果上 —— `e.filter(function (x) { e[0] = 99; return true; })` 给 `99,2` 而 node 给 `1,2`;
///     规范每步只 GetElement 一次, 该值同时作回调实参与收集值
///
/// 两链此前一条在 (a) 上错、一条在 (b) 上错, 单看任何一条都不会让 `both_chains` 变红
#[test]
fn test_array_iterator_element_read_timing() -> Void {
  let res = both_chains(
    r##"
    var out = [];
    var a = [1, 2, 3];
    out.push("cbNull=" + (function () { try { a.map(null); return "no"; } catch (e) { return "throw"; } })());
    out.push("cbNum=" + (function () { try { a.reduce(7); return "no"; } catch (e) { return "throw"; } })());
    var b = [1, 2, 3];
    out.push("pop=" + b.map(function (x, i) { if (i === 0) b.pop(); return x; }).join(","));
    var c = [1, 2, 3];
    out.push("popLen=" + c.map(function (x, i) { if (i === 0) c.pop(); return x; }).length);
    var n = 0;
    a.every(function (x) { n++; return x > 2; });
    out.push("everyStop=" + n);
    n = 0;
    a.some(function (x) { n++; return x > 1; });
    out.push("someStop=" + n);
    out.push("forEachU=" + String(a.forEach(function () { return 1; })));
    out.push("redEmpty=" + [].reduce(function (p, cc) { return p + cc; }, 42));
    out.push("red1=" + [5].reduce(function (p, cc) { return p + cc; }));
    var d = [1, 2];
    out.push("pushOut=" + d.map(function (x) { d.push(x); return x; }).join(","));
    var e = [1, 2];
    out.push("filterMut=" + e.filter(function (x) { e[0] = 99; return true; }).join(","));
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "cbNull=throw#cbNum=throw#pop=1,2,#popLen=3#everyStop=1#someStop=2#forEachU=undefined#\
       redEmpty=42#red1=5#pushOut=1,2#filterMut=1,2"
        .into()
    )
  );
  OK
}

/// 30. 对象字面量落表走 CreateDataProperty: 同名后定义整体覆盖先前定义, 且不打 setter
///
/// 原版靠 `kDefineNamedOwnPropertyInLiteral`/`kDefineKeyedOwnProperty` 两条字节码区分"字面量构建"
/// 与"普通赋值"; 本侧此前把字面量也发成 `SetNamedProperty`/`SetKeyedProperty`, 于是 accessor 伪键
/// 与同名数据键**并存** —— 读侧优先伪键, `{ get y(){ return 1; }, y: 2 }` 两链都读回 1 而 node 是
/// 2。两链当时错得一模一样, `both_chains` 根本抓不到, 只能靠与 node 逐字符对照 (本门禁的期望串
/// 即 node 输出)。计算键形如 `{ get [k](){...}, [k]: 2 }` 同理, 且只有运行期才知道键名, 覆盖必须
/// 发生在落表处而不是 lowering 处
#[test]
fn test_object_literal_uses_define_own_property_semantics() -> Void {
  let res = both_chains(
    r##"
    var k = "q";
    var hit = 0;
    var o1 = { set x(v) { hit = 1; }, x: 5 };
    var o2 = { get y() { return 1; }, y: 2 };
    var o3 = { z: 3, get z() { return 4; } };
    var o4 = { set w(v) { }, w: 7 };
    var o5 = { get [k]() { return 1; }, [k]: 2 };
    var o6 = { [k]: 2, get [k]() { return 1; } };
    var out = [];
    out.push("setterFired=" + hit + "|o1x=" + o1.x);
    out.push("o2y=" + o2.y);
    out.push("o3z=" + o3.z);
    out.push("o4keys=" + Object.keys(o4).sort().join(",") + "|w=" + o4.w);
    out.push("o5q=" + o5.q + "|keys=" + Object.keys(o5).sort().join(","));
    out.push("o6q=" + o6.q + "|keys=" + Object.keys(o6).sort().join(","));
    o2.y = 9;
    out.push("afterAssign=" + o2.y);
    o5.q = 8;
    out.push("ckAfterAssign=" + o5.q);
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "setterFired=0|o1x=5#o2y=2#o3z=4#o4keys=w|w=7#o5q=2|keys=q#o6q=1|keys=q#\
       afterAssign=9#ckAfterAssign=8"
        .into()
    )
  );
  OK
}

/// 31. 嵌套字面量的交接: 属性落表会作废累加器, 字面量末尾必须把对象本身取回来
///
/// 字节码链的非空字面量走 `CreateObjectLiteral` (属性数随 Flag8 交出, 执行期据此一次铺好表容量)
/// 再逐个 `Define{Named,Keyed}OwnProperty`, 后两者按原版 `kReadAndClobberAccumulator` 把累加器
/// **作废**。于是"内层字面量落完属性 → Ldar 取回对象 → 交给外层属性"这条交接一旦写错, 症状是
/// 嵌套形状静默变成 `undefined`, 而单层字面量全对 —— 门禁 30 那种平铺形状抓不到, 故单列一条
#[test]
fn test_nested_object_literal_handoff() -> Void {
  let res = both_chains(
    r##"
    var k = "ck";
    var out = [];
    var o = {a: {b: {c: 1}}, d: [1, {e: 2}], f: {get g() { return 3; }, h: {i: 4}}};
    out.push("nest=" + o.a.b.c + "|" + o.d[1].e + "|" + o.f.g + "|" + o.f.h.i);
    var arr = [{x: 1}, {x: {y: 2}}, {[k]: {z: 3}}];
    out.push("arrN=" + arr.length + "|" + arr[0].x + "|" + arr[1].x.y + "|" + arr[2].ck.z);
    function id(v) { return v; }
    out.push("arg=" + id({p: 1, q: {r: 2}}).q.r);
    var acc = {get s() { return 9; }, set s(v) { acc.v = v; }, t: {u: 5}};
    acc.s = 6;
    out.push("acc=" + acc.s + "," + acc.v + "," + acc.t.u);
    var dup = {n: 1, get n() { return 2; }, n: 3};
    out.push("dup=" + dup.n);
    var big = {};
    for (var i = 0; i < 3; i++) { big = {k0: i, k1: {k2: i * 2}, k3: i * 3, k4: i * 4, k5: i * 5}; }
    out.push("big=" + big.k1.k2 + "," + big.k5);
    out.push("empty=" + JSON.stringify({}) + "|" + JSON.stringify({a: {}}));
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "nest=1|2|3|4#arrN=3|1|2|3#arg=2#acc=9,6,5#dup=3#big=4,10#empty={}|{\"a\":{}}".into()
    )
  );
  OK
}

/// 32. Array.prototype 的 lastIndexOf/fill/unshift/find (十五·B #19)
///
/// 两链此前都缺这四个方法: `a.lastIndexOf(2)` 直接 TypeError。期望串为 node 实测逐字符对照:
/// lastIndexOf 用严格相等 (NaN 永不命中, 与 includes 的 SameValueZero 相区分), fromIndex 负值
/// 从尾部归一; fill 返回接收者本身且 start/end 走 ToIntegerOrInfinity; unshift 返回新长度;
/// find 交出命中元素本身并支持 thisArg (第二实参); 回调缺省/非可调用一律 TypeError
#[test]
fn test_array_lastindexof_fill_unshift_find() -> Void {
  let res = both_chains(
    r#"
    var out = [];
    var a = [1, 2, 3, 2];
    out.push(a.lastIndexOf(2));
    out.push(a.lastIndexOf(9));
    out.push(a.lastIndexOf(2, 1));
    out.push(a.lastIndexOf(2, -3));
    out.push(a.lastIndexOf(NaN));
    var b = [1, 2, 3];
    out.push(b.fill(0).join(","));
    out.push(String(b.fill(7) === b));
    var c = [1, 2, 3];
    out.push(c.fill(0, 1).join(","));
    var d = [1, 2, 3];
    out.push(d.fill(0, 1, 2).join(","));
    var e = [1, 2, 3];
    out.push(e.fill(9, -1).join(","));
    var f = [3, 1, 2];
    out.push(String(f.unshift(0)));
    out.push(f.join(","));
    var g = [];
    out.push(String(g.unshift()) + "|" + g.join(","));
    out.push([1, 5, 10].find(function (x) { return x > 4; }));
    out.push(String([1, 2].find(function (x) { return x > 9; })));
    var h = [1, 2, 3];
    out.push(String(h.find(function (x, i, arr) { return i === 1 && arr === h; })));
    out.push(String([undefined, 3].find(function (x) { return x === 3; })));
    out.push(String([1, 2, 3].find(function (x) { return x === this.want; }, { want: 3 })));
    try { [1].find(null); out.push("noThrow"); } catch (err) { out.push("throw"); }
    try { [1].find(); out.push("noThrowU"); } catch (err) { out.push("throwU"); }
    out.join("|");
  "#,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "3|-1|1|1|-1|0,0,0|true|1,0,0|1,0,3|1,2,9|4|0,3,1,2|0||5|undefined|2|3|3|throw|throwU".into()
    )
  );
  OK
}

/// 33. 内建方法表只是 Array.prototype 的替身: 用户覆盖/删除原型同名键后, 内建表必须让位 (#24)
///
/// 原版内建挂在 `Array.prototype` 上, 自有→原型→内建的回溯序天然给出正确语义; 本侧为了热路径
/// 把 22 个方法做成原生表直取, 于是 `Array.prototype.map = f` 后 `a.map(...)` 两链都还跑原生
/// (AST 链 `push`/`slice` 同病), `delete Array.prototype.map` 后 node 给 undefined 而 bc 给
/// 原生函数。修法: 写侧监视 —— 属性名命中内建表且接收者是 Array.prototype 时记名; 读/`in` 侧
/// 仅在内建表命中且该名字被监视时改为回溯原型。未监视路径零额外开销。输出收集一律 `out[n]=`
/// 索引赋值 —— `push` 本身就是被测覆盖对象, 用 `out.push` 会自指爆栈
#[test]
fn test_array_proto_override_wins_over_builtin_table() -> Void {
  let res = both_chains(
    r##"
    var out = []; var n = 0;
    function say(s) { out[n] = s; n++; }
    var a = [1, 2];
    say("before=" + a.map(function(x){ return x; }).join(","));
    Array.prototype.map = function() { return "custom"; };
    say("after=" + a.map(function(x){ return x; }));
    say("typeof=" + typeof a.map);
    Array.prototype.push = function(v) { say("myPush:" + v); return 99; };
    var ret = a.push(9);
    say("ret=" + ret + "|len=" + a.length);
    delete Array.prototype.map;
    say("afterDelete=" + typeof [].map + "|" + typeof a.map);
    say("inAfterDelete=" + ("map" in a) + "|inCall=" + (function(){ try { return a.map(function(x){return x;}); } catch (e) { return "TypeError"; } })());
    var k = "slice";
    Array.prototype[k] = function() { return "keyed-custom"; };
    say("keyed=" + a.slice(0) + "|" + a["slice"](0));
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "before=1,2#after=custom#typeof=function#myPush:9#ret=99|len=2#afterDelete=\
       undefined|undefined#inAfterDelete=false|inCall=TypeError#keyed=keyed-custom|keyed-custom"
        .into()
    )
  );
  OK
}

/// 34. `String.replace`/`match`/`exec` 的函数回调、$ 替换序列与 exec 结果的 `index`/`input` (#23)
///
/// node 语义钉桩: 回调实参 = 全匹配, 捕获组..., offset, 全串 (字符串模式无组); 替换串的
/// `$$`/`$&`/`` $` ``/`$'`/`$n` 序列对 regex 与字符串模式**同样**生效, 缺席组号原样保留;
/// 非全局 `match`(含字符串模式) 与 `exec` 返回带 `index`/`input` 的数组, 全局 `match` 无
/// `index`。宿主数组此前没有具名属性槽, `exec` 结果挂不上 `index` —— 一并补齐
#[test]
fn test_string_replace_match_callback_and_dollar_patterns() -> Void {
  let res = both_chains(
    r##"
    var out = []; var n = 0;
    function say(s) { out[n] = s; n = n + 1; }
    say("r1=" + "a1b2c".replace(/[0-9]/g, function(m, o, s) { return "[" + m + "@" + o + " of " + s + "]"; }));
    say("r2=" + "2024-01-15".replace(/(\d+)-(\d+)-(\d+)/, function(all, y, mo, d, off, s) {
      return mo + "/" + d + "/" + y + "|" + off + "|" + (s === "2024-01-15");
    }));
    say("r3=" + "xaxa".replace(/a/, function() { return "B"; }));
    say("r4=" + "aXa".replace("X", "[$&][$`][$']x[$$]y"));
    say("r5=" + "aXa".replace("X", function(m, o, s) { return "<" + m + "|" + o + "|" + s + ">"; }));
    say("r6=" + "a1b".replace(/([0-9])/, "[$&][`$`][$'][$1]"));
    say("r7=" + "a1b".replace(/[0-9]/, "[$1]"));
    say("r8=" + "x".replace(/()/g, function(m, g, off, s) { return "[" + m + "|" + g + "|" + off + "]"; }));
    var mm = /([0-9]+)/.exec("abc123def");
    say("e1=" + mm[0] + "|" + mm[1] + "|" + mm.index + "|" + mm.input);
    var ms = "abc".match("b");
    say("m1=" + ms[0] + "|" + ms.index + "|" + ms.input + "|" + ms.length);
    var mg = "abc".match(/(b)/);
    say("m2=" + mg[0] + "," + mg[1] + "|" + mg.index + "|" + mg.input);
    var mgs = "a1b2".match(/[a-z][0-9]/g);
    say("m3=" + mgs.join(",") + "|" + mgs.length + "|" + mgs.index);
    say("m4=" + ("abc".match(/zz/) === null) + "|" + ("abc".match("zz") === null));
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "r1=a[1@1 of a1b2c]b[2@3 of a1b2c]c#r2=01/15/2024|0|true#r3=xBxa\
       #r4=a[X][a][a]x[$]ya#r5=a<X|1|aXa>a#r6=a[1][`a][b][1]b#r7=a[$1]b\
       #r8=[||0]x[||1]#e1=123|123|3|abc123def#m1=b|1|abc|1#m2=b,b|1|abc\
       #m3=a1,b2|2|undefined#m4=true|true"
        .into()
    )
  );
  OK
}

/// 35. `$` 序列与回调的二轮边角: 回调返回值**不再展开**, 零宽全局, `$10` 两位优先试探,
///     替换后 `lastIndex` 复位, 全局 match 无 `index` (12.22 取证, 全 12 项 node-EXACT)
#[test]
fn test_dollar_substitution_edge_cases() -> Void {
  let res = both_chains(
    r##"
    var out = []; var n = 0;
    function say(s) { out[n] = s; n = n + 1; }
    say("t1=" + "abb".replace(/b/g, "x$&y"));
    say("t2=" + "abc".replace(/(?:)/g, "-"));
    say("t3=" + "a$b".replace("$", "x$&y"));
    say("t4=" + "aaa".replace(/a/g, function() { return "$&$1$$"; }));
    say("t5=" + "aaa".replace(/a/g, function() { return 42; }));
    say("t6=" + "abc".replace(/q*/g, "#"));
    var re = /b/g;
    say("t7=" + "abc".replace(re, "x") + "|" + re.lastIndex);
    say("t8=" + "hello".replace(/(l)/g, "[$1$1]"));
    say("t9=" + "x".replace(/x/, "$&$`$'").replace(/^$/, "e"));
    var m = "aaa".match(/a/g);
    say("t10=" + (m.index === undefined) + "|" + m.length);
    say("t11=" + "abc".replace(/(b)(c)?/, "$1$2$2"));
    say("t12=" + "a1".replace(/(a)(1)/, "$10$1$2"));
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "t1=axbyxby#t2=-a-b-c-#t3=ax$yb#t4=$&$1$$$&$1$$$&$1$$#t5=424242\
       #t6=#a#b#c##t7=axc|0#t8=he[ll][ll]o#t9=x#t10=true|3#t11=abcc#t12=a0a1"
        .into()
    )
  );
  OK
}

/// 36. 正则对象方法面 (12.23): node 自有面只有 exec/test, match/replace/split 是 undefined
///     —— bc 链 bridge 曾把三个实现挂成公开自有键泄漏; split 能力改走 `@@split` 哨兵后行为不变
///     (limit/空匹配/全局步长同 node), 布尔标志读值 (`ignoreCase/multiline/sticky`) 两链补齐,
///     AST 链 `in` 对正则对象不再 TypeError。`re.compile` (node 有, 废弃 API) 与
///     `"(1)".split` 捕获组交叉 (战役 H) 记待办不入钉
#[test]
fn test_regexp_object_method_surface() -> Void {
  let res = both_chains(
    r##"
    var out = []; var n = 0;
    function say(s) { out[n] = s; n = n + 1; }
    var re = /x/g;
    say("k1=" + (typeof re.exec) + "|" + (typeof re.test));
    say("k2=" + (typeof re.match) + "|" + (typeof re.replace) + "|" + (typeof re.split));
    say("k3=" + ("split" in re) + "|" + ("exec" in re) + "|" + ("source" in re));
    say("k4=" + JSON.stringify("a,b,c".split(/,/,2)));
    say("k5=" + JSON.stringify("aXb".split(/[ab]/)));
    say("k6=" + JSON.stringify("q".split(/(r)/)));
    say("k7=" + (typeof re.flags) + "|" + (typeof re.sticky) + "|" + (typeof re.ignoreCase));
    say("k8=" + re.flags + "|" + re.global + "|" + re.lastIndex);
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "k1=function|function#k2=undefined|undefined|undefined#k3=false|true|true\
       #k4=[\"a\",\"b\"]#k5=[\"\",\"X\",\"\"]#k6=[\"q\"]\
       #k7=string|boolean|boolean#k8=g|true|0"
        .into()
    )
  );
  OK
}

/// 37. 函数转字符串给源码文本 (12.24, #23 残项⑤): `toString()` / `String(f)` / `"" + f`
///     三路同一份 parser span 切片; 方法简写与匿名表达式同源; `String(decl) === decl.toString()`;
///     无源可切的内建退回 `[native code]` 摘要不外泄 Display 的 `[Function: x]`;
///     `in` 对函数沿原型链 (`"toString" in decl` 真)。`Function.prototype.toString` 恒等式
///     与 `f.constructor` 面物化归战役 C, 不入钉
#[test]
fn test_function_to_string_source_text() -> Void {
  let res = both_chains(
    r##"
    var out = []; var n = 0;
    function say(s) { out[n] = s; n = n + 1; }
    function decl(a, b) { return a + b; }
    var expr = function named( x ){ return x*x; };
    var anon = function(y) { return y+1; };
    var obj = { m(p) { return p; } };
    say("f1=" + decl.toString());
    say("f2=" + String(expr));
    say("f3=" + ("" + anon));
    say("f4=" + obj.m.toString());
    say("f6=" + typeof anon.toString);
    say("f7=" + (String(decl) === decl.toString()));
    say("f8=" + (function(){ return 1; }).toString());
    say("f9=" + ("toString" in decl) + "|" + (Object.keys(decl).join(",")));
    say("g1=" + String("abc") + "|" + String(123) + "|" + String(true) + "|" + String());
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "f1=function decl(a, b) { return a + b; }#f2=function named( x ){ return x*x; }\
       #f3=function(y) { return y+1; }#f4=m(p) { return p; }#f6=function#f7=true\
       #f8=function(){ return 1; }#f9=true|#g1=abc|123|true|"
        .into()
    )
  );
  OK
}

/// 38. 取整正确性 (12.25): `Math.round` 是 floor(x+0.5) 平局向 +∞ 而非 Rust 的远离零
///     (round(-2.5)=-2, round(-1.5)=-1), [-0.5,0) 归 **-0**; `parseInt` radix 缺省/0 时按前缀
///     自动判基 (`"0x10"`=16, `"-0x11"`=-17: 符号剥离后才判 0x 前缀), 显式 16 也要剥符号后的
///     0x 前缀。两链共用宿主内建, 一处修复两链同治
#[test]
fn test_math_round_and_parse_int_semantics() -> Void {
  let res = both_chains(
    r##"
    var out = []; var n = 0;
    function say(s) { out[n] = s; n = n + 1; }
    say("r1=" + Math.round(0.5) + "," + Math.round(1.5) + "," + Math.round(-0.5) + "," + Math.round(-1.5) + "," + Math.round(-2.5));
    say("r2=" + Math.round(2.4) + "," + Math.round(-2.6) + "," + (1/Math.round(-0.5)===-Infinity) + "," + (1/Math.round(-0.4)===-Infinity));
    say("i1=" + parseInt("0x10") + "," + parseInt("0X10",16) + "," + parseInt("  42  ") + "," + parseInt("42px") + "," + parseInt("-0x11"));
    say("i2=" + parseInt("08") + "," + parseInt("0o10") + "," + parseInt("10",2) + "," + parseInt("zz",36) + "," + parseInt("") + "," + parseInt("3.9"));
    say("i3=" + parseInt(null) + "," + parseInt(true) + "," + parseInt(123.45) + "," + parseInt("  -1f", 16));
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "r1=1,2,0,-1,-2#r2=2,-3,true,true\
       #i1=16,16,42,42,-17#i2=8,0,2,1295,NaN,3#i3=NaN,NaN,123,-31"
        .into()
    )
  );
  OK
}

/// 39. 字符串切片三法语义合一 (12.25): substring 负/NaN 夹 0 且 start>end 交换,
///     slice 负从尾起算**不交换**, substr 第二参是长度 (Annex B); `Infinity` 参走饱和索引
///     (`substring(2,Infinity)`="cdef" 而 `substring(2,-Infinity)`="ab")。bc 链此前把
///     substring/slice 共用一份混合实现且整体缺 `substr` (AST 有 —— 链分裂), 现两链共走
///     引擎侧 `slice_window` 单一真相, 多字节串按字符索引不裂
#[test]
fn test_string_slice_family_semantics() -> Void {
  let res = both_chains(
    r##"
    var out = []; var n = 0;
    function say(s) { out[n] = s; n = n + 1; }
    var s = "abcdef";
    say("c1=" + s.slice(-2) + "|" + s.slice(3,-1) + "|" + s.slice(4,2) + "|" + s.slice(10));
    say("c2=" + s.substring(-2) + "|" + s.substring(4,2) + "|" + s.substring(NaN,3) + "|" + s.substring(2,Infinity) + "|" + s.substring(2,-Infinity));
    say("c3=" + s.substr(2,2) + "|" + s.substr(-2) + "|" + s.substr(-3,2) + "|" + s.substr(9) + "|" + s.substr(2,0) + "|" + s.substr(2,-1));
    var u = "中文abc";
    say("c4=" + u.slice(1,3) + "|" + u.substring(-1) + "|" + u.substr(1,2) + "|" + typeof u.substr);
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "c1=ef|de||#c2=abcdef|cd|abc|cdef|ab#c3=cd|ef|de|||\
       #c4=文a|中文abc|文a|function"
        .into()
    )
  );
  OK
}

/// 40. 对象/数字/字符串静态面补齐 (12.26): JSON.parse、Object.values/entries/assign/is/
///     getPrototypeOf、Number.isInteger、String.fromCodePoint。宿主内建一处定义两链共读, 静态面
///     的**结构**结果 (值、键集、序列化回环) 两链一致且对 node 逐字
///
/// 注: 跨桥对象身份 (`Object.assign` 返回 target 本身、`getPrototypeOf(child)===base`、
/// `setPrototypeOf` 就地改写) 在字节码链因**深拷贝桥**丢失 (宿主 JsValue 与引擎 Value 双值模型,
/// 每次跨界都新建副本) —— 属战役 C (值模型合一) 未闭环项, 不入钉, 只钉两链一致的结构结果
#[test]
fn test_object_number_string_statics_surface() -> Void {
  let res = both_chains(
    r##"
    var out = []; var n = 0;
    function say(s) { out[n] = s; n = n + 1; }
    var o = { a: 1, b: "x", c: true };
    say("j1=" + JSON.stringify(JSON.parse('{"k":[1,"s",null,true,false,{"n":-1.5e2}]}')));
    say("j2=" + JSON.stringify(JSON.parse('[1,[2,{"a":3}],"z"]')));
    say("j3=" + JSON.parse("123") + "|" + JSON.parse("null") + "|" + (JSON.parse("true")===true) + "|" + JSON.parse(' " spaced " ').length);
    var bad=0; try { JSON.parse("{bad"); } catch(e) { bad=1; }
    say("j4=" + bad + "|" + (1/JSON.parse("0")));
    say("v1=" + Object.values(o).sort().join(",") + "|" + Object.entries(o).map(function(e){return e[0]+":"+e[1];}).sort().join(";"));
    var t = { p: 1, q: 2 };
    var r = Object.assign(t, { q: 9, s: 3 });
    say("a2=" + Object.keys(r).sort().join(",") + "|" + r.q + r.s + "|" + typeof r);
    var base = { hi: function(){ return "H"; } };
    var child = Object.create(base);
    say("c1=" + child.hi() + "|" + typeof Object.getPrototypeOf);
    say("is=" + Object.is(NaN,NaN) + "," + Object.is(0,-0) + "," + Object.is(1,1) + "," + Object.is("a","a"));
    say("ni=" + Number.isInteger(1.0) + "," + Number.isInteger("1") + "," + Number.isInteger(NaN) + "," + Number.isInteger(-0) + "," + Number.isInteger(1.5));
    say("fc=" + String.fromCharCode(67) + "|" + String.fromCodePoint(67) + "|" + String.fromCodePoint(20013));
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "j1={\"k\":[1,\"s\",null,true,false,{\"n\":-150}]}#j2=[1,[2,{\"a\":3}],\"z\"]\
       #j3=123|null|true|8#j4=1|Infinity#v1=1,true,x|a:1;b:x;c:true\
       #a2=p,q,s|93|object#c1=H|function#is=true,false,true,true\
       #ni=true,false,false,true,false#fc=C|C|中"
        .into()
    )
  );
  OK
}

/// 41. 字符串 ES2015+ 方法面 (12.26): includes/repeat/padStart/padEnd/at/codePointAt。
///     行为在引擎侧 `str_*` 纯函数一处定义, 两链 get_property 表共指 (切片三连已验证的收敛策略)。
///     `repeat(-1)` 与 `padStart(Infinity)` 抛 RangeError (可捕获性两链一致); `includes` 空针
///     命中夹取位 (`"abc".includes("",4)` 真); `codePointAt` 越界给 undefined (非 charCodeAt 的 NaN)
///
/// 注: astral 码位 (`"😀".length`/`.at`/`.codePointAt`) 走码点口径与 node UTF-16 码元口径
/// 有差 (战役 C), 只钉 BMP 面
#[test]
fn test_string_modern_methods_surface() -> Void {
  let res = both_chains(
    r##"
    var out = []; var n = 0;
    function say(s) { out[n] = s; n = n + 1; }
    say("s1=" + "hello world".includes("lo w") + "," + "hello".includes("ell", 2) + "," + "hello".includes("") + "," + "ABC".includes("abc") + "," + "abc".includes("", 3) + "," + "abc".includes("", 4));
    say("s2=" + "ab".repeat(3) + "|" + "ab".repeat(0) + "|" + "x".repeat(1.9) + "|" + typeof "ab".repeat);
    var rb=0; try { "ab".repeat(-1); } catch(e) { rb=1; }
    say("s3=" + rb + "|" + "abc".padStart(6) + "|" + "abc".padStart(6,"xy") + "|" + "abc".padEnd(5,"0") + "|" + "abc".padStart(2) + "|" + "abc".padStart(7,"..".padEnd(3,"=")) + "|" + "abc".padEnd(0,"z"));
    var pb=0; try { "abc".padStart(Infinity); } catch(e) { pb=1; }
    say("s4=" + pb + "|" + "abc".at(-1) + "|" + "abc".at(1) + "|" + "abc".at(9) + "|" + "abc".at(-9) + "|" + typeof "abc".at);
    say("s5=" + "a".codePointAt(0) + "|" + "中".codePointAt(0) + "|" + "abc".codePointAt(3));
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "s1=true,false,true,false,true,true#s2=ababab||x|function\
       #s3=1|   abc|xyxabc|abc00|abc|..=.abc|abc\
       #s4=1|c|b|undefined|undefined|function#s5=97|20013|undefined"
        .into()
    )
  );
  OK
}

/// 42. `Array.prototype.findIndex` (12.26): 回调截获与 `find` 同路 (宿主
///     `invoke_function_named` / 引擎 `ArrayIterKind::FindIndex`), 命中交下标、未命中 -1;
///     非函数回调抛 TypeError (可捕获)。回调内改写同下标不影响后续交出 (每步现取元素)
#[test]
fn test_array_find_index_semantics() -> Void {
  let res = both_chains(
    r##"
    var out = []; var n = 0;
    function say(s) { out[n] = s; n = n + 1; }
    say("a1=" + [5,8,3].findIndex(function(x){return x>6;}) + "|" + [5,8,3].findIndex(function(x){return x>9;}) + "|" + ["a"].findIndex(function(x,i,arr){return x===arr[0];}));
    say("a2=" + [].findIndex(function(){return true;}) + "|" + [7,7,7].findIndex(function(x,i){return i===2;}) + "|" + typeof [1].findIndex);
    var tb=0; try { [1].findIndex(); } catch(e) { tb=1; }
    var tb2=0; try { [1].findIndex("nope"); } catch(e) { tb2=1; }
    say("a3=" + tb + "|" + tb2);
    var e2 = [1,2,3];
    say("a4=" + e2.findIndex(function(x,i){ if (i===0) e2[1]=99; return x===99; }));
    out.join("#");
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String("a1=1|-1|0#a2=-1|2|function#a3=1|1#a4=1".into())
  );
  OK
}

/// 43. Math 单目面补齐 (12.27): ceil/trunc/cbrt/tan/asin/acos/atan/log2/log10/sign 收进
///     `NativeFn::MathUnary` 按名分派 (宿主 init_builtins 一处定义, BC 经桥接共用);
///     `Math.sign` 按 IEEE 口径 ±0 保号 (Rust signum 对 0 交 ±1, 口径不同), NaN 原样;
///     hypot 的 ±∞ 压过 NaN; fround/clz32/imul/atan2 走显式闭包
#[test]
fn test_math_unary_surface() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("m1=" + Math.ceil(1.2) + "|" + Math.ceil(-1.2) + "|" + (1 / Math.ceil(-0.5)) + "|" + Math.ceil(0));
say("m2=" + Math.trunc(1.9) + "|" + Math.trunc(-1.9) + "|" + (1 / Math.trunc(-0.5)));
say("m3=" + Math.sign(5) + "|" + Math.sign(-5) + "|" + Math.sign(0) + "|" + (1 / Math.sign(-0)) + "|" + Math.sign(NaN) + "|" + Math.sign("7"));
say("m4=" + Math.cbrt(8) + "|" + Math.cbrt(-27) + "|" + Math.cbrt(0));
say("m5=" + Math.hypot(3, 4) + "|" + Math.hypot() + "|" + Math.hypot(-5, 12) + "|" + Math.hypot(1, 2, 2) + "|" + Math.hypot(0, Infinity, NaN));
say("m6=" + Math.fround(1.1) + "|" + Math.fround(0.5) + "|" + Math.fround(1 / 1024));
say("m7=" + Math.clz32(1) + "|" + Math.clz32(0) + "|" + Math.clz32(-1) + "|" + Math.clz32(4294967295));
say("m8=" + Math.imul(-1, -1) + "|" + Math.imul(2147483647, 2147483647) + "|" + Math.imul(3, 4) + "|" + Math.imul(0, 5));
say("m9=" + Math.atan2(1, 1) * 2 + "|" + Math.acos(1) + "|" + Math.asin(0) + "|" + Math.atan(0) + "|" + Math.log2(8) + "|" + Math.log10(1000));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(res, JsValue::String("m1=2|-1|-Infinity|0#m2=1|-1|-Infinity#m3=1|-1|0|-Infinity|NaN|1#m4=2|-3|0#m5=5|0|13|3|Infinity#m6=1.100000023841858|0.5|0.0009765625#m7=31|32|0|0#m8=1|1|12|0#m9=1.5707963267948966|0|0|0|3|3".into()));
  OK
}

/// 44. 字符串实例面与 isSafeInteger (12.27): trim 族吃 \r\n 等 WhiteSpace;
///     startsWith/endsWith 位置夹取与缺省尾界; needle 省略按 ToString(undefined)=
///     "undefined" 而非空串 (旧默认 "" 是两链同犯的真实缺陷, 本门禁钉死);
///     String.prototype.concat 补 BC 链分裂缺失; Number.isSafeInteger 只认整值
#[test]
fn test_string_instance_and_is_safe_integer() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("s1=" + "\t\n a b \r\n".trim() + "|" + " \t x \r\n".trimStart() + "|" + "  a  ".trimEnd() + "|" + "".trim() + "|" + "   ".trim());
say("s2=" + "abc".startsWith("ab") + "|" + "abc".startsWith("bc") + "|" + "abc".startsWith("bc", 1) + "|" + "abc".startsWith("", 200) + "|" + "abc".startsWith("abc", 3));
say("s3=" + "abc".endsWith("bc") + "|" + "abc".endsWith("a") + "|" + "abc".endsWith("ab", 2) + "|" + "abc".endsWith("", 200) + "|" + "abc".endsWith("c", 0));
say("s4=" + "abc".startsWith() + "|" + "abc".endsWith() + "|" + "abc".includes() + "|" + "undefinedx".startsWith("undefined"));
say("s5=" + "a".concat("b", "c", 1, null, true) + "|" + "".concat() + "|" + "x".concat(1.5));
say("n1=" + Number.isSafeInteger(9007199254740991) + "|" + Number.isSafeInteger(9007199254740992) + "|" + Number.isSafeInteger(1.5) + "|" + Number.isSafeInteger("5") + "|" + Number.isSafeInteger(NaN));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(res, JsValue::String("s1=a b|x \r\n|  a||#s2=true|false|true|true|false#s3=true|false|true|true|false#s4=false|false|false|true#s5=abc1nulltrue||x1.5#n1=true|false|false|false|false".into()));
  OK
}

/// 45. replaceAll (12.27): 空针串在每个字符边界各插一次且末尾补一发;
///     $$ 交字面 $, $& 交匹配串; 非全局正则抛 TypeError (规范口径), 全局正则走
///     replace 主体; 回调替换实参挂战役 C
#[test]
fn test_replace_all_semantics() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("r1=" + "a.b.c".replaceAll(".", "-") + "|" + "aaa".replaceAll("a", "bb") + "|" + "".replaceAll("a", "b") + "|" + "abc".replaceAll("", "-"));
say("r2=" + "ab".replaceAll("b", "$&") + "|" + "aa".replaceAll("a", "$$") + "|" + "ab".replaceAll("b", "$$x"));
say("r3=" + (function () { try { "abc".replaceAll(/b/, "-"); return "no"; } catch (e) { return "yes"; } })());
say("r4=" + "aaa".replaceAll(/a/g, "b") + "|" + typeof "".replaceAll);
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String("r1=a-b-c|bbbbbb||-a-b-c-#r2=ab|$$|a$x#r3=yes#r4=bbb|function".into())
  );
  OK
}

/// 46. Array 实例面第三批 (12.27): at 负绕回; flat 缺省深度 1、Infinity 拉平、0 原样;
///     flatMap/reduceRight 回调经两链就地截获 (引擎 ArrayIterKind / 宿主
///     invoke_function_named); copyWithin 负下标夹取且返回接收者本体
#[test]
fn test_array_instance_round_three() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("a1=" + [1, 2, 3].at(-1) + "|" + [1, 2, 3].at(3) + "|" + [1, 2, 3].at(-5) + "|" + [].at(0) + "|" + [1, 2, 3].at(-3));
say("a2=" + (function () { var F = [1, [2, [3]]].flat(); return F.length + "|" + Array.isArray(F[2]) + "|" + [1, [2, [3]]].flat(Infinity).join(",") + "|" + [1, [2, [3]]].flat(0).length + "|" + [[1, [2]], 3].flat(2).join(","); })());
say("a3=" + [1, 2, 3].flatMap(function (x) { return [x, x]; }).join(",") + "|" + [1, 2].flatMap(function (x) { return []; }).length);
say("a4=" + [1, 2, 3].reduceRight(function (a, b) { return a + "-" + b; }) + "|" + [1, 2, 3].reduceRight(function (a, b) { return a + "-" + b; }, "x") + "|" + [7].reduceRight(function (a, b) { return a; }) + "|" + (function () { try { return [].reduceRight(function (a, b) { return a; }); } catch (e) { return "E"; } })() + "|" + [1, 2].reduceRight(function (a, b, i, arr) { return i + "" + arr.length; }, 0));
say("a5=" + [1, 2, 3, 4, 5].copyWithin(0, 3).join(",") + "|" + [1, 2, 3, 4, 5].copyWithin(-2).join(",") + "|" + [1, 2, 3, 4, 5].copyWithin(0, -2, -1).join(",") + "|" + (function () { var a = [1, 2]; return a.copyWithin(1, 0) === a; })());
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(res, JsValue::String("a1=3|undefined|undefined|undefined|1#a2=3|true|1,2,3|2|1,2,3#a3=1,1,2,2,3,3|0#a4=3-2-1|x-3-2-1|7|E|02#a5=4,5,3,4,5|1,2,3,1,2|4,2,3,4,5|true".into()));
  OK
}

/// 47. 静态面第三批 (12.27): Array.from 吃字符串 (mapfn 回调挂战役 C, 宁报错不静默);
///     Array.of 单参不塌长度; Object.getOwnPropertyNames 过滤内部键、数组交
///     下标+length; Object.fromEntries 支持数组对与 "0"/"1" 属性对
#[test]
fn test_static_surface_round_three() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("f1=" + Array.from("abc").join("-") + "|" + Array.from("中a").length + "|" + (function () { try { Array.from(null); return "no"; } catch (e) { return "yes"; } })());
say("f2=" + Array.of(7).join(",") + "|" + Array.of(1, 2, 3).length + "|" + Array.of().length + "|" + Array.of("x").length);
say("f3=" + Object.getOwnPropertyNames({ b: 1, a: 2 }).sort().join(",") + "|" + Object.getOwnPropertyNames([1, 2]).sort().join(","));
say("f4=" + JSON.stringify(Object.fromEntries([["a", 1]])) + "|" + (function () { var r = Object.fromEntries([["b", 2], ["a", 1]]); return Object.keys(r).sort().join(",") + ":" + r.a + r.b; })() + "|" + typeof Object.fromEntries([]) + "|" + Object.fromEntries([["x", 1]]).x);
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "f1=a-b-c|2|yes#f2=7|3|0|1#f3=a,b|0,1,length#f4={\"a\":1}|a,b:12|object|1".into()
    )
  );
  OK
}

/// 48. URI 编解码与 \u{...} 码点转义 (12.27): encodeURI 直通保留字、
///     encodeURIComponent 收紧集; decode 坏 % 序列抛 URI malformed (宿主名册暂无
///     URIError 变体, 挂战役 C); lexer \u{...} 花括号码点两链同源; astral 的
///     UTF-16 length 面按 BMP 豁免记账, 不在此钉
#[test]
fn test_uri_encode_decode_and_astral_escape() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("u1=" + encodeURIComponent("A b-c_d.e~f/g?h#i@j$k&l=m:n;o<p>q'r(s)t*u+v,1中") + "|" + encodeURIComponent("a b"));
say("u2=" + encodeURIComponent(undefined) + "|" + encodeURIComponent(null) + "|" + encodeURIComponent(1.5) + "|" + encodeURIComponent(""));
say("u3=" + decodeURIComponent("a%20b%E4%B8%AD") + "|" + decodeURIComponent("100%25") + "|" + decodeURIComponent("+"));
say("u4=" + (function () { try { decodeURIComponent("%"); return "no"; } catch (e) { return "yes"; } })() + "|" + (function () { try { decodeURIComponent("%E4%B8"); return "no"; } catch (e) { return "yes"; } })());
say("u5=" + encodeURI("a b?c=d&e#f") + "|" + encodeURI("中/€a") + "|" + decodeURI("a%20b?c%3Dd"));
say("u6=" + encodeURIComponent("\u{1F600}") + "|" + decodeURIComponent("%F0%9F%98%80"));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(res, JsValue::String("u1=A%20b-c_d.e~f%2Fg%3Fh%23i%40j%24k%26l%3Dm%3An%3Bo%3Cp%3Eq'r(s)t*u%2Bv%2C1%E4%B8%AD|a%20b#u2=undefined|null|1.5|#u3=a b中|100%|+#u4=yes|yes#u5=a%20b?c=d&e#f|%E4%B8%AD/%E2%82%ACa|a b?c%3Dd#u6=%F0%9F%98%80|😀".into()));
  OK
}

/// 49. 箭头函数形态与词法 this/arguments (12.28): 单参/空参/多参与表达式体、块体;
///     箭头没有自身 this/arguments, 体内引用回落到创建时刻定格的词法帧 (与原版
///     JSFunction.context 同路); 方法内箭头捕获调用点 receiver; 返回对象字面量需 parens
#[test]
fn test_arrow_lexical_capture() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("v1=" + (function () { return (x => x + 1)(2) + "|" + (() => 3)() + "|" + ((a, b) => a + b)(1, 2); })());
say("v2=" + (function (p) { var f = () => p * 2; return f(); })(5));
say("v3=" + (function () { return (() => arguments[0] + "/" + arguments.length)(); })(9));
say("v4=" + (function () { var o = { t: "OT", m: function () { var f = () => this.t; return f(); } }; return o.m(); })());
say("v5=" + ((a) => ({ a: a }))(4).a + "|" + [1, 2, 3].map(x => x * x).join(","));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String("v1=3|3|3#v2=10#v3=9/1#v4=OT#v5=4|1,4,9".into())
  );
  OK
}

/// 50. 箭头函数表面 (12.28): 无 prototype 属性 (普通函数有); new 一律 TypeError;
///     toString 交源码切片; length 只数形参 (默认值/解构在战役 C, 不在本钉)
#[test]
fn test_arrow_function_surface() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("w1=" + ("prototype" in (() => 1)) + "|" + ("prototype" in (function () {})));
say("w2=" + (function () { try { new (() => 1); return "no"; } catch (e) { return "yes"; } })());
say("w3=" + ((x) => x + 1).toString().trim() + "|" + ((a, b) => a).length + "|" + (() => 0).length);
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String("w1=false|true#w2=yes#w3=(x) => x + 1|2|0".into())
  );
  OK
}

/// 51. 模板串表达式分组 (12.28): 脱糖注入的 parens 让 `${1+2}` 先算加法再进字符串面,
///     缺了分组 `"a" + 1 + 2` 左结合会交出 a12b; 嵌套对象取属性/箭头调用/负数字面量
///     都在表达式位上照常成立
#[test]
fn test_template_literal_expression_grouping() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("t1=" + `a${1 + 2}b` + "|" + `${1}${2}` + "|" + `x${2 * 3 + 1}y`);
say("t2=" + `t${ { k: 1 }.k }x` + "|" + `c${ "a" + "b" }` + "|" + `n${ -5 }|${ 5 - 2 }`);
say("t3=" + `f${ (x => x * 2)(3) }` + "|" + `a${1}b`.length + "|" + `p${1}`.slice(0, 1) + "|" + `e${0/0}`);
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String("t1=a3b|12|x7y#t2=t1x|cab|n-5|3#t3=f6|3|p|eNaN".into())
  );
  OK
}

/// 52. spread 展开口径 (12.28): `[...str]` 逐字符 (直接 concat 会把整串当一个元素);
///     数组展开取快照 (改原数组不回头改已展平副本); 对象展开经 Object.assign
///     覆盖层的属性读点物化 accessor 值 (原生替身只搬值槽会整个丢掉 getter 键)
#[test]
fn test_spread_flattening_semantics() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("p1=" + [...""].length + "|" + [..."ab"].join(",") + "|" + [...[1, 2], 3].join(","));
say("p2=" + (function () { var a = [1, 2]; var c = [...a]; a[0] = 9; return c.join(","); })());
say("p3=" + JSON.stringify({ ...{ get g() { return 5; } } }) + "|" + JSON.stringify({ ...{ a: 1 }, b: 2 }));
say("p4=" + (function () { var o = { ...{ x: 1 }, y: 2 }; return o.x + "," + o.y + "|" + Object.keys(o).sort().join("+"); })());
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String("p1=0|a,b|1,2,3#p2=1,2#p3={\"g\":5}|{\"a\":1,\"b\":2}#p4=1,2|x+y".into())
  );
  OK
}

/// 53. Number.prototype.toExponential (12.28): 参数缺省 (含显式 undefined) 走最短
///     数字面 (f=-1), 与 toPrecision 的 ToString 回落同族但口径不同; 0..=100 之外
///     RangeError; 非有限值直吐 NaN/±Infinity
#[test]
fn test_number_to_exponential() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("e1=" + (123456).toExponential(2) + "|" + (0.0001).toExponential(1) + "|" + (123).toExponential() + "|" + (0).toExponential(0));
say("e2=" + (-1234).toExponential(3) + "|" + (1e21).toExponential(2) + "|" + (1.5e-7).toExponential() + "|" + (1).toExponential(100).slice(0, 3));
say("e3=" + (function () { try { (1).toExponential(101); return "no"; } catch (e) { return "yes"; } })() + "|" + (123456).toExponential(0) + "|" + (0 / 0).toExponential() + "|" + (1).toExponential(undefined));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "e1=1.23e+5|1.0e-4|1.23e+2|0e+0#e2=-1.234e+3|1.00e+21|1.5e-7|1.0#e3=yes|1e+5|NaN|1e+0".into()
    )
  );
  OK
}

/// 54. 历史别名与枚举面 (12.28): trimLeft/trimRight 挂附录 B 名册与 trimStart/End
///     同一实现; propertyIsEnumerable 只认自有可枚举面 (hasOwnProperty 的 `p in this`
///     替身会把链上方法算进来, 这里用 keys 真相); isPrototypeOf 沿链 identity 比对
///     的可对齐子集; assign/values/entries 物化 accessor 且 null target 抛 TypeError
#[test]
fn test_legacy_aliases_and_enumerability_surface() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("z1=" + "[" + "  a  ".trimLeft() + "][" + "  a  ".trimRight() + "]");
say("z2=" + ({ a: 1 }).propertyIsEnumerable("a") + "|" + ({}).propertyIsEnumerable("a") + "|" + typeof ({}).isPrototypeOf);
say("z3=" + Object.prototype.isPrototypeOf.call({}, []));
say("z4=" + Object.assign({}, { get g() { return 7; } }).g + "|" + Object.values({ get g() { return 1; } })[0] + "|" + Object.entries({ a: 2 })[0][1]);
say("z5=" + (function () { try { Object.assign(null, { a: 1 }); return "no"; } catch (e) { return "yes"; } })() + "|" + Object.assign({ a: [1], ...{ a: [2] } }).a[0] + "|" + Object.assign([7], { "1": 8 }).length);
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String("z1=[a  ][  a]#z2=true|false|function#z3=false#z4=7|1|2#z5=yes|2|2".into())
  );
  OK
}

/// 55. delete 口径收口 (12.29): sloppy 删除恒 true (缺失/继承/已删二次删亦然)，唯一
///     例外是数组 `length` 与 `delete b.length` 假; 下标 delete 以 Undefined 填槽近似
///     留洞 —— `join` 吐空段与 node 同象, `1 in a`/`Object.keys` 真洞口径挂战役 C;
///     字符串/数字接收者删属性不报错恒 true
#[test]
fn test_delete_sloppy_surface() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
function pad(x) { return x < 10 ? "0" + x : "" + x; }
say("k=" + (function () { var a = [1, 2, 3]; var p = pad(delete a[1]) + pad(delete a["length"]) + pad(delete a.huh) + pad(delete "abc".x) + pad(delete (5).x); return p + "|" + a.length + "|" + a[1] + "|" + a.join(","); })());
say("m=" + (function () { var o = { q: 1 }; delete o.q; return delete o.q + "|" + delete o.missing + "|" + ("q" in o) + "|" + Object.keys(o).length; })());
say("l=" + (function () { var b = [4, 5, 6]; return pad(delete b[0]) + pad(delete b.length) + "|" + b.length; })());
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "k=0true0false0true0true0true|3|undefined|1,,3#m=true|true|false|0#l=0true0false|3".into()
    )
  );
  OK
}

/// 56. typeof 名册与 == 强制表 (12.29): undeclared 标识符不抛且给 undefined,
///     typeof null==="object" 历史钉子, void 0 收口 undefined; null/undefined 只彼此
///     相等、NaN 自不等、字符串与数字面互转比较的整表两链与 node 逐字符对齐
#[test]
fn test_typeof_and_equality_coercion_table() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("q1=" + (null == undefined) + "|" + (null == 0) + "|" + (undefined === undefined) + "|" + (NaN == NaN) + "|" + (0 == "0") + "|" + ("" == 0));
say("q2=" + ("1" == 1) + "|" + (true == 1) + "|" + (false == 0) + "|" + ([1] == 1) + "|" + ("a" != "b") + "|" + (1 != "1"));
say("q3=" + ([] == 0) + "|" + (null > -1) + "|" + (undefined == false) + "|" + (NaN != NaN) + "|" + (" " == 0) + "|" + (null == false));
say("y1=" + typeof undevar + "|" + typeof (function () {}) + "|" + typeof null + "|" + typeof 1 + "|" + typeof "s" + "|" + typeof true + "|" + typeof undefined);
say("y2=" + typeof {} + "|" + typeof [] + "|" + typeof Math + "|" + typeof (1 + 2) + "|" + typeof (0 / 0) + "|" + typeof void 0);
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "q1=true|false|true|false|true|true#q2=true|true|true|true|true|false#q3=true|true|false|true|true|false#y1=undefined|function|object|number|string|boolean|undefined#y2=object|object|object|number|number|undefined"
        .into()
    )
  );
  OK
}

/// 57. for-in 接收者面 (12.29): 字符串按索引枚举 (两链同源, 引擎 `ForInEnumerate`
///     String 臂 / AST 同名臂), 数组给升序下标; 数字/null/undefined 零轮。
///     对象键插入序与继承键枚举均已记账 (战役 C / 原型链接线)
#[test]
fn test_for_in_receiver_surface() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("f1=" + (function () { var s = ""; for (var k in "ab") s = s + k; return s; })());
say("f2=" + (function () { var s = ""; for (var k in [10, 20]) s = s + k; return s; })());
say("f3=" + (function () { var r = []; for (var k2 in 5) r.push(k2); var r2 = []; for (var k3 in null) r2.push(k3); return r.length + "|" + r2.length; })());
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(res, JsValue::String("f1=01#f2=01#f3=0|0".into()));
  OK
}

/// 58. 内建面扫面第六轮之排序/回调/字符串/Number 面 (12.30): 缺省字典序与数值比较器、
///     sort 遇 undefined 沉尾、splice 负起点与超额删除、回调三实参与 forEach 返回 undefined、
///     at/charCodeAt 越界、repeat 负数 RangeError、padStart 用串、toString 基数与静态常量表、
///     JSON 槽位 undefined→null 而顶层 undefined→undefined 值, 两链与 node 逐字符对齐
#[test]
fn test_internal_surface_sweep_six_sort_string_number() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("s1=" + [3, 11, 2, 1].sort().join(","));
say("s2=" + [3, 11, 2, 1].sort(function (a, b) { return a - b; }).join(","));
say("s3=" + ["b", "A", "a", "_"].sort().join(","));
say("s4=" + [1, null, undefined, 2, NaN].sort(function (a, b) { return 0; }).length + "|" + [1, undefined, 2].sort().join(","));
say("s5=" + (function () { var a = [1, 2, 3, 4, 5]; var r = a.splice(1, 2); return r.join(",") + "|" + a.join(","); })());
say("s6=" + (function () { var a = [1, 2, 3, 4, 5]; var r = a.splice(-2); return r.join(",") + "|" + a.length; })());
say("s7=" + (function () { var a = [1, 2, 3, 4, 5]; var r = a.splice(1, 100); return r.join(",") + "|" + a.join(","); })());
say("s8=" + [1, 2, 3, 4].slice(-2).join(",") + "|" + [1, 2, 3, 4].slice(1, 100).join(",") + "|" + [1, 2, 3, 4].slice(3, 1).length);
say("s9=" + [5, 6, 7].map(function (v, i, arr) { return v + i + (arr === undefined ? "?" : "A"); }).join(","));
say("sa=" + [5, 6, 7].forEach(function (v, i) { return i; }) + "|" + [1, 2].filter(function (v, i, a) { return a.length == 2 && v > 1; }).join(","));
say("sb=" + [1, NaN, 3].includes(NaN) + "|" + [1, 2, 3].includes(2, 2) + "|" + [1, 2, 1].indexOf(1, 1) + "|" + [1, 2, 1].lastIndexOf(1) + "|" + "abcabc".indexOf("c", 4));
say("sc=" + "abcdef".at(-2) + "|" + "abc".at(9) + "|" + "abc".charCodeAt(9) + "|" + "abc".codePointAt(0));
say("sd=" + (function () { try { "a".repeat(-1); return "no"; } catch (e) { return "RE"; } })() + "|" + "ab".repeat(0) + "|" + ("ab".repeat(2) == "abab"));
say("se=" + "x".padStart(3, "ab") + "|" + "x".padEnd(5) + "|[" + "5".padStart(2, "") + "]");
say("sg=" + (255).toString(16) + "|" + (0.1).toFixed(20).slice(0, 8) + "|" + parseInt("0x10") + "|" + parseInt("10", 2) + "|" + parseFloat("12.5abc") + "|" + Number("0b101") + "|" + Number("0o17"));
say("sh=" + Math.pow(2, 53) + "|" + Number.MAX_SAFE_INTEGER + "|" + (Number.isSafeInteger(Math.pow(2, 53))) + "|" + Number.EPSILON + "|" + Number.MIN_SAFE_INTEGER);
say("sj=" + JSON.stringify([1, undefined, "s", null]) + "|" + JSON.stringify({ a: undefined }) + "|" + JSON.stringify("q\"x") + "|" + JSON.stringify(undefined));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "s1=1,11,2,3#s2=1,2,3,11#s3=A,_,a,b#s4=5|1,2,#s5=2,3|1,4,5#s6=4,5|3#s7=2,3,4,5|1#s8=3,4|2,3,4|0#s9=5A,7A,9A#sa=undefined|2#sb=true|false|2|2|5#sc=e|undefined|NaN|97#sd=RE||true#se=abx|x    |[5]#sg=ff|0.100000|16|2|12.5|5|15#sh=9007199254740992|9007199254740991|false|2.220446049250313e-16|-9007199254740991#sj=[1,null,\"s\",null]|{}|\"q\\\"x\"|undefined"
        .into()
    )
  );
  OK
}

/// 59. 内建面扫面第六轮之下标归一与进制前缀 (12.30): indexOf/includes/fill 的 fromIndex
///     走 ToIntegerOrInfinity + 负数从尾部归一 (两链共用 tail_index 口径, 越界钳到端点),
///     lastIndexOf 反向起算, Number() 认 0x/0o/0b 全前缀而 parseInt 只认 0x (V8 判基名册),
///     裸前缀串与带空白串收口 NaN/数值, 与 node 逐字符对齐
#[test]
fn test_internal_surface_sweep_six_index_tail_and_radix() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("a1=" + [1,2,3].includes(2,2) + "|" + [1,2,3].includes(3,9) + "|" + [1,2].includes(1,-1) + "|" + [1,2].indexOf(2,-1) + "|" + [1,2].indexOf(2,-99) + "|" + [1,2].indexOf(2,undefined));
say("a2=" + Number("0b101") + "|" + Number("0o17") + "|" + Number("0B11") + "|" + Number("0X1f") + "|" + Number("0b") + "|" + Number("0x") + "|" + Number(" 0b11 "));
say("a3=" + parseInt("0b101") + "|" + parseInt("0o17") + "|" + parseInt("11",2) + "|" + parseInt("17",8) + "|" + Number("08") + "|" + Number("-0x10"));
say("a4=" + [1,2,1].lastIndexOf(1,-1) + "|" + [1,2,1].lastIndexOf(1,-3) + "|" + [1,2,1].lastIndexOf(1,-4) + "|" + [1,2,1].lastIndexOf(1,NaN) + "|" + [1,2,1].lastIndexOf(1,-0));
say("a5=" + [1,2,3].fill(0,undefined,NaN).join(",") + "|" + [1,2,3].fill(0,-1).join(",") + "|" + [1,2,3].fill(9).join(","));
say("a6=" + [1,2,3].includes(1,1e100) + "|" + [1,2,3].indexOf(3,1e100) + "|" + [1,2,3].fill(8,NaN,NaN).join(",") + "|" + [1,2,3].fill(7,-99,-98).join(","));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "a1=false|false|false|1|1|1#a2=5|15|3|31|NaN|NaN|3#a3=0|0|3|15|8|NaN#a4=2|0|-1|0|0#a5=1,2,3|1,2,0|9,9,9#a6=false|-1|1,2,3|1,2,3"
        .into()
    )
  );
  OK
}

/// 60. JSON.stringify 缩进面 (12.30): 数值步长每层累计空格换行、字符串步长按绝对深度重复
///     且同样插换行 (早期 "不插换行" 模型系误读), 步长截 10、空串/0/NaN/负数走紧凑、
///     1.7 截断取整、Infinity 钳 10、数字串 "10" 按字面用、制表符步长, 与 node 逐字符对齐
#[test]
fn test_json_stringify_indent_surface() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("j1=" + JSON.stringify({a:1,b:[1,{c:2}]},null,2).replace(/\n/g,"~"));
say("j2=" + JSON.stringify([1,[2]],null,"ab").replace(/\n/g,"~") + "@" + JSON.stringify({m:{},n:[]},null,2).replace(/\n/g,"~"));
say("j3=" + JSON.stringify({a:1},null,"") + "|" + JSON.stringify({a:1},null,0) + "|" + JSON.stringify({a:1},null,true) + "|" + JSON.stringify({a:1},null,"xxxxxxxxxxxxxxxxxxxx").replace(/\n/g,"~"));
say("j4=" + JSON.stringify({a:1},null,NaN) + "|" + JSON.stringify({a:1},null,-1) + "|" + JSON.stringify({a:1},null,1.7).replace(/\n/g,"~") + "|" + JSON.stringify({a:1},null,Infinity).replace(/\n/g,"~"));
say("j5=" + JSON.stringify({a:1},null,"10").replace(/\n/g,"~") + "@" + JSON.stringify([1,[2]],null,"\t").replace(/\n/g,"~").replace(/\t/g,">"));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "j1={~  \"a\": 1,~  \"b\": [~    1,~    {~      \"c\": 2~    }~  ]~}#j2=[~ab1,~ab[~abab2~ab]~]@{~  \"m\": {},~  \"n\": []~}#j3={\"a\":1}|{\"a\":1}|{\"a\":1}|{~xxxxxxxxxx\"a\": 1~}#j4={\"a\":1}|{\"a\":1}|{~ \"a\": 1~}|{~          \"a\": 1~}#j5={~10\"a\": 1~}@[~>1,~>[~>>2~>]~]"
        .into()
    )
  );
  OK
}

/// 61. 内建面扫面第七轮 · reduce/every/some 族边界 (12.31): 空数组带初值恒等、零轮回调仍先查
///     IsCallable (缺回调即使空数组也抛)、累子正反向口径、map/filter/every 的 undefined 回调抛,
///     与 node 逐字符对齐
#[test]
fn test_internal_surface_sweep_seven_reduce_family() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("r1=" + [].reduce(function (a, b) { return a + b; }, 7) + "|" + [].reduceRight(function (a, b) { return a + b; }, 9) + "|" + [1,2,3].reduce(function (a, b) { return a + b; }) + "|" + [1,2,3].reduceRight(function (a, b) { return "" + a + b; }));
say("r2=" + (function () { try { [].reduce(function (a, b) { return a; }); return "no"; } catch (e) { return "RE"; } })() + "|" + (function () { try { return [1].reduce() + "|no"; } catch (e) { return "RE"; } })() + "|" + (function () { try { return [1,2].reduceRight(undefined) + "|no"; } catch (e) { return "RE"; } })() + "|" + (function () { try { return [].reduce(undefined, 1) + "|no"; } catch (e) { return "RE"; } })());
say("r3=" + [].every(function () { return false; }) + "|" + [].some(function () { return true; }) + "|" + [1,2,3].every(function (v, i, a) { return a.length === 3 && i >= 0; }) + "|" + (function () { try { return [1].map(undefined) + "|no"; } catch (e) { return "RE"; } })());
say("r4=" + [1,2,3].reduce(function (a, v, i, arr) { return a + v + i + (arr.length); }, 0) + "|" + ["a","b","c"].reduce(function(a,b,i){return a+b+i;}) + "|" + ["a","b","c"].reduce(function(a,b,i,arr){return a + i + arr.length;}, "z") + "|" + [1,2,3].reduceRight(function(a,b){return a-b;}));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "r1=7|9|6|321#r2=RE|RE|RE|RE#r3=true|false|true|RE#r4=18|ab1c2|z031323|0".into()
    )
  );
  OK
}

/// 62. 内建面扫面第七轮 · 位运算/一元加负与 try-finally 控制流 (12.31): ToInt32/ToUint32 饱和
///     (NaN/Infinity→0、5e9 回绕、"0x10" 走 Number 前缀)、~/- 口径, return+finally 求值序、
///     finally 内 return 覆写、循环 continue/break 走 finally、关系比较字符串分支与 ! 名册,
///     与 node 逐字符对齐
#[test]
fn test_internal_surface_sweep_seven_bitwise_and_finally() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("b1=" + (1.5 | 0) + "|" + (NaN | 0) + "|" + (Infinity | 0) + "|" + (-Infinity | 0) + "|" + (5000000000 | 0) + "|" + (~5) + "|" + (~0));
say("b2=" + (-1 >>> 0) + "|" + (-1 >> 0) + "|" + (1 << 33) + "|" + (1000 << 2) + "|" + (1000 >> 2) + "|" + (7 & 3) + "|" + (7 | 3) + "|" + (7 ^ 3));
say("b3=" + ("0x10" | 0) + "|" + (null | 1) + "|" + (undefined | 1) + "|" + ("abc" | 0) + "|" + (+"42") + "|" + (+null) + "|" + (+[]) + "|" + (-"7"));
say("t1=" + (function () { var log = ""; try { log += "A"; return log += "R"; } finally { log += "F"; } })());
say("t2=" + (function () { try { return "try"; } finally { return "fin"; } })());
say("t3=" + (function () { try { throw "x"; } catch (e) { return "c" + e; } finally { } })());
say("t4=" + (function () { var s = ""; for (var i = 0; i < 3; i++) { try { if (i === 1) continue; if (i === 2) break; s += "t" + i; } finally { s += "f" + i; } } return s; })());
say("t5=" + (function () { try { try { throw 1; } catch (e) { return "inner" + e; } } catch (e2) { return "outer"; } finally { } })());
say("c1=" + ("10" > "9") + "|" + (10 > "9") + "|" + ("abc" < "abd") + "|" + (null < 1) + "|" + (undefined < 1) + "|" + (NaN < 1));
say("c3=" + ({} + "") + "|[" + (!{}) + "|" + (!0) + "|" + (!"") + "|" + (!!2) + "]");
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "b1=1|0|0|0|705032704|-6|-1#b2=4294967295|-1|2|4000|250|3|7|4#b3=16|1|1|0|42|0|0|-7#t1=AR#t2=fin#t3=cx#t4=t0f0f1f2#t5=inner1#c1=false|true|true|true|false|false#c3=[object Object]|[false|true|true|true]"
        .into()
    )
  );
  OK
}

/// 63. 内建面扫面第七轮 · split 缺省针/limit 与 hasOwnProperty 基元接收者 (12.31):
///     undefined 针交 [ whole ] ("undefined" 不作针)、limit ToUint32 负数走全量、regex 针转发,
///     Object.prototype 面回落到字符串/数组基元接收者 (宿主闭表显式补臂)、get/set 访问符两链、
///     与 node 逐字符对齐
#[test]
fn test_internal_surface_sweep_seven_split_and_own_check() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("p1=" + JSON.stringify("a,b,c".split(",")) + "|" + JSON.stringify("a,b,c".split(",", 2)) + "|" + JSON.stringify("abc".split("")) + "|" + JSON.stringify("abc".split()) + "|" + JSON.stringify("a,b,c".split(",", 0)) + "|" + JSON.stringify("a,b,c".split(",", -1)));
say("p2=" + JSON.stringify("a1b2c".split(/\d/)) + "|" + JSON.stringify("a1b2c".split(/\d/, 2)) + "|" + JSON.stringify("ab".split("z")) + "|" + JSON.stringify("".split(",")) + "|" + JSON.stringify("x".split("x")) + "|" + JSON.stringify("aundefinedb".split()) + "|" + JSON.stringify("x1y".split(undefined, 2)));
say("h1=" + ({ a: 1 }.hasOwnProperty("a")) + "|" + ({ a: 1 }.hasOwnProperty("toString")) + "|" + ("a" in { a: 1 }) + "|" + ("toString" in {}) + "|" + ([1,2].hasOwnProperty(0)) + "|" + ([1,2].hasOwnProperty("length")));
say("h2=" + (function () { function P() { this.p = 1; } P.prototype.q = 2; var o = new P(); return o.hasOwnProperty("p") + "|" + o.hasOwnProperty("q") + "|" + ("q" in o); })());
say("h3=" + "ab".hasOwnProperty("0") + "|" + "ab".hasOwnProperty("length") + "|" + "ab".hasOwnProperty("1") + "|" + "ab".hasOwnProperty("2") + "|" + "ab".hasOwnProperty("x") + "|" + (5).hasOwnProperty("x"));
say("g1=" + (function () { var o = { get x() { return 42; }, set x(v) { this.y = v * 2; } }; var a = o.x; o.x = 5; return a + "|" + o.y; })());
say("g3=" + (function () { var o = { get x() { return 1; } }; return String(o.x + 1) + "|" + (o.x = 9) + "|" + o.x; })());
say("g5=" + (typeof Object.create) + "|" + (typeof Object.getOwnPropertyNames));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "p1=[\"a\",\"b\",\"c\"]|[\"a\",\"b\"]|[\"a\",\"b\",\"c\"]|[\"abc\"]|[]|[\"a\",\"b\",\"c\"]#p2=[\"a\",\"b\",\"c\"]|[\"a\",\"b\"]|[\"ab\"]|[\"\"]|[\"\",\"\"]|[\"aundefinedb\"]|[\"x1y\"]#h1=true|false|true|true|true|true#h2=true|false|true#h3=true|true|true|false|false|false#g1=42|10#g3=2|9|1#g5=function|function"
        .into()
    )
  );
  OK
}

/// 64. 内建面扫面第八轮 · String.lastIndexOf 全表 (12.32): 两链原缺该方法 (TypeError);
///     node 实测口径钉死 —— 负 pos 只扫下标 0 (非 includes/indexOf 的"从尾归一")、NaN/undefined
///     交 len、起点上界 min(pos, len-针长)、空针交钳位位、undefined 针走 "undefined" 字面
#[test]
fn test_internal_surface_sweep_eight_string_last_index_of() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("s1=" + "hello".lastIndexOf("l") + "|" + "hello".lastIndexOf("x") + "|" + "hello".lastIndexOf("l", 2) + "|" + "hello".lastIndexOf("l", -1) + "|" + "hello".lastIndexOf("l", 0) + "|" + "hello".lastIndexOf(""));
say("s2=" + "hello".lastIndexOf(undefined) + "|" + "aNullb".lastIndexOf(null) + "|" + "anullb".lastIndexOf(null) + "|" + "hello".lastIndexOf("LL", 0) + "|" + "hello".lastIndexOf("ell", 100) + "|" + "".lastIndexOf("a"));
say("s3=" + "你好a".lastIndexOf("a") + "|" + "你好a".lastIndexOf("你") + "|" + "abcabc".lastIndexOf("bc", 4) + "|" + "abcabc".lastIndexOf("abc", 2) + "|" + "aaa".lastIndexOf("aa") + "|" + "aaa".lastIndexOf("aa", 1));
say("s4=" + "hello".lastIndexOf("h", -1) + "|" + "hello".lastIndexOf("l", NaN) + "|" + "hello".lastIndexOf("l", -Infinity) + "|" + "hello".lastIndexOf("l", Infinity) + "|" + "hello".lastIndexOf("", -1) + "|" + "ababa".lastIndexOf("ba", -3));
say("s5=" + "hello".indexOf("l", 3) + "|" + "hello".indexOf("", 3) + "|" + "hello".indexOf("L") + "|" + "hello".indexOf(undefined) + "|" + "hello".includes("LL", 0) + "|" + "hello".startsWith("He", 0));
say("s6=" + "abc".at(-1.7) + "|" + "abc".at(-0.5) + "|" + "abc".at(null) + "|" + "abc".at(undefined) + "|" + "abc".slice(-2) + "|" + "abc".substring(3,1) + "|" + "abc".substr(1,0));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "s1=3|-1|2|-1|-1|5#s2=-1|-1|1|-1|1|-1#s3=2|0|4|0|1|1#s4=0|3|-1|3|0|-1#s5=3|3|-1|-1|false|false#s6=c|a|a|a|bc|bc|"
        .into()
    )
  );
  OK
}

/// 65. 内建面扫面第八轮 · 针参强制转换与 Array 无回调面 (12.32): indexOf/lastIndexOf/at 的
///     null→"null"、undefined→"undefined" 字面量针、at 截断口径; copyWithin 负 target、
///     flat 深度钳制 (0/负/NaN/Infinity)、flatMap、函数 .length 形参位 —— 与 node 逐字符对齐
#[test]
fn test_internal_surface_sweep_eight_coercion_and_array_faces() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("q1=" + "".indexOf(null) + "|" + "aNullb".indexOf(null) + "|" + "anullb".indexOf(null) + "|" + "abc".lastIndexOf(null) + "|" + "aNullb".lastIndexOf(null));
say("q2=" + "abc".at(-1.7) + "|" + "abc".at(-0.5) + "|" + "abc".at(null) + "|" + "abc".at(undefined) + "|" + "abc".indexOf(undefined));
say("q3=" + "abc".includes(undefined) + "|" + "abc".startsWith(undefined) + "|" + "abc".includes(null));
say("a1=" + JSON.stringify([1,2,3,4,5].copyWithin(-2)) + "|" + JSON.stringify([1,2,3,4,5].copyWithin(0,-2)) + "|" + JSON.stringify([1,2,3,4,5].copyWithin(1,3,4)) + "|" + JSON.stringify([1,2,3,4,5].copyWithin()));
say("a2=" + JSON.stringify([1,[2,[3,[4]]]].flat()) + "|" + JSON.stringify([1,[2,[3]]].flat(0)) + "|" + JSON.stringify([1,[2,[3]]].flat(-1)) + "|" + JSON.stringify([1,[2,[3]]].flat(NaN)) + "|" + JSON.stringify([1,[2,[3,[4,[5]]]]].flat(Infinity)));
say("a3=" + JSON.stringify([1,2,3].flatMap(function(x){return [x,x*2];})) + "|" + JSON.stringify([1,2].flatMap(function(x){return [x];})) + "|" + JSON.stringify([[1],[2]].flatMap(function(a){return a;})));
say("a4=" + (function f(a,b){}.length) + "|" + (function(){}.length) + "|" + (function(a){}+"").indexOf("function") + "|" + typeof Math.max);
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "q1=-1|-1|1|-1|-1#q2=c|a|a|a|-1#q3=false|false|false#a1=[1,2,3,1,2]|[4,5,3,4,5]|[1,4,3,4,5]|[1,2,3,4,5]#a2=[1,2,[3,[4]]]|[1,[2,[3]]]|[1,[2,[3]]]|[1,[2,[3]]]|[1,2,3,4,5]#a3=[1,2,2,4,3,6]|[1,2]|[1,2]#a4=2|0|0|function"
        .into()
    )
  );
  OK
}

/// 66. 内建面扫面第九轮 · Object.hasOwn 与 String.raw (12.33): hasOwn 走 hasOwnProperty 替身
///     同一口径 (基元接收者/length 自有面/null 首参 TypeError); String.raw 段数由 raw 名册定
///     (空名册吐 "")、实参**存在**才插入 (缺位不吐 "undefined" 字面、显式 undefined 才吐)、
///     多余实参忽略、下标对象形名册 —— 全矩阵与 node 逐字符对齐
#[test]
fn test_internal_surface_sweep_nine_has_own_and_string_raw() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
say("y1=" + Object.hasOwn({a:1},"a") + "|" + Object.hasOwn({a:1},"toString") + "|" + Object.hasOwn([1,2],"length") + "|" + Object.hasOwn("ab","0") + "|" + Object.hasOwn("ab","x"));
say("y2=" + (function(){try{return String(Object.hasOwn(null,"a"))+"|no";}catch(e){return "RE";}})() + "|" + Object.hasOwn({}, "0") + "|" + Object.hasOwn({1:"x"}, 1));
say("w1=" + String.raw({raw:["a","b","c"]}, 1, 2) + "|" + String.raw({raw:["x"]}) + "|" + String.raw({raw:["\\\\n","_"]}, "Q"));
say("w2=" + (function(){try{return String.raw(null)+"|no";}catch(e){return "RE";}})() + "|" + String.raw({raw:[1,2,3]}, "a") + "@" + String.raw({raw:[1,2,3]},"a","b") + "@" + String.raw({raw:[1,2,3]}));
say("r1=" + JSON.stringify([String.raw({raw:["a","b","c"]}), String.raw({raw:["a","b","c"]},undefined), String.raw({raw:["a","b","c"]},1), String.raw({raw:["a","b"]}), String.raw({raw:["a"]}), String.raw({raw:[]})]));
say("r2=" + String.raw({raw:["p","q","r"]}, "x", "y", "z", "w") + "|" + String.raw({raw:{0:"A",1:"B",length:2}}));
say("r3=" + (function(){try{return String.raw({})+"|no";}catch(e){return "RE";}})() + "|" + (function(){try{return String.raw(undefined)+"|no";}catch(e){return "RE";}})());
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "y1=true|false|true|true|false#y2=RE|false|true#w1=a1b2c|x|\\\\nQ_#w2=RE|1a23@1a2b3@123#r1=[\"abc\",\"aundefinedbc\",\"a1bc\",\"ab\",\"a\",\"\"]#r2=pxqyr|AB#r3=RE|RE"
        .into()
    )
  );
  OK
}

/// 67. 内建面扫面第十轮 · Object.groupBy / 空原型对象 / 数组字面量空位 (12.34):
///     `Object.groupBy` 段数与键口径 (Get 语义、键 String() 化、结果 `Object.create(null)`)、
///     null 原型对象无继承面 (`toString in c` 假、`c.toString` undefined、`c+""` 抛 TypeError)、
///     宿主 JS 函数过桥 `.length/.name` 合成面 (prelude 静态 `.length` 曾 BC 全 undefined)、
///     数组空位 `[1,,3]`/尾逗号/前导逗号 —— 全 node-EXACT (仅 `String(c)` 直转挂残档)
#[test]
fn test_internal_surface_sweep_ten_group_by_null_proto_and_elision() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
var g = Object.groupBy([1,2,3,4,5], function(x){ return x % 2 === 0 ? "even" : "odd"; });
say("h1=" + Object.keys(g).sort().join(",") + "|" + JSON.stringify(g.even) + "|" + JSON.stringify(g.odd) + "|" + Object.getPrototypeOf(g) + "|" + ("toString" in g));
var g2 = Object.groupBy(["a","bb","ccc"], function(s){ return s.length; });
say("h2=" + Object.keys(g2).sort().join(",") + "|" + JSON.stringify(g2["1"]) + "|" + JSON.stringify(g2["2"]) + "|" + JSON.stringify(g2["3"]) + "|" + String(g2["4"]) + "|" + JSON.stringify(Object.groupBy([1,2], function(){ return undefined; })));
say("h3=" + Object.groupBy.length + "|" + Object.assign.length + "|" + Object.hasOwn.length + "|" + String.raw.length + "|" + (function(){try{Object.groupBy(null,function(){});return "no";}catch(e){return "RE";}})() + "|" + (function(){try{Object.groupBy([1],"z");return "no";}catch(e){return "RE";}})() + "|" + (function(){try{Object.groupBy([1]);return "no";}catch(e){return "RE";}})());
var c = Object.create(null); c.a = 1;
say("h4=" + Object.keys(c).join(",") + "|" + Object.hasOwn(c,"a") + "|" + ("toString" in c) + "|" + String(c.toString) + "|" + Object.getPrototypeOf(c) + "|" + JSON.stringify([c.a, c.b]) + "|" + (function(){try{var z = c + "";return "no";}catch(e){return "RE";}})());
say("h5=" + [1,2,].length + "|" + [1,,].length + "|" + [,].length + "|" + JSON.stringify([1,,3]) + "|" + [1,,3].join("|") + "|" + [1,,3].indexOf(3) + "|" + JSON.stringify([].concat([1,,2])));
return out.join("#"); })();
  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "h1=even,odd|[2,4]|[1,3,5]|null|false#h2=1,2,3|[\"a\"]|[\"bb\"]|[\"ccc\"]|undefined|{\"undefined\":[1,2]}#h3=2|2|2|1|RE|RE|RE#h4=a|true|false|undefined|null|[1,null]|RE#h5=2|2|1|[1,null,3]|1||3|2|[1,null,2]"
        .into()
    )
  );
  OK
}

/// 68. 内建面扫面第十一轮 · RegExp 行为面 (12.35): exec 捕获组/lastIndex/index/input、
///     sticky y 锚定与推进、test() 的 g/y 副效应、gy 组合、toString/JSON.stringify 文本形
///     (链分裂修复: 引擎侧 regex_repr + 边界身份还原 + 自有 toString 遮蔽)、g 循环零宽推进、
///     replace $ 序列、split 捕获组拼入与空匹配规则 (regex_split_parts 一处定义两链共用)
#[test]
fn test_internal_surface_sweep_eleven_regexp_exec_sticky_split() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
var r = /a(b)c/g;
var m = r.exec("xxabcY");
say("k1=" + m.join(",") + "|" + r.lastIndex + "|" + m.index + "|" + m.input);
say("k2=" + String(r.exec("zz")) + "|" + r.lastIndex);
r.lastIndex = 0; say("k3=" + r.exec("aXXabcY").join(",") + "|" + r.lastIndex);
var y = /a+b/y; y.lastIndex = 2;
say("k4=" + y.exec("xxabb").join(",") + "|" + y.lastIndex);
say("k5=" + String(y.exec("ab")) + "|" + y.lastIndex);
var y3 = /b+/y; y3.lastIndex = 2;
say("k6=" + y3.exec("aabb").join(",") + "|" + y3.lastIndex);
var g1 = /a/g; say("k7=" + g1.test("ba") + "|" + g1.lastIndex + "|" + g1.test("ba") + "|" + g1.lastIndex);
var yy = /a/y; say("k8=" + yy.test("ba") + "|" + yy.lastIndex + "|" + yy.test("ab") + "|" + yy.lastIndex);
var p = /a/; say("k9=" + p.test("a") + "|" + p.lastIndex);
var g2 = /x/g; g2.lastIndex = 99; say("l1=" + String(g2.exec("abc")) + "|" + g2.lastIndex);
var gy = /a/gy; say("l2=" + gy.exec("aa").join(",") + "|" + gy.lastIndex + "|" + gy.exec("aa").join(",") + "|" + gy.lastIndex + "|" + String(gy.exec("aa")) + "|" + gy.lastIndex);
var ng = /z/; ng.lastIndex = 3; say("l3=" + String(ng.exec("zz")) + "|" + ng.lastIndex);
var t = /./g;
say("l4=" + t.toString() + "|" + (t + "") + "|" + String(t) + "|" + JSON.stringify(t) + "|" + JSON.stringify(/a(b)c/g));
say("l5=" + /a\/b/gi.toString() + "|" + /ab+c/.source + "|" + /q/i.flags + "|" + /u/y.sticky + "|" + /v/g.global);
var mm, mg = /a/g, ms = "bab", mr = [];
while ((mm = mg.exec(ms)) !== null) { mr.push(mm.index + ":" + mm[0].length); if (mg.lastIndex > 10) break; }
say("l6=" + mr.join(",") + "|" + mg.lastIndex);
say("l7=" + String(/b*/.exec("abc")).slice(0,3) + "|" + (/b*/.exec("abc") || { index: -1 }).index);
say("l8=" + String("abc".match(/q/)) + "|" + JSON.stringify("abc".match(/b/)) + "|" + /x/.test("y"));
say("m1=" + ("aXbXc".replace(/X/g, "Y")) + "|" + ("2024-05-01".replace(/(\d+)-(\d+)-(\d+)/, "$3/$2/$1")) + "|" + ("ab".replace(/(a)/, "$$1")) + "|" + ("abc".replace(/b/, "[$&]")));
function j(a) { return JSON.stringify(a); }
say("m2=" + j("a1b2".split(/(\d)/)) + "|" + j("abcd".split(/b*/)) + "|" + j("aba".split(/a/)) + "|" + j("".split(/x/)) + "|" + j("a1b2c3".split(/(\d)/, 2)) + "|" + j("x".split(/()/)));
var rg = /(\d)/g; var rt = "a1b".split(rg);
say("m3=" + rt.length + "|" + rg.lastIndex + "|" + j("ab".split(/a*/)) + "|" + j("aX".split(/[a-z]/)) + "|" + j("a1".split(/(\d)|(q)/)) + "|" + j("Te,st. ab".split(/[^a-zA-Z]+/, 3)) + "|" + j("a-b".split(/(?<hy>-)/)));
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "k1=abc,b|5|2|xxabcY#k2=null|0#k3=abc,b|6#k4=ab|4#k5=null|0#k6=bb|4#k7=true|2|false|0#k8=false|0|true|1#k9=true|0#l1=null|0#l2=a|1|a|2|null|0#l3=z|3#l4=/./g|/./g|/./g|{}|{}#l5=/a\\/b/gi|ab+c|i|true|true#l6=1:1|0#l7=|0#l8=null|[\"b\"]|false#m1=aYbYc|01/05/2024|$1b|a[b]c#m2=[\"a\",\"1\",\"b\",\"2\",\"\"]|[\"a\",\"c\",\"d\"]|[\"\",\"b\",\"\"]|[\"\"]|[\"a\",\"1\"]|[\"x\"]#m3=3|0|[\"\",\"b\"]|[\"\",\"X\"]|[\"a\",\"1\",null,\"\"]|[\"Te\",\"st\",\"ab\"]|[\"a\",\"-\",\"b\"]"
        .into()
    )
  );
  OK
}

/// 69. 内建面扫面第十二轮 · 正则对象枚举/原型面 (12.36): GON 仅 lastIndex (own_property_names
///     一处定义, 宿主 GON 单点两链共享)、keys 恒空、hasOwnProperty/propertyIsEnumerable 读臂
///     回落消 AST 链分裂、Object.hasOwn 走 prelude 级联、for-in 伪键泄漏 (引擎
///     is_regexp_pseudo_key 过滤, 标准正则/gy 组合交空)、字符串装箱枚举回归护栏
#[test]
fn test_internal_surface_sweep_twelve_regexp_enumeration_surface() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
var r = /a/g;
say("p1=" + JSON.stringify(Object.getOwnPropertyNames(r)) + "|" + JSON.stringify(Object.keys(r)));
say("p2=" + r.hasOwnProperty("lastIndex") + "|" + r.hasOwnProperty("source") + "|" + r.hasOwnProperty("toString") + "|" + r.hasOwnProperty("foo"));
say("p3=" + r.propertyIsEnumerable("lastIndex") + "|" + r.propertyIsEnumerable("source"));
say("p4=" + Object.hasOwn(r, "lastIndex") + "|" + Object.hasOwn(r, "global"));
say("p5=" + typeof r.hasOwnProperty + "|" + typeof r.propertyIsEnumerable);
var k1 = []; for (var p in /a/g) k1.push(p);
say("p6=" + JSON.stringify(k1.sort()));
var k2 = []; for (var q in /a/gy) k2.push(q);
say("p7=" + JSON.stringify(k2.sort()));
say("p8=" + JSON.stringify(Object.getOwnPropertyNames(/z/)));
var s2 = "x"; s2[0] = "y"; var k3 = []; for (var z in s2) k3.push(z);
say("p9=" + JSON.stringify(k3) + "|" + JSON.stringify(Object.keys(s2)));
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "p1=[\"lastIndex\"]|[]#p2=true|false|false|false#p3=false|false#p4=true|false#p5=function|function#p6=[]#p7=[]#p8=[\"lastIndex\"]#p9=[\"0\"]|[\"0\"]"
        .into()
    )
  );
  OK
}

/// 70. 内建面扫面第十三轮 · 正则 expando/方法遮蔽面 (12.37): RegExpData.props 自有袋 +
///     set 三路由 (lastIndex 字段/只读静默/入袋)、引擎 set/delete 护栏 (is_regexp_readonly_key
///     单点两链共读)、桥两向搬运 (host→engine props 落表; engine→host 按 `@@shadow:` 墓碑
///     区分代管形与用户遮蔽, exec 遮蔽可枚举可 hasOwn)、JSON.stringify 正则 expando 参与
///     (json_object_body 与 Object 臂一处定义)、delete expando 名册回收
#[test]
fn test_internal_surface_sweep_thirteen_regexp_expando_surface() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
var r = /a/g;
r.foo = 1;
say("p1=" + r.foo);
say("p2=" + JSON.stringify(Object.getOwnPropertyNames(r).sort()) + "|" + JSON.stringify(Object.keys(r)));
say("p3=" + r.hasOwnProperty("foo") + "|" + ("foo" in r) + "|" + Object.hasOwn(r, "foo"));
var k = []; for (var p in r) k.push(p);
say("p4=" + JSON.stringify(k));
var r2 = /b/; r2.source = "X"; r2.global = true;
say("p5=" + r2.source + "|" + r2.global);
var r3 = /c/; r3.exec = function () { return "shadow"; };
say("p6=" + r3.exec("cc") + "|" + r3.hasOwnProperty("exec"));
var k3 = []; for (var p3 in r3) k3.push(p3);
say("p7=" + JSON.stringify(k3));
var r5 = /e/g; r5.tag = "t"; r5.test("ee");
say("p8=" + r5.lastIndex + "|" + r5.tag);
var r6 = /f/; r6.bar = 2; var d = delete r6.bar;
say("p9=" + d + "|" + r6.bar + "|" + JSON.stringify(Object.getOwnPropertyNames(r6)));
var r7 = /g/g; r7.n = 7; var m = r7.exec("gg");
say("p10=" + m[0] + "|" + r7.n);
say("p11=" + JSON.stringify(r));
say("p12=" + JSON.stringify({k: /h/}));
say("p13=" + JSON.stringify(r, null, 1).replace(/\n/g, "|"));
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "p1=1#p2=[\"foo\",\"lastIndex\"]|[\"foo\"]#p3=true|true|true#p4=[\"foo\"]#p5=b|false#p6=shadow|true#p7=[\"exec\"]#p8=1|t#p9=true|undefined|[\"lastIndex\"]#p10=g|7#p11={\"foo\":1}#p12={\"k\":{}}#p13={| \"foo\": 1|}"
        .into()
    )
  );
  OK
}

/// 71. 内建面扫面第十四轮 · 解构族 (12.38): parser 侧暂态 Pattern + desugar 展开单点 (两链一份实现)、
///     嵌套数组/对象模式、默认值 (undefined 触发且非标识符源先快照)、数组 rest (slice) 与对象 rest
///     (Object.assign+逐个 delete)、空位/尾逗号、语句位 `[a]=b` 配平扫描判型、解构参数折 `$argN` 槽前言
///     (先于 SharedFunctionInfo 构造拼接, hoisted 统计见展开后形态)、右值一次性、空模式;
///     回归护栏: 普通索引语句 `[1,2][0]`、普通 var 多声明、if/for 单语句位展开包块
#[test]
fn test_internal_surface_sweep_fourteen_destructuring_family() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
var {x:[a,b], y:{c:d=7}} = {x:[1,2], y:{}}; say("t1=" + a + "|" + b + "|" + d);
var c2; [c2]=[1]; say("t2=" + c2);
var m1=1, [m2]=[2], m3=3; say("t3=" + (m1+m2+m3));
var h4, t4; [h4, ...t4] = [1,2,3]; say("t4=" + h4 + "|" + t4.join(","));
var {p5, ...o5} = {p5:1,q5:2,s5:3}; say("t5=" + JSON.stringify(o5));
var s0,s1; [s0,s1]='hi'; say("t6=" + s0+s1);
function f7({v}) { return v; } say("t7=" + f7({v:7}));
function f8([m, n=4]) { return m+n; } say("t8=" + f8([1]) + "|" + f8([1,2]));
var cnt=0; function mk(){ cnt=cnt+1; return [1,2]; } var [f1,f2]=mk(); say("t9=" + (f1+f2) + "|" + cnt);
var [] = [1,2]; var {} = {a1:1}; say("t10=ok");
var g1a, g3a; [g1a,,g3a,] = [1,2,3]; say("t11=" + (g1a+g3a));
var y12=1; [y12]=[2]; say("t12=" + y12);
say("t13=" + [1,2][0] + "|" + (function(){ var z1=1,z2=2; return z1+z2; })());
var w14; if (1) [w14] = [8]; say("t14=" + w14);
var acc=0; for (var i=0; i<2; i++) { var [v15] = [i]; acc = acc + v15; } say("t15=" + acc);
var q16 = (function({q}, [r]) { return q + "|" + r; })({q:3}, [4]); say("t16=" + q16);
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "t1=1|2|7#t2=1#t3=6#t4=1|2,3#t5={\"q5\":2,\"s5\":3}#t6=hi#t7=7#t8=5|3#t9=3|1#t10=ok#t11=4#t12=2#t13=1|3#t14=8#t15=1#t16=3|4"
        .into()
    )
  );
  OK
}

/// 72. 内建面扫面第十五轮 · 形参默认值与箭头解构形参 (12.39): 简单形参默认折 prelude
///     `var a = a===undefined ? fb : a` (undefined 触发、null 不触发、fallback 惰性求值、
///     可引用前序形参与 arguments)、箭头参数表并入 parse_param_table 单点 (解构/默认自动可用)、
///     arguments.length 用进帧快照 `frame_arguments_len` (修形参回写撑长链分裂)、
///     非可迭代/非方法源解构抛象两链一致 (错误文本名册另档)
#[test]
fn test_internal_surface_sweep_fifteen_param_defaults_and_arrow_params() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
function f1(a=1){ return a; } say("t1=" + f1() + "|" + f1(5) + "|" + f1(undefined));
function f2(a, b=a+1){ return b; } say("t2=" + f2(3) + "|" + String(f2()));
var c=0; function cx(){ c=c+1; return 9; } function f3(x=cx()){ return x; } say("t3=" + f3() + "|" + f3(2) + "|" + c);
function f4(a,b){ return arguments.length; } function f4d(a=1,b){ return arguments.length; } say("t4=" + f4d() + "|" + f4d(9) + "|" + f4());
function f5(v=3){ return v; } say("t5=" + f5(null) + "|" + f5(undefined) + "|" + f5(0));
say("t6=" + ((a=2)=>a)() + "|" + ((a=2)=>a)(5));
say("t7=" + (({v})=>v)({v:8}) + "|" + (([a,b])=>a+b)([1,2]) + "|" + (({x=5})=>x)({}));
function f8(a, b=arguments.length){ return String(a) + "/" + b; } say("t8=" + f8() + "|" + f8(7));
var f9 = function ({p, q=2}, [r=3]) { return p+q+r; }; say("t9=" + f9({p:1}, []));
function f10(a=[1,2]) { return a.join("-"); } var t10 = "no"; try { f10("xy"); } catch (e) { t10 = "yes"; } say("t10=" + f10() + "|" + t10);
function f11(a, [y=a]) { return String(y); } var t11 = "no"; try { t11 = f11(1); } catch (e) { t11 = "throw"; } say("t11=" + t11);
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "t1=1|5|1#t2=4|NaN#t3=9|2|1#t4=0|1|0#t5=null|3|0#t6=2|5#t7=8|3|5#t8=undefined/0|7/1#t9=6#t10=1-2|yes#t11=throw"
        .into()
    )
  );
  OK
}

/// 73. 内建面扫面第十六轮 · rest 形参 (12.40): SharedFunctionInfo.rest_param 元数据单点
///     (绑定名不占按位 params, 起点 = 按位形参数), AST 进帧 ActivationScopeInit.rest_param
///     绑实参尾段快照 / BC 生成器专属局部寄存器 + 分发器进帧写入 (零新字节码);
///     箭头吃自身调用实参 (与词法 arguments 互不干扰)、new 构造、解构/默认形参混排、
///     arguments 并存、闭包捕获与重赋值、同名 var 重声明、spread 喂 rest、`...r = x` 语法错
#[test]
fn test_internal_surface_sweep_sixteen_rest_parameters() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
function f1(a, ...r) { return a + "|" + r.length + "|" + r.join(","); }
say("t1=" + f1(1, 2, 3) + "#" + f1(1) + "#" + f1());
function f2(x = 5, y = 6, ...r) { return x + "|" + y + "|" + JSON.stringify(r); }
say("t2=" + f2() + "#" + f2(9) + "#" + f2(9, 9, 1, 2));
function f3(a, b = a * 2, ...r) { return a + "|" + b + "|" + r.length; }
say("t3=" + f3(3, 4, 5, 6) + "#" + f3(3));
say("t4=" + ((a, ...r) => a + "|" + r.join("-"))(1, 2, 3) + "#" + ((...z) => z.length)() + "#" + ((...z) => JSON.stringify(z))(7, 8));
function f5({ a }, [b], ...r) { return a + "|" + b + "|" + r.length; }
var o5 = { m(x, ...y) { return x + "|" + y.length; } };
say("t5=" + f5({ a: 1 }, [2], 3, 4) + "#" + o5.m(1, 2, 3));
function f6(a, ...r) { return arguments.length + "|" + r.length + "|" + arguments[2]; }
say("t6=" + f6(1, 2, 3, 4));
function sum3(a, b, c) { return a + b + c; }
function f7(...args) { return sum3(...args); }
say("t7=" + f7(1, 2, 3) + "#" + ((...a) => a.reduce((x, y) => x + y, 0))(1, 2, ...[3, 4]));
function f8(...z) { function inner() { return z.length; } var l1 = inner(); z.push(9); var l2 = inner(); return l1 + "|" + l2; }
say("t8=" + f8(1, 2));
function f9(...z) { z = [9, 9]; return z.join("+"); }
function f9b(...z) { var z = 5; return z; }
say("t9=" + f9(1) + "#" + f9b(1, 2));
function F(...a) { this.n = a.length; }
function f10(...a) { return a.length + "|" + String(a[2]); }
say("t10=" + new F(1, 2, 3).n + "#" + f10(1, 2));
function f11(...a) { return JSON.stringify(a); }
say("t11=" + f11(undefined, 1, "x"));
var o12 = { m(x = 2, ...y) { return x + "|" + y.length; } };
say("t12=" + o12.m(undefined, 1, 2));
function f13(x, ...r1) { return ((y, ...r2) => r1.length + "|" + r2.length + "|" + y)(5, 6, 7); }
say("t13=" + f13(1));
function f14(...z) { return (function (z) { return z; })(7) + "|" + z.length; }
function f14b(...z) { var g = function () { z = "R"; }; g(); return z; }
say("t14=" + f14(1) + "#" + f14b(1));
function f15(...a) { return a.map(function (...b) { return b[0]; }).join(","); }
say("t15=" + f15(3, 4, 5));
function f16({ a }, b = a + 1, ...r) { return a + b + "|" + r.length; }
say("t16=" + f16({ a: 1 }, 2, 3) + "#" + f16({ a: 4 }));
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "t1=1|2|2,3#1|0|#undefined|0|#t2=5|6|[]#9|6|[]#9|9|[1,2]#t3=3|4|2#3|6|0#t4=1|2-3#0#[7,8]#t5=1|2|2#1|2#t6=4|3|3#t7=6#10#t8=2|3#t9=9+9#5#t10=3#2|undefined#t11=[null,1,\"x\"]#t12=2|2#t13=0|2|5#t14=7|1#R#t15=3,4,5#t16=3|1#9|0"
        .into()
    )
  );
  OK
}

/// 74. 内建面扫面第十七轮 · for-of 迭代语句 (12.41): parser 摊成 `$fsrc/$fidx` 三段式索引
///     循环 (长度活读, 与 node Array Iterator 索引语义对齐), 声明/解构/成员/下标四类头部;
///     同轮修 BC 引擎 var 提升预绑槽 (循环头临时寄存器与体内晚绑 var 冲撞, f14 链分裂根因)。
///     两链一致的 node 偏差挂账: f2 块级作用域泄漏 (11.6 系统性)、f11 let≡var 逐轮捕获
///     (11.6)、f12/f15 非可迭代 RHS 静默零轮 (迭代协程协议 · 战役 C)
#[test]
fn test_internal_surface_sweep_seventeen_for_of() -> Void {
  let res = both_chains(
    r##"
(function () { var out = [];
var a1 = [1,2,3];
var s = 0;
for (var x of a1) s = s*10 + x;
out.push("f1=" + s + "|" + x);
for (let y of [10,20]) { s; }
out.push("f2=" + (typeof y));
var acc = "";
for (const c of "ab") acc += c + ",";
out.push("f3=" + acc);
var p = 0;
for (p of [1,2,3]) {}
out.push("f4=" + p);
var pairs = [[1,2],[3,4]];
var d = "";
for (const [ka, kb] of pairs) d += ka + ":" + kb + ";";
out.push("f5=" + d);
var e = "";
for (const [h, ...t] of [[1,2,3],[4,5]]) e += h + "[" + t.join("|") + "]";
out.push("f6=" + e);
var sum = 0;
for (const n of [1,2,3,4]) { if (n === 2) continue; if (n === 4) break; sum += n; }
out.push("f7=" + sum);
var sp = [1,,3];
var f8 = "";
for (const v of sp) f8 += String(v) + "/";
out.push("f8=" + f8);
var g9 = [1,2];
var c9 = 0;
for (const v of g9) { c9++; if (c9 === 2) g9.push(3); if (c9 > 5) break; }
out.push("f9=" + c9);
function fa() { var s10 = ""; for (const q of arguments) s10 += q; return s10; }
out.push("f10=" + fa("a","b"));
var fns = [];
for (let i of [1,2,3]) fns.push(() => i);
out.push("f11=" + fns.map(f => f()).join(","));
try { for (const v of 5) {} out.push("f12=no-throw"); } catch (e12) { out.push("f12=threw"); }
var c13 = 0;
for (const v of []) c13++;
for (const v of "") c13++;
out.push("f13=" + c13);
out.push("f14=" + (function(){ var r=""; for (var w of "xy") r+=w; return r+w; })());
try { for (const v of {}) {} out.push("f15=no-throw"); } catch (e15) { out.push("f15=threw"); }
var q16 = 0;
for (q16 of [1,2]) {}
out.push("f16=" + q16);
var o19 = {};
for (o19.p of [7,8]) {}
out.push("f19=" + o19.p);
var of = [7,8];
var t17 = "";
for (var v17 of of) t17 += v17;
for (v17 of of) t17 += v17;
out.push("f17=" + t17);
out.push("f18=" + (function(){ var c=0; var w=0; for(;c<3;){ w=c*100; c=c+1; } return w; })());
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "f1=123|3#f2=number#f3=a,b,#f4=3#f5=1:2;3:4;#f6=1[2|3]4[5]#f7=4#f8=1/undefined/3/#f9=3#f10=ab#f11=3,3,3#f12=no-throw#f13=0#f14=xyy#f15=no-throw#f16=2#f19=8#f17=7878#f18=200"
        .into()
    )
  );
  OK
}

/// 75. 标记语句第十八轮 (12.44): `L: for/while/do/for-in/for-of/switch/block/if` 全目标形态、
///     `A: B: for` 双标签同指、`break L` 指任意带标签语句、`continue L` 仅迭代语句、
///     裸 break 穿透标记块找最近循环、标记循环体内函数调用边界、break L 穿出 try-finally 先跑收尾;
///     解析期校验 (未定义标签/越界裸 break-continue = SyntaxError) 与原版 parser 解析 target 同口径
#[test]
fn test_labeled_statements_and_scoped_jumps() -> Void {
  let res = both_chains(
    r##"
(function () { var out = [];
outer1: for (var i=0;i<2;i++){ for (var j=0;j<2;j++){ if (j===1) break outer1; } out.push("i"+i); }
outer2: for (var i=0;i<2;i++){ for (var j=0;j<3;j++){ if (j===1) continue outer2; } out.push("never"); }
L3: { out.push("in-block"); break L3; out.push("skipped"); }
L4: do { out.push("in-do"); break L4; } while (true);
L5: if (true) { for (var k=0;k<3;k++){ if (k===0) break L5; out.push("never2"); } out.push("for-end"); }
out.push("after-if");
A6: B6: for (var i=0;i<2;i++){ if (i===1) continue A6; out.push("ab"+i); }
C7: D7: for (var i=0;i<1;i++){ break D7; out.push("never3"); } out.push("after-dd");
L8: switch (1) { case 1: out.push("sw"); break L8; }
out.push("post-sw");
L9: for (var i=0;i<3;i++){ for (var j=0;j<3;j++){ if (j===1) break; } out.push("b9-"+i); }
L10: for (const v of [1,2,3]) { if (v===2) break L10; out.push("of"+v); }
L11: for (var i=0;i<2;i++){ switch(i){ case 1: out.push("last"); break; default: out.push("def"+i); continue L11; } out.push("post"+i); }
for (var i=0;i<3;i++){ L12: { if (i===1) break L12; out.push("b12-"+i); } out.push("tail"+i); }
for (var i=0;i<1;i++){ out.push("fnlabel"+(function(){ return 1; })()); }
for (var i=0;i<1;i++){ L15: { break; } out.push("bare-after-block"); }
L16: for (var i=0;i<2;i++){ (function(){ out.push("fwd"+i); })(); if (i===0) continue L16; out.push("keep"+i); }
var w=0; L17: while (true) { w++; if (w>2) break L17; } out.push("w"+w);
var o20 = {a:1, b:2};
var acc20 = [];
L20: for (var p20 in o20) { acc20.push("fi"+p20); continue L20; acc20.push("neverfi"); }
L21: for (var p21 in o20) { break L21; acc20.push("never21"); }
out.push("o20=" + acc20.sort().join(","));
var c22 = 0; L22: do { c22++; if (c22 < 3) continue L22; } while (c22 < 3); out.push("ddw"+c22);
outer23: for (var i=0;i<3;i++){ try { out.push("t"+i); if (i===1) break outer23; } finally { out.push("f"+i); } }
return out.join("#"); })();
    "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "in-block#in-do#after-if#ab0#after-dd#sw#post-sw#b9-0#b9-1#b9-2#of1#def0#last#post1#b12-0#tail0#tail1#b12-2#tail2#fnlabel1#fwd0#fwd1#keep1#w3#o20=fia,fib#ddw3#t0#f0#t1#f1"
        .into()
    )
  );
  OK
}

/// 76. 标记语句解析期校验 (75 的错误面): 未定义标签、裸 break/continue 越界、
///     continue L 指非迭代语句、嵌套函数体内 break 指外层标签 (词法边界) ——
///     措辞逐条对齐 node V8 同版本报错
#[test]
fn test_label_statement_syntax_errors() -> Void {
  let cases: [(&str, &str); 7] = [
    (
      "for (var i=0;i<3;i++){ break nosuch; }",
      "Undefined label 'nosuch'",
    ),
    ("var x = 1; break;", "Illegal break statement"),
    (
      "if(1){ continue; }",
      "Illegal continue statement: no surrounding iteration statement",
    ),
    (
      "L: { x(); continue L; }",
      "Illegal continue statement: 'L' does not denote an iteration statement",
    ),
    ("for(;;){ continue zz; }", "Undefined label 'zz'"),
    (
      "for (var i=0;i<1;i++){ (function(){ break i; })(); }",
      "Undefined label 'i'",
    ),
    ("break outermost;", "Undefined label 'outermost'"),
  ];
  for (src, msg) in cases {
    let err = both_chains(src).expect_err("应为解析期 SyntaxError");
    let text = format!("{err}");
    assert!(text.contains(msg), "{src} => {text} 不含 {msg}");
  }
  OK
}

/// 77. Symbol 值跨桥 (12.48 战役 C-2a): 引擎补 `Value::Symbol` 变体 + 桥两向不再降级
///     Undefined; 属性键走 `@@symbol:` 前缀编码 (宿主 `element_key` 与引擎 `to_element_key`
///     一处定义)。身份模型 = 按描述串相等 (node 唯一 id 语义挂账), 用例避开两条依赖:
///     同描述 `Symbol("k")===Symbol("k")` (node false, 本模型 true) 与 `"@@symbol:k"` 字符串键别名
#[test]
fn test_symbol_value_bridge() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
var s1 = Symbol("k");
say("t1=" + typeof s1);
say("t2=" + String(s1));
say("t3=" + (s1 === s1));
say("t4=" + String(Symbol()));
var o = {};
o[s1] = 7;
o["Symbol(k)"] = 8;
say("t5=" + o[s1] + "|" + o["Symbol(k)"]);
say("t6=" + (s1 === "k"));
var s2 = Symbol("m");
o[s2] = 9;
say("t7=" + o[s1] + "|" + o[s2]);
say("t8=" + delete o[s1] + "|" + o[s1]);
var ks = []; for (var p in o) ks.push(p);
say("t9=" + JSON.stringify(ks));
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "t1=symbol#t2=Symbol(k)#t3=true#t4=Symbol()#t5=7|8#t6=false#t7=7|9#t8=true|undefined#t9=[\"Symbol(k)\"]"
        .into()
    )
  );
  OK
}

/// 78. 内建函数值的加法强转 (链分裂修复): `"" + f` 走 ToPrimitive —— 此前 AST 链在
///     Native (无原型面物化) 接收者上找不到 valueOf/toString 直接抛
///     "Cannot convert object to primitive value", BC 链则漏到继承的
///     Object.prototype.toString 吐 "[object Function]", 两链分叉。修复 = 两链各自
///     收口到与 `String(f)` 同一份摘要 (引擎 Native 臂自合成本机 toString, AST 回环
///     耗尽后走 `to_js_string` 兜底)。node 给具名形 `function toString() { [native code] }`
///     —— 内建函数名物化属 12.27 挂账的 Function.prototype 面工程, 本钉只锁两链一致
#[test]
fn test_builtin_function_toprimitive_add() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
var f = /m/i.toString;
say("t1=" + typeof f);
say("t2=" + String(f));
say("t3=" + ("" + f));
say("t4=" + (f + ""));
var g = [].push;
say("t5=" + (g + ""));
say("t6=" + (/m/i + "") + "|" + ("" + /m/i));
say("t7=" + (f + 1));
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "t1=function#t2=function () { [native code] }#t3=function () { [native code] }\
       #t4=function () { [native code] }#t5=function () { [native code] }\
       #t6=/m/i|/m/i#t7=function () { [native code] }1"
        .into()
    )
  );
  OK
}

/// 79. 数组洞 (TheHole) 表示面 (战役 C ③ 前半): 字面量空位/`new Array(n)`/delete 产洞,
///     读面物化 undefined、`in`/hasOwn/keys/for-in/gOPN/entries 洞即不存在、JSON 吐 null、
///     join/toString 洞为空段、写 undefined 补洞后存在
#[test]
fn test_array_hole_representation() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
var a = [1,,2];
say("t1=" + a.length + "|" + a[1] + "|" + (1 in a) + "|" + a.hasOwnProperty(1));
say("t2=" + JSON.stringify(Object.keys(a)) + "|" + JSON.stringify(Object.getOwnPropertyNames(a)));
var k = []; for (var p in a) k.push(p);
say("t3=" + JSON.stringify(k));
say("t4=" + JSON.stringify(a) + "|" + String(a));
var b = new Array(3);
say("t5=" + b.length + "|" + (0 in b) + "|" + JSON.stringify(Object.keys(b)));
b[1] = 5;
say("t6=" + JSON.stringify(b) + "|" + (0 in b) + "|" + (2 in b));
var c = [1,2,3]; delete c[1];
say("t7=" + JSON.stringify(c) + "|" + (1 in c) + "|" + c[1]);
say("t8=" + (a[1] === undefined) + "|" + (a[1] === a[1]) + "|" + (a[1] + 1));
var d = [1,,];
say("t9=" + d.length + "|" + (1 in d));
var e = [,1];
say("t10=" + e.length + "|" + (0 in e) + "|" + JSON.stringify(e));
var g = [1,,2]; g[1] = undefined;
say("t11=" + (1 in g) + "|" + JSON.stringify(g));
say("t12=" + Object.entries(a).length);
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "t1=3|undefined|false|false#t2=[\"0\",\"2\"]|[\"0\",\"2\",\"length\"]#t3=[\"0\",\"2\"]#t4=[1,null,2]|1,,2#t5=3|false|[]#t6=[null,5,null]|false|false#t7=[1,null,3]|false|undefined#t8=true|true|NaN#t9=2|false#t10=2|false|[null,1]#t11=true|[1,null,2]#t12=2"
        .into()
    )
  );
  OK
}

/// 80. 数组洞 (TheHole) 行为面 (战役 C ③ 后半): map/forEach/filter/reduce/every/some/find 跳洞、
///     includes 洞当 undefined、indexOf/lastIndexOf 跳洞、spread/Array.from 落密、
///     concat/slice 保洞、sort 洞沉底、reverse/copyWithin 洞照搬、join 洞为空段
#[test]
fn test_array_hole_builtin_behavior() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
var a = [1,,2];
say("t1=" + JSON.stringify([,...a]) + "|" + JSON.stringify(a.concat([])) + "|" + JSON.stringify(a.slice(0,3)));
var cnt = 0; a.map(function(){cnt = cnt+1;}); a.forEach(function(){cnt=cnt+10;});
say("t2=" + cnt);
say("t3=" + a.includes(undefined) + "|" + a.indexOf(undefined));
say("t4=" + Array.from(a).length + "|" + JSON.stringify(Array.from(a)) + "|" + JSON.stringify([...a]));
say("t5=" + a.filter(function(){return true;}).length + "|" + a.reduce(function(x,y){return x+y;}));
var f = [1,2,,3];
say("t6=" + JSON.stringify(f.copyWithin(0,2)) + "|" + (2 in f));
var g = [1,,2];
say("t7=" + JSON.stringify(g.sort()) + "|" + JSON.stringify([3,,1].sort()));
say("t8=" + JSON.stringify([3,,1].reverse()) + "|" + (1 in [3,,1].reverse()));
say("t9=" + [3,,1].join("|"));
say("t10=" + a.every(function(){return true;}) + "|" + a.some(function(){return true;}) + "|" + a.find(function(){return true;}));
say("t11=" + JSON.stringify(a.map(function(){return 9;})) + "|" + (1 in a.map(function(){return 9;})));
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "t1=[null,1,null,2]|[1,null,2]|[1,null,2]#t2=22#t3=true|-1#t4=3|[1,null,2]|[1,null,2]#t5=2|3#t6=[null,3,null,3]|false#t7=[1,2,null]|[1,3,null]#t8=[1,null,3]|false#t9=3||1#t10=true|true|1#t11=[9,null,9]|false"
        .into()
    )
  );
  OK
}

/// 81. 正则字面量 per-eval 新实例 × 编译缓存 (战役 C-④a): 同字面量两跳不等、lastIndex 不跨实例、
///     绑定内复用恒等、new RegExp 不等、exec 推进照常、坏 pattern 每跳照抛、source/flags 文本形对齐
#[test]
fn test_regexp_literal_per_eval_instance() -> Void {
  let res = both_chains(
    r##"
(function () { var out = []; var n = 0; function say(s) { out[n] = s; n = n + 1; }
function f() { return /a/g; }
say("t1=" + String(f() === f()));
var a = f(); var b = f(); a.lastIndex = 2;
say("t2=" + String(b.lastIndex));
say("t3=" + String((/a/) === (/a/)));
var r = /a/g;
say("t4=" + String(r === r));
say("t5=" + String(new RegExp("a") === new RegExp("a")));
var g = /ab/g;
say("t6=" + String(g.exec("xab")) + "|" + String(g.lastIndex) + "|" + String(g.exec("xab")));
try { new RegExp("["); say("t7=nothrow"); } catch (e) { say("t7=throw " + String(e.name)); }
say("t8=" + (/a/).source + "|" + String(/a/.global === /a/.global));
return out.join("#"); })();  "##,
  )?;
  assert_eq!(
    res,
    JsValue::String(
      "t1=false#t2=0#t3=false#t4=true#t5=false#t6=ab|3|null#t7=throw Error#t8=a|true"
        .into()
    )
  );
  OK
}
