// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::{cell::RefCell, rc::Rc};

use crate::rt::{ByteRepr, Value};

#[derive(Default)]
pub struct Pollfd {
  pub fd: Value<i32>,
  pub events: Value<i16>,
  pub revents: Value<i16>,
}

impl Clone for Pollfd {
  fn clone(&self) -> Self {
    Self {
      fd: Rc::new(RefCell::new(*self.fd.borrow())),
      events: Rc::new(RefCell::new(*self.events.borrow())),
      revents: Rc::new(RefCell::new(*self.revents.borrow())),
    }
  }
}

impl ByteRepr for Pollfd {}

impl ByteRepr for ::libc::pollfd {}
