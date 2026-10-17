//! 由 tool/src/handles/maybe-handles.js 从 v8/src/handles/maybe-handles{.h,-inl.h} 生成
//! 惯用法映射: ToHandle 的 bool+out 参数 → Option<Handle<T>>; ToHandleChecked 的 CHECK
//! 致命语义 → Result (错误集中 src/error.rs); NullMaybeHandleType 哨兵类型 → null() 构造。
use std::{
  marker::PhantomData,
  ptr::{eq, null_mut},
};

use v8_common::{Address, HeapObjectReferenceType, K_HEAP_OBJECT_TAG_MASK, K_WEAK_HEAP_OBJECT_TAG};

use crate::{
  error::{Error, Result},
  handles::{DirectHandle, Handle, HandleScope, Object, SlotPtr, Tagged},
};

/// 由强 tagged 指针造弱引用 (原 MakeWeak: 低位 tag 置 3)
pub const fn make_weak(ptr: Address) -> Address {
  (ptr & !K_HEAP_OBJECT_TAG_MASK) | K_WEAK_HEAP_OBJECT_TAG
}

/// 是否弱引用 (kHeapObjectReferenceTagMask 低位判定)
pub const fn is_weak(ptr: Address) -> bool {
  ptr & K_HEAP_OBJECT_TAG_MASK == K_WEAK_HEAP_OBJECT_TAG
}

/// 保活槽位 + 引用强度 → 读出值 (MaybeObject{,Direct}Handle::operator* 共用):
/// 弱则复原弱 tag, 强则原值。
fn resolve(reference_type: HeapObjectReferenceType, location: SlotPtr) -> Tagged<Object> {
  // SAFETY: 约定非空, 槽位在作用域存活约定内
  let strong = Handle::<Object>::from_location(location).value().ptr();
  Tagged::from_ptr(match reference_type {
    HeapObjectReferenceType::Weak => make_weak(strong),
    HeapObjectReferenceType::Strong => strong,
  })
}

/// 保活句柄对 + 引用强度 → 对象身份相等 (两个 is_identical_to 共用)
fn identical(
  a: (HeapObjectReferenceType, MaybeHandle<Object>),
  b: (HeapObjectReferenceType, MaybeHandle<Object>),
) -> bool {
  a.0 == b.0
    && match (a.1.to_handle(), b.1.to_handle()) {
      (Some(x), Some(y)) => x.is_identical_to(&y),
      (None, None) => true,
      _ => false,
    }
}

/// 可空句柄 (原 template<class T> class MaybeHandle final : HandleBase):
/// Handle → MaybeHandle 单向转换, 反向必须做空检查 —— 强制使用前校验 (原注释)。
/// 刻意不提供默认相等/散列 (语义歧义同 Handle, 原注释)。
/// 手动 impl Copy/Clone: derive 会强加 `T: Copy` bound, 幻类型参数不参与复制
pub struct MaybeHandle<T = Object> {
  location: SlotPtr,
  _m: PhantomData<fn() -> T>,
}

impl<T> Clone for MaybeHandle<T> {
  fn clone(&self) -> Self {
    *self
  }
}
impl<T> Copy for MaybeHandle<T> {}

impl<T> MaybeHandle<T> {
  /// 空 (原 MaybeHandle() / kNullMaybeHandle)
  pub const fn null() -> Self {
    Self {
      location: null_mut(),
      _m: PhantomData,
    }
  }

  /// 由句柄上转 (原 MaybeHandle(Handle<S>) 隐式上转)
  pub const fn from_handle(handle: Handle<T>) -> Self {
    Self {
      location: handle.location(),
      _m: PhantomData,
    }
  }

