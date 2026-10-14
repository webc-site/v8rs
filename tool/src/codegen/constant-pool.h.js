#!/usr/bin/env bun
// tool/src/codegen/constant-pool.h.js
// 机械提取 v8/src/codegen/constant-pool-entry.h (+constant-pool.h 派发头):
// ConstantPoolEntry 常量/布局/签名骨架 → v8-codegen-core/src/constant_pool.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const entrySrc = readV8("src/codegen/constant-pool-entry.h");
const poolSrc = readV8("src/codegen/constant-pool.h");

const m = (re, what, src = entrySrc) => {
  const r = src.match(re);
  if (!r) throw new Error(`未找到 ${what}`);
  return r;
};

// 枚举与常量对数
const types = m(/enum Type \{ (\w+), (\w+), (\w+) \};/, "Type 枚举");
const access = m(/enum Access \{ (\w+), (\w+) \};/, "Access 枚举");
const sharing = m(/enum \{ SHARING_PROHIBITED = (-?\d+), SHARING_ALLOWED = (-?\d+) \};/, "SHARING 常量");
if (types[1] !== "INTPTR" || types[2] !== "DOUBLE" || types[3] !== "NUMBER_OF_TYPES")
  throw new Error("Type 枚举异常");
if (access[1] !== "REGULAR" || access[2] !== "OVERFLOWED") throw new Error("Access 枚举异常");
const SHARING_PROHIBITED = parseInt(sharing[1], 10);
const SHARING_ALLOWED = parseInt(sharing[2], 10);

// size(Type): INTPTR → kSystemPointerSize(8), DOUBLE → kDoubleSize(8)
if (!entrySrc.includes("static int size(Type type)")) throw new Error("缺 size(Type)");

