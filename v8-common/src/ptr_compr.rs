// 由 tool/src/common.js 从 v8/src/common/ptr-compr.h/.cc/-inl.h 生成; 修订就地改, 复跑覆盖
//
// 修订 (Developer): V8HeapCompressionSchemeImpl 全语义落位 — MainCage/TrustedCage
// base_ 为进程级 AtomicUsize (shared cage, ptr-compr.cc:11-17), ExternalCode
// 4GB 回绕解压 (ptr-compr-inl.h:218-239), ProcessIntermediatePointers 双半字。

use core::{
  marker::PhantomData,
  sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
  error::{Error, Result},
  globals::{K_CLEARED_WEAK_HEAP_OBJECT_LOWER32, K_MIN_EXPECTED_OS_PAGE_SIZE, TaggedT},
  internal::{
    Address, GB, K_HEAP_OBJECT_TAG, K_HEAP_OBJECT_TAG_MASK, K_NULL_ADDRESS,
    K_PTR_COMPR_CAGE_BASE_ALIGNMENT, K_PTR_COMPR_CAGE_RESERVATION_SIZE, K_SMI_TAG, K_SMI_TAG_MASK,
    K_WEAK_HEAP_OBJECT_TAG,
  },
};

/// 原版 kPtrComprCageBaseMask (ptr-compr-inl.h:35)
const K_PTR_COMPR_CAGE_BASE_MASK: usize = !(K_PTR_COMPR_CAGE_BASE_ALIGNMENT - 1);
/// 原版 kMinExpectedOSPageSizeMask (ptr-compr-inl.h:156)
const K_MIN_EXPECTED_OS_PAGE_SIZE_MASK: usize = !(K_MIN_EXPECTED_OS_PAGE_SIZE - 1);

/// 原版 HAS_SMI_TAG — Smi 低位为 0
#[inline]
pub const fn has_smi_tag(v: usize) -> bool {
  v & K_SMI_TAG_MASK == K_SMI_TAG
}

/// 原版 RoundDown<kPtrComprCageBaseAlignment>
#[inline]
const fn round_down_4gb(addr: usize) -> usize {
  addr & K_PTR_COMPR_CAGE_BASE_MASK
}

/// 压缩 cage 存储抽象 (原版模板参数 Cage, ptr-compr.h:22-57)
///
/// 目标配置 shared cage → base_ 为进程级静态 (登记偏差: C++ 普通静态
/// V8_CONSTINIT, Rust AtomicUsize, 读写 relaxed)
pub trait CompressionCage {
  /// 原版 Cage::base_ (ptr-compr.cc:17/26/36)
  fn base_() -> &'static AtomicUsize;
}

/// 原版 PtrComprCageBase (ptr-compr-inl.h:26-29) — cage 基址值类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PtrComprCageBase {
  address: usize,
}

impl PtrComprCageBase {
  #[inline]
  pub const fn new(address: usize) -> Self {
    Self { address }
  }

  #[inline]
  pub const fn address(&self) -> usize {
    self.address
  }

  /// 原版 PtrComprCageBase(isolate) (ptr-compr-inl.h:26-29) —
  /// isolate 依赖 L3 接入点, 此处直接以 scheme base 构造
  #[inline]
  pub fn current() -> Self {
    Self::new(V8HeapCompressionScheme::base())
  }
}

impl From<usize> for PtrComprCageBase {
  #[inline]
  fn from(address: usize) -> Self {
    Self::new(address)
  }
}

// ── cage base 存储 (ptr-compr.cc:17-46) ───────────────────────────────────

static MAIN_CAGE_BASE: AtomicUsize = AtomicUsize::new(K_NULL_ADDRESS);
static TRUSTED_CAGE_BASE: AtomicUsize = AtomicUsize::new(K_NULL_ADDRESS);
static EXTERNAL_CODE_CAGE_BASE: AtomicUsize = AtomicUsize::new(K_NULL_ADDRESS);

/// 原版 MainCage (ptr-compr.h:60-73)
pub struct MainCage;

impl CompressionCage for MainCage {
  #[inline]
  fn base_() -> &'static AtomicUsize {
    &MAIN_CAGE_BASE
  }
}

/// 原版 TrustedCage (ptr-compr.h:79-91, sandbox 开)
pub struct TrustedCage;

