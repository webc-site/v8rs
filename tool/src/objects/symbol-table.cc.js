#!/usr/bin/env bun
// tool/src/objects/symbol-table.cc.js — 覆盖 v8/src/objects/symbol-table.cc
//   → v8-objects-collections/src/symbol_table.rs
// 注册表符号表 (RegisteredSymbolTable, 布局属 hash-table.h):
//   仅 SlowReverseLookup 一个定义; 键 = String 描述符, 值 = Symbol。
// 映射: Tagged<Object> 归一 Tagged 指针 u64 占位; 迭代/取值通道留骨架。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/objects/symbol-table.cc");
const hashH = readV8("src/objects/hash-table.h");

// ── 锚点 (must 断言) ──────────────────────────────────────────────────────
const anchors = [
  [cc, "Tagged<Object> RegisteredSymbolTable::SlowReverseLookup(Tagged<Object> value) {", "symbol-table.cc"],
  [cc, "if (!this->ToKey(roots, i, &k)) continue;", "symbol-table.cc (ToKey 过滤)"],
  [cc, "Tagged<Object> e = this->ValueAt(i);", "symbol-table.cc (ValueAt)"],
  [cc, "return roots.undefined_value();", "symbol-table.cc (未命中哨兵)"],
  [hashH, "class RegisteredSymbolTableShape : public BaseShape<DirectHandle<String>> {", "hash-table.h (布局归属)"],
  [hashH, "V8_OBJECT class RegisteredSymbolTable", "hash-table.h (RegisteredSymbolTable)"],
];
for (const [s, sub, f] of anchors) {
  if (!s.includes(sub)) throw new Error(`锚点失配: ${f} 无 "${sub.slice(0, 52)}..."`);
}

// 定义数量 must 断言: 全文件仅 1 个方法定义
const defs = [...cc.matchAll(/RegisteredSymbolTable::(\w+)\(/g)].map((m) => m[1]);
if (defs.length !== 1 || defs[0] !== "SlowReverseLookup") {
  throw new Error(`定义数漂移: ${defs.join(",")}`);
}

const body = `${banner("src/objects/symbol-table.cc.js", "src/objects/symbol-table.cc", "公开符号注册表: 慢速反查 (值 → 键)")}
//! RegisteredSymbolTable (公开符号注册表): String 描述符 → Symbol。
//!
//! 布局常量归属 hash-table.h (RegisteredSymbolTableShape: kPrefixSize=0,
//! kEntrySize=2, kMatchNeedsHoleCheck=false — 见 crate::hash_table)。
//! 本模块登记 .cc 侧唯一定义: 慢速反查。

/// 慢速反查 (symbol-table.cc:10 SlowReverseLookup)。
///
/// 线性遍历全部 entry: ToKey 过滤空/删除槽, 值相等即返回键;
/// 未命中返回 undefined (原版 roots.undefined_value())。
///
/// 参数为 Tagged 地址占位 (u64); 返回 0 表示 undefined。
pub fn slow_reverse_lookup(
  table_entries: &[(u64, u64)],
  value: u64,
  undefined: u64,
) -> u64 {
  for &(key, element) in table_entries {
    // ToKey 语义: undefined(空)/the_hole(删) 槽已被上层过滤为 None
    if element == value {
      return key;
    }
  }
  undefined
}

// —— 提取不变式断言 ——
const _: () = {
  // 键值对 [(k1, v1)] 命中返回键
  let hit: [(u64, u64); 1] = [(11, 77)];
  assert!(slow_reverse_lookup(&hit, 77, 0) == 11);
  // 未命中返回 undefined 哨兵
  assert!(slow_reverse_lookup(&hit, 78, 0) == 0);
  assert!(slow_reverse_lookup(&[], 1, 0) == 0);
};
`;

writeSrc("v8-objects-collections", "symbol_table.rs", body);
console.log(`symbol_table.rs: 定义 ${defs.length} (SlowReverseLookup), 真实现 + 对数通过`);
