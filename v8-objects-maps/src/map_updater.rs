// v8-objects-maps/src/map_updater.rs
// 对应原版 v8/src/objects/map-updater.h + map-updater.cc

//! MapUpdater: 负责 Map 形状演化、属性重配置、字段类型泛化与 Inobject Slack Tracking 结束。
//!
//! 在 V8 对象模型中，当已有属性的类型/表示法/常量性发生泛化时，
//! `MapUpdater` 负责协调就地修改（In-place reconfiguration）或过渡树分支迁移，
//! 并确保整棵过渡树中的相关描述符保持类型一致。

use v8_objects_core::{
  instance_type::InstanceType,
  map::{Map, NO_SLACK_TRACKING},
  property_details::{
    PropertyAttributes, PropertyConstness, PropertyKind, PropertyLocation, Representation,
    constness_is_generalizable_to as is_generalizable_to, generalize_constness,
  },
  tagged::{Object, Tagged},
};

use crate::{
  descriptor_array::{DescriptorArray, DescriptorArrayExt},
  error::Result,
  transitions::TransitionsAccessor,
};

/// 检查给定 Tagged 指针是否为 Map 对象
#[inline]
pub fn is_map(obj: Tagged<Object>) -> bool {
  if !obj.is_heap_object() {
    return false;
  }
  let map_candidate = obj.cast::<Map>();
  // SAFETY: obj 已通过 is_heap_object 检查，指针有效
  unsafe {
    let meta = (*map_candidate.as_ptr()).header.map;
    if !meta.is_heap_object() {
      return false;
    }
    (*meta.as_ptr()).instance_type == InstanceType::Map
  }
}

/// MapUpdater: 协调 Map 的属性演化与结构变迁
#[derive(Debug, Clone)]
pub struct MapUpdater {
  pub old_map: Tagged<Map>,
  pub modified_descriptor: Option<usize>,
  pub new_kind: PropertyKind,
  pub new_attributes: PropertyAttributes,
  pub new_constness: PropertyConstness,
  pub new_location: PropertyLocation,
  pub new_representation: Representation,
  pub new_elements_kind: u8,
}

impl MapUpdater {
  /// 创建 MapUpdater 实例
  #[inline]
  pub fn new(old_map: Tagged<Map>) -> Self {
    // SAFETY: old_map 指针有效
    let elements_kind = unsafe { (*old_map.as_ptr()).elements_kind() };
    Self {
      old_map,
      modified_descriptor: None,
      new_kind: PropertyKind::Data,
      new_attributes: PropertyAttributes::NONE,
      new_constness: PropertyConstness::Mutable,
      new_location: PropertyLocation::Field,
      new_representation: Representation::none(),
      new_elements_kind: elements_kind,
    }
  }

  /// 沿着回指针查找过渡树的根 Map (RootMap)
  pub fn find_root_map(map: Tagged<Map>) -> Tagged<Map> {
    let mut current = map;
    loop {
      // SAFETY: current 指针有效
      let back = unsafe { (*current.as_ptr()).constructor_or_back_pointer_or_native_context };
      if is_map(back) {
        current = back.cast::<Map>();
      } else {
        break;
      }
    }
    current
  }

  /// 查找首先引入指定描述符下标的拥有者 Map (FieldOwner, 对齐 map.cc:789-802)
  pub fn find_field_owner(map: Tagged<Map>, descriptor_index: usize) -> Tagged<Map> {
    let mut result = map;
    loop {
      // SAFETY: result 指针指向有效 Map
      let back = unsafe { (*result.as_ptr()).constructor_or_back_pointer_or_native_context };
      if !is_map(back) {
        break;
      }
      let parent = back.cast::<Map>();
      // SAFETY: parent 指向有效 Map
      let parent_nof = unsafe { (*parent.as_ptr()).number_of_own_descriptors() as usize };
      if parent_nof <= descriptor_index {
        break;
      }
      result = parent;
    }
    result
  }

  /// 尝试对属性执行就地重配置 (In-place Reconfiguration)
  ///
  /// 若满足以下条件则可直接在原地扩展字段表示法：
  /// 1. 当前 Map 未被弃用
  /// 2. 新表示法非 None
  /// 3. 属性种类、属性标志、存储位置保持不变
  /// 4. 旧表示法可通过原地更新兼容到新表示法 (例如 None -> Tagged/Smi 或 Smi/HeapObject -> Tagged)
  pub fn try_reconfigure_to_data_field_inplace(&mut self) -> Result<bool> {
    // SAFETY: old_map 指向有效 Map 内存
    let is_deprecated = unsafe { (*self.old_map.as_ptr()).is_deprecated() };
    if is_deprecated || self.new_representation.is_none() {
      return Ok(false);
    }
    let Some(descriptor_index) = self.modified_descriptor else {
      return Ok(false);
    };

    // SAFETY: instance_descriptors 槽位有效
    let descriptors: Tagged<DescriptorArray> =
      unsafe { (*self.old_map.as_ptr()).instance_descriptors.cast() };
    let old_details = descriptors.get_details(descriptor_index)?;

    if old_details.attributes() != self.new_attributes
      || old_details.kind() != self.new_kind
      || old_details.location() != self.new_location
    {
      return Ok(false);
    }

    let old_representation = old_details.representation();
    if !old_representation.can_be_in_place_changed_to(&self.new_representation) {
      return Ok(false);
    }

    Self::generalize_field(
      self.old_map,
      descriptor_index,
      self.new_constness,
      self.new_representation,
    )?;

    Ok(true)
  }

