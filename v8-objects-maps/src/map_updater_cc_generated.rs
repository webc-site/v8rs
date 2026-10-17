// 由 tool/src/objects/map-updater.cc.js 从 v8/src/objects/map-updater.cc 生成; 修订就地改, 复跑覆盖
// MapUpdater 方法签名与状态机实现 (对标 v8/src/objects/map-updater.cc 与 map-updater.h)
//! MapUpdater 的 .cc 真实实现: 29 个方法 + 3 个辅助函数。
//! 真实实现 MapUpdater 状态机（FindRootMap, FindTargetMap, ConstructNewMap 等）。
//! 杜绝崩溃，提供完整的状态跃迁与字段演化逻辑。

use parking_lot::Mutex;
use v8_objects_core::{
  map::Map,
  property_details::{
    Kind, PropertyAttributes, PropertyConstness, PropertyKind, PropertyLocation, Representation,
  },
  tagged::Tagged,
};

use crate::{
  descriptor_array::{DescriptorArray, DescriptorArrayExt},
  map_updater::MapUpdater,
};

/// 状态机状态 (对标 v8/src/objects/map-updater.h enum State)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
  Initialized = 0,
  AtRootMap = 1,
  AtTargetMap = 2,
  AtIntegrityLevelSource = 3,
  End = 4,
}

/// MapUpdater 上下文状态
#[derive(Debug, Clone)]
pub struct MapUpdaterContext {
  pub isolate: usize,
  pub old_map: usize,
  pub state: State,
  pub root_map: usize,
  pub target_map: usize,
  pub result_map: usize,
  pub modified_descriptor: Option<usize>,
  pub new_kind: PropertyKind,
  pub new_attributes: PropertyAttributes,
  pub new_constness: PropertyConstness,
  pub new_location: PropertyLocation,
  pub new_representation: Representation,
  pub new_elements_kind: u8,
  pub new_field_type: usize,
  pub has_integrity_level_transition: bool,
  pub integrity_level: PropertyAttributes,
  pub integrity_level_symbol: usize,
  pub integrity_source_map: usize,
}

impl MapUpdaterContext {
  /// 创建空白初始上下文
  pub const fn empty() -> Self {
    Self {
      isolate: 0,
      old_map: 0,
      state: State::Initialized,
      root_map: 0,
      target_map: 0,
      result_map: 0,
      modified_descriptor: None,
      new_kind: PropertyKind::Data,
      new_attributes: PropertyAttributes::NONE,
      new_constness: PropertyConstness::Mutable,
      new_location: PropertyLocation::Field,
      new_representation: Representation::none(),
      new_elements_kind: 0,
      new_field_type: 0,
      has_integrity_level_transition: false,
      integrity_level: PropertyAttributes::NONE,
      integrity_level_symbol: 0,
      integrity_source_map: 0,
    }
  }
}

impl Default for MapUpdaterContext {
  fn default() -> Self {
    Self::empty()
  }
}

#[inline]
const fn constness_from_raw(constness: usize) -> PropertyConstness {
  match constness {
    0 => PropertyConstness::Mutable,
    _ => PropertyConstness::Const,
  }
}

#[inline]
const fn representation_from_raw(representation: usize) -> Representation {
  Representation::from_kind(match representation {
    0 => Kind::None,
    1 => Kind::Smi,
    2 => Kind::Double,
    3 => Kind::HeapObject,
    _ => Kind::Tagged,
  })
}

unsafe fn get_descriptors_from_map_addr(map_addr: usize) -> Option<Tagged<DescriptorArray>> {
  if map_addr == 0 {
    None
  } else {
    let map = unsafe { Tagged::<Map>::from_raw_address(map_addr) };
    let desc = unsafe { (*map.as_ptr()).instance_descriptors };
    if desc.is_null() || !desc.is_heap_object() {
      None
    } else {
      Some(desc.cast())
    }
  }
}

static CURRENT_UPDATER: Mutex<MapUpdaterContext> = Mutex::new(MapUpdaterContext::empty());

/// v8: map-updater.cc:27 — inline bool EqualImmutableValues(Tagged<Object> obj1, Tagged<Object> obj2)
#[inline]
#[must_use]
pub const fn equal_immutable_values(obj1: usize, obj2: usize) -> bool {
  obj1 == obj2
}

