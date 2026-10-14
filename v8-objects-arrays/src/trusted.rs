// tool/src/objects/trusted-object.h.js
// 来源: v8/src/objects/trusted-object.h + trusted-object-inl.h + trusted-object.tq
//       + trusted-pointer.h + trusted-pointer-inl.h (非沙箱默认配置分支)
//! trusted 对象族: [TrustedObject] / [ExposedTrustedObject] 头 +
//! 受保护指针槽 + trusted 指针成员。
//!
//! 语义: trusted 对象位于沙箱外 (trusted space), 不受攻击者篡改;
//! 沙箱启用时经间接指针 (TPT 句柄) 引用, 非沙箱 (默认) 配置退化为常规
//! tagged 指针 — 本模块只落非沙箱分支。
//!
//! 推迟登记 (跨层, 非本族 L3):
//! - ExposedTrustedObject InitAndPublish/InitDontPublish/Publish/Unpublish/
//!   InitSelfIndirectPointerField/self_indirect_pointer_handle → TPT 表项分配
//!   与 Isolate 句柄族 (v8-sandbox/handles);
//! - VerifyProtectedPointerField → VERIFY_HEAP heap 校验 (heap 层);
//! - TrustedPointerMember 4 个 AllocationWitness ctor (witness 未移植);
//! - storage_address (sandbox-only 私有成员, 非沙箱形态不存在);
//! - CodePointerMember 别名与 TrustedTypeFor 模板解析 → Code/BytecodeArray
//!   等具体类型属上游 codegen 层, 本模块以 tag→类型名对照表承接。

use std::{fmt, mem::size_of};

use v8_objects_core::{
  heap_object::HeapObject,
  instance_type_checker as itc,
  map::Map,
  objects::{Cast, WriteBarrierMode, instance_type_of},
  smi::Smi,
  tagged::{Object, Tagged},
  tagged_field::{TaggedField, TaggedMember, TaggedSlot},
};

use crate::fixed_array_base::write_barrier_stub;

// ── 布局常量 (snapshot) ──────────────────────────────────────────────────
/// TrustedObject 头大小 = sizeof(HeapObject) (trusted-object.h:68)。
///
/// Rust 侧为全尺寸 (usize 槽) 逻辑布局, 头大小随 HeapObject 结构体实际
/// 尺寸; 不可用 v8_common::K_TAGGED_SIZE (=4, 压缩形态 API 常量) —
/// 否则字段偏移落 4 对齐, 读 8 宽 TaggedSlot 触发未对齐访问。
pub const K_HEADER_SIZE: usize = size_of::<HeapObject>();
/// ExposedTrustedObject 头大小 (非沙箱 = sizeof(TrustedObject);
/// 沙箱配置加 self_indirect_pointer_ 句柄 → v8-sandbox)。
pub const K_EXPOSED_HEADER_SIZE: usize = K_HEADER_SIZE;

// ── tag→类型对照表 (trusted-pointer.h:40-57 X-macro 机械展开, 14 项) ──────
/// TRUSTED_POINTER_TAG_TO_TYPE_MAP 展开表 (顺序与原文一致)。
///
/// C++ detail::TrustedPointerType 模板特化的数据化形态: 供间接指针 tag
/// 反查目标类型名 (类型本体 Code/BytecodeArray/Wasm* 属上游 codegen 层,
/// 本 crate 只落对照表; 默认形 `using type = ExposedTrustedObject` 对应
/// 表外 tag 的回退语义)。
pub const TRUSTED_POINTER_TAG_TO_TYPE: &[(&str, &str)] = &[
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

/// 受信对象 (trusted-object.h:31): 不受沙箱内攻击者篡改的对象头标记。
///
/// 典型成员: 字节码 / 代码元数据容器 — 被篡改时可被用于逃逸沙箱,
/// 故置于沙箱外; 不可被非受信对象直接引用 (ExposedTrustedObject 例外,
/// 经间接指针)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TrustedObject {
  /// HeapObject 头 (TrustedObject 不加字段)。
  pub header: HeapObject,
}

impl TrustedObject {
  /// 构造 (仅 map 头)。
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self {
      header: HeapObject::new(map),
    }
  }
}

/// 可被非受信对象引用的受信对象 (trusted-object.h:103)。
///
/// 沙箱配置下持 self 间接指针句柄 (一对象一条目, 非一引用一条目,
/// 防替换攻击); 非沙箱默认配置无额外字段。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ExposedTrustedObject {
  /// TrustedObject 头。
  pub trusted: TrustedObject,
}

