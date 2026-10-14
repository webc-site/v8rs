#!/usr/bin/env bun
// tool/src/codegen/code-factory.h.js
// 机械提取并生成 v8/src/codegen/code-factory.{h,cc}: AllocationSiteOverrideMode 枚举 + 工厂方法核心实现
// → v8-codegen-core/src/code_factory.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/code-factory.h");
const ccSrc = readV8("src/codegen/code-factory.cc");

// 枚举对数
if (!hSrc.includes("enum AllocationSiteOverrideMode {")) throw new Error("缺 AllocationSiteOverrideMode");
for (const v of ["DONT_OVERRIDE", "DISABLE_ALLOCATION_SITES"]) {
  if (!hSrc.includes(v)) throw new Error(`缺枚举值 ${v}`);
}

// 静态工厂清点 (对数): 逐个在场校验
const factories = [
  ["RuntimeCEntry", 3], ["CEntry", 4], ["LoadGlobalIC", 2], ["LoadGlobalICInOptimizedCode", 2],
  ["DefineNamedOwnIC", 1], ["DefineNamedOwnICInOptimizedCode", 1], ["CallApiCallback", 1],
  ["StringAdd", 2], ["FastNewFunctionContext", 2], ["Call", 2], ["Call_WithFeedback", 2],
  ["CallWithArrayLike", 1], ["CallWithSpread", 1], ["CallFunction", 2],
  ["CallForwardVarargs", 1], ["CallFunctionForwardVarargs", 1], ["Construct", 1],
  ["ConstructWithSpread", 1], ["ConstructForwardVarargs", 1], ["ConstructFunctionForwardVarargs", 1],
  ["ArrayNoArgumentConstructor", 3], ["ArraySingleArgumentConstructor", 3],
];
for (const [name] of factories) {
  const re = new RegExp(`static \\w+[\\w:<>* ]* ${name}\\(`);
  if (!re.test(hSrc)) throw new Error(`缺工厂 ${name}`);
  if (name !== "CallApiCallback" && !ccSrc.includes(`CodeFactory::${name}(`))
    throw new Error(`.cc 缺 ${name}`);
}

// TSAN 分支登记 (V8_IS_TSAN, 本目标不启用)
const tsanBranch = hSrc.includes("#ifdef V8_IS_TSAN");

let rust = banner(
  "src/codegen/code-factory.h.js",
  "src/codegen/code-factory.{h,cc}",
  "CodeFactory 工厂实现 + AllocationSiteOverrideMode",
);

