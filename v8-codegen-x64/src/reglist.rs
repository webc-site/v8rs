// 由 tool/src/codegen/x64/reglist-x64.h.js 从 v8/src/codegen/x64/reglist-x64.h 生成; 修订就地改, 复跑覆盖
// x64 寄存器列表 + caller-saved 位集

// 寄存器位码: rax=0 rcx=1 rdx=2 rbx=3 rsp=4 rbp=5 rsi=6 rdi=7, r8..r15 = 8..15

/// 原版 kJSCallerSaved: rax, rcx, rdx, rbx(JS 侧 caller-saved), rdi(callee 函数)
pub const K_JS_CALLER_SAVED: u64 = 0b10001111;
/// 原版 kNumJSCallerSaved
pub const K_NUM_JS_CALLER_SAVED: usize = 5;

/// 原版 kCallerSaved (V8_TARGET_OS_WIN 分支): rax, rcx, rdx, r8..r11
pub const K_CALLER_SAVED_WIN: u64 = 0b111100000111;

/// 原版 kCallerSaved (非 WIN: Linux/macOS, 本目标生效分支): rax, rcx, rdx, rdi, rsi, r8..r11
pub const K_CALLER_SAVED: u64 = 0b111111000111;

/// 原版 using RegList = RegListBase<Register> (基类见 v8-codegen-core::register_base)
pub type RegList = u64;
/// 原版 using DoubleRegList = RegListBase<DoubleRegister>
pub type DoubleRegList = u64;
/// 原版 using Simd128RegList = RegListBase<Simd128Register>
pub type Simd128RegList = u64;

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_JS_CALLER_SAVED.count_ones() as usize == K_NUM_JS_CALLER_SAVED);
  assert!(K_NUM_JS_CALLER_SAVED == 5);
  assert!(K_CALLER_SAVED_WIN.count_ones() == 7);
  assert!(K_CALLER_SAVED.count_ones() == 9);
  // 抽查位: rax=bit0, rdi=bit7, r11=bit11
  assert!(K_JS_CALLER_SAVED & 1 != 0);
  assert!(K_JS_CALLER_SAVED & (1 << 7) != 0);
  assert!(K_CALLER_SAVED & (1 << 11) != 0);
  assert!(K_CALLER_SAVED & (1 << 6) != 0); // rsi 在非 WIN 分支
  assert!(K_CALLER_SAVED_WIN & (1 << 6) == 0); // WIN 分支无 rsi
};