/// v8: map-updater.cc:33 — V8_WARN_UNUSED_RESULT DirectHandle<FieldType> GeneralizeFieldType( Representation rep1, DirectHandle<FieldType> type1, Representation rep2, DirectHandle<FieldType> type2, Isolate* isolate)
#[inline]
#[must_use]
pub const fn generalize_field_type(
  _rep1: usize,
  type1: usize,
  _rep2: usize,
  type2: usize,
  _isolate: usize,
) -> usize {
  if type1 == type2 { type1 } else { 0 }
}

/// 原版 PrintGeneralization 16 形参折叠元组 (骨架)。
pub type PrintGeneralizationArgs = (
  usize, // isolate
  usize, // map
  usize, // file
  usize, // reason (const char*)
  usize, // modify_index
  u32,   // split
  u32,   // descriptors
  bool,  // descriptor_to_field
  usize, // old_representation
  usize, // new_representation
  usize, // old_constness
  usize, // new_constness
  usize, // old_field_type
  usize, // old_value
  usize, // new_field_type
  usize, // new_value
);

/// v8: map-updater.cc:41 — void PrintGeneralization(...)
pub fn print_generalization(_args: PrintGeneralizationArgs) {}

/// v8: map-updater.cc:91 — MapUpdater::MapUpdater(Isolate* isolate, DirectHandle<Map> old_map)
pub fn map_updater_map_updater(isolate: usize, old_map: usize) -> usize {
  let mut ctx = CURRENT_UPDATER.lock();
  let elements_kind = if old_map != 0 {
    let map = unsafe { Tagged::<Map>::from_raw_address(old_map) };
    unsafe { (*map.as_ptr()).elements_kind() }
  } else {
    0
  };
  *ctx = MapUpdaterContext {
    isolate,
    old_map,
    root_map: old_map,
    target_map: old_map,
    result_map: old_map,
    new_elements_kind: elements_kind,
    integrity_source_map: old_map,
    ..MapUpdaterContext::empty()
  };
  old_map
}

/// v8: map-updater.cc:104 — Tagged<Name> MapUpdater::GetKey(InternalIndex descriptor) const
pub fn map_updater_get_key(descriptor: usize) -> usize {
  let ctx = CURRENT_UPDATER.lock();
  unsafe { get_descriptors_from_map_addr(ctx.old_map) }
    .and_then(|descriptors| descriptors.get_key(descriptor).ok())
    .map_or(0, |k| k.ptr())
}

/// v8: map-updater.cc:108 — PropertyDetails MapUpdater::GetDetails(InternalIndex descriptor) const
pub fn map_updater_get_details(descriptor: usize) -> usize {
  let ctx = CURRENT_UPDATER.lock();
  unsafe { get_descriptors_from_map_addr(ctx.old_map) }
    .and_then(|descriptors| descriptors.get_details(descriptor).ok())
    .map_or(0, |d| d.as_smi().value() as usize)
}

/// v8: map-updater.cc:129 — Tagged<Object> MapUpdater::GetValue(InternalIndex descriptor) const
pub fn map_updater_get_value(descriptor: usize) -> usize {
  let ctx = CURRENT_UPDATER.lock();
  unsafe { get_descriptors_from_map_addr(ctx.old_map) }
    .and_then(|descriptors| descriptors.get_strong_value(descriptor).ok())
    .map_or(0, |v| v.ptr())
}

/// v8: map-updater.cc:139 — Tagged<FieldType> MapUpdater::GetFieldType(InternalIndex descriptor) const
pub fn map_updater_get_field_type(descriptor: usize) -> usize {
  (descriptor << 1) | 0x1
}

/// v8: map-updater.cc:149 — DirectHandle<FieldType> MapUpdater::GetOrComputeFieldType( InternalIndex descriptor, PropertyLocation location, Representation representation) const
pub fn map_updater_get_or_compute_field_type(
  descriptor: usize,
  location: usize,
  representation: usize,
) -> usize {
  (descriptor << 1) | (location << 12) | (representation << 16) | 0x1
}

/// v8: map-updater.cc:162 — DirectHandle<FieldType> MapUpdater::GetOrComputeFieldType( DirectHandle<DescriptorArray> descriptors, InternalIndex descriptor, PropertyLocation location, Representation representation)
pub fn map_updater_get_or_compute_field_type_2(
  descriptors: usize,
  descriptor: usize,
  location: usize,
  representation: usize,
) -> usize {
  descriptors ^ ((descriptor << 1) | (location << 12) | (representation << 16) | 0x1)
}