// constant-pool.h 派发分支登记 (arm64/ppc/riscv 才有实现)
const poolBranches = [...poolSrc.matchAll(/#elif V8_TARGET_ARCH_(\w+)/g)].map((x) => x[1]);
if (poolBranches.length < 2) throw new Error("constant-pool.h 派发分支异常");

let rust = banner(
  "src/codegen/constant-pool.h.js",
  "src/codegen/constant-pool-entry.h;src/codegen/constant-pool.h",
  "ConstantPoolEntry 常量/布局/签名骨架 + 平台派发表",
);

rust += `
/// 原版 enum Type: 常量池条目类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Type {
  /// INTPTR: 指针宽整数 (kSystemPointerSize = 8)
  INTPTR = 0,
  /// DOUBLE: 双精度 (kDoubleSize = 8)
  DOUBLE = 1,
}

impl Type {
  /// NUMBER_OF_TYPES
  pub const NUMBER_OF_TYPES: usize = ${types.length - 1};

  /// 原版 size(Type): INTPTR → 8, DOUBLE → 8 (64 位目标)
  #[inline]
  #[must_use]
  pub const fn size(self) -> usize {
    match self {
      Self::INTPTR => 8, // kSystemPointerSize
      Self::DOUBLE => 8, // kDoubleSize
    }
  }
}

/// 原版 enum Access: 常规池 / 溢出池
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Access {
  REGULAR = 0,
  OVERFLOWED = 1,
}

/// 原版私有 SHARING 哨兵 (merged_index_ 复用为共享索引/偏移)
pub const SHARING_PROHIBITED: i32 = ${SHARING_PROHIBITED};
pub const SHARING_ALLOWED: i32 = ${SHARING_ALLOWED};

/// 原版 ConstantPoolEntry:
/// position_ | merged_index_ | union { value_ : isize, value64_ : u64 } | rmode_
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantPoolEntry {
  /// 池内位置 (原版 position_)
  position_: i32,
  /// 共享索引或偏移 (原版 merged_index_; SHARING_* 哨兵语义)
  merged_index_: i32,
  /// 载荷 (原版 union value_/value64_)
  value_: u64,
  /// 重定位模式 (原版 rmode_; RelocInfo::Mode 编码, 见 reloc_info)
  rmode_: u32,
}

/// RelocInfo::NO_INFO = 0 (默认重定位模式)
pub const RELOC_NO_INFO: u32 = 0;

impl Default for ConstantPoolEntry {
  fn default() -> Self {
    Self { position_: 0, merged_index_: SHARING_ALLOWED, value_: 0, rmode_: RELOC_NO_INFO }
  }
}

impl ConstantPoolEntry {
  /// 原版构造 1: (position, value, sharing_ok, rmode = NO_INFO)
  #[inline]
  #[must_use]
  pub const fn from_int(position: i32, value: i64, sharing_ok: bool, rmode: u32) -> Self {
    Self {
      position_: position,
      merged_index_: if sharing_ok { SHARING_ALLOWED } else { SHARING_PROHIBITED },
      value_: value as u64,
      rmode_: rmode,
    }
  }

  /// 原版构造 2: (position, double bits, rmode = NO_INFO) — sharing 恒 ALLOWED
  #[inline]
  #[must_use]
  pub const fn from_double(position: i32, value64: u64, rmode: u32) -> Self {
    Self {
      position_: position,
      merged_index_: SHARING_ALLOWED,
      value_: value64,
      rmode_: rmode,
    }
  }

  /// 原版 position()
  #[inline]
  #[must_use]
  pub const fn position(&self) -> i32 {
    self.position_
  }

  /// 原版 sharing_ok(): merged_index_ != SHARING_PROHIBITED
  #[inline]
  #[must_use]
  pub const fn sharing_ok(&self) -> bool {
    self.merged_index_ != SHARING_PROHIBITED
  }

  /// 原版 is_merged(): merged_index_ >= 0
  #[inline]
  #[must_use]
  pub const fn is_merged(&self) -> bool {
    self.merged_index_ >= 0
  }

  /// 原版 merged_index()
  #[inline]
  #[must_use]
  pub const fn merged_index(&self) -> i32 {
    self.merged_index_
  }

  /// 原版 set_merged_index(index)
  #[inline]
  pub const fn set_merged_index(&mut self, index: i32) {
    self.merged_index_ = index;
  }

  /// 原版 offset(): 读 merged_index_ 作偏移
  #[inline]
  #[must_use]
  pub const fn offset(&self) -> i32 {
    self.merged_index_
  }

  /// 原版 set_offset(offset)
  #[inline]
  pub const fn set_offset(&mut self, offset: i32) {
    self.merged_index_ = offset;
  }

  /// 原版 value(): intptr 载荷
  #[inline]
  #[must_use]
  pub const fn value(&self) -> i64 {
    self.value_ as i64
  }

  /// 原版 value64()
  #[inline]
  #[must_use]
  pub const fn value64(&self) -> u64 {
    self.value_
  }

  /// 原版 rmode()
  #[inline]
  #[must_use]
  pub const fn rmode(&self) -> u32 {
    self.rmode_
  }
}

/// 原版 constant-pool.h 平台派发表: 仅 arm64/ppc/riscv 有真实常量池实现;
/// x64 无独立常量池 (内联), 派发头为空。
pub const CONSTANT_POOL_ARCH_DISPATCH: &[(&str, bool)] = &[
  ("ARM64", true),
  ("PPC64", true),
  ("RISCV", true),
  ("X64", false), // 目标架构 x64: 无平台实现, 派发为空
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(Type::NUMBER_OF_TYPES == 2);
  assert!(Type::INTPTR.size() == 8 && Type::DOUBLE.size() == 8);
  assert!(SHARING_PROHIBITED == -2 && SHARING_ALLOWED == -1);
  let e = ConstantPoolEntry::from_int(3, -7, true, RELOC_NO_INFO);
  assert!(e.position() == 3 && e.value() == -7 && e.sharing_ok() && !e.is_merged());
  let d = ConstantPoolEntry::from_double(1, 0x3ff0_0000_0000_0000, RELOC_NO_INFO);
  assert!(d.value64() == 0x3ff0_0000_0000_0000 && d.sharing_ok());
};
`;

const p = writeSrc("v8-codegen-core", "constant_pool.rs", rust);
console.log(
  `constant-pool: Type 2 / Access 2 / SHARING 2 / 方法 12 / 派发 ${poolBranches.length + 1} → ${p}`,
);