impl CompressionCage for TrustedCage {
  #[inline]
  fn base_() -> &'static AtomicUsize {
    &TRUSTED_CAGE_BASE
  }
}

/// 原版 using V8HeapCompressionScheme = V8HeapCompressionSchemeImpl<MainCage>
pub type V8HeapCompressionScheme = V8HeapCompressionSchemeImpl<MainCage>;
/// 原版 using TrustedSpaceCompressionScheme = V8HeapCompressionSchemeImpl<TrustedCage>
pub type TrustedSpaceCompressionScheme = V8HeapCompressionSchemeImpl<TrustedCage>;

/// 原版 V8HeapCompressionSchemeImpl<Cage> (ptr-compr.h:22-57, ptr-compr-inl.h:38-148)
pub struct V8HeapCompressionSchemeImpl<C: CompressionCage> {
  _cage: PhantomData<C>,
}

impl<C: CompressionCage> V8HeapCompressionSchemeImpl<C> {
  /// 原版 GetPtrComprCageBaseAddress(Address) (ptr-compr-inl.h:38-42)
  #[inline]
  pub const fn get_ptr_compr_cage_base_address(on_heap_addr: usize) -> usize {
    round_down_4gb(on_heap_addr)
  }

  /// 原版 GetPtrComprCageBaseAddress(PtrComprCageBase) (ptr-compr-inl.h:46-53)
  #[inline]
  pub const fn get_ptr_compr_cage_base_from_cage(cage_base: PtrComprCageBase) -> usize {
    // 原版 V8_ASSUME((base & kPtrComprCageBaseMask) == base)
    debug_assert!(cage_base.address() & K_PTR_COMPR_CAGE_BASE_MASK == cage_base.address());
    round_down_4gb(cage_base.address())
  }

  /// 原版 InitBase (ptr-compr-inl.h:57-65) —
  /// CHECK_EQ(base, GetPtrComprCageBaseAddress(base)) → Err (登记)
  pub fn init_base(base: usize) -> Result<()> {
    if base != Self::get_ptr_compr_cage_base_address(base) {
      return Err(Error::CageBaseUnaligned { base });
    }
    C::base_().store(base, Ordering::Relaxed);
    Ok(())
  }

  /// 原版 base() (ptr-compr-inl.h:68-82)
  #[inline]
  pub fn base() -> usize {
    let base = C::base_().load(Ordering::Relaxed);
    // 原版 V8_ASSUME((base & kPtrComprCageBaseMask) == base)
    debug_assert!(base & K_PTR_COMPR_CAGE_BASE_MASK == base);
    base
  }

  /// 原版 CompressObject (ptr-compr-inl.h:86-96) — 压缩 = 截断低 32 位
  #[inline]
  pub fn compress_object(tagged: usize) -> TaggedT {
    // 原版 DCHECK_IMPLIES: 非 Smi 且非清除弱引用时必须落在本 cage
    if !has_smi_tag(tagged) && tagged != K_CLEARED_WEAK_HEAP_OBJECT_LOWER32 as usize {
      debug_assert!(tagged & K_PTR_COMPR_CAGE_BASE_MASK == Self::base());
    }
    tagged as TaggedT
  }

  /// 原版 CompressAny (ptr-compr-inl.h:100-103, const)
  #[inline]
  pub const fn compress_any(tagged: usize) -> TaggedT {
    tagged as TaggedT
  }

  /// 原版 DecompressTaggedSigned (ptr-compr-inl.h:107-111) —
  /// 运行时上 32 位不关心, 零扩展
  #[inline]
  pub const fn decompress_tagged_signed(raw_value: TaggedT) -> usize {
    raw_value as usize
  }

  /// 原版 DecompressTagged (ptr-compr-inl.h:115-130) — base | raw
  #[inline]
  pub fn decompress_tagged(raw_value: TaggedT) -> usize {
    let cage_base = Self::base();
    let result = cage_base | raw_value as usize;
    // 原版 V8_ASSUME(static_cast<uint32_t>(result) == raw_value)
    debug_assert!(result as TaggedT == raw_value);
    result
  }

  /// 原版 ProcessIntermediatePointers (ptr-compr-inl.h:135-148) —
  /// 栈上原始值的高低两个半字分别解压后回调
  #[inline]
  pub fn process_intermediate_pointers(raw_value: usize, mut callback: impl FnMut(usize)) {
    let decompressed_low = Self::decompress_tagged(raw_value as TaggedT);
    callback(decompressed_low);
    let decompressed_high =
      Self::decompress_tagged((raw_value >> (TaggedT::BITS as usize)) as TaggedT);
    callback(decompressed_high);
  }
}

