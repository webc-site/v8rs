// 由 tool/src/builtins/frames.tq.js 从 v8/src/builtins/frames.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 22 条)
pub const DECLS: &[TqDecl] = &[
  (
    "type",
    "FrameType",
    "type FrameType extends Smi constexpr 'StackFrame::Type'",
  ),
  (
    "const",
    "STUB_FRAME",
    "const STUB_FRAME: constexpr FrameType generates 'StackFrame::STUB'",
  ),
  (
    "const",
    "kFrameTypeCount",
    "const kFrameTypeCount: constexpr int31 generates 'StackFrame::NUMBER_OF_TYPES'",
  ),
  (
    "type",
    "FrameBase",
    "type FrameBase extends RawPtr constexpr 'void*'",
  ),
  (
    "type",
    "StandardFrame",
    "type StandardFrame extends FrameBase constexpr 'void*'",
  ),
  (
    "type",
    "StubFrame",
    "type StubFrame extends FrameBase constexpr 'void*'",
  ),
  (
    "type",
    "FrameWithArguments",
    "type FrameWithArguments = StandardFrame",
  ),
  ("type", "Frame", "type Frame = FrameWithArguments|StubFrame"),
  (
    "macro",
    "LoadFramePointer",
    "macro LoadFramePointer(): Frame",
  ),
  (
    "macro",
    "LoadParentFramePointer",
    "macro LoadParentFramePointer(): Frame",
  ),
  (
    "macro",
    "StackSlotPtr",
    "macro StackSlotPtr(constexpr int32, constexpr int32): RawPtr",
  ),
  (
    "macro",
    "LoadObjectFromFrame",
    "macro LoadObjectFromFrame(f: Frame, o: constexpr int32): Object",
  ),
  (
    "macro",
    "LoadPointerFromFrame",
    "macro LoadPointerFromFrame(f: Frame, o: constexpr int32): RawPtr",
  ),
  (
    "macro",
    "LoadIntptrFromFrame",
    "macro LoadIntptrFromFrame(f: Frame, o: constexpr int32): intptr",
  ),
  (
    "const",
    "kStandardFrameFunctionOffset",
    "const kStandardFrameFunctionOffset: constexpr int31 generates 'StandardFrameConstants::kFunctionOffset'",
  ),
  (
    "const",
    "kStandardFrameCallerFPOffset",
    "const kStandardFrameCallerFPOffset: constexpr int31 generates 'StandardFrameConstants::kCallerFPOffset'",
  ),
  (
    "const",
    "kStandardFrameArgCOffset",
    "const kStandardFrameArgCOffset: constexpr int31 generates 'StandardFrameConstants::kArgCOffset'",
  ),
  (
    "const",
    "kJSArgcReceiverSlots",
    "const kJSArgcReceiverSlots: constexpr int31 generates 'kJSArgcReceiverSlots'",
  ),
  (
    "type",
    "ContextOrFrameType",
    "type ContextOrFrameType = Context|FrameType",
  ),
  (
    "const",
    "kStandardFrameContextOrFrameTypeOffset",
    "const kStandardFrameContextOrFrameTypeOffset: constexpr int31 generates 'StandardFrameConstants::kContextOrFrameTypeOffset'",
  ),
  (
    "macro",
    "Cast",
    "macro Cast<A : type extends Frame>( implicit context: Context)(o: Frame): A labels CastError",
  ),
  (
    "macro",
    "LoadTargetFromFrame",
    "macro LoadTargetFromFrame(): JSFunction",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  ("STUB_FRAME", "StackFrame::STUB"),
  ("kFrameTypeCount", "StackFrame::NUMBER_OF_TYPES"),
  (
    "kStandardFrameFunctionOffset",
    "StandardFrameConstants::kFunctionOffset",
  ),
  (
    "kStandardFrameCallerFPOffset",
    "StandardFrameConstants::kCallerFPOffset",
  ),
  (
    "kStandardFrameArgCOffset",
    "StandardFrameConstants::kArgCOffset",
  ),
  ("kJSArgcReceiverSlots", "kJSArgcReceiverSlots"),
  (
    "kStandardFrameContextOrFrameTypeOffset",
    "StandardFrameConstants::kContextOrFrameTypeOffset",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 22);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 8);
};
