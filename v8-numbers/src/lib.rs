//! v8-numbers — 原版 v8/src/numbers (~2.9k 行) 的 Rust 转写
//!
//! | 原版文件 | 本 crate 模块 |
//! |---|---|
//! | conversions.{h,cc,-inl.h} | [conversions] (子模块分文件族) |
//! | hash-seed.{h,cc,-inl.h} | [hash_seed] |
//! | ieee754.{h,cc} | [ieee754] |
//! | integer-literal.{h,-inl.h} | [integer_literal] |
//! | math-random.{h,cc} | [math_random] |
//!
//! 语义红线 (todo 10.3): Number→String / parseFloat / parseInt 的
//! ECMAScript 格式层与解析层自写; zmij (最短十进制) / itoa (整数) /
//! v8-base strtod (正确舍入) 只作底层内核。Math.random 的 xorshift128+
//! 状态机 1:1 转写。
//! 对象系统绑定 (Isolate/Handle<String>/Tagged<Object>/BigInt) 推迟,
//! 范围见 crate README。

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod conversions;
mod error;
pub mod hash_seed;
pub mod hash_seed_cc_generated;
pub mod hash_seed_inl_generated;
pub mod ieee754;
pub mod ieee754_generated;
pub mod integer_literal;
pub mod integer_literal_inl_generated;
pub mod math_random;
pub mod math_random_generated;

pub use conversions::{double_to_int32, double_to_uint32};
pub use error::{Error, Result};