// ── SmiCompressionScheme (ptr-compr.h:103-114) ────────────────────────────

/// 只处理 Smi 的压缩方案
pub struct SmiCompressionScheme;

impl SmiCompressionScheme {
  /// 原版 DecompressTaggedSigned — 运行时上 32 位不关心
  #[inline]
  pub const fn decompress_tagged_signed(raw_value: TaggedT) -> usize {
    raw_value as usize
  }

  /// 原版 CompressObject — DCHECK(HAS_SMI_TAG)
  #[inline]
  pub fn compress_object(tagged: usize) -> TaggedT {
    debug_assert!(has_smi_tag(tagged));
    tagged as TaggedT
  }
}

// ── ExternalCodeCompressionScheme (ptr-compr.h:151-198, ptr-compr-inl.h:156-254)
// V8_EXTERNAL_CODE_SPACE 开 → 落位; cage 可跨 4GB, 解压按差值回绕

pub struct ExternalCodeCompressionScheme;

impl ExternalCodeCompressionScheme {
  #[inline]
  fn base_() -> &'static AtomicUsize {
    &EXTERNAL_CODE_CAGE_BASE
  }

  /// 原版 PrepareCageBaseAddress (ptr-compr-inl.h:159-162)
  #[inline]
  pub const fn prepare_cage_base_address(on_heap_addr: usize) -> usize {
    on_heap_addr & K_MIN_EXPECTED_OS_PAGE_SIZE_MASK
  }

  /// 原版 GetPtrComprCageBaseAddress(PtrComprCageBase) (ptr-compr-inl.h:165-172)
  /// 注: Address 重载在原版为 = delete, Rust 不提供 (登记)
  #[inline]
  pub const fn get_ptr_compr_cage_base_from_cage(cage_base: PtrComprCageBase) -> usize {
    debug_assert!(cage_base.address() & K_MIN_EXPECTED_OS_PAGE_SIZE_MASK == cage_base.address());
    cage_base.address() & K_MIN_EXPECTED_OS_PAGE_SIZE_MASK
  }

  /// 原版 InitBase (ptr-compr-inl.h:175-183) —
  /// CHECK_EQ(base, PrepareCageBaseAddress(base)) → Err (登记)
  pub fn init_base(base: usize) -> Result<()> {
    if base != Self::prepare_cage_base_address(base) {
      return Err(Error::CageBaseUnaligned { base });
    }
    Self::base_().store(base, Ordering::Relaxed);
    Ok(())
  }

  /// 原版 base() (ptr-compr-inl.h:186-199)
  #[inline]
  pub fn base() -> usize {
    let base = Self::base_().load(Ordering::Relaxed);
    debug_assert!(base & K_MIN_EXPECTED_OS_PAGE_SIZE_MASK == base);
    base
  }

  /// 原版 CompressObject (ptr-compr-inl.h:202-210) — 截断;
  /// shared cage 分支校验 tagged 落在 [base, base + 4GB)
  #[inline]
  pub fn compress_object(tagged: usize) -> TaggedT {
    if !has_smi_tag(tagged) {
      let base = Self::base();
      debug_assert!(base <= tagged && tagged < base + K_PTR_COMPR_CAGE_RESERVATION);
    }
    tagged as TaggedT
  }

  /// 原版 CompressAny (ptr-compr-inl.h:213-215, const)
  #[inline]
  pub const fn compress_any(tagged: usize) -> TaggedT {
    tagged as TaggedT
  }

  /// 原版 DecompressTagged (ptr-compr-inl.h:218-239) — 4GB 回绕
  #[inline]
  pub fn decompress_tagged(raw_value: TaggedT) -> usize {
    let cage_base = Self::base();
    // diff = raw - (uint32)base; 负值说明解压结果差 4GB, 补回
    let diff = (raw_value as usize).wrapping_sub(cage_base as u32 as usize);
    let diff = if (diff as isize) < 0 {
      // C++ 无符号回绕等价 (mod 2^64)
      diff.wrapping_add(4 * GB)
    } else {
      diff
    };
    // 原版 DCHECK(is_uint32(diff))
    debug_assert!(diff <= u32::MAX as usize);
    let result = cage_base.wrapping_add(diff);
    // 原版 DCHECK_EQ(static_cast<uint32_t>(result), raw_value)
    debug_assert!(result as TaggedT == raw_value);
    result
  }

  /// 原版 ProcessIntermediatePointers (ptr-compr-inl.h:243-254)
  #[inline]
  pub fn process_intermediate_pointers(raw_value: usize, mut callback: impl FnMut(usize)) {
    let decompressed_low = Self::decompress_tagged(raw_value as TaggedT);
    callback(decompressed_low);
    let decompressed_high =
      Self::decompress_tagged((raw_value >> (TaggedT::BITS as usize)) as TaggedT);
    callback(decompressed_high);
  }
}

