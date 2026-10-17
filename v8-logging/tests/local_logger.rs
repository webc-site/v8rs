//! `src/local_logger.rs` 测试。
//!
//! 规约来源: 现存树 `v8/test/unittests/logging/` **无 local-logger 专项用例**
//! (仅 counters/log/runtime-call-stats 三件; 任务简报所指 local-logger-test.cc
//! 不存在)。故本文件断言 1:1 取自 `local-logger.h/.cc` 语义本体:
//! 构造快照谓词(h:19-22 / cc:14-18)与 `ScriptEvent` 转发(cc:23-25)。

use std::{env::temp_dir, fs, process::id};

use aok::{OK, Void};
use v8_logging::{LocalLogger, LogFile, LogVersionInfo, ScriptEventType, set_log_function_events};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 对应原版 fixture 的 `v8_flags.log = true` + 临时文件(log-unittest.cc:60-67)。
fn temp_log_file(tag: &str) -> (String, LogFile) {
  v8_logging::set_log(true);
  let path = temp_dir()
    .join(format!("v8rs_ll_{}_{}_v8.log", tag, id()))
    .to_string_lossy()
    .into_owned();
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
  let log_file = LogFile::create(path.clone(), &version).expect("create temp log file");
  (path, log_file)
}

fn read_lines(path: &str, mut log_file: LogFile) -> Vec<String> {
  drop(log_file.close());
  let raw = fs::read_to_string(path).expect("log file readable");
  let _ = fs::remove_file(path);
  raw.split('\n').map(str::to_owned).collect()
}

/// local-logger.cc:14-18: 构造时快照 `V8FileLogger` 的两谓词, 之后只读。
#[test]
fn local_logger_snapshots_predicates() -> Void {
  let logger = LocalLogger::new(true, false);
  assert!(logger.is_logging());
  assert!(!logger.is_listening_to_code_events());
  let logger = LocalLogger::new(false, true);
  assert!(!logger.is_logging());
  assert!(logger.is_listening_to_code_events());
  OK
}

/// local-logger.cc:23-25: `LocalLogger::ScriptEvent` 纯转发 `V8FileLogger::ScriptEvent`
/// (log.cc:1914)。断言行格式与 log-unittest.cc 的 `{"script,create"}` 规约一致。
#[test]
fn local_logger_script_event_forwards() -> Void {
  let (path, mut log_file) = temp_log_file("script_forward");
  let logger = LocalLogger::new(true, true);
  // 原版门控 `v8_flags.log_function_events`(flags_lite)。
  set_log_function_events(true);
  logger.script_event(&mut log_file, ScriptEventType::Create, 7, 12345);
  let log = read_lines(&path, log_file);
  assert!(
    log
      .iter()
      .any(|line| line.contains("script,create,7,12345"))
  );
  OK
}

/// `is_logging_ == false` 快路径: `NewMessageBuilder` 返回空(log-file.cc), 不产生事件行。
#[test]
fn local_logger_not_logging_skips() -> Void {
  let (path, mut log_file) = temp_log_file("not_logging");
  let logger = LocalLogger::new(false, false);
  // flag 开启, 专测 `is_logging_ == false` 的 NewMessageBuilder 快路径
  // (log-file.cc): builder 为空 → 不产生事件行。
  set_log_function_events(true);
  logger.script_event(&mut log_file, ScriptEventType::Create, 7, 1);
  let log = read_lines(&path, log_file);
  assert!(log.iter().all(|line| !line.contains("script,")));
  OK
}
