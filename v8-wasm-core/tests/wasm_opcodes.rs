//! WasmOpcode 判定族边界 1:1: is_fp16_simd_opcode 区间穷尽 (wasm-opcodes-inl.h:85-88),
//! is_relaxed_simd_opcode (:82) / is_atomic_rmw_opcode (:90-95) 边界, signature 查表对位 (:Signature)
use aok::{OK, Void};
use v8_wasm_core::wasm_opcodes::{WasmOpcode, WasmOpcodeSig, prefixed_expr};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// fp16 区间端点常量与原版逐值一致
#[test]
fn fp16_endpoints() -> Void {
  assert_eq!(prefixed_expr::F16X8_SPLAT.as_u32(), 0xfd120);
  assert_eq!(prefixed_expr::F16X8_REPLACE_LANE.as_u32(), 0xfd122);
  assert_eq!(prefixed_expr::F16X8_ABS.as_u32(), 0xfd130);
  assert_eq!(prefixed_expr::F16X8_QFMS.as_u32(), 0xfd14f);
  OK
}

/// is_fp16_simd_opcode 边界穷尽: [0xfd120..0xfd122] ∪ [0xfd130..0xfd14f]
/// v8: return (op >= kExprF16x8Splat && op <= kExprF16x8ReplaceLane) || (op >= kExprF16x8Abs && op <= kExprF16x8Qfms)
#[test]
fn fp16_simd_boundaries() -> Void {
  // 区间一下界前 → 假
  assert!(!WasmOpcode(0xfd11f).is_fp16_simd_opcode());
  // 区间一 [SPLAT..REPLACE_LANE] 逐值真 (含两端点)
  for v in [0xfd120u32, 0xfd121, 0xfd122] {
    assert!(
      WasmOpcode(v).is_fp16_simd_opcode(),
      "{v:#x} 应属 fp16 区间一"
    );
  }
  // 空洞 (区间一后, 区间二前) → 假
  for v in [0xfd123u32, 0xfd124, 0xfd12f] {
    assert!(
      !WasmOpcode(v).is_fp16_simd_opcode(),
      "{v:#x} 应落 fp16 空洞"
    );
  }
  // 区间二边界与内部 → 真
  assert!(WasmOpcode(0xfd130).is_fp16_simd_opcode()); // ABS 下界
  assert!(prefixed_expr::F16X8_MIN.is_fp16_simd_opcode()); // 0xfd141 内点
  assert!(WasmOpcode(0xfd14f).is_fp16_simd_opcode()); // QFMS 上界
  // 区间二上界后 → 假
  assert!(!WasmOpcode(0xfd150).is_fp16_simd_opcode());
  OK
}

/// is_relaxed_simd_opcode: (op & 0xfff00) == 0xfd100 边界 (wasm-opcodes-inl.h:82)
#[test]
fn relaxed_simd_boundaries() -> Void {
  assert!(WasmOpcode(0xfd100).is_relaxed_simd_opcode());
  assert!(WasmOpcode(0xfd1ff).is_relaxed_simd_opcode());
  assert!(!WasmOpcode(0xfd0ff).is_relaxed_simd_opcode());
  assert!(!WasmOpcode(0xfd200).is_relaxed_simd_opcode());
  // fp16 (0xfd120) 落 relaxed 区
  assert!(prefixed_expr::F16X8_SPLAT.is_relaxed_simd_opcode());
  OK
}

/// is_atomic_rmw_opcode: [kExprI32AtomicAdd..kExprI64AtomicCompareExchange32U] 边界 (:90-95)
#[test]
fn atomic_rmw_boundaries() -> Void {
  assert_eq!(prefixed_expr::I32_ATOMIC_ADD.as_u32(), 0xfe1e);
  assert_eq!(
    prefixed_expr::I64_ATOMIC_COMPARE_EXCHANGE32_U.as_u32(),
    0xfe4e
  );
  assert!(!WasmOpcode(0xfdff).is_atomic_rmw_opcode());
  assert!(!WasmOpcode(0xfe1d).is_atomic_rmw_opcode()); // 下界前
  assert!(WasmOpcode(0xfe1e).is_atomic_rmw_opcode()); // 下界
  assert!(WasmOpcode(0xfe4e).is_atomic_rmw_opcode()); // 上界
  assert!(!WasmOpcode(0xfe4f).is_atomic_rmw_opcode()); // 上界后
  OK
}

/// signature 查表对位 (对应 WasmOpcodes::Signature / SignatureForAtomicOp)
#[test]
fn signature_lookup() -> Void {
  assert!(matches!(WasmOpcode(0x45).signature(), WasmOpcodeSig::I_I));
  assert!(matches!(WasmOpcode(0xfd00).signature(), WasmOpcodeSig::S_I));
  assert!(matches!(WasmOpcode(0xfc00).signature(), WasmOpcodeSig::I_F));
  assert!(matches!(
    WasmOpcode(0xfe10).signature_for_atomic_op(false),
    WasmOpcodeSig::I_I
  ));
  assert!(matches!(
    WasmOpcode(0xfe10).signature_for_atomic_op(true),
    WasmOpcodeSig::I_L
  ));
  OK
}
