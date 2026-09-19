use std::{
  cell::RefCell,
  collections::BTreeMap,
  io::{Read, Seek, Write, prelude::*},
  os::fd::AsFd,
  rc::{Rc, Weak},
};

use crate::*;
thread_local!(
  pub static kReturnAddressStackSlotCount_0: Value<i32> =
    Rc::new(RefCell::new(if false { 1 } else { 0 }));
);
thread_local!(
  pub static kPageSizeBits_1: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
  pub static kRegularPageSize_2: Value<i32> = Rc::new(RefCell::new(
    (1 << (*kPageSizeBits_1.with(Value::clone).borrow())),
  ));
);
thread_local!(
  pub static kMinimumOSPageSize_3: Value<i32> = Rc::new(RefCell::new((16 * 1024)));
);
