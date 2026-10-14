// 由 tool/src/builtins/builtins-regexp.js 生成; 来源 v8/src/builtins 下 12 文件
// 内建名表 + 核心调度元数据 (37 内建)
// 来源: builtins-regexp.cc, builtins-regexp-gen.cc, builtins-regexp-gen.h, regexp-*.tq 等

use strum::{Display, EnumIter, EnumString};
use v8_builtins_core::{descriptor_name, find_builtin};

/// RegExp 核心内建名表 (37 项)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum Builtin {
  RegExpCapture1Getter,
  RegExpCapture2Getter,
  RegExpCapture3Getter,
  RegExpCapture4Getter,
  RegExpCapture5Getter,
  RegExpCapture6Getter,
  RegExpCapture7Getter,
  RegExpCapture8Getter,
  RegExpCapture9Getter,
  RegExpConstructor,
  RegExpEscape,
  RegExpExecAtom,
  RegExpExperimentalTrampoline,
  RegExpInputGetter,
  RegExpInputSetter,
  RegExpInterpreterTrampoline,
  RegExpLastMatchGetter,
  RegExpLastParenGetter,
  RegExpLeftContextGetter,
  RegExpMatchFast,
  RegExpPrototypeCompile,
  RegExpPrototypeExec,
  RegExpPrototypeExecSlow,
  RegExpPrototypeMatch,
  RegExpPrototypeMatchAll,
  RegExpPrototypeReplace,
  RegExpPrototypeSearch,
  RegExpPrototypeSourceGetter,
  RegExpPrototypeSplit,
  RegExpPrototypeTest,
  RegExpPrototypeTestFast,
  RegExpPrototypeToString,
  RegExpReplace,
  RegExpRightContextGetter,
  RegExpSearchFast,
  RegExpSplit,
  RegExpStringIteratorPrototypeNext,
}

