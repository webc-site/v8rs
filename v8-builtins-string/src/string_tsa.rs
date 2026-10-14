// 由 tool/src/builtins/builtins-string.js 生成; 来源 v8/src/builtins 下 2 文件
// 内建名表 + 签名骨架 (0 内建)
// 修订就地改, 复跑覆盖; 只含规则代码, 不含语义

use strum::{Display, EnumIter, EnumString};

/// 内建名表 (来源: builtins-string-tsa.cc, builtins-string-tsa-inl.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum Builtin {}
// (无 .tq/.h 附加规则)