impl ExposedTrustedObject {
  /// 构造 (非沙箱形态)。
  #[inline]
  pub const fn new(map: Tagged<Map>) -> Self {
    Self {
      trusted: TrustedObject::new(map),
    }
  }

  /// IsPublished(isolate) (trusted-object-inl.h:166-177)。
  ///
  /// 非沙箱分支恒 true (直接指针引用, 无 TPT 发布态);
  /// 沙箱分支经 IsTrustedPointerFieldUnpublished → v8-sandbox 推迟。
  #[inline]
  pub const fn is_published(&self) -> bool {
    true
  }
}

// ── Cast (casting.h DEF_CAST_TRAITS 对应) ────────────────────────────────
impl Cast for TrustedObject {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_trusted_object)
  }
}

impl Cast for ExposedTrustedObject {
  #[inline]
  fn allow_from(value: Tagged<Object>) -> bool {
    instance_type_of(value).is_some_and(itc::is_exposed_trusted_object)
  }
}

// ── 受保护指针槽 (trusted-object-inl.h:25-78) ─────────────────────────────
/// [Tagged<TrustedObject>] 受保护指针槽语义。
///
/// 受保护指针 = trusted space 内对象间指针 (指针与所指对象均不可被
/// 沙箱内攻击者篡改); 非沙箱下与普通 tagged 槽同构。
pub trait TrustedObjectExt: Copy {
  /// ReadProtectedPointerField<T>(offset) (plain load)。
  ///
  /// # Safety
  /// offset 须落在宿主对象分配范围内的 tagged 对齐槽位。
  #[inline]
  unsafe fn read_protected_pointer_field<T>(&self, offset: usize) -> Tagged<T> {
    // SAFETY: 调用方契约; 非压缩 TaggedField 恒等转换
    unsafe { TaggedField::<T, 0>::load(self.as_heap_object(), offset) }
  }

  /// ReadProtectedPointerField<T>(offset, kAcquireLoad)。
  ///
  /// # Safety
  /// 同 [Self::read_protected_pointer_field]。
  #[inline]
  unsafe fn read_protected_pointer_field_acquire<T>(&self, offset: usize) -> Tagged<T> {
    // SAFETY: 调用方契约
    unsafe { TaggedField::<T, 0>::acquire_load(self.as_heap_object(), offset) }
  }

  /// WriteProtectedPointerField(offset, value)。
  ///
  /// # Safety
  /// 同 [Self::read_protected_pointer_field]; 无并发别名。
  #[inline]
  unsafe fn write_protected_pointer_field(&self, offset: usize, value: Tagged<TrustedObject>) {
    // SAFETY: 调用方契约; 写屏障桩 (TRUSTED_TO_SHARED_TRUSTED 条目归 heap 层)
    unsafe { TaggedField::<TrustedObject, 0>::store_at(self.as_heap_object(), offset, value) };
    write_barrier_stub(WriteBarrierMode::UpdateWriteBarrier);
  }

  /// WriteProtectedPointerField(offset, value, kReleaseStore)。
  ///
  /// # Safety
  /// 同 [Self::read_protected_pointer_field]。
  #[inline]
  unsafe fn write_protected_pointer_field_release(
    &self,
    offset: usize,
    value: Tagged<TrustedObject>,
  ) {
    // SAFETY: 调用方契约
    unsafe {
      TaggedField::<TrustedObject, 0>::release_store_at(self.as_heap_object(), offset, value)
    };
    write_barrier_stub(WriteBarrierMode::UpdateWriteBarrier);
  }

  /// IsProtectedPointerFieldEmpty(offset) (== Smi::zero)。
  ///
  /// # Safety
  /// 同 [Self::read_protected_pointer_field]。
  #[inline]
  unsafe fn is_protected_pointer_field_empty(&self, offset: usize) -> bool {
    // SAFETY: 调用方契约
    unsafe {
      TaggedField::<Object, 0>::acquire_load(self.as_heap_object(), offset).ptr()
        == Smi::zero().ptr()
    }
  }

