#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]

extern crate alloc;

mod error;
pub use error::{Error, Result};

pub mod isolate;
pub use isolate::{Isolate, IsolateExternalScope};

pub mod isolate_data;
pub use isolate_data::IsolateFieldId;

pub mod microtask;
pub use microtask::{MicrotaskCallback, MicrotaskQueue, MicrotasksPolicy};

pub mod protectors;
pub use protectors::{PROTECTOR_INVALID, PROTECTOR_VALID, Protector, Protectors};

pub mod stack_guard;
pub use stack_guard::{InterruptLevel, InterruptType, StackGuard, interrupt_flags};

pub mod vm_state;
pub use vm_state::{EncodedCSignature, ExternalCallbackScope, StateTag, ThreadId};
