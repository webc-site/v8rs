#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
pub use error::{Error, Result};

pub mod allocation;
pub mod allocation_observer;
pub mod free_list;
pub mod large_spaces;
pub mod linear_allocation_area;
pub mod main_allocator;
pub mod memory_allocator;
pub mod memory_balancer;
pub mod memory_chunk;
pub mod memory_pool;
pub mod slot_set;
pub mod spaces;
