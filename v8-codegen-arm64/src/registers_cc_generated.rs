// 由 tool/src/codegen/arm64/register-arm64.cc.js 从 v8/src/codegen/arm64/register-arm64.cc 生成; 修订就地改, 复跑覆盖
// VectorFormat 派生映射表

// 注: VectorFormat 枚举由 utils-arm64 覆盖面 (utils_generated) 提供;
// 尺寸常量由 constants-arm64 覆盖面 (constants_generated) 提供。
// 表中 default/UNREACHABLE 分支以 None 登记由调用方裁定。

use crate::{
  constants_generated::{
    BREG_SIZE_IN_BITS, DREG_SIZE_IN_BITS, HREG_SIZE_IN_BITS, SREG_SIZE_IN_BITS,
  },
  utils_generated::VectorFormat,
};

/// 原 VectorFormatHalfWidth 的 switch 表展开
#[rustfmt::skip]
pub fn halfWidth_table(vform: VectorFormat) -> Option<VectorFormat> {
  use crate::utils_generated::VectorFormat as VF;
  match vform {
    VF::kFormat8H => Some(VF::Format8B),
    VF::kFormat4S => Some(VF::Format4H),
    VF::kFormat2D => Some(VF::Format2S),
    VF::kFormat1Q => Some(VF::Format1D),
    VF::kFormatH => Some(VF::FormatB),
    VF::kFormatS => Some(VF::FormatH),
    VF::kFormatD => Some(VF::FormatS),
    _ => None,
  }
}

/// 原 VectorFormatDoubleWidth 的 switch 表展开
#[rustfmt::skip]
pub fn doubleWidth_table(vform: VectorFormat) -> Option<VectorFormat> {
  use crate::utils_generated::VectorFormat as VF;
  match vform {
    VF::kFormat8B => Some(VF::Format8H),
    VF::kFormat4H => Some(VF::Format4S),
    VF::kFormat2S => Some(VF::Format2D),
    VF::kFormatB => Some(VF::FormatH),
    VF::kFormatH => Some(VF::FormatS),
    VF::kFormatS => Some(VF::FormatD),
    _ => None,
  }
}

/// 原 VectorFormatFillQ 的 switch 表展开
#[rustfmt::skip]
pub fn fillQ_table(vform: VectorFormat) -> Option<VectorFormat> {
  use crate::utils_generated::VectorFormat as VF;
  match vform {
    VF::kFormatB | VF::kFormat8B | VF::kFormat16B => Some(VF::Format16B),
    VF::kFormatH | VF::kFormat4H | VF::kFormat8H => Some(VF::Format8H),
    VF::kFormatS | VF::kFormat2S | VF::kFormat4S => Some(VF::Format4S),
    VF::kFormatD | VF::kFormat1D | VF::kFormat2D => Some(VF::Format2D),
    _ => None,
  }
}

/// 原 LaneSizeInBitsFromFormat 的 switch 表展开
#[rustfmt::skip]
pub fn laneSizeInBitsFromFormat_table(vform: VectorFormat) -> Option<u32> {
  use crate::utils_generated::VectorFormat as VF;
  match vform {
    VF::kFormatB | VF::kFormat8B | VF::kFormat16B => Some(8),
    VF::kFormatH | VF::kFormat4H | VF::kFormat8H => Some(16),
    VF::kFormatS | VF::kFormat2S | VF::kFormat4S => Some(32),
    VF::kFormatD | VF::kFormat1D | VF::kFormat2D => Some(64),
    VF::kFormat1Q => Some(128),
    _ => None,
  }
}

/// 原 LaneCountFromFormat 的 switch 表展开
#[rustfmt::skip]
pub fn laneCountFromFormat_table(vform: VectorFormat) -> Option<i32> {
  use crate::utils_generated::VectorFormat as VF;
  match vform {
    VF::kFormat16B => Some(16),
    VF::kFormat8B | VF::kFormat8H => Some(8),
    VF::kFormat4H | VF::kFormat4S => Some(4),
    VF::kFormat2S | VF::kFormat2D => Some(2),
    VF::kFormat1D | VF::kFormat1Q | VF::kFormatB | VF::kFormatH | VF::kFormatS | VF::kFormatD => Some(1),
    _ => None,
  }
}

/// 原 RegisterSizeInBitsFromFormat 的 switch 表展开
#[rustfmt::skip]
pub fn registerSizeInBitsFromFormat_table(vform: VectorFormat) -> Option<u32> {
  use crate::utils_generated::VectorFormat as VF;
  match vform {
    VF::kFormatB => Some(BREG_SIZE_IN_BITS),
    VF::kFormatH => Some(HREG_SIZE_IN_BITS),
    VF::kFormatS => Some(SREG_SIZE_IN_BITS),
    VF::kFormatD => Some(DREG_SIZE_IN_BITS),
    VF::kFormat8B | VF::kFormat4H | VF::kFormat2S | VF::kFormat1D => Some(DREG_SIZE_IN_BITS),
    _ => None,
  }
}

// 其余派生函数 (RegisterSizeInBytesFromFormat / LaneSizeInBytesFromFormat /
// LaneSizeInBytesLog2FromFormat / MaxLaneCountFromFormat / IsVectorFormat /
// MaxIntFromFormat / MinIntFromFormat / AreSameLaneSize) 可由上表组合, 语义面登记跳过。

// —— 提取不变式断言 ——
const _: () = {
  assert!(lane_size_in_bits_from_format_table(VectorFormat::kFormat8B) == Some(8));
  assert!(lane_count_from_format_table(VectorFormat::kFormat16B) == Some(16));
};
