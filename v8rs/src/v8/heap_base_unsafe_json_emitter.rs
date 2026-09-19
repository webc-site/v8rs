use std::{
  cell::RefCell,
  collections::BTreeMap,
  io::{Read, Seek, Write, prelude::*},
  os::fd::AsFd,
  rc::{Rc, Weak},
};

use crate::*;
#[derive(Default)]
pub struct heap_base_UnsafeJsonEmitter {
  out_: Value<std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>,
  first_: Value<bool>,
}
impl heap_base_UnsafeJsonEmitter {
  pub fn UnsafeJsonEmitter_pmutheap_base_UnsafeJsonEmitter(
    _a0: Ptr<heap_base_UnsafeJsonEmitter>,
  ) -> Self {
    let __this: Value<heap_base_UnsafeJsonEmitter> = Rc::new(RefCell::new(Self {
      out_: Rc::new(RefCell::new(
        (*(*_a0.upgrade().deref()).out_.borrow_mut()).clone(),
      )),
      first_: Rc::new(RefCell::new((*(*_a0.upgrade().deref()).first_.borrow()))),
    }));
    let this: Ptr<heap_base_UnsafeJsonEmitter> = __this.as_pointer();
    Rc::try_unwrap(__this).ok().unwrap().into_inner()
  }
}
impl ByteRepr for heap_base_UnsafeJsonEmitter {
  fn byte_size() -> usize {
    288
  }
  fn to_bytes(&self, buf: &mut [u8]) {
    (*self.out_.borrow()).to_bytes(&mut buf[0..280]);
    (*self.first_.borrow()).to_bytes(&mut buf[280..281]);
  }
  fn from_bytes(buf: &[u8]) -> Self {
    Self {
      out_: Rc::new(RefCell::new(
        <std_basic_stringstream_char__std_char_traits_char___std_allocator_char__>::from_bytes(
          &buf[0..280],
        ),
      )),
      first_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[280..281]))),
    }
  }
}
pub trait heap_base_UnsafeJsonEmitterImpl {
  fn object_start(&self) -> Ptr<heap_base_UnsafeJsonEmitter>;
  fn object_end(&self) -> Ptr<heap_base_UnsafeJsonEmitter>;
  fn ToString(&self) -> Vec<u8>;
  fn emit_property_name(&self, name: Ptr<u8>);
  fn emit_value_bool(&self, b: bool);
  fn emit_value_Ptru8(&self, value: Ptr<u8>);
  fn operator_assign_pmutheap_base_UnsafeJsonEmitter(
    &self,
    _a0: Ptr<heap_base_UnsafeJsonEmitter>,
  ) -> Ptr<heap_base_UnsafeJsonEmitter>;
}
impl heap_base_UnsafeJsonEmitterImpl for Ptr<heap_base_UnsafeJsonEmitter> {
  fn emit_value_bool(&self, b: bool) {
    let b: Value<bool> = Rc::new(RefCell::new(b));
    write!(
      (*(*self).upgrade().deref()).out_.as_pointer(),
      "{:}",
      (if (*b.borrow()) {
        Ptr::from_string_literal(b"true")
      } else {
        Ptr::from_string_literal(b"false")
      }),
    );
  }
  fn emit_value_Ptru8(&self, value: Ptr<u8>) {
    let value: Value<Ptr<u8>> = Rc::new(RefCell::new(value));
    write!(
      (*(*self).upgrade().deref()).out_.as_pointer(),
      "\"{:}\"",
      (*value.borrow()),
    );
  }
  fn operator_assign_pmutheap_base_UnsafeJsonEmitter(
    &self,
    _a0: Ptr<heap_base_UnsafeJsonEmitter>,
  ) -> Ptr<heap_base_UnsafeJsonEmitter> {
    let __rhs = (*(*_a0.upgrade().deref()).out_.borrow()).clone();
    (*(*(*self).upgrade().deref()).out_.borrow_mut()) = __rhs;
    let __rhs = (*(*_a0.upgrade().deref()).first_.borrow());
    (*(*(*self).upgrade().deref()).first_.borrow_mut()) = __rhs;
    return (*self).clone();
  }
  fn object_start(&self) -> Ptr<heap_base_UnsafeJsonEmitter> {
    write!((*(*self).upgrade().deref()).out_.as_pointer(), "{{",);
    (*(*(*self).upgrade().deref()).first_.borrow_mut()) = true;
    return (*self).clone();
  }
  fn object_end(&self) -> Ptr<heap_base_UnsafeJsonEmitter> {
    write!((*(*self).upgrade().deref()).out_.as_pointer(), "}}",);
    return (*self).clone();
  }
  fn ToString(&self) -> Vec<u8> {
    return ({ (*(*(*self).upgrade().deref()).out_.borrow()).str_const_lref() });
  }
  fn emit_property_name(&self, name: Ptr<u8>) {
    let name: Value<Ptr<u8>> = Rc::new(RefCell::new(name));
    write!(
      (*(*self).upgrade().deref()).out_.as_pointer(),
      "\"{:}\":",
      (*name.borrow()),
    );
  }
}
