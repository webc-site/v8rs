//! LocalDeclEncoder Emit/AddLocals/Size 1:1 对原版 local-decl-encoder.cc:
//! 数值局部编码、同类型合并尾项 (local_decls_.back().second == type)、sig 参数偏移、堆/共享/精确分支 Size==Emit 不变式
use aok::{OK, Void};
use v8_wasm_core::{
  local_decl_encoder::LocalDeclEncoder,
  value_type::{GenericKind, K_NULLABLE, ModuleTypeIndex, NumericKind, RefTypeKind, ValueType},
  wasm_constants::ValueTypeCode,
  wasm_opcodes::{SigDef, SigType},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// Emit 数值局部: [decl 数 u32v][count u32v][code]...; 与 Size 一致
#[test]
fn emit_numeric_locals() -> Void {
  let mut e = LocalDeclEncoder::new();
  assert_eq!(e.add_locals(2, ValueType::primitive(NumericKind::I32)), 0);
  assert_eq!(e.add_locals(1, ValueType::primitive(NumericKind::I64)), 2);
  let mut buf = [0u8; 64];
  let n = e.emit(&mut buf);
  assert_eq!(
    &buf[..n],
    &[2, 2, ValueTypeCode::I32 as u8, 1, ValueTypeCode::I64 as u8]
  );
  assert_eq!(e.size(), n, "Size 必须等于 Emit 写入字节数");
  OK
}

/// AddLocals 同类型合并尾项 (v8: 与原版 back().second == type 分支一致)
#[test]
fn add_locals_merges_tail() -> Void {
  let mut e = LocalDeclEncoder::new();
  assert_eq!(e.add_locals(2, ValueType::primitive(NumericKind::I32)), 0);
  assert_eq!(e.add_locals(3, ValueType::primitive(NumericKind::I32)), 2);
  let mut buf = [0u8; 64];
  let n = e.emit(&mut buf);
  // 合并为单条 (5, i32)
  assert_eq!(&buf[..n], &[1, 5, ValueTypeCode::I32 as u8]);
  assert_eq!(e.size(), n);
  // 异类型不合并, 追加新条
  assert_eq!(e.add_locals(1, ValueType::primitive(NumericKind::F64)), 5);
  let n2 = e.emit(&mut buf);
  assert_eq!(
    &buf[..n2],
    &[2, 5, ValueTypeCode::I32 as u8, 1, ValueTypeCode::F64 as u8]
  );
  assert_eq!(e.size(), n2);
  OK
}

/// set_sig: add_locals 起始索引 = total_ + sig->parameter_count() (v8: 参数偏移)
#[test]
fn add_locals_with_sig() -> Void {
  let mut e = LocalDeclEncoder::new();
  assert!(!e.has_sig());
  e.set_sig(SigDef {
    ret: SigType::Void,
    params: &[SigType::I32, SigType::I32],
  });
  assert!(e.has_sig());
  assert_eq!(e.add_locals(4, ValueType::primitive(NumericKind::F32)), 2);
  assert_eq!(e.add_locals(1, ValueType::primitive(NumericKind::F64)), 6);
  OK
}

/// 堆类型/共享/精确 分支: Emit 与 Size 不变式 (v8: DCHECK_EQ(Size(), pos - buffer))
#[test]
fn emit_size_invariant_heap_refs() -> Void {
  let mut e = LocalDeclEncoder::new();
  // 索引引用 (needs_heap_type=true, 非空 → code Ref)
  e.add_locals(
    1,
    ValueType::r#ref(ModuleTypeIndex::new(1), false, RefTypeKind::Function),
  );
  // 共享抽象引用 (needs_shared=true + needs_heap_type=true)
  e.add_locals(2, ValueType::generic(GenericKind::Any, K_NULLABLE, true));
  let mut buf = [0u8; 64];
  let n = e.emit(&mut buf);
  assert!(n > 0, "至少写入 decl 数与类型码");
  assert_eq!(e.size(), n, "含堆类型/共享位时 Size 与 Emit 必须逐字节一致");
  OK
}