impl Builtin {
  /// 获取内建的 V8 原版规范名称
  #[inline]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::RegExpCapture1Getter => "RegExpCapture1Getter",
      Self::RegExpCapture2Getter => "RegExpCapture2Getter",
      Self::RegExpCapture3Getter => "RegExpCapture3Getter",
      Self::RegExpCapture4Getter => "RegExpCapture4Getter",
      Self::RegExpCapture5Getter => "RegExpCapture5Getter",
      Self::RegExpCapture6Getter => "RegExpCapture6Getter",
      Self::RegExpCapture7Getter => "RegExpCapture7Getter",
      Self::RegExpCapture8Getter => "RegExpCapture8Getter",
      Self::RegExpCapture9Getter => "RegExpCapture9Getter",
      Self::RegExpConstructor => "RegExpConstructor",
      Self::RegExpEscape => "RegExpEscape",
      Self::RegExpExecAtom => "RegExpExecAtom",
      Self::RegExpExperimentalTrampoline => "RegExpExperimentalTrampoline",
      Self::RegExpInputGetter => "RegExpInputGetter",
      Self::RegExpInputSetter => "RegExpInputSetter",
      Self::RegExpInterpreterTrampoline => "RegExpInterpreterTrampoline",
      Self::RegExpLastMatchGetter => "RegExpLastMatchGetter",
      Self::RegExpLastParenGetter => "RegExpLastParenGetter",
      Self::RegExpLeftContextGetter => "RegExpLeftContextGetter",
      Self::RegExpMatchFast => "RegExpMatchFast",
      Self::RegExpPrototypeCompile => "RegExpPrototypeCompile",
      Self::RegExpPrototypeExec => "RegExpPrototypeExec",
      Self::RegExpPrototypeExecSlow => "RegExpPrototypeExecSlow",
      Self::RegExpPrototypeMatch => "RegExpPrototypeMatch",
      Self::RegExpPrototypeMatchAll => "RegExpPrototypeMatchAll",
      Self::RegExpPrototypeReplace => "RegExpPrototypeReplace",
      Self::RegExpPrototypeSearch => "RegExpPrototypeSearch",
      Self::RegExpPrototypeSourceGetter => "RegExpPrototypeSourceGetter",
      Self::RegExpPrototypeSplit => "RegExpPrototypeSplit",
      Self::RegExpPrototypeTest => "RegExpPrototypeTest",
      Self::RegExpPrototypeTestFast => "RegExpPrototypeTestFast",
      Self::RegExpPrototypeToString => "RegExpPrototypeToString",
      Self::RegExpReplace => "RegExpReplace",
      Self::RegExpRightContextGetter => "RegExpRightContextGetter",
      Self::RegExpSearchFast => "RegExpSearchFast",
      Self::RegExpSplit => "RegExpSplit",
      Self::RegExpStringIteratorPrototypeNext => "RegExpStringIteratorPrototypeNext",
    }
  }

  /// 关联至全局注册表 `BUILTIN_TABLE` 获取全局内建 ID
  #[inline]
  pub fn global_id(&self) -> Option<usize> {
    find_builtin(self.name())
  }

  /// 获取对应的接口描述符符号
  #[inline]
  pub fn descriptor_name(&self) -> String {
    descriptor_name(self.name())
  }

  /// 是否为 RegExp 全局静态属性访问器
  #[inline]
  pub const fn is_static_getter_setter(&self) -> bool {
    matches!(
      self,
      Self::RegExpCapture1Getter
        | Self::RegExpCapture2Getter
        | Self::RegExpCapture3Getter
        | Self::RegExpCapture4Getter
        | Self::RegExpCapture5Getter
        | Self::RegExpCapture6Getter
        | Self::RegExpCapture7Getter
        | Self::RegExpCapture8Getter
        | Self::RegExpCapture9Getter
        | Self::RegExpInputGetter
        | Self::RegExpInputSetter
        | Self::RegExpLastMatchGetter
        | Self::RegExpLastParenGetter
        | Self::RegExpLeftContextGetter
        | Self::RegExpRightContextGetter
    )
  }

  /// 是否为原型链公开方法
  #[inline]
  pub const fn is_prototype_method(&self) -> bool {
    matches!(
      self,
      Self::RegExpPrototypeCompile
        | Self::RegExpPrototypeExec
        | Self::RegExpPrototypeMatch
        | Self::RegExpPrototypeMatchAll
        | Self::RegExpPrototypeReplace
        | Self::RegExpPrototypeSearch
        | Self::RegExpPrototypeSourceGetter
        | Self::RegExpPrototypeSplit
        | Self::RegExpPrototypeTest
        | Self::RegExpPrototypeToString
    )
  }

  /// 是否为快速执行路径优化内建
  #[inline]
  pub const fn is_fast_path(&self) -> bool {
    matches!(
      self,
      Self::RegExpExecAtom
        | Self::RegExpMatchFast
        | Self::RegExpSearchFast
        | Self::RegExpPrototypeTestFast
        | Self::RegExpReplace
        | Self::RegExpSplit
    )
  }

  /// 是否为正则解释器 / 试验性引擎蹦床入口
  #[inline]
  pub const fn is_trampoline(&self) -> bool {
    matches!(
      self,
      Self::RegExpInterpreterTrampoline | Self::RegExpExperimentalTrampoline
    )
  }

  /// 是否为构造函数或字符串转义入口
  #[inline]
  pub const fn is_constructor_or_escape(&self) -> bool {
    matches!(self, Self::RegExpConstructor | Self::RegExpEscape)
  }
}

// 统一提供 37 项内建工厂函数
pub const fn reg_exp_capture1_getter() -> Builtin {
  Builtin::RegExpCapture1Getter
}

pub const fn reg_exp_capture2_getter() -> Builtin {
  Builtin::RegExpCapture2Getter
}

pub const fn reg_exp_capture3_getter() -> Builtin {
  Builtin::RegExpCapture3Getter
}

pub const fn reg_exp_capture4_getter() -> Builtin {
  Builtin::RegExpCapture4Getter
}

pub const fn reg_exp_capture5_getter() -> Builtin {
  Builtin::RegExpCapture5Getter
}

pub const fn reg_exp_capture6_getter() -> Builtin {
  Builtin::RegExpCapture6Getter
}

pub const fn reg_exp_capture7_getter() -> Builtin {
  Builtin::RegExpCapture7Getter
}

