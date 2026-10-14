//! ValueType 位域编码测试: kind/heap-type 组合 raw 位表示往返、is_nullable 语义、
//! 码表、尺寸表、LoadType/StoreType 表 (位布局与原版 1:1, 禁改编码)
use aok::{OK, Void};
use v8_wasm_core::{value_type::*, wasm_constants::ValueTypeCode};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// NumericKind 判别值 (payload<<8 | shared 位): std 序 24..=31
#[test]
fn numeric_kind_discriminants() -> Void {
  assert_eq!(NumericKind::I32.bits(), 0x1810);
  assert_eq!(NumericKind::I64.bits(), 0x1910);
  assert_eq!(NumericKind::F32.bits(), 0x1A10);
  assert_eq!(NumericKind::F64.bits(), 0x1B10);
  assert_eq!(NumericKind::S128.bits(), 0x1C10);
  assert_eq!(NumericKind::I8.bits(), 0x1D10);
  assert_eq!(NumericKind::I16.bits(), 0x1E10);
  assert_eq!(NumericKind::F16.bits(), 0x1F10);
  // from_bits 往返
  for k in [
    NumericKind::I32,
    NumericKind::I64,
    NumericKind::F32,
    NumericKind::F64,
    NumericKind::S128,
    NumericKind::I8,
    NumericKind::I16,
    NumericKind::F16,
  ] {
    assert_eq!(NumericKind::from_bits(k.bits()), Some(k));
  }
  OK
}

/// GenericKind 判别值 (payload<<8 | extra bits): REF=1 SENTINEL=2 FUNC=0x20 STRUCT=0x40 ARRAY=0x60 CONT=0x80
#[test]
fn generic_kind_discriminants() -> Void {
  assert_eq!(GenericKind::Void.bits(), 0x0002);
  assert_eq!(GenericKind::Top.bits(), 0x0102);
  assert_eq!(GenericKind::Bottom.bits(), 0x0202);
  assert_eq!(GenericKind::ExternString.bits(), 0x0301);
  assert_eq!(GenericKind::NoCont.bits(), 0x0481); // REF|CONT
  assert_eq!(GenericKind::NoFunc.bits(), 0x0721); // REF|FUNC
  assert_eq!(GenericKind::None.bits(), 0x0901);
  assert_eq!(GenericKind::Func.bits(), 0x0A21); // REF|FUNC
  assert_eq!(GenericKind::Cont.bits(), 0x0E81); // REF|CONT
  assert_eq!(GenericKind::Eq.bits(), 0x1001);
  assert_eq!(GenericKind::Struct.bits(), 0x1241); // REF|STRUCT
  assert_eq!(GenericKind::Array.bits(), 0x1361); // REF|ARRAY
  assert_eq!(GenericKind::StringViewIter.bits(), 0x1701);
  // StandardType 序: generic 24 项在前, numeric 8 项在后
  assert_eq!(StandardType::I32 as u32, 24);
  assert_eq!(StandardType::F16 as u32, 31);
  assert_eq!(NUMERIC_LOOKUP_OFFSET, 24);
  OK
}

/// 数值类型: raw 位往返 + 查询语义
#[test]
fn numeric_value_type_bits() -> Void {
  let i32t = ValueType::primitive(NumericKind::I32);
  assert_eq!(i32t.bits(), 0x1810);
  assert!(i32t.is_numeric());
  assert!(!i32t.is_ref());
  assert!(!i32t.is_nullable());
  assert!(i32t.is_shared()); // 数值恒共享
  assert!(i32t.is_valid());
  assert_eq!(i32t.value_type_code(), ValueTypeCode::I32);
  assert_eq!(i32t.short_name(), 'i');
  assert_eq!(i32t.name(), "i32");
  // FromBits → bits 往返无损
  assert_eq!(ValueType::from_raw_bit_field(i32t.bits()), i32t);
  assert_eq!(i32t.value_kind_size(), 4);
  assert_eq!(i32t.value_kind_size_log2(), 2);
  OK
}

