use std::{fs, path::PathBuf};

use v8rs_fuzz::{
  MAX_FUZZILLI_INPUT_SIZE, fuzz_fuzzilli, fuzz_json, fuzz_parser, fuzz_regexp,
};

fn get_v8_fuzzer_dir(sub: &str) -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("../v8/test/fuzzer")
    .join(sub)
}

/// 通用语料目录驱动函数 (单次迭代、零冗余、无 panic)
fn run_corpus_dir(sub: &str, mut handler: impl FnMut(&[u8])) {
  let dir = get_v8_fuzzer_dir(sub);
  let Ok(entries) = fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_file()
      && let Ok(content) = fs::read(&path)
    {
      handler(&content);
    }
  }
}

#[test]
fn test_corpus_json() {
  run_corpus_dir("json", fuzz_json);
}

#[test]
fn test_corpus_regexp() {
  let flag_patterns: [u8; 6] = [
    0,  // None
    1,  // Global
    2,  // IgnoreCase
    32, // Unicode
    64, // Sticky
    46, // 典型组合 (Global | IgnoreCase | Multiline | Unicode)
  ];
  let mut payload = Vec::with_capacity(1024);
  run_corpus_dir("regexp", |content| {
    for &flag in &flag_patterns {
      payload.clear();
      payload.push(flag);
      payload.extend_from_slice(content);
      fuzz_regexp(&payload);
    }
  });
}

#[test]
fn test_corpus_parser() {
  run_corpus_dir("parser", fuzz_parser);
}

#[test]
fn test_corpus_fuzzilli() {
  let test_cases = [
    "",
    "FUZZILLI_CRASH",
    "FUZZILLI_CRASH 0",
    "FUZZILLI_CRASH 1",
    "FUZZILLI_CRASH 11",
    "FUZZILLI_CRASH 999",
    "FUZZILLI_CRASH -1",
    "FUZZILLI_PRINT hello_v8rs",
    "FUZZILLI_PRINT",
    "UNKNOWN_OP arg1 arg2",
    "   ",
  ];
  for case in test_cases {
    fuzz_fuzzilli(case.as_bytes());
  }

  let long_payload = vec![b'a'; MAX_FUZZILLI_INPUT_SIZE + 1];
  fuzz_fuzzilli(&long_payload);
}

#[test]
fn test_regression_crashes() {
  // Bug 1: json hex4 字符边界切片截断
  fuzz_json(b"\"\\u\x80000\"");
  fuzz_json(b"\"\\u\xff\"");
  // Bug 2: regexp 末尾反斜杠转义哨兵溢出
  fuzz_regexp(&[46, 221, 91, 92]);
  fuzz_regexp(b"\0\\");
  fuzz_regexp(b"\0[\\");
  fuzz_regexp(b"\0[a-\\");
  fuzz_regexp(&[32, b'\\']);
  fuzz_regexp(&[32, b'[', b'\\']);
}
