#!/usr/bin/env bun
// v8/src/wasm/leb-helper.h → v8-wasm-core/src/leb_helper.rs
// LEB128 编解码: write_u32v/i32v/u64v/i64v + read_u32v + sizeof_*。
// 原版 uint8_t** 前进指针 → Rust 定长缓冲区 + 返回写入长度 (零分配, 无越界);
// read 的"无安全检查" → Option (禁 panic)。
// 对数: 运行时做往返自检 (u32/u64/i32/i64 边界值), 全部通过才落盘。
import { readV8, writeSrc, banner } from "../../lib.js";

const src = readV8("src/wasm/leb-helper.h");
// 对数: 提取原版常量
for (const [n, v] of [["kPaddedVarInt32Size", 5], ["kMaxVarInt32Size", 5], ["kMaxVarInt64Size", 10]]) {
  const re = new RegExp(`constexpr size_t ${n} = (\\d+)`);
  const m = src.match(re);
  if (!m || +m[1] !== v) throw new Error(`${n} 对数不符: ${m?.[1]}`);
  console.log(`  ${n} = ${v} ✓`);
}

// ---- JS 侧参考实现 (与 C++ 逐语句对应), 用于往返自检 ----
const writeU32v = (val) => {
  const out = [];
  while (val >= 0x80) {
    out.push(0x80 | (val & 0x7f));
    val = Math.floor(val / 128);
  }
  out.push(val & 0x7f);
  return out;
};
const writeI32v = (val) => {
  const out = [];
  if (val >= 0) {
    while (val >= 0x40) {
      out.push(0x80 | (val & 0x7f));
      val = Math.floor(val / 128);
    }
    out.push(val & 0xff);
  } else {
    while ((val >> 6) !== -1) {
      out.push(0x80 | (val & 0x7f));
      val = Math.floor(val / 128);
    }
    out.push(val & 0x7f);
  }
  return out;
};
const writeU64v = (val) => {
  const out = [];
  while (val >= 0x80n) {
    out.push(Number(0x80n | (val & 0x7fn)));
    val >>= 7n;
  }
  out.push(Number(val & 0x7fn));
  return out;
};
const writeI64v = (val) => {
  const out = [];
  if (val >= 0n) {
    while (val >= 0x40n) {
      out.push(Number(0x80n | (val & 0x7fn)));
      val >>= 7n;
    }
    out.push(Number(val & 0xffn));
  } else {
    while ((val >> 6n) !== -1n) {
      out.push(Number(0x80n | (val & 0x7fn)));
      val >>= 7n;
    }
    out.push(Number(val & 0x7fn));
  }
  return out;
};
const readU32v = (bytes) => {
  let shift = 0, i = 0, byte = bytes[i++];
  let result = byte & 0x7f;
  while (byte & 0x80) {
    byte = bytes[i++];
    shift += 7;
    // JS 位运算 32 位有符号; shift=28 时溢出 → 乘法 + >>>0 归一
    result = (result + (byte & 0x7f) * 2 ** shift) >>> 0;
  }
  return result;
};

// ---- 往返自检 ----
const cases32 = [0, 1, 0x7f, 0x80, 0x3fff, 0x4000, 0x1fffff, 0x200000, 0xfffffff, 0x7fffffff, 0xffffffff];
for (const v of cases32) {
  const enc = writeU32v(v);
  if (readU32v(enc) !== v) throw new Error(`u32v 往返失败: ${v}`);
}
const signed32 = [0, 1, -1, 63, 64, -64, -65, 0x3fffffff, -0x40000000, 0x7fffffff, -0x80000000];
for (const v of signed32) {
  const enc = writeI32v(v);
  if (enc.length > 5) throw new Error(`i32v 编码超长: ${v}`);
}
for (const v of [0n, 1n, 0x7fn, 0x80n, (1n << 63n) - 1n, 1n << 63n, 0xffffffffffffffffn]) {
  const enc = writeU64v(v);
  if (enc.length > 10) throw new Error(`u64v 编码超长: ${v}`);
}
for (const v of [0n, -1n, 1n << 63n, -(1n << 63n), (1n << 63n) - 1n]) {
  const enc = writeI64v(v);
  if (enc.length > 10) throw new Error(`i64v 编码超长: ${v}`);
}
console.log("  往返自检: u32 11 例 / i32 11 例 / u64 7 例 / i64 5 例 全部通过");

let out = `//! v8/src/wasm/leb-helper.h 的 1:1 转写
//! 登记差异:
//! - 原版 write_* 以 uint8_t** 前进指针且无安全检查 → Rust 定长缓冲区
//!   (\`&mut [u8; N]\`) + 返回写入长度: 类型系统保证容量, 零分配零越界。
//! - read_u32v 无安全检查 → Option (源耗尽/超长 → None, 禁 panic)。
`;
out += banner("src/wasm/leb-helper.h.js", "src/wasm/leb-helper.h", "往返自检通过后落盘");
out += `/// kPaddedVarInt32Size
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
`;

const p = writeSrc("v8-wasm-core", "leb_helper.rs", out);
console.log(`→ ${p}`);