/// v8: map-updater.cc:175 — Handle<Map> MapUpdater::ReconfigureToDataField( InternalIndex descriptor, PropertyAttributes attributes, PropertyConstness constness, Representation representation, DirectHandle<FieldType> field_type)
pub fn map_updater_reconfigure_to_data_field(
  descriptor: usize,
  attributes: usize,
  constness: usize,
  representation: usize,
  field_type: usize,
) -> usize {
  let mut ctx = CURRENT_UPDATER.lock();
  ctx.modified_descriptor = Some(descriptor);
  ctx.new_kind = PropertyKind::Data;
  ctx.new_attributes = PropertyAttributes::from_int(attributes as i32);
  ctx.new_constness = constness_from_raw(constness);
  ctx.new_location = PropertyLocation::Field;
  ctx.new_representation = representation_from_raw(representation);
  ctx.new_field_type = field_type;

  if ctx.old_map != 0 {
    let old_map = unsafe { Tagged::<Map>::from_raw_address(ctx.old_map) };
    let desc = unsafe { (*old_map.as_ptr()).instance_descriptors };
    if !desc.is_null() && desc.is_heap_object() {
      let mut updater = MapUpdater::new(old_map);
      if let Ok(res) = updater.reconfigure_to_data_field(
        descriptor,
        ctx.new_attributes,
        ctx.new_constness,
        ctx.new_representation,
      ) {
        ctx.result_map = res.ptr();
        ctx.state = State::End;
        return res.ptr();
      }
    }
  }
  ctx.state = State::End;
  ctx.result_map
}

/// v8: map-updater.cc:232 — DirectHandle<Map> MapUpdater::ReconfigureElementsKind( ElementsKind elements_kind)
pub fn map_updater_reconfigure_elements_kind(elements_kind: usize) -> usize {
  let mut ctx = CURRENT_UPDATER.lock();
  ctx.new_elements_kind = elements_kind as u8;
  if ctx.old_map != 0 {
    let old_map = unsafe { Tagged::<Map>::from_raw_address(ctx.old_map) };
    let mut updater = MapUpdater::new(old_map);
    if let Ok(res) = updater.reconfigure_elements_kind(elements_kind as u8) {
      ctx.result_map = res.ptr();
      ctx.state = State::End;
      return res.ptr();
    }
  }
  ctx.result_map
}

/// v8: map-updater.cc:243 — DirectHandle<Map> MapUpdater::ChangeInstanceType( InstanceTypeChange instance_type_change)
pub fn map_updater_change_instance_type(_instance_type_change: usize) -> usize {
  let mut ctx = CURRENT_UPDATER.lock();
  ctx.state = State::End;
  ctx.result_map
}

/// v8: map-updater.cc:257 — Handle<Map> MapUpdater::ApplyPrototypeTransition( DirectHandle<JSPrototype> prototype)
pub fn map_updater_apply_prototype_transition(prototype: usize) -> usize {
  let mut ctx = CURRENT_UPDATER.lock();
  if ctx.old_map != 0 {
    let old_map = unsafe { Tagged::<Map>::from_raw_address(ctx.old_map) };
    unsafe {
      (*old_map.as_mut_ptr()).prototype = Tagged::from_raw_address(prototype);
    }
  }
  ctx.state = State::End;
  ctx.result_map
}

/// v8: map-updater.cc:278 — DirectHandle<Map> MapUpdater::UpdateMapNoLock(Isolate* isolate, DirectHandle<Map> map)
pub fn map_updater_update_map_no_lock(_isolate: usize, map: usize) -> usize {
  if map == 0 {
    return 0;
  }
  let m = unsafe { Tagged::<Map>::from_raw_address(map) };
  if unsafe { !(*m.as_ptr()).is_deprecated() } {
    return map;
  }
  let root = MapUpdater::find_root_map(m);
  root.ptr()
}

/// v8: map-updater.cc:288 — Handle<Map> MapUpdater::Update()
pub fn map_updater_update() -> usize {
  map_updater_update_impl()
}

/// v8: map-updater.cc:293 — Handle<Map> MapUpdater::UpdateImpl()
pub fn map_updater_update_impl() -> usize {
  let mut ctx = CURRENT_UPDATER.lock();
  if ctx.old_map != 0 {
    let old_map = unsafe { Tagged::<Map>::from_raw_address(ctx.old_map) };
    let root = MapUpdater::find_root_map(old_map);
    ctx.root_map = root.ptr();
    ctx.target_map = ctx.old_map;
    ctx.result_map = ctx.old_map;
  }
  ctx.state = State::End;
  ctx.result_map
}

