#!/usr/bin/env bun
// tool/src/codegen/handler-table.h.js
// 机械提取并生成 v8/src/codegen/handler-table.{h,cc}: 布局常量 + CatchPrediction + 位域 + 核心方法
// → v8-codegen-core/src/handler_table.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/handler-table.h");
const ccSrc = readV8("src/codegen/handler-table.cc");

const m = (re, what, src = hSrc) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

// 布局常量对数
for (const [name, val] of [
  ["kRangeStartIndex", "0"], ["kRangeEndIndex", "1"], ["kRangeHandlerIndex", "2"],
  ["kRangeDataIndex", "3"], ["kRangeEntrySize", "4"],
  ["kReturnOffsetIndex", "0"], ["kReturnHandlerIndex", "1"], ["kReturnEntrySize", "2"],
]) {
  const r = m(new RegExp(`static const int ${name} = (\\d+);`), name);
  if (r[1] !== val) throw new Error(`${name} = ${r[1]} != ${val}`);
}
const noHandler = m(/static const int kNoHandlerFound = (-?\d+);/, "kNoHandlerFound");
if (noHandler[1] !== "-1") throw new Error("kNoHandlerFound 异常");

// 位域: CatchPrediction 在 0..3, was_used 1 位, handler 偏移 28 位
m(/using HandlerPredictionField = base::BitField<CatchPrediction, 0, 3>;/, "HandlerPredictionField");
m(/using HandlerWasUsedField = HandlerPredictionField::Next<bool, 1>;/, "HandlerWasUsedField");
m(/using HandlerOffsetField = HandlerWasUsedField::Next<int, 28>;/, "HandlerOffsetField");
const lazyDeopt = m(/static const int kLazyDeopt = HandlerOffsetField::kMax;/, "kLazyDeopt");

// CatchPrediction 枚举 (原版 5 项)
const catchPred = [
  "UNCAUGHT",
  "CAUGHT",
  "PROMISE",
  "ASYNC_AWAIT",
  "UNCAUGHT_ASYNC_AWAIT",
];
for (const p of catchPred) {
  if (!hSrc.includes(p)) throw new Error(`缺 CatchPrediction ${p}`);
}
// EncodingMode 2 项
for (const e of ["kRangeBasedEncoding", "kReturnAddressBasedEncoding"]) {
  if (!hSrc.includes(e)) throw new Error(`缺 EncodingMode ${e}`);
}

// .cc 方法清点 (对数, 关键方法在场)
for (const want of ["GetRangeStart", "SetRangeStart", "LookupReturn", "NumberOfReturnEntries",
  "LengthForRange", "EmitReturnTableStart", "EntrySizeFromMode"]) {
  if (!ccSrc.includes(`HandlerTable::${want}`)) throw new Error(`.cc 缺 ${want}`);
}

let rust = banner(
  "src/codegen/handler-table.h.js",
  "src/codegen/handler-table.{h,cc}",
  "HandlerTable 布局常量 + CatchPrediction/位域 + 核心方法填体",
);

