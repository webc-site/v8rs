use aok::{OK, Void};
use log::info;
use v8_compiler_frontend::{
  CTypeInfo, CTypeInfoType, CallFrequency, FeedbackSource, ForInMode, GraphAssembler,
  GraphAssemblerLabel, GraphAssemblerLabelType, JSGraph, OverloadsResolutionResult, Reduction,
  c_type_info_flags,
};
use v8_compiler_graph::MachineRepresentation;
use v8_compiler_ops::BranchSemantics;
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_fast_api_calls() -> Void {
  info!("Testing fast api calls data structures");

  let type_info = CTypeInfo {
    ty: CTypeInfoType::Int32,
    flags: c_type_info_flags::ENFORCE_RANGE,
  };
  assert_eq!(type_info.get_type(), CTypeInfoType::Int32);
  assert_eq!(type_info.get_flags(), c_type_info_flags::ENFORCE_RANGE);

  let invalid_res = OverloadsResolutionResult::invalid();
  assert!(!invalid_res.is_valid());

  let valid_res = OverloadsResolutionResult::new(1, CTypeInfoType::Uint32);
  assert!(valid_res.is_valid());
  assert_eq!(valid_res.distinguishable_arg_index, 1);
  assert_eq!(valid_res.element_type, CTypeInfoType::Uint32);

  OK
}

#[test]
fn test_feedback_source() -> Void {
  info!("Testing feedback source");

  let invalid_src = FeedbackSource::invalid();
  assert!(!invalid_src.is_valid());

  let dummy_ptr = 0x1000 as *const ();
  let valid_src = FeedbackSource::new(dummy_ptr, 5);
  assert!(valid_src.is_valid());
  assert_eq!(valid_src.index(), 5);
  assert_ne!(valid_src.hash_value(), 0);

  let same_src = FeedbackSource::new(dummy_ptr, 5);
  assert_eq!(valid_src, same_src);

  OK
}

#[test]
fn test_js_operator_types() -> Void {
  info!("Testing js operator types");

  let freq_unknown = CallFrequency::default();
  assert!(freq_unknown.is_unknown());
  assert!(!freq_unknown.is_known());

  let freq_known = CallFrequency::new(0.75);
  assert!(freq_known.is_known());
  assert!((freq_known.value() - 0.75).abs() < f32::EPSILON);

  assert_eq!(ForInMode::Generic as u8, 2);

  OK
}

#[test]
fn test_reducer() -> Void {
  info!("Testing reducer Reduction");

  let no_change = Reduction::no_change();
  assert!(!no_change.changed());
  assert!(no_change.replacement().is_none());

  OK
}

#[test]
fn test_graph_assembler() -> Void {
  info!("Testing graph assembler and jsgraph");

  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test");
  let mut jsgraph = JSGraph::new(&zone);
  let temp_zone = Zone::new(&allocator, "temp");

  let assembler = GraphAssembler::new(&mut jsgraph, &temp_zone, BranchSemantics::JS, false);

  assert!(!assembler.has_active_block());

  let label = GraphAssemblerLabel::new(
    GraphAssemblerLabelType::NonDeferred,
    0,
    vec![MachineRepresentation::Word32],
  );
  assert_eq!(label.count(), 1);
  assert!(!label.is_bound);

  OK
}
