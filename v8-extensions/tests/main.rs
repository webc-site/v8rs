use aok::{OK, Void};
use v8_extensions::{
  CPUID_LEAF_BASE, CPUTRACEMARK_BUFFER_SIZE, CPUTRACEMARK_EXTENSION_NAME,
  CPUTRACEMARK_SOURCE_FORMAT, CREATE_DOMAIN_FAILED, CREATE_TASK_FAILED,
  DEFAULT_HEAP_SNAPSHOT_FILENAME, DISPATCH, ERR_FIRST_PARAM, ERR_PARAMS,
  EXTERNALIZE_STRING_BUFFER_SIZE, EXTERNALIZE_STRING_EXTENSION_NAME, Error, ExecutionType,
  FUNCTION_MAP, Flavor, GC_BUFFER_SIZE, GC_EXTENSION_NAME, GcType, INVOKE_ACTIONS, JS_CONST_NAMES,
  MIN_LENGTH_FORMULAS, NATIVE_FUNCTIONS, NO_DOMAIN_NAME, NO_TASK_NAME, OPTION_STRINGS,
  STAT_NUMBER_EXPRS, STAT_NUMBER_KEYS, STATISTICS_EXTENSION_NAME, STATISTICS_SOURCE,
  TASK_BEGIN_FAILED, TASK_END_FAILED, TRIGGER_FAILURE_EXTENSION_NAME, TRIGGER_FAILURE_SOURCE,
  UNKNOWN_PARAMS, VTUNEDOMAIN_SUPPORT_EXTENSION_NAME,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_gc_extension() -> Void {
  assert_eq!(GC_EXTENSION_NAME, "v8/gc");
  assert_eq!(GC_BUFFER_SIZE, 50);
  assert_eq!(DEFAULT_HEAP_SNAPSHOT_FILENAME, "heap.heapsnapshot");

  assert_eq!(GcType::Minor as u8, 0);
  assert_eq!(GcType::Major as u8, 1);
  assert_eq!(GcType::MajorWithSnapshot as u8, 2);
  assert!(GcType::Minor.is_minor());
  assert!(!GcType::Minor.is_major());
  assert!(GcType::Major.is_major());
  assert!(!GcType::Major.is_minor());
  assert!(GcType::MajorWithSnapshot.is_major_with_snapshot());

  let parsed: GcType = "MajorWithSnapshot"
    .parse()
    .map_err(|e| Error::Message(format!("{e}")))?;
  assert_eq!(parsed, GcType::MajorWithSnapshot);

  assert_eq!(ExecutionType::Async as u8, 0);
  assert_eq!(ExecutionType::Sync as u8, 1);
  assert!(ExecutionType::Async.is_async());
  assert!(!ExecutionType::Async.is_sync());
  assert!(ExecutionType::Sync.is_sync());

  assert_eq!(Flavor::Regular as u8, 0);
  assert_eq!(Flavor::LastResort as u8, 1);
  assert!(Flavor::Regular.is_regular());
  assert!(!Flavor::Regular.is_last_resort());
  assert!(Flavor::LastResort.is_last_resort());

  assert_eq!(OPTION_STRINGS.len(), 7);
  assert_eq!(INVOKE_ACTIONS.len(), 4);
  OK
}

#[test]
fn test_statistics_extension() -> Void {
  assert_eq!(STATISTICS_EXTENSION_NAME, "v8/statistics");
  assert_eq!(STATISTICS_SOURCE, "native function getV8Statistics();");
  assert_eq!(STAT_NUMBER_KEYS.len(), 22);
  assert_eq!(STAT_NUMBER_EXPRS.len(), 22);
  assert_eq!(STAT_NUMBER_KEYS[0], "total_committed_bytes");
  assert_eq!(STAT_NUMBER_KEYS[21], "trusted_lo_space_commited_bytes");
  OK
}

#[test]
fn test_cputracemark_extension() -> Void {
  assert_eq!(CPUTRACEMARK_EXTENSION_NAME, "v8/cpumark");
  assert_eq!(CPUTRACEMARK_SOURCE_FORMAT, "native function %s();");
  assert_eq!(CPUTRACEMARK_BUFFER_SIZE, 50);
  assert!(ERR_FIRST_PARAM.contains("cputracemark()"));
  assert_eq!(CPUID_LEAF_BASE, 0x4711);
  OK
}

#[test]
fn test_vtunedomain_support_extension() -> Void {
  assert_eq!(VTUNEDOMAIN_SUPPORT_EXTENSION_NAME, "v8/vtunedomain");
  assert_eq!(UNKNOWN_PARAMS, 1);
  assert_eq!(NO_DOMAIN_NAME, 2);
  assert_eq!(CREATE_DOMAIN_FAILED, 4);
  assert_eq!(NO_TASK_NAME, 8);
  assert_eq!(CREATE_TASK_FAILED, 16);
  assert_eq!(TASK_BEGIN_FAILED, 32);
  assert_eq!(TASK_END_FAILED, 64);
  assert_eq!(FUNCTION_MAP.len(), 2);
  assert!(ERR_PARAMS.contains("Parameter number should be exactly three"));
  OK
}

#[test]
fn test_externalize_string_and_trigger_failure() -> Void {
  assert_eq!(EXTERNALIZE_STRING_EXTENSION_NAME, "v8/externalize");
  assert_eq!(EXTERNALIZE_STRING_BUFFER_SIZE, 400);
  assert_eq!(JS_CONST_NAMES.len(), 4);
  assert_eq!(MIN_LENGTH_FORMULAS.len(), 4);
  assert_eq!(NATIVE_FUNCTIONS.len(), 4);

  assert_eq!(TRIGGER_FAILURE_EXTENSION_NAME, "v8/trigger-failure");
  assert!(TRIGGER_FAILURE_SOURCE.contains("triggerCheckFalse"));
  assert_eq!(DISPATCH.len(), 3);
  assert_eq!(DISPATCH[0].0, "triggerCheckFalse");
  assert_eq!(DISPATCH[1].0, "triggerAssertFalse");
  assert_eq!(DISPATCH[2].0, "triggerSlowAssertFalse");
  OK
}

#[test]
fn test_error_display() -> Void {
  let err = Error::InvalidOption {
    key: "type".into(),
    value: "super_gc".into(),
  };
  assert_eq!(
    format!("{err}"),
    "Invalid extension option: key=type, value=super_gc"
  );
  let err = Error::ExecutionFailed("out of memory".into());
  assert_eq!(
    format!("{err}"),
    "Extension execution failed: out of memory"
  );
  let err = Error::UnsupportedOperation("unsupported".into());
  assert_eq!(
    format!("{err}"),
    "Unsupported extension operation: unsupported"
  );
  let err = Error::Message("generic".into());
  assert_eq!(format!("{err}"), "Extension error: generic");
  OK
}
