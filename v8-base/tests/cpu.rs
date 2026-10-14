//! v8/test/unittests/base/cpu-unittest.cc 的规约迁移
//!
//! 规约清单:
//! - CPUTest.SuppressTagCheckingScope: ARM64 MTE 内联汇编用例,
//!   依赖 heap/base/memory-tagging.h (heap 层), v8-base 无对应 — 跳过 (登记)
//! - CPUTest.FeatureImplications: 11 条蕴含断言
//!   (!sse || mmx), (!sse2 || sse), (!sse3 || sse2), (!ssse3 || sse3),
//!   (!sse41 || sse3), (!sse42 || sse41), (!avx || sse2), (!fma3 || avx),
//!   (!avx2 || avx), (!avx10_1 || avx2), (!vfp3_d32 || vfp3)
//! - CPUTest.RequiredFeatures: 按宿主架构 (x64: fpu/cmov/mmx/sse/sse2)
//!   (arm64 宿主原版无硬性断言)

use std::ptr::eq;

use aok::{OK, Void};
use v8_base::cpu::Cpu;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// TEST(CPUTest, FeatureImplications)
#[test]
fn cpu_feature_implications() -> Void {
  let cpu = Cpu::new();

  // ia32 and x64 features (原版 11 条蕴含逐条 1:1)
  assert!(!cpu.has_sse() || cpu.has_mmx());
  assert!(!cpu.has_sse2() || cpu.has_sse());
  assert!(!cpu.has_sse3() || cpu.has_sse2());
  assert!(!cpu.has_ssse3() || cpu.has_sse3());
  assert!(!cpu.has_sse41() || cpu.has_sse3());
  assert!(!cpu.has_sse42() || cpu.has_sse41());
  assert!(!cpu.has_avx() || cpu.has_sse2());
  assert!(!cpu.has_fma3() || cpu.has_avx());
  assert!(!cpu.has_avx2() || cpu.has_avx());
  assert!(!cpu.has_avx10_1() || cpu.has_avx2());

  // arm features
  assert!(!cpu.has_vfp3_d32() || cpu.has_vfp3());
  OK
}

/// TEST(CPUTest, RequiredFeatures) — x64 宿主分支
#[cfg(target_arch = "x86_64")]
#[test]
fn cpu_required_features() -> Void {
  let cpu = Cpu::new();
  assert!(cpu.has_fpu());
  assert!(cpu.has_cmov());
  assert!(cpu.has_mmx());
  assert!(cpu.has_sse());
  assert!(cpu.has_sse2());
  OK
}

/// arm64 宿主: 原版 RequiredFeatures 无硬性断言 (仅 ARM/IA32/X64 有),
/// 补充 GetInstance 单例与 vendor 基本形态断言 (加强)
#[cfg(target_arch = "aarch64")]
#[test]
fn cpu_required_features() -> Void {
  let a = Cpu::get_instance();
  let b = Cpu::get_instance();
  assert!(eq(a, b), "GetInstance 须返回同一实例");
  // vendor 非空 (aarch64 探测路径未覆盖 vendor 时保持 "Unknown")
  assert!(!a.vendor().is_empty());
  OK
}

/// 探测路径基本形态 (跨架构): vendor 与 GetInstance 一致性
#[test]
fn cpu_instance_and_vendor() -> Void {
  let cpu = Cpu::new();
  // vendor 恒为 12 字节内 NUL 结尾串
  assert!(cpu.vendor().len() <= 12);
  let inst = Cpu::get_instance();
  assert!(eq(inst, Cpu::get_instance()));
  OK
}
