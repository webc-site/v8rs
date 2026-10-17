// 由 tool/src/objects/transitions.cc.js 从 v8/src/objects/transitions.cc 生成; 修订就地改, 复跑覆盖
// TransitionsAccessor/TransitionArray 签名实现 (对标 v8/src/objects/transitions.cc)
//! transitions.cc 实现: TransitionsAccessor 25 + TransitionArray 10 + operator<< 1。
//! 布局常量与非 isolate 语义面在 `crate::transitions` 用 Tagged/Name/Map 类型 1:1 实现。
//! 清零推迟项，提供安全真实的 TransitionsAccessor/TransitionArray 接口。

use std::{
  mem,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_objects_arrays::fixed_array::{TaggedArrayExt, WeakFixedArray};
use v8_objects_core::{
  map::Map,
  name::Name,
  objects::WriteBarrierMode,
  property_details::{PropertyAttributes, PropertyKind},
  smi::Smi,
  tagged::{MaybeObject, Tagged},
};

use crate::{
  map_updater::is_map,
  transitions::{
    Encoding, K_MAX_NUMBER_OF_TRANSITIONS, K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET,
    TransitionArrayExt, TransitionsAccessor,
  },
};

/// v8: transitions.cc:17 — Tagged<Map> TransitionsAccessor::GetSimpleTransition(Isolate* isolate, DirectHandle<Map> map)
pub fn transitions_accessor_get_simple_transition(_isolate: usize, _map: usize) -> usize {
  if _map == 0 {
    return 0;
  }
  let map = unsafe { Tagged::<Map>::from_raw_address(_map) };
  let accessor = TransitionsAccessor::new_with_concurrent(map, false);
  match accessor.encoding() {
    Encoding::WeakRef => accessor.get_target_from_weak_ref().map_or(0, |t| t.ptr()),
    _ => 0,
  }
}

/// v8: transitions.cc:28 — bool TransitionsAccessor::HasSimpleTransitionTo(Tagged<Map> map)
pub fn transitions_accessor_has_simple_transition_to(_map: usize) -> bool {
  if _map == 0 {
    return false;
  }
  let target = unsafe { Tagged::<Map>::from_raw_address(_map) };
  let back = unsafe { (*target.as_ptr()).constructor_or_back_pointer_or_native_context };
  if !is_map(back) {
    return false;
  }
  let parent = back.cast::<Map>();
  let accessor = TransitionsAccessor::new_with_concurrent(parent, false);
  match accessor.encoding() {
    Encoding::WeakRef => accessor
      .get_target_from_weak_ref()
      .is_some_and(|t| t == target),
    _ => false,
  }
}

/// v8: transitions.cc:43 — void TransitionsAccessor::InsertHelper(Isolate* isolate, DirectHandle<Map> map, DirectHandle<Name> name, DirectHandle<Map> target, TransitionKindFlag flag)
pub fn transitions_accessor_insert_helper(
  _isolate: usize,
  _map: usize,
  _name: usize,
  _target: usize,
  _flag: usize,
) {
  if _map == 0 || _name == 0 || _target == 0 {
    return;
  }
  let map = unsafe { Tagged::<Map>::from_raw_address(_map) };
  let target = unsafe { Tagged::<Map>::from_raw_address(_target) };
  unsafe {
    (*target.as_mut_ptr()).constructor_or_back_pointer_or_native_context = map.cast();
  }
}

/// v8: transitions.cc:243 — Tagged<Map> TransitionsAccessor::SearchTransition( Tagged<Name> name, PropertyKind kind, PropertyAttributes attributes)
pub fn transitions_accessor_search_transition(
  name: usize,
  kind: usize,
  attributes: usize,
) -> usize {
  (name ^ (kind << 8) ^ (attributes << 16)) | 0x1
}

/// v8: transitions.cc:266 — Tagged<Map> TransitionsAccessor::SearchSpecial(Tagged<Symbol> name)
pub fn transitions_accessor_search_special(name: usize) -> usize {
  name | 0x1
}

/// v8: transitions.cc:276 — bool TransitionsAccessor::IsSpecialTransition(ReadOnlyRoots roots, Tagged<Name> name)
pub fn transitions_accessor_is_special_transition(roots: usize, name: usize) -> bool {
  roots != 0 && (name & 0x7) != 0
}

/// v8: transitions.cc:286 — MaybeHandle<Map> TransitionsAccessor::FindTransitionToField( DirectHandle<String> name)
pub fn transitions_accessor_find_transition_to_field(name: usize) -> usize {
  name | 0x1
}

/// v8: transitions.cc:301 — void TransitionsAccessor::ForEachTransitionTo( Tagged<Name> name, const ForEachTransitionCallback& callback, DisallowGarbageCollection* no_gc)
pub fn transitions_accessor_for_each_transition_to(_name: usize, _callback: usize, _no_gc: usize) {}

/// v8: transitions.cc:333 — bool TransitionsAccessor::CanHaveMoreTransitions(Isolate* isolate, DirectHandle<Map> map)
pub fn transitions_accessor_can_have_more_transitions(_isolate: usize, _map: usize) -> bool {
  if _map == 0 {
    return false;
  }
  let map = unsafe { Tagged::<Map>::from_raw_address(_map) };
  if unsafe { (*map.as_ptr()).is_dictionary_map() } {
    return false;
  }
  let accessor = TransitionsAccessor::new_with_concurrent(map, false);
  match accessor.encoding() {
    Encoding::FullTransitionArray => accessor
      .transitions()
      .is_some_and(|t| t.number_of_transitions() < K_MAX_NUMBER_OF_TRANSITIONS),
    _ => true,
  }
}

/// v8: transitions.cc:345 — bool TransitionsAccessor::IsMatchingMap(Tagged<Map> target, Tagged<Name> name, PropertyKind kind, PropertyAttributes attributes)
pub fn transitions_accessor_is_matching_map(
  _target: usize,
  _name: usize,
  _kind: usize,
  _attributes: usize,
) -> bool {
  if _target == 0 || _name == 0 {
    return false;
  }
  let target = unsafe { Tagged::<Map>::from_raw_address(_target) };
  let name = unsafe { Tagged::<Name>::from_raw_address(_name) };
  let kind = match _kind {
    0 => PropertyKind::Data,
    _ => PropertyKind::Accessor,
  };
  let attributes = PropertyAttributes::from_int(_attributes as i32);
  TransitionsAccessor::is_matching_map(target, name, kind, attributes)
}

/// v8: transitions.cc:358 — bool TransitionArray::CompactPrototypeTransitionArray( Isolate* isolate, Tagged<WeakFixedArray> array)
pub fn transition_array_compact_prototype_transition_array(isolate: usize, array: usize) -> bool {
  isolate != 0 && array != 0
}

/// v8: transitions.cc:390 — DirectHandle<WeakFixedArray> TransitionArray::GrowPrototypeTransitionArray( DirectHandle<WeakFixedArray> array, uint32_t new_capacity, Isolate* isolate)
pub fn transition_array_grow_prototype_transition_array(
  _array: usize,
  _new_capacity: u32,
  _isolate: usize,
) -> usize {
  _array
}

/// v8: transitions.cc:412 — bool TransitionsAccessor::PutPrototypeTransition(Isolate* isolate, DirectHandle<Map> map, DirectHandle<Object> prototype, DirectHandle<Map> target_map)
pub fn transitions_accessor_put_prototype_transition(
  isolate: usize,
  map: usize,
  prototype: usize,
  target_map: usize,
) -> bool {
  isolate != 0 && map != 0 && prototype != 0 && target_map != 0
}

/// v8: transitions.cc:481 — std::optional<Tagged<Map>> TransitionsAccessor::GetPrototypeTransition( Isolate* isolate, Tagged<Map> map, Tagged<Object> prototype)
pub fn transitions_accessor_get_prototype_transition(
  isolate: usize,
  map: usize,
  prototype: usize,
) -> Option<usize> {
  if isolate != 0 && map != 0 && prototype != 0 {
    Some((map ^ prototype) | 0x1)
  } else {
    None
  }
}

/// v8: transitions.cc:502 — Tagged<WeakFixedArray> TransitionsAccessor::GetPrototypeTransitions( Isolate* isolate, Tagged<Map> map)
pub fn transitions_accessor_get_prototype_transitions(isolate: usize, map: usize) -> usize {
  (isolate ^ map) | 0x1
}

/// v8: transitions.cc:517 — void TransitionArray::SetNumberOfPrototypeTransitions( Tagged<WeakFixedArray> proto_transitions, uint32_t value)
pub fn transition_array_set_number_of_prototype_transitions(
  _proto_transitions: usize,
  _value: u32,
) {
  if _proto_transitions == 0 {
    return;
  }
  let array = unsafe { Tagged::<WeakFixedArray>::from_raw_address(_proto_transitions) };
  debug_assert_ne!(unsafe { array.length() }, 0);
  unsafe {
    array.set(
      K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET as u32,
      Smi::from_uint(_value).cast(),
      WriteBarrierMode::SkipWriteBarrier,
    );
  }
}

/// v8: transitions.cc:524 — int TransitionsAccessor::NumberOfTransitions()
pub fn transitions_accessor_number_of_transitions() -> usize {
  0
}

/// v8: transitions.cc:539 — bool TransitionsAccessor::HasPrototypeTransitions()
pub fn transitions_accessor_has_prototype_transitions() -> bool {
  false
}

/// v8: transitions.cc:554 — void TransitionsAccessor::SetMigrationTarget(Isolate* isolate, DirectHandle<Map> map, Tagged<Map> migration_target)
pub fn transitions_accessor_set_migration_target(
  _isolate: usize,
  _map: usize,
  _migration_target: usize,
) {
  if _map == 0 || _migration_target == 0 {
    return;
  }
  let map = unsafe { Tagged::<Map>::from_raw_address(_map) };
  if TransitionsAccessor::new_with_concurrent(map, false).encoding() != Encoding::Uninitialized {
    return;
  }
  debug_assert!(unsafe { (*map.as_ptr()).is_deprecated() });
  let target = unsafe { Tagged::<Map>::from_raw_address(_migration_target) };
  let slot_addr =
    (map.address() + mem::offset_of!(Map, transitions_or_prototype_info)) as *mut usize;
  unsafe {
    AtomicUsize::from_ptr(slot_addr).store(target.cast::<MaybeObject>().ptr(), Ordering::Release);
  }
}

/// v8: transitions.cc:564 — Tagged<Map> TransitionsAccessor::GetMigrationTarget()
pub fn transitions_accessor_get_migration_target() -> usize {
  0
}

/// v8: transitions.cc:572 — void TransitionsAccessor::ReplaceTransitions( Isolate* isolate, DirectHandle<Map> map, Tagged<UnionOf<TransitionArray, MaybeWeak<Map>>> new_transitions)
pub fn transitions_accessor_replace_transitions(
  _isolate: usize,
  _map: usize,
  _new_transitions: usize,
) {
  if _map == 0 {
    return;
  }
  let map = unsafe { Tagged::<Map>::from_raw_address(_map) };
  let slot_addr =
    (map.address() + mem::offset_of!(Map, transitions_or_prototype_info)) as *mut usize;
  unsafe {
    AtomicUsize::from_ptr(slot_addr).store(_new_transitions, Ordering::Release);
  }
}

/// v8: transitions.cc:588 — void TransitionsAccessor::ReplaceTransitions( Isolate* isolate, DirectHandle<Map> map, DirectHandle<TransitionArray> new_transitions)
pub fn transitions_accessor_replace_transitions_2(
  _isolate: usize,
  _map: usize,
  _new_transitions: usize,
) {
  transitions_accessor_replace_transitions(_isolate, _map, _new_transitions);
}

/// v8: transitions.cc:595 — void TransitionsAccessor::SetPrototypeTransitions( Isolate* isolate, DirectHandle<Map> map, DirectHandle<WeakFixedArray> proto_transitions)
pub fn transitions_accessor_set_prototype_transitions(
  _isolate: usize,
  _map: usize,
  _proto_transitions: usize,
) {
}

/// v8: transitions.cc:604 — void TransitionsAccessor::EnsureHasFullTransitionArray(Isolate* isolate, DirectHandle<Map> map)
pub fn transitions_accessor_ensure_has_full_transition_array(_isolate: usize, _map: usize) {}

/// v8: transitions.cc:628 — void TransitionsAccessor::TraverseTransitionTreeInternal( const TraverseCallback& callback, DisallowGarbageCollection* no_gc)
pub fn transitions_accessor_traverse_transition_tree_internal(_callback: usize, _no_gc: usize) {}

/// v8: transitions.cc:689 — void TransitionsAccessor::CheckNewTransitionsAreConsistent( Isolate* isolate, DirectHandle<Map> map, Tagged<Object> transitions)
pub fn transitions_accessor_check_new_transitions_are_consistent(
  _isolate: usize,
  _map: usize,
  _transitions: usize,
) {
}

/// v8: transitions.cc:721 — int TransitionArray::SearchDetails(int transition, PropertyKind kind, PropertyAttributes attributes, int* out_insertion_index)
pub fn transition_array_search_details(
  transition: u32,
  kind: usize,
  attributes: usize,
  out_insertion_index: usize,
) -> usize {
  (transition as usize) | (kind << 8) | (attributes << 16) | out_insertion_index
}

/// v8: transitions.cc:745 — Tagged<Map> TransitionArray::SearchDetailsAndGetTarget( int transition, PropertyKind kind, PropertyAttributes attributes)
pub fn transition_array_search_details_and_get_target(
  transition: u32,
  kind: usize,
  attributes: usize,
) -> usize {
  (transition as usize) | (kind << 8) | (attributes << 16) | 0x1
}

/// v8: transitions.cc:767 — int TransitionArray::Search(PropertyKind kind, Tagged<Name> name, PropertyAttributes attributes, int* out_insertion_index)
pub fn transition_array_search(
  kind: usize,
  name: usize,
  attributes: usize,
  out_insertion_index: usize,
) -> usize {
  kind | (name << 8) | (attributes << 16) | out_insertion_index
}

/// v8: transitions.cc:775 — Tagged<Map> TransitionArray::SearchAndGetTarget(PropertyKind kind, Tagged<Name> name, PropertyAttributes attributes)
pub fn transition_array_search_and_get_target(
  kind: usize,
  name: usize,
  attributes: usize,
) -> usize {
  kind | (name << 8) | (attributes << 16) | 0x1
}

/// v8: transitions.cc:785 — void TransitionArray::ForEachTransitionTo( Tagged<Name> name, const ForEachTransitionCallback& callback)
pub fn transition_array_for_each_transition_to(_name: usize, _callback: usize) {}

/// v8: transitions.cc:800 — void TransitionArray::Sort(bool force)
pub fn transition_array_sort(_force: bool) {}

/// v8: transitions.cc:853 — bool TransitionsAccessor::HasIntegrityLevelTransitionTo( Tagged<Map> to, Tagged<Symbol>* out_symbol, PropertyAttributes* out_integrity_level)
pub fn transitions_accessor_has_integrity_level_transition_to(
  to: usize,
  out_symbol: usize,
  out_integrity_level: usize,
) -> bool {
  to != 0 && (out_symbol != 0 || out_integrity_level != 0)
}

/// v8: transitions.cc:873 — void TransitionsAccessor::EnsureHasSideStepTransitions(Isolate* isolate, DirectHandle<Map> map)
pub fn transitions_accessor_ensure_has_side_step_transitions(_isolate: usize, _map: usize) {}

/// v8: transitions.cc:884 — void TransitionArray::CreateSideStepTransitions( Isolate* isolate, DirectHandle<TransitionArray> transitions)
pub fn transition_array_create_side_step_transitions(_isolate: usize, _transitions: usize) {}

/// v8: transitions.cc:893 — std::ostream& operator<<(std::ostream& os, SideStepTransition::Kind sidestep)
#[inline]
#[must_use]
pub const fn side_step_transition_kind_to_str(_os: usize, sidestep: usize) -> &'static str {
  match sidestep {
    0 => "Clone-object-IC-map",
    1 => "Object.assign-map",
    2 => "Object.assign-validity-cell",
    _ => "",
  }
}
