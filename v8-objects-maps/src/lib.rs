// v8-objects-maps/src/lib.rs

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod dependent_code_inl;
pub mod descriptor_array;
pub mod descriptor_array_tq;
pub mod error;
pub mod field_index;
pub mod field_index_inl;
pub mod field_type;
pub mod map_tq;
pub mod map_updater;
pub mod map_updater_cc_generated;
pub mod megadom_handler_inl;
pub mod name_ext;
pub mod property_array;
pub mod property_cell_tq;
pub mod property_descriptor;
pub mod property_descriptor_object_inl;
pub mod prototype;
pub mod prototype_info;
pub mod prototype_info_tq;
pub mod transitions;
pub mod transitions_cc_generated;

pub use descriptor_array::{
  DescriptorArray, DescriptorArrayBuf, DescriptorArrayExt, DescriptorEntry, DescriptorLookupCache,
  EnumCache, FastIterableState,
};
pub use error::{Error, Result};
pub use map_updater::MapUpdater;
pub use name_ext::NameTaggedExt;
pub use property_array::{PropertyArray, PropertyArrayBuf, PropertyArrayExt};
pub use transitions::{
  Encoding, MemoryAllocator, SideStepTransitionKind, TransitionArray, TransitionArrayBuf,
  TransitionArrayExt, TransitionsAccessor,
};
