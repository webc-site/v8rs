use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u8;
pub const anon_0_ONCE_STATE_UNINITIALIZED: anon_0 = 0;
pub const anon_0_ONCE_STATE_EXECUTING_FUNCTION: anon_0 = 1;
pub const anon_0_ONCE_STATE_DONE: anon_0 = 2;
pub fn CallOnce_1(once: Ptr<std_atomic_unsigned_char_>, init_func: std_function_void____) {
    let once: Value<Ptr<std_atomic_unsigned_char_>> = Rc::new(RefCell::new(once));
    let init_func: Value<std_function_void____> = Rc::new(RefCell::new(init_func));
    if ((({ (*(*once.borrow()).upgrade().deref()).load_const(Some(2)) }) as i32)
        != (anon_0_ONCE_STATE_DONE as i32))
    {
        ({ CallOnceImpl_2((*once.borrow()).clone(), (*init_func.borrow()).clone()) });
    }
}
pub fn CallOnceImpl_2(once: Ptr<std_atomic_unsigned_char_>, init_func: std_function_void____) {
    let once: Value<Ptr<std_atomic_unsigned_char_>> = Rc::new(RefCell::new(once));
    let init_func: Value<std_function_void____> = Rc::new(RefCell::new(init_func));
    if ((({ (*(*once.borrow()).upgrade().deref()).load_const(Some(2)) }) as i32)
        == (anon_0_ONCE_STATE_DONE as i32))
    {
        return;
    }
    let expected: Value<u8> = Rc::new(RefCell::new((anon_0_ONCE_STATE_UNINITIALIZED as u8)));
    if ({
        (*(*once.borrow()).upgrade().deref()).compare_exchange_strong_pmutu8_u8(
            expected.as_pointer(),
            (anon_0_ONCE_STATE_EXECUTING_FUNCTION as u8),
            Some(4),
        )
    }) {
        ({ (*init_func.borrow_mut())() });
        ({
            (*(*once.borrow()).upgrade().deref()).store_u8((anon_0_ONCE_STATE_DONE as u8), Some(3))
        });
    } else {
        'loop_: while ((({ (*(*once.borrow()).upgrade().deref()).load_const(Some(2)) }) as i32)
            == (anon_0_ONCE_STATE_EXECUTING_FUNCTION as i32))
        {
            ({ sched_yield_3() });
        }
    }
}
