//! `src/log.rs` 门面测试。1:1 规约来源: `v8/test/unittests/logging/log-unittest.cc`
//! (TestWithIsolate 引擎级断言推迟, 本轮落地其**日志行格式**可移植断言)。

use std::{env::temp_dir, fs, path::PathBuf, process::id};

use aok::{OK, Void};
use log::info;
use v8_logging::{
  CodeEvent, CompilationCacheEventData, FunctionEventData, LogFile, LogVersionInfo,
  ScriptEventType, code_name_event, compilation_cache_event, format_g, function_event,
  move_event_internal, script_event, script_event_name, set_log_code, set_log_function_events,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 `SetUpTestSuite`: `v8_flags.log = true; logfile = kLogToTemporaryFile`。
/// 此处以 flags 全局置位 + 显式临时路径等价(log-unittest.cc:60-67)。
fn temp_log_file(tag: &str) -> (PathBuf, LogFile) {
  v8_logging::set_log(true);
  let path = temp_dir().join(format!("v8rs_log_{}_{}_v8.log", tag, id()));
  let _ = fs::remove_file(&path);
  let version = LogVersionInfo {
    major: 13,
    minor: 2,
    build: 4,
    patch: 5,
    embedder: "",
    is_candidate: false,
    os_string: "mac",
    target_os_string: "macos",
  };
  let log_file =
    LogFile::create(path.to_string_lossy().into_owned(), &version).expect("create temp log file");
  (path, log_file)
}

fn finish(path: &PathBuf, mut log_file: LogFile) -> Vec<String> {
  // 对应 StopLogging(): TearDownAndGetLogFile + 读回 + Split(raw_log_, '\n')。
  drop(log_file.close());
  let raw = fs::read_to_string(path).expect("log file readable");
  let _ = fs::remove_file(path);
  raw.split('\n').map(str::to_owned).collect()
}

/// 对应原版 `IndexOfLine`(log-unittest.cc:132-143): 行含全部子串则命中。
fn index_of_line(log: &[String], terms: &[&str], start: usize) -> Option<usize> {
  for (i, line) in log.iter().enumerate().skip(start) {
    if terms.iter().all(|t| line.contains(t)) {
      return Some(i);
    }
  }
  None
}

/// 对应原版 `ContainsLine`(log-unittest.cc:145-148)。
fn contains_line(log: &[String], terms: &[&str]) -> bool {
  index_of_line(log, terms, 0).is_some()
}

/// 对应原版 `ContainsLinesInOrder`(log-unittest.cc:152-172): 各断言组按序出现。
fn contains_lines_in_order(log: &[String], groups: &[&[&str]]) -> bool {
  let mut start = 0usize;
  for terms in groups {
    match index_of_line(log, terms, start) {
      Some(i) => start = i + 1,
      None => return false,
    }
  }
  true
}

// TEST_F(LogTest, LogVersion)(log-unittest.cc:439-451):
// 断言行含 "v8-version," 与 "13,2,4,5,0"(major,minor,build,patch,is_candidate)。
#[test]
fn log_test_log_version() -> Void {
  let (path, log_file) = temp_log_file("version");
  let log = finish(&path, log_file);
  assert!(contains_line(&log, &["v8-version,", "13,2,4,5,0"]));
  // v8-platform 行(log-file.cc WriteLogHeader 第二行)。
  assert!(contains_line(&log, &["v8-platform,mac,macos"]));
  info!("log version lines ok");
  OK
}

// TEST_F(LogFunctionEventsTest, LogFunctionEvents)(log-unittest.cc:1138-1205)
// 的可移植行格式子集: script create → preparse → full-parse → interpreter 依序。
// reason 字面量取自现源(parser.cc:2992-2993 "preparse-*"/"full-parse"; 检索词
// "function,full-parse," 尾逗号即列分隔符, log-unittest.cc:1176)。
#[test]
fn log_function_events_test_log_function_events() -> Void {
  let (path, mut log_file) = temp_log_file("function_events");
  // 原版 fixture: `v8_flags.log_function_events = true`(log-unittest.cc:1138)。
  set_log_function_events(true);
  // {"script,create"}
  script_event(&mut log_file, true, ScriptEventType::Create, 1, 1000);
  // {"function,preparse-", ",lazyNotExecutedFunction"}(前缀命中 preparse-resolution)
  function_event(
    &mut log_file,
    true,
    FunctionEventData {
      reason: "preparse-resolution",
      script_id: 1,
      time_delta: 0.25,
      start_position: 9,
      end_position: 40,
      time: 1001,
      function_name: b"lazyNotExecutedFunction",
    },
  );
  // {"function,full-parse,", ",eagerFunction"}
  function_event(
    &mut log_file,
    true,
    FunctionEventData {
      reason: "full-parse",
      script_id: 1,
      time_delta: 0.5,
      start_position: 42,
      end_position: 80,
      time: 1002,
      function_name: b"eagerFunction",
    },
  );
  // {"function,interpreter,"}(无名重载: function_name_length == 0)
  function_event(
    &mut log_file,
    true,
    FunctionEventData {
      reason: "interpreter",
      script_id: 1,
      time_delta: 0.75,
      start_position: 0,
      end_position: 0,
      time: 1003,
      function_name: b"",
    },
  );
  let log = finish(&path, log_file);
  assert!(contains_lines_in_order(
    &log,
    &[
      &["script,create"][..],
      &["function,preparse-", ",lazyNotExecutedFunction"][..],
      &["function,full-parse,", ",eagerFunction"][..],
      &["function,interpreter,"][..],
    ]
  ));
  // 逐列格式 1:1: `function,preparse-resolution,1,9,40,0.25,1001,lazyNotExecutedFunction`
  assert!(contains_line(
    &log,
    &["function,preparse-resolution,1,9,40,0.25,1001,lazyNotExecutedFunction"]
  ));
  assert!(contains_line(&log, &["script,create,1,1000"]));
  OK
}

// ScriptEventType 6 变体字符串(log.cc:1919-1940 switch)机械对表。
#[test]
fn log_test_script_event_names() -> Void {
  assert_eq!(script_event_name(ScriptEventType::ReserveId), "reserve-id");
  assert_eq!(script_event_name(ScriptEventType::Create), "create");
  assert_eq!(
    script_event_name(ScriptEventType::Deserialize),
    "deserialize"
  );
  assert_eq!(
    script_event_name(ScriptEventType::BackgroundCompile),
    "background-compile"
  );
  assert_eq!(
    script_event_name(ScriptEventType::StreamingCompileBackground),
    "streaming-compile"
  );
  assert_eq!(
    script_event_name(ScriptEventType::StreamingCompileForeground),
    "streaming-compile-foreground"
  );
  OK
}

// 门控负例: log_function_events=false 时 ScriptEvent 不写行
// (log.cc:1915 `if (!v8_flags.log_function_events) return;`)。
#[test]
fn log_test_script_event_gated_off() -> Void {
  let (path, mut log_file) = temp_log_file("script_gate");
  // log_function_events 默认 false(nextest 进程级隔离)。
  script_event(&mut log_file, true, ScriptEventType::Create, 1, 1000);
  let log = finish(&path, log_file);
  assert!(!contains_line(&log, &["script,"]));
  OK
}

// CodeNameEvent 行格式(log.cc:1835-1842): `<snapshot-code-name>,<pos>,<code_name>`;
// 空名(对应 `code_name == nullptr`)与未监听均不写。
#[test]
fn log_test_code_name_event() -> Void {
  let (path, mut log_file) = temp_log_file("code_name");
  code_name_event(&mut log_file, true, true, 0x1000, 42, "snapshot-stub");
  code_name_event(&mut log_file, true, true, 0x1000, 1, "");
  code_name_event(&mut log_file, true, false, 0x1000, 2, "not-listening");
  let log = finish(&path, log_file);
  assert!(contains_line(
    &log,
    &["snapshot-code-name,42,snapshot-stub"]
  ));
  assert!(!contains_line(&log, &["snapshot-code-name,1,"]));
  assert!(!contains_line(&log, &["snapshot-code-name,2,"]));
  OK
}

// MoveEventInternal 行格式(log.cc:1844-1851): `<event>,0x<from>,0x<to>`;
// `v8_flags.log_code` 关断时不写。
#[test]
fn log_test_move_event_internal() -> Void {
  let (path, mut log_file) = temp_log_file("move_event");
  set_log_code(true);
  move_event_internal(&mut log_file, true, CodeEvent::CodeMove, 0x10, 0x20);
  move_event_internal(&mut log_file, true, CodeEvent::SharedFuncMove, 0xFF, 0x1FF);
  // `v8_flags.log_code` 关断时不写。
  set_log_code(false);
  move_event_internal(&mut log_file, true, CodeEvent::CodeMove, 0x30, 0x40);
  let log = finish(&path, log_file);
  assert!(contains_line(&log, &["code-move,0x10,0x20"]));
  assert!(contains_line(&log, &["sfi-move,0xff,0x1ff"]));
  assert!(!contains_line(&log, &["code-move,0x30,0x40"]));
  OK
}

// CompilationCacheEvent 行格式(log.cc:1897-1912):
// `compilation-cache,<action>,<cache_type>,<script_id>,<start>,<end>,<time>`;
// 门控关断不写。
#[test]
fn log_test_compilation_cache_event() -> Void {
  let (path, mut log_file) = temp_log_file("comp_cache");
  set_log_function_events(true);
  compilation_cache_event(
    &mut log_file,
    true,
    CompilationCacheEventData {
      action: "set",
      cache_type: "regular",
      script_id: -1,
      start_position: 3,
      end_position: 9,
      time: 1000,
    },
  );
  // 门控关断不写。
  set_log_function_events(false);
  compilation_cache_event(
    &mut log_file,
    true,
    CompilationCacheEventData {
      action: "get",
      cache_type: "regular",
      script_id: 1,
      start_position: 0,
      end_position: 5,
      time: 1001,
    },
  );
  let log = finish(&path, log_file);
  assert!(contains_line(
    &log,
    &["compilation-cache,set,regular,-1,3,9,1000"]
  ));
  assert!(!contains_line(&log, &["compilation-cache,get,"]));
  OK
}

// 打开失败降级(log-file.cc:41+59: FOpen 失败 → output_handle_=nullptr →
// os_ 降级 stdout,不视为致命错误):create_output_handle 返回 Ok(None) 而非 Err;
// 且后续消息照常走 stdout 路径(log-file.cc:59 `os_ = stdout`),不丢弃不 panic。
#[test]
fn log_test_open_failure_degrades_to_stdout() -> Void {
  v8_logging::set_log(true);
  let handle = v8_logging::create_output_handle("/nonexistent-dir-v8rs/v8.log")
    .expect("open failure must not be fatal");
  assert!(handle.is_none());
  // 回归(round2):句柄缺失时消息继续写 stdout(os_ 降级), 而非被静默丢弃。
  let mut log_file = LogFile::create(
    "/nonexistent-dir-v8rs/v8.log".to_owned(),
    &LogVersionInfo {
      major: 13,
      minor: 2,
      build: 4,
      patch: 5,
      embedder: "",
      is_candidate: false,
      os_string: "mac",
      target_os_string: "macos",
    },
  )
  .expect("LogFile::create must not fail on open failure");
  v8_logging::string_event(&mut log_file, true, "degraded", "to-stdout");
  drop(log_file.close());
  v8_logging::set_log(false);
  OK
}

// `format_g` = C++ `msg << double`(std::ostream `%g`, 精度 6)黄金对照。
#[test]
fn log_test_format_g_matches_ostream() -> Void {
  // 对照 `printf("%g")`(macOS/Linux libc 行为):
  let cases: &[(f64, &str)] = &[
    (0.0, "0"),
    (1.0, "1"),
    (0.1, "0.1"),
    (-0.1, "-0.1"),
    (1.5, "1.5"),
    (123.456, "123.456"),
    (100000.0, "100000"),
    (999999.0, "999999"),
    (1000000.0, "1e+06"),
    (10000000.0, "1e+07"),
    (123456789.0, "1.23457e+08"),
    (0.0001, "0.0001"),
    (0.00001, "1e-05"),
    (1e100, "1e+100"),
    (1.23456789e-7, "1.23457e-07"),
  ];
  for (v, want) in cases {
    assert_eq!(&format_g(*v), want, "format_g({v})");
  }
  OK
}

// 回归(round3): `AppendTwoByteCharacter` 低字节为 0 时 `%02x` 须补零
// (log-file.cc:170 `"\\u%02x%02x"`, c1=0 → "00"),输出 `\u0041` 而非 `\u041`;
// c2==0 退化为单字节转义;`AppendString(Tagged<String>)` 的 `>0xFF` 走 `\u%04x`。
#[test]
fn log_test_two_byte_character_zero_pad() -> Void {
  let (path, mut log_file) = temp_log_file("two_byte");
  if let Some(mut msg) = log_file.new_message_builder(true) {
    msg.append_two_byte_character(0, 0x41);
    msg.append_two_byte_character(b'a', 0);
    msg.append_string_utf16(&[0x100, 0x2442], None);
    msg.write_to_log_file();
  }
  let log = finish(&path, log_file);
  assert!(contains_line(&log, &[r"\u0041a\u0100\u2442"]));
  OK
}
