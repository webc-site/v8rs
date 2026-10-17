use aok::{OK, Void};
use log::info;
use v8_maglev_backend::{arm64, x64};
use v8_maglev_frontend::Graph;
use v8_zone::{AccountingAllocator, Zone};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_maglev_backend_x64() -> Void {
  info!("Testing maglev backend x64");

  assert_eq!(x64::scale_factor_from_int(1), Some(0));
  assert_eq!(x64::scale_factor_from_int(2), Some(1));
  assert_eq!(x64::scale_factor_from_int(4), Some(2));
  assert_eq!(x64::scale_factor_from_int(8), Some(3));
  assert_eq!(x64::scale_factor_from_int(16), None);

  assert_eq!(x64::CONDITION_FOR_NAN, ("parity_even", 10));
  const { assert!(x64::CONDITION_FOR_FLOAT64.len() == 6) };
  const { assert!(x64::SCALE_FACTOR.len() == 7) };

  assert_eq!(
    x64::ASM_METHODS
      .iter()
      .filter(|d| d.guard == Some("V8_STATIC_ROOTS_BOOL"))
      .count(),
    3
  );
  assert_eq!(
    x64::ASM_METHODS
      .iter()
      .filter(|d| d.guard == Some("V8_ENABLE_UNDEFINED_DOUBLE"))
      .count(),
    3
  );

  OK
}

#[test]
fn test_maglev_backend_arm64() -> Void {
  info!("Testing maglev backend arm64");

  assert_eq!(arm64::shift_from_scale(1), Some(0));
  assert_eq!(arm64::shift_from_scale(2), Some(1));
  assert_eq!(arm64::shift_from_scale(4), Some(2));
  assert_eq!(arm64::shift_from_scale(8), Some(3));
  assert_eq!(arm64::shift_from_scale(16), None);

  assert_eq!(arm64::CONDITION_FOR_NAN, ("vs", 6));
  assert_eq!(arm64::MAP_COMPARE_TEMPORARY_COUNT, 1);
  assert_eq!(arm64::LOOP_UNROLL_SIZE, 8);

  const { assert!(arm64::INL_METHODS.len() == 185) };

  assert_eq!(
    arm64::INL_METHODS
      .iter()
      .filter(|d| d.guard == Some("V8_STATIC_ROOTS_BOOL"))
      .count(),
    3
  );
  assert_eq!(
    arm64::INL_METHODS
      .iter()
      .filter(|d| d.guard == Some("V8_ENABLE_UNDEFINED_DOUBLE"))
      .count(),
    3
  );

  OK
}

#[test]
fn test_compile_maglev_graph() -> Void {
  let allocator = AccountingAllocator::default();
  let zone = Zone::new(&allocator, "test-maglev-zone");
  let graph = Graph::new(&zone);
  let res = v8_maglev_backend::compile_maglev_graph(&graph);
  assert!(res.is_ok());
  OK
}
