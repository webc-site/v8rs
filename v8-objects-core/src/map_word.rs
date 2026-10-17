// 由 tool/src/objects/map-word.h.js 从 v8/src/objects/map-word.h + map-word-inl.h + objects.cc IsMapOrForwarded 生成; 修订就地改, 复跑覆盖
// 钉死配置: V8_COMPRESS_POINTERS + V8_ENABLE_SANDBOX + V8_EXTERNAL_CODE_SPACE 开 (v8-common globals/config.rs); 落 ECS 分支

//! 堆对象首字 (MapWord): 正常态为 Map 指针, GC 期可编码转发地址
//!
//! 钉死配置 V8_EXTERNAL_CODE_SPACE 开 (v8-common/src/globals/config.rs:76):
//! 转发指针 = Smi 编码的与宿主对象地址差值 (kObjectAlignment 块数,
//! map-word-inl.h:48-77); 低 2 位 00 的非 ECS 形态属非目标配置, 不实现。
//! V8_MAP_PACKING 未启用: is_packed 恒 false, Pack/Unpack 不编译.

use v8_common::K_OBJECT_ALIGNMENT;

use crate::{
  heap_object::HeapObject,
  instance_type::InstanceType,
  map::Map,
  smi::Smi,
  tagged::Tagged,
  tagged_constants::{HEAP_OBJECT_TAG, SMI_TAG_MASK},
};

/// Map 打包 xor 掩码 (原版 Internals::kMapWordXorMask = 0b11, 仅 V8_MAP_PACKING 启用)
pub const MAP_WORD_XOR_MASK: usize = 3;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MapWord {
  value: usize,
}

impl MapWord {
  /// 从 Map 指针构造 (map-word-inl.h FromMap, 默认非打包直存)
  ///
  /// v8: src/objects/map-word-inl.h MapWord::FromMap
  #[inline]
  pub const fn from_map(map: Tagged<Map>) -> Self {
    Self { value: map.ptr() }
  }

  /// 还原为 Map 指针 (map-word-inl.h ToMap)
  ///
  /// v8: src/objects/map-word-inl.h MapWord::ToMap
  #[inline]
  pub const fn to_map(&self) -> Tagged<Map> {
    Tagged::new(self.value)
  }

  /// 是否转发地址 (ECS 分支: 转发指针为 Smi 编码, HAS_SMI_TAG 即 bit0=0)
  ///
  /// v8: src/objects/map-word-inl.h MapWord::IsForwardingAddress
  #[inline]
  pub const fn is_forwarding_address(&self) -> bool {
    self.value & SMI_TAG_MASK == 0
  }

  /// 从转发目标构造 (ECS 分支: Smi((object - host) / kObjectAlignment))
  ///
  /// 承接原版 C++ 语义: `static_cast<intptr_t>(unsigned 差)` 环绕收窄,
  /// Rust 用 wrapping 运算显式化 (map-word-inl.h:48-62)。
  ///
  /// v8: src/objects/map-word-inl.h MapWord::FromForwardingAddress
  #[inline]
  pub fn from_forwarding_address(
    map_word_host: Tagged<HeapObject>,
    object: Tagged<HeapObject>,
  ) -> Self {
    let diff = (object.ptr() as isize).wrapping_sub(map_word_host.ptr() as isize);
    debug_assert!(
      diff % K_OBJECT_ALIGNMENT as isize == 0,
      "转发差值须按 kObjectAlignment 对齐"
    );
    // 差值域: 单 cage 4GB 内 → /8 后 ≤ 2^28, 恒落 31 位 Smi 值域
    let smi = Smi::from_int31((diff / K_OBJECT_ALIGNMENT as isize) as i32);
    debug_assert!(Self::from_raw(smi.ptr()).is_forwarding_address());
    Self { value: smi.ptr() }
  }

  /// 还原转发目标 (ECS 分支: host.address() + Smi 值 × kObjectAlignment, 补堆对象 tag)
  ///
  /// v8: src/objects/map-word-inl.h MapWord::ToForwardingAddress
  #[inline]
  pub fn to_forwarding_address(&self, map_word_host: Tagged<HeapObject>) -> Tagged<HeapObject> {
    debug_assert!(self.is_forwarding_address());
    let diff = Smi::value_31(Tagged::new(self.value)) as isize;
    let diff = diff.wrapping_mul(K_OBJECT_ALIGNMENT as isize);
    let address = map_word_host.address().wrapping_add(diff as usize);
    // HeapObject::FromAddress 语义: 补 kHeapObjectTag
    Tagged::new(address + HEAP_OBJECT_TAG)
  }

  /// Map 自身是否为 Map 或被转发 (objects.cc:6565-6573)。
  ///
  /// 原版逐句: 读 `map->map_word(kRelaxedLoad)`; 转发态直接 true (GC 期
  /// 不可同步访问被转发 map); 否则 `map_word.ToMap()->instance_type()`
  /// 走 IsMap 检查 — 元 Map 被搬移后新旧位置 instance_type 不变仍判等。
  ///
  /// v8: src/objects/objects.cc MapWord::IsMapOrForwarded
  ///
  /// # Safety
  /// 调用方须确保 map 指针指向已初始化的 HeapObject 头部 (首字为 map word 槽)。
  #[inline]
  pub unsafe fn is_map_or_forwarded(map: Tagged<Map>) -> bool {
    // SAFETY: 契约由调用方承接 — 按本 crate HeapObject 布局约定 (map_word.rs /
    // heap_object.rs: 首字段 offset 0 为 map 槽) 以 usize 松读 map word。
    unsafe {
      let raw = (map.address() as *const usize).read();
      let map_word = MapWord::from_raw(raw);
      if map_word.is_forwarding_address() {
        return true;
      }
      let to_map = map_word.to_map();
      matches!((*to_map.as_ptr()).instance_type(), InstanceType::Map)
    }
  }

  /// 原始槽位值
  #[inline]
  pub const fn ptr(&self) -> usize {
    self.value
  }

  #[inline]
  pub const fn from_raw(value: usize) -> Self {
    Self { value }
  }

  /// 打包判定 (默认配置恒 false)
  ///
  /// v8: src/objects/map-word.h MapWord::IsPacked (V8_MAP_PACKING 未启用)
  #[inline]
  pub const fn is_packed(_mapword: usize) -> bool {
    false
  }
}
