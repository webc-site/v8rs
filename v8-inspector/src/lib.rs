// Copyright 2026 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![forbid(unsafe_code)]

//! `v8-inspector` — V8 Chrome DevTools 协议 (CDP) 调试与中枢支撑层
//!
//! 提供协议字符串、会话管理、断点、Console 消息、调用栈与各域代理核心数据结构。

mod agents;
mod console;
mod core;
mod debugger;
mod error;
mod runtime;
mod strings;

pub use core::*;

pub use agents::*;
pub use console::*;
pub use debugger::*;
pub use error::*;
pub use runtime::*;
pub use strings::*;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string16_basic_and_crc32() {
    let s = String16::from_str_slice("hello world");
    assert_eq!(s.length(), 11);
    assert!(!s.is_empty());
    assert_eq!(s.to_string_lossy(), "hello world");

    let sub = s.substring(6, 5);
    assert_eq!(sub.to_string_lossy(), "world");

    // 溢出防护测试 (len = usize::MAX, start = usize::MAX)
    let sub_max = s.substring(6, usize::MAX);
    assert_eq!(sub_max.to_string_lossy(), "world");
    let sub_overflow = s.substring(usize::MAX, 10);
    assert!(sub_overflow.is_empty());

    let pos = s.find_str("world", 0);
    assert_eq!(pos, 6);
    let pos_overflow = s.find_str("world", usize::MAX);
    assert_eq!(pos_overflow, K_STRING16_NOT_FOUND);

    let rev_pos = s.reverse_find("l", 11);
    assert_eq!(rev_pos, 9);

    // CRC32 计算对齐
    let crc = compute_crc32(&s);
    let s2 = String16::from_str_slice("hello world");
    assert_eq!(crc, compute_crc32(&s2));

    let empty = String16::new();
    assert_eq!(compute_crc32(&empty), 0);

    // 对齐 V8 空白符过滤 (包含 \v \f \t \r \n 空格)
    let ws = String16::from_str_slice(" \x0B\x0C\t abc \r\n");
    assert_eq!(ws.strip_white_space().to_string_lossy(), "abc");

    let i_str = String16::from_integer(12345);
    assert_eq!(i_str.to_integer().unwrap_or(0), 12345);

    let mut builder = String16Builder::new();
    builder.append_str("test_");
    builder.append_unsigned_as_hex(0xabcd);
    assert_eq!(builder.to_string16().to_string_lossy(), "test_0000abcd");
  }

  #[test]
  fn test_magic_comments() {
    let code = String16::from_str_slice("function foo() {}\n//# sourceURL=my_script.js\n");
    let url = find_source_url(&code, false);
    assert_eq!(url.to_string_lossy(), "my_script.js");

    let code_map =
      String16::from_str_slice("function bar() {}\n/*# sourceMappingURL=bundle.js.map */\n");
    let map_url = find_source_map_url(&code_map, true);
    assert_eq!(map_url.to_string_lossy(), "bundle.js.map");

    let code_debug = String16::from_str_slice("/*@ debugId=1234-abcd */");
    let debug_id = find_debug_id(&code_debug, true);
    assert_eq!(debug_id.to_string_lossy(), "1234-abcd");
  }

  #[test]
  fn test_core_identifiers_and_session() {
    let dbg_id = V8DebuggerId::new(42, 100);
    assert!(dbg_id.is_valid());
    assert_eq!(dbg_id.to_string_repr(), "42.100");
    let parsed_dbg = V8DebuggerId::parse("42.100").unwrap_or_default();
    assert_eq!(parsed_dbg, dbg_id);

    let id_str = RemoteObjectId::serialize(999, 1, 12);
    assert_eq!(id_str, "999.1.12");
    let roid = RemoteObjectId::parse(&id_str).unwrap();
    assert_eq!(roid.isolate_id(), 999);
    assert_eq!(roid.context_id(), 1);
    assert_eq!(roid.id(), 12);

    let frame_str = RemoteCallFrameId::serialize(999, 1, 3);
    let frame_id = RemoteCallFrameId::parse(&frame_str).unwrap();
    assert_eq!(frame_id.frame_ordinal(), 3);

    let mut inspector = V8InspectorImpl::new();
    let sid1 = inspector.connect(1);
    assert_eq!(sid1, 1);
    let session = inspector.session_by_id(1, sid1);
    assert!(session.is_some());
    assert_eq!(session.map(|s| s.session_id()), Some(1));
    assert_eq!(session.map(|s| s.is_instrumentation()), Some(false));

    let disconnected = inspector.disconnect(1, sid1);
    assert!(disconnected);
    assert!(inspector.session_by_id(1, sid1).is_none());
  }

  #[test]
  fn test_runtime_and_discard_reasons() {
    assert!(V8InternalValueType::None.is_none());
    assert!(V8InternalValueType::Scope.is_scope());
    assert!(V8InternalValueType::PrivateMethod.is_private_method());

    assert!(DiscardReason::Fulfilled.is_fulfilled());
    assert!(DiscardReason::TearDown.is_tear_down());

    let ctx = InspectedContext::new(1, 10, "http://localhost", "Main Context");
    assert_eq!(ctx.context_id(), 1);
    assert_eq!(ctx.context_group_id(), 10);
    assert_eq!(ctx.origin(), "http://localhost");
    assert_eq!(ctx.human_readable_name(), "Main Context");

    let ro_str = RemoteObject::string("antigravity");
    assert_eq!(ro_str.type_, "string");
    assert_eq!(ro_str.description.as_deref(), Some("antigravity"));

    let ro_num = RemoteObject::number(42.0);
    assert_eq!(ro_num.type_, "number");

    let mirror = ValueMirror::new("object", Some("array".to_string()), "Array(0)");
    assert_eq!(mirror.type_name(), "object");
    assert_eq!(mirror.subtype(), Some("array"));
    assert_eq!(mirror.description(), "Array(0)");
  }

  #[test]
  fn test_debugger_and_breakpoints() {
    assert!(WrapMode::Deep.is_deep());
    assert!(WrapMode::Preview.is_preview());

    let bp_url = BreakpointType::from_u8(1).unwrap();
    assert_eq!(bp_url, BreakpointType::ByUrl);
    assert!(bp_url.is_by_url());
    assert!(!bp_url.is_command());

    let mut dbg = V8Debugger::new();
    assert!(dbg.breakpoints_active());
    let bpid = dbg.set_breakpoint(bp_url, Some("x > 0".to_string()));
    let bp_ref = dbg.get_breakpoint(bpid);
    assert!(bp_ref.is_some());
    assert_eq!(bp_ref.map(|b| b.id()), Some(bpid));
    assert_eq!(bp_ref.and_then(|b| b.condition()), Some("x > 0"));

    let mut tracker = V8SerializationDuplicateTracker::new();
    assert!(!tracker.find_known_object(0x1000));
    assert!(tracker.remember_object(0x1000));
    assert!(tracker.find_known_object(0x1000));
  }

  #[test]
  fn test_console_and_agents() {
    assert!(V8MessageOrigin::Console.is_console());
    assert!(V8MessageOrigin::Exception.is_exception());

    assert!(ConsoleAPIType::Error.is_error_or_warning());
    assert!(ConsoleAPIType::StartGroup.is_group());
    assert!(ConsoleAPIType::TimeEnd.is_time());

    let mut storage = V8ConsoleMessageStorage::new();
    let msg =
      V8ConsoleMessage::create_for_console_api(1, 1000.0, ConsoleAPIType::Log, "test log", None);
    storage.add_message(msg);
    assert_eq!(storage.message_count(), 1);

    assert_eq!(storage.count(1, "my_counter"), 1);
    assert_eq!(storage.count(1, "my_counter"), 2);
    assert!(storage.count_reset(1, "my_counter"));

    assert!(storage.time(1, "my_timer", 100.0));
    assert_eq!(storage.time_end(1, "my_timer", 150.0), Some(50.0));

    let custom_preview = CustomPreview::new("header", false, None, 5);
    assert!(custom_preview.is_ok());

    let err_preview = CustomPreview::new("header", false, None, 25);
    assert!(err_preview.is_err());

    let loc = V8ExceptionLocation::new("http://example.com/err.js", 10, 5);
    let exc_msg =
      V8ConsoleMessage::create_for_exception(2000.0, loc, "Uncaught error", 1, 99, None);
    assert_eq!(exc_msg.origin, V8MessageOrigin::Exception);
    assert_eq!(exc_msg.line_number, 10);
    assert_eq!(exc_msg.column_number, 5);

    let span = ScriptPositionSpan::new(1, 0, 100, 20);
    let script = V8DebuggerScript::new(
      42,
      "http://example.com/main.js",
      Some("http://example.com/main.js.map".to_string()),
      "hash1234",
      span,
      DebuggerScriptLanguage::JavaScript,
    );
    assert_eq!(script.script_id(), 42);
    assert_eq!(script.url(), "http://example.com/main.js");
    assert_eq!(
      script.source_mapping_url(),
      Some("http://example.com/main.js.map")
    );
    assert_eq!(script.hash(), "hash1234");
    assert_eq!(script.span(), span);
    assert_eq!(script.start_line(), 1);
    assert_eq!(script.end_line(), 100);

    let mut profiler = V8ProfilerAgentImpl::new();
    assert!(!profiler.is_enabled());
    profiler.enable();
    assert!(profiler.is_enabled());
    assert_eq!(profiler.sampling_interval_us(), 1000);

    let schema = V8SchemaAgentImpl::new();
    assert!(schema.get_domains().contains(&"Debugger".to_string()));
  }
}
