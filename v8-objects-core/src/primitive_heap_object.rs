// 由 tool/src/objects/primitive-heap-object.h.js 从 v8/src/objects/primitive-heap-object.{h,tq} 生成; 修订就地改, 复跑覆盖
// PrimitiveHeapObject 抽象基类标记

/// 原始值堆对象抽象基类 (原版 V8_OBJECT class PrimitiveHeapObject : public HeapObject, V8_IT_ABSTRACT)
/// 子族: HeapNumber / Oddball / BigInt / Hole / 字符串等无属性基
pub struct PrimitiveHeapObject;

impl PrimitiveHeapObject {
  /// 原版 sizeof 断言: 与 HeapObject 头等大 (无附加字段)
  pub const SIZEOF_EQUALS_HEAP_OBJECT: bool = true;
}

// —— 签名骨架 ——
// explicit PrimitiveHeapObject(Tagged<ReadOnly<Map>> map); (inl)
// DECL_VERIFIER(PrimitiveHeapObject)
pub const SIGNATURES: [&str; 2] = ["explicit(Tagged<ReadOnly<Map>>)", "Verify"];

// —— 提取不变式断言 ——
const _: () = {
  assert!(PrimitiveHeapObject::SIZEOF_EQUALS_HEAP_OBJECT);
};
