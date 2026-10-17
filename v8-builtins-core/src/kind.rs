// 由 tool/src/builtins/builtins-core.js 生成; 来源 v8/src/builtins 下 2 文件
// 内建种类枚举 (X-macro 参数位对应)
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

use strum::{Display, EnumIter, EnumString};

/// 内建实现种类 — BUILTIN_LIST_* 参数位 (builtins-definitions.h:75 起)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
pub enum BuiltinKind {
  /// C++ 实现, 经 BUILTIN_EXIT 帧进入
  #[strum(serialize = "CPP")]
  CPP,
  /// Turboshaft JS linkage
  #[strum(serialize = "TFJ_TSA")]
  TFJTSA,
  /// Turbofan JS linkage
  #[strum(serialize = "TFJ")]
  TFJ,
  /// Turboshaft CodeStub linkage + 自定义 descriptor
  #[strum(serialize = "TFC_TSA")]
  TFCTSA,
  /// Turbofan CodeStub linkage + 自定义 descriptor
  #[strum(serialize = "TFC")]
  TFC,
  /// Turbofan CodeStub linkage
  #[strum(serialize = "TFS")]
  TFS,
  /// Turbofan handler
  #[strum(serialize = "TFH")]
  TFH,
  /// 平台相关汇编实现
  #[strum(serialize = "ASM")]
  ASM,
  /// 字节码 handler
  #[strum(serialize = "BCH")]
  BCH,
}

/// BUILTIN_LIST_BASE_TIER0 出现次数 (机械计数)
pub const BUILTIN_LIST_BASE_TIER0_USE_COUNT: usize = 3;
/// BUILTIN_LIST_BASE_TIER1 出现次数 (机械计数)
pub const BUILTIN_LIST_BASE_TIER1_USE_COUNT: usize = 3;
/// BUILTIN_LIST_TEMPORAL 出现次数 (机械计数)
pub const BUILTIN_LIST_TEMPORAL_USE_COUNT: usize = 4;
/// BUILTIN_LIST_INTL 出现次数 (机械计数)
pub const BUILTIN_LIST_INTL_USE_COUNT: usize = 4;

impl BuiltinKind {
  /// 是否为 C++ 实现内建
  #[inline]
  pub const fn is_cpp(self) -> bool {
    matches!(self, Self::CPP)
  }

  /// 是否具备 JS linkage
  #[inline]
  pub const fn has_js_linkage(self) -> bool {
    matches!(self, Self::TFJ | Self::TFJTSA)
  }

  /// 是否为 CodeStub linkage
  #[inline]
  pub const fn is_code_stub(self) -> bool {
    matches!(self, Self::TFC | Self::TFCTSA | Self::TFS)
  }

  /// 是否为平台相关汇编实现
  #[inline]
  pub const fn is_asm(self) -> bool {
    matches!(self, Self::ASM)
  }

  /// 是否为字节码处理器
  #[inline]
  pub const fn is_bytecode_handler(self) -> bool {
    matches!(self, Self::BCH)
  }
}