pub const fn reg_exp_capture8_getter() -> Builtin {
  Builtin::RegExpCapture8Getter
}

pub const fn reg_exp_capture9_getter() -> Builtin {
  Builtin::RegExpCapture9Getter
}

pub const fn reg_exp_constructor() -> Builtin {
  Builtin::RegExpConstructor
}

pub const fn reg_exp_escape() -> Builtin {
  Builtin::RegExpEscape
}

pub const fn reg_exp_exec_atom() -> Builtin {
  Builtin::RegExpExecAtom
}

pub const fn reg_exp_experimental_trampoline() -> Builtin {
  Builtin::RegExpExperimentalTrampoline
}

pub const fn reg_exp_input_getter() -> Builtin {
  Builtin::RegExpInputGetter
}

pub const fn reg_exp_input_setter() -> Builtin {
  Builtin::RegExpInputSetter
}

pub const fn reg_exp_interpreter_trampoline() -> Builtin {
  Builtin::RegExpInterpreterTrampoline
}

pub const fn reg_exp_last_match_getter() -> Builtin {
  Builtin::RegExpLastMatchGetter
}

pub const fn reg_exp_last_paren_getter() -> Builtin {
  Builtin::RegExpLastParenGetter
}

pub const fn reg_exp_left_context_getter() -> Builtin {
  Builtin::RegExpLeftContextGetter
}

pub const fn reg_exp_match_fast() -> Builtin {
  Builtin::RegExpMatchFast
}

pub const fn reg_exp_prototype_compile() -> Builtin {
  Builtin::RegExpPrototypeCompile
}

pub const fn reg_exp_prototype_exec() -> Builtin {
  Builtin::RegExpPrototypeExec
}

pub const fn reg_exp_prototype_exec_slow() -> Builtin {
  Builtin::RegExpPrototypeExecSlow
}

pub const fn reg_exp_prototype_match() -> Builtin {
  Builtin::RegExpPrototypeMatch
}

pub const fn reg_exp_prototype_match_all() -> Builtin {
  Builtin::RegExpPrototypeMatchAll
}

pub const fn reg_exp_prototype_replace() -> Builtin {
  Builtin::RegExpPrototypeReplace
}

pub const fn reg_exp_prototype_search() -> Builtin {
  Builtin::RegExpPrototypeSearch
}

pub const fn reg_exp_prototype_source_getter() -> Builtin {
  Builtin::RegExpPrototypeSourceGetter
}

pub const fn reg_exp_prototype_split() -> Builtin {
  Builtin::RegExpPrototypeSplit
}

pub const fn reg_exp_prototype_test() -> Builtin {
  Builtin::RegExpPrototypeTest
}

pub const fn reg_exp_prototype_test_fast() -> Builtin {
  Builtin::RegExpPrototypeTestFast
}

pub const fn reg_exp_prototype_to_string() -> Builtin {
  Builtin::RegExpPrototypeToString
}

pub const fn reg_exp_replace() -> Builtin {
  Builtin::RegExpReplace
}

pub const fn reg_exp_right_context_getter() -> Builtin {
  Builtin::RegExpRightContextGetter
}

pub const fn reg_exp_search_fast() -> Builtin {
  Builtin::RegExpSearchFast
}

pub const fn reg_exp_split() -> Builtin {
  Builtin::RegExpSplit
}

pub const fn reg_exp_string_iterator_prototype_next() -> Builtin {
  Builtin::RegExpStringIteratorPrototypeNext
}

/// 来自 `regexp*.tq` 的声明清单 (共 24 条)
pub const TQ_DECLS: &[(&str, &str)] = &[
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("macro", "IsFastRegExpForMatch"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("macro", "IsFastRegExpForSearch"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("macro", "IsFastRegExpStrict"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("macro", "IsFastRegExpPermissive"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("macro", "LoadRegExpFunction"),
  ("macro", "HasInitialRegExpMap"),
  ("macro", "IsReceiverInitialRegExpPrototype"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("macro", "FlagGetter"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("extern macro", "ClearTrustedPointerField"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("extern macro", "RegExpBuiltinsAssembler"),
  ("macro", "IsRegExpResult"),
];
