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
pub fn __mulodi4_4(a: i64, b: i64, overflow: Ptr<i32>) -> i64 {
  let a: Value<i64> = Rc::new(RefCell::new(a));
  let b: Value<i64> = Rc::new(RefCell::new(b));
  let overflow: Value<Ptr<i32>> = Rc::new(RefCell::new(overflow));
  let a_low: Value<u64> = Rc::new(RefCell::new((((*a.borrow()) & 4294967295_i64) as u64)));
  let a_high: Value<u64> = Rc::new(RefCell::new((((*a.borrow()) as u64) >> 32)));
  let b_low: Value<u64> = Rc::new(RefCell::new((((*b.borrow()) & 4294967295_i64) as u64)));
  let b_high: Value<u64> = Rc::new(RefCell::new((((*b.borrow()) as u64) >> 32)));
  let r_low: Value<u64> = Rc::new(RefCell::new(
    (*a_low.borrow()).wrapping_mul((*b_low.borrow())),
  ));
  let r_mid1: Value<u64> = Rc::new(RefCell::new(
    (*a_low.borrow()).wrapping_mul((*b_high.borrow())),
  ));
  let r_mid2: Value<u64> = Rc::new(RefCell::new(
    (*a_high.borrow()).wrapping_mul((*b_low.borrow())),
  ));
  let r_high: Value<u64> = Rc::new(RefCell::new(
    (*a_high.borrow()).wrapping_mul((*b_high.borrow())),
  ));
  let result1: Value<u64> = Rc::new(RefCell::new(
    (*r_low.borrow()).wrapping_add(((*r_mid1.borrow()) << 32)),
  ));
  if ((*result1.borrow()) < (*r_low.borrow())) {
    (*r_high.borrow_mut()).postfix_inc();
  }
  let result2: Value<u64> = Rc::new(RefCell::new(
    (*result1.borrow()).wrapping_add(((*r_mid2.borrow()) << 32)),
  ));
  if ((*result2.borrow()) < (*result1.borrow())) {
    (*r_high.borrow_mut()).postfix_inc();
  }
  {
    let rhs_0 = (*r_high.borrow())
      .wrapping_add(((*r_mid1.borrow()) >> 32).wrapping_add(((*r_mid2.borrow()) >> 32)));
    (*r_high.borrow_mut()) = rhs_0
  };
  let result: Value<i64> = Rc::new(RefCell::new(((*result2.borrow()) as i64)));
  let result_sign: Value<u64> = Rc::new(RefCell::new((((*result.borrow()) >> 63) as u64)));
  let expected_result_sign: Value<u64> = Rc::new(RefCell::new(
    ((((*a.borrow()) >> 63) ^ ((*b.borrow()) >> 63)) as u64),
  ));
  let __rhs = if (((*r_high.borrow()) > 0_u64)
    || ((*result_sign.borrow()) != (*expected_result_sign.borrow())))
  {
    1
  } else {
    0
  };
  (*overflow.borrow()).write(__rhs);
  return (*result.borrow());
}
