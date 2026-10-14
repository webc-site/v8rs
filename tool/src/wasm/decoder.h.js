#!/usr/bin/env bun
// v8/src/wasm/decoder.h → v8-wasm-core/src/decoder.rs (+ src/error.rs 的 WasmError)
// Decoder 流式读取基类: read_u8/u32/u64, LEB 族 (u32v/i32v/u64v/i64v/i33v),
// consume 族 (pc 前进), 错误累积 (只报首错)。
// 对数: 生成前自检 LEB 语义常量 (kMaxLength/extra bits 界), 运行后由 Rust 测试覆盖。
// 登记差异 (见文件头注释):
// - 模板 ValidationTag → trait ValidationTag + NoValidation/FullValidation 零尺寸类型
// - 裸指针 start/pc/end → &[u8] + usize 偏移
// - ITracer 依赖 WasmModule/FunctionSig → 推迟 (module-decoder 梯队)
// - TRACE 宏 (v8_flags.trace_wasm_decoder) → log::trace!
// - read_prefixed_opcode 依赖 WasmOpcode → WasmOpcode = u32 别名 + K_EXPR_UNREACHABLE
//   (wasm-opcodes.h 全量表推迟)
import { readV8, writeSrc, banner } from "../../lib.js";

// 对数: LEB 尺寸界 (与 C++ constexpr 推导一致)
const lebChecks = [
  ["u32", 32, false, 5, 4, 4], ["i32", 32, true, 5, 4, 3],
  ["u64", 64, false, 10, 1, 1], ["i64", 64, true, 10, 1, 0], ["i33", 33, true, 5, 5, 4],
];
for (const [name, bits, signed, maxLen, extra, signExt] of lebChecks) {
  const ml = Math.floor((bits + 6) / 7);
  const eb = bits - (ml - 1) * 7;
  const sx = eb - (signed ? 1 : 0);
  if (ml !== maxLen || eb !== extra || sx !== signExt) {
    throw new Error(`${name} LEB 界不符: ${ml}/${eb}/${sx} != ${maxLen}/${extra}/${signExt}`);
  }
  console.log(`  ${name}: maxLen=${ml} extraBits=${eb} signExtBits=${sx} ✓`);
}

// ---- src/error.rs ----
let err = `//! v8-wasm-core 错误集中定义
//! WasmError 对应 v8/src/wasm/wasm-result.h 的 WasmError (offset + message);
//! Result<T> 对应 wasm-result.h 的 Result<T> (T | WasmError)。
//! 登记差异: wasm-result.h 全量转写推迟 (Result 模板族由 std::result 覆盖)。
// 由 tool/src/wasm/decoder.h.js 生成 (wasm-result.h 的最小子集); 修订就地改, 复跑覆盖
use std::result;
use thiserror::Error;

/// wasm 解码错误 (原版 WasmError: offset + message)
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{message} (at offset {offset})")]
pub struct WasmError {
  /// 二进制内字节偏移
  pub offset: u32,
  /// 错误消息
  pub message: String,
}

impl WasmError {
  /// kNoErrorOffset = kMaxUInt32
  pub const NO_ERROR_OFFSET: u32 = u32::MAX;

  /// WasmError(offset, message)
  pub fn new(offset: u32, message: impl Into<String>) -> Self {
    debug_assert!(offset != Self::NO_ERROR_OFFSET);
    let message = message.into();
    debug_assert!(!message.is_empty());
    Self { offset, message }
  }

  /// has_error 语义由 Option 形态承载; offset()
  #[inline]
  pub const fn offset(&self) -> u32 {
    self.offset
  }

  /// message()
  #[inline]
  pub fn message(&self) -> &str {
    &self.message
  }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  /// 解码错误 (含偏移与消息)
  #[error(transparent)]
  Wasm(#[from] WasmError),
}

pub type Result<T> = result::Result<T, Error>;
`;

writeSrc("v8-wasm-core", "error.rs", err);
console.log("→ v8-wasm-core/src/error.rs");

