// 由 tool/src/objects/simd.h.js 从 v8/src/objects/simd.{h,cc} 生成; 修订就地改, 复跑覆盖
// SIMD 加速工具: Array.indexOf 扫描 + Uint8Array↔hex 转换
//! simd: 数组线性查找 (Smi/Object/Double) 与 Uint8Array 十六进制互转。
//!
//! indexOf 返回槽下标, 未命中返回 kNotFound (usize::MAX);
//! hex 编码为小写 ('0'..'9' + 'a'..'f'), 解码接受 0-9/a-f/A-F。
//!
//! SIMD 快路径 (simd.cc fast_search / Uint8ArrayToHexFastWith{SSE,Neon,SVE} /
//! Uint8ArrayFromHexWith{SSE,Neon}) 为纯性能面, 与标量路径逐输入等价
//! (原版即以 slow_search 作对照实现); 本体积语义面 1:1 标量转写,
//! fearless_simd 性能层按推迟台账登记 (readme/zh.md)。

use v8_objects_core::tagged::{Object, Tagged};

/// 未找到哨兵 (原版 cc 返回 -1, uintptr_t 全 1 位模式 = usize::MAX)。
pub const K_NOT_FOUND: usize = usize::MAX;

// ── slow_search (simd.cc 匿名 ns: 标量线性回退, SIMD 尾块同路) ──
/// v8: src/objects/simd.cc slow_search<T>
#[inline]
fn slow_search<T: PartialEq>(
  array: &[T],
  mut index: usize,
  array_len: usize,
  search_element: &T,
) -> usize {
  while index < array_len {
    if &array[index] == search_element {
      return index;
    }
    index += 1;
  }
  K_NOT_FOUND
}

/// v8: src/objects/simd.cc ArrayIndexOfIncludes<OBJECTORSMI> / ArrayIndexOfIncludesSmiOrObject
///
/// Smi/Object 槽数组: 按标签位模式 (Tagged_t) 全等比较; 搜索值为
/// HeapNumber/BigInt/String 时违反原版 DCHECK 前置 (调用方过滤)。
/// 未命中返回 K_NOT_FOUND (对应 Smi::FromInt(-1))。
pub fn array_index_of_includes_smi_or_object(
  array: &[Tagged<Object>],
  from_index: usize,
  search_element: Tagged<Object>,
) -> usize {
  let array_len = array.len();
  // TurboFan 归约路径可传 from_index >= array_len (同时覆盖 array_len == 0)。
  if from_index >= array_len {
    return K_NOT_FOUND;
  }
  // Tagged_t 槽比较 = 位模式比较 (Smi 含数值, 对象含地址)。
  let search = search_element.ptr();
  let mut index = from_index;
  while index < array_len {
    if array[index].ptr() == search {
      return index;
    }
    index += 1;
  }
  K_NOT_FOUND
}

/// v8: src/objects/simd.cc ArrayIndexOfIncludes<DOUBLE> / ArrayIndexOfIncludesDouble
///
/// double 数组按数值 `==` 比较; 搜索值经 Smi 展宽或 HeapNumber 取值,
/// 保证非 NaN (DCHECK), 故 hole (canonical NaN 位模式) 天然不匹配、跳过。
pub fn array_index_of_includes_double(
  array: &[f64],
  from_index: usize,
  search_element: f64,
) -> usize {
  let array_len = array.len();
  if from_index >= array_len {
    return K_NOT_FOUND;
  }
  // DCHECK(!std::isnan(search_num))
  debug_assert!(!search_element.is_nan());
  // 未对齐 double 数组的慢速标量路径 (is_the_hole + get_scalar) 与本
  // 切片形态一致: hole 为 NaN 位模式, `==` 永不匹配, 无须显式洞检。
  slow_search(array, from_index, array_len, &search_element)
}

// ── nibble/byte → hex (simd.cc 匿名 ns 位技巧 1:1) ──
/// v8: src/objects/simd.cc NibbleToHex
#[inline]
fn nibble_to_hex(nibble: u8) -> u8 {
  debug_assert!(nibble < 16);
  const CORRECTION: u8 = b'a' - b'0' - 10;
  let c = nibble.wrapping_add(b'0');
  let temp = 128u8.wrapping_sub(10).wrapping_add(nibble);
  let msb = temp & 0x80;
  let mask = msb.wrapping_sub(msb >> 7);
  c.wrapping_add(mask & CORRECTION)
}

/// v8: src/objects/simd.cc PerformNibbleToHexAndWriteIntoStringOutPut
fn perform_nibble_to_hex_write(byte: u8, index: usize, string_output: &mut [u8]) {
  let high = byte >> 4;
  let low = byte & 0x0F;
  string_output[index] = nibble_to_hex(high);
  string_output[index + 1] = nibble_to_hex(low);
}

