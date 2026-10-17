//! 战役 C-2b 第二刀钉: 引擎 `Value::RegExp` 变体的属性面 / 身份 / 原型链口径
//! (物化哨兵机器退役后, 这些行为从"对象表 + 名册过滤"改为"内核直读 + expando 袋")

use std::sync::Arc;

use v8_interpreter::{NativeFn, RegExpCore, Value};

fn re(pattern: &str, flags: &str) -> Value {
  let core = Arc::new(RegExpCore::new(pattern, flags).expect("compile"));
  Value::new_regexp(core)
}

fn core_of(v: &Value) -> Arc<RegExpCore> {
  match v {
    Value::RegExp(core, _) => Arc::clone(core),
    other => panic!("期望 RegExp 变体, 实得 {other:?}"),
  }
}

fn native_of(v: &Value) -> &NativeFn {
  match v {
    Value::Native(nf, _) => nf,
    other => panic!("期望 Native 方法, 实得 {other:?}"),
  }
}

#[test]
fn regexp_variant_property_surface() {
  let r = re("a(b)c", "gi");
  // 内核派生面直读
  assert_eq!(&*r.get_property("source").unwrap().to_str_lossy(), "a(b)c");
  assert_eq!(&*r.get_property("flags").unwrap().to_str_lossy(), "gi");
  assert!(matches!(
    r.get_property("global").unwrap(),
    Value::Boolean(true)
  ));
  assert!(matches!(
    r.get_property("ignoreCase").unwrap(),
    Value::Boolean(true)
  ));
  assert!(matches!(
    r.get_property("multiline").unwrap(),
    Value::Boolean(false)
  ));
  assert!(matches!(
    r.get_property("sticky").unwrap(),
    Value::Boolean(false)
  ));
  // lastIndex 专存内核: 写内核, 读内核
  r.set_property("lastIndex", Value::Number(4.0)).unwrap();
  assert_eq!(*core_of(&r).last_index.read(), 4);
  assert!(matches!(
    r.get_property("lastIndex").unwrap(),
    Value::Number(n) if n == 4.0
  ));
  // 只读访问器赋值静默忽略 (名册与宿主共读)
  r.set_property("source", Value::new_string("X")).unwrap();
  assert_eq!(&*r.get_property("source").unwrap().to_str_lossy(), "a(b)c");
  // 文本面与 typeof/toBoolean/toNumber
  assert_eq!(&*r.to_str_lossy(), "/a(b)c/gi");
  assert_eq!(r.type_of(), "object");
  assert!(r.to_boolean());
  assert!(r.to_number().is_nan());
}

#[test]
fn regexp_expando_and_shadow_beacon() {
  let r = re("m", "i");
  r.set_property("foo", Value::Number(7.0)).unwrap();
  assert!(matches!(
    r.get_property("foo").unwrap(),
    Value::Number(n) if n == 7.0
  ));
  // 同名赋值 exec = 用户遮蔽 (node e10): 袋先于内建面
  let shadow = Value::new_native(NativeFn::Const(Box::new(Value::new_string("shadowed"))));
  r.set_property("exec", shadow.clone()).unwrap();
  let got = r.get_property("exec").unwrap();
  assert_eq!(got, shadow);
  // delete 只回收袋键; 派生名删不动但交 true (sloppy)
  assert!(r.delete_property("foo"));
  assert!(matches!(r.get_property("foo").unwrap(), Value::Undefined));
  assert!(r.delete_property("source"));
  assert_eq!(&*r.get_property("source").unwrap().to_str_lossy(), "m");
  // hasOwnProperty 自有册 = lastIndex 恒自有 + 袋 (与宿主 own_property_names 同册);
  // propertyIsEnumerable 只认袋 (lastIndex 自有但不可枚举, node p2/p3 口径)
  let hop = r.get_property("hasOwnProperty").unwrap();
  let own = native_of(&hop)
    .call(&r, &[Value::new_string("exec")])
    .unwrap();
  assert!(matches!(own, Value::Boolean(true)));
  let not_own = native_of(&hop)
    .call(&r, &[Value::new_string("source")])
    .unwrap();
  assert!(matches!(not_own, Value::Boolean(false)));
  let li_own = native_of(&hop)
    .call(&r, &[Value::new_string("lastIndex")])
    .unwrap();
  assert!(matches!(li_own, Value::Boolean(true)));
  let penum = r.get_property("propertyIsEnumerable").unwrap();
  let li_enum = native_of(&penum)
    .call(&r, &[Value::new_string("lastIndex")])
    .unwrap();
  assert!(matches!(li_enum, Value::Boolean(false)));
  let foo_enum = native_of(&penum)
    .call(&r, &[Value::new_string("foo")])
    .unwrap();
  assert!(matches!(foo_enum, Value::Boolean(false)));
}

#[test]
fn regexp_identity_and_lastindex_shared_across_hops() {
  let r = re("a", "g");
  // 同内核浅拷贝相等 (跨桥身份); 独立内核不等 (node /a/ !== /a/)
  let twin = Value::RegExp(core_of(&r), Default::default());
  assert!(r.test_equal_strict(&twin));
  assert!(!r.test_equal_strict(&re("a", "g")));
  // exec 经内建面走共享内核: g 标志推进 lastIndex, 两个视图读回同值
  let exec = r.get_property("exec").unwrap();
  native_of(&exec)
    .call(&Value::Undefined, &[Value::new_string("xaa")])
    .unwrap();
  assert_eq!(*core_of(&r).last_index.read(), 2);
  assert!(matches!(
    twin.get_property("lastIndex").unwrap(),
    Value::Number(n) if n == 2.0
  ));
  let miss = native_of(&exec)
    .call(&Value::Undefined, &[Value::new_string("zzz")])
    .unwrap();
  assert!(matches!(miss, Value::Null));
  assert_eq!(*core_of(&r).last_index.read(), 0);
}

#[test]
fn regexp_test_delegates_exec_and_string_methods_use_variant() {
  let r = re("b", "y");
  r.set_property("lastIndex", Value::Number(1.0)).unwrap();
  let test = r.get_property("test").unwrap();
  let hit = native_of(&test)
    .call(&Value::Undefined, &[Value::new_string("abc")])
    .unwrap();
  assert!(matches!(hit, Value::Boolean(true)));
  // sticky 锚定推进后复位口径与 exec 同源 (一处定义)
  assert_eq!(*core_of(&r).last_index.read(), 2);
  // 再 test: 锚点停在 'c' 处不再命中 → 复位 0 (global/sticky 才读写 lastIndex)
  let miss = native_of(&test)
    .call(&Value::Undefined, &[Value::new_string("abc")])
    .unwrap();
  assert!(matches!(miss, Value::Boolean(false)));
  assert_eq!(*core_of(&r).last_index.read(), 0);
  // String.prototype.split 的正则分隔符走内核快路径 (变体直判, 不再经 @@split 委托)
  let parts = Value::string_split_inlined("a1b2c3", &[re("[12]", ""), Value::Undefined]).unwrap();
  assert_eq!(&*parts.to_str_lossy(), "a,b,c3");
  let replaced =
    Value::string_replace_inlined("xax", &[re("a", "g"), Value::new_string("Z")]).unwrap();
  assert_eq!(&*replaced.to_str_lossy(), "xZx");
}

#[test]
fn regexp_error_paths_stay_non_panic() {
  // 未闭合模式: 内核编译错误经 Result 上交, 不 panic
  assert!(RegExpCore::new("(", "").is_err());
  assert!(RegExpCore::new("[a", "").is_err());
}
