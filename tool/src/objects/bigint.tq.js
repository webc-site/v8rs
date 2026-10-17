#!/usr/bin/env bun
// tool/src/objects/bigint.tq.js — 覆盖 v8/src/objects/bigint.tq
// 组→文件映射 (本脚本管辖): 席位表差集 (.h 布局/位标志归既有 tool/src/objects/bigint.h.js)
// → v8-objects-js-async/src/bigint_tq.rs
// 机械提取: BigIntBase 外部声明 (generates 'BigInt') + BigInt/MutableBigInt 类型别名关系

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/bigint.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[bigint.tq.js] ${m}`); };

// ── 1. BigIntBase 外部声明 (布局在 .h, generates 'BigInt') ───────────────
must(tq.includes("extern class BigIntBase extends PrimitiveHeapObject\n    generates 'BigInt';"),
  "tq 缺 BigIntBase extends PrimitiveHeapObject generates 'BigInt'");

// ── 2. 类型别名 (BigInt / MutableBigInt 同基) ────────────────────────────
must(tq.includes("type BigInt extends BigIntBase;"), "tq 缺 type BigInt extends BigIntBase");
must(tq.includes("type MutableBigInt extends BigIntBase;"), "tq 缺 type MutableBigInt extends BigIntBase");

// ── 3. Convert 宏 (MutableBigInt → BigInt 规范化断言) ────────────────────
const conv = tq.match(/Convert<BigInt, MutableBigInt>\(i: MutableBigInt\): BigInt \{([\s\S]*?)\}/);
must(conv, "tq 缺 Convert<BigInt, MutableBigInt> 宏");
must(conv[1].includes("dcheck(bigint::IsCanonicalized(i));"), "Convert 宏缺 IsCanonicalized dcheck");
must(conv[1].includes("%RawDownCast<BigInt>"), "Convert 宏缺 RawDownCast");

let rust = banner("src/objects/bigint.tq.js", "src/objects/bigint.tq",
  "席位表差集: tq 类型别名面 (.h 布局/Sign/位标志归既有 bigint.h.js)");
rust += `//! BigInt tq 差集: 三个名字共享一个运行时布局。
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
`;

const p = writeSrc("v8-objects-js-async", "bigint_tq.rs", rust);
console.log(`bigint.tq.js: ${p} — BigIntBase 0 字段声明 + 2 别名 + Convert 宏锚, 断言 ${checks} 项全过`);
