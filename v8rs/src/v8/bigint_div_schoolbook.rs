use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static kLog2DigitBits_0: Value<i32> = Rc::new(RefCell::new(6));
);
thread_local!(
    pub static kDigitBits_1: Value<i32> = Rc::new(RefCell::new(64));
);
#[derive()]
pub struct v8_bigint_Digits {
    digits_: Value<Ptr<u64>>,
    len_: Value<u32>,
}
impl v8_bigint_Digits {
    pub fn v8_bigint_Digits1(mem: Ptr<u64>, len: u32) -> Self {
        let mem: Value<Ptr<u64>> = Rc::new(RefCell::new(mem));
        let len: Value<u32> = Rc::new(RefCell::new(len));
        let __this: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Self {
            digits_: Rc::new(RefCell::new(Ptr::<u64>::null())),
            len_: <Value<u32>>::default(),
        }));
        let this: Ptr<v8_bigint_Digits> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_bigint_Digits2(mem: Ptr<u64>, len: u32) -> Self {
        let mem: Value<Ptr<u64>> = Rc::new(RefCell::new(mem));
        let len: Value<u32> = Rc::new(RefCell::new(len));
        let __this: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Self {
            digits_: Rc::new(RefCell::new((*mem.borrow()).clone())),
            len_: Rc::new(RefCell::new((*len.borrow()))),
        }));
        let this: Ptr<v8_bigint_Digits> = __this.as_pointer();
        (&(0));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_bigint_Digits3(src: v8_bigint_Digits, offset: u32, len: u32) -> Self {
        let src: Value<v8_bigint_Digits> = Rc::new(RefCell::new(src));
        let offset: Value<u32> = Rc::new(RefCell::new(offset));
        let len: Value<u32> = Rc::new(RefCell::new(len));
        let __this: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Self {
            digits_: Rc::new(RefCell::new(
                (*(*src.borrow()).digits_.borrow()).offset((*offset.borrow()) as isize),
            )),
            len_: Rc::new(RefCell::new({
                let __tmp_1: Value<u32> = Rc::new(RefCell::new(
                    if ((*(*src.borrow()).len_.borrow()) > (*offset.borrow())) {
                        (*(*src.borrow()).len_.borrow()).wrapping_sub((*offset.borrow()))
                    } else {
                        0_u32
                    },
                ));
                (if len.as_pointer().read() <= __tmp_1.as_pointer().read() {
                    len.as_pointer()
                } else {
                    __tmp_1.as_pointer()
                }
                .read())
            })),
        }));
        let this: Ptr<v8_bigint_Digits> = __this.as_pointer();
        (&(0));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_bigint_Digits4() -> Self {
        let __this: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Self {
            digits_: Rc::new(RefCell::new(Ptr::<u64>::null())),
            len_: <Value<u32>>::default(),
        }));
        let this: Ptr<v8_bigint_Digits> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for v8_bigint_Digits {
    fn eq(&self, other: &Self) -> bool {
        {
            v8_bigint_DigitsImpl::operator_eq(
                &Rc::new(RefCell::new(v8_bigint_Digits {
                    digits_: self.digits_.clone(),
                    len_: self.len_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(v8_bigint_Digits {
                    digits_: other.digits_.clone(),
                    len_: other.len_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for v8_bigint_Digits {}
impl Clone for v8_bigint_Digits {
    fn clone(&self) -> Self {
        let __this: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Self {
            digits_: Rc::new(RefCell::new((*self.digits_.borrow()).clone())),
            len_: Rc::new(RefCell::new((*self.len_.borrow()))),
        }));
        let this: Ptr<v8_bigint_Digits> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_bigint_Digits {
    fn default() -> Self {
        { v8_bigint_Digits::v8_bigint_Digits4() }
    }
}
impl ByteRepr for v8_bigint_Digits {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.digits_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.len_.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            digits_: Rc::new(RefCell::new(<Ptr<u64>>::from_bytes(&buf[0..8]))),
            len_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[8..12]))),
        }
    }
}
#[derive(Default)]
pub struct v8_bigint_RWDigits_WritableDigitReference {
    ptr_: Value<Ptr<u32>>,
}
impl v8_bigint_RWDigits_WritableDigitReference {
    fn v8_bigint_RWDigits_WritableDigitReference(ptr: Ptr<u64>) -> Self {
        let ptr: Value<Ptr<u64>> = Rc::new(RefCell::new(ptr));
        let __this: Value<v8_bigint_RWDigits_WritableDigitReference> =
            Rc::new(RefCell::new(Self {
                ptr_: Rc::new(RefCell::new((*ptr.borrow()).reinterpret_cast::<u32>())),
            }));
        let this: Ptr<v8_bigint_RWDigits_WritableDigitReference> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_bigint_RWDigits_WritableDigitReference {
    fn clone(&self) -> Self {
        let __this: Value<v8_bigint_RWDigits_WritableDigitReference> =
            Rc::new(RefCell::new(Self {
                ptr_: Rc::new(RefCell::new(Ptr::<u32>::null())),
            }));
        let this: Ptr<v8_bigint_RWDigits_WritableDigitReference> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_RWDigits_WritableDigitReference {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.ptr_.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ptr_: Rc::new(RefCell::new(<Ptr<u32>>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct v8_bigint_RWDigits {}
impl v8_bigint_RWDigits {
    pub fn v8_bigint_RWDigits1(mem: Ptr<u64>, len: u32) -> Self {
        let mem: Value<Ptr<u64>> = Rc::new(RefCell::new(mem));
        let len: Value<u32> = Rc::new(RefCell::new(len));
        let __this: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_bigint_RWDigits> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_bigint_RWDigits2(src: v8_bigint_RWDigits, offset: u32, len: u32) -> Self {
        let src: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(src));
        let offset: Value<u32> = Rc::new(RefCell::new(offset));
        let len: Value<u32> = Rc::new(RefCell::new(len));
        let __this: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_bigint_RWDigits> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_bigint_RWDigits {
    fn clone(&self) -> Self {
        let __this: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_bigint_RWDigits> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_RWDigits {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub trait v8_bigint_Platform {
    fn Allocate(&self, count: usize) -> Ptr<u64>;
    fn Free(&self, ptr: Ptr<u64>);
    fn InterruptRequested(&self) -> bool;
}
#[derive(Default)]
pub struct v8_bigint_DefaultPlatform {}
impl v8_bigint_Platform for v8_bigint_DefaultPlatform {
    fn Allocate(&self, count: usize) -> Ptr<u64> {
        let count: Value<usize> = Rc::new(RefCell::new(count));
        return Ptr::alloc_array(
            (0..(*count.borrow()))
                .map(|_| <u64>::default())
                .collect::<Box<[u64]>>(),
        );
    }
    fn Free(&self, ptr: Ptr<u64>) {
        let ptr: Value<Ptr<u64>> = Rc::new(RefCell::new(ptr));
        (*ptr.borrow()).delete_array();
    }
    fn InterruptRequested(&self) -> bool {
        return false;
    }
}
impl Clone for v8_bigint_DefaultPlatform {
    fn clone(&self) -> Self {
        let __this: Value<v8_bigint_DefaultPlatform> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_bigint_DefaultPlatform> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_DefaultPlatform {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_bigint_RightShiftState {
    pub must_round_down: Value<bool>,
}
impl Clone for v8_bigint_RightShiftState {
    fn clone(&self) -> Self {
        let __this: Value<v8_bigint_RightShiftState> = Rc::new(RefCell::new(Self {
            must_round_down: Rc::new(RefCell::new((*self.must_round_down.borrow()))),
        }));
        let this: Ptr<v8_bigint_RightShiftState> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_RightShiftState {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.must_round_down.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            must_round_down: Rc::new(RefCell::new(<bool>::from_bytes(&buf[0..1]))),
        }
    }
}
pub type v8_bigint_Status = i32;
pub const v8_bigint_Status_kOk: v8_bigint_Status = 0;
pub const v8_bigint_Status_kInterrupted: v8_bigint_Status = 1;
thread_local!(
    pub static kMaxCachedModDivisorSize_2: Value<u32> = Rc::new(RefCell::new(32));
);
thread_local!(
    static kSmallScratchSize_3: Value<u32> = Rc::new(RefCell::new(100));
);
#[derive(Default)]
pub struct v8_bigint_Processor_Destroyer {}
impl Clone for v8_bigint_Processor_Destroyer {
    fn clone(&self) -> Self {
        let __this: Value<v8_bigint_Processor_Destroyer> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<v8_bigint_Processor_Destroyer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_Processor_Destroyer {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct v8_bigint_Processor {
    platform_: Value<Option<Value<v8_bigint_Platform>>>,
    status_: Value<v8_bigint_Status>,
    work_estimate_: Value<u64>,
    small_scratch_: Value<Option<Value<Box<[u64]>>>>,
    cached_inverse_storage_: Value<Option<Value<Box<[u64]>>>>,
    cached_divisor_: Value<v8_bigint_RWDigits>,
    cached_inverse_: Value<v8_bigint_RWDigits>,
    cached_inverse_allocated_length_: Value<u32>,
    divisor_count_: Value<i32>,
    cached_mod_fold_factor_: Value<u64>,
}
impl v8_bigint_Processor {
    pub fn New(platform: PtrDyn<dyn v8_bigint_Platform>) -> Ptr<v8_bigint_Processor> {
        let platform: Value<PtrDyn<dyn v8_bigint_Platform>> = Rc::new(RefCell::new(platform));
        return Ptr::alloc(v8_bigint_Processor::v8_bigint_Processor({
            (*platform.borrow()).clone()
        }));
    }
    fn v8_bigint_Processor(platform: PtrDyn<dyn v8_bigint_Platform>) -> Self {
        let platform: Value<PtrDyn<dyn v8_bigint_Platform>> = Rc::new(RefCell::new(platform));
        let __this : Value<v8_bigint_Processor> = Rc::new(RefCell::new(Self { platform_ : Rc::new(RefCell::new((*platform.borrow()) .to_owned_opt() )) , status_ : Rc::new(RefCell::new(v8_bigint_Status_kOk  )) , work_estimate_ : Rc::new(RefCell::new(0_u64  )) , small_scratch_ : Rc::new(RefCell::new(std_unique_ptr_unsigned_longarrarr__v8_bigint_Platform_Deleter_ :: std_unique_ptr_unsigned_longarrarr__v8_bigint_Platform_Deleter_1 ( {  Default::default()   } , { let __tmp_0 : Value<v8_bigint_Platform_Deleter > = Rc::new(RefCell::new(v8_bigint_Platform_Deleter :: v8_bigint_Platform_Deleter ( {  ((*platform.borrow()) ).clone()  } , ) )); __tmp_0.as_pointer()  } , ) )) , cached_inverse_storage_ : Rc::new(RefCell::new(std_unique_ptr_unsigned_longarrarr__v8_bigint_Platform_Deleter_ :: std_unique_ptr_unsigned_longarrarr__v8_bigint_Platform_Deleter_1 ( {  Default::default()   } , { let __tmp_1 : Value<v8_bigint_Platform_Deleter > = Rc::new(RefCell::new(v8_bigint_Platform_Deleter :: v8_bigint_Platform_Deleter ( {  ((*platform.borrow()) ).clone()  } , ) )); __tmp_1.as_pointer()  } , ) )) , cached_divisor_ : Rc::new(RefCell::new(v8_bigint_RWDigits :: v8_bigint_RWDigits1 ( {  Ptr::<u64>::null()   } , {  0_u32   } , ) )) , cached_inverse_ : Rc::new(RefCell::new(v8_bigint_RWDigits :: v8_bigint_RWDigits1 ( {  Ptr::<u64>::null()   } , {  0_u32   } , ) )) , cached_inverse_allocated_length_ : Rc::new(RefCell::new(0_u32  )) , divisor_count_ : Rc::new(RefCell::new(0  )) , cached_mod_fold_factor_ : Rc::new(RefCell::new(0_u64  )) , } )) ;
        let this: Ptr<v8_bigint_Processor> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_Processor {
    fn byte_size() -> usize {
        104
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.platform_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.status_.borrow()).to_bytes(&mut buf[8..12]);
        (*self.work_estimate_.borrow()).to_bytes(&mut buf[16..24]);
        (*self.small_scratch_.borrow()).to_bytes(&mut buf[24..40]);
        (*self.cached_inverse_storage_.borrow()).to_bytes(&mut buf[40..56]);
        (*self.cached_divisor_.borrow()).to_bytes(&mut buf[56..72]);
        (*self.cached_inverse_.borrow()).to_bytes(&mut buf[72..88]);
        (*self.cached_inverse_allocated_length_.borrow()).to_bytes(&mut buf[88..92]);
        (*self.divisor_count_.borrow()).to_bytes(&mut buf[92..96]);
        (*self.cached_mod_fold_factor_.borrow()).to_bytes(&mut buf[96..104]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            platform_: Rc::new(RefCell::new(
                <Option<Value<v8_bigint_Platform>>>::from_bytes(&buf[0..8]),
            )),
            status_: Rc::new(RefCell::new(<v8_bigint_Status>::from_bytes(&buf[8..12]))),
            work_estimate_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[16..24]))),
            small_scratch_: Rc::new(RefCell::new(<Option<Value<Box<[u64]>>>>::from_bytes(
                &buf[24..40],
            ))),
            cached_inverse_storage_: Rc::new(RefCell::new(
                <Option<Value<Box<[u64]>>>>::from_bytes(&buf[40..56]),
            )),
            cached_divisor_: Rc::new(RefCell::new(<v8_bigint_RWDigits>::from_bytes(&buf[56..72]))),
            cached_inverse_: Rc::new(RefCell::new(<v8_bigint_RWDigits>::from_bytes(&buf[72..88]))),
            cached_inverse_allocated_length_: Rc::new(RefCell::new(<u32>::from_bytes(
                &buf[88..92],
            ))),
            divisor_count_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[92..96]))),
            cached_mod_fold_factor_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[96..104]))),
        }
    }
}
pub fn AddResultLength_4(x_length: u32, y_length: u32) -> u32 {
    let x_length: Value<u32> = Rc::new(RefCell::new(x_length));
    let y_length: Value<u32> = Rc::new(RefCell::new(y_length));
    return (if x_length.as_pointer().read() >= y_length.as_pointer().read() {
        x_length.as_pointer()
    } else {
        y_length.as_pointer()
    }
    .read())
    .wrapping_add(1_u32);
}
pub fn AddSignedResultLength_5(x_length: u32, y_length: u32, same_sign: bool) -> u32 {
    let x_length: Value<u32> = Rc::new(RefCell::new(x_length));
    let y_length: Value<u32> = Rc::new(RefCell::new(y_length));
    let same_sign: Value<bool> = Rc::new(RefCell::new(same_sign));
    return if (*same_sign.borrow()) {
        ({ AddResultLength_4((*x_length.borrow()), (*y_length.borrow())) })
    } else {
        (if x_length.as_pointer().read() >= y_length.as_pointer().read() {
            x_length.as_pointer()
        } else {
            y_length.as_pointer()
        }
        .read())
    };
}
pub fn SubtractResultLength_6(x_length: u32, y_length: u32) -> u32 {
    let x_length: Value<u32> = Rc::new(RefCell::new(x_length));
    let y_length: Value<u32> = Rc::new(RefCell::new(y_length));
    return (*x_length.borrow());
}
pub fn SubtractSignedResultLength_7(x_length: u32, y_length: u32, same_sign: bool) -> u32 {
    let x_length: Value<u32> = Rc::new(RefCell::new(x_length));
    let y_length: Value<u32> = Rc::new(RefCell::new(y_length));
    let same_sign: Value<bool> = Rc::new(RefCell::new(same_sign));
    return if (*same_sign.borrow()) {
        (if x_length.as_pointer().read() >= y_length.as_pointer().read() {
            x_length.as_pointer()
        } else {
            y_length.as_pointer()
        }
        .read())
    } else {
        ({ AddResultLength_4((*x_length.borrow()), (*y_length.borrow())) })
    };
}
pub fn MultiplyResultLength_8(X: v8_bigint_Digits, Y: v8_bigint_Digits) -> u32 {
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    (&(0));
    return ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })
        .wrapping_add(({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }));
}
pub fn ModuloResultLength_9(B: v8_bigint_Digits) -> u32 {
    let B: Value<v8_bigint_Digits> = Rc::new(RefCell::new(B));
    return ({ v8_bigint_DigitsImpl::len(&B.as_pointer()) });
}
thread_local!(
    pub static kStringZapValue_10: Value<u8> = Rc::new(RefCell::new(63));
);
pub fn AsUintN_Neg_ResultLength_11(n: u32) -> u32 {
    let n: Value<u32> = Rc::new(RefCell::new(n));
    return (((*n.borrow()).wrapping_sub(1_u32)).wrapping_div((64 as u32))).wrapping_add(1_u32);
}
thread_local!(
    pub static kWorkEstimateThreshold_12: Value<u64> = Rc::new(RefCell::new(5000000_u64));
);
#[derive(Default)]
pub struct v8_bigint_ProcessorImpl {}
impl ByteRepr for v8_bigint_ProcessorImpl {
    fn byte_size() -> usize {
        104
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static kMaxNumDigits_13: Value<u32> = Rc::new(RefCell::new(67108863));
);
#[derive(Default)]
pub struct v8_bigint_Storage {
    ptr_: Value<Option<Value<Box<[u64]>>>>,
}
impl v8_bigint_Storage {
    pub fn v8_bigint_Storage(count: u32, platform: PtrDyn<dyn v8_bigint_Platform>) -> Self {
        let count: Value<u32> = Rc::new(RefCell::new(count));
        let platform: Value<PtrDyn<dyn v8_bigint_Platform>> = Rc::new(RefCell::new(platform));
        let __this : Value<v8_bigint_Storage> = Rc::new(RefCell::new(Self { ptr_ : Rc::new(RefCell::new(std_unique_ptr_unsigned_longarrarr__v8_bigint_Platform_Deleter_ :: std_unique_ptr_unsigned_longarrarr__v8_bigint_Platform_Deleter_2 ( {  (  { (*(*platform.borrow()) .upgrade().deref()) . Allocate ( ( ( (*count.borrow()) as usize ) )  , ) } )    } , { let __tmp_2 : Value<v8_bigint_Platform_Deleter > = Rc::new(RefCell::new(v8_bigint_Platform_Deleter :: v8_bigint_Platform_Deleter ( {  ((*platform.borrow()) ).clone()  } , ) )); __tmp_2.as_pointer()  } , ) )) , } )) ;
        let this: Ptr<v8_bigint_Storage> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Storage_pmutv8_bigint_Storage(_a0: Ptr<v8_bigint_Storage>) -> Self {
        let __this: Value<v8_bigint_Storage> = Rc::new(RefCell::new(Self {
            ptr_: Rc::new(RefCell::new(
                (*(*_a0.upgrade().deref()).ptr_.borrow_mut()).clone(),
            )),
        }));
        let this: Ptr<v8_bigint_Storage> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_Storage {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.ptr_.borrow()).to_bytes(&mut buf[0..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ptr_: Rc::new(RefCell::new(<Option<Value<Box<[u64]>>>>::from_bytes(
                &buf[0..16],
            ))),
        }
    }
}
#[derive(Default)]
pub struct v8_bigint_ScratchDigits {
    storage_: Value<v8_bigint_Storage>,
}
impl v8_bigint_ScratchDigits {
    pub fn v8_bigint_ScratchDigits(len: u32, platform: PtrDyn<dyn v8_bigint_Platform>) -> Self {
        let len: Value<u32> = Rc::new(RefCell::new(len));
        let platform: Value<PtrDyn<dyn v8_bigint_Platform>> = Rc::new(RefCell::new(platform));
        let __this: Value<v8_bigint_ScratchDigits> = Rc::new(RefCell::new(Self {
            storage_: Rc::new(RefCell::new(v8_bigint_Storage::v8_bigint_Storage(
                { (*len.borrow()) },
                { (*platform.borrow()).clone() },
            ))),
        }));
        let this: Ptr<v8_bigint_ScratchDigits> = __this.as_pointer();
        (*(*this.upgrade().deref()).digits_.borrow_mut()) =
            ({ v8_bigint_StorageImpl::get(&(*this.upgrade().deref()).storage_.as_pointer()) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_ScratchDigits {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.storage_.borrow()).to_bytes(&mut buf[16..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            storage_: Rc::new(RefCell::new(<v8_bigint_Storage>::from_bytes(&buf[16..32]))),
        }
    }
}
thread_local!(
    pub static kKaratsubaThreshold_14: Value<u32> = Rc::new(RefCell::new(34));
);
thread_local!(
    pub static kBurnikelThreshold_15: Value<u32> = Rc::new(RefCell::new(57));
);
thread_local!(
    pub static kNewtonInversionThreshold_16: Value<u32> = Rc::new(RefCell::new(25));
);
thread_local!(
    pub static kToomThreshold_17: Value<u32> = Rc::new(RefCell::new(210));
);
thread_local!(
    pub static kFftThreshold_18: Value<u32> = Rc::new(RefCell::new(720));
);
thread_local!(
    pub static kFftInnerThreshold_19: Value<u32> = Rc::new(RefCell::new(200));
);
thread_local!(
    pub static kBarrettThreshold_20: Value<u32> = Rc::new(RefCell::new(13000));
);
thread_local!(
    pub static kToStringFastThreshold_21: Value<u32> = Rc::new(RefCell::new(23));
);
thread_local!(
    pub static kFromStringLargeThreshold_22: Value<u32> = Rc::new(RefCell::new(25));
);
#[derive(Default)]
pub struct v8_bigint_GrowableDigitsVector {
    data_: Value<Ptr<u64>>,
    end_: Value<Ptr<u64>>,
    capacity_: Value<Ptr<u64>>,
    platform_: Value<PtrDyn<dyn v8_bigint_Platform>>,
}
impl v8_bigint_GrowableDigitsVector {
    pub fn v8_bigint_GrowableDigitsVector(platform: PtrDyn<dyn v8_bigint_Platform>) -> Self {
        let platform: Value<PtrDyn<dyn v8_bigint_Platform>> = Rc::new(RefCell::new(platform));
        let __this: Value<v8_bigint_GrowableDigitsVector> = Rc::new(RefCell::new(Self {
            data_: Rc::new(RefCell::new(Ptr::<u64>::null())),
            end_: Rc::new(RefCell::new(Ptr::<u64>::null())),
            capacity_: Rc::new(RefCell::new(Ptr::<u64>::null())),
            platform_: Rc::new(RefCell::new((*platform.borrow()).clone())),
        }));
        let this: Ptr<v8_bigint_GrowableDigitsVector> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_bigint_GrowableDigitsVector {
    fn clone(&self) -> Self {
        let __this: Value<v8_bigint_GrowableDigitsVector> = Rc::new(RefCell::new(Self {
            data_: Rc::new(RefCell::new((*self.data_.borrow()).clone())),
            end_: Rc::new(RefCell::new((*self.end_.borrow()).clone())),
            capacity_: Rc::new(RefCell::new((*self.capacity_.borrow()).clone())),
            platform_: Rc::new(RefCell::new((*self.platform_.borrow()).clone())),
        }));
        let this: Ptr<v8_bigint_GrowableDigitsVector> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_GrowableDigitsVector {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.end_.borrow()).to_bytes(&mut buf[8..16]);
        (*self.capacity_.borrow()).to_bytes(&mut buf[16..24]);
        (*self.platform_.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data_: Rc::new(RefCell::new(<Ptr<u64>>::from_bytes(&buf[0..8]))),
            end_: Rc::new(RefCell::new(<Ptr<u64>>::from_bytes(&buf[8..16]))),
            capacity_: Rc::new(RefCell::new(<Ptr<u64>>::from_bytes(&buf[16..24]))),
            platform_: Rc::new(RefCell::new(<PtrDyn<dyn v8_bigint_Platform>>::from_bytes(
                &buf[24..32],
            ))),
        }
    }
}
pub type v8_bigint_FromStringAccumulator_Result = i32;
pub const v8_bigint_FromStringAccumulator_Result_kOk: v8_bigint_FromStringAccumulator_Result = 0;
pub const v8_bigint_FromStringAccumulator_Result_kMaxSizeExceeded:
    v8_bigint_FromStringAccumulator_Result = 1;
thread_local!(
    pub static kStackParts_23: Value<u32> = Rc::new(RefCell::new(8));
);
#[derive()]
pub struct v8_bigint_FromStringAccumulator {
    stack_parts_: Value<Box<[u64]>>,
    heap_parts_: Value<v8_bigint_GrowableDigitsVector>,
    max_multiplier_: Value<u64>,
    last_multiplier_: Value<u64>,
    max_digits_: Value<u32>,
    result_: Value<v8_bigint_FromStringAccumulator_Result>,
    stack_parts_used_: Value<u32>,
    inline_everything_: Value<bool>,
    radix_: Value<u8>,
}
impl v8_bigint_FromStringAccumulator {
    pub fn v8_bigint_FromStringAccumulator(
        max_digits: u32,
        platform: PtrDyn<dyn v8_bigint_Platform>,
    ) -> Self {
        let max_digits: Value<u32> = Rc::new(RefCell::new(max_digits));
        let platform: Value<PtrDyn<dyn v8_bigint_Platform>> = Rc::new(RefCell::new(platform));
        let __this: Value<v8_bigint_FromStringAccumulator> = Rc::new(RefCell::new(Self {
            stack_parts_: Rc::new(RefCell::new(
                (0..8).map(|_| <u64>::default()).collect::<Box<[u64]>>(),
            )),
            heap_parts_: Rc::new(RefCell::new(
                v8_bigint_GrowableDigitsVector::v8_bigint_GrowableDigitsVector({
                    (*platform.borrow()).clone()
                }),
            )),
            max_multiplier_: Rc::new(RefCell::new(0_u64)),
            last_multiplier_: <Value<u64>>::default(),
            max_digits_: Rc::new(RefCell::new(
                (if max_digits.as_pointer().read()
                    >= kStackParts_23.with(Value::clone).as_pointer().read()
                {
                    max_digits.as_pointer()
                } else {
                    kStackParts_23.with(Value::clone).as_pointer()
                }
                .read()),
            )),
            result_: Rc::new(RefCell::new(v8_bigint_FromStringAccumulator_Result_kOk)),
            stack_parts_used_: Rc::new(RefCell::new(0_u32)),
            inline_everything_: Rc::new(RefCell::new(false)),
            radix_: Rc::new(RefCell::new(0_u8)),
        }));
        let this: Ptr<v8_bigint_FromStringAccumulator> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_bigint_FromStringAccumulator {
    fn clone(&self) -> Self {
        let __this: Value<v8_bigint_FromStringAccumulator> = Rc::new(RefCell::new(Self {
            stack_parts_: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 8, _>(
                |__i: usize| (*self.stack_parts_.borrow())[(__i) as usize],
            )))),
            heap_parts_: Rc::new(RefCell::new((*self.heap_parts_.borrow()).clone())),
            max_multiplier_: Rc::new(RefCell::new((*self.max_multiplier_.borrow()))),
            last_multiplier_: Rc::new(RefCell::new((*self.last_multiplier_.borrow()))),
            max_digits_: Rc::new(RefCell::new((*self.max_digits_.borrow()))),
            result_: Rc::new(RefCell::new((*self.result_.borrow()))),
            stack_parts_used_: Rc::new(RefCell::new((*self.stack_parts_used_.borrow()))),
            inline_everything_: Rc::new(RefCell::new((*self.inline_everything_.borrow()))),
            radix_: Rc::new(RefCell::new((*self.radix_.borrow()))),
        }));
        let this: Ptr<v8_bigint_FromStringAccumulator> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_bigint_FromStringAccumulator {
    fn default() -> Self {
        v8_bigint_FromStringAccumulator {
            stack_parts_: Rc::new(RefCell::new(
                (0..8).map(|_| <u64>::default()).collect::<Box<[u64]>>(),
            )),
            heap_parts_: <Value<v8_bigint_GrowableDigitsVector>>::default(),
            max_multiplier_: <Value<u64>>::default(),
            last_multiplier_: <Value<u64>>::default(),
            max_digits_: <Value<u32>>::default(),
            result_: <Value<v8_bigint_FromStringAccumulator_Result>>::default(),
            stack_parts_used_: <Value<u32>>::default(),
            inline_everything_: <Value<bool>>::default(),
            radix_: <Value<u8>>::default(),
        }
    }
}
impl ByteRepr for v8_bigint_FromStringAccumulator {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.stack_parts_.borrow()).to_bytes(&mut buf[0..64]);
        (*self.heap_parts_.borrow()).to_bytes(&mut buf[64..96]);
        (*self.max_multiplier_.borrow()).to_bytes(&mut buf[96..104]);
        (*self.last_multiplier_.borrow()).to_bytes(&mut buf[104..112]);
        (*self.max_digits_.borrow()).to_bytes(&mut buf[112..116]);
        (*self.result_.borrow()).to_bytes(&mut buf[116..120]);
        (*self.stack_parts_used_.borrow()).to_bytes(&mut buf[120..124]);
        (*self.inline_everything_.borrow()).to_bytes(&mut buf[124..125]);
        (*self.radix_.borrow()).to_bytes(&mut buf[125..126]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            stack_parts_: Rc::new(RefCell::new(<Box<[u64]>>::from_bytes(&buf[0..64]))),
            heap_parts_: Rc::new(RefCell::new(<v8_bigint_GrowableDigitsVector>::from_bytes(
                &buf[64..96],
            ))),
            max_multiplier_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[96..104]))),
            last_multiplier_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[104..112]))),
            max_digits_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[112..116]))),
            result_: Rc::new(RefCell::new(
                <v8_bigint_FromStringAccumulator_Result>::from_bytes(&buf[116..120]),
            )),
            stack_parts_used_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[120..124]))),
            inline_everything_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[124..125]))),
            radix_: Rc::new(RefCell::new(<u8>::from_bytes(&buf[125..126]))),
        }
    }
}
thread_local!(
    pub static kCharValue_24: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8,
        255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8,
        255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8,
        255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8,
        255_u8, 255_u8, 255_u8, 255_u8, 0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8,
        255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8, 10_u8, 11_u8, 12_u8, 13_u8, 14_u8,
        15_u8, 16_u8, 17_u8, 18_u8, 19_u8, 20_u8, 21_u8, 22_u8, 23_u8, 24_u8, 25_u8, 26_u8, 27_u8,
        28_u8, 29_u8, 30_u8, 31_u8, 32_u8, 33_u8, 34_u8, 35_u8, 255_u8, 255_u8, 255_u8, 255_u8,
        255_u8, 255_u8, 10_u8, 11_u8, 12_u8, 13_u8, 14_u8, 15_u8, 16_u8, 17_u8, 18_u8, 19_u8,
        20_u8, 21_u8, 22_u8, 23_u8, 24_u8, 25_u8, 26_u8, 27_u8, 28_u8, 29_u8, 30_u8, 31_u8, 32_u8,
        33_u8, 34_u8, 35_u8, 255_u8, 255_u8, 255_u8, 255_u8, 255_u8,
    ])));
);
thread_local!(
    pub static kCharBits_25: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        1_u8, 2_u8, 3_u8, 0_u8, 4_u8, 0_u8, 0_u8, 0_u8, 5_u8,
    ])));
);
pub fn CountLeadingZeros_26(value: u64) -> i32 {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u64) {
        64
    } else {
        ({ __builtin_clzll_27((*value.borrow())) })
    };
}
pub fn CountLeadingZeros_28(value: u32) -> i32 {
    let value: Value<u32> = Rc::new(RefCell::new(value));
    return if ((*value.borrow()) == 0_u32) {
        32
    } else {
        (*value.borrow()).leading_zeros() as i32
    };
}
pub fn BitLength_29(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return (32 - ({ CountLeadingZeros_28(((*n.borrow()) as u32)) }));
}
thread_local!(
    pub static kHalfDigitBits_30: Value<i32> = Rc::new(RefCell::new(32));
);
thread_local!(
    pub static kHalfDigitBase_31: Value<u64> = Rc::new(RefCell::new(4294967296));
);
thread_local!(
    pub static kHalfDigitMask_32: Value<u64> = Rc::new(RefCell::new(4294967295));
);
pub fn digit_ismax_33(x: u64) -> bool {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    return (!(*x.borrow()) == 0_u64);
}
pub fn digit_add2_34(a: u64, b: u64, carry: Ptr<u64>) -> u64 {
    let a: Value<u64> = Rc::new(RefCell::new(a));
    let b: Value<u64> = Rc::new(RefCell::new(b));
    let carry: Value<Ptr<u64>> = Rc::new(RefCell::new(carry));
    let result: Value<u128> = Rc::new(RefCell::new(
        ((*a.borrow()) as u128).wrapping_add(((*b.borrow()) as u128)),
    ));
    let __rhs = (((*result.borrow()) >> 64) as u64);
    (*carry.borrow()).write(__rhs);
    return ((*result.borrow()) as u64);
}
pub fn digit_add3_35(a: u64, b: u64, c: u64, carry: Ptr<u64>) -> u64 {
    let a: Value<u64> = Rc::new(RefCell::new(a));
    let b: Value<u64> = Rc::new(RefCell::new(b));
    let c: Value<u64> = Rc::new(RefCell::new(c));
    let carry: Value<Ptr<u64>> = Rc::new(RefCell::new(carry));
    let result: Value<u128> = Rc::new(RefCell::new(
        (((*a.borrow()) as u128).wrapping_add(((*b.borrow()) as u128)))
            .wrapping_add(((*c.borrow()) as u128)),
    ));
    let __rhs = (((*result.borrow()) >> 64) as u64);
    (*carry.borrow()).write(__rhs);
    return ((*result.borrow()) as u64);
}
pub fn digit_sub_36(a: u64, b: u64, borrow: Ptr<u64>) -> u64 {
    let a: Value<u64> = Rc::new(RefCell::new(a));
    let b: Value<u64> = Rc::new(RefCell::new(b));
    let borrow: Value<Ptr<u64>> = Rc::new(RefCell::new(borrow));
    let result: Value<u128> = Rc::new(RefCell::new(
        ((*a.borrow()) as u128).wrapping_sub(((*b.borrow()) as u128)),
    ));
    let __rhs = ((((*result.borrow()) >> 64) & 1_u128) as u64);
    (*borrow.borrow()).write(__rhs);
    return ((*result.borrow()) as u64);
}
pub fn digit_sub2_37(a: u64, b: u64, borrow_in: u64, borrow_out: Ptr<u64>) -> u64 {
    let a: Value<u64> = Rc::new(RefCell::new(a));
    let b: Value<u64> = Rc::new(RefCell::new(b));
    let borrow_in: Value<u64> = Rc::new(RefCell::new(borrow_in));
    let borrow_out: Value<Ptr<u64>> = Rc::new(RefCell::new(borrow_out));
    let subtrahend: Value<u128> = Rc::new(RefCell::new(
        ((*b.borrow()) as u128).wrapping_add(((*borrow_in.borrow()) as u128)),
    ));
    let result: Value<u128> = Rc::new(RefCell::new(
        ((*a.borrow()) as u128).wrapping_sub((*subtrahend.borrow())),
    ));
    let __rhs = ((((*result.borrow()) >> 64) & 1_u128) as u64);
    (*borrow_out.borrow()).write(__rhs);
    return ((*result.borrow()) as u64);
}
pub fn digit_mul_38(a: u64, b: u64, high: Ptr<u64>) -> u64 {
    let a: Value<u64> = Rc::new(RefCell::new(a));
    let b: Value<u64> = Rc::new(RefCell::new(b));
    let high: Value<Ptr<u64>> = Rc::new(RefCell::new(high));
    let result: Value<u128> = Rc::new(RefCell::new(
        ((*a.borrow()) as u128).wrapping_mul(((*b.borrow()) as u128)),
    ));
    let __rhs = (((*result.borrow()) >> 64) as u64);
    (*high.borrow()).write(__rhs);
    return ((*result.borrow()) as u64);
}
pub fn digit_div_39(high: u64, low: u64, divisor: u64, remainder: Ptr<u64>) -> u64 {
    let high: Value<u64> = Rc::new(RefCell::new(high));
    let low: Value<u64> = Rc::new(RefCell::new(low));
    let divisor: Value<u64> = Rc::new(RefCell::new(divisor));
    let remainder: Value<Ptr<u64>> = Rc::new(RefCell::new(remainder));
    (&(0));
    (&(0));
    let s: Value<i32> = Rc::new(RefCell::new(
        ({ CountLeadingZeros_26((*divisor.borrow())) }),
    ));
    (&(0));
    (*divisor.borrow_mut()) <<= (*s.borrow());
    let vn1: Value<u64> = Rc::new(RefCell::new(((*divisor.borrow()) >> 32)));
    let vn0: Value<u64> = Rc::new(RefCell::new(((*divisor.borrow()) & 4294967295)));
    let kShiftMask: Value<i32> = Rc::new(RefCell::new((64 - 1)));
    let s_zero_mask: Value<u64> =
        Rc::new(RefCell::new((((-(*s.borrow()) as i64) >> (64 - 1)) as u64)));
    let un32: Value<u64> = Rc::new(RefCell::new(
        (((*high.borrow()) << (*s.borrow()))
            | (((*low.borrow()) >> ((64 - (*s.borrow())) & (*kShiftMask.borrow())))
                & (*s_zero_mask.borrow()))),
    ));
    let un10: Value<u64> = Rc::new(RefCell::new(((*low.borrow()) << (*s.borrow()))));
    let un1: Value<u64> = Rc::new(RefCell::new(((*un10.borrow()) >> 32)));
    let un0: Value<u64> = Rc::new(RefCell::new(((*un10.borrow()) & 4294967295)));
    let q1: Value<u64> = Rc::new(RefCell::new((*un32.borrow()).wrapping_div((*vn1.borrow()))));
    let rhat: Value<u64> = Rc::new(RefCell::new(
        (*un32.borrow()).wrapping_sub((*q1.borrow()).wrapping_mul((*vn1.borrow()))),
    ));
    'loop_: while ((*q1.borrow()) >= 4294967296)
        || ((*q1.borrow()).wrapping_mul((*vn0.borrow()))
            > ((*rhat.borrow()).wrapping_mul(4294967296)).wrapping_add((*un1.borrow())))
    {
        (*q1.borrow_mut()).postfix_dec();
        {
            let rhs_0 = (*rhat.borrow()).wrapping_add((*vn1.borrow()));
            (*rhat.borrow_mut()) = rhs_0
        };
        if ((*rhat.borrow()) >= 4294967296) {
            break;
        }
    }
    let un21: Value<u64> = Rc::new(RefCell::new(
        (((*un32.borrow()).wrapping_mul(4294967296)).wrapping_add((*un1.borrow())))
            .wrapping_sub((*q1.borrow()).wrapping_mul((*divisor.borrow()))),
    ));
    let q0: Value<u64> = Rc::new(RefCell::new((*un21.borrow()).wrapping_div((*vn1.borrow()))));
    (*rhat.borrow_mut()) =
        (*un21.borrow()).wrapping_sub((*q0.borrow()).wrapping_mul((*vn1.borrow())));
    'loop_: while ((*q0.borrow()) >= 4294967296)
        || ((*q0.borrow()).wrapping_mul((*vn0.borrow()))
            > ((*rhat.borrow()).wrapping_mul(4294967296)).wrapping_add((*un0.borrow())))
    {
        (*q0.borrow_mut()).postfix_dec();
        {
            let rhs_0 = (*rhat.borrow()).wrapping_add((*vn1.borrow()));
            (*rhat.borrow_mut()) = rhs_0
        };
        if ((*rhat.borrow()) >= 4294967296) {
            break;
        }
    }
    let __rhs = (((((*un21.borrow()).wrapping_mul(4294967296)).wrapping_add((*un0.borrow())))
        .wrapping_sub((*q0.borrow()).wrapping_mul((*divisor.borrow()))))
        >> (*s.borrow()));
    (*remainder.borrow()).write(__rhs);
    return ((*q1.borrow()).wrapping_mul(4294967296)).wrapping_add((*q0.borrow()));
}
pub fn GreaterThanOrEqual_40(A: v8_bigint_Digits, B: v8_bigint_Digits) -> bool {
    let A: Value<v8_bigint_Digits> = Rc::new(RefCell::new(A));
    let B: Value<v8_bigint_Digits> = Rc::new(RefCell::new(B));
    return (({ Compare_41((*A.borrow()).clone(), (*B.borrow()).clone()) }) >= 0);
}
pub fn IsDigitNormalized_42(X: v8_bigint_Digits) -> bool {
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    return (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }) == 0_u32)
        || (({ v8_bigint_DigitsImpl::msd(&X.as_pointer()) }) != 0_u64);
}
pub fn CompareNoNormalize_43(A: v8_bigint_Digits, B: v8_bigint_Digits) -> i32 {
    let A: Value<v8_bigint_Digits> = Rc::new(RefCell::new(A));
    let B: Value<v8_bigint_Digits> = Rc::new(RefCell::new(B));
    (&(0));
    let diff: Value<i32> = Rc::new(RefCell::new(
        ((({ v8_bigint_DigitsImpl::len(&A.as_pointer()) })
            .wrapping_sub(({ v8_bigint_DigitsImpl::len(&B.as_pointer()) }))) as i32),
    ));
    if ((*diff.borrow()) != 0) {
        return (*diff.borrow());
    }
    let i: Value<i32> = Rc::new(RefCell::new(
        ((({ v8_bigint_DigitsImpl::len(&A.as_pointer()) }).wrapping_sub(1_u32)) as i32),
    ));
    'loop_: while ((*i.borrow()) >= 0)
        && (({ v8_bigint_DigitsImpl::operator_index(&A.as_pointer(), ((*i.borrow()) as u32)) })
            == ({ v8_bigint_DigitsImpl::operator_index(&B.as_pointer(), ((*i.borrow()) as u32)) }))
    {
        (*i.borrow_mut()).postfix_dec();
    }
    if ((*i.borrow()) < 0) {
        return 0;
    }
    return if (({ v8_bigint_DigitsImpl::operator_index(&A.as_pointer(), ((*i.borrow()) as u32)) })
        > ({ v8_bigint_DigitsImpl::operator_index(&B.as_pointer(), ((*i.borrow()) as u32)) }))
    {
        1
    } else {
        -1_i32
    };
}
pub fn Compare_41(A: v8_bigint_Digits, B: v8_bigint_Digits) -> i32 {
    let A: Value<v8_bigint_Digits> = Rc::new(RefCell::new(A));
    let B: Value<v8_bigint_Digits> = Rc::new(RefCell::new(B));
    ({ v8_bigint_DigitsImpl::Normalize(&A.as_pointer()) });
    ({ v8_bigint_DigitsImpl::Normalize(&B.as_pointer()) });
    return ({ CompareNoNormalize_43((*A.borrow()).clone(), (*B.borrow()).clone()) });
}
pub fn Add_44(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) -> u64 {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    if (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })
        < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }))
    {
        {
            let tmp = X.as_pointer().read();
            X.as_pointer().write(Y.as_pointer().read());
            Y.as_pointer().write(tmp);
        };
    }
    (&(0));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    let top: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        ({
            let _digit: u64 = {
                (*top.borrow_mut()) = ({
                    let _a: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
                    let _b: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) });
                    let _c: u64 = (*carry.borrow());
                    let _carry: Ptr<u64> = (carry.as_pointer());
                    digit_add3_35(_a, _b, _c, _carry)
                });
                (*top.borrow())
            };
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 = {
                (*top.borrow_mut()) = ({
                    let _b: u64 = (*carry.borrow());
                    let _carry: Ptr<u64> = (carry.as_pointer());
                    digit_add2_34(
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                        _b,
                        _carry,
                    )
                });
                (*top.borrow())
            };
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                {
                    (*top.borrow_mut()) = (*carry.borrow());
                    (*top.borrow())
                },
            )
        });
        (*carry.borrow_mut()) = 0_u64;
        (*i.borrow_mut()).postfix_inc();
    }
    return (*top.borrow());
}
pub fn Subtract_45(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) -> u64 {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    (&(0));
    (&(0));
    (&(0));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    let borrow: Value<u64> = Rc::new(RefCell::new(0_u64));
    let top: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        ({
            let _digit: u64 = {
                (*top.borrow_mut()) = ({
                    let _a: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
                    let _b: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) });
                    let _borrow_in: u64 = (*borrow.borrow());
                    let _borrow_out: Ptr<u64> = (borrow.as_pointer());
                    digit_sub2_37(_a, _b, _borrow_in, _borrow_out)
                });
                (*top.borrow())
            };
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 = {
                (*top.borrow_mut()) = ({
                    let _b: u64 = (*borrow.borrow());
                    let _borrow: Ptr<u64> = (borrow.as_pointer());
                    digit_sub_36(
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                        _b,
                        _borrow,
                    )
                });
                (*top.borrow())
            };
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    (&(0));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    {
                        (*top.borrow_mut()) = 0_u64;
                        (*top.borrow())
                    },
                )
            });
        };
        (*i.borrow_mut()).postfix_inc();
    }
    return (*top.borrow());
}
pub fn SubtractWithNormalization_46(
    Z: v8_bigint_RWDigits,
    X: v8_bigint_Digits,
    Y: v8_bigint_Digits,
) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    ({ v8_bigint_DigitsImpl::Normalize(&X.as_pointer()) });
    ({ v8_bigint_DigitsImpl::Normalize(&Y.as_pointer()) });
    ({
        Subtract_45(
            (*Z.borrow()).clone(),
            (*X.borrow()).clone(),
            (*Y.borrow()).clone(),
        )
    });
}
pub fn AddSigned_47(
    Z: v8_bigint_RWDigits,
    X: v8_bigint_Digits,
    x_negative: bool,
    Y: v8_bigint_Digits,
    y_negative: bool,
) -> bool {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let x_negative: Value<bool> = Rc::new(RefCell::new(x_negative));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let y_negative: Value<bool> = Rc::new(RefCell::new(y_negative));
    if (((*x_negative.borrow()) as i32) == ((*y_negative.borrow()) as i32)) {
        ({
            Add_44(
                (*Z.borrow()).clone(),
                (*X.borrow()).clone(),
                (*Y.borrow()).clone(),
            )
        });
        return (*x_negative.borrow());
    }
    ({ v8_bigint_DigitsImpl::Normalize(&X.as_pointer()) });
    ({ v8_bigint_DigitsImpl::Normalize(&Y.as_pointer()) });
    if (({ CompareNoNormalize_43((*X.borrow()).clone(), (*Y.borrow()).clone()) }) >= 0) {
        ({
            Subtract_45(
                (*Z.borrow()).clone(),
                (*X.borrow()).clone(),
                (*Y.borrow()).clone(),
            )
        });
        return (*x_negative.borrow());
    }
    ({
        Subtract_45(
            (*Z.borrow()).clone(),
            (*Y.borrow()).clone(),
            (*X.borrow()).clone(),
        )
    });
    return !(*x_negative.borrow());
}
pub fn SubtractSigned_48(
    Z: v8_bigint_RWDigits,
    X: v8_bigint_Digits,
    x_negative: bool,
    Y: v8_bigint_Digits,
    y_negative: bool,
) -> bool {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let x_negative: Value<bool> = Rc::new(RefCell::new(x_negative));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let y_negative: Value<bool> = Rc::new(RefCell::new(y_negative));
    if (((*x_negative.borrow()) as i32) != ((*y_negative.borrow()) as i32)) {
        ({
            Add_44(
                (*Z.borrow()).clone(),
                (*X.borrow()).clone(),
                (*Y.borrow()).clone(),
            )
        });
        return (*x_negative.borrow());
    }
    ({ v8_bigint_DigitsImpl::Normalize(&X.as_pointer()) });
    ({ v8_bigint_DigitsImpl::Normalize(&Y.as_pointer()) });
    if (({ CompareNoNormalize_43((*X.borrow()).clone(), (*Y.borrow()).clone()) }) >= 0) {
        ({
            Subtract_45(
                (*Z.borrow()).clone(),
                (*X.borrow()).clone(),
                (*Y.borrow()).clone(),
            )
        });
        return (*x_negative.borrow());
    }
    ({
        Subtract_45(
            (*Z.borrow()).clone(),
            (*Y.borrow()).clone(),
            (*X.borrow()).clone(),
        )
    });
    return !(*x_negative.borrow());
}
pub fn AddOne_49(Z: v8_bigint_RWDigits, X: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let carry: Value<u64> = Rc::new(RefCell::new(1_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*carry.borrow()) > 0_u64)
        && ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }))
    {
        ({
            let _digit: u64 = ({
                let _b: u64 = (*carry.borrow());
                let _carry: Ptr<u64> = (carry.as_pointer());
                digit_add2_34(
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                    _b,
                    _carry,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    if ((*carry.borrow()) > 0_u64) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({
                        v8_bigint_RWDigitsImpl::operator_index(
                            &Z.as_pointer(),
                            (*i.borrow_mut()).postfix_inc(),
                        )
                    }),
                ))
                .as_pointer(),
                (*carry.borrow()),
            )
        });
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 =
                ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn SubtractOne_50(Z: v8_bigint_RWDigits, X: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let borrow: Value<u64> = Rc::new(RefCell::new(1_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*borrow.borrow()) > 0_u64) {
        ({
            let _digit: u64 = ({
                let _b: u64 = (*borrow.borrow());
                let _borrow: Ptr<u64> = (borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 =
                ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn Add_51(X: v8_bigint_RWDigits, y: u64) {
    let X: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(X));
    let y: Value<u64> = Rc::new(RefCell::new(y));
    let carry: Value<u64> = Rc::new(RefCell::new((*y.borrow())));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    let mut __do_while = true;
    'loop_: while __do_while || ((*carry.borrow()) != 0_u64) {
        __do_while = false;
        ({
            let _digit: u64 = ({
                let _b: u64 = (*carry.borrow());
                let _carry: Ptr<u64> = (carry.as_pointer());
                digit_add2_34(
                    ({
                        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                            &Rc::new(RefCell::new(
                                ({
                                    v8_bigint_RWDigitsImpl::operator_index(
                                        &X.as_pointer(),
                                        (*i.borrow()),
                                    )
                                }),
                            ))
                            .as_pointer(),
                        )
                    }),
                    _b,
                    _carry,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn Subtract_52(X: v8_bigint_RWDigits, y: u64) {
    let X: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(X));
    let y: Value<u64> = Rc::new(RefCell::new(y));
    let borrow: Value<u64> = Rc::new(RefCell::new((*y.borrow())));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    let mut __do_while = true;
    'loop_: while __do_while || ((*borrow.borrow()) != 0_u64) {
        __do_while = false;
        ({
            let _digit: u64 = ({
                let _b: u64 = (*borrow.borrow());
                let _borrow: Ptr<u64> = (borrow.as_pointer());
                digit_sub_36(
                    ({
                        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                            &Rc::new(RefCell::new(
                                ({
                                    v8_bigint_RWDigitsImpl::operator_index(
                                        &X.as_pointer(),
                                        (*i.borrow()),
                                    )
                                }),
                            ))
                            .as_pointer(),
                        )
                    }),
                    _b,
                    _borrow,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn InplaceAddAndReturnCarry_53(Z: v8_bigint_RWDigits, X: v8_bigint_Digits) -> u64 {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 = ({
                let _a: u64 = ({
                    v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                        &Rc::new(RefCell::new(
                            ({
                                v8_bigint_RWDigitsImpl::operator_index(
                                    &Z.as_pointer(),
                                    (*i.borrow()),
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                });
                let _b: u64 =
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
                let _c: u64 = (*carry.borrow());
                let _carry: Ptr<u64> = (carry.as_pointer());
                digit_add3_35(_a, _b, _c, _carry)
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    return (*carry.borrow());
}
pub fn InplaceSubAndReturnBorrow_54(Z: v8_bigint_RWDigits, X: v8_bigint_Digits) -> u64 {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let borrow: Value<u64> = Rc::new(RefCell::new(0_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 = ({
                let _a: u64 = ({
                    v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                        &Rc::new(RefCell::new(
                            ({
                                v8_bigint_RWDigitsImpl::operator_index(
                                    &Z.as_pointer(),
                                    (*i.borrow()),
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                });
                let _b: u64 =
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
                let _borrow_in: u64 = (*borrow.borrow());
                let _borrow_out: Ptr<u64> = (borrow.as_pointer());
                digit_sub2_37(_a, _b, _borrow_in, _borrow_out)
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    return (*borrow.borrow());
}
pub fn AddAndReturnCarry_55(
    Z: v8_bigint_RWDigits,
    X: v8_bigint_Digits,
    Y: v8_bigint_Digits,
) -> u64 {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    (&(0));
    let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        ({
            let _digit: u64 = ({
                let _a: u64 =
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
                let _b: u64 =
                    ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) });
                let _c: u64 = (*carry.borrow());
                let _carry: Ptr<u64> = (carry.as_pointer());
                digit_add3_35(_a, _b, _c, _carry)
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    return (*carry.borrow());
}
pub fn SubtractAndReturnBorrow_56(
    Z: v8_bigint_RWDigits,
    X: v8_bigint_Digits,
    Y: v8_bigint_Digits,
) -> u64 {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    (&(0));
    let borrow: Value<u64> = Rc::new(RefCell::new(0_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        ({
            let _digit: u64 = ({
                let _a: u64 =
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
                let _b: u64 =
                    ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) });
                let _borrow_in: u64 = (*borrow.borrow());
                let _borrow_out: Ptr<u64> = (borrow.as_pointer());
                digit_sub2_37(_a, _b, _borrow_in, _borrow_out)
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    return (*borrow.borrow());
}
pub fn MultiplySingle_57(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, y: u64) -> u64 {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let y: Value<u64> = Rc::new(RefCell::new(y));
    (&(0));
    let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    let high: Value<u64> = Rc::new(RefCell::new(0_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        let new_high: Value<u64> = <Value<u64>>::default();
        let low: Value<u64> = Rc::new(RefCell::new(
            ({
                digit_mul_38(
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                    (*y.borrow()),
                    (new_high.as_pointer()),
                )
            }),
        ));
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                ({
                    let _c: u64 = (*carry.borrow());
                    let _carry: Ptr<u64> = (carry.as_pointer());
                    digit_add3_35((*low.borrow()), (*high.borrow()), _c, _carry)
                }),
            )
        });
        (*high.borrow_mut()) = (*new_high.borrow());
        (*i.borrow_mut()).postfix_inc();
    }
    let top: Value<u64> = Rc::new(RefCell::new(
        (*carry.borrow()).wrapping_add((*high.borrow())),
    ));
    ({
        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
            &Rc::new(RefCell::new(
                ({
                    v8_bigint_RWDigitsImpl::operator_index(
                        &Z.as_pointer(),
                        ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
                    )
                }),
            ))
            .as_pointer(),
            (*top.borrow()),
        )
    });
    let i: Value<u32> = Rc::new(RefCell::new(
        ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }).wrapping_add(1_u32),
    ));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    {
                        (*top.borrow_mut()) = 0_u64;
                        (*top.borrow())
                    },
                )
            });
        };
        (*i.borrow_mut()).postfix_inc();
    }
    return (*top.borrow());
}
pub fn MultiplySchoolbook_58(
    Z: v8_bigint_RWDigits,
    X: v8_bigint_Digits,
    Y: v8_bigint_Digits,
) -> u64 {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    (&(0));
    (&(0));
    (&(0));
    (&(0));
    if (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }) == 0_u32)
        || (({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }) == 0_u32)
    {
        ({ v8_bigint_RWDigitsImpl::Clear(&Z.as_pointer()) });
        return 0_u64;
    }
    let next: Value<u64> = <Value<u64>>::default();
    let next_carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    ({
        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
            &Rc::new(RefCell::new(
                ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), 0_u32) }),
            ))
            .as_pointer(),
            ({
                let _a: u64 = ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), 0_u32) });
                let _b: u64 = ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), 0_u32) });
                digit_mul_38(_a, _b, (next.as_pointer()))
            }),
        )
    });
    let i: Value<u32> = Rc::new(RefCell::new(1_u32));
    if ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        let zi: Value<u64> = Rc::new(RefCell::new((*next.borrow())));
        (*next.borrow_mut()) = 0_u64;
        let j: Value<u32> = Rc::new(RefCell::new(0_u32));
        'loop_: while ((*j.borrow()) <= 1_u32) {
            let high: Value<u64> = <Value<u64>>::default();
            let low: Value<u64> = Rc::new(RefCell::new(
                ({
                    let _a: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*j.borrow())) });
                    let _b: u64 = ({
                        v8_bigint_DigitsImpl::operator_index(
                            &Y.as_pointer(),
                            (*i.borrow()).wrapping_sub((*j.borrow())),
                        )
                    });
                    digit_mul_38(_a, _b, (high.as_pointer()))
                }),
            ));
            let carrybit: Value<u64> = <Value<u64>>::default();
            let __rhs =
                ({ digit_add2_34((*zi.borrow()), (*low.borrow()), (carrybit.as_pointer())) });
            (*zi.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*carry.borrow_mut()) = rhs_0
            };
            let __rhs =
                ({ digit_add2_34((*next.borrow()), (*high.borrow()), (carrybit.as_pointer())) });
            (*next.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*next_carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*next_carry.borrow_mut()) = rhs_0
            };
            (*j.borrow_mut()).postfix_inc();
        }
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                (*zi.borrow()),
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        let zi: Value<u64> = Rc::new(RefCell::new(
            ({
                let _b: u64 = (*carry.borrow());
                let _carry: Ptr<u64> = (carry.as_pointer());
                digit_add2_34((*next.borrow()), _b, _carry)
            }),
        ));
        (*next.borrow_mut()) = (*next_carry.borrow()).wrapping_add((*carry.borrow()));
        (*carry.borrow_mut()) = 0_u64;
        (*next_carry.borrow_mut()) = 0_u64;
        let j: Value<u32> = Rc::new(RefCell::new(0_u32));
        'loop_: while ((*j.borrow()) <= (*i.borrow())) {
            let high: Value<u64> = <Value<u64>>::default();
            let low: Value<u64> = Rc::new(RefCell::new(
                ({
                    let _a: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*j.borrow())) });
                    let _b: u64 = ({
                        v8_bigint_DigitsImpl::operator_index(
                            &Y.as_pointer(),
                            (*i.borrow()).wrapping_sub((*j.borrow())),
                        )
                    });
                    digit_mul_38(_a, _b, (high.as_pointer()))
                }),
            ));
            let carrybit: Value<u64> = <Value<u64>>::default();
            let __rhs =
                ({ digit_add2_34((*zi.borrow()), (*low.borrow()), (carrybit.as_pointer())) });
            (*zi.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*carry.borrow_mut()) = rhs_0
            };
            let __rhs =
                ({ digit_add2_34((*next.borrow()), (*high.borrow()), (carrybit.as_pointer())) });
            (*next.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*next_carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*next_carry.borrow_mut()) = rhs_0
            };
            (*j.borrow_mut()).postfix_inc();
        }
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                (*zi.borrow()),
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    let loop_end: Value<u32> = Rc::new(RefCell::new(
        (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })
            .wrapping_add(({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })))
        .wrapping_sub(2_u32),
    ));
    'loop_: while ((*i.borrow()) <= (*loop_end.borrow())) {
        let max_x_index: Value<u32> = Rc::new(RefCell::new({
            let __tmp_1: Value<u32> = Rc::new(RefCell::new(
                ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }).wrapping_sub(1_u32),
            ));
            (if i.as_pointer().read() <= __tmp_1.as_pointer().read() {
                i.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }));
        let max_y_index: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }).wrapping_sub(1_u32),
        ));
        let min_x_index: Value<u32> = Rc::new(RefCell::new(
            (*i.borrow()).wrapping_sub((*max_y_index.borrow())),
        ));
        let zi: Value<u64> = Rc::new(RefCell::new(
            ({
                let _b: u64 = (*carry.borrow());
                let _carry: Ptr<u64> = (carry.as_pointer());
                digit_add2_34((*next.borrow()), _b, _carry)
            }),
        ));
        (*next.borrow_mut()) = (*next_carry.borrow()).wrapping_add((*carry.borrow()));
        (*carry.borrow_mut()) = 0_u64;
        (*next_carry.borrow_mut()) = 0_u64;
        let j: Value<u32> = Rc::new(RefCell::new((*min_x_index.borrow())));
        'loop_: while ((*j.borrow()) <= (*max_x_index.borrow())) {
            let high: Value<u64> = <Value<u64>>::default();
            let low: Value<u64> = Rc::new(RefCell::new(
                ({
                    let _a: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*j.borrow())) });
                    let _b: u64 = ({
                        v8_bigint_DigitsImpl::operator_index(
                            &Y.as_pointer(),
                            (*i.borrow()).wrapping_sub((*j.borrow())),
                        )
                    });
                    digit_mul_38(_a, _b, (high.as_pointer()))
                }),
            ));
            let carrybit: Value<u64> = <Value<u64>>::default();
            let __rhs =
                ({ digit_add2_34((*zi.borrow()), (*low.borrow()), (carrybit.as_pointer())) });
            (*zi.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*carry.borrow_mut()) = rhs_0
            };
            let __rhs =
                ({ digit_add2_34((*next.borrow()), (*high.borrow()), (carrybit.as_pointer())) });
            (*next.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*next_carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*next_carry.borrow_mut()) = rhs_0
            };
            (*j.borrow_mut()).postfix_inc();
        }
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                (*zi.borrow()),
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    let top: Value<u64> = Rc::new(RefCell::new(
        ({
            let _b: u64 = (*carry.borrow());
            let _carry: Ptr<u64> = (carry.as_pointer());
            digit_add2_34((*next.borrow()), _b, _carry)
        }),
    ));
    ({
        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
            &Rc::new(RefCell::new(
                ({
                    v8_bigint_RWDigitsImpl::operator_index(
                        &Z.as_pointer(),
                        (*i.borrow_mut()).postfix_inc(),
                    )
                }),
            ))
            .as_pointer(),
            (*top.borrow()),
        )
    });
    (&(0));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    {
                        (*top.borrow_mut()) = 0_u64;
                        (*top.borrow())
                    },
                )
            });
        };
        (*i.borrow_mut()).postfix_inc();
    }
    return (*top.borrow());
}
pub fn MultiplySpecialHigh_59(
    Z: v8_bigint_RWDigits,
    X: v8_bigint_Digits,
    Y: v8_bigint_Digits,
    start_position: u32,
) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let start_position: Value<u32> = Rc::new(RefCell::new(start_position));
    (&(0));
    (&(0));
    (&(0));
    (&(0));
    let next: Value<u64> = Rc::new(RefCell::new(0_u64));
    let next_carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    let loop_end: Value<u32> = Rc::new(RefCell::new(
        (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })
            .wrapping_add(({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })))
        .wrapping_sub(2_u32),
    ));
    let i: Value<u32> = Rc::new(RefCell::new((*start_position.borrow())));
    'loop_: while ((*i.borrow()) <= (*loop_end.borrow())) {
        let max_y_index: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }).wrapping_sub(1_u32),
        ));
        let min_x_index: Value<u32> = Rc::new(RefCell::new(
            (*i.borrow()).wrapping_sub((*max_y_index.borrow())),
        ));
        let max_x_index: Value<u32> = Rc::new(RefCell::new({
            let __tmp_1: Value<u32> = Rc::new(RefCell::new(
                ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }).wrapping_sub(1_u32),
            ));
            (if i.as_pointer().read() <= __tmp_1.as_pointer().read() {
                i.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }));
        let zi: Value<u64> = Rc::new(RefCell::new(
            ({
                let _b: u64 = (*carry.borrow());
                let _carry: Ptr<u64> = (carry.as_pointer());
                digit_add2_34((*next.borrow()), _b, _carry)
            }),
        ));
        (*next.borrow_mut()) = (*next_carry.borrow()).wrapping_add((*carry.borrow()));
        (*carry.borrow_mut()) = 0_u64;
        (*next_carry.borrow_mut()) = 0_u64;
        let j: Value<u32> = Rc::new(RefCell::new((*min_x_index.borrow())));
        'loop_: while ((*j.borrow()) <= (*max_x_index.borrow())) {
            let high: Value<u64> = <Value<u64>>::default();
            let low: Value<u64> = Rc::new(RefCell::new(
                ({
                    let _a: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*j.borrow())) });
                    let _b: u64 = ({
                        v8_bigint_DigitsImpl::operator_index(
                            &Y.as_pointer(),
                            (*i.borrow()).wrapping_sub((*j.borrow())),
                        )
                    });
                    digit_mul_38(_a, _b, (high.as_pointer()))
                }),
            ));
            let carrybit: Value<u64> = <Value<u64>>::default();
            let __rhs =
                ({ digit_add2_34((*zi.borrow()), (*low.borrow()), (carrybit.as_pointer())) });
            (*zi.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*carry.borrow_mut()) = rhs_0
            };
            let __rhs =
                ({ digit_add2_34((*next.borrow()), (*high.borrow()), (carrybit.as_pointer())) });
            (*next.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*next_carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*next_carry.borrow_mut()) = rhs_0
            };
            (*j.borrow_mut()).postfix_inc();
        }
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                (*zi.borrow()),
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    ({
        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
            &Rc::new(RefCell::new(
                ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
            ))
            .as_pointer(),
            ({
                let _b: u64 = (*carry.borrow());
                let _carry: Ptr<u64> = (carry.as_pointer());
                digit_add2_34((*next.borrow()), _b, _carry)
            }),
        )
    });
    (&(0));
}
pub fn MultiplySpecialLow_60(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    (&(0));
    (&(0));
    (&(0));
    let next: Value<u64> = <Value<u64>>::default();
    let next_carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    ({
        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
            &Rc::new(RefCell::new(
                ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), 0_u32) }),
            ))
            .as_pointer(),
            ({
                let _a: u64 = ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), 0_u32) });
                let _b: u64 = ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), 0_u32) });
                digit_mul_38(_a, _b, (next.as_pointer()))
            }),
        )
    });
    let i: Value<u32> = Rc::new(RefCell::new(1_u32));
    if ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        let zi: Value<u64> = Rc::new(RefCell::new((*next.borrow())));
        (*next.borrow_mut()) = 0_u64;
        let j: Value<u32> = Rc::new(RefCell::new(0_u32));
        'loop_: while ((*j.borrow()) <= 1_u32) {
            let high: Value<u64> = <Value<u64>>::default();
            let low: Value<u64> = Rc::new(RefCell::new(
                ({
                    let _a: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*j.borrow())) });
                    let _b: u64 = ({
                        v8_bigint_DigitsImpl::operator_index(
                            &Y.as_pointer(),
                            (*i.borrow()).wrapping_sub((*j.borrow())),
                        )
                    });
                    digit_mul_38(_a, _b, (high.as_pointer()))
                }),
            ));
            let carrybit: Value<u64> = <Value<u64>>::default();
            let __rhs =
                ({ digit_add2_34((*zi.borrow()), (*low.borrow()), (carrybit.as_pointer())) });
            (*zi.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*carry.borrow_mut()) = rhs_0
            };
            let __rhs =
                ({ digit_add2_34((*next.borrow()), (*high.borrow()), (carrybit.as_pointer())) });
            (*next.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*next_carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*next_carry.borrow_mut()) = rhs_0
            };
            (*j.borrow_mut()).postfix_inc();
        }
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                (*zi.borrow()),
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    let loop_end: Value<u32> = Rc::new(RefCell::new(
        ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) }).wrapping_sub(1_u32),
    ));
    let main_end: Value<u32> = Rc::new(RefCell::new(
        (if if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .to_strong()
        .as_pointer()
        .read()
            <= loop_end.as_pointer().read()
        {
            if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .to_strong()
            .as_pointer()
        } else {
            loop_end.as_pointer()
        }
        .read()),
    ));
    'loop_: while ((*i.borrow()) < (*main_end.borrow())) {
        let zi: Value<u64> = Rc::new(RefCell::new(
            ({
                let _b: u64 = (*carry.borrow());
                let _carry: Ptr<u64> = (carry.as_pointer());
                digit_add2_34((*next.borrow()), _b, _carry)
            }),
        ));
        (*next.borrow_mut()) = (*next_carry.borrow()).wrapping_add((*carry.borrow()));
        (*carry.borrow_mut()) = 0_u64;
        (*next_carry.borrow_mut()) = 0_u64;
        let j: Value<u32> = Rc::new(RefCell::new(0_u32));
        'loop_: while ((*j.borrow()) <= (*i.borrow())) {
            let high: Value<u64> = <Value<u64>>::default();
            let low: Value<u64> = Rc::new(RefCell::new(
                ({
                    let _a: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*j.borrow())) });
                    let _b: u64 = ({
                        v8_bigint_DigitsImpl::operator_index(
                            &Y.as_pointer(),
                            (*i.borrow()).wrapping_sub((*j.borrow())),
                        )
                    });
                    digit_mul_38(_a, _b, (high.as_pointer()))
                }),
            ));
            let carrybit: Value<u64> = <Value<u64>>::default();
            let __rhs =
                ({ digit_add2_34((*zi.borrow()), (*low.borrow()), (carrybit.as_pointer())) });
            (*zi.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*carry.borrow_mut()) = rhs_0
            };
            let __rhs =
                ({ digit_add2_34((*next.borrow()), (*high.borrow()), (carrybit.as_pointer())) });
            (*next.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*next_carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*next_carry.borrow_mut()) = rhs_0
            };
            (*j.borrow_mut()).postfix_inc();
        }
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                (*zi.borrow()),
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) <= (*loop_end.borrow())) {
        let max_x_index: Value<u32> = Rc::new(RefCell::new({
            let __tmp_1: Value<u32> = Rc::new(RefCell::new(
                ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }).wrapping_sub(1_u32),
            ));
            (if i.as_pointer().read() <= __tmp_1.as_pointer().read() {
                i.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }));
        let max_y_index: Value<u32> = Rc::new(RefCell::new({
            let __tmp_1: Value<u32> = Rc::new(RefCell::new(
                ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }).wrapping_sub(1_u32),
            ));
            (if i.as_pointer().read() <= __tmp_1.as_pointer().read() {
                i.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }));
        let min_x_index: Value<u32> = Rc::new(RefCell::new(
            (*i.borrow()).wrapping_sub((*max_y_index.borrow())),
        ));
        let zi: Value<u64> = Rc::new(RefCell::new(
            ({
                let _b: u64 = (*carry.borrow());
                let _carry: Ptr<u64> = (carry.as_pointer());
                digit_add2_34((*next.borrow()), _b, _carry)
            }),
        ));
        (*next.borrow_mut()) = (*next_carry.borrow()).wrapping_add((*carry.borrow()));
        (*carry.borrow_mut()) = 0_u64;
        (*next_carry.borrow_mut()) = 0_u64;
        let j: Value<u32> = Rc::new(RefCell::new((*min_x_index.borrow())));
        'loop_: while ((*j.borrow()) <= (*max_x_index.borrow())) {
            let high: Value<u64> = <Value<u64>>::default();
            let low: Value<u64> = Rc::new(RefCell::new(
                ({
                    let _a: u64 =
                        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*j.borrow())) });
                    let _b: u64 = ({
                        v8_bigint_DigitsImpl::operator_index(
                            &Y.as_pointer(),
                            (*i.borrow()).wrapping_sub((*j.borrow())),
                        )
                    });
                    digit_mul_38(_a, _b, (high.as_pointer()))
                }),
            ));
            let carrybit: Value<u64> = <Value<u64>>::default();
            let __rhs =
                ({ digit_add2_34((*zi.borrow()), (*low.borrow()), (carrybit.as_pointer())) });
            (*zi.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*carry.borrow_mut()) = rhs_0
            };
            let __rhs =
                ({ digit_add2_34((*next.borrow()), (*high.borrow()), (carrybit.as_pointer())) });
            (*next.borrow_mut()) = __rhs;
            {
                let rhs_0 = (*next_carry.borrow()).wrapping_add((*carrybit.borrow()));
                (*next_carry.borrow_mut()) = rhs_0
            };
            (*j.borrow_mut()).postfix_inc();
        }
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                (*zi.borrow()),
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn MultiplySmall_61(
    Z: Ptr<v8_bigint_RWDigits>,
    X: Ptr<v8_bigint_Digits>,
    Y: Ptr<v8_bigint_Digits>,
) -> (Value<bool>, Value<u64>) {
    (&(0));
    (&(0));
    if (({ v8_bigint_DigitsImpl::len(&X) }) == 0_u32)
        || (({ v8_bigint_DigitsImpl::len(&Y) }) == 0_u32)
    {
        ({ v8_bigint_RWDigitsImpl::Clear(&Z) });
        return (
            Rc::new(RefCell::new(true.try_into().expect("failed conversion"))),
            Rc::new(RefCell::new(0.try_into().expect("failed conversion"))),
        );
    }
    if {
        let _lhs = ({ v8_bigint_DigitsImpl::len(&X) });
        _lhs < ({ v8_bigint_DigitsImpl::len(&Y) })
    } {
        {
            let tmp = (X).clone().read();
            (X).clone().write((Y).clone().read());
            (Y).clone().write(tmp);
        };
    }
    if (({ v8_bigint_DigitsImpl::len(&Y) }) == 1_u32) {
        let top: Value<u64> = Rc::new(RefCell::new(
            ({
                let _Z: v8_bigint_RWDigits = (*Z.upgrade().deref()).clone();
                let _X: v8_bigint_Digits = (*X.upgrade().deref()).clone();
                let _y: u64 = ({ v8_bigint_DigitsImpl::operator_index(&Y, 0_u32) });
                MultiplySingle_57(_Z, _X, _y)
            }),
        ));
        return (
            Rc::new(RefCell::new(true.try_into().expect("failed conversion"))),
            Rc::new(RefCell::new(
                (*top.borrow()).try_into().expect("failed conversion"),
            )),
        );
    }
    if {
        let _lhs = ({ v8_bigint_DigitsImpl::len(&Y) });
        _lhs >= 34
    } {
        return (
            Rc::new(RefCell::new(false.try_into().expect("failed conversion"))),
            Rc::new(RefCell::new(0.try_into().expect("failed conversion"))),
        );
    }
    let top: Value<u64> = Rc::new(RefCell::new(
        ({
            let _Z: v8_bigint_RWDigits = (*Z.upgrade().deref()).clone();
            let _X: v8_bigint_Digits = (*X.upgrade().deref()).clone();
            let _Y: v8_bigint_Digits = (*Y.upgrade().deref()).clone();
            MultiplySchoolbook_58(_Z, _X, _Y)
        }),
    ));
    return (
        Rc::new(RefCell::new(true.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(
            (*top.borrow()).try_into().expect("failed conversion"),
        )),
    );
}
pub fn DivideSingle_62(
    Q: v8_bigint_RWDigits,
    remainder: Ptr<u64>,
    A: v8_bigint_Digits,
    b: u64,
) -> u64 {
    let Q: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Q));
    let remainder: Value<Ptr<u64>> = Rc::new(RefCell::new(remainder));
    let A: Value<v8_bigint_Digits> = Rc::new(RefCell::new(A));
    let b: Value<u64> = Rc::new(RefCell::new(b));
    (&(0));
    (&(0));
    (&(0));
    let rem: Value<u64> = Rc::new(RefCell::new(0_u64));
    let top: Value<u64> = <Value<u64>>::default();
    let length: Value<u32> = Rc::new(RefCell::new(
        ({ v8_bigint_DigitsImpl::len(&A.as_pointer()) }),
    ));
    if (({
        v8_bigint_DigitsImpl::operator_index(
            &A.as_pointer(),
            (*length.borrow()).wrapping_sub(1_u32),
        )
    }) >= (*b.borrow()))
    {
        (&(0));
        let i: Value<u32> = Rc::new(RefCell::new((*length.borrow()).wrapping_sub(1_u32)));
        ({
            let _digit: u64 = {
                (*top.borrow_mut()) = ({
                    let _high: u64 = (*rem.borrow());
                    let _remainder: Ptr<u64> = (rem.as_pointer());
                    digit_div_39(
                        _high,
                        ({ v8_bigint_DigitsImpl::operator_index(&A.as_pointer(), (*i.borrow())) }),
                        (*b.borrow()),
                        _remainder,
                    )
                });
                (*top.borrow())
            };
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Q.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        'loop_: while ((*i.borrow_mut()).postfix_dec() > 0_u32) {
            ({
                let _digit: u64 = ({
                    let _high: u64 = (*rem.borrow());
                    let _remainder: Ptr<u64> = (rem.as_pointer());
                    digit_div_39(
                        _high,
                        ({ v8_bigint_DigitsImpl::operator_index(&A.as_pointer(), (*i.borrow())) }),
                        (*b.borrow()),
                        _remainder,
                    )
                });
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Q.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    _digit,
                )
            });
        }
        let j: Value<u32> = Rc::new(RefCell::new((*length.borrow())));
        'loop_: while ((*j.borrow()) < ({ v8_bigint_DigitsImpl::len(&Q.as_pointer()) })) {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Q.as_pointer(), (*j.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    {
                        (*top.borrow_mut()) = 0_u64;
                        (*top.borrow())
                    },
                )
            });
            (*j.borrow_mut()).postfix_inc();
        }
    } else {
        (&(0));
        (*rem.borrow_mut()) = ({
            v8_bigint_DigitsImpl::operator_index(
                &A.as_pointer(),
                (*length.borrow()).wrapping_sub(1_u32),
            )
        });
        (*top.borrow_mut()) = 0_u64;
        let i: Value<u32> = Rc::new(RefCell::new((*length.borrow()).wrapping_sub(2_u32)));
        if ((*i.borrow()) >= 0_u32) {
            {
                ({
                    let _digit: u64 = {
                        (*top.borrow_mut()) = ({
                            let _high: u64 = (*rem.borrow());
                            let _remainder: Ptr<u64> = (rem.as_pointer());
                            digit_div_39(
                                _high,
                                ({
                                    v8_bigint_DigitsImpl::operator_index(
                                        &A.as_pointer(),
                                        (*i.borrow()),
                                    )
                                }),
                                (*b.borrow()),
                                _remainder,
                            )
                        });
                        (*top.borrow())
                    };
                    v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                        &Rc::new(RefCell::new(
                            ({
                                v8_bigint_RWDigitsImpl::operator_index(
                                    &Q.as_pointer(),
                                    (*i.borrow()),
                                )
                            }),
                        ))
                        .as_pointer(),
                        _digit,
                    )
                });
                'loop_: while ((*i.borrow_mut()).postfix_dec() > 0_u32) {
                    ({
                        let _digit: u64 = ({
                            let _high: u64 = (*rem.borrow());
                            let _remainder: Ptr<u64> = (rem.as_pointer());
                            digit_div_39(
                                _high,
                                ({
                                    v8_bigint_DigitsImpl::operator_index(
                                        &A.as_pointer(),
                                        (*i.borrow()),
                                    )
                                }),
                                (*b.borrow()),
                                _remainder,
                            )
                        });
                        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                            &Rc::new(RefCell::new(
                                ({
                                    v8_bigint_RWDigitsImpl::operator_index(
                                        &Q.as_pointer(),
                                        (*i.borrow()),
                                    )
                                }),
                            ))
                            .as_pointer(),
                            _digit,
                        )
                    });
                }
            };
        }
        let j: Value<u32> = Rc::new(RefCell::new((*length.borrow()).wrapping_sub(1_u32)));
        'loop_: while ((*j.borrow()) < ({ v8_bigint_DigitsImpl::len(&Q.as_pointer()) })) {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Q.as_pointer(), (*j.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    {
                        (*top.borrow_mut()) = 0_u64;
                        (*top.borrow())
                    },
                )
            });
            (*j.borrow_mut()).postfix_inc();
        }
    }
    let __rhs = (*rem.borrow());
    (*remainder.borrow()).write(__rhs);
    return (*top.borrow());
}
pub fn ModSingle_63(A: v8_bigint_Digits, b: u64) -> u64 {
    let A: Value<v8_bigint_Digits> = Rc::new(RefCell::new(A));
    let b: Value<u64> = Rc::new(RefCell::new(b));
    (&(0));
    (&(0));
    let rem: Value<u64> = Rc::new(RefCell::new(0_u64));
    let length: Value<u32> = Rc::new(RefCell::new(
        ({ v8_bigint_DigitsImpl::len(&A.as_pointer()) }),
    ));
    if (({
        v8_bigint_DigitsImpl::operator_index(
            &A.as_pointer(),
            (*length.borrow()).wrapping_sub(1_u32),
        )
    }) >= (*b.borrow()))
    {
        let i: Value<u32> = Rc::new(RefCell::new((*length.borrow())));
        'loop_: while ((*i.borrow_mut()).postfix_dec() > 0_u32) {
            ({
                let _high: u64 = (*rem.borrow());
                let _remainder: Ptr<u64> = (rem.as_pointer());
                digit_div_39(
                    _high,
                    ({ v8_bigint_DigitsImpl::operator_index(&A.as_pointer(), (*i.borrow())) }),
                    (*b.borrow()),
                    _remainder,
                )
            });
        }
    } else {
        (*rem.borrow_mut()) = ({
            v8_bigint_DigitsImpl::operator_index(
                &A.as_pointer(),
                (*length.borrow()).wrapping_sub(1_u32),
            )
        });
        let i: Value<u32> = Rc::new(RefCell::new((*length.borrow()).wrapping_sub(1_u32)));
        'loop_: while ((*i.borrow_mut()).postfix_dec() > 0_u32) {
            ({
                let _high: u64 = (*rem.borrow());
                let _remainder: Ptr<u64> = (rem.as_pointer());
                digit_div_39(
                    _high,
                    ({ v8_bigint_DigitsImpl::operator_index(&A.as_pointer(), (*i.borrow())) }),
                    (*b.borrow()),
                    _remainder,
                )
            });
        }
    }
    return (*rem.borrow());
}
pub fn DivideSmall_64(
    Q: Ptr<v8_bigint_RWDigits>,
    A: Ptr<v8_bigint_Digits>,
    B: Ptr<v8_bigint_Digits>,
) -> (Value<bool>, Value<u64>) {
    (&(0));
    (&(0));
    (&(0));
    let cmp: Value<i32> = Rc::new(RefCell::new(
        ({
            let _A: v8_bigint_Digits = (*A.upgrade().deref()).clone();
            let _B: v8_bigint_Digits = (*B.upgrade().deref()).clone();
            CompareNoNormalize_43(_A, _B)
        }),
    ));
    if ((*cmp.borrow()) < 0) {
        ({ v8_bigint_RWDigitsImpl::Clear(&Q) });
        return (
            Rc::new(RefCell::new(true.try_into().expect("failed conversion"))),
            Rc::new(RefCell::new(0.try_into().expect("failed conversion"))),
        );
    }
    if ((*cmp.borrow()) == 0) {
        let top: Value<u64> = Rc::new(RefCell::new(1_u64));
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Q, 0_u32) }),
                ))
                .as_pointer(),
                1_u64,
            )
        });
        let i: Value<u32> = Rc::new(RefCell::new(1_u32));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < ({ v8_bigint_DigitsImpl::len(&Q) })
        } {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            let _i: u32 = (*i.borrow());
                            v8_bigint_RWDigitsImpl::operator_index(&Q, _i)
                        }),
                    ))
                    .as_pointer(),
                    {
                        (*top.borrow_mut()) = 0_u64;
                        (*top.borrow())
                    },
                )
            });
            (*i.borrow_mut()).postfix_inc();
        }
        return (
            Rc::new(RefCell::new(true.try_into().expect("failed conversion"))),
            Rc::new(RefCell::new(
                (*top.borrow()).try_into().expect("failed conversion"),
            )),
        );
    }
    if (({ v8_bigint_DigitsImpl::len(&B) }) == 1_u32) {
        let remainder: Value<u64> = <Value<u64>>::default();
        let top: Value<u64> = Rc::new(RefCell::new(
            ({
                let _Q: v8_bigint_RWDigits = (*Q.upgrade().deref()).clone();
                let _remainder: Ptr<u64> = (remainder.as_pointer());
                let _A: v8_bigint_Digits = (*A.upgrade().deref()).clone();
                let _b: u64 = ({ v8_bigint_DigitsImpl::operator_index(&B, 0_u32) });
                DivideSingle_62(_Q, _remainder, _A, _b)
            }),
        ));
        return (
            Rc::new(RefCell::new(true.try_into().expect("failed conversion"))),
            Rc::new(RefCell::new(
                (*top.borrow()).try_into().expect("failed conversion"),
            )),
        );
    }
    return (
        Rc::new(RefCell::new(false.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(0.try_into().expect("failed conversion"))),
    );
}
pub fn ModuloSmall_65(
    R: Ptr<v8_bigint_RWDigits>,
    A: Ptr<v8_bigint_Digits>,
    B: Ptr<v8_bigint_Digits>,
) -> (Value<bool>, Value<u64>) {
    (&(0));
    (&(0));
    (&(0));
    let cmp: Value<i32> = Rc::new(RefCell::new(
        ({
            let _A: v8_bigint_Digits = (*A.upgrade().deref()).clone();
            let _B: v8_bigint_Digits = (*B.upgrade().deref()).clone();
            CompareNoNormalize_43(_A, _B)
        }),
    ));
    if ((*cmp.borrow()) < 0) {
        let top: Value<u64> = <Value<u64>>::default();
        let i: Value<u32> = Rc::new(RefCell::new(0_u32));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < ({ v8_bigint_DigitsImpl::len(&A) })
        } {
            ({
                let _digit: u64 = {
                    (*top.borrow_mut()) = ({
                        let _i: u32 = (*i.borrow());
                        v8_bigint_DigitsImpl::operator_index(&A, _i)
                    });
                    (*top.borrow())
                };
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            let _i: u32 = (*i.borrow());
                            v8_bigint_RWDigitsImpl::operator_index(&R, _i)
                        }),
                    ))
                    .as_pointer(),
                    _digit,
                )
            });
            (*i.borrow_mut()).postfix_inc();
        }
        let i: Value<u32> = Rc::new(RefCell::new(({ v8_bigint_DigitsImpl::len(&A) })));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < ({ v8_bigint_DigitsImpl::len(&R) })
        } {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            let _i: u32 = (*i.borrow());
                            v8_bigint_RWDigitsImpl::operator_index(&R, _i)
                        }),
                    ))
                    .as_pointer(),
                    {
                        (*top.borrow_mut()) = 0_u64;
                        (*top.borrow())
                    },
                )
            });
            (*i.borrow_mut()).postfix_inc();
        }
        return (
            Rc::new(RefCell::new(true.try_into().expect("failed conversion"))),
            Rc::new(RefCell::new(
                (*top.borrow()).try_into().expect("failed conversion"),
            )),
        );
    }
    if ((*cmp.borrow()) == 0) {
        ({ v8_bigint_RWDigitsImpl::Clear(&R) });
        return (
            Rc::new(RefCell::new(true.try_into().expect("failed conversion"))),
            Rc::new(RefCell::new(0.try_into().expect("failed conversion"))),
        );
    }
    if (({ v8_bigint_DigitsImpl::len(&B) }) == 1_u32) {
        let top: Value<u64> = Rc::new(RefCell::new(
            ({
                let _A: v8_bigint_Digits = (*A.upgrade().deref()).clone();
                let _b: u64 = ({ v8_bigint_DigitsImpl::operator_index(&B, 0_u32) });
                ModSingle_63(_A, _b)
            }),
        ));
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&R, 0_u32) }),
                ))
                .as_pointer(),
                (*top.borrow()),
            )
        });
        let i: Value<u32> = Rc::new(RefCell::new(1_u32));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < ({ v8_bigint_DigitsImpl::len(&R) })
        } {
            {
                ({
                    v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                        &Rc::new(RefCell::new(
                            ({
                                let _i: u32 = (*i.borrow());
                                v8_bigint_RWDigitsImpl::operator_index(&R, _i)
                            }),
                        ))
                        .as_pointer(),
                        {
                            (*top.borrow_mut()) = 0_u64;
                            (*top.borrow())
                        },
                    )
                });
            };
            (*i.borrow_mut()).postfix_inc();
        }
        return (
            Rc::new(RefCell::new(true.try_into().expect("failed conversion"))),
            Rc::new(RefCell::new(
                (*top.borrow()).try_into().expect("failed conversion"),
            )),
        );
    }
    return (
        Rc::new(RefCell::new(false.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(0.try_into().expect("failed conversion"))),
    );
}
pub fn DivideResultLength_66(A: v8_bigint_Digits, B: v8_bigint_Digits) -> u32 {
    let A: Value<v8_bigint_Digits> = Rc::new(RefCell::new(A));
    let B: Value<v8_bigint_Digits> = Rc::new(RefCell::new(B));
    (&(0));
    let kBarrettExtraScratch: Value<u32> = Rc::new(RefCell::new(
        (if (({ v8_bigint_DigitsImpl::len(&B.as_pointer()) }) >= 13000) {
            1
        } else {
            0
        } as u32),
    ));
    return ((({ v8_bigint_DigitsImpl::len(&A.as_pointer()) })
        .wrapping_sub(({ v8_bigint_DigitsImpl::len(&B.as_pointer()) })))
    .wrapping_add(1_u32))
    .wrapping_add((*kBarrettExtraScratch.borrow()));
}
pub fn BitwiseAnd_PosPos_67(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let pairs: Value<u32> = Rc::new(RefCell::new({
        let __tmp_0: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
        ));
        let __tmp_1: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    (&(0));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*pairs.borrow())) {
        ({
            let _digit: u64 =
                (({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) })
                    & ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }));
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn BitwiseAnd_NegNeg_68(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let pairs: Value<u32> = Rc::new(RefCell::new({
        let __tmp_0: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
        ));
        let __tmp_1: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    let x_borrow: Value<u64> = Rc::new(RefCell::new(1_u64));
    let y_borrow: Value<u64> = Rc::new(RefCell::new(1_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*pairs.borrow())) {
        ({
            let _digit: u64 = (({
                let _b: u64 = (*x_borrow.borrow());
                let _borrow: Ptr<u64> = (x_borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            }) | ({
                let _b: u64 = (*y_borrow.borrow());
                let _borrow: Ptr<u64> = (y_borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            }));
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 = ({
                let _b: u64 = (*x_borrow.borrow());
                let _borrow: Ptr<u64> = (x_borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        ({
            let _digit: u64 = ({
                let _b: u64 = (*y_borrow.borrow());
                let _borrow: Ptr<u64> = (y_borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    (&(0));
    (&(0));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    ({ Add_51((*Z.borrow()).clone(), 1_u64) });
}
pub fn BitwiseAnd_PosNeg_69(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let pairs: Value<u32> = Rc::new(RefCell::new({
        let __tmp_0: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
        ));
        let __tmp_1: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    let borrow: Value<u64> = Rc::new(RefCell::new(1_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*pairs.borrow())) {
        ({
            let _digit: u64 =
                (({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) })
                    & !({
                        let _b: u64 = (*borrow.borrow());
                        let _borrow: Ptr<u64> = (borrow.as_pointer());
                        digit_sub_36(
                            ({
                                v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow()))
                            }),
                            _b,
                            _borrow,
                        )
                    }));
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 =
                ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn BitwiseOr_PosPos_70(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let pairs: Value<u32> = Rc::new(RefCell::new({
        let __tmp_0: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
        ));
        let __tmp_1: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*pairs.borrow())) {
        ({
            let _digit: u64 =
                (({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) })
                    | ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }));
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 =
                ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        ({
            let _digit: u64 =
                ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn BitwiseOr_NegNeg_71(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let pairs: Value<u32> = Rc::new(RefCell::new({
        let __tmp_0: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
        ));
        let __tmp_1: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    let x_borrow: Value<u64> = Rc::new(RefCell::new(1_u64));
    let y_borrow: Value<u64> = Rc::new(RefCell::new(1_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*pairs.borrow())) {
        ({
            let _digit: u64 = (({
                let _b: u64 = (*x_borrow.borrow());
                let _borrow: Ptr<u64> = (x_borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            }) & ({
                let _b: u64 = (*y_borrow.borrow());
                let _borrow: Ptr<u64> = (y_borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            }));
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    ({ Add_51((*Z.borrow()).clone(), 1_u64) });
}
pub fn BitwiseOr_PosNeg_72(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let pairs: Value<u32> = Rc::new(RefCell::new({
        let __tmp_0: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
        ));
        let __tmp_1: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    let borrow: Value<u64> = Rc::new(RefCell::new(1_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*pairs.borrow())) {
        ({
            let _digit: u64 =
                (({
                    let _b: u64 = (*borrow.borrow());
                    let _borrow: Ptr<u64> = (borrow.as_pointer());
                    digit_sub_36(
                        ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }),
                        _b,
                        _borrow,
                    )
                }) & !({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }));
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        ({
            let _digit: u64 = ({
                let _b: u64 = (*borrow.borrow());
                let _borrow: Ptr<u64> = (borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    (&(0));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    ({ Add_51((*Z.borrow()).clone(), 1_u64) });
}
pub fn BitwiseXor_PosPos_73(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let pairs: Value<u32> = Rc::new(RefCell::new(
        ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
    ));
    if (({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })
        < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }))
    {
        {
            let tmp = X.as_pointer().read();
            X.as_pointer().write(Y.as_pointer().read());
            Y.as_pointer().write(tmp);
        };
        (*pairs.borrow_mut()) = ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) });
    }
    (&(0));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*pairs.borrow())) {
        ({
            let _digit: u64 =
                (({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) })
                    ^ ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }));
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        ({
            let _digit: u64 =
                ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn BitwiseXor_NegNeg_74(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let pairs: Value<u32> = Rc::new(RefCell::new({
        let __tmp_0: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
        ));
        let __tmp_1: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    let x_borrow: Value<u64> = Rc::new(RefCell::new(1_u64));
    let y_borrow: Value<u64> = Rc::new(RefCell::new(1_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*pairs.borrow())) {
        ({
            let _digit: u64 = (({
                let _b: u64 = (*x_borrow.borrow());
                let _borrow: Ptr<u64> = (x_borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            }) ^ ({
                let _b: u64 = (*y_borrow.borrow());
                let _borrow: Ptr<u64> = (y_borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            }));
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 = ({
                let _b: u64 = (*x_borrow.borrow());
                let _borrow: Ptr<u64> = (x_borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        ({
            let _digit: u64 = ({
                let _b: u64 = (*y_borrow.borrow());
                let _borrow: Ptr<u64> = (y_borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    (&(0));
    (&(0));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn BitwiseXor_PosNeg_75(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, Y: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let Y: Value<v8_bigint_Digits> = Rc::new(RefCell::new(Y));
    let pairs: Value<u32> = Rc::new(RefCell::new({
        let __tmp_0: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
        ));
        let __tmp_1: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) }),
        ));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    let borrow: Value<u64> = Rc::new(RefCell::new(1_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*pairs.borrow())) {
        ({
            let _digit: u64 =
                (({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) })
                    ^ ({
                        let _b: u64 = (*borrow.borrow());
                        let _borrow: Ptr<u64> = (borrow.as_pointer());
                        digit_sub_36(
                            ({
                                v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow()))
                            }),
                            _b,
                            _borrow,
                        )
                    }));
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 =
                ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Y.as_pointer()) })) {
        ({
            let _digit: u64 = ({
                let _b: u64 = (*borrow.borrow());
                let _borrow: Ptr<u64> = (borrow.as_pointer());
                digit_sub_36(
                    ({ v8_bigint_DigitsImpl::operator_index(&Y.as_pointer(), (*i.borrow())) }),
                    _b,
                    _borrow,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    (&(0));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    ({ Add_51((*Z.borrow()).clone(), 1_u64) });
}
pub fn LeftShift_76(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, shift: u64) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    let digit_shift: Value<u32> = Rc::new(RefCell::new(
        (((*shift.borrow()).wrapping_div((64 as u64))) as u32),
    ));
    let bits_shift: Value<i32> = Rc::new(RefCell::new(
        (((*shift.borrow()).wrapping_rem((64 as u64))) as i32),
    ));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*digit_shift.borrow())) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    if ((*bits_shift.borrow()) == 0) {
        'loop_: while ((*i.borrow())
            < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })
                .wrapping_add((*digit_shift.borrow())))
        {
            ({
                let _digit: u64 = ({
                    v8_bigint_DigitsImpl::operator_index(
                        &X.as_pointer(),
                        (*i.borrow()).wrapping_sub((*digit_shift.borrow())),
                    )
                });
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    _digit,
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
        'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    0_u64,
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    } else {
        let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
        'loop_: while ((*i.borrow())
            < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })
                .wrapping_add((*digit_shift.borrow())))
        {
            let d: Value<u64> = Rc::new(RefCell::new(
                ({
                    v8_bigint_DigitsImpl::operator_index(
                        &X.as_pointer(),
                        (*i.borrow()).wrapping_sub((*digit_shift.borrow())),
                    )
                }),
            ));
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    (((*d.borrow()) << (*bits_shift.borrow())) | (*carry.borrow())),
                )
            });
            (*carry.borrow_mut()) = ((*d.borrow()) >> (64 - (*bits_shift.borrow())));
            (*i.borrow_mut()).prefix_inc();
        }
        if ((*carry.borrow()) != 0_u64) {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(
                                &Z.as_pointer(),
                                (*i.borrow_mut()).postfix_inc(),
                            )
                        }),
                    ))
                    .as_pointer(),
                    (*carry.borrow()),
                )
            });
        }
        'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    0_u64,
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    }
}
pub fn RightShift_77(
    Z: v8_bigint_RWDigits,
    X: v8_bigint_Digits,
    shift: u64,
    state: Ptr<v8_bigint_RightShiftState>,
) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    let digit_shift: Value<u32> = Rc::new(RefCell::new(
        (((*shift.borrow()).wrapping_div((64 as u64))) as u32),
    ));
    let bits_shift: Value<i32> = Rc::new(RefCell::new(
        (((*shift.borrow()).wrapping_rem((64 as u64))) as i32),
    ));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    if ((*bits_shift.borrow()) == 0) {
        'loop_: while ((*i.borrow())
            < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })
                .wrapping_sub((*digit_shift.borrow())))
        {
            ({
                let _digit: u64 = ({
                    v8_bigint_DigitsImpl::operator_index(
                        &X.as_pointer(),
                        (*i.borrow()).wrapping_add((*digit_shift.borrow())),
                    )
                });
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    _digit,
                )
            });
            (*i.borrow_mut()).prefix_inc();
        }
    } else {
        let carry: Value<u64> = Rc::new(RefCell::new(
            (({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*digit_shift.borrow())) })
                >> (*bits_shift.borrow())),
        ));
        'loop_: while ((*i.borrow())
            < (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })
                .wrapping_sub((*digit_shift.borrow())))
            .wrapping_sub(1_u32))
        {
            let d: Value<u64> = Rc::new(RefCell::new(
                ({
                    v8_bigint_DigitsImpl::operator_index(
                        &X.as_pointer(),
                        ((*i.borrow()).wrapping_add((*digit_shift.borrow()))).wrapping_add(1_u32),
                    )
                }),
            ));
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    (((*d.borrow()) << (64 - (*bits_shift.borrow()))) | (*carry.borrow())),
                )
            });
            (*carry.borrow_mut()) = ((*d.borrow()) >> (*bits_shift.borrow()));
            (*i.borrow_mut()).prefix_inc();
        }
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({
                        v8_bigint_RWDigitsImpl::operator_index(
                            &Z.as_pointer(),
                            (*i.borrow_mut()).postfix_inc(),
                        )
                    }),
                ))
                .as_pointer(),
                (*carry.borrow()),
            )
        });
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).prefix_inc();
    }
    if (*(*state.upgrade().deref()).must_round_down.borrow()) {
        ({ Add_51((*Z.borrow()).clone(), 1_u64) });
    }
}
pub fn RightShift_ResultLength_78(
    X: v8_bigint_Digits,
    x_sign: bool,
    shift: u64,
    state: Ptr<v8_bigint_RightShiftState>,
) -> u32 {
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let x_sign: Value<bool> = Rc::new(RefCell::new(x_sign));
    let shift: Value<u64> = Rc::new(RefCell::new(shift));
    let state: Value<Ptr<v8_bigint_RightShiftState>> = Rc::new(RefCell::new(state));
    let digit_shift: Value<u32> = Rc::new(RefCell::new(
        (((*shift.borrow()).wrapping_div((64 as u64))) as u32),
    ));
    let bits_shift: Value<i32> = Rc::new(RefCell::new(
        (((*shift.borrow()).wrapping_rem((64 as u64))) as i32),
    ));
    if (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }) <= (*digit_shift.borrow())) {
        return 0_u32;
    }
    let result_length: Value<u32> = Rc::new(RefCell::new(
        ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }).wrapping_sub((*digit_shift.borrow())),
    ));
    let must_round_down: Value<bool> = Rc::new(RefCell::new(false));
    if (*x_sign.borrow()) {
        let mask: Value<u64> = Rc::new(RefCell::new(
            (1_u64 << (*bits_shift.borrow())).wrapping_sub(1_u64),
        ));
        if ((({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*digit_shift.borrow())) })
            & (*mask.borrow()))
            != 0_u64)
        {
            (*must_round_down.borrow_mut()) = true;
        } else {
            let i: Value<u32> = Rc::new(RefCell::new(0_u32));
            'loop_: while ((*i.borrow()) < (*digit_shift.borrow())) {
                if (({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) })
                    != 0_u64)
                {
                    (*must_round_down.borrow_mut()) = true;
                    break;
                }
                (*i.borrow_mut()).postfix_inc();
            }
        }
    }
    if (*must_round_down.borrow()) && ((*bits_shift.borrow()) == 0) {
        let rounding_can_overflow: Value<bool> = Rc::new(RefCell::new(
            ({ digit_ismax_33(({ v8_bigint_DigitsImpl::msd(&X.as_pointer()) })) }),
        ));
        if (*rounding_can_overflow.borrow()) {
            (*result_length.borrow_mut()).prefix_inc();
        }
    }
    if !(*state.borrow()).is_null() {
        (&(0));
        (*(*(*state.borrow()).upgrade().deref())
            .must_round_down
            .borrow_mut()) = (*must_round_down.borrow());
    }
    return (*result_length.borrow());
}
pub fn TruncateToNBits_79(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, n: u32) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let n: Value<u32> = Rc::new(RefCell::new(n));
    let digits: Value<u32> = Rc::new(RefCell::new(
        ((((*n.borrow()).wrapping_sub(1_u32)).wrapping_div(((64) as u32))).wrapping_add(1_u32)),
    ));
    let bits: Value<i32> = Rc::new(RefCell::new(
        (((*n.borrow()).wrapping_rem((64 as u32))) as i32),
    ));
    let last: Value<u32> = Rc::new(RefCell::new((*digits.borrow()).wrapping_sub(1_u32)));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*last.borrow())) {
        ({
            let _digit: u64 =
                ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    let msd: Value<u64> = Rc::new(RefCell::new(
        ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*last.borrow())) }),
    ));
    if ((*bits.borrow()) != 0) {
        let drop: Value<i32> = Rc::new(RefCell::new((64 - (*bits.borrow()))));
        let __rhs = (((*msd.borrow()) << (*drop.borrow())) >> (*drop.borrow()));
        (*msd.borrow_mut()) = __rhs;
    }
    ({
        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
            &Rc::new(RefCell::new(
                ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*last.borrow())) }),
            ))
            .as_pointer(),
            (*msd.borrow()),
        )
    });
}
pub fn TruncateAndSubFromPowerOfTwo_80(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, n: u32) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let n: Value<u32> = Rc::new(RefCell::new(n));
    let digits: Value<u32> = Rc::new(RefCell::new(
        ((((*n.borrow()).wrapping_sub(1_u32)).wrapping_div(((64) as u32))).wrapping_add(1_u32)),
    ));
    let bits: Value<i32> = Rc::new(RefCell::new(
        (((*n.borrow()).wrapping_rem((64 as u32))) as i32),
    ));
    let last: Value<u32> = Rc::new(RefCell::new((*digits.borrow()).wrapping_sub(1_u32)));
    let have_x: Value<u32> = Rc::new(RefCell::new({
        let __tmp_1: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }),
        ));
        (if last.as_pointer().read() <= __tmp_1.as_pointer().read() {
            last.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    }));
    let borrow: Value<u64> = Rc::new(RefCell::new(0_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*have_x.borrow())) {
        ({
            let _digit: u64 = ({
                let _borrow_in: u64 = (*borrow.borrow());
                let _borrow_out: Ptr<u64> = (borrow.as_pointer());
                digit_sub2_37(
                    0_u64,
                    ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
                    _borrow_in,
                    _borrow_out,
                )
            });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < (*last.borrow())) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                ({
                    let _b: u64 = (*borrow.borrow());
                    let _borrow: Ptr<u64> = (borrow.as_pointer());
                    digit_sub_36(0_u64, _b, _borrow)
                }),
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    let msd: Value<u64> = Rc::new(RefCell::new(
        if ((*last.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
            ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*last.borrow())) })
        } else {
            0_u64
        },
    ));
    if ((*bits.borrow()) == 0) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*last.borrow())) }),
                ))
                .as_pointer(),
                ({
                    let _borrow_in: u64 = (*borrow.borrow());
                    let _borrow_out: Ptr<u64> = (borrow.as_pointer());
                    digit_sub2_37(0_u64, (*msd.borrow()), _borrow_in, _borrow_out)
                }),
            )
        });
    } else {
        let drop: Value<i32> = Rc::new(RefCell::new((64 - (*bits.borrow()))));
        let __rhs = (((*msd.borrow()) << (*drop.borrow())) >> (*drop.borrow()));
        (*msd.borrow_mut()) = __rhs;
        let minuend_msd: Value<u64> = Rc::new(RefCell::new((1_u64 << (*bits.borrow()))));
        let result_msd: Value<u64> = Rc::new(RefCell::new(
            ({
                let _borrow_in: u64 = (*borrow.borrow());
                let _borrow_out: Ptr<u64> = (borrow.as_pointer());
                digit_sub2_37(
                    (*minuend_msd.borrow()),
                    (*msd.borrow()),
                    _borrow_in,
                    _borrow_out,
                )
            }),
        ));
        (&(0));
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*last.borrow())) }),
                ))
                .as_pointer(),
                ((*result_msd.borrow()) & ((*minuend_msd.borrow()).wrapping_sub(1_u64))),
            )
        });
    }
}
pub fn AsIntN_81(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, x_negative: bool, n: u32) -> bool {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let x_negative: Value<bool> = Rc::new(RefCell::new(x_negative));
    let n: Value<u32> = Rc::new(RefCell::new(n));
    (&(0));
    (&(0));
    (&(0));
    let needed_digits: Value<u32> = Rc::new(RefCell::new(
        ((((*n.borrow()).wrapping_sub(1_u32)).wrapping_div(((64) as u32))).wrapping_add(1_u32)),
    ));
    let top_digit: Value<u64> = Rc::new(RefCell::new(
        ({
            v8_bigint_DigitsImpl::operator_index(
                &X.as_pointer(),
                (*needed_digits.borrow()).wrapping_sub(1_u32),
            )
        }),
    ));
    let compare_digit: Value<u64> = Rc::new(RefCell::new(
        (1_u64 << (((*n.borrow()).wrapping_sub(1_u32)).wrapping_rem((64 as u32)))),
    ));
    let has_bit: Value<bool> = Rc::new(RefCell::new(
        (((*top_digit.borrow()) & (*compare_digit.borrow())) == (*compare_digit.borrow())),
    ));
    if !(*has_bit.borrow()) {
        ({ TruncateToNBits_79((*Z.borrow()).clone(), (*X.borrow()).clone(), (*n.borrow())) });
        return (*x_negative.borrow());
    }
    ({
        TruncateAndSubFromPowerOfTwo_80((*Z.borrow()).clone(), (*X.borrow()).clone(), (*n.borrow()))
    });
    if !(*x_negative.borrow()) {
        return true;
    }
    if (((*top_digit.borrow()) & ((*compare_digit.borrow()).wrapping_sub(1_u64))) != 0_u64) {
        return false;
    }
    let i: Value<i32> = Rc::new(RefCell::new(
        (((*needed_digits.borrow()).wrapping_sub(2_u32)) as i32),
    ));
    'loop_: while ((*i.borrow()) >= 0) {
        if (({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), ((*i.borrow()) as u32)) })
            != 0_u64)
        {
            return false;
        }
        (*i.borrow_mut()).postfix_dec();
    }
    return true;
}
pub fn AsIntNResultLength_82(X: v8_bigint_Digits, x_negative: bool, n: u32) -> i32 {
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let x_negative: Value<bool> = Rc::new(RefCell::new(x_negative));
    let n: Value<u32> = Rc::new(RefCell::new(n));
    let needed_digits: Value<u32> = Rc::new(RefCell::new(
        ((((*n.borrow()).wrapping_sub(1_u32)).wrapping_div(((64) as u32))).wrapping_add(1_u32)),
    ));
    if (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }) < (*needed_digits.borrow())) {
        return -1_i32;
    }
    if (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }) > (*needed_digits.borrow())) {
        return ((*needed_digits.borrow()) as i32);
    }
    let top_digit: Value<u64> = Rc::new(RefCell::new(
        ({
            v8_bigint_DigitsImpl::operator_index(
                &X.as_pointer(),
                (*needed_digits.borrow()).wrapping_sub(1_u32),
            )
        }),
    ));
    let compare_digit: Value<u64> = Rc::new(RefCell::new(
        (1_u64 << (((*n.borrow()).wrapping_sub(1_u32)).wrapping_rem((64 as u32)))),
    ));
    if ((*top_digit.borrow()) < (*compare_digit.borrow())) {
        return -1_i32;
    }
    if ((*top_digit.borrow()) > (*compare_digit.borrow())) {
        return ((*needed_digits.borrow()) as i32);
    }
    if !(*x_negative.borrow()) {
        return ((*needed_digits.borrow()) as i32);
    }
    let i: Value<i32> = Rc::new(RefCell::new(
        (((*needed_digits.borrow()).wrapping_sub(2_u32)) as i32),
    ));
    'loop_: while ((*i.borrow()) >= 0) {
        if (({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), ((*i.borrow()) as u32)) })
            != 0_u64)
        {
            return ((*needed_digits.borrow()) as i32);
        }
        (*i.borrow_mut()).postfix_dec();
    }
    return -1_i32;
}
pub fn AsUintN_Pos_ResultLength_83(X: v8_bigint_Digits, n: u32) -> i32 {
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let n: Value<u32> = Rc::new(RefCell::new(n));
    let needed_digits: Value<u32> = Rc::new(RefCell::new(
        ((((*n.borrow()).wrapping_sub(1_u32)).wrapping_div(((64) as u32))).wrapping_add(1_u32)),
    ));
    if (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }) < (*needed_digits.borrow())) {
        return -1_i32;
    }
    if (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }) > (*needed_digits.borrow())) {
        return ((*needed_digits.borrow()) as i32);
    }
    let bits_in_top_digit: Value<i32> = Rc::new(RefCell::new(
        (((*n.borrow()).wrapping_rem((64 as u32))) as i32),
    ));
    if ((*bits_in_top_digit.borrow()) == 0) {
        return -1_i32;
    }
    let top_digit: Value<u64> = Rc::new(RefCell::new(
        ({
            v8_bigint_DigitsImpl::operator_index(
                &X.as_pointer(),
                (*needed_digits.borrow()).wrapping_sub(1_u32),
            )
        }),
    ));
    if (((*top_digit.borrow()) >> (*bits_in_top_digit.borrow())) == 0_u64) {
        return -1_i32;
    }
    return ((*needed_digits.borrow()) as i32);
}
pub fn AsUintN_Pos_84(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, n: u32) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let n: Value<u32> = Rc::new(RefCell::new(n));
    (&(0));
    ({ TruncateToNBits_79((*Z.borrow()).clone(), (*X.borrow()).clone(), (*n.borrow())) });
}
pub fn AsUintN_Neg_85(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, n: u32) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let n: Value<u32> = Rc::new(RefCell::new(n));
    ({
        TruncateAndSubFromPowerOfTwo_80((*Z.borrow()).clone(), (*X.borrow()).clone(), (*n.borrow()))
    });
}
pub fn DivideBarrettScratchSpace_86(n: u32) -> u32 {
    let n: Value<u32> = Rc::new(RefCell::new(n));
    return (*n.borrow()).wrapping_add(2_u32);
}
thread_local!(
    pub static kInvertNewtonExtraSpace_87: Value<u32> = Rc::new(RefCell::new(5));
);
pub fn InvertNewtonScratchSpace_88(n: u32) -> u32 {
    let n: Value<u32> = Rc::new(RefCell::new(n));
    return ((3_u32).wrapping_mul((*n.borrow()))).wrapping_add(((2_u32).wrapping_mul(5) as u32));
}
pub fn InvertScratchSpace_89(n: u32) -> u32 {
    let n: Value<u32> = Rc::new(RefCell::new(n));
    return if ((*n.borrow()) < 25) {
        (2_u32).wrapping_mul((*n.borrow()))
    } else {
        ({ InvertNewtonScratchSpace_88((*n.borrow())) })
    };
}
pub fn Copy_90(Z: v8_bigint_RWDigits, X: v8_bigint_Digits) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    if ({ v8_bigint_DigitsImpl::operator_eq(&Z.as_pointer(), X.as_pointer()) }) {
        return;
    }
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        ({
            let _digit: u64 =
                ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn LeftShift_91(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, shift: i32) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let shift: Value<i32> = Rc::new(RefCell::new(shift));
    (&(0));
    (&(0));
    (&(0));
    if ((*shift.borrow()) == 0) {
        ({ Copy_90((*Z.borrow()).clone(), (*X.borrow()).clone()) });
        return;
    }
    let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) })) {
        let d: Value<u64> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), (*i.borrow())) }),
        ));
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                (((*d.borrow()) << (*shift.borrow())) | (*carry.borrow())),
            )
        });
        (*carry.borrow_mut()) = ((*d.borrow()) >> (64 - (*shift.borrow())));
        (*i.borrow_mut()).postfix_inc();
    }
    if ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({
                        v8_bigint_RWDigitsImpl::operator_index(
                            &Z.as_pointer(),
                            (*i.borrow_mut()).postfix_inc(),
                        )
                    }),
                ))
                .as_pointer(),
                (*carry.borrow()),
            )
        });
    } else {
        (&(0));
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn RightShift_92(Z: v8_bigint_RWDigits, X: v8_bigint_Digits, shift: i32) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let X: Value<v8_bigint_Digits> = Rc::new(RefCell::new(X));
    let shift: Value<i32> = Rc::new(RefCell::new(shift));
    (&(0));
    (&(0));
    ({ v8_bigint_DigitsImpl::Normalize(&X.as_pointer()) });
    (&(0));
    if ((*shift.borrow()) == 0) {
        ({ Copy_90((*Z.borrow()).clone(), (*X.borrow()).clone()) });
        return;
    }
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    if (({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }) > 0_u32) {
        let carry: Value<u64> = Rc::new(RefCell::new(
            (({ v8_bigint_DigitsImpl::operator_index(&X.as_pointer(), 0_u32) })
                >> (*shift.borrow())),
        ));
        let last: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&X.as_pointer()) }).wrapping_sub(1_u32),
        ));
        'loop_: while ((*i.borrow()) < (*last.borrow())) {
            let d: Value<u64> = Rc::new(RefCell::new(
                ({
                    v8_bigint_DigitsImpl::operator_index(
                        &X.as_pointer(),
                        (*i.borrow()).wrapping_add(1_u32),
                    )
                }),
            ));
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow()))
                        }),
                    ))
                    .as_pointer(),
                    (((*d.borrow()) << (64 - (*shift.borrow()))) | (*carry.borrow())),
                )
            });
            (*carry.borrow_mut()) = ((*d.borrow()) >> (*shift.borrow()));
            (*i.borrow_mut()).postfix_inc();
        }
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({
                        v8_bigint_RWDigitsImpl::operator_index(
                            &Z.as_pointer(),
                            (*i.borrow_mut()).postfix_inc(),
                        )
                    }),
                ))
                .as_pointer(),
                (*carry.borrow()),
            )
        });
    }
    'loop_: while ((*i.borrow()) < ({ v8_bigint_DigitsImpl::len(&Z.as_pointer()) })) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
