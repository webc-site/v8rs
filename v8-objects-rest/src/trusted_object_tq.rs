// 由 tool/src/objects/trusted-object.tq.js 从 v8/src/objects/trusted-object.tq (+ trusted-object.h 跨源核对) 生成; 修订就地改, 复跑覆盖
// 非压缩沙箱关: 自间接指针字段不存在

//! 受信对象 tq 布局 (原版 trusted-object.tq)
//!
//! 类层级: HeapObject → TrustedObject (abstract) → ExposedTrustedObject (abstract).
//! ExposedTrustedObject 在沙箱开时增加 self_indirect_pointer (自引用句柄,
//! 供间接指针表反查); 基线沙箱关 → 头与 TrustedObject 同为 8 字节.

/// tq 类层级 (注解 + 条件字段 1:1)
pub const TRUSTED_OBJECT_TQ_CLASSES: [(&str, &str, &str, &str); 2] = [
  // (类, 基类, 注解, 条件字段)
  (
    "TrustedObject",
    "HeapObject",
    "@abstract @cppObjectLayoutDefinition",
    "",
  ),
  (
    "ExposedTrustedObject",
    "TrustedObject",
    "@abstract @cppObjectLayoutDefinition",
    "@if(V8_ENABLE_SANDBOX) self_indirect_pointer: TrustedPointer",
  ),
];

/// 沙箱开时自间接指针偏移 (ExposedTrustedObject 头内, 紧跟 map;
/// 基线沙箱关: 字段不存在, 本常量仅登记生成侧语义)
pub const K_SELF_INDIRECT_POINTER_OFFSET_SANDBOX_ON: usize = 8;

// 就地修订: 原版 self_indirect_pointer 槽 (IndirectPointerHandle, kSelfIndirectPointerTag)
// 仅存在于沙箱开配置; 基线沙箱关 → 字段与访问面均不存在, 存根移除, 语义由上方常量登记。
