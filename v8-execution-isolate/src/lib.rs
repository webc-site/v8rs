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
use v8_heap_core::memory_chunk::K_PAGES_IN_MAIN_CAGE;
use v8_heap_gc::is_below_activation_thresholds;
use v8_interpreter::BYTECODE_COUNT;
pub use vm_state::{EncodedCSignature, ExternalCallbackScope, StateTag, ThreadId};

/// 桥接: 校验字节码操作码是否在解释器合法范围内 (真实消费 Bytecode 派发校验)
pub fn is_valid_interpreter_bytecode(bytecode_raw: u8) -> bool {
  (bytecode_raw as usize) < BYTECODE_COUNT
}

/// 桥接: 获取堆核心主 Cage 包含的常规页数量 (真实消费 MemoryChunk 核心尺寸)
pub fn heap_core_main_cage_pages() -> usize {
  K_PAGES_IN_MAIN_CAGE
}

/// 桥接: 评估指定字节量是否处于增量 GC 激活门限以下 (真实消费字节参数判定)
pub fn is_below_gc_activation_threshold(v8_bytes: usize, global_bytes: usize) -> bool {
  is_below_activation_thresholds(v8_bytes, global_bytes, cfg!(debug_assertions))
}
