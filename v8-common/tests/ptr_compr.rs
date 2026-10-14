//! ptr-compr 压缩/解压往返与 4GB 回绕校验
//!
//! 运行提示: MainCage / TrustedCage / ExternalCode cage base 为进程级静态
//! (init_base 写入, 见 src/ptr_compr.rs:21-24), 仅限 nextest 进程隔离运行
//! (根 ./test.sh); 裸 cargo test 同进程并行会互相覆写 cage base。

use core::sync::atomic::Ordering;

use aok::{OK, Void};
use v8_common::{
  CompressionCage, ExternalCodeCompressionScheme, K_CLEARED_WEAK_HEAP_OBJECT_LOWER32,
  K_TAGGED_SIZE, MainCage, PtrComprCageBase, SmiCompressionScheme, TrustedSpaceCompressionScheme,
  V8HeapCompressionScheme,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// V8HeapCompressionScheme: 初始化 → 压缩 → 解压往返
#[test]
fn ptr_compr_main_cage_roundtrip() -> Void {
  // cage base 必须按 4GB 对齐 (原版 InitBase CHECK)
  let base = 0x0000_5555_0000_0000usize;
  V8HeapCompressionScheme::init_base(base)?;
  assert_eq!(V8HeapCompressionScheme::base(), base);
  assert_eq!(MainCage::base_().load(Ordering::Relaxed), base);

  // 堆对象 (带 tag) 往返: 解压 = base | 低 32 位
  for raw in [0x1u32, 0x1234_5679u32, 0xFFFF_FFFDu32] {
    let full = base | raw as usize;
    let compressed = V8HeapCompressionScheme::compress_object(full);
    assert_eq!(compressed, raw);
    assert_eq!(V8HeapCompressionScheme::decompress_tagged(compressed), full);
  }
  // CompressAny 截断
  assert_eq!(
    V8HeapCompressionScheme::compress_any(0xABCD_EF01_1234_5678),
    0x1234_5678
  );
  // DecompressTaggedSigned 零扩展
  assert_eq!(
    V8HeapCompressionScheme::decompress_tagged_signed(0x8000_0001),
    0x8000_0001
  );
  // cage base 未对齐落错 (原版 CHECK_IMPLIES)
  assert!(V8HeapCompressionScheme::init_base(base + 1).is_err());
  // 清除态弱引用不触发 cage 校验 (ptr-compr-inl.h:91-93)
  assert_eq!(
    V8HeapCompressionScheme::compress_object(K_CLEARED_WEAK_HEAP_OBJECT_LOWER32 as usize),
    K_CLEARED_WEAK_HEAP_OBJECT_LOWER32
  );
  OK
}

/// TrustedSpaceCompressionScheme 与主 cage 相互独立 (sandbox 双 base)
#[test]
fn ptr_compr_trusted_cage_independent() -> Void {
  let main_base = 0x0000_1234_0000_0000usize;
  let trusted_base = 0x0000_4321_0000_0000usize;
  V8HeapCompressionScheme::init_base(main_base)?;
  TrustedSpaceCompressionScheme::init_base(trusted_base)?;
  assert_eq!(V8HeapCompressionScheme::base(), main_base);
  assert_eq!(TrustedSpaceCompressionScheme::base(), trusted_base);
  let compressed = TrustedSpaceCompressionScheme::compress_object(trusted_base | 0x9);
  assert_eq!(
    TrustedSpaceCompressionScheme::decompress_tagged(compressed),
    trusted_base | 0x9
  );
  OK
}

/// SmiCompressionScheme: 仅 Smi, 截断即解压
#[test]
fn ptr_compr_smi_scheme() -> Void {
  assert_eq!(SmiCompressionScheme::decompress_tagged_signed(0x2A), 0x2A);
  assert_eq!(SmiCompressionScheme::compress_object(0x2A << 1), 0x54);
  OK
}

/// ExternalCodeCompressionScheme: 4GB 回绕解压 (ptr-compr-inl.h:218-239)
#[test]
fn ptr_compr_external_code_wraparound() -> Void {
  // base 落在 4KB 对齐边界, code range 跨 4GB 边界
  let base = 0x0000_7000_0000_1000usize;
  ExternalCodeCompressionScheme::init_base(base)?;
  assert_eq!(ExternalCodeCompressionScheme::base(), base);

  // 高位部分: raw >= base 低 32 位 → base | raw
  let above = base + 0x100;
  let compressed = ExternalCodeCompressionScheme::compress_object(above);
  assert_eq!(
    ExternalCodeCompressionScheme::decompress_tagged(compressed),
    above
  );

  // 跨 4GB 回绕: raw 越过 [base, base+4GB) 上边界, 低 32 位回卷到
  // base 低 32 位之下 → 差值为负 → 补 4GB (ptr-compr.h:129-149 图示)
  let wrap = base + 0xFFFF_F000;
  let compressed = ExternalCodeCompressionScheme::compress_object(wrap);
  assert_eq!(compressed as usize, 0); // 低 32 位回卷为 0
  assert_eq!(
    ExternalCodeCompressionScheme::decompress_tagged(compressed),
    wrap
  );

  // 压缩均为 32 位截断
  assert_eq!(compressed as usize, wrap as u32 as usize);
  // base 未按 4KB 对齐落错
  assert!(ExternalCodeCompressionScheme::init_base(base + 2).is_err());
  OK
}

/// ProcessIntermediatePointers: 高低半字分别解压 (ptr-compr-inl.h:135-148)
#[test]
fn ptr_compr_process_intermediate_pointers() -> Void {
  let base = 0x0000_2222_0000_0000usize;
  V8HeapCompressionScheme::init_base(base)?;

  let low = 0x0000_00ABu32;
  let high = 0x0000_0CD1u32;
  let raw = (high as usize) << 32 | low as usize;

  let mut seen = Vec::with_capacity(2);
  V8HeapCompressionScheme::process_intermediate_pointers(raw, |p| seen.push(p));
  assert_eq!(seen, vec![base | low as usize, base | high as usize]);
  OK
}

/// GetPtrComprCageBase 族与 PtrComprCageBase 构造
#[test]
fn ptr_compr_cage_base_helpers() -> Void {
  let base = 0x0000_3333_0000_0000usize;
  V8HeapCompressionScheme::init_base(base)?;
  // 任意堆内地址归约到 cage base
  let on_heap = base | (K_TAGGED_SIZE + 0x1);
  let cage = v8_common::get_ptr_compr_cage_base_from_on_heap_address(on_heap);
  assert_eq!(cage.address(), base);
  assert_eq!(v8_common::get_ptr_compr_cage_base().address(), base);
  assert_eq!(PtrComprCageBase::from(base).address(), base);
  OK
}

/// ExternalCodeCompressionScheme 边缘边界:
/// 1. raw 低 32 位恰与 base 低 32 位相等 (diff == 0, 无回绕)
/// 2. 高位 cage base (高地址空间) 往返正确
/// 3. ExternalCodeCompressionScheme::process_intermediate_pointers 验证
#[test]
fn ptr_compr_external_code_high_address_and_edge_wraparound() -> Void {
  let base = 0x0000_7FFF_8000_0000usize;
  ExternalCodeCompressionScheme::init_base(base)?;

  // 1. diff == 0: raw_value 等于 base 低 32 位
  let exact_base_low = base as u32;
  let decomp = ExternalCodeCompressionScheme::decompress_tagged(exact_base_low);
  assert_eq!(decomp, base);

  // 2. 跨 4GB 前后两点
  let just_before_wrap = base + 0x7FFF_FFFF;
  let comp1 = ExternalCodeCompressionScheme::compress_object(just_before_wrap);
  assert_eq!(
    ExternalCodeCompressionScheme::decompress_tagged(comp1),
    just_before_wrap
  );

  let just_after_wrap = base + 0x8000_0001;
  let comp2 = ExternalCodeCompressionScheme::compress_object(just_after_wrap);
  assert_eq!(
    ExternalCodeCompressionScheme::decompress_tagged(comp2),
    just_after_wrap
  );

  // 3. process_intermediate_pointers
  let low = comp1;
  let high = comp2;
  let raw = (high as usize) << 32 | low as usize;
  let mut seen = Vec::with_capacity(2);
  ExternalCodeCompressionScheme::process_intermediate_pointers(raw, |p| seen.push(p));
  assert_eq!(seen, vec![just_before_wrap, just_after_wrap]);
  OK
}
