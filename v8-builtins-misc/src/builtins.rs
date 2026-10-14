// 由 tool/src/builtins/builtins.js 从 v8/src/builtins/builtins-inl.h + src/builtins/builtins.cc + src/builtins/builtins.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-inl.h, src/builtins/builtins.cc, src/builtins/builtins.h)
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 0 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[];

/// 原版 enum Builtin : int32_t 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Builtin {
  /// kNoBuiltinId = -1
  NoBuiltinId,
  /// DEF_ENUM
  DEFENUM,
  /// kFirstBytecodeHandler = FirstFromVarArgs( BUILTIN_LIST_BYTECODE_HANDLERS(EXTRACT_NAME
  FirstBytecodeHandler,
  /// kFirstLoadICHandler = FirstFromVarArgs(BUILTIN_LOAD_IC_HANDLER_LIST(EXTRACT_NAME) 0)
  FirstLoadICHandler,
}

/// 原版 enum TieringBuiltin : int32_t 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TieringBuiltin {}

/// 原版 enum Kind 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
  /// CPP
  CPP,
  /// TFJ_TSA
  TFJTSA,
  /// TFJ
  TFJ,
  /// TFC_TSA
  TFCTSA,
  /// TFC
  TFC,
  /// TFS
  TFS,
  /// TFH
  TFH,
  /// BCH_TSA
  BCHTSA,
  /// BCH
  BCH,
  /// ASM
  ASM,
}

/// 原版 enum ForwardWhichFrame 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForwardWhichFrame {
  /// kCurrentFrame
  CurrentFrame,
  /// kParentFrame
  ParentFrame,
}

/// 原版 enum JSBuiltinStateFlag 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JSBuiltinStateFlag {
  /// kNone = 0
  None,
  /// kEnabled = 1 << 0
  Enabled,
  /// kCoreV8JSTrampoline = 1 << 1
  CoreV8JSTrampoline,
  /// kCoreJS = 1 << 2
  CoreJS,
  /// kFlagDependent = 1 << 3
  FlagDependent,
  /// kLazy = 1 << 4
  Lazy,
  /// kNonJSLinkage = 1 << 5
  NonJSLinkage,
  /// kJSTrampoline = kEnabled | kCoreV8JSTrampoline
  JSTrampoline,
  /// kCoreJSMandatory = kEnabled | kCoreJS
  CoreJSMandatory,
  /// kCoreJSLazy = kEnabled | kCoreJS | kLazy
  CoreJSLazy,
  /// kDisabledJSBuiltin = kNone
  DisabledJSBuiltin,
  /// kDisabledNonJSBuiltin = kNonJSLinkage
  DisabledNonJSBuiltin,
  /// kEnabledFlagDependent = kEnabled | kFlagDependent
  EnabledFlagDependent,
  /// kDisabledFlagDependent = kFlagDependent
  DisabledFlagDependent,
  /// kEnabledFlagDependentLazy = kEnabled | kFlagDependent | kLazy
  EnabledFlagDependentLazy,
  /// kDisabledFlagDependentLazy = kFlagDependent | kLazy
  DisabledFlagDependentLazy,
}

/// 原版 enum CallOrConstructMode 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallOrConstructMode {
  /// kCall
  Call,
  /// kConstruct
  Construct,
}

/// 原版 enum InterpreterEntryTrampolineMode 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InterpreterEntryTrampolineMode {
  /// kDefault
  Default,
  /// kForProfiling
  ForProfiling,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[
  ("Builtins", ""),
  ("ForwardWhichFrame", ""),
  ("JSBuiltinStateFlag", ""),
  ("CallOrConstructMode", ""),
  ("InterpreterEntryTrampolineMode", ""),
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 0);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 5);
};
