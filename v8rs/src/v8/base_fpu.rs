use std::{
  cell::RefCell,
  collections::BTreeMap,
  io::{Read, Seek, Write, prelude::*},
  os::fd::AsFd,
  rc::{Rc, Weak},
};

use crate::*;
#[derive(Default)]
pub struct v8_base_FPU {}
impl Clone for v8_base_FPU {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_FPU> = Rc::new(RefCell::new(Self {}));
    let this: Ptr<v8_base_FPU> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_FPU {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {}
  fn from_bytes(buf: &[u8]) -> Self {
    Self {}
  }
}
#[derive(Default)]
pub struct v8_base_FlushDenormalsScope {
  old_flush_state_: Value<bool>,
}
impl v8_base_FlushDenormalsScope {
  pub fn v8_base_FlushDenormalsScope(value: bool) -> Self {
    let value: Value<bool> = Rc::new(RefCell::new(value));
    let __this: Value<v8_base_FlushDenormalsScope> = Rc::new(RefCell::new(Self {
      old_flush_state_: Rc::new(RefCell::new(({ v8_base_FPU::GetFlushDenormals() }))),
    }));
    let this: Ptr<v8_base_FlushDenormalsScope> = __this.as_pointer();
    ({ v8_base_FPU::SetFlushDenormals((*value.borrow())) });
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl Clone for v8_base_FlushDenormalsScope {
  fn clone(&self) -> Self {
    let __this: Value<v8_base_FlushDenormalsScope> = Rc::new(RefCell::new(Self {
      old_flush_state_: Rc::new(RefCell::new((*self.old_flush_state_.borrow()))),
    }));
    let this: Ptr<v8_base_FlushDenormalsScope> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for v8_base_FlushDenormalsScope {
  fn byte_size() -> usize {
    1
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    (*self.old_flush_state_.borrow()).to_bytes(&mut buf[0..1]);
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self {
      old_flush_state_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[0..1]))),
    }
  }
}
thread_local!(
  pub static kFlushDenormToZeroBit_0: Value<i32> = Rc::new(RefCell::new(16777216));
);
pub fn GetStatusWord_1() -> i32 {
  let result: Value<i32> = <Value<i32>>::default();
  b"mrs %x[result], FPCR";
  return (*result.borrow());
}
pub fn SetStatusWord_2(a: i32) {
  let a: Value<i32> = Rc::new(RefCell::new(a));
  b"msr FPCR, %x[src]";
}
impl v8_base_FPU {
  pub fn GetFlushDenormals() -> bool {
    let csr: Value<i32> = Rc::new(RefCell::new(({ GetStatusWord_1() })));
    return (((*csr.borrow()) & 16777216) != 0);
  }
}
impl v8_base_FPU {
  pub fn SetFlushDenormals(value: bool) {
    let value: Value<bool> = Rc::new(RefCell::new(value));
    let old_csr: Value<i32> = Rc::new(RefCell::new(({ GetStatusWord_1() })));
    let new_csr: Value<i32> = Rc::new(RefCell::new(if (*value.borrow()) {
      ((*old_csr.borrow()) | 16777216)
    } else {
      ((*old_csr.borrow()) & !16777216)
    }));
    ({ SetStatusWord_2((*new_csr.borrow())) });
  }
}
pub trait v8_base_FlushDenormalsScopeImpl {
  fn destructor(&self);
}
impl v8_base_FlushDenormalsScopeImpl for Ptr<v8_base_FlushDenormalsScope> {
  fn destructor(&self) {
    ({ v8_base_FPU::SetFlushDenormals((*(*(*self).upgrade().deref()).old_flush_state_.borrow())) });
  }
}
