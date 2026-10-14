#!/usr/bin/env bun
// tool/src/compiler/machine-operator.cc.js
// 从 v8/src/compiler/machine-operator.cc 的 MACHINE_PURE_OP_LIST 宏尾部提取
// Pair 二元算子表项 (Int32Pair{Add,Sub,Mul} / Word32Pair{Shl,Shr,Sar}, cc:879-884),
// 与 machine-operator.h 无参 getter 声明 (h:573-578) 交叉核对后,
// 生成 v8-compiler-ops/src/machine_pair_operator.rs
//
// 台账锚定: [compiler-ops] BinaryOverlappingLsaLength 非精确符号 (v8/src 全库无此名),
// 转述对应 machine-operator 层的 Pair 二元算子表项。真实消费面 (独立复核):
// Int64Lowering 将 Int64 运算降级为本表算子 (int64-lowering.cc:457-577), 由 32 位后端
// arm/ia32/riscv32 的 instruction selector 消费 (VisitInt32PairAdd 等), CSA 直接构造
// (raw-machine-assembler.h:592-608, code-assembler.cc:764-774)。mips64/loong64 的
// kMips64Lsa/kMips64Dlsa (code-generator-mips64.cc:1149-1157) 是寻址指令 (rd = rt + rs<<sa),
// 本库中与本表无消费边; kLoadStorePairs 仅 arm64 selector 启用 (instruction-selector-arm64.cc:7431),
// Lsa 指令本体归架构 crate 推迟登记。
import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/compiler/machine-operator.cc");
const h = readV8("src/compiler/machine-operator.h");

// 1. 圈定 MACHINE_PURE_OP_LIST 宏区: 从 #define 行到首个不以反斜杠续行的行
const ccLines = cc.split("\n");
const begin = ccLines.findIndex((l) => l.startsWith("#define MACHINE_PURE_OP_LIST(V)"));
if (begin < 0) throw new Error("未找到 MACHINE_PURE_OP_LIST 宏");
const blockLines = [];
for (let i = begin; i < ccLines.length; i++) {
  blockLines.push(ccLines[i]);
  if (!ccLines[i].endsWith("\\")) break;
}

// 2. 提取 Pair 二元表项: V(Name, properties, value_input_count, control_input_count, output_count)
const entryRe =
  /V\(((?:Int32|Word32)Pair(?:Add|Sub|Mul|Shl|Shr|Sar)), Operator::(\w+), (\d+), (\d+), (\d+)\)/g;
const entries = [...blockLines.join("\n").matchAll(entryRe)].map((m) => ({
  name: m[1],
  props: m[2],
  valueIn: Number(m[3]),
  controlIn: Number(m[4]),
  valueOut: Number(m[5]),
}));
if (entries.length !== 6) throw new Error(`Pair 表项应为 6, 实得 ${entries.length}`);

// 3. 表内容与 cc:879-884 逐字段 1:1 断言 (kPure 由 PURE 宏族注入, 对应 Properties::PURE)
const expect = {
  Int32PairAdd: [4, 0, 2],
  Int32PairSub: [4, 0, 2],
  Int32PairMul: [4, 0, 2],
  Word32PairShl: [3, 0, 2],
  Word32PairShr: [3, 0, 2],
  Word32PairSar: [3, 0, 2],
};
for (const e of entries) {
  const want = expect[e.name];
  if (!want) throw new Error(`意外 Pair 表项: ${e.name}`);
  if (e.props !== "kNoProperties") throw new Error(`${e.name} 属性应为 kNoProperties`);
  if (
    want[0] !== e.valueIn ||
    want[1] !== e.controlIn ||
    want[2] !== e.valueOut
  ) {
    throw new Error(`${e.name} 计数 (${e.valueIn},${e.controlIn},${e.valueOut}) 与 cc 不符`);
  }
}

// 4. 与 machine-operator.h 的无参 getter 声明集合对数 (h:573-578)
const declRe =
  /const Operator\* ((?:Int32|Word32)Pair(?:Add|Sub|Mul|Shl|Shr|Sar))\(\);/g;
const decls = [...h.matchAll(declRe)].map((m) => m[1]).sort();
const names = entries.map((e) => e.name).sort();
if (decls.join(",") !== names.join(",")) {
  throw new Error(`h 声明与 cc 表项不齐: [${decls}] vs [${names}]`);
}

const snake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

const rustSrc = `${banner("src/compiler/machine-operator.cc.js", "src/compiler/machine-operator.cc + machine-operator.h")}
//! Machine Pair 二元算子表项
//!
//! 对应原版 machine-operator.cc MACHINE_PURE_OP_LIST 尾部 Pair 表项 (cc:879-884)
//! 与 machine-operator.h 的 Int32Pair*/Word32Pair* getter (h:573-578)。
//! 消费面: Int64Lowering 将 Int64 运算降级为本表算子 (int64-lowering.cc:457-577),
//! 由 32 位后端 (arm/ia32/riscv32) 的 instruction selector 消费, CSA 直接构造
//! pair 节点 (raw-machine-assembler.h)。

use std::sync::LazyLock;

use v8_compiler_graph::{IrOpcode, Operator, OperatorCounts, Properties};

use crate::machine_operator::MachineOperatorBuilder;

/// 全局共享 Machine Pair 纯算子缓存 (原版 MachineOperatorGlobalCache 的 Pair 表项分片)
pub struct MachinePairOperatorGlobalCache {
${entries.map((e) => `  pub ${snake(e.name)}: Operator,`).join("\n")}
}

/// Pair 表项构造: 原版 PURE 宏 Operator(kPure | properties, ...), 构造期不可能失败
fn mk(opcode: IrOpcode, name: &'static str, value_in: u32, value_out: u32) -> Operator {
  let r = Operator::new(
    opcode as u16,
    Properties::PURE,
    name,
    OperatorCounts::new(value_in, 0, 0, value_out, 0, 0),
  );
  match r {
    Ok(op) => op,
    Err(_) => unreachable!(),
  }
}

impl MachinePairOperatorGlobalCache {
  fn new() -> Self {
    Self {
${entries
  .map(
    (e) =>
      `      ${snake(e.name)}: mk(IrOpcode::${e.name}, "${e.name}", ${e.valueIn}, ${e.valueOut}),`,
  )
  .join("\n")}
    }
  }
}

static PAIR_CACHE: LazyLock<MachinePairOperatorGlobalCache> =
  LazyLock::new(MachinePairOperatorGlobalCache::new);

/// MachineOperatorBuilder 的 Pair 二元算子分片 (原版同一 builder 的 h:573-578 getter)
impl MachineOperatorBuilder<'_, '_> {
${entries
  .map(
    (e) => `  /// ${e.name} (${e.valueIn} 值输入 → ${e.valueOut} 值输出)
  #[inline]
  pub fn ${snake(e.name)}(&self) -> &'static Operator {
    &PAIR_CACHE.${snake(e.name)}
  }`,
  )
  .join("\n\n")}
}
`;

writeSrc("v8-compiler-ops", "machine_pair_operator.rs", rustSrc);
console.log(
  `written machine_pair_operator.rs (pair binary ops: cc=${entries.length}, h=${decls.length}, expect=6)`,
);
