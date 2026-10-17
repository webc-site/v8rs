//! Trusted 族规约测试 (src/trusted.rs, 独立 test bin)。
//!
//! 规约来源 (实测: v8/test/unittests 下无 trusted-object-unittest.cc /
//! trusted-pointer-unittest.cc — `find v8/test -iname '*trusted*'` 仅命中
//! inspector/wasm 无关文件, 原版无直接 unittest, 按票单约定以
//! 布局/判别值锚点测试兜底):
//! - trusted-object.h:68 / :173 — kHeaderSize = sizeof(HeapObject) /
//!   sizeof(ExposedTrustedObject) (非沙箱无附加字段 → 两者相等)
//! - trusted-object.tq — 两抽象类零字段 / self_indirect_pointer 仅沙箱
//! - trusted-object.h:31-61 / trusted-object-inl.h:24-84 — 受保护字段
//!   访问器全集 (R2/W2/Empty2/Clear2/Raw2), empty/clear 锚 Smi::zero
//! - instance-type-inl.h — IsTrustedObject [175,202] /
//!   IsExposedTrustedObject [175,192] 判别区间逐边界
//! - trusted-pointer-inl.h:152-251 — 委托链 load→Acquire_Load /
//!   load_maybe_empty→Acquire_Load_maybe_empty / store→Release_Store,
//!   非沙箱 is_unpublished 恒 false, IsPublished 恒 true (object-inl:175)
//! - trusted-pointer.h:40-57 — X-macro TRUSTED_POINTER_TAG_TO_TYPE_MAP
//!   14 项逐值 (顺序一致)
//!
//! 跳过登记 (推迟, 非 objects L3):
//! - ExposedTrustedObject InitAndPublish/InitDontPublish/Publish/Unpublish/
//!   InitSelfIndirectPointerField/self_indirect_pointer_handle → TPT 分配
//!   (v8-sandbox/handles)
//! - VerifyProtectedPointerField → VERIFY_HEAP (heap 层)
//! - AllocationWitness ctor x4 / storage_address (sandbox-only)
//! - TrustedTypeFor 模板解析 (Code/BytecodeArray 具体类型属 codegen 层)

pub mod common;

use std::mem;

use aok::{OK, Void};
use common::Arena;
use v8_objects_arrays::trusted::{
  ExposedTrustedObject, K_EXPOSED_HEADER_SIZE, K_HEADER_SIZE, TRUSTED_POINTER_TAG_TO_TYPE,
  TrustedObject, TrustedObjectExt, TrustedPointerMember, clear_trusted_pointer_field,
  is_trusted_pointer_field_empty, is_trusted_pointer_field_unpublished,
  read_maybe_empty_trusted_pointer_field, read_trusted_pointer_field, write_trusted_pointer_field,
};
use v8_objects_core::{
  heap_object::HeapObject,
  instance_type::InstanceType,
  objects::{Cast, WriteBarrierMode, is},
  smi::Smi,
  tagged::{Object, Tagged},
  tagged_field::TaggedSlot,
};

/// 槽位步长: 逻辑布局 Tagged 槽 usize 宽 (非压缩 API 常量 4)。
const SLOT: usize = mem::size_of::<TaggedSlot>();
/// 受保护字段槽偏移: 头后首槽 (map 槽 0 禁走字段访问器)。
const FIELD_0: usize = K_HEADER_SIZE;
/// 第二槽偏移。
const FIELD_1: usize = K_HEADER_SIZE + SLOT;

/// 布局锚点: kHeaderSize = sizeof(HeapObject) (trusted-object.h:68),
/// ExposedTrustedObject 非沙箱零附加字段 → 两常量相等, 字段偏移归零。
#[test]
fn trusted_object_layout_anchors() -> Void {
  assert_eq!(K_HEADER_SIZE, mem::size_of::<HeapObject>());
  assert_eq!(K_HEADER_SIZE, mem::size_of::<TrustedObject>());
  assert_eq!(
    K_EXPOSED_HEADER_SIZE,
    mem::size_of::<ExposedTrustedObject>()
  );
  assert_eq!(K_EXPOSED_HEADER_SIZE, K_HEADER_SIZE);
  assert_eq!(
    mem::offset_of!(TrustedObject, header),
    HeapObject::MAP_OFFSET
  );
  assert_eq!(mem::offset_of!(ExposedTrustedObject, trusted), 0);
  assert_eq!(mem::offset_of!(TrustedObject, header.map), 0);
  OK
}

