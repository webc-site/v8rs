// 由 tool/src/compiler/machine-operator.cc.js 从 v8/src/compiler/machine-operator.cc + machine-operator.h 生成; 修订就地改, 复跑覆盖

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
  pub int32_pair_add: Operator,
  pub int32_pair_sub: Operator,
  pub int32_pair_mul: Operator,
  pub word32_pair_shl: Operator,
  pub word32_pair_shr: Operator,
  pub word32_pair_sar: Operator,
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
      int32_pair_add: mk(IrOpcode::Int32PairAdd, "Int32PairAdd", 4, 2),
      int32_pair_sub: mk(IrOpcode::Int32PairSub, "Int32PairSub", 4, 2),
      int32_pair_mul: mk(IrOpcode::Int32PairMul, "Int32PairMul", 4, 2),
      word32_pair_shl: mk(IrOpcode::Word32PairShl, "Word32PairShl", 3, 2),
      word32_pair_shr: mk(IrOpcode::Word32PairShr, "Word32PairShr", 3, 2),
      word32_pair_sar: mk(IrOpcode::Word32PairSar, "Word32PairSar", 3, 2),
    }
  }
}

static PAIR_CACHE: LazyLock<MachinePairOperatorGlobalCache> =
  LazyLock::new(MachinePairOperatorGlobalCache::new);

/// MachineOperatorBuilder 的 Pair 二元算子分片 (原版同一 builder 的 h:573-578 getter)
impl MachineOperatorBuilder<'_, '_> {
  /// Int32PairAdd (4 值输入 → 2 值输出)
  #[inline]
  pub fn int32_pair_add(&self) -> &'static Operator {
    &PAIR_CACHE.int32_pair_add
  }

  /// Int32PairSub (4 值输入 → 2 值输出)
  #[inline]
  pub fn int32_pair_sub(&self) -> &'static Operator {
    &PAIR_CACHE.int32_pair_sub
  }

  /// Int32PairMul (4 值输入 → 2 值输出)
  #[inline]
  pub fn int32_pair_mul(&self) -> &'static Operator {
    &PAIR_CACHE.int32_pair_mul
  }

  /// Word32PairShl (3 值输入 → 2 值输出)
  #[inline]
  pub fn word32_pair_shl(&self) -> &'static Operator {
    &PAIR_CACHE.word32_pair_shl
  }

  /// Word32PairShr (3 值输入 → 2 值输出)
  #[inline]
  pub fn word32_pair_shr(&self) -> &'static Operator {
    &PAIR_CACHE.word32_pair_shr
  }

  /// Word32PairSar (3 值输入 → 2 值输出)
  #[inline]
  pub fn word32_pair_sar(&self) -> &'static Operator {
    &PAIR_CACHE.word32_pair_sar
  }
}