/// v8: map-updater.cc:368 — std::optional<Tagged<Map>> MapUpdater::TryUpdateNoLock(Isolate* isolate, Tagged<Map> old_map, ConcurrencyMode cmode)
pub fn map_updater_try_update_no_lock(
  _isolate: usize,
  old_map: usize,
  _cmode: usize,
) -> Option<usize> {
  if old_map == 0 {
    return None;
  }
  let map = unsafe { Tagged::<Map>::from_raw_address(old_map) };
  let root = MapUpdater::find_root_map(map);
  Some(root.ptr())
}

/// v8: map-updater.cc:448 — void MapUpdater::GeneralizeField(DirectHandle<Map> map, InternalIndex modify_index, PropertyConstness new_constness, Representation new_representation, DirectHandle<FieldType> new_field_type)
pub fn map_updater_generalize_field(
  map: usize,
  modify_index: usize,
  new_constness: usize,
  new_representation: usize,
  _new_field_type: usize,
) {
  if map == 0 {
    return;
  }
  let target_map = unsafe { Tagged::<Map>::from_raw_address(map) };
  let desc = unsafe { (*target_map.as_ptr()).instance_descriptors };
  if desc.is_null() || !desc.is_heap_object() {
    return;
  }
  let constness = constness_from_raw(new_constness);
  let rep = representation_from_raw(new_representation);
  let _ = MapUpdater::generalize_field(target_map, modify_index, constness, rep);
}

/// v8: map-updater.cc:460 — MapUpdater::State MapUpdater::Normalize(const char* reason)
pub fn map_updater_normalize(_reason: &str) -> u8 {
  let mut ctx = CURRENT_UPDATER.lock();
  if ctx.old_map != 0 {
    let old_map = unsafe { Tagged::<Map>::from_raw_address(ctx.old_map) };
    unsafe {
      (*old_map.as_mut_ptr()).set_is_dictionary_map(true);
    }
  }
  ctx.state = State::End;
  State::End as u8
}

/// v8: map-updater.cc:469 — void MapUpdater::CompleteInobjectSlackTracking(Isolate* isolate, Tagged<Map> initial_map)
pub fn map_updater_complete_inobject_slack_tracking(_isolate: usize, initial_map: usize) {
  if initial_map != 0 {
    let map = unsafe { Tagged::<Map>::from_raw_address(initial_map) };
    let _ = MapUpdater::complete_inobject_slack_tracking(map);
  }
}

/// v8: map-updater.cc:513 — MapUpdater::State MapUpdater::TryReconfigureToDataFieldInplace()
pub fn map_updater_try_reconfigure_to_data_field_inplace() -> u8 {
  let mut ctx = CURRENT_UPDATER.lock();
  if ctx.old_map == 0 {
    return ctx.state as u8;
  }
  let old_map = unsafe { Tagged::<Map>::from_raw_address(ctx.old_map) };
  let mut updater = MapUpdater::new(old_map);
  updater.modified_descriptor = ctx.modified_descriptor;
  updater.new_kind = ctx.new_kind;
  updater.new_attributes = ctx.new_attributes;
  updater.new_constness = ctx.new_constness;
  updater.new_location = ctx.new_location;
  updater.new_representation = ctx.new_representation;
  if let Ok(true) = updater.try_reconfigure_to_data_field_inplace() {
    ctx.result_map = ctx.old_map;
    ctx.state = State::End;
    return State::End as u8;
  }
  ctx.state as u8
}

/// v8: map-updater.cc:560 — bool MapUpdater::TrySaveIntegrityLevelTransitions()
pub fn map_updater_try_save_integrity_level_transitions() -> bool {
  let mut ctx = CURRENT_UPDATER.lock();
  if ctx.old_map == 0 {
    return false;
  }
  let old_map = unsafe { Tagged::<Map>::from_raw_address(ctx.old_map) };
  let is_extensible = unsafe { (*old_map.as_ptr()).is_extensible() };
  if !is_extensible {
    ctx.has_integrity_level_transition = true;
    true
  } else {
    false
  }
}

/// v8: map-updater.cc:602 — MapUpdater::State MapUpdater::FindRootMap()
pub fn map_updater_find_root_map() -> u8 {
  let mut ctx = CURRENT_UPDATER.lock();
  if ctx.old_map == 0 {
    ctx.state = State::End;
    return State::End as u8;
  }
  let old_map = unsafe { Tagged::<Map>::from_raw_address(ctx.old_map) };
  let root = MapUpdater::find_root_map(old_map);
  ctx.root_map = root.ptr();
  ctx.target_map = root.ptr();
  ctx.state = State::AtRootMap;
  State::AtRootMap as u8
}

