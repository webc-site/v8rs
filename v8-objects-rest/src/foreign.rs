// 由 tool/src/objects/foreign.h.js 从 v8/src/objects/foreign.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// Foreign 外部指针包装

/// 外部指针包装 (原版 V8_OBJECT class Foreign : public HeapObject)
/// 布局 (tq): map + foreign_address (ExternalPointer, 沙箱内为句柄)
/// API 暴露实例类型值 (原版 @apiExposedInstanceTypeValue(0xcc))
pub const API_EXPOSED_INSTANCE_TYPE: u32 = 0xcc;

/// 受信形态 (原版 TrustedForeign : TrustedObject, foreign_address 为 RawPtr)
pub struct TrustedForeign;

/// 外部地址读写访问器族 (inl 侧)
pub const FOREIGN_ACCESSORS: [&str; 4] = ["foreign_address", "set_foreign_address", "raw_foreign_address", "init_foreign_address"];

// —— 签名骨架 ——
// Address foreign_address(IsolateForSandbox) const; (+NoIsolate 变体)
// void set_foreign_address(IsolateForSandbox, Address);
// DECL_VERIFIER(Foreign) / BodyDescriptor (ExternalPointer 外部指针体描述)
pub const TRUSTED_FOREIGN_POINTER_KIND: &str = "RawPtr";

// —— 提取不变式断言 ——
const _: () = {
  assert!(API_EXPOSED_INSTANCE_TYPE == 0xcc);
  assert!(FOREIGN_ACCESSORS.len() == 4);
};