rust += `
use crate::machine_type::Callable;

/// 原版 enum AllocationSiteOverrideMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AllocationSiteOverrideMode {
  DontOverride = 0,
  DisableAllocationSites = 1,
}

pub const DONT_OVERRIDE: AllocationSiteOverrideMode = AllocationSiteOverrideMode::DontOverride;
pub const DISABLE_ALLOCATION_SITES: AllocationSiteOverrideMode =
  AllocationSiteOverrideMode::DisableAllocationSites;

/// 原版 enum ScopeType (v8/src/common/globals.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ScopeType {
  Script = 0,
  ReplMode = 1,
  Eval = 2,
  Module = 3,
  Class = 4,
  Function = 5,
  Catch = 6,
  Block = 7,
  With = 8,
  ShadowRealm = 9,
}

/// 原版 enum ConvertReceiverMode (v8/src/common/globals.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConvertReceiverMode {
  NullOrUndefined = 0,
  NotNullOrUndefined = 1,
  Any = 2,
}

/// 原版 enum class TypeofMode (v8/src/common/globals.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TypeofMode {
  Inside = 0,
  NotInside = 1,
}

/// 原版 enum StringAddFlags (v8/src/objects/type-hints.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StringAddFlags {
  CheckNone = 0,
  ConvertLeft = 1,
  ConvertRight = 2,
}

/// 原版 enum class ArgvMode (v8/src/common/globals.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArgvMode {
  Stack = 0,
  Register = 1,
}

/// 原版 enum ElementsKind (v8/src/objects/elements-kind.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElementsKind {
  PackedSmi = 0,
  HoleySmi = 1,
  Packed = 2,
  Holey = 3,
  PackedDouble = 4,
  HoleyDouble = 5,
}

/// CodeFactory 分派关联的 Builtin ID 标识
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum BuiltinId {
  CEntry = 1,
  WasmCEntry,
  CEntryReturn1ArgvOnStackNoBuiltinExit,
  CEntryReturn1ArgvOnStackBuiltinExit,
  CEntryReturn1ArgvInRegisterNoBuiltinExit,
  CEntryReturn2ArgvOnStackNoBuiltinExit,
  CEntryReturn2ArgvInRegisterNoBuiltinExit,
  LoadGlobalIC,
  LoadGlobalICInsideTypeof,
  LoadGlobalICTrampoline,
  LoadGlobalICInsideTypeofTrampoline,
  LoadGlobalICInOptimizedCode,
  DefineNamedOwnIC,
  DefineNamedOwnICTrampoline,
  CallApiCallback,
  StringAdd,
  StringAddNoMapCheck,
  StringAddConvertLeft,
  StringAddConvertRight,
  FastNewFunctionContextEval,
  FastNewFunctionContextFunction,
  CallAny,
  CallNullOrUndefined,
  CallNotNullOrUndefined,
  CallWithFeedbackAny,
  CallWithFeedbackNullOrUndefined,
  CallWithFeedbackNotNullOrUndefined,
  CallWithArrayLike,
  CallWithSpread,
  CallFunction,
  CallFunctionAny,
  CallFunctionNullOrUndefined,
  CallFunctionNotNullOrUndefined,
  CallForwardVarargs,
  CallFunctionForwardVarargs,
  Construct,
  ConstructWithSpread,
  ConstructForwardVarargs,
  ConstructFunctionForwardVarargs,
  ArrayNoArgumentConstructorPackedSmiDontOverride,
  ArrayNoArgumentConstructorHoleySmiDontOverride,
  ArrayNoArgumentConstructorPackedSmiDisableAllocationSites,
  ArrayNoArgumentConstructorHoleySmiDisableAllocationSites,
  ArrayNoArgumentConstructorPackedDisableAllocationSites,
  ArrayNoArgumentConstructorHoleyDisableAllocationSites,
  ArrayNoArgumentConstructorPackedDoubleDisableAllocationSites,
  ArrayNoArgumentConstructorHoleyDoubleDisableAllocationSites,
  ArraySingleArgumentConstructorPackedSmiDontOverride,
  ArraySingleArgumentConstructorHoleySmiDontOverride,
  ArraySingleArgumentConstructorPackedSmiDisableAllocationSites,
  ArraySingleArgumentConstructorHoleySmiDisableAllocationSites,
  ArraySingleArgumentConstructorPackedDisableAllocationSites,
  ArraySingleArgumentConstructorHoleyDisableAllocationSites,
  ArraySingleArgumentConstructorPackedDoubleDisableAllocationSites,
  ArraySingleArgumentConstructorHoleyDoubleDisableAllocationSites,
}

/// 原版 CodeFactory: builtin 工厂 (AllStatic)。
pub struct CodeFactory;

/// 原版 allocation-site-inl.h AllocationSite::ShouldTrack:
/// V8_ALLOCATION_SITE_TRACKING_BOOL(v8_allocation_site_tracking) && IsSmiElementsKind(kind)。
/// 基线假定 flag 为 true (flag 读取推迟, 已登记 README 台账); IsSmiElementsKind = PACKED_SMI/HOLEY_SMI。
#[must_use]
const fn allocation_site_should_track(elements_kind: i32) -> bool {
  elements_kind == ElementsKind::PackedSmi as i32 || elements_kind == ElementsKind::HoleySmi as i32
}

impl CodeFactory {
  /// v8: src/codegen/code-factory.cc CodeFactory::RuntimeCEntry
  /// 原版: return CodeFactory::CEntry(isolate, result_size, ArgvMode::kStack, false, switch_to_central_stack);
  #[must_use]
  pub fn runtime_c_entry(
    isolate: usize,
    result_size: i32,
    switch_to_central_stack: bool,
  ) -> Callable {
    Self::c_entry(
      isolate,
      result_size,
      ArgvMode::Stack as i32,
      false,
      switch_to_central_stack,
    )
  }

  /// v8: src/codegen/code-factory.cc CodeFactory::CEntry
  /// 分派 1:1 对照 v8: src/builtins/builtins-inl.h Builtins::CEntry (Builtin 数值待 v8-builtins 基线接线)
  #[must_use]
  pub fn c_entry(
    _isolate: usize,
    result_size: i32,
    argv_mode: i32,
    builtin_exit_frame: bool,
    switch_to_central_stack: bool,
  ) -> Callable {
    let builtin = if switch_to_central_stack {
      // 原版: DCHECK_EQ(result_size, 1); DCHECK_EQ(argv_mode, ArgvMode::kStack); DCHECK_EQ(builtin_exit_frame, false);
      debug_assert!(result_size == 1, "central stack 要求 result_size == 1");
      debug_assert!(
        argv_mode == ArgvMode::Stack as i32,
        "central stack 要求 argv_mode == kStack"
      );
      debug_assert!(!builtin_exit_frame, "central stack 要求 !builtin_exit_frame");
      BuiltinId::WasmCEntry
    } else if result_size == 1 && argv_mode == ArgvMode::Stack as i32 && !builtin_exit_frame {
      BuiltinId::CEntryReturn1ArgvOnStackNoBuiltinExit
    } else if result_size == 1 && argv_mode == ArgvMode::Stack as i32 && builtin_exit_frame {
      BuiltinId::CEntryReturn1ArgvOnStackBuiltinExit
    } else if result_size == 1 && argv_mode == ArgvMode::Register as i32 && !builtin_exit_frame {
      BuiltinId::CEntryReturn1ArgvInRegisterNoBuiltinExit
    } else if result_size == 2 && argv_mode == ArgvMode::Stack as i32 && !builtin_exit_frame {
      BuiltinId::CEntryReturn2ArgvOnStackNoBuiltinExit
    } else if result_size == 2 && argv_mode == ArgvMode::Register as i32 && !builtin_exit_frame {
      BuiltinId::CEntryReturn2ArgvInRegisterNoBuiltinExit
    } else {
      // 原版 UNREACHABLE()
      unreachable!("CEntry 非法参数组合 (原版 UNREACHABLE)")
    };
    Callable::new(builtin as u32, 0)
  }

  /// 默认参数调用 CEntry (result_size = 1, argv_mode = Stack, builtin_exit = false, switch_central = false)
  #[must_use]
  pub fn c_entry_default(isolate: usize) -> Callable {
    Self::c_entry(isolate, 1, ArgvMode::Stack as i32, false, false)
  }

  /// 默认参数调用 RuntimeCEntry (result_size = 1, switch_central = false)
  #[must_use]
  pub fn runtime_c_entry_default(isolate: usize) -> Callable {
    Self::runtime_c_entry(isolate, 1, false)
  }

  /// v8: src/codegen/code-factory.cc CodeFactory::LoadGlobalIC
  /// 1:1 对照 Builtins::LoadGlobalIC: kNotInside ? kLoadGlobalICTrampoline : kLoadGlobalICInsideTypeofTrampoline
  #[must_use]
  pub fn load_global_ic(_isolate: usize, typeof_mode: i32) -> Callable {
    let builtin = match typeof_mode {
      m if m == TypeofMode::NotInside as i32 => BuiltinId::LoadGlobalICTrampoline,
      m if m == TypeofMode::Inside as i32 => BuiltinId::LoadGlobalICInsideTypeofTrampoline,
      // 原版 switch 穷尽后 UNREACHABLE()
      _ => unreachable!("LoadGlobalIC 非法 typeof_mode (原版 UNREACHABLE)"),
    };
    Callable::new(builtin as u32, 0)
  }

  /// v8: src/codegen/code-factory.cc CodeFactory::LoadGlobalICInOptimizedCode
  /// 1:1 对照 Builtins::LoadGlobalICInOptimizedCode: kNotInside ? kLoadGlobalIC : kLoadGlobalICInsideTypeof
  #[must_use]
  pub fn load_global_ic_in_optimized_code(_isolate: usize, typeof_mode: i32) -> Callable {
    let builtin = match typeof_mode {
      m if m == TypeofMode::NotInside as i32 => BuiltinId::LoadGlobalIC,
      m if m == TypeofMode::Inside as i32 => BuiltinId::LoadGlobalICInsideTypeof,
      _ => unreachable!("LoadGlobalICInOptimizedCode 非法 typeof_mode (原版 UNREACHABLE)"),
    };
    Callable::new(builtin as u32, 0)
  }

  /// 原版 CodeFactory::DefineNamedOwnIC
  #[must_use]
  pub fn define_named_own_ic(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::DefineNamedOwnICTrampoline as u32, 0)
  }

  /// 原版 CodeFactory::DefineNamedOwnICInOptimizedCode
  #[must_use]
  pub fn define_named_own_ic_in_optimized_code(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::DefineNamedOwnIC as u32, 0)
  }

  /// 原版 CodeFactory::CallApiCallback
  #[must_use]
  pub fn call_api_callback(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::CallApiCallback as u32, 0)
  }

  /// v8: src/codegen/code-factory.cc CodeFactory::StringAdd
  /// 1:1 对照 Builtins::StringAdd (builtins-inl.h): CHECK_NONE/CONVERT_LEFT/CONVERT_RIGHT 三档, 穷尽后 UNREACHABLE
  #[must_use]
  pub fn string_add(_isolate: usize, flags: u32) -> Callable {
    let builtin = match flags {
      f if f == StringAddFlags::CheckNone as u32 => BuiltinId::StringAddNoMapCheck,
      f if f == StringAddFlags::ConvertLeft as u32 => BuiltinId::StringAddConvertLeft,
      f if f == StringAddFlags::ConvertRight as u32 => BuiltinId::StringAddConvertRight,
      _ => unreachable!("StringAdd 非法 flags (原版 UNREACHABLE)"),
    };
    Callable::new(builtin as u32, 0)
  }

  /// 默认参数调用 StringAdd (StringAddFlags::CheckNone)
  #[must_use]
  pub fn string_add_default(isolate: usize) -> Callable {
    Self::string_add(isolate, StringAddFlags::CheckNone as u32)
  }

  /// v8: src/codegen/code-factory.cc CodeFactory::FastNewFunctionContext
  /// EVAL_SCOPE(2)/FUNCTION_SCOPE(5) 两档, default 为原版 UNREACHABLE()
  #[must_use]
  pub fn fast_new_function_context(_isolate: usize, scope_type: i32) -> Callable {
    let builtin = match scope_type {
      s if s == ScopeType::Eval as i32 => BuiltinId::FastNewFunctionContextEval,
      s if s == ScopeType::Function as i32 => BuiltinId::FastNewFunctionContextFunction,
      _ => unreachable!("FastNewFunctionContext 非法 scope_type (原版 UNREACHABLE)"),
    };
    Callable::new(builtin as u32, 0)
  }

  /// v8: src/codegen/code-factory.cc CodeFactory::Call
  /// 1:1 对照 Builtins::Call (builtins-inl.h): 三档 receiver 模式, 穷尽后 UNREACHABLE
  #[must_use]
  pub fn call(_isolate: usize, mode: i32) -> Callable {
    let builtin = match mode {
      m if m == ConvertReceiverMode::NullOrUndefined as i32 => BuiltinId::CallNullOrUndefined,
      m if m == ConvertReceiverMode::NotNullOrUndefined as i32 => BuiltinId::CallNotNullOrUndefined,
      m if m == ConvertReceiverMode::Any as i32 => BuiltinId::CallAny,
      _ => unreachable!("Call 非法 receiver_mode (原版 UNREACHABLE)"),
    };
    Callable::new(builtin as u32, 0)
  }

  /// 默认参数调用 Call (ConvertReceiverMode::Any)
  #[must_use]
  pub fn call_default(isolate: usize) -> Callable {
    Self::call(isolate, ConvertReceiverMode::Any as i32)
  }

  /// v8: src/codegen/code-factory.cc CodeFactory::Call_WithFeedback
  /// 三档 receiver 模式直接映射 *_WithFeedback builtin, 穷尽后 UNREACHABLE
  #[must_use]
  pub fn call_with_feedback(_isolate: usize, mode: i32) -> Callable {
    let builtin = match mode {
      m if m == ConvertReceiverMode::NullOrUndefined as i32 => {
        BuiltinId::CallWithFeedbackNullOrUndefined
      }
      m if m == ConvertReceiverMode::NotNullOrUndefined as i32 => {
        BuiltinId::CallWithFeedbackNotNullOrUndefined
      }
      m if m == ConvertReceiverMode::Any as i32 => BuiltinId::CallWithFeedbackAny,
      _ => unreachable!("Call_WithFeedback 非法 receiver_mode (原版 UNREACHABLE)"),
    };
    Callable::new(builtin as u32, 0)
  }

  /// 原版 CodeFactory::CallWithArrayLike
  #[must_use]
  pub fn call_with_array_like(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::CallWithArrayLike as u32, 0)
  }

  /// 原版 CodeFactory::CallWithSpread
  #[must_use]
  pub fn call_with_spread(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::CallWithSpread as u32, 0)
  }

  /// v8: src/codegen/code-factory.cc CodeFactory::CallFunction
  /// 1:1 对照 Builtins::CallFunction (builtins-inl.h): 三档 receiver 模式, 穷尽后 UNREACHABLE
  #[must_use]
  pub fn call_function(_isolate: usize, mode: i32) -> Callable {
    let builtin = match mode {
      m if m == ConvertReceiverMode::NullOrUndefined as i32 => {
        BuiltinId::CallFunctionNullOrUndefined
      }
      m if m == ConvertReceiverMode::NotNullOrUndefined as i32 => {
        BuiltinId::CallFunctionNotNullOrUndefined
      }
      m if m == ConvertReceiverMode::Any as i32 => BuiltinId::CallFunctionAny,
      _ => unreachable!("CallFunction 非法 receiver_mode (原版 UNREACHABLE)"),
    };
    Callable::new(builtin as u32, 0)
  }

  /// 默认参数调用 CallFunction (ConvertReceiverMode::Any)
  #[must_use]
  pub fn call_function_default(isolate: usize) -> Callable {
    Self::call_function(isolate, ConvertReceiverMode::Any as i32)
  }

  /// 原版 CodeFactory::CallForwardVarargs
  #[must_use]
  pub fn call_forward_varargs(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::CallForwardVarargs as u32, 0)
  }

  /// 原版 CodeFactory::CallFunctionForwardVarargs
  #[must_use]
  pub fn call_function_forward_varargs(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::CallFunctionForwardVarargs as u32, 0)
  }

  /// 原版 CodeFactory::Construct
  #[must_use]
  pub fn construct(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::Construct as u32, 0)
  }

  /// 原版 CodeFactory::ConstructWithSpread
  #[must_use]
  pub fn construct_with_spread(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::ConstructWithSpread as u32, 0)
  }

  /// 原版 CodeFactory::ConstructForwardVarargs
  #[must_use]
  pub fn construct_forward_varargs(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::ConstructForwardVarargs as u32, 0)
  }

  /// 原版 CodeFactory::ConstructFunctionForwardVarargs
  #[must_use]
  pub fn construct_function_forward_varargs(_isolate: usize) -> Callable {
    Callable::new(BuiltinId::ConstructFunctionForwardVarargs as u32, 0)
  }

  /// v8: src/codegen/code-factory.cc CodeFactory::ArrayNoArgumentConstructor
  /// 结构 1:1: DontOverride && ShouldTrack → 2 个 Smi 档 (DCHECK IsSmiElementsKind);
  /// 否则 → 6 档 DisableAllocationSites; 域外 UNREACHABLE()。
  #[must_use]
  pub fn array_no_argument_constructor(
    _isolate: usize,
    elements_kind: i32,
    override_mode: AllocationSiteOverrideMode,
  ) -> Callable {
    let builtin = if override_mode == AllocationSiteOverrideMode::DontOverride
      && allocation_site_should_track(elements_kind)
    {
      debug_assert!(
        allocation_site_should_track(elements_kind),
        "DontOverride 分支要求 Smi elements kind"
      );
      match elements_kind {
        k if k == ElementsKind::PackedSmi as i32 => {
          BuiltinId::ArrayNoArgumentConstructorPackedSmiDontOverride
        }
        k if k == ElementsKind::HoleySmi as i32 => {
          BuiltinId::ArrayNoArgumentConstructorHoleySmiDontOverride
        }
        _ => unreachable!("ArrayNoArgumentConstructor(DontOverride) 非 Smi kind (原版 UNREACHABLE)"),
      }
    } else {
      debug_assert!(
        override_mode == AllocationSiteOverrideMode::DisableAllocationSites
          || !allocation_site_should_track(elements_kind),
        "else 分支前置条件"
      );
      match elements_kind {
        k if k == ElementsKind::PackedSmi as i32 => {
          BuiltinId::ArrayNoArgumentConstructorPackedSmiDisableAllocationSites
        }
        k if k == ElementsKind::HoleySmi as i32 => {
          BuiltinId::ArrayNoArgumentConstructorHoleySmiDisableAllocationSites
        }
        k if k == ElementsKind::Packed as i32 => {
          BuiltinId::ArrayNoArgumentConstructorPackedDisableAllocationSites
        }
        k if k == ElementsKind::Holey as i32 => {
          BuiltinId::ArrayNoArgumentConstructorHoleyDisableAllocationSites
        }
        k if k == ElementsKind::PackedDouble as i32 => {
          BuiltinId::ArrayNoArgumentConstructorPackedDoubleDisableAllocationSites
        }
        k if k == ElementsKind::HoleyDouble as i32 => {
          BuiltinId::ArrayNoArgumentConstructorHoleyDoubleDisableAllocationSites
        }
        _ => unreachable!("ArrayNoArgumentConstructor 非法 elements_kind (原版 UNREACHABLE)"),
      }
    };
    Callable::new(builtin as u32, 0)
  }

  /// v8: src/codegen/code-factory.cc CodeFactory::ArraySingleArgumentConstructor
  /// 结构与 ArrayNoArgumentConstructor 完全同形, 仅 builtin 前缀不同。
  #[must_use]
  pub fn array_single_argument_constructor(
    _isolate: usize,
    elements_kind: i32,
    override_mode: AllocationSiteOverrideMode,
  ) -> Callable {
    let builtin = if override_mode == AllocationSiteOverrideMode::DontOverride
      && allocation_site_should_track(elements_kind)
    {
      debug_assert!(
        allocation_site_should_track(elements_kind),
        "DontOverride 分支要求 Smi elements kind"
      );
      match elements_kind {
        k if k == ElementsKind::PackedSmi as i32 => {
          BuiltinId::ArraySingleArgumentConstructorPackedSmiDontOverride
        }
        k if k == ElementsKind::HoleySmi as i32 => {
          BuiltinId::ArraySingleArgumentConstructorHoleySmiDontOverride
        }
        _ => {
          unreachable!("ArraySingleArgumentConstructor(DontOverride) 非 Smi kind (原版 UNREACHABLE)")
        }
      }
    } else {
      debug_assert!(
        override_mode == AllocationSiteOverrideMode::DisableAllocationSites
          || !allocation_site_should_track(elements_kind),
        "else 分支前置条件"
      );
      match elements_kind {
        k if k == ElementsKind::PackedSmi as i32 => {
          BuiltinId::ArraySingleArgumentConstructorPackedSmiDisableAllocationSites
        }
        k if k == ElementsKind::HoleySmi as i32 => {
          BuiltinId::ArraySingleArgumentConstructorHoleySmiDisableAllocationSites
        }
        k if k == ElementsKind::Packed as i32 => {
          BuiltinId::ArraySingleArgumentConstructorPackedDisableAllocationSites
        }
        k if k == ElementsKind::Holey as i32 => {
          BuiltinId::ArraySingleArgumentConstructorHoleyDisableAllocationSites
        }
        k if k == ElementsKind::PackedDouble as i32 => {
          BuiltinId::ArraySingleArgumentConstructorPackedDoubleDisableAllocationSites
        }
        k if k == ElementsKind::HoleyDouble as i32 => {
          BuiltinId::ArraySingleArgumentConstructorHoleyDoubleDisableAllocationSites
        }
        _ => unreachable!("ArraySingleArgumentConstructor 非法 elements_kind (原版 UNREACHABLE)"),
      }
    };
    Callable::new(builtin as u32, 0)
  }

  // TSAN 分支 (V8_IS_TSAN): 本目标不启用 TSAN, 登记: 未提取
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(AllocationSiteOverrideMode::DontOverride as u8 == 0);
  assert!(AllocationSiteOverrideMode::DisableAllocationSites as u8 == 1);
  assert!(ScopeType::Eval as u8 == 2);
  assert!(ScopeType::Function as u8 == 5);
  assert!(ConvertReceiverMode::NullOrUndefined as u8 == 0);
  assert!(ConvertReceiverMode::NotNullOrUndefined as u8 == 1);
  assert!(ConvertReceiverMode::Any as u8 == 2);
  assert!(TypeofMode::Inside as u8 == 0);
  assert!(TypeofMode::NotInside as u8 == 1);
  assert!(StringAddFlags::CheckNone as u8 == 0);
  assert!(StringAddFlags::ConvertLeft as u8 == 1);
  assert!(StringAddFlags::ConvertRight as u8 == 2);
  assert!(ArgvMode::Stack as u8 == 0);
  assert!(ArgvMode::Register as u8 == 1);
  assert!(ElementsKind::PackedSmi as u8 == 0);
  assert!(ElementsKind::HoleyDouble as u8 == 5);
};
`;

const p = writeSrc("v8-codegen-core", "code_factory.rs", rust);
console.log(`code-factory: 枚举 1 / 工厂 ${factories.length} (TSAN 分支登记: ${tsanBranch}) → ${p}`);
