#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod error;
pub use error::{Error, Result};

pub mod frame_constants;
pub use frame_constants::{
  SYSTEM_POINTER_SIZE, SYSTEM_POINTER_SIZE_LOG2, common_frame_constants, standard_frame_constants,
  typed_frame_constants,
};

pub mod frame_constants_arm64;
pub mod frame_constants_x64;

pub mod frames;
pub use frames::{
  BuiltinContinuationMode, FrameInfoKind, JsFrameMarker, StackFrame, StackFrameType,
};
