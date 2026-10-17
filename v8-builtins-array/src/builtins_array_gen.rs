// 由 tool/src/builtins/builtins-array-gen.js 从 v8/src/builtins/builtins-array-gen.cc + src/builtins/builtins-array-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-array-gen.cc, src/builtins/builtins-array-gen.h)
//! 任务映射: v8-builtins-array; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 28 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayPrototypePop",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayPrototypePush",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ExtractFastJSArray",
    assembler: "ArrayBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CloneFastJSArray",
    assembler: "ArrayBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CloneFastJSArrayFillingHoles",
    assembler: "ArrayBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "TypedArrayPrototypeMap",
    assembler: "ArrayBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIncludes",
    assembler: "ArrayIncludesIndexofAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIncludesSmi",
    assembler: "ArrayIncludesIndexofAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIncludesSmiOrObject",
    assembler: "ArrayIncludesIndexofAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIncludesPackedDoubles",
    assembler: "ArrayIncludesIndexofAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIncludesHoleyDoubles",
    assembler: "ArrayIncludesIndexofAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIndexOf",
    assembler: "ArrayIncludesIndexofAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIndexOfSmi",
    assembler: "ArrayIncludesIndexofAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIndexOfSmiOrObject",
    assembler: "ArrayIncludesIndexofAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIndexOfPackedDoubles",
    assembler: "ArrayIncludesIndexofAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIndexOfHoleyDoubles",
    assembler: "ArrayIncludesIndexofAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayPrototypeValues",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayPrototypeEntries",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayPrototypeKeys",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayIteratorPrototypeNext",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayConstructor",
    assembler: "ArrayBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayConstructorImpl",
    assembler: "ArrayBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ArrayNArgumentsConstructor",
    assembler: "ArrayBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Array",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CreateArrayFromSlowBoilerplate",
    assembler: "SlowBoilerplateCloneAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CreateObjectFromSlowBoilerplate",
    assembler: "SlowBoilerplateCloneAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CreateArrayFromSlowBoilerplateHelper",
    assembler: "SlowBoilerplateCloneAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CreateObjectFromSlowBoilerplateHelper",
    assembler: "SlowBoilerplateCloneAssembler",
  },
];

/// 原版 enum SearchVariant 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchVariant {
  /// kIncludes
  Includes,
  /// kIndexOf
  IndexOf,
}

/// 原版 enum SimpleElementKind 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleElementKind {
  /// kSmiOrHole
  SmiOrHole,
  /// kAny
  Any,
}

/// 原版 enum ArrayFromAsyncIterableResolveContextSlots 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayFromAsyncIterableResolveContextSlots {
  /// kArrayFromAsyncIterableResolveResumeStateStepSlot = Context::MIN_CONTEXT_SLOTS
  ArrayFromAsyncIterableResolveResumeStateStepSlot,
  /// kArrayFromAsyncIterableResolveResumeStateAwaitedValueSlot
  ArrayFromAsyncIterableResolveResumeStateAwaitedValueSlot,
  /// kArrayFromAsyncIterableResolveResumeStateIndexSlot
  ArrayFromAsyncIterableResolveResumeStateIndexSlot,
  /// kArrayFromAsyncIterableResolvePromiseSlot
  ArrayFromAsyncIterableResolvePromiseSlot,
  /// kArrayFromAsyncIterableResolvePromiseFunctionSlot
  ArrayFromAsyncIterableResolvePromiseFunctionSlot,
  /// kArrayFromAsyncIterableResolveOnFulfilledFunctionSlot
  ArrayFromAsyncIterableResolveOnFulfilledFunctionSlot,
  /// kArrayFromAsyncIterableResolveOnRejectedFunctionSlot
  ArrayFromAsyncIterableResolveOnRejectedFunctionSlot,
  /// kArrayFromAsyncIterableResolveResultArraySlot
  ArrayFromAsyncIterableResolveResultArraySlot,
  /// kArrayFromAsyncIterableResolveIteratorSlot
  ArrayFromAsyncIterableResolveIteratorSlot,
  /// kArrayFromAsyncIterableResolveNextMethodSlot
  ArrayFromAsyncIterableResolveNextMethodSlot,
  /// kArrayFromAsyncIterableResolveErrorSlot
  ArrayFromAsyncIterableResolveErrorSlot,
  /// kArrayFromAsyncIterableResolveMapfnSlot
  ArrayFromAsyncIterableResolveMapfnSlot,
  /// kArrayFromAsyncIterableResolveThisArgSlot
  ArrayFromAsyncIterableResolveThisArgSlot,
  /// kArrayFromAsyncIterableResolveLength
  ArrayFromAsyncIterableResolveLength,
}

