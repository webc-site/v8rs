//! v8/src/wasm/leb-helper.h 的 1:1 转写
//! 登记差异:
//! - 原版 write_* 以 uint8_t** 前进指针且无安全检查 → Rust 定长缓冲区
//!   (`&mut [u8; N]`) + 返回写入长度: 类型系统保证容量, 零分配零越界。
//! - read_u32v 无安全检查 → Option (源耗尽/超长 → None, 禁 panic)。
// 由 tool/src/wasm/leb-helper.h.js 从 v8/src/wasm/leb-helper.h 生成; 修订就地改, 复跑覆盖
// 往返自检通过后落盘
/// kPaddedVarInt32Size
pub const PADDED_VAR_INT32_SIZE: usize = 5;
/// kMaxVarInt32Size
pub const MAX_VAR_INT32_SIZE: usize = 5;
/// kMaxVarInt64Size
pub const MAX_VAR_INT64_SIZE: usize = 10;

/// u32 LEB128 最大编码长度
pub const MAX_U32V_SIZE: usize = MAX_VAR_INT32_SIZE;
/// i32 LEB128 最大编码长度
pub const MAX_I32V_SIZE: usize = MAX_VAR_INT32_SIZE;
/// u64/i64 LEB128 最大编码长度
pub const MAX_U64V_SIZE: usize = MAX_VAR_INT64_SIZE;
pub const MAX_I64V_SIZE: usize = MAX_VAR_INT64_SIZE;

/// LEB128 编解码器 (原版 LEBHelper 静态类)
pub struct LebHelper;

impl LebHelper {
  /// write_u32v: 无符号 32 位 LEB → dest, 返回写入字节数
  pub fn write_u32v(dest: &mut [u8; MAX_VAR_INT32_SIZE], val: u32) -> usize {
    let mut val = val;
    let mut n = 0;
    while val >= 0x80 {
      dest[n] = 0x80 | (val & 0x7f) as u8;
      n += 1;
      val >>= 7;
    }
    dest[n] = (val & 0x7f) as u8;
    n + 1
  }

  /// write_i32v: 有符号 32 位 LEB → dest
  pub fn write_i32v(dest: &mut [u8; MAX_VAR_INT32_SIZE], val: i32) -> usize {
    let mut val = val;
    let mut n = 0;
    if val >= 0 {
      while val >= 0x40 {
        // 防符号扩展
        dest[n] = 0x80 | (val & 0x7f) as u8;
        n += 1;
        val >>= 7;
      }
      dest[n] = (val & 0xff) as u8;
    } else {
      while (val >> 6) != -1 {
        dest[n] = 0x80 | (val & 0x7f) as u8;
        n += 1;
        val >>= 7;
      }
      dest[n] = (val & 0x7f) as u8;
    }
    n + 1
  }

  /// write_u64v: 无符号 64 位 LEB → dest
  pub fn write_u64v(dest: &mut [u8; MAX_VAR_INT64_SIZE], val: u64) -> usize {
    let mut val = val;
    let mut n = 0;
    while val >= 0x80 {
      dest[n] = 0x80 | (val & 0x7f) as u8;
      n += 1;
      val >>= 7;
    }
    dest[n] = (val & 0x7f) as u8;
    n + 1
  }

  /// write_i64v: 有符号 64 位 LEB → dest
  pub fn write_i64v(dest: &mut [u8; MAX_VAR_INT64_SIZE], val: i64) -> usize {
    let mut val = val;
    let mut n = 0;
    if val >= 0 {
      while val >= 0x40 {
        dest[n] = 0x80 | (val & 0x7f) as u8;
        n += 1;
        val >>= 7;
      }
      dest[n] = (val & 0xff) as u8;
    } else {
      while (val >> 6) != -1 {
        dest[n] = 0x80 | (val & 0x7f) as u8;
        n += 1;
        val >>= 7;
      }
      dest[n] = (val & 0x7f) as u8;
    }
    n + 1
  }

  /// read_u32v: 无符号 32 位 LEB; 源耗尽或超长 (>5 字节) → None
  /// (原版无检查, 禁 panic 改 Option)
  pub fn read_u32v(src: &[u8]) -> Option<u32> {
    let mut shift = 0;
    let mut i = 0;
    let mut byte = *src.first()?;
    i += 1;
    let mut result = u32::from(byte & 0x7f);
    while byte & 0x80 != 0 {
      byte = *src.get(i)?;
      i += 1;
      shift += 7;
      if shift > 28 {
        // 超长 LEB (u32 至多 5 字节, 第 6 字节 shift=35 越过位宽):
        // 原版 DCHECK_LE 后为 UB; 禁 panic → 收敛为 None
        return None;
      }
      result |= u32::from(byte & 0x7f) << shift;
    }
    Some(result)
  }

  /// sizeof_u32v: val 的 LEB32 长度
  pub const fn sizeof_u32v(mut val: u32) -> usize {
    let mut size = 0;
    loop {
      size += 1;
      val >>= 7;
      if val == 0 {
        return size;
      }
    }
  }

  /// sizeof_i32v
  pub const fn sizeof_i32v(mut val: i32) -> usize {
    let mut size = 1;
    if val >= 0 {
      while val >= 0x40 {
        size += 1;
        val >>= 7;
      }
    } else {
      while (val >> 6) != -1 {
        size += 1;
        val >>= 7;
      }
    }
    size
  }

  /// sizeof_u64v
  pub const fn sizeof_u64v(mut val: u64) -> usize {
    let mut size = 0;
    loop {
      size += 1;
      val >>= 7;
      if val == 0 {
        return size;
      }
    }
  }
}
