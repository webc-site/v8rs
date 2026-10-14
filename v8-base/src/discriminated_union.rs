//! v8/src/base/discriminated-union.h 的 1:1 转写
//! tag + 原始字节存储的可判别联合: 判别枚举值 = 成员类型下标。
//! 原版模板参数 `TagEnum + typename... Ts` → Rust 以 `UnionMember` trait 的
//! 常量 TAG 承担 index_of_type 映射, const generic 承担 max(sizeof/alignof)。
//! 登记差异:
//! - static_assert(trivially_destructible) → UnionMember: Copy (Rust 无非 Copy
//!   平凡析构类型; Copy 即隐式 memcpy 语义等价)。
//! - static_assert(index < sizeof...(Ts)/255) → 类型系统无法按成员集合校验,
//!   构造时按存储容量 (SIZE/对齐 ≤ 8) 运行时守卫, 不合规则返回 None (禁 panic)。
//! - 原版 get<Tag tag>() 模板形态: Rust const-generic 无法反查类型, 以
//!   get::<M>() (按类型即按 tag) 覆盖 — 原版按类型/按 tag 两形态信息等价。
//! - alignas(max(alignof Ts...)) → repr(C, align(8)) 固定超集对齐 (align ≤ 8
//!   成员由构造守卫保证)。

use std::{
  mem::{align_of, size_of},
  ptr::write,
};

/// 联合成员契约: Copy (平凡析构) + 判别 tag (原版枚举下标)
pub trait UnionMember: Copy {
  /// 原版 static_cast<uint8_t>(index_of_type_v<T, Ts...>)
  const TAG: u8;
}

/// 原版 `DiscriminatedUnion<TagEnum, Ts...>` (字节存储形态)
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct DiscriminatedUnion<const SIZE: usize> {
  /// alignas(...) char data_[max(sizeof Ts...)]
  data: [u8; SIZE],
  /// uint8_t tag_
  tag: u8,
}

impl<const SIZE: usize> DiscriminatedUnion<SIZE> {
  /// 成员容量守卫: 尺寸入存储且对齐不超固定 repr(align = 8)
  fn fits<M: UnionMember>() -> bool {
    size_of::<M>() <= SIZE && align_of::<M>() <= 8
  }

  /// constexpr explicit DiscriminatedUnion(T&& data) — 按类型推 tag
  pub fn new<M: UnionMember>(value: M) -> Option<Self> {
    if !Self::fits::<M>() {
      return None;
    }
    let mut out = Self {
      data: [0; SIZE],
      tag: M::TAG,
    };
    // SAFETY: fits 守卫保证 data 前 size_of::<M>() 字节容量足且 8 对齐可作 M 落点;
    // M: Copy 平凡析构, 直接写入无需旧值处理; tag 已同步
    unsafe { write(out.data.as_mut_ptr().cast::<M>(), value) };
    Some(out)
  }

  /// constexpr explicit DiscriminatedUnion(Tag tag, T&& data) —
  /// 原版 DCHECK_EQ(tag, index): 不合规则 None (显式错误路径)
  pub fn with_tag<M: UnionMember>(tag: u8, value: M) -> Option<Self> {
    if tag != M::TAG {
      return None;
    }
    Self::new(value)
  }

  /// constexpr Tag tag() const
  #[inline]
  pub fn tag(&self) -> u8 {
    self.tag
  }

  /// constexpr const auto& get<T>() const —
  /// 原版 DCHECK_EQ(index_of_type, tag): 失配即 None (释放版误读在 Rust 侧禁止)
  pub fn get<M: UnionMember>(&self) -> Option<&M> {
    debug_assert!(Self::fits::<M>());
    if M::TAG != self.tag || !Self::fits::<M>() {
      return None;
    }
    // SAFETY: tag 匹配 ⇒ 存储由 new 以同 M 写入 (fits 校验过容量/对齐),
    // M: Copy 无活动不变量, 字节复用即读取本体
    unsafe { Some(&*self.data.as_ptr().cast::<M>()) }
  }

  /// constexpr auto& get<T>() 可变形态
  pub fn get_mut<M: UnionMember>(&mut self) -> Option<&mut M> {
    debug_assert!(Self::fits::<M>());
    if M::TAG != self.tag || !Self::fits::<M>() {
      return None;
    }
    // SAFETY: 同 get; &mut 借用保证唯一写路径
    unsafe { Some(&mut *self.data.as_mut_ptr().cast::<M>()) }
  }
}