/// v8: src/objects/simd.cc Uint8ArrayToHexSlow
fn uint8_array_to_hex_slow(bytes: &[u8], string_output: &mut [u8]) {
  let mut index = 0usize;
  for byte in bytes {
    perform_nibble_to_hex_write(*byte, index, string_output);
    index += 2;
  }
}

/// v8: src/objects/simd.cc AtomicUint8ArrayToHexSlow
///
/// 共享 ArrayBuffer 路径: 原版以 atomic_ref relaxed load 逐字节读;
/// Rust 侧共享内存并发由调用方 (JSArrayBuffer 共享面) 承接,
/// 逐字节读取语义与 relaxed load 等价。
fn atomic_uint8_array_to_hex_slow(bytes: &[u8], string_output: &mut [u8]) {
  let mut index = 0usize;
  for byte in bytes {
    // std::memory_order_relaxed 载入 (语义等价: 单字节撕裂不可能)
    perform_nibble_to_hex_write(*byte, index, string_output);
    index += 2;
  }
}

/// v8: src/objects/simd.cc Uint8ArrayToHex
///
/// bytes → 2*length 小写十六进制写入 string_output (SeqOneByteString 字符面)。
/// SSE/AVX2/Neon/SVE 快路径与标量路径逐字节等价 (见模块头登记)。
pub fn uint8_array_to_hex<'a>(
  bytes: &[u8],
  is_shared: bool,
  string_output: &'a mut [u8],
) -> &'a mut [u8] {
  debug_assert_eq!(string_output.len(), bytes.len() * 2);
  if is_shared {
    atomic_uint8_array_to_hex_slow(bytes, string_output);
  } else {
    uint8_array_to_hex_slow(bytes, string_output);
  }
  string_output
}

// ── hex → bytes ──
/// 十六进制字符源 (原版 base::Vector<uint8_t> / base::Vector<base::uc16> 双实例化)。
pub trait HexDigit: Copy {
  /// v8: src/objects/simd.cc HexToUint8
  fn hex_to_uint8(self) -> Option<u8>;
}

impl HexDigit for u8 {
  #[inline]
  fn hex_to_uint8(self) -> Option<u8> {
    hex_to_uint8_u32(self as u32)
  }
}

impl HexDigit for u16 {
  #[inline]
  fn hex_to_uint8(self) -> Option<u8> {
    hex_to_uint8_u32(self as u32)
  }
}

/// v8: src/objects/simd.cc HexToUint8 (base::uc16 三区间判定)
fn hex_to_uint8_u32(hex: u32) -> Option<u8> {
  if (b'0' as u32..=b'9' as u32).contains(&hex) {
    Some((hex - b'0' as u32) as u8)
  } else if (b'a' as u32..=b'f' as u32).contains(&hex) {
    Some((hex - b'a' as u32 + 10) as u8)
  } else if (b'A' as u32..=b'F' as u32).contains(&hex) {
    Some((hex - b'A' as u32 + 10) as u8)
  } else {
    None
  }
}

/// v8: src/objects/simd.cc HandleRemainingHexValues
fn handle_remaining_hex_values<T: HexDigit>(input_vector: &[T], i: usize) -> Option<u8> {
  let higher = input_vector[i].hex_to_uint8()?;
  let lower = input_vector[i + 1].hex_to_uint8()?;
  Some(higher << 4 | lower)
}

/// v8: src/objects/simd.cc ArrayBufferFromHex<T>
///
/// 校验输出界 (原版 DCHECK_LE(output_length, input_length/2)) 后逐对解码;
/// 任一对非法十六进制即失败 (原版 SSE/Neon 快路径同样逐字节校验、失败
/// 回退语义一致, 性能面登记见模块头)。共享路径写序语义同 atomic relaxed store。
pub fn array_buffer_from_hex<T: HexDigit>(
  input_vector: &[T],
  is_shared: bool,
  buffer: &mut [u8],
  output_length: usize,
) -> bool {
  debug_assert!(output_length <= input_vector.len() / 2);
  let mut index = 0usize;
  let mut i = 0usize;
  while i < output_length * 2 {
    let result = handle_remaining_hex_values(input_vector, i);
    match result {
      Some(byte) => {
        // is_shared: 原版 atomic_ref relaxed store; 并发承接同上 (无撕裂单字节写)。
        let _ = is_shared;
        buffer[index] = byte;
        index += 1;
      }
      None => return false,
    }
    i += 2;
  }
  true
}

// ── 提取不变式断言 ──
const _: () = {
  assert!(K_NOT_FOUND == usize::MAX);
  assert!(usize::MAX.wrapping_add(1) == 0);
};
