//! unsafe-json-emitter API 语义测试 (原版无 gtest 用例, 按输出格式规约补)。
use aok::{OK, Void};
use v8_heap_base::UnsafeJsonEmitter;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// 对象: 属性间逗号分隔, "name":value (原 p/object_start/object_end 组合)
#[test]
fn object_format() -> Void {
  let out = UnsafeJsonEmitter::new()
    .object_start()
    .p("a", true)
    .p("b", 42)
    .p("c", 1.5)
    .p("d", "x")
    .object_end()
    .to_string();
  assert_eq!(out, r#"{"a":true,"b":42,"c":1.5,"d":"x"}"#);
  OK
}

// 空对象: 无属性无逗号
#[test]
fn empty_object() -> Void {
  let out = UnsafeJsonEmitter::new()
    .object_start()
    .object_end()
    .to_string();
  assert_eq!(out, "{}");
  OK
}

// 布尔/整型各宽度: true/false 小写, 整数十进制 (含负数与 u64 上限)
#[test]
fn value_types() -> Void {
  let out = UnsafeJsonEmitter::new()
    .object_start()
    .p("t", true)
    .p("f", false)
    .p("i8", -128i8)
    .p("i64", i64::MIN)
    .p("u8", 255u8)
    .p("u64", u64::MAX)
    .p("usize", 0usize)
    .object_end()
    .to_string();
  assert_eq!(
    out,
    r#"{"t":true,"f":false,"i8":-128,"i64":-9223372036854775808,"u8":255,"u64":18446744073709551615,"usize":0}"#
  );
  OK
}

// 字符串值与 String 引号包裹; 浮点整数形式省尾零
#[test]
fn string_and_float() -> Void {
  let mut e = UnsafeJsonEmitter::new();
  e.object_start()
    .p("s", "hello")
    .p("owned", String::from("world"))
    .p("f", 2.0_f64)
    .object_end();
  assert_eq!(e.to_string(), r#"{"s":"hello","owned":"world","f":2}"#);
  OK
}

// object_end 后 first_ 保持 (原版 1:1): 重开对象需重新 object_start
#[test]
fn restart_object() -> Void {
  let mut e = UnsafeJsonEmitter::new();
  e.object_start().p("a", 1).object_end();
  e.object_start().p("b", 2).object_end();
  assert_eq!(e.to_string(), r#"{"a":1}{"b":2}"#);
  OK
}