/// 判别值锚点: Cast::allow_from 走 IsTrustedObject [175,202] /
/// IsExposedTrustedObject [175,192] 逐边界 (instance-type-inl.h)。
#[test]
fn cast_trusted_object_allow_from() -> Void {
  let mut arena = Arena::new();
  let cases: &[bool] = &[
    false, false, // 174 WasmExceptionTag — 区间外
    true, true, // 175 下边界 (exposed 区间起)
    true, true, // 185 BytecodeArray — exposed 成员
    true, true, // 192 WasmTrustedInstanceData — exposed 上边界
    true, false, // 193 ProtectedFixedArray — trusted-only (exposed 区间外)
    true, false, // 202 WasmImportData — trusted 上边界
    false, false, // 203 AccessorInfo — trusted 区间外
    false, false, // 205 FixedArray — 常规对象
  ];
  let types = [
    InstanceType::WasmExceptionTag,
    InstanceType::UncompiledDataWithPreparseData,
    InstanceType::BytecodeArray,
    InstanceType::WasmTrustedInstanceData,
    InstanceType::ProtectedFixedArray,
    InstanceType::WasmImportData,
    InstanceType::AccessorInfo,
    InstanceType::FixedArray,
  ];
  for (i, ty) in types.iter().enumerate() {
    let map = arena.map(*ty);
    let obj = arena.alloc_with(TrustedObject::new(map), 0);
    let raw: Tagged<Object> = obj.cast();
    assert_eq!(
      TrustedObject::allow_from(raw),
      cases[i * 2],
      "trusted @ {ty:?}"
    );
    assert_eq!(
      ExposedTrustedObject::allow_from(raw),
      cases[i * 2 + 1],
      "exposed @ {ty:?}"
    );
    assert_eq!(
      is::<TrustedObject>(raw),
      cases[i * 2],
      "is::<Trusted> @ {ty:?}"
    );
  }
  OK
}

/// TrustedObjectExt 受保护字段访问器全集往返 (trusted-object-inl.h:24-78):
/// write↔read、acquire/release 变体一致、empty/clear 锚 Smi::zero、
/// Raw 槽指针与 location 同址 (Raw x2 含 MaybeObject 形)。
#[test]
fn protected_pointer_field_roundtrip() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::BytecodeArray);
  let host = arena.alloc_with(TrustedObject::new(map), 2 * SLOT);
  let target = arena.alloc_with(TrustedObject::new(map), 0);
  let target2 = arena.alloc_with(TrustedObject::new(map), 0);
  assert_ne!(target, target2);

  // 零填充槽 = Smi::zero 形态 (SMI_TAG 0) → 空
  // SAFETY: host 指针有效且偏移处于分配尺寸内
  assert!(unsafe { host.is_protected_pointer_field_empty(FIELD_0) });
  // SAFETY: host 指针有效且偏移处于分配尺寸内
  assert!(unsafe { host.is_protected_pointer_field_empty_acquire(FIELD_0) });

  // SAFETY: host 指针有效且偏移处于分配尺寸内
  unsafe {
    // 普通写 → 两读变体同值
    host.write_protected_pointer_field(FIELD_0, target);
    assert_eq!(
      host.read_protected_pointer_field::<TrustedObject>(FIELD_0),
      target
    );
    assert_eq!(
      host.read_protected_pointer_field_acquire::<TrustedObject>(FIELD_0),
      target
    );
    assert!(!host.is_protected_pointer_field_empty(FIELD_0));
    assert!(!host.is_protected_pointer_field_empty_acquire(FIELD_0));

    // release 写 → 普通读同值 (非沙箱两态同构)
    host.write_protected_pointer_field_release(FIELD_1, target2);
    assert_eq!(
      host.read_protected_pointer_field::<TrustedObject>(FIELD_1),
      target2
    );

    // Raw 槽: 地址锚 host.address()+offset, 值锚 tagged ptr;
    // MaybeObject 形与 Pointer 形同址 (C++ 共 field_address)
    let raw = host.raw_protected_pointer_field(FIELD_0);
    assert_eq!(raw as usize, host.address() + FIELD_0);
    assert_eq!(*raw, target.ptr());
    assert_eq!(
      host.raw_protected_maybe_object_field(FIELD_0),
      host.raw_protected_pointer_field(FIELD_0)
    );

    // clear → Smi::zero 空形态; raw 读出 == Smi::zero().ptr()
    host.clear_protected_pointer_field(FIELD_0);
    assert!(host.is_protected_pointer_field_empty(FIELD_0));
    assert_eq!(
      *host.raw_protected_pointer_field(FIELD_0),
      Smi::zero().ptr()
    );
    host.clear_protected_pointer_field_release(FIELD_1);
    assert!(host.is_protected_pointer_field_empty_acquire(FIELD_1));
  }
  OK
}

