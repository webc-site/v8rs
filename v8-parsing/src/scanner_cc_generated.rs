// 由 tool/src/parsing/scanner.cc.js 从 v8/src/parsing/scanner.cc + scanner-character-streams.{h,cc} 生成; 修订就地改, 复跑覆盖
// 扫描器与字符流骨架

// Scanner 定义头清单 (对数基准, 46 项):
// Scanner::Scanner(Utf16CharacterStream* source, UnoptimizedCompileFlags flags) : flags_(flags), source_(source), found_html_comment_(false), octal_pos_(Location::invalid()), octal_message_(MessageTemplate::kNone)
// void Scanner::Initialize()
// bool Scanner::IsInvalid(base::uc32 c)
// base::uc32 Scanner::ScanHexNumber(int expected_length)
// base::uc32 Scanner::ScanUnlimitedLengthHexNumber(base::uc32 max_value, int beg_pos)
// Token::Value Scanner::Next()
// Token::Value Scanner::PeekAhead()
// Token::Value Scanner::PeekAheadAhead()
// Token::Value Scanner::SkipSingleHTMLComment()
// Token::Value Scanner::SkipSingleLineComment()
// Token::Value Scanner::SkipMagicComment(base::uc32 hash_or_at_sign)
// void Scanner::TryToParseMagicComment(base::uc32 hash_or_at_sign)
// bool Scanner::HasPerFunctionCompileHint(int position)
// Token::Value Scanner::SkipMultiLineComment()
// Token::Value Scanner::ScanHtmlComment()
// void Scanner::SanityCheckTokenDesc(const TokenDesc& token) const
// void Scanner::SeekForward(int pos)
// bool Scanner::ScanEscape()
// base::uc32 Scanner::ScanOctalEscape(base::uc32 c, int length)
// Token::Value Scanner::ScanString()
// Token::Value Scanner::ScanPrivateName()
// Token::Value Scanner::ScanTemplateSpan()
// DirectHandle<String> Scanner::SourceUrl(IsolateT* isolate) const
// DirectHandle<String> Scanner::SourceMappingUrl(IsolateT* isolate) const
// DirectHandle<String> Scanner::DebugId(IsolateT* isolate) const
// bool Scanner::ScanDigitsWithNumericSeparators(bool (*predicate)(base::uc32 ch), bool is_check_first_digit)
// bool Scanner::ScanDecimalDigits(bool allow_numeric_separator)
// bool Scanner::ScanDecimalAsSmiWithNumericSeparators(uint64_t* value)
// bool Scanner::ScanDecimalAsSmi(uint64_t* value, bool allow_numeric_separator)
// bool Scanner::ScanBinaryDigits()
// bool Scanner::ScanOctalDigits()
// bool Scanner::ScanImplicitOctalDigits(int start_pos, Scanner::NumberKind* kind)
// bool Scanner::ScanHexDigits()
// bool Scanner::ScanSignedInteger()
// Token::Value Scanner::ScanNumber(bool seen_period)
// base::uc32 Scanner::ScanIdentifierUnicodeEscape()
// base::uc32 Scanner::ScanUnicodeEscape()
// Token::Value Scanner::ScanIdentifierOrKeywordInnerSlow(bool escaped, bool can_be_keyword)
// bool Scanner::ScanRegExpPattern()
// std::optional<regexp::Flags> Scanner::ScanRegExpFlags()
// const AstRawString* Scanner::CurrentSymbol( AstValueFactory* ast_value_factory) const
// const AstRawString* Scanner::NextSymbol( AstValueFactory* ast_value_factory) const
// const AstRawString* Scanner::CurrentRawSymbol( AstValueFactory* ast_value_factory) const
// double Scanner::DoubleValue()
// const char* Scanner::CurrentLiteralAsCString(Zone* zone) const
// void Scanner::SeekNext(size_t position)

/// 字符流门面 (原版 scanner-character-streams.h ScannerStream:
/// 底座 Utf16CharacterStream, 分派缓冲/无缓冲实现)
pub const CHARACTER_STREAM_CLASSES: [&str; 1] = ["ScannerStream"];

pub mod scanner_cc_skel {
  use crate::{
    character_stream::{CharacterStream, OneByteCharacterStream, ScannerStream},
    scanner::Scanner,
    token_generated::Token,
  };

  /// 扫描热路径锚点
  pub const HOT_PATHS: [&str; 3] = ["ErrorState", "Location", "Scan"];

  /// 单 token 扫描骨架: 使用内部空流执行最小合法单 token 扫描 (原版 Scanner::ScanSingleToken)
  pub fn scan_single_token() {
    let stream = OneByteCharacterStream::new(b"");
    let mut scanner = Scanner::new(stream);
    scanner.initialize();
    let _ = scanner.next_token();
  }

  /// 从指定源码扫描首个 token (支持完整字符集)
  pub fn scan_token(source: &str) -> Token {
    let stream = ScannerStream::for_testing(source);
    let mut scanner = Scanner::new(stream);
    scanner.initialize();
    scanner.next_token()
  }

  /// 从字符流中扫描下一个 token
  pub fn scan_from_stream<S: CharacterStream>(stream: S) -> Token {
    let mut scanner = Scanner::new(stream);
    scanner.initialize();
    scanner.next_token()
  }
}