  /// IsProtectedPointerFieldEmpty(offset, kAcquireLoad)。
  ///
  /// # Safety
  /// 同 [Self::read_protected_pointer_field]。
  #[inline]
  unsafe fn is_protected_pointer_field_empty_acquire(&self, offset: usize) -> bool {
    // SAFETY: 同上 (acquire 语义即本态)
    unsafe { self.is_protected_pointer_field_empty(offset) }
  }

  /// ClearProtectedPointerField(offset) (写 Smi::zero)。
  ///
  /// # Safety
  /// 同 [Self::read_protected_pointer_field]; 无并发别名。
  #[inline]
  unsafe fn clear_protected_pointer_field(&self, offset: usize) {
    // SAFETY: 调用方契约 (原版 plain → Release 变体委托)
    unsafe { self.clear_protected_pointer_field_release(offset) };
  }

  /// ClearProtectedPointerField(offset, kReleaseStore)。
  ///
  /// # Safety
  /// 同 [Self::read_protected_pointer_field]。
  #[inline]
  unsafe fn clear_protected_pointer_field_release(&self, offset: usize) {
    // SAFETY: 调用方契约
    unsafe { TaggedField::<Smi, 0>::release_store_at(self.as_heap_object(), offset, Smi::zero()) };
  }

  /// RawProtectedPointerField(byte_offset) → 槽位裸指针。
  ///
  /// # Safety
  /// 同 [Self::read_protected_pointer_field]。
  #[inline]
  unsafe fn raw_protected_pointer_field(&self, byte_offset: usize) -> *mut TaggedSlot {
    // SAFETY: 调用方契约
    unsafe { TaggedField::<Object, 0>::location(self.as_heap_object(), byte_offset) }
  }

  /// RawProtectedMaybeObjectField(byte_offset) → 槽位裸指针 (trusted-object.h:60)。
  ///
  /// 与 [Self::raw_protected_pointer_field] 同址, C++ 槽类型区分
  /// (ProtectedMaybeObjectSlot) 在 Rust 统一为 [TaggedSlot] 视图。
  ///
  /// # Safety
  /// 同 [Self::read_protected_pointer_field]。
  #[inline]
  unsafe fn raw_protected_maybe_object_field(&self, byte_offset: usize) -> *mut TaggedSlot {
    // SAFETY: 调用方契约 (与 RawProtectedPointerField 共用 field_address)
    unsafe { TaggedField::<Object, 0>::location(self.as_heap_object(), byte_offset) }
  }

  /// 本对象转 HeapObject tagged 视图 (单继承 → 零偏移)。
  fn as_heap_object(&self) -> Tagged<HeapObject>;
}

impl TrustedObjectExt for Tagged<TrustedObject> {
  #[inline]
  fn as_heap_object(&self) -> Tagged<HeapObject> {
    self.cast()
  }
}

// ── TrustedPointerField 静态访问器组 (trusted-pointer-inl.h:24-119 非沙箱) ──
/// ReadTrustedPointerField (acquire 语义; 非沙箱 = tagged 槽 acquire 读)。
///
/// # Safety
/// offset 须落在宿主对象分配范围内的 tagged 对齐槽位。
#[inline]
pub unsafe fn read_trusted_pointer_field<T>(host: Tagged<HeapObject>, offset: usize) -> Tagged<T> {
  // SAFETY: 调用方契约
  unsafe { TaggedField::<T, 0>::acquire_load(host, offset) }
}

/// ReadMaybeEmptyTrustedPointerField (空槽 = Smi, 原样返回)。
///
/// # Safety
/// 同 [read_trusted_pointer_field]。
#[inline]
pub unsafe fn read_maybe_empty_trusted_pointer_field(
  host: Tagged<HeapObject>,
  offset: usize,
) -> Tagged<Object> {
  // SAFETY: 调用方契约; 空槽语义由调用方判 is_smi
  unsafe { TaggedField::<Object, 0>::acquire_load(host, offset) }
}

/// WriteTrustedPointerField (release 语义)。
///
/// # Safety
/// 同 [read_trusted_pointer_field]; 无并发别名。
#[inline]
pub unsafe fn write_trusted_pointer_field(
  host: Tagged<HeapObject>,
  offset: usize,
  value: Tagged<ExposedTrustedObject>,
) {
  // SAFETY: 调用方契约
  unsafe { TaggedField::<ExposedTrustedObject, 0>::release_store_at(host, offset, value) };
  write_barrier_stub(WriteBarrierMode::UpdateWriteBarrier);
}

