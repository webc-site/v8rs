#!/usr/bin/env bun
// tool/src/roots/roots.cc.js
// 机械提取 v8/src/roots/roots.{cc,-inl.h}:
// RootIndex↔Internals 静态断言、root_names_、ReadOnlyRoots/RootsTable 签名骨架
// (RootIndex 枚举本体已由 tool/src/roots/roots.h.js 产出 root_index.rs, 不重复)

import { readV8, writeSrc, banner } from "../../lib.js";

const ccSrc = readV8("src/roots/roots.cc");
const inlSrc = readV8("src/roots/roots-inl.h");

// 1. cc 顶部 static_assert 对数: RootIndex ↔ include/v8-internal.h 常量
const asserts = [...ccSrc.matchAll(/static_assert\(static_cast<int>\(RootIndex::(k\w+)\)\s*==\s*Internals::(k\w+RootIndex)\)/g)];
if (asserts.length !== 6) throw new Error(`RootIndex 静态断言应 6, 实得 ${asserts.length}`);
const pairs = asserts.map((m) => ({ root: m[1], intern: m[2] }));
if (pairs[0].root !== "kUndefinedValue" || pairs[5].root !== "kempty_string") {
  throw new Error("断言序漂移: " + pairs.map((p) => p.root).join(","));
}

// 2. include/v8-internal.h 取对应数值 (对数基准侧)
const internalsSrc = readV8("include/v8-internal.h");
const internalsVals = pairs.map((p) => {
  const m = internalsSrc.match(new RegExp(`\\b${p.intern}\\s*=\\s*(\\d+)`));
  if (!m) throw new Error(`v8-internal.h 未找到 ${p.intern}`);
  return { ...p, val: parseInt(m[1], 10) };
});

// 3. 签名骨架清单 (机械提取, 对数)
const mustCc = [
  "RootsTable::FindHeapNumber",
  "ReadOnlyRoots::one_pointer_filler_map_word",
  "ReadOnlyRoots::Iterate",
  "ReadOnlyRoots::VerifyNameForProtectorsPages",
  "ReadOnlyRoots::InitFromStaticRootsTable",
  "RootsTable::root_names_",
];
for (const k of mustCc) {
  if (!ccSrc.includes(k)) throw new Error(`roots.cc 未找到 ${k}`);
}
const mustInl = [
  "operator<(RootIndex",
  "operator++(RootIndex",
  "RootsTable::IsRootHandleLocation",
  "RootsTable::IsRootHandle",
  "RootsTable::handle_at",
  "ReadOnlyRoots::ReadOnlyRoots",
];
for (const k of mustInl) {
  if (!inlSrc.includes(k)) throw new Error(`roots-inl.h 未找到 ${k}`);
}

// 4. DEBUG 校验段登记 (机械存在性)
const debugFns = ["VerifyNameForProtectors", "VerifyTypes"];
for (const k of debugFns) {
  if (!ccSrc.includes(`ReadOnlyRoots::${k}`)) throw new Error(`roots.cc DEBUG 段未找到 ${k}`);
}

let rust = banner("src/roots/roots.cc.js", "src/roots/roots.{cc,-inl.h}", "根表实现层: 断言/名称表/签名骨架");

rust += `
use super::root_index::RootIndex;

/// RootIndex 序与公开 API 索引常量的静态对齐 (原版 roots.cc 顶部 static_assert, 6 条)
pub mod root_index_assertions {
${internalsVals
  .map(
    (p) => `  /// RootIndex::${p.root} == v8-internal.h ${p.intern} = ${p.val}
  pub const ${p.root.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()}_ROOT_INDEX: u32 = ${p.val};`,
  )
  .join("\n")}

  /// 总数守卫: 6 条断言
  pub const ASSERT_COUNT: usize = 6;
}

/// 根名称表 (原版 RootsTable::root_names_[kEntriesCount], 宏 ROOT_NAME 展开
/// 由 tool/src/roots/roots.h.js 的 root_index.rs 承接, 此处登记签名)
pub mod roots_table_skel {
  /// 根下标比较 (原版 -inl operator<)
  pub fn root_index_lt(_lhs: RootIndex, _rhs: RootIndex) -> bool {
    unimplemented!("骨架")
  }

  /// 根下标自增 (原版 -inl operator++, 越末项为 UB)
  pub fn root_index_next(_index: &mut RootIndex) {
    unimplemented!("骨架")
  }

  /// 判定句柄位置是否落在根表内并回填下标 (原版 IsRootHandleLocation, -inl)
  pub fn is_root_handle_location(_location: usize) -> Option<RootIndex> {
    unimplemented!("骨架")
  }

  /// 间接句柄版 (原版 IsRootHandle, -inl)
  pub fn is_root_handle(_handle_addr: usize) -> Option<RootIndex> {
    unimplemented!("骨架")
  }

  /// 按下标取根槽地址 (原版 RootsTable::operator[])
  pub fn root_at(_index: RootIndex) -> usize {
    unimplemented!("骨架")
  }

  /// 按下标取根句柄 (原版 handle_at, -inl)
  pub fn handle_at(_index: RootIndex) -> usize {
    unimplemented!("骨架")
  }

  /// 在 HeapNumber 只读根区间内按位型查找 (原版 FindHeapNumber, bit_cast 比对)
  pub fn find_heap_number(_value: f64) -> Option<usize> {
    unimplemented!("骨架")
  }

  /// 根槽是否落在根表范围内 (原版 IsRootHandleLocation, -inl)
  pub fn is_root_slot(_slot: usize) -> bool {
    unimplemented!("骨架")
  }
}

/// 只读根实现骨架 (原版 ReadOnlyRoots)
pub mod read_only_roots_impl_skel {
  /// one_pointer_filler 的 MapWord (原版 one_pointer_filler_map_word)
  pub fn one_pointer_filler_map_word() -> usize {
    unimplemented!("骨架")
  }

  /// 遍历只读根 (原版 Iterate: VisitRootPointers + Synchronize(kReadOnlyRootList))
  pub fn iterate(_visitor: &mut dyn FnMut(usize, usize)) {
    unimplemented!("骨架")
  }

  /// protector 命名区同页校验 (原版 VerifyNameForProtectorsPages)
  pub fn verify_name_for_protectors_pages() {
    unimplemented!("骨架")
  }

  /// DEBUG: protector 命名相邻性逐项校验 (原版 VerifyNameForProtectors)
  pub fn verify_name_for_protectors() {
    unimplemented!("骨架")
  }

  /// DEBUG: 只读根类型校验 (原版 VerifyTypes, 含 oddball kind 慢检与 trusted map 检)
  pub fn verify_types() {
    unimplemented!("骨架")
  }

  /// 从静态根表初始化 (原版 InitFromStaticRootsTable:
  /// 逐项 DecompressTagged 填 read_only_roots_, 末项必须 == kLastReadOnlyRoot)
  pub fn init_from_static_roots_table(_cage_base: usize, _table: &[u32]) {
    unimplemented!("骨架")
  }
}
`;

const p = writeSrc("v8-roots", "roots_impl_generated.rs", rust);
console.log(`written ${p}; 静态断言=${asserts.length}, cc签名=${mustCc.length}, inl签名=${mustInl.length}, DEBUG=${debugFns.length}`);
