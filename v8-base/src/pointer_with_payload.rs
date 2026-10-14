//! v8/src/base/pointer-with-payload.h 的 1:1 转写 — 指针低位 payload 打包
//!
//! 原版 `PointerWithPayload<PointerType, PayloadType, NumPayloadBits>`:
//! 对齐富余的低位存 payload (alignof >= 8 → 3 位可用)。Rust 无 C++ 级
//! 指针泛型打包, 以裸地址 usize + 常量泛型 NUM_PAYLOAD_BITS 承接,
//! 指针以 `*mut T` 进出 (get/set 处按 T 的对齐断言校验可用位)。
//! PayloadType 取 usize (原版枚举/小整数 payload 的位语义等价, 调用方自行转枚举)。

use core::mem::align_of;

/// 指针类型的最小可用低位位数 (原版 PointerWithPayloadTraits)
/// alignof >= 8 → 3; >= 4 → 2; 否则 1
#[inline]
#[must_use]
pub fn available_bits<T>() -> u32 {
  match align_of::<T>() {
    a if a >= 8 => 3,
    a if a >= 4 => 2,
    _ => 1,
  }
}

/// 裸地址 + 低位 payload 打包 (原版 PointerWithPayload 主结构)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerWithPayload<const NUM_PAYLOAD_BITS: usize = 3> {
  raw: usize,
}

impl<const NUM_PAYLOAD_BITS: usize> Default for PointerWithPayload<NUM_PAYLOAD_BITS> {
  #[inline]
  fn default() -> Self {
    Self { raw: 0 }
  }
}

impl<const NUM_PAYLOAD_BITS: usize> PointerWithPayload<NUM_PAYLOAD_BITS> {
  /// kPayloadMask = 2^NumPayloadBits - 1 (编译期)
  const PAYLOAD_MASK: usize = (1_usize << NUM_PAYLOAD_BITS) - 1;
  /// kPointerMask = ~kPayloadMask
  const POINTER_MASK: usize = !Self::PAYLOAD_MASK;

  /// PointerWithPayload(): 空
  #[inline]
  pub const fn new() -> Self {
    Self { raw: 0 }
  }

  /// 对 T 的对齐足够承载 NUM_PAYLOAD_BITS 的编译期/运行期断言
  /// (原版 static_assert kAvailableBits >= NumPayloadBits → 运行期对齐检查)
  #[inline]
  pub fn assert_fits<T>() {
    debug_assert!(
      available_bits::<T>() as usize >= NUM_PAYLOAD_BITS,
      "Ptr does not have sufficient alignment for the selected amount of storage bits"
    );
  }

  /// PointerWithPayload(PointerType*): 零 payload
  #[inline]
  pub fn from_pointer<T>(ptr: *mut T) -> Self {
    Self::assert_fits::<T>();
    let r = Self {
      raw: ptr as usize & Self::POINTER_MASK,
    };
    debug_assert_eq!(r.get_pointer::<T>(), ptr);
    debug_assert_eq!(r.get_payload(), 0);
    r
  }

  /// PointerWithPayload(PayloadType): 空指针 + payload
  /// 原版 pointer-with-payload.h:52-53 构造自检 (DCHECK) 随行
  #[inline]
  pub const fn from_payload(payload: usize) -> Self {
    debug_assert!(payload & Self::POINTER_MASK == 0, "payload 混入指针位");
    debug_assert!(
      payload & Self::PAYLOAD_MASK == payload,
      "payload 超出掩码范围"
    );
    Self { raw: payload }
  }

  /// PointerWithPayload(pointer, payload)
  #[inline]
  pub fn from_pointer_and_payload<T>(ptr: *mut T, payload: usize) -> Self {
    let mut r = Self::new();
    r.update(ptr, payload);
    r
  }

  /// GetPointer
  #[inline]
  #[must_use]
  pub fn get_pointer<T>(&self) -> *mut T {
    (self.raw & Self::POINTER_MASK) as *mut T
  }

  /// GetPointerWithKnownPayload: 已知 payload 时以减法免掩码
  #[inline]
  #[must_use]
  pub fn get_pointer_with_known_payload<T>(&self, payload: usize) -> *mut T {
    debug_assert_eq!(self.get_payload(), payload);
    (self.raw - payload) as *mut T
  }

  /// Update(pointer, payload)
  #[inline]
  pub fn update<T>(&mut self, new_pointer: *mut T, new_payload: usize) {
    Self::assert_fits::<T>();
    self.raw = (new_pointer as usize & Self::POINTER_MASK) | new_payload;
    debug_assert_eq!(self.get_payload(), new_payload);
    debug_assert_eq!(self.get_pointer::<T>(), new_pointer);
  }

  /// SetPointer: 保留 payload
  #[inline]
  pub fn set_pointer<T>(&mut self, newptr: *mut T) {
    debug_assert_eq!(newptr as usize & Self::PAYLOAD_MASK, 0);
    self.raw = (newptr as usize & Self::POINTER_MASK) | (self.raw & Self::PAYLOAD_MASK);
    debug_assert_eq!(self.get_pointer::<T>(), newptr);
  }

  /// GetPayload
  #[inline]
  #[must_use]
  pub const fn get_payload(&self) -> usize {
    self.raw & Self::PAYLOAD_MASK
  }

  /// SetPayload: 保留指针
  #[inline]
  pub fn set_payload(&mut self, new_payload: usize) {
    debug_assert_eq!(new_payload & Self::PAYLOAD_MASK, new_payload);
    self.raw = (self.raw & Self::POINTER_MASK) | new_payload;
    debug_assert_eq!(self.get_payload(), new_payload);
  }

  /// raw()
  #[inline]
  #[must_use]
  pub const fn raw(&self) -> usize {
    self.raw
  }
}
