#!/usr/bin/env bun
// tool/src/handles/maybe-handles.js — maybe-handles.h + maybe-handles-inl.h
// → v8-handles/src/maybe_handles.rs
// MaybeHandle/MaybeDirectHandle/MaybeObjectHandle/MaybeObjectDirectHandle。
// 惯用法映射: bool+out 参数 → Option; CHECK 致命转换 → Result (错误集中 error.rs)。
import { readV8, writeSrc } from "../../lib.js";

const h = readV8("src/handles/maybe-handles.h");
const inl = readV8("src/handles/maybe-handles-inl.h");

// ---- 1. 枚举对数: HeapObjectReferenceType (src/common/globals.h) ----
const globalsH = readV8("src/common/globals.h");
const em = globalsH.match(/enum class V8_EXPORT_ENUM HeapObjectReferenceType \{([\s\S]*?)\};/);
if (!em) throw new Error("HeapObjectReferenceType 枚举未找到");
const variants = em[1].split(",").map(s => s.trim()).filter(Boolean);
const expectVariants = ["WEAK", "STRONG"];
if (JSON.stringify(variants) !== JSON.stringify(expectVariants))
  throw new Error(`HeapObjectReferenceType 变体不符: ${variants}`);

// 弱引用 tag (include/v8-internal.h)
const internalH = readV8("include/v8-internal.h");
const weakTag = internalH.match(/const int kWeakHeapObjectTag = (\d+);/);
if (!weakTag) throw new Error("kWeakHeapObjectTag 未找到");
const refTagMask = internalH.match(/const intptr_t kHeapObjectReferenceTagMask = \(1 << \(kHeapObjectTagSize - 1\)\);/) ||
  internalH.match(/kHeapObjectReferenceTagMask = [^;]+;/);
if (!refTagMask) throw new Error("kHeapObjectReferenceTagMask 未找到");

// ---- 2. API 面对数 ----
const maybeApi = [
  "MaybeHandle() : HandleBase(nullptr)", "MaybeHandle(NullMaybeHandleType)", "is_identical_to",
  "ToHandleChecked() const", "bool ToHandle(Handle<S>* out)", "bool To(DirectHandle<S>* out)",
  "IsEmpty() const", "bool equals(MaybeHandle<T> other)",
];
for (const sig of maybeApi) if (!(h + inl).includes(sig)) throw new Error(`MaybeHandle ${sig} 未命中`);

const objApi = [
  "MaybeObjectHandle()", "static inline MaybeObjectHandle Weak(Tagged<Object> object, Isolate* isolate)",
  "static inline MaybeObjectHandle Weak(Handle<Object> object)", "reference_type() const",
  "IndirectHandle<Object> object() const", "is_identical_to(const MaybeObjectHandle& other)",
];
for (const sig of objApi) if (!(h + inl).includes(sig)) throw new Error(`MaybeObjectHandle ${sig} 未命中`);