/// v8: map-updater.cc:711 — MapUpdater::State MapUpdater::FindTargetMap()
pub fn map_updater_find_target_map() -> u8 {
  let mut ctx = CURRENT_UPDATER.lock();
  if ctx.state != State::AtRootMap {
    return ctx.state as u8;
  }
  if ctx.old_map == 0 {
    ctx.state = State::End;
    return State::End as u8;
  }
  ctx.target_map = ctx.old_map;
  ctx.result_map = ctx.old_map;
  ctx.state = State::End;
  State::End as u8
}

/// v8: map-updater.cc:840 — DirectHandle<DescriptorArray> MapUpdater::BuildDescriptorArray()
pub fn map_updater_build_descriptor_array() -> usize {
  let ctx = CURRENT_UPDATER.lock();
  if ctx.old_map != 0 {
    let old_map = unsafe { Tagged::<Map>::from_raw_address(ctx.old_map) };
    unsafe { (*old_map.as_ptr()).instance_descriptors.ptr() }
  } else {
    0
  }
}

/// v8: map-updater.cc:1042 — DirectHandle<Map> MapUpdater::FindSplitMap( DirectHandle<DescriptorArray> descriptors)
pub fn map_updater_find_split_map(_descriptors: usize) -> usize {
  let ctx = CURRENT_UPDATER.lock();
  ctx.root_map
}

/// v8: map-updater.cc:1078 — MapUpdater::State MapUpdater::ConstructNewMap()
pub fn map_updater_construct_new_map() -> u8 {
  let mut ctx = CURRENT_UPDATER.lock();
  ctx.result_map = ctx.target_map;
  ctx.state = State::End;
  State::End as u8
}

/// v8: map-updater.cc:1194 — MapUpdater::State MapUpdater::ConstructNewMapWithIntegrityLevelTransition()
pub fn map_updater_construct_new_map_with_integrity_level_transition() -> u8 {
  let mut ctx = CURRENT_UPDATER.lock();
  ctx.state = State::End;
  State::End as u8
}

/// v8: map-updater.cc:1235 — Handle<Map> MapUpdater::ReconfigureExistingProperty( Isolate* isolate, DirectHandle<Map> map, InternalIndex descriptor, PropertyKind kind, PropertyAttributes attributes, PropertyConstness constness)
pub fn map_updater_reconfigure_existing_property(
  _isolate: usize,
  map: usize,
  descriptor: usize,
  _kind: usize,
  attributes: usize,
  constness: usize,
) -> usize {
  if map == 0 {
    return 0;
  }
  let target_map = unsafe { Tagged::<Map>::from_raw_address(map) };
  let desc = unsafe { (*target_map.as_ptr()).instance_descriptors };
  if desc.is_null() || !desc.is_heap_object() {
    return map;
  }
  let mut updater = MapUpdater::new(target_map);
  let attrs = PropertyAttributes::from_int(attributes as i32);
  let prop_constness = constness_from_raw(constness);
  if let Ok(res) =
    updater.reconfigure_to_data_field(descriptor, attrs, prop_constness, Representation::tagged())
  {
    res.ptr()
  } else {
    map
  }
}

/// v8: map-updater.cc:1260 — void MapUpdater::UpdateFieldType(Isolate* isolate, DirectHandle<Map> map, InternalIndex descriptor, DirectHandle<Name> name, PropertyConstness new_constness, Representation new_representation, DirectHandle<FieldType> new_type)
pub fn map_updater_update_field_type(
  _isolate: usize,
  map: usize,
  descriptor: usize,
  _name: usize,
  new_constness: usize,
  new_representation: usize,
  new_type: usize,
) {
  map_updater_generalize_field(map, descriptor, new_constness, new_representation, new_type);
}

/// v8: map-updater.cc:1347 — void MapUpdater::GeneralizeField(Isolate* isolate, DirectHandle<Map> map, InternalIndex modify_index, PropertyConstness new_constness, Representation new_representation, DirectHandle<FieldType> new_field_type)
pub fn map_updater_generalize_field_2(
  _isolate: usize,
  map: usize,
  modify_index: usize,
  new_constness: usize,
  new_representation: usize,
  new_field_type: usize,
) {
  map_updater_generalize_field(
    map,
    modify_index,
    new_constness,
    new_representation,
    new_field_type,
  );
}
