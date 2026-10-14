//! 原版 v8/src/base/numerics — 数值安全转换/检查运算族 (Rust 重设计, 语义保持)
//!
//! C++ 模板机制 (requires/concepts/偏特化) 在 Rust 中以 trait + 泛型函数表达:
//! - checked_cast 对应 Dst::try_from + 越界即 crash (原版 CHECK 语义);
//! - saturated_cast 对 float→int 与 `as` 饱和转换一致 (NaN→0), int↔int 用 clamp;
//! - StrictNumeric 为纯编译期区间检查, Rust 静态类型天然覆盖, 不再迁移.

pub mod angle_conversions;
pub mod byte_conversions;
pub mod checked_math;
pub mod clamped_math;
pub mod math_constants;
pub mod ranges;
pub mod safe_conversions;
pub mod safe_math;
pub mod wrapping_math;
