// 由 tool/src/codegen/external-reference-encoder.h.js 从 v8/src/codegen/external-reference-encoder.{h,cc} 生成; 修订就地改, 复跑覆盖
// 外部引用编码器位域与骨架

/// 原 ExternalReferenceEncoder::Value — 位域打包
/// Index = BitField<uint32_t, 0, 31>; IsFromAPI = BitField<bool, 31, 1>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value {
  value: u32,
}

impl Value {
  pub const INDEX_SHIFT: u32 = 0;
  pub const INDEX_BITS: u32 = 31;
  pub const INDEX_MASK: u32 = (1 << 31) - 1;
  pub const IS_FROM_API_SHIFT: u32 = 31;
  pub const IS_FROM_API_BITS: u32 = 1;

  /// 原 Value::Encode
  #[inline]
  #[must_use]
  pub const fn encode(index: u32, is_from_api: bool) -> Self {
    Value {
      value: (index & Self::INDEX_MASK) | ((is_from_api as u32) << Self::IS_FROM_API_SHIFT),
    }
  }

  /// 原 Value::index
  #[inline]
  #[must_use]
  pub const fn index(self) -> u32 {
    (self.value >> Self::INDEX_SHIFT) & Self::INDEX_MASK
  }

  /// 原 Value::is_from_api
  #[inline]
  #[must_use]
  pub const fn is_from_api(self) -> bool {
    (self.value >> Self::IS_FROM_API_SHIFT) & 1 != 0
  }
}

/// 原 class ExternalReferenceEncoder — 地址→索引哈希编码器
/// - 原签名: explicit ExternalReferenceEncoder(Isolate*)
/// - 原签名: Value Encode(Address key) const
/// - 原签名: Maybe<Value> TryEncode(Address key) const
/// - 原签名: const char* NameOfAddress(Isolate*, Address) const
/// DEBUG 态: count_ 计数 + api_references_ 槽
pub struct ExternalReferenceEncoder;

// TODO(dev): 补哈希表编码实现。

// —— 提取不变式断言 ——
const _: () = {
  let v = Value::encode(0x1234, true);
  assert!(v.index() == 0x1234);
  assert!(v.is_from_api());
};
