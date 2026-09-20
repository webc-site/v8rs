// 复刻 v8/src/heap/base/unsafe-json-emitter.cc 的 .cc 侧行为
// 头文件模板成员 p() 的多属性逗号逻辑未转译（见 TODO.md 已知缺口），此处只测 .cc 方法序列
use std::{cell::RefCell, rc::Rc};

use aok::{OK, Void};
// 同名符号可能被多个 TU 重复产出，走精确模块路径避免 glob 歧义
use v8rs::v8::heap_base_unsafe_json_emitter::{
  heap_base_UnsafeJsonEmitter, heap_base_UnsafeJsonEmitterImpl,
};
use v8rs::{AsPointer, Ptr};

fn emit(seq: impl FnOnce(Ptr<heap_base_UnsafeJsonEmitter>)) -> Vec<u8> {
  let em = Rc::new(RefCell::new(heap_base_UnsafeJsonEmitter::default()));
  let p = em.as_pointer();
  seq(p.clone());
  p.ToString()
}

#[test]
fn object_with_bool_property() -> Void {
  let out = emit(|p| {
    p.object_start();
    p.emit_property_name(Ptr::<u8>::from_string_literal(b"a"));
    p.emit_value_bool(true);
    p.object_end();
  });
  assert_eq!(out, br#"{"a":true}"#.to_vec());
  OK
}

#[test]
fn object_with_string_property() -> Void {
  let out = emit(|p| {
    p.object_start();
    p.emit_property_name(Ptr::<u8>::from_string_literal(b"k"));
    p.emit_value_Ptru8(Ptr::<u8>::from_string_literal(b"v"));
    p.object_end();
  });
  assert_eq!(out, br#"{"k":"v"}"#.to_vec());
  OK
}