/// 引用类型: nullable/exact/shared 位与 heap type 组合
#[test]
fn ref_value_type_bits() -> Void {
  // funcref (可空抽象引用): REF|FUNC payload + nullable 位
  let funcref = ValueType::generic(GenericKind::Func, K_NULLABLE, false);
  assert!(funcref.is_ref());
  assert!(funcref.is_nullable());
  assert!(!funcref.is_non_nullable());
  assert!(!funcref.is_shared());
  assert!(funcref.is_valid());
  assert_eq!(funcref.value_type_code(), ValueTypeCode::FuncRef); // 0x70 短编码
  assert!(!funcref.encoding_needs_heap_type()); // 可空且非共享 → 短编码
  // 非空 funcref 需 (ref func) 编码
  let func_nonnull = funcref.as_non_null();
  assert!(func_nonnull.is_non_nullable());
  assert!(func_nonnull.encoding_needs_heap_type());
  assert_eq!(func_nonnull.value_type_code(), ValueTypeCode::Ref); // 0x64
  // exact 位
  assert!(!funcref.is_exact());
  // 索引引用: (ref $5) 共享函数类型
  let idx = ModuleTypeIndex::new(5);
  let r = ValueType::r#ref(idx, true, RefTypeKind::Function);
  assert!(r.has_index());
  assert!(!r.is_nullable());
  assert!(r.is_shared());
  assert_eq!(r.ref_index(), idx);
  assert!(r.encoding_needs_heap_type());
  assert_eq!(r.value_type_code(), ValueTypeCode::Ref);
  // RefNull 翻转 nullable
  let rn = ValueType::ref_null(idx, true, RefTypeKind::Function);
  assert!(rn.is_nullable());
  assert!(rn.is_defaultable());
  assert!(!r.is_defaultable());
  // payload 位: bits 8-28
  assert_eq!((rn.bits() >> 8) & 0x1F_FFFF, 5);
  // TypeKind = IndexedRef = 0b11
  assert_eq!(rn.bits() & 0b11, 0b11);
  OK
}

/// HeapType: 等值忽略 is_ref/nullable 位; from_code 码表
#[test]
fn heap_type_semantics() -> Void {
  // nullable 位不同但堆类型相等 (原版 operator== 掩掉 is_ref/nullable 位)
  let a = HeapType::from_bits(ValueType::generic(GenericKind::Func, K_NON_NULLABLE, false).bits());
  let b = HeapType::from_bits(ValueType::generic(GenericKind::Func, K_NULLABLE, false).bits());
  assert_ne!(a.bits(), b.bits()); // 原始位域确有差异
  assert_eq!(a, b); // 掩码后相等
  // shared 位参与等值 (类型本体一部分)
  let c = HeapType::generic(GenericKind::Func, true);
  assert_ne!(a, c);
  assert_ne!(a, HeapType::generic(GenericKind::Any, false));
  // from_code: wire 堆类型码表
  assert_eq!(
    HeapType::from_code(0x70, false).generic_kind(),
    Some(GenericKind::Func)
  );
  assert_eq!(
    HeapType::from_code(0x6e, false).generic_kind(),
    Some(GenericKind::Any)
  );
  assert_eq!(
    HeapType::from_code(0x5b, false).generic_kind(),
    Some(GenericKind::NoWaitqueue)
  );
  assert_eq!(
    HeapType::from_code(0x5c, false).generic_kind(),
    Some(GenericKind::Waitqueue)
  );
  assert_eq!(
    HeapType::from_code(0x6a, false).generic_kind(),
    Some(GenericKind::Array)
  );
  // 界外码 → Bottom; 区间内未分配码 (0x5d) → Bottom
  assert_eq!(
    HeapType::from_code(0x40, false).generic_kind(),
    Some(GenericKind::Bottom)
  );
  assert_eq!(
    HeapType::from_code(0x5d, false).generic_kind(),
    Some(GenericKind::Bottom)
  );
  // code(): 抽象码符号扩展 (0x70 → -0x10)
  assert_eq!(a.code(), -128i32 | 0x70);
  // 索引堆类型 code = 索引
  let hi = HeapType::index(
    ModuleTypeIndex::new(9),
    false,
    RefTypeKind::Struct,
    K_ANY_SUBTYPE,
  );
  assert_eq!(hi.code(), 9);
  assert_eq!(hi.ref_index(), ModuleTypeIndex::new(9));
  OK
}

