// 由 tool/src/builtins/builtins-utils-inl.js 从 v8/src/builtins/builtins-utils-inl.h 生成; 修订就地改, 复跑覆盖
// BuiltinArguments 内联方法签名骨架, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线另归 v8-builtins-core (并存不覆盖)

/// 方法签名条目
#[derive(Debug, Clone, Copy)]
pub struct MethodSig {
  pub ret: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整)
  pub sig: &'static str,
}

/// BuiltinArguments 内联方法清单 (共 4 条)
pub const METHODS: &[MethodSig] = &[
  MethodSig {
    ret: "Handle<Object>",
    name: "atOrUndefined",
    sig: "Handle<Object> BuiltinArguments::atOrUndefined(Isolate* isolate, int index) const",
  },
  MethodSig {
    ret: "Handle<JSAny>",
    name: "receiver",
    sig: "Handle<JSAny> BuiltinArguments::receiver() const",
  },
  MethodSig {
    ret: "Handle<JSFunction>",
    name: "target",
    sig: "Handle<JSFunction> BuiltinArguments::target() const",
  },
  MethodSig {
    ret: "Handle<HeapObject>",
    name: "new_target",
    sig: "Handle<HeapObject> BuiltinArguments::new_target() const",
  },
];

// —— 对数断言 ——
const _: () = {
  assert!(METHODS.len() == 4);
  assert!(METHODS[0].name == "atOrUndefined");
};
