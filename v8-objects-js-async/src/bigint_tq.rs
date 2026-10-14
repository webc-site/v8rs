// 由 tool/src/objects/bigint.tq.js 从 v8/src/objects/bigint.tq 生成; 修订就地改, 复跑覆盖
// 席位表差集: tq 类型别名面 (.h 布局/Sign/位标志归既有 bigint.h.js)
//! BigInt tq 差集: 三个名字共享一个运行时布局。
//!
//! BigIntBase 是 tq 侧的布局载体 (extern class, generates 'BigInt');
//! BigInt (不可变视图) 与 MutableBigInt (构造期可变视图) 均为类型别名。
//! 布局 (length/sign/digits) 归既有 bigint.h.js 产物。

/// BigIntBase: tq 零字段外部声明, 布局在 .h (BigInt: length + sign + digits)。
pub const BIGINT_BASE_TQ_NEW_FIELDS: usize = 0;

/// BigInt = 不可变视图 (类型别名, 同 BigIntBase 布局)。
pub const BIGINT_IS_ALIAS_OF_BASE: bool = true;

/// MutableBigInt = 构造期可变视图 (类型别名, 同 BigIntBase 布局)。
pub const MUTABLE_BIGINT_IS_ALIAS_OF_BASE: bool = true;

// ── Convert<BigInt, MutableBigInt> 宏语义 (tq; Rust 侧为视图转换) ──
//   前置 dcheck(bigint::IsCanonicalized(i)) — 规范化 (最高位非零, 零无符号)
//   转换 = RawDownCast<BigInt>, 零成本视图替换

// ── 提取不变式断言 ──
const _: () = {
  assert!(BIGINT_BASE_TQ_NEW_FIELDS == 0);
  assert!(BIGINT_IS_ALIAS_OF_BASE);
  assert!(MUTABLE_BIGINT_IS_ALIAS_OF_BASE);
};