/// 预定义常量 (kWasm* 族)
#[test]
fn predefined_constants() -> Void {
  assert!(K_WASM_I32.is_numeric());
  assert_eq!(K_WASM_I32.name(), "i32");
  assert_eq!(K_WASM_S128.value_kind_size(), 16);
  assert!(K_WASM_VOID.is_void());
  assert!(K_WASM_TOP.is_top());
  assert!(K_WASM_BOTTOM.is_bottom());
  assert!(K_WASM_BOTTOM.is_none_or_bottom());
  assert!(K_WASM_NULL_REF.is_none_or_bottom());
  // IndependentHeapType → ValueType 转换后位域一致
  let vt: ValueType = K_WASM_FUNC_REF.into();
  assert!(vt.is_nullable());
  assert_eq!(vt.generic_kind(), Some(GenericKind::Func));
  // waitqueue 默认共享
  assert!(K_WASM_WAITQUEUE_REF.is_shared());
  // 非空变体
  assert!(K_WASM_REF_NONE.is_non_nullable());
  assert!(K_WASM_REF_NONE.is_uninhabited()); // 非空 none 引用无居民
  OK
}

/// is_valid: 位域合法性校验
#[test]
fn is_valid_checks() -> Void {
  // 保留位非零 → 非法
  assert!(!ValueType::from_raw_bit_field(0x1810 | (1 << 30)).is_valid());
  // 数值类型固定位被破坏 → 非法
  assert!(!ValueType::from_raw_bit_field(NumericKind::I32.bits() | (1 << 2)).is_valid());
  // 泛型 payload 越界 (>= 24) → 非法
  assert!(!ValueType::from_raw_bit_field(0b01 | (100 << 8)).is_valid());
  // 泛型 exact 位 → 非法 (仅索引可 exact)
  assert!(
    !ValueType::generic(GenericKind::Func, K_NON_NULLABLE, false)
      .as_exact(K_EXACT)
      .is_valid()
  );
  // 合法索引引用 (含 exact)
  let r = ValueType::r#ref(ModuleTypeIndex::new(3), false, RefTypeKind::Struct).as_exact(K_EXACT);
  assert!(r.is_valid());
  assert!(r.is_exact());
  OK
}

/// 名字表 (value-type.cc)
#[test]
fn names() -> Void {
  assert_eq!(ValueType::primitive(NumericKind::F64).name(), "f64");
  // 可空 null 堆类型简写
  let nullref = ValueType::generic(GenericKind::None, K_NULLABLE, false);
  assert_eq!(nullref.name(), "nullref");
  let shared_null = ValueType::generic(GenericKind::None, K_NULLABLE, true);
  assert_eq!(shared_null.name(), "(shared nullref)");
  assert_eq!(
    ValueType::generic(GenericKind::NoFunc, K_NULLABLE, false).name(),
    "nullfuncref"
  );
  // 非空抽象引用 → (ref func)
  assert_eq!(
    ValueType::generic(GenericKind::Func, K_NON_NULLABLE, false).name(),
    "(ref func)"
  );
  // 可空非 null 抽象 → 简写 + ref 后缀
  assert_eq!(
    ValueType::generic(GenericKind::Any, K_NULLABLE, false).name(),
    "anyref"
  );
  // 索引引用 → (ref null exact N)
  let r = ValueType::ref_null(ModuleTypeIndex::new(7), false, RefTypeKind::Array).as_exact(K_EXACT);
  assert_eq!(r.name(), "(ref null exact 7)");
  // 哨兵
  assert_eq!(K_WASM_VOID.name(), "<void>");
  assert_eq!(K_WASM_BOTTOM.name(), "<bot>");
  OK
}

/// 尺寸表: 引用 → kTaggedSize(4), 数值查 FOREACH 表
#[test]
fn sizes() -> Void {
  assert_eq!(ValueType::primitive(NumericKind::I8).value_kind_size(), 1);
  assert_eq!(ValueType::primitive(NumericKind::I16).value_kind_size(), 2);
  assert_eq!(ValueType::primitive(NumericKind::I32).value_kind_size(), 4);
  assert_eq!(ValueType::primitive(NumericKind::I64).value_kind_size(), 8);
  assert_eq!(
    ValueType::primitive(NumericKind::S128).value_kind_size(),
    16
  );
  // 引用 → kTaggedSize = 4 (压缩指针), full → 8
  let r = ValueType::generic(GenericKind::Any, K_NULLABLE, false);
  assert_eq!(r.value_kind_size(), 4);
  assert_eq!(r.value_kind_full_size(), 8);
  // ValueKind free functions
  assert_eq!(value_kind_size(ValueKind::S128), Some(16));
  assert_eq!(value_kind_size(ValueKind::Void), None); // 哨兵无尺寸
  assert_eq!(value_kind_full_size(ValueKind::RefNull), Some(8));
  assert!(is_reference(ValueKind::RefNull));
  assert!(!is_reference(ValueKind::I32));
  assert!(is_packed(ValueKind::I8));
  assert_eq!(unpacked(ValueKind::I8), ValueKind::I32);
  assert_eq!(unpacked(ValueKind::F16), ValueKind::F32);
  OK
}