/// 原版 kPtrComprCageReservationSize (internal 模块承载, 此处类型别名)
const K_PTR_COMPR_CAGE_RESERVATION: usize = K_PTR_COMPR_CAGE_RESERVATION_SIZE;

// ── 杂项 (ptr-compr-inl.h:262-271) ────────────────────────────────────────

/// 原版 GetPtrComprCageBaseFromOnHeapAddress
#[inline]
pub fn get_ptr_compr_cage_base_from_on_heap_address(address: usize) -> PtrComprCageBase {
  PtrComprCageBase::new(V8HeapCompressionScheme::get_ptr_compr_cage_base_address(
    address,
  ))
}

/// 原版 GetPtrComprCageBase() — 载入主 cage 基址
#[inline]
pub fn get_ptr_compr_cage_base() -> PtrComprCageBase {
  PtrComprCageBase::new(V8HeapCompressionScheme::base())
}

/// 原版 GetPtrComprCageBase(Tagged<HeapObject>) — L3 对象层接入前以裸地址形式提供
#[inline]
pub fn get_ptr_compr_cage_base_for_object(object_ptr: usize) -> PtrComprCageBase {
  get_ptr_compr_cage_base_from_on_heap_address(object_ptr)
}

// ── 未对齐读写 (ptr-compr.h:204-238) ──────────────────────────────────────

/// 原版 ReadMaybeUnalignedValue<V> — 压缩配置下 >kTaggedSize 类型一律未对齐;
/// Rust read_unaligned 对对齐读取亦无额外开销, 直读 (登记)
///
/// # SAFETY
/// `p` 必须指向一块 ≥ size_of::<V>() 的可读内存, 语义等价 base::Memory
#[inline]
pub unsafe fn read_maybe_unaligned_value<V>(p: Address) -> V {
  unsafe { (p as *const V).read_unaligned() }
}

/// 原版 WriteMaybeUnalignedValue<V>
///
/// # SAFETY
/// `p` 必须指向一块 ≥ size_of::<V>() 的可写内存
#[inline]
pub unsafe fn write_maybe_unaligned_value<V>(p: Address, value: V) {
  unsafe { (p as *mut V).write_unaligned(value) }
}

/// 原版 Smi 压缩判断的对称封装: 堆对象 tag 校验 (checks.h 联动)
#[inline]
pub const fn has_heap_object_tag(v: usize) -> bool {
  v & K_HEAP_OBJECT_TAG_MASK == K_HEAP_OBJECT_TAG
}

/// 原版 HAS_WEAK_HEAP_OBJECT_TAG (globals.h:2035-2037)
#[inline]
pub const fn has_weak_heap_object_tag(v: usize) -> bool {
  v & K_HEAP_OBJECT_TAG_MASK == K_WEAK_HEAP_OBJECT_TAG
}

// ── PtrComprCageAccessScope (ptr-compr.h:240-267) ─────────────────────────
// V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES 关 → no-op 分支

/// 多 cage 切换作用域 — 目标配置 no-op
pub struct PtrComprCageAccessScope;

impl PtrComprCageAccessScope {
  /// 原版 no-op 构造 (ptr-compr.h:263-266); isolate 依赖 L3
  #[inline]
  pub fn new() -> Self {
    Self
  }
}

impl Default for PtrComprCageAccessScope {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

// 原版 V8_COMPRESS_POINTERS_IN_MULTIPLE_CAGES 分支 (ptr-compr-inl.h:342-378)
// 目标配置关 → 不落位, 登记。
// 原版非压缩分支 UNREACHABLE() 族 (ptr-compr-inl.h:280-321) — 压缩恒开, 不落位。
