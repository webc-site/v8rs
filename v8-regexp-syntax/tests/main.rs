use aok::{OK, Void};
use log::info;
use v8_regexp_syntax::{RegExpFlags, RegexpError, parse, parse_utf16};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_top_level_parse() -> Void {
  info!("test top level parse");
  let data = parse("abc+", RegExpFlags::GLOBAL)?;
  assert_eq!(data.capture_count, 0);

  let utf16: Vec<u16> = "(?<grp>hello|world)".encode_utf16().collect();
  let data16 = parse_utf16(&utf16, RegExpFlags::UNICODE)?;
  assert_eq!(data16.capture_count, 1);
  assert!(data16.named_captures.is_some());
  OK
}

#[test]
fn test_fuzz_regexp_escape_eof() -> Void {
  let payload: &[u8] = &[46, 221, 91, 92];
  let flags = RegExpFlags((payload[0] as u16) & 0x01FF);
  let utf16_chars: Vec<u16> = payload[1..].iter().map(|&b| b as u16).collect();
  let err = parse_utf16(&utf16_chars, flags).unwrap_err();
  assert_eq!(err.kind(), RegexpError::EscapeAtEndOfPattern);

  // 验证不同模式下的反斜杠末尾截断均精确返回 EscapeAtEndOfPattern
  assert_eq!(
    parse(r"\", RegExpFlags::NONE).unwrap_err().kind(),
    RegexpError::EscapeAtEndOfPattern
  );
  assert_eq!(
    parse(r"\", RegExpFlags::UNICODE).unwrap_err().kind(),
    RegexpError::EscapeAtEndOfPattern
  );
  assert_eq!(
    parse(r"[a-\", RegExpFlags::NONE).unwrap_err().kind(),
    RegexpError::EscapeAtEndOfPattern
  );
  assert_eq!(
    parse(r"[\", RegExpFlags::NONE).unwrap_err().kind(),
    RegexpError::EscapeAtEndOfPattern
  );
  assert_eq!(
    parse(r"[\", RegExpFlags::UNICODE).unwrap_err().kind(),
    RegexpError::EscapeAtEndOfPattern
  );
  assert_eq!(
    parse(r"[\", RegExpFlags::UNICODE_SETS).unwrap_err().kind(),
    RegexpError::EscapeAtEndOfPattern
  );
  assert_eq!(
    parse(r"[a--\", RegExpFlags::UNICODE_SETS)
      .unwrap_err()
      .kind(),
    RegexpError::EscapeAtEndOfPattern
  );
  OK
}
