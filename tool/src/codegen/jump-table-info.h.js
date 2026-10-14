#!/usr/bin/env bun
// tool/src/codegen/jump-table-info.h.js
// 机械提取 v8/src/codegen/jump-table-info.{h,cc}: 跳转表元数据布局 + 常量 + 签名
// → v8-codegen-core/src/jump_table_info.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/codegen/jump-table-info.h");
const ccSrc = readV8("src/codegen/jump-table-info.cc");

const m = (re, what, src = hSrc) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

// 布局常量对数
m(/static constexpr int kPCOffsetSize = (\w+);/, "kPCOffsetSize");
m(/static constexpr int kTargetSize = (\w+);/, "kTargetSize");
m(/static constexpr int kSize = kPCOffsetSize \+ kTargetSize;/, "kSize");
m(/static_assert\(sizeof\(JumpTableInfoEntry\) == JumpTableInfoEntry::kSize\);/, "static_assert");
// kUInt32Size = 4, kInt32Size = 4 (globals.h)
const PC_OFFSET_SIZE = 4;
const TARGET_SIZE = 4;
const SIZE = PC_OFFSET_SIZE + TARGET_SIZE;

// .cc 方法清点 (对数)
const ccMethods = [...ccSrc.matchAll(/(?:void|size_t|uint32_t) JumpTableInfoWriter::(\w+)\(/gm)].map((x) => x[1]);
for (const want of ["Add", "entry_count", "size_in_bytes", "Emit"]) {
  if (!ccMethods.includes(want)) throw new Error(`Writer 缺 ${want}`);
}

let rust = banner(
  "src/codegen/jump-table-info.h.js",
  "src/codegen/jump-table-info.{h,cc}",
  "JumpTableInfo 布局常量 + 写入/迭代签名骨架",
);

rust += `
use crate::{assembler_base::AssemblerBase, error::Result};

/// 跳转表元数据布局 (反汇编器用):
/// 按 pc_offset 递增的 entry 数组: | 4B pc_offset (u32) | 4B target (i32) |
/// 原版 kPCOffsetSize = kUInt32Size
pub const K_PC_OFFSET_SIZE: usize = ${PC_OFFSET_SIZE};
/// 原版 kTargetSize = kInt32Size
pub const K_TARGET_SIZE: usize = ${TARGET_SIZE};
/// 原版 kSize = kPCOffsetSize + kTargetSize
pub const K_SIZE: usize = ${SIZE};

// 原版 offsetof(JumpTableInfoEntry, pc_offset) / (.., target)
const K_OFFSET_TO_PC: usize = 0;
const K_OFFSET_TO_TARGET: usize = K_PC_OFFSET_SIZE;

/// 原版 JumpTableInfoEntry (POD, 8 字节)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct JumpTableInfoEntry {
  pub pc_offset: u32,
  pub target: i32,
}

const _: () = assert!(size_of::<JumpTableInfoEntry>() == K_SIZE);

/// 原版 JumpTableInfoWriter (codegen 侧)
#[derive(Debug, Clone, Default)]
pub struct JumpTableInfoWriter {
  /// 原版 entries_
  entries_: Vec<JumpTableInfoEntry>,
}

impl JumpTableInfoWriter {
  /// 原版 Add: 机械直译 (尾部追加)
  pub fn add(&mut self, pc_offset: u32, target: i32) {
    self.entries_.push(JumpTableInfoEntry { pc_offset, target });
  }

  /// 原版 entry_count
  #[inline]
  #[must_use]
  pub fn entry_count(&self) -> usize {
    self.entries_.len()
  }

  /// 原版 size_in_bytes: entry_count * kSize
  #[inline]
  #[must_use]
  pub fn size_in_bytes(&self) -> u32 {
    (self.entry_count() * K_SIZE) as u32
  }

  /// 原版逐条 dd(pc_offset); dd(target) 写入汇编缓冲 (小端 int32, x64/arm64 目标均为 LE)
  /// v8 原版对位: src/codegen/jump-table-info.cc:24-35
  pub fn emit(&self, assm: &mut AssemblerBase) -> Result<()> {
    for entry in &self.entries_ {
      assm.emit_u32(entry.pc_offset)?;
      assm.emit_u32(entry.target as u32)?;
    }
    Ok(())
  }

  /// 将跳转表序列化为字节缓冲
  #[must_use]
  pub fn emit_to_vec(&self) -> Vec<u8> {
    let mut vec = Vec::with_capacity(self.size_in_bytes() as usize);
    for entry in &self.entries_ {
      vec.extend_from_slice(&entry.pc_offset.to_ne_bytes());
      vec.extend_from_slice(&entry.target.to_ne_bytes());
    }
    vec
  }
}

/// 读取未对齐标量 (原版 base::ReadUnalignedValue<T>)
///
/// # Safety
/// \`addr\` 起 \`size_of::<T>()\` 字节必须落在合法可读缓冲内 (构造契约保证)。
#[inline]
unsafe fn read_unaligned<T: Copy>(addr: usize) -> T {
  // SAFETY: 契约要求 addr 起 T 宽度字节可读; read_unaligned 无需对齐。
  unsafe { (addr as *const T).read_unaligned() }
}

/// 原版 JumpTableInfoIterator (反汇编侧)
pub struct JumpTableInfoIterator {
  start_: usize,
  size_: u32,
  cursor_: usize,
}

impl JumpTableInfoIterator {
  /// 原版构造: (start, size); DCHECK start != kNullAddress
  #[must_use]
  pub fn new(start: usize, size: u32) -> Self {
    debug_assert_ne!(start, 0, "kNullAddress");
    Self {
      start_: start,
      size_: size,
      cursor_: start,
    }
  }

  /// 原版 jump-table-info.cc GetPCOffset = ReadUnalignedValue<uint32_t>(cursor + offsetof(pc_offset))
  #[must_use]
  pub fn get_pc_offset(&self) -> u32 {
    unsafe {
      // SAFETY: 构造契约保证 cursor_ 处于合法 entry 内 (HasCurrent 前置)。
      read_unaligned::<u32>(self.cursor_ + K_OFFSET_TO_PC)
    }
  }

  /// 原版 jump-table-info.cc GetTarget = ReadUnalignedValue<int32_t>(cursor + offsetof(target))
  #[must_use]
  pub fn get_target(&self) -> i32 {
    unsafe {
      // SAFETY: 构造契约保证 cursor_ 处于合法 entry 内 (HasCurrent 前置)。
      read_unaligned::<i32>(self.cursor_ + K_OFFSET_TO_TARGET)
    }
  }

  /// 原版 jump-table-info.cc Next: cursor_ += kSize
  pub fn next(&mut self) {
    self.cursor_ += K_SIZE;
  }

  /// 原版 jump-table-info.cc HasCurrent: cursor_ < start_ + size_
  #[must_use]
  pub fn has_current(&self) -> bool {
    self.cursor_ < self.start_ + self.size_ as usize
  }
}
`;

const p = writeSrc("v8-codegen-core", "jump_table_info.rs", rust);
console.log(`jump-table-info: 布局常量 3 / Writer 方法 ${ccMethods.length} / Iterator 方法 4 → ${p}`);