rust += `
use std::{
  fmt,
  ptr::{read_unaligned, write_unaligned},
};

use crate::{
  assembler_base::AssemblerBase,
  error::{Error, Result},
};

/// 原版 enum CatchPrediction: 异常局部捕获的保守预测 (调试器近似)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CatchPrediction {
  /// 可能向外重抛
  Uncaught = 0,
  /// 会被 handler 捕获
  Caught = 1,
  /// 捕获并转为 promise 拒绝
  Promise = 2,
  /// async/await 脱糖中捕获并 promise 拒绝
  AsyncAwait = 3,
  /// async REPL 捕获并 promise 拒绝
  UncaughtAsyncAwait = 4,
}

impl fmt::Display for CatchPrediction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Uncaught => write!(f, "UNCAUGHT"),
      Self::Caught => write!(f, "CAUGHT"),
      Self::Promise => write!(f, "PROMISE"),
      Self::AsyncAwait => write!(f, "ASYNC_AWAIT"),
      Self::UncaughtAsyncAwait => write!(f, "UNCAUGHT_ASYNC_AWAIT"),
    }
  }
}

/// 原版 enum EncodingMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EncodingMode {
  /// 未优化码: [range_start, range_end, handler, data]
  RangeBased = 0,
  /// 优化码: [return_offset, handler]
  ReturnAddressBased = 1,
}

/// 语义别名: 对齐旧版或测试用名
pub type HandlerTableType = EncodingMode;

// —— 布局常量 (range 编码) ——
pub const K_RANGE_START_INDEX: usize = 0;
pub const K_RANGE_END_INDEX: usize = 1;
pub const K_RANGE_HANDLER_INDEX: usize = 2;
pub const K_RANGE_DATA_INDEX: usize = 3;
pub const K_RANGE_ENTRY_SIZE: usize = 4;

// —— 布局常量 (return 编码) ——
pub const K_RETURN_OFFSET_INDEX: usize = 0;
pub const K_RETURN_HANDLER_INDEX: usize = 1;
pub const K_RETURN_ENTRY_SIZE: usize = 2;

/// 未找到 handler 哨兵
pub const K_NO_HANDLER_FOUND: i32 = ${noHandler[1]};

// —— handler 字段位域: [0..3] prediction | [3] was_used | [4..32] offset ——
pub const HANDLER_PREDICTION_SHIFT: u32 = 0;
pub const HANDLER_PREDICTION_BITS: u32 = 3;
pub const HANDLER_WAS_USED_SHIFT: u32 = 3;
pub const HANDLER_WAS_USED_BITS: u32 = 1;
pub const HANDLER_OFFSET_SHIFT: u32 = 4;
pub const HANDLER_OFFSET_BITS: u32 = 28;
pub const HANDLER_OFFSET_MASK: i32 = ((1i32 << HANDLER_OFFSET_BITS) - 1) << HANDLER_OFFSET_SHIFT;
/// 原版 kLazyDeopt = HandlerOffsetField::kMax (28 位全 1: 0x0fff_ffff)
pub const K_LAZY_DEOPT: i32 = (1i32 << HANDLER_OFFSET_BITS) - 1;

// —— 位域编解码 (原版 base::BitField<...>) ——

/// 原版 HandlerPredictionField (BitField<CatchPrediction, 0, 3>)::decode
#[inline]
const fn handler_prediction_decode(bitfield: i32) -> u32 {
  ((bitfield as u32) >> HANDLER_PREDICTION_SHIFT) & ((1u32 << HANDLER_PREDICTION_BITS) - 1)
}

/// 原版 HandlerPredictionField::encode
#[inline]
const fn handler_prediction_encode(pred: CatchPrediction) -> i32 {
  (((pred as u32) & ((1u32 << HANDLER_PREDICTION_BITS) - 1)) << HANDLER_PREDICTION_SHIFT) as i32
}

/// 原版 HandlerWasUsedField (Next<bool, 1>, shift 3)::decode
#[inline]
const fn handler_was_used_decode(bitfield: i32) -> bool {
  ((bitfield as u32) >> HANDLER_WAS_USED_SHIFT) & 1 != 0
}

/// 原版 HandlerWasUsedField::encode
#[inline]
const fn handler_was_used_encode(value: bool) -> i32 {
  ((value as u32) << HANDLER_WAS_USED_SHIFT) as i32
}

/// 原版 HandlerWasUsedField::update(x, value): 更新 was_used 位
#[inline]
const fn handler_was_used_update(x: i32, value: bool) -> i32 {
  (x & !(((1i32 << HANDLER_WAS_USED_BITS) - 1) << HANDLER_WAS_USED_SHIFT))
    | handler_was_used_encode(value)
}

/// 原版 HandlerOffsetField (Next<int, 28>, shift 4)::decode
#[inline]
const fn handler_offset_decode(bitfield: i32) -> i32 {
  (((bitfield as u32) >> HANDLER_OFFSET_SHIFT) & ((1u32 << HANDLER_OFFSET_BITS) - 1)) as i32
}

/// 原版 HandlerOffsetField::encode
#[inline]
const fn handler_offset_encode(offset: i32) -> i32 {
  (((offset as u32) & ((1u32 << HANDLER_OFFSET_BITS) - 1)) << HANDLER_OFFSET_SHIFT) as i32
}

/// 原版 HandlerOffsetField::is_valid: 0 <= offset <= kLazyDeopt (kMax 亦在有效位域内)
#[inline]
const fn handler_offset_is_valid(offset: i32) -> bool {
  offset >= 0 && offset <= K_LAZY_DEOPT
}

/// 原版 Memory<int32_t>(raw + field_index*4) 读 (entry 基址 + 字段偏移)
///
/// # Safety
/// \`base\` 起 (field_index*4) 处必须为合法可读 i32 槽 (表缓冲覆盖 index)。
#[inline]
unsafe fn read_i32(base: usize, field_index: u32) -> i32 {
  // SAFETY: 契约保证 base 起 field_index*4 处落在 i32 表内; read_unaligned 避免对齐 UB。
  unsafe {
    let ptr = (base + (field_index as usize) * 4) as *const i32;
    read_unaligned(ptr)
  }
}

/// 原版 Memory<int32_t>(raw + field_index*4) 写
///
/// # Safety
/// \`base\` 起 (field_index*4) 处必须为合法可写 i32 槽。
#[inline]
unsafe fn write_i32(base: usize, field_index: u32, value: i32) {
  // SAFETY: 契约保证 base 起 field_index*4 处落在可写 i32 表内; write_unaligned 避免对齐 UB。
  unsafe {
    let ptr = (base + (field_index as usize) * 4) as *mut i32;
    write_unaligned(ptr, value);
  }
}

/// 原版 HandlerTable: 异常 handler 表 (range/return 双编码)
pub struct HandlerTable {
  /// 原版 number_of_entries_
  number_of_entries_: u32,
  /// 原版 mode_ (DEBUG 才有; 基线保留)
  mode_: EncodingMode,
  /// 原版 raw_encoded_data_ (指向 TrustedByteArray/InstructionStream 内部)
  raw_encoded_data_: usize,
}

impl HandlerTable {
  /// 原版 EntrySizeFromMode: range=4 * int, return=2 * int (单位: 字段数)
  #[inline]
  #[must_use]
  pub const fn entry_size_from_mode(mode: EncodingMode) -> i32 {
    match mode {
      EncodingMode::RangeBased => K_RANGE_ENTRY_SIZE as i32,
      EncodingMode::ReturnAddressBased => K_RETURN_ENTRY_SIZE as i32,
    }
  }

  /// 原版 LengthForRange: entries * kRangeEntrySize * sizeof(int32_t)
  #[inline]
  #[must_use]
  pub const fn length_for_range(entries: u32) -> u32 {
    entries * K_RANGE_ENTRY_SIZE as u32 * 4
  }

  // —— 构造族 ——

  /// 原版 HandlerTable(InstructionStream) — 依赖堆对象 (v8-objects-rest), 推迟
  pub fn from_instruction_stream(_code: *mut ()) -> Self {
    todo!("推迟: 依赖 InstructionStream 堆对象读取 handler_table_address/size")
  }

  /// 原版 HandlerTable(Code) — 依赖堆对象 (v8-objects-rest), 推迟
  pub fn from_code(_code: *mut ()) -> Self {
    todo!("推迟: 依赖 Code 对象 handler_table_address/size (objects-rest)")
  }

  /// 原版 HandlerTable(TrustedByteArray) — 依赖堆对象 (v8-objects-rest), 推迟
  pub fn from_byte_array(_byte_array: *mut ()) -> Self {
    todo!("推迟: 依赖 TrustedByteArray begin/ulength (objects-rest)")
  }

  /// 原版 HandlerTable(BytecodeArray) — 依赖堆对象 (v8-objects-rest), 推迟
  pub fn from_bytecode_array(_bytecode_array: *mut ()) -> Self {
    todo!("推迟: 依赖 BytecodeArray::handler_table (objects-rest)")
  }

  /// 原版 handler-table.cc HandlerTable(Address, uint32_t size, EncodingMode):
  /// number_of_entries_ = size / EntrySizeFromMode(mode) / sizeof(int32_t)
  #[must_use]
  pub fn from_raw(
    handler_table: usize,
    handler_table_size: u32,
    encoding_mode: EncodingMode,
  ) -> Self {
    let entry_bytes = Self::entry_size_from_mode(encoding_mode) as u32 * 4;
    let max_padding = match encoding_mode {
      EncodingMode::ReturnAddressBased => 4,
      EncodingMode::RangeBased => 0,
    };
    debug_assert!(
      handler_table_size % entry_bytes <= max_padding,
      "handler_table_size padding 校验失败"
    );
    Self {
      number_of_entries_: handler_table_size
        / Self::entry_size_from_mode(encoding_mode) as u32
        / 4, // sizeof(int32_t)
      mode_: encoding_mode,
      raw_encoded_data_: handler_table,
    }
  }

  /// 获取表项条数
  #[inline]
  #[must_use]
  pub const fn num_entries(&self) -> u32 {
    self.number_of_entries_
  }

  // —— range 访问器 ——

  /// 原版 handler-table.cc GetRangeHandlerBitfield
  #[inline]
  fn get_range_handler_bitfield(&self, index: u32) -> i32 {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    assert!(index < self.number_of_range_entries(), "越界访问 range 表项");
    let field_index = index * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_HANDLER_INDEX as u32;
    unsafe {
      // SAFETY: number_of_entries 覆盖 index; raw_encoded_data_ 指向合法 i32 表。
      read_i32(self.raw_encoded_data_, field_index)
    }
  }

  /// 原版 handler-table.cc GetRangeStart
  #[must_use]
  pub fn get_range_start(&self, index: u32) -> i32 {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    assert!(index < self.number_of_range_entries(), "越界访问 range 表项");
    let field_index = index * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_START_INDEX as u32;
    unsafe {
      // SAFETY: number_of_entries 覆盖 index。
      read_i32(self.raw_encoded_data_, field_index)
    }
  }

  /// 原版 handler-table.cc GetRangeEnd
  #[must_use]
  pub fn get_range_end(&self, index: u32) -> i32 {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    assert!(index < self.number_of_range_entries(), "越界访问 range 表项");
    let field_index = index * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_END_INDEX as u32;
    unsafe {
      // SAFETY: number_of_entries 覆盖 index。
      read_i32(self.raw_encoded_data_, field_index)
    }
  }

  /// 原版 handler-table.cc GetRangeHandler = HandlerOffsetField::decode(bitfield)
  #[must_use]
  pub fn get_range_handler(&self, index: u32) -> i32 {
    handler_offset_decode(self.get_range_handler_bitfield(index))
  }

  /// 别名访问器: get_handler_offset
  #[inline]
  #[must_use]
  pub fn get_handler_offset(&self, index: u32) -> i32 {
    self.get_range_handler(index)
  }

  /// 原版 handler-table.cc GetRangeData
  #[must_use]
  pub fn get_range_data(&self, index: u32) -> i32 {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    assert!(index < self.number_of_range_entries(), "越界访问 range 表项");
    let field_index = index * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_DATA_INDEX as u32;
    unsafe {
      // SAFETY: number_of_entries 覆盖 index。
      read_i32(self.raw_encoded_data_, field_index)
    }
  }

  /// 原版 handler-table.cc GetRangePrediction = HandlerPredictionField::decode(bitfield)
  #[must_use]
  pub fn get_range_prediction(&self, index: u32) -> CatchPrediction {
    match handler_prediction_decode(self.get_range_handler_bitfield(index)) {
      0 => CatchPrediction::Uncaught,
      1 => CatchPrediction::Caught,
      2 => CatchPrediction::Promise,
      3 => CatchPrediction::AsyncAwait,
      _ => CatchPrediction::UncaughtAsyncAwait,
    }
  }

  /// 原版 handler-table.cc HandlerWasUsed = HandlerWasUsedField::decode(bitfield)
  #[must_use]
  pub fn handler_was_used(&self, index: u32) -> bool {
    handler_was_used_decode(self.get_range_handler_bitfield(index))
  }

  /// 原版 handler-table.cc MarkHandlerUsed: Relaxed_Store(update(mem, true))
  pub fn mark_handler_used(&mut self, index: u32) {
    self.set_handler_was_used(index, true);
  }

  /// 更新 was_used 位状态
  pub fn set_handler_was_used(&mut self, index: u32, was_used: bool) {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    assert!(index < self.number_of_range_entries(), "越界访问 range 表项");
    let field_index = index * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_HANDLER_INDEX as u32;
    unsafe {
      // SAFETY: number_of_entries 覆盖 index。
      let mem = read_i32(self.raw_encoded_data_, field_index);
      write_i32(
        self.raw_encoded_data_,
        field_index,
        handler_was_used_update(mem, was_used),
      );
    }
  }

  /// 原版 handler-table.cc SetRangeStart
  pub fn set_range_start(&mut self, index: u32, value: i32) {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    assert!(index < self.number_of_range_entries(), "越界写入 range 表项");
    let field_index = index * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_START_INDEX as u32;
    unsafe {
      // SAFETY: 调用方保证 index 在表容量内。
      write_i32(self.raw_encoded_data_, field_index, value);
    }
  }

  /// 原版 handler-table.cc SetRangeEnd
  pub fn set_range_end(&mut self, index: u32, value: i32) {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    assert!(index < self.number_of_range_entries(), "越界写入 range 表项");
    let field_index = index * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_END_INDEX as u32;
    unsafe {
      // SAFETY: 调用方保证 index 在表容量内。
      write_i32(self.raw_encoded_data_, field_index, value);
    }
  }

  /// 原版 handler-table.cc SetRangeData
  pub fn set_range_data(&mut self, index: u32, value: i32) {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    assert!(index < self.number_of_range_entries(), "越界写入 range 表项");
    let field_index = index * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_DATA_INDEX as u32;
    unsafe {
      // SAFETY: 调用方保证 index 在表容量内。
      write_i32(self.raw_encoded_data_, field_index, value);
    }
  }

  /// 原版 handler-table.cc SetRangeHandler(index, offset, pred)
  pub fn set_range_handler(&mut self, index: u32, offset: i32, pred: CatchPrediction) {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    assert!(index < self.number_of_range_entries(), "越界写入 range 表项");
    assert!(
      handler_offset_is_valid(offset),
      "handler offset 越界 (超出 [0, kLazyDeopt])"
    );
    let value = handler_offset_encode(offset)
      | handler_was_used_encode(false)
      | handler_prediction_encode(pred);
    let field_index = index * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_HANDLER_INDEX as u32;
    unsafe {
      // SAFETY: 调用方保证 index 在表容量内。
      write_i32(self.raw_encoded_data_, field_index, value);
    }
  }

  /// 批量设置 Range 表项字段
  pub fn set_range(
    &mut self,
    index: u32,
    start: i32,
    end: i32,
    offset: i32,
    data: i32,
    pred: CatchPrediction,
  ) {
    self.set_range_start(index, start);
    self.set_range_end(index, end);
    self.set_range_handler(index, offset, pred);
    self.set_range_data(index, data);
  }

  // —— return 访问器 ——

  /// 原版 handler-table.cc GetReturnOffset
  #[must_use]
  pub fn get_return_offset(&self, index: u32) -> i32 {
    assert!(self.mode_ == EncodingMode::ReturnAddressBased, "需 ReturnAddressBased 模式");
    assert!(index < self.number_of_return_entries(), "越界访问 return 表项");
    let field_index = index * K_RETURN_ENTRY_SIZE as u32 + K_RETURN_OFFSET_INDEX as u32;
    unsafe {
      // SAFETY: number_of_entries 覆盖 index。
      read_i32(self.raw_encoded_data_, field_index)
    }
  }

  /// 原版 handler-table.cc GetReturnHandler = HandlerOffsetField::decode(Memory[int])
  #[must_use]
  pub fn get_return_handler(&self, index: u32) -> i32 {
    assert!(self.mode_ == EncodingMode::ReturnAddressBased, "需 ReturnAddressBased 模式");
    assert!(index < self.number_of_return_entries(), "越界访问 return 表项");
    let field_index = index * K_RETURN_ENTRY_SIZE as u32 + K_RETURN_HANDLER_INDEX as u32;
    let bitfield = unsafe {
      // SAFETY: number_of_entries 覆盖 index。
      read_i32(self.raw_encoded_data_, field_index)
    };
    handler_offset_decode(bitfield)
  }

  /// 获取 return 表项预测 (解出 prediction 位)
  #[must_use]
  pub fn get_return_prediction(&self, index: u32) -> CatchPrediction {
    assert!(self.mode_ == EncodingMode::ReturnAddressBased, "需 ReturnAddressBased 模式");
    assert!(index < self.number_of_return_entries(), "越界访问 return 表项");
    let field_index = index * K_RETURN_ENTRY_SIZE as u32 + K_RETURN_HANDLER_INDEX as u32;
    let bitfield = unsafe { read_i32(self.raw_encoded_data_, field_index) };
    match handler_prediction_decode(bitfield) {
      0 => CatchPrediction::Uncaught,
      1 => CatchPrediction::Caught,
      2 => CatchPrediction::Promise,
      3 => CatchPrediction::AsyncAwait,
      _ => CatchPrediction::UncaughtAsyncAwait,
    }
  }

  /// 设置 Return 表项 offset
  pub fn set_return_offset(&mut self, index: u32, offset: i32) {
    assert!(self.mode_ == EncodingMode::ReturnAddressBased, "需 ReturnAddressBased 模式");
    assert!(index < self.number_of_return_entries(), "越界写入 return 表项");
    let field_index = index * K_RETURN_ENTRY_SIZE as u32 + K_RETURN_OFFSET_INDEX as u32;
    unsafe {
      write_i32(self.raw_encoded_data_, field_index, offset);
    }
  }

  /// 设置 Return 表项 handler offset
  pub fn set_return_handler(&mut self, index: u32, handler: i32) {
    assert!(self.mode_ == EncodingMode::ReturnAddressBased, "需 ReturnAddressBased 模式");
    assert!(index < self.number_of_return_entries(), "越界写入 return 表项");
    assert!(
      handler_offset_is_valid(handler),
      "handler offset 越界 (超出 [0, kLazyDeopt])"
    );
    let field_index = index * K_RETURN_ENTRY_SIZE as u32 + K_RETURN_HANDLER_INDEX as u32;
    unsafe {
      write_i32(self.raw_encoded_data_, field_index, handler_offset_encode(handler));
    }
  }

  /// 设置完整的 Return 表项
  pub fn set_return_entry(&mut self, index: u32, offset: i32, handler: i32) {
    self.set_return_offset(index, offset);
    self.set_return_handler(index, handler);
  }

  /// 简易别名设置 return offset
  pub fn set_return(&mut self, index: u32, offset: i32) {
    self.set_return_offset(index, offset);
  }

  /// 原版 handler-table.cc LookupHandlerIndexForRange: 遍历嵌套 range 找最内层
  #[must_use]
  pub fn lookup_handler_index_for_range(&self, pc_offset: i32) -> i32 {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    let mut innermost_handler: i32 = K_NO_HANDLER_FOUND;
    let n = self.number_of_entries_;
    for i in 0..n {
      let start_offset = unsafe {
        read_i32(
          self.raw_encoded_data_,
          i * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_START_INDEX as u32,
        )
      };
      let end_offset = unsafe {
        read_i32(
          self.raw_encoded_data_,
          i * K_RANGE_ENTRY_SIZE as u32 + K_RANGE_END_INDEX as u32,
        )
      };
      if end_offset <= pc_offset {
        continue;
      }
      if start_offset > pc_offset {
        break;
      }
      innermost_handler = i as i32;
    }
    innermost_handler
  }

  /// 原版 handler-table.cc LookupReturn: 对 return offset 有序表 lower_bound 二分查找
  #[must_use]
  pub fn lookup_return(&self, pc_offset: i32) -> i32 {
    assert!(
      self.mode_ == EncodingMode::ReturnAddressBased,
      "需 ReturnAddressBased 模式"
    );
    let count = self.number_of_entries_;
    let mut lo = 0u32;
    let mut hi = count;
    while lo < hi {
      let mid = lo + (hi - lo) / 2;
      let offset = unsafe {
        read_i32(
          self.raw_encoded_data_,
          mid * K_RETURN_ENTRY_SIZE as u32 + K_RETURN_OFFSET_INDEX as u32,
        )
      };
      if offset < pc_offset {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }
    if lo < count {
      let offset = unsafe {
        read_i32(
          self.raw_encoded_data_,
          lo * K_RETURN_ENTRY_SIZE as u32 + K_RETURN_OFFSET_INDEX as u32,
        )
      };
      if offset == pc_offset {
        let bitfield = unsafe {
          read_i32(
            self.raw_encoded_data_,
            lo * K_RETURN_ENTRY_SIZE as u32 + K_RETURN_HANDLER_INDEX as u32,
          )
        };
        return handler_offset_decode(bitfield);
      }
    }
    K_NO_HANDLER_FOUND
  }

  /// 原版 NumberOfRangeEntries
  #[inline]
  #[must_use]
  pub const fn number_of_range_entries(&self) -> u32 {
    self.number_of_entries_
  }

  /// 原版 NumberOfReturnEntries
  #[inline]
  #[must_use]
  pub const fn number_of_return_entries(&self) -> u32 {
    self.number_of_entries_
  }

  /// 原版 handler-table.cc EmitReturnTableStart(masm)
  pub fn emit_return_table_start(masm: &mut AssemblerBase) -> Result<i32> {
    // 4 字节元数据对齐, 填充 0xcc
    while !masm.pc_offset().is_multiple_of(4) {
      masm.emit_u8(0xcc)?;
    }
    Ok(masm.pc_offset() as i32)
  }

  /// 原版 handler-table.cc EmitReturnEntry(masm, offset, handler)
  pub fn emit_return_entry(masm: &mut AssemblerBase, offset: i32, handler: i32) -> Result<()> {
    if !handler_offset_is_valid(handler) {
      return Err(Error::InvalidHandlerTable(
        "handler offset 越界 (超出 [0, kLazyDeopt])",
      ));
    }
    masm.emit_u32(offset as u32)?;
    masm.emit_u32(handler_offset_encode(handler) as u32)?;
    Ok(())
  }

  /// 原版 HandlerTableRangePrint 反汇编格式化
  #[must_use]
  pub fn format_range(&self) -> String {
    assert!(self.mode_ == EncodingMode::RangeBased, "需 RangeBased 模式");
    let mut out = String::from("   from   to       hdlr (prediction,   data)\\n");
    for i in 0..self.number_of_range_entries() {
      let pc_start = self.get_range_start(i);
      let pc_end = self.get_range_end(i);
      let handler_offset = self.get_range_handler(i);
      let handler_data = self.get_range_data(i);
      let prediction = self.get_range_prediction(i);
      out.push_str(&format!(
        "  ({:4},{:4})  ->  {:4} (prediction={}, data={})\\n",
        pc_start, pc_end, handler_offset, prediction, handler_data
      ));
    }
    out
  }

  /// 原版 HandlerTableReturnPrint 反汇编格式化
  #[must_use]
  pub fn format_return(&self) -> String {
    assert!(
      self.mode_ == EncodingMode::ReturnAddressBased,
      "需 ReturnAddressBased 模式"
    );
    let mut out = String::from("  offset   handler\\n");
    for i in 0..self.number_of_return_entries() {
      let pc_offset = self.get_return_offset(i);
      let handler_offset = self.get_return_handler(i);
      out.push_str(&format!("    {:4x}  ->  {:4x}\\n", pc_offset, handler_offset));
    }
    out
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(K_RANGE_ENTRY_SIZE == 4 && K_RETURN_ENTRY_SIZE == 2);
  assert!(K_NO_HANDLER_FOUND == -1);
  assert!(HANDLER_OFFSET_BITS == 28 && HANDLER_PREDICTION_BITS == 3);
  assert!(K_LAZY_DEOPT == (1 << 28) - 1);
  assert!(CatchPrediction::Uncaught as u8 == 0);
  assert!(CatchPrediction::UncaughtAsyncAwait as u8 == 4);
  assert!(HandlerTable::entry_size_from_mode(EncodingMode::RangeBased) == 4);
  assert!(HandlerTable::length_for_range(3) == 48);
  assert!(handler_offset_is_valid(0));
  assert!(handler_offset_is_valid(K_LAZY_DEOPT));
  assert!(!handler_offset_is_valid(-1));
  assert!(!handler_offset_is_valid(K_LAZY_DEOPT + 1));
};
`;

const p = writeSrc("v8-codegen-core", "handler_table.rs", rust);
console.log(
  `handler-table: 布局常量 8 / CatchPrediction ${catchPred.length} / 位域 3 / 方法填体完成 → ${p}`,
);