// ---- 3. 生成 Rust 初版 ----
const body = `//! 由 tool/src/handles/maybe-handles.js 从 v8/src/handles/maybe-handles{.h,-inl.h} 生成
//! 惯用法映射: ToHandle 的 bool+out 参数 → Option<Handle<T>>; ToHandleChecked 的 CHECK
//! 致命语义 → Result (错误集中 src/error.rs); NullMaybeHandleType 哨兵类型 → null() 构造。
use std::marker::PhantomData;

use crate::error::{Error, Result};
use crate::handles::{Handle, HandleScope, Object, SlotPtr, Tagged};
use v8_common::{Address, K_HEAP_OBJECT_TAG_MASK, K_WEAK_HEAP_OBJECT_TAG};

/// 引用强度 (原 enum class HeapObjectReferenceType, src/common/globals.h)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HeapObjectReferenceType {
  /// 弱引用 (原 WEAK)
  Weak,
  /// 强引用 (原 STRONG)
  Strong,
}

/// 由强 tagged 指针造弱引用 (原 MakeWeak: 低位 tag 置 3)
pub const fn make_weak(ptr: Address) -> Address {
  (ptr & !K_HEAP_OBJECT_TAG_MASK) | K_WEAK_HEAP_OBJECT_TAG
}

/// 是否弱引用 (kHeapObjectReferenceTagMask 低位判定)
pub const fn is_weak(ptr: Address) -> bool {
  ptr & K_HEAP_OBJECT_TAG_MASK == K_WEAK_HEAP_OBJECT_TAG
}

/// 可空句柄 (原 template<class T> class MaybeHandle final : HandleBase):
/// Handle → MaybeHandle 单向转换, 反向必须做空检查 —— 强制使用前校验 (原注释)。
/// 刻意不提供默认相等/散列 (语义歧义同 Handle, 原注释)。
#[derive(Copy, Clone)]
pub struct MaybeHandle<T = Object> {
  location: SlotPtr,
  _m: PhantomData<fn() -> T>,
}

impl<T> MaybeHandle<T> {
  /// 空 (原 MaybeHandle() / kNullMaybeHandle)
  pub const fn null() -> Self {
    Self { location: std::ptr::null_mut(), _m: PhantomData }
  }

  /// 由句柄上转 (原 MaybeHandle(Handle<S>) 隐式上转)
  pub const fn from_handle(handle: Handle<T>) -> Self {
    Self { location: handle.location(), _m: PhantomData }
  }

  /// 由槽位构造 (原 private explicit MaybeHandle(Address*), 供 UncheckedCast)
  pub const fn from_location(location: SlotPtr) -> Self {
    Self { location, _m: PhantomData }
  }

  /// 是否为空 (原 is_null / IsEmpty)
  pub fn is_null(&self) -> bool {
    self.location.is_null()
  }

  /// 原 IsEmpty()
  pub fn is_empty(&self) -> bool {
    self.is_null()
  }

  /// debug 断言非空 (原 Assert)
  pub fn assert(&self) {
    debug_assert!(!self.is_null());
  }

  /// 空/非空调试检查 (原 Check: CHECK 语义收窄为 debug 断言, 致命路径走 to_handle_checked)
  pub fn check(&self) {
    self.assert();
  }

  /// 转句柄, 空则 None (原 bool ToHandle(Handle<S>* out) → Option 惯用法)
  pub fn to_handle(&self) -> Option<Handle<T>> {
    if self.is_null() {
      None
    } else {
      Some(Handle::from_location(self.location))
    }
  }

  /// 转句柄, 空即错误 (原 ToHandleChecked 的 CHECK → Result)
  pub fn to_handle_checked(&self) -> Result<Handle<T>> {
    self.to_handle().ok_or(Error::NullHandle)
  }

  /// 槽位指针 (原 HandleBase::location(): 仅供散列/间接层内部使用)
  pub fn location(&self) -> SlotPtr {
    self.location
  }

  /// location 相等 (原 equals)
  pub fn equals(&self, other: &Self) -> bool {
    self.location as Address == other.location as Address
  }

  /// 无检查类型换型 (原 UncheckedCast(MaybeIndirectHandle<From>))
  pub fn unchecked_cast<U>(self) -> MaybeHandle<U> {
    MaybeHandle::from_location(self.location)
  }
}

impl<T> Default for MaybeHandle<T> {
  fn default() -> Self {
    Self::null()
  }
}

/// 可空直连句柄 (原 !V8_ENABLE_DIRECT_HANDLE 分支: MaybeHandle 的包装)
#[derive(Copy, Clone, Default)]
pub struct MaybeDirectHandle<T = Object> {
  handle: MaybeHandle<T>,
}

impl<T> MaybeDirectHandle<T> {
  /// 空 (原 MaybeDirectHandle() / kNullMaybeHandle)
  pub const fn null() -> Self {
    Self { handle: MaybeHandle::null() }
  }

  /// 由直连句柄构造 (原 MaybeDirectHandle(DirectHandle<S>) 上转)
  pub const fn from_direct(handle: crate::handles::DirectHandle<T>) -> Self {
    Self { handle: MaybeHandle::from_handle(handle.indirect()) }
  }

  /// 由可空句柄构造 (原 MaybeDirectHandle(MaybeIndirectHandle<S>) 上转)
  pub const fn from_maybe(handle: MaybeHandle<T>) -> Self {
    Self { handle }
  }

  /// 分配新可空直连句柄 (原 MaybeDirectHandle(Tagged<T>, Isolate*))
  pub fn new(value: Tagged<T>, scope: &HandleScope) -> Self {
    Self { handle: MaybeHandle::from_handle(Handle::new(value, scope)) }
  }

  /// 是否为空 (原 is_null / IsEmpty)
  pub fn is_null(&self) -> bool {
    self.handle.is_null()
  }

  /// 原 IsEmpty()
  pub fn is_empty(&self) -> bool {
    self.handle.is_empty()
  }

  /// debug 断言非空 (原 Assert)
  pub fn assert(&self) {
    self.handle.assert();
  }

  /// 空/非空调试检查 (原 Check)
  pub fn check(&self) {
    self.handle.check();
  }

  /// 转直连句柄 (原 ToHandle → Option 惯用法)
  pub fn to_handle(&self) -> Option<crate::handles::DirectHandle<T>> {
    self.handle.to_handle().map(crate::handles::DirectHandle::from_handle)
  }

  /// 转直连句柄, 空即错误 (原 ToHandleChecked)
  pub fn to_handle_checked(&self) -> Result<crate::handles::DirectHandle<T>> {
    self.handle.to_handle_checked().map(crate::handles::DirectHandle::from_handle)
  }

  /// location 相等 (原 equals)
  pub fn equals(&self, other: &Self) -> bool {
    self.handle.equals(&other.handle)
  }

  /// 底层可空句柄 (原 indirect_handle(MaybeDirectHandle) 自由函数)
  pub const fn indirect(self) -> MaybeHandle<T> {
    self.handle
  }

  /// 无检查类型换型 (原 UncheckedCast)
  pub fn unchecked_cast<U>(self) -> MaybeDirectHandle<U> {
    MaybeDirectHandle { handle: self.handle.unchecked_cast() }
  }
}

/// 可弱引用句柄 (原 class MaybeObjectHandle): 持强保活, 读出时按 reference_type 复原弱/强
#[derive(Copy, Clone)]
pub struct MaybeObjectHandle {
  /// 引用强度 (原 reference_type_)
  reference_type: HeapObjectReferenceType,
  /// 保活句柄 (原 handle_)
  handle: MaybeHandle<Object>,
}

impl MaybeObjectHandle {
  /// 原默认构造: 强引用 + 空句柄
  pub const fn new() -> Self {
    Self { reference_type: HeapObjectReferenceType::Strong, handle: MaybeHandle::null() }
  }

  /// 强引用构造 (原 MaybeObjectHandle(Tagged<Object>, Isolate*))
  pub fn strong(object: Tagged<Object>, scope: &HandleScope) -> Self {
    Self {
      reference_type: HeapObjectReferenceType::Strong,
      handle: MaybeHandle::from_handle(Handle::new(object, scope)),
    }
  }

  /// 弱引用构造 (原 static Weak(Tagged<Object>, Isolate*)):
  /// 强保活 + 标记弱, 读出时复原弱 tag
  pub fn weak(object: Tagged<Object>, scope: &HandleScope) -> Self {
    Self {
      reference_type: HeapObjectReferenceType::Weak,
      handle: MaybeHandle::from_handle(Handle::new(object, scope)),
    }
  }

  /// 由句柄构造强引用 (原 MaybeObjectHandle(Handle<Object>))
  pub const fn from_handle(object: Handle<Object>) -> Self {
    Self { reference_type: HeapObjectReferenceType::Strong, handle: MaybeHandle::from_handle(object) }
  }

  /// 弱引用版由句柄构造 (原 static Weak(Handle<Object>))
  pub const fn weak_from_handle(object: Handle<Object>) -> Self {
    Self { reference_type: HeapObjectReferenceType::Weak, handle: MaybeHandle::from_handle(object) }
  }

  /// 读出引用值 (原 operator*): 弱则复原弱 tag, 强则原值。
  /// 空句柄读值属调用方违约 (原 ToHandleChecked 的 CHECK → debug_assert)
  pub fn value(&self) -> Tagged<Object> {
    debug_assert!(!self.handle.is_null(), "MaybeObjectHandle 为空");
    // SAFETY: 约定非空, 槽位在作用域存活约定内
    let strong = Handle::<Object>::from_location(self.handle.location());
    let ptr = if self.reference_type == HeapObjectReferenceType::Weak {
      make_weak(strong.value().ptr())
    } else {
      strong.value().ptr()
    };
    Tagged::from_ptr(ptr)
  }

  /// 同 operator* (原 operator->)
  pub fn deref(&self) -> Tagged<Object> {
    self.value()
  }

  /// 保活句柄 (原 object(): ToHandleChecked)
  pub fn object(&self) -> Result<Handle<Object>> {
    self.handle.to_handle_checked()
  }

  /// 引用强度 (原 reference_type())
  pub const fn reference_type(&self) -> HeapObjectReferenceType {
    self.reference_type
  }

  /// 是否为空 (原 is_null)
  pub fn is_null(&self) -> bool {
    self.handle.is_null()
  }

  /// 对象身份相等: 强度一致 + 保活句柄解引用一致 (原 is_identical_to)
  pub fn is_identical_to(&self, other: &Self) -> bool {
    self.reference_type == other.reference_type
      && match (self.handle.to_handle(), other.handle.to_handle()) {
        (Some(a), Some(b)) => a.is_identical_to(&b),
        (None, None) => true,
        _ => false,
      }
  }
}

impl Default for MaybeObjectHandle {
  fn default() -> Self {
    Self::new()
  }
}

/// 可弱引用直连句柄 (原 class MaybeObjectDirectHandle, 非 direct 分支)
#[derive(Copy, Clone, Default)]
pub struct MaybeObjectDirectHandle {
  /// 引用强度 (原 reference_type_)
  reference_type: HeapObjectReferenceType,
  /// 保活直连句柄 (原 handle_)
  handle: MaybeDirectHandle<Object>,
}

impl MaybeObjectDirectHandle {
  /// 原默认构造
  pub fn new() -> Self {
    Self {
      reference_type: HeapObjectReferenceType::Strong,
      handle: MaybeDirectHandle::null(),
    }
  }

  /// 强引用构造 (原 MaybeObjectDirectHandle(Tagged<T>, Isolate*) 强分支)
  pub fn strong(object: Tagged<Object>, scope: &HandleScope) -> Self {
    Self { reference_type: HeapObjectReferenceType::Strong, handle: MaybeDirectHandle::new(object, scope) }
  }

  /// 弱引用构造 (原 static Weak(Tagged<Object>, Isolate*))
  pub fn weak(object: Tagged<Object>, scope: &HandleScope) -> Self {
    Self { reference_type: HeapObjectReferenceType::Weak, handle: MaybeDirectHandle::new(object, scope) }
  }

  /// 由直连句柄构造强引用 (原 MaybeObjectDirectHandle(DirectHandle<Object>))
  pub const fn from_direct(object: crate::handles::DirectHandle<Object>) -> Self {
    Self {
      reference_type: HeapObjectReferenceType::Strong,
      handle: MaybeDirectHandle::from_direct(object),
    }
  }

  /// 弱引用版 (原 static Weak(DirectHandle<Object>))
  pub const fn weak_from_direct(object: crate::handles::DirectHandle<Object>) -> Self {
    Self { reference_type: HeapObjectReferenceType::Weak, handle: MaybeDirectHandle::from_direct(object) }
  }

  /// 由可弱引用句柄转换 (原 MaybeObjectDirectHandle(MaybeObjectHandle))
  pub const fn from_maybe_object(obj: MaybeObjectHandle) -> Self {
    Self {
      reference_type: obj.reference_type(),
      handle: MaybeDirectHandle::from_maybe(obj.indirect()),
    }
  }

  /// 读出引用值 (原 operator*): 空句柄读值属调用方违约 (debug_assert)
  pub fn value(&self) -> Tagged<Object> {
    debug_assert!(!self.handle.is_null(), "MaybeObjectDirectHandle 为空");
    // SAFETY: 约定非空, 槽位在作用域存活约定内
    let strong = Handle::<Object>::from_location(self.handle.indirect().location());
    let ptr = if self.reference_type == HeapObjectReferenceType::Weak {
      make_weak(strong.value().ptr())
    } else {
      strong.value().ptr()
    };
    Tagged::from_ptr(ptr)
  }

  /// 保活直连句柄 (原 object(): handle_.ToHandleChecked())
  pub fn object(&self) -> Result<crate::handles::DirectHandle<Object>> {
    self.handle.to_handle_checked()
  }

  /// 引用强度 (原 reference_type())
  pub const fn reference_type(&self) -> HeapObjectReferenceType {
    self.reference_type
  }

  /// 是否为空 (原 is_null)
  pub fn is_null(&self) -> bool {
    self.handle.is_null()
  }

  /// 对象身份相等 (原 is_identical_to)
  pub fn is_identical_to(&self, other: &Self) -> bool {
    self.reference_type == other.reference_type
      && match (self.handle.to_handle(), other.handle.to_handle()) {
        (Some(a), Some(b)) => a.is_identical_to(&b),
        (None, None) => true,
        _ => false,
      }
  }
}
`;

const p = writeSrc("v8-handles", "maybe_handles.rs", body);
console.log(`✔ v8-handles/src/maybe_handles.rs (${body.split("\n").length} 行)`);
console.log(`  枚举对数: HeapObjectReferenceType ${variants.length} 变体 ${variants.join("/")} (1:1)`);
console.log(`  常量对数: kWeakHeapObjectTag=${weakTag[1]}, kHeapObjectReferenceTagMask 表达式命中`);
console.log(`  API 对数: MaybeHandle ${maybeApi.length} 面, MaybeObjectHandle ${objApi.length} 面全部命中`);
