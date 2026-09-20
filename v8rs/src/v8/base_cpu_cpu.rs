use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type v8_base_AbortMode = i32;
pub const v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures: v8_base_AbortMode = 0;
pub const v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures: v8_base_AbortMode = 1;
pub const v8_base_AbortMode_kExitIfNoSecurityImpact: v8_base_AbortMode = 2;
pub const v8_base_AbortMode_kImmediateCrash: v8_base_AbortMode = 3;
pub const v8_base_AbortMode_kDefault: v8_base_AbortMode = 4;
thread_local!();
pub fn ControlledCrashesAreHarmless_1() -> bool {
    return ((*g_abort_mode_0.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_0.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn DcheckFailuresAreIgnored_2() -> bool {
    return ((*g_abort_mode_0.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitWithSuccessAndIgnoreDcheckFailures)
        || ((*g_abort_mode_0.with(Value::clone).borrow())
            == v8_base_AbortMode_kExitWithFailureAndIgnoreDcheckFailures);
}
pub fn FatalErrorsWithNoSecurityImpactShouldExit_3() -> bool {
    return ((*g_abort_mode_0.with(Value::clone).borrow())
        == v8_base_AbortMode_kExitIfNoSecurityImpact);
}
thread_local!(
    pub static kReturnAddressStackSlotCount_4: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kPageSizeBits_5: Value<i32> = Rc::new(RefCell::new(18));
);
thread_local!(
    pub static kRegularPageSize_6: Value<i32> = Rc::new(RefCell::new(262144));
);
thread_local!(
    pub static kMinimumOSPageSize_7: Value<i32> = Rc::new(RefCell::new(16384));
);
thread_local!(
    pub static kUnimplementedCodeMessage_8: Value<Ptr<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"unimplemented code"),
    ));
);
thread_local!(
    pub static kUnreachableCodeMessage_9: Value<Ptr<u8>> =
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
    pub static is_enum_10: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static is_enum_11: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_12: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_13: Value<bool> = Rc::new(RefCell::new(true));
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
    pub static value_14: Value<bool> = Rc::new(RefCell::new(false));
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
    pub static value_15: Value<bool> = Rc::new(RefCell::new(false));
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
pub fn make_uint64_16(high: u32, low: u32) -> u64 {
    let high: Value<u32> = Rc::new(RefCell::new(high));
    let low: Value<u32> = Rc::new(RefCell::new(low));
    return (((*high.borrow()) as u64) << 32).wrapping_add(((*low.borrow()) as u64));
}
pub fn RoundDown_17(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    return ((*x.borrow()) & (-(*m.borrow()) as u64));
}
pub fn RoundUp_18(x: u64, m: i64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    let m: Value<i64> = Rc::new(RefCell::new(m));
    (&(0));
    (&(0));
    return ({
        let _x: u64 = ((*x.borrow()).wrapping_add((((*m.borrow()) - 1_i64) as u64)) as u64);
        let _m: i64 = (*m.borrow());
        RoundDown_17(_x, _m)
    });
}
pub fn IsAligned_19(value: u64, alignment: u64) -> bool {
    let value: Value<u64> = Rc::new(RefCell::new(value));
    let alignment: Value<u64> = Rc::new(RefCell::new(alignment));
    return (((*value.borrow()) & ((*alignment.borrow()).wrapping_sub(1_u64))) == 0_u64);
}
thread_local!(
    pub static kArm_20: Value<i32> = Rc::new(RefCell::new(65));
);
thread_local!(
    pub static kNvidia_21: Value<i32> = Rc::new(RefCell::new(78));
);
thread_local!(
    pub static kQualcomm_22: Value<i32> = Rc::new(RefCell::new(81));
);
thread_local!(
    pub static kNvidiaDenver_23: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kArmCortexA5_24: Value<i32> = Rc::new(RefCell::new(3077));
);
thread_local!(
    pub static kArmCortexA7_25: Value<i32> = Rc::new(RefCell::new(3079));
);
thread_local!(
    pub static kArmCortexA8_26: Value<i32> = Rc::new(RefCell::new(3080));
);
thread_local!(
    pub static kArmCortexA9_27: Value<i32> = Rc::new(RefCell::new(3081));
);
thread_local!(
    pub static kArmCortexA12_28: Value<i32> = Rc::new(RefCell::new(3084));
);
thread_local!(
    pub static kArmCortexA15_29: Value<i32> = Rc::new(RefCell::new(3087));
);
thread_local!(
    pub static kNvidiaDenverV10_30: Value<i32> = Rc::new(RefCell::new(2));
);
pub type anon_31 = u32;
pub const anon_31_kPPCPower9: anon_31 = 0;
pub const anon_31_kPPCPower10: anon_31 = 1;
pub const anon_31_kPPCPower11: anon_31 = 2;
thread_local!(
    pub static kUnknownCacheLineSize_32: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static kUnknownNumVirtualAddressBits_33: Value<i32> = Rc::new(RefCell::new(0));
);
pub type v8_base_CPU_RV_MMU_MODE = i32;
pub const v8_base_CPU_RV_MMU_MODE_kRiscvSV39: v8_base_CPU_RV_MMU_MODE = 0;
pub const v8_base_CPU_RV_MMU_MODE_kRiscvSV48: v8_base_CPU_RV_MMU_MODE = 1;
pub const v8_base_CPU_RV_MMU_MODE_kRiscvSV57: v8_base_CPU_RV_MMU_MODE = 2;
thread_local!(
    pub static kUnknownVlen_34: Value<u32> = Rc::new(RefCell::new(0));
);
#[derive()]
pub struct v8_base_CPU {
    vendor_: Value<Box<[u8]>>,
    stepping_: Value<i32>,
    model_: Value<i32>,
    ext_model_: Value<i32>,
    family_: Value<i32>,
    ext_family_: Value<i32>,
    type__: Value<i32>,
    implementer_: Value<i32>,
    architecture_: Value<i32>,
    variant_: Value<i32>,
    part_: Value<i32>,
    icache_line_size_: Value<i32>,
    dcache_line_size_: Value<i32>,
    num_virtual_address_bits_: Value<i32>,
    has_fpu_: Value<bool>,
    has_cmov_: Value<bool>,
    has_sahf_: Value<bool>,
    has_mmx_: Value<bool>,
    has_sse_: Value<bool>,
    has_sse2_: Value<bool>,
    has_sse3_: Value<bool>,
    has_ssse3_: Value<bool>,
    has_sse41_: Value<bool>,
    has_sse42_: Value<bool>,
    is_atom_: Value<bool>,
    has_intel_jcc_erratum_: Value<bool>,
    has_cetss_: Value<bool>,
    has_osxsave_: Value<bool>,
    has_avx_: Value<bool>,
    has_avx2_: Value<bool>,
    has_avx_vnni_: Value<bool>,
    has_avx_vnni_int8_: Value<bool>,
    has_fma3_: Value<bool>,
    has_f16c_: Value<bool>,
    has_bmi1_: Value<bool>,
    has_bmi2_: Value<bool>,
    has_lzcnt_: Value<bool>,
    has_popcnt_: Value<bool>,
    has_apx_f_: Value<bool>,
    has_avx10_1_: Value<bool>,
    has_idiva_: Value<bool>,
    has_neon_: Value<bool>,
    has_thumb2_: Value<bool>,
    has_vfp_: Value<bool>,
    has_vfp3_: Value<bool>,
    has_vfp3_d32_: Value<bool>,
    has_jscvt_: Value<bool>,
    has_dot_prod_: Value<bool>,
    has_lse_: Value<bool>,
    has_mte_: Value<bool>,
    has_sha3_: Value<bool>,
    has_pmull1q_: Value<bool>,
    has_fp16_: Value<bool>,
    has_hbc_: Value<bool>,
    has_cssc_: Value<bool>,
    has_mops_: Value<bool>,
    has_sve_: Value<bool>,
    has_svebitperm_: Value<bool>,
    is_fp64_mode_: Value<bool>,
    has_non_stop_time_stamp_counter_: Value<bool>,
    is_running_in_vm_: Value<bool>,
    has_msa_: Value<bool>,
    riscv_mmu_: Value<v8_base_CPU_RV_MMU_MODE>,
    vlen_: Value<u32>,
    has_rvv_: Value<bool>,
    has_zba_: Value<bool>,
    has_zbb_: Value<bool>,
    has_zbs_: Value<bool>,
    has_zfa_: Value<bool>,
    has_zfh_: Value<bool>,
    has_zvfh_: Value<bool>,
    has_rvc_: Value<bool>,
    has_lsx_: Value<bool>,
    has_lasx_: Value<bool>,
}
impl v8_base_CPU {
    pub fn v8_base_CPU() -> Self {
        let __this: Value<v8_base_CPU> = Rc::new(RefCell::new(Self {
            vendor_: Rc::new(RefCell::new(Box::new([
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
                <u8>::default(),
            ]))),
            stepping_: Rc::new(RefCell::new(0)),
            model_: Rc::new(RefCell::new(0)),
            ext_model_: Rc::new(RefCell::new(0)),
            family_: Rc::new(RefCell::new(0)),
            ext_family_: Rc::new(RefCell::new(0)),
            type__: Rc::new(RefCell::new(0)),
            implementer_: Rc::new(RefCell::new(0)),
            architecture_: Rc::new(RefCell::new(0)),
            variant_: Rc::new(RefCell::new(-1_i32)),
            part_: Rc::new(RefCell::new(0)),
            icache_line_size_: Rc::new(RefCell::new(0)),
            dcache_line_size_: Rc::new(RefCell::new(0)),
            num_virtual_address_bits_: Rc::new(RefCell::new(0)),
            has_fpu_: Rc::new(RefCell::new(false)),
            has_cmov_: Rc::new(RefCell::new(false)),
            has_sahf_: Rc::new(RefCell::new(false)),
            has_mmx_: Rc::new(RefCell::new(false)),
            has_sse_: Rc::new(RefCell::new(false)),
            has_sse2_: Rc::new(RefCell::new(false)),
            has_sse3_: Rc::new(RefCell::new(false)),
            has_ssse3_: Rc::new(RefCell::new(false)),
            has_sse41_: Rc::new(RefCell::new(false)),
            has_sse42_: Rc::new(RefCell::new(false)),
            is_atom_: Rc::new(RefCell::new(false)),
            has_intel_jcc_erratum_: Rc::new(RefCell::new(false)),
            has_cetss_: Rc::new(RefCell::new(false)),
            has_osxsave_: Rc::new(RefCell::new(false)),
            has_avx_: Rc::new(RefCell::new(false)),
            has_avx2_: Rc::new(RefCell::new(false)),
            has_avx_vnni_: Rc::new(RefCell::new(false)),
            has_avx_vnni_int8_: Rc::new(RefCell::new(false)),
            has_fma3_: Rc::new(RefCell::new(false)),
            has_f16c_: Rc::new(RefCell::new(false)),
            has_bmi1_: Rc::new(RefCell::new(false)),
            has_bmi2_: Rc::new(RefCell::new(false)),
            has_lzcnt_: Rc::new(RefCell::new(false)),
            has_popcnt_: Rc::new(RefCell::new(false)),
            has_apx_f_: Rc::new(RefCell::new(false)),
            has_avx10_1_: Rc::new(RefCell::new(false)),
            has_idiva_: Rc::new(RefCell::new(false)),
            has_neon_: Rc::new(RefCell::new(false)),
            has_thumb2_: Rc::new(RefCell::new(false)),
            has_vfp_: Rc::new(RefCell::new(false)),
            has_vfp3_: Rc::new(RefCell::new(false)),
            has_vfp3_d32_: Rc::new(RefCell::new(false)),
            has_jscvt_: Rc::new(RefCell::new(false)),
            has_dot_prod_: Rc::new(RefCell::new(false)),
            has_lse_: Rc::new(RefCell::new(false)),
            has_mte_: Rc::new(RefCell::new(false)),
            has_sha3_: Rc::new(RefCell::new(false)),
            has_pmull1q_: Rc::new(RefCell::new(false)),
            has_fp16_: Rc::new(RefCell::new(false)),
            has_hbc_: Rc::new(RefCell::new(false)),
            has_cssc_: Rc::new(RefCell::new(false)),
            has_mops_: Rc::new(RefCell::new(false)),
            has_sve_: Rc::new(RefCell::new(false)),
            has_svebitperm_: Rc::new(RefCell::new(false)),
            is_fp64_mode_: Rc::new(RefCell::new(false)),
            has_non_stop_time_stamp_counter_: Rc::new(RefCell::new(false)),
            is_running_in_vm_: Rc::new(RefCell::new(false)),
            has_msa_: Rc::new(RefCell::new(false)),
            riscv_mmu_: Rc::new(RefCell::new(v8_base_CPU_RV_MMU_MODE_kRiscvSV48)),
            vlen_: Rc::new(RefCell::new(0)),
            has_rvv_: Rc::new(RefCell::new(false)),
            has_zba_: Rc::new(RefCell::new(false)),
            has_zbb_: Rc::new(RefCell::new(false)),
            has_zbs_: Rc::new(RefCell::new(false)),
            has_zfa_: Rc::new(RefCell::new(false)),
            has_zfh_: Rc::new(RefCell::new(false)),
            has_zvfh_: Rc::new(RefCell::new(false)),
            has_rvc_: Rc::new(RefCell::new(false)),
            has_lsx_: Rc::new(RefCell::new(false)),
            has_lasx_: Rc::new(RefCell::new(false)),
        }));
        let this: Ptr<v8_base_CPU> = __this.as_pointer();
        {
            (((*this.upgrade().deref()).vendor_.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .memcpy(
                    &Ptr::from_string_literal(b"Unknown").to_any(),
                    8_usize as usize,
                );
            (((*this.upgrade().deref()).vendor_.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
        };
        ({ v8_base_CPUImpl::DetectFeatures(&this) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for v8_base_CPU {
    fn clone(&self) -> Self {
        let __this: Value<v8_base_CPU> = Rc::new(RefCell::new(Self {
            vendor_: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 13, _>(
                |__i: usize| (*self.vendor_.borrow())[(__i) as usize],
            )))),
            stepping_: Rc::new(RefCell::new((*self.stepping_.borrow()))),
            model_: Rc::new(RefCell::new((*self.model_.borrow()))),
            ext_model_: Rc::new(RefCell::new((*self.ext_model_.borrow()))),
            family_: Rc::new(RefCell::new((*self.family_.borrow()))),
            ext_family_: Rc::new(RefCell::new((*self.ext_family_.borrow()))),
            type__: Rc::new(RefCell::new((*self.type__.borrow()))),
            implementer_: Rc::new(RefCell::new((*self.implementer_.borrow()))),
            architecture_: Rc::new(RefCell::new((*self.architecture_.borrow()))),
            variant_: Rc::new(RefCell::new((*self.variant_.borrow()))),
            part_: Rc::new(RefCell::new((*self.part_.borrow()))),
            icache_line_size_: Rc::new(RefCell::new((*self.icache_line_size_.borrow()))),
            dcache_line_size_: Rc::new(RefCell::new((*self.dcache_line_size_.borrow()))),
            num_virtual_address_bits_: Rc::new(RefCell::new(
                (*self.num_virtual_address_bits_.borrow()),
            )),
            has_fpu_: Rc::new(RefCell::new((*self.has_fpu_.borrow()))),
            has_cmov_: Rc::new(RefCell::new((*self.has_cmov_.borrow()))),
            has_sahf_: Rc::new(RefCell::new((*self.has_sahf_.borrow()))),
            has_mmx_: Rc::new(RefCell::new((*self.has_mmx_.borrow()))),
            has_sse_: Rc::new(RefCell::new((*self.has_sse_.borrow()))),
            has_sse2_: Rc::new(RefCell::new((*self.has_sse2_.borrow()))),
            has_sse3_: Rc::new(RefCell::new((*self.has_sse3_.borrow()))),
            has_ssse3_: Rc::new(RefCell::new((*self.has_ssse3_.borrow()))),
            has_sse41_: Rc::new(RefCell::new((*self.has_sse41_.borrow()))),
            has_sse42_: Rc::new(RefCell::new((*self.has_sse42_.borrow()))),
            is_atom_: Rc::new(RefCell::new((*self.is_atom_.borrow()))),
            has_intel_jcc_erratum_: Rc::new(RefCell::new((*self.has_intel_jcc_erratum_.borrow()))),
            has_cetss_: Rc::new(RefCell::new((*self.has_cetss_.borrow()))),
            has_osxsave_: Rc::new(RefCell::new((*self.has_osxsave_.borrow()))),
            has_avx_: Rc::new(RefCell::new((*self.has_avx_.borrow()))),
            has_avx2_: Rc::new(RefCell::new((*self.has_avx2_.borrow()))),
            has_avx_vnni_: Rc::new(RefCell::new((*self.has_avx_vnni_.borrow()))),
            has_avx_vnni_int8_: Rc::new(RefCell::new((*self.has_avx_vnni_int8_.borrow()))),
            has_fma3_: Rc::new(RefCell::new((*self.has_fma3_.borrow()))),
            has_f16c_: Rc::new(RefCell::new((*self.has_f16c_.borrow()))),
            has_bmi1_: Rc::new(RefCell::new((*self.has_bmi1_.borrow()))),
            has_bmi2_: Rc::new(RefCell::new((*self.has_bmi2_.borrow()))),
            has_lzcnt_: Rc::new(RefCell::new((*self.has_lzcnt_.borrow()))),
            has_popcnt_: Rc::new(RefCell::new((*self.has_popcnt_.borrow()))),
            has_apx_f_: Rc::new(RefCell::new((*self.has_apx_f_.borrow()))),
            has_avx10_1_: Rc::new(RefCell::new((*self.has_avx10_1_.borrow()))),
            has_idiva_: Rc::new(RefCell::new((*self.has_idiva_.borrow()))),
            has_neon_: Rc::new(RefCell::new((*self.has_neon_.borrow()))),
            has_thumb2_: Rc::new(RefCell::new((*self.has_thumb2_.borrow()))),
            has_vfp_: Rc::new(RefCell::new((*self.has_vfp_.borrow()))),
            has_vfp3_: Rc::new(RefCell::new((*self.has_vfp3_.borrow()))),
            has_vfp3_d32_: Rc::new(RefCell::new((*self.has_vfp3_d32_.borrow()))),
            has_jscvt_: Rc::new(RefCell::new((*self.has_jscvt_.borrow()))),
            has_dot_prod_: Rc::new(RefCell::new((*self.has_dot_prod_.borrow()))),
            has_lse_: Rc::new(RefCell::new((*self.has_lse_.borrow()))),
            has_mte_: Rc::new(RefCell::new((*self.has_mte_.borrow()))),
            has_sha3_: Rc::new(RefCell::new((*self.has_sha3_.borrow()))),
            has_pmull1q_: Rc::new(RefCell::new((*self.has_pmull1q_.borrow()))),
            has_fp16_: Rc::new(RefCell::new((*self.has_fp16_.borrow()))),
            has_hbc_: Rc::new(RefCell::new((*self.has_hbc_.borrow()))),
            has_cssc_: Rc::new(RefCell::new((*self.has_cssc_.borrow()))),
            has_mops_: Rc::new(RefCell::new((*self.has_mops_.borrow()))),
            has_sve_: Rc::new(RefCell::new((*self.has_sve_.borrow()))),
            has_svebitperm_: Rc::new(RefCell::new((*self.has_svebitperm_.borrow()))),
            is_fp64_mode_: Rc::new(RefCell::new((*self.is_fp64_mode_.borrow()))),
            has_non_stop_time_stamp_counter_: Rc::new(RefCell::new(
                (*self.has_non_stop_time_stamp_counter_.borrow()),
            )),
            is_running_in_vm_: Rc::new(RefCell::new((*self.is_running_in_vm_.borrow()))),
            has_msa_: Rc::new(RefCell::new((*self.has_msa_.borrow()))),
            riscv_mmu_: Rc::new(RefCell::new((*self.riscv_mmu_.borrow()))),
            vlen_: Rc::new(RefCell::new((*self.vlen_.borrow()))),
            has_rvv_: Rc::new(RefCell::new((*self.has_rvv_.borrow()))),
            has_zba_: Rc::new(RefCell::new((*self.has_zba_.borrow()))),
            has_zbb_: Rc::new(RefCell::new((*self.has_zbb_.borrow()))),
            has_zbs_: Rc::new(RefCell::new((*self.has_zbs_.borrow()))),
            has_zfa_: Rc::new(RefCell::new((*self.has_zfa_.borrow()))),
            has_zfh_: Rc::new(RefCell::new((*self.has_zfh_.borrow()))),
            has_zvfh_: Rc::new(RefCell::new((*self.has_zvfh_.borrow()))),
            has_rvc_: Rc::new(RefCell::new((*self.has_rvc_.borrow()))),
            has_lsx_: Rc::new(RefCell::new((*self.has_lsx_.borrow()))),
            has_lasx_: Rc::new(RefCell::new((*self.has_lasx_.borrow()))),
        }));
        let this: Ptr<v8_base_CPU> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for v8_base_CPU {
    fn default() -> Self {
        { v8_base_CPU::v8_base_CPU() }
    }
}
impl ByteRepr for v8_base_CPU {
    fn byte_size() -> usize {
        136
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.vendor_.borrow()).to_bytes(&mut buf[0..13]);
        (*self.stepping_.borrow()).to_bytes(&mut buf[16..20]);
        (*self.model_.borrow()).to_bytes(&mut buf[20..24]);
        (*self.ext_model_.borrow()).to_bytes(&mut buf[24..28]);
        (*self.family_.borrow()).to_bytes(&mut buf[28..32]);
        (*self.ext_family_.borrow()).to_bytes(&mut buf[32..36]);
        (*self.type__.borrow()).to_bytes(&mut buf[36..40]);
        (*self.implementer_.borrow()).to_bytes(&mut buf[40..44]);
        (*self.architecture_.borrow()).to_bytes(&mut buf[44..48]);
        (*self.variant_.borrow()).to_bytes(&mut buf[48..52]);
        (*self.part_.borrow()).to_bytes(&mut buf[52..56]);
        (*self.icache_line_size_.borrow()).to_bytes(&mut buf[56..60]);
        (*self.dcache_line_size_.borrow()).to_bytes(&mut buf[60..64]);
        (*self.num_virtual_address_bits_.borrow()).to_bytes(&mut buf[64..68]);
        (*self.has_fpu_.borrow()).to_bytes(&mut buf[68..69]);
        (*self.has_cmov_.borrow()).to_bytes(&mut buf[69..70]);
        (*self.has_sahf_.borrow()).to_bytes(&mut buf[70..71]);
        (*self.has_mmx_.borrow()).to_bytes(&mut buf[71..72]);
        (*self.has_sse_.borrow()).to_bytes(&mut buf[72..73]);
        (*self.has_sse2_.borrow()).to_bytes(&mut buf[73..74]);
        (*self.has_sse3_.borrow()).to_bytes(&mut buf[74..75]);
        (*self.has_ssse3_.borrow()).to_bytes(&mut buf[75..76]);
        (*self.has_sse41_.borrow()).to_bytes(&mut buf[76..77]);
        (*self.has_sse42_.borrow()).to_bytes(&mut buf[77..78]);
        (*self.is_atom_.borrow()).to_bytes(&mut buf[78..79]);
        (*self.has_intel_jcc_erratum_.borrow()).to_bytes(&mut buf[79..80]);
        (*self.has_cetss_.borrow()).to_bytes(&mut buf[80..81]);
        (*self.has_osxsave_.borrow()).to_bytes(&mut buf[81..82]);
        (*self.has_avx_.borrow()).to_bytes(&mut buf[82..83]);
        (*self.has_avx2_.borrow()).to_bytes(&mut buf[83..84]);
        (*self.has_avx_vnni_.borrow()).to_bytes(&mut buf[84..85]);
        (*self.has_avx_vnni_int8_.borrow()).to_bytes(&mut buf[85..86]);
        (*self.has_fma3_.borrow()).to_bytes(&mut buf[86..87]);
        (*self.has_f16c_.borrow()).to_bytes(&mut buf[87..88]);
        (*self.has_bmi1_.borrow()).to_bytes(&mut buf[88..89]);
        (*self.has_bmi2_.borrow()).to_bytes(&mut buf[89..90]);
        (*self.has_lzcnt_.borrow()).to_bytes(&mut buf[90..91]);
        (*self.has_popcnt_.borrow()).to_bytes(&mut buf[91..92]);
        (*self.has_apx_f_.borrow()).to_bytes(&mut buf[92..93]);
        (*self.has_avx10_1_.borrow()).to_bytes(&mut buf[93..94]);
        (*self.has_idiva_.borrow()).to_bytes(&mut buf[94..95]);
        (*self.has_neon_.borrow()).to_bytes(&mut buf[95..96]);
        (*self.has_thumb2_.borrow()).to_bytes(&mut buf[96..97]);
        (*self.has_vfp_.borrow()).to_bytes(&mut buf[97..98]);
        (*self.has_vfp3_.borrow()).to_bytes(&mut buf[98..99]);
        (*self.has_vfp3_d32_.borrow()).to_bytes(&mut buf[99..100]);
        (*self.has_jscvt_.borrow()).to_bytes(&mut buf[100..101]);
        (*self.has_dot_prod_.borrow()).to_bytes(&mut buf[101..102]);
        (*self.has_lse_.borrow()).to_bytes(&mut buf[102..103]);
        (*self.has_mte_.borrow()).to_bytes(&mut buf[103..104]);
        (*self.has_sha3_.borrow()).to_bytes(&mut buf[104..105]);
        (*self.has_pmull1q_.borrow()).to_bytes(&mut buf[105..106]);
        (*self.has_fp16_.borrow()).to_bytes(&mut buf[106..107]);
        (*self.has_hbc_.borrow()).to_bytes(&mut buf[107..108]);
        (*self.has_cssc_.borrow()).to_bytes(&mut buf[108..109]);
        (*self.has_mops_.borrow()).to_bytes(&mut buf[109..110]);
        (*self.has_sve_.borrow()).to_bytes(&mut buf[110..111]);
        (*self.has_svebitperm_.borrow()).to_bytes(&mut buf[111..112]);
        (*self.is_fp64_mode_.borrow()).to_bytes(&mut buf[112..113]);
        (*self.has_non_stop_time_stamp_counter_.borrow()).to_bytes(&mut buf[113..114]);
        (*self.is_running_in_vm_.borrow()).to_bytes(&mut buf[114..115]);
        (*self.has_msa_.borrow()).to_bytes(&mut buf[115..116]);
        (*self.riscv_mmu_.borrow()).to_bytes(&mut buf[116..120]);
        (*self.vlen_.borrow()).to_bytes(&mut buf[120..124]);
        (*self.has_rvv_.borrow()).to_bytes(&mut buf[124..125]);
        (*self.has_zba_.borrow()).to_bytes(&mut buf[125..126]);
        (*self.has_zbb_.borrow()).to_bytes(&mut buf[126..127]);
        (*self.has_zbs_.borrow()).to_bytes(&mut buf[127..128]);
        (*self.has_zfa_.borrow()).to_bytes(&mut buf[128..129]);
        (*self.has_zfh_.borrow()).to_bytes(&mut buf[129..130]);
        (*self.has_zvfh_.borrow()).to_bytes(&mut buf[130..131]);
        (*self.has_rvc_.borrow()).to_bytes(&mut buf[131..132]);
        (*self.has_lsx_.borrow()).to_bytes(&mut buf[132..133]);
        (*self.has_lasx_.borrow()).to_bytes(&mut buf[133..134]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            vendor_: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[0..13]))),
            stepping_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[16..20]))),
            model_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[20..24]))),
            ext_model_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[24..28]))),
            family_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[28..32]))),
            ext_family_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[32..36]))),
            type__: Rc::new(RefCell::new(<i32>::from_bytes(&buf[36..40]))),
            implementer_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[40..44]))),
            architecture_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[44..48]))),
            variant_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[48..52]))),
            part_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[52..56]))),
            icache_line_size_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[56..60]))),
            dcache_line_size_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[60..64]))),
            num_virtual_address_bits_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[64..68]))),
            has_fpu_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[68..69]))),
            has_cmov_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[69..70]))),
            has_sahf_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[70..71]))),
            has_mmx_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[71..72]))),
            has_sse_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[72..73]))),
            has_sse2_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[73..74]))),
            has_sse3_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[74..75]))),
            has_ssse3_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[75..76]))),
            has_sse41_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[76..77]))),
            has_sse42_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[77..78]))),
            is_atom_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[78..79]))),
            has_intel_jcc_erratum_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[79..80]))),
            has_cetss_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[80..81]))),
            has_osxsave_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[81..82]))),
            has_avx_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[82..83]))),
            has_avx2_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[83..84]))),
            has_avx_vnni_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[84..85]))),
            has_avx_vnni_int8_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[85..86]))),
            has_fma3_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[86..87]))),
            has_f16c_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[87..88]))),
            has_bmi1_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[88..89]))),
            has_bmi2_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[89..90]))),
            has_lzcnt_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[90..91]))),
            has_popcnt_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[91..92]))),
            has_apx_f_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[92..93]))),
            has_avx10_1_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[93..94]))),
            has_idiva_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[94..95]))),
            has_neon_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[95..96]))),
            has_thumb2_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[96..97]))),
            has_vfp_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[97..98]))),
            has_vfp3_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[98..99]))),
            has_vfp3_d32_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[99..100]))),
            has_jscvt_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[100..101]))),
            has_dot_prod_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[101..102]))),
            has_lse_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[102..103]))),
            has_mte_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[103..104]))),
            has_sha3_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[104..105]))),
            has_pmull1q_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[105..106]))),
            has_fp16_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[106..107]))),
            has_hbc_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[107..108]))),
            has_cssc_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[108..109]))),
            has_mops_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[109..110]))),
            has_sve_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[110..111]))),
            has_svebitperm_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[111..112]))),
            is_fp64_mode_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[112..113]))),
            has_non_stop_time_stamp_counter_: Rc::new(RefCell::new(<bool>::from_bytes(
                &buf[113..114],
            ))),
            is_running_in_vm_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[114..115]))),
            has_msa_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[115..116]))),
            riscv_mmu_: Rc::new(RefCell::new(<v8_base_CPU_RV_MMU_MODE>::from_bytes(
                &buf[116..120],
            ))),
            vlen_: Rc::new(RefCell::new(<u32>::from_bytes(&buf[120..124]))),
            has_rvv_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[124..125]))),
            has_zba_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[125..126]))),
            has_zbb_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[126..127]))),
            has_zbs_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[127..128]))),
            has_zfa_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[128..129]))),
            has_zfh_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[129..130]))),
            has_zvfh_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[130..131]))),
            has_rvc_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[131..132]))),
            has_lsx_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[132..133]))),
            has_lasx_: Rc::new(RefCell::new(<bool>::from_bytes(&buf[133..134]))),
        }
    }
}
pub fn Fopen_35(filename: Ptr<u8>, mode: Ptr<u8>) -> Ptr<CFile> {
    let filename: Value<Ptr<u8>> = Rc::new(RefCell::new(filename));
    let mode: Value<Ptr<u8>> = Rc::new(RefCell::new(mode));
    return match CFile::open(
        &(*filename.borrow()).to_rust_string(),
        &(*mode.borrow()).to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
}
pub fn Fclose_36(stream: Ptr<CFile>) -> i32 {
    let stream: Value<Ptr<CFile>> = Rc::new(RefCell::new(stream));
    return {
        let __r = (*stream.borrow()).with(|__f| __f.close());
        (*stream.borrow()).delete();
        __r
    };
}
impl v8_base_CPU {
    pub fn GetInstance() -> Ptr<v8_base_CPU> {
        thread_local!(
            static cpu_37: Value<v8_base_CPU> = Rc::new(RefCell::new(v8_base_CPU::v8_base_CPU()));
        );
        return cpu_37.with(Value::clone).as_pointer();
    }
}
pub trait v8_base_CPUImpl {
    fn vendor(&self) -> Ptr<u8>;
    fn stepping(&self) -> i32;
    fn model(&self) -> i32;
    fn ext_model(&self) -> i32;
    fn family(&self) -> i32;
    fn ext_family(&self) -> i32;
    fn type_(&self) -> i32;
    fn implementer(&self) -> i32;
    fn architecture(&self) -> i32;
    fn variant(&self) -> i32;
    fn part(&self) -> i32;
    fn has_fpu(&self) -> bool;
    fn icache_line_size(&self) -> i32;
    fn dcache_line_size(&self) -> i32;
    fn has_cmov(&self) -> bool;
    fn has_sahf(&self) -> bool;
    fn has_mmx(&self) -> bool;
    fn has_sse(&self) -> bool;
    fn has_sse2(&self) -> bool;
    fn has_sse3(&self) -> bool;
    fn has_ssse3(&self) -> bool;
    fn has_sse41(&self) -> bool;
    fn has_sse42(&self) -> bool;
    fn has_osxsave(&self) -> bool;
    fn has_avx(&self) -> bool;
    fn has_avx2(&self) -> bool;
    fn has_avx_vnni(&self) -> bool;
    fn has_avx_vnni_int8(&self) -> bool;
    fn has_fma3(&self) -> bool;
    fn has_f16c(&self) -> bool;
    fn has_bmi1(&self) -> bool;
    fn has_bmi2(&self) -> bool;
    fn has_lzcnt(&self) -> bool;
    fn has_popcnt(&self) -> bool;
    fn has_apx_f(&self) -> bool;
    fn has_avx10_1(&self) -> bool;
    fn is_atom(&self) -> bool;
    fn has_intel_jcc_erratum(&self) -> bool;
    fn has_cetss(&self) -> bool;
    fn has_non_stop_time_stamp_counter(&self) -> bool;
    fn is_running_in_vm(&self) -> bool;
    fn exposes_num_virtual_address_bits(&self) -> bool;
    fn num_virtual_address_bits(&self) -> i32;
    fn has_idiva(&self) -> bool;
    fn has_neon(&self) -> bool;
    fn has_thumb2(&self) -> bool;
    fn has_vfp(&self) -> bool;
    fn has_vfp3(&self) -> bool;
    fn has_vfp3_d32(&self) -> bool;
    fn has_jscvt(&self) -> bool;
    fn has_dot_prod(&self) -> bool;
    fn has_lse(&self) -> bool;
    fn has_mte(&self) -> bool;
    fn has_sha3(&self) -> bool;
    fn has_pmull1q(&self) -> bool;
    fn has_fp16(&self) -> bool;
    fn has_hbc(&self) -> bool;
    fn has_cssc(&self) -> bool;
    fn has_mops(&self) -> bool;
    fn has_sve(&self) -> bool;
    fn has_svebitperm(&self) -> bool;
    fn is_fp64_mode(&self) -> bool;
    fn has_msa(&self) -> bool;
    fn vlen(&self) -> u32;
    fn has_rvv(&self) -> bool;
    fn has_zba(&self) -> bool;
    fn has_zbb(&self) -> bool;
    fn has_zbs(&self) -> bool;
    fn has_zfa(&self) -> bool;
    fn has_rvc(&self) -> bool;
    fn has_zfh(&self) -> bool;
    fn has_zvfh(&self) -> bool;
    fn riscv_mmu(&self) -> v8_base_CPU_RV_MMU_MODE;
    fn has_lsx(&self) -> bool;
    fn has_lasx(&self) -> bool;
}
impl v8_base_CPUImpl for Ptr<v8_base_CPU> {
    fn vendor(&self) -> Ptr<u8> {
        return ((*(*self).upgrade().deref()).vendor_.as_pointer() as Ptr<u8>);
    }
    fn stepping(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).stepping_.borrow());
    }
    fn model(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).model_.borrow());
    }
    fn ext_model(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).ext_model_.borrow());
    }
    fn family(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).family_.borrow());
    }
    fn ext_family(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).ext_family_.borrow());
    }
    fn type_(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).type__.borrow());
    }
    fn implementer(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).implementer_.borrow());
    }
    fn architecture(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).architecture_.borrow());
    }
    fn variant(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).variant_.borrow());
    }
    fn part(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).part_.borrow());
    }
    fn has_fpu(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_fpu_.borrow());
    }
    fn icache_line_size(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).icache_line_size_.borrow());
    }
    fn dcache_line_size(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).dcache_line_size_.borrow());
    }
    fn has_cmov(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_cmov_.borrow());
    }
    fn has_sahf(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_sahf_.borrow());
    }
    fn has_mmx(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_mmx_.borrow());
    }
    fn has_sse(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_sse_.borrow());
    }
    fn has_sse2(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_sse2_.borrow());
    }
    fn has_sse3(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_sse3_.borrow());
    }
    fn has_ssse3(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_ssse3_.borrow());
    }
    fn has_sse41(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_sse41_.borrow());
    }
    fn has_sse42(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_sse42_.borrow());
    }
    fn has_osxsave(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_osxsave_.borrow());
    }
    fn has_avx(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_avx_.borrow());
    }
    fn has_avx2(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_avx2_.borrow());
    }
    fn has_avx_vnni(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_avx_vnni_.borrow());
    }
    fn has_avx_vnni_int8(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_avx_vnni_int8_.borrow());
    }
    fn has_fma3(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_fma3_.borrow());
    }
    fn has_f16c(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_f16c_.borrow());
    }
    fn has_bmi1(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_bmi1_.borrow());
    }
    fn has_bmi2(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_bmi2_.borrow());
    }
    fn has_lzcnt(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_lzcnt_.borrow());
    }
    fn has_popcnt(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_popcnt_.borrow());
    }
    fn has_apx_f(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_apx_f_.borrow());
    }
    fn has_avx10_1(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_avx10_1_.borrow());
    }
    fn is_atom(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_atom_.borrow());
    }
    fn has_intel_jcc_erratum(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_intel_jcc_erratum_.borrow());
    }
    fn has_cetss(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_cetss_.borrow());
    }
    fn has_non_stop_time_stamp_counter(&self) -> bool {
        return (*(*(*self).upgrade().deref())
            .has_non_stop_time_stamp_counter_
            .borrow());
    }
    fn is_running_in_vm(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_running_in_vm_.borrow());
    }
    fn exposes_num_virtual_address_bits(&self) -> bool {
        return ((*(*(*self).upgrade().deref())
            .num_virtual_address_bits_
            .borrow())
            != 0);
    }
    fn num_virtual_address_bits(&self) -> i32 {
        (&(0));
        return (*(*(*self).upgrade().deref())
            .num_virtual_address_bits_
            .borrow());
    }
    fn has_idiva(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_idiva_.borrow());
    }
    fn has_neon(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_neon_.borrow());
    }
    fn has_thumb2(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_thumb2_.borrow());
    }
    fn has_vfp(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_vfp_.borrow());
    }
    fn has_vfp3(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_vfp3_.borrow());
    }
    fn has_vfp3_d32(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_vfp3_d32_.borrow());
    }
    fn has_jscvt(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_jscvt_.borrow());
    }
    fn has_dot_prod(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_dot_prod_.borrow());
    }
    fn has_lse(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_lse_.borrow());
    }
    fn has_mte(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_mte_.borrow());
    }
    fn has_sha3(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_sha3_.borrow());
    }
    fn has_pmull1q(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_pmull1q_.borrow());
    }
    fn has_fp16(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_fp16_.borrow());
    }
    fn has_hbc(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_hbc_.borrow());
    }
    fn has_cssc(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_cssc_.borrow());
    }
    fn has_mops(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_mops_.borrow());
    }
    fn has_sve(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_sve_.borrow());
    }
    fn has_svebitperm(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_svebitperm_.borrow());
    }
    fn is_fp64_mode(&self) -> bool {
        return (*(*(*self).upgrade().deref()).is_fp64_mode_.borrow());
    }
    fn has_msa(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_msa_.borrow());
    }
    fn vlen(&self) -> u32 {
        return (*(*(*self).upgrade().deref()).vlen_.borrow());
    }
    fn has_rvv(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_rvv_.borrow());
    }
    fn has_zba(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_zba_.borrow());
    }
    fn has_zbb(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_zbb_.borrow());
    }
    fn has_zbs(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_zbs_.borrow());
    }
    fn has_zfa(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_zfa_.borrow());
    }
    fn has_rvc(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_rvc_.borrow());
    }
    fn has_zfh(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_zfh_.borrow());
    }
    fn has_zvfh(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_zvfh_.borrow());
    }
    fn riscv_mmu(&self) -> v8_base_CPU_RV_MMU_MODE {
        return (*(*(*self).upgrade().deref()).riscv_mmu_.borrow());
    }
    fn has_lsx(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_lsx_.borrow());
    }
    fn has_lasx(&self) -> bool {
        return (*(*(*self).upgrade().deref()).has_lasx_.borrow());
    }
}