  /// 将指定属性重配置为数据字段并返回演化后的 Map
  pub fn reconfigure_to_data_field(
    &mut self,
    descriptor: usize,
    attributes: PropertyAttributes,
    constness: PropertyConstness,
    representation: Representation,
  ) -> Result<Tagged<Map>> {
    self.modified_descriptor = Some(descriptor);
    self.new_kind = PropertyKind::Data;
    self.new_attributes = attributes;
    self.new_constness = constness;
    self.new_location = PropertyLocation::Field;
    self.new_representation = representation;

    if self.try_reconfigure_to_data_field_inplace()? {
      return Ok(self.old_map);
    }

    // 无法就地重配置时，标记原 Map 弃用 (Deprecated)
    // SAFETY: old_map 独占修改
    unsafe {
      (*self.old_map.as_mut_ptr()).set_is_deprecated(true);
    }
    Ok(self.old_map)
  }

  /// 泛化字段的常量性与表示法，并同步通知与更新过渡树各层级
  pub fn generalize_field(
    map: Tagged<Map>,
    modify_index: usize,
    new_constness: PropertyConstness,
    new_representation: Representation,
  ) -> Result<()> {
    // SAFETY: map 实例描述符有效
    let descriptors: Tagged<DescriptorArray> =
      unsafe { (*map.as_ptr()).instance_descriptors.cast() };
    let old_details = descriptors.get_details(modify_index)?;
    let old_constness = old_details.constness();
    let old_representation = old_details.representation();

    // 若当前描述符已足够通用，直接返回
    if is_generalizable_to(new_constness, old_constness)
      && old_representation.equals(&new_representation)
    {
      return Ok(());
    }

    let field_owner = Self::find_field_owner(map, modify_index);
    // SAFETY: field_owner 指针有效
    let owner_descriptors: Tagged<DescriptorArray> =
      unsafe { (*field_owner.as_ptr()).instance_descriptors.cast() };
    let owner_details = owner_descriptors.get_details(modify_index)?;

    let combined_constness = generalize_constness(owner_details.constness(), new_constness);
    let combined_representation = owner_details
      .representation()
      .generalize(new_representation);

    let updated_details = owner_details
      .copy_with_constness(combined_constness)
      .copy_with_representation(combined_representation);
    owner_descriptors.set_details(modify_index, updated_details)?;

    // 向过渡子树的所有子孙 Map 广播泛化
    TransitionsAccessor::traverse_transition_tree(field_owner, |current| {
      // SAFETY: current 指向有效 Map
      let own_desc_count = unsafe { (*current.as_ptr()).number_of_own_descriptors() as usize };
      if own_desc_count > modify_index {
        let desc_ptr = unsafe { (*current.as_ptr()).instance_descriptors };
        if !desc_ptr.is_null() {
          let current_descriptors: Tagged<DescriptorArray> = desc_ptr.cast();
          if let Ok(cur_details) = current_descriptors.get_details(modify_index) {
            let cur_combined_constness =
              generalize_constness(cur_details.constness(), combined_constness);
            let cur_combined_rep = cur_details
              .representation()
              .generalize(combined_representation);
            let new_cur_details = cur_details
              .copy_with_constness(cur_combined_constness)
              .copy_with_representation(cur_combined_rep);
            let _ = current_descriptors.set_details(modify_index, new_cur_details);
          }
        }
      }
    });

    Ok(())
  }

  /// 重配置 elements kind
  pub fn reconfigure_elements_kind(&mut self, new_elements_kind: u8) -> Result<Tagged<Map>> {
    self.new_elements_kind = new_elements_kind;
    // SAFETY: old_map 独占修改
    unsafe {
      (*self.old_map.as_mut_ptr()).set_elements_kind(new_elements_kind);
    }
    Ok(self.old_map)
  }

  /// 计算整棵过渡树中的最小对象内剩余 slack 槽位数
  pub fn compute_min_object_slack(initial_map: Tagged<Map>) -> i32 {
    // SAFETY: initial_map 指针有效
    let mut min_slack = unsafe { (*initial_map.as_ptr()).unused_property_fields() };
    TransitionsAccessor::traverse_transition_tree(initial_map, |m| {
      // SAFETY: m 指针有效
      let unused = unsafe { (*m.as_ptr()).unused_property_fields() };
      if unused < min_slack {
        min_slack = unused;
      }
    });
    min_slack.max(0)
  }

  /// 结束初始 Map 及其过渡树的 Inobject Slack Tracking
  ///
  /// 根据构造计数期测算出的最小未用属性槽数（slack），折减所有相关 Map 的实例分配大小，
  /// 并将构造计数器置为 NO_SLACK_TRACKING。
  pub fn complete_inobject_slack_tracking(initial_map: Tagged<Map>) -> Result<()> {
    let slack = Self::compute_min_object_slack(initial_map);
    TransitionsAccessor::traverse_transition_tree(initial_map, |m| {
      // SAFETY: m 拥有独占写入权限
      unsafe {
        if slack > 0 {
          let new_size = (*m.as_ptr()).instance_size_from_slack(slack);
          if new_size > 0 {
            (*m.as_mut_ptr()).set_instance_size(new_size as usize);
            let cur_unused = (*m.as_ptr()).unused_property_fields();
            (*m.as_mut_ptr()).set_in_object_unused_property_fields((cur_unused - slack).max(0));
          }
        }
        (*m.as_mut_ptr()).set_construction_counter(NO_SLACK_TRACKING);
      }
    });
    Ok(())
  }
}
