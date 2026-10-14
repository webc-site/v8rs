#!/usr/bin/env bun
// tool/src/objects/tagged-value-inl.h.js — 压缩值包装生成
// 组→文件映射 (本脚本管辖):
//   v8/src/objects/tagged-value-inl.h (+ tagged-value.h 类面) → v8-objects-core/src/tagged_value_inl.rs
//   (tagged-value.h 的 Tagged<T> 同构登记归 tool/src/objects/tagged-field.h.js 注释; 本脚本落压缩转换语义)
// 规则面: StrongTaggedValue/TaggedValue 的压缩-解压 4 法 (CompressObject/CompressAny/DecompressTagged)
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/objects/tagged-value.h");
const inl = readV8("src/objects/tagged-value-inl.h");

// —— 类面断言 ——
if (!/class V8_GSL_POINTER StrongTaggedValue\s*\n\s*: public TaggedImpl<HeapObjectReferenceType::STRONG, Tagged_t>/.test(h))
  throw new Error("StrongTaggedValue 基类面漂移 (应 STRONG + Tagged_t)");
if (!/class V8_GSL_POINTER TaggedValue\s*\n\s*: public TaggedImpl<HeapObjectReferenceType::WEAK, Tagged_t>/.test(h))
  throw new Error("TaggedValue 基类面漂移 (应 WEAK + Tagged_t)");

// —— inl 转换四法 (压缩分支) ——
if (!/V8HeapCompressionScheme::CompressObject\(o\.ptr\(\)\)/.test(inl)) throw new Error("StrongTaggedValue 压缩走 CompressObject 缺失");
if (!/V8HeapCompressionScheme::CompressAny\(o\.ptr\(\)\)/.test(inl)) throw new Error("TaggedValue 压缩走 CompressAny 缺失");
const decompress = [...inl.matchAll(/V8HeapCompressionScheme::DecompressTagged\(object\.ptr\(\)\)/g)].length;
if (decompress !== 2) throw new Error(`ToObject/ToMaybeObject 解压应 2 处, 实得 ${decompress}`);

const body = `${banner("src/objects/tagged-value-inl.h.js", "src/objects/tagged-value-inl.h + tagged-value.h", "压缩配置专用值包装; 非压缩基线为恒等映射")}
//! 压缩值包装 (原版 tagged-value.{h,-inl.h})
//!
//! \`StrongTaggedValue\` / \`TaggedValue\` 是"槽内原始表示"的受限包装:
//! 只存 \`Tagged_t\` (压缩配置为 u32), 不持有 cage base, 故只提供
//! 免解压的比较与经外部 cage base 的转换. 非压缩基线下 Tagged_t=usize,
//! 转换退化为恒等.

/// 强引用压缩包装 (原版 StrongTaggedValue: STRONG + Tagged_t)
pub struct StrongTaggedValue { pub raw: u32 }

/// 可弱压缩包装 (原版 TaggedValue: WEAK + Tagged_t)
pub struct TaggedValue { pub raw: u32 }

impl StrongTaggedValue {
  /// 从完整 Object 压缩 (原版构造: CompressObject — 断言输入是合法强引用)
  pub fn from_object(o: usize) -> Self {
    todo!("就地修订: 非压缩恒等; 压缩走 V8HeapCompressionScheme::CompressObject(o)")
  }

  /// 解压回完整 Object (原版 ToObject: DecompressTagged(object.ptr()))
  pub fn to_object(&self, _cage_base: usize) -> usize {
    todo!("就地修订: 非压缩恒等; 压缩走 DecompressTagged(self.raw)")
  }
}

impl TaggedValue {
  /// 从 MaybeObject 压缩 (原版构造: CompressAny — 强弱皆可)
  pub fn from_maybe_object(o: usize) -> Self {
    todo!("就地修订: 非压缩恒等; 压缩走 CompressAny(o)")
  }

  /// 解压回 MaybeObject (原版 ToMaybeObject: DecompressTagged)
  pub fn to_maybe_object(&self, _cage_base: usize) -> usize {
    todo!("就地修订: 非压缩恒等; 压缩走 DecompressTagged(self.raw)")
  }
}
`;

const p = writeSrc("v8-objects-core", "tagged_value_inl.rs", body);
console.log(`tagged-value-inl.h.js: 已生成 ${p} (类 2, 转换法 4, 解压点 ${decompress})`);
