use aok::{OK, Void};
use rs8::runtime::Interpreter;

#[test]
fn probe_completion_value() -> Void {
  for (label, src) in [
    ("eval-else", r#"eval('2; if (false) { } else { 3; }')"#),
    ("direct-if", r#"var z = 2; if (false) { } else { 3; }"#),
    ("eval-block", r#"eval('2; { 7; 8; }')"#),
    ("eval-loop", r#"eval('2; for (var i = 0; i < 3; i++) { i; }')"#),
    ("eval-assign", r#"eval('2; x = 5; x')"#),
  ] {
    let mut interp = Interpreter::new();
    println!("PROBE {label} ast={:?}", interp.run(src));
    let mut interp2 = Interpreter::new();
    interp2.mode = rs8::runtime::ExecMode::AstOnly;
    println!("PROBE {label} astonly={:?}", interp2.run(src));
  }
  OK
}
