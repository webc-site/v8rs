use aok::{OK, Void};
use log::info;
use v8_diagnostics::decode_uleb128;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// crate 冒烟: 真实断言导出面可用 (替代模板恒真占位测试, round3 D1)
#[test]
fn crate_smoke() -> Void {
  assert_eq!(decode_uleb128(&[0xE5, 0x8E, 0x26]), Some((624485, 3)));
  info!("crate smoke ok");
  OK
}
