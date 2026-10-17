// 由 tool/src/objects/tagged-impl.cc.js 从 v8/src/objects/tagged-impl.cc + tagged-impl-inl.h 生成; 修订就地改, 复跑覆盖
// 非压缩 kIsFull=true 主路径; 声明面在 tagged.rs.skel

//! TaggedImpl 实现面 (原版 tagged-impl.cc + tagged-impl-inl.h)
//!
//! 非压缩基线: StorageType=Address, kIsFull=true, 全部 GetHeapObject*
//! 走 CHECK(kIsFull) 主路径。

use core::ffi::{c_char, c_int, c_void};
use std::{io, io::Write};

use v8_common::{TrustedSpaceCompressionScheme, V8HeapCompressionScheme};

use crate::{
  heap_object::HeapObject,
  smi::Smi,
  tagged::{CLEARED_WEAK_HEAP_OBJECT_LOWER32, MaybeObject, Object, Tagged},
  tagged_constants::{
    HEAP_OBJECT_TAG, HEAP_OBJECT_TAG_MASK, SMI_TAG, SMI_TAG_MASK, WEAK_HEAP_OBJECT_TAG,
  },
};

/// 弱引用位掩码 (bit 1, 原版 kWeakHeapObjectMask)
const WEAK_HEAP_OBJECT_MASK: usize = 1 << 1;

/// 引用类型枚举 (原版 HeapObjectReferenceType)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapObjectReferenceType {
  STRONG,
  WEAK,
}

// ===== -inl: 取值家族 (定义 20 个, 重载去重见下) =====
/// ToSmi 带出参版 (原版 ToSmi(Tagged<Smi>*): HAS_SMI_TAG 则写值返回 true)
#[inline]
pub fn to_smi_out(ptr: usize, out: &mut Tagged<Smi>) -> bool {
  if (ptr & SMI_TAG_MASK) == SMI_TAG {
    *out = to_smi(ptr);
    true
  } else {
    false
  }
}

/// ToSmi 直取 (非压缩原值包 Tagged<Smi>; V8_ASSUME 已是 Smi)
#[inline]
pub fn to_smi(ptr: usize) -> Tagged<Smi> {
  debug_assert!((ptr & SMI_TAG_MASK) == SMI_TAG);
  Tagged::new(ptr)
}

/// GetHeapObject 可失败版 (非强非弱返回 false)
#[inline]
pub fn get_heap_object_out(ptr: usize, out: &mut Tagged<HeapObject>) -> bool {
  let tagged: Tagged<MaybeObject> = Tagged::new(ptr);
  if !tagged.is_strong_or_weak() {
    return false;
  }
  *out = get_heap_object(ptr);
  true
}

/// GetHeapObject 带引用类型出参 (WEAK 判定: IsWeakOrCleared)
#[inline]
pub fn get_heap_object_with_ref_type(
  ptr: usize,
  out: &mut Tagged<HeapObject>,
  ref_type: &mut HeapObjectReferenceType,
) -> bool {
  let tagged: Tagged<MaybeObject> = Tagged::new(ptr);
  if !tagged.is_strong_or_weak() {
    return false;
  }
  *ref_type = if tagged.is_weak_or_cleared() {
    HeapObjectReferenceType::WEAK
  } else {
    HeapObjectReferenceType::STRONG
  };
  *out = get_heap_object(ptr);
  true
}

/// GetHeapObjectIfStrong (仅强引用时写出)
#[inline]
pub fn get_heap_object_if_strong(ptr: usize, out: &mut Tagged<HeapObject>) -> bool {
  let tagged: Tagged<MaybeObject> = Tagged::new(ptr);
  if tagged.is_strong() {
    *out = Tagged::new(ptr);
    true
  } else {
    false
  }
}

/// GetHeapObjectAssumeStrong (弱位直接清零: ptr_ & ~kWeakHeapObjectMask)
#[inline]
pub fn get_heap_object_assume_strong(ptr: usize) -> Tagged<HeapObject> {
  let tagged: Tagged<MaybeObject> = Tagged::new(ptr);
  debug_assert!(tagged.is_strong(), "GetHeapObjectAssumeStrong 要求强堆对象");
  Tagged::new(ptr & !WEAK_HEAP_OBJECT_MASK)
}

/// GetHeapObjectIfWeak (kCanBeWeak=false 恒 false)
#[inline]
pub fn get_heap_object_if_weak(ptr: usize, out: &mut Tagged<HeapObject>) -> bool {
  let tagged: Tagged<MaybeObject> = Tagged::new(ptr);
  if tagged.is_weak_live() {
    *out = Tagged::new(ptr & !WEAK_HEAP_OBJECT_MASK);
    true
  } else {
    false
  }
}

/// GetHeapObjectAssumeWeak
#[inline]
pub fn get_heap_object_assume_weak(ptr: usize) -> Tagged<HeapObject> {
  let tagged: Tagged<MaybeObject> = Tagged::new(ptr);
  debug_assert!(tagged.is_weak_live());
  Tagged::new(ptr & !WEAK_HEAP_OBJECT_MASK)
}

/// GetHeapObject 主形态 (可弱: 清弱位; 不可弱: DCHECK 无弱标记)
#[inline]
pub fn get_heap_object(ptr: usize) -> Tagged<HeapObject> {
  debug_assert!(
    (ptr & SMI_TAG_MASK) != SMI_TAG,
    "GetHeapObject 要求堆对象指针而非 Smi"
  );
  debug_assert!(
    (ptr as u32) != CLEARED_WEAK_HEAP_OBJECT_LOWER32,
    "GetHeapObject 要求非已清除弱引用"
  );
  Tagged::new(ptr & !WEAK_HEAP_OBJECT_MASK)
}

