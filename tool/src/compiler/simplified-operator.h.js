#!/usr/bin/env bun
// tool/src/compiler/simplified-operator.h.js
// 从 v8/src/compiler/simplified-operator.h + simplified-operator.cc
// 提取投机检查算子与纯算子，生成 v8-compiler-ops/src/simplified_operator.rs
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/simplified-operator.h");
const cc = readV8("src/compiler/simplified-operator.cc");

// 提取 PURE_OP_LIST
const pureMatch = cc.match(/#define PURE_OP_LIST\(V\)\s*\\([\s\S]*?)\n\n/);
if (!pureMatch) throw new Error("未找到 PURE_OP_LIST");
const pureOps = [];
for (const line of pureMatch[1].split("\n")) {
  const m = line.match(/V\((\w+),\s*Operator::k(\w+),\s*(\d+),\s*(\d+)\)/);
  if (m) {
    const prop = m[2] === "NoProperties" ? "Properties::PURE" : `Properties::PURE | Properties::${m[2].replace(/^No/, "NO_").toUpperCase()}`;
    pureOps.push({
      name: m[1],
      prop,
      value_in: Number(m[3]),
      control_in: Number(m[4]),
    });
  }
}

// 提取 CHECKED_OP_LIST
const checkedMatch = cc.match(/#define CHECKED_OP_LIST\(V\)\s*\\([\s\S]*?)\n\n/);
if (!checkedMatch) throw new Error("未找到 CHECKED_OP_LIST");
const checkedOps = [];
for (const line of checkedMatch[1].split("\n")) {
  const m = line.match(/V\((\w+),\s*(\d+),\s*(\d+)\)/);
  if (m) {
    checkedOps.push({
      name: m[1],
      value_in: Number(m[2]),
      value_out: Number(m[3]),
    });
  }
}

const toSnakeCase = (str) =>
  str.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const rustSrc = `${banner("src/compiler/simplified-operator.h.js", "src/compiler/simplified-operator.h + simplified-operator.cc")}
//! SimplifiedOperatorBuilder 与 Simplified 算子族
//!
//! 对应原版 simplified-operator.h / simplified-operator.cc。
//! 包含 NumberAdd, NumberSubtract, CheckBounds, CheckMaps 等投机优化算子。

use std::sync::LazyLock;
use v8_compiler_graph::{
  CheckForMinusZeroMode, FeedbackSource, IrOpcode, Operator, Operator1, OperatorCounts, Properties,
};
use v8_zone::Zone;

use crate::{common_operator::DeoptimizeReason, Error};

/// CheckBounds 标志位 (CheckBoundsFlags)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CheckBoundsFlags(u8);

impl CheckBoundsFlags {
  pub const NONE: Self = Self(0);
  pub const CONVERT_STRING_AND_MINUS_ZERO: Self = Self(1 << 0);
  pub const ABORT_ON_OUT_OF_BOUNDS: Self = Self(1 << 1);

  pub const fn bits(self) -> u8 {
    self.0
  }
}

/// CheckBounds 参数 (CheckBoundsParameters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckBoundsParameters {
  pub feedback: FeedbackSource,
  pub flags: CheckBoundsFlags,
}

impl CheckBoundsParameters {
  pub const fn new(feedback: FeedbackSource, flags: CheckBoundsFlags) -> Self {
    Self { feedback, flags }
  }
}

/// CheckMaps 标志位 (CheckMapsFlags)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CheckMapsFlags(u8);

impl CheckMapsFlags {
  pub const NONE: Self = Self(0);
  pub const CAN_BE_CLEARED: Self = Self(1 << 0);
  pub const TRY_MIGRATE_INSTANCE: Self = Self(1 << 1);

  pub const fn bits(self) -> u8 {
    self.0
  }
}

/// CheckMaps 参数 (CheckMapsParameters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckMapsParameters {
  pub flags: CheckMapsFlags,
  pub feedback: FeedbackSource,
}

impl CheckMapsParameters {
  pub const fn new(flags: CheckMapsFlags, feedback: FeedbackSource) -> Self {
    Self { flags, feedback }
  }
}

/// 全局共享 Simplified 算子缓存
pub struct SimplifiedOperatorGlobalCache {
${pureOps.map((op) => `  pub ${toSnakeCase(op.name)}: Operator,`).join("\n")}
${checkedOps.map((op) => `  pub ${toSnakeCase(op.name)}: Operator,`).join("\n")}
  pub check_bounds: Operator1<CheckBoundsParameters>,
  pub check_maps: Operator1<CheckMapsParameters>,
}

#[inline]
fn unwrap_op<T>(res: core::result::Result<T, v8_compiler_graph::Error>) -> T {
  match res {
    Ok(v) => v,
    Err(_) => unreachable!(),
  }
}

impl SimplifiedOperatorGlobalCache {
  fn new() -> Self {
    Self {
${pureOps
  .map(
    (op) => `      ${toSnakeCase(op.name)}: unwrap_op(Operator::new(
        IrOpcode::${op.name} as u16,
        ${op.prop},
        "${op.name}",
        OperatorCounts::new(${op.value_in}, 0, ${op.control_in}, 1, 0, 0),
      )),`,
  )
  .join("\n")}
${checkedOps
  .map(
    (op) => `      ${toSnakeCase(op.name)}: unwrap_op(Operator::new(
        IrOpcode::${op.name} as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "${op.name}",
        OperatorCounts::new(${op.value_in}, 1, 1, ${op.value_out}, 1, 0),
      )),`,
  )
  .join("\n")}
      check_bounds: unwrap_op(Operator1::new(
        IrOpcode::CheckBounds as u16,
        Properties::FOLDABLE | Properties::NO_THROW,
        "CheckBounds",
        OperatorCounts::new(2, 1, 1, 1, 1, 0),
        CheckBoundsParameters::new(FeedbackSource::invalid(), CheckBoundsFlags::NONE),
      )),
      check_maps: unwrap_op(Operator1::new(
        IrOpcode::CheckMaps as u16,
        Properties::NO_THROW | Properties::NO_WRITE,
        "CheckMaps",
        OperatorCounts::new(1, 1, 1, 0, 1, 0),
        CheckMapsParameters::new(CheckMapsFlags::NONE, FeedbackSource::invalid()),
      )),
    }
  }
}

unsafe impl Send for SimplifiedOperatorGlobalCache {}
unsafe impl Sync for SimplifiedOperatorGlobalCache {}

static SIMPLIFIED_CACHE: LazyLock<SimplifiedOperatorGlobalCache> =
  LazyLock::new(SimplifiedOperatorGlobalCache::new);

/// SimplifiedOperatorBuilder
pub struct SimplifiedOperatorBuilder<'z, 'a> {
  zone: &'z Zone<'a>,
}

impl<'z, 'a> SimplifiedOperatorBuilder<'z, 'a> {
  /// 创建构建器
  pub fn new(zone: &'z Zone<'a>) -> Self {
    Self { zone }
  }

  /// 底层 Zone 引用
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  // —— 纯算子 ——

${pureOps
  .map(
    (op) => `  /// ${op.name}
  #[inline]
  pub fn ${toSnakeCase(op.name)}(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.${toSnakeCase(op.name)}
  }`,
  )
  .join("\n\n")}

  // —— Checked 算子 ——

${checkedOps
  .map(
    (op) => `  /// ${op.name}
  #[inline]
  pub fn ${toSnakeCase(op.name)}(&self) -> &'static Operator {
    &SIMPLIFIED_CACHE.${toSnakeCase(op.name)}
  }`,
  )
  .join("\n\n")}

  /// CheckBounds 算子
  pub fn check_bounds(
    &self,
    feedback: FeedbackSource,
    flags: CheckBoundsFlags,
  ) -> Result<&'z Operator, Error> {
    if !feedback.is_valid() && flags.bits() == 0 {
      return Ok(&SIMPLIFIED_CACHE.check_bounds.base);
    }
    let op = Operator1::new(
      IrOpcode::CheckBounds as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckBounds",
      OperatorCounts::new(2, 1, 1, 1, 1, 0),
      CheckBoundsParameters::new(feedback, flags),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckMaps 算子
  pub fn check_maps(
    &self,
    flags: CheckMapsFlags,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    if !feedback.is_valid() && flags.bits() == 0 {
      return Ok(&SIMPLIFIED_CACHE.check_maps.base);
    }
    let mut props = Properties::NO_THROW;
    if (flags.bits() & CheckMapsFlags::TRY_MIGRATE_INSTANCE.bits()) == 0 {
      props |= Properties::NO_WRITE;
    }
    let op = Operator1::new(
      IrOpcode::CheckMaps as u16,
      props,
      "CheckMaps",
      OperatorCounts::new(1, 1, 1, 0, 1, 0),
      CheckMapsParameters::new(flags, feedback),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckNumber 算子
  pub fn check_number(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::CheckNumber as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckNumber",
      OperatorCounts::new(1, 1, 1, 1, 1, 0),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckSmi 算子
  pub fn check_smi(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::CheckSmi as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckSmi",
      OperatorCounts::new(1, 1, 1, 1, 1, 0),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckString 算子
  pub fn check_string(&self, feedback: FeedbackSource) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::CheckString as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckString",
      OperatorCounts::new(1, 1, 1, 1, 1, 0),
      feedback,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckIf 算子
  pub fn check_if(
    &self,
    reason: DeoptimizeReason,
    feedback: FeedbackSource,
  ) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::CheckIf as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckIf",
      OperatorCounts::new(1, 1, 1, 0, 1, 0),
      crate::common_operator::DeoptimizeParameters::new(reason, feedback),
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// CheckedInt32Mul 算子
  pub fn checked_int32_mul(&self, mode: CheckForMinusZeroMode) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::CheckedInt32Mul as u16,
      Properties::FOLDABLE | Properties::NO_THROW,
      "CheckedInt32Mul",
      OperatorCounts::new(2, 1, 1, 1, 1, 0),
      mode,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }
}
`;

writeSrc("v8-compiler-ops", "simplified_operator.rs", rustSrc);
console.log(`written simplified_operator.rs (${pureOps.length} pure ops, ${checkedOps.length} checked ops)`);