pub fn PutAt_93(Z: v8_bigint_RWDigits, A: v8_bigint_Digits, count: u32) {
    let Z: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(Z));
    let A: Value<v8_bigint_Digits> = Rc::new(RefCell::new(A));
    let count: Value<u32> = Rc::new(RefCell::new(count));
    let len: Value<u32> = Rc::new(RefCell::new({
        let __tmp_0: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&A.as_pointer()) }),
        ));
        (if __tmp_0.as_pointer().read() <= count.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            count.as_pointer()
        }
        .read())
    }));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < (*len.borrow())) {
        ({
            let _digit: u64 =
                ({ v8_bigint_DigitsImpl::operator_index(&A.as_pointer(), (*i.borrow())) });
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                _digit,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
    'loop_: while ((*i.borrow()) < (*count.borrow())) {
        ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                &Rc::new(RefCell::new(
                    ({ v8_bigint_RWDigitsImpl::operator_index(&Z.as_pointer(), (*i.borrow())) }),
                ))
                .as_pointer(),
                0_u64,
            )
        });
        (*i.borrow_mut()).postfix_inc();
    }
}
#[derive(Default)]
pub struct v8_bigint_ShiftedDigits {
    shift_: Value<i32>,
    inplace_: Value<bool>,
    storage_: Value<Option<Value<Box<[u64]>>>>,
}
impl v8_bigint_ShiftedDigits {
    pub fn v8_bigint_ShiftedDigits1(
        original: Ptr<v8_bigint_Digits>,
        platform: PtrDyn<dyn v8_bigint_Platform>,
        shift: Option<i32>,
        allow_inplace: Option<bool>,
    ) -> Self {
        let platform: Value<PtrDyn<dyn v8_bigint_Platform>> = Rc::new(RefCell::new(platform));
        let shift: Value<i32> = Rc::new(RefCell::new(shift.unwrap_or(-1_i32)));
        let allow_inplace: Value<bool> = Rc::new(RefCell::new(allow_inplace.unwrap_or(false)));
        let __this : Value<v8_bigint_ShiftedDigits> = Rc::new(RefCell::new(Self { shift_ : <Value<i32 > >::default() , inplace_ : <Value<bool > >::default() , storage_ : Rc::new(RefCell::new(std_unique_ptr_unsigned_longarrarr__v8_bigint_Platform_Deleter_ :: std_unique_ptr_unsigned_longarrarr__v8_bigint_Platform_Deleter_1 ( {  Default::default()   } , { let __tmp_3 : Value<v8_bigint_Platform_Deleter > = Rc::new(RefCell::new(v8_bigint_Platform_Deleter :: v8_bigint_Platform_Deleter ( {  ((*platform.borrow()) ).clone()  } , ) )); __tmp_3.as_pointer()  } , ) )) , } )) ;
        let this: Ptr<v8_bigint_ShiftedDigits> = __this.as_pointer();
        let leading_zeros: Value<i32> = Rc::new(RefCell::new(
            ({ CountLeadingZeros_26(({ v8_bigint_DigitsImpl::msd(&original) })) }),
        ));
        if ((*shift.borrow()) < 0) {
            (*shift.borrow_mut()) = (*leading_zeros.borrow());
        } else if ((*shift.borrow()) > (*leading_zeros.borrow())) {
            (*allow_inplace.borrow_mut()) = false;
            (*(*this.upgrade().deref()).len_.borrow_mut()).postfix_inc();
        }
        (*(*this.upgrade().deref()).shift_.borrow_mut()) = (*shift.borrow());
        if ((*shift.borrow()) == 0) {
            (*(*this.upgrade().deref()).inplace_.borrow_mut()) = true;
            return;
        }
        (*(*this.upgrade().deref()).inplace_.borrow_mut()) = (*allow_inplace.borrow());
        if !(*(*this.upgrade().deref()).inplace_.borrow()) {
            ({
                let ___ptr: Ptr<u64> = ({
                    (*(*platform.borrow()).upgrade().deref())
                        .Allocate(((*(*this.upgrade().deref()).len_.borrow()) as usize))
                });
                (*(*this.upgrade().deref()).storage_.borrow()).reset(___ptr)
            });
            (*(*this.upgrade().deref()).digits_.borrow_mut()) =
                ({ (*(*this.upgrade().deref()).storage_.borrow()).get() });
        }
        let rw_view: Value<v8_bigint_RWDigits> =
            Rc::new(RefCell::new(v8_bigint_RWDigits::v8_bigint_RWDigits1(
                { (*(*this.upgrade().deref()).digits_.borrow()).clone() },
                { (*(*this.upgrade().deref()).len_.borrow()) },
            )));
        ({
            let _Z: v8_bigint_RWDigits = (*rw_view.borrow()).clone();
            let _X: v8_bigint_Digits = (*original.upgrade().deref()).clone();
            let _shift: i32 = (*(*this.upgrade().deref()).shift_.borrow());
            LeftShift_91(_Z, _X, _shift)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn v8_bigint_ShiftedDigits2(
        original: Ptr<v8_bigint_Digits>,
        scratch: v8_bigint_RWDigits,
    ) -> Self {
        let scratch: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(scratch));
        let __this : Value<v8_bigint_ShiftedDigits> = Rc::new(RefCell::new(Self { shift_ : <Value<i32 > >::default() , inplace_ : <Value<bool > >::default() , storage_ : Rc::new(RefCell::new(std_unique_ptr_unsigned_longarrarr__v8_bigint_Platform_Deleter_ :: std_unique_ptr_unsigned_longarrarr__v8_bigint_Platform_Deleter_1 ( {  Default::default()   } , { let __tmp_4 : Value<v8_bigint_Platform_Deleter > = Rc::new(RefCell::new(v8_bigint_Platform_Deleter :: v8_bigint_Platform_Deleter ( {  Ptr::<dyn v8_bigint_Platform>::null()   } , ) )); __tmp_4.as_pointer()  } , ) )) , } )) ;
        let this: Ptr<v8_bigint_ShiftedDigits> = __this.as_pointer();
        (&(0));
        (*(*this.upgrade().deref()).shift_.borrow_mut()) =
            ({ CountLeadingZeros_26(({ v8_bigint_DigitsImpl::msd(&original) })) });
        if ((*(*this.upgrade().deref()).shift_.borrow()) == 0) {
            (*(*this.upgrade().deref()).inplace_.borrow_mut()) = true;
            let __rhs = (*(*original.upgrade().deref()).digits_.borrow()).clone();
            (*(*this.upgrade().deref()).digits_.borrow_mut()) = __rhs;
            return;
        }
        (*(*this.upgrade().deref()).inplace_.borrow_mut()) = false;
        let rw_view: Value<v8_bigint_RWDigits> =
            Rc::new(RefCell::new(v8_bigint_RWDigits::v8_bigint_RWDigits1(
                { (*(*this.upgrade().deref()).digits_.borrow()).clone() },
                { (*(*this.upgrade().deref()).len_.borrow()) },
            )));
        ({
            let _Z: v8_bigint_RWDigits = (*rw_view.borrow()).clone();
            let _X: v8_bigint_Digits = (*original.upgrade().deref()).clone();
            let _shift: i32 = (*(*this.upgrade().deref()).shift_.borrow());
            LeftShift_91(_Z, _X, _shift)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_ShiftedDigits {
    fn byte_size() -> usize {
        40
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.shift_.borrow()).to_bytes(&mut buf[12..16]);
        (*self.inplace_.borrow()).to_bytes(&mut buf[16..17]);
        (*self.storage_.borrow()).to_bytes(&mut buf[24..40]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            shift_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[12..16]))),
            inplace_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[16..17]))),
            storage_: Rc::new(RefCell::new(<Option<Value<Box<[u64]>>>>::from_bytes(
                &buf[24..40],
            ))),
        }
    }
}
pub fn ProductGreaterThan_94(factor1: u64, factor2: u64, high: u64, low: u64) -> bool {
    let factor1: Value<u64> = Rc::new(RefCell::new(factor1));
    let factor2: Value<u64> = Rc::new(RefCell::new(factor2));
    let high: Value<u64> = Rc::new(RefCell::new(high));
    let low: Value<u64> = Rc::new(RefCell::new(low));
    let result_high: Value<u64> = <Value<u64>>::default();
    let result_low: Value<u64> = Rc::new(RefCell::new(
        ({
            digit_mul_38(
                (*factor1.borrow()),
                (*factor2.borrow()),
                (result_high.as_pointer()),
            )
        }),
    ));
    return ((*result_high.borrow()) > (*high.borrow()))
        || (((*result_high.borrow()) == (*high.borrow()))
            && ((*result_low.borrow()) > (*low.borrow())));
}
#[derive(Default)]
pub struct v8_bigint_MultiplicativeDigitDiv {
    divisor_: Value<u64>,
    inverse_: Value<u64>,
}
impl v8_bigint_MultiplicativeDigitDiv {
    pub fn v8_bigint_MultiplicativeDigitDiv(divisor: u64) -> Self {
        let divisor: Value<u64> = Rc::new(RefCell::new(divisor));
        let __this: Value<v8_bigint_MultiplicativeDigitDiv> = Rc::new(RefCell::new(Self {
            divisor_: Rc::new(RefCell::new((*divisor.borrow()))),
            inverse_: Rc::new(RefCell::new(
                ({ v8_bigint_MultiplicativeDigitDiv::ComputeInverse((*divisor.borrow())) }),
            )),
        }));
        let this: Ptr<v8_bigint_MultiplicativeDigitDiv> = __this.as_pointer();
        (&(0));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    fn ComputeInverse(divisor: u64) -> u64 {
        let divisor: Value<u64> = Rc::new(RefCell::new(divisor));
        let high: Value<u64> = Rc::new(RefCell::new(!(*divisor.borrow())));
        let low: Value<u64> = Rc::new(RefCell::new(!0_u64));
        return ((((((*high.borrow()) as u128) << 64) | ((*low.borrow()) as u128))
            .wrapping_div(((*divisor.borrow()) as u128))) as u64);
    }
}
impl Clone for v8_bigint_MultiplicativeDigitDiv {
    fn clone(&self) -> Self {
        let __this: Value<v8_bigint_MultiplicativeDigitDiv> = Rc::new(RefCell::new(Self {
            divisor_: Rc::new(RefCell::new((*self.divisor_.borrow()))),
            inverse_: Rc::new(RefCell::new((*self.inverse_.borrow()))),
        }));
        let this: Ptr<v8_bigint_MultiplicativeDigitDiv> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for v8_bigint_MultiplicativeDigitDiv {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.divisor_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.inverse_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            divisor_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[0..8]))),
            inverse_: Rc::new(RefCell::new(<u64>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_bigint_Platform_Deleter;
pub trait v8_bigint_DigitsImpl {
    fn operator_add(&self, i: u32) -> v8_bigint_Digits;
    fn operator_index(&self, i: u32) -> u64;
    fn msd(&self) -> u64;
    fn operator_eq(&self, other: Ptr<v8_bigint_Digits>) -> bool;
    fn Normalize(&self);
    fn TrimOne(&self);
    fn len(&self) -> u32;
    fn digits(&self) -> Ptr<u64>;
    fn read_4byte_aligned(&self, i: u32) -> u64;
}
impl v8_bigint_DigitsImpl for Ptr<v8_bigint_Digits> {
    fn operator_add(&self, i: u32) -> v8_bigint_Digits {
        let i: Value<u32> = Rc::new(RefCell::new(i));
        (&(0));
        return v8_bigint_Digits::v8_bigint_Digits2(
            { (*(*(*self).upgrade().deref()).digits_.borrow()).offset((*i.borrow()) as isize) },
            { (*(*(*self).upgrade().deref()).len_.borrow()).wrapping_sub((*i.borrow())) },
        );
    }
    fn operator_index(&self, i: u32) -> u64 {
        let i: Value<u32> = Rc::new(RefCell::new(i));
        (&(0));
        return ({ v8_bigint_DigitsImpl::read_4byte_aligned(self, (*i.borrow())) });
    }
    fn msd(&self) -> u64 {
        (&(0));
        return ({
            let _i: u32 = (*(*(*self).upgrade().deref()).len_.borrow()).wrapping_sub(1_u32);
            v8_bigint_DigitsImpl::read_4byte_aligned(self, _i)
        });
    }
    fn operator_eq(&self, other: Ptr<v8_bigint_Digits>) -> bool {
        return ({
            let _lhs = (*(*(*self).upgrade().deref()).digits_.borrow()).clone();
            _lhs == (*(*other.upgrade().deref()).digits_.borrow()).clone()
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).len_.borrow());
            _lhs == (*(*other.upgrade().deref()).len_.borrow())
        });
    }
    fn Normalize(&self) {
        'loop_: while ((*(*(*self).upgrade().deref()).len_.borrow()) > 0_u32)
            && (({ v8_bigint_DigitsImpl::msd(self) }) == 0_u64)
        {
            (*(*(*self).upgrade().deref()).len_.borrow_mut()).postfix_dec();
        }
    }
    fn TrimOne(&self) {
        (&(0));
        (*(*(*self).upgrade().deref()).len_.borrow_mut()).postfix_dec();
    }
    fn len(&self) -> u32 {
        return (*(*(*self).upgrade().deref()).len_.borrow());
    }
    fn digits(&self) -> Ptr<u64> {
        return (*(*(*self).upgrade().deref()).digits_.borrow()).clone();
    }
    fn read_4byte_aligned(&self, i: u32) -> u64 {
        let i: Value<u32> = Rc::new(RefCell::new(i));
        if (::std::mem::size_of::<u64>() == 4_usize) {
            return ((*(*(*self).upgrade().deref()).digits_.borrow())
                .offset((*i.borrow()) as isize)
                .read());
        } else {
            let result: Value<u64> = <Value<u64>>::default();
            {
                ((result.as_pointer()) as Ptr<u64>).to_any().memcpy(
                    &((*(*(*self).upgrade().deref()).digits_.borrow())
                        .offset((*i.borrow()) as isize) as Ptr<u64>)
                        .to_any(),
                    ::std::mem::size_of::<u64>() as usize,
                );
                ((result.as_pointer()) as Ptr<u64>).to_any()
            };
            return (*result.borrow());
        }
        panic!("ub: non-void function does not return a value")
    }
}
pub trait v8_bigint_FromStringAccumulatorImpl {
    fn result(&self) -> v8_bigint_FromStringAccumulator_Result;
    fn ResultLength(&self) -> u32;
    fn AddPart_u64_u64_bool(&self, multiplier: u64, part: u64, is_last: bool) -> bool;
    fn AddPart_u64(&self, part: u64) -> bool;
    fn destructor(&self);
}
impl v8_bigint_FromStringAccumulatorImpl for Ptr<v8_bigint_FromStringAccumulator> {
    fn result(&self) -> v8_bigint_FromStringAccumulator_Result {
        return (*(*(*self).upgrade().deref()).result_.borrow());
    }
    fn ResultLength(&self) -> u32 {
        return {
            let __tmp_1: Value<u32> = Rc::new(RefCell::new(
                (({
                    v8_bigint_GrowableDigitsVectorImpl::size(
                        &(*(*self).upgrade().deref()).heap_parts_.as_pointer(),
                    )
                }) as u32),
            ));
            (if (*(*self).upgrade().deref())
                .stack_parts_used_
                .as_pointer()
                .read()
                >= __tmp_1.as_pointer().read()
            {
                (*(*self).upgrade().deref()).stack_parts_used_.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        };
    }
    fn destructor(&self) {
        (*self.upgrade().deref())
            .heap_parts_
            .as_pointer()
            .destructor();
    }
    fn AddPart_u64_u64_bool(&self, multiplier: u64, part: u64, is_last: bool) -> bool {
        let multiplier: Value<u64> = Rc::new(RefCell::new(multiplier));
        let part: Value<u64> = Rc::new(RefCell::new(part));
        let is_last: Value<bool> = Rc::new(RefCell::new(is_last));
        if (*(*(*self).upgrade().deref()).inline_everything_.borrow()) {
            let carry: Value<u64> = Rc::new(RefCell::new((*part.borrow())));
            let high: Value<u64> = Rc::new(RefCell::new(0_u64));
            let i: Value<u32> = Rc::new(RefCell::new(0_u32));
            'loop_: while ((*i.borrow())
                < (*(*(*self).upgrade().deref()).stack_parts_used_.borrow()))
            {
                let result: Value<u128> = Rc::new(RefCell::new(
                    ((*(*(*self).upgrade().deref()).stack_parts_.borrow())[(*i.borrow()) as usize]
                        as u128)
                        .wrapping_mul(((*multiplier.borrow()) as u128)),
                ));
                let new_high: Value<u64> =
                    Rc::new(RefCell::new((((*result.borrow()) >> 64) as u64)));
                let low: Value<u64> = Rc::new(RefCell::new(((*result.borrow()) as u64)));
                (*result.borrow_mut()) = (((*low.borrow()) as u128)
                    .wrapping_add(((*high.borrow()) as u128)))
                .wrapping_add(((*carry.borrow()) as u128));
                (*carry.borrow_mut()) = (((*result.borrow()) >> 64) as u64);
                (*(*(*self).upgrade().deref()).stack_parts_.borrow_mut())[(*i.borrow()) as usize] =
                    ((*result.borrow()) as u64);
                (*high.borrow_mut()) = (*new_high.borrow());
                (*i.borrow_mut()).postfix_inc();
            }
            {
                let rhs_0 = (*high.borrow()).wrapping_add((*carry.borrow()));
                (*high.borrow_mut()) = rhs_0
            };
            if ((*high.borrow()) != 0_u64) {
                (*(*(*self).upgrade().deref()).stack_parts_.borrow_mut())[((*(*(*self)
                    .upgrade()
                    .deref())
                .stack_parts_used_
                .borrow_mut())
                .postfix_inc())
                    as usize] = (*high.borrow());
            }
            (&(0));
            return true;
        }
        if (*is_last.borrow()) {
            (*(*(*self).upgrade().deref()).last_multiplier_.borrow_mut()) = (*multiplier.borrow());
        } else {
            (&(0));
            (*(*(*self).upgrade().deref()).max_multiplier_.borrow_mut()) = (*multiplier.borrow());
        }
        return ({ v8_bigint_FromStringAccumulatorImpl::AddPart_u64(self, (*part.borrow())) });
    }
    fn AddPart_u64(&self, part: u64) -> bool {
        let part: Value<u64> = Rc::new(RefCell::new(part));
        if ((*(*(*self).upgrade().deref()).stack_parts_used_.borrow()) < 8) {
            (*(*(*self).upgrade().deref()).stack_parts_.borrow_mut())[((*(*(*self)
                .upgrade()
                .deref())
            .stack_parts_used_
            .borrow_mut())
            .postfix_inc())
                as usize] = (*part.borrow());
            return true;
        }
        if (({
            v8_bigint_GrowableDigitsVectorImpl::size(
                &(*(*self).upgrade().deref()).heap_parts_.as_pointer(),
            )
        }) == 0_usize)
        {
            ({
                v8_bigint_GrowableDigitsVectorImpl::Init(
                    &(*(*self).upgrade().deref()).heap_parts_.as_pointer(),
                    (((8).wrapping_mul(2_u32)) as usize),
                )
            });
            let i: Value<u32> = Rc::new(RefCell::new(0_u32));
            'loop_: while ((*i.borrow()) < 8) {
                ({
                    let _value: u64 = (*(*(*self).upgrade().deref()).stack_parts_.borrow())
                        [(*i.borrow()) as usize];
                    v8_bigint_GrowableDigitsVectorImpl::push_back(
                        &(*(*self).upgrade().deref()).heap_parts_.as_pointer(),
                        _value,
                    )
                });
                (*i.borrow_mut()).postfix_inc();
            }
        }
        if ((({
            v8_bigint_GrowableDigitsVectorImpl::size(
                &(*(*self).upgrade().deref()).heap_parts_.as_pointer(),
            )
        }) as u32)
            >= (*(*(*self).upgrade().deref()).max_digits_.borrow()))
        {
            (*(*(*self).upgrade().deref()).result_.borrow_mut()) =
                v8_bigint_FromStringAccumulator_Result_kMaxSizeExceeded;
            return false;
        }
        ({
            v8_bigint_GrowableDigitsVectorImpl::push_back(
                &(*(*self).upgrade().deref()).heap_parts_.as_pointer(),
                (*part.borrow()),
            )
        });
        return true;
    }
}
pub trait v8_bigint_GrowableDigitsVectorImpl {
    fn destructor(&self);
    fn Init(&self, capacity: usize);
    fn push_back(&self, value: u64);
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn operator_index(&self, i: usize) -> u64;
    fn back(&self) -> u64;
    fn data(&self) -> Ptr<u64>;
}
impl v8_bigint_GrowableDigitsVectorImpl for Ptr<v8_bigint_GrowableDigitsVector> {
    fn destructor(&self) {
        if !(*(*(*self).upgrade().deref()).data_.borrow()).is_null() {
            ({
                let _ptr: Ptr<u64> = (*(*(*self).upgrade().deref()).data_.borrow()).clone();
                (*(*(*(*self).upgrade().deref()).platform_.borrow())
                    .upgrade()
                    .deref())
                .Free(_ptr)
            });
        }
    }
    fn Init(&self, capacity: usize) {
        let capacity: Value<usize> = Rc::new(RefCell::new(capacity));
        (&(0));
        (*(*(*self).upgrade().deref()).data_.borrow_mut()) = ({
            (*(*(*self).upgrade().deref()).end_.borrow_mut()) = ({
                (*(*(*(*self).upgrade().deref()).platform_.borrow())
                    .upgrade()
                    .deref())
                .Allocate((*capacity.borrow()))
            });
            (*(*(*self).upgrade().deref()).end_.borrow()).clone()
        })
        .clone();
        (*(*(*self).upgrade().deref()).capacity_.borrow_mut()) =
            (*(*(*self).upgrade().deref()).data_.borrow()).offset((*capacity.borrow()) as isize);
    }
    fn push_back(&self, value: u64) {
        let value: Value<u64> = Rc::new(RefCell::new(value));
        if {
            let _lhs = (*(*(*self).upgrade().deref()).end_.borrow()).clone();
            _lhs == (*(*(*self).upgrade().deref()).capacity_.borrow()).clone()
        } {
            {
                let capacity: Value<usize> = Rc::new(RefCell::new(
                    ((((*(*(*self).upgrade().deref()).capacity_.borrow()).clone()
                        - (*(*(*self).upgrade().deref()).data_.borrow()).clone())
                        as i64) as usize),
                ));
                let new_data: Value<Ptr<u64>> = Rc::new(RefCell::new(
                    ({
                        (*(*(*(*self).upgrade().deref()).platform_.borrow())
                            .upgrade()
                            .deref())
                        .Allocate((2_usize).wrapping_mul((*capacity.borrow())))
                    }),
                ));
                {
                    ((*new_data.borrow()).clone() as Ptr<u64>).to_any().memcpy(
                        &((*(*(*self).upgrade().deref()).data_.borrow()).clone() as Ptr<u64>)
                            .to_any(),
                        (((*capacity.borrow()) as u64)
                            .wrapping_mul((::std::mem::size_of::<u64>() as u64))
                            as usize) as usize,
                    );
                    ((*new_data.borrow()).clone() as Ptr<u64>).to_any()
                };
                ({
                    let _ptr: Ptr<u64> = (*(*(*self).upgrade().deref()).data_.borrow()).clone();
                    (*(*(*(*self).upgrade().deref()).platform_.borrow())
                        .upgrade()
                        .deref())
                    .Free(_ptr)
                });
                (*(*(*self).upgrade().deref()).data_.borrow_mut()) = (*new_data.borrow()).clone();
                (*(*(*self).upgrade().deref()).capacity_.borrow_mut()) =
                    (*(*(*self).upgrade().deref()).data_.borrow())
                        .offset(((2_usize).wrapping_mul((*capacity.borrow()))) as isize);
                (*(*(*self).upgrade().deref()).end_.borrow_mut()) =
                    (*(*(*self).upgrade().deref()).data_.borrow())
                        .offset((*capacity.borrow()) as isize);
            };
        }
        let __rhs = (*value.borrow());
        (*(*(*self).upgrade().deref()).end_.borrow_mut())
            .postfix_inc()
            .write(__rhs);
    }
    fn empty(&self) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).end_.borrow()).clone();
            _lhs == (*(*(*self).upgrade().deref()).data_.borrow()).clone()
        };
    }
    fn size(&self) -> usize {
        return ((((*(*(*self).upgrade().deref()).end_.borrow()).clone()
            - (*(*(*self).upgrade().deref()).data_.borrow()).clone()) as i64)
            as usize);
    }
    fn operator_index(&self, i: usize) -> u64 {
        let i: Value<usize> = Rc::new(RefCell::new(i));
        return ((*(*(*self).upgrade().deref()).data_.borrow())
            .offset((*i.borrow()) as isize)
            .read());
    }
    fn back(&self) -> u64 {
        return (((*(*(*self).upgrade().deref()).end_.borrow()).offset(-((1) as isize))).read());
    }
    fn data(&self) -> Ptr<u64> {
        return (*(*(*self).upgrade().deref()).data_.borrow()).clone();
    }
}
pub trait v8_bigint_MultiplicativeDigitDivImpl {
    fn div(&self, high: u64, low: u64, remainder: Ptr<u64>) -> u64;
}
impl v8_bigint_MultiplicativeDigitDivImpl for Ptr<v8_bigint_MultiplicativeDigitDiv> {
    fn div(&self, high: u64, low: u64, remainder: Ptr<u64>) -> u64 {
        let high: Value<u64> = Rc::new(RefCell::new(high));
        let low: Value<u64> = Rc::new(RefCell::new(low));
        let remainder: Value<Ptr<u64>> = Rc::new(RefCell::new(remainder));
        let q1: Value<u64> = <Value<u64>>::default();
        let q0: Value<u64> = Rc::new(RefCell::new(
            ({
                digit_mul_38(
                    (*high.borrow()),
                    (*(*(*self).upgrade().deref()).inverse_.borrow()),
                    (q1.as_pointer()),
                )
            }),
        ));
        let carry: Value<u64> = <Value<u64>>::default();
        let __rhs = ({ digit_add2_34((*q0.borrow()), (*low.borrow()), (carry.as_pointer())) });
        (*q0.borrow_mut()) = __rhs;
        let __rhs = ({
            let _c: u64 = (*carry.borrow());
            let _carry: Ptr<u64> = (carry.as_pointer());
            digit_add3_35((*q1.borrow()), (*high.borrow()), _c, _carry)
        });
        (*q1.borrow_mut()) = __rhs;
        (*q1.borrow_mut()).postfix_inc();
        let r: Value<u64> = Rc::new(RefCell::new((*low.borrow()).wrapping_sub(
            (*q1.borrow()).wrapping_mul((*(*(*self).upgrade().deref()).divisor_.borrow())),
        )));
        if ((*r.borrow()) > (*q0.borrow())) {
            (*q1.borrow_mut()).postfix_dec();
            {
                let rhs_0 =
                    (*r.borrow()).wrapping_add((*(*(*self).upgrade().deref()).divisor_.borrow()));
                (*r.borrow_mut()) = rhs_0
            };
        }
        if ((*r.borrow()) >= (*(*(*self).upgrade().deref()).divisor_.borrow())) {
            {
                (*q1.borrow_mut()).postfix_inc();
                {
                    let rhs_0 = (*r.borrow())
                        .wrapping_sub((*(*(*self).upgrade().deref()).divisor_.borrow()));
                    (*r.borrow_mut()) = rhs_0
                };
            };
        }
        let __rhs = (*r.borrow());
        (*remainder.borrow()).write(__rhs);
        return (*q1.borrow());
    }
}
pub trait v8_bigint_ProcessorImpl {
    fn CachedMod(&self, R: Ptr<v8_bigint_RWDigits>, A: Ptr<v8_bigint_Digits>) -> u64;
    fn GetCachedDivisor(&self) -> Ptr<v8_bigint_Digits>;
    fn inc_divisor_count(&self) -> i32;
    fn reset_divisor_count(&self);
    fn ResetInterruptCheckBudget(&self);
    fn platform(&self) -> PtrDyn<dyn v8_bigint_Platform>;
    fn GetSmallScratch(&self) -> v8_bigint_RWDigits;
    fn GetSmallScratch_NoCheck(&self) -> v8_bigint_RWDigits;
    fn GetCachedInverse(&self) -> Ptr<v8_bigint_Digits>;
    fn CachedModFold(&self, R: Ptr<v8_bigint_RWDigits>, A: Ptr<v8_bigint_Digits>, c: u64) -> u64;
    fn GetCachedModFoldFactor(&self) -> u64;
    fn set_cached_mod_fold_factor(&self, factor: u64);
}
impl v8_bigint_ProcessorImpl for Ptr<v8_bigint_Processor> {
    fn GetCachedDivisor(&self) -> Ptr<v8_bigint_Digits> {
        (&(0));
        return (*(*self).upgrade().deref()).cached_divisor_.as_pointer();
    }
    fn inc_divisor_count(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).divisor_count_.borrow_mut()).prefix_inc();
    }
    fn reset_divisor_count(&self) {
        (*(*(*self).upgrade().deref()).divisor_count_.borrow_mut()) = 1;
    }
    fn ResetInterruptCheckBudget(&self) {
        (*(*(*self).upgrade().deref()).work_estimate_.borrow_mut()) = 0_u64;
    }
    fn platform(&self) -> PtrDyn<dyn v8_bigint_Platform> {
        return (*(*(*self).upgrade().deref()).platform_.borrow()).as_pointer();
    }
    fn GetSmallScratch(&self) -> v8_bigint_RWDigits {
        if !({ (*(*(*self).upgrade().deref()).small_scratch_.borrow())() }) {
            ({
                let ___ptr: Ptr<u64> = ({
                    (*(*(*(*self).upgrade().deref()).platform_.borrow())
                        .as_ref()
                        .unwrap()
                        .borrow())
                    .Allocate((100 as usize))
                });
                (*(*(*self).upgrade().deref()).small_scratch_.borrow()).reset(___ptr)
            });
        }
        return v8_bigint_RWDigits::v8_bigint_RWDigits1(
            { ({ (*(*(*self).upgrade().deref()).small_scratch_.borrow()).get() }) },
            { 100 },
        );
    }
    fn GetSmallScratch_NoCheck(&self) -> v8_bigint_RWDigits {
        (&(0));
        return v8_bigint_RWDigits::v8_bigint_RWDigits1(
            { ({ (*(*(*self).upgrade().deref()).small_scratch_.borrow()).get() }) },
            { 100 },
        );
    }
    fn GetCachedInverse(&self) -> Ptr<v8_bigint_Digits> {
        (&(0));
        return (*(*self).upgrade().deref()).cached_inverse_.as_pointer();
    }
    fn GetCachedModFoldFactor(&self) -> u64 {
        return (*(*(*self).upgrade().deref())
            .cached_mod_fold_factor_
            .borrow());
    }
    fn set_cached_mod_fold_factor(&self, factor: u64) {
        let factor: Value<u64> = Rc::new(RefCell::new(factor));
        (*(*(*self).upgrade().deref())
            .cached_mod_fold_factor_
            .borrow_mut()) = (*factor.borrow());
    }
    fn CachedModFold(&self, R: Ptr<v8_bigint_RWDigits>, A: Ptr<v8_bigint_Digits>, c: u64) -> u64 {
        let c: Value<u64> = Rc::new(RefCell::new(c));
        let B: Ptr<v8_bigint_Digits> = ({ v8_bigint_ProcessorImpl::GetCachedDivisor(self) });
        let n: Value<u32> = Rc::new(RefCell::new(({ v8_bigint_DigitsImpl::len(&B) })));
        (&(0));
        (&(0));
        (&(0));
        let carry: Value<u64> = Rc::new(RefCell::new(0_u64));
        let i: Value<u32> = Rc::new(RefCell::new(0_u32));
        'loop_: while ((*i.borrow()) < (*n.borrow())) {
            let high: Value<u64> = Rc::new(RefCell::new(0_u64));
            let low: Value<u64> = Rc::new(RefCell::new(
                ({
                    let _i: u32 = (*i.borrow());
                    v8_bigint_DigitsImpl::operator_index(&A, _i)
                }),
            ));
            if {
                let _lhs = (*n.borrow()).wrapping_add((*i.borrow()));
                _lhs < ({ v8_bigint_DigitsImpl::len(&A) })
            } {
                let product_low: Value<u64> = Rc::new(RefCell::new(
                    ({
                        digit_mul_38(
                            ({
                                let _i: u32 = (*n.borrow()).wrapping_add((*i.borrow()));
                                v8_bigint_DigitsImpl::operator_index(&A, _i)
                            }),
                            (*c.borrow()),
                            (high.as_pointer()),
                        )
                    }),
                ));
                let add_carry: Value<u64> = <Value<u64>>::default();
                let __rhs = ({
                    digit_add2_34(
                        (*low.borrow()),
                        (*product_low.borrow()),
                        (add_carry.as_pointer()),
                    )
                });
                (*low.borrow_mut()) = __rhs;
                {
                    let rhs_0 = (*high.borrow()).wrapping_add((*add_carry.borrow()));
                    (*high.borrow_mut()) = rhs_0
                };
            }
            let sum_carry: Value<u64> = <Value<u64>>::default();
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            let _i: u32 = (*i.borrow());
                            v8_bigint_RWDigitsImpl::operator_index(&R, _i)
                        }),
                    ))
                    .as_pointer(),
                    ({
                        digit_add2_34((*low.borrow()), (*carry.borrow()), (sum_carry.as_pointer()))
                    }),
                )
            });
            (&(0));
            (*carry.borrow_mut()) = (*high.borrow()).wrapping_add((*sum_carry.borrow()));
            (*i.borrow_mut()).prefix_inc();
        }
        if ((*carry.borrow()) != 0) {
            let product_high: Value<u64> = <Value<u64>>::default();
            let product_low: Value<u64> = Rc::new(RefCell::new(
                ({
                    digit_mul_38(
                        (*carry.borrow()),
                        (*c.borrow()),
                        (product_high.as_pointer()),
                    )
                }),
            ));
            ({
                let _digit: u64 = ({
                    digit_add2_34(
                        ({
                            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                                &Rc::new(RefCell::new(
                                    ({ v8_bigint_RWDigitsImpl::operator_index(&R, 0_u32) }),
                                ))
                                .as_pointer(),
                            )
                        }),
                        (*product_low.borrow()),
                        (carry.as_pointer()),
                    )
                });
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({ v8_bigint_RWDigitsImpl::operator_index(&R, 0_u32) }),
                    ))
                    .as_pointer(),
                    _digit,
                )
            });
            {
                let rhs_0 = (*carry.borrow()).wrapping_add((*product_high.borrow()));
                (*carry.borrow_mut()) = rhs_0
            };
            let i: Value<u32> = Rc::new(RefCell::new(1_u32));
            'loop_: while ((*i.borrow()) < (*n.borrow())) && ((*carry.borrow()) != 0) {
                ({
                    let _digit: u64 = ({
                        let _b: u64 = (*carry.borrow());
                        let _carry: Ptr<u64> = (carry.as_pointer());
                        digit_add2_34(
                            ({
                                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                                    &Rc::new(RefCell::new(
                                        ({
                                            let _i: u32 = (*i.borrow());
                                            v8_bigint_RWDigitsImpl::operator_index(&R, _i)
                                        }),
                                    ))
                                    .as_pointer(),
                                )
                            }),
                            _b,
                            _carry,
                        )
                    });
                    v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                        &Rc::new(RefCell::new(
                            ({
                                let _i: u32 = (*i.borrow());
                                v8_bigint_RWDigitsImpl::operator_index(&R, _i)
                            }),
                        ))
                        .as_pointer(),
                        _digit,
                    )
                });
                (*i.borrow_mut()).prefix_inc();
            }
        }
        'loop_: while ((*carry.borrow()) != 0)
            || ({
                let _A: v8_bigint_Digits = (*R.upgrade().deref()).clone();
                let _B: v8_bigint_Digits = (*B.upgrade().deref()).clone();
                GreaterThanOrEqual_40(_A, _B)
            })
        {
            {
                let rhs_0 = (*carry.borrow()).wrapping_sub(
                    ({
                        let _Z: v8_bigint_RWDigits = (*R.upgrade().deref()).clone();
                        let _X: v8_bigint_Digits = (*B.upgrade().deref()).clone();
                        InplaceSubAndReturnBorrow_54(_Z, _X)
                    }),
                );
                (*carry.borrow_mut()) = rhs_0
            };
        }
        return ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                &Rc::new(RefCell::new(
                    ({
                        let _i: u32 = (*n.borrow()).wrapping_sub(1_u32);
                        v8_bigint_RWDigitsImpl::operator_index(&R, _i)
                    }),
                ))
                .as_pointer(),
            )
        });
    }
    fn CachedMod(&self, R: Ptr<v8_bigint_RWDigits>, A: Ptr<v8_bigint_Digits>) -> u64 {
        let c: Value<u64> = Rc::new(RefCell::new(
            ({ v8_bigint_ProcessorImpl::GetCachedModFoldFactor(self) }),
        ));
        if ((*c.borrow()) != 0_u64) {
            return ({
                let _R: Ptr<v8_bigint_RWDigits> = (R).clone();
                let _A: Ptr<v8_bigint_Digits> = (A).clone();
                let _c: u64 = (*c.borrow());
                v8_bigint_ProcessorImpl::CachedModFold(self, _R, _A, _c)
            });
        }
        let B: Ptr<v8_bigint_Digits> = ({ v8_bigint_ProcessorImpl::GetCachedDivisor(self) });
        let inv: Ptr<v8_bigint_Digits> = ({ v8_bigint_ProcessorImpl::GetCachedInverse(self) });
        let n: Value<u32> = Rc::new(RefCell::new(({ v8_bigint_DigitsImpl::len(&B) })));
        (&(0));
        (&(0));
        (&(0));
        (&(0));
        let scratch_space: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&A) }).wrapping_add(({ v8_bigint_DigitsImpl::len(&inv) })),
        ));
        let scratch: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(
            ({ v8_bigint_ProcessorImpl::GetSmallScratch_NoCheck(self) }),
        ));
        ({ v8_bigint_RWDigitsImpl::set_len(&scratch.as_pointer(), (*scratch_space.borrow())) });
        let start_position: Value<u32> = Rc::new(RefCell::new(
            ((2_u32).wrapping_mul((*n.borrow()))).wrapping_sub(2_u32),
        ));
        if {
            let _lhs = ({ v8_bigint_DigitsImpl::len(&A) });
            _lhs >= ({ v8_bigint_DigitsImpl::len(&inv) })
        } {
            ({
                let _Z: v8_bigint_RWDigits = (*scratch.borrow()).clone();
                let _X: v8_bigint_Digits = (*A.upgrade().deref()).clone();
                let _Y: v8_bigint_Digits = (*inv.upgrade().deref()).clone();
                let _start_position: u32 = (*start_position.borrow());
                MultiplySpecialHigh_59(_Z, _X, _Y, _start_position)
            });
        } else {
            ({
                let _Z: v8_bigint_RWDigits = (*scratch.borrow()).clone();
                let _X: v8_bigint_Digits = (*inv.upgrade().deref()).clone();
                let _Y: v8_bigint_Digits = (*A.upgrade().deref()).clone();
                let _start_position: u32 = (*start_position.borrow());
                MultiplySpecialHigh_59(_Z, _X, _Y, _start_position)
            });
        }
        let Q: Value<v8_bigint_Digits> = Rc::new(RefCell::new(
            ({
                v8_bigint_RWDigitsImpl::operator_add(
                    &scratch.as_pointer(),
                    (2_u32).wrapping_mul((*n.borrow())),
                )
            }),
        ));
        let product_low: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(
            v8_bigint_RWDigits::v8_bigint_RWDigits2({ (*scratch.borrow()).clone() }, { 0_u32 }, {
                (*n.borrow()).wrapping_add(1_u32)
            }),
        ));
        ({
            let _Z: v8_bigint_RWDigits = (*product_low.borrow()).clone();
            let _X: v8_bigint_Digits = (*B.upgrade().deref()).clone();
            let _Y: v8_bigint_Digits = (*Q.borrow()).clone();
            MultiplySpecialLow_60(_Z, _X, _Y)
        });
        let borrow: Value<u64> = Rc::new(RefCell::new(
            ({
                let _Z: v8_bigint_RWDigits = (*R.upgrade().deref()).clone();
                let _X: v8_bigint_Digits = (*A.upgrade().deref()).clone();
                let _Y: v8_bigint_Digits = v8_bigint_Digits::v8_bigint_Digits3(
                    { (*product_low.borrow()).clone() },
                    { 0_u32 },
                    { (*n.borrow()) },
                );
                SubtractAndReturnBorrow_56(_Z, _X, _Y)
            }),
        ));
        let An: Value<u64> = Rc::new(RefCell::new(
            if {
                let _lhs = ({ v8_bigint_DigitsImpl::len(&A) });
                _lhs > (*n.borrow())
            } {
                ({
                    let _i: u32 = (*n.borrow());
                    v8_bigint_DigitsImpl::operator_index(&A, _i)
                })
            } else {
                0_u64
            },
        ));
        let r_high: Value<u64> = Rc::new(RefCell::new(
            ((*An.borrow()).wrapping_sub(
                ({
                    v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                        &Rc::new(RefCell::new(
                            ({
                                v8_bigint_RWDigitsImpl::operator_index(
                                    &product_low.as_pointer(),
                                    (*n.borrow()),
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                }),
            ))
            .wrapping_sub((*borrow.borrow())),
        ));
        if (((*r_high.borrow()) & (1_u64 << (64 - 1))) != 0) {
            let mut __do_while = true;
            'loop_: while __do_while || ((*r_high.borrow()) != 0_u64) {
                __do_while = false;
                {
                    let rhs_0 = (*r_high.borrow()).wrapping_add(
                        ({
                            let _Z: v8_bigint_RWDigits = (*R.upgrade().deref()).clone();
                            let _X: v8_bigint_Digits = (*B.upgrade().deref()).clone();
                            InplaceAddAndReturnCarry_53(_Z, _X)
                        }),
                    );
                    (*r_high.borrow_mut()) = rhs_0
                };
            }
        } else {
            'loop_: while ((*r_high.borrow()) != 0_u64)
                || ({
                    let _A: v8_bigint_Digits = (*R.upgrade().deref()).clone();
                    let _B: v8_bigint_Digits = (*B.upgrade().deref()).clone();
                    GreaterThanOrEqual_40(_A, _B)
                })
            {
                {
                    let rhs_0 = (*r_high.borrow()).wrapping_sub(
                        ({
                            let _Z: v8_bigint_RWDigits = (*R.upgrade().deref()).clone();
                            let _X: v8_bigint_Digits = (*B.upgrade().deref()).clone();
                            InplaceSubAndReturnBorrow_54(_Z, _X)
                        }),
                    );
                    (*r_high.borrow_mut()) = rhs_0
                };
            }
        }
        return ({
            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                &Rc::new(RefCell::new(
                    ({
                        let _i: u32 = (*n.borrow()).wrapping_sub(1_u32);
                        v8_bigint_RWDigitsImpl::operator_index(&R, _i)
                    }),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait v8_bigint_ProcessorImplImpl {
    fn DivideSchoolbook(
        &self,
        Q: Ptr<v8_bigint_RWDigits>,
        R: Ptr<v8_bigint_RWDigits>,
        A: Ptr<v8_bigint_Digits>,
        B: Ptr<v8_bigint_Digits>,
    );
    fn should_terminate(&self) -> bool;
    fn AddWorkEstimate(&self, estimate: u64);
}
impl v8_bigint_ProcessorImplImpl for Ptr<v8_bigint_ProcessorImpl> {
    fn should_terminate(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).status_.borrow()) == v8_bigint_Status_kInterrupted);
    }
    fn AddWorkEstimate(&self, estimate: u64) {
        let estimate: Value<u64> = Rc::new(RefCell::new(estimate));
        {
            let rhs_0 = (*(*(*self).upgrade().deref()).work_estimate_.borrow())
                .wrapping_add((*estimate.borrow()));
            (*(*(*self).upgrade().deref()).work_estimate_.borrow_mut()) = rhs_0
        };
        if ((*(*(*self).upgrade().deref()).work_estimate_.borrow())
            >= (*kWorkEstimateThreshold_12.with(Value::clone).borrow()))
        {
            (*(*(*self).upgrade().deref()).work_estimate_.borrow_mut()) = 0_u64;
            if ({
                (*(*(*(*self).upgrade().deref()).platform_.borrow())
                    .as_ref()
                    .unwrap()
                    .borrow())
                .InterruptRequested()
            }) {
                (*(*(*self).upgrade().deref()).status_.borrow_mut()) =
                    v8_bigint_Status_kInterrupted;
            }
        }
    }
    fn DivideSchoolbook(
        &self,
        Q: Ptr<v8_bigint_RWDigits>,
        R: Ptr<v8_bigint_RWDigits>,
        A: Ptr<v8_bigint_Digits>,
        B: Ptr<v8_bigint_Digits>,
    ) {
        (&(0));
        (&(0));
        (&(0));
        (&(0));
        let n: Value<u32> = Rc::new(RefCell::new(({ v8_bigint_DigitsImpl::len(&B) })));
        let m: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&A) }).wrapping_sub((*n.borrow())),
        ));
        let qhatv_len: Value<u32> = Rc::new(RefCell::new((*n.borrow()).wrapping_add(1_u32)));
        let b_normalized_storage_len: Value<u32> = Rc::new(RefCell::new((*n.borrow())));
        let U_len: Value<u32> = Rc::new(RefCell::new(
            ({ v8_bigint_DigitsImpl::len(&A) }).wrapping_add(1_u32),
        ));
        let needed_scratch_space: Value<u32> = Rc::new(RefCell::new(
            ((*qhatv_len.borrow()).wrapping_add((*b_normalized_storage_len.borrow())))
                .wrapping_add((*U_len.borrow())),
        ));
        let allocated_scratch: Value<std_optional_v8_bigint_ScratchDigits_> =
            Rc::new(RefCell::new(
                std_optional_v8_bigint_ScratchDigits_::std_optional_v8_bigint_ScratchDigits_1(),
            ));
        let scratch: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(
            v8_bigint_RWDigits::v8_bigint_RWDigits1({ Ptr::<u64>::null() }, { 0_u32 }),
        ));
        if ((*needed_scratch_space.borrow()) <= 100) {
            (*scratch.borrow_mut()) = ({ v8_bigint_ProcessorImpl::GetSmallScratch(self) });
        } else {
            (*allocated_scratch.borrow_mut()) = v8_bigint_ScratchDigits::v8_bigint_ScratchDigits(
                { (*needed_scratch_space.borrow()) },
                { ({ v8_bigint_ProcessorImpl::platform(self) }) },
            );
            (*scratch.borrow_mut()) = (*({ (*allocated_scratch.borrow()).value_lref() })
                .upgrade()
                .deref())
            .clone();
        }
        let qhatv: Value<v8_bigint_RWDigits> = Rc::new(RefCell::new(
            v8_bigint_RWDigits::v8_bigint_RWDigits2({ (*scratch.borrow()).clone() }, { 0_u32 }, {
                (*qhatv_len.borrow())
            }),
        ));
        let b_normalized_storage: Value<v8_bigint_RWDigits> =
            Rc::new(RefCell::new(v8_bigint_RWDigits::v8_bigint_RWDigits2(
                { (*scratch.borrow()).clone() },
                { (*qhatv_len.borrow()) },
                { (*b_normalized_storage_len.borrow()) },
            )));
        let BN: Value<v8_bigint_ShiftedDigits> = Rc::new(RefCell::new(
            v8_bigint_ShiftedDigits::v8_bigint_ShiftedDigits2({ (B).clone() }, {
                (*b_normalized_storage.borrow()).clone()
            }),
        ));
        let U: Value<v8_bigint_RWDigits> =
            Rc::new(RefCell::new(v8_bigint_RWDigits::v8_bigint_RWDigits2(
                { (*scratch.borrow()).clone() },
                { (*qhatv_len.borrow()).wrapping_add((*b_normalized_storage_len.borrow())) },
                { (*U_len.borrow()) },
            )));
        ({
            let _Z: v8_bigint_RWDigits = (*U.borrow()).clone();
            let _X: v8_bigint_Digits = (*A.upgrade().deref()).clone();
            let _shift: i32 = ({ v8_bigint_ShiftedDigitsImpl::shift(&BN.as_pointer()) });
            LeftShift_91(_Z, _X, _shift)
        });
        let vn1: Value<u64> = Rc::new(RefCell::new(
            ({
                v8_bigint_DigitsImpl::operator_index(
                    &BN.as_pointer(),
                    (*n.borrow()).wrapping_sub(1_u32),
                )
            }),
        ));
        let vn1_divisor: Value<v8_bigint_MultiplicativeDigitDiv> = Rc::new(RefCell::new(
            v8_bigint_MultiplicativeDigitDiv::v8_bigint_MultiplicativeDigitDiv({ (*vn1.borrow()) }),
        ));
        let j: Value<i32> = Rc::new(RefCell::new(((*m.borrow()) as i32)));
        'loop_: while ((*j.borrow()) >= 0) {
            let qhat: Value<u64> = Rc::new(RefCell::new(<u64>::MAX));
            let ujn: Value<u64> = Rc::new(RefCell::new(
                ({
                    v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                        &Rc::new(RefCell::new(
                            ({
                                v8_bigint_RWDigitsImpl::operator_index(
                                    &U.as_pointer(),
                                    ((*j.borrow()) as u32).wrapping_add((*n.borrow())),
                                )
                            }),
                        ))
                        .as_pointer(),
                    )
                }),
            ));
            if ((*ujn.borrow()) != (*vn1.borrow())) {
                let rhat: Value<u64> = Rc::new(RefCell::new(0_u64));
                (*qhat.borrow_mut()) = ({
                    v8_bigint_MultiplicativeDigitDivImpl::div(
                        &vn1_divisor.as_pointer(),
                        (*ujn.borrow()),
                        ({
                            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                                &Rc::new(RefCell::new(
                                    ({
                                        v8_bigint_RWDigitsImpl::operator_index(
                                            &U.as_pointer(),
                                            (((*j.borrow()) as u32).wrapping_add((*n.borrow())))
                                                .wrapping_sub(1_u32),
                                        )
                                    }),
                                ))
                                .as_pointer(),
                            )
                        }),
                        (rhat.as_pointer()),
                    )
                });
                let vn2: Value<u64> = Rc::new(RefCell::new(
                    ({
                        v8_bigint_DigitsImpl::operator_index(
                            &BN.as_pointer(),
                            (*n.borrow()).wrapping_sub(2_u32),
                        )
                    }),
                ));
                let ujn2: Value<u64> = Rc::new(RefCell::new(
                    ({
                        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                            &Rc::new(RefCell::new(
                                ({
                                    v8_bigint_RWDigitsImpl::operator_index(
                                        &U.as_pointer(),
                                        (((*j.borrow()) as u32).wrapping_add((*n.borrow())))
                                            .wrapping_sub(2_u32),
                                    )
                                }),
                            ))
                            .as_pointer(),
                        )
                    }),
                ));
                'loop_: while ({
                    ProductGreaterThan_94(
                        (*qhat.borrow()),
                        (*vn2.borrow()),
                        (*rhat.borrow()),
                        (*ujn2.borrow()),
                    )
                }) {
                    (*qhat.borrow_mut()).postfix_dec();
                    let prev_rhat: Value<u64> = Rc::new(RefCell::new((*rhat.borrow())));
                    {
                        let rhs_0 = (*rhat.borrow()).wrapping_add((*vn1.borrow()));
                        (*rhat.borrow_mut()) = rhs_0
                    };
                    if ((*rhat.borrow()) < (*prev_rhat.borrow())) {
                        break;
                    }
                }
            }
            if ((*qhat.borrow()) != 0_u64) {
                ({
                    MultiplySingle_57(
                        (*qhatv.borrow()).clone(),
                        (*BN.borrow()).clone(),
                        (*qhat.borrow()),
                    )
                });
                ({ v8_bigint_ProcessorImplImpl::AddWorkEstimate(self, ((*n.borrow()) as u64)) });
                let c: Value<u64> = Rc::new(RefCell::new(
                    ({
                        InplaceSubAndReturnBorrow_54(
                            ({
                                v8_bigint_RWDigitsImpl::operator_add(
                                    &U.as_pointer(),
                                    ((*j.borrow()) as u32),
                                )
                            }),
                            (*qhatv.borrow()).clone(),
                        )
                    }),
                ));
                if ((*c.borrow()) != 0_u64) {
                    (*c.borrow_mut()) = ({
                        InplaceAddAndReturnCarry_53(
                            ({
                                v8_bigint_RWDigitsImpl::operator_add(
                                    &U.as_pointer(),
                                    ((*j.borrow()) as u32),
                                )
                            }),
                            (*BN.borrow()).clone(),
                        )
                    });
                    ({
                        let _digit: u64 = ({
                            v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_digit_t(
                                &Rc::new(RefCell::new(
                                    ({
                                        v8_bigint_RWDigitsImpl::operator_index(
                                            &U.as_pointer(),
                                            ((*j.borrow()) as u32).wrapping_add((*n.borrow())),
                                        )
                                    }),
                                ))
                                .as_pointer(),
                            )
                        })
                        .wrapping_add((*c.borrow()));
                        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                            &Rc::new(RefCell::new(
                                ({
                                    v8_bigint_RWDigitsImpl::operator_index(
                                        &U.as_pointer(),
                                        ((*j.borrow()) as u32).wrapping_add((*n.borrow())),
                                    )
                                }),
                            ))
                            .as_pointer(),
                            _digit,
                        )
                    });
                    (*qhat.borrow_mut()).postfix_dec();
                }
            }
            if (({ v8_bigint_DigitsImpl::len(&Q) }) != 0_u32) {
                if {
                    let _lhs = ((*j.borrow()) as u32);
                    _lhs >= ({ v8_bigint_DigitsImpl::len(&Q) })
                } {
                    (&(0));
                } else {
                    ({
                        v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                            &Rc::new(RefCell::new(
                                ({
                                    let _i: u32 = ((*j.borrow()) as u32);
                                    v8_bigint_RWDigitsImpl::operator_index(&Q, _i)
                                }),
                            ))
                            .as_pointer(),
                            (*qhat.borrow()),
                        )
                    });
                }
            }
            (*j.borrow_mut()).postfix_dec();
        }
        if (({ v8_bigint_DigitsImpl::len(&R) }) != 0_u32) {
            ({
                let _Z: v8_bigint_RWDigits = (*R.upgrade().deref()).clone();
                let _X: v8_bigint_Digits = (*U.borrow()).clone();
                let _shift: i32 = ({ v8_bigint_ShiftedDigitsImpl::shift(&BN.as_pointer()) });
                RightShift_92(_Z, _X, _shift)
            });
        }
        let i: Value<u32> = Rc::new(RefCell::new((*m.borrow()).wrapping_add(1_u32)));
        'loop_: while {
            let _lhs = (*i.borrow());
            _lhs < ({ v8_bigint_DigitsImpl::len(&Q) })
        } {
            ({
                v8_bigint_RWDigits_WritableDigitReferenceImpl::operator_assign_u64(
                    &Rc::new(RefCell::new(
                        ({
                            let _i: u32 = (*i.borrow());
                            v8_bigint_RWDigitsImpl::operator_index(&Q, _i)
                        }),
                    ))
                    .as_pointer(),
                    0_u64,
                )
            });
            (*i.borrow_mut()).postfix_inc();
        }
    }
}
pub trait v8_bigint_Processor_DestroyerImpl {
    fn operator_call(&self, proc: Ptr<v8_bigint_Processor>);
}
impl v8_bigint_Processor_DestroyerImpl for Ptr<v8_bigint_Processor_Destroyer> {
    fn operator_call(&self, proc: Ptr<v8_bigint_Processor>) {
        let proc: Value<Ptr<v8_bigint_Processor>> = Rc::new(RefCell::new(proc));
        ({ v8_bigint_ProcessorImpl::Destroy(&(*proc.borrow())) });
    }
}
pub trait v8_bigint_RWDigitsImpl {
    fn operator_add(&self, i: u32) -> v8_bigint_RWDigits;
    fn operator_index(&self, i: u32) -> v8_bigint_RWDigits_WritableDigitReference;
    fn digits(&self) -> Ptr<u64>;
    fn set_len(&self, len: u32);
    fn Clear(&self);
}
impl v8_bigint_RWDigitsImpl for Ptr<v8_bigint_RWDigits> {
    fn operator_add(&self, i: u32) -> v8_bigint_RWDigits {
        let i: Value<u32> = Rc::new(RefCell::new(i));
        (&(0));
        return v8_bigint_RWDigits::v8_bigint_RWDigits1(
            { (*(*(*self).upgrade().deref()).digits_.borrow()).offset((*i.borrow()) as isize) },
            { (*(*(*self).upgrade().deref()).len_.borrow()).wrapping_sub((*i.borrow())) },
        );
    }
    fn operator_index(&self, i: u32) -> v8_bigint_RWDigits_WritableDigitReference {
        let i: Value<u32> = Rc::new(RefCell::new(i));
        (&(0));
        return v8_bigint_RWDigits_WritableDigitReference :: v8_bigint_RWDigits_WritableDigitReference ( {  (*(*(* self ) .upgrade().deref()) . digits_ .borrow()) . offset ( ( (*i.borrow()) ) as isize )   } , )   ;
    }
    fn digits(&self) -> Ptr<u64> {
        return (*(*(*self).upgrade().deref()).digits_.borrow()).clone();
    }
    fn set_len(&self, len: u32) {
        let len: Value<u32> = Rc::new(RefCell::new(len));
        (*(*(*self).upgrade().deref()).len_.borrow_mut()) = (*len.borrow());
    }
    fn Clear(&self) {
        {
            ((*(*(*self).upgrade().deref()).digits_.borrow()).clone() as Ptr<u64>)
                .to_any()
                .memset(
                    (0) as u8,
                    ((*(*(*self).upgrade().deref()).len_.borrow()) as usize)
                        .wrapping_mul((::std::mem::size_of::<u64>() as usize))
                        as usize,
                );
            ((*(*(*self).upgrade().deref()).digits_.borrow()).clone() as Ptr<u64>).to_any()
        };
    }
}
pub trait v8_bigint_RWDigits_WritableDigitReferenceImpl {
    fn operator_assign_u64(&self, digit: u64);
    fn operator_assign_pconstv8_bigint_RWDigits_WritableDigitReference(
        &self,
        src: Ptr<v8_bigint_RWDigits_WritableDigitReference>,
    ) -> Ptr<v8_bigint_RWDigits_WritableDigitReference>;
    fn operator_digit_t(&self) -> u64;
}
impl v8_bigint_RWDigits_WritableDigitReferenceImpl
    for Ptr<v8_bigint_RWDigits_WritableDigitReference>
{
    fn operator_assign_u64(&self, digit: u64) {
        let digit: Value<u64> = Rc::new(RefCell::new(digit));
        {
            ((*(*(*self).upgrade().deref()).ptr_.borrow()).clone() as Ptr<u32>)
                .to_any()
                .memcpy(
                    &((digit.as_pointer()) as Ptr<u64>).to_any(),
                    ::std::mem::size_of::<u64>() as usize,
                );
            ((*(*(*self).upgrade().deref()).ptr_.borrow()).clone() as Ptr<u32>).to_any()
        };
    }
    fn operator_assign_pconstv8_bigint_RWDigits_WritableDigitReference(
        &self,
        src: Ptr<v8_bigint_RWDigits_WritableDigitReference>,
    ) -> Ptr<v8_bigint_RWDigits_WritableDigitReference> {
        {
            ((*(*(*self).upgrade().deref()).ptr_.borrow()).clone() as Ptr<u32>)
                .to_any()
                .memcpy(
                    &((*(*src.upgrade().deref()).ptr_.borrow()).clone() as Ptr<u32>).to_any(),
                    ::std::mem::size_of::<u64>() as usize,
                );
            ((*(*(*self).upgrade().deref()).ptr_.borrow()).clone() as Ptr<u32>).to_any()
        };
        return (*self).clone();
    }
    fn operator_digit_t(&self) -> u64 {
        let result: Value<u64> = <Value<u64>>::default();
        {
            ((result.as_pointer()) as Ptr<u64>).to_any().memcpy(
                &((*(*(*self).upgrade().deref()).ptr_.borrow()).clone() as Ptr<u32>).to_any(),
                ::std::mem::size_of::<u64>() as usize,
            );
            ((result.as_pointer()) as Ptr<u64>).to_any()
        };
        return (*result.borrow());
    }
}
pub trait v8_bigint_ShiftedDigitsImpl {
    fn Reset(&self);
    fn shift(&self) -> i32;
}
impl v8_bigint_ShiftedDigitsImpl for Ptr<v8_bigint_ShiftedDigits> {
    fn Reset(&self) {
        if (*(*(*self).upgrade().deref()).inplace_.borrow())
            && ((*(*(*self).upgrade().deref()).shift_.borrow()) != 0)
        {
            let rw_view: Value<v8_bigint_RWDigits> =
                Rc::new(RefCell::new(v8_bigint_RWDigits::v8_bigint_RWDigits1(
                    { (*(*(*self).upgrade().deref()).digits_.borrow()).clone() },
                    { (*(*(*self).upgrade().deref()).len_.borrow()) },
                )));
            ({
                let _Z: v8_bigint_RWDigits = (*rw_view.borrow()).clone();
                let _X: v8_bigint_Digits = (*rw_view.borrow()).clone();
                RightShift_92(_Z, _X, (*(*(*self).upgrade().deref()).shift_.borrow()))
            });
        }
    }
    fn shift(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).shift_.borrow());
    }
}
pub trait v8_bigint_StorageImpl {
    fn get(&self) -> Ptr<u64>;
    fn operator_assign_pmutv8_bigint_Storage(
        &self,
        _a0: Ptr<v8_bigint_Storage>,
    ) -> Ptr<v8_bigint_Storage>;
}
impl v8_bigint_StorageImpl for Ptr<v8_bigint_Storage> {
    fn get(&self) -> Ptr<u64> {
        return ({ (*(*(*self).upgrade().deref()).ptr_.borrow()).get() });
    }
    fn operator_assign_pmutv8_bigint_Storage(
        &self,
        _a0: Ptr<v8_bigint_Storage>,
    ) -> Ptr<v8_bigint_Storage> {
        let __rhs = (*(*_a0.upgrade().deref()).ptr_.borrow()).clone();
        (*(*(*self).upgrade().deref()).ptr_.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
