use std::{
  cell::RefCell,
  collections::BTreeMap,
  io::{Read, Seek, Write, prelude::*},
  os::fd::AsFd,
  rc::{Rc, Weak},
};

use crate::*;
thread_local!();
thread_local!();
thread_local!();
thread_local!();
thread_local!(
  pub static v8_Default_embedded_blob_code__0: Value<Box<[u8]>> =
    Rc::new(RefCell::new(Box::new([0_u8])));
);
thread_local!(
  pub static v8_Default_embedded_blob_code_size__1: Value<u32> = Rc::new(RefCell::new(0_u32));
);
thread_local!(
  pub static v8_Default_embedded_blob_data__2: Value<Box<[u8]>> =
    Rc::new(RefCell::new(Box::new([0_u8])));
);
thread_local!(
  pub static v8_Default_embedded_blob_data_size__3: Value<u32> = Rc::new(RefCell::new(0_u32));
);
