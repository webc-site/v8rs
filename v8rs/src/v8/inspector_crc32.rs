use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_add_1(a: Ptr<u8>, b: Ptr<v8_inspector_String16>) -> v8_inspector_String16 {
    let a: Value<Ptr<u8>> = Rc::new(RefCell::new(a));
    return ({
        let _other: Ptr<v8_inspector_String16> = (b).clone();
        v8_inspector_String16Impl::operator_add(
            &v8_inspector_String16::v8_inspector_String162({ (*a.borrow()).clone() }),
            _other,
        )
    });
}
#[derive(Default)]
pub struct std_hash_v8_inspector_String16_ {}
impl Clone for std_hash_v8_inspector_String16_ {
    fn clone(&self) -> Self {
        let __this: Value<std_hash_v8_inspector_String16_> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<std_hash_v8_inspector_String16_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for std_hash_v8_inspector_String16_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub type v8_base_AbortMode = i32;
pub const v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures: v8_base_AbortMode = 0;
pub const v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures: v8_base_AbortMode = 1;
pub const v8_base_AbortMode_kExitIfNoSecurityImpact: v8_base_AbortMode = 2;
pub const v8_base_AbortMode_kImmediateCrash: v8_base_AbortMode = 3;
pub const v8_base_AbortMode_kDefault: v8_base_AbortMode = 4;
thread_local!();
pub fn ControlledCrashesAreHarmless_3() -> bool {
    return ((*g_abort_mode_2.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_2.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn DcheckFailuresAreIgnored_4() -> bool {
    return ((*g_abort_mode_2.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_2.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn FatalErrorsWithNoSecurityImpactShouldExit_5() -> bool {
    return ((*g_abort_mode_2.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitIfNoSecurityImpact);
}
thread_local!(
    pub static kReturnAddressStackSlotCount_6: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kPageSizeBits_7: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_8: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
    pub static kMinimumOSPageSize_9: Value<i32> = Rc::new(RefCell::new(16384));
);
thread_local!(
    pub static kUnimplementedCodeMessage_10: Value<Ptr<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"unimplemented code"),
    ));
);
thread_local!(
    pub static kUnreachableCodeMessage_11: Value<Ptr<u8>> =
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
    pub static is_enum_12: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static is_enum_13: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_14: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_15: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static value_16: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_17: Value<bool> = Rc::new(RefCell::new(false));
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
pub fn bit_cast_18(source: Ptr<u32>) -> i32 {
    return ({ bit_cast_19((source).clone()) });
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
thread_local!(
    pub static kCrcTable_24: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        0_u32,
        1996959894_u32,
        3993919788_u32,
        2567524794_u32,
        124634137_u32,
        1886057615_u32,
        3915621685_u32,
        2657392035_u32,
        249268274_u32,
        2044508324_u32,
        3772115230_u32,
        2547177864_u32,
        162941995_u32,
        2125561021_u32,
        3887607047_u32,
        2428444049_u32,
        498536548_u32,
        1789927666_u32,
        4089016648_u32,
        2227061214_u32,
        450548861_u32,
        1843258603_u32,
        4107580753_u32,
        2211677639_u32,
        325883990_u32,
        1684777152_u32,
        4251122042_u32,
        2321926636_u32,
        335633487_u32,
        1661365465_u32,
        4195302755_u32,
        2366115317_u32,
        997073096_u32,
        1281953886_u32,
        3579855332_u32,
        2724688242_u32,
        1006888145_u32,
        1258607687_u32,
        3524101629_u32,
        2768942443_u32,
        901097722_u32,
        1119000684_u32,
        3686517206_u32,
        2898065728_u32,
        853044451_u32,
        1172266101_u32,
        3705015759_u32,
        2882616665_u32,
        651767980_u32,
        1373503546_u32,
        3369554304_u32,
        3218104598_u32,
        565507253_u32,
        1454621731_u32,
        3485111705_u32,
        3099436303_u32,
        671266974_u32,
        1594198024_u32,
        3322730930_u32,
        2970347812_u32,
        795835527_u32,
        1483230225_u32,
        3244367275_u32,
        3060149565_u32,
        1994146192_u32,
        31158534_u32,
        2563907772_u32,
        4023717930_u32,
        1907459465_u32,
        112637215_u32,
        2680153253_u32,
        3904427059_u32,
        2013776290_u32,
        251722036_u32,
        2517215374_u32,
        3775830040_u32,
        2137656763_u32,
        141376813_u32,
        2439277719_u32,
        3865271297_u32,
        1802195444_u32,
        476864866_u32,
        2238001368_u32,
        4066508878_u32,
        1812370925_u32,
        453092731_u32,
        2181625025_u32,
        4111451223_u32,
        1706088902_u32,
        314042704_u32,
        2344532202_u32,
        4240017532_u32,
        1658658271_u32,
        366619977_u32,
        2362670323_u32,
        4224994405_u32,
        1303535960_u32,
        984961486_u32,
        2747007092_u32,
        3569037538_u32,
        1256170817_u32,
        1037604311_u32,
        2765210733_u32,
        3554079995_u32,
        1131014506_u32,
        879679996_u32,
        2909243462_u32,
        3663771856_u32,
        1141124467_u32,
        855842277_u32,
        2852801631_u32,
        3708648649_u32,
        1342533948_u32,
        654459306_u32,
        3188396048_u32,
        3373015174_u32,
        1466479909_u32,
        544179635_u32,
        3110523913_u32,
        3462522015_u32,
        1591671054_u32,
        702138776_u32,
        2966460450_u32,
        3352799412_u32,
        1504918807_u32,
        783551873_u32,
        3082640443_u32,
        3233442989_u32,
        3988292384_u32,
        2596254646_u32,
        62317068_u32,
        1957810842_u32,
        3939845945_u32,
        2647816111_u32,
        81470997_u32,
        1943803523_u32,
        3814918930_u32,
        2489596804_u32,
        225274430_u32,
        2053790376_u32,
        3826175755_u32,
        2466906013_u32,
        167816743_u32,
        2097651377_u32,
        4027552580_u32,
        2265490386_u32,
        503444072_u32,
        1762050814_u32,
        4150417245_u32,
        2154129355_u32,
        426522225_u32,
        1852507879_u32,
        4275313526_u32,
        2312317920_u32,
        282753626_u32,
        1742555852_u32,
        4189708143_u32,
        2394877945_u32,
        397917763_u32,
        1622183637_u32,
        3604390888_u32,
        2714866558_u32,
        953729732_u32,
        1340076626_u32,
        3518719985_u32,
        2797360999_u32,
        1068828381_u32,
        1219638859_u32,
        3624741850_u32,
        2936675148_u32,
        906185462_u32,
        1090812512_u32,
        3747672003_u32,
        2825379669_u32,
        829329135_u32,
        1181335161_u32,
        3412177804_u32,
        3160834842_u32,
        628085408_u32,
        1382605366_u32,
        3423369109_u32,
        3138078467_u32,
        570562233_u32,
        1426400815_u32,
        3317316542_u32,
        2998733608_u32,
        733239954_u32,
        1555261956_u32,
        3268935591_u32,
        3050360625_u32,
        752459403_u32,
        1541320221_u32,
        2607071920_u32,
        3965973030_u32,
        1969922972_u32,
        40735498_u32,
        2617837225_u32,
        3943577151_u32,
        1913087877_u32,
        83908371_u32,
        2512341634_u32,
        3803740692_u32,
        2075208622_u32,
        213261112_u32,
        2463272603_u32,
        3855990285_u32,
        2094854071_u32,
        198958881_u32,
        2262029012_u32,
        4057260610_u32,
        1759359992_u32,
        534414190_u32,
        2176718541_u32,
        4139329115_u32,
        1873836001_u32,
        414664567_u32,
        2282248934_u32,
        4279200368_u32,
        1711684554_u32,
        285281116_u32,
        2405801727_u32,
        4167216745_u32,
        1634467795_u32,
        376229701_u32,
        2685067896_u32,
        3608007406_u32,
        1308918612_u32,
        956543938_u32,
        2808555105_u32,
        3495958263_u32,
        1231636301_u32,
        1047427035_u32,
        2932959818_u32,
        3654703836_u32,
        1088359270_u32,
        936918000_u32,
        2847714899_u32,
        3736837829_u32,
        1202900863_u32,
        817233897_u32,
        3183342108_u32,
        3401237130_u32,
        1404277552_u32,
        615818150_u32,
        3134207493_u32,
        3453421203_u32,
        1423857449_u32,
        601450431_u32,
        3009837614_u32,
        3294710456_u32,
        1567103746_u32,
        711928724_u32,
        3020668471_u32,
        3272380065_u32,
        1510334235_u32,
        755167117_u32,
    ])));
);
pub fn computeCrc32_25(text: Ptr<v8_inspector_String16>) -> i32 {
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ({ v8_inspector_String16Impl::characters16(&text) }).reinterpret_cast::<u8>(),
    ));
    let byteLength: Value<usize> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<u16>() as u64)
            .wrapping_mul((({ v8_inspector_String16Impl::length(&text) }) as u64))
            as usize),
    ));
    let checksum: Value<u32> = Rc::new(RefCell::new(0_u32));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*byteLength.borrow())) {
        let index: Value<u32> = Rc::new(RefCell::new(
            (({
                let _lhs = (*checksum.borrow());
                _lhs ^ (((*bytes.borrow()).offset((*i.borrow()) as isize).read()) as u32)
            }) & 255_u32),
        ));
        let __rhs = (((*checksum.borrow()) >> 8)
            ^ (*kCrcTable_24.with(Value::clone).borrow())[(*index.borrow()) as usize]);
        (*checksum.borrow_mut()) = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    return ({ bit_cast_18(checksum.as_pointer()) });
}
#[derive(Clone, Copy, Default, ByteRepr)]
pub struct v8_inspector_String16;
pub trait std_hash_v8_inspector_String16_Impl {
    fn operator_call(&self, string: Ptr<v8_inspector_String16>) -> usize;
}
impl std_hash_v8_inspector_String16_Impl for Ptr<std_hash_v8_inspector_String16_> {
    fn operator_call(&self, string: Ptr<v8_inspector_String16>) -> usize {
        return ({ v8_inspector_String16Impl::hash(&string) });
    }
}
