// 从 v8 复刻过来的转换测试入口（路线图见 example/TODO.md）
mod converted;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}
