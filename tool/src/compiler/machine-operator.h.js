#!/usr/bin/env bun
// tool/src/compiler/machine-operator.h.js
// 从 v8/src/compiler/machine-operator.h + machine-operator.cc (+ write-barrier-kind.h)
// 提取 MachineOperator 常量与签名，生成 v8-compiler-ops/src/machine_operator.rs
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/machine-operator.h");
const cc = readV8("src/compiler/machine-operator.cc");
const wbH = readV8("src/compiler/write-barrier-kind.h");

// 1. 提取 WriteBarrierKind
const wbKinds = [];
const wbMatches = wbH.matchAll(/k(\w+Barrier),?/g);
for (const m of wbMatches) {
  wbKinds.push(m[1]);
}
if (wbKinds.length < 5) throw new Error("WriteBarrierKind 提取不足");

// 2. 提取 PURE_BINARY_OP_LIST_32
const parsePureList = (macroName) => {
  const re = new RegExp(`#define ${macroName}\\(V\\)\\s*\\\\([\\s\\S]*?)(?:\\n\\n|#define)`);
  const m = cc.match(re);
  if (!m) throw new Error(`未找到 ${macroName}`);
  const list = [];
  for (const line of m[1].split("\n")) {
    const item = line.match(/V\((\w+),\s*([^,]+),\s*(\d+),\s*(\d+),\s*(\d+)\)/);
    if (item) {
      const propStr = item[2].trim();
      let prop = "Properties::PURE";
      if (propStr !== "Operator::kNoProperties") {
        const parts = propStr.split("|").map((p) => {
          const t = p.trim().replace(/^Operator::k/, "");
          return `Properties::${t.replace(/^No/, "NO_").toUpperCase()}`;
        });
        prop = `Properties::PURE | ${parts.join(" | ")}`;
      }
      list.push({
        name: item[1],
        prop,
        value_in: Number(item[3]),
        control_in: Number(item[4]),
        value_out: Number(item[5]),
      });
    }
  }
  return list;
};

const pure32 = parsePureList("PURE_BINARY_OP_LIST_32");
const pure64 = parsePureList("PURE_BINARY_OP_LIST_64");

// 常用浮点/比较算子集合
const floatAndCompareOps = [
  { name: "Float32Add", prop: "Properties::PURE | Properties::COMMUTATIVE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float32Sub", prop: "Properties::PURE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float32Mul", prop: "Properties::PURE | Properties::COMMUTATIVE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float32Div", prop: "Properties::PURE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float32Equal", prop: "Properties::PURE | Properties::COMMUTATIVE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float32LessThan", prop: "Properties::PURE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float32LessThanOrEqual", prop: "Properties::PURE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float64Add", prop: "Properties::PURE | Properties::COMMUTATIVE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float64Sub", prop: "Properties::PURE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float64Mul", prop: "Properties::PURE | Properties::COMMUTATIVE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float64Div", prop: "Properties::PURE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float64Mod", prop: "Properties::PURE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float64Equal", prop: "Properties::PURE | Properties::COMMUTATIVE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float64LessThan", prop: "Properties::PURE", value_in: 2, control_in: 0, value_out: 1 },
  { name: "Float64LessThanOrEqual", prop: "Properties::PURE", value_in: 2, control_in: 0, value_out: 1 },
];

const allPureOps = [...pure32, ...pure64, ...floatAndCompareOps];

