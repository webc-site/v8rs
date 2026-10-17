// 由 tool/src/objects/megadom-handler-inl.h.js 从 v8/src/objects/megadom-handler-inl.h 生成; 修订就地改, 复跑覆盖
// MegaDomHandler 两字段 inline 存取器 (accessor 含内存序变体)
//! MegaDomHandler inline 面: accessor (MaybeObject, API 回调槽) 与 context 两槽。
//! accessor 提供 Acquire_Load / Release_Store 内存序变体; context 仅普通 load/store。
//!
//! 布局 (v8/src/objects/megadom-handler.h: HeapObject + accessor_ + context_):
//! 两 Tagged 槽紧邻 map 头之后, 经 Tagged 指针地址做原子读写; 写屏障模式随 store
//! 透传 (本层不驱动 GC 屏障, 语义面归 v8-heap-core, 与 crate 既有 TaggedMember.store 一致)。

use std::{
  mem::{offset_of, size_of},
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_core::{
  heap_object::HeapObject,
  objects::WriteBarrierMode,
  tagged::{MaybeObject, Tagged},
};

/// accessor 变体数 (load/acquire/store/release)。
pub const ACCESSOR_VARIANTS: usize = 4;
/// context 变体数 (load/store)。
pub const CONTEXT_VARIANTS: usize = 2;

/// MegaDomHandler 头布局 (原版 V8_OBJECT class MegaDomHandler : public HeapObject)。
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MegaDomHandler {
  /// HeapObject 头 (map 槽)。
  pub header: HeapObject,
  /// TaggedMember<MaybeObject> accessor_。
  pub accessor: Tagged<MaybeObject>,
  /// TaggedMember<MaybeObject> context_。
  pub context: Tagged<MaybeObject>,
}

impl MegaDomHandler {
  /// accessor_ 槽相对对象首地址的字节偏移 (对齐 offsetof(MegaDomHandler, accessor_))。
  pub const ACCESSOR_OFFSET: usize = offset_of!(Self, accessor);
  /// context_ 槽偏移。
  pub const CONTEXT_OFFSET: usize = offset_of!(Self, context);
  /// 原版对象头 (固定字段) 尺寸 (== offsetof(MegaDomHandler, accessor_) == kHeaderSize)。
  pub const K_HEADER_SIZE: usize = Self::ACCESSOR_OFFSET;
  /// 整对象尺寸 (含 accessor 与 context 弱槽)。
  pub const K_OBJECT_SIZE: usize = size_of::<Self>();
}

/// 对 `Tagged<MegaDomHandler>` 的字段存取 (对齐 megadom-handler-inl.h 逐方法)。
///
/// `Tagged` 为跨 crate 类型, 故以本地 trait 扩展 (与 crate::transitions::TransitionArrayExt 同法)。
pub trait MegaDomHandlerExt {
  #[must_use]
  fn accessor(&self) -> Tagged<MaybeObject>;
  #[must_use]
  fn accessor_acquire(&self) -> Tagged<MaybeObject>;
  fn set_accessor(&self, value: Tagged<MaybeObject>, mode: WriteBarrierMode);
  fn set_accessor_release(&self, value: Tagged<MaybeObject>, mode: WriteBarrierMode);
  #[must_use]
  fn context(&self) -> Tagged<MaybeObject>;
  fn set_context(&self, value: Tagged<MaybeObject>, mode: WriteBarrierMode);
}

impl MegaDomHandlerExt for Tagged<MegaDomHandler> {
  /// 原版 accessor() — 普通 load (megadom-handler-inl.h:19)。
  #[inline]
  fn accessor(&self) -> Tagged<MaybeObject> {
    let addr = self.address() + MegaDomHandler::ACCESSOR_OFFSET;
    // SAFETY: addr 落在有效 MegaDomHandler 的 accessor_ 槽内。
    let raw = unsafe { AtomicUsize::from_ptr(addr as *mut usize).load(Ordering::Relaxed) };
    Tagged::new(raw)
  }

  /// 原版 accessor(AcquireLoadTag) — acquire 语义 load (megadom-handler-inl.h:23)。
  #[inline]
  fn accessor_acquire(&self) -> Tagged<MaybeObject> {
    let addr = self.address() + MegaDomHandler::ACCESSOR_OFFSET;
    // SAFETY: addr 落在有效 MegaDomHandler 的 accessor_ 槽内。
    let raw = unsafe { AtomicUsize::from_ptr(addr as *mut usize).load(Ordering::Acquire) };
    Tagged::new(raw)
  }

  /// 原版 set_accessor(value, mode) — 普通 store, 写屏障模式透传 (megadom-handler-inl.h:27)。
  #[inline]
  fn set_accessor(&self, value: Tagged<MaybeObject>, _mode: WriteBarrierMode) {
    let addr = self.address() + MegaDomHandler::ACCESSOR_OFFSET;
    // SAFETY: addr 落在有效 MegaDomHandler 的 accessor_ 槽内, 且具备写权限。
    unsafe { AtomicUsize::from_ptr(addr as *mut usize).store(value.ptr(), Ordering::Relaxed) };
  }

  /// 原版 set_accessor(value, ReleaseStoreTag, mode) — release 语义 store (megadom-handler-inl.h:33)。
  #[inline]
  fn set_accessor_release(&self, value: Tagged<MaybeObject>, _mode: WriteBarrierMode) {
    let addr = self.address() + MegaDomHandler::ACCESSOR_OFFSET;
    // SAFETY: addr 落在有效 MegaDomHandler 的 accessor_ 槽内, 且具备写权限。
    unsafe { AtomicUsize::from_ptr(addr as *mut usize).store(value.ptr(), Ordering::Release) };
  }

  /// 原版 context() — 普通 load (无内存序变体) (megadom-handler-inl.h:38)。
  #[inline]
  fn context(&self) -> Tagged<MaybeObject> {
    let addr = self.address() + MegaDomHandler::CONTEXT_OFFSET;
    // SAFETY: addr 落在有效 MegaDomHandler 的 context_ 槽内。
    let raw = unsafe { AtomicUsize::from_ptr(addr as *mut usize).load(Ordering::Relaxed) };
    Tagged::new(raw)
  }

  /// 原版 set_context(value, mode) (megadom-handler-inl.h:41)。
  #[inline]
  fn set_context(&self, value: Tagged<MaybeObject>, _mode: WriteBarrierMode) {
    let addr = self.address() + MegaDomHandler::CONTEXT_OFFSET;
    // SAFETY: addr 落在有效 MegaDomHandler 的 context_ 槽内, 且具备写权限。
    unsafe { AtomicUsize::from_ptr(addr as *mut usize).store(value.ptr(), Ordering::Relaxed) };
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(ACCESSOR_VARIANTS == 4);
  assert!(CONTEXT_VARIANTS == 2);
  // 两槽紧邻 map 头, 序 1:1 (offset(accessor_)=sizeof(HeapObject), offset(context_)=+1 槽)。
  assert!(MegaDomHandler::ACCESSOR_OFFSET == size_of::<HeapObject>());
  assert!(MegaDomHandler::CONTEXT_OFFSET == MegaDomHandler::ACCESSOR_OFFSET + size_of::<usize>());
  // 金标准字面量槽序锚定: map(槽0), accessor_(槽1), context_(槽2), 弱体起点 kHeaderSize = 1 槽
  assert!(MegaDomHandler::ACCESSOR_OFFSET == size_of::<usize>());
  assert!(MegaDomHandler::CONTEXT_OFFSET == 2 * size_of::<usize>());
  assert!(MegaDomHandler::K_HEADER_SIZE == size_of::<usize>());
  assert!(MegaDomHandler::K_OBJECT_SIZE == 3 * size_of::<usize>());
};