/// 原版 enum ArrayFromAsyncArrayLikeResolveContextSlots 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayFromAsyncArrayLikeResolveContextSlots {
  /// kArrayFromAsyncArrayLikeResolveResumeStateStepSlot = Context::MIN_CONTEXT_SLOTS
  ArrayFromAsyncArrayLikeResolveResumeStateStepSlot,
  /// kArrayFromAsyncArrayLikeResolveResumeStateAwaitedValueSlot
  ArrayFromAsyncArrayLikeResolveResumeStateAwaitedValueSlot,
  /// kArrayFromAsyncArrayLikeResolveResumeStateLenSlot
  ArrayFromAsyncArrayLikeResolveResumeStateLenSlot,
  /// kArrayFromAsyncArrayLikeResolveResumeStateIndexSlot
  ArrayFromAsyncArrayLikeResolveResumeStateIndexSlot,
  /// kArrayFromAsyncArrayLikeResolvePromiseSlot
  ArrayFromAsyncArrayLikeResolvePromiseSlot,
  /// kArrayFromAsyncArrayLikeResolvePromiseFunctionSlot
  ArrayFromAsyncArrayLikeResolvePromiseFunctionSlot,
  /// kArrayFromAsyncArrayLikeResolveOnFulfilledFunctionSlot
  ArrayFromAsyncArrayLikeResolveOnFulfilledFunctionSlot,
  /// kArrayFromAsyncArrayLikeResolveOnRejectedFunctionSlot
  ArrayFromAsyncArrayLikeResolveOnRejectedFunctionSlot,
  /// kArrayFromAsyncArrayLikeResolveResultArraySlot
  ArrayFromAsyncArrayLikeResolveResultArraySlot,
  /// kArrayFromAsyncArrayLikeResolveArrayLikeSlot
  ArrayFromAsyncArrayLikeResolveArrayLikeSlot,
  /// kArrayFromAsyncArrayLikeResolveErrorSlot
  ArrayFromAsyncArrayLikeResolveErrorSlot,
  /// kArrayFromAsyncArrayLikeResolveMapfnSlot
  ArrayFromAsyncArrayLikeResolveMapfnSlot,
  /// kArrayFromAsyncArrayLikeResolveThisArgSlot
  ArrayFromAsyncArrayLikeResolveThisArgSlot,
  /// kArrayFromAsyncArrayLikeResolveLength
  ArrayFromAsyncArrayLikeResolveLength,
}

/// 原版 enum ArrayFromAsyncLabels 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayFromAsyncLabels {
  /// kGetIteratorStep
  GetIteratorStep,
  /// kCheckIteratorValueAndMapping
  CheckIteratorValueAndMapping,
  /// kIteratorMapping
  IteratorMapping,
  /// kGetIteratorValueWithMapping
  GetIteratorValueWithMapping,
  /// kAddIteratorValueToTheArray
  AddIteratorValueToTheArray,
  /// kGetArrayLikeValue
  GetArrayLikeValue,
  /// kCheckArrayLikeValueAndMapping
  CheckArrayLikeValueAndMapping,
  /// kGetArrayLikeValueWithMapping
  GetArrayLikeValueWithMapping,
  /// kAddArrayLikeValueToTheArray
  AddArrayLikeValueToTheArray,
  /// kDoneAndResolvePromise
  DoneAndResolvePromise,
  /// kCloseAsyncIterator
  CloseAsyncIterator,
  /// kRejectPromise
  RejectPromise,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[
  ("ArrayPopulatorAssembler", "CodeStubAssembler"),
  ("ArrayIncludesIndexofAssembler", "CodeStubAssembler"),
  ("SimpleElementKind", ""),
  ("SlowBoilerplateCloneAssembler", "CodeStubAssembler"),
  ("ArrayBuiltinsAssembler", "CodeStubAssembler"),
  ("ArrayBuiltins", ""),
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 28);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 6);
};
