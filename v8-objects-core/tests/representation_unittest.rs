// 迁移规约:
//   representation-unittest.cc TEST_F(RepresentationTest, RepresentationMoreGeneralThan):
//     None/Smi/HeapObject/Double/Tagged 两两 is_more_general_than 正负对
//   property-details.h:95-240 Representation 其余谓词语义自证
//     (fits_into/generalize/MightCauseMapDeprecation/CanBeInPlaceChangedTo/
//      IsCompatibleForLoad/Mnemonic/size)
use aok::{OK, Void};
use v8_objects_core::property_details::Representation as Rep;

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

fn pair_positive(more_general: Rep, less_general: Rep) {
  assert!(
    more_general.is_more_general_than(&less_general),
    "{more_general:?} 应比 {less_general:?} 更泛化"
  );
}

fn pair_negative(more_general: Rep, less_general: Rep) {
  assert!(
    !more_general.is_more_general_than(&less_general),
    "{more_general:?} 不应比 {less_general:?} 更泛化"
  );
}

/// 规约 (representation-unittest.cc:45-57)。
#[test]
fn representation_more_general_than() -> Void {
  pair_negative(Rep::none(), Rep::none());
  pair_positive(Rep::smi(), Rep::none());
  pair_positive(Rep::heap_object(), Rep::none());
  pair_positive(Rep::double(), Rep::none());
  pair_positive(Rep::tagged(), Rep::none());

  pair_negative(Rep::none(), Rep::smi());
  pair_negative(Rep::smi(), Rep::smi());
  pair_negative(Rep::heap_object(), Rep::smi());
  pair_positive(Rep::double(), Rep::smi());
  pair_positive(Rep::tagged(), Rep::smi());
  OK
}

/// 规约 (property-details.h:141-193):
/// 弃用触发: 仅 None→X / Smi→X (装箱分配); HeapObject/Double/Tagged/WasmValue 可原地。
#[test]
fn deprecation_and_inplace_predicates() -> Void {
  assert!(Rep::none().might_cause_map_deprecation());
  assert!(Rep::smi().might_cause_map_deprecation());
  for r in [
    Rep::double(),
    Rep::heap_object(),
    Rep::tagged(),
    Rep::wasm_value(),
  ] {
    assert!(!r.might_cause_map_deprecation(), "{r:?}");
  }

  // can_be_in_place_changed_to: 自身恒真; WasmValue 不出入; None 可到除 Double 外任意
  for r in [
    Rep::none(),
    Rep::smi(),
    Rep::double(),
    Rep::heap_object(),
    Rep::tagged(),
    Rep::wasm_value(),
  ] {
    assert!(r.can_be_in_place_changed_to(&r), "{r:?}");
  }
  assert!(Rep::none().can_be_in_place_changed_to(&Rep::tagged()));
  assert!(!Rep::none().can_be_in_place_changed_to(&Rep::double()));
  assert!(!Rep::none().can_be_in_place_changed_to(&Rep::wasm_value()));
  assert!(!Rep::smi().can_be_in_place_changed_to(&Rep::double()));
  assert!(!Rep::smi().can_be_in_place_changed_to(&Rep::heap_object()));
  assert!(Rep::smi().can_be_in_place_changed_to(&Rep::tagged()));
  assert!(Rep::double().can_be_in_place_changed_to(&Rep::tagged()));
  assert!(Rep::heap_object().can_be_in_place_changed_to(&Rep::tagged()));

  // 最泛化原地变更: WasmValue 保持自身, 其余 → Tagged
  assert_eq!(
    Rep::wasm_value().most_generic_in_place_change(),
    Rep::wasm_value()
  );
  assert_eq!(Rep::smi().most_generic_in_place_change(), Rep::tagged());
  OK
}

/// 规约 (property-details.h:185-200): fits_into / generalize 格。
#[test]
fn fits_into_and_generalize_lattice() -> Void {
  assert!(Rep::none().fits_into(&Rep::tagged()));
  assert!(Rep::smi().fits_into(&Rep::tagged()));
  assert!(Rep::smi().fits_into(&Rep::smi()));
  assert!(!Rep::tagged().fits_into(&Rep::smi()));

  // 泛化格 None < Smi < Double < Tagged, HeapObject < Tagged
  assert_eq!(Rep::smi().generalize(Rep::double()), Rep::double());
  assert_eq!(Rep::none().generalize(Rep::smi()), Rep::smi());
  assert_eq!(Rep::smi().generalize(Rep::tagged()), Rep::tagged());
  assert_eq!(Rep::double().generalize(Rep::double()), Rep::double());
  OK
}

/// 规约 (property-details.h:127-135, 195-227):
/// 加载兼容只看 double 位; size: double=8 其余=4; 助记符表。
#[test]
fn compatibility_size_and_mnemonic() -> Void {
  assert!(Rep::double().is_compatible_for_load(&Rep::double()));
  assert!(Rep::smi().is_compatible_for_load(&Rep::heap_object()));
  assert!(!Rep::smi().is_compatible_for_load(&Rep::double()));

  assert_eq!(Rep::double().size(), 8);
  for r in [Rep::smi(), Rep::tagged(), Rep::heap_object()] {
    assert_eq!(r.size(), 4);
  }

  let mnemonics = [
    (Rep::none(), "v"),
    (Rep::smi(), "s"),
    (Rep::double(), "d"),
    (Rep::heap_object(), "h"),
    (Rep::tagged(), "t"),
    (Rep::wasm_value(), "w"),
  ];
  for (r, m) in mnemonics {
    assert_eq!(r.mnemonic(), m);
  }
  OK
}