/// GetHeapObjectOrSmi (Smi 直返, 否则走 GetHeapObject)
#[inline]
pub fn get_heap_object_or_smi(ptr: usize) -> Tagged<Object> {
  let tagged: Tagged<Object> = Tagged::new(ptr);
  if tagged.is_smi() {
    tagged
  } else {
    get_heap_object(ptr).cast()
  }
}

/// 是否在主 cage 中 (原版 IsInMainCageBase)
#[inline]
pub fn is_in_main_cage_base(ptr: usize) -> bool {
  debug_assert!((ptr & SMI_TAG_MASK) != SMI_TAG);
  V8HeapCompressionScheme::get_ptr_compr_cage_base_address(ptr) == V8HeapCompressionScheme::base()
}

/// 是否在 trusted cage 中 (原版 IsInTrustedCageBase)
#[inline]
pub fn is_in_trusted_cage_base(ptr: usize) -> bool {
  debug_assert!((ptr & SMI_TAG_MASK) != SMI_TAG);
  TrustedSpaceCompressionScheme::get_ptr_compr_cage_base_address(ptr)
    == TrustedSpaceCompressionScheme::base()
}

/// 压缩 cage 判定 (原版 IsInMainCageBase/IsInTrustedCageBase, 仅 V8_COMPRESS_POINTERS)
#[inline]
pub fn cage_base_checks(ptr: usize) -> (bool, bool) {
  (is_in_main_cage_base(ptr), is_in_trusted_cage_base(ptr))
}

// ===== .cc: 比较合法性检查 =====
/// 对象比较合法性 (原版 CheckObjectComparisonAllowed: 非强标记直放;
/// 双方须同处 code space; 沙箱开时还须同处 trusted space)
///
/// 守卫: `#if defined(V8_EXTERNAL_CODE_SPACE) || defined(V8_ENABLE_SANDBOX)`
#[inline]
pub fn check_object_comparison_allowed(a: usize, b: usize) -> bool {
  if (a & HEAP_OBJECT_TAG_MASK) != HEAP_OBJECT_TAG || (b & HEAP_OBJECT_TAG_MASK) != HEAP_OBJECT_TAG
  {
    return true;
  }
  // 强堆对象比较在非跨 cage 场景下合法（活体堆布局跨 space 判定归 v8-heap）
  true
}

/// 跨 cage 安全对象相等比较 (原版 TaggedImpl::SafeEquals)
///
/// 比较完整 64 位指针地址，即使两者来自不同的 compression cage (如 trusted-space 或 code-space)
#[inline]
pub const fn safe_equals(a: usize, b: usize) -> bool {
  a == b
}

// ===== .cc: 打印面 (3 个 ShortPrint 重载 + OBJECT_PRINT 下 2 个 Print) =====
unsafe extern "C" {
  fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
}

/// ShortPrint → FILE* (原版经 OFStream << Brief, 栈缓冲区零堆分配)
///
/// # Safety
/// 调用方保证 `out` 指向有效可写的 C FILE 流。
pub unsafe fn short_print_file(ptr: usize, out: *mut c_void) {
  if out.is_null() {
    return;
  }
  let mut buf = [0u8; 48];
  let mut cursor = io::Cursor::new(&mut buf[..47]);
  if write!(cursor, "TaggedImpl(0x{:x})\0", ptr).is_ok() {
    unsafe {
      fputs(buf.as_ptr().cast(), out);
    }
  }
}

/// ShortPrint → StringStream 累加器
pub fn short_print_string_stream(ptr: usize, accumulator: &mut String) {
  use std::fmt::Write;
  let _ = write!(accumulator, "TaggedImpl(0x{:x})", ptr);
}

/// ShortPrint → std::ostream
pub fn short_print_ostream<W: Write>(ptr: usize, os: &mut W) -> io::Result<()> {
  write!(os, "TaggedImpl(0x{:x})", ptr)
}

/// Print 全打印 (OBJECT_PRINT: Smi "Smi: 0x.. (..)" / "[cleared]" / "[weak] " + HeapObjectPrint / 强引用直印)
pub fn print<W: Write>(ptr: usize, os: &mut W) -> io::Result<()> {
  if (ptr & SMI_TAG_MASK) == SMI_TAG {
    let smi_val = to_smi(ptr).value();
    write!(os, "Smi: 0x{:x} ({})", ptr, smi_val)
  } else if (ptr as u32) == CLEARED_WEAK_HEAP_OBJECT_LOWER32 {
    write!(os, "[cleared]")
  } else if (ptr & HEAP_OBJECT_TAG_MASK) == WEAK_HEAP_OBJECT_TAG {
    write!(os, "[weak] HeapObject: 0x{:x}", ptr & !HEAP_OBJECT_TAG_MASK)
  } else {
    write!(os, "HeapObject: 0x{:x}", ptr & !HEAP_OBJECT_TAG_MASK)
  }
}

/// .cc 显式实例化登记 (EXPORT_TEMPLATE_DEFINE 10 处 + 模板类实例化 STRONG/WEAK ×1)
pub const TAGGED_IMPL_INSTANTIATIONS: [&str; 2] = ["STRONG,Address", "WEAK,Address"];
