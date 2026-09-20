// 复刻 v8/src/base/fpu.cc：原始 JS 测试见 v8/test/mjsunit/hardcoded/flush-denormals-*.js，
// 待引擎可运行后回收（见 TODO.md 阶段4）
use aok::{OK, Void};
use v8rs::Value;
// 同名符号可能被多个 TU 重复产出，走精确模块路径避免 glob 歧义
use v8rs::v8::base_fpu::{kFlushDenormToZeroBit_0, v8_base_FPU};

#[test]
fn flush_denorm_to_zero_bit_constant() -> Void {
  assert_eq!(
    *kFlushDenormToZeroBit_0.with(Value::clone).borrow(),
    1 << 24
  );
  OK
}

#[test]
#[ignore = "cpp2rust 丢弃了内联汇编，GetStatusWord/SetStatusWord 目前是空操作（TODO.md 阶段2）"]
fn flush_denormals_roundtrip() -> Void {
  let before = v8_base_FPU::GetFlushDenormals();
  v8_base_FPU::SetFlushDenormals(!before);
  assert_eq!(v8_base_FPU::GetFlushDenormals(), !before);
  v8_base_FPU::SetFlushDenormals(before);
  OK
}
