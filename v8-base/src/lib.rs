#![cfg_attr(docsrs, feature(doc_cfg))]

//! v8/src/base (除 platform/) 的 Rust 转写 — 模块与原版文件 1:1
//! 本轮以 `pub mod` 分区, 惯用法重构 (pub use 收敛) 在审查环后统一处理

mod error;
pub use error::{Error, Result};

// ==== A1 数据结构与算法层 (owner: base-collections) ====
pub mod algorithm;
pub mod atomic_utils;
pub mod bit_field;
pub mod bits;
pub mod bits_iterator;
pub mod container_utils;
pub mod contextual;
pub mod discriminated_union;
pub mod doubly_threaded_list;
pub mod enum_set;
pub mod flags;
pub mod free_deleter;
pub mod functional;
pub mod hashing;
pub mod hashmap;
pub mod hashmap_entry;
pub mod intrusive_set;
pub mod iterator;
pub mod lazy_instance;
pub mod macros;
pub mod once;
pub mod overflowing_math;
pub mod pointer_with_payload;
pub mod random_number_generator;
pub mod ring_buffer;
pub mod small_map;
pub mod small_vector;
pub mod strong_alias;
pub mod template_meta_programming;
pub mod template_utils;
pub mod threaded_list;
pub mod unique_array;
pub mod vector;

// ==== A2 内存/数值/系统层 (owner: base-sys) ====
pub mod abort_mode;
pub mod address_region;
pub mod atomicops;
pub mod bounded_page_allocator;
pub mod bounds;
pub mod build_config;
pub mod compiler_specific;
pub mod cpu;
pub mod debug;
pub mod division_by_constant;
pub mod emulated_virtual_address_subspace;
pub mod file_utils;
pub mod float16;
pub mod fpu;
pub mod ieee754;
pub mod immediate_crash;
pub mod logging;
pub mod memcopy;
pub mod memory;
pub mod numbers;
pub mod numerics;
pub mod page_allocator;
pub mod region_allocator;
pub mod sanitizer;
pub mod string_format;
pub mod strings;
pub mod sys_info;
pub mod timezone_cache;
pub mod types;
pub mod virtual_address_space;
pub mod virtual_address_space_page_allocator;
pub mod vlq;
pub mod vlq_base64;
