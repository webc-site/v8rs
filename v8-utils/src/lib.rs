//! `v8/src/utils` 的 Rust 移植。
//!
//! 覆盖映射与跳过项理由见 README.md; 基线由 tool/src/utils/*.js 生成后就地修订。

mod error;
pub use error::{Error, Result};

// ==== 数据结构与容器族 (上游: address-map/bit-vector/detachable-vector/identity-map/locked-queue/scoped-list/sparse-bit-vector) ====
pub mod address_map;
pub mod bit_vector;
pub mod detachable_vector;
pub mod identity_map;
pub mod locked_queue;
pub mod scoped_list;
pub mod sparse_bit_vector;

// ==== 格式化/散列/工具族 (上游: allocation/boxed-float/hex-format/memcopy/ostreams/output-stream/sha-256/utils/version) ====
pub mod allocation;
pub mod boxed_float;
pub mod hex_format;
pub mod memcopy;
pub mod ostreams;
pub mod output_stream;
pub mod sha256;
pub mod utils;
pub mod version;