/// IsTrustedPointerFieldEmpty (非沙箱 = acquire 读出值为 Smi)。
///
/// # Safety
/// 同 [read_trusted_pointer_field]。
#[inline]
pub unsafe fn is_trusted_pointer_field_empty(host: Tagged<HeapObject>, offset: usize) -> bool {
  // SAFETY: 调用方契约
  unsafe { TaggedField::<Object, 0>::acquire_load(host, offset) }.is_smi()
}

/// IsTrustedPointerFieldUnpublished (非沙箱恒 false)。
#[inline]
#[must_use]
pub const fn is_trusted_pointer_field_unpublished() -> bool {
  false
}

/// ClearTrustedPointerField (release 写 Smi::zero)。
///
/// # Safety
/// 同 [read_trusted_pointer_field]; 无并发别名。
#[inline]
pub unsafe fn clear_trusted_pointer_field(host: Tagged<HeapObject>, offset: usize) {
  // SAFETY: 调用方契约
  unsafe { TaggedField::<Smi, 0>::release_store_at(host, offset, Smi::zero()) };
}

// ── TrustedPointerMember (trusted-pointer.h:137-184 非沙箱形态) ────────────
/// trusted 指针成员 (非沙箱: TaggedMember<T> + host 感知屏障参数;
/// 沙箱形态为 TPT 句柄 → v8-sandbox)。
///
/// 读恒 acquire / 写恒 release (对齐间接指针底层语义)。
#[repr(transparent)]
pub struct TrustedPointerMember<T> {
  slot: TaggedMember<T>,
}

impl<T> fmt::Debug for TrustedPointerMember<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "TrustedPointerMember(0x{:x})",
      self.slot.acquire_load().ptr()
    )
  }
}

impl<T> Default for TrustedPointerMember<T> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl<T> TrustedPointerMember<T> {
  /// 构造空槽
  #[inline]
  pub const fn new() -> Self {
    Self {
      slot: TaggedMember::from_raw(0),
    }
  }

  /// load() (acquire)。
  #[inline]
  pub fn load(&self) -> Tagged<T> {
    self.acquire_load()
  }

  /// load_maybe_empty() (空槽返回 Smi::zero 语义的 Object 视图)。
  #[inline]
  pub fn load_maybe_empty(&self) -> Tagged<Object> {
    self.acquire_load_maybe_empty()
  }

  /// store(host, value, mode) (release + 写屏障桩)。
  ///
  /// host 供屏障定位宿主 (heap 层接入后使用)。
  ///
  #[inline]
  pub fn store(&self, _host: &HeapObject, value: Tagged<T>, mode: WriteBarrierMode) {
    self.slot.release_store(value, mode);
  }

  /// store_no_write_barrier(host, value) (release, 免屏障)。
  #[inline]
  pub fn store_no_write_barrier(&self, _host: &HeapObject, value: Tagged<T>) {
    self.slot.release_store_no_write_barrier(value);
  }

  /// Acquire_Load()。
  #[inline]
  pub fn acquire_load(&self) -> Tagged<T> {
    self.slot.acquire_load()
  }

  /// Acquire_Load_maybe_empty() (trusted-pointer.h:164)。
  ///
  /// 非沙箱: 与 [Self::acquire_load] 同形 (沙箱分支空句柄 → Smi::zero,
  /// 归 v8-sandbox 推迟)。
  #[inline]
  pub fn acquire_load_maybe_empty(&self) -> Tagged<Object> {
    self.slot.acquire_load().cast()
  }

  /// Release_Store(host, value, mode)。
  #[inline]
  pub fn release_store(&self, host: &HeapObject, value: Tagged<T>, mode: WriteBarrierMode) {
    self.store(host, value, mode);
  }

  /// Release_Store_no_write_barrier(host, value)。
  #[inline]
  pub fn release_store_no_write_barrier(&self, host: &HeapObject, value: Tagged<T>) {
    self.store_no_write_barrier(host, value);
  }

  /// is_empty() (acquire 读为 null)。
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.acquire_load().is_null()
  }

  /// is_unpublished() (非沙箱恒 false)。
  #[inline]
  #[must_use]
  pub const fn is_unpublished(&self) -> bool {
    false
  }

  /// clear(host) (release 写 null)。
  #[inline]
  pub fn clear(&self, _host: &HeapObject) {
    self.slot.release_store_no_write_barrier(Tagged::NULL);
  }
}
