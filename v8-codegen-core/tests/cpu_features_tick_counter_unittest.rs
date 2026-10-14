// v8: src/codegen/cpu-features.h / tick-counter.cc 已填浅水面行为核对
// (原版 test/unittests 无 cpu-features/tick-counter 专属件, 此处按填体语义回归)

use std::ptr::from_mut;

use aok::{OK, Void};
use log::info;
use v8_codegen_core::{CpuFeatureX64, CpuFeatures, TickCounter};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 IsSupported/SetSupported/SetUnsupported 位集语义 (EnumSet 载体)。
/// 单测集中一个用例顺序执行, 避免全局位集在并行用例间的交叉。
#[test]
fn cpu_features_supported_bitset_round_trip() -> Void {
  info!("测试 CpuFeatures supported_ 位集往返");

  let sse3 = CpuFeatureX64::SSE3 as u8;
  let avx2 = CpuFeatureX64::AVX2 as u8;

  // 初始 (本进程未 set 过) 应为 false
  assert!(!CpuFeatures::is_supported(sse3));
  CpuFeatures::set_supported(sse3);
  assert!(CpuFeatures::is_supported(sse3));
  // 相邻位不受影响
  assert!(!CpuFeatures::is_supported(avx2));
  CpuFeatures::set_supported(avx2);
  assert!(CpuFeatures::is_supported(sse3));
  assert!(CpuFeatures::is_supported(avx2));
  CpuFeatures::set_unsupported(sse3);
  assert!(!CpuFeatures::is_supported(sse3));
  assert!(CpuFeatures::is_supported(avx2));
  // 幂等面
  CpuFeatures::set_unsupported(sse3);
  assert!(!CpuFeatures::is_supported(sse3));
  CpuFeatures::set_unsupported(avx2);

  // 探测面前置: Probe 未接线前 initialized 为 false (原版 Probe 幂等门语义)
  assert!(!CpuFeatures::initialized());

  OK
}

/// 原版 TickCounter::AttachLocalHeap/DetachLocalHeap (tick-counter.cc:15-21):
/// DCHECK_NULL 绑定 → 复位可重绑; Detach 置空。
#[test]
fn tick_counter_attach_detach_local_heap() -> Void {
  info!("测试 TickCounter 本地堆绑定");

  let mut counter = TickCounter::default();
  let heap = Box::into_raw(Box::new(()));

  counter.attach_local_heap(heap);
  // v8: tick-counter.h TickAndMaybeEnterSafepoint: ++ticks_ 面 (已填体回归)
  counter.tick_and_maybe_enter_safepoint();
  counter.tick_and_maybe_enter_safepoint();
  assert_eq!(counter.current_ticks(), 2);

  counter.detach_local_heap();
  // Detach 后可再 Attach (DCHECK_NULL 满足)
  let mut placeholder = ();
  counter.attach_local_heap(from_mut(&mut placeholder));
  counter.detach_local_heap();

  drop(unsafe { Box::from_raw(heap) });
  OK
}