const toSnakeCase = (str) =>
  str.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const rustSrc = `${banner("src/compiler/machine-operator.h.js", "src/compiler/machine-operator.h + machine-operator.cc")}
//! MachineOperatorBuilder 与 Machine 算子族
//!
//! 对应原版 machine-operator.h / machine-operator.cc。
//! 包含 Word32And, Word64And, Int32Add, Float64Add, Load, Store 等，
//! 支持 LoadRepresentation 与 StoreRepresentation。

use std::sync::LazyLock;
use v8_compiler_graph::{
  IrOpcode, MachineRepresentation, MachineType, Operator, Operator1, OperatorCounts, Properties,
};
use v8_zone::Zone;

use crate::Error;

/// 写屏障种类 (WriteBarrierKind)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum WriteBarrierKind {
  #[default]
  NoWriteBarrier = 0,
  AssertNoWriteBarrier = 1,
  SkippedWriteBarrier = 2,
  MapWriteBarrier = 3,
  PointerWriteBarrier = 4,
  IndirectPointerWriteBarrier = 5,
  EphemeronKeyWriteBarrier = 6,
  FullWriteBarrier = 7,
}

/// LoadRepresentation = MachineType (1:1 原版 using LoadRepresentation = MachineType)
pub type LoadRepresentation = MachineType;

/// StoreRepresentation (机器表示 + 写屏障类型)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreRepresentation {
  representation: MachineRepresentation,
  write_barrier_kind: WriteBarrierKind,
}

impl StoreRepresentation {
  /// 创建 StoreRepresentation
  pub const fn new(representation: MachineRepresentation, write_barrier_kind: WriteBarrierKind) -> Self {
    Self {
      representation,
      write_barrier_kind,
    }
  }

  /// 机器表示
  #[inline]
  pub const fn representation(&self) -> MachineRepresentation {
    self.representation
  }

  /// 写屏障种类
  #[inline]
  pub const fn write_barrier_kind(&self) -> WriteBarrierKind {
    self.write_barrier_kind
  }
}

/// 全局共享 Machine 纯算子缓存
pub struct MachineOperatorGlobalCache {
${allPureOps.map((op) => `  pub ${toSnakeCase(op.name)}: Operator,`).join("\n")}
}

#[inline]
fn unwrap_op<T>(res: core::result::Result<T, v8_compiler_graph::Error>) -> T {
  match res {
    Ok(v) => v,
    Err(_) => unreachable!(),
  }
}

impl MachineOperatorGlobalCache {
  fn new() -> Self {
    Self {
${allPureOps
  .map(
    (op) => `      ${toSnakeCase(op.name)}: unwrap_op(Operator::new(
        IrOpcode::${op.name} as u16,
        ${op.prop},
        "${op.name}",
        OperatorCounts::new(${op.value_in}, 0, ${op.control_in}, ${op.value_out}, 0, 0),
      )),`,
  )
  .join("\n")}
    }
  }
}

static MACHINE_CACHE: LazyLock<MachineOperatorGlobalCache> =
  LazyLock::new(MachineOperatorGlobalCache::new);

/// MachineOperatorBuilder
pub struct MachineOperatorBuilder<'z, 'a> {
  zone: &'z Zone<'a>,
}

impl<'z, 'a> MachineOperatorBuilder<'z, 'a> {
  /// 创建构建器
  pub fn new(zone: &'z Zone<'a>) -> Self {
    Self { zone }
  }

  /// 底层 Zone 引用
  pub fn zone(&self) -> &'z Zone<'a> {
    self.zone
  }

  // —— 纯计算算子 ——

${allPureOps
  .map(
    (op) => `  /// ${op.name}
  #[inline]
  pub fn ${toSnakeCase(op.name)}(&self) -> &'static Operator {
    &MACHINE_CACHE.${toSnakeCase(op.name)}
  }`,
  )
  .join("\n\n")}

  /// Load 算子
  pub fn load(&self, rep: LoadRepresentation) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::Load as u16,
      Properties::ELIMINATABLE,
      "Load",
      OperatorCounts::new(2, 1, 1, 1, 1, 0),
      rep,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// Store 算子
  pub fn store(&self, rep: StoreRepresentation) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::Store as u16,
      Properties::NO_DEOPT | Properties::NO_READ | Properties::NO_THROW,
      "Store",
      OperatorCounts::new(3, 1, 1, 0, 1, 0),
      rep,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// UnalignedLoad 算子
  pub fn unaligned_load(&self, rep: LoadRepresentation) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::UnalignedLoad as u16,
      Properties::ELIMINATABLE,
      "UnalignedLoad",
      OperatorCounts::new(2, 1, 1, 1, 1, 0),
      rep,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }

  /// UnalignedStore 算子
  pub fn unaligned_store(&self, rep: MachineRepresentation) -> Result<&'z Operator, Error> {
    let op = Operator1::new(
      IrOpcode::UnalignedStore as u16,
      Properties::NO_DEOPT | Properties::NO_READ | Properties::NO_THROW,
      "UnalignedStore",
      OperatorCounts::new(3, 1, 1, 0, 1, 0),
      rep,
    )?;
    let allocated = self.zone.make(op)?;
    Ok(&allocated.base)
  }
}
`;

writeSrc("v8-compiler-ops", "machine_operator.rs", rustSrc);
console.log(`written machine_operator.rs (${allPureOps.length} pure ops, ${wbKinds.length} write barrier kinds)`);
