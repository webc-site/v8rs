#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod error;
pub use error::{Error, Result};

pub mod messages;
pub use messages::{
  CallSiteFrame, FrameSkipMode, MessageFormatter, MessageLocation, StackTraceCollection,
  StackTraceFormatter, TEMPLATES_WITH_MISMATCHED_ARGUMENTS, ToStringMessageSource,
};
