use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn SignedDivisionByConstant_0(d: u32) -> v8_base_MagicNumbersForDivision_unsigned_int_ {
    let d: Value<u32> = Rc::new(RefCell::new(d));
    (&(0));
    let bits: Value<u32> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<u32>() as u32) as u32).wrapping_mul(8_u32),
    ));
    let min: Value<u32> = Rc::new(RefCell::new(
        (1_u32 << ((*bits.borrow()).wrapping_sub(1_u32))),
    ));
    let neg: Value<bool> = Rc::new(RefCell::new((((*min.borrow()) & (*d.borrow())) != 0_u32)));
    let ad: Value<u32> = Rc::new(RefCell::new(if (*neg.borrow()) {
        ((0_u32).wrapping_sub((*d.borrow())))
    } else {
        (*d.borrow())
    }));
    let t: Value<u32> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_add(((*d.borrow()) >> ((*bits.borrow()).wrapping_sub(1_u32)))),
    ));
    let anc: Value<u32> = Rc::new(RefCell::new(
        ((*t.borrow()).wrapping_sub(1_u32))
            .wrapping_sub((*t.borrow()).wrapping_rem((*ad.borrow()))),
    ));
    let p: Value<u32> = Rc::new(RefCell::new((*bits.borrow()).wrapping_sub(1_u32)));
    let q1: Value<u32> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*anc.borrow()))));
    let r1: Value<u32> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q1.borrow()).wrapping_mul((*anc.borrow()))),
    ));
    let q2: Value<u32> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*ad.borrow()))));
    let r2: Value<u32> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q2.borrow()).wrapping_mul((*ad.borrow()))),
    ));
    let delta: Value<u32> = <Value<u32>>::default();
    let mut __do_while = true;
    'loop_: while __do_while
        || (((*q1.borrow()) < (*delta.borrow()))
            || (((*q1.borrow()) == (*delta.borrow())) && ((*r1.borrow()) == 0_u32)))
    {
        __do_while = false;
        let __rhs = (*p.borrow()).wrapping_add(1_u32);
        (*p.borrow_mut()) = __rhs;
        let __rhs = (2_u32).wrapping_mul((*q1.borrow()));
        (*q1.borrow_mut()) = __rhs;
        let __rhs = (2_u32).wrapping_mul((*r1.borrow()));
        (*r1.borrow_mut()) = __rhs;
        if ((*r1.borrow()) >= (*anc.borrow())) {
            let __rhs = (*q1.borrow()).wrapping_add(1_u32);
            (*q1.borrow_mut()) = __rhs;
            let __rhs = (*r1.borrow()).wrapping_sub((*anc.borrow()));
            (*r1.borrow_mut()) = __rhs;
        }
        let __rhs = (2_u32).wrapping_mul((*q2.borrow()));
        (*q2.borrow_mut()) = __rhs;
        let __rhs = (2_u32).wrapping_mul((*r2.borrow()));
        (*r2.borrow_mut()) = __rhs;
        if ((*r2.borrow()) >= (*ad.borrow())) {
            let __rhs = (*q2.borrow()).wrapping_add(1_u32);
            (*q2.borrow_mut()) = __rhs;
            let __rhs = (*r2.borrow()).wrapping_sub((*ad.borrow()));
            (*r2.borrow_mut()) = __rhs;
        }
        (*delta.borrow_mut()) = (*ad.borrow()).wrapping_sub((*r2.borrow()));
    }
    let mul: Value<u32> = Rc::new(RefCell::new((*q2.borrow()).wrapping_add(1_u32)));
    return v8_base_MagicNumbersForDivision_unsigned_int_ :: v8_base_MagicNumbersForDivision_unsigned_int_ ( {  if (*neg.borrow()) { ( ( 0_u32 ) . wrapping_sub ( (*mul.borrow()) ) )  } else { (*mul.borrow())  }   } , {  ( (*p.borrow()) ) . wrapping_sub ( (*bits.borrow()) )   } , {  false   } , )   ;
}
pub fn SignedDivisionByConstant_1(d: u64) -> v8_base_MagicNumbersForDivision_unsigned_long_long_ {
    let d: Value<u64> = Rc::new(RefCell::new(d));
    (&(0));
    let bits: Value<u32> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<u64>() as u32) as u32).wrapping_mul(8_u32),
    ));
    let min: Value<u64> = Rc::new(RefCell::new(
        (1_u64 << ((*bits.borrow()).wrapping_sub(1_u32))),
    ));
    let neg: Value<bool> = Rc::new(RefCell::new((((*min.borrow()) & (*d.borrow())) != 0_u64)));
    let ad: Value<u64> = Rc::new(RefCell::new(if (*neg.borrow()) {
        ((0_u64).wrapping_sub((*d.borrow())))
    } else {
        (*d.borrow())
    }));
    let t: Value<u64> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_add(((*d.borrow()) >> ((*bits.borrow()).wrapping_sub(1_u32)))),
    ));
    let anc: Value<u64> = Rc::new(RefCell::new(
        ((*t.borrow()).wrapping_sub(1_u64))
            .wrapping_sub((*t.borrow()).wrapping_rem((*ad.borrow()))),
    ));
    let p: Value<u32> = Rc::new(RefCell::new((*bits.borrow()).wrapping_sub(1_u32)));
    let q1: Value<u64> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*anc.borrow()))));
    let r1: Value<u64> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q1.borrow()).wrapping_mul((*anc.borrow()))),
    ));
    let q2: Value<u64> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*ad.borrow()))));
    let r2: Value<u64> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q2.borrow()).wrapping_mul((*ad.borrow()))),
    ));
    let delta: Value<u64> = <Value<u64>>::default();
    let mut __do_while = true;
    'loop_: while __do_while
        || (((*q1.borrow()) < (*delta.borrow()))
            || (((*q1.borrow()) == (*delta.borrow())) && ((*r1.borrow()) == 0_u64)))
    {
        __do_while = false;
        let __rhs = (*p.borrow()).wrapping_add(1_u32);
        (*p.borrow_mut()) = __rhs;
        let __rhs = (2_u64).wrapping_mul((*q1.borrow()));
        (*q1.borrow_mut()) = __rhs;
        let __rhs = (2_u64).wrapping_mul((*r1.borrow()));
        (*r1.borrow_mut()) = __rhs;
        if ((*r1.borrow()) >= (*anc.borrow())) {
            let __rhs = (*q1.borrow()).wrapping_add(1_u64);
            (*q1.borrow_mut()) = __rhs;
            let __rhs = (*r1.borrow()).wrapping_sub((*anc.borrow()));
            (*r1.borrow_mut()) = __rhs;
        }
        let __rhs = (2_u64).wrapping_mul((*q2.borrow()));
        (*q2.borrow_mut()) = __rhs;
        let __rhs = (2_u64).wrapping_mul((*r2.borrow()));
        (*r2.borrow_mut()) = __rhs;
        if ((*r2.borrow()) >= (*ad.borrow())) {
            let __rhs = (*q2.borrow()).wrapping_add(1_u64);
            (*q2.borrow_mut()) = __rhs;
            let __rhs = (*r2.borrow()).wrapping_sub((*ad.borrow()));
            (*r2.borrow_mut()) = __rhs;
        }
        (*delta.borrow_mut()) = (*ad.borrow()).wrapping_sub((*r2.borrow()));
    }
    let mul: Value<u64> = Rc::new(RefCell::new((*q2.borrow()).wrapping_add(1_u64)));
    return v8_base_MagicNumbersForDivision_unsigned_long_long_ :: v8_base_MagicNumbersForDivision_unsigned_long_long_ ( {  if (*neg.borrow()) { ( ( 0_u64 ) . wrapping_sub ( (*mul.borrow()) ) )  } else { (*mul.borrow())  }   } , {  ( (*p.borrow()) ) . wrapping_sub ( (*bits.borrow()) )   } , {  false   } , )   ;
}
pub fn UnsignedDivisionByConstant_2(
    d: u32,
    leading_zeros: Option<u32>,
) -> v8_base_MagicNumbersForDivision_unsigned_int_ {
    let d: Value<u32> = Rc::new(RefCell::new(d));
    let leading_zeros: Value<u32> = Rc::new(RefCell::new(leading_zeros.unwrap_or()));
    (&(0));
    let bits: Value<u32> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<u32>() as u32) as u32).wrapping_mul(8_u32),
    ));
    let ones: Value<u32> = Rc::new(RefCell::new((!0_u32 >> (*leading_zeros.borrow()))));
    let min: Value<u32> = Rc::new(RefCell::new(
        (1_u32 << ((*bits.borrow()).wrapping_sub(1_u32))),
    ));
    let max: Value<u32> = Rc::new(RefCell::new((!0_u32 >> 1)));
    let nc: Value<u32> = Rc::new(RefCell::new((*ones.borrow()).wrapping_sub(
        ((*ones.borrow()).wrapping_sub((*d.borrow()))).wrapping_rem((*d.borrow())),
    )));
    let a: Value<bool> = Rc::new(RefCell::new(false));
    let p: Value<u32> = Rc::new(RefCell::new((*bits.borrow()).wrapping_sub(1_u32)));
    let q1: Value<u32> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*nc.borrow()))));
    let r1: Value<u32> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q1.borrow()).wrapping_mul((*nc.borrow()))),
    ));
    let q2: Value<u32> = Rc::new(RefCell::new((*max.borrow()).wrapping_div((*d.borrow()))));
    let r2: Value<u32> = Rc::new(RefCell::new(
        (*max.borrow()).wrapping_sub((*q2.borrow()).wrapping_mul((*d.borrow()))),
    ));
    let delta: Value<u32> = <Value<u32>>::default();
    let mut __do_while = true;
    'loop_: while __do_while
        || (((*p.borrow()) < (*bits.borrow()).wrapping_mul(2_u32))
            && (((*q1.borrow()) < (*delta.borrow()))
                || (((*q1.borrow()) == (*delta.borrow())) && ((*r1.borrow()) == 0_u32))))
    {
        __do_while = false;
        let __rhs = (*p.borrow()).wrapping_add(1_u32);
        (*p.borrow_mut()) = __rhs;
        if ((*r1.borrow()) >= (*nc.borrow()).wrapping_sub((*r1.borrow()))) {
            let __rhs = ((2_u32).wrapping_mul((*q1.borrow()))).wrapping_add(1_u32);
            (*q1.borrow_mut()) = __rhs;
            let __rhs = ((2_u32).wrapping_mul((*r1.borrow()))).wrapping_sub((*nc.borrow()));
            (*r1.borrow_mut()) = __rhs;
        } else {
            let __rhs = (2_u32).wrapping_mul((*q1.borrow()));
            (*q1.borrow_mut()) = __rhs;
            let __rhs = (2_u32).wrapping_mul((*r1.borrow()));
            (*r1.borrow_mut()) = __rhs;
        }
        if ((*r2.borrow()).wrapping_add(1_u32) >= (*d.borrow()).wrapping_sub((*r2.borrow()))) {
            if ((*q2.borrow()) >= (*max.borrow())) {
                (*a.borrow_mut()) = true;
            }
            let __rhs = ((2_u32).wrapping_mul((*q2.borrow()))).wrapping_add(1_u32);
            (*q2.borrow_mut()) = __rhs;
            let __rhs = (((2_u32).wrapping_mul((*r2.borrow()))).wrapping_add(1_u32))
                .wrapping_sub((*d.borrow()));
            (*r2.borrow_mut()) = __rhs;
        } else {
            if ((*q2.borrow()) >= (*min.borrow())) {
                (*a.borrow_mut()) = true;
            }
            let __rhs = (2_u32).wrapping_mul((*q2.borrow()));
            (*q2.borrow_mut()) = __rhs;
            let __rhs = ((2_u32).wrapping_mul((*r2.borrow()))).wrapping_add(1_u32);
            (*r2.borrow_mut()) = __rhs;
        }
        (*delta.borrow_mut()) = ((*d.borrow()).wrapping_sub(1_u32)).wrapping_sub((*r2.borrow()));
    }
    return v8_base_MagicNumbersForDivision_unsigned_int_ :: v8_base_MagicNumbersForDivision_unsigned_int_ ( {  ( (*q2.borrow()) ) . wrapping_add ( 1_u32 )   } , {  ( (*p.borrow()) ) . wrapping_sub ( (*bits.borrow()) )   } , {  (*a.borrow())   } , )   ;
}
pub fn UnsignedDivisionByConstant_3(
    d: u64,
    leading_zeros: Option<u32>,
) -> v8_base_MagicNumbersForDivision_unsigned_long_long_ {
    let d: Value<u64> = Rc::new(RefCell::new(d));
    let leading_zeros: Value<u32> = Rc::new(RefCell::new(leading_zeros.unwrap_or()));
    (&(0));
    let bits: Value<u32> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<u64>() as u32) as u32).wrapping_mul(8_u32),
    ));
    let ones: Value<u64> = Rc::new(RefCell::new((!0_u64 >> (*leading_zeros.borrow()))));
    let min: Value<u64> = Rc::new(RefCell::new(
        (1_u64 << ((*bits.borrow()).wrapping_sub(1_u32))),
    ));
    let max: Value<u64> = Rc::new(RefCell::new((!0_u64 >> 1)));
    let nc: Value<u64> = Rc::new(RefCell::new((*ones.borrow()).wrapping_sub(
        ((*ones.borrow()).wrapping_sub((*d.borrow()))).wrapping_rem((*d.borrow())),
    )));
    let a: Value<bool> = Rc::new(RefCell::new(false));
    let p: Value<u32> = Rc::new(RefCell::new((*bits.borrow()).wrapping_sub(1_u32)));
    let q1: Value<u64> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*nc.borrow()))));
    let r1: Value<u64> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q1.borrow()).wrapping_mul((*nc.borrow()))),
    ));
    let q2: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_div((*d.borrow()))));
    let r2: Value<u64> = Rc::new(RefCell::new(
        (*max.borrow()).wrapping_sub((*q2.borrow()).wrapping_mul((*d.borrow()))),
    ));
    let delta: Value<u64> = <Value<u64>>::default();
    let mut __do_while = true;
    'loop_: while __do_while
        || (((*p.borrow()) < (*bits.borrow()).wrapping_mul(2_u32))
            && (((*q1.borrow()) < (*delta.borrow()))
                || (((*q1.borrow()) == (*delta.borrow())) && ((*r1.borrow()) == 0_u64))))
    {
        __do_while = false;
        let __rhs = (*p.borrow()).wrapping_add(1_u32);
        (*p.borrow_mut()) = __rhs;
        if ((*r1.borrow()) >= (*nc.borrow()).wrapping_sub((*r1.borrow()))) {
            let __rhs = ((2_u64).wrapping_mul((*q1.borrow()))).wrapping_add(1_u64);
            (*q1.borrow_mut()) = __rhs;
            let __rhs = ((2_u64).wrapping_mul((*r1.borrow()))).wrapping_sub((*nc.borrow()));
            (*r1.borrow_mut()) = __rhs;
        } else {
            let __rhs = (2_u64).wrapping_mul((*q1.borrow()));
            (*q1.borrow_mut()) = __rhs;
            let __rhs = (2_u64).wrapping_mul((*r1.borrow()));
            (*r1.borrow_mut()) = __rhs;
        }
        if ((*r2.borrow()).wrapping_add(1_u64) >= (*d.borrow()).wrapping_sub((*r2.borrow()))) {
            if ((*q2.borrow()) >= (*max.borrow())) {
                (*a.borrow_mut()) = true;
            }
            let __rhs = ((2_u64).wrapping_mul((*q2.borrow()))).wrapping_add(1_u64);
            (*q2.borrow_mut()) = __rhs;
            let __rhs = (((2_u64).wrapping_mul((*r2.borrow()))).wrapping_add(1_u64))
                .wrapping_sub((*d.borrow()));
            (*r2.borrow_mut()) = __rhs;
        } else {
            if ((*q2.borrow()) >= (*min.borrow())) {
                (*a.borrow_mut()) = true;
            }
            let __rhs = (2_u64).wrapping_mul((*q2.borrow()));
            (*q2.borrow_mut()) = __rhs;
            let __rhs = ((2_u64).wrapping_mul((*r2.borrow()))).wrapping_add(1_u64);
            (*r2.borrow_mut()) = __rhs;
        }
        (*delta.borrow_mut()) = ((*d.borrow()).wrapping_sub(1_u64)).wrapping_sub((*r2.borrow()));
    }
    return v8_base_MagicNumbersForDivision_unsigned_long_long_ :: v8_base_MagicNumbersForDivision_unsigned_long_long_ ( {  ( (*q2.borrow()) ) . wrapping_add ( 1_u64 )   } , {  ( (*p.borrow()) ) . wrapping_sub ( (*bits.borrow()) )   } , {  (*a.borrow())   } , )   ;
}
#[derive(Default)]
pub struct v8_base_MagicNumbersForDivision_unsigned_int_ {
    pub multiplier: Value<u32>,
    pub shift: Value<u32>,
    pub add: Value<bool>,
}
impl v8_base_MagicNumbersForDivision_unsigned_int_ {
    pub fn v8_base_MagicNumbersForDivision_unsigned_int_(m: u32, s: u32, a: bool) -> Self {
        let m: Value<u32> = Rc::new(RefCell::new(m));
        let s: Value<u32> = Rc::new(RefCell::new(s));
        let a: Value<bool> = Rc::new(RefCell::new(a));
        let __this: Value<v8_base_MagicNumbersForDivision_unsigned_int_> =
            Rc::new(RefCell::new(Self {
                multiplier: Rc::new(RefCell::new((*m.borrow()))),
                shift: Rc::new(RefCell::new((*s.borrow()))),
                add: Rc::new(RefCell::new((*a.borrow()))),
            }));
        let this: Ptr<v8_base_MagicNumbersForDivision_unsigned_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for v8_base_MagicNumbersForDivision_unsigned_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_MagicNumbersForDivision_unsigned_int_Impl::operator_eq(
                &Rc::new(RefCell::new(
                    v8_base_MagicNumbersForDivision_unsigned_int_ {
                        multiplier: self.multiplier.clone(),
                        shift: self.shift.clone(),
                        add: self.add.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    v8_base_MagicNumbersForDivision_unsigned_int_ {
                        multiplier: other.multiplier.clone(),
                        shift: other.shift.clone(),
                        add: other.add.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for v8_base_MagicNumbersForDivision_unsigned_int_ {}
impl Clone for v8_base_MagicNumbersForDivision_unsigned_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_MagicNumbersForDivision_unsigned_int_> =
            Rc::new(RefCell::new(Self {
                multiplier: Rc::new(RefCell::new((*self.multiplier.borrow()))),
                shift: Rc::new(RefCell::new((*self.shift.borrow()))),
                add: Rc::new(RefCell::new((*self.add.borrow()))),
            }));
        let this: Ptr<v8_base_MagicNumbersForDivision_unsigned_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_MagicNumbersForDivision_unsigned_int_ {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.multiplier.borrow()).to_bytes(&mut buf[0..4]);
        (*self.shift.borrow()).to_bytes(&mut buf[4..8]);
        (*self.add.borrow()).to_bytes(&mut buf[8..9]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            multiplier: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            shift: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
            add: Rc::new(RefCell::new(<bool>::from_bytes(&buf[8..9]))),
        }
    }
}
#[derive(Default)]
pub struct v8_base_MagicNumbersForDivision_unsigned_long_long_ {
    pub multiplier: Value<u64>,
    pub shift: Value<u32>,
    pub add: Value<bool>,
}
impl v8_base_MagicNumbersForDivision_unsigned_long_long_ {
    pub fn v8_base_MagicNumbersForDivision_unsigned_long_long_(m: u64, s: u32, a: bool) -> Self {
        let m: Value<u64> = Rc::new(RefCell::new(m));
        let s: Value<u32> = Rc::new(RefCell::new(s));
        let a: Value<bool> = Rc::new(RefCell::new(a));
        let __this: Value<v8_base_MagicNumbersForDivision_unsigned_long_long_> =
            Rc::new(RefCell::new(Self {
                multiplier: Rc::new(RefCell::new((*m.borrow()))),
                shift: Rc::new(RefCell::new((*s.borrow()))),
                add: Rc::new(RefCell::new((*a.borrow()))),
            }));
        let this: Ptr<v8_base_MagicNumbersForDivision_unsigned_long_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for v8_base_MagicNumbersForDivision_unsigned_long_long_ {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_base_MagicNumbersForDivision_unsigned_long_long_Impl::operator_eq(
                &Rc::new(RefCell::new(
                    v8_base_MagicNumbersForDivision_unsigned_long_long_ {
                        multiplier: self.multiplier.clone(),
                        shift: self.shift.clone(),
                        add: self.add.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    v8_base_MagicNumbersForDivision_unsigned_long_long_ {
                        multiplier: other.multiplier.clone(),
                        shift: other.shift.clone(),
                        add: other.add.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for v8_base_MagicNumbersForDivision_unsigned_long_long_ {}
impl Clone for v8_base_MagicNumbersForDivision_unsigned_long_long_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_MagicNumbersForDivision_unsigned_long_long_> =
            Rc::new(RefCell::new(Self {
                multiplier: Rc::new(RefCell::new((*self.multiplier.borrow()))),
                shift: Rc::new(RefCell::new((*self.shift.borrow()))),
                add: Rc::new(RefCell::new((*self.add.borrow()))),
            }));
        let this: Ptr<v8_base_MagicNumbersForDivision_unsigned_long_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_MagicNumbersForDivision_unsigned_long_long_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.multiplier.borrow()).to_bytes(&mut buf[0..8]);
        (*self.shift.borrow()).to_bytes(&mut buf[8..12]);
        (*self.add.borrow()).to_bytes(&mut buf[12..13]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            multiplier: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            shift: Rc::new(RefCell::new(<u32>::from_bytes(&buf[8..12]))),
            add: Rc::new(RefCell::new(<bool>::from_bytes(&buf[12..13]))),
        }
    }
}
pub type v8_base_AbortMode = i32;
pub const v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures: v8_base_AbortMode = 0;
pub const v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures: v8_base_AbortMode = 1;
pub const v8_base_AbortMode_kExitIfNoSecurityImpact: v8_base_AbortMode = 2;
pub const v8_base_AbortMode_kImmediateCrash: v8_base_AbortMode = 3;
pub const v8_base_AbortMode_kDefault: v8_base_AbortMode = 4;
thread_local!();
pub fn ControlledCrashesAreHarmless_5() -> bool {
    return ((*g_abort_mode_4.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_4.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn DcheckFailuresAreIgnored_6() -> bool {
    return ((*g_abort_mode_4.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_4.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn FatalErrorsWithNoSecurityImpactShouldExit_7() -> bool {
    return ((*g_abort_mode_4.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitIfNoSecurityImpact);
}
thread_local!(
    pub static kReturnAddressStackSlotCount_8: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kPageSizeBits_9: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_10: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
    pub static kMinimumOSPageSize_11: Value<i32> = Rc::new(RefCell::new(16384));
);
thread_local!(
    pub static kUnimplementedCodeMessage_12: Value<Ptr<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"unimplemented code"),
    ));
);
thread_local!(
    pub static kUnreachableCodeMessage_13: Value<Ptr<u8>> =
        Rc::new(RefCell::new(Ptr::from_string_literal(b"unreachable code")));
);
#[derive(Default)]
pub struct v8_base_CheckMessageStream {}
impl ByteRepr for v8_base_CheckMessageStream {
    fn byte_size() -> usize {
        264
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub type v8_base_OOMType = i32;
pub const v8_base_OOMType_kJavaScript: v8_base_OOMType = 0;
pub const v8_base_OOMType_kProcess: v8_base_OOMType = 1;
pub type v8_base_comparison_underlying_type_unsigned_int__Dummy = u32;
thread_local!(
    pub static is_enum_14: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_comparison_underlying_type_unsigned_int_ {}
impl Clone for v8_base_comparison_underlying_type_unsigned_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_comparison_underlying_type_unsigned_int_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_comparison_underlying_type_unsigned_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_comparison_underlying_type_unsigned_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub type v8_base_comparison_underlying_type_int__Dummy = u32;
thread_local!(
    pub static is_enum_15: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_comparison_underlying_type_int_ {}
impl Clone for v8_base_comparison_underlying_type_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_comparison_underlying_type_int_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_comparison_underlying_type_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_comparison_underlying_type_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static value_16: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_unsigned_int__int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_unsigned_int__int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_unsigned_int__int_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_unsigned_int__int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_unsigned_int__int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static value_17: Value<bool> = Rc::new(RefCell::new(true));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_int__unsigned_int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_int__unsigned_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_int__unsigned_int_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_int__unsigned_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_int__unsigned_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static value_18: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_> =
            __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_unsigned_int__unsigned_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static value_19: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Default)]
pub struct v8_base_is_signed_vs_unsigned_int__int_ {}
impl Clone for v8_base_is_signed_vs_unsigned_int__int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_signed_vs_unsigned_int__int_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_signed_vs_unsigned_int__int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_signed_vs_unsigned_int__int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_is_unsigned_vs_signed_unsigned_int__int_ {}
impl Clone for v8_base_is_unsigned_vs_signed_unsigned_int__int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_unsigned_vs_signed_unsigned_int__int_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_unsigned_vs_signed_unsigned_int__int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_unsigned_vs_signed_unsigned_int__int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_base_is_unsigned_vs_signed_int__unsigned_int_ {}
impl Clone for v8_base_is_unsigned_vs_signed_int__unsigned_int_ {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_is_unsigned_vs_signed_int__unsigned_int_> =
            Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_is_unsigned_vs_signed_int__unsigned_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_is_unsigned_vs_signed_int__unsigned_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
impl v8_base_Use {
    pub fn v8_base_Use(_a0: Ptr<bool>) -> Self {
        let __this: Value<v8_base_Use> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Use> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Default)]
pub struct v8_base_Use {}
impl Clone for v8_base_Use {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_Use> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_base_Use> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_base_Use {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn make_uint64_20(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_21(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_22(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_21(_x, _m)
    });
}
pub fn IsAligned_23(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
pub fn SignedDivisionByConstant_0(d: u32) -> v8_base_MagicNumbersForDivision_unsigned_int_ {
    let d: Value<u32> = Rc::new(RefCell::new(d));
    (&(0));
    let bits: Value<u32> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<u32>() as u32) as u32).wrapping_mul(8_u32),
    ));
    let min: Value<u32> = Rc::new(RefCell::new(
        (1_u32 << ((*bits.borrow()).wrapping_sub(1_u32))),
    ));
    let neg: Value<bool> = Rc::new(RefCell::new((((*min.borrow()) & (*d.borrow())) != 0_u32)));
    let ad: Value<u32> = Rc::new(RefCell::new(if (*neg.borrow()) {
        ((0_u32).wrapping_sub((*d.borrow())))
    } else {
        (*d.borrow())
    }));
    let t: Value<u32> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_add(((*d.borrow()) >> ((*bits.borrow()).wrapping_sub(1_u32)))),
    ));
    let anc: Value<u32> = Rc::new(RefCell::new(
        ((*t.borrow()).wrapping_sub(1_u32))
            .wrapping_sub((*t.borrow()).wrapping_rem((*ad.borrow()))),
    ));
    let p: Value<u32> = Rc::new(RefCell::new((*bits.borrow()).wrapping_sub(1_u32)));
    let q1: Value<u32> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*anc.borrow()))));
    let r1: Value<u32> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q1.borrow()).wrapping_mul((*anc.borrow()))),
    ));
    let q2: Value<u32> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*ad.borrow()))));
    let r2: Value<u32> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q2.borrow()).wrapping_mul((*ad.borrow()))),
    ));
    let delta: Value<u32> = <Value<u32>>::default();
    let mut __do_while = true;
    'loop_: while __do_while
        || (((*q1.borrow()) < (*delta.borrow()))
            || (((*q1.borrow()) == (*delta.borrow())) && ((*r1.borrow()) == 0_u32)))
    {
        __do_while = false;
        let __rhs = (*p.borrow()).wrapping_add(1_u32);
        (*p.borrow_mut()) = __rhs;
        let __rhs = (2_u32).wrapping_mul((*q1.borrow()));
        (*q1.borrow_mut()) = __rhs;
        let __rhs = (2_u32).wrapping_mul((*r1.borrow()));
        (*r1.borrow_mut()) = __rhs;
        if ((*r1.borrow()) >= (*anc.borrow())) {
            let __rhs = (*q1.borrow()).wrapping_add(1_u32);
            (*q1.borrow_mut()) = __rhs;
            let __rhs = (*r1.borrow()).wrapping_sub((*anc.borrow()));
            (*r1.borrow_mut()) = __rhs;
        }
        let __rhs = (2_u32).wrapping_mul((*q2.borrow()));
        (*q2.borrow_mut()) = __rhs;
        let __rhs = (2_u32).wrapping_mul((*r2.borrow()));
        (*r2.borrow_mut()) = __rhs;
        if ((*r2.borrow()) >= (*ad.borrow())) {
            let __rhs = (*q2.borrow()).wrapping_add(1_u32);
            (*q2.borrow_mut()) = __rhs;
            let __rhs = (*r2.borrow()).wrapping_sub((*ad.borrow()));
            (*r2.borrow_mut()) = __rhs;
        }
        (*delta.borrow_mut()) = (*ad.borrow()).wrapping_sub((*r2.borrow()));
    }
    let mul: Value<u32> = Rc::new(RefCell::new((*q2.borrow()).wrapping_add(1_u32)));
    return v8_base_MagicNumbersForDivision_unsigned_int_ :: v8_base_MagicNumbersForDivision_unsigned_int_ ( {  if (*neg.borrow()) { ( ( 0_u32 ) . wrapping_sub ( (*mul.borrow()) ) )  } else { (*mul.borrow())  }   } , {  ( (*p.borrow()) ) . wrapping_sub ( (*bits.borrow()) )   } , {  false   } , )   ;
}
pub fn SignedDivisionByConstant_1(d: u64) -> v8_base_MagicNumbersForDivision_unsigned_long_long_ {
    let d: Value<u64> = Rc::new(RefCell::new(d));
    (&(0));
    let bits: Value<u32> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<u64>() as u32) as u32).wrapping_mul(8_u32),
    ));
    let min: Value<u64> = Rc::new(RefCell::new(
        (1_u64 << ((*bits.borrow()).wrapping_sub(1_u32))),
    ));
    let neg: Value<bool> = Rc::new(RefCell::new((((*min.borrow()) & (*d.borrow())) != 0_u64)));
    let ad: Value<u64> = Rc::new(RefCell::new(if (*neg.borrow()) {
        ((0_u64).wrapping_sub((*d.borrow())))
    } else {
        (*d.borrow())
    }));
    let t: Value<u64> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_add(((*d.borrow()) >> ((*bits.borrow()).wrapping_sub(1_u32)))),
    ));
    let anc: Value<u64> = Rc::new(RefCell::new(
        ((*t.borrow()).wrapping_sub(1_u64))
            .wrapping_sub((*t.borrow()).wrapping_rem((*ad.borrow()))),
    ));
    let p: Value<u32> = Rc::new(RefCell::new((*bits.borrow()).wrapping_sub(1_u32)));
    let q1: Value<u64> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*anc.borrow()))));
    let r1: Value<u64> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q1.borrow()).wrapping_mul((*anc.borrow()))),
    ));
    let q2: Value<u64> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*ad.borrow()))));
    let r2: Value<u64> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q2.borrow()).wrapping_mul((*ad.borrow()))),
    ));
    let delta: Value<u64> = <Value<u64>>::default();
    let mut __do_while = true;
    'loop_: while __do_while
        || (((*q1.borrow()) < (*delta.borrow()))
            || (((*q1.borrow()) == (*delta.borrow())) && ((*r1.borrow()) == 0_u64)))
    {
        __do_while = false;
        let __rhs = (*p.borrow()).wrapping_add(1_u32);
        (*p.borrow_mut()) = __rhs;
        let __rhs = (2_u64).wrapping_mul((*q1.borrow()));
        (*q1.borrow_mut()) = __rhs;
        let __rhs = (2_u64).wrapping_mul((*r1.borrow()));
        (*r1.borrow_mut()) = __rhs;
        if ((*r1.borrow()) >= (*anc.borrow())) {
            let __rhs = (*q1.borrow()).wrapping_add(1_u64);
            (*q1.borrow_mut()) = __rhs;
            let __rhs = (*r1.borrow()).wrapping_sub((*anc.borrow()));
            (*r1.borrow_mut()) = __rhs;
        }
        let __rhs = (2_u64).wrapping_mul((*q2.borrow()));
        (*q2.borrow_mut()) = __rhs;
        let __rhs = (2_u64).wrapping_mul((*r2.borrow()));
        (*r2.borrow_mut()) = __rhs;
        if ((*r2.borrow()) >= (*ad.borrow())) {
            let __rhs = (*q2.borrow()).wrapping_add(1_u64);
            (*q2.borrow_mut()) = __rhs;
            let __rhs = (*r2.borrow()).wrapping_sub((*ad.borrow()));
            (*r2.borrow_mut()) = __rhs;
        }
        (*delta.borrow_mut()) = (*ad.borrow()).wrapping_sub((*r2.borrow()));
    }
    let mul: Value<u64> = Rc::new(RefCell::new((*q2.borrow()).wrapping_add(1_u64)));
    return v8_base_MagicNumbersForDivision_unsigned_long_long_ :: v8_base_MagicNumbersForDivision_unsigned_long_long_ ( {  if (*neg.borrow()) { ( ( 0_u64 ) . wrapping_sub ( (*mul.borrow()) ) )  } else { (*mul.borrow())  }   } , {  ( (*p.borrow()) ) . wrapping_sub ( (*bits.borrow()) )   } , {  false   } , )   ;
}
pub fn UnsignedDivisionByConstant_2(
    d: u32,
    leading_zeros: Option<u32>,
) -> v8_base_MagicNumbersForDivision_unsigned_int_ {
    let d: Value<u32> = Rc::new(RefCell::new(d));
    let leading_zeros: Value<u32> = Rc::new(RefCell::new(leading_zeros.unwrap_or()));
    (&(0));
    let bits: Value<u32> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<u32>() as u32) as u32).wrapping_mul(8_u32),
    ));
    let ones: Value<u32> = Rc::new(RefCell::new((!0_u32 >> (*leading_zeros.borrow()))));
    let min: Value<u32> = Rc::new(RefCell::new(
        (1_u32 << ((*bits.borrow()).wrapping_sub(1_u32))),
    ));
    let max: Value<u32> = Rc::new(RefCell::new((!0_u32 >> 1)));
    let nc: Value<u32> = Rc::new(RefCell::new((*ones.borrow()).wrapping_sub(
        ((*ones.borrow()).wrapping_sub((*d.borrow()))).wrapping_rem((*d.borrow())),
    )));
    let a: Value<bool> = Rc::new(RefCell::new(false));
    let p: Value<u32> = Rc::new(RefCell::new((*bits.borrow()).wrapping_sub(1_u32)));
    let q1: Value<u32> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*nc.borrow()))));
    let r1: Value<u32> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q1.borrow()).wrapping_mul((*nc.borrow()))),
    ));
    let q2: Value<u32> = Rc::new(RefCell::new((*max.borrow()).wrapping_div((*d.borrow()))));
    let r2: Value<u32> = Rc::new(RefCell::new(
        (*max.borrow()).wrapping_sub((*q2.borrow()).wrapping_mul((*d.borrow()))),
    ));
    let delta: Value<u32> = <Value<u32>>::default();
    let mut __do_while = true;
    'loop_: while __do_while
        || (((*p.borrow()) < (*bits.borrow()).wrapping_mul(2_u32))
            && (((*q1.borrow()) < (*delta.borrow()))
                || (((*q1.borrow()) == (*delta.borrow())) && ((*r1.borrow()) == 0_u32))))
    {
        __do_while = false;
        let __rhs = (*p.borrow()).wrapping_add(1_u32);
        (*p.borrow_mut()) = __rhs;
        if ((*r1.borrow()) >= (*nc.borrow()).wrapping_sub((*r1.borrow()))) {
            let __rhs = ((2_u32).wrapping_mul((*q1.borrow()))).wrapping_add(1_u32);
            (*q1.borrow_mut()) = __rhs;
            let __rhs = ((2_u32).wrapping_mul((*r1.borrow()))).wrapping_sub((*nc.borrow()));
            (*r1.borrow_mut()) = __rhs;
        } else {
            let __rhs = (2_u32).wrapping_mul((*q1.borrow()));
            (*q1.borrow_mut()) = __rhs;
            let __rhs = (2_u32).wrapping_mul((*r1.borrow()));
            (*r1.borrow_mut()) = __rhs;
        }
        if ((*r2.borrow()).wrapping_add(1_u32) >= (*d.borrow()).wrapping_sub((*r2.borrow()))) {
            if ((*q2.borrow()) >= (*max.borrow())) {
                (*a.borrow_mut()) = true;
            }
            let __rhs = ((2_u32).wrapping_mul((*q2.borrow()))).wrapping_add(1_u32);
            (*q2.borrow_mut()) = __rhs;
            let __rhs = (((2_u32).wrapping_mul((*r2.borrow()))).wrapping_add(1_u32))
                .wrapping_sub((*d.borrow()));
            (*r2.borrow_mut()) = __rhs;
        } else {
            if ((*q2.borrow()) >= (*min.borrow())) {
                (*a.borrow_mut()) = true;
            }
            let __rhs = (2_u32).wrapping_mul((*q2.borrow()));
            (*q2.borrow_mut()) = __rhs;
            let __rhs = ((2_u32).wrapping_mul((*r2.borrow()))).wrapping_add(1_u32);
            (*r2.borrow_mut()) = __rhs;
        }
        (*delta.borrow_mut()) = ((*d.borrow()).wrapping_sub(1_u32)).wrapping_sub((*r2.borrow()));
    }
    return v8_base_MagicNumbersForDivision_unsigned_int_ :: v8_base_MagicNumbersForDivision_unsigned_int_ ( {  ( (*q2.borrow()) ) . wrapping_add ( 1_u32 )   } , {  ( (*p.borrow()) ) . wrapping_sub ( (*bits.borrow()) )   } , {  (*a.borrow())   } , )   ;
}
pub fn UnsignedDivisionByConstant_3(
    d: u64,
    leading_zeros: Option<u32>,
) -> v8_base_MagicNumbersForDivision_unsigned_long_long_ {
    let d: Value<u64> = Rc::new(RefCell::new(d));
    let leading_zeros: Value<u32> = Rc::new(RefCell::new(leading_zeros.unwrap_or()));
    (&(0));
    let bits: Value<u32> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<u64>() as u32) as u32).wrapping_mul(8_u32),
    ));
    let ones: Value<u64> = Rc::new(RefCell::new((!0_u64 >> (*leading_zeros.borrow()))));
    let min: Value<u64> = Rc::new(RefCell::new(
        (1_u64 << ((*bits.borrow()).wrapping_sub(1_u32))),
    ));
    let max: Value<u64> = Rc::new(RefCell::new((!0_u64 >> 1)));
    let nc: Value<u64> = Rc::new(RefCell::new((*ones.borrow()).wrapping_sub(
        ((*ones.borrow()).wrapping_sub((*d.borrow()))).wrapping_rem((*d.borrow())),
    )));
    let a: Value<bool> = Rc::new(RefCell::new(false));
    let p: Value<u32> = Rc::new(RefCell::new((*bits.borrow()).wrapping_sub(1_u32)));
    let q1: Value<u64> = Rc::new(RefCell::new((*min.borrow()).wrapping_div((*nc.borrow()))));
    let r1: Value<u64> = Rc::new(RefCell::new(
        (*min.borrow()).wrapping_sub((*q1.borrow()).wrapping_mul((*nc.borrow()))),
    ));
    let q2: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_div((*d.borrow()))));
    let r2: Value<u64> = Rc::new(RefCell::new(
        (*max.borrow()).wrapping_sub((*q2.borrow()).wrapping_mul((*d.borrow()))),
    ));
    let delta: Value<u64> = <Value<u64>>::default();
    let mut __do_while = true;
    'loop_: while __do_while
        || (((*p.borrow()) < (*bits.borrow()).wrapping_mul(2_u32))
            && (((*q1.borrow()) < (*delta.borrow()))
                || (((*q1.borrow()) == (*delta.borrow())) && ((*r1.borrow()) == 0_u64))))
    {
        __do_while = false;
        let __rhs = (*p.borrow()).wrapping_add(1_u32);
        (*p.borrow_mut()) = __rhs;
        if ((*r1.borrow()) >= (*nc.borrow()).wrapping_sub((*r1.borrow()))) {
            let __rhs = ((2_u64).wrapping_mul((*q1.borrow()))).wrapping_add(1_u64);
            (*q1.borrow_mut()) = __rhs;
            let __rhs = ((2_u64).wrapping_mul((*r1.borrow()))).wrapping_sub((*nc.borrow()));
            (*r1.borrow_mut()) = __rhs;
        } else {
            let __rhs = (2_u64).wrapping_mul((*q1.borrow()));
            (*q1.borrow_mut()) = __rhs;
            let __rhs = (2_u64).wrapping_mul((*r1.borrow()));
            (*r1.borrow_mut()) = __rhs;
        }
        if ((*r2.borrow()).wrapping_add(1_u64) >= (*d.borrow()).wrapping_sub((*r2.borrow()))) {
            if ((*q2.borrow()) >= (*max.borrow())) {
                (*a.borrow_mut()) = true;
            }
            let __rhs = ((2_u64).wrapping_mul((*q2.borrow()))).wrapping_add(1_u64);
            (*q2.borrow_mut()) = __rhs;
            let __rhs = (((2_u64).wrapping_mul((*r2.borrow()))).wrapping_add(1_u64))
                .wrapping_sub((*d.borrow()));
            (*r2.borrow_mut()) = __rhs;
        } else {
            if ((*q2.borrow()) >= (*min.borrow())) {
                (*a.borrow_mut()) = true;
            }
            let __rhs = (2_u64).wrapping_mul((*q2.borrow()));
            (*q2.borrow_mut()) = __rhs;
            let __rhs = ((2_u64).wrapping_mul((*r2.borrow()))).wrapping_add(1_u64);
            (*r2.borrow_mut()) = __rhs;
        }
        (*delta.borrow_mut()) = ((*d.borrow()).wrapping_sub(1_u64)).wrapping_sub((*r2.borrow()));
    }
    return v8_base_MagicNumbersForDivision_unsigned_long_long_ :: v8_base_MagicNumbersForDivision_unsigned_long_long_ ( {  ( (*q2.borrow()) ) . wrapping_add ( 1_u64 )   } , {  ( (*p.borrow()) ) . wrapping_sub ( (*bits.borrow()) )   } , {  (*a.borrow())   } , )   ;
}
pub trait v8_base_MagicNumbersForDivision_unsigned_int_Impl {
    fn operator_eq(&self, rhs: Ptr<v8_base_MagicNumbersForDivision_unsigned_int_>) -> bool;
}
impl v8_base_MagicNumbersForDivision_unsigned_int_Impl
    for Ptr<v8_base_MagicNumbersForDivision_unsigned_int_>
{
    fn operator_eq(&self, rhs: Ptr<v8_base_MagicNumbersForDivision_unsigned_int_>) -> bool {
        return (({
            let _lhs = (*(*(*self).upgrade().deref()).multiplier.borrow());
            _lhs == (*(*rhs.upgrade().deref()).multiplier.borrow())
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).shift.borrow());
            _lhs == (*(*rhs.upgrade().deref()).shift.borrow())
        })) && ({
            let _lhs = ((*(*(*self).upgrade().deref()).add.borrow()) as i32);
            _lhs == ((*(*rhs.upgrade().deref()).add.borrow()) as i32)
        });
    }
}
pub trait v8_base_MagicNumbersForDivision_unsigned_long_long_Impl {
    fn operator_eq(&self, rhs: Ptr<v8_base_MagicNumbersForDivision_unsigned_long_long_>) -> bool;
}
impl v8_base_MagicNumbersForDivision_unsigned_long_long_Impl
    for Ptr<v8_base_MagicNumbersForDivision_unsigned_long_long_>
{
    fn operator_eq(&self, rhs: Ptr<v8_base_MagicNumbersForDivision_unsigned_long_long_>) -> bool {
        return (({
            let _lhs = (*(*(*self).upgrade().deref()).multiplier.borrow());
            _lhs == (*(*rhs.upgrade().deref()).multiplier.borrow())
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).shift.borrow());
            _lhs == (*(*rhs.upgrade().deref()).shift.borrow())
        })) && ({
            let _lhs = ((*(*(*self).upgrade().deref()).add.borrow()) as i32);
            _lhs == ((*(*rhs.upgrade().deref()).add.borrow()) as i32)
        });
    }
}