/// TrustedPointerField 静态访问器组 (trusted-pointer-inl.h:23-119 非沙箱):
/// write→read 往返、maybe-empty 空槽 Smi 形态、empty 走 IsSmi、
/// clear 锚 Smi::zero、unpublished 非沙箱恒 false。
#[test]
fn trusted_pointer_field_ops() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::BytecodeArray);
  let host: Tagged<HeapObject> = arena.alloc_with(HeapObject::new(map), 2 * SLOT);
  let value = arena.alloc_with(ExposedTrustedObject::new(map), 0);

  // 空槽 (零填充) → is_smi 空形态
  // SAFETY: host 指针有效且偏移处于分配尺寸内
  assert!(unsafe { is_trusted_pointer_field_empty(host, FIELD_0) });
  // SAFETY: host 指针有效且偏移处于分配尺寸内
  assert!(unsafe { read_maybe_empty_trusted_pointer_field(host, FIELD_0) }.is_smi());
  assert!(!is_trusted_pointer_field_unpublished());

  // SAFETY: host 指针有效且偏移处于分配尺寸内
  unsafe {
    write_trusted_pointer_field(host, FIELD_0, value);
    assert_eq!(
      read_trusted_pointer_field::<ExposedTrustedObject>(host, FIELD_0),
      value
    );
    assert_eq!(
      read_maybe_empty_trusted_pointer_field(host, FIELD_0),
      value.cast()
    );
    assert!(!is_trusted_pointer_field_empty(host, FIELD_0));

    clear_trusted_pointer_field(host, FIELD_0);
    assert!(is_trusted_pointer_field_empty(host, FIELD_0));
    assert_eq!(
      read_maybe_empty_trusted_pointer_field(host, FIELD_0).ptr(),
      Smi::zero().ptr()
    );
    assert!(!is_trusted_pointer_field_unpublished());
  }
  OK
}

