#!/usr/bin/env bun
// tool/src/codegen/external-reference-encoder.h.js
// 机械提取 v8/src/codegen/external-reference-encoder.{h,cc} — 编码器位域与骨架。
// 产物: v8-codegen-core/src/external_reference_encoder.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const H_REL = "src/codegen/external-reference-encoder.h";
const CC_REL = "src/codegen/external-reference-encoder.cc";
const h = readV8(H_REL);
readV8(CC_REL); // 家族覆盖锚

// Value 位域: Index = BitField<uint32_t, 0, 31>, IsFromAPI = BitField<bool, 31, 1>
const indexM = h.match(/using Index = base::BitField<uint32_t,\s*(\d+),\s*(\d+)>;/);
const apiM = h.match(/using IsFromAPI = base::BitField<bool,\s*(\d+),\s*(\d+)>;/);
if (!indexM || indexM[1] !== "0" || indexM[2] !== "31") throw new Error("Index 位域锚不符");
if (!apiM || apiM[1] !== "31" || apiM[2] !== "1") throw new Error("IsFromAPI 位域锚不符");
for (const anchor of [
  "static uint32_t Encode(uint32_t index, bool is_from_api)",
  "Value Encode(Address key) const;",
  "Maybe<Value> TryEncode(Address key) const;",
  "const char* NameOfAddress(Isolate* isolate, Address address) const;",
]) {
  if (!h.includes(anchor)) throw new Error(`缺锚 ${anchor.slice(0, 40)}`);
}
if (!h.includes("AddressToIndexHashMap* map_;")) throw new Error("缺 map_ 字段锚");

let rust = banner("src/codegen/external-reference-encoder.h.js", "src/codegen/external-reference-encoder.{h,cc}", "外部引用编码器位域与骨架");
rust += `
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
      value: (index & Self::INDEX_MASK) | (((is_from_api as u32) << Self::IS_FROM_API_SHIFT)),
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
`;

const p = writeSrc("v8-codegen-core", "external_reference_encoder.rs", rust);
console.log(`written ${p}`);
