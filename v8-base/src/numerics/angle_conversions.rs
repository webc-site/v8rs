//! 原版 v8/src/base/numerics/angle_conversions.h — 角度/弧度转换.
//! C++ `std::numbers::pi_v<T>` 对应 Rust `core::f{32,64}::consts::PI`.

use core::{f32::consts::PI as PI_F32, f64::consts::PI as PI_F64};

/// 角度/弧度转换方法 (原版 `DegToRad`/`RadToDeg` 模板).
pub trait AngleConversions {
  fn deg_to_rad(deg: Self) -> Self;
  fn rad_to_deg(rad: Self) -> Self;
}

macro_rules! impl_angle {
  ($t:ty, $pi:path) => {
    impl AngleConversions for $t {
      fn deg_to_rad(deg: Self) -> Self {
        deg * $pi / 180.0
      }
      fn rad_to_deg(rad: Self) -> Self {
        rad * 180.0 / $pi
      }
    }
  };
}

impl_angle!(f32, PI_F32);
impl_angle!(f64, PI_F64);

/// 原版 `v8::base::DegToRad(T deg)` — 泛型入口.
pub fn deg_to_rad<T: AngleConversions>(deg: T) -> T {
  T::deg_to_rad(deg)
}

/// 原版 `v8::base::RadToDeg(T rad)` — 泛型入口.
pub fn rad_to_deg<T: AngleConversions>(rad: T) -> T {
  T::rad_to_deg(rad)
}
