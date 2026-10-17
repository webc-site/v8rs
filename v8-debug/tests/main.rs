use aok::{OK, Void};
use log::info;
use strum::IntoEnumIterator;
use v8_debug::{
  BreakLocation, BreakReason, Coverage, CoverageBlock, CoverageFunction, CoverageMode,
  CoverageScript, DebugBreakType, DebugScope, DisableBreak, Error, EvaluateGlobalMode,
  ExceptionBreakState, ExceptionBreakType, GdbPacket, GetSourceOffsetMode, K_NO_SOURCE_POSITION,
  K_SCOPE_DETAILS_END_POSITION_INDEX, K_SCOPE_DETAILS_FUNCTION_INDEX, K_SCOPE_DETAILS_NAME_INDEX,
  K_SCOPE_DETAILS_OBJECT_INDEX, K_SCOPE_DETAILS_SIZE, K_SCOPE_DETAILS_START_POSITION_INDEX,
  K_SCOPE_DETAILS_TYPE_INDEX, K_TRANSPORT_BUF_SIZE, NativeAccessorType, PacketError,
  PrivateMemberFilter, ProcessPacketResult, PropertyIteratorPhase, RedirectMode, ScopeIteratorMode,
  ScopeType, StepAction, TargetStatus, VariableInfo, WasmAddr, WasmAddressType,
  WasmValueFieldOffsets,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_debug_core_types() -> Void {
  info!("Testing debug core types");
  // StepAction
  assert_eq!(StepAction::iter().count(), 4);
  assert!(StepAction::Into.is_step_into());
  assert!(StepAction::Over.is_step_over());
  assert!(StepAction::Out.is_step_out());
  assert!(StepAction::None.is_none());

  // ExceptionBreakType
  assert_eq!(ExceptionBreakType::iter().count(), 2);
  assert!(ExceptionBreakType::Caught.is_caught());
  assert!(ExceptionBreakType::Uncaught.is_uncaught());

  // DebugBreakType (7 variants)
  assert_eq!(DebugBreakType::iter().count(), 7);
  assert!(DebugBreakType::DebuggerStatement.is_debugger_statement());
  assert!(DebugBreakType::DebugBreakSlotAtCall.is_break_slot());
  assert!(!DebugBreakType::NotDebugBreak.is_break_slot());

  // BreakReason (9 variants)
  assert_eq!(BreakReason::iter().count(), 9);
  assert!(BreakReason::Step.is_step());
  assert!(BreakReason::AsyncStep.is_step());
  assert!(BreakReason::Exception.is_exception());
  assert!(BreakReason::DebuggerStatement.is_debugger_statement());

  // CoverageMode (4 variants)
  assert_eq!(CoverageMode::iter().count(), 4);

  // ExceptionBreakState (4 states)
  assert_eq!(ExceptionBreakState::iter().count(), 4);
  assert!(ExceptionBreakState::BreakOnCaughtException.is_break_on_caught());
  assert!(ExceptionBreakState::BreakOnUncaughtException.is_break_on_uncaught());
  assert!(ExceptionBreakState::BreakOnAnyException.is_break_on_any());

  // PrivateMemberFilter & NativeAccessorType & EvaluateGlobalMode & GetSourceOffsetMode
  assert_eq!(PrivateMemberFilter::PrivateMethods as u8, 1);
  assert_eq!(NativeAccessorType::HasGetter as u8, 1);
  assert_eq!(EvaluateGlobalMode::iter().count(), 3);
  assert_eq!(GetSourceOffsetMode::iter().count(), 2);

  // BreakLocation
  let loc = BreakLocation::new(128, DebugBreakType::DebuggerStatement);
  assert!(loc.is_valid());
  assert_eq!(loc.position, 128);
  assert!(loc.is_debugger_statement());
  assert!(!loc.is_call());
  assert!(!loc.is_suspend());

  let loc_ret = BreakLocation::new(256, DebugBreakType::DebugBreakSlotAtReturn);
  assert!(loc_ret.is_return());
  assert!(loc_ret.is_return_or_suspend());
  assert!(loc_ret.is_debug_break_slot());

  let loc_sus = BreakLocation::new(512, DebugBreakType::DebugBreakSlotAtSuspend);
  assert!(loc_sus.is_suspend());
  assert!(loc_sus.is_return_or_suspend());
  assert!(loc_sus.is_debug_break_slot());

  let loc_call = BreakLocation::new(1024, DebugBreakType::DebugBreakSlotAtCall);
  assert!(loc_call.is_call());
  assert!(loc_call.is_debug_break_slot());

  let loc_entry = BreakLocation::new(0, DebugBreakType::DebugBreakAtEntry);
  assert!(loc_entry.is_debug_break_at_entry());

  let inv = BreakLocation::invalid();
  assert!(!inv.is_valid());

  // DisableBreak & DebugScope
  let db = DisableBreak::new(true);
  assert!(db.previous_state);

  let mut scope = DebugScope::new();
  assert!(scope.active);
  scope.close();
  assert!(!scope.active);

  OK
}

#[test]
fn test_debug_frames_and_coverage() -> Void {
  info!("Testing frames and coverage models");
  // RedirectMode
  assert!(RedirectMode::UseOriginalBytecode.is_original());
  assert!(RedirectMode::UseDebugBytecode.is_debug());

  // CoverageBlock
  let b = CoverageBlock::new(10, 50, 5);
  assert_eq!(b.start, 10);
  assert_eq!(b.end, 50);
  assert_eq!(b.count, 5);

  let def_b = CoverageBlock::default();
  assert_eq!(def_b.start, K_NO_SOURCE_POSITION);
  assert_eq!(def_b.count, 0);

  // CoverageFunction & CoverageScript & Coverage
  let func = CoverageFunction::new(0, 100, 1, "testFunc", vec![b], true);
  assert_eq!(func.name, "testFunc");
  assert!(func.has_block_coverage);

  let script = CoverageScript::new(42, vec![func]);
  assert_eq!(script.script_id, 42);

  let cov = Coverage::new(vec![script]);
  assert_eq!(cov.len(), 1);
  assert!(!cov.is_empty());

  // WasmValueFieldOffsets
  assert_eq!(WasmValueFieldOffsets::K_TYPE_OFFSET, 0);
  assert_eq!(WasmValueFieldOffsets::K_VALUE_OFFSET, 8);
  assert_eq!(WasmValueFieldOffsets::K_SIZE, 16);

  OK
}

#[test]
fn test_debug_scopes_and_details() -> Void {
  info!("Testing scope types and details constants");
  assert_eq!(ScopeType::iter().count(), 10);
  assert!(ScopeType::Global.is_global());
  assert!(ScopeType::Local.is_local());
  assert!(ScopeType::Closure.is_closure());
  assert!(ScopeType::Module.is_module());
  assert!(ScopeType::WasmExpressionStack.is_wasm());

  // Scope details constants
  assert_eq!(K_SCOPE_DETAILS_TYPE_INDEX, 0);
  assert_eq!(K_SCOPE_DETAILS_OBJECT_INDEX, 1);
  assert_eq!(K_SCOPE_DETAILS_NAME_INDEX, 2);
  assert_eq!(K_SCOPE_DETAILS_START_POSITION_INDEX, 3);
  assert_eq!(K_SCOPE_DETAILS_END_POSITION_INDEX, 4);
  assert_eq!(K_SCOPE_DETAILS_FUNCTION_INDEX, 5);
  assert_eq!(K_SCOPE_DETAILS_SIZE, 6);

  assert_eq!(ScopeIteratorMode::iter().count(), 2);
  assert_eq!(VariableInfo::iter().count(), 3);
  assert_eq!(PropertyIteratorPhase::iter().count(), 3);

  OK
}

#[test]
fn test_wasm_gdb_server_protocol() -> Void {
  info!("Testing wasm gdb server types and packet codec");
  assert_eq!(PacketError::iter().count(), 4);
  assert!(ProcessPacketResult::Paused.is_paused());
  assert!(ProcessPacketResult::Continue.is_continue());

  assert!(TargetStatus::Running.is_running());
  assert!(TargetStatus::Suspended.is_suspended());

  assert_eq!(WasmAddressType::Memory as u8, 0x00);
  assert_eq!(WasmAddressType::Object as u8, 0x01);
  assert_eq!(WasmAddressType::Invalid as u8, 0x03);
  assert_eq!(K_TRANSPORT_BUF_SIZE, 4096);

  // WasmAddr bitfield round-trip
  let addr = WasmAddr::new(0x1234_5678, 0x0abc_def0, WasmAddressType::Object);
  assert_eq!(addr.offset, 0x1234_5678);
  assert_eq!(addr.module_id, 0x0abc_def0 & 0x3fff_ffff);
  assert_eq!(addr.address_type, 1);

  let enc = addr.encode();
  let dec = WasmAddr::decode(enc);
  assert_eq!(dec.offset, addr.offset);
  assert_eq!(dec.module_id, addr.module_id);
  assert_eq!(dec.address_type, addr.address_type);

  // GDB RSP checksum & format packet
  // "OK" -> 'O'(79) + 'K'(75) = 154 = 0x9a
  let csum = GdbPacket::calculate_checksum(b"OK");
  assert_eq!(csum, 0x9a);
  let pkt = GdbPacket::format_packet("OK");
  assert_eq!(pkt, "$OK#9a");

  OK
}

#[test]
fn test_debug_error_variants() -> Void {
  info!("Testing Error variants");
  let e1 = Error::InvalidBreakLocation(-5);
  assert_eq!(e1.to_string(), "Invalid break location: -5");

  let e2 = Error::EvaluationFailed("ReferenceError".to_string());
  assert_eq!(e2.to_string(), "Debug evaluation failed: ReferenceError");

  let e3 = Error::GdbPacketError(PacketError::BadFormat);
  assert_eq!(e3.to_string(), "GDB protocol packet error: BadFormat");

  let e4 = Error::ScopeError("Out of bounds".to_string());
  assert_eq!(e4.to_string(), "Debug scope error: Out of bounds");

  OK
}

#[test]
fn test_debug_objects_and_modules() -> Void {
  info!("Testing debug objects and module references");
  use v8_debug::contexts::{CONTEXT_CELL_FIELDS, ContextCellState, ContextMode};
  assert_eq!(ContextCellState::COUNT, 5);
  assert_eq!(ContextCellState::Const as i32, 0);
  assert_eq!(ContextCellState::Detached as i32, 4);
  assert_eq!(ContextMode::NoContextCells as u32, 0);
  assert_eq!(CONTEXT_CELL_FIELDS.len(), 5);

  use v8_debug::source_text_module::{
    K_ASYNC_EVALUATE_DID_FINISH, K_FIRST_ASYNC_EVALUATION_ORDINAL, K_NOT_ASYNC_EVALUATED,
    ModuleImportPhase,
  };
  assert_eq!(ModuleImportPhase::Source as u32, 0);
  assert_eq!(ModuleImportPhase::Evaluation as u32, 2);
  assert_eq!(K_NOT_ASYNC_EVALUATED, 0);
  assert_eq!(K_ASYNC_EVALUATE_DID_FINISH, 1);
  assert_eq!(K_FIRST_ASYNC_EVALUATION_ORDINAL, 2);

  use v8_debug::module::MODULE_FIELDS;
  assert_eq!(MODULE_FIELDS.len(), 7);

  OK
}
