#![cfg_attr(docsrs, feature(doc_cfg))]

//! v8/src/bigint 的 Rust 转写 — 模块与原版文件 1:1
//! 本轮两族并发: B1 核心与除法 / B2 乘法与字符串
//! lib.rs 由编排者预建, 两个代理都不得修改; 惯用法收敛 (pub use) 在审查环后统一处理

pub mod error;

// ==== B1 核心与除法 (owner: bigint-core) ====
pub mod bigint;
pub mod bigint_internal;
pub mod bitwise;
pub mod div_barrett;
pub mod div_burnikel;
pub mod div_helpers;
pub mod div_schoolbook;
pub mod util;
pub mod vector_arithmetic;

// ==== B2 乘法与字符串 (owner: bigint-mul-str) ====
pub mod from_string;
pub mod mul_fft;
pub mod mul_helpers;
pub mod mul_karatsuba;
pub mod mul_toom;
pub mod to_string;