  /// 由槽位构造 (原 private explicit MaybeHandle(Address*), 供 UncheckedCast)
  pub const fn from_location(location: SlotPtr) -> Self {
    Self {
      location,
      _m: PhantomData,
    }
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

  /// location 相等 (原 equals; std::ptr::eq 使指针比较意图显式)
  pub fn equals(&self, other: &Self) -> bool {
    eq(self.location, other.location)
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

/// 可空直连句柄 (原 !V8_ENABLE_DIRECT_HANDLE 分支: MaybeHandle 的包装)。
/// 手动 impl Copy/Clone/Default: derive 会强加 `T: Copy`/`T: Default` bound
pub struct MaybeDirectHandle<T = Object> {
  handle: MaybeHandle<T>,
}

impl<T> Clone for MaybeDirectHandle<T> {
  fn clone(&self) -> Self {
    *self
  }
}
impl<T> Copy for MaybeDirectHandle<T> {}

impl<T> MaybeDirectHandle<T> {
  /// 空 (原 MaybeDirectHandle() / kNullMaybeHandle)
  pub const fn null() -> Self {
    Self {
      handle: MaybeHandle::null(),
    }
  }

  /// 由直连句柄构造 (原 MaybeDirectHandle(DirectHandle<S>) 上转)
  pub const fn from_direct(handle: DirectHandle<T>) -> Self {
    Self {
      handle: MaybeHandle::from_handle(handle.indirect()),
    }
  }

  /// 由可空句柄构造 (原 MaybeDirectHandle(MaybeIndirectHandle<S>) 上转)
  pub const fn from_maybe(handle: MaybeHandle<T>) -> Self {
    Self { handle }
  }

  /// 分配新可空直连句柄 (原 MaybeDirectHandle(Tagged<T>, Isolate*))
  pub fn new(value: Tagged<T>, scope: &HandleScope) -> Self {
    Self {
      handle: MaybeHandle::from_handle(Handle::new(value, scope)),
    }
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
  pub fn to_handle(&self) -> Option<DirectHandle<T>> {
    self.handle.to_handle().map(DirectHandle::from_handle)
  }

  /// 转直连句柄, 空即错误 (原 ToHandleChecked)
  pub fn to_handle_checked(&self) -> Result<DirectHandle<T>> {
    self
      .handle
      .to_handle_checked()
      .map(DirectHandle::from_handle)
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
    MaybeDirectHandle {
      handle: self.handle.unchecked_cast(),
    }
  }
}

impl<T> Default for MaybeDirectHandle<T> {
  fn default() -> Self {
    Self::null()
  }
}

/// 可弱引用句柄 (原 class MaybeObjectHandle): 持强保活, 读出时按 reference_type 复原弱/强。
/// 手动 impl Copy/Clone: 字段虽全 Copy, 但 derive 会经 MaybeHandle<T> 强加 `T: Copy`
#[derive(Clone, Copy)]
pub struct MaybeObjectHandle {
  /// 引用强度 (原 reference_type_)
  reference_type: HeapObjectReferenceType,
  /// 保活句柄 (原 handle_)
  handle: MaybeHandle<Object>,
}

impl MaybeObjectHandle {
  /// 原默认构造: 强引用 + 空句柄
  pub const fn new() -> Self {
    Self {
      reference_type: HeapObjectReferenceType::Strong,
      handle: MaybeHandle::null(),
    }
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
    Self {
      reference_type: HeapObjectReferenceType::Strong,
      handle: MaybeHandle::from_handle(object),
    }
  }

  /// 弱引用版由句柄构造 (原 static Weak(Handle<Object>))
  pub const fn weak_from_handle(object: Handle<Object>) -> Self {
    Self {
      reference_type: HeapObjectReferenceType::Weak,
      handle: MaybeHandle::from_handle(object),
    }
  }

  /// 读出引用值 (原 operator*): 弱则复原弱 tag, 强则原值。
  /// 空句柄读值属调用方违约 (原 ToHandleChecked 的 CHECK → debug_assert)
  pub fn value(&self) -> Tagged<Object> {
    debug_assert!(!self.handle.is_null(), "MaybeObjectHandle 为空");
    resolve(self.reference_type, self.handle.location())
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
    identical(
      (self.reference_type, self.handle),
      (other.reference_type, other.handle),
    )
  }
}

impl Default for MaybeObjectHandle {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for MaybeObjectDirectHandle {
  fn default() -> Self {
    Self::new()
  }
}

/// 可弱引用直连句柄 (原 class MaybeObjectDirectHandle, 非 direct 分支)。
/// Default 手动 impl: HeapObjectReferenceType (v8_common 单一真相) 不提供 Default
#[derive(Clone, Copy)]
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
    Self {
      reference_type: HeapObjectReferenceType::Strong,
      handle: MaybeDirectHandle::new(object, scope),
    }
  }

  /// 弱引用构造 (原 static Weak(Tagged<Object>, Isolate*))
  pub fn weak(object: Tagged<Object>, scope: &HandleScope) -> Self {
    Self {
      reference_type: HeapObjectReferenceType::Weak,
      handle: MaybeDirectHandle::new(object, scope),
    }
  }

  /// 由直连句柄构造强引用 (原 MaybeObjectDirectHandle(DirectHandle<Object>))
  pub const fn from_direct(object: DirectHandle<Object>) -> Self {
    Self {
      reference_type: HeapObjectReferenceType::Strong,
      handle: MaybeDirectHandle::from_direct(object),
    }
  }

  /// 弱引用版 (原 static Weak(DirectHandle<Object>))
  pub const fn weak_from_direct(object: DirectHandle<Object>) -> Self {
    Self {
      reference_type: HeapObjectReferenceType::Weak,
      handle: MaybeDirectHandle::from_direct(object),
    }
  }

  /// 由可弱引用句柄转换 (原 MaybeObjectDirectHandle(MaybeObjectHandle)):
  /// 直读保活句柄字段 (原版经 handle_ 字段访问, 无 indirect 间接层)
  pub const fn from_maybe_object(obj: MaybeObjectHandle) -> Self {
    Self {
      reference_type: obj.reference_type,
      handle: MaybeDirectHandle::from_maybe(obj.handle),
    }
  }

  /// 读出引用值 (原 operator*): 弱则复原弱 tag, 强则原值;
  /// 空句柄读值属调用方违约 (debug_assert)
  pub fn value(&self) -> Tagged<Object> {
    debug_assert!(!self.handle.is_null(), "MaybeObjectDirectHandle 为空");
    resolve(self.reference_type, self.handle.indirect().location())
  }

  /// 保活直连句柄 (原 object(): handle_.ToHandleChecked())
  pub fn object(&self) -> Result<DirectHandle<Object>> {
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
    identical(
      (self.reference_type, self.handle.indirect()),
      (other.reference_type, other.handle.indirect()),
    )
  }
}
