// 由 tool/src/objects/tagged-value-inl.h.js 从 v8/src/objects/tagged-value-inl.h + tagged-value.h 生成; 修订就地改, 复跑覆盖
// 钉死配置: V8_COMPRESS_POINTERS 开 (Tagged_t=u32); 非压缩恒等分支属非目标配置不实现

//! 压缩值包装 (原版 tagged-value.{h,-inl.h})
//!
//! `StrongTaggedValue` / `TaggedValue` 是"槽内原始表示"的受限包装:
//! 只存 `Tagged_t` (压缩配置为 u32), 不持有 cage base, 故只提供
//! 免解压的比较与经外部 cage base 的转换.

use v8_common::{TrustedSpaceCompressionScheme, V8HeapCompressionScheme};

/// 强引用压缩包装 (原版 StrongTaggedValue: STRONG + Tagged_t)
pub struct StrongTaggedValue {
  pub raw: u32,
}

/// 可弱压缩包装 (原版 TaggedValue: WEAK + Tagged_t)
pub struct TaggedValue {
  pub raw: u32,
}

impl StrongTaggedValue {
  /// 从完整 Object 压缩 (原版构造: CompressObject — 断言输入是合法强引用)
  ///
  /// v8: src/objects/tagged-value-inl.h StrongTaggedValue::StrongTaggedValue
  #[inline]
  pub fn from_object(o: usize) -> Self {
    Self {
      raw: V8HeapCompressionScheme::compress_object(o),
    }
  }

  /// 解压回完整 Object (原版 ToObject: DecompressTagged(object.ptr()))
  ///
  /// v8: src/objects/tagged-value-inl.h StrongTaggedValue::ToObject
  #[inline]
  pub fn to_object(&self, _cage_base: usize) -> usize {
    // 原版压缩分支经 cage_base 解压 (base|零扩展); 全局主 cage 与入参 cage
    // 在钉死 shared-cage 配置下同值, 保留形参对齐调用面。
    V8HeapCompressionScheme::decompress_tagged(self.raw)
  }
}

impl TaggedValue {
  /// 从 MaybeObject 压缩 (原版构造: CompressAny — 强弱皆可)
  ///
  /// v8: src/objects/tagged-value-inl.h TaggedValue::TaggedValue
  #[inline]
  pub const fn from_maybe_object(o: usize) -> Self {
    Self {
      raw: V8HeapCompressionScheme::compress_any(o),
    }
  }

  /// 解压回 MaybeObject (原版 ToMaybeObject: DecompressTagged)
  ///
  /// v8: src/objects/tagged-value-inl.h TaggedValue::ToMaybeObject
  #[inline]
  pub fn to_maybe_object(&self, _cage_base: usize) -> usize {
    // SAFETY 面: 同 to_object; 弱位/cleared 值在低 32 位内保留。
    V8HeapCompressionScheme::decompress_tagged(self.raw)
  }

  /// trusted cage 变体解压 (原版 TrustedTaggedValue::ToMaybeObject 路径)
  ///
  /// v8: src/objects/tagged-value-inl.h TrustedTaggedValue::ToMaybeObject
  #[inline]
  pub fn to_maybe_object_trusted(&self, _cage_base: usize) -> usize {
    TrustedSpaceCompressionScheme::decompress_tagged(self.raw)
  }
}
