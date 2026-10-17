#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[path = "arm64/deoptimizer_arm64.rs"]
pub mod arm64;
pub mod deoptimized_frame_info;
pub mod deoptimizer_generated;
pub mod error;
pub mod materialized_object_store;
pub mod translated_state_generated;
pub mod translation_opcode_generated;
#[path = "x64/deoptimizer_x64.rs"]
pub mod x64;

pub use deoptimized_frame_info::DeoptimizedFrameInfo;
pub use deoptimizer_generated::*;
pub use error::{Error, Result};
pub use materialized_object_store::MaterializedObjectStore;
pub use translated_state_generated::*;
pub use translation_opcode_generated::*;
