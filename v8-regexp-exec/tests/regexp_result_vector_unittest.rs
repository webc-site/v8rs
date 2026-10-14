//! ResultVectorScope 与 ResultVector 规约测试 (v8-regexp-exec)。

use aok::{OK, Void};
use v8_regexp_exec::{
  regexp_result_vector::{ResultVector, ResultVectorScope},
  regexp_utils::IsolateMarker,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn result_vector_scope_lifecycle() -> Void {
  let isolate = IsolateMarker;
  let mut scope = ResultVectorScope::new(&isolate);
  assert!(scope.value().is_none());

  let slice = scope.initialize(16);
  assert!(slice.is_empty());

  let scope_sized = ResultVectorScope::with_size(&isolate, 32);
  assert!(scope_sized.value().is_none());

  let ptr = ResultVector::allocate(&isolate, 64);
  assert!(ptr.is_null());
  ResultVector::free(&isolate, ptr);

  OK
}