// ---- src/decoder.rs ----
let out = `//! v8/src/wasm/decoder.h 的 1:1 转写
//! Decoder: 字节流读取基类 (后续 decoder 族的地基)。
//! 登记差异:
//! - 模板 ValidationTag → trait + NoValidation/FullValidation 零尺寸类型;
//!   NoValidation 下原版不检查 (DCHECK 即 UB 风险) → Rust 统一返回 Result (禁 panic),
//!   仅错误累积行为不同 (Full 记录首错并触发 onFirstError, No 直接上抛)。
//! - 裸指针 start_/pc_/end_ → \`&'a [u8]\` + usize 偏移 pc (语义等价, 借用安全)。
//! - C++ read_*/consume_* 返回值 + error_ 成员累积 → Rust Result + 内部首错记录
//!   (失败时错误同时进 Result 与 error_, 调用方可任选一种消费形态)。
//! - consume_little_endian 失败时 pc_ = end_ 的语义保留。
//! - ITracer 依赖 WasmModule/FunctionSig (未转写) → 推迟至 module-decoder 梯队。
//! - TRACE/TRACE_IF (v8_flags.trace_wasm_decoder) → log::trace!。
//! - read_prefixed_opcode 的 WasmOpcode (wasm-opcodes.h) → u32 别名 + 少量常量,
//!   opcode 全量表推迟登记。
//! - set_end/Reset 指针族 → reset (切片整体替换); set_end 推迟 (streaming 梯队)。
`;
out += banner("src/wasm/decoder.h.js", "src/wasm/decoder.h", "LEB 界自检通过");
out += `use crate::error::{Error, Result, WasmError};

/// wasm 操作码 (原版 enum WasmOpcode : uint32_t, wasm-opcodes.h; 全量表推迟)
pub type WasmOpcode = u32;

/// kExprUnreachable = 0 (read_prefixed_opcode 校验失败返回值依赖)
pub const K_EXPR_UNREACHABLE: WasmOpcode = 0;
const _: () = assert!(K_EXPR_UNREACHABLE == 0);

/// 校验标签 (原版模板 ValidationTag)
pub trait ValidationTag {
  /// 是否运行完整校验
  const VALIDATE: bool;
}

/// kNoValidation: 不校验, 假定输入有效
pub struct NoValidation;
/// kFullValidation: 完整校验, 记录错误位置与消息
pub struct FullValidation;

impl ValidationTag for NoValidation {
  const VALIDATE: bool = false;
}
impl ValidationTag for FullValidation {
  const VALIDATE: bool = true;
}

/// LEB 整型契约 (read_leb 的 IntType 模板参数)
pub trait LebInt: Copy {
  /// 是否有符号
  const SIGNED: bool;
  /// 无符号中间累积类型
  type Accum: Copy
    + core::ops::Shl<u32, Output = Self::Accum>
    + core::ops::Shr<u32, Output = Self::Accum>
    + core::ops::BitOr<Output = Self::Accum>
    + From<u8>;
  /// Accum 位宽
  const ACCUM_BITS: u32;
  /// Accum → Self
  fn from_accum(a: Self::Accum) -> Self;
  /// 经有符号类型做算术右移的符号扩展 (C++ IntType >> 语义)
  fn sign_extend(a: Self::Accum, shift: u32) -> Self::Accum;
}

impl LebInt for u32 {
  const SIGNED: bool = false;
  type Accum = u32;
  const ACCUM_BITS: u32 = 32;
  #[inline]
  fn from_accum(a: u32) -> Self {
    a
  }
  #[inline]
  fn sign_extend(a: u32, shift: u32) -> u32 {
    (((a << shift) as i32) >> shift) as u32
  }
}

impl LebInt for i32 {
  const SIGNED: bool = true;
  type Accum = u32;
  const ACCUM_BITS: u32 = 32;
  #[inline]
  fn from_accum(a: u32) -> Self {
    a as i32
  }
  #[inline]
  fn sign_extend(a: u32, shift: u32) -> u32 {
    (((a << shift) as i32) >> shift) as u32
  }
}

impl LebInt for u64 {
  const SIGNED: bool = false;
  type Accum = u64;
  const ACCUM_BITS: u32 = 64;
  #[inline]
  fn from_accum(a: u64) -> Self {
    a
  }
  #[inline]
  fn sign_extend(a: u64, shift: u32) -> u64 {
    (((a << shift) as i64) >> shift) as u64
  }
}

impl LebInt for i64 {
  const SIGNED: bool = true;
  type Accum = u64;
  const ACCUM_BITS: u32 = 64;
  #[inline]
  fn from_accum(a: u64) -> Self {
    a as i64
  }
  #[inline]
  fn sign_extend(a: u64, shift: u32) -> u64 {
    (((a << shift) as i64) >> shift) as u64
  }
}

/// 字节流解码器 (原版 Decoder; virtual 继承点由组合替代)
pub struct Decoder<'a> {
  /// start_ (缓冲区起点)
  start: &'a [u8],
  /// pc_ 相对 start 的偏移
  pc: usize,
  /// buffer_offset_: 当前缓冲区在模块内的偏移 (流式解码用)
  buffer_offset: u32,
  /// error_: 只记录首个错误 (原版 verrorf 语义)
  error: Option<WasmError>,
}

impl<'a> Decoder<'a> {
  /// Decoder(start, end) / Decoder(bytes)
  pub fn new(bytes: &'a [u8]) -> Self {
    Self::with_buffer_offset(bytes, 0)
  }

  /// Decoder(start, end, buffer_offset)
  pub fn with_buffer_offset(bytes: &'a [u8], buffer_offset: u32) -> Self {
    Self { start: bytes, pc: 0, buffer_offset, error: None }
  }

  /// available_bytes(): pc 与 end 间剩余字节数
  #[inline]
  pub const fn available_bytes(&self) -> u32 {
    (self.start.len() - self.pc) as u32
  }

  /// checkAvailable(size): 至少 {size} 字节可用; 不足记录错误
  pub fn check_available(&mut self, size: u32) -> bool {
    if size > self.available_bytes() {
      self.errorf(self.pc_offset(), format!("expected {size} bytes, fell off end"));
      return false;
    }
    true
  }

  // --- 错误报告 (只报首错; 原版 verrorf) ---
  /// error(msg): 在当前 pc 记录首错 (原版 error(msg) 重载; Rust 侧避 getter 冲突更名)
  pub fn report(&mut self, msg: impl Into<String>) {
    self.errorf(self.pc_offset(), msg);
  }

  /// error(pc, msg)
  pub fn error_at_pc(&mut self, pc: usize, msg: impl Into<String>) {
    self.errorf(self.pc_offset_at(pc), msg);
  }

  /// error(offset, msg)
  pub fn errorf(&mut self, offset: u32, msg: impl Into<String>) {
    // 只报告第一个错误
    if self.failed() {
      return;
    }
    self.error = Some(WasmError::new(offset, msg));
    self.on_first_error();
  }

  /// onFirstError(): 首错触发点 (原版 virtual; 子类组合点)
  fn on_first_error(&mut self) {}

  /// ok()
  #[inline]
  pub const fn ok(&self) -> bool {
    !self.failed()
  }
  /// failed(): 已有错误
  #[inline]
  pub const fn failed(&self) -> bool {
    self.error.is_some()
  }
  /// more(): pc < end
  #[inline]
  pub const fn more(&self) -> bool {
    self.pc < self.start.len()
  }
  /// error(): 已记录的错误
  #[inline]
  pub const fn error(&self) -> Option<&WasmError> {
    self.error.as_ref()
  }

  // --- 位置族 ---
  /// position(): pc - start
  #[inline]
  pub const fn position(&self) -> u32 {
    self.pc as u32
  }
  /// pc_offset(pc): 相对模块偏移
  #[inline]
  pub const fn pc_offset_at(&self, pc: usize) -> u32 {
    pc as u32 + self.buffer_offset
  }
  /// pc_offset()
  #[inline]
  pub const fn pc_offset(&self) -> u32 {
    self.pc_offset_at(self.pc)
  }
  /// buffer_offset()
  #[inline]
  pub const fn buffer_offset(&self) -> u32 {
    self.buffer_offset
  }
  /// GetBufferRelativeOffset(): 模块偏移 → 当前缓冲区相对偏移
  #[inline]
  pub const fn get_buffer_relative_offset(&self, offset: u32) -> u32 {
    debug_assert!(self.buffer_offset <= offset);
    offset - self.buffer_offset
  }
  /// start()
  #[inline]
  pub const fn start(&self) -> &'a [u8] {
    self.start
  }
  /// pc() (相对偏移)
  #[inline]
  pub const fn pc(&self) -> usize {
    self.pc
  }
  /// end() (相对偏移 = 缓冲区长度)
  #[inline]
  pub const fn end(&self) -> usize {
    self.start.len()
  }

  /// lookahead(offset, expected): pc+{offset} 处字节是否为 {expected}
  #[inline]
  pub fn lookahead(&self, offset: usize, expected: u8) -> bool {
    self.end() - self.pc() > offset && self.start[self.pc + offset] == expected
  }

  /// Reset(bytes, buffer_offset): 重置边界与错误
  pub fn reset(&mut self, bytes: &'a [u8], buffer_offset: u32) {
    self.start = bytes;
    self.pc = 0;
    self.buffer_offset = buffer_offset;
    self.error = None;
  }

  // ------------------------------------------------------------------
  // read 族: 不前进 pc; 越界 → Err (Full 校验同时记录首错)
  // ------------------------------------------------------------------

  /// read_u8(pc)
  pub fn read_u8<V: ValidationTag>(&mut self, pc: usize) -> Result<u8> {
    self.read_le_bytes::<u8, V>(pc, "expected 1 byte")
  }

  /// read_u32(pc): 小端
  pub fn read_u32<V: ValidationTag>(&mut self, pc: usize) -> Result<u32> {
    self.read_le_bytes::<u32, V>(pc, "expected 4 bytes")
  }

  /// read_u64(pc): 小端
  pub fn read_u64<V: ValidationTag>(&mut self, pc: usize) -> Result<u64> {
    self.read_le_bytes::<u64, V>(pc, "expected 8 bytes")
  }

  fn read_le_bytes<T: LeBytes, V: ValidationTag>(&mut self, pc: usize, msg: &str) -> Result<T> {
    match self.start.get(pc..).and_then(T::read_le) {
      Some(v) => Ok(v),
      None => {
        if V::VALIDATE {
          self.error_at_pc(pc, msg);
        }
        Err(Error::Wasm(WasmError::new(self.pc_offset_at(pc), msg)))
      }
    }
  }

  /// read_u32v(pc): 无符号 LEB32 → (值, 字节数)
  pub fn read_u32v<V: ValidationTag>(&mut self, pc: usize) -> Result<(u32, u32)> {
    self.read_leb::<u32, V, 32>(pc, "LEB32")
  }

  /// read_i32v(pc): 有符号 LEB32
  pub fn read_i32v<V: ValidationTag>(&mut self, pc: usize) -> Result<(i32, u32)> {
    self.read_leb::<i32, V, 32>(pc, "signed LEB32")
  }

  /// read_u64v(pc): 无符号 LEB64
  pub fn read_u64v<V: ValidationTag>(&mut self, pc: usize) -> Result<(u64, u32)> {
    self.read_leb::<u64, V, 64>(pc, "LEB64")
  }

  /// read_i64v(pc): 有符号 LEB64
  pub fn read_i64v<V: ValidationTag>(&mut self, pc: usize) -> Result<(i64, u32)> {
    self.read_leb::<i64, V, 64>(pc, "signed LEB64")
  }

  /// read_i33v(pc): 有符号 LEB33 (33 位, i64 承载)
  pub fn read_i33v<V: ValidationTag>(&mut self, pc: usize) -> Result<(i64, u32)> {
    self.read_leb::<i64, V, 33>(pc, "signed LEB33")
  }

  /// read_leb: LEB 解码核心 (原版模板递归 → 循环; 语义逐条对应)
  ///
  /// SIZE_IN_BITS ≤ 8*sizeof(T); 最后字节 extra bits 校验:
  /// 无符号要求高位为 0, 有符号要求高位全 0 或全 1。
  pub fn read_leb<T: LebInt, V: ValidationTag, const SIZE_IN_BITS: u32>(
    &mut self,
    pc: usize,
    name: &str,
  ) -> Result<(T, u32)> {
    const fn max_length(size_in_bits: u32) -> u32 {
      size_in_bits.div_ceil(7)
    }
    let k_max_length = max_length(SIZE_IN_BITS);
    debug_assert!(SIZE_IN_BITS <= T::ACCUM_BITS, "leb does not fit in type");

    // 快路径: 单字节且无 continuation
    if let Some(&b) = self.start.get(pc).filter(|&b| b & 0x80 == 0) {
      let result = if T::SIGNED {
        // 符号扩展: 7 位 → 全宽 (算术右移)
        T::sign_extend(T::Accum::from(b), T::ACCUM_BITS - 7)
      } else {
        T::Accum::from(b)
      };
      return Ok((T::from_accum(result), 1));
    }

    // 慢路径: 逐字节累积
    let mut result: T::Accum = T::Accum::from(0u8);
    let mut byte_index: u32 = 0;
    let mut b: u8;
    loop {
      // at_end 校验 (原版 validate 时); 报错位置 = 越界字节处
      let at_end = pc + byte_index as usize >= self.start.len();
      if at_end {
        let at_pc = pc + byte_index as usize;
        let msg = format!("reached end while decoding {name}");
        if V::VALIDATE {
          self.error_at_pc(at_pc, msg.clone());
        }
        return Err(Error::Wasm(WasmError::new(self.pc_offset_at(at_pc), msg)));
      }
      b = self.start[pc + byte_index as usize];
      let shift = byte_index * 7;
      result = result | (T::Accum::from(b & 0x7f) << shift);
      if b & 0x80 == 0 {
        break;
      }
      byte_index += 1;
      if byte_index >= k_max_length {
        // 尾字节仍带 continuation 位 → 长度溢出; 报错位置 = 溢出字节处 (原版递归 pc)
        let overflow_pc = pc + byte_index as usize - 1;
        let msg = format!("length overflow while decoding {name}");
        if V::VALIDATE {
          self.error_at_pc(overflow_pc, msg.clone());
        }
        return Err(Error::Wasm(WasmError::new(
          self.pc_offset_at(overflow_pc),
          msg,
        )));
      }
    }

    // 末字节 extra bits 校验
    if byte_index == k_max_length - 1 {
      let k_extra_bits = SIZE_IN_BITS - (k_max_length - 1) * 7;
      let k_sign_ext_bits = k_extra_bits - u32::from(T::SIGNED);
      let checked_bits = b & (0xFFu8 << k_sign_ext_bits);
      let k_sign_extended_extra_bits: u8 = 0x7f & (0xFF << k_sign_ext_bits);
      let valid_extra_bits = checked_bits == 0
        || (T::SIGNED && checked_bits == k_sign_extended_extra_bits);
      if V::VALIDATE && !valid_extra_bits {
        let last_pc = pc + byte_index as usize;
        let msg = "extra bits in varint".to_string();
        self.error_at_pc(last_pc, msg.clone());
        return Err(Error::Wasm(WasmError::new(self.pc_offset_at(last_pc), msg)));
      }
      debug_assert!(valid_extra_bits || !V::VALIDATE);
    }

    // 符号扩展 (有符号且非快路径; 算术右移)
    let shift = byte_index * 7;
    let sign_ext_shift = T::ACCUM_BITS.saturating_sub(shift + 7);
    if T::SIGNED {
      result = T::sign_extend(result, sign_ext_shift);
    }
    let length = byte_index + 1;
    Ok((T::from_accum(result), length))
  }

  /// read_prefixed_opcode(pc): 前缀操作码 (LEB 索引);
  /// 返回 (opcode, 总长含前缀字节)。索引 > 0xFFF → 错误。
  pub fn read_prefixed_opcode<V: ValidationTag>(&mut self, pc: usize) -> Result<(WasmOpcode, u32)> {
    // 前缀 opcodes 均 LEB128 编码
    let (index, index_length) = self.read_u32v::<V>(pc + 1)?;
    let length = index_length + 1; // 1 为前缀字节
    // 仅支持解码后 ≤ 0xFFF 的操作码; 更大值需 >2 字节, 下方 <<12 将出错
    if V::VALIDATE && index > 0xfff {
      let msg = format!("Invalid prefixed opcode {index}");
      self.error_at_pc(pc, msg.clone());
      // 校验失败返回 "unreachable" (opcode 0)
      return Ok((K_EXPR_UNREACHABLE, 0));
    }
    let prefix = *self.start.get(pc).unwrap_or(&0);
    if index > 0xff {
      return Ok((u32::from(prefix) << 12 | index, length));
    }
    Ok((u32::from(prefix) << 8 | index, length))
  }

  // ------------------------------------------------------------------
  // consume 族: 成功前进 pc; 失败 pc = end (原版语义)
  // ------------------------------------------------------------------

  /// consume_u8: 读 1 字节并前进
  pub fn consume_u8(&mut self, name: &str) -> Result<u8> {
    self.consume_le_bytes::<u8>(name)
  }

  /// consume_u32: 读 4 字节 (小端) 并前进
  pub fn consume_u32(&mut self, name: &str) -> Result<u32> {
    self.consume_le_bytes::<u32>(name)
  }

  fn consume_le_bytes<T: LeBytes>(&mut self, name: &str) -> Result<T> {
    let Some(val) = self.start.get(self.pc..).and_then(T::read_le) else {
      // 原版: checkAvailable 记录错误 + pc_ = end_
      self.errorf(self.pc_offset(), format!("expected {} bytes, fell off end", T::BYTE_SIZE));
      self.pc = self.start.len();
      return Err(self.error.clone().map(Error::Wasm).unwrap_or_else(|| {
        Error::Wasm(WasmError::new(self.pc_offset(), format!("expected {name}")))
      }));
    };
    log::trace!("  +{}  {}: {:?}", self.pc_offset(), name, val);
    self.pc += T::BYTE_SIZE;
    Ok(val)
  }

  /// consume_u32v: LEB128 无符号 32 位并前进
  pub fn consume_u32v(&mut self, name: &str) -> Result<u32> {
    let (result, length) = self.read_leb::<u32, FullValidation, 32>(self.pc, name)?;
    log::trace!("  +{}  {}: {}", self.pc_offset(), name, result);
    self.pc += length as usize;
    Ok(result)
  }

  /// consume_i32v: LEB128 有符号 32 位并前进
  pub fn consume_i32v(&mut self, name: &str) -> Result<i32> {
    let (result, length) = self.read_leb::<i32, FullValidation, 32>(self.pc, name)?;
    log::trace!("  +{}  {}: {}", self.pc_offset(), name, result);
    self.pc += length as usize;
    Ok(result)
  }

  /// consume_u64v: LEB128 无符号 64 位并前进
  pub fn consume_u64v(&mut self, name: &str) -> Result<u64> {
    let (result, length) = self.read_leb::<u64, FullValidation, 64>(self.pc, name)?;
    log::trace!("  +{}  {}: {}", self.pc_offset(), name, result);
    self.pc += length as usize;
    Ok(result)
  }

  /// consume_bytes(size): 跳过 {size} 字节; 不足则 pc = end
  pub fn consume_bytes(&mut self, size: u32, name: &str) -> Result<()> {
    // 仅在 name 非空时 trace (原版 TRACE_IF(name, ...))
    if !name.is_empty() {
      log::trace!("  +{}  {}: {} bytes", self.pc_offset(), name, size);
    }
    if self.check_available(size) {
      self.pc += size as usize;
      Ok(())
    } else {
      self.pc = self.start.len();
      Err(self.error.clone().map(Error::Wasm).unwrap_or(Error::Wasm(WasmError::new(
        self.pc_offset(),
        "fell off end",
      ))))
    }
  }
}

/// 小端定宽读取契约 (read_little_endian 的 IntType)
/// (数组长度泛型受限 → read_le 切片形态, 长度不足 → None)
trait LeBytes: Copy + core::fmt::Debug {
  /// 字节数
  const BYTE_SIZE: usize;
  /// 小端读取
  fn read_le(bytes: &[u8]) -> Option<Self>;
}

impl LeBytes for u8 {
  const BYTE_SIZE: usize = 1;
  #[inline]
  fn read_le(bytes: &[u8]) -> Option<Self> {
    bytes.first().copied()
  }
}

impl LeBytes for u32 {
  const BYTE_SIZE: usize = 4;
  #[inline]
  fn read_le(bytes: &[u8]) -> Option<Self> {
    let b: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
    Some(u32::from_le_bytes(b))
  }
}

impl LeBytes for u64 {
  const BYTE_SIZE: usize = 8;
  #[inline]
  fn read_le(bytes: &[u8]) -> Option<Self> {
    let b: [u8; 8] = bytes.get(..8)?.try_into().ok()?;
    Some(u64::from_le_bytes(b))
  }
}
`;

const p = writeSrc("v8-wasm-core", "decoder.rs", out);
console.log(`→ ${p}`);