/// Unpacked / Canonicalize
#[test]
fn unpack_and_canonicalize() -> Void {
  assert_eq!(
    ValueType::primitive(NumericKind::I8).unpacked(),
    ValueType::primitive(NumericKind::I32)
  );
  assert_eq!(
    ValueType::primitive(NumericKind::F16).unpacked(),
    ValueType::primitive(NumericKind::F32)
  );
  assert_eq!(
    ValueType::primitive(NumericKind::I64).unpacked(),
    ValueType::primitive(NumericKind::I64)
  );
  // Canonicalize: 模块索引 → 规范索引 (payload 替换)
  let m = ValueType::r#ref(ModuleTypeIndex::new(42), false, RefTypeKind::Function);
  let c = m.canonicalize(CanonicalTypeIndex::new(1000));
  assert!(c.has_index());
  assert_eq!(c.ref_index(), CanonicalTypeIndex::new(1000));
  // 除 payload 外位不变
  assert!(c.is_equal_except_index(m.canonicalize(CanonicalTypeIndex::new(2000))));
  assert!(c.is_function_type());
  OK
}

/// LoadType / StoreType 表 (FOREACH_LOAD_TYPE/FOREACH_STORE_TYPE)
#[test]
fn load_store_types() -> Void {
  // I32 → I32Load (is_signed 仅对 I8/I16 有意义)
  assert_eq!(
    LoadType::for_value_kind(ValueKind::I32, true),
    LoadType(LoadTypeValue::I32Load)
  );
  // I8 有符号 → I32Load8S: 1 字节
  let l8s = LoadType::for_value_kind(ValueKind::I8, true);
  assert_eq!(l8s, LoadType(LoadTypeValue::I32Load8S));
  assert_eq!(l8s.size(), 1);
  assert_eq!(l8s.size_log_2(), 0);
  assert!(l8s.mem_type().signed);
  // I8 无符号 → I32Load8U
  assert_eq!(
    LoadType::for_value_kind(ValueKind::I8, false),
    LoadType(LoadTypeValue::I32Load8U)
  );
  // I64Load: 8 字节
  let l64 = LoadType(LoadTypeValue::I64Load);
  assert_eq!(l64.size(), 8);
  assert_eq!(l64.value_type(), ValueType::primitive(NumericKind::I64));
  // S128Load: 16 字节
  assert_eq!(LoadType(LoadTypeValue::S128Load).size(), 16);
  // F32LoadF16: 2 字节, 值类型 f32
  let f16 = LoadType::for_value_kind(ValueKind::F16, false);
  assert_eq!(f16, LoadType(LoadTypeValue::F32LoadF16));
  assert_eq!(f16.size(), 2);
  assert_eq!(f16.value_type(), ValueType::primitive(NumericKind::F32));
  // Store: I32Store8 → 1 字节 Word8
  let s8 = StoreType::for_value_kind(ValueKind::I8);
  assert_eq!(s8, StoreType(StoreTypeValue::I32Store8));
  assert_eq!(s8.size_log_2(), 0);
  assert_eq!(s8.mem_rep(), MachineRepresentation::Word8);
  // S128Store → 16 字节 Simd128
  let s128 = StoreType(StoreTypeValue::S128Store);
  assert_eq!(s128.size(), 16);
  assert_eq!(s128.mem_rep(), MachineRepresentation::Simd128);
  OK
}

/// TypeIndex 哨兵
#[test]
fn type_index_invalid() -> Void {
  assert!(!ModuleTypeIndex::INVALID.is_valid());
  assert!(ModuleTypeIndex::new(0).is_valid());
  assert!(!CanonicalTypeIndex::INVALID.is_valid());
  assert_eq!(K_NO_SUPER_TYPE, ModuleTypeIndex::INVALID);
  OK
}