/// TrustedPointerMember 委托链与语义 (trusted-pointer-inl.h:124-251 非沙箱):
/// load→acquire_load / load_maybe_empty→acquire_load_maybe_empty /
/// store→release_store / store_no_write_barrier→release 无屏障形;
/// new/Default 空态、is_empty/is_unpublished/clear、Debug 形态。
#[test]
fn trusted_pointer_member_semantics() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::BytecodeArray);
  let host = HeapObject::new(map);
  let target = arena.alloc_with(TrustedObject::new(map), 0);
  let target2 = arena.alloc_with(TrustedObject::new(map), 0);

  let member: TrustedPointerMember<TrustedObject> = TrustedPointerMember::new();
  assert!(member.is_empty());
  assert!(!member.is_unpublished());
  assert_eq!(format!("{member:?}"), "TrustedPointerMember(0x0)");
  let defaulted: TrustedPointerMember<TrustedObject> = Default::default();
  assert_eq!(defaulted.load(), member.load());

  // store 委托 release_store: 写后各读变体同值, maybe_empty 同址
  member.store(&host, target, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(member.load(), target);
  assert_eq!(member.acquire_load(), target);
  assert_eq!(member.load_maybe_empty(), member.acquire_load_maybe_empty());
  assert_eq!(member.acquire_load_maybe_empty().ptr(), target.ptr());
  assert_eq!(
    format!("{member:?}"),
    format!("TrustedPointerMember(0x{:x})", target.ptr())
  );

  member.release_store(&host, target2, WriteBarrierMode::UpdateWriteBarrier);
  assert_eq!(member.load(), target2);
  member.store_no_write_barrier(&host, target);
  assert_eq!(member.acquire_load(), target);
  member.release_store_no_write_barrier(&host, target2);
  assert_eq!(member.load(), target2);
  assert!(!member.is_empty());

  // clear → release 空写
  member.clear(&host);
  assert!(member.is_empty());
  assert!(member.load().is_null());
  assert!(!member.is_unpublished());
  OK
}

/// X-macro TRUSTED_POINTER_TAG_TO_TYPE_MAP 机械展开逐值锚 (trusted-pointer.h:40-57):
/// 14 项, 顺序与原文一致, tag→类型名逐对比对。
#[test]
fn trusted_pointer_tag_to_type_map_values() -> Void {
  const EXPECTED: &[(&str, &str)] = &[
    ("kCodeIndirectPointerTag", "Code"),
    ("kBytecodeArrayIndirectPointerTag", "BytecodeArray"),
    ("kInterpreterDataIndirectPointerTag", "InterpreterData"),
    ("kUncompiledDataIndirectPointerTag", "UncompiledData"),
    ("kRegExpDataIndirectPointerTag", "RegExpData"),
    ("kWasmDispatchTableIndirectPointerTag", "WasmDispatchTable"),
    (
      "kSharedWasmDispatchTableIndirectPointerTag",
      "WasmDispatchTable",
    ),
    (
      "kWasmTrustedInstanceDataIndirectPointerTag",
      "WasmTrustedInstanceData",
    ),
    (
      "kSharedWasmTrustedInstanceDataIndirectPointerTag",
      "WasmTrustedInstanceData",
    ),
    (
      "kWasmInternalFunctionIndirectPointerTag",
      "WasmInternalFunction",
    ),
    ("kWasmSuspenderIndirectPointerTag", "WasmSuspenderObject"),
    (
      "kWasmFunctionDataIndirectPointerTagRange",
      "WasmFunctionData",
    ),
    (
      "kWasmExportedFunctionDataIndirectPointerTag",
      "WasmExportedFunctionData",
    ),
    (
      "kWasmCapiFunctionDataIndirectPointerTag",
      "WasmCapiFunctionData",
    ),
  ];
  assert_eq!(TRUSTED_POINTER_TAG_TO_TYPE.len(), 14);
  for (i, (tag, ty)) in TRUSTED_POINTER_TAG_TO_TYPE.iter().enumerate() {
    assert_eq!((*tag, *ty), EXPECTED[i], "映射 [{i}] 漂移");
  }
  // 默认形锚: 表外 tag 回退 ExposedTrustedObject (TrustedPointerType 默认特化)
  let covered = TRUSTED_POINTER_TAG_TO_TYPE
    .iter()
    .filter(|entry| entry.1 == "ExposedTrustedObject")
    .count();
  assert_eq!(covered, 0);
  OK
}

/// IsPublished 非沙箱恒 true (trusted-object-inl.h:166-177 #else 分支)。
#[test]
fn exposed_trusted_object_is_published() -> Void {
  let mut arena = Arena::new();
  let map = arena.map(InstanceType::BytecodeArray);
  let obj = arena.alloc_with(ExposedTrustedObject::new(map), 0);
  // SAFETY: obj 指向 arena 已初始化的 ExposedTrustedObject
  let r = unsafe { obj.as_ref() };
  assert!(r.is_published());
  assert_eq!(r.trusted.header.map, map);
  OK
}
