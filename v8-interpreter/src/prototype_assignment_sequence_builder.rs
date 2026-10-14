// 由 tool/src/interpreter/prototype-assignment-sequence-builder.h.js 从 v8/src/interpreter/prototype-assignment-sequence-builder.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 原型赋值项 (原版 PrototypeAssignment = <AstRawString*, Expression*> 对; 表达式侧属 ast crate)
pub type PrototypeAssignmentKey = [u8; 0]; // 占位: 键为内部化字符串索引, 修订阶段接入

/// 原版 class/struct ProtoAssignmentSeqBuilder 成员声明 (机械 1:1 提取, 10 条)
pub const PROTO_ASSIGNMENT_SEQ_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] explicit ProtoAssignmentSeqBuilder( ZoneVector<PrototypeAssignment>&& properties) : properties_(std::move(properties))",
  "[public/method] ~ProtoAssignmentSeqBuilder() = default",
  "[public/method] ProtoAssignmentSeqBuilder(const ProtoAssignmentSeqBuilder& other) = delete",
  "[public/method] ProtoAssignmentSeqBuilder(ProtoAssignmentSeqBuilder&& other) V8_NOEXCEPT = delete",
  "[public/method] ProtoAssignmentSeqBuilder& operator=(const ProtoAssignmentSeqBuilder& other) V8_NOEXCEPT = delete",
  "[public/method] ProtoAssignmentSeqBuilder& operator=(ProtoAssignmentSeqBuilder&& other) V8_NOEXCEPT = delete",
  "[public/method] DirectHandle<ObjectBoilerplateDescription> boilerplate_description() const",
  "[public/method] const ZoneVector<PrototypeAssignment>* properties()",
  "[private/field] ZoneVector<PrototypeAssignment> properties_",
  "[private/field] IndirectHandle<ObjectBoilerplateDescription> boilerplate_description_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(PROTO_ASSIGNMENT_SEQ_BUILDER_DECLARATIONS.len() == 10);
};