/// from_code: 界内未分配码保留 shared; 界外码丢弃 shared (原版 lookup 表语义)
#[test]
fn from_code_shared_semantics() -> Void {
  // 界内未分配码 (0x5d..0x65 洞: kExactCode/kRefNullCode/kRefCode 等) → Bottom 保留 shared
  for hole in [0x5d, 0x5e, 0x5f, 0x62, 0x63, 0x64, 0x65] {
    let t = HeapType::from_code(hole, true);
    assert_eq!(t.generic_kind(), Some(GenericKind::Bottom), "码 {hole:#x}");
    assert!(t.is_shared(), "界内未分配码 {hole:#x} 应保留 shared");
  }
  let t = HeapType::from_code(0x5d, false);
  assert_eq!(t.generic_kind(), Some(GenericKind::Bottom));
  assert!(!t.is_shared());
  // 界外码 → Bottom 且丢弃 shared
  let t = HeapType::from_code(0x40, true);
  assert_eq!(t.generic_kind(), Some(GenericKind::Bottom));
  assert!(!t.is_shared(), "界外码丢弃 shared");
  let t = HeapType::from_code(0x76, true);
  assert!(!t.is_shared(), "界外码丢弃 shared");
  // 界内已分配码正常带 shared
  let t = HeapType::from_code(0x6e, true);
  assert_eq!(t.generic_kind(), Some(GenericKind::Any));
  assert!(t.is_shared());
  OK
}

/// kind(): 位域 → 粗粒度 ValueKind (原版 ValueTypeBase::kind)
#[test]
fn kind_mapping() -> Void {
  assert_eq!(
    ValueType::primitive(NumericKind::I32).kind(),
    ValueKind::I32
  );
  assert_eq!(
    ValueType::primitive(NumericKind::F16).kind(),
    ValueKind::F16
  );
  assert_eq!(
    ValueType::generic(GenericKind::Func, K_NULLABLE, false).kind(),
    ValueKind::RefNull
  );
  assert_eq!(
    ValueType::generic(GenericKind::Func, K_NON_NULLABLE, false).kind(),
    ValueKind::Ref
  );
  assert_eq!(K_WASM_VOID.kind(), ValueKind::Void);
  assert_eq!(K_WASM_TOP.kind(), ValueKind::Top);
  assert_eq!(K_WASM_BOTTOM.kind(), ValueKind::Bottom);
  // 索引引用按 nullable 分
  let idx = ModuleTypeIndex::new(1);
  assert_eq!(
    ValueType::r#ref(idx, false, RefTypeKind::Struct).kind(),
    ValueKind::Ref
  );
  assert_eq!(
    ValueType::ref_null(idx, false, RefTypeKind::Struct).kind(),
    ValueKind::RefNull
  );
  OK
}

/// machine_type(kind) free fn (FOREACH_VALUE_TYPE machineType 列)
#[test]
fn machine_type_by_kind() -> Void {
  assert_eq!(machine_type(ValueKind::I32), MachineType::int32());
  assert_eq!(machine_type(ValueKind::I64), MachineType::int64());
  assert_eq!(machine_type(ValueKind::F32), MachineType::float32());
  assert_eq!(machine_type(ValueKind::F64), MachineType::float64());
  assert_eq!(machine_type(ValueKind::S128), MachineType::simd128());
  assert_eq!(machine_type(ValueKind::I8), MachineType::int8());
  assert_eq!(machine_type(ValueKind::I16), MachineType::int16());
  assert_eq!(machine_type(ValueKind::F16), MachineType::float16());
  assert_eq!(machine_type(ValueKind::Ref), MachineType::any_tagged());
  assert_eq!(machine_type(ValueKind::RefNull), MachineType::any_tagged());
  assert_eq!(machine_type(ValueKind::Void), MachineType::none());
  assert_eq!(machine_type(ValueKind::Top), MachineType::none());
  assert_eq!(machine_type(ValueKind::Bottom), MachineType::none());
  // 与 ValueType::machine_type 一致性
  for k in [
    ValueKind::I32,
    ValueKind::I64,
    ValueKind::F32,
    ValueKind::F64,
    ValueKind::S128,
    ValueKind::I8,
    ValueKind::I16,
    ValueKind::F16,
  ] {
    let vt = ValueType::primitive_kind(k);
    assert_eq!(vt.machine_type(), machine_type(k), "{k:?}");
  }
  OK
}
