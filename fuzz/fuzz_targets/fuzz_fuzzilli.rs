#![allow(unexpected_cfgs)]
#![cfg_attr(fuzzing, no_main)]

#[cfg(not(fuzzing))]
fn main() {}

#[cfg(fuzzing)]
libfuzzer_sys::fuzz_target!(|data: &[u8]| {
  v8rs_fuzz::fuzz_fuzzilli(data);
});

