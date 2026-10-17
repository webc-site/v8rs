use aok::{OK, Void};
use log::info;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test() -> Void {
  info!("> test {}", 123456);
  OK
}

#[test]
fn test_scanner_cc_skel() -> Void {
  use v8_parsing::{Token, scanner_cc_skel};

  scanner_cc_skel::scan_single_token();

  let tok_let = scanner_cc_skel::scan_token("let x = 1;");
  assert_eq!(tok_let, Token::Let);

  let tok_fn = scanner_cc_skel::scan_token("function foo() {}");
  assert_eq!(tok_fn, Token::Function);

  let tok_num = scanner_cc_skel::scan_token("12345");
  assert_eq!(tok_num, Token::Smi);

  OK
}

#[test]
fn test_parsing_misc_skel() -> Void {
  use v8_parsing::{Location, Token, parsing_misc_skel};

  // 1. parse_program & parse_source
  parsing_misc_skel::parse_program();
  let tokens = parsing_misc_skel::parse_source("const a = 42;");
  assert!(tokens.contains(&Token::Const));
  assert!(tokens.contains(&Token::Identifier));
  assert!(tokens.contains(&Token::Assign));
  assert!(tokens.contains(&Token::Smi));
  assert!(tokens.contains(&Token::Semicolon));

  // 2. parse_info_skel
  let info = parsing_misc_skel::parse_info_skel();
  assert!(info.is_toplevel);
  assert!(info.is_eager);
  assert!(!info.is_module);

  // 3. literal_buffer_skel
  let buf = parsing_misc_skel::literal_buffer_skel();
  assert!(buf.is_empty());

  // 4. func_name_inferrer_skel
  let mut fni = parsing_misc_skel::func_name_inferrer_skel();
  assert_eq!(fni.infer(), None);
  fni.enter();
  assert!(fni.is_open());
  fni.push_enclosing_name("MyClass".to_string());
  fni.push_literal_name("myMethod".to_string());
  assert_eq!(fni.infer(), Some("MyClass.myMethod".to_string()));
  fni.leave();
  assert!(!fni.is_open());

  // 5. import_attributes_skel
  let mut attrs = parsing_misc_skel::import_attributes_skel();
  assert!(!attrs.has("type"));
  attrs.insert(
    "type".to_string(),
    "json".to_string(),
    Location {
      beg_pos: 0,
      end_pos: 4,
    },
  );
  assert!(attrs.has("type"));
  assert_eq!(attrs.get("type").unwrap().1, "json");

  // 6. pending_error_handler_skel
  let mut handler = parsing_misc_skel::pending_error_handler_skel();
  assert!(!handler.has_pending_error);
  handler.report_warning_at(0, 5, "deprecated feature".to_string());
  assert_eq!(handler.warnings.len(), 1);
  handler.report_error_at(10, 15, "syntax error".to_string());
  assert!(handler.has_pending_error);
  assert_eq!(handler.error_message.as_ref().unwrap().2, "syntax error");
  handler.set_stack_overflow();
  assert!(handler.stack_overflow);

  // 7. rewriter_skel
  let mut rewriter = parsing_misc_skel::rewriter_skel();
  assert!(!rewriter.result_assigned);
  assert!(rewriter.rewrite());
  assert!(rewriter.result_assigned);

  OK
}
