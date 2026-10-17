// 由 tool/src/objects/symbol-table.cc.js 从 v8/src/objects/symbol-table.cc 生成; 修订就地改, 复跑覆盖
// 公开符号注册表: 慢速反查 (值 → 键)

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
