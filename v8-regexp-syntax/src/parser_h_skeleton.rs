// 由 tool/src/regexp/regexp-parser.h.js 从 v8/src/regexp/regexp-parser.h 生成; 修订就地改, 复跑覆盖
// 正则解析器入口骨架 (签名登记; 语义实现见 regexp-parser.cc 侧生成物)

use crate::{flags_generated::Flags, parser_generated::CompileData};

/// 正则解析器 (对应原版 regexp::Parser, AllStatic — 无实例状态)
pub struct Parser;

impl Parser {
  /// 原版 Parser::ParseRegExpFromHeapString(Isolate*, Zone*, String, Flags, CompileData*)
  /// 解析堆上字符串为 CompileData; 失败返回 false
  pub fn parse_regexp_from_heap_string(
    _input: &str,
    _flags: Flags,
    _result: &mut CompileData,
  ) -> bool {
    unimplemented!("登记骨架: 实现见 v8/src/regexp/regexp-parser.cc")
  }

  /// 原版 Parser::VerifyRegExpSyntax<CharT>(Zone*, stack_limit, input, len, Flags, CompileData*, no_gc)
  /// 仅校验语法, 不产出数据
  pub fn verify_regexp_syntax(
    _stack_limit: usize,
    _input: &str,
    _flags: Flags,
    _result: &mut CompileData,
  ) -> bool {
    unimplemented!("登记骨架: 实现见 v8/src/regexp/regexp-parser.cc")
  }
}
