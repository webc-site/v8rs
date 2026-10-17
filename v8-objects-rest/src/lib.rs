// v8-objects-rest/src/lib.rs

#![cfg_attr(docsrs, feature(doc_cfg))]

use v8_objects_js_core as _;

pub mod error;
pub mod feedback_cell;
pub mod feedback_vector;
pub mod property_cell;
pub mod symbol;
pub mod templates;

pub mod abstract_code;
pub mod bit_field_group;
pub mod bytecode_array;
pub mod code;
pub mod code_kind;
pub mod deoptimization_data;
pub mod deoptimization_data_inl;
pub mod instruction_stream;
pub mod torque_defined_classes;
pub mod trusted_object_tq;
pub mod turbofan_types_inl;
pub mod turboshaft_types;

pub use abstract_code::{AbstractCode, AbstractCodeExt};
pub use bit_field_group::BitFieldGroup;
pub use bytecode_array::{BytecodeArray, BytecodeArrayBuf, BytecodeArrayExt};
pub use code::{Code, CodeBuf, CodeExt, GcSafeCode, GcSafeCodeExt};
pub use code_kind::{CodeKind, CodeKindFlag, CodeKinds};
pub use deoptimization_data::DeoptimizationLiteralKind;
pub use deoptimization_data_inl::{
  DeoptimizationData, DeoptimizationDataExt, DeoptimizationLiteralArray,
  DeoptimizationLiteralArrayExt,
};
pub use error::{Error, Result};
pub use feedback_cell::{ClosureCountTransition, FeedbackCell};
pub use feedback_vector::{
  FeedbackSlot, FeedbackSlotKind, FeedbackVector, FeedbackVectorBuf, FeedbackVectorExt,
};
pub use instruction_stream::{InstructionStream, InstructionStreamExt};
pub use property_cell::{PropertyCell, PropertyCellType};
pub use symbol::{PrivateSymbolKind, Symbol, WellKnownSymbolId};
pub use templates::{
  DictionaryTemplateInfo, FunctionTemplateInfo, FunctionTemplateRareData, ObjectTemplateInfo,
  TemplateInfo, TemplateInfoWithProperties,
};
