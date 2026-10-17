// 由 tool/src/objects/megadom-handler.h.js 从 v8/src/objects/megadom-handler.{h,-inl.h,tq} 生成; 修订就地改, 复跑覆盖
// MegaDomHandler 布局字段表
//! MegaDomHandler: mega-dom 桩访问器对 (accessor + context, 均为弱引用位 MaybeObject)。

/// 布局字段表 (megadom-handler.tq, 顺序 1:1)。
pub const MEGADOM_HANDLER_FIELDS: [(&str, &str); 2] = [
  ("accessor", "MaybeObject"),
  ("context", "MaybeObject"),
];

/// 字段槽总数。
pub const FIELD_COUNT: usize = 2;

// —— 提取不变式断言 ——
const _: () = {
  assert!(FIELD_COUNT == 2);
  assert!(MEGADOM_HANDLER_FIELDS[0] == ("accessor", "MaybeObject"));
  assert!(MEGADOM_HANDLER_FIELDS[1] == ("context", "MaybeObject"));
};
