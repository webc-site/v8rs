use std::{
  cell::RefCell,
  collections::BTreeMap,
  io::{Read, Seek, Write, prelude::*},
  os::fd::AsFd,
  rc::{Rc, Weak},
};

use crate::*;
thread_local!(
  pub static kReturnAddressStackSlotCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
  pub static kPageSizeBits_1: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
  pub static kRegularPageSize_2: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
  pub static kMinimumOSPageSize_3: Value<i32> = Rc::new(RefCell::new(16384));
);
